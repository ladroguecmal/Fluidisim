//! ADR-092 : ce qu'un objet qui entre dans l'eau donne au modèle.
//!
//! `WaveEvent::impact` exige une longueur d'onde et une énergie. L'une se **dérive** de la
//! taille de l'objet, l'autre non — et ce module tient les deux statuts séparés plutôt que de
//! les mélanger dans une formule d'apparence physique.
use crate::impact_field::Medium;
use crate::radial_impact::SLOPE_L1_RATIO;

/// Rapport `λ / b` : la forme spatiale initiale du candidat s'annule à `0,2985 λ`, et c'est
/// cette étendue centrale — celle que la cavité creuse — qu'on fait coïncider avec la
/// demi-largeur mouillée de Wagner, égale à `b` à la fin de l'impact (SPEC-001 §5 bis).
///
/// **Mesuré, non choisi** (S136) : la forme est exactement homothétique en `λ`, donc ce rapport
/// ne dépend que de la définition du rayon. Les deux autres lectures raisonnables — mi-hauteur
/// et rayon de giration — donnent 5,46 et 6,11 : l'incertitude est d'un facteur 1,8, et le banc
/// B2 la resserrera.
pub const ALPHA: f32 = 3.35;
/// Constante sans dimension de la borne d'énergie `E_max = K·ρ·g·λ⁴·s²`. Mesurée par
/// dichotomie sur le candidat lui-même : `E_max/λ⁴` est constant à 8,9401e-2 pour λ de 0,5 à
/// 8 m, et le rapport vaut exactement 16 quand la pente admise quadruple (S136).
///
/// **S141 :** `8,891e-4` valait quand `max_slope` bornait la borne L1. Depuis que la frontière
/// compare la pente **réelle** (ADR-094), le candidat admet `SLOPE_L1_RATIO` fois plus de pente,
/// donc son carré en énergie — `E ∝ pente²`. Le facteur est écrit ici plutôt que multiplié dans
/// la valeur : si le rapport mesuré change, la borne annoncée suit, au lieu de mentir en silence.
pub const K_ENERGIE: f32 = 8.891e-4 * SLOPE_L1_RATIO * SLOPE_L1_RATIO;
/// Entrée d'un objet dans l'eau, telle que le gameplay la connaît.
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    /// Demi-largeur de l'objet, en mètres. C'est l'étendue mouillée à la fin de l'impact.
    pub half_width_m: f32,
    /// Vitesse d'entrée, en mètres par seconde.
    pub speed_ms: f32,
    /// Fraction de l'énergie de la masse ajoutée qui part en ondes de gravité.
    ///
    /// **À calibrer — banc B2.** Le reste va dans la gerbe, la cavité, la turbulence et la
    /// chaleur ; aucune lecture du modèle ne produit ce nombre, et c'est pourquoi il est ici
    /// et non enfoui dans une constante. `max_transferred_fraction` en donne la borne haute.
    pub transferred_fraction: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Taille, vitesse ou fraction hors du domaine : non finie, négative, ou fraction > 1.
    Entry,
    /// Milieu non fini ou non positif.
    Medium,
    /// L'énergie demandée dépasse ce que le candidat peut porter à cette longueur d'onde.
    /// `max_transferred_fraction` dit ce que l'appelant peut demander.
    TooEnergetic,
}
/// Longueur d'onde d'un impact : `λ = α·b`, dérivée de la forme du modèle (ADR-092).
pub fn wavelength_m(entry: &Entry) -> f32 {
    ALPHA * entry.half_width_m
}
/// Énergie de la masse ajoutée emportée à la vitesse d'entrée : `½ρb³v²`.
/// Ce n'est **pas** l'énergie transférée aux ondes — c'est ce dont elle est une fraction.
pub fn reference_energy_j(entry: &Entry, medium: &Medium) -> f32 {
    0.5 * medium.density * entry.half_width_m.powi(3) * entry.speed_ms * entry.speed_ms
}
/// Borne du modèle sur l'énergie, `E_max = K·ρ·g·λ⁴·s²` (ADR-092).
pub fn max_energy_j(wavelength_m: f32, medium: &Medium) -> f32 {
    K_ENERGIE * medium.density * medium.gravity * wavelength_m.powi(4) * medium.max_slope.powi(2)
}
/// Fraction transférée que le modèle peut porter, `2·K·g·α⁴·b·s²/v²`.
///
/// Elle **décroît comme le carré de la vitesse et croît avec la taille** : un petit objet
/// rapide est celui dont le candidat peut représenter la plus faible part d'énergie.
pub fn max_transferred_fraction(entry: &Entry, medium: &Medium) -> f32 {
    let reference = reference_energy_j(entry, medium);
    if reference <= 0.0 {
        return 0.0;
    }
    max_energy_j(wavelength_m(entry), medium) / reference
}
/// Les deux nombres que `WaveEvent::impact` attend, ou le motif du refus.
///
/// L'identité, le référentiel, la naissance et la durée de vie restent au gameplay : ce sont
/// ses données, pas de la physique, et ce générateur ne les invente pas.
pub fn impact_from_entry(entry: &Entry, medium: &Medium) -> Result<(f32, f32), Error> {
    if !entry.half_width_m.is_finite()
        || entry.half_width_m <= 0.0
        || !entry.speed_ms.is_finite()
        || entry.speed_ms <= 0.0
        || !entry.transferred_fraction.is_finite()
        || entry.transferred_fraction <= 0.0
        || entry.transferred_fraction > 1.0
    {
        return Err(Error::Entry);
    }
    if [
        medium.gravity,
        medium.density,
        medium.depth,
        medium.max_slope,
    ]
    .iter()
    .any(|v| !v.is_finite() || *v <= 0.0)
    {
        return Err(Error::Medium);
    }
    let lambda = wavelength_m(entry);
    let energy = entry.transferred_fraction * reference_energy_j(entry, medium);
    if !energy.is_finite() || energy <= 0.0 || energy > max_energy_j(lambda, medium) {
        return Err(Error::TooEnergetic);
    }
    Ok((energy, lambda))
}
#[cfg(test)]
#[path = "tests_impact_generator.rs"]
mod tests;
