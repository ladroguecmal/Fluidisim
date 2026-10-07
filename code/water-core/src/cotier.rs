//! **La bibliothèque côtière** (S599, listes 12.3 et 2.8 ; SPEC-005 §6 ; ADR-022 §3).
//!
//! Une zone de déferlement met des dizaines de secondes à s'établir (ADR-013 §4) : on la **cuit**. Par plage, pour chaque état de mer et
//! chaque phase de marée, un `CoastalState` : une **condition initiale 2D** au pas de la grille (hauteur de houle, `u`, `v`, intensité du
//! rouleau) en **`f16`**, et la polyligne de déferlement (S588). La hauteur hors de la zone de déferlement vient de la référence de B
//! (`bathymetrie::transformer`, la levée) ; dedans, elle est saturée à `0,78·h` ; le rouleau est la dissipation `d(E·c_g)/dx`. `u`, `v`
//! restent **nuls** tant que le courant de dérive littorale n'est pas calculé. **L'empreinte** (FNV-1a) des entrées dit quand une plage est
//! obsolète. **La recherche se fait par les paramètres** (I-09) : l'état le plus proche en `(Hs, phase)`, jamais un mélange de champs.
//!
//! Une donnée de cuisson : elle s'alloue hors de l'exécution (I-06 ne s'applique qu'au pas).

use crate::bathymetrie::{transformer, MCCOWAN};
use crate::deferlement::{polyligne, Houle, Sommet};
use crate::hash::Hasher64;

/// Une entrée refusée : grille vide ou mal dimensionnée, pas non positif, aucun état, période non positive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refus;

/// **f32 → f16** (binary16), arrondi au plus proche pair ; les valeurs hors de l'étendue saturent à l'infini.
pub fn vers_f16(x: f32) -> u16 {
    let b = x.to_bits();
    let signe = ((b >> 16) & 0x8000) as u16;
    let exp = ((b >> 23) & 0xff) as i32;
    let mant = b & 0x7f_ffff;
    if exp == 0xff {
        return signe | 0x7c00 | if mant != 0 { 0x200 } else { 0 };
    }
    let e = exp - 127 + 15;
    if e >= 0x1f {
        return signe | 0x7c00;
    }
    if e <= 0 {
        if e < -10 {
            return signe;
        }
        let m = mant | 0x80_0000;
        let decale = (14 - e) as u32;
        let q = m >> decale;
        let reste = m & ((1 << decale) - 1);
        let moitie = 1 << (decale - 1);
        let q = if reste > moitie || (reste == moitie && (q & 1) == 1) { q + 1 } else { q };
        return signe | q as u16;
    }
    let q = mant >> 13;
    let reste = mant & 0x1fff;
    let mut h = ((e as u32) << 10) | q;
    if reste > 0x1000 || (reste == 0x1000 && (q & 1) == 1) {
        h += 1;
    }
    signe | h as u16
}

/// **f16 → f32**, exact.
pub fn depuis_f16(h: u16) -> f32 {
    let signe = ((h & 0x8000) as u32) << 16;
    let exp = ((h >> 10) & 0x1f) as u32;
    let mant = (h & 0x3ff) as u32;
    let bits = if exp == 0 {
        if mant == 0 {
            signe
        } else {
            return f32::from_bits(signe) + (mant as f32) * 2f32.powi(-24) * if signe != 0 { -1.0 } else { 1.0 };
        }
    } else if exp == 0x1f {
        signe | 0x7f80_0000 | (mant << 13)
    } else {
        signe | ((exp + 127 - 15) << 23) | (mant << 13)
    };
    f32::from_bits(bits)
}

/// Un état cuit : ses paramètres, son champ 2D (`nx × ny` texels × [hauteur, u, v, rouleau] en `f16`), sa polyligne.
#[derive(Clone, Debug, PartialEq)]
pub struct CoastalState {
    pub hs_m: f32,
    pub phase: f32,
    pub champ: Vec<u16>,
    pub polyligne: Vec<Sommet>,
}

/// La description d'une plage à cuire.
pub struct Plage<'a> {
    /// La profondeur au centre de chaque texel au niveau moyen (m ; négative : la terre), `nx × ny`, `x` le plus rapide.
    pub fond: &'a [f64],
    pub nx: usize,
    pub ny: usize,
    pub pas_m: f64,
    pub origine: [f64; 2],
    pub periode_s: f64,
    pub maree_m: f64,
}

/// **La bibliothèque d'une plage** : ses états, ses phases, son empreinte.
pub struct Bibliotheque {
    pub etats: Vec<CoastalState>,
    pub hs: Vec<f32>,
    pub phases: Vec<f32>,
    pub empreinte: u64,
}

/// L'empreinte des entrées de la cuisson.
fn empreinte(plage: &Plage<'_>, hs: &[f32], phases: &[f32]) -> u64 {
    let mut h = Hasher64::new();
    h.write_u64(plage.nx as u64);
    h.write_u64(plage.ny as u64);
    h.write_u64(plage.pas_m.to_bits());
    h.write_u64(plage.origine[0].to_bits());
    h.write_u64(plage.origine[1].to_bits());
    h.write_u64(plage.periode_s.to_bits());
    h.write_u64(plage.maree_m.to_bits());
    for f in plage.fond {
        h.write_u64(f.to_bits());
    }
    for x in hs.iter().chain(phases) {
        h.write_u32(x.to_bits());
    }
    h.finish()
}

/// **Cuire** une plage pour les états de mer `hs` (m) et les phases de marée `phases` (tours ; la marée `maree_m·sin(2π·phase)`).
pub fn cuire(plage: &Plage<'_>, hs: &[f32], phases: &[f32], g: f64, rho: f64) -> Result<Bibliotheque, Refus> {
    let (nx, ny) = (plage.nx, plage.ny);
    if nx < 2 || ny < 1 || plage.fond.len() != nx * ny || !(plage.pas_m > 0.0) || hs.is_empty() || phases.is_empty()
        || !(plage.periode_s > 0.0) || hs.iter().any(|h| !(*h > 0.0)) {
        return Err(Refus);
    }
    let omega = core::f64::consts::TAU / plage.periode_s;
    let mut etats = Vec::with_capacity(hs.len() * phases.len());
    for &h0 in hs {
        for &phase in phases {
            let eta = plage.maree_m * (core::f64::consts::TAU * phase as f64).sin();
            let profondeur: Vec<f64> = plage.fond.iter().map(|f| f + eta).collect();
            let hauteur = |h: f64| -> f64 {
                if h <= 0.0 {
                    return 0.0;
                }
                let lineaire = transformer(omega, 0.0, 0.5 * h0 as f64, h, g).map_or(MCCOWAN * h, |s| 2.0 * s.amplitude);
                lineaire.min(MCCOWAN * h)
            };
            let flux = |h: f64| -> f64 {
                let hh = hauteur(h);
                if h <= 0.0 || hh == 0.0 {
                    return 0.0;
                }
                let cg = transformer(omega, 0.0, 0.5, h, g).map_or(0.0, |s| s.cg);
                rho * g * hh * hh / 8.0 * cg
            };
            let mut champ = vec![0u16; nx * ny * 4];
            for j in 0..ny {
                for i in 0..nx {
                    let c = j * nx + i;
                    let h = profondeur[c];
                    let hh = hauteur(h);
                    let deferle = h > 0.0 && transformer(omega, 0.0, 0.5 * h0 as f64, h, g).map_or(true, |s| 2.0 * s.amplitude >= MCCOWAN * h);
                    // Le rouleau : la dissipation d(E·c_g)/dx dans la zone de déferlement (la houle va vers −x), nulle au large.
                    let rouleau = if deferle && i + 1 < nx && i > 0 {
                        ((flux(profondeur[c + 1]) - flux(profondeur[c - 1])) / (2.0 * plage.pas_m)).max(0.0)
                    } else {
                        0.0
                    };
                    champ[4 * c] = vers_f16(hh as f32);
                    champ[4 * c + 1] = vers_f16(0.0);
                    champ[4 * c + 2] = vers_f16(0.0);
                    champ[4 * c + 3] = vers_f16(rouleau as f32);
                }
            }
            let mut ecart = vec![0.0; nx * ny];
            let mut sommets = vec![Sommet::default(); ny];
            let n = polyligne(plage.origine, plage.pas_m, nx, ny, &profondeur, Houle { omega, theta0: 0.0, hauteur0: h0 as f64 }, g, rho,
                &mut ecart, &mut sommets).map_err(|_| Refus)?;
            sommets.truncate(n);
            etats.push(CoastalState { hs_m: h0, phase, champ, polyligne: sommets });
        }
    }
    Ok(Bibliotheque { etats, hs: hs.to_vec(), phases: phases.to_vec(), empreinte: empreinte(plage, hs, phases) })
}

impl Bibliotheque {
    /// La taille des champs cuits, octets.
    pub fn octets(&self) -> usize {
        self.etats.iter().map(|e| e.champ.len() * 2).sum()
    }

    /// **La recherche par paramètres** (I-09) : l'état le plus proche en Hs, et en phase repliée sur un tour.
    pub fn recherche(&self, hs: f32, phase: f32) -> &CoastalState {
        let i = (0..self.hs.len()).min_by(|&a, &b| (self.hs[a] - hs).abs().total_cmp(&(self.hs[b] - hs).abs())).unwrap();
        let tour = |a: f32| { let d = (a - phase).rem_euclid(1.0); d.min(1.0 - d) };
        let k = (0..self.phases.len()).min_by(|&a, &b| tour(self.phases[a]).total_cmp(&tour(self.phases[b]))).unwrap();
        &self.etats[i * self.phases.len() + k]
    }

    /// La bibliothèque est-elle à jour pour cette plage ?
    pub fn a_jour(&self, plage: &Plage<'_>) -> bool {
        self.empreinte == empreinte(plage, &self.hs, &self.phases)
    }
}

#[cfg(test)]
#[path = "tests_cotier.rs"]
mod tests;
