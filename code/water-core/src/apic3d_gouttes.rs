//! **S488 (K2-4, ADR-014 §4) — la nappe rompue en gouttes.**
//!
//! Sans tension de surface ni viscosité, rien n'arrête l'amincissement d'une nappe d'APIC : la couronne et le jet d'un impact dépendent
//! de la maille (A312). Ici, une particule d'eau dont la maille **et ses six voisines** sont sans eau — une nappe plus mince qu'une maille,
//! que la reconstruction ne voit plus, détachée du corps de l'eau — et dont la vitesse donne `We = ρ·v²·d/σ > 12` (ADR-014 §4 ; `d` le diamètre d'une goutte du volume de la
//! particule, `dx/2·(6/π)^(1/3)`) **devient une goutte** :
//!
//! - **hors de la grille** : ni transfert vers la grille, ni reconstruction, ni séparation ;
//! - **balistique** : la gravité `g_eff` et la traînée de l'air, `a = −(3/4)·(ρ_air/ρ)·(C_d/d)·|v|·v` (`C_d` = 0,47, ρ_air = 1,2 kg/m³),
//!   intégrées semi-implicitement (`v ← (v + g·dt)/(1 + k·|v|·dt)`, stable à tout pas) ;
//! - elle **redevient de l'eau** en entrant dans une maille d'eau, sa vitesse transmise par le transfert suivant.
//!
//! Aucune particule ne naît ni ne meurt : la masse est exacte par construction. Sans `enable_droplets`, le pas d'avant, au bit. Refusé
//! avec une zone de colonnes (l'échange y retire et pose des particules ; K2-5).

use super::*;

/// Le nombre de Weber au-delà duquel une nappe se fragmente (ADR-014 §4).
pub const WEBER_RUPTURE: f64 = 12.;
/// La tension de surface de l'eau, N/m.
pub const SIGMA_EAU: f64 = 0.072;
/// La masse volumique de l'air, kg/m³, et le coefficient de traînée d'une sphère.
pub const RHO_AIR: f64 = 1.2;
pub const CD_GOUTTE: f64 = 0.47;

pub(crate) struct Gouttes {
    /// 1 : la particule est une goutte.
    pub(crate) est: Vec<u8>,
    /// Gouttes nées et retombées depuis la mise en route.
    pub(crate) nees: u64,
    pub(crate) retombees: u64,
}

/// Le diamètre d'une goutte du volume d'une particule (`dx³/8`), m.
pub fn droplet_diameter(dx: f64) -> f64 {
    (6. / core::f64::consts::PI * dx * dx * dx / 8.).cbrt()
}

/// **Un pas balistique** d'une goutte : la vitesse, puis la position (semi-implicite pour la traînée). `g` : l'accélération de la
/// pesanteur (vecteur, m/s²) ; `d` : le diamètre (m) ; `rho` : la masse volumique de l'eau.
pub fn ballistic_step(x: [f32; 3], v: [f32; 3], g: [f32; 3], d: f64, rho: f64, dt: f32) -> ([f32; 3], [f32; 3]) {
    let k = (0.75 * RHO_AIR / rho * CD_GOUTTE / d) as f32;
    let w = [v[0] + g[0] * dt, v[1] + g[1] * dt, v[2] + g[2] * dt];
    let s = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
    let f = 1. / (1. + k * s * dt);
    let vn = [w[0] * f, w[1] * f, w[2] * f];
    ([x[0] + vn[0] * dt, x[1] + vn[1] * dt, x[2] + vn[2] * dt], vn)
}

impl Apic3 {
    /// **S488 — les gouttes** (K2-4) : à l'initialisation (I-06), un octet par particule. Refus : déjà actives, ou une zone de colonnes.
    pub fn enable_droplets(&mut self, host: &mut HostServices) -> Result<(), Error> {
        if self.gouttes.is_some() || self.columns.is_some() || self.sortie_droite.is_some() {
            return Err(Error::Domain);
        }
        let cap = self.x.len();
        host.alloc.alloc_persistent(cap).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        self.gouttes = Some(Box::new(Gouttes { est: vec![0; cap], nees: 0, retombees: 0 }));
        Ok(())
    }

    /// Les gouttes du moment, et celles nées et retombées depuis la mise en route.
    pub fn droplet_counts(&self) -> (usize, u64, u64) {
        self.gouttes.as_ref().map_or((0, 0, 0), |g| (g.est[..self.n].iter().filter(|e| **e != 0).count(), g.nees, g.retombees))
    }

    /// La particule `k` est-elle une goutte ?
    #[inline]
    pub(crate) fn is_droplet(&self, k: usize) -> bool {
        self.gouttes.as_ref().is_some_and(|g| g.est[k] != 0)
    }

    /// **Après les étiquettes** : une particule d'eau dans une maille d'air, assez rapide, devient goutte ; une goutte dans une maille
    /// d'eau redevient de l'eau.
    pub(crate) fn droplets_classify(&mut self) {
        let Some(mut g) = self.gouttes.take() else { return };
        let dx = self.domain.dx as f64;
        let d = droplet_diameter(dx);
        let v2_min = (WEBER_RUPTURE * SIGMA_EAU / (self.rho as f64 * d)) as f32;
        for k in 0..self.n {
            let (i, j, l) = self.cell_of(self.x[k]);
            let lab = self.label[self.cell(i, j, l)];
            let v = self.vel[k];
            if g.est[k] == 0 {
                // Une nappe détachée : sa maille et ses six voisines sans eau (une particule de surface a de l'eau à côté — S488 :
                // la seule maille d'air faisait de chaque surface en mouvement une pluie de gouttes, et changeait la cavité de B10).
                let detachee = lab == AIR && self.neighbours(i, j, l).into_iter().flatten().all(|(m, _, _)| self.label[m] != WATER);
                if detachee && v[0] * v[0] + v[1] * v[1] + v[2] * v[2] > v2_min {
                    g.est[k] = 1;
                    g.nees += 1;
                }
            } else if lab == WATER {
                g.est[k] = 0;
                g.retombees += 1;
            }
        }
        self.gouttes = Some(g);
    }

    /// La gravité du pas, en vecteur (le cœur suit `−z`, comme le reste d'APIC ; ADR-002 : `g_eff` injectée).
    pub(crate) fn droplet_gravity(&self) -> [f32; 3] {
        [0., 0., -self.g_eff]
    }
}
