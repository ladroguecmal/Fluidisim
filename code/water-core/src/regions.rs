//! **Les régions de mer décrites par descripteur** (S594, liste 11.2 ; I-09 : « on interpole des paramètres, jamais des réalisations »).
//!
//! Une région est un rectangle du repère et un **descripteur** — ses paramètres : la hauteur significative de sa mer, son niveau moyen.
//! En un point, chaque région pèse 1 à l'intérieur, 0 au-delà de son bord, et une transition en `smoothstep` sur une bande centrée sur
//! son bord ; les poids sont normalisés (une partition de l'unité) et **les paramètres se mélangent** — jamais les champs : deux
//! réalisations indépendantes moyennées donnent une mer plus calme que les deux (A11 : −29 % à Hs égal).
//!
//! Les composantes de B sont les mêmes partout (ADR-004 §2.1) ; seule leur amplitude suit le paramètre : [`echelle`] met l'échantillon
//! de B à l'échelle `Hs_local/Hs_réf` et le décale du niveau moyen local. Ne fait pas : la période et la direction par région (elles
//! changent les composantes : une pondération spectrale), la marée par région, le placement sur la planète (11.1).

/// Le nombre de régions, au plus, qu'un point consulte.
pub const MAX_REGIONS: usize = 16;

/// Une entrée refusée : un rectangle vide, une bande non positive, trop de régions, un point hors de toute région.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Les paramètres d'une région.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Descripteur {
    pub hs_m: f32,
    pub niveau_moyen_m: f32,
}

/// Une région : un rectangle `[min, max]` (m) et son descripteur.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Region {
    pub min: [f64; 2],
    pub max: [f64; 2],
    pub descripteur: Descripteur,
}

fn smoothstep(u: f64) -> f64 {
    let u = u.clamp(0.0, 1.0);
    u * u * (3.0 - 2.0 * u)
}

/// Le poids brut d'une région en `(x, y)` : la distance signée à son bord (positive dedans), passée par la transition de largeur `bande`.
fn poids(r: &Region, x: f64, y: f64, bande: f64) -> f64 {
    let d = (x - r.min[0]).min(r.max[0] - x).min(y - r.min[1]).min(r.max[1] - y);
    smoothstep((d + 0.5 * bande) / bande)
}

/// **Les paramètres en `(x, y)`** : le mélange des descripteurs, pondéré par la partition de l'unité. Rend aussi la somme des poids
/// bruts (avant normalisation).
pub fn parametres_en(regions: &[Region], x: f64, y: f64, bande: f64) -> Result<(Descripteur, f64), Refus> {
    if regions.is_empty() || regions.len() > MAX_REGIONS || !(bande > 0.0) || !bande.is_finite() || !x.is_finite() || !y.is_finite()
        || regions.iter().any(|r| !(r.min[0] < r.max[0] && r.min[1] < r.max[1])) {
        return Err(Refus);
    }
    let total: f64 = regions.iter().map(|r| poids(r, x, y, bande)).sum();
    if !(total > 0.0) {
        return Err(Refus);
    }
    let (mut hs, mut niveau) = (0.0f64, 0.0f64);
    for r in regions {
        let w = poids(r, x, y, bande) / total;
        hs += w * r.descripteur.hs_m as f64;
        niveau += w * r.descripteur.niveau_moyen_m as f64;
    }
    Ok((Descripteur { hs_m: hs as f32, niveau_moyen_m: niveau as f32 }, total))
}

/// **L'échantillon de B à l'échelle d'un paramètre** : `η`, la vitesse, `∂η/∂t` et la cambrure multipliés par `facteur`, la pente aussi
/// (la normale recalculée), `η` décalé de `niveau`.
pub fn echelle(mut s: crate::WaterSample, facteur: f32, niveau: f32) -> crate::WaterSample {
    s.eta = s.eta * facteur + niveau;
    for u in &mut s.u_total {
        *u *= facteur;
    }
    s.deta_dt *= facteur;
    s.steepness *= facteur;
    let (sx, sy) = (-s.normal[0] / s.normal[2] * facteur, -s.normal[1] / s.normal[2] * facteur);
    let inv = 1.0 / (sx * sx + sy * sy + 1.0).sqrt();
    s.normal = [-sx * inv, -sy * inv, inv];
    s
}

#[cfg(test)]
#[path = "tests_regions.rs"]
mod tests;
