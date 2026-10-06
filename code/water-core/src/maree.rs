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
}

#[cfg(test)]
#[path = "tests_maree.rs"]
mod tests;
