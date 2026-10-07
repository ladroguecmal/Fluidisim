//! **Le précalcul avant l'impact** (S610, liste 9.6 ; ADR-013 §3–4).
//!
//! En palier T2, δ vaut identiquement 0 : un domaine préparé ne porte que des blocs alloués — aucune information physique. Un déplacement
//! du point d'impact prévu se corrige en translatant l'ensemble (ré-indexer), sans erreur ; les seuls seuils réels sont réallouer (au-delà
//! de la capacité réservée), rebâtir (un autre `dx`), libérer (l'événement n'aura pas lieu). Aucun seuil de tolérance physique.
//!
//! L'avance temporelle n'est légitime que pour un domaine **substitutif**, qui naît faux et doit s'établir : ADR-013 §4 l'estime entre
//! `L/c_g` et `L/c_g + 2T`. [`etablissement`] le mesure sur le domaine de S609 né au repos.
//!
//! Ne fait pas : les collisions et proxys, l'avance d'un domaine 3D mesurée en temps réel, la graine qui remplace l'établissement.

use crate::substitutif::{Domaine1D, Exterieur};

/// Une entrée refusée : rayon ou `dx` non positifs, une capacité sous l'ensemble initial.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// Les mailles par côté d'un bloc (ADR-006 §3 : 8³ ; une colonne de 8 × 8 ici, ADR-175).
pub const MAILLES_PAR_BLOC: f64 = 8.0;

/// La décision d'une révision de la prévision (ADR-013 §3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    /// Un déplacement d'un nombre entier de blocs : l'ensemble ré-indexé.
    Translater { di: i64, dj: i64 },
    /// Un déplacement hors réseau : l'ensemble rebâti dans la capacité réservée.
    Rebatir,
    /// L'ensemble rebâti dépasse la capacité : réserver `blocs`.
    Reallouer { blocs: usize },
    /// Le `dx` requis a changé : rebâtir au nouveau niveau.
    RebatirDx,
    /// L'événement n'aura pas lieu.
    Liberer,
}

/// **Un domaine préparé** en T2 : les colonnes de blocs du réseau du référentiel autour de l'impact prévu, la capacité réservée.
#[derive(Clone, Debug, PartialEq)]
pub struct Preparation {
    pub impact: [f64; 2],
    pub rayon_m: f64,
    pub dx: f64,
    pub capacite: usize,
    pub blocs: Vec<[i64; 2]>,
}

/// Les blocs dont le centre est à moins de `rayon + s·√2/2` de `p` (`s` le côté d'un bloc), triés.
fn ensemble(p: [f64; 2], rayon: f64, s: f64) -> Vec<[i64; 2]> {
    let lo = [((p[0] - rayon) / s).floor() as i64 - 1, ((p[1] - rayon) / s).floor() as i64 - 1];
    let hi = [((p[0] + rayon) / s).floor() as i64 + 1, ((p[1] + rayon) / s).floor() as i64 + 1];
    let mut v = Vec::new();
    for i in lo[0]..=hi[0] {
        for j in lo[1]..=hi[1] {
            if ((i as f64 + 0.5) * s - p[0]).hypot((j as f64 + 0.5) * s - p[1]) <= rayon + s * core::f64::consts::SQRT_2 / 2.0 {
                v.push([i, j]);
            }
        }
    }
    v
}

impl Preparation {
    pub fn nouvelle(impact: [f64; 2], rayon_m: f64, dx: f64, capacite: usize) -> Result<Preparation, Refus> {
        if !(rayon_m > 0.0) || !(dx > 0.0) || !rayon_m.is_finite() || !dx.is_finite() || impact.iter().any(|v| !v.is_finite()) {
            return Err(Refus);
        }
        let blocs = ensemble(impact, rayon_m, MAILLES_PAR_BLOC * dx);
        if blocs.len() > capacite {
            return Err(Refus);
        }
        Ok(Preparation { impact, rayon_m, dx, capacite, blocs })
    }

    /// L'état initial d'une colonne : δ = 0 — le domaine préparé ne porte rien d'autre.
    pub fn delta_initial(&self) -> impl Iterator<Item = f64> + '_ {
        self.blocs.iter().flat_map(|_| core::iter::repeat_n(0.0, (MAILLES_PAR_BLOC * MAILLES_PAR_BLOC) as usize))
    }

    /// **Réviser** sur une nouvelle prévision `(impact, dx)` — `None` : l'événement n'aura pas lieu. La préparation est mise à jour.
    pub fn reviser(&mut self, prevision: Option<([f64; 2], f64)>) -> Decision {
        let Some((p, dx)) = prevision else {
            self.blocs.clear();
            return Decision::Liberer;
        };
        if dx != self.dx {
            *self = Preparation { impact: p, dx, blocs: ensemble(p, self.rayon_m, MAILLES_PAR_BLOC * dx), ..*self };
            self.capacite = self.capacite.max(self.blocs.len());
            return Decision::RebatirDx;
        }
        let s = MAILLES_PAR_BLOC * self.dx;
        let d = [(p[0] - self.impact[0]) / s, (p[1] - self.impact[1]) / s];
        self.impact = p;
        if d[0].fract() == 0.0 && d[1].fract() == 0.0 {
            let (di, dj) = (d[0] as i64, d[1] as i64);
            for b in &mut self.blocs {
                *b = [b[0] + di, b[1] + dj];
            }
            return Decision::Translater { di, dj };
        }
        self.blocs = ensemble(p, self.rayon_m, s);
        if self.blocs.len() <= self.capacite {
            Decision::Rebatir
        } else {
            self.capacite = self.blocs.len();
            Decision::Reallouer { blocs: self.blocs.len() }
        }
    }
}

/// **Le temps d'établissement** d'un domaine substitutif (s) : le premier instant après lequel l'écart à B reste sous `tol` jusqu'à
/// `pas` pas ; `None` s'il n'y est pas au dernier pas.
pub fn etablissement(d: &mut Domaine1D, b: Exterieur<'_>, tol: f64, pas: usize, dx: f64) -> Option<f64> {
    let dt = d.temps_pas();
    let mut dernier = None;
    for n in 0..pas {
        d.avancer(b);
        let t = d.temps();
        let e = d.eta().iter().enumerate().map(|(i, v)| (v - b((i as f64 + 0.5) * dx, t).0).abs()).fold(0.0, f64::max);
        if e > tol {
            dernier = Some(n);
        }
    }
    match dernier {
        Some(n) if n + 1 == pas => None,
        Some(n) => Some((n + 2) as f64 * dt),
        None => Some(dt),
    }
}

#[cfg(test)]
#[path = "tests_precalcul.rs"]
mod tests;
