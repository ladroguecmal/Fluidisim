//! **La graine d'un domaine substitutif** (S623, liste 4.11 ; ADR-022 §3, I-17, I-09, ADR-013 §4).
//!
//! Un domaine substitutif naît faux et met une minute à s'établir (60,65 s mesurées en S610) : aucune fenêtre de prévision ne le couvre.
//! Il repart donc d'une **graine cuite** hors ligne — jamais d'une capture d'exécution (I-17). Trois règles d'ADR-022 §3.5 :
//! - **`condense` est une opération d'outil** : la fonction n'est compilée que pour un hôte de cuisson (`cfg(test)` ou la fonctionnalité
//!   `cuisson`) — un hôte de jeu ne l'a pas (L19 : l'interdit inexprimable) ;
//! - **interpoler les paramètres, jamais les champs** (I-09) : [`choisir`] prend la graine la plus proche dans la tolérance, ou aucune ;
//! - **la masse est autoritaire, la graine ne l'est pas** : [`SeedState::restaurer`] renormalise la forme sur le volume du nœud V (des ml).
//!
//! L'identifiant est l'empreinte du contenu (FNV-1a ; SPEC-005 §7.3 demande sha256 : noté). Ne fait pas : le 2D, la graine côtière de 12.3
//! branchée, la tolérance mesurée (banc B4).

use crate::cotier::depuis_f16;
use crate::hash::Hasher64;
use crate::substitutif::Domaine1D;

/// Une entrée refusée : hors tolérance, un volume de nœud non positif, des tableaux de taille fausse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Le genre d'une graine (ADR-022 §3.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Genre {
    Cotier,
    Bassin,
    Auteur,
}

/// Ce sous quoi la graine a été cuite.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parametres {
    pub hs_m: f64,
    pub tp_s: f64,
    pub theta_rad: f64,
    pub phase_maree: u8,
    pub liquide: u8,
}

/// L'écart de paramètres au-delà duquel une graine se refuse (ADR-022 §3.5 : le seul seuil physique du système ; à calibrer, B4).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tolerance {
    pub hs_relatif: f64,
    pub tp_s: f64,
    pub theta_rad: f64,
}

impl Tolerance {
    fn admet(&self, cuite: &Parametres, voulue: &Parametres) -> bool {
        (voulue.hs_m - cuite.hs_m).abs() <= self.hs_relatif * cuite.hs_m
            && (voulue.tp_s - cuite.tp_s).abs() <= self.tp_s
            && (voulue.theta_rad - cuite.theta_rad).abs() <= self.theta_rad
            && voulue.phase_maree == cuite.phase_maree
            && voulue.liquide == cuite.liquide
    }
}

/// **Une graine 1D** : la hauteur totale aux centres et la vitesse aux faces, en f16 ; la maille, la profondeur au repos.
#[derive(Clone, Debug, PartialEq)]
pub struct SeedState {
    pub id: u64,
    pub genre: Genre,
    pub parametres: Parametres,
    pub dx: f64,
    pub profondeur: f64,
    pub h: Vec<u16>,
    pub u: Vec<u16>,
}

fn empreinte(genre: Genre, p: &Parametres, dx: f64, profondeur: f64, h: &[u16], u: &[u16]) -> u64 {
    let mut e = Hasher64::new();
    e.write_u8(genre as u8);
    for v in [p.hs_m, p.tp_s, p.theta_rad, dx, profondeur] {
        e.write_u64(v.to_bits());
    }
    e.write_u8(p.phase_maree);
    e.write_u8(p.liquide);
    for &v in h.iter().chain(u) {
        e.write_u32(v as u32);
    }
    e.finish()
}

/// **Cuire une graine** d'un domaine établi — un hôte de cuisson seulement.
#[cfg(any(test, feature = "cuisson"))]
pub fn condense(d: &Domaine1D, genre: Genre, parametres: Parametres) -> SeedState {
    use crate::cotier::vers_f16;
    let (h0, dx) = (d.profondeur(), d.maille());
    let h: Vec<u16> = d.eta().iter().map(|e| vers_f16((h0 + e) as f32)).collect();
    let u: Vec<u16> = d.u().iter().map(|v| vers_f16(*v as f32)).collect();
    SeedState { id: empreinte(genre, &parametres, dx, h0, &h, &u), genre, parametres, dx, profondeur: h0, h, u }
}

impl SeedState {
    /// L'empreinte recalculée sur le contenu.
    pub fn empreinte(&self) -> u64 {
        empreinte(self.genre, &self.parametres, self.dx, self.profondeur, &self.h, &self.u)
    }

    /// **Restaurer** sous les paramètres voulus : refusé hors tolérance ; la forme renormalisée sur le volume du nœud (ml par mètre de
    /// largeur, autoritaire).
    pub fn restaurer(&self, voulue: &Parametres, tol: &Tolerance, volume_noeud_ml: i64, g: f64, dt: f64) -> Result<Domaine1D, Refus> {
        if !tol.admet(&self.parametres, voulue) || volume_noeud_ml <= 0 || self.h.is_empty() || self.u.len() != self.h.len() + 1 {
            return Err(Refus);
        }
        let h: Vec<f64> = self.h.iter().map(|&v| depuis_f16(v) as f64).collect();
        let v_graine = h.iter().sum::<f64>() * self.dx;
        let facteur = volume_noeud_ml as f64 * 1e-6 / v_graine;
        let eta = h.iter().map(|v| v * facteur - self.profondeur).collect();
        let u = self.u.iter().map(|&v| depuis_f16(v) as f64).collect();
        Domaine1D::depuis_etat(g, self.profondeur, self.dx, dt, eta, u).map_err(|_| Refus)
    }
}

/// **Choisir** la graine la plus proche des paramètres voulus parmi celles que la tolérance admet — l'écart relatif de `hs`, puis de `tp`,
/// puis de `θ` ; aucune si aucune n'est admise. Jamais de mélange de champs (I-09).
pub fn choisir<'a>(graines: &'a [SeedState], voulue: &Parametres, tol: &Tolerance) -> Option<&'a SeedState> {
    let distance = |g: &SeedState| {
        let p = &g.parametres;
        ((voulue.hs_m - p.hs_m) / p.hs_m).abs() + (voulue.tp_s - p.tp_s).abs() / p.tp_s + (voulue.theta_rad - p.theta_rad).abs()
    };
    graines.iter().filter(|g| tol.admet(&g.parametres, voulue)).min_by(|a, b| distance(a).total_cmp(&distance(b)))
}

#[cfg(test)]
#[path = "tests_graine.rs"]
mod tests;
