//! **S479 (K2-1, ADR-220 D1) — l'air enfermé : les poches d'ADR-015 (T2) dans APIC 3D.**
//!
//! Sans elles, une poche d'air que l'eau referme garde la pression atmosphérique quelle que soit sa profondeur : l'eau s'y
//! engouffre sous la pression hydrostatique, et à maille fine le calcul s'emballe à la fermeture (A311). Ici, chaque
//! **composante d'air enfermé** — les mailles d'air que l'air libre (la rangée du haut) n'atteint pas — est une **poche** :
//!
//! - **son air** : l'invariant adiabatique `a = V·P^(1/γ)`, γ = 1,4 (l'échelle de temps d'un impact, ADR-015 §3) ; à la
//!   naissance, `P` est la pression de l'eau qui la borde (au pas d'avant) plus `P_atm` ; une poche suivie d'un pas à l'autre
//!   par recouvrement de mailles garde son air — des poches qui fusionnent additionnent les leurs, une poche qui se scinde
//!   partage le sien au prorata des mailles ; une poche qui rejoint l'air libre se vide ;
//! - **son volume** : **suivi par le flux** que la projection prévoit (`ΔV` de sa loi linéarisée), et **rappelé** vers le volume
//!   géométrique — la fraction d'air `clamp(0,5 + φ/dx, 0, 1)` de ses mailles et des mailles d'eau qui la bordent — en
//!   `RAPPEL_VOLUME_S`. Le volume géométrique seul saute d'un pas à l'autre quand des mailles changent d'étiquette (S479 :
//!   jusqu'à 1 %, trois mailles d'un coup) ; chaque saut frappe la poche d'un coup de pression et fausse sa fréquence. Le flux
//!   est lisse et cohérent avec les vitesses ; le rappel empêche la dérive ;
//! - **sa pression, inconnue de la projection** : la loi linéarisée `P^(n+1) − Pⁿ = −(γPⁿ/Vⁿ)·ΔV`, `ΔV` le flux de ses faces
//!   après correction, donne la ligne `(s + Σ 1/θ)·p_b − Σ p_c/θ = s·p_bⁿ − (ρ·dx/dt)·Σ u*_sortant`, `s = ρ·Vⁿ/(γ·Pⁿ·dt²·dx)`
//!   — symétrique avec les lignes des mailles d'eau qui la bordent (le coefficient `−1/θ` des deux côtés) : le système reste
//!   défini positif et le même gradient conjugué le résout. **Implicite** : stable quel que soit le pas devant la raideur de
//!   la poche.
//!
//! Une poche de moins d'une maille d'air (`V < dx³`), ou de moins de `POCHE_MAILLES_MIN` mailles (S481), se résorbe — l'air y redevient libre — : sa raideur n'a plus de sens à
//! cette échelle (les microbulles, K2-7, la prendront). Plus de `MAX_POCKETS` poches : les suivantes restent libres, comptées.

use super::*;

/// Le nombre de poches suivies au plus ; au-delà, l'air reste libre (`overflow`).
pub const MAX_POCKETS: usize = 64;
/// La pression atmosphérique, Pa.
pub const P_ATM: f64 = 101_325.;
/// L'exposant adiabatique de l'air.
pub const GAMMA_AIR: f64 = 1.4;
/// **S481** — les mailles d'air au moins d'une poche gardée : en deçà (un cube de 2 × 2 × 2), elle se résorbe. Mesuré sur la scène
/// `--v1` (dx = 5 cm) : des poches d'une ou deux mailles, nées dans l'eau brassée sous la cavité, sautent de volume à chaque changement
/// d'étiquette et la pression y bondit (283 kPa) jusqu'à l'emballement ; leur raideur n'a pas de sens à cette échelle (les microbulles,
/// K2-7). Leur volume compte la part d'air des mailles d'eau voisines : la règle du volume (`V < dx³`) ne les attrapait pas.
pub const POCHE_MAILLES_MIN: u32 = 8;
/// Le temps de rappel du volume suivi par le flux vers le volume géométrique, s (voir le volume, en tête).
pub const RAPPEL_VOLUME_S: f64 = 0.1;

/// Une poche, telle que le pas la rend.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AirPocket {
    /// Volume, m³.
    pub volume: f64,
    /// Pression absolue, Pa.
    pub pressure: f64,
    /// Le centre de ses mailles d'air (pondéré par leur fraction), m.
    pub centroid: [f64; 3],
    /// Ses mailles d'air.
    pub cells: u32,
}

/// S481 — l'état des poches entre deux pas (`Apic3::air_pocket_state`).
#[derive(Clone, Copy, Debug)]
pub struct AirPocketState<'a> {
    /// La poche de chaque maille : 0 aucune, `b + 1` la poche `b`.
    pub of: &'a [u32],
    /// L'air de chaque poche, `V·P^(1/γ)`.
    pub air: &'a [f64],
    /// Le volume de chaque poche suivi par le flux, m³.
    pub vol_flux: &'a [f64],
    /// Le dernier pas, s (le rappel du volume en dépend).
    pub last_dt: f64,
}

pub(crate) struct Poches {
    /// La poche de chaque maille (0 : aucune ; `b + 1`) ; celle du pas d'avant.
    pub(crate) of: Vec<u32>,
    pub(crate) old_of: Vec<u32>,
    /// La pile du remplissage, et le marqueur des mailles d'eau déjà comptées dans un volume.
    stack: Vec<u32>,
    seen: Vec<u32>,
    stamp: u32,
    pub(crate) count: usize,
    old_count: usize,
    /// L'air de chaque poche, `a = V·P^(1/γ)` ; et celui du pas d'avant.
    air: [f64; MAX_POCKETS],
    old_air: [f64; MAX_POCKETS],
    pub(crate) volume: [f64; MAX_POCKETS],
    pub(crate) pressure: [f64; MAX_POCKETS],
    centroid: [[f64; 3]; MAX_POCKETS],
    cells: [u32; MAX_POCKETS],
    /// Recouvrements (nouvelle × ancienne), en mailles.
    overlap: Vec<u32>,
    /// Le gradient conjugué, entrées des poches : solution (pression relative, Pa), résidu, préconditionné, direction, `A·d`,
    /// second membre, diagonale, et `s`.
    pub(crate) x: [f32; MAX_POCKETS],
    pub(crate) r: [f32; MAX_POCKETS],
    pub(crate) z: [f32; MAX_POCKETS],
    pub(crate) d: [f32; MAX_POCKETS],
    pub(crate) q: [f32; MAX_POCKETS],
    pub(crate) rhs: [f32; MAX_POCKETS],
    pub(crate) diag: [f32; MAX_POCKETS],
    /// Les couples (maille d'eau, poche, `1/θ`) de la frontière, pour `A·x`.
    pub(crate) pairs: Vec<(u32, u16, f32)>,
    /// Poches au-delà de `MAX_POCKETS`, laissées libres, depuis la mise en route.
    pub(crate) overflow: u64,
    /// Diagnostic : la variation de volume que la projection prévoit pour chaque poche (le flux de ses faces corrigées × dt).
    pub(crate) dv_prevu: [f64; MAX_POCKETS],
    /// Le volume suivi par le flux, et celui du pas d'avant ; le volume géométrique ; le dernier pas, s.
    vol_flux: [f64; MAX_POCKETS],
    old_vol_flux: [f64; MAX_POCKETS],
    pub(crate) geo: [f64; MAX_POCKETS],
    last_dt: f64,
}

/// Octets que `enable_air_pockets` réserve pour `cells` mailles.
pub fn pockets_reserved_bytes(cells: usize) -> Option<usize> {
    // of, old_of, stack, seen (4 octets chacun) ; couples de frontière, au plus trois par maille (4 + 2 + 4, arrondis à 12).
    cells.checked_mul(4 * 4 + 3 * 12)?.checked_add(MAX_POCKETS * MAX_POCKETS * 4)
}

impl Apic3 {
    /// **S479 — les poches d'air enfermé** (ADR-220 D1). À l'initialisation (I-06) : la mémoire réservée auprès de l'hôte.
    /// Refus : déjà actives, ou mode relatif (`Domain`). Sans cet appel, le pas est celui d'avant, au bit.
    pub fn enable_air_pockets(&mut self, host: &mut HostServices) -> Result<(), Error> {
        if self.poches.is_some() || self.is_relative() {
            return Err(Error::Domain);
        }
        let Domain3 { nx, ny, nz, .. } = self.domain;
        let cells = nx * ny * nz;
        let bytes = pockets_reserved_bytes(cells).ok_or(Error::Domain)?;
        host.alloc.alloc_persistent(bytes).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        self.poches = Some(Box::new(Poches {
            of: vec![0; cells],
            old_of: vec![0; cells],
            stack: Vec::with_capacity(cells),
            seen: vec![0; cells],
            stamp: 0,
            count: 0,
            old_count: 0,
            air: [0.; MAX_POCKETS],
            old_air: [0.; MAX_POCKETS],
            volume: [0.; MAX_POCKETS],
            pressure: [0.; MAX_POCKETS],
            centroid: [[0.; 3]; MAX_POCKETS],
            cells: [0; MAX_POCKETS],
            overlap: vec![0; MAX_POCKETS * MAX_POCKETS],
            x: [0.; MAX_POCKETS],
            r: [0.; MAX_POCKETS],
            z: [0.; MAX_POCKETS],
            d: [0.; MAX_POCKETS],
            q: [0.; MAX_POCKETS],
            rhs: [0.; MAX_POCKETS],
            diag: [0.; MAX_POCKETS],
            pairs: Vec::with_capacity(3 * cells),
            overflow: 0,
            dv_prevu: [0.; MAX_POCKETS],
            vol_flux: [0.; MAX_POCKETS],
            old_vol_flux: [0.; MAX_POCKETS],
            geo: [0.; MAX_POCKETS],
            last_dt: 0.,
        }));
        Ok(())
    }

    /// Les poches du dernier pas (vide sans `enable_air_pockets`).
    pub fn air_pockets(&self, out: &mut [AirPocket]) -> usize {
        let Some(ps) = &self.poches else { return 0 };
        let n = ps.count.min(out.len());
        for b in 0..n {
            out[b] = AirPocket { volume: ps.volume[b], pressure: ps.pressure[b], centroid: ps.centroid[b], cells: ps.cells[b] };
        }
        n
    }

    /// Diagnostic : la variation de volume de la première poche que la dernière projection a prévue (m³).
    pub fn air_pocket_predicted_dv(&self) -> f64 {
        self.poches.as_ref().map_or(0., |p| p.dv_prevu[0])
    }

    /// **S481 (K2-2)** — l'état des poches entre deux pas, celui que la carte charge pour reprendre au même point : la poche de
    /// chaque maille (0 : aucune, `b + 1`), et pour chaque poche son air `a = V·P^(1/γ)` et son volume suivi par le flux ; le
    /// dernier pas, s. `None` sans `enable_air_pockets`.
    pub fn air_pocket_state(&self) -> Option<AirPocketState<'_>> {
        let ps = self.poches.as_ref()?;
        Some(AirPocketState {
            of: &ps.of,
            air: &ps.air[..ps.count],
            vol_flux: &ps.vol_flux[..ps.count],
            last_dt: ps.last_dt,
        })
    }

    /// Poches au-delà de `MAX_POCKETS`, laissées libres, depuis la mise en route.
    pub fn air_pockets_overflow(&self) -> u64 {
        self.poches.as_ref().map_or(0, |p| p.overflow)
    }

    /// **S479 — mise en poche de l'air**, après les étiquettes : les composantes d'air enfermé, leur volume, leur air hérité
    /// ou né, leur pression `(a/V)^γ`.
    pub(crate) fn pockets_detect(&mut self) {
        let Some(mut ps) = self.poches.take() else { return };
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let cells = nx * ny * nz;
        let cell_volume = (dx as f64).powi(3);
        let frac = |phi: f32| (0.5 + phi / dx).clamp(0., 1.) as f64;
        // 1. L'ancien état.
        core::mem::swap(&mut ps.of, &mut ps.old_of);
        ps.old_count = ps.count;
        ps.old_air = ps.air;
        ps.old_vol_flux = ps.vol_flux;
        // 2. L'air libre : le remplissage depuis la rangée du haut. `u32::MAX` le marque.
        for v in ps.of.iter_mut() {
            *v = 0;
        }
        const LIBRE: u32 = u32::MAX;
        ps.stack.clear();
        for j in 0..ny {
            for i in 0..nx {
                let c = self.cell(i, j, nz - 1);
                if self.label[c] == AIR {
                    ps.of[c] = LIBRE;
                    ps.stack.push(c as u32);
                }
            }
        }
        while let Some(c) = ps.stack.pop() {
            let c = c as usize;
            let (i, j, k) = (c % nx, (c / nx) % ny, c / (nx * ny));
            for (n, _, _) in self.neighbours(i, j, k).into_iter().flatten() {
                if self.label[n] == AIR && ps.of[n] == 0 {
                    ps.of[n] = LIBRE;
                    ps.stack.push(n as u32);
                }
            }
        }
        // 3. Les composantes enfermées, leur volume et leur centre.
        let mut count = 0usize;
        ps.stamp = ps.stamp.wrapping_add(1);
        if ps.stamp == 0 {
            for v in ps.seen.iter_mut() {
                *v = 0;
            }
            ps.stamp = 1;
        }
        for start in 0..cells {
            if self.label[start] != AIR || ps.of[start] != 0 {
                continue;
            }
            if count == MAX_POCKETS {
                // Au-delà : laissée libre, comptée.
                ps.overflow += 1;
                ps.of[start] = LIBRE;
                ps.stack.push(start as u32);
                while let Some(c) = ps.stack.pop() {
                    let c = c as usize;
                    let (i, j, k) = (c % nx, (c / nx) % ny, c / (nx * ny));
                    for (n, _, _) in self.neighbours(i, j, k).into_iter().flatten() {
                        if self.label[n] == AIR && ps.of[n] == 0 {
                            ps.of[n] = LIBRE;
                            ps.stack.push(n as u32);
                        }
                    }
                }
                continue;
            }
            let id = count as u32 + 1;
            let (mut vol, mut cen, mut n_cells) = (0f64, [0f64; 3], 0u32);
            ps.of[start] = id;
            ps.stack.push(start as u32);
            while let Some(c) = ps.stack.pop() {
                let c = c as usize;
                let (i, j, k) = (c % nx, (c / nx) % ny, c / (nx * ny));
                let f = frac(self.phi[c]) * cell_volume;
                vol += f;
                n_cells += 1;
                let q = [(i as f64 + 0.5) * dx as f64, (j as f64 + 0.5) * dx as f64, (k as f64 + 0.5) * dx as f64];
                for m in 0..3 {
                    cen[m] += f * q[m];
                }
                for (n, _, _) in self.neighbours(i, j, k).into_iter().flatten() {
                    if self.label[n] == AIR && ps.of[n] == 0 {
                        ps.of[n] = id;
                        ps.stack.push(n as u32);
                    } else if self.label[n] == WATER && ps.seen[n] != ps.stamp {
                        // La part d'air des mailles d'eau qui la bordent, comptée une fois.
                        ps.seen[n] = ps.stamp;
                        vol += frac(self.phi[n]) * cell_volume;
                    }
                }
            }
            let b = count;
            ps.volume[b] = vol;
            ps.cells[b] = n_cells;
            ps.centroid[b] = if vol > 0. { [cen[0] / vol, cen[1] / vol, cen[2] / vol] } else { [0.; 3] };
            count += 1;
            // Le marqueur d'eau : une poche voisine recompterait une maille d'eau partagée — un nouveau tampon par poche.
            ps.stamp = ps.stamp.wrapping_add(1);
            if ps.stamp == 0 {
                for v in ps.seen.iter_mut() {
                    *v = 0;
                }
                ps.stamp = 1;
            }
        }
        // 4. L'air hérité : recouvrements avec les poches d'avant, l'air de chacune partagé au prorata de ses mailles.
        for v in ps.overlap.iter_mut() {
            *v = 0;
        }
        for c in 0..cells {
            let (b, o) = (ps.of[c], ps.old_of[c]);
            if b != 0 && b != LIBRE && o != 0 && o != LIBRE {
                ps.overlap[(b as usize - 1) * MAX_POCKETS + o as usize - 1] += 1;
            }
        }
        for b in 0..count {
            ps.air[b] = 0.;
            ps.vol_flux[b] = 0.;
        }
        for o in 0..ps.old_count {
            let total: u64 = (0..count).map(|b| ps.overlap[b * MAX_POCKETS + o] as u64).sum();
            if total == 0 {
                continue;
            }
            for b in 0..count {
                let w = ps.overlap[b * MAX_POCKETS + o];
                if w > 0 {
                    ps.air[b] += ps.old_air[o] * w as f64 / total as f64;
                    ps.vol_flux[b] += ps.old_vol_flux[o] * w as f64 / total as f64;
                }
            }
        }
        // 5. Naissance : la pression de l'eau qui la borde, au pas d'avant (`p` garde la dernière projection).
        let mut sp = [0f64; MAX_POCKETS];
        let mut np = [0u32; MAX_POCKETS];
        if (0..count).any(|b| ps.air[b] <= 0.) {
            for c in 0..cells {
                let b = ps.of[c];
                if b == 0 || b == LIBRE || ps.air[b as usize - 1] > 0. {
                    continue;
                }
                let (i, j, k) = (c % nx, (c / nx) % ny, c / (nx * ny));
                for (w, _, _) in self.neighbours(i, j, k).into_iter().flatten() {
                    if self.label[w] == WATER {
                        sp[b as usize - 1] += self.p[w] as f64;
                        np[b as usize - 1] += 1;
                    }
                }
            }
        }
        for b in 0..count {
            if ps.air[b] > 0. {
                continue;
            }
            let p0 = (P_ATM + if np[b] > 0 { sp[b] / np[b] as f64 } else { 0. }).max(0.1 * P_ATM);
            ps.air[b] = ps.volume[b] * p0.powf(1. / GAMMA_AIR);
            ps.vol_flux[b] = ps.volume[b];
        }
        // Le volume de la poche : le flux suivi, rappelé vers la géométrie (`volume` ne porte jusqu'ici que la géométrie).
        let rappel = (ps.last_dt / RAPPEL_VOLUME_S).min(1.);
        for b in 0..count {
            ps.geo[b] = ps.volume[b];
            ps.volume[b] = ps.vol_flux[b] + (ps.geo[b] - ps.vol_flux[b]) * rappel;
        }
        // 6. La pression, et la résorption des poches de moins d'une maille.
        let mut kept = 0usize;
        let mut remap = [0u32; MAX_POCKETS];
        for b in 0..count {
            if ps.geo[b] < cell_volume || ps.cells[b] < POCHE_MAILLES_MIN {
                remap[b] = LIBRE;
                continue;
            }
            remap[b] = kept as u32 + 1;
            ps.volume[kept] = ps.volume[b];
            ps.geo[kept] = ps.geo[b];
            ps.cells[kept] = ps.cells[b];
            ps.centroid[kept] = ps.centroid[b];
            ps.air[kept] = ps.air[b];
            ps.pressure[kept] = (ps.air[b] / ps.volume[b]).powf(GAMMA_AIR);
            kept += 1;
        }
        if kept != count {
            for v in ps.of.iter_mut() {
                if *v != 0 && *v != LIBRE {
                    *v = remap[*v as usize - 1];
                }
            }
        }
        for v in ps.of.iter_mut() {
            if *v == LIBRE {
                *v = 0;
            }
        }
        ps.count = kept;
        self.poches = Some(ps);
    }

    /// **La projection avec poches** : celle de `project`, plus une inconnue par poche (module). Rend (itérations, résidu).
    pub(crate) fn project_with_pockets(&mut self, dt: f32) -> (u32, f64) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let scale = -self.rho * dx * dx / dt;
        let mut ps = self.poches.take().expect("poches");
        let count = ps.count;
        ps.pairs.clear();
        let mut flux = [0f64; MAX_POCKETS];
        let mut inv_theta_sum = [0f64; MAX_POCKETS];
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.cell(i, j, k);
                    self.p[c] = 0.;
                    if self.label[c] != WATER {
                        self.rhs[c] = 0.;
                        self.diag[c] = 0.;
                        continue;
                    }
                    let mut div = 0f32;
                    let mut diag = 0f32;
                    let mut ghost = 0f32;
                    for (m, nb) in self.neighbours(i, j, k).into_iter().enumerate() {
                        let sign = if m % 2 == 0 { -1. } else { 1. };
                        let face = match m {
                            0 => self.u[(k * ny + j) * (nx + 1) + i],
                            1 => self.u[(k * ny + j) * (nx + 1) + i + 1],
                            2 => self.v[(k * (ny + 1) + j) * nx + i],
                            3 => self.v[(k * (ny + 1) + j + 1) * nx + i],
                            4 => self.w[(k * ny + j) * nx + i],
                            _ => self.w[((k + 1) * ny + j) * nx + i],
                        };
                        div += sign * face;
                        if let Some((n, _, _)) = nb {
                            match self.label[n] {
                                WATER => diag += 1.,
                                AIR => {
                                    let t = self.theta(c, n);
                                    diag += 1. / t;
                                    match ps.of[n] {
                                        0 => ghost += self.surface_pressure(c, n) / t,
                                        b => {
                                            let b = b as usize - 1;
                                            // La sortie de la poche vers cette maille d'eau, le long de l'axe de la face.
                                            flux[b] += (-sign * face) as f64;
                                            inv_theta_sum[b] += 1. / t as f64;
                                            ps.pairs.push((c as u32, b as u16, 1. / t));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    self.rhs[c] = scale * div / dx + ghost;
                    self.diag[c] = diag;
                }
            }
        }
        // Les lignes des poches.
        let dtd = dt as f64;
        for b in 0..count {
            let s = self.rho as f64 * ps.volume[b] / (GAMMA_AIR * ps.pressure[b] * dtd * dtd * dx as f64);
            let pn = ps.pressure[b] - P_ATM;
            ps.diag[b] = (s + inv_theta_sum[b]) as f32;
            ps.rhs[b] = (s * pn + (scale as f64 / dx as f64) * flux[b]) as f32;
            ps.x[b] = 0.;
        }
        // Le gradient conjugué, poches comprises.
        let cells = self.p.len();
        let dot2 = |a: &[f32], pa: &[f32], b: &[f32], pb: &[f32]| Self::dot(a, b) + Self::dot(&pa[..count], &pb[..count]);
        let b2 = dot2(&self.rhs, &ps.rhs, &self.rhs, &ps.rhs);
        self.r.copy_from_slice(&self.rhs);
        ps.r = ps.rhs;
        for c in 0..cells {
            self.z[c] = if self.diag[c] > 0. { self.r[c] / self.diag[c] } else { 0. };
        }
        for b in 0..count {
            ps.z[b] = ps.r[b] / ps.diag[b];
        }
        self.d.copy_from_slice(&self.z);
        ps.d = ps.z;
        let mut rz = dot2(&self.r, &ps.r, &self.z, &ps.z);
        let mut rr = b2;
        let mut it = 0u32;
        while b2 > 0. && rr > PRESSURE_TOLERANCE2 * b2 && it < PRESSURE_MAX_ITERATIONS {
            let (d, mut q) = (core::mem::take(&mut self.d), core::mem::take(&mut self.q));
            self.apply_with_pockets(&ps, &d, &mut q);
            // Les lignes des poches : `s·d_b + Σ (d_b − d_c)/θ`.
            for b in 0..count {
                ps.q[b] = (ps.diag[b] as f64 * ps.d[b] as f64) as f32;
            }
            for &(c, b, it_) in ps.pairs.iter() {
                ps.q[b as usize] -= it_ * d[c as usize];
            }
            let dq = Self::dot(&d, &q) + Self::dot(&ps.d[..count], &ps.q[..count]);
            self.d = d;
            self.q = q;
            if !(dq > 0.) {
                break;
            }
            let alpha = (rz / dq) as f32;
            for c in 0..cells {
                self.p[c] += alpha * self.d[c];
                self.r[c] -= alpha * self.q[c];
                self.z[c] = if self.diag[c] > 0. { self.r[c] / self.diag[c] } else { 0. };
            }
            for b in 0..count {
                ps.x[b] += alpha * ps.d[b];
                ps.r[b] -= alpha * ps.q[b];
                ps.z[b] = ps.r[b] / ps.diag[b];
            }
            let zn = dot2(&self.r, &ps.r, &self.z, &ps.z);
            let beta = (zn / rz) as f32;
            for c in 0..cells {
                self.d[c] = self.z[c] + beta * self.d[c];
            }
            for b in 0..count {
                ps.d[b] = ps.z[b] + beta * ps.d[b];
            }
            rz = zn;
            rr = dot2(&self.r, &ps.r, &self.r, &ps.r);
            it += 1;
        }
        // Correction des faces qui touchent l'eau ; vers une poche, sa pression.
        let k1 = dt / (self.rho * dx);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.cell(i, j, k);
                    for (m, nb) in self.neighbours(i, j, k).into_iter().enumerate() {
                        if m % 2 == 0 {
                            continue;
                        }
                        let Some((n, f, axis)) = nb else { continue };
                        if self.label[c] == SOLID || self.label[n] == SOLID {
                            continue;
                        }
                        let air = |w: usize, a: usize| match ps.of[a] {
                            0 => self.surface_pressure(w, a),
                            b => ps.x[b as usize - 1],
                        };
                        let (wc, wn) = (self.label[c] == WATER, self.label[n] == WATER);
                        let grad = if wc && wn {
                            self.p[n] - self.p[c]
                        } else if wc {
                            (air(c, n) - self.p[c]) / self.theta(c, n)
                        } else if wn {
                            (self.p[n] - air(n, c)) / self.theta(n, c)
                        } else {
                            continue;
                        };
                        let field = match axis {
                            0 => &mut self.u,
                            1 => &mut self.v,
                            _ => &mut self.w,
                        };
                        field[f] -= k1 * grad;
                    }
                }
            }
        }
        // Diagnostic : la variation de volume prévue — la loi linéarisée la donne, `ΔV = −(p_b − p_bⁿ)/K`.
        for b in 0..count {
            let kb = GAMMA_AIR * ps.pressure[b] / ps.volume[b];
            ps.dv_prevu[b] = -((ps.x[b] as f64) - (ps.pressure[b] - P_ATM)) / kb;
        }
        // La pression de chaque poche au bout du pas (sa loi linéarisée) ; son air ne change pas ; son volume suit le flux.
        for b in 0..count {
            ps.pressure[b] = P_ATM + ps.x[b] as f64;
            ps.vol_flux[b] = ps.volume[b] + ps.dv_prevu[b];
        }
        ps.last_dt = dtd;
        self.poches = Some(ps);
        (it, if b2 > 0. { (rr / b2).sqrt() } else { 0. })
    }

    /// `y = A·x` sur les mailles d'eau, la poche voisine lue dans `ps.d` (le vecteur des poches du même gradient conjugué).
    fn apply_with_pockets(&self, ps: &Poches, x: &[f32], y: &mut [f32]) {
        // S483 (ADR-222 D2) : une écriture par maille, en parallèle avec un système de tâches, au bit.
        let Domain3 { nx, ny, .. } = self.domain;
        let fill = |start: usize, out: &mut [f32]| {
            for (o, yc) in out.iter_mut().enumerate() {
                let c = start + o;
                let (i, j, k) = (c % nx, (c / nx) % ny, c / (nx * ny));
                if self.label[c] != WATER {
                    *yc = 0.;
                    continue;
                }
                let mut s = 0f32;
                for (n, _, _) in self.neighbours(i, j, k).into_iter().flatten() {
                    match self.label[n] {
                        WATER => s += x[c] - x[n],
                        AIR => match ps.of[n] {
                            0 => s += x[c] / self.theta(c, n),
                            b => s += (x[c] - ps.d[b as usize - 1]) / self.theta(c, n),
                        },
                        _ => {}
                    }
                }
                *yc = s;
            }
        };
        match &self.jobs {
            Some(jobs) => jobs.parallel_fill_f32(y, nx * ny * 4, &fill),
            None => fill(0, y),
        }
    }
}
