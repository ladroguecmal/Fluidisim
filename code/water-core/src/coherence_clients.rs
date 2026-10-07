//! **Les grandes formes cohérentes entre clients, les détails locaux libres** (S615, liste 10.5 ; ADR-001 B2, ADR-007 §3).
//!
//! δ n'est jamais le même d'un client à l'autre : une autre maille, d'autres détails semés localement, jamais D1 (SPEC-003). Ce qui doit
//! être vu pareil par tous passe à W — répliqué par événement — au-delà d'une coupure `λ_cut` ; le reste demeure dans δ, local et libre.
//! [`transduire_coupe`] est la transduction de S612 appliquée à la seule part passe-bas de δ : un filtre gaussien d'écart-type `σ` sur `η`
//! et sur `ū`, tronqué à `4σ` et normalisé.
//!
//! Ne fait pas : le 2D/3D, le choix de `λ_cut` par la physique, l'autorité de l'émission de W (qui émet l'événement répliqué : 10.1), le
//! transport.

use crate::changement_solveur::TrainW1D;
use crate::substitutif::Domaine1D;

/// Une entrée refusée : `σ` ou `dx` non positifs ou non finis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// **La coupure passe-bas** : la convolution de `v` (pas `dx`) par une gaussienne d'écart-type `sigma`, tronquée à `4σ`, de poids
/// normalisés ; nulle au-delà du profil.
pub fn filtre_gaussien(v: &[f64], dx: f64, sigma: f64) -> Result<Vec<f64>, Refus> {
    if !(dx > 0.0) || !(sigma > 0.0) || !dx.is_finite() || !sigma.is_finite() {
        return Err(Refus);
    }
    let m = (4.0 * sigma / dx).ceil() as i64;
    let noyau: Vec<f64> = (-m..=m).map(|j| (-0.5 * (j as f64 * dx / sigma).powi(2)).exp()).collect();
    let somme: f64 = noyau.iter().sum();
    let noyau: Vec<f64> = noyau.iter().map(|k| k / somme).collect();
    let n = v.len() as i64;
    Ok((0..n).map(|i| {
        (-m..=m).filter(|d| (0..n).contains(&(i - d))).map(|d| noyau[(m + d) as usize] * v[(i - d) as usize]).sum()
    }).collect())
}

/// **La transduction coupée** : la part passe-bas de δ part dans W (les invariants de Riemann de S612 sur `η` et `ū` filtrés) ; le reste,
/// rendu, demeure au client.
pub fn transduire_coupe(d: &Domaine1D, sigma: f64) -> Result<(TrainW1D, Vec<f64>), Refus> {
    let (g, h, dx) = (d.gravite(), d.profondeur(), d.maille());
    let (eta, u) = (d.eta(), d.u());
    let ub: Vec<f64> = (0..eta.len()).map(|i| 0.5 * (u[i + 1] + u[i])).collect();
    let (ef, uf) = (filtre_gaussien(eta, dx, sigma)?, filtre_gaussien(&ub, dx, sigma)?);
    let k = (h / g).sqrt();
    let droite = ef.iter().zip(&uf).map(|(e, v)| 0.5 * (e + k * v)).collect();
    let gauche = ef.iter().zip(&uf).map(|(e, v)| 0.5 * (e - k * v)).collect();
    let reste = eta.iter().zip(&ef).map(|(e, f)| e - f).collect();
    let train = TrainW1D { x0: 0.5 * dx, dx, c: (g * h).sqrt(), r: (g / h).sqrt(), t0: d.temps(), droite, gauche };
    Ok((train, reste))
}

#[cfg(test)]
#[path = "tests_coherence_clients.rs"]
mod tests;
