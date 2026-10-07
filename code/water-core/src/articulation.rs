//! **L'articulation V↔δ** (S637, liste 5.10 ; ADR-025 §3, C21).
//!
//! La masse appartient à V ; δ montre la forme. V **déclenche** δ quand sa surface a bougé de façon significative (le seuil de S564, en
//! hauteur de surface, contre le dernier état publié) ; δ naît au niveau du nœud (**amorçage**, ADR-025 §3.1) ; puis, à chaque pas de V, sa
//! masse est **relaxée** vers celle du nœud — `(M_nœud − M_δ)·dt_V/τ`, τ ≈ 1 s (§3.2) : un forçage doux, jamais une remise à zéro. δ
//! n'écrit jamais dans V (C21) : aucune fonction de ce module ne le peut.
//!
//! Ne fait pas : un bassin quelconque (le niveau d'amorçage est celui d'un fond plat, volume/aire ; la forme générale passe par
//! `shape_lut`), la dérive d'un solveur réel, la destruction de δ quand V se calme.

use crate::hydro_network::{seuil::changement_significatif, Error, HydroNode, Shapes};
use crate::saint_venant_2d::SaintVenant2D;

/// Une entrée refusée : `τ` ou `dt_V` non positifs, un niveau non fini, une grille invalide.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// **V déclenche-t-il δ ?** — sa surface a bougé d'au moins `seuil_um` depuis le volume publié.
pub fn declenche(node: &HydroNode, publie_ml: i64, shapes: &Shapes<'_>, g_eff: [f32; 3], seuil_um: f64) -> Result<bool, Error> {
    changement_significatif(node, publie_ml, shapes, g_eff, seuil_um)
}

/// **L'amorçage** : δ né au niveau `niveau_m` du nœud sur le fond `z` (`nx × ny` mailles de `dx`), au repos.
pub fn amorcer(nx: usize, ny: usize, dx: f64, g: f64, z: Vec<f64>, niveau_m: f64) -> Result<SaintVenant2D, Refus> {
    if !niveau_m.is_finite() {
        return Err(Refus);
    }
    let h: Vec<f64> = z.iter().map(|z| (niveau_m - z).max(0.0)).collect();
    let n = h.len();
    SaintVenant2D::nouveau(nx, ny, dx, g, z, h, vec![0.0; n], vec![0.0; n]).map_err(|_| Refus)
}

/// **Le forçage** d'un pas de V : la masse de δ relaxée vers `volume_noeud_m3` — une couche uniforme sur les mailles mouillées, le relief
/// de la surface intact. Rend la correction appliquée (m³).
pub fn forcer(dom: &mut SaintVenant2D, volume_noeud_m3: f64, dt_v_s: f64, tau_s: f64) -> Result<f64, Refus> {
    if !(dt_v_s > 0.0) || !(tau_s > 0.0) || !volume_noeud_m3.is_finite() {
        return Err(Refus);
    }
    let correction = (volume_noeud_m3 - dom.volume()) * dt_v_s / tau_s;
    let mouillees = dom.h.iter().filter(|&&h| h > 0.0).count();
    if mouillees > 0 {
        let couche = correction / (mouillees as f64 * dom.dx * dom.dx);
        for h in dom.h.iter_mut().filter(|h| **h > 0.0) {
            *h += couche;
        }
    }
    Ok(correction)
}

/// **S638** — ce qu'un pas de V décide de δ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evenement {
    Rien,
    Naissance,
    Mort,
}

/// **S638 — le cycle de vie de δ attaché à un nœud** : le dernier volume publié, δ vivant ou non, le calme compté.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vie {
    pub publie_ml: i64,
    pub vivant: bool,
    pub calme: u32,
    pub calme_requis: u32,
}

impl Vie {
    /// Rien de vivant ; `publie_ml` le volume publié au départ ; `calme_requis` pas de calme avant la mort (l'hystérésis d'ADR-022 §2.6).
    pub fn nouvelle(publie_ml: i64, calme_requis: u32) -> Result<Vie, Refus> {
        if calme_requis == 0 {
            return Err(Refus);
        }
        Ok(Vie { publie_ml, vivant: false, calme: 0, calme_requis })
    }

    /// **Un pas de V** : un changement significatif publie, remet le calme à zéro et fait naître δ s'il n'existe pas ; sinon le calme
    /// compte, et δ meurt au bout de `calme_requis` pas.
    pub fn pas(&mut self, node: &HydroNode, shapes: &Shapes<'_>, g_eff: [f32; 3], seuil_um: f64) -> Result<Evenement, Error> {
        if declenche(node, self.publie_ml, shapes, g_eff, seuil_um)? {
            self.publie_ml = node.volume_ml;
            self.calme = 0;
            if !self.vivant {
                self.vivant = true;
                return Ok(Evenement::Naissance);
            }
        } else if self.vivant {
            self.calme += 1;
            if self.calme >= self.calme_requis {
                self.vivant = false;
                return Ok(Evenement::Mort);
            }
        }
        Ok(Evenement::Rien)
    }
}

#[cfg(test)]
#[path = "tests_articulation.rs"]
mod tests;
