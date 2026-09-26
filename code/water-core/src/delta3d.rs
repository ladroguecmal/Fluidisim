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

/// **S335 : l'ouverture la plus mince que le couvercle partiel prenne au mot** (A317). Une colonne en lamelle
/// plus mince garde la raideur de celle-ci : la découpe d'une coque qui tourne laisse, d'un pas à l'autre, un
/// reste dans la hauteur de remplissage que `1/a` changerait en pointe — 5,5 m/s sous 10 %, 0,55 m/s à 10 %.
pub const PARTIAL_LID_MIN_APERTURE: f32 = 0.1;

/// S375, ADR-201 — **le plancher de
/// l'échelle de vitesse** du critère de divergence du pas mobile 3D (ADR-143 : `max|div u|·dx/max|u|` ≤ 10⁻⁵), m/s. Sous
/// 0,1 mm/s, le critère devient absolu : `max|div u|·dx` ≤ 10⁻⁹ m/s, une dérive de surface de l'ordre de 50 µm par heure
/// sur 1,4 m d'eau — bien sous la tolérance d'image (3 mm). Au-dessus, le critère d'ADR-143, au bit.
pub const PROJECTION_VELOCITY_FLOOR: f32 = 1e-4;

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

/// S332 : la composante `axis` de la vitesse d'une paroi rigide au point `x` — `V + Ω × (x − c)`.
fn wall_velocity(v: [f32; 3], w: [f32; 3], c: [f32; 3], axis: usize, x: [f32; 3]) -> f32 {
    let r = [x[0] - c[0], x[1] - c[1], x[2] - c[2]];
    match axis {
        0 => v[0] + (w[1] * r[2] - w[2] * r[1]),
        1 => v[1] + (w[2] * r[0] - w[0] * r[2]),
        _ => v[2] + (w[0] * r[1] - w[1] * r[0]),
    }
}

/// S330 : le volume du solide dans chaque colonne, m³ — ce que la découpe retire au fond seul.
fn solid_columns(domain: Domain3, base: &[f32], frac: &[f32], out: &mut [f32]) {
    let Domain3 { nx, ny, nz, dx } = domain;
    let cube = (dx as f64).powi(3);
    for j in 0..ny {
        for i in 0..nx {
            let mut s = 0f64;
            for k in 0..nz {
                let c = (k * ny + j) * nx + i;
                s += (base[c] - frac[c]) as f64;
            }
            out[j * nx + i] = (s * cube) as f32;
        }
    }
}

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
    /// **S369, A289 : les termes propres à B retirés du pas couplé** — masque de `RELATIVE_*` ; 0, le pas de S297.
    relative_background: u8,
    /// **S391, A321 — le terme de second ordre de l'advection** (ADR-209), éteint par défaut : le pas d'avant au bit.
    advection_correction: bool,
    /// L'erreur de pression de B à sa propre surface, par colonne (S369) : `ρ·g·η_B − p_B(repos + η_B)`.
    ghost_bg_error: Vec<f32>,
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
    /// S375 — le rapport de la dernière projection mobile **refusée** (`Convergence`), pour le diagnostic ; `None` tant
    /// qu'aucun refus n'a eu lieu. Aucun état de δ, rien de sérialisé (I-17).
    refused_report: Option<Report>,
    /// **S324 : la découpe du fond** — fractions et ouvertures, `None` pour le fond plat de S295,
    /// dont tous les chemins restent ceux d'avant, au bit. Mode linéaire seulement : les pas mobile
    /// et couplé la refusent tant qu'ils ne la portent pas.
    cut: Option<cut::Cut3>,
    /// **S326 : Jacobi sur le chemin coupé** (A315), actif par défaut quand `ny > 1` ; à `ny = 1`, la 3D
    /// reste la 2D au bit, et la 2D résout sans préconditionneur. Le couper sert à la mesure.
    precondition_cut: bool,
    /// **S334 : la surface d'une colonne en partie couverte** (A317). Sous une coque qui perce le couvercle,
    /// `η` reste la hauteur de *remplissage* de la colonne — l'excès d'eau rapporté à sa section entière, ce
    /// que le transport conserve —, mais cet excès se tient dans la seule part libre `a` du couvercle, où la
    /// surface et la pression valent `(η − z₀)/a` fois `ρg`. **Actif par défaut depuis S335** : il converge avec
    /// la maille, le couvercle de S332 non (S334) ; l'ouverture prise au mot ne descend pas sous
    /// `PARTIAL_LID_MIN_APERTURE`, sans quoi une coque qui tourne ferait des pointes. Le couper sert à la mesure.
    partial_lid: bool,
    /// Ouverture minimale de ce quotient : `dt²·g/dx` du pas en cours. Une lamelle plus mince aurait une surface
    /// plus raide que le pas explicite de la hauteur ne la porte ; elle garde celle de ce plancher.
    lid_floor: f32,
    /// **S337 : l'éponge du mode linéaire** — celle du pas couplé (ADR-164) ; `None` par défaut : des murs.
    linear_sponge: Option<Sponge3>,
    /// Le volume que cette éponge a retiré depuis la configuration, m³ — ce que le bilan doit lui rendre.
    sponge_removed: f64,
    /// **S385 : la multigrille 3D** (ADR-207 D3), préconditionneur du pas mobile ; `None` par défaut — le Jacobi de
    /// S296, au bit. `enable_multigrid` la réserve auprès de l'hôte, avant `seal()`.
    mg: Option<multigrid3::Multigrid3>,
    /// **S386 : la colonne graduée** (ADR-208) du pas linéaire ; `None` par défaut — toutes les couches, au bit.
    /// `enable_graded` la réserve auprès de l'hôte, avant `seal()`.
    graded: Option<graded::Graded3>,
    /// **S401 : l'ensemble épars** (ADR-006 §3, C8b) — les colonnes du domaine dans la fenêtre ; `None` par défaut : la boîte
    /// entière, chaque opérateur au bit. `enable_sparse` le réserve auprès de l'hôte, avant `seal()`.
    sparse: Option<sparse::Sparse3>,
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
        // sept de colonnes (hauteur/reste/sauvegardes, total, fantôme vertical, erreur de B), fantômes
        // latéraux u/v, et deux jeux de flux (perturbation et bande) par axe horizontal.
        let floats = nu.checked_add(nv).and_then(|n| n.checked_add(nw)).and_then(|n| n.checked_mul(3))
            .and_then(|n| cells.checked_mul(8).and_then(|c| n.checked_add(c)))
            .and_then(|n| cols.checked_mul(7).and_then(|c| n.checked_add(c)))
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
            relative_background: 0,
            advection_correction: false,
            ghost_bg_error: vec![0.; cols],
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
            refused_report: None,
            cut: None,
            precondition_cut: true,
            partial_lid: true,
            lid_floor: 1.,
            linear_sponge: None,
            sponge_removed: 0.,
            mg: None,
            graded: None,
            sparse: None,
        })
    }

    /// **S324 : la référence sur un fond coupé.** Le fond est fourni au centre des colonnes, `nx·ny`
    /// valeurs, `x` le plus rapide, entre `0` et `z₀` ; la découpe de `delta3d_cut.rs` en tire les
    /// fractions et les ouvertures, **comptées auprès de l'hôte** avant `seal()` (I-06). Modes linéaire
    /// — le couvercle entièrement mouillé, comme en 2D — et mobile (S328) ; le pas couplé la refuse.
    pub fn configure_with_bottom(host: &mut HostServices, domain: Domain3, rho: f32, g_eff: f32,
        bottom: &[f32]) -> Result<Self, Error> {
        let Domain3 { nx, ny, nz, .. } = domain;
        if bottom.len() != nx.checked_mul(ny).ok_or(Error::Domain)? {
            return Err(Error::Shape);
        }
        if bottom.iter().any(|b| !b.is_finite() || *b < 0. || *b >= domain.z0()) {
            return Err(Error::NotFinite);
        }
        let mut v = Self::configure(host, domain, rho, g_eff)?;
        let faces = (nx + 1) * ny * nz + nx * (ny + 1) * nz + nx * ny * (nz + 1);
        // S328 : plus le plancher de chaque colonne, garde du pas mobile.
        let bytes = (faces + nx * ny * nz + nx * ny).checked_mul(core::mem::size_of::<f32>()).ok_or(Error::Domain)?;
        host.alloc.alloc_persistent(bytes).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        v.cut = Some(cut::cut(domain, bottom));
        v.prec_cut();
        Ok(v)
    }

    /// **S329 : la référence autour d'un solide quelconque**, posé sur le fond coupé de S324. Le solide est
    /// donné par sa distance signée aux nœuds, négative dedans, `(nx+1)·(ny+1)·(nz+1)` valeurs, `x` le plus
    /// rapide puis `y` puis `z` ; coupé exactement pour le champ linéaire qu'elles définissent
    /// (`delta3d_cut.rs`). Il ne partage aucune maille ni aucune face avec le fond et ne touche pas la
    /// couche du couvercle (`Domain`). Mêmes tampons que `configure_with_bottom` : le solide n'ajoute rien à
    /// la mémoire de δ.
    pub fn configure_with_solid(host: &mut HostServices, domain: Domain3, rho: f32, g_eff: f32, bottom: &[f32],
        solid: &[f32]) -> Result<Self, Error> {
        Self::configure_solid(host, domain, rho, g_eff, bottom, solid, false)
    }

    /// **S332 : la référence autour d'une coque qui flotte** — le solide peut percer le couvercle du mode
    /// linéaire, dont les faces qu'il couvre deviennent paroi ; la condition de surface ne tient que sur la
    /// part libre de chaque face. Le fond, lui, ne doit toujours pas l'atteindre. Le pas mobile refuse un
    /// solide dans la couche du couvercle par son plancher.
    pub fn configure_with_floating_solid(host: &mut HostServices, domain: Domain3, rho: f32, g_eff: f32, bottom: &[f32],
        solid: &[f32]) -> Result<Self, Error> {
        Self::configure_solid(host, domain, rho, g_eff, bottom, solid, true)
    }

    fn configure_solid(host: &mut HostServices, domain: Domain3, rho: f32, g_eff: f32, bottom: &[f32],
        solid: &[f32], piercing: bool) -> Result<Self, Error> {
        let Domain3 { nx, ny, nz, .. } = domain;
        let nodes = (nx + 1).checked_mul(ny + 1).and_then(|n| n.checked_mul(nz + 1)).ok_or(Error::Domain)?;
        if solid.len() != nodes {
            return Err(Error::Shape);
        }
        if solid.iter().any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        let mut v = Self::configure_with_bottom(host, domain, rho, g_eff, bottom)?;
        // S330 : la découpe du fond seul et le volume du solide par colonne, pour qu'il puisse bouger ; S334 : l'eau
        // qu'il dépose dans chaque colonne.
        let faces = (nx + 1) * ny * nz + nx * (ny + 1) * nz + nx * ny * (nz + 1);
        let bytes = (faces + nx * ny * nz + 3 * nx * ny).checked_mul(core::mem::size_of::<f32>()).ok_or(Error::Domain)?;
        host.alloc.alloc_persistent(bytes).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        let g = v.cut.as_mut().expect("fond coupé");
        g.piercing = piercing;
        let mut base = g.base(nx * ny);
        cut::add_solid(g, domain, solid)?;
        solid_columns(domain, &base.frac, &g.frac, &mut base.solid_col);
        g.base = Some(base);
        v.prec_cut();
        Ok(v)
    }

    /// **S330 : le solide bouge.** L'hôte donne sa distance signée à la nouvelle position et sa vitesse
    /// de translation. La découpe est refaite **en place, sans allocation**, depuis celle du fond seul ;
    /// une face qui s'ouvre naît à la vitesse du solide, une face qui se ferme perd la sienne ; l'eau que
    /// le solide déplace dans une colonne en élève la surface ; la diagonale de Jacobi suit. Refus
    /// atomique — `Shape`, `NotFinite`, `Domain` si le volume n'a pas de solide ou si le solide touche
    /// le fond ou la couche du couvercle — : rien n'est écrit.
    pub fn set_solid(&mut self, solid: &[f32], velocity: [f32; 3]) -> Result<(), Error> {
        self.set_solid_rigid(solid, velocity, [0.; 3], [0.; 3])
    }

    /// **S332 : le solide en mouvement de corps rigide** — translation `velocity`, rotation `angular`
    /// (rad/s) autour de `center` : la paroi avance à `V + Ω × (x − c)`, évaluée au centre de chaque face.
    /// Sans rotation, c'est `set_solid`, au bit.
    pub fn set_solid_rigid(&mut self, solid: &[f32], velocity: [f32; 3], angular: [f32; 3], center: [f32; 3])
        -> Result<(), Error> {
        let domain = self.domain;
        let Domain3 { nx, ny, nz, dx } = domain;
        if solid.len() != (nx + 1) * (ny + 1) * (nz + 1) {
            return Err(Error::Shape);
        }
        if solid.iter().chain(&velocity).chain(&angular).chain(&center).any(|x| !x.is_finite()) {
            return Err(Error::NotFinite);
        }
        let g = self.cut.as_mut().ok_or(Error::Domain)?;
        let mut base = g.base.take().ok_or(Error::Domain)?;
        if let Err(e) = cut::check_solid(&base.frac, &base.open_u, &base.open_v, &base.open_w, domain, solid, g.piercing) {
            g.base = Some(base);
            return Err(e);
        }
        // Les ouvertures d'avant, dans les tampons de sauvegarde, libres entre deux pas.
        self.saved_u.copy_from_slice(&g.open_u);
        self.saved_v.copy_from_slice(&g.open_v);
        self.saved_w.copy_from_slice(&g.open_w);
        g.frac.copy_from_slice(&base.frac);
        g.open_u.copy_from_slice(&base.open_u);
        g.open_v.copy_from_slice(&base.open_v);
        g.open_w.copy_from_slice(&base.open_w);
        g.floor.copy_from_slice(&base.floor);
        cut::add_solid(g, domain, solid).expect("vérifié");
        g.solid_velocity = velocity;
        g.solid_angular = angular;
        g.solid_center = center;
        // Une face qui s'ouvre naît à la vitesse normale de la paroi en son centre ; une face fermée perd la sienne.
        let paroi = |axis: usize, x: [f32; 3]| wall_velocity(velocity, angular, center, axis, x);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..=nx {
                    let f = (k * ny + j) * (nx + 1) + i;
                    if g.open_u[f] == 0. { self.u[f] = 0.; } else if self.saved_u[f] == 0. {
                        self.u[f] = paroi(0, [i as f32 * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx]);
                    }
                }
            }
            for j in 0..=ny {
                for i in 0..nx {
                    let f = (k * (ny + 1) + j) * nx + i;
                    if g.open_v[f] == 0. { self.v[f] = 0.; } else if self.saved_v[f] == 0. {
                        self.v[f] = paroi(1, [(i as f32 + 0.5) * dx, j as f32 * dx, (k as f32 + 0.5) * dx]);
                    }
                }
            }
        }
        for k in 0..=nz {
            for j in 0..ny {
                for i in 0..nx {
                    let f = (k * ny + j) * nx + i;
                    if g.open_w[f] == 0. { self.w[f] = 0.; } else if self.saved_w[f] == 0. {
                        self.w[f] = paroi(2, [(i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, k as f32 * dx]);
                    }
                }
            }
        }
        // L'eau déplacée : le volume solide gagné par une colonne en élève la surface. Le nouveau volume
        // est écrit dans `rhs`, libre entre deux pas : aucune allocation.
        solid_columns(domain, &base.frac, &g.frac, &mut self.rhs[..nx * ny]);
        // Somme compensée f32, comme le transport (S233) : le reste d'arrondi porte ce que `η` perd.
        let area = dx * dx;
        for c in 0..nx * ny {
            base.deposit[c] += (self.rhs[c] - base.solid_col[c]) / area;
            let increment = (self.rhs[c] - base.solid_col[c]) / area - self.eta_roundoff[c];
            let height = self.eta[c] + increment;
            self.eta_roundoff[c] = (height - self.eta[c]) - increment;
            self.eta[c] = height;
            base.solid_col[c] = self.rhs[c];
        }
        // S334 : l'eau poussée par une paroi qui glisse (A317). Quand l'ouverture du couvercle d'une colonne se
        // referme, l'eau de surface de la part recouverte — hors l'eau que la coque vient d'y déposer, que son
        // flux retire — passe aux voisines de la couche du haut, au prorata de l'ouverture de la face partagée et
        // du couvercle voisin : la paroi la pousse devant elle. Sans ce transfert, `1/a` en ferait une pointe de
        // pression. Transferts écrits dans `rhs[nx·ny..2·nx·ny]`, libre entre deux pas, puis appliqués ensemble :
        // l'ordre des colonnes n'y entre pas ; la dernière voisine reçoit le reste, la somme est exacte.
        if self.partial_lid && nz >= 2 {
            let (top, z0, k) = (nz * nx * ny, domain.z0(), nz - 1);
            let t = &mut self.rhs[nx * ny..2 * nx * ny];
            t.fill(0.);
            for j in 0..ny {
                for i in 0..nx {
                    let c = j * nx + i;
                    let (avant, apres) = (self.saved_w[top + c], g.open_w[top + c]);
                    if !(avant > 0. && apres < avant) {
                        continue;
                    }
                    let surface = ((self.eta[c] - z0) - self.eta_roundoff[c]) - base.deposit[c];
                    let pousse = surface * (1. - apres / avant);
                    let fu = |i: usize| (k * ny + j) * (nx + 1) + i;
                    let fv = |j: usize| (k * (ny + 1) + j) * nx + i;
                    let mut cibles = [(0usize, 0f32); 4];
                    if i > 0 { cibles[0] = (c - 1, g.open_u[fu(i)] * g.open_w[top + c - 1]); }
                    if i + 1 < nx { cibles[1] = (c + 1, g.open_u[fu(i + 1)] * g.open_w[top + c + 1]); }
                    if j > 0 { cibles[2] = (c - nx, g.open_v[fv(j)] * g.open_w[top + c - nx]); }
                    if j + 1 < ny { cibles[3] = (c + nx, g.open_v[fv(j + 1)] * g.open_w[top + c + nx]); }
                    let total: f32 = cibles.iter().map(|x| x.1).sum();
                    if !(total > 0.) || pousse == 0. {
                        continue;
                    }
                    let (mut reste, mut derniere) = (pousse, c);
                    for (n, poids) in cibles {
                        if poids > 0. {
                            let part = pousse * (poids / total);
                            t[n] += part;
                            reste -= part;
                            derniere = n;
                        }
                    }
                    t[derniere] += reste;
                    t[c] -= pousse;
                }
            }
            for c in 0..nx * ny {
                if t[c] != 0. {
                    let increment = t[c] - self.eta_roundoff[c];
                    let height = self.eta[c] + increment;
                    self.eta_roundoff[c] = (height - self.eta[c]) - increment;
                    self.eta[c] = height;
                }
            }
        }
        g.base = Some(base);
        self.prec_cut();
        Ok(())
    }

    /// **S330 : la force de la pression de δ sur la paroi du solide**, en newtons — la pression de la
    /// maille, sur les polygones de coupe qu'elle contient. `Domain` sans découpe, `Shape` sur une
    /// mauvaise longueur.
    pub fn solid_force(&self, solid: &[f32]) -> Result<[f64; 3], Error> {
        let Domain3 { nx, ny, nz, .. } = self.domain;
        if self.cut.is_none() {
            return Err(Error::Domain);
        }
        if solid.len() != (nx + 1) * (ny + 1) * (nz + 1) {
            return Err(Error::Shape);
        }
        Ok(cut::solid_wall_force(self.domain, solid, &|_, c| self.p[c] as f64))
    }

    /// S326 : active ou coupe le Jacobi du chemin coupé — pour la mesure ; actif par défaut.
    pub fn set_precondition_cut(&mut self, on: bool) {
        self.precondition_cut = on;
    }

    /// S334 : active ou coupe la surface des colonnes en partie couvertes (A317) — pour la mesure ; **active par
    /// défaut depuis S335**. Coupée, une colonne en lamelle garde la surface de S332, `1/a` fois trop molle.
    pub fn set_partial_lid(&mut self, on: bool) {
        self.partial_lid = on;
    }

    /// **S337 : l'éponge du mode linéaire**, celle du pas couplé (ADR-164) : au bord du domaine, les vitesses
    /// prédites s'amortissent et la hauteur revient au repos, au taux quadratique `Sponge3`. Sans elle, le mode
    /// linéaire a des murs, qui renvoient les ondes — et le bord d'un δ local se voit (verdict R15). `None` :
    /// des murs, le défaut. Refus `Domain` sur une éponge que le pas couplé refuserait.
    pub fn set_linear_sponge(&mut self, sponge: Option<Sponge3>) -> Result<(), Error> {
        if let Some(e) = sponge {
            e.validate(self.domain)?;
        }
        self.linear_sponge = sponge;
        Ok(())
    }

    /// S337 : le volume que l'éponge du mode linéaire a retiré depuis la configuration, m³ — positif quand elle
    /// retire. Le volume de δ plus celui-ci suit ce que les parois déplacent.
    pub fn linear_sponge_removed(&self) -> f64 {
        self.sponge_removed
    }

    /// **S326 : la diagonale de Jacobi du chemin coupé** — ouvertures des faces vers une maille fluide,
    /// deux fois celle du couvercle, comme la ligne de `apply_cut` ; zéro sur une maille solide. La
    /// géométrie est fixe : calculée une fois, à la configuration.
    pub(super) fn prec_cut(&mut self) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        let g = self.cut.as_ref().expect("fond coupé");
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.c(i, j, k);
                    if g.frac[c] == 0. {
                        self.prec[c] = 0.;
                        continue;
                    }
                    let mut diag = 0f32;
                    let mut face = |a: f32, n: Option<usize>, lid: bool| {
                        if a == 0. {
                            return;
                        }
                        match n {
                            Some(m) if g.frac[m] > 0. => diag += a,
                            Some(_) => {}
                            None if lid => diag += 2. * a,
                            None => {}
                        }
                    };
                    face(g.open_u[self.fu(i, j, k)], (i > 0).then(|| c - 1), false);
                    face(g.open_u[self.fu(i + 1, j, k)], (i + 1 < nx).then(|| c + 1), false);
                    face(g.open_v[self.fv(i, j, k)], (j > 0).then(|| c - nx), false);
                    face(g.open_v[self.fv(i, j + 1, k)], (j + 1 < ny).then(|| c + nx), false);
                    face(g.open_w[self.fw(i, j, k)], (k > 0).then(|| c - nx * ny), false);
                    face(g.open_w[self.fw(i, j, k + 1)], (k + 1 < nz).then(|| c + nx * ny), true);
                    self.prec[c] = if diag > 0. { 1. / (diag * inv) } else { 0. };
                }
            }
        }
    }

    /// S324 : la fraction fluide de chaque maille ; `None` sur le fond plat.
    pub fn fluid_fraction(&self) -> Option<&[f32]> {
        self.cut.as_ref().map(|g| g.frac.as_slice())
    }

    /// S324 : les ouvertures des faces `u`, `v` et `w` ; `None` sur le fond plat.
    pub fn apertures(&self) -> Option<(&[f32], &[f32], &[f32])> {
        self.cut.as_ref().map(|g| (g.open_u.as_slice(), g.open_v.as_slice(), g.open_w.as_slice()))
    }

    /// S324 : le pas couplé ne porte pas encore la découpe ; il la refuse. Le pas mobile la porte
    /// depuis S328.
    pub(crate) fn refuse_cut(&self) -> Result<(), Error> {
        if self.cut.is_some() { Err(Error::Domain) } else { Ok(()) }
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
        // S401 : hors de l'ensemble épars, le repos.
        if !self.sparse_surface_ok(eta, self.rest) {
            return Err(Error::Domain);
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
        // S401 : les faces des colonnes hors de l'ensemble épars sont nulles, comme les murs.
        self.close_sparse_walls();
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
        // S324 : une face que le fond ferme n'a pas de vitesse.
        if let Some(g) = &self.cut {
            for (x, a) in self.u.iter_mut().zip(&g.open_u) { if *a == 0. { *x = 0.; } }
            for (x, a) in self.v.iter_mut().zip(&g.open_v) { if *a == 0. { *x = 0.; } }
            for (x, a) in self.w.iter_mut().zip(&g.open_w) { if *a == 0. { *x = 0.; } }
        }
    }

    /// Pression dynamique imposée au couvercle de la colonne `(i, j)` ; `homogeneous` l'annule. **S334** : sous
    /// une coque qui perce le couvercle, l'excès d'eau d'une colonne en partie couverte se tient dans sa part
    /// libre `a`, où la surface — et la pression — valent `1/a` fois celles de la hauteur de remplissage, l'eau
    /// que la coque vient d'y déposer mise à part (A317). Couvercle plein ou fermé : la valeur d'avant, au bit.
    #[inline]
    fn lid(&self, i: usize, j: usize) -> f32 {
        let c = self.col(i, j);
        let pression = self.rho * self.g_eff * ((self.eta[c] - self.domain.z0()) - self.eta_roundoff[c]);
        match &self.cut {
            Some(g) if self.partial_lid => {
                let a = g.open_w[self.fw(i, j, self.domain.nz)];
                if a > 0. && a < 1. {
                    // L'eau que la coque vient de déposer dans la colonne n'est pas à la surface : le flux de sa
                    // paroi la retire pendant ce pas. Divisée par `a`, elle ferait une pression de pure comptabilité.
                    let depot = g.base.as_ref().map_or(0., |b| b.deposit[c]);
                    self.rho * self.g_eff * (((self.eta[c] - self.domain.z0()) - self.eta_roundoff[c]) - depot) / a.max(self.lid_floor)
                } else {
                    pression
                }
            }
            _ => pression,
        }
    }

    /// `L p`, avec `L = −∇·∇` sur la grille MAC, Neumann aux murs et au fond, **Dirichlet
    /// homogène** au couvercle à une demi-maille. La valeur imposée vit dans le second membre :
    /// c'est ce qui garde l'opérateur symétrique, donc le gradient conjugué valide. Ordre des faces
    /// gauche, droite, avant, arrière, bas, haut : à `ny = 1`, les faces `y` sont des murs et
    /// n'ajoutent rien, et la somme est celle de la 2D, terme à terme.
    pub(crate) fn apply(&self, p: &[f32], out: &mut [f32]) {
        if let Some(g) = &self.cut {
            return self.apply_cut(g, p, out);
        }
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
        if let Some(g) = &self.cut {
            return self.divergence_cut(g, u, v, w, out);
        }
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

    /// **S324 : `L p` pondéré par les ouvertures**, la ligne de la 2D portée à six faces : une maille
    /// solide rend zéro, une face ouverte sur du solide ne porte rien, le couvercle garde sa demi-maille.
    /// Ordre gauche, droite, avant, arrière, bas, haut : à `ny = 1` les faces `y` sont des murs
    /// d'ouverture nulle, et la somme est celle de la 2D, terme à terme.
    fn apply_cut(&self, g: &cut::Cut3, p: &[f32], out: &mut [f32]) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.c(i, j, k);
                    if g.frac[c] == 0. {
                        out[c] = 0.;
                        continue;
                    }
                    let mut acc = 0.0f32;
                    let mut face = |a: f32, n: Option<usize>, lid: bool| {
                        if a == 0. {
                            return;
                        }
                        match n {
                            Some(m) if g.frac[m] > 0. => acc += a * (p[c] - p[m]),
                            Some(_) => {}
                            None if lid => acc += 2. * a * p[c],
                            None => {}
                        }
                    };
                    face(g.open_u[self.fu(i, j, k)], (i > 0).then(|| c - 1), false);
                    face(g.open_u[self.fu(i + 1, j, k)], (i + 1 < nx).then(|| c + 1), false);
                    face(g.open_v[self.fv(i, j, k)], (j > 0).then(|| c - nx), false);
                    face(g.open_v[self.fv(i, j + 1, k)], (j + 1 < ny).then(|| c + nx), false);
                    face(g.open_w[self.fw(i, j, k)], (k > 0).then(|| c - nx * ny), false);
                    face(g.open_w[self.fw(i, j, k + 1)], (k + 1 < nz).then(|| c + nx * ny), true);
                    out[c] = acc * inv;
                }
            }
        }
    }

    /// **S324 : divergence pondérée par les ouvertures**, parties `x` puis `z` comme la 2D, `y`
    /// ensuite ; une maille solide rend zéro.
    fn divergence_cut(&self, g: &cut::Cut3, u: &[f32], v: &[f32], w: &[f32], out: &mut [f32]) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.c(i, j, k);
                    if g.frac[c] == 0. {
                        out[c] = 0.;
                        continue;
                    }
                    let a = |o: f32, x: f32| o * x;
                    let fl = a(g.open_u[self.fu(i, j, k)], u[self.fu(i, j, k)]);
                    let fr = a(g.open_u[self.fu(i + 1, j, k)], u[self.fu(i + 1, j, k)]);
                    let ff = a(g.open_v[self.fv(i, j, k)], v[self.fv(i, j, k)]);
                    let fk = a(g.open_v[self.fv(i, j + 1, k)], v[self.fv(i, j + 1, k)]);
                    let fb = a(g.open_w[self.fw(i, j, k)], w[self.fw(i, j, k)]);
                    let ft = a(g.open_w[self.fw(i, j, k + 1)], w[self.fw(i, j, k + 1)]);
                    out[c] = ((fr - fl + ft - fb) + (fk - ff)) / dx;
                    // S330 : la part des faces que le solide couvre avance à sa vitesse ; S332 : en rotation,
                    // à la vitesse de la paroi au centre de chaque face.
                    if let (Some(b), true) = (&g.base, g.solid_angular != [0.; 3]) {
                        let cov = |o: f32, base: f32| base - o;
                        let pw = |axis: usize, x: [f32; 3]| wall_velocity(g.solid_velocity, g.solid_angular, g.solid_center, axis, x);
                        let (xc, yc, zc) = ((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx);
                        let (x0, x1) = (i as f32 * dx, (i + 1) as f32 * dx);
                        let (y0, y1) = (j as f32 * dx, (j + 1) as f32 * dx);
                        let (z0, z1) = (k as f32 * dx, (k + 1) as f32 * dx);
                        let fx = cov(g.open_u[self.fu(i + 1, j, k)], b.open_u[self.fu(i + 1, j, k)]) * pw(0, [x1, yc, zc])
                            - cov(g.open_u[self.fu(i, j, k)], b.open_u[self.fu(i, j, k)]) * pw(0, [x0, yc, zc]);
                        let fy = cov(g.open_v[self.fv(i, j + 1, k)], b.open_v[self.fv(i, j + 1, k)]) * pw(1, [xc, y1, zc])
                            - cov(g.open_v[self.fv(i, j, k)], b.open_v[self.fv(i, j, k)]) * pw(1, [xc, y0, zc]);
                        let fz = cov(g.open_w[self.fw(i, j, k + 1)], b.open_w[self.fw(i, j, k + 1)]) * pw(2, [xc, yc, z1])
                            - cov(g.open_w[self.fw(i, j, k)], b.open_w[self.fw(i, j, k)]) * pw(2, [xc, yc, z0]);
                        out[c] += (fx + fy + fz) / dx;
                    } else if let (Some(b), [su, sv, sw]) = (&g.base, g.solid_velocity) {
                        if su != 0. || sv != 0. || sw != 0. {
                            let cov = |o: f32, base: f32| base - o;
                            let x = cov(g.open_u[self.fu(i + 1, j, k)], b.open_u[self.fu(i + 1, j, k)])
                                - cov(g.open_u[self.fu(i, j, k)], b.open_u[self.fu(i, j, k)]);
                            let y = cov(g.open_v[self.fv(i, j + 1, k)], b.open_v[self.fv(i, j + 1, k)])
                                - cov(g.open_v[self.fv(i, j, k)], b.open_v[self.fv(i, j, k)]);
                            let z = cov(g.open_w[self.fw(i, j, k + 1)], b.open_w[self.fw(i, j, k + 1)])
                                - cov(g.open_w[self.fw(i, j, k)], b.open_w[self.fw(i, j, k)]);
                            out[c] += (su * x + sv * y + sw * z) / dx;
                        }
                    }
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
        if let Some(g) = &self.cut {
            return self.backward_error_cut(g);
        }
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

    /// S324 : l'erreur inverse sur les lignes pondérées, comme la 2D en mode fixe.
    fn backward_error_cut(&self, g: &cut::Cut3) -> f32 {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let inv = 1. / (dx * dx);
        let mut worst = 0f32;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let c = self.c(i, j, k);
                    if g.frac[c] == 0. {
                        continue;
                    }
                    let pc = self.p[c].abs();
                    let mut acc = 0f32;
                    let mut face = |a: f32, n: Option<usize>, lid: bool| {
                        if a == 0. {
                            return;
                        }
                        match n {
                            Some(m) if g.frac[m] > 0. => acc += a * (pc + self.p[m].abs()),
                            Some(_) => {}
                            None if lid => acc += a * pc * 2.,
                            None => {}
                        }
                    };
                    face(g.open_u[self.fu(i, j, k)], (i > 0).then(|| c - 1), false);
                    face(g.open_u[self.fu(i + 1, j, k)], (i + 1 < nx).then(|| c + 1), false);
                    face(g.open_v[self.fv(i, j, k)], (j > 0).then(|| c - nx), false);
                    face(g.open_v[self.fv(i, j + 1, k)], (j + 1 < ny).then(|| c + nx), false);
                    face(g.open_w[self.fw(i, j, k)], (k > 0).then(|| c - nx * ny), false);
                    face(g.open_w[self.fw(i, j, k + 1)], (k + 1 < nz).then(|| c + nx * ny), true);
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
        if self.cut.is_some() {
            return self.correct_cut(k1);
        }
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

    /// **S324 : la correction sur fond coupé**, comme la 2D : une face fermée n'est pas corrigée, une
    /// face ne l'est qu'entre deux mailles fluides, le couvercle à sa demi-maille.
    fn correct_cut(&mut self, k1: f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        self.u.copy_from_slice(&self.us);
        self.v.copy_from_slice(&self.vs);
        self.w.copy_from_slice(&self.ws);
        let g = self.cut.as_ref().expect("fond coupé");
        for k in 0..nz {
            for j in 0..ny {
                for i in 1..nx {
                    let f = self.fu(i, j, k);
                    let (l, r) = (self.c(i - 1, j, k), self.c(i, j, k));
                    if g.open_u[f] != 0. && g.frac[l] > 0. && g.frac[r] > 0. {
                        self.u[f] -= k1 * (self.p[r] - self.p[l]) / dx;
                    }
                }
            }
        }
        for k in 0..nz {
            for j in 1..ny {
                for i in 0..nx {
                    let f = self.fv(i, j, k);
                    let (a, b) = (self.c(i, j - 1, k), self.c(i, j, k));
                    if g.open_v[f] != 0. && g.frac[a] > 0. && g.frac[b] > 0. {
                        self.v[f] -= k1 * (self.p[b] - self.p[a]) / dx;
                    }
                }
            }
        }
        for k in 1..=nz {
            for j in 0..ny {
                for i in 0..nx {
                    let f = self.fw(i, j, k);
                    let below = self.c(i, j, k - 1);
                    if g.open_w[f] == 0. || g.frac[below] == 0. {
                        continue;
                    }
                    if k < nz {
                        let above = self.c(i, j, k);
                        if g.frac[above] > 0. {
                            self.w[f] -= k1 * (self.p[above] - self.p[below]) / dx;
                        }
                    } else {
                        let lid = self.lid(i, j);
                        self.w[f] -= k1 * (lid - self.p[below]) / (0.5 * dx);
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
                    if let Some(g) = &self.cut {
                        // S324 : pondérée par l'ouverture du couvercle, comme la 2D.
                        if g.frac[c] == 0. {
                            b = 0.;
                        } else if k + 1 == nz {
                            let a = g.open_w[self.fw(i, j, k + 1)];
                            if a > 0. { b += 2. * a * self.lid(i, j) * inv; }
                        }
                    } else if k + 1 == nz { b += 2. * self.lid(i, j) * inv; }
                    self.rhs[c] = b;
                }
            }
        }
        let b2 = self.norm2(&self.rhs, jobs)?;
        self.p.fill(0.);
        self.res.copy_from_slice(&self.rhs);
        let mut rr = b2;
        let mut primed = false;
        // S326 : Jacobi sur le chemin coupé quand `ny > 1` ; ailleurs, le gradient conjugué de S295.
        let jacobi = self.cut.is_some() && self.precondition_cut && ny > 1;
        let mut rz = 0f32;
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
                    if jacobi {
                        rz = self.prime_mobile3(0., jobs)?;
                    } else {
                        self.dir.copy_from_slice(&self.res);
                    }
                    primed = true;
                }
                let mut tmp = core::mem::take(&mut self.tmp);
                self.apply(&self.dir, &mut tmp);
                self.tmp = tmp;
                let dq = self.dot(&self.dir, &self.tmp, jobs);
                if !(dq > 0.) { break; }
                let alpha = if jacobi { rz / dq } else { rr / dq };
                for c in 0..self.p.len() {
                    self.p[c] += alpha * self.dir[c];
                    self.res[c] -= alpha * self.tmp[c];
                }
                let rn = self.norm2(&self.res, jobs)?;
                if jacobi {
                    let zn = self.dot_prec3(jobs)?;
                    let beta = zn / rz;
                    self.prime_mobile3(beta, jobs)?;
                    rz = zn;
                } else {
                    let beta = rn / rr;
                    for c in 0..self.dir.len() {
                        self.dir[c] = self.res[c] + beta * self.dir[c];
                    }
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
            if jacobi {
                rz = self.prime_mobile3(0., jobs)?;
            } else {
                self.dir.copy_from_slice(&self.res);
            }
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
        // S401 : le pas linéaire ne porte pas l'ensemble épars.
        self.refuse_sparse()?;
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
        let retire_avant = self.sponge_removed;
        // S334 : le plancher d'ouverture du couvercle en partie couvert, garde du pas explicite de la hauteur ;
        // S335 : jamais sous `PARTIAL_LID_MIN_APERTURE`.
        self.lid_floor = ((dt * dt * self.g_eff as f64 / dx as f64) as f32).min(1.).max(PARTIAL_LID_MIN_APERTURE);
        let result = self.linear(scale, correction, transport, max_iters, dt, jobs);
        if result.is_ok() {
            // S334 : le flux de paroi a retiré pendant le pas l'eau que la coque avait déposée.
            if let Some(b) = self.cut.as_mut().and_then(|g| g.base.as_mut()) {
                b.deposit.fill(0.);
            }
        }
        if result.is_err() {
            self.u.copy_from_slice(&self.saved_u);
            self.v.copy_from_slice(&self.saved_v);
            self.w.copy_from_slice(&self.saved_w);
            self.p.copy_from_slice(&self.saved_p);
            self.eta.copy_from_slice(&self.saved_eta);
            self.eta_roundoff.copy_from_slice(&self.saved_eta_roundoff);
            self.sponge_removed = retire_avant;
        }
        result
    }

    fn linear(&mut self, scale: f32, correction: f32, transport: f32, max_iters: u32, dt: f64,
        jobs: &dyn JobSystem) -> Result<Report, Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        // S324 : sur fond coupé, le couvercle doit rester entièrement mouillé, comme en 2D ; S332 : une coque
        // qui perce le couvercle en ferme une part, mais le fond seul ne doit jamais l'atteindre.
        if let Some(g) = &self.cut {
            let couvercle = match (&g.base, g.piercing) {
                (Some(b), true) => &b.open_w,
                _ => &g.open_w,
            };
            if (0..nx * ny).any(|c| couvercle[nz * nx * ny + c] != 1.) {
                return Err(Error::Domain);
            }
        }
        // Le modèle est linéaire : le champ prédit est le champ publié, sans terme quadratique.
        self.us.copy_from_slice(&self.u);
        self.vs.copy_from_slice(&self.v);
        self.ws.copy_from_slice(&self.w);
        // S337 : l'éponge amortit les vitesses prédites au bord, comme le pas couplé.
        if let Some(e) = self.linear_sponge {
            let d = self.domain;
            for k in 0..nz {
                for j in 0..ny {
                    for i in 0..=nx {
                        let f = self.fu(i, j, k);
                        self.us[f] *= e.factor(i as f32 * dx, (j as f32 + 0.5) * dx, d, dt);
                    }
                }
                for j in 0..=ny {
                    for i in 0..nx {
                        let f = self.fv(i, j, k);
                        self.vs[f] *= e.factor((i as f32 + 0.5) * dx, j as f32 * dx, d, dt);
                    }
                }
            }
            for k in 0..=nz {
                for j in 0..ny {
                    for i in 0..nx {
                        let f = self.fw(i, j, k);
                        self.ws[f] *= e.factor((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, d, dt);
                    }
                }
            }
        }
        // S386 : la colonne graduée, si elle est réservée (ADR-208).
        let report = if self.graded.is_some() {
            self.project_graded(scale, correction, max_iters)?
        } else {
            self.project(scale, correction, max_iters, jobs)?
        };
        if report.degraded {
            return Err(Error::Convergence);
        }
        // Flux de colonne à chaque face latérale, sommés du fond vers le couvercle comme en 2D.
        for j in 0..ny {
            for i in 0..=nx {
                let mut q = 0f32;
                match &self.cut {
                    // S324 : flux ouvert, `ouverture·u·dx`, dans l'ordre de la 2D.
                    Some(g) => for k in 0..nz { let f = self.fu(i, j, k); q += g.open_u[f] * self.u[f] * dx; },
                    None => for k in 0..nz { q += self.u[self.fu(i, j, k)] * dx; },
                }
                self.flux_x[j * (nx + 1) + i] = q;
            }
        }
        for j in 0..=ny {
            for i in 0..nx {
                let mut q = 0f32;
                match &self.cut {
                    Some(g) => for k in 0..nz { let f = self.fv(i, j, k); q += g.open_v[f] * self.v[f] * dx; },
                    None => for k in 0..nz { q += self.v[self.fv(i, j, k)] * dx; },
                }
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
        // S337 : puis la hauteur revient au repos dans l'éponge ; le volume retiré est compté.
        if let Some(e) = self.linear_sponge {
            self.sponge_removed += self.relax_coupled3(e, dt);
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
#[path = "delta3d_multigrid.rs"]
mod multigrid3;
#[path = "delta3d_regions.rs"]
mod regions;
#[path = "delta3d_advection.rs"]
mod advection;
#[path = "delta3d_graded.rs"]
mod graded;
#[path = "delta3d_sparse.rs"]
mod sparse;
pub use sparse::SparseChange;
