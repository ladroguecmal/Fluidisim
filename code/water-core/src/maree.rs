//! **La marée harmonique** (S577, liste 2.2 ; sert 7.7 : SPEC-006 §5.4, « la marée, analytique, fiable à l'horizon publié »).
//!
//! `η(t) = Z₀ + Σ Aₖ·cos(ωₖ·t − gₖ)`, au plus [`MAX_COMPOSANTES`] composantes. Les périodes des composantes usuelles sont des faits
//! astronomiques ([`M2`], [`S2`]…) ; l'amplitude et la phase de chaque composante sont celles du lieu (une carte cotidale viendra avec les
//! régions décrites, 11.2) ; le niveau moyen `Z₀` est un paramètre.
//!
//! **Déterminisme (I-03).** Chaque phase est **entière** : `PhaseQ32::from_time` (la fréquence en tours par seconde, virgule fixe 2³²,
//! le temps en microsecondes, un produit sur 128 bits) — identique sur toute plateforme. La fréquence est convertie une fois ; son
//! arrondi (au pire 10⁻⁵ relatif, O1) fait dériver la phase de M2 de 1,2·10⁻³ tour par an (1,4 min) : déterministe, et sous la
//! précision d'une table de marée.

use crate::phase::{freq_hz_to_q32, PhaseQ32};
use crate::SimTime;

/// Le nombre de composantes d'une marée, au plus.
pub const MAX_COMPOSANTES: usize = 8;

/// Les périodes des composantes usuelles, heures solaires moyennes.
pub const M2: f64 = 12.420_601_2;
pub const S2: f64 = 12.0;
pub const N2: f64 = 12.658_347_51;
pub const K2: f64 = 11.967_236_06;
pub const K1: f64 = 23.934_472_13;
pub const O1: f64 = 25.819_338_71;
pub const P1: f64 = 24.065_887_66;
pub const Q1: f64 = 26.868_350;

/// Une composante : période (h), amplitude (m), phase de Greenwich du lieu (tours, 0 à 1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Composante {
    pub periode_h: f64,
    pub amplitude_m: f32,
    pub phase_tours: f64,
}

/// Une entrée refusée : plus de [`MAX_COMPOSANTES`], une période non positive, une valeur non finie.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// **La marée d'un lieu**, prête à l'exécution : fréquences et phases converties une fois.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Maree {
    frequences_q32: [u64; MAX_COMPOSANTES],
    phases: [PhaseQ32; MAX_COMPOSANTES],
    amplitudes_m: [f32; MAX_COMPOSANTES],
    n: usize,
    niveau_moyen_m: f32,
}

impl Maree {
    /// Construit la marée d'un lieu. Réservé à l'initialisation : les conversions flottantes ont lieu ici, jamais à l'exécution.
    pub fn new(composantes: &[Composante], niveau_moyen_m: f32) -> Result<Self, Refus> {
        if composantes.len() > MAX_COMPOSANTES || !niveau_moyen_m.is_finite() {
            return Err(Refus);
        }
        let mut m = Maree { frequences_q32: [0; MAX_COMPOSANTES], phases: [PhaseQ32(0); MAX_COMPOSANTES], amplitudes_m: [0.0; MAX_COMPOSANTES],
            n: composantes.len(), niveau_moyen_m };
        for (k, c) in composantes.iter().enumerate() {
            if !(c.periode_h > 0.0) || !c.periode_h.is_finite() || !c.amplitude_m.is_finite() || !c.phase_tours.is_finite() {
                return Err(Refus);
            }
            m.frequences_q32[k] = freq_hz_to_q32(1.0 / (c.periode_h * 3600.0));
            let frac = c.phase_tours - c.phase_tours.floor();
            m.phases[k] = PhaseQ32((frac * 4_294_967_296.0) as u32);
            m.amplitudes_m[k] = c.amplitude_m;
        }
        Ok(m)
    }

    /// **Le niveau de la mer** à l'instant `t`, m — entièrement déterministe.
    pub fn niveau(&self, t: SimTime) -> f32 {
        let mut eta = self.niveau_moyen_m;
        for k in 0..self.n {
            let phase = PhaseQ32::from_time(self.frequences_q32[k], t);
            eta += self.amplitudes_m[k] * PhaseQ32(phase.0.wrapping_sub(self.phases[k].0)).cos();
        }
        eta
    }

    /// **S579 — la vitesse du niveau** à l'instant `t`, m/s : `−Σ Aₖ·ωₖ·sin(ωₖt − gₖ)`, `ωₖ` la pulsation de la fréquence **arrondie**
    /// (celle des phases).
    pub fn vitesse(&self, t: SimTime) -> f32 {
        let mut v = 0.0f32;
        for k in 0..self.n {
            let phase = PhaseQ32::from_time(self.frequences_q32[k], t);
            v -= self.amplitudes_m[k] * pulsation(self.frequences_q32[k]) * PhaseQ32(phase.0.wrapping_sub(self.phases[k].0)).sin();
        }
        v
    }
}

/// La pulsation d'une fréquence Q32, rad/s — l'écriture de `Background::eval_local`.
fn pulsation(freq_q32: u64) -> f32 {
    (freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU) as f32
}

/// **S579 — la marée dans l'échantillon de B** : `η += niveau`, `∂η/∂t += vitesse`, et la vitesse verticale de surface `w += vitesse` (la
/// condition cinématique) ; le reste inchangé. Le courant horizontal de marée (il demande la profondeur) et la pente de la marée
/// (`k·A` ≈ 10⁻⁵) n'y sont pas.
pub fn avec_maree(mut s: crate::WaterSample, niveau: f32, vitesse: f32) -> crate::WaterSample {
    s.eta += niveau;
    s.deta_dt += vitesse;
    s.u_total[2] += vitesse;
    s
}

// --- S578 — la carte cotidale.

/// **S578 — la carte cotidale d'une région** : pour chaque composante, l'amplitude complexe `H = A·e^(−ig)` aux nœuds d'une grille
/// régulière — `(Re H, Im H)`, rangées composante par composante, `y` puis `x`. Interpolée **sous forme complexe** (bilinéaire sur `Re H`
/// et `Im H`) : `η = Z₀ + Σ (Re Hₖ·cos ωₖt − Im Hₖ·sin ωₖt)` = `Z₀ + Σ Aₖ·cos(ωₖt − gₖ)`. Que des opérations IEEE de base et nos polynômes
/// de `PhaseQ32` : déterministe, sans `atan2`, sans saut de phase à 2π. Au milieu d'une maille, la corde creuse l'amplitude de
/// `1 − cos(k·Δx/2)` ; la phase y reste exacte.
pub struct CarteCotidale<'a> {
    frequences_q32: [u64; MAX_COMPOSANTES],
    n: usize,
    origine: [f64; 2],
    pas_m: f64,
    nx: usize,
    ny: usize,
    h: &'a [[f32; 2]],
    niveau_moyen_m: f32,
}

impl<'a> CarteCotidale<'a> {
    /// `periodes_h` : une période par composante ; `h` : `composantes × ny × nx` amplitudes complexes, m. Refus : plus de
    /// [`MAX_COMPOSANTES`], une période non positive, une grille de moins de 2 × 2 nœuds, un pas non positif, une longueur fausse, une
    /// valeur non finie.
    pub fn new(periodes_h: &[f64], origine: [f64; 2], pas_m: f64, nx: usize, ny: usize, h: &'a [[f32; 2]], niveau_moyen_m: f32)
        -> Result<Self, Refus> {
        if periodes_h.is_empty() || periodes_h.len() > MAX_COMPOSANTES || nx < 2 || ny < 2 || !(pas_m > 0.0) || !pas_m.is_finite()
            || h.len() != periodes_h.len() * nx * ny || h.iter().flatten().any(|v| !v.is_finite()) || !niveau_moyen_m.is_finite()
            || !origine.iter().all(|v| v.is_finite()) {
            return Err(Refus);
        }
        let mut frequences_q32 = [0u64; MAX_COMPOSANTES];
        for (f, p) in frequences_q32.iter_mut().zip(periodes_h) {
            if !(*p > 0.0) || !p.is_finite() {
                return Err(Refus);
            }
            *f = freq_hz_to_q32(1.0 / (p * 3600.0));
        }
        Ok(CarteCotidale { frequences_q32, n: periodes_h.len(), origine, pas_m, nx, ny, h, niveau_moyen_m })
    }

    /// **Le niveau de la mer** au point `(x, y)` (m, repère de la carte) à l'instant `t`. Refus hors de la grille.
    pub fn niveau(&self, x: f64, y: f64, t: SimTime) -> Result<f32, Refus> {
        let (u, v) = ((x - self.origine[0]) / self.pas_m, (y - self.origine[1]) / self.pas_m);
        if !(u >= 0.0 && v >= 0.0 && u <= (self.nx - 1) as f64 && v <= (self.ny - 1) as f64) {
            return Err(Refus);
        }
        let (i, j) = ((u as usize).min(self.nx - 2), (v as usize).min(self.ny - 2));
        let (fx, fy) = ((u - i as f64) as f32, (v - j as f64) as f32);
        let mut eta = self.niveau_moyen_m;
        for k in 0..self.n {
            let base = k * self.nx * self.ny;
            let at = |ii: usize, jj: usize| self.h[base + jj * self.nx + ii];
            let (a, b, c, d) = (at(i, j), at(i + 1, j), at(i, j + 1), at(i + 1, j + 1));
            let lerp = |p: f32, q: f32, f: f32| p + (q - p) * f;
            let re = lerp(lerp(a[0], b[0], fx), lerp(c[0], d[0], fx), fy);
            let im = lerp(lerp(a[1], b[1], fx), lerp(c[1], d[1], fx), fy);
            let (s, co) = PhaseQ32::from_time(self.frequences_q32[k], t).sin_cos();
            eta += re * co - im * s;
        }
        Ok(eta)
    }

    /// **S579 — le niveau et sa vitesse** au point `(x, y)` à l'instant `t` : `∂η/∂t = −Σ ωₖ·(Re Hₖ·sin ωₖt + Im Hₖ·cos ωₖt)`.
    pub fn niveau_et_vitesse(&self, x: f64, y: f64, t: SimTime) -> Result<(f32, f32), Refus> {
        let eta = self.niveau(x, y, t)?;
        let (u, v) = ((x - self.origine[0]) / self.pas_m, (y - self.origine[1]) / self.pas_m);
        let (i, j) = ((u as usize).min(self.nx - 2), (v as usize).min(self.ny - 2));
        let (fx, fy) = ((u - i as f64) as f32, (v - j as f64) as f32);
        let mut vit = 0.0f32;
        for k in 0..self.n {
            let base = k * self.nx * self.ny;
            let at = |ii: usize, jj: usize| self.h[base + jj * self.nx + ii];
            let (a, b, c, d) = (at(i, j), at(i + 1, j), at(i, j + 1), at(i + 1, j + 1));
            let lerp = |p: f32, q: f32, f: f32| p + (q - p) * f;
            let re = lerp(lerp(a[0], b[0], fx), lerp(c[0], d[0], fx), fy);
            let im = lerp(lerp(a[1], b[1], fx), lerp(c[1], d[1], fx), fy);
            let (s, co) = PhaseQ32::from_time(self.frequences_q32[k], t).sin_cos();
            vit -= pulsation(self.frequences_q32[k]) * (re * s + im * co);
        }
        Ok((eta, vit))
    }

    /// **S580 — le courant de marée** au point `(x, y)` à l'instant `t`, m/s, sous la gravité `g` (m/s²) : la quantité de mouvement
    /// linéaire sans frottement ni Coriolis, `∂u/∂t = −g·∇η` — pour chaque composante `u = −(g/ω)·(∇Re H·sin ωt + ∇Im H·cos ωt)`, le
    /// gradient étant celui de l'interpolation bilinéaire (la pente de la corde : au milieu d'une maille, l'amplitude × `sinc(kΔx/2)`,
    /// la phase exacte). Sans la profondeur : la carte porte déjà la propagation. Refus hors de la grille ou sous une gravité non positive.
    pub fn courant(&self, x: f64, y: f64, t: SimTime, g: f32) -> Result<[f32; 2], Refus> {
        if !(g > 0.0) || !g.is_finite() {
            return Err(Refus);
        }
        self.niveau(x, y, t)?;
        let (u, v) = ((x - self.origine[0]) / self.pas_m, (y - self.origine[1]) / self.pas_m);
        let (i, j) = ((u as usize).min(self.nx - 2), (v as usize).min(self.ny - 2));
        let (fx, fy) = ((u - i as f64) as f32, (v - j as f64) as f32);
        let pas = self.pas_m as f32;
        let mut c = [0.0f32; 2];
        for k in 0..self.n {
            let base = k * self.nx * self.ny;
            let at = |ii: usize, jj: usize| self.h[base + jj * self.nx + ii];
            let (a, b, cc, d) = (at(i, j), at(i + 1, j), at(i, j + 1), at(i + 1, j + 1));
            let (s, co) = PhaseQ32::from_time(self.frequences_q32[k], t).sin_cos();
            let facteur = g / pulsation(self.frequences_q32[k]);
            for (axe, cv) in c.iter_mut().enumerate() {
                let grad = |n: usize| if axe == 0 {
                    ((b[n] - a[n]) * (1.0 - fy) + (d[n] - cc[n]) * fy) / pas
                } else {
                    ((cc[n] - a[n]) * (1.0 - fx) + (d[n] - b[n]) * fx) / pas
                };
                *cv -= facteur * (grad(0) * s + grad(1) * co);
            }
        }
        Ok(c)
    }

    /// **S634 — le courant de marée avec frottement et Coriolis**, m/s : pour chaque composante, la solution harmonique établie de
    /// `∂u/∂t + r·u − f·v = −g·∂η/∂x`, `∂v/∂t + r·v + f·u = −g·∂η/∂y` (`f` le paramètre de Coriolis, `r` le frottement linéarisé, s⁻¹) —
    /// avec `a = iω + r` et `G` le gradient complexe de l'interpolation (celui de [`Self::courant`]) : `U = −g·(a·Gx + f·Gy)/(a² + f²)`,
    /// `V = −g·(a·Gy − f·Gx)/(a² + f²)`, `u = Re U·cos ωt − Im U·sin ωt`. `f` = `r` = 0 redonne `courant`. Refus : hors de la grille, `g`
    /// non positif, `r` négatif, une valeur non finie.
    pub fn courant_amorti(&self, x: f64, y: f64, t: SimTime, g: f32, f: f32, r: f32) -> Result<[f32; 2], Refus> {
        if !(g > 0.0) || !g.is_finite() || !(r >= 0.0) || !r.is_finite() || !f.is_finite() {
            return Err(Refus);
        }
        self.niveau(x, y, t)?;
        let (u, v) = ((x - self.origine[0]) / self.pas_m, (y - self.origine[1]) / self.pas_m);
        let (i, j) = ((u as usize).min(self.nx - 2), (v as usize).min(self.ny - 2));
        let (fx, fy) = ((u - i as f64) as f32, (v - j as f64) as f32);
        let pas = self.pas_m as f32;
        let mul = |p: [f32; 2], q: [f32; 2]| [p[0] * q[0] - p[1] * q[1], p[0] * q[1] + p[1] * q[0]];
        let mut c = [0.0f32; 2];
        for k in 0..self.n {
            let base = k * self.nx * self.ny;
            let at = |ii: usize, jj: usize| self.h[base + jj * self.nx + ii];
            let (pa, pb, pc, pd) = (at(i, j), at(i + 1, j), at(i, j + 1), at(i + 1, j + 1));
            let gx = |n: usize| ((pb[n] - pa[n]) * (1.0 - fy) + (pd[n] - pc[n]) * fy) / pas;
            let gy = |n: usize| ((pc[n] - pa[n]) * (1.0 - fx) + (pd[n] - pb[n]) * fx) / pas;
            let (gxc, gyc) = ([gx(0), gx(1)], [gy(0), gy(1)]);
            let a = [r, pulsation(self.frequences_q32[k])];
            let aa = mul(a, a);
            let d = [aa[0] + f * f, aa[1]];
            let (nu, nv) = (mul(a, gxc), mul(a, gyc));
            let (nu, nv) = ([nu[0] + f * gyc[0], nu[1] + f * gyc[1]], [nv[0] - f * gxc[0], nv[1] - f * gxc[1]]);
            let dd = d[0] * d[0] + d[1] * d[1];
            let div = |n: [f32; 2]| [-g * (n[0] * d[0] + n[1] * d[1]) / dd, -g * (n[1] * d[0] - n[0] * d[1]) / dd];
            let (uc, vc) = (div(nu), div(nv));
            let (s, co) = PhaseQ32::from_time(self.frequences_q32[k], t).sin_cos();
            c[0] += uc[0] * co - uc[1] * s;
            c[1] += vc[0] * co - vc[1] * s;
        }
        Ok(c)
    }
}

/// **S580 — le courant de marée dans l'échantillon de B** : ajouté à la vitesse horizontale `u_total[0..2]` ; le reste inchangé.
pub fn avec_courant(mut s: crate::WaterSample, courant: [f32; 2]) -> crate::WaterSample {
    s.u_total[0] += courant[0];
    s.u_total[1] += courant[1];
    s
}

#[cfg(test)]
#[path = "tests_maree.rs"]
mod tests;
