//! **Les microbulles visuelles** (S597, liste 7.3).
//!
//! Un **nuage** laissé par un déferlement : des classes de diamètre, `N` bulles par m² chacune, réparties uniformément sur une profondeur
//! `D`. Chaque classe remonte à sa vitesse terminale (`bulle::vitesse_terminale`, Tomiyama, S540) et quitte le nuage par la surface : la
//! **fraction restante** est `max(0, 1 − v·t/D)`. Ce que le rendu blanchit : l'**épaisseur optique** `τ = Σ N·2·π·r²·fraction` (l'extinction
//! géométrique, `Q_ext` = 2) et l'**opacité** `1 − e^(−τ)`.
//!
//! Ne fait pas : la dissolution des bulles, l'émission (combien et de quelles tailles un déferlement en fait), la turbulence qui les
//! retient, le rendu.

use crate::bulle::{vitesse_terminale, Eau};

/// Le nombre de classes, au plus.
pub const MAX_CLASSES: usize = 16;

/// Une entrée refusée : diamètre, nombre, profondeur ou gravité non positifs ; trop de classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Une classe de bulles : diamètre (m), nombre par m² de surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Classe {
    pub diametre_m: f64,
    pub nombre_par_m2: f64,
}

/// **Un nuage de microbulles** : ses classes, leur vitesse terminale (calculée une fois), sa profondeur initiale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Nuage {
    classes: [Classe; MAX_CLASSES],
    vitesses: [f64; MAX_CLASSES],
    n: usize,
    profondeur_m: f64,
}

impl Nuage {
    pub fn new(classes: &[Classe], profondeur_m: f64, g: f64, eau: Eau) -> Result<Self, Refus> {
        if classes.is_empty() || classes.len() > MAX_CLASSES || !(profondeur_m > 0.0) || !profondeur_m.is_finite() || !(g > 0.0)
            || classes.iter().any(|c| !(c.diametre_m > 0.0) || !(c.nombre_par_m2 > 0.0) || !c.diametre_m.is_finite()) {
            return Err(Refus);
        }
        let mut nuage = Nuage { classes: [classes[0]; MAX_CLASSES], vitesses: [0.0; MAX_CLASSES], n: classes.len(), profondeur_m };
        for (k, c) in classes.iter().enumerate() {
            nuage.classes[k] = *c;
            nuage.vitesses[k] = vitesse_terminale(c.diametre_m, g, eau);
        }
        Ok(nuage)
    }

    /// La vitesse de remontée de la classe `k`, m/s.
    pub fn vitesse(&self, k: usize) -> f64 {
        self.vitesses[k]
    }

    /// **La fraction restante** de la classe `k` à l'instant `t` (s) après la formation du nuage.
    pub fn fraction(&self, k: usize, t: f64) -> f64 {
        (1.0 - self.vitesses[k] * t.max(0.0) / self.profondeur_m).max(0.0)
    }

    /// **L'épaisseur optique** du nuage à l'instant `t`.
    pub fn epaisseur_optique(&self, t: f64) -> f64 {
        (0..self.n).map(|k| {
            let r = 0.5 * self.classes[k].diametre_m;
            self.classes[k].nombre_par_m2 * 2.0 * core::f64::consts::PI * r * r * self.fraction(k, t)
        }).sum()
    }

    /// **L'opacité** `1 − e^(−τ)` à l'instant `t`.
    pub fn opacite(&self, t: f64) -> f64 {
        1.0 - (-self.epaisseur_optique(t)).exp()
    }
}

#[cfg(test)]
#[path = "tests_microbulles.rs"]
mod tests;
