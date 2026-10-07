//! **Les niveaux d'activité des cellules** (S608, liste 1.6 ; ADR-006 §1–3, `architecture_globale` §3.3).
//!
//! Trois structures distinctes, sans alignement imposé (ADR-006 §1) : la cellule de la `HydroGrid` (adressage, 64 m), le domaine (des
//! blocs de 8³ mailles de côté `dx`, posés depuis une origine quelconque), la structure interne du solveur (privée, absente ici). Le
//! serveur ne connaît du domaine que ses métadonnées : de quoi dire, par cellule, quel volume il couvre.
//!
//! Les cinq niveaux de la source : **inactive** (B seul), **simplifiée** (W la marque), **partiellement active** (un domaine la touche),
//! **simulation active** (un domaine la couvre entière), **niveau de détail supérieur** (un domaine de `dx ≤ dx_detail` la touche). Le dernier
//! dit la précision du solveur, non la subdivision : le niveau spatial et la précision physique ne sont pas équivalents.
//!
//! Ne fait pas : l'hystérésis des niveaux (celle des domaines existe, ADR-006 §4, `scheduler::ON/OFF`), la forme réduite d'une
//! perturbation qui disparaît (§3.4), la publication des niveaux au réseau.

use std::collections::BTreeMap;

use crate::hydro_grid::{CellId, TAILLES_M};

/// Une entrée refusée : un `dx` hors des six niveaux d'ADR-006 §3.2, une origine non finie, un bloc hors de portée de la grille.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Les six niveaux de `dx` d'ADR-006 §3.2 (m).
pub const DX_NIVEAUX: [f64; 6] = [0.02, 0.05, 0.10, 0.25, 0.50, 1.00];

/// Le niveau d'activité d'une cellule, du plus bas au plus haut.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Activite {
    Inactive,
    Simplifiee,
    Partielle,
    Active,
    Detail,
}

/// Ce que le serveur sait d'un domaine : son référentiel, l'origine de son réseau de blocs, son `dx`, ses blocs.
#[derive(Clone, Debug)]
pub struct DomaineMeta {
    pub frame: u32,
    pub origine: [f64; 3],
    pub dx: f64,
    pub blocs: Vec<[i32; 3]>,
}

/// La part d'une cellule pleine en dessous de laquelle elle n'est que touchée : l'arrondi d'une somme de volumes en f64 (10⁻¹²) cent fois.
const PLEINE: f64 = 1.0 - 1e-9;

/// **La couverture** : par cellule de niveau 0 du référentiel du domaine, le volume (m³) que ses blocs couvrent. Les blocs ne se
/// recouvrent pas : les volumes s'ajoutent.
pub fn couverture(d: &DomaineMeta) -> Result<BTreeMap<CellId, f64>, Refus> {
    if !DX_NIVEAUX.contains(&d.dx) || d.origine.iter().any(|v| !v.is_finite()) {
        return Err(Refus);
    }
    let (c, s) = (TAILLES_M[0], 8.0 * d.dx);
    let mut cov = BTreeMap::new();
    for b in &d.blocs {
        let bx: [(f64, f64); 3] = core::array::from_fn(|a| (d.origine[a] + b[a] as f64 * s, d.origine[a] + (b[a] + 1) as f64 * s));
        let rng: [(i64, i64); 3] = core::array::from_fn(|a| ((bx[a].0 / c).floor() as i64, (bx[a].1 / c).ceil() as i64));
        for k in rng[2].0..rng[2].1 {
            for j in rng[1].0..rng[1].1 {
                for i in rng[0].0..rng[0].1 {
                    let v: f64 = [i, j, k].iter().zip(&bx).map(|(&q, &(lo, hi))| (hi.min((q + 1) as f64 * c) - lo.max(q as f64 * c)).max(0.0))
                        .product();
                    if v > 0.0 {
                        let centre = [(i as f64 + 0.5) * c, (j as f64 + 0.5) * c, (k as f64 + 0.5) * c];
                        let id = CellId::cellule(d.frame, 0, centre).map_err(|_| Refus)?;
                        *cov.entry(id).or_insert(0.0) += v;
                    }
                }
            }
        }
    }
    Ok(cov)
}

/// **Les niveaux d'activité** des cellules du référentiel `frame` que W marque (`w`) ou qu'un domaine touche ; un domaine d'un autre
/// référentiel ne couvre rien ici. Toute autre cellule est inactive.
pub fn niveaux(frame: u32, w: &[CellId], domaines: &[DomaineMeta], dx_detail: f64) -> Result<BTreeMap<CellId, Activite>, Refus> {
    let plein = TAILLES_M[0].powi(3);
    let mut n: BTreeMap<CellId, Activite> = w.iter().filter(|c| c.frame == frame && c.niveau == 0).map(|&c| (c, Activite::Simplifiee)).collect();
    for d in domaines {
        let cov = couverture(d)?;
        if d.frame != frame {
            continue;
        }
        for (id, v) in cov {
            let a = if d.dx <= dx_detail {
                Activite::Detail
            } else if v >= PLEINE * plein {
                Activite::Active
            } else {
                Activite::Partielle
            };
            let e = n.entry(id).or_insert(Activite::Inactive);
            *e = (*e).max(a);
        }
    }
    Ok(n)
}

#[cfg(test)]
#[path = "tests_activite.rs"]
mod tests;
