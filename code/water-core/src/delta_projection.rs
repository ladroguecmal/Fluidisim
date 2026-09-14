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
use budget::Control;

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
}

/// Ce qu'un pas rend à l'appelant, sans qu'il ait à deviner.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Report {
    /// Itérations de pression réellement faites.
    pub iterations: u32,
    /// `true` si le solveur n'a pas convergé, notamment au plafond d'itérations.
    pub degraded: bool,
    /// Résidu relatif atteint par le solveur de pression.
    pub residual: f64,
    /// `max |div u|·dx / max|u|` après projection ; `0` si le champ est au repos.
    pub divergence: f64,
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
    last_cost_ms: Option<f32>,
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
        let floats = nu.checked_add(nw).and_then(|n| n.checked_mul(4))
            .and_then(|n| n.checked_add(c))
            .and_then(|n| nx.checked_mul(2).and_then(|x| n.checked_add(x)))
            .ok_or(Error::Domain)?;
        let bytes = floats.checked_mul(core::mem::size_of::<f32>())
            .and_then(|n| c.checked_mul(6 * core::mem::size_of::<f32>()).and_then(|p| n.checked_add(p)))
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
            last_cost_ms: None,
        };
        v.cut();
        v.seal_isolated();
        if v.frac.iter().all(|f| *f == 0.) {
            return Err(Error::Domain);
        }
        Ok(v)
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
        self.last_cost_ms = None;
        Ok(())
    }

    /// Pression dynamique imposée au couvercle de la colonne `i`.
    #[inline]
    fn lid(&self, i: usize) -> f32 {
        self.rho * self.g_eff * (self.eta[i] - self.domain.z0())
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
            ctl.poll(Phase::Advect)?;
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
    fn project(&mut self, dt: f32, max_iters: u32, jobs: &dyn JobSystem, ctl: &mut Control) -> Result<Report, Error> {
        ctl.check(Phase::Rhs)?;
        let (nx, nz, dx) = (self.domain.nx, self.domain.nz, self.domain.dx);
        let inv = 1. / (dx * dx);
        let mut rhs = core::mem::take(&mut self.rhs);
        let result = self.divergence(&self.us, &self.ws, &mut rhs, ctl, Phase::Rhs);
        self.rhs = rhs;
        result?;
        let scale = -self.rho / dt;
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
        // Gradient conjugué, départ `p = 0` : le résidu initial **est** le second membre.
        ctl.check(Phase::Pressure)?;
        for p in &mut self.p { ctl.poll(Phase::Pressure)?; *p = 0.; }
        budget::copy(&self.rhs, &mut self.res, ctl, Phase::Pressure)?;
        budget::copy(&self.res, &mut self.dir, ctl, Phase::Pressure)?;
        let b2 = self.norm2(&self.rhs, jobs, ctl)?;
        let mut rr = b2;
        let mut it = 0;
        let tol = 1e-12_f32;
        // À convergence récurrente, vérifier b-Ap puis redémarrer depuis le vrai résidu.
        // Le plafond porte sur toutes les itérations, corrections comprises.
        let mut previous_actual = f32::INFINITY;
        let actual_rr = loop {
            let before_iterations = it;
            // Au repos, `b` est exactement nul : aucune itération, et la correction est nulle.
            while b2 > 0. && rr > tol * b2 && it < max_iters {
                let mut tmp = core::mem::take(&mut self.tmp);
                let result = self.apply(&self.dir, &mut tmp, ctl);
                self.tmp = tmp;
                result?;
                let dq = self.dot(&self.dir, &self.tmp, jobs, ctl)?;
                if !(dq > 0.) {
                    break;
                }
                let alpha = rr / dq;
                for c in 0..self.domain.cells() {
                    ctl.poll(Phase::Pressure)?;
                    if self.frac[c] > 0. {
                        self.p[c] += alpha * self.dir[c];
                        self.res[c] -= alpha * self.tmp[c];
                    }
                }
                let rn = self.norm2(&self.res, jobs, ctl)?;
                let beta = rn / rr;
                for c in 0..self.domain.cells() {
                    ctl.poll(Phase::Pressure)?;
                    if self.frac[c] > 0. {
                        self.dir[c] = self.res[c] + beta * self.dir[c];
                    }
                }
                rr = rn;
                it += 1;
            }
            let mut tmp = core::mem::take(&mut self.tmp);
            let result = self.apply(&self.p, &mut tmp, ctl);
            self.tmp = tmp;
            result?;
            for c in 0..self.domain.cells() {
                ctl.poll(Phase::Pressure)?;
                self.res[c] = self.rhs[c] - self.tmp[c];
            }
            let actual = self.norm2(&self.res, jobs, ctl)?;
            if actual <= tol * b2 || it >= max_iters || it == before_iterations || actual >= previous_actual {
                break actual;
            }
            previous_actual = actual;
            rr = actual;
            budget::copy(&self.res, &mut self.dir, ctl, Phase::Pressure)?;
        };
        // Correction : `u = u* − (dt/ρ)·∂p/∂x`, demi-maille au couvercle.
        ctl.check(Phase::Correct)?;
        let k1 = (dt / self.rho) as f32;
        budget::copy(&self.us, &mut self.u, ctl, Phase::Correct)?;
        budget::copy(&self.ws, &mut self.w, ctl, Phase::Correct)?;
        for i in 1..nx {
            for k in 0..nz {
                ctl.poll(Phase::Correct)?;
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
                ctl.poll(Phase::Correct)?;
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
        let residual = if b2 > 0. { (actual_rr / b2).sqrt() } else { 0. };
        let mut tmp = core::mem::take(&mut self.tmp);
        let result = self.divergence(&self.u, &self.w, &mut tmp, ctl, Phase::Diagnostics);
        self.tmp = tmp;
        result?;
        let mut dmax = 0f32;
        for v in &self.tmp { ctl.poll(Phase::Diagnostics)?; dmax = dmax.max(v.abs()); }
        let mut umax = 0f32;
        for v in self.u.iter().chain(&self.w) { ctl.poll(Phase::Diagnostics)?; umax = umax.max((*v as f32).abs()); }
        Ok(Report {
            iterations: it,
            degraded: b2 > 0. && actual_rr > tol * b2,
            residual: residual as f64,
            divergence: (if umax > 0. { dmax * dx / umax } else { 0. }) as f64,
        })
    }

    /// Un pas à plafond d'itérations, sans allocation. Un `Err` numérique conserve
    /// u/w/p ; les tampons internes sont recalculés lors du prochain appel.
    /// `max_iters` n'est PAS un budget en millisecondes (I-05 reste non reçu).
    /// À zéro, advection et diagnostics restent exécutés ; `degraded` annonce
    /// la non-convergence, sans garantie sur le coût de ces phases.
    pub fn step(&mut self, dt: f32, max_iters: u32, jobs: &dyn JobSystem) -> Result<Report, Error> {
        self.run(dt, max_iters, jobs, &mut Control::unlimited())
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
            let r = self.project(dt, max_iters, jobs, ctl)?;
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
