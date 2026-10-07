//! **La glace** (S574, listes 7.6 et 7.7 ; SPEC-002 §4 ; ADR-027 §3 : retenue, bornée aux lacs et aux baies abritées).
//!
//! - **La croissance de Stefan** : `h = √(h₀² + 2·k·ΔT·t/(ρ·L))` — la chaleur latente évacuée par conduction à travers la glace déjà
//!   formée ; en degrés-jours de gel (FDD), `h = 0,0352·√FDD` depuis zéro.
//! - **La portance de Gold** : `P = A·h²`, `A` = 3,5 kg/cm² — la valeur prudente de Gold pour des charges mobiles. Avec des masses de
//!   référence (une personne équipée 100 kg, un groupe ou une motoneige 400 kg, une voiture légère 1 500 kg, un camion léger 5 000 kg),
//!   elle reproduit la table de SPEC-002 ligne à ligne.
//! - **La flottaison** : la fraction émergée `1 − ρ_glace/ρ_eau`.
//! - **La formation en plaque** exige une mer calme : `Hs < 0,15 m`.
//!
//! La charge admissible est dérivée **une fois, ici** (SPEC-006 §5.1) : l'IA, le jeu et l'audio ne la recalculent pas avec trois
//! constantes différentes.

/// Conductivité thermique de la glace, W/m/K.
pub const K_GLACE: f64 = 2.2;
/// Chaleur latente de fusion, J/kg.
pub const L_FUSION: f64 = 334e3;
/// Masse volumique de la glace, kg/m³.
pub const RHO_GLACE: f64 = 917.0;
/// Le coefficient de Gold, kg/m² (3,5 kg/cm²).
pub const A_GOLD: f64 = 3.5e4;
/// La houle significative au-delà de laquelle la glace ne prend pas en plaque, m.
pub const HS_PLAQUE: f64 = 0.15;

/// Une entrée refusée : épaisseur, gel, temps ou charge négatifs ou non finis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

fn fini_positif(x: f64) -> Result<f64, Refus> {
    if x >= 0.0 && x.is_finite() { Ok(x) } else { Err(Refus) }
}

/// **L'épaisseur après `t` secondes** sous `delta_t` kelvins de gel, depuis `h0` (m) — Stefan.
pub fn epaisseur(h0: f64, delta_t: f64, t: f64) -> Result<f64, Refus> {
    let (h0, delta_t, t) = (fini_positif(h0)?, fini_positif(delta_t)?, fini_positif(t)?);
    Ok((h0 * h0 + 2.0 * K_GLACE * delta_t * t / (RHO_GLACE * L_FUSION)).sqrt())
}

/// **L'épaisseur après `fdd` degrés-jours de gel**, depuis zéro (m).
pub fn epaisseur_fdd(fdd: f64) -> Result<f64, Refus> {
    epaisseur(0.0, fini_positif(fdd)?, 86_400.0)
}

/// **La charge admissible** d'une glace de `h` mètres, kg — Gold.
pub fn charge_admissible_kg(h: f64) -> Result<f64, Refus> {
    let h = fini_positif(h)?;
    Ok(A_GOLD * h * h)
}

/// **L'épaisseur qui porte `masse_kg`**, m — Gold inversé.
pub fn epaisseur_requise(masse_kg: f64) -> Result<f64, Refus> {
    Ok((fini_positif(masse_kg)? / A_GOLD).sqrt())
}

/// **La fraction émergée** d'une glace qui flotte dans une eau de masse volumique `rho_eau` (kg/m³, au-dessus de celle de la glace).
pub fn fraction_emergee(rho_eau: f64) -> Result<f64, Refus> {
    if !(rho_eau > RHO_GLACE) || !rho_eau.is_finite() {
        return Err(Refus);
    }
    Ok(1.0 - RHO_GLACE / rho_eau)
}

/// **La glace prend-elle en plaque** sous une houle significative `hs` (m) ?
pub fn prend_en_plaque(hs: f64) -> Result<bool, Refus> {
    Ok(fini_positif(hs)? < HS_PLAQUE)
}

// --- S575 — la glace d'un plan d'eau de V (C15 ; ADR-203 D6 : le gel des contenants passe par V).

/// Un quantum de gel : 917 ml d'eau deviennent 1 000 ml de glace — `917·1000 = 1000·917`, la masse à l'entier.
pub const QUANTUM_EAU_ML: i64 = 917;
/// Le volume de glace d'un quantum, ml.
pub const QUANTUM_GLACE_ML: i64 = 1000;

/// **S575 — amener la glace d'un nœud de V vers l'épaisseur visée.** La glace est une couche des liquides de V (ADR-241), de densité
/// 917, au-dessus de l'eau : `ligne` est la composition du nœud, `eau` et `glace` les indices de leurs liquides, `aire_m2` l'aire de la
/// surface libre. Le volume de glace visé est `aire·h_visee` ; on gèle (ou fond) par quanta entiers vers lui — chaque quantum change
/// `volume_ml` de ±83 ml. Rend le nombre de quanta (positif : gel, négatif : fonte). Refus, sans rien écrire : indices, épaisseur ou aire
/// invalides, une ligne qui ne somme pas au volume, la capacité du nœud dépassée.
pub fn ajuster_glace(node: &mut crate::hydro_network::HydroNode, ligne: &mut [i64], eau: usize, glace: usize, aire_m2: f64, h_visee: f64)
    -> Result<i64, Refus> {
    if eau >= ligne.len() || glace >= ligne.len() || eau == glace || !(aire_m2 > 0.0) || !aire_m2.is_finite() {
        return Err(Refus);
    }
    fini_positif(h_visee)?;
    if ligne.iter().any(|v| *v < 0) || ligne.iter().sum::<i64>() != node.volume_ml {
        return Err(Refus);
    }
    let visee_ml = aire_m2 * h_visee * 1e6;
    let ecart = visee_ml - ligne[glace] as f64;
    let q = if ecart >= 0.0 {
        ((ecart / QUANTUM_GLACE_ML as f64).floor() as i64).min(ligne[eau] / QUANTUM_EAU_ML)
    } else {
        -(((-ecart) / QUANTUM_GLACE_ML as f64).ceil() as i64).min(ligne[glace] / QUANTUM_GLACE_ML)
    };
    let croit = q * (QUANTUM_GLACE_ML - QUANTUM_EAU_ML);
    if node.volume_ml + croit > node.capacity_ml {
        return Err(Refus);
    }
    ligne[eau] -= q * QUANTUM_EAU_ML;
    ligne[glace] += q * QUANTUM_GLACE_ML;
    node.volume_ml += croit;
    Ok(q)
}

// --- S635 — le dégel physique (le bilan d'énergie de surface).

/// **S635 — le flux de fonte** à la surface de la glace, W/m² : `q = α·T_air + (1 − albédo)·S` — la convection de l'air (`α`, W/m²/K ;
/// `T_air` en °C au-dessus de 0) et le soleil absorbé (`S`, W/m²) ; nul s'il est négatif (alors la glace croît : Stefan). Ne compte ni
/// l'infrarouge, ni l'eau sous la glace, ni la neige. Refus : `α` ou `S` négatifs, un albédo hors de [0, 1], une valeur non finie.
pub fn flux_de_fonte(t_air: f64, alpha: f64, albedo: f64, solaire: f64) -> Result<f64, Refus> {
    if !t_air.is_finite() || !(0.0..=1.0).contains(&albedo) {
        return Err(Refus);
    }
    let (alpha, solaire) = (fini_positif(alpha)?, fini_positif(solaire)?);
    Ok((alpha * t_air + (1.0 - albedo) * solaire).max(0.0))
}

/// **S635 — l'épaisseur après `t` secondes de fonte** sous le flux `q` : `max(0, h₀ − q·t/(ρ_glace·L))`.
pub fn epaisseur_fondue(h0: f64, q: f64, t: f64) -> Result<f64, Refus> {
    let (h0, q, t) = (fini_positif(h0)?, fini_positif(q)?, fini_positif(t)?);
    Ok((h0 - q * t / (RHO_GLACE * L_FUSION)).max(0.0))
}

/// **S635 — la durée de fonte** d'une épaisseur `h₀` sous le flux `q > 0`, s.
pub fn duree_de_fonte(h0: f64, q: f64) -> Result<f64, Refus> {
    let h0 = fini_positif(h0)?;
    if !(q > 0.0) || !q.is_finite() {
        return Err(Refus);
    }
    Ok(h0 * RHO_GLACE * L_FUSION / q)
}

#[cfg(test)]
#[path = "tests_glace.rs"]
mod tests;
