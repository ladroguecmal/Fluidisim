//! **Le relais au rivage** (S684, liste 4.14 ; [ADR-271](../../../docs/adr/ADR-271-le-film-du-rivage-a-saint-venant.md)) : APIC 3D tient
//! la bande où la surface se retourne, Saint-Venant 2D le film du rivage, côte à côte en `x`, à la même maille et au même nombre de
//! rangées `j`. À chaque pas :
//!
//! 1. l'état du bord 3D — le niveau `η` (la plus haute particule de la dernière colonne, plus `dx/4`) et la vitesse `u` moyenne de ses
//!    particules — nourrit le bord gauche caractéristique de Saint-Venant (`h_e = η − z` de sa première maille) ;
//! 2. Saint-Venant fait son pas et rend son flux de masse par rangée (S680) ;
//! 3. APIC reçoit ce flux comme vitesse normale de son bord droit, fait entrer le reflux (S683) et fait son pas ;
//! 4. ce qu'APIC a laissé sortir (S682) contre ce que Saint-Venant a pris devient une **dette** par rangée.
//!
//! La masse se compte : Saint-Venant + particules × quantum + réservoir − dette ([`RelaisRivage::volume`]).
//!
//! **S689 — la dette remboursée au quantum** : quand elle atteint un quantum, APIC rend la particule de sa dernière colonne la plus proche
//! du bord ; sous moins un quantum, Saint-Venant reçoit ce volume dans sa première maille. Elle reste sous un quantum par rangée.
//!
//! Ne fait pas : un état extérieur par rangée (le bord de Saint-Venant prend la moyenne des rangées — exact au repos et sur une côte
//! uniforme) ; le relais au large (S650).

use crate::apic3d::Apic3;
use crate::delta_projection::Error as ErreurApic;
use crate::saint_venant_2d::SaintVenant2D;

/// Pourquoi le relais refuse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refus {
    /// Des mailles ou des rangées différentes, Saint-Venant sans l'ordre deux, APIC sans sa sortie à droite.
    Montage,
    /// APIC a refusé son pas.
    Apic(ErreurApic),
    /// Saint-Venant a refusé son pas (Courant au-delà de ½).
    SaintVenant,
}

/// Les deux solveurs côte à côte.
pub struct RelaisRivage {
    pub apic: Apic3,
    pub sv: SaintVenant2D,
    /// Par rangée, le volume que Saint-Venant a pris de plus qu'APIC n'a laissé sortir (m³).
    pub dette: Vec<f64>,
    /// S689 — les remboursements : les particules rendues par APIC, les quanta ajoutés à Saint-Venant.
    pub rendues: u64,
    pub ajoutes: u64,
    /// S690 — le relevé du dernier pas : `[η du bord 3D (moyenne), h_e de Saint-Venant, u du bord, flux moyen, la plus grande vitesse
    /// imposée au bord droit d'APIC, la vitesse des particules posées]`.
    pub releve: [f64; 6],
    temps: f64,
    droite: Vec<f32>,
    gauche: Vec<f32>,
}

impl RelaisRivage {
    /// Le relais : APIC avec ses bords ouverts et sa sortie à droite (`enable_right_outlet`), Saint-Venant à l'ordre deux, la même maille
    /// et le même nombre de rangées. Saint-Venant commence au bord droit d'APIC.
    pub fn nouveau(apic: Apic3, sv: SaintVenant2D) -> Result<RelaisRivage, Refus> {
        let d = apic.domain();
        if sv.ny != d.ny || ((sv.dx - d.dx as f64).abs() > 1e-6 * sv.dx) || apic.right_outlet().is_none()
            || sv.flux_des_bords().0.len() != d.ny {
            return Err(Refus::Montage);
        }
        let n = d.ny * d.nz;
        Ok(RelaisRivage { apic, sv, dette: vec![0.; d.ny], rendues: 0, ajoutes: 0, releve: [0.; 6], temps: 0., droite: vec![0.; n], gauche: vec![0.; n] })
    }

    /// Le quantum d'une particule, m³.
    pub fn quantum(&self) -> f64 {
        (self.apic.domain().dx as f64).powi(3) / 8.
    }

    /// **La masse** : Saint-Venant + particules × quantum + réservoir − dette (m³).
    pub fn volume(&self) -> f64 {
        let reservoir: f64 = self.apic.right_inlet().map_or(0., |r| r.0.iter().sum());
        self.sv.volume() + self.apic.particle_count() as f64 * self.quantum() + reservoir - self.dette.iter().sum::<f64>()
    }

    /// S690 — le fond du bord droit de la 3D, rangée `j` : le fond lisse au bord, sinon le dessus de l'escalier de la dernière colonne.
    fn fond_bord(&self, j: usize) -> f64 {
        let d = self.apic.domain();
        if self.apic.face_fractions().is_some() {
            self.apic.smooth_seabed_height(d.nx as f32 * d.dx, (j as f32 + 0.5) * d.dx) as f64
        } else {
            self.apic.seabed_height(d.nx - 1, j) as f64
        }
    }

    /// L'état du bord 3D par rangée : le niveau (la plus haute particule de la dernière colonne + `dx/4`, le fond sans particule) et la
    /// vitesse `u` moyenne de ses particules.
    pub fn bord_3d(&self) -> Vec<(f64, f64)> {
        let d = self.apic.domain();
        let x0 = (d.nx - 1) as f32 * d.dx;
        let mut etat = vec![(f32::MIN, 0f64, 0usize); d.ny];
        for k in 0..self.apic.n {
            let p = self.apic.x[k];
            if p[0] >= x0 {
                let j = ((p[1] / d.dx) as usize).min(d.ny - 1);
                etat[j].0 = etat[j].0.max(p[2]);
                etat[j].1 += self.apic.vel[k][0] as f64;
                etat[j].2 += 1;
            }
        }
        let q = self.quantum();
        etat.iter().enumerate().map(|(j, &(haut, su, n))| {
            let zb = self.fond_bord(j);
            if n == 0 {
                (zb, 0.)
            } else {
                // S690 : borné par le volume de la colonne, plus une couche — une éclaboussure ne soulève plus le niveau.
                let par_volume = zb + n as f64 * q / (d.dx as f64 * d.dx as f64) + d.dx as f64 / 2.;
                ((haut as f64 + d.dx as f64 / 4.).min(par_volume), su / n as f64)
            }
        }).collect()
    }

    /// **S690 — le pas stable du relais** (µs) : le plus petit du pas stable d'APIC (plafonné à `plafond_us`) et de celui de Saint-Venant,
    /// Courant 0,4 sur la célérité réelle `|u| + √(g·h)` de ses mailles (au lieu d'une borne fixe, deux à quatre fois plus courte).
    pub fn pas_stable_us(&self, plafond_us: u64) -> u64 {
        let sv = &self.sv;
        let mut c = 0f64;
        for k in 0..sv.nx * sv.ny {
            let h = sv.h[k];
            if h > 1e-6 {
                c = c.max((sv.qx[k] / h).abs().max((sv.qy[k] / h).abs()) + (sv.g * h).sqrt());
            }
        }
        let p_sv = if c > 0. { (0.4 * sv.dx / c * 1e6) as u64 } else { plafond_us };
        self.apic.stable_step_us(plafond_us).min(p_sv).max(1)
    }

    /// **Un pas** de `us` microsecondes (le plus petit des deux pas stables, à l'appelant).
    pub fn pas(&mut self, us: u64) -> Result<(), Refus> {
        let d = self.apic.domain();
        let dt = us as f64 * 1e-6;
        let bord = self.bord_3d();
        let (eta, u) = bord.iter().fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1));
        let (eta, u) = (eta / d.ny as f64, u / d.ny as f64);
        // La première maille de Saint-Venant, rangée 0 (la côte uniforme le long de ses bords).
        let he = (eta - self.sv.z[0]).max(0.);
        let ext = move |_t: f64| (he, u);
        self.sv.pas_avec_bords(dt, self.temps, Some(&ext), None).map_err(|_| Refus::SaintVenant)?;
        let flux: Vec<f64> = self.sv.flux_des_bords().0.to_vec();
        // Le bord droit d'APIC : la vitesse normale `F/h` (sortante positive), le reflux posé.
        let mut entree = vec![0.; d.ny];
        let mut vmax = 0f64;
        for j in 0..d.ny {
            let zb = self.fond_bord(j);
            // S690 : le plancher d'un quart de maille, la vitesse bornée par la célérité.
            let h = (bord[j].0 - zb).max(d.dx as f64 / 4.);
            let borne = bord[j].1.abs() + 2. * (self.sv.g * h).sqrt();
            let v = (flux[j] / h).clamp(-borne, borne) as f32;
            vmax = vmax.max((v as f64).abs());
            for k in 0..d.nz {
                self.droite[k * d.ny + j] = v;
            }
            if flux[j] < 0. {
                entree[j] = -flux[j] * dt * d.dx as f64;
            }
        }
        self.apic.set_open_boundaries(&self.gauche, &self.droite).map_err(Refus::Apic)?;
        let h_moy = (eta - self.sv.z[0]).max(d.dx as f64 / 4.);
        let borne = u.abs() + 2. * (self.sv.g * h_moy).sqrt();
        let v_posee = (flux.iter().sum::<f64>() / d.ny as f64 / h_moy).clamp(-borne, borne);
        self.releve = [eta, he, u, flux.iter().sum::<f64>() / d.ny as f64, vmax, v_posee];
        self.apic.feed_right(&entree, [v_posee as f32, 0., 0.]).map_err(Refus::Apic)?;
        self.apic.step(us).map_err(Refus::Apic)?;
        let sorti: Vec<f64> = self.apic.right_outlet().map(|s| s.0.to_vec()).unwrap_or_default();
        let q = self.quantum();
        for j in 0..d.ny {
            let pris = flux[j].max(0.) * dt * d.dx as f64;
            self.dette[j] += pris - sorti.get(j).copied().unwrap_or(0.);
            // S689 : le remboursement, au quantum.
            while self.dette[j] >= q && self.apic.take_right(j) {
                self.dette[j] -= q;
                self.rendues += 1;
            }
            while self.dette[j] <= -q {
                self.sv.h[j] += q / (self.sv.dx * self.sv.dx);
                self.dette[j] += q;
                self.ajoutes += 1;
            }
        }
        self.temps += dt;
        Ok(())
    }
}

#[cfg(test)]
#[path = "tests_relais_rivage.rs"]
mod tests;
