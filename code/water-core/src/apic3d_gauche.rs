//! **S698 — le bord gauche par particules** (le raccord du large, ADR-275) : le miroir de la sortie et de l'entrée à droite (S682–S683).
//!
//! S697 a montré que le raccord du large à la façon de S650 fausse la vague : une zone de colonnes hydrostatique, une vitesse uniforme sur
//! la verticale. Ici, ni zone ni colonnes :
//!
//! - une particule qui franchit le bord gauche est retirée, et son volume compté par rangée ;
//! - un volume donné par face du bord (rangée et couche) s'ajoute au réservoir de la face. Chaque quantum entier devient une particule posée
//!   dans la maille derrière la face, avec la vitesse que l'appelant donne **à sa hauteur** (le profil vertical d'un porteur dispersif,
//!   SGN) ;
//! - la vitesse normale du bord vient de l'appelant, couche par couche (`set_open_boundaries`).

use super::*;

/// S698 — le compte du bord gauche.
pub(crate) struct BordGauche {
    pub(crate) sorti: Vec<f64>,
    pub(crate) sorti_total: f64,
    pub(crate) retirees: u64,
    pub(crate) reservoir: Vec<f64>,
    pub(crate) entre: f64,
    pub(crate) posees: u64,
    pub(crate) refusees: u64,
}

impl Apic3 {
    /// **S698 — le bord gauche par particules** : demande les bords ouverts ; refusé avec les gouttes. Réservé à la configuration (I-06).
    pub fn enable_left_inlet(&mut self, host: &mut HostServices) -> Result<(), Error> {
        if self.open_x.is_none() || self.gauche.is_some() || self.gouttes.is_some() {
            return Err(Error::Domain);
        }
        let ny = self.domain.ny;
        host.alloc.alloc_persistent(ny * 8 + ny * self.domain.nz * 8).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        self.gauche = Some(Box::new(BordGauche { sorti: vec![0.; ny], sorti_total: 0., retirees: 0, reservoir: vec![0.; ny * self.domain.nz], entre: 0.,
            posees: 0, refusees: 0 }));
        Ok(())
    }

    /// S698 — le bord gauche : `(sorti par rangée au dernier pas, sorti au total, retirées, réservoir par face, reçu, posées, refusées)`.
    #[allow(clippy::type_complexity)]
    pub fn left_inlet(&self) -> Option<(&[f64], f64, u64, &[f64], f64, u64, u64)> {
        self.gauche.as_ref().map(|g| (&g.sorti[..], g.sorti_total, g.retirees, &g.reservoir[..], g.entre, g.posees, g.refusees))
    }

    /// S698 : retire les particules passées à gauche du bord, compte leur volume.
    pub(crate) fn drain_left(&mut self) {
        let Some(mut g) = self.gauche.take() else { return };
        let Domain3 { ny, dx, .. } = self.domain;
        let quantum = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        g.sorti.fill(0.);
        let mut k = 0;
        while k < self.n {
            if self.x[k][0] < 0. {
                let j = ((self.x[k][1] / dx) as usize).min(ny - 1);
                g.sorti[j] += quantum;
                g.sorti_total += quantum;
                g.retirees += 1;
                let last = self.n - 1;
                self.x[k] = self.x[last];
                self.vel[k] = self.vel[last];
                self.c[k] = self.c[last];
                self.n = last;
            } else {
                k += 1;
            }
        }
        self.gauche = Some(g);
    }

    /// **S702 — l'entrée à gauche par la grille** : `volumes` (m³ par face au pas, rangés `k·ny + j`, positifs) au réservoir de la face.
    /// Chaque quantum entier naît dans la tranche que le flux a balayée, `x ∈ [0, balayage)` (`balayage`, m par face, ≤ dx), à la
    /// sous-maille (y, z) la moins occupée de la face, décalée par une suite à faible discrépance. Sa vitesse et sa matrice affine sont
    /// celles que la grille lui donne là (le G2P), comme à toute particule : le porteur ne fournit que le flux et la vitesse du bord.
    pub fn feed_left_grid(&mut self, volumes: &[f64], balayage: &[f32]) -> Result<(), Error> {
        let Domain3 { ny, nz, dx, .. } = self.domain;
        let Some(mut g) = self.gauche.take() else { return Err(Error::Domain) };
        let bon = volumes.len() == ny * nz && balayage.len() == ny * nz;
        if !bon || volumes.iter().any(|v| !v.is_finite() || *v < 0.) || balayage.iter().any(|b| !b.is_finite() || *b < 0.) {
            self.gauche = Some(g);
            return Err(if bon { Error::Domain } else { Error::Shape });
        }
        let quantum = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let h = dx / PER_AXIS as f32;
        let n2 = PER_AXIS * PER_AXIS;
        // Une seule passe : l'occupation (y, z) de la première colonne, par face.
        let mut occupation = vec![0u32; ny * nz * n2];
        for p in &self.x[..self.n] {
            if p[0] >= 0. && p[0] < dx {
                let (j, k) = (((p[1] / dx) as usize).min(ny - 1), ((p[2] / dx) as usize).min(nz - 1));
                let b = (((p[1] - j as f32 * dx) / h) as usize).min(PER_AXIS - 1);
                let c = (((p[2] - k as f32 * dx) / h) as usize).min(PER_AXIS - 1);
                occupation[(k * ny + j) * n2 + c * PER_AXIS + b] += 1;
            }
        }
        for (f, &v) in volumes.iter().enumerate() {
            g.reservoir[f] += v;
            g.entre += v;
            let (k, j) = (f / ny, f % ny);
            let (y0, z0) = (j as f32 * dx, k as f32 * dx);
            let zb = if self.lisse.is_some() { self.smooth_seabed_height(0., y0 + 0.5 * dx) } else { self.seabed_height(0, j) };
            let large = balayage[f].min(dx * 0.999);
            while g.reservoir[f] >= quantum {
                let occ = &mut occupation[f * n2..(f + 1) * n2];
                let mut choix: Option<(u32, usize)> = None;
                for (i, &o) in occ.iter().enumerate() {
                    let z = z0 + ((i / PER_AXIS) as f32 + 0.5) * h;
                    if z > zb + 0.05 * dx && choix.is_none_or(|(o0, _)| o < o0) {
                        choix = Some((o, i));
                    }
                }
                let Some((_, i)) = choix else { break };
                if self.n >= self.x.len() {
                    g.refusees += 1;
                    break;
                }
                // La suite R₂ (Roberts) : trois décalages dans [0, 1), sans hasard ni état autre que le compte des posées.
                let r = g.posees as f64;
                let (rx, ry, rz) = ((0.5 + r * 0.819_172_513_396_164_4).fract(), (0.5 + r * 0.671_043_606_703_789_2).fract(),
                    (0.5 + r * 0.549_700_477_901_970_4).fract());
                let (b, c) = (i % PER_AXIS, i / PER_AXIS);
                let m = self.n;
                self.x[m] = [large * rx as f32, y0 + (b as f32 + ry as f32) * h, z0 + (c as f32 + rz as f32) * h];
                self.n += 1;
                let (vk, ck) = self.gather_particle(m);
                self.vel[m] = vk;
                self.c[m] = ck;
                occ[i] += 1;
                g.reservoir[f] -= quantum;
                g.posees += 1;
            }
        }
        self.gauche = Some(g);
        Ok(())
    }

    /// **S699 — une particule posée telle quelle** au bord gauche (le rejeu d'un enregistrement) : sa position (`x` ramené dans
    /// `[0, dx)`), sa vitesse, sa matrice affine ; son volume compté comme reçu.
    pub fn pose_left(&mut self, x: [f32; 3], vel: [f32; 3], c: [[f32; 3]; 3]) -> Result<(), Error> {
        let Domain3 { ny, nz, dx, .. } = self.domain;
        let Some(g) = self.gauche.as_mut() else { return Err(Error::Domain) };
        if self.n >= self.x.len() {
            g.refusees += 1;
            return Ok(());
        }
        let quantum = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let m = self.n;
        self.x[m] = [x[0].clamp(0., 0.999 * dx), x[1].clamp(0., ny as f32 * dx * 0.99999), x[2].clamp(0., nz as f32 * dx * 0.99999)];
        self.vel[m] = vel;
        self.c[m] = c;
        self.n += 1;
        g.entre += quantum;
        g.posees += 1;
        Ok(())
    }

    /// **S698 — l'entrée à gauche, face par face** : `volumes` (m³ par face du bord, rangée `j` et couche `k`, rangés `k·ny + j`, positifs)
    /// au réservoir de la face. Chaque quantum entier devient une particule posée **dans la maille derrière sa face** (la première colonne,
    /// couche `k`), à la sous-maille la moins occupée, la plus basse d'abord ; sa vitesse, `vitesse(z)`. L'eau entre ainsi à la hauteur où
    /// elle franchit le bord : une pose par le bas de la colonne vidait les couches hautes (un trou d'air sous la surface, S698).
    pub fn feed_left(&mut self, volumes: &[f64], vitesse: &dyn Fn(f32) -> [f32; 3]) -> Result<(), Error> {
        let Domain3 { ny, nz, dx, .. } = self.domain;
        let Some(mut g) = self.gauche.take() else { return Err(Error::Domain) };
        if volumes.len() != ny * nz || volumes.iter().any(|v| !v.is_finite() || *v < 0.) {
            let e = if volumes.len() != ny * nz { Error::Shape } else { Error::Domain };
            self.gauche = Some(g);
            return Err(e);
        }
        let quantum = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let h = dx / PER_AXIS as f32;
        let n3 = PER_AXIS * PER_AXIS * PER_AXIS;
        let mut occupation: Vec<u32> = Vec::new();
        for (f, &v) in volumes.iter().enumerate() {
            g.reservoir[f] += v;
            g.entre += v;
            if g.reservoir[f] < quantum {
                continue;
            }
            let (k, j) = (f / ny, f % ny);
            let (y0, z0) = (j as f32 * dx, k as f32 * dx);
            let zb = if self.lisse.is_some() { self.smooth_seabed_height(0., y0 + 0.5 * dx) } else { self.seabed_height(0, j) };
            occupation.clear();
            occupation.resize(n3, 0);
            for m in 0..self.n {
                let p = self.x[m];
                if p[0] >= 0. && p[0] < dx && p[1] >= y0 && p[1] < y0 + dx && p[2] >= z0 && p[2] < z0 + dx {
                    let a = ((p[0] / h) as usize).min(PER_AXIS - 1);
                    let b = (((p[1] - y0) / h) as usize).min(PER_AXIS - 1);
                    let c = (((p[2] - z0) / h) as usize).min(PER_AXIS - 1);
                    occupation[(c * PER_AXIS + b) * PER_AXIS + a] += 1;
                }
            }
            while g.reservoir[f] >= quantum {
                let mut choix: Option<(u32, usize)> = None;
                for (i, &o) in occupation.iter().enumerate() {
                    let z = z0 + ((i / (PER_AXIS * PER_AXIS)) as f32 + 0.5) * h;
                    if z > zb + 0.05 * dx && choix.is_none_or(|(o0, _)| o < o0) {
                        choix = Some((o, i));
                    }
                }
                let Some((_, i)) = choix else { break };
                if self.n >= self.x.len() {
                    g.refusees += 1;
                    break;
                }
                let (a, b, c) = (i % PER_AXIS, (i / PER_AXIS) % PER_AXIS, i / (PER_AXIS * PER_AXIS));
                let m = self.n;
                let z = z0 + (c as f32 + 0.5) * h;
                self.x[m] = [(a as f32 + 0.5) * h, y0 + (b as f32 + 0.5) * h, z];
                self.vel[m] = vitesse(z);
                self.c[m] = [[0.; 3]; 3];
                self.n += 1;
                occupation[i] += 1;
                g.reservoir[f] -= quantum;
                g.posees += 1;
            }
        }
        self.gauche = Some(g);
        Ok(())
    }
}
