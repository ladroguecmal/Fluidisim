//! **S709 — la projection de densité** (Kugelstadt et al. 2019, *Implicit density projection for volume conserving liquids*) : S708 a
//! mesuré qu'APIC, à compte de particules exact, perd ≈ 1,3 % par seconde de volume géométrique dès que l'eau bouge — ses particules se
//! tassent (+3,8 % sous la crête, S707), et la séparation n'écarte que les paires à moins de 0,4 maille.
//!
//! À la fin de chaque pas, après la séparation :
//!
//! - la densité des particules aux centres des mailles, par les poids trilinéaires, rapportée à la nominale (`PER_AXIS³`) ;
//! - sur les mailles d'eau, `Δq = ρ − 1` à l'intérieur et `max(ρ − 1, 0)` à la surface, la surface à `q = 0` (la fraction fantôme de la pression), les parois sans flux — le
//!   gradient conjugué de la pression, sur ses propres tableaux sauvés et rendus ;
//! - chaque particule se déplace de `∇q` (borné à un quart de maille) : la divergence du déplacement vaut l'excès, la densité revient
//!   à 1. **Les vitesses ne changent pas.**
//!
//! Au repos, la densité vaut 1 partout (0 moins aux mailles de surface) : rien ne bouge.

use super::*;

/// **S709 — la variante de la projection de densité** (les témoins de E3, une cause chacun).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DensityVariant {
    /// L'intérieur dans les deux sens, la surface en excès seul.
    Complete,
    /// Sans correction aux mailles de surface (voisines de l'air).
    WithoutSurface,
    /// Comme `Complete`, mais l'excès seul aux mailles voisines d'une paroi solide (les poids trilinéaires s'y perdent dans le solide).
    SolidExcessOnly,
    /// **S747 — hybride** : comme `Complete` dans une colonne qui porte au moins `LAME_MINCE_S747` mailles d'eau ; aucune correction dans
    /// une colonne plus mince (la lame, le jet de rive). `Complete` garde l'onde solitaire mais freine la lame (S744) ; `WithoutSurface` libère
    /// la lame mais laisse une traîne derrière l'onde (S745).
    Hybrid,
}

/// S747 — le nombre de mailles d'eau d'une colonne sous lequel la variante hybride ne corrige rien (la règle du film du rivage, S678).
pub const LAME_MINCE_S747: u16 = 3;

/// S709 — les tableaux de la projection de densité, réservés à la configuration (I-06).
pub(crate) struct Densite {
    pub(crate) variante: DensityVariant,
    /// S710 : la fraction de l'écart corrigée par pas (1, le défaut de S709).
    pub(crate) relaxation: f32,
    pub(crate) rho: Vec<f32>,
    pub(crate) du: Vec<f32>,
    pub(crate) dv: Vec<f32>,
    pub(crate) dw: Vec<f32>,
    pub(crate) p_sauve: Vec<f32>,
    /// Le déplacement maximal du dernier pas, m (pour la mesure).
    pub(crate) deplacement_max: f32,
    /// **S744 — la densité consciente du fond** (`set_density_bed_aware`) : la densité d'une maille rapportée à sa nominale, celle d'un
    /// réseau régulier posé partout hors du fond ; `false`, le défaut : rapportée à 1 (S709, au bit).
    pub(crate) conscient: bool,
    /// S744 — la nominale de chaque maille, calculée au premier pas (`nominale_faite`).
    pub(crate) nominale: Vec<f32>,
    pub(crate) nominale_faite: bool,
    /// S747 — les mailles d'eau de chaque colonne (`nx·ny`), comptées à chaque pas pour la variante hybride.
    pub(crate) colonnes: Vec<u16>,
}

impl Apic3 {
    /// **S709 — la projection de densité**, en option (`None`, le défaut — au bit). Réservé à la configuration (I-06).
    pub fn enable_density_projection(&mut self, host: &mut HostServices) -> Result<(), Error> {
        self.enable_density_projection_variant(host, DensityVariant::Complete)
    }

    /// S709 — la même, avec une variante (les témoins).
    pub fn enable_density_projection_variant(&mut self, host: &mut HostServices, variante: DensityVariant) -> Result<(), Error> {
        if self.densite.is_some() {
            return Err(Error::Domain);
        }
        let Domain3 { nx, ny, nz, .. } = self.domain;
        let cells = nx * ny * nz;
        let (fu, fv, fw) = ((nx + 1) * ny * nz, nx * (ny + 1) * nz, nx * ny * (nz + 1));
        host.alloc.alloc_persistent((3 * cells + fu + fv + fw) * 4 + nx * ny * 2).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        self.densite = Some(Box::new(Densite { variante, relaxation: 1., rho: vec![0.; cells], du: vec![0.; fu], dv: vec![0.; fv], dw: vec![0.; fw],
            p_sauve: vec![0.; cells], deplacement_max: 0., conscient: false, nominale: vec![0.; cells], nominale_faite: false,
            colonnes: vec![0; nx * ny] }));
        Ok(())
    }

    /// **S744 — la projection de densité consciente du fond** (REPOS-PENTE-S743) : la densité de chaque maille est rapportée à sa nominale,
    /// `Σ w/8` d'un réseau régulier de `2 × 2 × 2` points par maille posé partout hors du fond (l'escalier ou le fond lisse), calculée au
    /// premier pas — le fond doit être posé avant. Sans elle, les poids qui tombent dans le solide font paraître creuses les mailles qui touchent
    /// le fond, et la projection les comble. Refus sans projection.
    pub fn set_density_bed_aware(&mut self, on: bool) -> Result<(), Error> {
        let Some(d) = self.densite.as_mut() else { return Err(Error::Domain) };
        d.conscient = on;
        d.nominale_faite = false;
        Ok(())
    }

    /// **S710 — la projection faible** : ne corriger qu'une fraction `κ ∈ (0, 1]` de l'écart de densité par pas (S709 : κ = 1 lisse le
    /// front et empêche le plongeon). Refus sans projection, ou hors de `(0, 1]`.
    pub fn set_density_relaxation(&mut self, kappa: f32) -> Result<(), Error> {
        let Some(d) = self.densite.as_mut() else { return Err(Error::Domain) };
        if !(kappa > 0. && kappa <= 1.) {
            return Err(Error::Domain);
        }
        d.relaxation = kappa;
        Ok(())
    }

    /// S709 — le déplacement maximal du dernier pas (m) ; `None` sans projection de densité.
    pub fn density_projection_shift(&self) -> Option<f32> {
        self.densite.as_ref().map(|d| d.deplacement_max)
    }

    /// S709 — la projection de densité (rien sans `enable_density_projection`).
    pub(crate) fn density_project(&mut self) {
        let Some(mut dens) = self.densite.take() else { return };
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let nominal = (PER_AXIS * PER_AXIS * PER_AXIS) as f32;
        // La densité aux centres des mailles, par les poids trilinéaires.
        dens.rho.fill(0.);
        for k in 0..self.n {
            if self.is_droplet(k) {
                continue;
            }
            for (idx, wt, _) in weights(self.x[k], dx, [0.5; 3], [nx, ny, nz]) {
                dens.rho[idx] += wt / nominal;
            }
        }
        // S744 : la densité rapportée à la nominale de chaque maille (le fond compté).
        if dens.conscient {
            if !dens.nominale_faite {
                dens.nominale.fill(0.);
                let pas = 1. / PER_AXIS as f32;
                for k in 0..nz {
                    for j in 0..ny {
                        for i in 0..nx {
                            for a in 0..PER_AXIS * PER_AXIS * PER_AXIS {
                                let (ax, ay, az) = (a % PER_AXIS, (a / PER_AXIS) % PER_AXIS, a / (PER_AXIS * PER_AXIS));
                                let q = [(i as f32 + (ax as f32 + 0.5) * pas) * dx, (j as f32 + (ay as f32 + 0.5) * pas) * dx,
                                    (k as f32 + (az as f32 + 0.5) * pas) * dx];
                                let sous = if self.lisse.is_some() {
                                    q[2] < self.smooth_seabed_height(q[0], q[1])
                                } else {
                                    q[2] < self.seabed_height(i, j)
                                };
                                if sous {
                                    continue;
                                }
                                for (idx, wt, _) in weights(q, dx, [0.5; 3], [nx, ny, nz]) {
                                    dens.nominale[idx] += wt / nominal;
                                }
                            }
                        }
                    }
                }
                dens.nominale_faite = true;
            }
            for (r, n) in dens.rho.iter_mut().zip(&dens.nominale) {
                if *n > 0.05 {
                    *r /= *n;
                }
            }
        }
        // S747 : les mailles d'eau de chaque colonne, pour la variante hybride.
        if dens.variante == DensityVariant::Hybrid {
            for j in 0..ny {
                for i in 0..nx {
                    dens.colonnes[j * nx + i] = (0..nz).filter(|&k| self.label[self.cell(i, j, k)] == WATER).count().min(u16::MAX as usize) as u16;
                }
            }
        }
        // Le second membre et la diagonale, comme la pression : la surface à q = 0 (la fraction fantôme), les parois sans flux.
        dens.p_sauve.copy_from_slice(&self.p);
        let mut excess = false;
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
                    let (mut diag, mut surface, mut solide) = (0f32, false, false);
                    for (n, f, axis) in self.neighbours(i, j, k).into_iter().flatten() {
                        let a = self.fraction(axis, f);
                        match self.label[n] {
                            WATER => diag += a,
                            AIR => {
                                diag += a / self.theta(c, n);
                                surface = true;
                            }
                            _ => solide = true,
                        }
                    }
                    // E2 de S709 : corrigé d'un seul côté (l'excès), le bruit de la densité dilatait l'eau à chaque pas (+7,4 % en
                    // 1,6 s). À l'intérieur, l'écart dans les deux sens ; à la surface, l'excès seul (une maille de surface est en partie
                    // vide, sa densité basse est normale).
                    let e = match dens.variante {
                        DensityVariant::Hybrid if dens.colonnes[j * nx + i] < LAME_MINCE_S747 => 0.,
                        DensityVariant::WithoutSurface if surface => 0.,
                        DensityVariant::SolidExcessOnly if solide => (dens.rho[c] - 1.).max(0.),
                        _ if surface => (dens.rho[c] - 1.).max(0.),
                        _ => dens.rho[c] - 1.,
                    };
                    excess |= e != 0.;
                    // `A·q = −dx²·Δq` : pour `Δq = e`, le second membre est `−dx²·e`.
                    self.rhs[c] = -dx * dx * e * dens.relaxation;
                    self.diag[c] = diag;
                }
            }
        }
        dens.deplacement_max = 0.;
        if excess {
            self.pcg();
            // Le déplacement aux faces : `∇q`, la surface à q = 0 à la fraction fantôme, rien à une paroi ni à une face fermée.
            dens.du.fill(0.);
            dens.dv.fill(0.);
            dens.dw.fill(0.);
            for k in 0..nz {
                for j in 0..ny {
                    for i in 0..nx {
                        let c = self.cell(i, j, k);
                        for (m, nb) in self.neighbours(i, j, k).into_iter().enumerate() {
                            if m % 2 == 0 {
                                continue;
                            }
                            let Some((n, f, axis)) = nb else { continue };
                            if self.label[c] == SOLID || self.label[n] == SOLID || self.fraction(axis, f) == 0. {
                                continue;
                            }
                            let (wc, wn) = (self.label[c] == WATER, self.label[n] == WATER);
                            let g = if wc && wn {
                                (self.p[n] - self.p[c]) / dx
                            } else if wc {
                                -self.p[c] / (self.theta(c, n) * dx)
                            } else if wn {
                                self.p[n] / (self.theta(n, c) * dx)
                            } else {
                                continue;
                            };
                            match axis {
                                0 => dens.du[f] = g,
                                1 => dens.dv[f] = g,
                                _ => dens.dw[f] = g,
                            }
                        }
                    }
                }
            }
            // Chaque particule se déplace de `∇q` interpolé, borné à un quart de maille ; les vitesses ne changent pas.
            let borne = 0.25 * dx;
            for k in 0..self.n {
                if self.is_droplet(k) {
                    continue;
                }
                let mut d = [0f32; 3];
                for (axis, field) in [&dens.du, &dens.dv, &dens.dw].into_iter().enumerate() {
                    let (origin, dims) = staggered(self.domain, axis);
                    for (idx, wt, _) in weights(self.x[k], dx, origin, dims) {
                        d[axis] += wt * field[idx];
                    }
                }
                let norme = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                let s = if norme > borne { borne / norme } else { 1. };
                for a in 0..3 {
                    self.x[k][a] += s * d[a];
                }
                dens.deplacement_max = dens.deplacement_max.max(s * norme);
            }
            self.bin_fresh = false;
        }
        self.p.copy_from_slice(&dens.p_sauve);
        self.densite = Some(dens);
    }
}
