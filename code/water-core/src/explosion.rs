//! **Les explosions sous-marines : la bulle** (S587, liste 3.3 ; ADR-001 : les ondes d'explosion sont de W).
//!
//! Une charge libère son énergie ; une fraction (un paramètre d'auteur : 0,4 pour le TNT) reste dans la bulle de gaz, qui s'étend jusqu'à
//! l'équilibre d'énergie `E_b = (4/3)·π·R³·p` contre la pression du fond (`p = p_atm + ρ·g·d`), puis s'effondre en `t_c = C·R·√(ρ/p)` —
//! l'effondrement de Rayleigh d'une cavité vide, `C = √(3π/2)·Γ(5/6)/Γ(1/3)` = 0,914681 —, et rebondit : la période du premier battement
//! est `T = 2·t_c`. Les lois d'échelle en découlent : `R ∝ (W/p)^(1/3)`, `T ∝ W^(1/3)·p^(−5/6)` (la forme de Willis).
//!
//! Ne fait pas : les battements suivants (les pertes), la migration de la bulle vers la surface, l'onde de choc, les ondes de surface et
//! la gerbe, l'entrée dans W.

/// La constante de Rayleigh, `√(3π/2)·Γ(5/6)/Γ(1/3)` (éprouvée par une intégration indépendante, S587).
pub const RAYLEIGH: f64 = 0.914_681_356_501_962;
/// L'énergie massique du TNT, J/kg.
pub const TNT_J_PAR_KG: f64 = 4.184e6;
/// La pression atmosphérique de référence, Pa.
pub const P_ATM: f64 = 101_325.0;

/// Une entrée refusée : masse, profondeur, fraction, masse volumique ou gravité non positives ou non finies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// La bulle d'une explosion : rayon maximal (m), période du premier battement (s).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bulle {
    pub rayon_max_m: f64,
    pub periode_s: f64,
}

/// **La bulle** d'une charge de `masse_kg` d'équivalent TNT à `profondeur_m`, une fraction `fraction` de l'énergie restant dans la bulle.
pub fn bulle(masse_kg: f64, profondeur_m: f64, fraction: f64, rho: f64, g: f64) -> Result<Bulle, Refus> {
    let positif = |x: f64| x > 0.0 && x.is_finite();
    if !positif(masse_kg) || !(profondeur_m >= 0.0) || !profondeur_m.is_finite() || !positif(fraction) || fraction > 1.0
        || !positif(rho) || !positif(g) {
        return Err(Refus);
    }
    let energie = fraction * TNT_J_PAR_KG * masse_kg;
    let p = P_ATM + rho * g * profondeur_m;
    let rayon = (3.0 * energie / (4.0 * core::f64::consts::PI * p)).cbrt();
    Ok(Bulle { rayon_max_m: rayon, periode_s: 2.0 * RAYLEIGH * rayon * (rho / p).sqrt() })
}

#[cfg(test)]
#[path = "tests_explosion.rs"]
mod tests;
