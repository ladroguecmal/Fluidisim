//! **δ en trois dimensions — la référence CPU de la porte B** (S295, ADR-175).
//!
//! # Ce que ce module est
//!
//! La **référence** d'ADR-175 D1 : le schéma tridimensionnel est défini et reçu ici, dans le cœur,
//! sans dépendance, contre des oracles indépendants. Elle n'est **pas** le chemin de production —
//! celui-ci sera résident sur GPU dans l'hôte — et ne tourne pas dans la boucle d'image. Elle est
//! la spécification que la production reproduira, et l'instrument qui la jugera.
//!
//! Le lot 1 (S295) porte le mode le plus simple : **surface linéarisée** sur géométrie fixe, le
//! mode 2D de S233 (ADR-141) étendu à la grille MAC x-y-z. Bassin à fond plat et murs latéraux,
//! couvercle entièrement ouvert à `z₀ = nz·dx`, où la pression dynamique vaut `ρ·g·(η − z₀)`.
//! La 2D (`delta_projection`) reste intacte : elle est le témoin de `ny = 1`.
//!
//! # Ce qu'il n'est pas encore
//!
//! Ni surface mobile (lot 2), ni couplage à B/W, ni faces coupées, ni budget coopératif : la
//! référence sort de la boucle d'image, I-05 y est porté par la production. Aucun préconditionneur :
//! le gradient conjugué est celui du chemin 2D à couvercle fixe, ce qui garde une hauteur
//! indépendante de `y` **exactement** indépendante de `y` — un Jacobi, dont la diagonale change
//! au mur, la briserait à l'arrondi.
//!
//! # Rangement
//!
//! Mailles `c = (k·ny + j)·nx + i`, `x` le plus rapide : à `ny = 1`, l'ordre est celui de la 2D.
//! Faces `u` (normale x) `(nx+1)·ny·nz`, faces `v` (normale y) `nx·(ny+1)·nz`, faces `w`
//! (normale z) `nx·ny·(nz+1)` ; colonnes `j·nx + i`.
use crate::delta_projection::{Error, Report, PROJECTION_DIVERGENCE_TOLERANCE};
use crate::host::{AllocError, HostServices, JobSystem};

/// Domaine local, cellules cubiques de côté `dx`. `z₀ = nz·dx` est le couvercle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Domain3 {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    /// Côté d'une cellule, m.
    pub dx: f32,
}

impl Domain3 {
    pub fn z0(&self) -> f32 {
        self.nz as f32 * self.dx
    }
    pub fn cells(&self) -> usize {
        self.nx * self.ny * self.nz
    }
    pub fn columns(&self) -> usize {
        self.nx * self.ny
    }
}

/// ADR-143 transposé à six faces, **dérivé et non transposé** comme ADR-143 l'exige : une ligne à
/// `m = 6` faces évalue chaque terme `a·(p_c − p_j)` avec `γ₂`, les somme avec `γ₅`, multiplie par
/// `1/dx²` et soustrait de `b`, soit `γ_{m+3} = γ₉` ; la représentation f32 de la solution ajoute
/// `u`. `γ_{m+4} = γ₁₀ = 10u/(1 − 10u)`, `u = 2⁻²⁴`. Critère d'**arrêt** seulement.
pub const ROUNDOFF_BACKWARD_ERROR_3D: f32 = {
    let u = 1. / 16_777_216.;
    10. * u / (1. - 10. * u)
};

/// La référence tridimensionnelle. Tampons réservés auprès de l'hôte avant `seal()` (I-06) ;
/// aucun appel à l'allocateur global dans le pas.
pub struct Volume3 {
    domain: Domain3,
    rho: f32,
    /// **Fournie**, jamais codée en dur (ADR-007 §2, I-07).
    g_eff: f32,
    /// Élévation de surface par colonne, m. `η ≡ z₀` est le repos.
    eta: Vec<f32>,
    /// Reste de la somme compensée de `η` (S233) : un déplacement plus petit que l'ulp de `z₀`
    /// survit au pas suivant et participe à la pression.
    eta_roundoff: Vec<f32>,
    u: Vec<f32>,
    v: Vec<f32>,
    w: Vec<f32>,
    us: Vec<f32>,
    vs: Vec<f32>,
    ws: Vec<f32>,
    p: Vec<f32>,
    rhs: Vec<f32>,
    res: Vec<f32>,
    dir: Vec<f32>,
    tmp: Vec<f32>,
    saved_u: Vec<f32>,
    saved_v: Vec<f32>,
    saved_w: Vec<f32>,
    saved_p: Vec<f32>,
    saved_eta: Vec<f32>,
    saved_eta_roundoff: Vec<f32>,
    /// Flux de colonne à travers les faces `u` et `v`, `Σ_k u·dx`, pour le transport de `η`.
    flux_x: Vec<f32>,
    flux_y: Vec<f32>,
}

impl Volume3 {
    #[inline]
    fn c(&self, i: usize, j: usize, k: usize) -> usize {
        (k * self.domain.ny + j) * self.domain.nx + i
    }
    #[inline]
    fn fu(&self, i: usize, j: usize, k: usize) -> usize {
        (k * self.domain.ny + j) * (self.domain.nx + 1) + i
    }
    #[inline]
    fn fv(&self, i: usize, j: usize, k: usize) -> usize {
        (k * (self.domain.ny + 1) + j) * self.domain.nx + i
    }
    #[inline]
    fn fw(&self, i: usize, j: usize, k: usize) -> usize {
        (k * self.domain.ny + j) * self.domain.nx + i
    }
    #[inline]
    fn col(&self, i: usize, j: usize) -> usize {
        j * self.domain.nx + i
    }

    /// Construit la référence. **À l'initialisation uniquement, avant `seal()`** (I-06) ; `g_eff`
    /// est fourni. Tous les tampons sont comptés à leur taille réelle avant toute allocation.
    pub fn configure(host: &mut HostServices, domain: Domain3, rho: f32, g_eff: f32) -> Result<Self, Error> {
        let Domain3 { nx, ny, nz, dx } = domain;
        if nx == 0 || ny == 0 || nz == 0 || !dx.is_finite() || dx <= 0. {
            return Err(Error::Domain);
        }
        if !rho.is_finite() || rho <= 0. || !g_eff.is_finite() {
            return Err(Error::NotFinite);
        }
        let cells = nx.checked_mul(ny).and_then(|n| n.checked_mul(nz)).ok_or(Error::Domain)?;
        let cols = nx.checked_mul(ny).ok_or(Error::Domain)?;
        let nu = (nx + 1).checked_mul(ny).and_then(|n| n.checked_mul(nz)).ok_or(Error::Domain)?;
        let nv = (ny + 1).checked_mul(nx).and_then(|n| n.checked_mul(nz)).ok_or(Error::Domain)?;
        let nw = (nz + 1).checked_mul(cols).ok_or(Error::Domain)?;
        let fx = (nx + 1).checked_mul(ny).ok_or(Error::Domain)?;
        let fy = (ny + 1).checked_mul(nx).ok_or(Error::Domain)?;
        // Trois jeux de faces (publié, prédit, sauvegardé), six champs de mailles (pression,
        // second membre, résidu, direction, produit, sauvegarde), quatre de colonnes (hauteur,
        // reste, et leurs sauvegardes), deux jeux de flux de colonne.
        let floats = nu.checked_add(nv).and_then(|n| n.checked_add(nw)).and_then(|n| n.checked_mul(3))
            .and_then(|n| cells.checked_mul(6).and_then(|c| n.checked_add(c)))
            .and_then(|n| cols.checked_mul(4).and_then(|c| n.checked_add(c)))
            .and_then(|n| n.checked_add(fx)).and_then(|n| n.checked_add(fy))
            .ok_or(Error::Domain)?;
        let bytes = floats.checked_mul(core::mem::size_of::<f32>()).ok_or(Error::Domain)?;
        if !(domain.z0().is_finite() && (nx as f32 * dx).is_finite() && (ny as f32 * dx).is_finite()) {
            return Err(Error::Domain);
        }
        host.alloc.alloc_persistent(bytes).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        let z0 = domain.z0();
        Ok(Volume3 {
            domain,
            rho,
            g_eff,
            eta: vec![z0; cols],
            eta_roundoff: vec![0.; cols],
            u: vec![0.; nu],
            v: vec![0.; nv],
            w: vec![0.; nw],
            us: vec![0.; nu],
            vs: vec![0.; nv],
            ws: vec![0.; nw],
            p: vec![0.; cells],
            rhs: vec![0.; cells],
            res: vec![0.; cells],
            dir: vec![0.; cells],
            tmp: vec![0.; cells],
            saved_u: vec![0.; nu],
            saved_v: vec![0.; nv],
            saved_w: vec![0.; nw],
            saved_p: vec![0.; cells],
            saved_eta: vec![z0; cols],
            saved_eta_roundoff: vec![0.; cols],
            flux_x: vec![0.; fx],
            flux_y: vec![0.; fy],
        })
    }

    pub fn domain(&self) -> Domain3 {
        self.domain
    }

    /// Élévation de surface par colonne ; `η ≡ z₀` est le repos. Réinitialise le reste d'arrondi.
    pub fn set_surface(&mut self, eta: &[f32]) -> Result<(), Error> {
        if eta.len() != self.eta.len() {
            return Err(Error::Shape);
        }
        if eta.iter().any(|e| !e.is_finite()) {
            return Err(Error::NotFinite);
        }
        self.eta.copy_from_slice(eta);
        self.eta_roundoff.fill(0.);
        Ok(())
    }

    /// Injecte un champ de vitesse, pour les réceptions. Les faces de mur doivent être nulles.
    pub fn set_velocity(&mut self, u: &[f32], v: &[f32], w: &[f32]) -> Result<(), Error> {
        if u.len() != self.u.len() || v.len() != self.v.len() || w.len() != self.w.len() {
            return Err(Error::Shape);
        }
        if u.iter().chain(v).chain(w).any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        self.u.copy_from_slice(u);
        self.v.copy_from_slice(v);
        self.w.copy_from_slice(w);
        self.close_walls();
        Ok(())
    }

    pub fn surface(&self) -> &[f32] {
        &self.eta
    }
    pub fn velocity_u(&self) -> &[f32] {
        &self.u
    }
    pub fn velocity_v(&self) -> &[f32] {
        &self.v
    }
    pub fn velocity_w(&self) -> &[f32] {
        &self.w
    }
    pub fn pressure(&self) -> &[f32] {
        &self.p
    }

    /// Murs latéraux et fond : vitesse normale nulle, par construction et non par tolérance.
    fn close_walls(&mut self) {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        for k in 0..nz {
            for j in 0..ny {
                let (a, b) = (self.fu(0, j, k), self.fu(nx, j, k));
                self.u[a] = 0.;
                self.u[b] = 0.;
            }
            for i in 0..nx {
                let (a, b) = (self.fv(i, 0, k), self.fv(i, ny, k));
                self.v[a] = 0.;
                self.v[b] = 0.;
            }
        }
        for j in 0..ny {
            for i in 0..nx {
                let f = self.fw(i, j, 0);
                self.w[f] = 0.;
            }
        }
    }

    /// Pression dynamique imposée au couvercle de la colonne `(i, j)` ; `homogeneous` l'annule.
    #[inline]
    fn lid(&self, i: usize, j: usize) -> f32 {
        let c = self.col(i, j);
        self.rho * self.g_eff * ((self.eta[c] - self.domain.z0()) - self.eta_roundoff[c])
    }

    /// `L p`, avec `L = −∇·∇` sur la grille MAC, Neumann aux murs et au fond, **Dirichlet
    /// homogène** au couvercle à une demi-maille. La valeur imposée vit dans le second membre :
    /// c'est ce qui garde l'opérateur symétrique, donc le gradient conjugué valide. Ordre des faces
    /// gauche, droite, avant, arrière, bas, haut : à `ny = 1`, les faces `y` sont des murs et
    /// n'ajoutent rien, et la somme est celle de la 2D, terme à terme.
    pub(crate) fn apply(&self, p: &[f32], out: &mut [f32]) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.c(i, j, k);
                    let pc = p[c];
                    let mut acc = 0.0f32;
                    if i > 0 { acc += pc - p[c - 1]; }
                    if i + 1 < nx { acc += pc - p[c + 1]; }
                    if j > 0 { acc += pc - p[c - nx]; }
                    if j + 1 < ny { acc += pc - p[c + nx]; }
                    if k > 0 { acc += pc - p[c - nx * ny]; }
                    if k + 1 < nz { acc += pc - p[c + nx * ny]; } else { acc += 2. * pc; }
                    out[c] = acc * inv;
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "tests_delta3d.rs"]
mod tests;
