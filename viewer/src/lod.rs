//! S234 — LOD spatial de la grille projetée : la densité suit la charge que le contenu exige.
//!
//! **Critère, à provenance.** Sur un triangle de sommets `v_i`, l'interpolation linéaire d'un
//! champ `f` vérifie `f(x) − Σ λ_i f(v_i) = −½ Σ λ_i (v_i − x)ᵀ H (v_i − x)`, et
//! `Σ λ_i |v_i − x|² = R² − |x − c|² ≤ R²` (cercle circonscrit). D'où
//! `|erreur| ≤ ½ · M · R²`, `M` majorant la norme de la hessienne. Pour une cellule de côtés
//! `h_r` (le long de la visée) et `h_l` (latéral) coupée en deux triangles rectangles,
//! `R² = (h_r² + h_l²)/4`. La tolérance est celle de l'image S201 (résidu d'intersection
//! < 3 mm), déjà reprise par `Gpu::verify` aux sommets.

/// Tolérance de hauteur entre sommets : résidu d'intersection de l'image de référence S201.
pub const TOLERANCE_M: f32 = 0.003;

/// Majorants des dérivées d'ordre deux et trois d'une couche publiée à l'image.
#[derive(Clone, Copy, Debug, Default)]
pub struct Bound {
    /// Majorant de la norme spectrale de la hessienne de η (1/m).
    pub hessian: f32,
    /// Majorant de la dérivée troisième de η (1/m²) ; sert à publier l'erreur de pente.
    pub third: f32,
}

/// B : lignes `[amplitude, kx, ky, phase]`. Hessienne d'une onde plane : `a·k kᵀ·sin`, de norme
/// `|a|·k²` ; somme des valeurs absolues, seule forme sans hypothèse de phase.
pub fn background(components: &[[f32; 4]]) -> Bound {
    plane(components.iter().map(|c| (c[0].abs(), c[1].hypot(c[2]))))
}

/// Sillage : lignes `[A, B, kx, ky]`, `η = A cos − B sin`, amplitude `√(A²+B²)`.
pub fn wake(components: &[[f32; 4]]) -> Bound {
    plane(components.iter().map(|c| (c[0].hypot(c[1]), c[2].hypot(c[3]))))
}

fn plane(rows: impl Iterator<Item = (f32, f32)>) -> Bound {
    let (mut h, mut t) = (0f64, 0f64);
    for (a, k) in rows {
        let (a, k) = (a as f64, k as f64);
        h += a * k * k;
        t += a * k * k * k;
    }
    Bound {
        hessian: h as f32,
        third: t as f32,
    }
}

/// Impact : profil radial `(η, η')` au pas `step`, interpolé par Hermite comme dans le shader.
///
/// Pour un champ radial, la hessienne a pour valeurs propres `η''(r)` et `η'(r)/r`. Si
/// `η'(0) = 0`, `|η'(r)/r| ≤ max_[0,r] |η''|` : la norme est donc bornée par `max |η''|` du
/// polynôme par morceaux, atteint à une extrémité de segment (`η''` y est affine). Sinon le
/// terme `|η'(0)|/r` n'est pas borné, et la fonction le signale en rendant `None`.
pub fn impact(profile: &[(f32, f32)], step: f32) -> Option<Bound> {
    if profile.len() < 2 {
        return Some(Bound::default());
    }
    if profile[0].1 != 0. {
        return None;
    }
    let s = step as f64;
    let (mut h, mut t) = (0f64, 0f64);
    for w in profile.windows(2) {
        let (p0, p1) = (w[0].0 as f64, w[1].0 as f64);
        let (m0, m1) = (w[0].1 as f64 * s, w[1].1 as f64 * s);
        let at0 = -6. * p0 - 4. * m0 + 6. * p1 - 2. * m1;
        let at1 = 6. * p0 + 2. * m0 - 6. * p1 + 4. * m1;
        h = h.max(at0.abs().max(at1.abs()) / (s * s));
        t = t.max((12. * (p0 - p1) + 6. * (m0 + m1)).abs() / (s * s * s));
    }
    Some(Bound {
        hessian: h as f32,
        third: t as f32,
    })
}

/// Majorants `Σ|a|k⁴` et `Σ|a|k⁵` du sillage, pour sa reconstruction bicubique.
pub fn wake_smooth(components: &[[f32; 4]]) -> (f32, f32) {
    let (mut m4, mut m5) = (0f64, 0f64);
    for c in components {
        let a = c[0].hypot(c[1]) as f64;
        let k = c[2].hypot(c[3]) as f64;
        m4 += a * k.powi(4);
        m5 += a * k.powi(5);
    }
    (m4 as f32, m5 as f32)
}

/// Erreur maximale d'une reconstruction Hermite bicubique (valeurs, dérivées premières et
/// croisée exactes aux nœuds) sur une maille carrée de côté `h`.
///
/// En 1D, `|f − H f| ≤ h⁴/384 · max|f⁗|`. Pour le produit tensoriel, `f − HxHy f =
/// (f − Hx f) + Hx(f − Hy f)` ; les fonctions de base des valeurs sont positives et de somme un,
/// celles des dérivées ont `|ψ0|+|ψ1| ≤ h/4`, d'où
/// `|f − HxHy f| ≤ h⁴/384 · (max|f_xxxx| + max|f_yyyy| + h/4 · max|f_xyyyy|)`, et pour une somme
/// d'ondes planes `≤ h⁴/384 · (2·Σ|a|k⁴ + h/4 · Σ|a|k⁵)`.
pub fn bicubic_error(h: f32, m4: f32, m5: f32) -> f32 {
    h.powi(4) / 384. * (2. * m4 + h / 4. * m5)
}

/// Plus grand pas de maille dont l'erreur bicubique reste sous la tolérance, par dichotomie.
pub fn bicubic_step(m4: f32, m5: f32) -> f32 {
    if m4 <= 0. && m5 <= 0. {
        return f32::INFINITY;
    }
    let (mut lo, mut hi) = (0f32, 64f32);
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if bicubic_error(mid, m4, m5) <= TOLERANCE_M {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

/// Pas isotrope maximal d'une cellule sous la tolérance : `(2h²)/4 · M/2 ≤ tol`.
pub fn isotropic_step(hessian: f32) -> f32 {
    if hessian <= 0. {
        f32::INFINITY
    } else {
        (4. * TOLERANCE_M / hessian).sqrt()
    }
}

/// Paramètres de projection de `ocean_vertex`, sans la hauteur (la grille se pose sur z = 0).
#[derive(Clone, Copy, Debug)]
pub struct Projection {
    pub eye: [f32; 3],
    pub forward: [f32; 3],
    pub right: [f32; 3],
    pub up: [f32; 3],
    pub tan_half: f32,
    pub aspect: f32,
}

impl Projection {
    /// Ordonnée écran de la dernière rangée, comme `ocean_vertex`.
    pub fn horizon(&self) -> f32 {
        (-self.forward[2] / (self.up[2] * self.tan_half)).clamp(-0.95, 1.2) - 0.003
    }
    /// Point horizontal relatif à la caméra pour une abscisse et une ordonnée écran de grille.
    pub fn ground(&self, x: f32, y: f32) -> [f32; 2] {
        let ray: [f32; 3] = core::array::from_fn(|i| {
            self.forward[i]
                + self.right[i] * x * self.tan_half * self.aspect
                + self.up[i] * y * self.tan_half
        });
        let distance = (self.eye[2] / (-ray[2]).max(0.00001)).min(1500.);
        [ray[0] * distance, ray[1] * distance]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_wave_bound_is_a_k_squared() {
        let b = background(&[[0.5, 3., 4., 1.]]);
        assert!((b.hessian - 12.5).abs() < 1e-4);
        assert!((b.third - 62.5).abs() < 1e-3);
        let w = wake(&[[0.3, 0.4, 0., 2.]]);
        assert!((w.hessian - 2.).abs() < 1e-5);
    }

    #[test]
    fn hermite_second_derivative_of_a_parabola_is_exact() {
        // η = r², η' = 2r : η'' = 2 partout, η''' = 0.
        let step = 0.5f32;
        let profile: Vec<_> = (0..8)
            .map(|i| {
                let r = i as f32 * step;
                (r * r, 2. * r)
            })
            .collect();
        let b = impact(&profile, step).unwrap();
        assert!((b.hessian - 2.).abs() < 1e-4, "{b:?}");
        assert!(b.third.abs() < 1e-3, "{b:?}");
        assert!(impact(&[(0., 1.), (1., 1.)], 1.).is_none());
    }

    #[test]
    fn isotropic_step_meets_the_tolerance() {
        let h = isotropic_step(0.05);
        // Cellule carrée h×h : R² = h²/2, erreur ≤ ½·M·R².
        assert!((0.5 * 0.05 * h * h / 2. - TOLERANCE_M).abs() < 1e-7);
    }
}
