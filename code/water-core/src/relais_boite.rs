//! **S725 — le raccord de la boîte** (LOD-ETAPE-3-S722, B3) : une boîte d'APIC 3D, ouverte sur ses quatre côtés (S724), tient un trou de
//! Saint-Venant 2D (S723), à la même maille, ses colonnes sur les mailles du trou. À chaque pas, pour chaque face du trou :
//!
//! 1. l'état de la colonne 3D voisine — son niveau par la surface reconstruite (S708), ses vitesses par la moyenne de ses particules ;
//! 2. le **flux complet** de Rusanov entre la maille active de Saint-Venant et cette colonne (S723 : la masse seule réfléchissait), donné
//!    à Saint-Venant ;
//! 3. sa masse devient la vitesse normale du bord d'APIC (`F₀/h`, bornée par la célérité), et l'eau qui entre dans la boîte est posée
//!    (la grille à gauche et en y, S702, S724 ; le réseau à droite, S683) ;
//! 4. APIC fait son pas ;
//! 5. ce que Saint-Venant a cédé par la face, contre ce qu'APIC a reçu et rendu par elle : l'écart va à la maille active, si bien que la
//!    masse est exacte par construction.
//!
//! La masse se compte : Saint-Venant (hors du trou) + particules × quantum + les réservoirs ([`RelaisBoite::volume`]).
//!
//! Ne fait pas : un fond qui n'est pas plat ; un corps (B4) ; le déclencheur (B5).

use crate::apic3d::Apic3;
use crate::relais_rivage::Refus;
use crate::saint_venant_2d::{flux_rusanov, SaintVenant2D};

/// La boîte et Saint-Venant troué.
pub struct RelaisBoite {
    pub apic: Apic3,
    pub sv: SaintVenant2D,
    /// L'origine du trou dans Saint-Venant (la maille de la colonne 3D `(0, 0)`).
    pub i0: usize,
    pub j0: usize,
    /// S725 — le volume rendu à Saint-Venant pour garder la masse exacte, au total (m³, en valeur absolue).
    pub correction: f64,
}

impl RelaisBoite {
    /// Les deux solveurs ; le trou de Saint-Venant doit couvrir exactement les colonnes d'APIC, à la même maille, et APIC avoir ses quatre
    /// bords par particules (`enable_open_boundaries`, `enable_left_inlet`, `enable_right_outlet`, `enable_y_boundaries`).
    pub fn nouveau(apic: Apic3, mut sv: SaintVenant2D, i0: usize, j0: usize) -> Result<Self, Refus> {
        let d = apic.domain();
        if (d.dx as f64 - sv.dx).abs() > 1e-6 * sv.dx || apic.left_inlet().is_none() || apic.right_outlet().is_none() || apic.y_boundaries().is_none() {
            return Err(Refus::Montage);
        }
        sv.regler_trou(i0, i0 + d.nx, j0, j0 + d.ny).map_err(|_| Refus::Montage)?;
        Ok(RelaisBoite { apic, sv, i0, j0, correction: 0. })
    }

    /// Le quantum d'APIC (m³).
    pub fn quantum(&self) -> f64 {
        (self.apic.domain().dx as f64).powi(3) / 8.
    }

    /// La masse : Saint-Venant hors du trou + particules × quantum + les réservoirs (m³).
    pub fn volume(&self) -> f64 {
        let d = self.apic.domain();
        let (ny_sv, dx) = (self.sv.ny, self.sv.dx);
        let mut v = 0.;
        for i in 0..self.sv.nx {
            for j in 0..ny_sv {
                let dans = (self.i0..self.i0 + d.nx).contains(&i) && (self.j0..self.j0 + d.ny).contains(&j);
                if !dans {
                    v += self.sv.h[i * ny_sv + j];
                }
            }
        }
        v *= dx * dx;
        v += self.apic.particle_count() as f64 * self.quantum();
        v += self.apic.left_inlet().map_or(0., |g| g.3.iter().sum::<f64>());
        v += self.apic.right_inlet().map_or(0., |r| r.0.iter().sum::<f64>());
        v += self.apic.y_boundaries().map_or(0., |b| b.5);
        v
    }

    /// Le pas stable (µs) sous `plafond_us` : APIC, et Saint-Venant à Courant 0,4.
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

    /// **Un pas** de `us` microsecondes.
    pub fn pas(&mut self, us: u64) -> Result<(), Refus> {
        let d = self.apic.domain();
        let (nx, ny, nz) = (d.nx, d.ny, d.nz);
        let (dx, dt, g) = (d.dx as f64, us as f64 * 1e-6, self.sv.g);
        let nsv = self.sv.ny;
        // 1. L'état des colonnes 3D : le niveau par la surface reconstruite, les vitesses moyennes des particules.
        let (phi, l) = (self.apic.distance(), self.apic.labels());
        let mut hc = vec![0f64; nx * ny];
        for i in 0..nx {
            for j in 0..ny {
                let mut e = 0.;
                for k in 0..nz {
                    let m = (k * ny + j) * nx + i;
                    if l[m] != crate::apic3d::SOLID {
                        e += (0.5 - phi[m] as f64 / dx).clamp(0., 1.) * dx;
                    }
                }
                hc[j * nx + i] = e;
            }
        }
        let (mut su, mut sv_, mut n) = (vec![0f64; nx * ny], vec![0f64; nx * ny], vec![0usize; nx * ny]);
        for (p, v) in self.apic.particles().iter().zip(self.apic.velocities()) {
            let c = ((p[1] / d.dx).max(0.) as usize).min(ny - 1) * nx + ((p[0] / d.dx).max(0.) as usize).min(nx - 1);
            su[c] += v[0] as f64;
            sv_[c] += v[1] as f64;
            n[c] += 1;
        }
        let col = |i: usize, j: usize| {
            let c = j * nx + i;
            let m = n[c].max(1) as f64;
            (hc[c], su[c] / m, sv_[c] / m)
        };
        let svc = |sv: &SaintVenant2D, i: usize, j: usize| {
            let k = i * nsv + j;
            let h = sv.h[k];
            let (u, v) = if h > 1e-6 { (sv.qx[k] / h, sv.qy[k] / h) } else { (0., 0.) };
            (h, u, v)
        };
        // 2. Les flux complets des faces, orientés vers +x ou +y : la gauche, la droite (par j), le devant, le derrière (par i).
        let (i0, j0) = (self.i0, self.j0);
        let mut f = vec![[0f64; 3]; 2 * (nx + ny)];
        for j in 0..ny {
            let (hs, us_, vs) = svc(&self.sv, i0 - 1, j0 + j);
            let (ha, ua, va) = col(0, j);
            f[j] = flux_rusanov(g, hs, us_, vs, ha, ua, va);
            let (ha, ua, va) = col(nx - 1, j);
            let (hs, us_, vs) = svc(&self.sv, i0 + nx, j0 + j);
            f[ny + j] = flux_rusanov(g, ha, ua, va, hs, us_, vs);
        }
        for i in 0..nx {
            let (hs, us_, vs) = svc(&self.sv, i0 + i, j0 - 1);
            let (ha, ua, va) = col(i, 0);
            f[2 * ny + i] = flux_rusanov(g, hs, vs, us_, ha, va, ua);
            let (ha, ua, va) = col(i, ny - 1);
            let (hs, us_, vs) = svc(&self.sv, i0 + i, j0 + ny);
            f[2 * ny + nx + i] = flux_rusanov(g, ha, va, ua, hs, vs, us_);
        }
        self.sv.pas_avec_flux_trou(dt, &f).map_err(|_| Refus::SaintVenant)?;
        // 3. Les vitesses normales des bords d'APIC (`F₀/h`, bornées par la célérité), et l'eau qui entre, posée.
        let borne = |h: f64, u: f64| u.abs() + 2. * (g * h).sqrt();
        let vitesse = |f0: f64, h: f64, u: f64| {
            let h = h.max(dx / 4.);
            (f0 / h).clamp(-borne(h, u), borne(h, u))
        };
        let part = |h: f64, k: usize| ((h - k as f64 * dx) / dx).clamp(0., 1.);
        let (mut gx, mut dxr) = (vec![0f32; nz * ny], vec![0f32; nz * ny]);
        let (mut vg, mut bg, mut vd) = (vec![0f64; nz * ny], vec![0f32; nz * ny], vec![0f64; ny]);
        let mut vd_v = 0f64;
        for j in 0..ny {
            let (h, u, _) = col(0, j);
            let w = vitesse(f[j][0], h, u);
            for k in 0..nz {
                if part(h, k) > 0. {
                    gx[k * ny + j] = w as f32;
                }
                if f[j][0] > 0. {
                    vg[k * ny + j] = f[j][0] * part(h, k) / (h.max(dx / 4.)) * dt * dx * dx;
                    bg[k * ny + j] = (w.max(0.) * dt) as f32;
                }
            }
            let (h, u, _) = col(nx - 1, j);
            let w = vitesse(f[ny + j][0], h, u);
            for k in 0..nz {
                if part(h, k) > 0. {
                    dxr[k * ny + j] = w as f32;
                }
            }
            if f[ny + j][0] < 0. {
                vd[j] = -f[ny + j][0] * dt * dx;
                vd_v = vd_v.min(w);
            }
        }
        let (mut gy, mut vy, mut by) = (vec![0f32; 2 * nz * nx], vec![0f64; 2 * nz * nx], vec![0f32; 2 * nz * nx]);
        for i in 0..nx {
            for (cote, (fi, (h, _, v))) in [(2 * ny + i, col(i, 0)), (2 * ny + nx + i, col(i, ny - 1))].into_iter().enumerate() {
                let f0 = f[fi][0];
                let w = vitesse(f0, h, v);
                let entre = if cote == 0 { f0 > 0. } else { f0 < 0. };
                for k in 0..nz {
                    let face = cote * nz * nx + k * nx + i;
                    if part(h, k) > 0. {
                        gy[face] = w as f32;
                    }
                    if entre {
                        vy[face] = f0.abs() * part(h, k) / h.max(dx / 4.) * dt * dx * dx;
                        by[face] = (w.abs() * dt) as f32;
                    }
                }
            }
        }
        self.apic.set_open_boundaries(&gx, &dxr).map_err(Refus::Apic)?;
        let (devant, derriere) = gy.split_at(nz * nx);
        self.apic.set_y_boundaries(devant, derriere).map_err(Refus::Apic)?;
        self.apic.feed_left_grid(&vg, &bg).map_err(Refus::Apic)?;
        self.apic.feed_right(&vd, [vd_v as f32, 0., 0.]).map_err(Refus::Apic)?;
        self.apic.feed_y_grid(&vy, &by).map_err(Refus::Apic)?;
        // 4. Le pas d'APIC.
        self.apic.step(us).map_err(Refus::Apic)?;
        // 5. Le bilan de chaque face : ce que Saint-Venant a cédé, contre ce qu'APIC a reçu (aux réservoirs) et rendu ; l'écart à la maille
        // active.
        let sorti_g: Vec<f64> = self.apic.left_inlet().map(|g| g.0.to_vec()).unwrap_or_default();
        let sorti_d: Vec<f64> = self.apic.right_outlet().map(|s| s.0.to_vec()).unwrap_or_default();
        let sorti_y: Vec<f64> = self.apic.y_outlet_step().map(|s| s.to_vec()).unwrap_or_default();
        let ajouter = |sv: &mut SaintVenant2D, i: usize, j: usize, delta: f64, total: &mut f64| {
            sv.h[i * nsv + j] = (sv.h[i * nsv + j] + delta / (dx * dx)).max(0.);
            *total += delta.abs();
        };
        let mut total = 0.;
        for j in 0..ny {
            // la gauche : F₀ va de Saint-Venant vers APIC.
            let recu = vg[j..].iter().step_by(ny).sum::<f64>();
            let delta = f[j][0] * dt * dx - recu + sorti_g.get(j).copied().unwrap_or(0.);
            ajouter(&mut self.sv, i0 - 1, j0 + j, delta, &mut total);
            // la droite : F₀ va d'APIC vers Saint-Venant.
            let delta = -(f[ny + j][0] * dt * dx + vd[j] - sorti_d.get(j).copied().unwrap_or(0.));
            ajouter(&mut self.sv, i0 + nx, j0 + j, delta, &mut total);
        }
        for i in 0..nx {
            let recu: f64 = (0..nz).map(|k| vy[k * nx + i]).sum();
            let delta = f[2 * ny + i][0] * dt * dx - recu + sorti_y.get(i).copied().unwrap_or(0.);
            ajouter(&mut self.sv, i0 + i, j0 - 1, delta, &mut total);
            let recu: f64 = (0..nz).map(|k| vy[nz * nx + k * nx + i]).sum();
            let delta = -(f[2 * ny + nx + i][0] * dt * dx + recu - sorti_y.get(nx + i).copied().unwrap_or(0.));
            ajouter(&mut self.sv, i0 + i, j0 + ny, delta, &mut total);
        }
        self.correction += total;
        Ok(())
    }
}

#[cfg(test)]
#[path = "tests_relais_boite.rs"]
mod tests;
