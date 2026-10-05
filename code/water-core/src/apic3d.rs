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
//!
//! **S393** : un corps **cinématique** — une sphère dont l'hôte impose le mouvement, comme le cylindre du banc 2D de S320
//! (B10) : ses mailles sont solides, les faces qui les touchent prennent sa vitesse, la pression y voit une paroi mobile, et
//! les particules qu'il atteint sont repoussées à sa surface. Aucune force ne revient au corps.

use crate::delta3d::Domain3;
use crate::delta_projection::Error;
use crate::host::{AllocError, HostServices};

/// Particules posées par maille et par direction : 2 × 2 × 2 = 8.
pub const PER_AXIS: usize = 2;

/// Une maille d'eau, d'air ou de paroi.
pub const AIR: u8 = 0;
pub const WATER: u8 = 1;
/// Une maille dont le centre est dans le corps (S393) : paroi mobile pour la pression, vitesse imposée sur ses faces.
pub const SOLID: u8 = 2;

/// Un corps **cinématique** (S393) : une sphère, son centre au début du pas et sa vitesse, imposés par l'hôte.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere3 {
    pub center: [f32; 3],
    pub radius: f32,
    pub velocity: [f32; 3],
}

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
    /// Rayon du noyau de la reconstruction, en mailles (`KERNEL_CELLS` ; un autre sert à la mesure).
    pub(crate) kernel: f32,
    /// Le corps cinématique, s'il y en a un (S393) ; le pas l'avance de `velocity·dt`.
    pub(crate) body: Option<Sphere3>,
    /// La zone des colonnes (S398), si elle est active : surface `η`, vitesse eulérienne advectée.
    pub(crate) columns: Option<columns::Columns3>,
    /// **S444 (C7d-3c, c1, ADR-214) — le fond B et le mode relatif** : `Some(B)`, les particules et la grille portent `u′`, la
    /// vitesse propre de δ ; les particules se déplacent avec `U + u′`. `None`, le défaut : l'eau totale, au bit.
    pub(crate) background: [Option<LinearSwell>; 2],
    /// L'instant de B au début du prochain pas, s.
    pub(crate) background_time_s: f64,
    /// **S446 (C7d-3c, c2) — les bords ouverts en `x`** : la vitesse normale imposée sur les faces `u` des bords `i = 0` (les `ny·nz`
    /// premières, rangées `k·ny + j`) puis `i = nx` ; `None`, des parois — au bit.
    pub(crate) open_x: Option<Vec<f32>>,
    /// **S479 (K2-1, ADR-220 D1) — les poches d'air enfermé** (`enable_air_pockets`, `apic3d_poches.rs`) ; `None`, le défaut :
    /// l'air enfermé à la pression atmosphérique, au bit.
    pub(crate) poches: Option<Box<poches::Poches>>,
    /// **S483 (ADR-222 D2)** — le système de tâches de l'hôte pour les écritures disjointes du pas (`set_jobs`) ; `None`, le
    /// défaut : les boucles séquentielles. Le résultat ne dépend pas de ce choix (S243).
    pub(crate) jobs: Option<std::sync::Arc<dyn crate::host::JobSystem + Send + Sync>>,
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
            kernel: KERNEL_CELLS,
            body: None,
            columns: None,
            background: [None; 2],
            background_time_s: 0.,
            open_x: None,
            poches: None,
            jobs: None,
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
    /// S418 : la capacité réservée, en particules (banc de la carte).
    pub fn particle_capacity(&self) -> usize {
        self.x.len()
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
        self.columns_tabulate();
    }
    /// **Pour la mesure** : le noyau de la reconstruction, en mailles, et le rayon minimax qui lui répond (défaut :
    /// `KERNEL_CELLS`). Calcul `f64` de quelques millisecondes : hors du pas.
    pub fn set_reconstruction_kernel(&mut self, cells: f32) {
        self.kernel = cells;
        self.radius = minimax_radius(self.domain.dx as f64, cells as f64).0 as f32;
        self.columns_tabulate();
    }
    /// **Pour la mesure** : la séparation des particules (défaut : active).
    pub fn set_separation(&mut self, on: bool) {
        self.separation = on;
    }
    /// **Le corps cinématique** (S393) : sa position au début du prochain pas et sa vitesse, que l'hôte impose ; `None`
    /// l'enlève. Refus `NotFinite` si une grandeur n'est pas finie, `Domain` si le rayon n'est pas positif.
    /// **S444 (C7d-3c, c1, ADR-214) — le mode relatif à B.** `Some(B)` : la grille et les particules portent la vitesse propre `u′`
    /// de δ ; les particules se déplacent avec `U + u′` (leur position est celle de l'eau) ; la grille reçoit `−dt·u′·∇U` (gradient
    /// exact de B) et plus la gravité en volume ; la pression est `p′ = p − p_B`, dont la valeur à la surface totale est
    /// `−p_B(z_s)` moins l'erreur de B à sa propre surface (ADR-198, la forme d'A324). `t_s` est l'instant de B au prochain pas. Sans
    /// zone de colonnes seulement (c1) : avec une zone, refus (`Domain`). `None` : l'eau totale, le pas d'avant au bit.
    pub fn set_relative_background(&mut self, background: Option<LinearSwell>, t_s: f64) -> Result<(), Error> {
        self.set_relative_backgrounds(&background.into_iter().collect::<Vec<_>>(), t_s)
    }

    /// S444 : le mode relatif sur **une ou deux composantes** de B, superposées (linéaires) — deux houles opposées font une
    /// houle stationnaire, dont la vitesse horizontale s'annule aux parois. Une liste vide : l'eau totale. Plus de deux : refus.
    pub fn set_relative_backgrounds(&mut self, components: &[LinearSwell], t_s: f64) -> Result<(), Error> {
        if components.len() > 2 || (!components.is_empty() && self.columns.is_some()) {
            return Err(Error::Domain);
        }
        self.background = [components.first().copied(), components.get(1).copied()];
        self.background_time_s = t_s;
        Ok(())
    }

    /// **S446 (C7d-3c, c2) — les bords ouverts en `x`**, réservés à la configuration (I-06) : les faces `u` des bords `i = 0` et
    /// `i = nx` portent une vitesse normale imposée (`set_open_boundaries`), nulle d'abord ; la projection la prend comme donnée, le
    /// transport des colonnes compte son débit, mouillé à la hauteur de la colonne du bord. Le raccord d'un domaine de bande à la mer
    /// qui l'entoure (ADR-214). Sans eux : des parois, au bit.
    pub fn enable_open_boundaries(&mut self, host: &mut HostServices) -> Result<(), Error> {
        if self.open_x.is_some() {
            return Err(Error::Domain);
        }
        let Domain3 { ny, nz, .. } = self.domain;
        let n = 2 * ny * nz;
        host.alloc.alloc_persistent(n * 4).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        self.open_x = Some(vec![0.; n]);
        Ok(())
    }

    /// S446 : les vitesses normales des bords ouverts, `left` en `i = 0` et `right` en `i = nx`, chacune `ny·nz` valeurs rangées
    /// `k·ny + j`. Refus : sans bords ouverts (`Domain`), longueur (`Shape`), valeur non finie.
    pub fn set_open_boundaries(&mut self, left: &[f32], right: &[f32]) -> Result<(), Error> {
        let Domain3 { ny, nz, .. } = self.domain;
        let o = self.open_x.as_mut().ok_or(Error::Domain)?;
        if left.len() != ny * nz || right.len() != ny * nz {
            return Err(Error::Shape);
        }
        if left.iter().chain(right).any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        o[..ny * nz].copy_from_slice(left);
        o[ny * nz..].copy_from_slice(right);
        Ok(())
    }

    /// S446 : impose la vitesse de la grille (faces `u`, `v`, `w`) — l'état d'une zone de colonnes, pour un raccord ou une
    /// réception. Refus : longueur (`Shape`), valeur non finie.
    pub fn set_grid_velocities(&mut self, u: &[f32], v: &[f32], w: &[f32]) -> Result<(), Error> {
        if u.len() != self.u.len() || v.len() != self.v.len() || w.len() != self.w.len() {
            return Err(Error::Shape);
        }
        if u.iter().chain(v).chain(w).any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        self.u.copy_from_slice(u);
        self.v.copy_from_slice(v);
        self.w.copy_from_slice(w);
        Ok(())
    }

    /// S444 : vrai en mode relatif.
    pub fn is_relative(&self) -> bool {
        self.background[0].is_some()
    }

    /// S444 : la vitesse de B (la somme de ses composantes) au point `(x, ·, z)`.
    pub fn background_velocity(&self, x: f32, z: f32, t_s: f64) -> [f32; 3] {
        let mut u = [0f32; 3];
        for b in self.background.iter().flatten() {
            let v = b.velocity(x, z, t_s);
            for a in 0..3 {
                u[a] += v[a];
            }
        }
        u
    }

    /// S444 : le gradient exact de la vitesse de B.
    fn background_gradient(&self, x: f32, z: f32, t_s: f64) -> [[f32; 3]; 3] {
        let mut g = [[0f32; 3]; 3];
        for b in self.background.iter().flatten() {
            let h = b.velocity_gradient(x, z, t_s);
            for i in 0..3 {
                for j in 0..3 {
                    g[i][j] += h[i][j];
                }
            }
        }
        g
    }

    /// S444 : l'élévation de B au-dessus de son niveau moyen.
    pub fn background_elevation(&self, x: f32, t_s: f64) -> f32 {
        self.background.iter().flatten().map(|b| b.elevation(x, t_s)).sum()
    }

    /// S444 : la pression totale de B, `−ρ·g·ζ + Σ p_dyn`, Pa (en `f64`).
    fn background_pressure(&self, x: f32, z: f32, t_s: f64) -> f64 {
        let Some(first) = self.background[0] else { return 0. };
        let rg = self.rho as f64 * self.g_eff as f64;
        -rg * (z - first.mean_level) as f64
            + self.background.iter().flatten().map(|b| b.dynamic_pressure(x, z, t_s, self.rho, self.g_eff) as f64).sum::<f64>()
    }

    pub fn set_body(&mut self, body: Option<Sphere3>) -> Result<(), Error> {
        if let Some(b) = body {
            if !b.center.iter().chain(b.velocity.iter()).chain([b.radius].iter()).all(|x| x.is_finite()) {
                return Err(Error::NotFinite);
            }
            if b.radius <= 0. {
                return Err(Error::Domain);
            }
        }
        self.body = body;
        Ok(())
    }
    /// Le corps, avancé à la fin du dernier pas.
    pub fn body(&self) -> Option<Sphere3> {
        self.body
    }
    /// Les étiquettes des mailles : `AIR`, `WATER` ou `SOLID` (même ordre que `distance`).
    pub fn labels(&self) -> &[u8] {
        &self.label
    }
    /// Les vitesses `u` de la grille, faces `(nx + 1) × ny × nz` (`x` le plus rapide) — pour la mesure (S399).
    pub fn velocity_u(&self) -> &[f32] {
        &self.u
    }

    /// S415 — les vitesses `v` et `w` de la grille, aux faces (pour les bancs).
    pub fn velocity_v(&self) -> &[f32] {
        &self.v
    }
    pub fn velocity_w(&self) -> &[f32] {
        &self.w
    }

    /// **S415** — la vorticité de la grille au centre d'une maille, s⁻¹ (`vorticity`, pour les bancs).
    pub fn grid_vorticity(&self, i: usize, j: usize, k: usize) -> f32 {
        self.vorticity(i, j, k)
    }
    /// Distance signée reconstruite aux centres des mailles (`x` le plus rapide, puis `y`, puis `z`).
    /// S416 : les matrices affines des particules actives (banc de la carte).
    pub fn affine(&self) -> &[[[f32; 3]; 3]] {
        &self.c[..self.n]
    }

    /// S416 : les poids de transfert des faces, `u`, `v`, `w` (banc de la carte).
    pub fn face_weights(&self) -> (&[f32], &[f32], &[f32]) {
        (&self.wu, &self.wv, &self.ww)
    }

    /// S416 : la pression du dernier pas, aux centres des mailles (banc de la carte).
    pub fn pressure(&self) -> &[f32] {
        &self.p
    }

    /// S416 : le tri par maille du dernier `bin` — début de chaque maille (`cells + 1`), puis l'ordre des particules.
    pub fn bins(&self) -> (&[u32], &[u32]) {
        (&self.bin_start, &self.order[..self.n])
    }

    /// S416 : réglages de la reconstruction et de la séparation — rayon (m), noyau (mailles), séparation active.
    pub fn settings(&self) -> (f32, f32, bool) {
        (self.radius, self.kernel, self.separation)
    }

    /// S416 : densité et gravité fournies.
    pub fn physics(&self) -> (f32, f32) {
        (self.rho, self.g_eff)
    }

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

    /// **S410 — la vitesse des particules**, celle d'un champ donné : `field(x)` rend la vitesse et son gradient,
    /// `C_ab = ∂_b v_a` (APIC, Jiang et al. 2015 : un champ affine passe à la grille exactement). Pour poser un état initial en
    /// mouvement — la houle de C6b — ; `seed` laisse les particules au repos. Refus `NotFinite` si une valeur n'est pas finie :
    /// rien n'est alors changé.
    pub fn set_particle_velocities(&mut self, field: &dyn Fn([f32; 3]) -> ([f32; 3], [[f32; 3]; 3])) -> Result<(), Error> {
        for k in 0..self.n {
            let (v, c) = field(self.x[k]);
            if !v.iter().chain(c.iter().flatten()).all(|x| x.is_finite()) {
                return Err(Error::NotFinite);
            }
        }
        for k in 0..self.n {
            let (v, c) = field(self.x[k]);
            self.vel[k] = v;
            self.c[k] = c;
        }
        Ok(())
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

/// Rayon du noyau de la reconstruction, en mailles. **S389 : deux** — calculée sur une surface qui parcourt continûment une
/// maille, la lecture se trompe jusqu'à 9,9 % de maille avec un noyau d'une maille (S318–S388), 2,5 % avec deux ; le nombre
/// de particules par maille n'y change presque rien ([preuve](../../docs/validation/APIC3D-S388.md) §5).
pub const KERNEL_CELLS: f32 = 2.0;

/// Positions de la surface, en fraction de maille au-dessus d'un centre, sur lesquelles le rayon est réglé et la lecture
/// jugée : huit, au milieu de huitièmes.
pub const READ_POSITIONS: usize = 8;

/// La distance d'un point de la verticale, à la hauteur `qz`, à la moyenne pondérée d'une nappe au repos de surface
/// `surface` (m), noyau de rayon `kernel·dx` — en `f64`, huit particules par maille, latéralement au centre d'une maille ;
/// `None` sans voisine.
pub fn lattice_mean_distance(dx: f64, kernel: f64, qz: f64, surface: f64) -> Option<f64> {
    let h = dx / PER_AXIS as f64;
    let radius = kernel * dx;
    let layers = ((radius + (qz - surface).abs() + dx) / h).ceil() as i32 + 2;
    let lateral = (radius / h).ceil() as i32 + 1;
    let (mut sw, mut sz) = (0f64, 0f64);
    for k in 0..layers {
        let z = surface - (k as f64 + 0.5) * h;
        for j in -lateral..lateral {
            let y = (j as f64 + 0.5) * h;
            for i in -lateral..lateral {
                let x = (i as f64 + 0.5) * h;
                let s2 = (x * x + y * y + (z - qz) * (z - qz)) / (radius * radius);
                if s2 < 1. {
                    let w = (1. - s2).powi(3);
                    sw += w;
                    sz += w * z;
                }
            }
        }
    }
    (sw > 0.).then(|| (qz - sz / sw).abs())
}

/// L'erreur de hauteur **lue** (m) pour un rayon `r` et un noyau de `kernel` mailles, la surface à `offset·dx` au-dessus d'un
/// centre de maille (`0 ≤ offset < 1`) : l'iso-zéro interpolée entre deux centres, comme la lit la pression. `φ = dx` sans
/// voisine, comme `reconstruct`.
pub fn lattice_read_error(dx: f64, kernel: f64, r: f64, offset: f64) -> f64 {
    let surface = offset * dx;
    let phi = |z: f64| lattice_mean_distance(dx, kernel, z, surface).map_or(dx, |d| d - r);
    let reach = kernel.ceil() as i32 + 2;
    for m in -reach..reach {
        let (za, zb) = (m as f64 * dx, (m + 1) as f64 * dx);
        let (a, b) = (phi(za), phi(zb));
        if a < 0. && b >= 0. {
            return za + dx * a / (a - b) - surface;
        }
    }
    f64::NAN
}

/// Le pire écart de lecture sur les `READ_POSITIONS` positions, pour un rayon et un noyau donnés.
pub fn lattice_worst_read(dx: f64, kernel: f64, r: f64) -> f64 {
    (0..READ_POSITIONS)
        .map(|m| lattice_read_error(dx, kernel, r, (m as f64 + 0.5) / READ_POSITIONS as f64).abs())
        .fold(0f64, |w, e| if e.is_nan() { f64::INFINITY } else { w.max(e) })
}

/// **Le rayon minimax** d'un noyau : celui qui rend le plus petit le pire écart de lecture — recherche sur une grille de 21
/// valeurs de `[0, 1,5·kernel·dx]`, affinée trois fois autour du meilleur. Rend (rayon, pire écart), m.
pub fn minimax_radius(dx: f64, kernel: f64) -> (f64, f64) {
    let (mut lo, mut hi) = (0f64, 1.5 * kernel * dx);
    let mut best = (f64::INFINITY, 0f64);
    for _ in 0..3 {
        for t in 0..=20 {
            let r = lo + (hi - lo) * t as f64 / 20.;
            let w = lattice_worst_read(dx, kernel, r);
            if w < best.0 {
                best = (w, r);
            }
        }
        let step = (hi - lo) / 20.;
        lo = (best.1 - step).max(0.);
        hi = best.1 + step;
    }
    (best.1, best.0)
}

/// **Le rayon au repos** : le minimax du noyau retenu (`KERNEL_CELLS`), calculé en `f64` à la configuration (S389).
pub fn rest_radius(dx: f32) -> f32 {
    minimax_radius(dx as f64, KERNEL_CELLS as f64).0 as f32
}

/// Le réglage de S318, `d₀` : la distance du point de la surface à la moyenne de ses voisines, noyau d'une maille. Gardé
/// pour la mesure.
pub fn rest_radius_at_the_plane(dx: f32) -> f32 {
    lattice_mean_distance(dx as f64, 1., 0., 0.).expect("une voisine") as f32
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
    /// particules à moins de `R = kernel·dx` (deux mailles depuis S389), pondérée par `(1 − s²/R²)³` ; `φ = dx` sans voisine.
    /// Puis les étiquettes : eau où `φ < 0`. Trie les particules d'abord ; aucune allocation.
    pub(crate) fn reconstruct(&mut self) {
        self.bin();
        let Domain3 { nx, ny, nz, .. } = self.domain;
        // S483 (ADR-222 D2) : chaque maille ne lit que les particules et écrit sa seule valeur — une écriture disjointe, en
        // parallèle quand l'hôte a donné un système de tâches (`set_jobs`), au bit de la boucle séquentielle quel que soit le
        // nombre de fils (S243). Une maille de la zone des colonnes ou sous le fond garde sa valeur (`columns_label`).
        let mut phi = core::mem::take(&mut self.phi);
        {
            let this = &*self;
            let fill = |start: usize, out: &mut [f32]| {
                for (o, v) in out.iter_mut().enumerate() {
                    let c = start + o;
                    let (i, j, k) = (c % nx, (c / nx) % ny, c / (nx * ny));
                    if let Some(x) = this.reconstruct_cell(i, j, k) {
                        *v = x;
                    }
                }
            };
            match &self.jobs {
                Some(jobs) => jobs.parallel_fill_f32(&mut phi, nx * ny, &fill),
                None => fill(0, &mut phi),
            }
        }
        self.phi = phi;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    if self.columns.is_some() && self.grid_cell(i, j, k) {
                        continue;
                    }
                    let c = self.cell(i, j, k);
                    self.label[c] = if self.phi[c] < 0. { WATER } else { AIR };
                }
            }
        }
    }

    /// La surface reconstruite en une maille : `φ` (la distance au centre pondéré des particules voisines moins le rayon), ou
    /// `None` pour une maille de la zone des colonnes ou sous le fond de la bande (sa valeur vient d'ailleurs).
    fn reconstruct_cell(&self, i: usize, j: usize, k: usize) -> Option<f32> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let radius = self.kernel * dx;
        let inv_r2 = 1. / (radius * radius);
        let reach = self.kernel.ceil() as usize;
        // S398–S399 : une maille de la zone des colonnes prend `φ = z − η` (`columns_label`) ; rien à reconstruire.
        // S413 : ni une maille sous le fond de la bande, à la grille (`φ = z − fond`).
        if self.columns.is_some() && self.grid_cell(i, j, k) {
            return None;
        }
        let q = [(i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx];
        // S389 : les parois **reflètent** les particules — sans quoi, près d'une paroi latérale, le noyau n'en
        // trouve que d'un côté, la moyenne se décale vers l'intérieur et la surface y paraît plus basse. Une image
        // n'est cherchée que si le centre est à moins d'un rayon de noyau de la paroi ; le couvercle n'en a pas.
        let (lx, ly) = (nx as f32 * dx, ny as f32 * dx);
        let near = [q[0] < radius, q[0] > lx - radius, q[1] < radius, q[1] > ly - radius, q[2] < radius];
        let images_x = [Some(1f32), near[0].then_some(-1.), near[1].then_some(2.)];
        let images_y = [Some(1f32), near[2].then_some(-1.), near[3].then_some(2.)];
        let images_z = [Some(1f32), near[4].then_some(-1.)];
        // S393 : le corps **reflète** aussi, pour la même raison — sans quoi la surface paraît plus basse contre
        // lui et l'eau y monte (9,4 cm/s au repos). Image radiale, `c + (2R − d)·n`, cherchée seulement près du corps.
        let body = self.body.filter(|b| {
            let e = [q[0] - b.center[0], q[1] - b.center[1], q[2] - b.center[2]];
            (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt() < b.radius + radius
        });
        let (mut sw, mut sx) = (0f32, [0f32; 3]);
        for c in k.saturating_sub(reach)..(k + reach + 1).min(nz) {
            for b in j.saturating_sub(reach)..(j + reach + 1).min(ny) {
                for a in i.saturating_sub(reach)..(i + reach + 1).min(nx) {
                    let cell = self.cell(a, b, c);
                    for s in self.bin_start[cell]..self.bin_start[cell + 1] {
                        let p0 = self.x[self.order[s as usize] as usize];
                        // Image : 1 — la particule ; −1 — reflétée par la paroi basse ; 2 — par la paroi haute.
                        let mirror = |v: f32, m: f32, l: f32| if m == 1. { v } else if m == -1. { -v } else { 2. * l - v };
                        for mx in images_x.iter().flatten() {
                            for my in images_y.iter().flatten() {
                                for mz in images_z.iter().flatten() {
                                    let p = [mirror(p0[0], *mx, lx), mirror(p0[1], *my, ly), mirror(p0[2], *mz, 0.)];
                                    let mut add = |p: [f32; 3]| {
                                        let d = [p[0] - q[0], p[1] - q[1], p[2] - q[2]];
                                        let wt = kernel((d[0] * d[0] + d[1] * d[1] + d[2] * d[2]) * inv_r2);
                                        if wt > 0. {
                                            sw += wt;
                                            for m in 0..3 {
                                                sx[m] += wt * p[m];
                                            }
                                        }
                                    };
                                    add(p);
                                    if let Some(b) = body {
                                        let e = [p[0] - b.center[0], p[1] - b.center[1], p[2] - b.center[2]];
                                        let d = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt();
                                        if d > 0. && d < 2. * b.radius {
                                            let f = (2. * b.radius - d) / d;
                                            add([b.center[0] + f * e[0], b.center[1] + f * e[1], b.center[2] + f * e[2]]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // S399 : près de la zone, la reconstruction compte aussi les particules **virtuelles** des colonnes.
        if self.columns.is_some() {
            let (w, v) = self.virtual_column_sums(q, i, j, reach, radius, inv_r2, [images_x, [images_y[0], images_y[1], images_y[2]]], images_z);
            sw += w;
            for m in 0..3 {
                sx[m] += v[m];
            }
        }
        Some(if sw > 0. {
            let m = [sx[0] / sw - q[0], sx[1] / sw - q[1], sx[2] / sw - q[2]];
            (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt() - self.radius
        } else {
            dx
        })
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
pub const PRESSURE_TOLERANCE2: f64 = 1e-12;
pub const PRESSURE_MAX_ITERATIONS: u32 = 4000;
/// Couches d'extrapolation des vitesses vers l'air (S318).
pub const EXTRAPOLATION_LAYERS: usize = 3;
/// Séparation des particules : distance minimale en mailles, passes (S320 P3).
pub const SEPARATION: f32 = 0.4;
pub const SEPARATION_PASSES: usize = 2;
/// Plancher de la fraction fantôme, comme en 2D.
pub const THETA_MIN: f32 = 0.01;

/// **S416 — les étages du pas**, pour qu'un banc (la carte, C7) compare la production à la référence étage par étage, sur le
/// même état d'entrée. `step_upto(d, s)` exécute le pas jusqu'à l'étage `s` compris, puis s'arrête : l'état est alors celui
/// d'un pas interrompu, que seul un banc lit. `Full` est le pas, au bit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApicStage {
    /// Particules → grille.
    ParticlesToGrid,
    /// Tri par maille, surface reconstruite, étiquettes.
    Reconstruct,
    /// Gravité, parois, projection.
    Project,
    /// Extrapolation vers l'air.
    Extrapolate,
    /// Grille → particules.
    GridToParticles,
    /// Advection RK2.
    Advect,
    /// Le pas entier : séparation, corps, échange de la zone.
    Full,
}

impl Apic3 {
    /// Le plus grand pas stable, µs, sous `max_us` : `0,5·dx / (max|v| + √(g·dx))`, comme en 2D.
    pub fn stable_step_us(&self, max_us: u64) -> u64 {
        let vmax = self.vel[..self.n].iter().fold(0f32, |m, v| m.max(v[0].abs()).max(v[1].abs()).max(v[2].abs()));
        // S398 : la zone des colonnes n'a pas de particules ; sa vitesse est celle de la grille.
        let vmax = vmax.max(self.columns_max_speed());
        let dx = self.domain.dx as f64;
        let dt = 0.5 * dx / (vmax as f64 + (self.g_eff.abs() as f64 * dx).sqrt());
        ((dt * 1e6) as u64).clamp(1, max_us)
    }

    /// **Un pas** de `duration_us` µs : particules → grille, surface reconstruite, gravité, projection à fluide fantôme,
    /// extrapolation, grille → particules, advection RK2, séparation. Aucune allocation. Refus `NotFinite` si un champ
    /// cesse d'être fini — l'état est alors celui du pas interrompu (la référence n'est pas atomique).
    pub fn step(&mut self, duration_us: u64) -> Result<ApicReport, Error> {
        self.step_upto(duration_us, ApicStage::Full)
    }

    /// **S416 — le pas jusqu'à l'étage `upto` compris** (banc de la carte, C7). `Full` est `step`, au bit ; avant `Full`,
    /// le pas s'arrête sans ses contrôles de fin et rend ce qu'il a déjà mesuré.
    pub fn step_upto(&mut self, duration_us: u64, upto: ApicStage) -> Result<ApicReport, Error> {
        self.step_marked(duration_us, upto, &mut |_| {})
    }

    /// **S483 (ADR-222 D2)** — le système de tâches des écritures disjointes du pas (la reconstruction, …) ; `None` : séquentiel.
    /// Changer de système, ou son nombre de fils, change la vitesse, jamais le résultat (S243).
    pub fn set_jobs(&mut self, jobs: Option<std::sync::Arc<dyn crate::host::JobSystem + Send + Sync>>) {
        self.jobs = jobs;
    }

    /// **S483** — le pas, avec un repère nommé à la fin de chaque étage (`mark`) : un banc y lit son horloge (le cœur n'en lit
    /// aucune). Le résultat est celui de `step_upto`, au bit.
    pub fn step_marked(&mut self, duration_us: u64, upto: ApicStage, mark: &mut dyn FnMut(&'static str)) -> Result<ApicReport, Error> {
        if duration_us == 0 || duration_us > (1u64 << 40) {
            return Err(Error::NotFinite);
        }
        // Les coefficients dimensionnés se construisent en f64 et s'arrondissent en f32 (I-08, ADR-141).
        let dt = (duration_us as f64 * 1e-6) as f32;
        // S398 : sans zone de colonnes, ces quatre appels ne font rien.
        self.columns_begin();
        self.particles_to_grid();
        mark("p2g");
        if upto == ApicStage::ParticlesToGrid {
            return Ok(ApicReport::default());
        }
        self.columns_advect(dt);
        self.reconstruct();
        mark("reconstruction");
        self.columns_label();
        self.label_body();
        // S479 : les poches d'air enfermé (rien sans `enable_air_pockets`).
        self.pockets_detect();
        mark("etiquettes_poches");
        if upto == ApicStage::Reconstruct {
            return Ok(ApicReport::default());
        }
        if self.is_relative() {
            // S444 : en mode relatif, `−dt·u′·∇U` sur la grille ; la gravité est dans la pression de B.
            self.relative_strain(dt);
        } else {
            let gdt = (self.g_eff as f64 * duration_us as f64 * 1e-6) as f32;
            for w in self.w.iter_mut() {
                *w -= gdt;
            }
        }
        self.walls();
        self.impose_body();
        let (iterations, residual) = if self.poches.is_some() { self.project_with_pockets(dt) } else { self.project(dt) };
        mark("projection");
        let divergence = self.divergence_metric();
        let partial = ApicReport { iterations, residual, divergence, max_speed: 0. };
        if upto == ApicStage::Project {
            return Ok(partial);
        }
        self.extrapolate();
        self.impose_body();
        mark("extrapolation");
        if upto == ApicStage::Extrapolate {
            return Ok(partial);
        }
        self.columns_transport(dt);
        self.grid_to_particles();
        mark("g2p");
        if upto == ApicStage::GridToParticles {
            return Ok(partial);
        }
        self.advect(dt);
        mark("advection");
        if upto == ApicStage::Advect {
            return Ok(partial);
        }
        if self.separation {
            self.separate();
        }
        self.move_body(dt);
        // S399 : l'échange à la frontière de la zone des colonnes (rien sans zone).
        self.columns_exchange();
        mark("separation_corps_echange");
        let mut max_speed = 0f32;
        for k in 0..self.n {
            let (p, v) = (self.x[k], self.vel[k]);
            if !(p.iter().chain(v.iter()).all(|x| x.is_finite())) {
                return Err(Error::NotFinite);
            }
            max_speed = max_speed.max((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt());
        }
        if !self.columns_finite() {
            return Err(Error::NotFinite);
        }
        self.iterations = iterations;
        if self.is_relative() {
            self.background_time_s += dt as f64;
        }
        Ok(ApicReport { iterations, residual, divergence, max_speed })
    }

    /// Vitesse normale nulle sur le bord du domaine (parois).
    fn walls(&mut self) {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        for k in 0..nz {
            for j in 0..ny {
                // S446 : un bord ouvert impose sa vitesse normale.
                let (left, right) = match &self.open_x {
                    Some(o) => (o[k * ny + j], o[ny * nz + k * ny + j]),
                    None => (0., 0.),
                };
                self.u[(k * ny + j) * (nx + 1)] = left;
                self.u[(k * ny + j) * (nx + 1) + nx] = right;
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

    /// **Le corps dans la grille** (S393) : les mailles dont le centre est dans la sphère deviennent solides, par-dessus les
    /// étiquettes de la surface reconstruite.
    fn label_body(&mut self) {
        let Some(b) = self.body else { return };
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let r2 = b.radius * b.radius;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let q = [(i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx];
                    let d = [q[0] - b.center[0], q[1] - b.center[1], q[2] - b.center[2]];
                    if d[0] * d[0] + d[1] * d[1] + d[2] * d[2] < r2 {
                        let c = self.cell(i, j, k);
                        self.label[c] = SOLID;
                    }
                }
            }
        }
    }

    /// Toute face qui touche une maille solide prend la vitesse du corps. Appelé après chaque opération qui écrit les faces —
    /// sans quoi une extrapolation réécrirait la paroi (S320).
    fn impose_body(&mut self) {
        let Some(b) = self.body else { return };
        let Domain3 { nx, ny, nz, .. } = self.domain;
        let solid = |a: usize, j: usize, k: usize| self.label[(k * ny + j) * nx + a] == SOLID;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    if !solid(i, j, k) {
                        continue;
                    }
                    self.u[(k * ny + j) * (nx + 1) + i] = b.velocity[0];
                    self.u[(k * ny + j) * (nx + 1) + i + 1] = b.velocity[0];
                    self.v[(k * (ny + 1) + j) * nx + i] = b.velocity[1];
                    self.v[(k * (ny + 1) + j + 1) * nx + i] = b.velocity[1];
                    self.w[(k * ny + j) * nx + i] = b.velocity[2];
                    self.w[((k + 1) * ny + j) * nx + i] = b.velocity[2];
                }
            }
        }
        // Une face de corps sur le bord du domaine reste une paroi.
        self.walls();
    }

    /// Le corps avance de `velocity·dt`, puis repousse à sa surface (plus 0,05 maille) les particules qu'il a atteintes, avec
    /// une vitesse normale au moins égale à la sienne : elles ne rentrent plus (S320).
    fn move_body(&mut self, dt: f32) {
        let Some(mut b) = self.body else { return };
        for a in 0..3 {
            b.center[a] += b.velocity[a] * dt;
        }
        self.body = Some(b);
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let (lx, ly, lz) = (nx as f32 * dx, ny as f32 * dx, nz as f32 * dx);
        let (reach, margin) = (b.radius + 0.05 * dx, 1e-3 * dx);
        for k in 0..self.n {
            let p = self.x[k];
            let e = [p[0] - b.center[0], p[1] - b.center[1], p[2] - b.center[2]];
            let d = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt();
            if d >= reach {
                continue;
            }
            let n = if d > 0. { [e[0] / d, e[1] / d, e[2] / d] } else { [0., 0., 1.] };
            self.x[k] = [
                (b.center[0] + n[0] * reach).clamp(margin, lx - margin),
                (b.center[1] + n[1] * reach).clamp(margin, ly - margin),
                (b.center[2] + n[2] * reach).clamp(margin, lz - margin),
            ];
            let v = &mut self.vel[k];
            let (vn, wn) = (v[0] * n[0] + v[1] * n[1] + v[2] * n[2], b.velocity[0] * n[0] + b.velocity[1] * n[1] + b.velocity[2] * n[2]);
            if vn < wn {
                for a in 0..3 {
                    v[a] += (wn - vn) * n[a];
                }
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
                        match self.label[n] {
                            WATER => s += x[c] - x[n],
                            AIR => s += x[c] / self.theta(c, n),
                            // Le corps : paroi mobile, flux imposé, pas de pression.
                            _ => {}
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
                                    // S444 : la valeur de `p′` à la surface, en mode relatif (nulle sans fond).
                                    ghost += self.surface_pressure(c, n) / t;
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
                        if self.label[c] == SOLID || self.label[n] == SOLID {
                            continue;
                        }
                        let (wc, wn) = (self.label[c] == WATER, self.label[n] == WATER);
                        let grad = if wc && wn {
                            self.p[n] - self.p[c]
                        } else if wc {
                            (self.surface_pressure(c, n) - self.p[c]) / self.theta(c, n)
                        } else if wn {
                            (self.p[n] - self.surface_pressure(n, c)) / self.theta(n, c)
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

    /// **S444 — la valeur de `p′` au point de surface** entre la maille d'eau `w` et sa voisine d'air `a` : `−p_B` au point, moins
    /// l'erreur de B à sa propre surface à la même abscisse, `p_B(x, niveau + η_B)` — nulle quand la surface de l'eau est celle de
    /// B. Sans fond : 0 (la surface libre de l'eau totale).
    fn surface_pressure(&self, w: usize, a: usize) -> f32 {
        let Some(first) = self.background[0] else { return 0. };
        let Domain3 { nx, ny, dx, .. } = self.domain;
        let centre = |c: usize| {
            let (i, k) = (c % nx, c / (nx * ny));
            [(i as f32 + 0.5) * dx, (k as f32 + 0.5) * dx]
        };
        let (cw, ca) = (centre(w), centre(a));
        let t = self.theta(w, a);
        let (x, z) = (cw[0] + t * (ca[0] - cw[0]), cw[1] + t * (ca[1] - cw[1]));
        let ts = self.background_time_s;
        let own = first.mean_level + self.background_elevation(x, ts);
        (self.background_pressure(x, own, ts) - self.background_pressure(x, z, ts)) as f32
    }

    /// **S444 — `−dt·u′·∇U` sur la grille**, avec le gradient exact de B à chaque face ; `u′` lu par `grid_velocity` au point
    /// de la face (les trois composantes interpolées). Les incréments sont calculés avant d'être appliqués.
    fn relative_strain(&mut self, dt: f32) {
        if !self.is_relative() {
            return;
        }
        let ts = self.background_time_s;
        let Domain3 { nx, ny, nz, dx } = self.domain;
        for axis in 0..3 {
            let dims = [nx + usize::from(axis == 0), ny + usize::from(axis == 1), nz + usize::from(axis == 2)];
            let mut increments = core::mem::take(&mut self.old_u);
            for k in 0..dims[2] {
                for j in 0..dims[1] {
                    for i in 0..dims[0] {
                        let q = [
                            (i as f32 + if axis == 0 { 0. } else { 0.5 }) * dx,
                            (j as f32 + if axis == 1 { 0. } else { 0.5 }) * dx,
                            (k as f32 + if axis == 2 { 0. } else { 0.5 }) * dx,
                        ];
                        let v = self.grid_velocity(q);
                        let g = self.background_gradient(q[0], q[2], ts);
                        let f = (k * dims[1] + j) * dims[0] + i;
                        increments[f] = -dt * (v[0] * g[axis][0] + v[1] * g[axis][1] + v[2] * g[axis][2]);
                    }
                }
            }
            let field = match axis {
                0 => &mut self.u,
                1 => &mut self.v,
                _ => &mut self.w,
            };
            for (f, x) in field.iter_mut().enumerate() {
                *x += increments[f];
            }
            self.old_u = increments;
        }
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
        // S444 : en mode relatif, la vitesse de l'eau est `U + u′` — B à l'instant du début du pas, puis du milieu.
        let t0 = self.background_time_s;
        let relative = self.is_relative();
        let with_b = |a: &Self, v: [f32; 3], q: [f32; 3], t: f64| {
            if !relative {
                return v;
            }
            let u = a.background_velocity(q[0], q[2], t);
            [v[0] + u[0], v[1] + u[1], v[2] + u[2]]
        };
        for k in 0..self.n {
            let p = self.x[k];
            let v1 = with_b(self, self.grid_velocity(p), p, t0);
            let mid = [p[0] + 0.5 * dt * v1[0], p[1] + 0.5 * dt * v1[1], p[2] + 0.5 * dt * v1[2]];
            let v2 = with_b(self, self.grid_velocity(mid), mid, t0 + 0.5 * dt as f64);
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
                let mut q = [
                    (p[0] + d[0]).clamp(margin, lx - margin),
                    (p[1] + d[1]).clamp(margin, ly - margin),
                    (p[2] + d[2]).clamp(margin, lz - margin),
                ];
                // S400 : la séparation est tenue du côté de la bande — une particule qu'elle pousserait dans une colonne de la
                // zone garde sa position horizontale (l'échange ne passe que par le flux de la face).
                if self.columns.is_some() {
                    let (a, b) = (self.cell_of(q), self.cell_of(p));
                    if self.column_of(a.0, a.1) && !self.column_of(b.0, b.1) {
                        q[0] = p[0];
                        q[1] = p[1];
                    }
                }
                self.x[k] = q;
            }
        }
    }
}

#[path = "apic3d_columns.rs"]
mod columns;
#[path = "apic3d_poches.rs"]
mod poches;
pub use poches::{pockets_reserved_bytes, AirPocket, AirPocketState, GAMMA_AIR, MAX_POCKETS, POCHE_MAILLES_MIN, P_ATM, RAPPEL_VOLUME_S};
pub use columns::{columns_reserved_bytes, ColumnsChange, ColumnsSwitch, FloorChange, LinearSwell};

#[cfg(test)]
#[path = "tests_apic3d.rs"]
pub(crate) mod tests;
