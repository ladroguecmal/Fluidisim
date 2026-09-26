//! **S388 — APIC en trois dimensions**, la seconde représentation de surface libre de δ
//! ([ADR-186](../../docs/adr/ADR-186-apic-seconde-representation.md)), C4 de la campagne du solveur volumique 3D
//! ([ADR-207](../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)).
//!
//! Le candidat 2D de S318–S320 (`examples/lot5_comparaison.rs`) porté aux trois dimensions, **avec ses leçons** :
//!
//! - des particules portent l'eau — position, vitesse et matrice affine `C` (APIC, Jiang et al. 2015) — ; une grille MAC
//!   calcule la pression ; les transferts sont **trilinéaires** ;
//! - la surface se **reconstruit des particules** (Zhu et Bridson 2005) : aux centres des mailles, `φ = |q − x̄| − r`, `x̄` la
//!   position moyenne des particules voisines ; la pression voit l'iso-zéro à une fraction de maille (fluide fantôme). En 2D,
//!   `p = 0` au centre des mailles d'air éteignait un ballottement en trois secondes (S318, faute 1) ;
//! - les faces d'air sont **extrapolées** sur trois couches puis **remises à zéro**, sauf celles que des particules
//!   alimentent — sans quoi elles accumulent la gravité (faute 5), ou les particules juste au-dessus de la surface gardent une
//!   vitesse balistique et l'énergie monte (faute 6) ;
//! - les particules trop proches sont **séparées** (0,4 maille, deux passes), en position seulement (S320 P3).
//!
//! Huit particules par maille (2 × 2 × 2), `f32` local (I-08), `g_eff` injecté (I-07), tous les tampons réservés à la
//! configuration auprès de l'hôte (I-06) : le pas n'alloue rien. Aucune grandeur de jeu n'en sort (I-04), rien n'est
//! sérialisé (I-17). Le bord du domaine est une paroi.

use crate::delta3d::Domain3;
use crate::delta_projection::Error;
use crate::host::{AllocError, HostServices};

/// Particules posées par maille et par direction : 2 × 2 × 2 = 8.
pub const PER_AXIS: usize = 2;

/// Une maille d'eau, d'air ou de paroi.
pub const AIR: u8 = 0;
pub const WATER: u8 = 1;

/// La référence APIC 3D : grille MAC, particules et tampons de travail, tous réservés à la configuration.
pub struct Apic3 {
    pub(crate) domain: Domain3,
    pub(crate) rho: f32,
    /// **Fournie**, jamais codée en dur (I-07).
    pub(crate) g_eff: f32,
    // Grille : vitesses aux faces, leurs poids de transfert, drapeaux de validité ; pression, distance, étiquettes.
    pub(crate) u: Vec<f32>,
    pub(crate) v: Vec<f32>,
    pub(crate) w: Vec<f32>,
    pub(crate) wu: Vec<f32>,
    pub(crate) wv: Vec<f32>,
    pub(crate) ww: Vec<f32>,
    pub(crate) valid_u: Vec<u8>,
    pub(crate) valid_v: Vec<u8>,
    pub(crate) valid_w: Vec<u8>,
    /// Copie d'une composante pendant l'extrapolation, à la taille du plus grand jeu de faces.
    pub(crate) old_u: Vec<f32>,
    pub(crate) old_valid: Vec<u8>,
    pub(crate) p: Vec<f32>,
    pub(crate) phi: Vec<f32>,
    pub(crate) label: Vec<u8>,
    pub(crate) rhs: Vec<f32>,
    pub(crate) r: Vec<f32>,
    pub(crate) z: Vec<f32>,
    pub(crate) d: Vec<f32>,
    pub(crate) q: Vec<f32>,
    pub(crate) diag: Vec<f32>,
    // Tri des particules par maille : décompte, début de chaque maille, ordre.
    pub(crate) bin_count: Vec<u32>,
    pub(crate) bin_start: Vec<u32>,
    pub(crate) order: Vec<u32>,
    // Particules : `n` actives sur `capacity`.
    pub(crate) n: usize,
    pub(crate) x: Vec<[f32; 3]>,
    pub(crate) vel: Vec<[f32; 3]>,
    pub(crate) c: Vec<[[f32; 3]; 3]>,
    pub(crate) shift: Vec<[f32; 3]>,
    /// Rayon de la reconstruction, calculé pour qu'une nappe au repos ait son iso-zéro à sa hauteur (P4).
    pub(crate) radius: f32,
    /// Itérations du dernier gradient conjugué.
    pub(crate) iterations: u32,
    /// Séparation des particules active (S320) ; la couper sert à la mesure.
    pub(crate) separation: bool,
}

/// Flottants (4 octets) et octets que la configuration réserve pour `domain` et `capacity` particules.
pub fn reserved_bytes(domain: Domain3, capacity: usize) -> Option<usize> {
    let Domain3 { nx, ny, nz, .. } = domain;
    let cells = nx.checked_mul(ny)?.checked_mul(nz)?;
    let nu = (nx + 1).checked_mul(ny)?.checked_mul(nz)?;
    let nv = nx.checked_mul(ny + 1)?.checked_mul(nz)?;
    let nw = nx.checked_mul(ny)?.checked_mul(nz + 1)?;
    let faces = nu.checked_add(nv)?.checked_add(nw)?;
    let largest = nu.max(nv).max(nw);
    // Faces : vitesse et poids (4 + 4 octets), drapeau (1) ; une copie de composante et de ses drapeaux, à la taille du plus
    // grand des trois jeux (4 + 1).
    let face_bytes = faces.checked_mul(4 + 4 + 1)?.checked_add(largest.checked_mul(4 + 1)?)?;
    // Mailles : p, φ, rhs, r, z, d, q, diag (4 octets), étiquette (1), décompte et début (4 + 4, plus un début).
    let cell_bytes = cells.checked_mul(8 * 4 + 1 + 8)?.checked_add(4)?;
    // Particules : x, vitesse, C, décalage (18 flottants), ordre (4 octets).
    let particle_bytes = capacity.checked_mul(18 * 4 + 4)?;
    face_bytes.checked_add(cell_bytes)?.checked_add(particle_bytes)
}

impl Apic3 {
    /// Construit la référence. **À l'initialisation, avant `seal()`** (I-06) : grille et `capacity` particules comptées
    /// auprès de l'hôte, à leur taille réelle. Refus : dimensions nulles ou `dx` non fini (`Domain`) ; densité ou gravité non
    /// finies (`NotFinite`) ; hôte qui refuse (`Domain`).
    pub fn configure(host: &mut HostServices, domain: Domain3, rho: f32, g_eff: f32, capacity: usize) -> Result<Self, Error> {
        let Domain3 { nx, ny, nz, dx } = domain;
        if nx == 0 || ny == 0 || nz == 0 || !dx.is_finite() || dx <= 0. || capacity == 0 {
            return Err(Error::Domain);
        }
        if !rho.is_finite() || rho <= 0. || !g_eff.is_finite() {
            return Err(Error::NotFinite);
        }
        let bytes = reserved_bytes(domain, capacity).ok_or(Error::Domain)?;
        host.alloc.alloc_persistent(bytes).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        let cells = nx * ny * nz;
        let (nu, nv, nw) = ((nx + 1) * ny * nz, nx * (ny + 1) * nz, nx * ny * (nz + 1));
        let largest = nu.max(nv).max(nw);
        Ok(Apic3 {
            domain,
            rho,
            g_eff,
            u: vec![0.; nu],
            v: vec![0.; nv],
            w: vec![0.; nw],
            wu: vec![0.; nu],
            wv: vec![0.; nv],
            ww: vec![0.; nw],
            valid_u: vec![0; nu],
            valid_v: vec![0; nv],
            valid_w: vec![0; nw],
            old_u: vec![0.; largest],
            old_valid: vec![0; largest],
            p: vec![0.; cells],
            phi: vec![0.; cells],
            label: vec![AIR; cells],
            rhs: vec![0.; cells],
            r: vec![0.; cells],
            z: vec![0.; cells],
            d: vec![0.; cells],
            q: vec![0.; cells],
            diag: vec![0.; cells],
            bin_count: vec![0; cells],
            bin_start: vec![0; cells + 1],
            order: vec![0; capacity],
            n: 0,
            x: vec![[0.; 3]; capacity],
            vel: vec![[0.; 3]; capacity],
            c: vec![[[0.; 3]; 3]; capacity],
            shift: vec![[0.; 3]; capacity],
            radius: rest_radius(dx),
            iterations: 0,
            separation: true,
        })
    }

    pub fn domain(&self) -> Domain3 {
        self.domain
    }
    /// Les particules actives.
    pub fn particles(&self) -> &[[f32; 3]] {
        &self.x[..self.n]
    }
    pub fn velocities(&self) -> &[[f32; 3]] {
        &self.vel[..self.n]
    }
    pub fn particle_count(&self) -> usize {
        self.n
    }
    /// Masse d'une particule, kg : une maille d'eau partagée entre huit.
    pub fn particle_mass(&self) -> f32 {
        let dx = self.domain.dx;
        self.rho * dx * dx * dx / (PER_AXIS * PER_AXIS * PER_AXIS) as f32
    }
    /// Itérations du dernier gradient conjugué.
    pub fn iterations(&self) -> u32 {
        self.iterations
    }
    /// **Pour la mesure** : le rayon de la reconstruction (défaut : `rest_radius`, le minimax de S388).
    pub fn set_reconstruction_radius(&mut self, r: f32) {
        self.radius = r;
    }
    /// **Pour la mesure** : la séparation des particules (défaut : active).
    pub fn set_separation(&mut self, on: bool) {
        self.separation = on;
    }
    /// Distance signée reconstruite aux centres des mailles (`x` le plus rapide, puis `y`, puis `z`).
    pub fn distance(&self) -> &[f32] {
        &self.phi
    }

    #[inline]
    pub(crate) fn cell(&self, i: usize, j: usize, k: usize) -> usize {
        (k * self.domain.ny + j) * self.domain.nx + i
    }

    /// La maille qui contient un point, bornée au domaine.
    #[inline]
    pub(crate) fn cell_of(&self, p: [f32; 3]) -> (usize, usize, usize) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let f = |x: f32, n: usize| ((x / dx).max(0.) as usize).min(n - 1);
        (f(p[0], nx), f(p[1], ny), f(p[2], nz))
    }

    /// **Ensemence** les mailles dont les huit positions d'une grille au quart de maille tombent dans `inside` : une
    /// particule par position intérieure, vitesse et `C` nuls. Remplace les particules présentes. Refus `Domain` si la
    /// capacité réservée ne suffit pas — rien n'est alors changé.
    pub fn seed(&mut self, inside: &dyn Fn([f32; 3]) -> bool) -> Result<usize, Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let step = 1. / PER_AXIS as f32;
        let position = |i: usize, j: usize, k: usize, a: usize| {
            let (ax, ay, az) = (a % PER_AXIS, (a / PER_AXIS) % PER_AXIS, a / (PER_AXIS * PER_AXIS));
            [
                (i as f32 + (ax as f32 + 0.5) * step) * dx,
                (j as f32 + (ay as f32 + 0.5) * step) * dx,
                (k as f32 + (az as f32 + 0.5) * step) * dx,
            ]
        };
        let per_cell = PER_AXIS * PER_AXIS * PER_AXIS;
        let mut count = 0usize;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    count += (0..per_cell).filter(|a| inside(position(i, j, k, *a))).count();
                }
            }
        }
        if count > self.x.len() {
            return Err(Error::Domain);
        }
        let mut m = 0;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    for a in 0..per_cell {
                        let p = position(i, j, k, a);
                        if inside(p) {
                            self.x[m] = p;
                            self.vel[m] = [0.; 3];
                            self.c[m] = [[0.; 3]; 3];
                            m += 1;
                        }
                    }
                }
            }
        }
        self.n = m;
        Ok(m)
    }

    /// **Tri des particules par maille** (comptage) : `bin_start[c]..bin_start[c + 1]` indexe `order`. Aucune allocation.
    pub(crate) fn bin(&mut self) {
        self.bin_count.fill(0);
        for k in 0..self.n {
            let (i, j, l) = self.cell_of(self.x[k]);
            let c = self.cell(i, j, l);
            self.bin_count[c] += 1;
        }
        let mut s = 0u32;
        for c in 0..self.bin_count.len() {
            self.bin_start[c] = s;
            s += self.bin_count[c];
        }
        let cells = self.bin_count.len();
        self.bin_start[cells] = s;
        self.bin_count.fill(0);
        for k in 0..self.n {
            let (i, j, l) = self.cell_of(self.x[k]);
            let c = self.cell(i, j, l);
            let slot = self.bin_start[c] + self.bin_count[c];
            self.order[slot as usize] = k as u32;
            self.bin_count[c] += 1;
        }
    }
}

/// Le noyau de la reconstruction, `(1 − s²/R²)³` pour `s < R`.
#[inline]
pub(crate) fn kernel(s2_over_r2: f32) -> f32 {
    if s2_over_r2 < 1. {
        let t = 1. - s2_over_r2;
        t * t * t
    } else {
        0.
    }
}

/// **Le rayon au repos**, réglé en S388 (P4) sur ce que la pression lit : l'iso-zéro **interpolée entre deux centres de
/// maille**. Sur une nappe régulière de particules au quart de maille, la hauteur d'eau que les particules portent tombe
/// soit sur une face de maille, soit sur un centre (rangées de `dx/2`). Un rayon unique ne met pas les deux à zéro :
/// `r = d₀` (S318, la distance au point de la surface) lit un centre exactement et une face à −15 % de maille ; `r` réglé
/// sur les centres lit une face exactement et un centre à +9,9 %. Le rayon retenu est le **minimax** : celui qui égalise
/// les deux erreurs, de signes opposés, par dichotomie en `f64` ([preuve](../../docs/validation/APIC3D-S388.md)).
pub fn rest_radius(dx: f32) -> f32 {
    let dx = dx as f64;
    let (face, centre) = (|r: f64| read_error(dx, r, true), |r: f64| read_error(dx, r, false));
    let at = |z: f64| rest_mean_distance(dx, z).expect("une voisine");
    let (mut lo, mut hi) = (at(0.), 0.5 * (at(-0.5 * dx) + at(0.5 * dx)));
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if face(mid) + centre(mid) < 0. { lo = mid; } else { hi = mid; }
    }
    (0.5 * (lo + hi)) as f32
}

/// La distance d'un point de la verticale, à la hauteur `qz` au-dessus d'une nappe au repos de surface `z = 0`, à la moyenne
/// pondérée de ses voisines — en `f64`, sur un réseau de huit particules par maille, latéralement au centre d'une maille ;
/// `None` sans voisine à moins de `dx`.
pub fn rest_mean_distance(dx: f64, qz: f64) -> Option<f64> {
    let h = dx / PER_AXIS as f64;
    let (mut sw, mut sz) = (0f64, 0f64);
    for k in 0..8 {
        for j in -8i32..8 {
            for i in -8i32..8 {
                let p = [(i as f64 + 0.5) * h, (j as f64 + 0.5) * h, -(k as f64 + 0.5) * h];
                let s2 = (p[0] * p[0] + p[1] * p[1] + (p[2] - qz) * (p[2] - qz)) / (dx * dx);
                if s2 < 1. {
                    let w = (1. - s2).powi(3);
                    sw += w;
                    sz += w * p[2];
                }
            }
        }
    }
    (sw > 0.).then(|| (qz - sz / sw).abs())
}

/// L'erreur de hauteur lue (m) pour un rayon `r` : surface sur une face (`face`, centres en `∓dx/2`) ou sur un centre.
pub fn read_error(dx: f64, r: f64, face: bool) -> f64 {
    // Comme `reconstruct` : `φ = dx` sans voisine (S388 : le modèle mettait d'abord `2·dx − r`, et lisait un centre à +4,5 %
    // quand la reconstruction lisait +7,2 %).
    let phi = |z: f64| rest_mean_distance(dx, z).map_or(dx, |d| d - r);
    let centres: [f64; 4] = if face { [-1.5 * dx, -0.5 * dx, 0.5 * dx, 1.5 * dx] } else { [-dx, 0., dx, 2. * dx] };
    for w in centres.windows(2) {
        let (a, b) = (phi(w[0]), phi(w[1]));
        if a < 0. && b >= 0. {
            return w[0] + (w[1] - w[0]) * a / (a - b);
        }
    }
    f64::NAN
}

/// Les distances `(d₋, d₊)` des centres `z = −dx/2` et `z = +dx/2` à la moyenne pondérée d'une nappe au repos.
pub fn rest_distances(dx: f64) -> (f64, f64) {
    let at = |z: f64| rest_mean_distance(dx, z).expect("une voisine");
    (at(-0.5 * dx), at(0.5 * dx))
}

/// Le réglage sur les centres qui encadrent une surface posée sur une face : `(d₋ + d₊)/2`. Gardé pour la mesure.
pub fn rest_radius_at_the_faces(dx: f32) -> f32 {
    let (a, b) = rest_distances(dx as f64);
    (0.5 * (a + b)) as f32
}

/// Le réglage de S318, `d₀` : la distance du point de la surface à la moyenne de ses voisines. Gardé pour la mesure.
pub fn rest_radius_at_the_plane(dx: f32) -> f32 {
    let dx = dx as f64;
    let h = dx / PER_AXIS as f64;
    let (mut sw, mut sz) = (0f64, 0f64);
    for k in 0..8 {
        for j in -8i32..8 {
            for i in -8i32..8 {
                let p = [(i as f64 + 0.5) * h, (j as f64 + 0.5) * h, -(k as f64 + 0.5) * h];
                let s2 = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]) / (dx * dx);
                if s2 < 1. {
                    let w = (1. - s2).powi(3);
                    sw += w;
                    sz += w * p[2];
                }
            }
        }
    }
    (-(sz / sw)) as f32
}

/// Les trois grilles décalées : origine du nœud `(0, 0, 0)` en mailles, et dimensions. `u` : `(0, ½, ½)`, `(nx+1, ny, nz)`.
#[inline]
pub(crate) fn staggered(domain: Domain3, axis: usize) -> ([f32; 3], [usize; 3]) {
    let Domain3 { nx, ny, nz, .. } = domain;
    match axis {
        0 => ([0., 0.5, 0.5], [nx + 1, ny, nz]),
        1 => ([0.5, 0., 0.5], [nx, ny + 1, nz]),
        _ => ([0.5, 0.5, 0.], [nx, ny, nz + 1]),
    }
}

/// Poids **trilinéaires** d'un point sur une grille décalée : les huit nœuds (indice linéaire, `x` le plus rapide), leurs
/// poids et les gradients de ces poids (m⁻¹). Les coordonnées sont bornées au bloc des nœuds, comme en 2D : près d'une paroi,
/// l'interpolation transverse se fige sur la rangée du bord.
#[inline]
pub(crate) fn weights(p: [f32; 3], dx: f32, origin: [f32; 3], dims: [usize; 3]) -> [(usize, f32, [f32; 3]); 8] {
    let mut base = [0usize; 3];
    let mut frac = [0f32; 3];
    let mut next = [0usize; 3];
    for a in 0..3 {
        let top = (dims[a] - 1) as f32;
        let f = (p[a] / dx - origin[a]).clamp(0., (top - 1e-4).max(0.));
        base[a] = f.floor() as usize;
        frac[a] = f - base[a] as f32;
        next[a] = (base[a] + 1).min(dims[a] - 1);
    }
    let mut out = [(0usize, 0f32, [0f32; 3]); 8];
    for (m, slot) in out.iter_mut().enumerate() {
        let pick = [m & 1, (m >> 1) & 1, (m >> 2) & 1];
        let mut idx = [0usize; 3];
        let mut wa = [0f32; 3];
        let mut ga = [0f32; 3];
        for a in 0..3 {
            if pick[a] == 1 {
                idx[a] = next[a];
                wa[a] = frac[a];
                ga[a] = 1. / dx;
            } else {
                idx[a] = base[a];
                wa[a] = 1. - frac[a];
                ga[a] = -1. / dx;
            }
        }
        let weight = wa[0] * wa[1] * wa[2];
        let grad = [ga[0] * wa[1] * wa[2], wa[0] * ga[1] * wa[2], wa[0] * wa[1] * ga[2]];
        *slot = ((idx[2] * dims[1] + idx[1]) * dims[0] + idx[0], weight, grad);
    }
    out
}

impl Apic3 {
    /// La position d'un nœud de la grille `axis`, m.
    #[inline]
    fn node(&self, axis: usize, index: usize) -> [f32; 3] {
        let (origin, dims) = staggered(self.domain, axis);
        let (i, j, k) = (index % dims[0], (index / dims[0]) % dims[1], index / (dims[0] * dims[1]));
        let dx = self.domain.dx;
        [(i as f32 + origin[0]) * dx, (j as f32 + origin[1]) * dx, (k as f32 + origin[2]) * dx]
    }

    /// **Particules → grille**, APIC : `u_f = Σ w·(v_a + C_a·(x_f − x_p)) / Σ w`. Les poids restent dans `wu`, `wv`, `ww`
    /// — une face de poids nul n'a reçu aucune particule.
    pub(crate) fn particles_to_grid(&mut self) {
        let dx = self.domain.dx;
        self.u.fill(0.);
        self.v.fill(0.);
        self.w.fill(0.);
        self.wu.fill(0.);
        self.wv.fill(0.);
        self.ww.fill(0.);
        for k in 0..self.n {
            let (p, v, c) = (self.x[k], self.vel[k], self.c[k]);
            for axis in 0..3 {
                let (origin, dims) = staggered(self.domain, axis);
                for (idx, wt, _) in weights(p, dx, origin, dims) {
                    if wt == 0. {
                        continue;
                    }
                    let f = self.node(axis, idx);
                    let affine = c[axis][0] * (f[0] - p[0]) + c[axis][1] * (f[1] - p[1]) + c[axis][2] * (f[2] - p[2]);
                    let (field, weight) = match axis {
                        0 => (&mut self.u, &mut self.wu),
                        1 => (&mut self.v, &mut self.wv),
                        _ => (&mut self.w, &mut self.ww),
                    };
                    field[idx] += wt * (v[axis] + affine);
                    weight[idx] += wt;
                }
            }
        }
        for (f, wt) in self.u.iter_mut().zip(&self.wu).chain(self.v.iter_mut().zip(&self.wv)).chain(self.w.iter_mut().zip(&self.ww)) {
            *f = if *wt > 0. { *f / *wt } else { 0. };
        }
    }

    /// **Grille → particules**, APIC : la vitesse interpolée et la matrice affine `C_ab = Σ ∂_b w · u_a`.
    pub(crate) fn grid_to_particles(&mut self) {
        let dx = self.domain.dx;
        for k in 0..self.n {
            let p = self.x[k];
            let mut v = [0f32; 3];
            let mut c = [[0f32; 3]; 3];
            for axis in 0..3 {
                let (origin, dims) = staggered(self.domain, axis);
                let field = match axis {
                    0 => &self.u,
                    1 => &self.v,
                    _ => &self.w,
                };
                for (idx, wt, g) in weights(p, dx, origin, dims) {
                    let f = field[idx];
                    v[axis] += wt * f;
                    for b in 0..3 {
                        c[axis][b] += g[b] * f;
                    }
                }
            }
            self.vel[k] = v;
            self.c[k] = c;
        }
    }

    /// Vitesse de la grille interpolée en un point (trilinéaire par composante décalée).
    pub(crate) fn grid_velocity(&self, p: [f32; 3]) -> [f32; 3] {
        let dx = self.domain.dx;
        let mut v = [0f32; 3];
        for axis in 0..3 {
            let (origin, dims) = staggered(self.domain, axis);
            let field = match axis {
                0 => &self.u,
                1 => &self.v,
                _ => &self.w,
            };
            for (idx, wt, _) in weights(p, dx, origin, dims) {
                v[axis] += wt * field[idx];
            }
        }
        v
    }
}

impl Apic3 {
    /// **La surface reconstruite** (Zhu et Bridson 2005) : aux centres des mailles, `φ = |q − x̄| − r`, `x̄` la moyenne des
    /// particules à moins d'une maille, pondérée par `(1 − s²/dx²)³` ; `φ = dx` sans voisine. Puis les étiquettes : eau où
    /// `φ < 0`. Trie les particules d'abord ; aucune allocation.
    pub(crate) fn reconstruct(&mut self) {
        self.bin();
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv_r2 = 1. / (dx * dx);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let q = [(i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx];
                    let (mut sw, mut sx) = (0f32, [0f32; 3]);
                    for c in k.saturating_sub(1)..(k + 2).min(nz) {
                        for b in j.saturating_sub(1)..(j + 2).min(ny) {
                            for a in i.saturating_sub(1)..(i + 2).min(nx) {
                                let cell = self.cell(a, b, c);
                                for s in self.bin_start[cell]..self.bin_start[cell + 1] {
                                    let p = self.x[self.order[s as usize] as usize];
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
                    let c = self.cell(i, j, k);
                    self.phi[c] = if sw > 0. {
                        let m = [sx[0] / sw - q[0], sx[1] / sw - q[1], sx[2] / sw - q[2]];
                        (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt() - self.radius
                    } else {
                        dx
                    };
                    self.label[c] = if self.phi[c] < 0. { WATER } else { AIR };
                }
            }
        }
    }
}

/// Ce qu'un pas rend : le solveur de pression et l'état des particules.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ApicReport {
    /// Itérations du gradient conjugué.
    pub iterations: u32,
    /// `‖r‖/‖b‖` à l'arrêt.
    pub residual: f64,
    /// `max |div u|·dx / max|u|` sur les mailles d'eau sans voisine d'air, après la projection.
    pub divergence: f64,
    /// Plus grande vitesse de particule après le pas, m/s.
    pub max_speed: f32,
}

/// Tolérance du gradient conjugué, sur les carrés des normes : `‖r‖ ≤ 10⁻⁶·‖b‖`, la précision que `f32` permet (ADR-143).
const PRESSURE_TOLERANCE2: f64 = 1e-12;
const PRESSURE_MAX_ITERATIONS: u32 = 4000;
/// Couches d'extrapolation des vitesses vers l'air (S318).
const EXTRAPOLATION_LAYERS: usize = 3;
/// Séparation des particules : distance minimale en mailles, passes (S320 P3).
const SEPARATION: f32 = 0.4;
const SEPARATION_PASSES: usize = 2;
/// Plancher de la fraction fantôme, comme en 2D.
const THETA_MIN: f32 = 0.01;

impl Apic3 {
    /// Le plus grand pas stable, µs, sous `max_us` : `0,5·dx / (max|v| + √(g·dx))`, comme en 2D.
    pub fn stable_step_us(&self, max_us: u64) -> u64 {
        let vmax = self.vel[..self.n].iter().fold(0f32, |m, v| m.max(v[0].abs()).max(v[1].abs()).max(v[2].abs()));
        let dx = self.domain.dx as f64;
        let dt = 0.5 * dx / (vmax as f64 + (self.g_eff.abs() as f64 * dx).sqrt());
        ((dt * 1e6) as u64).clamp(1, max_us)
    }

    /// **Un pas** de `duration_us` µs : particules → grille, surface reconstruite, gravité, projection à fluide fantôme,
    /// extrapolation, grille → particules, advection RK2, séparation. Aucune allocation. Refus `NotFinite` si un champ
    /// cesse d'être fini — l'état est alors celui du pas interrompu (la référence n'est pas atomique).
    pub fn step(&mut self, duration_us: u64) -> Result<ApicReport, Error> {
        if duration_us == 0 || duration_us > (1u64 << 40) {
            return Err(Error::NotFinite);
        }
        // Les coefficients dimensionnés se construisent en f64 et s'arrondissent en f32 (I-08, ADR-141).
        let dt = (duration_us as f64 * 1e-6) as f32;
        self.particles_to_grid();
        self.reconstruct();
        let gdt = (self.g_eff as f64 * duration_us as f64 * 1e-6) as f32;
        for w in self.w.iter_mut() {
            *w -= gdt;
        }
        self.walls();
        let (iterations, residual) = self.project(dt);
        let divergence = self.divergence_metric();
        self.extrapolate();
        self.grid_to_particles();
        self.advect(dt);
        if self.separation {
            self.separate();
        }
        let mut max_speed = 0f32;
        for k in 0..self.n {
            let (p, v) = (self.x[k], self.vel[k]);
            if !(p.iter().chain(v.iter()).all(|x| x.is_finite())) {
                return Err(Error::NotFinite);
            }
            max_speed = max_speed.max((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt());
        }
        self.iterations = iterations;
        Ok(ApicReport { iterations, residual, divergence, max_speed })
    }

    /// Vitesse normale nulle sur le bord du domaine (parois).
    fn walls(&mut self) {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        for k in 0..nz {
            for j in 0..ny {
                self.u[(k * ny + j) * (nx + 1)] = 0.;
                self.u[(k * ny + j) * (nx + 1) + nx] = 0.;
            }
            for i in 0..nx {
                self.v[(k * (ny + 1)) * nx + i] = 0.;
                self.v[(k * (ny + 1) + ny) * nx + i] = 0.;
            }
        }
        for j in 0..ny {
            for i in 0..nx {
                self.w[j * nx + i] = 0.;
                self.w[(nz * ny + j) * nx + i] = 0.;
            }
        }
    }

    /// Les six voisines d'une maille : (maille voisine, face commune, axe, signe de la face vue de la maille), `None` au bord.
    #[inline]
    fn neighbours(&self, i: usize, j: usize, k: usize) -> [Option<(usize, usize, usize)>; 6] {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        let fu = |i: usize| (k * ny + j) * (nx + 1) + i;
        let fv = |j: usize| (k * (ny + 1) + j) * nx + i;
        let fw = |k: usize| (k * ny + j) * nx + i;
        [
            (i > 0).then(|| (self.cell(i - 1, j, k), fu(i), 0)),
            (i + 1 < nx).then(|| (self.cell(i + 1, j, k), fu(i + 1), 0)),
            (j > 0).then(|| (self.cell(i, j - 1, k), fv(j), 1)),
            (j + 1 < ny).then(|| (self.cell(i, j + 1, k), fv(j + 1), 1)),
            (k > 0).then(|| (self.cell(i, j, k - 1), fw(k), 2)),
            (k + 1 < nz).then(|| (self.cell(i, j, k + 1), fw(k + 1), 2)),
        ]
    }

    /// La fraction fantôme de la maille d'eau `c` vers sa voisine d'air `a` : `θ = φ_c/(φ_c − φ_a)`, bornée à 0,01.
    #[inline]
    fn theta(&self, c: usize, a: usize) -> f32 {
        let (fc, fa) = (self.phi[c], self.phi[a]);
        (fc / (fc - fa)).max(THETA_MIN)
    }

    fn dot(a: &[f32], b: &[f32]) -> f64 {
        a.iter().zip(b).map(|(x, y)| *x as f64 * *y as f64).sum()
    }

    /// `y = A·x` sur les mailles d'eau : `Σ (x_c − x_n)` vers l'eau, `x_c/θ` vers l'air, rien vers une paroi (sans `1/dx²`).
    fn apply(&self, x: &[f32], y: &mut [f32]) {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.cell(i, j, k);
                    if self.label[c] != WATER {
                        y[c] = 0.;
                        continue;
                    }
                    let mut s = 0f32;
                    for (n, _, _) in self.neighbours(i, j, k).into_iter().flatten() {
                        if self.label[n] == WATER {
                            s += x[c] - x[n];
                        } else {
                            s += x[c] / self.theta(c, n);
                        }
                    }
                    y[c] = s;
                }
            }
        }
    }

    /// **La projection** : `A·p = −(ρ·dx²/dt)·div u*` sur l'eau, gradient conjugué préconditionné par la diagonale ;
    /// puis `u −= (dt/(ρ·dx))·∇p` sur les faces qui touchent l'eau, la pression d'air nulle à `θ·dx`. Rend (itérations,
    /// résidu relatif).
    fn project(&mut self, dt: f32) -> (u32, f64) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let scale = -self.rho * dx * dx / dt;
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
                            diag += if self.label[n] == WATER { 1. } else { 1. / self.theta(c, n) };
                        }
                    }
                    self.rhs[c] = scale * div / dx;
                    self.diag[c] = diag;
                }
            }
        }
        // Gradient conjugué préconditionné par la diagonale.
        let cells = self.p.len();
        let b2 = Self::dot(&self.rhs, &self.rhs);
        self.r.copy_from_slice(&self.rhs);
        for c in 0..cells {
            self.z[c] = if self.diag[c] > 0. { self.r[c] / self.diag[c] } else { 0. };
        }
        self.d.copy_from_slice(&self.z);
        let mut rz = Self::dot(&self.r, &self.z);
        let mut rr = b2;
        let mut it = 0u32;
        while b2 > 0. && rr > PRESSURE_TOLERANCE2 * b2 && it < PRESSURE_MAX_ITERATIONS {
            let (d, mut q) = (core::mem::take(&mut self.d), core::mem::take(&mut self.q));
            self.apply(&d, &mut q);
            let dq = Self::dot(&d, &q);
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
            let zn = Self::dot(&self.r, &self.z);
            let beta = (zn / rz) as f32;
            for c in 0..cells {
                self.d[c] = self.z[c] + beta * self.d[c];
            }
            rz = zn;
            rr = Self::dot(&self.r, &self.r);
            it += 1;
        }
        // Correction des faces qui touchent l'eau ; une paroi n'est jamais corrigée.
        let k1 = dt / (self.rho * dx);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.cell(i, j, k);
                    // Chaque face intérieure est vue par ses deux mailles : on la traite depuis la maille du côté négatif.
                    for (m, nb) in self.neighbours(i, j, k).into_iter().enumerate() {
                        if m % 2 == 0 {
                            continue;
                        }
                        let Some((n, f, axis)) = nb else { continue };
                        let (wc, wn) = (self.label[c] == WATER, self.label[n] == WATER);
                        let grad = if wc && wn {
                            self.p[n] - self.p[c]
                        } else if wc {
                            -self.p[c] / self.theta(c, n)
                        } else if wn {
                            self.p[n] / self.theta(n, c)
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
        (it, if b2 > 0. { (rr / b2).sqrt() } else { 0. })
    }

    /// `max |div u|·dx / max|u|` sur les mailles d'eau dont aucune voisine n'est d'air.
    fn divergence_metric(&self) -> f64 {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        let mut worst = 0f32;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.cell(i, j, k);
                    if self.label[c] != WATER {
                        continue;
                    }
                    let nb = self.neighbours(i, j, k);
                    if nb.iter().flatten().any(|(n, _, _)| self.label[*n] != WATER) {
                        continue;
                    }
                    let div = self.u[(k * ny + j) * (nx + 1) + i + 1] - self.u[(k * ny + j) * (nx + 1) + i]
                        + self.v[(k * (ny + 1) + j + 1) * nx + i] - self.v[(k * (ny + 1) + j) * nx + i]
                        + self.w[((k + 1) * ny + j) * nx + i] - self.w[(k * ny + j) * nx + i];
                    worst = worst.max(div.abs());
                }
            }
        }
        let umax = self.u.iter().chain(&self.v).chain(&self.w).fold(0f32, |m, x| m.max(x.abs()));
        if umax > 0. { (worst / umax) as f64 } else { 0. }
    }

    /// **Extrapolation** des vitesses de l'eau vers l'air sur trois couches ; au-delà, remise à zéro — sauf les faces qu'une
    /// particule a alimentées (S318, fautes 5 et 6). Puis les parois.
    fn extrapolate(&mut self) {
        let domain = self.domain;
        for axis in 0..3 {
            let (_, dims) = staggered(domain, axis);
            let count = dims[0] * dims[1] * dims[2];
            // Valide : une face qui touche une maille d'eau.
            for f in 0..count {
                let (i, j, k) = (f % dims[0], (f / dims[0]) % dims[1], f / (dims[0] * dims[1]));
                let touches = |a: isize, b: isize, c: isize| {
                    a >= 0 && b >= 0 && c >= 0 && (a as usize) < domain.nx && (b as usize) < domain.ny
                        && (c as usize) < domain.nz && self.label[self.cell(a as usize, b as usize, c as usize)] == WATER
                };
                let (i, j, k) = (i as isize, j as isize, k as isize);
                let ok = match axis {
                    0 => touches(i - 1, j, k) || touches(i, j, k),
                    1 => touches(i, j - 1, k) || touches(i, j, k),
                    _ => touches(i, j, k - 1) || touches(i, j, k),
                };
                let valid = match axis {
                    0 => &mut self.valid_u,
                    1 => &mut self.valid_v,
                    _ => &mut self.valid_w,
                };
                valid[f] = ok as u8;
            }
            for _ in 0..EXTRAPOLATION_LAYERS {
                let (field, valid) = match axis {
                    0 => (&mut self.u, &mut self.valid_u),
                    1 => (&mut self.v, &mut self.valid_v),
                    _ => (&mut self.w, &mut self.valid_w),
                };
                self.old_u[..count].copy_from_slice(field);
                self.old_valid[..count].copy_from_slice(valid);
                for f in 0..count {
                    if self.old_valid[f] != 0 {
                        continue;
                    }
                    let (i, j, k) = (f % dims[0], (f / dims[0]) % dims[1], f / (dims[0] * dims[1]));
                    let (mut s, mut n) = (0f32, 0u32);
                    let mut add = |a: isize, b: isize, c: isize| {
                        if a >= 0 && b >= 0 && c >= 0 && (a as usize) < dims[0] && (b as usize) < dims[1] && (c as usize) < dims[2] {
                            let g = (c as usize * dims[1] + b as usize) * dims[0] + a as usize;
                            if self.old_valid[g] != 0 {
                                s += self.old_u[g];
                                n += 1;
                            }
                        }
                    };
                    let (i, j, k) = (i as isize, j as isize, k as isize);
                    add(i - 1, j, k);
                    add(i + 1, j, k);
                    add(i, j - 1, k);
                    add(i, j + 1, k);
                    add(i, j, k - 1);
                    add(i, j, k + 1);
                    if n > 0 {
                        field[f] = s / n as f32;
                        valid[f] = 1;
                    }
                }
            }
            let (field, valid, weight) = match axis {
                0 => (&mut self.u, &self.valid_u, &self.wu),
                1 => (&mut self.v, &self.valid_v, &self.wv),
                _ => (&mut self.w, &self.valid_w, &self.ww),
            };
            for f in 0..count {
                if valid[f] == 0 && weight[f] == 0. {
                    field[f] = 0.;
                }
            }
        }
        self.walls();
    }

    /// Advection des particules dans la vitesse de la grille, RK2 (point milieu), bornée au domaine.
    fn advect(&mut self, dt: f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let (lx, ly, lz) = (nx as f32 * dx, ny as f32 * dx, nz as f32 * dx);
        let margin = 1e-3 * dx;
        for k in 0..self.n {
            let p = self.x[k];
            let v1 = self.grid_velocity(p);
            let mid = [p[0] + 0.5 * dt * v1[0], p[1] + 0.5 * dt * v1[1], p[2] + 0.5 * dt * v1[2]];
            let v2 = self.grid_velocity(mid);
            self.x[k] = [
                (p[0] + dt * v2[0]).clamp(margin, lx - margin),
                (p[1] + dt * v2[1]).clamp(margin, ly - margin),
                (p[2] + dt * v2[2]).clamp(margin, lz - margin),
            ];
        }
    }

    /// **Séparation** : deux particules plus proches que 0,4 maille s'écartent chacune du quart de leur recouvrement,
    /// deux passes ; les vitesses ne changent pas, une eau au repos (à une demi-maille d'écart) non plus (S320 P3).
    fn separate(&mut self) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let (lx, ly, lz) = (nx as f32 * dx, ny as f32 * dx, nz as f32 * dx);
        let (d_min, margin) = (SEPARATION * dx, 1e-3 * dx);
        for _ in 0..SEPARATION_PASSES {
            self.bin();
            for s in self.shift[..self.n].iter_mut() {
                *s = [0.; 3];
            }
            for k in 0..nz {
                for j in 0..ny {
                    for i in 0..nx {
                        let cell = self.cell(i, j, k);
                        for sa in self.bin_start[cell]..self.bin_start[cell + 1] {
                            let a = self.order[sa as usize] as usize;
                            for c in k.saturating_sub(1)..(k + 2).min(nz) {
                                for b_ in j.saturating_sub(1)..(j + 2).min(ny) {
                                    for a_ in i.saturating_sub(1)..(i + 2).min(nx) {
                                        let other = self.cell(a_, b_, c);
                                        for sb in self.bin_start[other]..self.bin_start[other + 1] {
                                            let b = self.order[sb as usize] as usize;
                                            if b <= a {
                                                continue;
                                            }
                                            let e = [self.x[b][0] - self.x[a][0], self.x[b][1] - self.x[a][1], self.x[b][2] - self.x[a][2]];
                                            let d = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt();
                                            if d < d_min && d > 1e-6 * dx {
                                                let m = 0.25 * (d_min - d) / d;
                                                for t in 0..3 {
                                                    self.shift[a][t] -= m * e[t];
                                                    self.shift[b][t] += m * e[t];
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for k in 0..self.n {
                let (p, d) = (self.x[k], self.shift[k]);
                self.x[k] = [
                    (p[0] + d[0]).clamp(margin, lx - margin),
                    (p[1] + d[1]).clamp(margin, ly - margin),
                    (p[2] + d[2]).clamp(margin, lz - margin),
                ];
            }
        }
    }
}

#[cfg(test)]
#[path = "tests_apic3d.rs"]
mod tests;
