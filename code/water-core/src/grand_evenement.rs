//! **Un très grand événement, du large à la plage** (S614, liste 11.3 ; ADR-001 §3.1).
//!
//! Macroscopique au large, local à l'interaction : le tsunami voyage comme un objet de W le long d'un rayon (S582, la levée de Green) ;
//! au bord d'un domaine local, sa hauteur pose une onde solitaire, et le domaine local — Saint-Venant 2D avec mouillage et séchage
//! (S613) — calcule la remontée sur la plage. La loi de Synolakis (1987) juge la remontée d'une onde non déferlante sur une pente plane :
//! `R/d = 2,831·√cot β·(H/d)^(5/4)`.
//!
//! Ne fait pas : le niveau macroscopique imposé au bord comme condition aux limites (ici, une onde solitaire de la hauteur donnée), le
//! déferlement, le 3D local, le crash et le très grand navire.

use crate::saint_venant_2d::SaintVenant2D;
use crate::tsunami::Rayon;

/// Une entrée refusée : hauteur, profondeur, pente non positives ; un point hors du rayon.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// **La hauteur au bord** du domaine local : la levée de Green du rayon en `s`.
pub fn hauteur_au_bord(rayon: &Rayon<'_>, a0: f64, s: f64) -> Result<f64, Refus> {
    rayon.amplitude(a0, s).map_err(|_| Refus)
}

/// **La remontée de Synolakis** (m) d'une onde solitaire de hauteur `h_onde` sur une pente `1/cot_beta` depuis une profondeur `d`.
pub fn remontee_synolakis(h_onde: f64, d: f64, cot_beta: f64) -> Result<f64, Refus> {
    if !(h_onde > 0.0) || !(d > 0.0) || !(cot_beta > 0.0) {
        return Err(Refus);
    }
    Ok(2.831 * d * cot_beta.sqrt() * (h_onde / d).powf(1.25))
}

/// **Une onde solitaire** de hauteur `h` sur la profondeur `d`, centrée en `x1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OndeSolitaire {
    pub h: f64,
    pub d: f64,
    pub x1: f64,
    pub g: f64,
}

impl OndeSolitaire {
    /// `γ = √(3H/4d³)`.
    pub fn gamma(&self) -> f64 {
        (3.0 * self.h / (4.0 * self.d.powi(3))).sqrt()
    }

    /// L'élévation `H·sech²(γ(x − x₁))`.
    pub fn eta(&self, x: f64) -> f64 {
        self.h / (self.gamma() * (x - self.x1)).cosh().powi(2)
    }

    /// La vitesse `c·η/(d + η)`, `c = √(g(d + H))`.
    pub fn u(&self, x: f64) -> f64 {
        let e = self.eta(x);
        (self.g * (self.d + self.h)).sqrt() * e / (self.d + e)
    }
}

/// **Une plage** : une bande de `ny` mailles de `dx`, fond plat à `−d` jusqu'au pied `x_pied`, puis la pente `1/cot_beta`, plafonnée à
/// `z_max` ; l'onde posée à la distance canonique `arccosh(√20)/γ` en avant du pied (Synolakis).
pub struct Plage {
    pub domaine: SaintVenant2D,
    pub fond_x: Vec<f64>,
}

impl Plage {
    #[allow(clippy::too_many_arguments)]
    pub fn nouvelle(h_onde: f64, d: f64, cot_beta: f64, x_pied: f64, dx: f64, nx: usize, ny: usize, z_max: f64, g: f64) -> Result<Plage, Refus> {
        remontee_synolakis(h_onde, d, cot_beta)?;
        let x = |i: usize| (i as f64 + 0.5) * dx;
        let fond_x: Vec<f64> = (0..nx).map(|i| (-d + (x(i) - x_pied).max(0.0) / cot_beta).min(z_max)).collect();
        let mut onde = OndeSolitaire { h: h_onde, d, x1: 0.0, g };
        onde.x1 = x_pied - 20f64.sqrt().acosh() / onde.gamma();
        let (mut z, mut h, mut qx) = (Vec::new(), Vec::new(), Vec::new());
        for (i, &zi) in fond_x.iter().enumerate() {
            let e = onde.eta(x(i));
            let hi = (e - zi).max(0.0);
            let ui = if zi < 0.0 { (g * (d + h_onde)).sqrt() * e / (d + e) } else { 0.0 };
            for _ in 0..ny {
                z.push(zi);
                h.push(hi);
                qx.push(hi * ui);
            }
        }
        let domaine = SaintVenant2D::nouveau(nx, ny, dx, g, z, h, qx, vec![0.0; nx * ny]).map_err(|_| Refus)?;
        Ok(Plage { domaine, fond_x })
    }

    /// La plus haute cote mouillée (`h > seuil`) sur la rangée médiane.
    pub fn cote_mouillee(&self, seuil: f64) -> Option<f64> {
        let ny = self.domaine.ny;
        (0..self.domaine.nx).filter(|&i| self.domaine.h[i * ny + ny / 2] > seuil).map(|i| self.fond_x[i]).reduce(f64::max)
    }
}

#[cfg(test)]
#[path = "tests_grand_evenement.rs"]
mod tests;
