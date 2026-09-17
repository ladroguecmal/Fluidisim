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

/// Nœuds de la grille du sillage réservés une fois (I-06). Choix de mémoire, pas de physique :
/// il autorise un pas d'environ 0,9 m sur l'emprise S212, et un dépassement est annoncé.
pub const LATTICE_CAPACITY: usize = 16_384;

/// Grille locale du sillage pour un instant : pas, dimensions et erreur garantie.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lattice {
    /// Pas retenu (m), multiple de 1/16 m pour que la grille ne bouge pas à chaque image.
    pub step: f32,
    pub nx: u32,
    pub ny: u32,
    /// Pas que la borne autorisait avant arrondi et capacité.
    pub bound_step: f32,
    /// Majorant de l'erreur de reconstruction au pas retenu (m).
    pub error_bound: f32,
    /// Vrai si la capacité a imposé un pas plus grand que la borne : l'erreur publiée dépasse
    /// alors la tolérance, et l'hôte l'annonce.
    pub clamped: bool,
}

impl Lattice {
    /// Grille couvrant `[min, max]` au pas de la borne bicubique, dans `capacity` nœuds.
    pub fn plan(m4: f32, m5: f32, min: [f32; 2], max: [f32; 2], capacity: usize) -> Self {
        let bound_step = bicubic_step(m4, m5);
        let extent = [max[0] - min[0], max[1] - min[1]];
        let dims = |s: f32| {
            let n = |i: usize| ((extent[i] / s).ceil() as u32).max(1) + 1;
            (n(0), n(1))
        };
        // Un champ nul n'impose aucun pas : la plus grosse maille qui couvre l'emprise suffit.
        let mut step = if bound_step.is_finite() {
            ((bound_step * 16.).floor() / 16.).max(1. / 16.)
        } else {
            extent[0].max(extent[1])
        };
        let mut clamped = false;
        while {
            let (nx, ny) = dims(step);
            nx as usize * ny as usize > capacity
        } {
            step += 1. / 16.;
            clamped = true;
        }
        let (nx, ny) = dims(step);
        Self {
            step,
            nx,
            ny,
            bound_step,
            error_bound: bicubic_error(step, m4, m5),
            clamped,
        }
    }
    pub fn nodes(&self) -> usize {
        self.nx as usize * self.ny as usize
    }
}

/// Reconstruction Hermite bicubique d'une maille : coins `(η, ηx, ηy, ηxy)` dans l'ordre
/// `[(0,0), (1,0), (0,1), (1,1)]`, coordonnées locales `t ∈ [0,1]²`. Rend `(η, ηx, ηy)`.
/// **Mêmes opérations que `wake_lattice` de `water.wgsl`** ; sert de référence CPU aux tests.
#[cfg_attr(not(test), allow(dead_code))]
pub fn hermite(corners: &[[f32; 4]; 4], step: f32, t: [f32; 2]) -> [f32; 3] {
    let basis = |t: f32| {
        let (t2, t3) = (t * t, t * t * t);
        (
            [2. * t3 - 3. * t2 + 1., -2. * t3 + 3. * t2],
            [t3 - 2. * t2 + t, t3 - t2],
            [6. * t2 - 6. * t, -6. * t2 + 6. * t],
            [3. * t2 - 4. * t + 1., 3. * t2 - 2. * t],
        )
    };
    let (vx, dx, vx1, dx1) = basis(t[0]);
    let (vy, dy, vy1, dy1) = basis(t[1]);
    let mut out = [0f32; 3];
    for j in 0..2 {
        for i in 0..2 {
            let c = corners[j * 2 + i];
            let (f, fx, fy, fxy) = (c[0], c[1] * step, c[2] * step, c[3] * step * step);
            out[0] += vx[i] * vy[j] * f + dx[i] * vy[j] * fx + vx[i] * dy[j] * fy + dx[i] * dy[j] * fxy;
            out[1] += vx1[i] * vy[j] * f + dx1[i] * vy[j] * fx + vx1[i] * dy[j] * fy + dx1[i] * dy[j] * fxy;
            out[2] += vx[i] * vy1[j] * f + dx[i] * vy1[j] * fx + vx[i] * dy1[j] * fy + dx[i] * dy1[j] * fxy;
        }
    }
    out[1] /= step;
    out[2] /= step;
    out
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
    /// S262 : distance lointaine de la grille (m) — 1 500 sous la brume, horizon géométrique sous le ciel clair.
    pub far: f32,
}

impl Projection {
    pub fn grid_point(&self, ix: f32, iy: f32, nx: u32, ny: u32) -> [f32; 2] {
        self.ground((ix / (nx - 1) as f32 * 2. - 1.) * 1.18,
            -1.18 + (self.horizon() + 1.18) * iy / (ny - 1) as f32)
    }
    /// Plus longue arête locale, diagonales comprises (ADR-148).
    pub fn spacing(&self, ix: f32, iy: f32, nx: u32, ny: u32) -> f32 {
        let q = self.grid_point(ix, iy, nx, ny);
        let mut h = 0f32;
        for y in -1..=1 {
            for x in -1..=1 {
                let r = self.grid_point((ix + x as f32).clamp(0., (nx - 1) as f32),
                    (iy + y as f32).clamp(0., (ny - 1) as f32), nx, ny);
                h = h.max((q[0] - r[0]).hypot(q[1] - r[1]));
            }
        }
        h
    }
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
        let distance = (self.eye[2] / (-ray[2]).max(0.00001)).min(self.far);
        [ray[0] * distance, ray[1] * distance]
    }
}

/// S235 — **visibilité** : contour, sur le plan d'eau, de l'emprise de la grille projetée.
///
/// Les sommets ne se déplacent qu'en hauteur : une source ne contribue à l'image que si un sommet
/// tombe dans son domaine horizontal. L'emprise est l'image continue et injective du rectangle
/// écran ; son bord est l'image du bord du rectangle, échantillonné ici **à chaque sommet de bord**
/// avec les opérations de `ocean_vertex`. Rend le contour et sa plus longue arête (publiée).
pub fn footprint(p: &Projection, nx: u32, ny: u32) -> (Vec<[f32; 2]>, f32) {
    let mut poly = Vec::new();
    let chord = footprint_into(p, nx, ny, &mut poly);
    (poly, chord)
}

/// S240, I-06 — la même emprise dans un tampon **fourni par l'appelant**. Il garde sa capacité
/// d'une image à l'autre : le premier contour d'un format l'agrandit, les suivants n'allouent
/// plus rien. Rend la plus longue corde entre deux échantillons de bord.
pub fn footprint_into(p: &Projection, nx: u32, ny: u32, poly: &mut Vec<[f32; 2]>) -> f32 {
    let horizon = p.horizon();
    let at = |ix: u32, iy: u32| {
        let x = (ix as f32 / (nx - 1) as f32 * 2. - 1.) * 1.18;
        let y = -1.18 + (horizon + 1.18) * (iy as f32 / (ny - 1) as f32);
        p.ground(x, y)
    };
    poly.clear();
    poly.reserve(2 * (nx + ny) as usize);
    poly.extend((0..nx).map(|i| at(i, 0)));
    poly.extend((1..ny).map(|j| at(nx - 1, j)));
    poly.extend((0..nx - 1).rev().map(|i| at(i, ny - 1)));
    poly.extend((1..ny - 1).rev().map(|j| at(0, j)));
    let mut chord = 0f32;
    for k in 0..poly.len() {
        let (a, b) = (poly[k], poly[(k + 1) % poly.len()]);
        chord = chord.max((b[0] - a[0]).hypot(b[1] - a[1]));
    }
    chord
}

fn inside_polygon(poly: &[[f32; 2]], q: [f32; 2]) -> bool {
    let mut inside = false;
    for k in 0..poly.len() {
        let (a, b) = (poly[k], poly[(k + 1) % poly.len()]);
        if (a[1] > q[1]) != (b[1] > q[1]) && q[0] < a[0] + (q[1] - a[1]) / (b[1] - a[1]) * (b[0] - a[0]) {
            inside = !inside;
        }
    }
    inside
}

fn segment_hits_rect(a: [f32; 2], b: [f32; 2], min: [f32; 2], max: [f32; 2]) -> bool {
    // Liang–Barsky : intervalle du paramètre dans chaque bande.
    let (mut t0, mut t1) = (0f32, 1f32);
    for i in 0..2 {
        let d = b[i] - a[i];
        if d == 0. {
            if a[i] < min[i] || a[i] > max[i] {
                return false;
            }
            continue;
        }
        let (mut u, mut v) = ((min[i] - a[i]) / d, (max[i] - a[i]) / d);
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        t0 = t0.max(u);
        t1 = t1.min(v);
        if t0 > t1 {
            return false;
        }
    }
    true
}

/// Le rectangle `[min, max]` touche-t-il l'emprise ?
///
/// **Marge par arête.** Sans la borne de distance, rangées et colonnes de la grille se projettent
/// en segments droits (projection centrale d'une droite) : les arêtes du contour sont exactes.
/// La borne de 1500 m brise ce seul cas en un coude entre deux échantillons ; chaque arête est
/// donc élargie de sa propre longueur, qui majore l'écart du coude à la corde. Les arêtes
/// proches font quelques centimètres, celles de l'horizon quelques centaines de mètres : le test
/// reste conservateur partout sans gonfler le premier plan.
pub fn touches_rect(poly: &[[f32; 2]], min: [f32; 2], max: [f32; 2]) -> bool {
    let corner_inside = [[min[0], min[1]], [max[0], min[1]], [min[0], max[1]], [max[0], max[1]]]
        .iter()
        .any(|c| inside_polygon(poly, *c));
    corner_inside
        || (0..poly.len()).any(|k| {
            let (a, b) = (poly[k], poly[(k + 1) % poly.len()]);
            let m = (b[0] - a[0]).hypot(b[1] - a[1]);
            segment_hits_rect(a, b, [min[0] - m, min[1] - m], [max[0] + m, max[1] + m])
        })
}

/// Le disque de centre `c` et de rayon `radius` touche-t-il l'emprise ? Même marge par arête.
pub fn touches_disc(poly: &[[f32; 2]], c: [f32; 2], radius: f32) -> bool {
    inside_polygon(poly, c)
        || (0..poly.len()).any(|k| {
            let (a, b) = (poly[k], poly[(k + 1) % poly.len()]);
            let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
            let len2 = dx * dx + dy * dy;
            let t = if len2 > 0. { (((c[0] - a[0]) * dx + (c[1] - a[1]) * dy) / len2).clamp(0., 1.) } else { 0. };
            (a[0] + t * dx - c[0]).hypot(a[1] + t * dy - c[1]) <= radius + len2.sqrt()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polygon_tests_cover_inside_outside_and_crossing() {
        // Contour finement échantillonné (arêtes de 1) : la marge par arête reste d'une unité.
        let mut square = Vec::new();
        for i in 0..10 {
            square.push([i as f32, 0.]);
        }
        for i in 0..10 {
            square.push([10., i as f32]);
        }
        for i in (1..=10).rev() {
            square.push([i as f32, 10.]);
        }
        for i in (1..=10).rev() {
            square.push([0., i as f32]);
        }
        assert!(touches_rect(&square, [2., 2.], [3., 3.])); // rectangle intérieur
        assert!(touches_rect(&square, [-5., -5.], [20., 20.])); // emprise intérieure
        assert!(touches_rect(&square, [9.5, -3.], [9.6, 20.])); // traversée sans sommet dedans
        assert!(!touches_rect(&square, [12., 12.], [13., 13.]));
        assert!(touches_rect(&square, [10.5, 10.5], [13., 13.])); // dans la marge d'une arête
        assert!(touches_disc(&square, [5., 5.], 1.));
        assert!(touches_disc(&square, [5., -2.], 1.5));
        assert!(touches_disc(&square, [5., 5.], 100.));
        assert!(!touches_disc(&square, [15., 15.], 5.));
    }

    /// S240, I-06 — le tampon gardé rend **exactement** le même contour, format après format, et
    /// il n'alloue plus une fois sa capacité prise. Un format plus petit ne laisse pas de queue.
    #[test]
    fn a_kept_footprint_buffer_gives_the_same_contour_s240() {
        let pitch = -(7.0f32 / 53.).atan();
        let (sp, cp) = pitch.sin_cos();
        let p = Projection {
            eye: [0., -18., 7.],
            forward: [0., cp, sp],
            right: [1., 0., 0.],
            up: [0., -sp, cp],
            tan_half: 25f32.to_radians().tan(),
            aspect: 16. / 9.,
            far: 1500.,
        };
        let mut kept = Vec::new();
        for (nx, ny) in [(481u32, 271u32), (321, 181), (481, 271)] {
            let (fresh, chord) = footprint(&p, nx, ny);
            let kept_chord = footprint_into(&p, nx, ny, &mut kept);
            assert_eq!(kept.len(), fresh.len(), "{nx}x{ny}");
            assert_eq!(kept_chord.to_bits(), chord.to_bits(), "{nx}x{ny}");
            for (a, b) in kept.iter().zip(&fresh) {
                assert_eq!(a[0].to_bits(), b[0].to_bits());
                assert_eq!(a[1].to_bits(), b[1].to_bits());
            }
        }
        // La capacité prise au plus grand format sert au plus petit sans nouvelle demande.
        let before = kept.capacity();
        footprint_into(&p, 321, 181, &mut kept);
        assert_eq!(kept.capacity(), before);
    }

    #[test]
    fn footprint_of_the_s201_camera_sees_ahead_not_behind() {
        let (sy, cy) = 0f32.sin_cos();
        let pitch = -(7.0f32 / 53.).atan();
        let (sp, cp) = pitch.sin_cos();
        let p = Projection {
            eye: [0., -18., 7.],
            forward: [sy * cp, cy * cp, sp],
            right: [cy, -sy, 0.],
            up: [-sy * sp, -cy * sp, cp],
            tan_half: 25f32.to_radians().tan(),
            aspect: 16. / 9.,
            far: 1500.,
        };
        let (poly, longest) = footprint(&p, 481, 271);
        assert!(longest > 1. && poly.len() == 2 * (481 + 271) - 4);
        // Relatif à la caméra : l'impact S203 (0, 10) est à (0, 28) devant ; derrière, (0, −300).
        assert!(touches_disc(&poly, [0., 28.], 0.5));
        assert!(!touches_disc(&poly, [0., -300.], 52.));
        assert!(touches_rect(&poly, [-64., -30.], [64., 74.]));
        // Tout sommet de la grille est dans l'emprise : le contour contient l'intérieur.
        let horizon = p.horizon();
        for iy in (0..271).step_by(27) {
            for ix in (0..481).step_by(48) {
                let x = (ix as f32 / 480. * 2. - 1.) * 1.18;
                let y = -1.18 + (horizon + 1.18) * (iy as f32 / 270.);
                assert!(touches_disc(&poly, p.ground(x, y), 0.), "sommet {ix},{iy}");
            }
        }
    }

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

    /// Suite pseudo-aléatoire déterministe, sans dépendance.
    fn lcg(state: &mut u64) -> f32 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (*state >> 40) as f32 / (1u64 << 24) as f32
    }

    #[test]
    fn hermite_is_exact_on_bicubic_polynomials() {
        // f = x³y² − 2xy³ + y, cubique en chaque variable : la reconstruction est exacte.
        let f = |x: f32, y: f32| {
            [
                x * x * x * y * y - 2. * x * y * y * y + y,
                3. * x * x * y * y - 2. * y * y * y,
                2. * x * x * x * y - 6. * x * y * y + 1.,
                6. * x * x * y - 6. * y * y,
            ]
        };
        let (h, x0, y0) = (0.75f32, -0.5f32, 0.25f32);
        let corners = [f(x0, y0), f(x0 + h, y0), f(x0, y0 + h), f(x0 + h, y0 + h)];
        for t in [[0.5, 0.5], [0.1, 0.9], [1., 0.], [0.3, 0.7]] {
            let got = hermite(&corners, h, t);
            let want = f(x0 + t[0] * h, y0 + t[1] * h);
            for k in 0..3 {
                assert!((got[k] - want[k]).abs() < 2e-5, "t={t:?} k={k} {got:?} {want:?}");
            }
        }
    }

    #[test]
    fn hermite_error_on_plane_waves_stays_under_its_bound() {
        let mut s = 7u64;
        // Soixante-quatre ondes planes d'amplitude et de direction quelconques, |k| ≤ 3 rad/m :
        // la bande de la recette du sillage (cutoff 3).
        let modes: Vec<[f32; 4]> = (0..64)
            .map(|_| {
                let k = 3. * lcg(&mut s);
                let theta = 6.283185 * lcg(&mut s);
                [0.02 * lcg(&mut s) - 0.01, 0.02 * lcg(&mut s) - 0.01, k * theta.cos(), k * theta.sin()]
            })
            .collect();
        let field = |x: f64, y: f64| {
            let mut v = [0f64; 4];
            for m in &modes {
                let (a, b, kx, ky) = (m[0] as f64, m[1] as f64, m[2] as f64, m[3] as f64);
                let phase = kx * x + ky * y;
                let (sn, co) = phase.sin_cos();
                v[0] += a * co - b * sn;
                v[1] += -(a * sn + b * co) * kx;
                v[2] += -(a * sn + b * co) * ky;
                v[3] += -(a * co - b * sn) * kx * ky;
            }
            v
        };
        let (m4, m5) = wake_smooth(&modes);
        for h in [0.25f32, bicubic_step(m4, m5), 2.] {
            let bound = bicubic_error(h, m4, m5);
            let mut worst = 0f64;
            for cell in 0..40 {
                let (x0, y0) = (7.3 * cell as f64, -3.1 * cell as f64);
                let hd = h as f64;
                let c = |i: f64, j: f64| field(x0 + i * hd, y0 + j * hd).map(|v| v as f32);
                let corners = [c(0., 0.), c(1., 0.), c(0., 1.), c(1., 1.)];
                for p in 0..25 {
                    let t = [(p % 5) as f32 / 4., (p / 5) as f32 / 4.];
                    let got = hermite(&corners, h, t)[0] as f64;
                    let want = field(x0 + t[0] as f64 * hd, y0 + t[1] as f64 * hd)[0];
                    worst = worst.max((got - want).abs());
                }
            }
            assert!(worst <= bound as f64 + 1e-6, "h={h} pire={worst} borne={bound}");
        }
        assert!(bicubic_error(bicubic_step(m4, m5), m4, m5) <= TOLERANCE_M * 1.0001);
    }

    #[test]
    fn lattice_covers_the_footprint_on_a_sixteenth_grid() {
        let (min, max) = ([-64., -48.], [64., 56.]);
        let l = Lattice::plan(0.21296, 0.26535, min, max, LATTICE_CAPACITY);
        assert!(!l.clamped);
        assert_eq!((l.step * 16.).fract(), 0.);
        assert!(l.step <= l.bound_step && l.error_bound <= TOLERANCE_M);
        assert!((l.nx - 1) as f32 * l.step >= 128. && (l.ny - 1) as f32 * l.step >= 104.);
        // Capacité insuffisante : pas élargi, erreur publiée au-delà de la tolérance.
        let small = Lattice::plan(0.21296, 0.26535, min, max, 1_000);
        assert!(small.clamped && small.nodes() <= 1_000 && small.error_bound > TOLERANCE_M);
        // Champ nul : une seule maille.
        let empty = Lattice::plan(0., 0., min, max, LATTICE_CAPACITY);
        assert_eq!((empty.nx, empty.ny, empty.error_bound), (2, 2, 0.));
    }

    #[test]
    fn isotropic_step_meets_the_tolerance() {
        let h = isotropic_step(0.05);
        // Cellule carrée h×h : R² = h²/2, erreur ≤ ½·M·R².
        assert!((0.5 * 0.05 * h * h / 2. - TOLERANCE_M).abs() < 1e-7);
    }
}
