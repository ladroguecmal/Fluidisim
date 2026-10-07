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
//! Ne fait pas : un état extérieur par rangée (le bord de Saint-Venant prend la moyenne des rangées — exact au repos et sur une côte
//! uniforme) ; le remboursement de la dette (des particules retirées quand elle dépasse un quantum) ; le relais au large (S650).

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
        Ok(RelaisRivage { apic, sv, dette: vec![0.; d.ny], temps: 0., droite: vec![0.; n], gauche: vec![0.; n] })
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
        etat.iter().enumerate().map(|(j, &(haut, su, n))| {
            if n == 0 {
                let x = d.nx as f32 * d.dx;
                (self.apic.smooth_seabed_height(x, (j as f32 + 0.5) * d.dx) as f64, 0.)
            } else {
                (haut as f64 + d.dx as f64 / 4., su / n as f64)
            }
        }).collect()
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
        for j in 0..d.ny {
            let x = d.nx as f32 * d.dx;
            let zb = self.apic.smooth_seabed_height(x, (j as f32 + 0.5) * d.dx) as f64;
            let h = (bord[j].0 - zb).max(1e-3);
            let v = (flux[j] / h) as f32;
            for k in 0..d.nz {
                self.droite[k * d.ny + j] = v;
            }
            if flux[j] < 0. {
                entree[j] = -flux[j] * dt * d.dx as f64;
            }
        }
        self.apic.set_open_boundaries(&self.gauche, &self.droite).map_err(Refus::Apic)?;
        self.apic.feed_right(&entree, [(flux.iter().sum::<f64>() / d.ny as f64 / (eta - self.sv.z[0]).max(1e-3)) as f32, 0., 0.])
            .map_err(Refus::Apic)?;
        self.apic.step(us).map_err(Refus::Apic)?;
        let sorti: Vec<f64> = self.apic.right_outlet().map(|s| s.0.to_vec()).unwrap_or_default();
        for j in 0..d.ny {
            let pris = flux[j].max(0.) * dt * d.dx as f64;
            self.dette[j] += pris - sorti.get(j).copied().unwrap_or(0.);
        }
        self.temps += dt;
        Ok(())
    }
}

#[cfg(test)]
#[path = "tests_relais_rivage.rs"]
mod tests;
