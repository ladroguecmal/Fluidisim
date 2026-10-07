//! **L'eau en amont du terrain, le géoïde dans l'outil de terrain** (S605, liste 12.4 ; SPEC-005 §3–4, ADR-002 §2.4).
//!
//! Le « zéro » d'une scène n'est pas une altitude mais une distance au centre de la planète : un outil qui travaille dans le plan tangent
//! d'une ancre avec un `z` vertical place une plage à 30 km de l'ancre 70 m trop haut. [`Geoide::altitude`] et [`Geoide::z_local`] passent du
//! plan tangent à l'altitude au-dessus du niveau moyen et retour, sans perte au rayon de la planète (les différences s'écrivent sans
//! soustraire deux rayons).
//!
//! L'ordre imposé (SPEC-005 §3) : le squelette hydrographique d'abord, le terrain gravé pour le satisfaire. [`conformer`] est l'étape 2 :
//! il lit les biefs (leurs lignes d'eau en altitude, S604) et ne modifie que le terrain — chaque cellule à moins d'une demi-largeur d'un
//! segment descend au fond `z_eau − h` (Manning), placé par le géoïde.
//!
//! Ne fait pas : l'anomalie régionale et la marée du niveau moyen, le trait de côte et la bathymétrie du squelette, les dérivations
//! (étape 3), les ancres multiples.

use crate::riviere::Bief;

/// Une entrée refusée : rayon non positif, grille vide ou incohérente, pas non positif, un bief qui ne descend pas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Le niveau moyen sphérique, de rayon `rayon_m`, vu du plan tangent d'une ancre posée sur lui (`z` le long du radial de l'ancre).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Geoide {
    rayon_m: f64,
}

impl Geoide {
    pub fn nouveau(rayon_m: f64) -> Result<Geoide, Refus> {
        (rayon_m > 0.0 && rayon_m.is_finite()).then_some(Geoide { rayon_m }).ok_or(Refus)
    }

    /// **L'altitude** au-dessus du niveau moyen du point `p` du plan tangent : `√(x² + y² + (R + z)²) − R`, écrite
    /// `(x² + y² + z·(2R + z)) / (√(…) + R)`.
    pub fn altitude(&self, p: [f64; 3]) -> f64 {
        let r = self.rayon_m;
        let d2 = p[0] * p[0] + p[1] * p[1];
        (d2 + p[2] * (2.0 * r + p[2])) / ((d2 + (r + p[2]) * (r + p[2])).sqrt() + r)
    }

    /// **Le `z` du plan tangent** du point d'altitude `altitude` à la verticale de `(x, y)` : `√((R + a)² − x² − y²) − R`, écrit sans
    /// soustraire deux rayons. `None` au-delà de l'horizon.
    pub fn z_local(&self, x: f64, y: f64, altitude: f64) -> Option<f64> {
        let r = self.rayon_m;
        let d2 = x * x + y * y;
        let a = (r + altitude) * (r + altitude) - d2;
        (a >= 0.0).then(|| (altitude * (2.0 * r + altitude) - d2) / (a.sqrt() + r))
    }

    /// **L'écart entre la sphère et son plan tangent** à la distance `d` de l'ancre, le long de la sphère : `R·(1 − cos(d/R))`, écrit
    /// `2R·sin²(d/2R)` (SPEC-005 §4).
    pub fn ecart_plan_tangent(&self, d: f64) -> f64 {
        let s = (d / (2.0 * self.rayon_m)).sin();
        2.0 * self.rayon_m * s * s
    }
}

/// Le terrain de l'outil : `nx × ny` cellules de `pas_m`, le coin `origine` dans le plan tangent, `z` le long du radial de l'ancre (`x` le
/// plus rapide) ; le centre de la cellule `(i, j)` en `origine + (i + ½, j + ½)·pas`.
#[derive(Clone, Debug, PartialEq)]
pub struct Grille {
    pub nx: usize,
    pub ny: usize,
    pub pas_m: f64,
    pub origine: [f64; 2],
    pub z: Vec<f64>,
}

/// Ce que la gravure a fait : les cellules gravées, le plus grand creusement (en altitude, m).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rapport {
    pub gravees: usize,
    pub creusement_max_m: f64,
}

/// **Conformer le terrain au squelette** : chaque cellule dont le centre est à moins d'une demi-largeur d'un segment d'un bief, et plus
/// haute que le fond `z_eau − h` à sa projection sur le segment, y descend. Les biefs ne sont pas modifiés.
pub fn conformer(grille: &mut Grille, geoide: &Geoide, biefs: &[Bief], g: f64) -> Result<Rapport, Refus> {
    if grille.nx == 0 || grille.ny == 0 || grille.z.len() != grille.nx * grille.ny || !(grille.pas_m > 0.0) || !grille.pas_m.is_finite() {
        return Err(Refus);
    }
    let mut gravee = vec![false; grille.z.len()];
    let mut creusement_max_m = 0.0f64;
    for b in biefs {
        let profil = b.profil(g).map_err(|_| Refus)?;
        let demi = 0.5 * b.largeur_m;
        for (seg, w) in profil.iter().zip(b.ligne.windows(2)) {
            let h = seg.hauteur_m.ok_or(Refus)?;
            let (p0, d) = ([w[0][0], w[0][1]], [w[1][0] - w[0][0], w[1][1] - w[0][1]]);
            let l2 = d[0] * d[0] + d[1] * d[1];
            let bornes = |a: f64, b: f64, o: f64, n: usize| {
                let lo = ((a.min(b) - demi - o) / grille.pas_m - 0.5).floor().max(0.0) as usize;
                let hi = (((a.max(b) + demi - o) / grille.pas_m - 0.5).ceil().max(0.0) as usize).min(n - 1);
                lo..=hi
            };
            for j in bornes(w[0][1], w[1][1], grille.origine[1], grille.ny) {
                for i in bornes(w[0][0], w[1][0], grille.origine[0], grille.nx) {
                    let c = [grille.origine[0] + (i as f64 + 0.5) * grille.pas_m, grille.origine[1] + (j as f64 + 0.5) * grille.pas_m];
                    let t = (((c[0] - p0[0]) * d[0] + (c[1] - p0[1]) * d[1]) / l2).clamp(0.0, 1.0);
                    if (c[0] - p0[0] - t * d[0]).hypot(c[1] - p0[1] - t * d[1]) > demi {
                        continue;
                    }
                    let fond = w[0][2] + t * (w[1][2] - w[0][2]) - h;
                    let z_fond = geoide.z_local(c[0], c[1], fond).ok_or(Refus)?;
                    let k = j * grille.nx + i;
                    if grille.z[k] > z_fond {
                        creusement_max_m = creusement_max_m.max(geoide.altitude([c[0], c[1], grille.z[k]]) - fond);
                        grille.z[k] = z_fond;
                        gravee[k] = true;
                    }
                }
            }
        }
    }
    Ok(Rapport { gravees: gravee.iter().filter(|&&v| v).count(), creusement_max_m })
}

#[cfg(test)]
#[path = "tests_geoide.rs"]
mod tests;
