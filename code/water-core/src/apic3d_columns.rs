//! **S398 — la zone des colonnes dans APIC 3D**, C5b de la campagne du solveur volumique 3D
//! ([ADR-207](../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) ; conception §4.1 A1, §4.2).
//!
//! Le raccord demande **une seule projection** pour deux représentations : des colonnes là où la surface est un graphe, des
//! particules dans une bande là où elle ne l'est plus. Ce module donne à `Apic3` sa zone de colonnes, désignée par un masque :
//!
//! - la surface y est **`η` par colonne** — `φ = z − η`, sans reconstruction, donc sans le biais qui dépend de l'arrangement
//!   des particules (S323) ; les fractions fantômes qui en sortent sont celles de δ (`ghost_up3`, `ghost_side3`) ;
//! - la vitesse y est **eulérienne**, gardée sur la grille d'un pas à l'autre et **advectée** — semi-lagrangienne, au pied de la
//!   caractéristique : les colonnes du banc 2D qui ne le faisaient pas entretenaient une circulation à la frontière (S397) ;
//! - `η` y est transporté par les **débits mouillés**, hauteur de face moyenne des deux colonnes et somme compensée, comme le
//!   pas mobile de δ (`transport_mobile3`).
//!
//! Sans masque, rien ne change : `Apic3` au bit. **Cette part** : la zone éprouvée seule ; une face entre une colonne et une
//! colonne de particules n'échange encore rien (C5b, deuxième part).
use super::*;

/// La zone des colonnes : masque, surface, copies du pas précédent, débits. Réservée à la configuration (I-06).
pub(crate) struct Columns3 {
    /// 1 : colonne portée par `η` et la vitesse de la grille ; 0 : par les particules.
    pub(crate) mask: Vec<u8>,
    /// Surface absolue, m ; seule celle des colonnes du masque a un sens.
    pub(crate) eta: Vec<f32>,
    /// Reste de la somme compensée de `η`, comme δ (S233).
    pub(crate) eta_roundoff: Vec<f32>,
    pub(crate) prev_u: Vec<f32>,
    pub(crate) prev_v: Vec<f32>,
    pub(crate) prev_w: Vec<f32>,
    /// Débits par unité de largeur à travers les faces de colonnes `x` et `y`, m²/s.
    pub(crate) flux_x: Vec<f32>,
    pub(crate) flux_y: Vec<f32>,
}

/// Octets réservés par `enable_columns` pour `domain`.
pub fn columns_reserved_bytes(domain: Domain3) -> Option<usize> {
    let Domain3 { nx, ny, nz, .. } = domain;
    let columns = nx.checked_mul(ny)?;
    let faces = (nx + 1).checked_mul(ny)?.checked_mul(nz)?
        .checked_add(nx.checked_mul(ny + 1)?.checked_mul(nz)?)?
        .checked_add(columns.checked_mul(nz + 1)?)?;
    let flux = (nx + 1).checked_mul(ny)?.checked_add(nx.checked_mul(ny + 1)?)?;
    // Masque (1 octet), surface et reste (4 + 4) par colonne ; trois copies de faces ; deux familles de débits.
    columns.checked_mul(1 + 4 + 4)?.checked_add(faces.checked_mul(4)?)?.checked_add(flux.checked_mul(4)?)
}

/// Vitesse d'un champ MAC en un point, trilinéaire par composante — celle de `grid_velocity`, sur des tableaux donnés.
fn sample(domain: Domain3, u: &[f32], v: &[f32], w: &[f32], p: [f32; 3]) -> [f32; 3] {
    let mut out = [0f32; 3];
    for axis in 0..3 {
        let (origin, dims) = staggered(domain, axis);
        let field = match axis {
            0 => u,
            1 => v,
            _ => w,
        };
        for (idx, wt, _) in weights(p, domain.dx, origin, dims) {
            out[axis] += wt * field[idx];
        }
    }
    out
}

impl Apic3 {
    /// **Active la zone des colonnes** désignée par `mask` (une valeur par colonne, `x` le plus rapide ; non nul : colonne).
    /// **À l'initialisation, avant `seal()`** (I-06). La surface part au fond ; `set_columns_surface` la pose. Refus `Shape`
    /// (longueur), `Domain` (hôte qui refuse, ou zone déjà active).
    pub fn enable_columns(&mut self, host: &mut HostServices, mask: &[u8]) -> Result<(), Error> {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        if mask.len() != nx * ny {
            return Err(Error::Shape);
        }
        if self.columns.is_some() {
            return Err(Error::Domain);
        }
        let bytes = columns_reserved_bytes(self.domain).ok_or(Error::Domain)?;
        host.alloc.alloc_persistent(bytes).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        self.columns = Some(Columns3 {
            mask: mask.iter().map(|m| (*m != 0) as u8).collect(),
            eta: vec![0.; nx * ny],
            eta_roundoff: vec![0.; nx * ny],
            prev_u: vec![0.; self.u.len()],
            prev_v: vec![0.; self.v.len()],
            prev_w: vec![0.; nx * ny * (nz + 1)],
            flux_x: vec![0.; (nx + 1) * ny],
            flux_y: vec![0.; nx * (ny + 1)],
        });
        Ok(())
    }

    /// Pose la surface absolue des colonnes, m (une valeur par colonne). Refus `Domain` sans zone, `Shape` (longueur),
    /// `NotFinite`.
    pub fn set_columns_surface(&mut self, eta: &[f32]) -> Result<(), Error> {
        let cols = self.columns.as_mut().ok_or(Error::Domain)?;
        if eta.len() != cols.eta.len() {
            return Err(Error::Shape);
        }
        if eta.iter().any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        cols.eta.copy_from_slice(eta);
        cols.eta_roundoff.fill(0.);
        Ok(())
    }

    /// La surface des colonnes, s'il y a une zone.
    pub fn columns_surface(&self) -> Option<&[f32]> {
        self.columns.as_ref().map(|c| &c.eta[..])
    }

    /// Le volume d'eau des colonnes du masque, m³ (`Σ η·dx²`, somme compensée comprise), en `f64`.
    pub fn columns_volume(&self) -> f64 {
        let Some(c) = &self.columns else { return 0. };
        let a = (self.domain.dx as f64).powi(2);
        c.mask.iter().zip(c.eta.iter().zip(&c.eta_roundoff)).filter(|(m, _)| **m != 0).map(|(_, (e, r))| (*e as f64 - *r as f64) * a).sum()
    }

    #[inline]
    pub(crate) fn column_of(&self, i: usize, j: usize) -> bool {
        self.columns.as_ref().is_some_and(|c| c.mask[j * self.domain.nx + i] != 0)
    }

    /// Au début du pas : la vitesse de la grille, celle de la fin du pas précédent, gardée pour l'advection.
    pub(crate) fn columns_begin(&mut self) {
        let Some(c) = self.columns.as_mut() else { return };
        c.prev_u.copy_from_slice(&self.u);
        c.prev_v.copy_from_slice(&self.v);
        c.prev_w.copy_from_slice(&self.w);
    }

    /// Après le transfert des particules : les faces de la zone des colonnes prennent la vitesse du pas précédent, advectée
    /// **au pied de la caractéristique** — `u(x_f) ← u⁻(x_f − dt·u⁻(x_f))`. Une face `u` en est si ses deux colonnes en sont
    /// (sa seule colonne, au bord) ; une face `w`, si sa colonne en est.
    pub(crate) fn columns_advect(&mut self, dt: f32) {
        let Some(c) = self.columns.as_ref() else { return };
        let domain = self.domain;
        let Domain3 { nx, ny, nz, dx } = domain;
        let inside = |i: isize, j: isize| i >= 0 && j >= 0 && (i as usize) < nx && (j as usize) < ny && c.mask[j as usize * nx + i as usize] != 0;
        let (pu, pv, pw) = (&c.prev_u[..], &c.prev_v[..], &c.prev_w[..]);
        let advected = |x: [f32; 3], axis: usize| {
            let v = sample(domain, pu, pv, pw, x);
            let foot = [x[0] - dt * v[0], x[1] - dt * v[1], x[2] - dt * v[2]];
            sample(domain, pu, pv, pw, foot)[axis]
        };
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..=nx {
                    let (a, b) = (inside(i as isize - 1, j as isize), inside(i as isize, j as isize));
                    if (a || i == 0) && (b || i == nx) && (a || b) {
                        let x = [i as f32 * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx];
                        self.u[(k * ny + j) * (nx + 1) + i] = advected(x, 0);
                    }
                }
            }
            for j in 0..=ny {
                for i in 0..nx {
                    let (a, b) = (inside(i as isize, j as isize - 1), inside(i as isize, j as isize));
                    if (a || j == 0) && (b || j == ny) && (a || b) {
                        let x = [(i as f32 + 0.5) * dx, j as f32 * dx, (k as f32 + 0.5) * dx];
                        self.v[(k * (ny + 1) + j) * nx + i] = advected(x, 1);
                    }
                }
            }
        }
        for k in 0..=nz {
            for j in 0..ny {
                for i in 0..nx {
                    if inside(i as isize, j as isize) {
                        let x = [(i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, k as f32 * dx];
                        self.w[(k * ny + j) * nx + i] = advected(x, 2);
                    }
                }
            }
        }
    }

    /// Après la reconstruction : dans les colonnes, `φ = z − η` et l'eau sous `η`.
    pub(crate) fn columns_label(&mut self) {
        let Some(c) = self.columns.as_ref() else { return };
        let Domain3 { nx, ny, nz, dx } = self.domain;
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                if c.mask[col] == 0 {
                    continue;
                }
                for k in 0..nz {
                    let cell = (k * ny + j) * nx + i;
                    let phi = (k as f32 + 0.5) * dx - c.eta[col];
                    self.phi[cell] = phi;
                    self.label[cell] = if phi < 0. { WATER } else { AIR };
                }
            }
        }
    }

    /// Après la projection et l'extrapolation : `η` transporté par les débits mouillés entre colonnes de la zone — hauteur de
    /// face moyenne des deux colonnes, somme compensée —, comme le pas mobile de δ. Une face vers une colonne de particules, ou
    /// un mur, ne porte rien (cette part).
    pub(crate) fn columns_transport(&mut self, dt: f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let transport = (dt as f64 / dx as f64) as f32;
        let (u, v) = (&self.u, &self.v);
        let Some(c) = self.columns.as_mut() else { return };
        c.flux_x.fill(0.);
        c.flux_y.fill(0.);
        for j in 0..ny {
            for i in 0..nx {
                for axis in 0..2 {
                    if (axis == 0 && i == 0) || (axis == 1 && j == 0) {
                        continue;
                    }
                    let (x, y) = if axis == 0 { (i - 1, j) } else { (i, j - 1) };
                    if c.mask[y * nx + x] == 0 || c.mask[j * nx + i] == 0 {
                        continue;
                    }
                    let surface = 0.5 * (c.eta[y * nx + x] + c.eta[j * nx + i]);
                    let mut q = 0f32;
                    for k in 0..nz {
                        let wet = ((surface - k as f32 * dx) / dx).clamp(0., 1.);
                        if wet == 0. {
                            break;
                        }
                        let vel = if axis == 0 { u[(k * ny + j) * (nx + 1) + i] } else { v[(k * (ny + 1) + j) * nx + i] };
                        q += vel * dx * wet;
                    }
                    if axis == 0 {
                        c.flux_x[j * (nx + 1) + i] = q;
                    } else {
                        c.flux_y[j * nx + i] = q;
                    }
                }
            }
        }
        for j in 0..ny {
            for i in 0..nx {
                let col = j * nx + i;
                if c.mask[col] == 0 {
                    continue;
                }
                let x = c.flux_x[j * (nx + 1) + i + 1] - c.flux_x[j * (nx + 1) + i];
                let y = c.flux_y[(j + 1) * nx + i] - c.flux_y[j * nx + i];
                let increment = -transport * (x + y) - c.eta_roundoff[col];
                let height = c.eta[col] + increment;
                c.eta_roundoff[col] = (height - c.eta[col]) - increment;
                c.eta[col] = height;
            }
        }
    }

    /// La plus grande vitesse de la grille sur les faces, m/s — ce que la zone des colonnes ajoute au pas stable.
    pub(crate) fn columns_max_speed(&self) -> f32 {
        if self.columns.is_none() {
            return 0.;
        }
        self.u.iter().chain(&self.v).chain(&self.w).fold(0f32, |m, x| m.max(x.abs()))
    }

    /// La zone est-elle finie ?
    pub(crate) fn columns_finite(&self) -> bool {
        self.columns.as_ref().map_or(true, |c| c.eta.iter().chain(&c.eta_roundoff).all(|x| x.is_finite()))
            && (self.columns.is_none() || self.u.iter().chain(&self.v).chain(&self.w).all(|x| x.is_finite()))
    }

    /// **S399 — les particules virtuelles des colonnes**, pour la reconstruction d'une maille de la bande au centre `q` : chaque
    /// colonne de la zone à portée du noyau compte comme `2 × 2` particules par rangée, `round(2η/dx)` rangées étirées sur
    /// `[0, η]` — la densité nominale, sans marche quand `η` passe un quart de maille (S327) ; les images aux parois sont celles
    /// des particules. C'est l'idée du champ de densité de Chentanez, Müller et Kim : la grille ajoute sa part à celle des
    /// particules, et la frontière n'est plus une paroi vue d'un seul côté. Rend `(Σw, Σw·p)`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn virtual_column_sums(&self, q: [f32; 3], i: usize, j: usize, reach: usize, radius: f32, inv_r2: f32,
        images_xy: [[Option<f32>; 3]; 2], images_z: [Option<f32>; 2]) -> (f32, [f32; 3]) {
        let Some(c) = self.columns.as_ref() else { return (0., [0.; 3]) };
        let Domain3 { nx, ny, dx, .. } = self.domain;
        let (lx, ly) = (nx as f32 * dx, ny as f32 * dx);
        let mirror = |v: f32, m: f32, l: f32| if m == 1. { v } else if m == -1. { -v } else { 2. * l - v };
        let (mut sw, mut sx) = (0f32, [0f32; 3]);
        for b in j.saturating_sub(reach)..(j + reach + 1).min(ny) {
            for a in i.saturating_sub(reach)..(i + reach + 1).min(nx) {
                let col = b * nx + a;
                if c.mask[col] == 0 {
                    continue;
                }
                let eta = c.eta[col].max(0.);
                let rows = ((2. * eta / dx).round() as usize).max(1);
                let pitch = eta / rows as f32;
                // Les rangées à portée verticale du noyau (images au fond comprises : |z| suffit).
                let lo = (((q[2] - radius).max(0.) / pitch) - 0.5).floor().max(0.) as usize;
                let hi = ((((q[2] + radius) / pitch) - 0.5).ceil().max(0.) as usize).min(rows - 1);
                for r in lo..=hi {
                    let z = (r as f32 + 0.5) * pitch;
                    for (ox, oy) in [(0.25f32, 0.25f32), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
                        let p0 = [(a as f32 + ox) * dx, (b as f32 + oy) * dx, z];
                        for mx in images_xy[0].iter().flatten() {
                            for my in images_xy[1].iter().flatten() {
                                for mz in images_z.iter().flatten() {
                                    let p = [mirror(p0[0], *mx, lx), mirror(p0[1], *my, ly), mirror(p0[2], *mz, 0.)];
                                    let d = [p[0] - q[0], p[1] - q[1], p[2] - q[2]];
                                    let wt = kernel((d[0] * d[0] + d[1] * d[1] + d[2] * d[2]) * inv_r2);
                                    if wt > 0. {
                                        sw += wt;
                                        for m in 0..3 {
                                            sx[m] += wt * p[m];
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        (sw, sx)
    }

    /// Une colonne de la zone ? (pour les essais et le banc)
    pub fn is_column(&self, i: usize, j: usize) -> bool {
        self.column_of(i, j)
    }
}
