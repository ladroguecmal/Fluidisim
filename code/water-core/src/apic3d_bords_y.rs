//! **S724 — les bords en y par particules** (LOD-ETAPE-3-S722, B2) : la boîte de 3D autour d'un corps est raccordée sur ses quatre côtés.
//! En x, les bords existent (à gauche S698, à droite S682) ; ici, le devant (`y = 0`) et le derrière (`y = ly`), sur le même modèle :
//!
//! - la vitesse normale de chaque face (couche `k`, colonne `i`) vient de l'appelant (`set_y_boundaries`) ;
//! - une particule qui franchit un bord en y est retirée, et son volume compté ;
//! - le volume qui entre par une face s'ajoute à son réservoir. Chaque quantum entier naît dans la tranche que le flux a balayée, à la
//!   sous-maille (x, z) la moins occupée, avec la vitesse et l'affine du G2P (la pose par la grille, S702).

use super::*;

/// S724 — les bords en y : les vitesses normales imposées, les comptes.
pub(crate) struct BordsY {
    /// Les vitesses normales (`v`) des faces du devant puis du derrière, `nx·nz` chacune, rangées `k·nx + i`.
    pub(crate) v: Vec<f32>,
    pub(crate) sorti: f64,
    pub(crate) retirees: u64,
    /// Les réservoirs des faces, le devant puis le derrière.
    pub(crate) reservoir: Vec<f64>,
    pub(crate) entre: f64,
    pub(crate) posees: u64,
    pub(crate) refusees: u64,
}

impl Apic3 {
    /// **S724 — les bords en y par particules** ; refusé avec les gouttes ou s'ils existent déjà. Réservé à la configuration (I-06).
    pub fn enable_y_boundaries(&mut self, host: &mut HostServices) -> Result<(), Error> {
        if self.bords_y.is_some() || self.gouttes.is_some() {
            return Err(Error::Domain);
        }
        let Domain3 { nx, nz, .. } = self.domain;
        host.alloc.alloc_persistent(2 * nx * nz * 12).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        self.bords_y = Some(Box::new(BordsY { v: vec![0.; 2 * nx * nz], sorti: 0., retirees: 0, reservoir: vec![0.; 2 * nx * nz], entre: 0.,
            posees: 0, refusees: 0 }));
        Ok(())
    }

    /// **S724 — les vitesses normales des bords en y** : `devant` en `j = 0`, `derriere` en `j = ny`, `nx·nz` valeurs chacune, rangées
    /// `k·nx + i`. Refus : sans bords en y (`Domain`), longueur (`Shape`), valeur non finie.
    pub fn set_y_boundaries(&mut self, devant: &[f32], derriere: &[f32]) -> Result<(), Error> {
        let Domain3 { nx, nz, .. } = self.domain;
        let b = self.bords_y.as_mut().ok_or(Error::Domain)?;
        if devant.len() != nx * nz || derriere.len() != nx * nz {
            return Err(Error::Shape);
        }
        if devant.iter().chain(derriere).any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        b.v[..nx * nz].copy_from_slice(devant);
        b.v[nx * nz..].copy_from_slice(derriere);
        Ok(())
    }

    /// S724 — les bords en y : `(sorti au total, retirées, reçu, posées, refusées, le reste des réservoirs)`.
    pub fn y_boundaries(&self) -> Option<(f64, u64, f64, u64, u64, f64)> {
        self.bords_y.as_ref().map(|b| (b.sorti, b.retirees, b.entre, b.posees, b.refusees, b.reservoir.iter().sum()))
    }

    /// S724 : les vitesses imposées aux faces des bords en y (dans `walls`).
    pub(crate) fn walls_y(&mut self) {
        let Some(b) = self.bords_y.as_ref() else { return };
        let Domain3 { nx, ny, nz, .. } = self.domain;
        for k in 0..nz {
            for i in 0..nx {
                self.v[(k * (ny + 1)) * nx + i] = b.v[k * nx + i];
                self.v[(k * (ny + 1) + ny) * nx + i] = b.v[nx * nz + k * nx + i];
            }
        }
    }

    /// S724 : retire les particules passées au-delà des bords en y, compte leur volume.
    pub(crate) fn drain_y(&mut self) {
        let Some(mut b) = self.bords_y.take() else { return };
        let Domain3 { ny, dx, .. } = self.domain;
        let ly = ny as f32 * dx;
        let quantum = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let mut k = 0;
        while k < self.n {
            let y = self.x[k][1];
            if y < 0. || y >= ly {
                b.sorti += quantum;
                b.retirees += 1;
                let last = self.n - 1;
                self.x[k] = self.x[last];
                self.vel[k] = self.vel[last];
                self.c[k] = self.c[last];
                self.n = last;
            } else {
                k += 1;
            }
        }
        self.bords_y = Some(b);
    }

    /// **S724 — l'entrée en y par la grille** : `volumes` (m³ par face au pas, le devant puis le derrière, `k·nx + i`, positifs) au réservoir
    /// de la face ; chaque quantum entier naît dans la tranche balayée `balayage` (m par face, ≤ dx) contre le bord, à la sous-maille (x, z)
    /// la moins occupée, décalée par la suite R₂ ; sa vitesse et son affine, celles de la grille (G2P).
    pub fn feed_y_grid(&mut self, volumes: &[f64], balayage: &[f32]) -> Result<(), Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let Some(mut b) = self.bords_y.take() else { return Err(Error::Domain) };
        let bon = volumes.len() == 2 * nx * nz && balayage.len() == 2 * nx * nz;
        if !bon || volumes.iter().any(|v| !v.is_finite() || *v < 0.) || balayage.iter().any(|x| !x.is_finite() || *x < 0.) {
            self.bords_y = Some(b);
            return Err(if bon { Error::Domain } else { Error::Shape });
        }
        let quantum = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let (h, ly) = (dx / PER_AXIS as f32, ny as f32 * dx);
        let n2 = PER_AXIS * PER_AXIS;
        // Une seule passe : l'occupation (x, z) des premières et dernières rangées, par face.
        let mut occupation = vec![0u32; 2 * nx * nz * n2];
        for p in &self.x[..self.n] {
            let cote = if p[1] >= 0. && p[1] < dx { 0 } else if p[1] >= ly - dx && p[1] < ly { 1 } else { continue };
            let (i, k) = (((p[0] / dx) as usize).min(nx - 1), ((p[2] / dx) as usize).min(nz - 1));
            let a = (((p[0] - i as f32 * dx) / h) as usize).min(PER_AXIS - 1);
            let c = (((p[2] - k as f32 * dx) / h) as usize).min(PER_AXIS - 1);
            occupation[(cote * nx * nz + k * nx + i) * n2 + c * PER_AXIS + a] += 1;
        }
        for (f, &vol) in volumes.iter().enumerate() {
            b.reservoir[f] += vol;
            b.entre += vol;
            let (cote, kk, i) = (f / (nx * nz), (f % (nx * nz)) / nx, f % nx);
            let (x0, z0) = (i as f32 * dx, kk as f32 * dx);
            let zb = if self.lisse.is_some() { self.smooth_seabed_height(x0 + 0.5 * dx, if cote == 0 { 0. } else { ly }) } else {
                self.seabed_height(i, if cote == 0 { 0 } else { ny - 1 }) };
            let large = balayage[f].min(dx * 0.999);
            while b.reservoir[f] >= quantum {
                let occ = &mut occupation[f * n2..(f + 1) * n2];
                let mut choix: Option<(u32, usize)> = None;
                for (s, &o) in occ.iter().enumerate() {
                    let z = z0 + ((s / PER_AXIS) as f32 + 0.5) * h;
                    if z > zb + 0.05 * dx && choix.is_none_or(|(o0, _)| o < o0) {
                        choix = Some((o, s));
                    }
                }
                let Some((_, s)) = choix else { break };
                if self.n >= self.x.len() {
                    b.refusees += 1;
                    break;
                }
                let r = b.posees as f64;
                let (rx, ry, rz) = ((0.5 + r * 0.819_172_513_396_164_4).fract(), (0.5 + r * 0.671_043_606_703_789_2).fract(),
                    (0.5 + r * 0.549_700_477_901_970_4).fract());
                let (a, c) = (s % PER_AXIS, s / PER_AXIS);
                let y = if cote == 0 { large * ry as f32 } else { ly - large * ry as f32 - 1e-6 };
                let m = self.n;
                self.x[m] = [x0 + (a as f32 + rx as f32) * h, y.clamp(0., ly - 1e-6), z0 + (c as f32 + rz as f32) * h];
                self.n += 1;
                let (vk, ck) = self.gather_particle(m);
                self.vel[m] = vk;
                self.c[m] = ck;
                occ[s] += 1;
                b.reservoir[f] -= quantum;
                b.posees += 1;
            }
        }
        self.bords_y = Some(b);
        Ok(())
    }
}
