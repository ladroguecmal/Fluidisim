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
//! Surface mobile (S296) : fonction hauteur et fantômes dans `delta3d_mobile.rs`, Jacobi.
//! Couplage B/W (S297) dans `delta3d_coupling.rs`. Ni faces coupées ni budget coopératif : la
//! référence sort de la boucle d'image, I-05 y est porté par la production. Le mode linéaire reste sans préconditionneur :
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
    rest: f32,
    prec: Vec<f32>,
    surface_coupled: bool,
    homogeneous_ghost: bool,
    surface_total: Vec<f32>,
    ghost_bg_up: Vec<f32>,
    ghost_bg_x: Vec<f32>,
    ghost_bg_y: Vec<f32>,
    pressure_base: Vec<f32>,
    band_x: Vec<f32>,
    band_y: Vec<f32>,
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
    /// S310 : bilan de masse du dernier pas couplé. Des `f64` : aucune allocation, aucun tampon.
    balance: Balance3,
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
        // Trois jeux de faces ; huit champs de mailles (dont Jacobi et pression avant affinage),
        // six de colonnes (hauteur/reste/sauvegardes, total, fantôme vertical), fantômes
        // latéraux u/v, et deux jeux de flux (perturbation et bande) par axe horizontal.
        let floats = nu.checked_add(nv).and_then(|n| n.checked_add(nw)).and_then(|n| n.checked_mul(3))
            .and_then(|n| cells.checked_mul(8).and_then(|c| n.checked_add(c)))
            .and_then(|n| cols.checked_mul(6).and_then(|c| n.checked_add(c)))
            .and_then(|n| n.checked_add(nu)).and_then(|n| n.checked_add(nv))
            .and_then(|n| fx.checked_mul(2).and_then(|c| n.checked_add(c)))
            .and_then(|n| fy.checked_mul(2).and_then(|c| n.checked_add(c)))
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
            rest: z0,
            prec: vec![0.; cells],
            surface_coupled: false,
            homogeneous_ghost: false,
            surface_total: vec![z0; cols],
            ghost_bg_up: vec![0.; cols],
            ghost_bg_x: vec![0.; nu],
            ghost_bg_y: vec![0.; nv],
            pressure_base: vec![0.; cells],
            band_x: vec![0.; fx],
            band_y: vec![0.; fy],
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
            balance: Balance3::default(),
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

    /// Divergence des faces, dans `out`. Parties `x` puis `z` dans l'ordre de la 2D, la partie `y`
    /// ajoutée ensuite : à `ny = 1` elle vaut zéro et la valeur est celle de la 2D.
    fn divergence(&self, u: &[f32], v: &[f32], w: &[f32], out: &mut [f32]) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.c(i, j, k);
                    let (fl, fr) = (u[self.fu(i, j, k)], u[self.fu(i + 1, j, k)]);
                    let (ff, fk) = (v[self.fv(i, j, k)], v[self.fv(i, j + 1, k)]);
                    let (fb, ft) = (w[self.fw(i, j, k)], w[self.fw(i, j, k + 1)]);
                    out[c] = ((fr - fl + ft - fb) + (fk - ff)) / dx;
                }
            }
        }
    }

    /// Produit scalaire, réduit dans l'ordre des indices par tranches de 64 (SPEC-004 §8.2) ;
    /// produits, sommes de tranches et fusion arrondis en f32 — la sémantique de la 2D, et aucune
    /// accumulation double cachée (I-08, précision S233).
    fn dot(&self, a: &[f32], b: &[f32], jobs: &dyn JobSystem) -> f32 {
        let reduce = |start: usize, end: usize| {
            let mut acc = 0f32;
            for c in start..end { acc += a[c] * b[c]; }
            acc as f64
        };
        let merge = |x: f64, y: f64| (x as f32 + y as f32) as f64;
        jobs.parallel_reduce_ordered_f64(a.len(), 64, &reduce, &merge, 0.) as f32
    }

    /// Une norme non représentable ne fait jamais passer un second membre non nul pour le repos.
    fn norm2(&self, a: &[f32], jobs: &dyn JobSystem) -> Result<f32, Error> {
        let n = self.dot(a, a, jobs);
        if !n.is_finite() { return Err(Error::NotFinite); }
        if n == 0. && a.iter().any(|x| *x != 0.) { return Err(Error::NotFinite); }
        Ok(n)
    }

    /// Erreur inverse composante par composante (Oettli–Prager) du vrai résidu `res`,
    /// `max_i |r_i| / (|b| + |A||p|)_i`. Certificat d'arrêt au plancher (ADR-143), jamais critère
    /// d'acceptation. Doit être appelée quand `res = rhs − A·p`.
    fn backward_error(&self) -> f32 {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        let mut worst = 0f32;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.c(i, j, k);
                    let pc = self.p[c].abs();
                    let mut acc = 0f32;
                    if i > 0 { acc += pc + self.p[c - 1].abs(); }
                    if i + 1 < nx { acc += pc + self.p[c + 1].abs(); }
                    if j > 0 { acc += pc + self.p[c - nx].abs(); }
                    if j + 1 < ny { acc += pc + self.p[c + nx].abs(); }
                    if k > 0 { acc += pc + self.p[c - nx * ny].abs(); }
                    if k + 1 < nz { acc += pc + self.p[c + nx * ny].abs(); } else { acc += pc * 2.; }
                    let scale = self.rhs[c].abs() + acc * inv;
                    if scale > 0. { worst = worst.max(self.res[c].abs() / scale); }
                }
            }
        }
        worst
    }

    /// Correction du champ prédit dans les tampons publiés : `u = u* − (dt/ρ)·∇p`, demi-maille au
    /// couvercle, faces de mur intouchées.
    fn correct(&mut self, k1: f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        self.u.copy_from_slice(&self.us);
        self.v.copy_from_slice(&self.vs);
        self.w.copy_from_slice(&self.ws);
        for k in 0..nz {
            for j in 0..ny {
                for i in 1..nx {
                    let (f, l, r) = (self.fu(i, j, k), self.c(i - 1, j, k), self.c(i, j, k));
                    self.u[f] -= k1 * (self.p[r] - self.p[l]) / dx;
                }
            }
        }
        for k in 0..nz {
            for j in 1..ny {
                for i in 0..nx {
                    let (f, a, b) = (self.fv(i, j, k), self.c(i, j - 1, k), self.c(i, j, k));
                    self.v[f] -= k1 * (self.p[b] - self.p[a]) / dx;
                }
            }
        }
        for k in 1..=nz {
            for j in 0..ny {
                for i in 0..nx {
                    let (f, below) = (self.fw(i, j, k), self.c(i, j, k - 1));
                    if k < nz {
                        let above = self.c(i, j, k);
                        self.w[f] -= k1 * (self.p[above] - self.p[below]) / dx;
                    } else {
                        self.w[f] -= k1 * (self.lid(i, j) - self.p[below]) / (0.5 * dx);
                    }
                }
            }
        }
    }

    /// `D = max|div u|·dx / max|u|` sur le champ corrigé : la tolérance physique de S199, que la
    /// référence tient comme ADR-144 le demande. Sans fantôme de surface, toutes les lignes sont
    /// franches. Consomme `tmp`.
    fn divergence_metric(&mut self) -> f64 {
        let mut tmp = core::mem::take(&mut self.tmp);
        self.divergence(&self.u, &self.v, &self.w, &mut tmp);
        let dmax = tmp.iter().fold(0f32, |m, x| m.max(x.abs()));
        self.tmp = tmp;
        let umax = self.u.iter().chain(&self.v).chain(&self.w).fold(0f32, |m, x| m.max(x.abs()));
        let ratio = if umax > 0. { self.domain.dx / umax } else { 0. };
        (dmax * ratio) as f64
    }

    /// Projection : résout `L p = scale·div(u*)` plus le couvercle, puis corrige.
    ///
    /// Le gradient conjugué est celui du chemin 2D à couvercle fixe, départ `p = 0`, et ses arrêts
    /// sont ceux de la référence 2D : critère premier `‖b − Ap‖/‖b‖ ≤ 10⁻⁶` sur le vrai résidu
    /// recalculé ; tolérance physique d'ADR-144 exigée pour accepter, avec cible resserrée tant
    /// qu'elle manque ; arrêt au plancher par l'erreur inverse `≤ γ₁₀` ou par retour au bit de `p`
    /// (Brent, ADR-143). `max_iters` borne toutes les itérations, relances comprises.
    fn project(&mut self, scale: f32, k1: f32, max_iters: u32, jobs: &dyn JobSystem) -> Result<Report, Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        let mut rhs = core::mem::take(&mut self.rhs);
        self.divergence(&self.us, &self.vs, &self.ws, &mut rhs);
        self.rhs = rhs;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.c(i, j, k);
                    let mut b = scale * self.rhs[c];
                    // La valeur imposée au couvercle entre ici, et nulle part ailleurs.
                    if k + 1 == nz { b += 2. * self.lid(i, j) * inv; }
                    self.rhs[c] = b;
                }
            }
        }
        let b2 = self.norm2(&self.rhs, jobs)?;
        self.p.fill(0.);
        self.res.copy_from_slice(&self.rhs);
        let mut rr = b2;
        let mut primed = false;
        let mut it = 0u32;
        let tol = 1e-12_f32;
        let mut floor_stop = false;
        let mut physical_target = f32::INFINITY;
        let (mut checkpoint, mut power, mut since): (Option<u64>, u32, u32) = (None, 1, 0);
        // `settled` porte la divergence quand la porte d'ADR-144 vient de la mesurer sur le `p`
        // publié : la queue ne refait alors ni la correction ni la mesure.
        let (actual_rr, settled) = loop {
            let before = it;
            while b2 > 0. && (rr > tol * b2 || rr > physical_target) && it < max_iters {
                if !primed {
                    self.dir.copy_from_slice(&self.res);
                    primed = true;
                }
                let mut tmp = core::mem::take(&mut self.tmp);
                self.apply(&self.dir, &mut tmp);
                self.tmp = tmp;
                let dq = self.dot(&self.dir, &self.tmp, jobs);
                if !(dq > 0.) { break; }
                let alpha = rr / dq;
                for c in 0..self.p.len() {
                    self.p[c] += alpha * self.dir[c];
                    self.res[c] -= alpha * self.tmp[c];
                }
                let rn = self.norm2(&self.res, jobs)?;
                let beta = rn / rr;
                for c in 0..self.dir.len() {
                    self.dir[c] = self.res[c] + beta * self.dir[c];
                }
                rr = rn;
                it += 1;
            }
            // Vrai résidu recalculé : c'est lui qui décide, jamais la récurrence.
            let mut tmp = core::mem::take(&mut self.tmp);
            self.apply(&self.p, &mut tmp);
            self.tmp = tmp;
            for c in 0..self.res.len() { self.res[c] = self.rhs[c] - self.tmp[c]; }
            let actual = self.norm2(&self.res, jobs)?;
            let exhausted = it >= max_iters || it == before;
            let mut measured = None;
            if actual <= tol * b2 {
                if exhausted { break (actual, None); }
                self.correct(k1);
                let reached = self.divergence_metric();
                if reached <= PROJECTION_DIVERGENCE_TOLERANCE { break (actual, Some(reached)); }
                measured = Some(reached);
                let ratio = (PROJECTION_DIVERGENCE_TOLERANCE / reached) as f32;
                physical_target = actual * ratio * ratio;
            } else if exhausted {
                break (actual, None);
            }
            if self.backward_error() <= ROUNDOFF_BACKWARD_ERROR_3D {
                floor_stop = true;
                break (actual, measured);
            }
            let state = fingerprint(&self.p);
            if checkpoint == Some(state) {
                floor_stop = true;
                break (actual, measured);
            }
            since += 1;
            if since == power {
                checkpoint = Some(state);
                power = power.saturating_mul(2);
                since = 0;
            }
            rr = actual;
            self.dir.copy_from_slice(&self.res);
        };
        let residual = if b2 > 0. { (actual_rr / b2).sqrt() } else { 0. };
        let divergence = match settled {
            Some(d) => d,
            None => {
                self.correct(k1);
                self.divergence_metric()
            }
        };
        let accepted = (actual_rr <= tol * b2 || floor_stop) && divergence <= PROJECTION_DIVERGENCE_TOLERANCE;
        Ok(Report {
            refinements: 0,
            iterations: it,
            degraded: b2 > 0. && !accepted,
            residual: residual as f64,
            divergence,
            floor: floor_stop,
            backward_error: self.backward_error() as f64,
            divergence_plain: divergence,
        })
    }
}

// ---------------------------------------------------------------- le pas à surface linéarisée

impl Volume3 {
    /// Surface linéarisée sur géométrie fixe — ADR-141 en trois dimensions. La durée entre en
    /// microsecondes entières et ne devient jamais f32 : seuls `ρ/dt`, `dt/ρ` et `dt/dx` le
    /// deviennent. Pression d'abord (modèle entièrement linéaire, aucune advection), puis hauteur
    /// transportée par les flux de colonne. Garde `dt²·g/dx ≤ 1` de S233.
    ///
    /// **Refus atomique** : une pression non convergée rend `Convergence`, un champ non fini
    /// `NotFinite`, et `u`, `v`, `w`, `p`, `η` et son reste sont rendus au bit. Aucune allocation.
    pub fn step_surface_linear(&mut self, duration_us: u64, max_iters: u32, jobs: &dyn JobSystem) -> Result<Report, Error> {
        if duration_us == 0 || duration_us > (1u64 << 53) {
            return Err(Error::NotFinite);
        }
        let dt = duration_us as f64 * 1e-6;
        let dx = self.domain.dx;
        if self.g_eff <= 0. || dt * dt * self.g_eff as f64 / dx as f64 > 1. {
            return Err(Error::Domain);
        }
        let scale = (-self.rho as f64 / dt) as f32;
        let correction = (dt / self.rho as f64) as f32;
        let transport = (dt / dx as f64) as f32;
        if !scale.is_finite() || !correction.is_finite() || correction == 0.
            || !transport.is_finite() || transport == 0. {
            return Err(Error::NotFinite);
        }
        self.saved_u.copy_from_slice(&self.u);
        self.saved_v.copy_from_slice(&self.v);
        self.saved_w.copy_from_slice(&self.w);
        self.saved_p.copy_from_slice(&self.p);
        self.saved_eta.copy_from_slice(&self.eta);
        self.saved_eta_roundoff.copy_from_slice(&self.eta_roundoff);
        let result = self.linear(scale, correction, transport, max_iters, jobs);
        if result.is_err() {
            self.u.copy_from_slice(&self.saved_u);
            self.v.copy_from_slice(&self.saved_v);
            self.w.copy_from_slice(&self.saved_w);
            self.p.copy_from_slice(&self.saved_p);
            self.eta.copy_from_slice(&self.saved_eta);
            self.eta_roundoff.copy_from_slice(&self.saved_eta_roundoff);
        }
        result
    }

    fn linear(&mut self, scale: f32, correction: f32, transport: f32, max_iters: u32,
        jobs: &dyn JobSystem) -> Result<Report, Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        // Le modèle est linéaire : le champ prédit est le champ publié, sans terme quadratique.
        self.us.copy_from_slice(&self.u);
        self.vs.copy_from_slice(&self.v);
        self.ws.copy_from_slice(&self.w);
        let report = self.project(scale, correction, max_iters, jobs)?;
        if report.degraded {
            return Err(Error::Convergence);
        }
        // Flux de colonne à chaque face latérale, sommés du fond vers le couvercle comme en 2D.
        for j in 0..ny {
            for i in 0..=nx {
                let mut q = 0f32;
                for k in 0..nz { q += self.u[self.fu(i, j, k)] * dx; }
                self.flux_x[j * (nx + 1) + i] = q;
            }
        }
        for j in 0..=ny {
            for i in 0..nx {
                let mut q = 0f32;
                for k in 0..nz { q += self.v[self.fv(i, j, k)] * dx; }
                self.flux_y[j * nx + i] = q;
            }
        }
        for j in 0..ny {
            for i in 0..nx {
                let (left, right) = (self.flux_x[j * (nx + 1) + i], self.flux_x[j * (nx + 1) + i + 1]);
                let (front, back) = (self.flux_y[j * nx + i], self.flux_y[(j + 1) * nx + i]);
                let c = self.col(i, j);
                // Somme compensée f32 : un déplacement plus petit que l'ulp de z₀ survit au pas
                // suivant et participe à la pression (S233). Partie `y` ajoutée après la partie `x`.
                let increment = -transport * ((right - left) + (back - front)) - self.eta_roundoff[c];
                let height = self.eta[c] + increment;
                self.eta_roundoff[c] = (height - self.eta[c]) - increment;
                self.eta[c] = height;
            }
        }
        let finite = self.u.iter().chain(&self.v).chain(&self.w).chain(&self.p).chain(&self.eta)
            .chain(&self.eta_roundoff).chain(&self.rhs).chain(&self.res).chain(&self.dir).chain(&self.tmp)
            .all(|x| x.is_finite());
        if !finite || !report.residual.is_finite() || !report.divergence.is_finite() {
            return Err(Error::NotFinite);
        }
        Ok(report)
    }
}

/// Empreinte 64 bits (FNV-1a) d'un champ f32, sur ses bits exacts : la détection de cycle
/// d'ADR-143.
fn fingerprint(values: &[f32]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for v in values {
        for byte in v.to_bits().to_le_bytes() {
            h ^= byte as u64;
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    }
    h
}

#[cfg(test)]
#[path = "tests_delta3d.rs"]
mod tests;

#[path = "delta3d_mobile.rs"]
mod mobile;

#[path = "delta3d_coupling.rs"]
mod coupling;
pub use coupling::{BackgroundFaces3, Sponge3};
#[path = "delta3d_background.rs"]
mod background_grid;
pub use background_grid::BackgroundGrid3;
#[path = "delta3d_balance.rs"]
mod balance;
pub use balance::{Balance3, Energy3};
#[path = "delta3d_transfer.rs"]
mod transfer;
pub use transfer::{Ledger3, LedgerError};
#[path = "delta3d_closure.rs"]
mod closure;
pub use closure::{Closure3, ClosureError};
#[path = "delta3d_cut.rs"]
mod cut;
