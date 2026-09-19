//! **δ — premier candidat volumétrique.** Noyau à projection sur grille décalée MAC.
//! Protocole, choix et filtres : `docs/validation/CANDIDAT-DELTA-S199.md`.
//!
//! # Ce que ce module est, et ce qu'il n'est pas
//!
//! Il n'est **aucune** des cinq familles listées par ADR-007 §5 : il est ce que trois
//! d'entre elles partagent. FLIP et APIC *sont* des solveurs à projection dont l'advection
//! est portée par des particules ; MPM projette de même ; l'eulérien projette après
//! advection. Le bâtir ne parie donc sur aucune, et **n'élimine aucune**.
//!
//! Il ne traite **aucun** des quatre scénarios de B3 — ni coque mobile, ni impact, ni
//! déferlement, ni référentiel accéléré. S232 corrige la mesure du débit ouvert et reçoit
//! son filtre spatial ; les scénarios de B3 et l'ordre local restent à recevoir.
//!
//! S200 : pas sans allocation et refus numériques atomiques reçus. S231 : pression et
//! opérateur f32, résidu réel contrôlé et corrections bornées. La limite d'itérations n'est pas le budget
//! temporel I-05. S230 ajoute `step_budgeted`, arrêt coopératif atomique ; la garantie murale
//! complète reste non reçue. Voir `docs/validation/BUDGET-DELTA-S230.md`.
//!
//! # L'équilibrage, gagné par construction
//!
//! ADR-030 fait de l'équilibrage un critère d'**entrée** : un candidat mal équilibré échoue
//! C01, et aucun budget de calcul ne le rattrape — le raffinement coûte ×10 500 en 2D.
//!
//! Le défaut classique est de discrétiser `−∇p + ρg` et d'espérer que les deux s'annulent.
//! Ils ne s'annulent qu'à l'ordre du schéma. Ici la pression est portée en deux parts :
//!
//! ```text
//! p = p_hydro + p_dyn        p_hydro(z) = ρ · g_eff · (z₀ − z)
//! ```
//!
//! `p_hydro` est **analytique** et n'est jamais différenciée numériquement ; seul `p_dyn`
//! entre dans le gradient discret, et la gravité n'apparaît **pas** dans la mise à jour de
//! la quantité de mouvement. Au repos, `p_dyn ≡ 0` : l'accélération est exactement nulle,
//! quelle que soit la forme du fond, parce qu'aucune annulation n'est demandée à deux
//! termes discrets. C'est le procédé qu'ADR-114 emploie déjà dans B — hérité, pas réinventé.
//!
//! `g_eff` entre par la **condition de couvercle** : `p_dyn = ρ·g_eff·(η(x) − z₀)` à
//! `z = z₀`, où `η` est l'élévation de surface **fournie** (W peut la publier). Elle est
//! donc portante, et non décorative.
use crate::host::{AllocError, HostServices, JobSystem, MonotonicClock};

#[path = "delta_budget.rs"]
mod budget;
pub use budget::{BudgetReport, Phase};
pub use budget::{STAGES, STAGE_NAMES};
use budget::{Control, Stage};
// S237 : surface géométriquement mobile (fonction hauteur, fluide fantôme). Voir SURFACE-MOBILE-S237.
#[path = "delta_mobile.rs"]
mod mobile;
// S245 : hiérarchie multigrille, préconditionneur du chemin à couvercle fixe (MULTIGRILLE-S245).
#[path = "delta_multigrid.rs"]
mod multigrid;
#[path = "delta_coupling.rs"]
mod coupling;
pub use coupling::{BackgroundFaces, Sponge};
pub use mobile::SURFACE_THETA_MIN;
pub use mobile::{ExternalPressure, PressureCandidate, PressureProblem, PressureRow};

// S238 P3 : trace de mesure du plancher (tests seulement) — à chaque vrai résidu recalculé :
// itérations, résidu relatif, erreur inverse composante par composante.
#[cfg(test)]
thread_local! {
    pub(crate) static PRESSURE_TRACE: core::cell::RefCell<Vec<(u32, f64, f64)>> =
        const { core::cell::RefCell::new(Vec::new()) };
    pub(crate) static PRESSURE_FINGERPRINTS: core::cell::RefCell<Vec<u64>> =
        const { core::cell::RefCell::new(Vec::new()) };
    // S238 P5 : le certificat d'arrondi devance le cycle partout où il a été mesuré ; les tests le
    // retirent pour éprouver la détection de cycle seule.
    pub(crate) static ROUNDOFF_CERTIFICATE_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    // S239 P3 (A273) : trace de la tolerance physique — iterations, residu relatif,
    // concentration du residu max|r|/||r||2, divergence projetee du champ corrige.
    pub(crate) static TOLERANCE_TRACE_ON: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    pub(crate) static TOLERANCE_TRACE: core::cell::RefCell<Vec<(u32, f64, f64, f64)>> =
        const { core::cell::RefCell::new(Vec::new()) };
    // S239 P3 : abaissement du critere premier, pour mesurer combien d'iterations separent la
    // convergence declaree de la tolerance physique. Zero = aucun remplacement.
    pub(crate) static PRESSURE_TOL_OVERRIDE: core::cell::Cell<f32> = const { core::cell::Cell::new(0.) };
    // S245 : le préconditionneur multigrille est **construit et prouvé, mais pas allumé** — mesuré,
    // il ne paie pas encore (MULTIGRILLE-S245 §3). Les essais l'allument pour le mesurer ; le chemin
    // de production reste exactement celui de S244, au bit.
    pub(crate) static MULTIGRID_ON: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// ADR-167 : témoin de Jacobi du mode mobile, pour comparer au cycle multigrille.
    pub(crate) static MOBILE_MULTIGRID_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// ADR-169 : témoin à départ nul des pas mobiles.
    pub(crate) static WARM_PRESSURE_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    pub(crate) static TOLERANCE_TRACE_PLAIN: core::cell::RefCell<Vec<f64>> =
        const { core::cell::RefCell::new(Vec::new()) };
}

/// S239 : la divergence projetée, sur toutes les lignes mouillées et sur les seules lignes
/// **franches** — celles dont aucune face ne porte un fantôme de surface.
#[derive(Clone, Copy, Debug)]
struct Projected { all: f64, plain: f64 }

/// Domaine local, cellules carrées. `z₀ = nz·dx` est le couvercle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Domain {
    pub nx: usize,
    pub nz: usize,
    /// Côté d'une cellule, m.
    pub dx: f32,
}
impl Domain {
    pub fn z0(&self) -> f32 {
        self.nz as f32 * self.dx
    }
    fn cells(&self) -> usize {
        self.nx * self.nz
    }
}

/// `SolverCaps` d'ADR-007 §2, restreint à ce que ce noyau peut honnêtement déclarer.
/// `None` indique une borne non reçue ; aucune plage de stabilité n'est inventée.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Caps {
    /// Coût du dernier `step_measured` réussi ou du dernier pas budgété complet, en ms.
    /// Le pas budgété mesure jusqu'au contrôle de publication, hors retour O(1).
    /// S202 : un domaine = un bloc de banc ; pas de moyenne entre blocs fictifs.
    /// `None` avant mesure, après modification des entrées, pas non mesuré ou refus.
    pub cost_per_block_ms: Option<f32>,
    pub supports_substitutive: bool,
    pub supports_air_phase: bool,
    pub supports_moving_solid: bool,
    pub supports_frame_accel: bool,
    pub min_dx: Option<f32>,
    pub max_dx: Option<f32>,
    pub stability_cfl_max: Option<f32>,
    pub latency_frames: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Domaine vide, ou pas de cellule fluide.
    Domain,
    /// Densité, gravité, pas de temps ou fond non finis ou hors bornes.
    NotFinite,
    /// Longueur de tableau fournie incorrecte.
    Shape,
    /// Expiration coopérative interne ; step_budgeted la traduit en temps restant explicite.
    Budget,
    /// L'horloge injectée a reculé pendant le pas.
    Clock,
    /// Le mode évolutif refuse de transporter la surface avec une pression non convergée.
    Convergence,
    /// Échantillons d'un autre domaine, instant, milieu ou mode de surface.
    BackgroundContext,
    /// Le fond fourni dépend de y ou porte une vitesse transverse : pas une coupe x-z.
    NonPlanar,
}

/// Pas de surface linéarisée : durée exacte, zéro avancée sur expiration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceReport {
    pub advanced_us: u64,
    pub remaining_us: u64,
    pub elapsed_ns: u64,
    pub stopped_at: Option<Phase>,
    pub report: Option<Report>,
}

/// Ce qu'un pas rend à l'appelant, sans qu'il ait à deviner.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Report {
    /// S251 : affinages de divergence sur la vitesse (0 ou 1, pas couplé seulement).
    /// residual/floor/backward_error décrivent alors le dernier système incrémental.
    pub refinements: u32,
    /// Itérations de pression réellement faites.
    pub iterations: u32,
    /// `true` si le solveur n'a pas convergé, notamment au plafond d'itérations.
    pub degraded: bool,
    /// Résidu relatif atteint par le solveur de pression.
    pub residual: f64,
    /// `max |div u|·dx / max|u|` après projection ; `0` si le champ est au repos.
    pub divergence: f64,
    /// S238, ADR-143 : arrêt **au plancher** de f32 — état de pression revenu au bit (cycle certifié)
    /// ou vrai résidu indiscernable de l'arrondi de son calcul (`ω ≤ γ₈`). Aucune itération ne peut
    /// plus progresser ; le pas n'est reçu que si la tolérance de divergence de S199 est tenue.
    pub floor: bool,
    /// S238 : erreur inverse composante par composante `max_i |r_i|/(|b| + |A||p|)_i` du résultat.
    /// Diagnostic (PRESSION-PLANCHER-S238 §2), jamais seuil.
    pub backward_error: f64,
    /// S239, ADR-144 : la même divergence, restreinte aux lignes **franches** — celles dont aucune
    /// face ne porte un fantôme de surface. C'est **elle** qui décide l'acceptation : une ligne à
    /// fantôme est une condition de Dirichlet de raideur `1/θ`, dont le résidu a son propre
    /// plancher f32 (TOLERANCE-PRESSION-S239 §3). Sans fantôme, elle vaut `divergence`.
    pub divergence_plain: f64,
}

/// S238, ADR-143 : tolérance **physique** de la projection, déclarée par S199 §5 (critère 4) avant
/// toute construction : `max|div u|·dx/max|u|`. Elle ne décide qu'à cycle certifié.
pub const PROJECTION_DIVERGENCE_TOLERANCE: f64 = 1e-5;

/// S238, ADR-143 : erreur inverse composante par composante en deçà de laquelle le vrai résidu est
/// indiscernable de l'arrondi de son propre calcul. Modèle standard de la virgule flottante (Higham,
/// *Accuracy and Stability*, §3.1–3.4), `u = 2⁻²⁴` en f32 : une ligne à `m = 4` faces évalue chaque
/// terme `a·(p_c − p_j)` avec `γ₂`, les somme avec `γ_{m−1}`, multiplie par `1/dx²` et soustrait de `b`,
/// soit `γ_{m+3}` ; la représentation f32 de la solution ajoute `u`. `γ_{m+4} = γ₈ = 8u/(1 − 8u)`.
/// Critère d'**arrêt** seulement : jamais d'acceptation sans la tolérance de divergence.
pub const ROUNDOFF_BACKWARD_ERROR: f32 = {
    let u = 1. / 16_777_216.;
    8. * u / (1. - 8. * u)
};

/// Empreinte 64 bits (FNV-1a) d'un champ f32, sur ses bits exacts.
fn fingerprint(values: &[f32], ctl: &mut Control) -> Result<u64, Error> {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for v in values {
        ctl.poll(Phase::Pressure)?;
        for byte in v.to_bits().to_le_bytes() {
            h ^= byte as u64;
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    }
    Ok(h)
}

/// Le candidat. Stockage réservé auprès de l'hôte et Vec construits avant `seal()`.
/// Aucun appel à l'allocateur global dans le pas ; réception S200.
pub struct Volume {
    domain: Domain,
    rho: f32,
    /// **Fournie**, jamais codée en dur. ADR-007 §2 : un `−9,81·Z` en dur disqualifie.
    g_eff: f32,
    /// Hauteur de fond par colonne, m.
    bottom: Vec<f32>,
    /// Élévation de surface imposée au couvercle, m. `η ≡ z₀` est le repos.
    eta: Vec<f32>,
    /// Ouverture des faces verticales, `(nx+1)·nz`.
    open_u: Vec<f32>,
    /// Ouverture des faces horizontales, `nx·(nz+1)`.
    open_w: Vec<f32>,
    /// Fraction fluide de la maille, `nx·nz`. Zéro = solide.
    frac: Vec<f32>,
    u: Vec<f32>,
    w: Vec<f32>,
    us: Vec<f32>,
    ws: Vec<f32>,
    p: Vec<f32>,
    rhs: Vec<f32>,
    res: Vec<f32>,
    dir: Vec<f32>,
    tmp: Vec<f32>,
    saved_u: Vec<f32>,
    saved_w: Vec<f32>,
    saved_p: Vec<f32>,
    /// S251 : pression principale conservée pendant l'affinage de vitesse, ADR-150.
    pressure_base: Vec<f32>,
    homogeneous_lid: bool,
    /// S253, ADR-153 : affinage en mode mobile — valeurs fantômes nulles, `θ` inchangés.
    homogeneous_ghost: bool,
    /// S253, ADR-152 : pas perturbatif mobile en cours — la géométrie lit `surface_total`.
    surface_coupled: bool,
    /// S253 : surface totale `ζ = η' + ζ_fond` du pas couplé, par colonne.
    surface_total: Vec<f32>,
    /// S253 : `ρg·ζ_fond − P_fond(Γ)` du fantôme vertical de chaque colonne.
    ghost_bg_up: Vec<f32>,
    /// S253 : `−P_fond(Γ)` du fantôme latéral porté par chaque face u.
    ghost_bg_side: Vec<f32>,
    saved_eta: Vec<f32>,
    eta_roundoff: Vec<f32>,
    saved_eta_roundoff: Vec<f32>,
    last_cost_ms: Option<f32>,
    /// S291 : temps du dernier pas, par phase publique et par étape fine, en ns. Les deux sommes
    /// valent `elapsed_ns` du rapport. Nuls si le pas a reçu une horloge figée.
    last_phase_ns: [u64; 8],
    last_stage_ns: [u64; STAGES],
    /// S237 : niveau de référence de la pression hydrostatique du mode mobile. `z₀` par défaut.
    rest: f32,
    /// S237 : vrai pendant un pas mobile — opérateur, second membre et correction lisent alors
    /// la surface réelle ; faux, les chemins S199–S233 sont exécutés tels quels.
    mobile: bool,
    /// S245 : niveaux grossiers du préconditionneur multigrille, du plus fin au plus grossier.
    /// Vide quand la grille ne se divise pas ; la géométrie y est figée par `cut()`.
    levels: Vec<multigrid::Level>,
    /// S237 : inverse de la diagonale de l'opérateur mobile (préconditionneur de Jacobi).
    prec: Vec<f32>,
    /// ADR-167 : `z = M⁻¹r` du cycle multigrille mobile ; vide quand la grille ne se divise pas.
    mobile_z: Vec<f32>,
    /// S276, ADR-169 : la projection principale d'un pas mobile part de la pression publiée.
    /// Allumé par le pas pour ce seul appel ; l'affinage part toujours de zéro.
    warm_pressure: bool,
    /// S291 : l'erreur inverse du rapport est un **diagnostic** (S238, « jamais seuil »), et elle
    /// coûte une passe de stencil complète — 0,34 ms sur 4,86 à 6 656 mailles. L'hôte peut la
    /// couper ; `Report.backward_error` vaut alors `NaN`, qui dit « non mesurée » au lieu de
    /// mentir. Le certificat d'arrêt d'ADR-143, lui, n'est **pas** concerné : il reste calculé
    /// dans la boucle, parce qu'il décide.
    report_backward_error: bool,
}

impl Volume {
    #[inline]
    fn c(&self, i: usize, k: usize) -> usize {
        k * self.domain.nx + i
    }
    #[inline]
    fn fu(&self, i: usize, k: usize) -> usize {
        k * (self.domain.nx + 1) + i
    }
    #[inline]
    fn fw(&self, i: usize, k: usize) -> usize {
        k * self.domain.nx + i
    }

    /// Construit le candidat. **À l'initialisation uniquement, avant `seal()`** (I-06).
    /// `g_eff` et `bottom` sont **fournis** : le noyau n'en invente aucun.
    pub fn configure(
        host: &mut HostServices,
        domain: Domain,
        rho: f32,
        g_eff: f32,
        bottom: &[f32],
    ) -> Result<Self, Error> {
        if domain.nx == 0 || domain.nz == 0 || !domain.dx.is_finite() || domain.dx <= 0. {
            return Err(Error::Domain);
        }
        if bottom.len() != domain.nx {
            return Err(Error::Shape);
        }
        if !rho.is_finite()
            || rho <= 0.
            || !g_eff.is_finite()
            || bottom.iter().any(|b| !b.is_finite() || *b < 0.)
        {
            return Err(Error::NotFinite);
        }
        let (nx, nz) = (domain.nx, domain.nz);
        // Compter tous les tampons à leur précision réelle, avant toute allocation.
        let c = nx.checked_mul(nz).ok_or(Error::Domain)?;
        let nu = nx.checked_add(1).and_then(|n| n.checked_mul(nz)).ok_or(Error::Domain)?;
        let nw = nz.checked_add(1).and_then(|n| n.checked_mul(nx)).ok_or(Error::Domain)?;
        // S253 (ADR-152) : une face u de plus (fantôme latéral du fond), deux colonnes de plus
        // (surface totale, fantôme vertical du fond).
        let floats = nu.checked_add(nw).and_then(|n| n.checked_mul(4))
            .and_then(|n| n.checked_add(nu))
            .and_then(|n| n.checked_add(c))
            .and_then(|n| nx.checked_mul(7).and_then(|x| n.checked_add(x)))
            .ok_or(Error::Domain)?;
        // S245 : la hiérarchie multigrille entre dans le même comptage, avant toute allocation.
        let floats = floats
            .checked_add(multigrid::hierarchy_floats(nx, nz))
            .ok_or(Error::Domain)?;
        let bytes = floats.checked_mul(core::mem::size_of::<f32>())
            .and_then(|n| c.checked_mul(8 * core::mem::size_of::<f32>()).and_then(|p| n.checked_add(p)))
            .ok_or(Error::Domain)?;
        if !(domain.z0().is_finite() && (nx as f32 * domain.dx).is_finite()) {
            return Err(Error::Domain);
        }
        // I-06 : l'allocation est demandée à l'hôte, et elle échoue si le système est scellé.
        host.alloc
            .alloc_persistent(bytes)
            .map_err(|e| match e {
                AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
            })?;

        let mut v = Volume {
            domain,
            rho,
            g_eff,
            bottom: bottom.to_vec(),
            eta: vec![domain.z0(); nx],
            open_u: vec![0.; (nx + 1) * nz],
            open_w: vec![0.; nx * (nz + 1)],
            frac: vec![0.; nx * nz],
            u: vec![0.; (nx + 1) * nz],
            w: vec![0.; nx * (nz + 1)],
            us: vec![0.; (nx + 1) * nz],
            ws: vec![0.; nx * (nz + 1)],
            p: vec![0.0f32; nx * nz],
            rhs: vec![0.0f32; nx * nz],
            res: vec![0.0f32; nx * nz],
            dir: vec![0.0f32; nx * nz],
            tmp: vec![0.0f32; nx * nz],
            saved_u: vec![0.; nu],
            saved_w: vec![0.; nw],
            saved_p: vec![0.; c],
            pressure_base: vec![0.; c],
            homogeneous_lid: false,
            homogeneous_ghost: false,
            surface_coupled: false,
            surface_total: vec![0.; nx],
            ghost_bg_up: vec![0.; nx],
            ghost_bg_side: vec![0.; nu],
            saved_eta: vec![domain.z0(); nx],
            eta_roundoff: vec![0.; nx],
            saved_eta_roundoff: vec![0.; nx],
            last_cost_ms: None,
            report_backward_error: true,
            last_phase_ns: [0; 8],
            last_stage_ns: [0; STAGES],
            rest: domain.z0(),
            mobile: false,
            prec: vec![0.0f32; nx * nz],
            levels: Vec::new(),
            // Compté par `hierarchy_floats` dès qu'un niveau existe.
            mobile_z: vec![0.0f32; if multigrid::level_count(nx, nz) > 0 { c } else { 0 }],
            warm_pressure: false,
        };
        v.cut();
        v.seal_isolated();
        // S245 : la géométrie est figée par `cut` ; la hiérarchie s'en déduit une fois pour toutes.
        v.build_hierarchy();
        if v.frac.iter().all(|f| *f == 0.) {
            return Err(Error::Domain);
        }
        Ok(v)
    }

    /// S245 — un cycle en V, puis `dir ← z + β·dir` et le produit `⟨r, z⟩` que le gradient
    /// conjugué préconditionné consomme. Même forme que le chemin Jacobi du mode mobile.
    /// `z = M⁻¹r`, puis `dir ← z + β·dir` avec `β = ⟨r, z⟩ / previous_rz` ; rend `⟨r, z⟩`.
    /// `previous_rz = 0` : départ ou relance, `dir ← z`. S252 (A285) : de S245 à S251, `β` recevait
    /// `‖r‖²/⟨r_n, z_n⟩`, calculé avant le cycle — la direction croissait d'environ `4/dx²` par
    /// itération jusqu'à `⟨d, Ad⟩` non fini, et la boucle s'arrêtait loin de sa récurrence.
    fn multigrid_into_dir(
        &mut self,
        previous_rz: f32,
        jobs: &dyn JobSystem,
        ctl: &mut Control,
    ) -> Result<f32, Error> {
        let res = core::mem::take(&mut self.res);
        let cycle = self.v_cycle(&res, ctl);
        self.res = res;
        cycle?;
        let rz = self.dot(&self.res, &self.prec, jobs, ctl)?;
        let beta = if previous_rz == 0. { 0. } else { rz / previous_rz };
        for c in 0..self.domain.cells() {
            ctl.poll(Phase::Pressure)?;
            if self.frac[c] > 0. {
                self.dir[c] = self.prec[c] + beta * self.dir[c];
            }
        }
        Ok(rz)
    }

    /// S245 — **cycle en V** : rend `z = M⁻¹ r` dans `self.prec`, en se servant de `self.tmp` comme
    /// résidu de la grille fine. Les deux tampons sont libres à cet instant : `prec` ne sert qu'au
    /// mode mobile, et `tmp` a déjà été consommé par le produit qui précède.
    ///
    /// **Pourquoi cette forme et pas une autre.** Le gradient conjugué n'est valide qu'avec un
    /// préconditionneur symétrique. Trois choses le garantissent ici, et aucune n'est décorative :
    /// le lisseur est **diagonal** (donc son propre adjoint), la restriction est la **transposée**
    /// de la prolongation à un facteur constant près, et il y a **autant de lissages après
    /// qu'avant**. Un essai le vérifie numériquement au lieu de le supposer.
    fn v_cycle(&mut self, r: &[f32], ctl: &mut Control) -> Result<(), Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let inv = 1. / (dx * dx);
        let cells = nx * nz;
        ctl.check(Phase::Pressure)?;
        // Grille fine : partir de zéro et pré-lisser.
        for c in 0..cells {
            self.prec[c] = 0.;
        }
        let mut z = core::mem::take(&mut self.prec);
        let mut tmp = core::mem::take(&mut self.tmp);
        for _ in 0..multigrid::PRE_SWEEPS {
            multigrid::apply_level(nx, nz, inv, &self.open_u, &self.open_w, &self.frac, &z, &mut tmp);
            for c in 0..cells {
                let d = self.diag_fine(c, inv);
                if d > 0. {
                    z[c] += multigrid::SMOOTH_DAMPING * (r[c] - tmp[c]) / d;
                }
            }
        }
        if let Some(first) = self.levels.first_mut() {
            // Résidu fin, puis restriction vers le premier niveau grossier.
            multigrid::apply_level(nx, nz, inv, &self.open_u, &self.open_w, &self.frac, &z, &mut tmp);
            for c in 0..cells {
                tmp[c] = r[c] - tmp[c];
            }
            let (cx, cz) = (first.nx, first.nz);
            multigrid::restrict(nx, &tmp, cx, cz, &mut first.r);
            self.coarse_cycle(false, ctl)?;
            let first = &self.levels[0];
            multigrid::prolong_add(nx, &mut z, first.nx, first.nz, &first.x);
        }
        for _ in 0..multigrid::POST_SWEEPS {
            multigrid::apply_level(nx, nz, inv, &self.open_u, &self.open_w, &self.frac, &z, &mut tmp);
            for c in 0..cells {
                let d = self.diag_fine(c, inv);
                if d > 0. {
                    z[c] += multigrid::SMOOTH_DAMPING * (r[c] - tmp[c]) / d;
                }
            }
        }
        self.prec = z;
        self.tmp = tmp;
        ctl.poll(Phase::Pressure)
    }

    /// Descente puis remontée sur les niveaux grossiers. Écrite en deux boucles plutôt qu'en
    /// récursion : les emprunts de deux niveaux voisins y restent lisibles.
    /// ADR-167 : `mobile` choisit les fractions et diagonales mobiles des niveaux.
    fn coarse_cycle(&mut self, mobile: bool, ctl: &mut Control) -> Result<(), Error> {
        let last = self.levels.len() - 1;
        for l in 0..=last {
            ctl.poll(Phase::Pressure)?;
            let sweeps = if l == last { multigrid::COARSE_SWEEPS } else { multigrid::PRE_SWEEPS };
            {
                let level = &mut self.levels[l];
                for c in 0..level.cells() {
                    level.x[c] = 0.;
                }
                for _ in 0..sweeps {
                    let mut t = core::mem::take(&mut level.t);
                    let (frac, diag) = if mobile { (&level.mobile_frac, &level.mobile_diag) } else { (&level.frac, &level.diag) };
                    multigrid::smooth(
                        level.nx, level.nz, level.inv, &level.open_u, &level.open_w,
                        frac, diag, &level.r, &mut level.x, &mut t,
                    );
                    level.t = t;
                }
            }
            if l < last {
                let (head, tail) = self.levels.split_at_mut(l + 1);
                let level = &mut head[l];
                let next = &mut tail[0];
                let mut t = core::mem::take(&mut level.t);
                let frac = if mobile { &level.mobile_frac } else { &level.frac };
                multigrid::apply_level(
                    level.nx, level.nz, level.inv, &level.open_u, &level.open_w,
                    frac, &level.x, &mut t,
                );
                for c in 0..level.cells() {
                    t[c] = level.r[c] - t[c];
                }
                multigrid::restrict(level.nx, &t, next.nx, next.nz, &mut next.r);
                level.t = t;
            }
        }
        for l in (0..last).rev() {
            ctl.poll(Phase::Pressure)?;
            {
                let (head, tail) = self.levels.split_at_mut(l + 1);
                let level = &mut head[l];
                let next = &tail[0];
                multigrid::prolong_add(level.nx, &mut level.x, next.nx, next.nz, &next.x);
            }
            let level = &mut self.levels[l];
            for _ in 0..multigrid::POST_SWEEPS {
                let mut t = core::mem::take(&mut level.t);
                let (frac, diag) = if mobile { (&level.mobile_frac, &level.mobile_diag) } else { (&level.frac, &level.diag) };
                multigrid::smooth(
                    level.nx, level.nz, level.inv, &level.open_u, &level.open_w,
                    frac, diag, &level.r, &mut level.x, &mut t,
                );
                level.t = t;
            }
        }
        Ok(())
    }

    /// ADR-167 — fractions et diagonales mobiles de tous les niveaux, depuis les mailles mouillées
    /// de la géométrie courante. Appelée après `rhs_mobile`, la surface étant figée pour la
    /// projection. Aucune allocation : les tableaux sont ceux de la configuration.
    fn prepare_mobile_levels(&mut self, ctl: &mut Control) -> Result<(), Error> {
        ctl.check(Phase::Pressure)?;
        let mut levels = core::mem::take(&mut self.levels);
        let nx = self.domain.nx;
        for l in 0..levels.len() {
            let (head, tail) = levels.split_at_mut(l);
            let level = &mut tail[0];
            match head.last() {
                None => {
                    let fine = |c: usize| {
                        if self.wet_cell(c) { self.frac[c] } else if self.frac[c] > 0. { -1. } else { 0. }
                    };
                    multigrid::coarsen_mobile(nx, level.nx, level.nz, &fine, &mut level.mobile_frac);
                }
                Some(prev) => {
                    let fine = |c: usize| prev.mobile_frac[c];
                    multigrid::coarsen_mobile(prev.nx, level.nx, level.nz, &fine, &mut level.mobile_frac);
                }
            }
            multigrid::diagonal(
                level.nx, level.nz, level.inv, &level.open_u, &level.open_w,
                &level.mobile_frac, &mut level.mobile_diag,
            );
        }
        self.levels = levels;
        ctl.poll(Phase::Pressure)
    }

    /// ADR-167 — cycle en V du mode mobile : `mobile_z = M⁻¹·res`. Niveau fin : opérateur mobile
    /// exact et Jacobi amorti sur sa diagonale (`prec`, écrit par `rhs_mobile`) ; niveaux grossiers
    /// mobiles ; résultat nul hors des mailles de diagonale positive. Même symétrie qu'ADR-147.
    fn v_cycle_mobile(&mut self, ctl: &mut Control) -> Result<(), Error> {
        let cells = self.domain.cells();
        let r = core::mem::take(&mut self.res);
        let mut z = core::mem::take(&mut self.mobile_z);
        let mut tmp = core::mem::take(&mut self.tmp);
        let result = self.v_cycle_mobile_in(&r, &mut z, &mut tmp, cells, ctl);
        self.res = r;
        self.mobile_z = z;
        self.tmp = tmp;
        result
    }

    fn v_cycle_mobile_in(&mut self, r: &[f32], z: &mut [f32], tmp: &mut [f32], cells: usize,
        ctl: &mut Control) -> Result<(), Error> {
        z.fill(0.);
        for _ in 0..multigrid::PRE_SWEEPS {
            self.apply_mobile(z, tmp, ctl)?;
            for c in 0..cells {
                if self.prec[c] > 0. {
                    z[c] += multigrid::SMOOTH_DAMPING * self.prec[c] * (r[c] - tmp[c]);
                }
            }
        }
        self.apply_mobile(z, tmp, ctl)?;
        for c in 0..cells {
            tmp[c] = if self.prec[c] > 0. { r[c] - tmp[c] } else { 0. };
        }
        let (nx, cx, cz) = (self.domain.nx, self.levels[0].nx, self.levels[0].nz);
        multigrid::restrict(nx, tmp, cx, cz, &mut self.levels[0].r);
        self.coarse_cycle(true, ctl)?;
        let first = &self.levels[0];
        multigrid::prolong_add(nx, z, first.nx, first.nz, &first.x);
        for _ in 0..multigrid::POST_SWEEPS {
            self.apply_mobile(z, tmp, ctl)?;
            for c in 0..cells {
                if self.prec[c] > 0. {
                    z[c] += multigrid::SMOOTH_DAMPING * self.prec[c] * (r[c] - tmp[c]);
                }
            }
        }
        for c in 0..cells {
            if !(self.prec[c] > 0.) {
                z[c] = 0.;
            }
        }
        ctl.poll(Phase::Pressure)
    }

    /// ADR-167 — `dir ← z + β·dir` avec `z` du cycle mobile ; rend `⟨r, z⟩`. Même forme que
    /// `multigrid_into_dir` (A285 : `β` formé après le cycle, sur `⟨r_n, z_n⟩`).
    fn mobile_multigrid_into_dir(&mut self, previous_rz: f32, jobs: &dyn JobSystem,
        ctl: &mut Control) -> Result<f32, Error> {
        self.v_cycle_mobile(ctl)?;
        let rz = self.dot(&self.res, &self.mobile_z, jobs, ctl)?;
        if !rz.is_finite() {
            return Err(Error::NotFinite);
        }
        let beta = if previous_rz == 0. { 0. } else { rz / previous_rz };
        for c in 0..self.domain.cells() {
            ctl.poll(Phase::Pressure)?;
            if self.frac[c] > 0. {
                self.dir[c] = self.mobile_z[c] + beta * self.dir[c];
            }
        }
        Ok(rz)
    }

    /// Diagonale de l'opérateur fin en une maille, dans l'ordre de sommation d'`apply`.
    fn diag_fine(&self, c: usize, inv: f32) -> f32 {
        let (nx, nz) = (self.domain.nx, self.domain.nz);
        if self.frac[c] == 0. {
            return 0.;
        }
        let (i, k) = (c % nx, c / nx);
        let mut acc = 0.0f32;
        let mut face = |af: f32, n: Option<usize>, dirichlet: bool| {
            if af == 0. {
                return;
            }
            match n {
                Some(j) if self.frac[j] > 0. => acc += af,
                Some(_) => {}
                None if dirichlet => acc += 2. * af,
                None => {}
            }
        };
        let left = (i > 0).then(|| c - 1);
        let right = (i + 1 < nx).then(|| c + 1);
        let down = (k > 0).then(|| c - nx);
        let up = (k + 1 < nz).then(|| c + nx);
        face(self.open_u[self.fu(i, k)], left, false);
        face(self.open_u[self.fu(i + 1, k)], right, false);
        face(self.open_w[self.fw(i, k)], down, false);
        face(self.open_w[self.fw(i, k + 1)], up, true);
        acc * inv
    }

    /// S245 — construit les niveaux grossiers depuis la géométrie que `cut` vient de figer. Chaque
    /// niveau est moyenné depuis celui du dessus, et sa diagonale calculée dans la foulée. Rien
    /// n'est alloué ailleurs qu'ici : après `seal`, plus une maille (I-06).
    fn build_hierarchy(&mut self) {
        let (mut nx, mut nz, mut dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let count = multigrid::level_count(nx, nz);
        self.levels.reserve_exact(count);
        for _ in 0..count {
            let (cx, cz) = (nx / 2, nz / 2);
            let cdx = dx * 2.;
            let mut level = multigrid::Level {
                nx: cx,
                nz: cz,
                inv: 1. / (cdx * cdx),
                open_u: vec![0.; (cx + 1) * cz],
                open_w: vec![0.; cx * (cz + 1)],
                frac: vec![0.; cx * cz],
                diag: vec![0.; cx * cz],
                x: vec![0.; cx * cz],
                r: vec![0.; cx * cz],
                t: vec![0.; cx * cz],
                mobile_frac: vec![0.; cx * cz],
                mobile_diag: vec![0.; cx * cz],
            };
            match self.levels.last() {
                Some(prev) => multigrid::coarsen_into(
                    prev.nx, prev.nz, &prev.open_u, &prev.open_w, &prev.frac, &mut level,
                ),
                None => multigrid::coarsen_into(
                    nx, nz, &self.open_u, &self.open_w, &self.frac, &mut level,
                ),
            }
            multigrid::diagonal(
                level.nx, level.nz, level.inv,
                &level.open_u, &level.open_w, &level.frac, &mut level.diag,
            );
            self.levels.push(level);
            (nx, nz, dx) = (cx, cz, cdx);
        }
    }

    /// Hauteur du fond **aux arêtes** de colonnes. Le fond fourni est échantillonné aux
    /// centres ; l'interpoler aux arêtes en fait un profil **linéaire par morceaux** au lieu
    /// d'un escalier, et c'est toute la différence pour l'ordre en espace.
    ///
    /// **S199, filtre 2.** Le premier jet prenait `max(b[i−1], b[i])` pour la face verticale
    /// et un tout-ou-rien pour la face horizontale. Résultat mesuré : ordre **1,95** sur fond
    /// plat — l'intérieur et le couvercle sont d'ordre deux — et **aucune convergence** dès
    /// que le fond n'est plus plat, les incréments changeant de signe. C'est la signature
    /// d'une géométrie en escalier : à chaque raffinement, une face bascule d'ouverte à
    /// fermée et déplace la solution d'une quantité qui ne décroît pas.
    fn edges(&self) -> Vec<f32> {
        let nx = self.domain.nx;
        let mut e = vec![0.; nx + 1];
        e[0] = self.bottom[0];
        e[nx] = self.bottom[nx - 1];
        for i in 1..nx {
            e[i] = 0.5 * (self.bottom[i - 1] + self.bottom[i]);
        }
        e
    }

    /// Découpe du fond : fractions de volume et ouvertures de faces, sur un fond **linéaire
    /// par morceaux**. C'est ici que l'ordre deux se gagne ou se perd au bord.
    fn cut(&mut self) {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let be = self.edges();
        for i in 0..nx {
            let (a, b) = (be[i], be[i + 1]);
            for k in 0..nz {
                let top = (k + 1) as f32 * dx;
                let c = self.c(i, k);
                self.frac[c] = Self::cut_fraction(a, b, top, dx);
            }
        }
        // Face verticale à `x = i·dx` : le fond y vaut `be[i]`, sans maximum ni bascule.
        for i in 0..=nx {
            for k in 0..nz {
                let top = (k + 1) as f32 * dx;
                let idx = self.fu(i, k);
                // Bords latéraux : murs. Le noyau ne prétend pas encore à une frontière
                // ouverte, et un mur est une condition qu'on peut vérifier.
                self.open_u[idx] = if i == 0 || i == nx {
                    0.
                } else {
                    ((top - be[i]) / dx).clamp(0., 1.)
                };
            }
        }
        // Face horizontale à `z = k·dx` : la **fraction** de la largeur où le fond est
        // dessous. Sur un segment linéaire, elle se calcule en forme fermée.
        for i in 0..nx {
            let (a, b) = (be[i], be[i + 1]);
            for k in 0..=nz {
                let z = k as f32 * dx;
                let idx = self.fw(i, k);
                self.open_w[idx] = if k == 0 {
                    0. // fond imperméable
                } else {
                    let open = if a < z && b < z {
                        1.
                    } else if a >= z && b >= z {
                        0.
                    } else {
                        let t = (z - a) / (b - a);
                        if a < z { t } else { 1. - t }
                    };
                    // Couvercle : **frontière à pression imposée**, donc ouverte. C'est par
                    // elle que `η` — et donc `g_eff` — agit ; un couvercle fermé rendrait
                    // la gravité inerte et le contrôle de `g_eff` vide de sens.
                    open.clamp(0., 1.)
                };
            }
        }
    }

    /// Intégrale de clamp((top-b(t))/dx, 0, 1), b linéaire. Rectangle plein puis
    /// trapèze coupé ; aucun échantillonnage ne peut supprimer un triangle étroit.
    fn cut_fraction(a: f32, b: f32, top: f32, dx: f32) -> f32 {
        let (low, high) = (a.min(b), a.max(b));
        if low >= top { return 0.; }
        if high <= top - dx { return 1.; }
        if low == high { return ((top - low) / dx).clamp(0., 1.); }
        let span = high - low;
        let full = ((top - dx - low) / span).clamp(0., 1.);
        let wet = ((top - low) / span).clamp(0., 1.);
        let h0 = ((top - low) / dx).clamp(0., 1.);
        let h1 = ((top - high) / dx).clamp(0., 1.);
        (full + (wet - full) * (0.5 * (h0 + h1))).clamp(0., 1.)
    }

    /// S291 : couper le diagnostic d'erreur inverse du rapport. `true` par défaut : rien ne change
    /// tant que l'hôte ne le demande pas. Ne touche ni ADR-143, ni ADR-144, ni l'acceptation.
    pub fn set_report_backward_error(&mut self, on: bool) { self.report_backward_error = on; }

    /// S291 : carte du coût du dernier pas. `[u64; 8]` dans l'ordre de `Phase`.
    pub fn last_phase_ns(&self) -> [u64; 8] { self.last_phase_ns }
    /// S291 : la même durée, découpée plus finement ; noms dans `STAGE_NAMES`.
    pub fn last_stage_ns(&self) -> [u64; STAGES] { self.last_stage_ns }

    pub fn caps(&self) -> Caps {
        Caps {
            cost_per_block_ms: self.last_cost_ms,
            // Aucun de ces trois-là n'est traité, et le dire est le rôle de ce type.
            supports_substitutive: false,
            supports_air_phase: false,
            supports_moving_solid: false,
            // Une gravité scalaire constante ne reçoit pas un référentiel accéléré général.
            supports_frame_accel: false,
            min_dx: None,
            max_dx: None,
            stability_cfl_max: None,
            latency_frames: 0,
        }
    }

    /// Élévation de surface imposée au couvercle. `η ≡ z₀` est l'état de repos, et c'est le
    /// seul endroit par lequel `g_eff` agit — voir l'en-tête du module.
    pub fn set_surface(&mut self, eta: &[f32]) -> Result<(), Error> {
        if eta.len() != self.domain.nx {
            return Err(Error::Shape);
        }
        if eta.iter().any(|e| !e.is_finite()) {
            return Err(Error::NotFinite);
        }
        self.eta.copy_from_slice(eta);
        self.eta_roundoff.fill(0.);
        self.last_cost_ms = None;
        Ok(())
    }

    /// Pression dynamique imposée au couvercle de la colonne `i`.
    #[inline]
    fn lid(&self, i: usize) -> f32 {
        if self.homogeneous_lid { return 0.; }
        self.rho * self.g_eff * ((self.eta[i] - self.domain.z0()) - self.eta_roundoff[i])
    }

    pub fn velocity_u(&self) -> &[f32] {
        &self.u
    }
    pub fn velocity_w(&self) -> &[f32] {
        &self.w
    }
    pub fn pressure(&self) -> &[f32] {
        &self.p
    }
    /// Hauteur imposée ou évoluée par step_surface_linear ; pas une frontière mobile 3D.
    pub fn surface(&self) -> &[f32] { &self.eta }
    pub fn fluid_fraction(&self) -> &[f32] {
        &self.frac
    }
    pub fn domain(&self) -> Domain {
        self.domain
    }

    /// Injecte un champ de vitesse, pour les réceptions et les solutions manufacturées.
    pub fn set_velocity(&mut self, u: &[f32], w: &[f32]) -> Result<(), Error> {
        if u.len() != self.u.len() || w.len() != self.w.len() {
            return Err(Error::Shape);
        }
        if u.iter().chain(w).any(|v| !v.is_finite()) {
            return Err(Error::NotFinite);
        }
        self.u.copy_from_slice(u);
        self.w.copy_from_slice(w);
        self.last_cost_ms = None;
        Ok(())
    }
}

// ---------------------------------------------------------------- le pas

impl Volume {
    /// Une maille fluide dont **aucune** face n'est ouverte rendrait l'opérateur singulier.
    /// Elle est déclarée solide, et le dire ici vaut mieux qu'un solveur qui diverge.
    fn seal_isolated(&mut self) {
        let (nx, nz) = (self.domain.nx, self.domain.nz);
        for i in 0..nx {
            for k in 0..nz {
                let c = self.c(i, k);
                if self.frac[c] == 0. {
                    continue;
                }
                let a = self.open_u[self.fu(i, k)]
                    + self.open_u[self.fu(i + 1, k)]
                    + self.open_w[self.fw(i, k)]
                    + self.open_w[self.fw(i, k + 1)];
                if a == 0. {
                    self.frac[c] = 0.;
                }
            }
        }
    }

    /// Divergence pondérée par les ouvertures, dans `out`. Les mailles solides rendent zéro.
    fn divergence(&self, u: &[f32], w: &[f32], out: &mut [f32], ctl: &mut Control, phase: Phase) -> Result<(), Error> {
        ctl.check(phase)?;
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        for i in 0..nx {
            for k in 0..nz {
                ctl.poll(phase)?;
                let c = self.c(i, k);
                if self.frac[c] == 0. {
                    out[c] = 0.;
                    continue;
                }
                let a = |o: f32, v: f32| o * v;
                let fl = a(self.open_u[self.fu(i, k)], u[self.fu(i, k)]);
                let fr = a(self.open_u[self.fu(i + 1, k)], u[self.fu(i + 1, k)]);
                let fb = a(self.open_w[self.fw(i, k)], w[self.fw(i, k)]);
                let ft = a(self.open_w[self.fw(i, k + 1)], w[self.fw(i, k + 1)]);
                out[c] = (fr - fl + ft - fb) / dx;
            }
        }
        Ok(())
    }

    /// `L p`, avec `L = −∇·∇` pondéré par les ouvertures et **Dirichlet homogène** au
    /// couvercle. La valeur imposée du couvercle vit dans le second membre, pas ici :
    /// c'est ce qui garde l'opérateur symétrique, donc le gradient conjugué valide.
    fn apply(&self, p: &[f32], out: &mut [f32], ctl: &mut Control) -> Result<(), Error> {
        if self.mobile {
            return self.apply_mobile(p, out, ctl);
        }
        ctl.check(Phase::Pressure)?;
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let inv = 1. / (dx * dx);
        for i in 0..nx {
            for k in 0..nz {
                ctl.poll(Phase::Pressure)?;
                let c = self.c(i, k);
                if self.frac[c] == 0. {
                    out[c] = 0.;
                    continue;
                }
                let mut acc = 0.0f32;
                {
                    let mut face = |af: f32, n: Option<usize>, dirichlet: bool| {
                        if af == 0. {
                            return;
                        }
                        let a = af;
                        match n {
                            Some(j) if self.frac[j] > 0. => acc += a * (p[c] - p[j]),
                            // Face ouverte sur du solide : flux nul, rien à ajouter.
                            Some(_) => {}
                            // Couvercle : demi-maille jusqu'à la valeur imposée.
                            None if dirichlet => acc += 2. * a * p[c],
                            None => {}
                        }
                    };
                    let left = if i > 0 { Some(self.c(i - 1, k)) } else { None };
                    let right = if i + 1 < nx { Some(self.c(i + 1, k)) } else { None };
                    let down = if k > 0 { Some(self.c(i, k - 1)) } else { None };
                    let up = if k + 1 < nz { Some(self.c(i, k + 1)) } else { None };
                    face(self.open_u[self.fu(i, k)], left, false);
                    face(self.open_u[self.fu(i + 1, k)], right, false);
                    face(self.open_w[self.fw(i, k)], down, false);
                    face(self.open_w[self.fw(i, k + 1)], up, true);
                }
                out[c] = acc * inv;
            }
        }
        Ok(())
    }

    /// S238 : erreur inverse composante par composante (Oettli–Prager) du vrai résidu `res`,
    /// `max_i |r_i| / (|b| + |A||p|)_i` sur les lignes du système. Certificat d'arrêt au plancher
    /// (ADR-143), jamais critère d'acceptation. Doit être appelée quand `res = rhs − A·p`.
    fn backward_error(&self, ctl: &mut Control) -> Result<f32, Error> {
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let inv = 1. / (dx * dx);
        let mut worst = 0f32;
        for i in 0..nx {
            for k in 0..nz {
                ctl.poll(Phase::Pressure)?;
                let c = self.c(i, k);
                let row = if self.mobile { self.wet(i, k) } else { self.frac[c] > 0. };
                if !row {
                    continue;
                }
                let pc = self.p[c].abs();
                let mut acc = 0f32;
                let mut face = |a: f32, j: Option<usize>, dirichlet: Option<f32>| {
                    if a == 0. {
                        return;
                    }
                    match j {
                        Some(j) if (if self.mobile { self.wet_cell(j) } else { self.frac[j] > 0. }) => acc += a * (pc + self.p[j].abs()),
                        Some(j) if self.frac[j] == 0. => {}
                        _ => if let Some(it) = dirichlet { acc += a * pc * it },
                    }
                };
                let left = (i > 0).then(|| self.c(i - 1, k));
                let right = (i + 1 < nx).then(|| self.c(i + 1, k));
                let down = (k > 0).then(|| self.c(i, k - 1));
                let up = (k + 1 < nz).then(|| self.c(i, k + 1));
                if self.mobile {
                    face(self.open_u[self.fu(i, k)], left, left.map(|_| self.ghost_side_inv(i, k, i.wrapping_sub(1))));
                    face(self.open_u[self.fu(i + 1, k)], right, right.map(|_| self.ghost_side_inv(i, k, i + 1)));
                    face(self.open_w[self.fw(i, k)], down, None);
                    face(self.open_w[self.fw(i, k + 1)], up, Some(self.ghost_up_inv(i, k)));
                } else {
                    face(self.open_u[self.fu(i, k)], left, None);
                    face(self.open_u[self.fu(i + 1, k)], right, None);
                    face(self.open_w[self.fw(i, k)], down, None);
                    face(self.open_w[self.fw(i, k + 1)], up, if up.is_none() { Some(2.) } else { None });
                }
                let scale = self.rhs[c].abs() + acc * inv;
                if scale > 0. {
                    worst = worst.max(self.res[c].abs() / scale);
                }
            }
        }
        Ok(worst)
    }

    /// Produit scalaire sur les mailles fluides. **I-03** : toute accumulation flottante du
    /// système passe par `parallel_reduce_ordered_f64` (SPEC-004 §8.2), qui fusionne dans
    /// l'ordre des indices et non dans l'ordre d'arrivée.
    fn dot(&self, a: &[f32], b: &[f32], jobs: &dyn JobSystem, ctl: &mut Control) -> Result<f32, Error> {
        ctl.check(Phase::Pressure)?;
        // L'interface historique transporte exactement les f32 en f64. Produits, sommes de
        // groupes ET fusion sont arrondis en f32 ; aucune accumulation double cachée.
        let reduce = |start: usize, end: usize| {
            let mut acc = 0f32;
            for c in start..end { if self.frac[c] > 0. { acc += a[c] * b[c]; } }
            acc as f64
        };
        let merge = |x: f64, y: f64| (x as f32 + y as f32) as f64;
        if !ctl.limited() {
            return Ok(jobs.parallel_reduce_ordered_f64(self.domain.cells(), 64, &reduce, &merge, 0.) as f32);
        }
        let mut acc = 0.;
        for start in (0..self.domain.cells()).step_by(64) {
            ctl.check(Phase::Pressure)?;
            let n = (self.domain.cells() - start).min(64);
            acc = jobs.parallel_reduce_ordered_f64(n, 64, &|s,e| reduce(start+s,start+e), &merge, acc);
        }
        ctl.check(Phase::Pressure)?;
        Ok(acc as f32)
    }

    /// Une norme non représentable ne doit jamais faire passer un second membre non nul
    /// pour le repos. Le domaine numérique extrême est refusé, pas silencieusement annulé.
    fn norm2(&self, a: &[f32], jobs: &dyn JobSystem, ctl: &mut Control) -> Result<f32, Error> {
        let n = self.dot(a, a, jobs, ctl)?;
        if !n.is_finite() { return Err(Error::NotFinite); }
        if n == 0. {
            for (x, fraction) in a.iter().zip(&self.frac) {
                ctl.poll(Phase::Pressure)?;
                if *fraction > 0. && *x != 0. { return Err(Error::NotFinite); }
            }
        }
        Ok(n)
    }

    /// Advection centrée d'ordre deux sur la grille décalée, de `u,w` vers `us,ws`.
    /// Aucune stabilité n'est revendiquée : le schéma est centré, et c'est son **ordre** qui
    /// est en cause dans le filtre 2, pas sa plage de `dt`.
    fn advect(&mut self, dt: f32, ctl: &mut Control) -> Result<(), Error> {
        ctl.check(Phase::Advect)?;
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let h = 0.5 / dx;
        budget::copy(&self.u, &mut self.us, ctl, Phase::Advect)?;
        budget::copy(&self.w, &mut self.ws, ctl, Phase::Advect)?;
        for i in 1..nx {
            for k in 0..nz {
                ctl.poll(Phase::Advect)?;
                let f = self.fu(i, k);
                if self.open_u[f] == 0. {
                    continue;
                }
                let uc = self.u[f];
                let ux = (self.u[self.fu(i + 1, k)] - self.u[self.fu(i - 1, k)]) * h;
                let up = if k + 1 < nz { self.u[self.fu(i, k + 1)] } else { uc };
                let dn = if k > 0 { self.u[self.fu(i, k - 1)] } else { uc };
                let uz = (up - dn) * h;
                // `w` au centre de la face verticale : moyenne des quatre voisins.
                let wc = 0.25
                    * (self.w[self.fw(i - 1, k)]
                        + self.w[self.fw(i, k)]
                        + self.w[self.fw(i - 1, k + 1)]
                        + self.w[self.fw(i, k + 1)]);
                self.us[f] = uc - dt * (uc * ux + wc * uz);
            }
        }
        for i in 0..nx {
            for k in 1..nz {
                ctl.poll(Phase::Advect)?;
                let f = self.fw(i, k);
                if self.open_w[f] == 0. {
                    continue;
                }
                let wc = self.w[f];
                let rt = if i + 1 < nx { self.w[self.fw(i + 1, k)] } else { wc };
                let lf = if i > 0 { self.w[self.fw(i - 1, k)] } else { wc };
                let wx = (rt - lf) * h;
                let wz = (self.w[self.fw(i, k + 1)] - self.w[self.fw(i, k - 1)]) * h;
                let uc = 0.25
                    * (self.u[self.fu(i, k - 1)]
                        + self.u[self.fu(i + 1, k - 1)]
                        + self.u[self.fu(i, k)]
                        + self.u[self.fu(i + 1, k)]);
                self.ws[f] = wc - dt * (uc * wx + wc * wz);
            }
        }
        Ok(())
    }

    /// Projection : résout `L p = −(ρ/dt)·div(u*)` plus le couvercle, puis corrige.
    /// `max_iters` **est** la variable de dégradation exigée par ADR-007 §2.
    fn project(&mut self, scale: f32, k1: f32, max_iters: u32, multigrid: bool, jobs: &dyn JobSystem, ctl: &mut Control) -> Result<Report, Error> {
        self.project_with(scale, k1, max_iters, multigrid, jobs, ctl, None)
    }

    /// S289 : même projection, avec un candidat externe facultatif. `None` reproduit `project`
    /// au bit — le candidat n'entre que par le départ du chemin warm, et aucune porte ne bouge.
    fn project_with(&mut self, scale: f32, k1: f32, max_iters: u32, multigrid: bool, jobs: &dyn JobSystem,
        ctl: &mut Control, external: Option<&mut mobile::ExternalPressure<'_>>) -> Result<Report, Error> {
        ctl.check(Phase::Rhs)?;
        ctl.mark(Stage::Rhs);
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let inv = 1. / (dx * dx);
        let mut rhs = core::mem::take(&mut self.rhs);
        let result = self.divergence(&self.us, &self.ws, &mut rhs, ctl, Phase::Rhs);
        self.rhs = rhs;
        result?;
        // S237 : le mode mobile assemble son second membre et son préconditionneur à part.
        // ADR-167 : il est préconditionné par la multigrille mobile quand la grille se divise ;
        // Jacobi reste son chemin sans niveaux, et le témoin des essais.
        #[cfg(test)]
        let mobile_mg_off = MOBILE_MULTIGRID_OFF.with(|c| c.get());
        #[cfg(not(test))]
        let mobile_mg_off = false;
        let mobile_mg = self.mobile && !self.levels.is_empty() && !mobile_mg_off;
        let jacobi = self.mobile && !mobile_mg;
        // S245 : multigrille sur le chemin à couvercle fixe, quand la grille se divise. Un
        // préconditionneur ne change que les directions de recherche ; l'acceptation reste celle
        // d'ADR-144, appliquée au même vrai résidu recalculé.
        #[cfg(test)]
        let forced = MULTIGRID_ON.with(|c| c.get());
        #[cfg(not(test))]
        let forced = false;
        let multigrid_on = (multigrid || forced) && !self.mobile && !self.levels.is_empty();
        if self.mobile {
            self.rhs_mobile(scale, ctl)?;
            if mobile_mg {
                self.prepare_mobile_levels(ctl)?;
            }
        } else {
        for i in 0..nx {
            for k in 0..nz {
                ctl.poll(Phase::Rhs)?;
                let c = self.c(i, k);
                if self.frac[c] == 0. {
                    self.rhs[c] = 0.;
                    continue;
                }
                let mut b = scale * self.rhs[c];
                // La valeur imposée au couvercle entre ici, et nulle part ailleurs.
                if k + 1 == nz {
                    let a = self.open_w[self.fw(i, k + 1)];
                    if a > 0. {
                        b += 2. * a * self.lid(i) * inv;
                    }
                }
                self.rhs[c] = b;
            }
        }
        }
        // Gradient conjugué, départ `p = 0` : le résidu initial **est** le second membre.
        // S276, ADR-169 : en mode mobile, sur demande du pas, départ depuis la pression publiée,
        // nulle hors des mailles mouillées ; résidu vrai `b − A·p`. Second membre nul : départ nul.
        ctl.check(Phase::Pressure)?;
        ctl.mark(Stage::Warm);
        #[cfg(test)]
        let cold = WARM_PRESSURE_OFF.with(|c| c.get());
        #[cfg(not(test))]
        let cold = false;
        // S291 : `‖b‖²` était réduit **deux fois** sur le même tableau — une fois pour décider du
        // départ chaud, une fois comme `b2`. Une seule réduction, et la garde la réutilise.
        let b2 = self.norm2(&self.rhs, jobs, ctl)?;
        let warm = self.warm_pressure && self.mobile && !cold && b2 > 0.;
        if warm {
            for c in 0..self.domain.cells() {
                ctl.poll(Phase::Pressure)?;
                if !self.wet_cell(c) { self.p[c] = 0.; }
            }
            // S289 : le candidat externe propose ici, et **seulement** ici — sur un départ déjà
            // nettoyé, avant que le vrai résidu ne soit recalculé. Il ne voit ni les portes, ni la
            // correction, ni la publication ; la suite de la fonction est identique.
            if let Some(ext) = external {
                ext.used = false;
                ext.refused = false;
                ctl.mark(Stage::Export);
                self.write_rows(ext.rows, ctl)?;
                // `tmp` est libre jusqu'au `apply` ci-dessous : il garde le départ du cœur, pour
                // qu'un refus le restaure exactement. Aucune allocation.
                budget::copy(&self.p, &mut self.tmp, ctl, Phase::Pressure)?;
                let mut p = core::mem::take(&mut self.p);
                let problem = mobile::PressureProblem { domain: self.domain, rows: ext.rows, rhs: &self.rhs };
                ctl.mark(Stage::Candidate);
                let proposed = ext.candidate.propose(problem, &mut p);
                self.p = p;
                ctl.mark(Stage::Verify);
                let finite = !proposed || {
                    let mut ok = true;
                    for v in &self.p { ctl.poll(Phase::Pressure)?; ok &= v.is_finite(); }
                    ok
                };
                if proposed && finite {
                    for c in 0..self.domain.cells() {
                        ctl.poll(Phase::Pressure)?;
                        if !self.wet_cell(c) { self.p[c] = 0.; }
                    }
                    ext.used = true;
                } else {
                    // Refus **atomique** : le départ du cœur est restauré au bit, le pas continue
                    // exactement comme si aucun candidat n'avait été consulté.
                    let tmp = core::mem::take(&mut self.tmp);
                    let result = budget::copy(&tmp, &mut self.p, ctl, Phase::Pressure);
                    self.tmp = tmp;
                    result?;
                    ext.refused = proposed;
                }
                ctl.mark(Stage::Warm);
            }
            ctl.mark(Stage::Residual);
            let mut tmp = core::mem::take(&mut self.tmp);
            let result = self.apply(&self.p, &mut tmp, ctl);
            self.tmp = tmp;
            result?;
            for c in 0..self.domain.cells() {
                ctl.poll(Phase::Pressure)?;
                self.res[c] = self.rhs[c] - self.tmp[c];
            }
            ctl.mark(Stage::Warm);
        } else {
            for p in &mut self.p { ctl.poll(Phase::Pressure)?; *p = 0.; }
            budget::copy(&self.rhs, &mut self.res, ctl, Phase::Pressure)?;
        }
        let mut rr = if warm { self.norm2(&self.res, jobs, ctl)? } else { b2 };
        // S291 : **l'amorçage de la direction est différé jusqu'à la première itération qui
        // l'emploie.** C'était un cycle multigrille complet appliqué avant la boucle ; sur un pas
        // dont le départ converge déjà — ce que le candidat de S289 produit à presque tous les
        // pas — la boucle ne tournait pas et ce cycle était jeté. Mesuré : 0,83 ms sur 5,79 à
        // 6 656 mailles. Le décalage ne change rien numériquement : quand une itération tourne,
        // elle reçoit exactement la même direction et le même `⟨r, M⁻¹r⟩`, calculés à partir du
        // même `res`, que rien ne touche entre les deux points.
        let mut rz = 0f32;
        let mut primed = false;
        let mut it = 0;
        let tol = 1e-12_f32;
        #[cfg(test)]
        let tol = { let o = PRESSURE_TOL_OVERRIDE.with(|c| c.get()); if o > 0. { o } else { tol } };
        // À convergence récurrente, vérifier b-Ap puis redémarrer depuis le vrai résidu.
        // Le plafond porte sur toutes les itérations, corrections comprises.
        // S233 : une hausse isolée du résidu arrondi n'est pas une preuve de stagnation.
        // Continuer sous le plafond global ; arrêt si CG ne peut faire aucune itération.
        // S238 : un retour **au bit** de `p` à une relance antérieure en est une — la relance est
        // une fonction de `p` seul, la suite est périodique et le plafond ne peut qu'être atteint.
        // Détection de Brent : empreinte de référence renouvelée aux puissances de deux, mémoire
        // constante, toute période détectée (mesurée à 420 relances au cas S237).
        // S238 (mesure, PRESSION-PLANCHER-S238 §3) : un cycle exact peut être très long (7 386
        // itérations au pas 397 du banc S237, plus de 16 000 au pas 404) ; le vrai résidu y est déjà
        // indiscernable de son arrondi. L'erreur inverse est donc aussi un certificat d'arrêt.
        let mut floor_stop = false;
        // S239 (ADR-144) : cible resserree sur ‖r‖² quand le critere premier est atteint sans
        // que la tolerance physique de S199 le soit. `INFINITY` = aucune contrainte de plus.
        let mut physical_target = f32::INFINITY;
        let (mut checkpoint, mut power, mut since): (Option<u64>, u32, u32) = (None, 1, 0);
        // S291 : porte l'état corrigé et sa divergence quand la porte d'ADR-144 vient de les
        // produire pour le `p` courant. Remis à zéro à chaque tour, parce que la boucle interne
        // peut alors déplacer `p`.
        let mut settled: Option<Projected> = None;
        let actual_rr = loop {
            ctl.mark(Stage::Iterate);
            settled = None;
            let before_iterations = it;
            // Au repos, `b` est exactement nul : aucune itération, et la correction est nulle.
            while b2 > 0. && (rr > tol * b2 || rr > physical_target) && it < max_iters {
                if !primed {
                    ctl.mark(Stage::Precondition);
                    rz = self.prime_direction(jacobi, mobile_mg, multigrid_on, b2, jobs, ctl)?;
                    primed = true;
                    ctl.mark(Stage::Iterate);
                }
                let mut tmp = core::mem::take(&mut self.tmp);
                let result = self.apply(&self.dir, &mut tmp, ctl);
                self.tmp = tmp;
                result?;
                let dq = self.dot(&self.dir, &self.tmp, jobs, ctl)?;
                if !(dq > 0.) {
                    break;
                }
                let alpha = rz / dq;
                for c in 0..self.domain.cells() {
                    ctl.poll(Phase::Pressure)?;
                    if self.frac[c] > 0. {
                        self.p[c] += alpha * self.dir[c];
                        self.res[c] -= alpha * self.tmp[c];
                    }
                }
                let rn = self.norm2(&self.res, jobs, ctl)?;
                if jacobi {
                    let zn = self.dot_prec(&self.res, jobs, ctl)?;
                    let beta = zn / rz;
                    self.precondition_into_dir(beta, ctl)?;
                    rz = zn;
                } else if mobile_mg {
                    rz = self.mobile_multigrid_into_dir(rz, jobs, ctl)?;
                } else if multigrid_on {
                    // `β = ⟨r_{n+1}, z_{n+1}⟩ / ⟨r_n, z_n⟩` : le cycle est appliqué d'abord, et
                    // c'est lui qui fournit le produit. S252 (A285) : on lui passe `⟨r_n, z_n⟩`,
                    // et c'est lui qui forme `β` — jamais `‖r_{n+1}‖²`.
                    rz = self.multigrid_into_dir(rz, jobs, ctl)?;
                } else {
                    let beta = rn / rr;
                    for c in 0..self.domain.cells() {
                        ctl.poll(Phase::Pressure)?;
                        if self.frac[c] > 0. {
                            self.dir[c] = self.res[c] + beta * self.dir[c];
                        }
                    }
                    rz = rn;
                }
                rr = rn;
                it += 1;
            }
            ctl.mark(Stage::Gate);
            let mut tmp = core::mem::take(&mut self.tmp);
            let result = self.apply(&self.p, &mut tmp, ctl);
            self.tmp = tmp;
            result?;
            for c in 0..self.domain.cells() {
                ctl.poll(Phase::Pressure)?;
                self.res[c] = self.rhs[c] - self.tmp[c];
            }
            let actual = self.norm2(&self.res, jobs, ctl)?;
            #[cfg(test)]
            {
                let omega = self.backward_error(&mut Control::unlimited()).unwrap_or(f32::NAN) as f64;
                let relative = if b2 > 0. { ((actual / b2) as f64).sqrt() } else { 0. };
                PRESSURE_TRACE.with(|t| t.borrow_mut().push((it, relative, omega)));
            }
            // S239 P3 (A273) : la tolerance physique du champ corrige, a chaque vrai residu.
            #[cfg(test)]
            if TOLERANCE_TRACE_ON.with(|c| c.get()) {
                let theta = if actual > 0. { (self.residual_max() as f64) / (actual as f64).sqrt() } else { 0. };
                let relative = if b2 > 0. { ((actual / b2) as f64).sqrt() } else { 0. };
                self.correct_into_uw(k1, ctl, Phase::Pressure)?;
                let d = self.divergence_metric(ctl, Phase::Pressure)?;
                TOLERANCE_TRACE.with(|t| t.borrow_mut().push((it, relative, theta, d.all)));
                TOLERANCE_TRACE_PLAIN.with(|t| t.borrow_mut().push(d.plain));
            }
            let exhausted = it >= max_iters || it == before_iterations;
            if actual <= tol * b2 {
                if exhausted {
                    break actual;
                }
                // S239, ADR-144 : le critere premier est atteint. C'est la tolerance **physique**
                // declaree avant construction par S199 qui decide de l'acceptation ; tant qu'elle
                // n'est pas tenue, la boucle poursuit. Les cas qui la tiennent deja s'arretent
                // exactement ou ils s'arretaient, et gardent leurs bits.
                self.correct_into_uw(k1, ctl, Phase::Pressure)?;
                // S291 : le résultat est conservé. `p`, `us` et `ws` ne bougent plus si l'on sort
                // ici, et la queue refaisait à l'identique cette correction **et** cette mesure.
                let measured = self.divergence_metric(ctl, Phase::Pressure)?;
                settled = Some(measured);
                let reached = measured.plain;
                if reached <= PROJECTION_DIVERGENCE_TOLERANCE {
                    break actual;
                }
                // La cible se deduit de la mesure, sans facteur choisi : `D` est proportionnelle a
                // `max|r|` par l'identite `div u = r/scale`, donc reduire `D` du rapport voulu
                // demande de reduire `‖r‖₂` du meme rapport a concentration egale. L'acceptation,
                // elle, reste la valeur **exacte** de `D` a la relance suivante.
                let ratio = (PROJECTION_DIVERGENCE_TOLERANCE / reached) as f32;
                physical_target = actual * ratio * ratio;
            } else if exhausted {
                break actual;
            }
            #[cfg(test)]
            let certify = !ROUNDOFF_CERTIFICATE_OFF.with(|c| c.get());
            #[cfg(not(test))]
            let certify = true;
            if certify && self.backward_error(ctl)? <= ROUNDOFF_BACKWARD_ERROR {
                floor_stop = true;
                break actual;
            }
            let state = fingerprint(&self.p, ctl)?;
            #[cfg(test)]
            PRESSURE_FINGERPRINTS.with(|t| t.borrow_mut().push(state));
            if checkpoint == Some(state) {
                floor_stop = true;
                break actual;
            }
            since += 1;
            if since == power {
                checkpoint = Some(state);
                power = power.saturating_mul(2);
                since = 0;
            }
            rr = actual;
            ctl.mark(Stage::Precondition);
            rz = self.prime_direction(jacobi, mobile_mg, multigrid_on, actual, jobs, ctl)?;
            primed = true;
        };
        // Correction : `u = u* − (dt/ρ)·∂p/∂x`, demi-maille au couvercle.
        let residual = if b2 > 0. { (actual_rr / b2).sqrt() } else { 0. };
        let projected = match settled {
            // Déjà faites par la porte d'ADR-144, sur ce même `p` et ces mêmes `us`/`ws`.
            // Les deux phases restent **traversées** : `Phase` est le vocabulaire public des
            // points d'expiration (réception S230), et sauter le calcul ne doit pas retirer un
            // point d'arrêt du pas. Deux lectures d'horloge, aucun parcours de champ.
            Some(measured) => {
                ctl.check(Phase::Correct)?;
                ctl.check(Phase::Diagnostics)?;
                measured
            }
            None => {
                ctl.mark(Stage::Correct);
                self.correct_into_uw(k1, ctl, Phase::Correct)?;
                ctl.mark(Stage::Diagnose);
                self.divergence_metric(ctl, Phase::Diagnostics)?
            }
        };
        let divergence = projected.all;
        // S239, ADR-144 : la tolérance physique de S199 est nécessaire dans **tout** chemin
        // d'acceptation. ADR-143 l'exigeait déjà au plancher ; le critère premier ne l'exigeait pas,
        // et des pas convergés la dépassaient (A273). Un pas qui ne peut pas la tenir est dégradé.
        let tolerance_held = projected.plain <= PROJECTION_DIVERGENCE_TOLERANCE;
        let accepted = (actual_rr <= tol * b2 || floor_stop) && tolerance_held;
        // `res` porte encore le vrai résidu du `p` publié : correction et diagnostic de divergence ne
        // touchent ni `p`, ni `rhs`, ni `res`.
        ctl.mark(Stage::Backward);
        let backward_error = if self.report_backward_error {
            self.backward_error(ctl)? as f64
        } else {
            f64::NAN
        };
        Ok(Report {
            refinements: 0,
            iterations: it,
            degraded: b2 > 0. && !accepted,
            residual: residual as f64,
            divergence,
            floor: floor_stop,
            backward_error,
            divergence_plain: projected.plain,
        })
    }

    /// S291 : `dir ← M⁻¹·res` et le produit `⟨r, M⁻¹r⟩` qui va avec, quel que soit le
    /// préconditionneur. Une seule écriture de ce choix, employée à l'amorçage **et** à chaque
    /// relance ; `fallback` est la valeur que rend le chemin sans préconditionneur.
    fn prime_direction(&mut self, jacobi: bool, mobile_mg: bool, multigrid_on: bool, fallback: f32,
        jobs: &dyn JobSystem, ctl: &mut Control) -> Result<f32, Error> {
        if jacobi {
            self.precondition_into_dir(0., ctl)?;
            self.dot_prec(&self.res, jobs, ctl)
        } else if mobile_mg {
            self.mobile_multigrid_into_dir(0., jobs, ctl)
        } else if multigrid_on {
            self.multigrid_into_dir(0., jobs, ctl)
        } else {
            budget::copy(&self.res, &mut self.dir, ctl, Phase::Pressure)?;
            Ok(fallback)
        }
    }

    /// Correction du champ prédit **dans les tampons de travail** `u`/`w` (mode fixe ou mobile).
    /// Ces deux tampons sont libres pendant `project` : `run` et les pas de surface ont mis les
    /// champs publiés à l'abri dans `saved_*` avant l'appel, et c'est eux que le contrat `Err`
    /// restaure. S239 : source unique de la correction, appelée par la queue de `project` et par
    /// le calcul de la tolérance physique dans la boucle.
    fn correct_into_uw(&mut self, k1: f32, ctl: &mut Control, phase: Phase) -> Result<(), Error> {
        ctl.check(phase)?;
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        budget::copy(&self.us, &mut self.u, ctl, phase)?;
        budget::copy(&self.ws, &mut self.w, ctl, phase)?;
        if self.mobile {
            return self.correct_mobile(k1, ctl);
        }
        for i in 1..nx {
            for k in 0..nz {
                ctl.poll(phase)?;
                let f = self.fu(i, k);
                if self.open_u[f] == 0. {
                    continue;
                }
                let (l, r) = (self.c(i - 1, k), self.c(i, k));
                if self.frac[l] > 0. && self.frac[r] > 0. {
                    self.u[f] -= (k1 * (self.p[r] - self.p[l]) / dx) as f32;
                }
            }
        }
        for i in 0..nx {
            for k in 1..=nz {
                ctl.poll(phase)?;
                let f = self.fw(i, k);
                if self.open_w[f] == 0. {
                    continue;
                }
                let below = self.c(i, k - 1);
                if self.frac[below] == 0. {
                    continue;
                }
                if k < nz {
                    let above = self.c(i, k);
                    if self.frac[above] > 0. {
                        self.w[f] -= (k1 * (self.p[above] - self.p[below]) / dx) as f32;
                    }
                } else {
                    // Couvercle : la pression imposée est à une demi-maille.
                    self.w[f] -= (k1 * (self.lid(i) - self.p[below]) / (0.5 * dx)) as f32;
                }
            }
        }
        Ok(())
    }

    /// `D = max|div u|·dx / max|u|` sur le champ corrigé présent dans `u`/`w` : la tolérance
    /// **physique** déclarée avant construction par S199 §5 critère 4. Consomme `tmp`, libre dès
    /// que le vrai résidu en a été tiré. Source unique : rapport de fin et boucle l'appellent.
    fn divergence_metric(&mut self, ctl: &mut Control, phase: Phase) -> Result<Projected, Error> {
        let mut tmp = core::mem::take(&mut self.tmp);
        let result = self.divergence(&self.u, &self.w, &mut tmp, ctl, phase);
        self.tmp = tmp;
        result?;
        let (mobile, nx) = (self.mobile, self.domain.nx);
        let (mut dmax, mut plain_max) = (0f32, 0f32);
        for (c, v) in self.tmp.iter().enumerate() {
            ctl.poll(phase)?;
            // S237 : les mailles d'air portent des vitesses extrapolées, pas une contrainte.
            if mobile && !self.wet_cell(c) { continue; }
            dmax = dmax.max(v.abs());
            // S239 : une ligne à face fantôme est une condition de Dirichlet de raideur `1/θ`, pas
            // une conservation ; son résidu a son propre plancher f32. Mesurées à part.
            if !mobile || !self.has_ghost_face(c % nx, c / nx) { plain_max = plain_max.max(v.abs()); }
        }
        let mut umax = 0f32;
        for v in self.u.iter().chain(&self.w) { ctl.poll(phase)?; umax = umax.max(v.abs()); }
        let ratio = if umax > 0. { self.domain.dx / umax } else { 0. };
        Ok(Projected { all: (dmax * ratio) as f64, plain: (plain_max * ratio) as f64 })
    }

    /// S239 : la maille mouillée touche-t-elle un fantôme de surface (voisin non mouillé qui n'est
    /// pas du solide) ? Sa ligne porte alors un coefficient en `1/θ`, borné par `SURFACE_THETA_MIN`.
    fn has_ghost_face(&self, i: usize, k: usize) -> bool {
        let (nx, nz) = (self.domain.nx, self.domain.nz);
        let ghost = |x: usize, z: usize, face: f32| {
            face > 0. && !self.wet(x, z) && self.frac[self.c(x, z)] > 0.
        };
        (i > 0 && ghost(i - 1, k, self.open_u[self.fu(i, k)]))
            || (i + 1 < nx && ghost(i + 1, k, self.open_u[self.fu(i + 1, k)]))
            || (k + 1 < nz && ghost(i, k + 1, self.open_w[self.fw(i, k + 1)]))
            || (k + 1 == nz && self.open_w[self.fw(i, k + 1)] > 0.)
    }

    /// S239 P3 : norme maximale du vrai résidu sur les **lignes du système**, pour la
    /// concentration `θ = max|r| / ‖r‖₂` de TOLERANCE-PRESSION-S239 §1.2.
    #[cfg(test)]
    fn residual_max(&self) -> f32 {
        let (nx, nz) = (self.domain.nx, self.domain.nz);
        let mut worst = 0f32;
        for i in 0..nx {
            for k in 0..nz {
                let c = self.c(i, k);
                let row = if self.mobile { self.wet(i, k) } else { self.frac[c] > 0. };
                if row { worst = worst.max(self.res[c].abs()); }
            }
        }
        worst
    }

    /// Un pas à plafond d'itérations, sans allocation. Un `Err` numérique conserve
    /// u/w/p ; les tampons internes sont recalculés lors du prochain appel.
    /// `max_iters` n'est PAS un budget en millisecondes (I-05 reste non reçu).
    /// À zéro, advection et diagnostics restent exécutés ; `degraded` annonce
    /// la non-convergence, sans garantie sur le coût de ces phases.
    pub fn step(&mut self, dt: f32, max_iters: u32, jobs: &dyn JobSystem) -> Result<Report, Error> {
        self.run(dt, max_iters, jobs, &mut Control::unlimited())
    }

    /// ADR-150 : une correction à l'échelle du défaut de vitesse, sans reconstruire
    /// celle-ci depuis la pression totale arrondie. Le couvercle de q est homogène.
    /// S252, ADR-151 : partagée par le pas couplé (`multigrid = false`, inchangé au bit) et par
    /// le pas à couvercle fixe (`run`), qui la résout avec le préconditionneur du repli refusé.
    fn refine_divergence(&mut self, scale: f32, correction: f32, max_iters: u32, multigrid: bool,
        jobs: &dyn JobSystem, ctl: &mut Control) -> Result<Report, Error> {
        budget::copy(&self.p, &mut self.pressure_base, ctl, Phase::Prepare)?;
        budget::copy(&self.u, &mut self.us, ctl, Phase::Prepare)?;
        budget::copy(&self.w, &mut self.ws, ctl, Phase::Prepare)?;
        // S253, ADR-153 : en mode mobile, les valeurs fantômes jouent le rôle du couvercle.
        self.homogeneous_lid = true;
        self.homogeneous_ghost = true;
        let result = self.project(scale, correction, max_iters, multigrid, jobs, ctl);
        self.homogeneous_lid = false; // y compris Err(Budget/Clock), avant toute propagation
        self.homogeneous_ghost = false;
        let mut report = result?;
        for (p, base) in self.p.iter_mut().zip(&self.pressure_base) {
            ctl.poll(Phase::Correct)?;
            *p += base;
        }
        report.refinements = 1;
        Ok(report)
    }

    fn swap_state(&mut self) {
        core::mem::swap(&mut self.u, &mut self.saved_u);
        core::mem::swap(&mut self.w, &mut self.saved_w);
        core::mem::swap(&mut self.p, &mut self.saved_p);
    }

    fn run(&mut self, dt: f32, max_iters: u32, jobs: &dyn JobSystem, ctl: &mut Control) -> Result<Report, Error> {
        self.last_cost_ms = None;
        if !dt.is_finite() || dt <= 0. { return Err(Error::NotFinite); }
        ctl.check(Phase::Prepare)?;
        // Avant échange, les champs publiés restent intacts même si une copie est interrompue.
        budget::copy(&self.u, &mut self.saved_u, ctl, Phase::Prepare)?;
        budget::copy(&self.w, &mut self.saved_w, ctl, Phase::Prepare)?;
        budget::copy(&self.p, &mut self.saved_p, ctl, Phase::Prepare)?;
        self.swap_state();
        let result = (|| {
            self.advect(dt, ctl)?;
            // S245 : chemin ordinaire d'abord. **S'il est refusé**, et seulement alors, le même
            // pas est rejoué avec le préconditionneur multigrille : à 32 768 mailles il fait passer
            // la divergence de 1,34·10⁻⁵ — refusée par ADR-144 — à 8,5·10⁻⁶, parce qu'il converge
            // en 134 itérations au lieu de 425 et accumule donc moins d'arrondi. Il coûte trois
            // fois plus cher : on ne le paie que là où l'autre échoue.
            let mut r = self.project(-self.rho / dt, dt / self.rho, max_iters, false, jobs, ctl)?;
            let mut iterations = r.iterations;
            let mut multigrid = false;
            if r.degraded && !self.mobile && !self.levels.is_empty() {
                r = self.project(-self.rho / dt, dt / self.rho, max_iters, true, jobs, ctl)?;
                iterations = iterations.saturating_add(r.iterations);
                multigrid = true;
            }
            // S252, ADR-151 : refusé **au plancher** après le repli, le pas reçoit l'affinage de
            // divergence d'ADR-150 — une fois. Sans lui, corriger le β multigrille (A285) rouvrait
            // A275 à 32 768 mailles. Les pas déjà reçus n'y passent pas et gardent leurs bits.
            if r.degraded && r.floor && !self.mobile {
                r = self.refine_divergence(-self.rho / dt, dt / self.rho, max_iters, multigrid, jobs, ctl)?;
                iterations = iterations.saturating_add(r.iterations);
            }
            // S252 : `iterations` compte le travail de toutes les projections du pas, comme le pas
            // couplé depuis S251 ; il ne rendait que celui de la dernière.
            r.iterations = iterations;
            let r = r;
            ctl.check(Phase::Validate)?;
            for v in self.u.iter().chain(&self.w) {
                ctl.poll(Phase::Validate)?;
                if !v.is_finite() { return Err(Error::NotFinite); }
            }
            for v in self.p.iter().chain(&self.rhs).chain(&self.res).chain(&self.dir).chain(&self.tmp) {
                ctl.poll(Phase::Validate)?;
                if !v.is_finite() { return Err(Error::NotFinite); }
            }
            if !r.residual.is_finite() || !r.divergence.is_finite() { return Err(Error::NotFinite); }
            ctl.check(Phase::Publish)?;
            Ok(r)
        })();
        if result.is_err() { self.swap_state(); }
        result
    }

    /// Budget coopératif en ms, contrôlé par tranches de 64 éléments au plus. L'expiration
    /// conserve u/w/p et annonce dt entier restant. Le prochain appel recommence le pas.
    /// L'hôte doit borner horloge/réductions ; aucune préemption ni borne de retard OS implicite.
    /// Le coût d'un abandon n'est jamais publié comme coût d'un bloc avancé dans Caps.
    pub fn step_budgeted(&mut self, dt: f32, max_iters: u32, budget_ms: f32,
        jobs: &dyn JobSystem, clock: &dyn MonotonicClock) -> Result<BudgetReport, Error> {
        self.last_cost_ms = None;
        let mut ctl = Control::new(clock, budget_ms)?;
        match self.run(dt, max_iters, jobs, &mut ctl) {
            Ok(report) => {
                let elapsed_ns = ctl.elapsed();
                self.last_cost_ms = (elapsed_ns > 0).then_some(elapsed_ns as f32 / 1_000_000.);
                Ok(BudgetReport { advanced_dt: dt, remaining_dt: 0., elapsed_ns,
                    stopped_at: None, report: Some(report) })
            }
            Err(Error::Budget) => Ok(BudgetReport { advanced_dt: 0., remaining_dt: dt,
                elapsed_ns: ctl.elapsed(), stopped_at: Some(ctl.phase), report: None }),
            Err(e) => Err(e),
        }
    }

    /// Surface linéarisée sur géométrie fixe (ADR-141). La durée et le budget entrent
    /// en microsecondes entières. Aucune advection quadratique ; ni déferlement ni air.
    /// Une pression non convergée rend Convergence et conserve u/w/p/eta.
    pub fn step_surface_linear(&mut self, duration_us: u64, max_iters: u32, budget_us: u64,
        jobs: &dyn JobSystem, clock: &dyn MonotonicClock) -> Result<SurfaceReport, Error> {
        self.last_cost_ms = None;
        let limit = budget_us.checked_mul(1000).ok_or(Error::NotFinite)?;
        // Entier exactement représentable en f64 ; aucun temps n'est arrondi en f32.
        if duration_us == 0 || duration_us > (1u64 << 53) { return Err(Error::NotFinite); }
        let dt = duration_us as f64 * 1e-6;
        let dx = self.domain.dx;
        if self.g_eff <= 0. || dt*dt*self.g_eff as f64/dx as f64 > 1. { return Err(Error::Domain); }
        // ADR-141 : seuls les coefficients dimensionnés atteignent les champs f32.
        let scale = (-self.rho as f64/dt) as f32;
        let correction = (dt/self.rho as f64) as f32;
        let transport = (dt/dx as f64) as f32;
        if !scale.is_finite() || !correction.is_finite() || correction == 0.
            || !transport.is_finite() || transport == 0. { return Err(Error::NotFinite); }
        let mut ctl = Control::from_ns(clock, limit);
        let mut swapped = false;
        let result = (|| {
            ctl.check(Phase::Prepare)?;
            for i in 0..self.domain.nx {
                ctl.poll(Phase::Prepare)?;
                if self.open_w[self.fw(i,self.domain.nz)] != 1. { return Err(Error::Domain); }
            }
            budget::copy(&self.u,&mut self.saved_u,&mut ctl,Phase::Prepare)?;
            budget::copy(&self.w,&mut self.saved_w,&mut ctl,Phase::Prepare)?;
            budget::copy(&self.p,&mut self.saved_p,&mut ctl,Phase::Prepare)?;
            budget::copy(&self.eta,&mut self.saved_eta,&mut ctl,Phase::Prepare)?;
            budget::copy(&self.eta_roundoff,&mut self.saved_eta_roundoff,&mut ctl,Phase::Prepare)?;
            self.swap_state();
            core::mem::swap(&mut self.eta,&mut self.saved_eta);
            core::mem::swap(&mut self.eta_roundoff,&mut self.saved_eta_roundoff);
            swapped = true;
            // Le modèle est entièrement linéaire : ne pas garder seulement une partie
            // des termes quadratiques en réutilisant l'advection du mode imposé.
            budget::copy(&self.u,&mut self.us,&mut ctl,Phase::Advect)?;
            budget::copy(&self.w,&mut self.ws,&mut ctl,Phase::Advect)?;
            let report = self.project(scale,correction,max_iters,false,jobs,&mut ctl)?;
            if report.degraded { return Err(Error::Convergence); }
            ctl.check(Phase::Correct)?;
            let mut left = 0f32; // mur latéral ; même flux partagé par les deux colonnes.
            for i in 0..self.domain.nx {
                let mut right = 0f32;
                for k in 0..self.domain.nz {
                    ctl.poll(Phase::Correct)?;
                    let face = self.fu(i+1,k);
                    right += self.open_u[face]*self.u[face]*dx;
                }
                // Somme compensée f32 : un déplacement plus petit que l'ulp de z0
                // survit au pas suivant et participe également à la pression.
                let increment = -transport*(right-left)-self.eta_roundoff[i];
                let height = self.eta[i]+increment;
                self.eta_roundoff[i] = (height-self.eta[i])-increment;
                self.eta[i] = height;
                left = right;
            }
            ctl.check(Phase::Validate)?;
            for value in self.u.iter().chain(&self.w).chain(&self.p).chain(&self.eta)
                .chain(&self.eta_roundoff)
                .chain(&self.rhs).chain(&self.res).chain(&self.dir).chain(&self.tmp) {
                ctl.poll(Phase::Validate)?;
                if !value.is_finite() { return Err(Error::NotFinite); }
            }
            if !report.residual.is_finite() || !report.divergence.is_finite() { return Err(Error::NotFinite); }
            ctl.check(Phase::Publish)?;
            Ok(report)
        })();
        if result.is_err() && swapped {
            self.swap_state();
            core::mem::swap(&mut self.eta,&mut self.saved_eta);
            core::mem::swap(&mut self.eta_roundoff,&mut self.saved_eta_roundoff);
        }
        match result {
            Ok(report) => {
                let elapsed_ns = ctl.elapsed();
                self.last_cost_ms = (elapsed_ns>0).then_some(elapsed_ns as f32/1_000_000.);
                Ok(SurfaceReport {advanced_us:duration_us,remaining_us:0,elapsed_ns,
                    stopped_at:None,report:Some(report)})
            }
            Err(Error::Budget) => Ok(SurfaceReport {advanced_us:0,remaining_us:duration_us,
                elapsed_ns:ctl.elapsed(),stopped_at:Some(ctl.phase),report:None}),
            Err(error) => Err(error),
        }
    }

    /// Pas complet chronométré : sauvegarde, advection, pression, diagnostic et
    /// contrôle numérique inclus. Configuration, dessin et I/O sont hors fenêtre.
    /// Une horloge égale/reculant invalide la mesure, pas le résultat physique.
    /// Coût observé uniquement : aucune garantie de respecter un budget futur.
    pub fn step_measured(&mut self, dt: f32, max_iters: u32, jobs: &dyn JobSystem,
        clock: &dyn MonotonicClock) -> Result<Report, Error> {
        let start=clock.now_ns();
        let result=self.step(dt,max_iters,jobs);
        let end=clock.now_ns();
        if result.is_ok() {
            self.last_cost_ms=end.checked_sub(start).filter(|n|*n>0).map(|n|n as f32/1_000_000.);
        }
        result
    }
}

#[cfg(test)]
#[path = "tests_delta_projection.rs"]
mod tests;
