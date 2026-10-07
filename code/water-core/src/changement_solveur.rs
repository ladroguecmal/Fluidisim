//! **Le changement de solveur par W** (S612, liste 4.20 ; ADR-007 §3).
//!
//! Pas de transfert d'état entre solveurs : `transduction δ → W → destruction → création à δ = 0 → nouveau solveur`. L'énergie part dans
//! W, B + W est inchangé, et deux solveurs voisins ne communiquent que par W — aucun code de conversion n'est écrit.
//!
//! En eau peu profonde linéaire 1D, la transduction est exacte : l'état `(η, u)` se décompose en deux invariants de Riemann aux centres,
//! `R = (η + √(h/g)·ū)/2` qui voyage à `+c` et `L = (η − √(h/g)·ū)/2` à `−c` (`ū` la moyenne des deux faces) ; W les porte analytiquement
//! (d'Alembert), interpolés linéairement entre les centres, nuls au-delà.
//!
//! Ne fait pas : la transduction 2D/3D (S312–S316, sur la référence), un solveur d'une autre famille, W non linéaire, la décision de
//! changer (l'ordonnanceur).

use crate::substitutif::Domaine1D;

/// **Deux trains d'ondes longues** issus d'un domaine à l'instant `t0` : les profils `droite` (`R`) et `gauche` (`L`) aux centres
/// `x0 + i·dx`.
#[derive(Clone, Debug, PartialEq)]
pub struct TrainW1D {
    pub x0: f64,
    pub dx: f64,
    pub c: f64,
    pub r: f64,
    pub t0: f64,
    pub droite: Vec<f64>,
    pub gauche: Vec<f64>,
}

/// L'interpolation linéaire d'un profil aux centres `x0 + i·dx`, nulle hors de `[x0, x0 + (n − 1)·dx]`.
fn profil(v: &[f64], x0: f64, dx: f64, x: f64) -> f64 {
    let n = v.len();
    let s = (x - x0) / dx;
    if n == 0 || s < 0.0 || s > (n - 1) as f64 {
        return 0.0;
    }
    let j = (s.floor() as usize).min(n.saturating_sub(2));
    if n == 1 {
        return v[0];
    }
    v[j] + (x - (x0 + j as f64 * dx)) * (v[j + 1] - v[j]) / dx
}

impl TrainW1D {
    /// L'élévation de W en `(x, t)`.
    pub fn eta(&self, x: f64, t: f64) -> f64 {
        let d = self.c * (t - self.t0);
        profil(&self.droite, self.x0, self.dx, x - d) + profil(&self.gauche, self.x0, self.dx, x + d)
    }

    /// La vitesse de W en `(x, t)` : `√(g/h)·(R − L)`.
    pub fn u(&self, x: f64, t: f64) -> f64 {
        let d = self.c * (t - self.t0);
        self.r * (profil(&self.droite, self.x0, self.dx, x - d) - profil(&self.gauche, self.x0, self.dx, x + d))
    }

    /// L'énergie par unité de largeur (J/m) : `ρ·g·Σ(R² + L²)·dx` — constante, W voyage sans perte.
    pub fn energie(&self, rho: f64) -> f64 {
        let g = self.c * self.r; // √(g·h)·√(g/h)
        rho * g * self.droite.iter().chain(&self.gauche).map(|v| v * v).sum::<f64>() * self.dx
    }
}

/// **La transduction** δ → W d'un domaine à son instant courant ; le domaine est ensuite détruit (il est consommé).
pub fn transduire(d: Domaine1D) -> TrainW1D {
    let (g, h, dx) = (d.gravite(), d.profondeur(), d.maille());
    let k = (h / g).sqrt();
    let (eta, u) = (d.eta(), d.u());
    let mut droite = Vec::with_capacity(eta.len());
    let mut gauche = Vec::with_capacity(eta.len());
    for (i, e) in eta.iter().enumerate() {
        let ub = 0.5 * (u[i + 1] + u[i]);
        droite.push(0.5 * (e + k * ub));
        gauche.push(0.5 * (e - k * ub));
    }
    TrainW1D { x0: 0.5 * dx, dx, c: (g * h).sqrt(), r: (g / h).sqrt(), t0: d.temps(), droite, gauche }
}

/// L'énergie d'un domaine δ par unité de largeur (J/m) : `½ρgΣη²dx + ½ρhΣu²dx`, les deux faces de bord comptées pour moitié.
pub fn energie_delta(d: &Domaine1D, rho: f64) -> f64 {
    let (g, h, dx) = (d.gravite(), d.profondeur(), d.maille());
    let u = d.u();
    let n = u.len() - 1;
    0.5 * rho * g * d.eta().iter().map(|v| v * v).sum::<f64>() * dx
        + 0.5 * rho * h * u[1..n].iter().map(|v| v * v).sum::<f64>() * dx
        + 0.25 * rho * h * (u[0] * u[0] + u[n] * u[n]) * dx
}

#[cfg(test)]
#[path = "tests_changement_solveur.rs"]
mod tests;
