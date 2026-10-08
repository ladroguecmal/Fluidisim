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
}

/// S709 — les tableaux de la projection de densité, réservés à la configuration (I-06).
pub(crate) struct Densite {
    pub(crate) variante: DensityVariant,
    pub(crate) rho: Vec<f32>,
    pub(crate) du: Vec<f32>,
    pub(crate) dv: Vec<f32>,
    pub(crate) dw: Vec<f32>,
    pub(crate) p_sauve: Vec<f32>,
    /// Le déplacement maximal du dernier pas, m (pour la mesure).
    pub(crate) deplacement_max: f32,
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
        host.alloc.alloc_persistent((2 * cells + fu + fv + fw) * 4).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        self.densite = Some(Box::new(Densite { variante, rho: vec![0.; cells], du: vec![0.; fu], dv: vec![0.; fv], dw: vec![0.; fw],
            p_sauve: vec![0.; cells], deplacement_max: 0. }));
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
                        DensityVariant::WithoutSurface if surface => 0.,
                        DensityVariant::SolidExcessOnly if solide => (dens.rho[c] - 1.).max(0.),
                        _ if surface => (dens.rho[c] - 1.).max(0.),
                        _ => dens.rho[c] - 1.,
                    };
                    excess |= e != 0.;
                    // `A·q = −dx²·Δq` : pour `Δq = e`, le second membre est `−dx²·e`.
                    self.rhs[c] = -dx * dx * e;
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
