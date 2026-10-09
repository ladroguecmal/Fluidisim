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
//!
//! **S639 — le fond** ([`Apic3::set_seabed`]) : une hauteur par colonne, mise en escalier — les mailles dont le centre est sous le fond
//! sont solides (`SOLID`), leurs faces des parois immobiles ; les particules qui y entrent sont reposées au-dessus, sans vitesse
//! descendante ; la reconstruction reflète les particules sous le fond, comme les parois (S389). Éprouvé : le repos sur une pente (S639).
//!
//! **S640 — le fond lisse** ([`Apic3::set_seabed_smooth`], `apic3d_lisse.rs`) : les faces coupées — chaque face porte sa fraction
//! ouverte à l'eau, la projection les pondère (Batty, Bertails et Bridson 2007).

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
    /// S483 : le tri par maille vient d'être fait sur les positions courantes (`particles_to_grid`) ; la reconstruction le reprend.
    pub(crate) bin_fresh: bool,
    /// **S488 (K2-4)** — les gouttes (`enable_droplets`, `apic3d_gouttes.rs`) ; `None`, le défaut : le pas d'avant, au bit.
    pub(crate) gouttes: Option<Box<gouttes::Gouttes>>,
    /// **S682 — la sortie à droite** (`enable_right_outlet`) : le volume des particules retirées au bord droit, par rangée `j` au dernier
    /// pas (m³), puis le total et le nombre ; `None`, le défaut : le domaine retient les particules — au bit.
    pub(crate) sortie_droite: Option<Box<SortieDroite>>,
    /// **S698 — le bord gauche par particules** (`enable_left_inlet`, `apic3d_gauche.rs`) ; `None`, le défaut — au bit.
    pub(crate) gauche: Option<Box<gauche::BordGauche>>,
    /// **S709 — la projection de densité** (`enable_density_projection`, `apic3d_densite.rs`) ; `None`, le défaut — au bit.
    pub(crate) densite: Option<Box<densite::Densite>>,
    /// **S724 — les bords en y par particules** (`enable_y_boundaries`, `apic3d_bords_y.rs`) ; `None`, le défaut — au bit.
    pub(crate) bords_y: Option<Box<bords_y::BordsY>>,
    /// **S639 — le fond en escalier** : par colonne (`j·nx + i`), le nombre de mailles solides depuis le bas ; `None`, pas de fond.
    pub(crate) seabed: Option<Vec<u16>>,
    /// **S640 — le fond lisse** (`set_seabed_smooth`, `apic3d_lisse.rs`) : les hauteurs et les fractions ouvertes des faces ;
    /// `None`, toutes les faces ouvertes — au bit.
    pub(crate) lisse: Option<Box<lisse::FondLisse>>,
    /// **S645 — le fond glissant** (`set_seabed_slip`) : sous l'escalier, les faces tangentielles du dessus des marches prennent la
    /// vitesse de la face juste au-dessus ; `false`, le défaut : nulles (S639).
    pub(crate) seabed_slip: bool,
    /// **S645 — l'air balistique** (`set_ballistic_air`) : une face d'air qu'une particule a alimentée garde sa vitesse (et la gravité)
    /// au lieu de recevoir l'extrapolation ; `false`, le défaut : extrapolée (S318).
    pub(crate) ballistic_air: bool,
    /// **S740** — le plafond d'itérations du gradient conjugué (`set_pressure_max_iterations`) ; défaut `PRESSURE_MAX_ITERATIONS`, au bit.
    pub(crate) pression_max_iterations: u32,
    /// **S740** — les passes de la séparation des particules (`set_separation_passes`) ; défaut `SEPARATION_PASSES`, au bit.
    pub(crate) separation_passes: usize,
    /// **S653 — le corps libre** (`set_body_mass`) : la masse de la sphère, kg ; `None`, le défaut : la sphère imposée (S393), au bit.
    pub(crate) body_mass: Option<f32>,
    /// **S655** — l'accélération du corps libre au pas précédent (la masse ajoutée implicite).
    pub(crate) body_accel: [f32; 3],
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
            seabed: None,
            lisse: None,
            seabed_slip: false,
            ballistic_air: false,
            pression_max_iterations: PRESSURE_MAX_ITERATIONS,
            separation_passes: SEPARATION_PASSES,
            body_mass: None,
            body_accel: [0.; 3],
            columns: None,
            background: [None; 2],
            background_time_s: 0.,
            open_x: None,
            poches: None,
            jobs: None,
            bin_fresh: false,
            gouttes: None,
            sortie_droite: None,
            gauche: None,
            densite: None,
            bords_y: None,
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

    /// **S682 — la sortie à droite** (le relais au rivage, ADR-271) : une particule qui franchit le bord droit n'est plus retenue par
    /// le domaine ; elle est retirée et son volume (`dx³/8`) compté par rangée ([`Apic3::right_outlet`]). Demande les bords ouverts ;
    /// refusée avec les gouttes (leur état par particule). S693 : permise avec la zone des colonnes, qui n'a que des champs de grille.
    /// Réservée à la configuration (I-06).
    pub fn enable_right_outlet(&mut self, host: &mut HostServices) -> Result<(), Error> {
        if self.open_x.is_none() || self.sortie_droite.is_some() || self.gouttes.is_some() {
            return Err(Error::Domain);
        }
        let ny = self.domain.ny;
        host.alloc.alloc_persistent(ny * 8).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => Error::Domain,
        })?;
        self.sortie_droite = Some(Box::new(SortieDroite { pas: vec![0.; ny], total: 0., retirees: 0, reservoir: vec![0.; ny], entre: 0., posees: 0,
            refusees: 0 }));
        Ok(())
    }

    /// S682 — la sortie à droite : `(le volume retiré par rangée au dernier pas, m³ ; le volume total ; le nombre de particules)` ;
    /// `None` sans sortie.
    pub fn right_outlet(&self) -> Option<(&[f64], f64, u64)> {
        self.sortie_droite.as_ref().map(|s| (&s.pas[..], s.total, s.retirees))
    }

    /// **S683 — l'entrée à droite** : `volumes` (m³, par rangée `j`, positifs) s'ajoutent au réservoir de chaque rangée ; chaque quantum
    /// entier (`dx³/8`) devient une particule posée dans la dernière colonne, sur le réseau au quart de maille, à la place la moins
    /// occupée sous la surface (la plus haute particule de la colonne plus `dx/4`) et au-dessus du fond, la plus basse d'abord ; elle
    /// reçoit `vitesse`. Une particule que la capacité refuse est comptée, son quantum reste au réservoir. Demande la sortie à droite.
    pub fn feed_right(&mut self, volumes: &[f64], vitesse: [f32; 3]) -> Result<(), Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let Some(mut s) = self.sortie_droite.take() else { return Err(Error::Domain) };
        if volumes.len() != ny {
            self.sortie_droite = Some(s);
            return Err(Error::Shape);
        }
        if volumes.iter().any(|v| !v.is_finite()) || vitesse.iter().any(|v| !v.is_finite()) {
            self.sortie_droite = Some(s);
            return Err(Error::NotFinite);
        }
        if volumes.iter().any(|&v| v < 0.) {
            self.sortie_droite = Some(s);
            return Err(Error::Domain);
        }
        let quantum = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        let h = dx / PER_AXIS as f32;
        let nz2 = PER_AXIS * nz;
        let mut occupation = vec![0u32; PER_AXIS * PER_AXIS * nz2];
        for j in 0..ny {
            s.reservoir[j] += volumes[j];
            s.entre += volumes[j];
            if s.reservoir[j] < quantum {
                continue;
            }
            // L'occupation des places de la rangée dans la dernière colonne, et la surface.
            occupation.fill(0);
            let (x0, y0) = ((nx - 1) as f32 * dx, j as f32 * dx);
            let mut haut = f32::MIN;
            for k in 0..self.n {
                let p = self.x[k];
                if p[0] >= x0 && p[1] >= y0 && p[1] < y0 + dx {
                    let a = (((p[0] - x0) / h) as usize).min(PER_AXIS - 1);
                    let b = (((p[1] - y0) / h) as usize).min(PER_AXIS - 1);
                    let c = ((p[2] / h) as usize).min(nz2 - 1);
                    occupation[(c * PER_AXIS + b) * PER_AXIS + a] += 1;
                    haut = haut.max(p[2]);
                }
            }
            while s.reservoir[j] >= quantum {
                let mut choix: Option<(u32, usize)> = None;
                for c in 0..nz2 {
                    for b in 0..PER_AXIS {
                        for a in 0..PER_AXIS {
                            let q = [x0 + (a as f32 + 0.5) * h, y0 + (b as f32 + 0.5) * h, (c as f32 + 0.5) * h];
                            let zb = if self.lisse.is_some() { self.smooth_seabed_height(q[0], q[1]) } else { self.seabed_height(nx - 1, j) };
                            let surface = if haut > f32::MIN { haut + 0.5 * h } else { zb + dx };
                            if q[2] <= zb + 0.05 * dx || (q[2] > surface && choix.is_some()) {
                                continue;
                            }
                            let i = (c * PER_AXIS + b) * PER_AXIS + a;
                            if choix.is_none_or(|(o, _)| occupation[i] < o) {
                                choix = Some((occupation[i], i));
                            }
                        }
                    }
                }
                let Some((_, i)) = choix else { break };
                if self.n >= self.x.len() {
                    s.refusees += 1;
                    break;
                }
                let (a, b, c) = (i % PER_AXIS, (i / PER_AXIS) % PER_AXIS, i / (PER_AXIS * PER_AXIS));
                let m = self.n;
                self.x[m] = [x0 + (a as f32 + 0.5) * h, y0 + (b as f32 + 0.5) * h, (c as f32 + 0.5) * h];
                self.vel[m] = vitesse;
                self.c[m] = [[0.; 3]; 3];
                self.n += 1;
                occupation[i] += 1;
                haut = haut.max(self.x[m][2]);
                s.reservoir[j] -= quantum;
                s.posees += 1;
            }
        }
        self.sortie_droite = Some(s);
        Ok(())
    }

    /// **S689 — rendre une particule au bord droit** : la particule de la rangée `j` la plus proche du bord droit est retirée (le
    /// remboursement d'une dette du relais au rivage) — S690 : dans n'importe quelle colonne, quand le ressaut a vidé la dernière. `false`
    /// si la rangée n'en a pas ; non comptée dans la sortie.
    pub fn take_right(&mut self, j: usize) -> bool {
        let Domain3 { ny, dx, .. } = self.domain;
        if j >= ny {
            return false;
        }
        let y0 = j as f32 * dx;
        let mut choix: Option<usize> = None;
        for k in 0..self.n {
            let p = self.x[k];
            if p[1] >= y0 && p[1] < y0 + dx && choix.is_none_or(|c| p[0] > self.x[c][0]) {
                choix = Some(k);
            }
        }
        let Some(k) = choix else { return false };
        let last = self.n - 1;
        self.x[k] = self.x[last];
        self.vel[k] = self.vel[last];
        self.c[k] = self.c[last];
        self.n = last;
        true
    }

    /// S683 — l'entrée à droite : `(le réservoir par rangée, m³ ; le volume reçu ; les particules posées ; refusées)` ; `None` sans
    /// sortie à droite.
    pub fn right_inlet(&self) -> Option<(&[f64], f64, u64, u64)> {
        self.sortie_droite.as_ref().map(|s| (&s.reservoir[..], s.entre, s.posees, s.refusees))
    }

    /// S682 : retire les particules au-delà du bord droit, compte leur volume.
    fn drain_right(&mut self) {
        let Some(mut s) = self.sortie_droite.take() else { return };
        let Domain3 { nx, ny, dx, .. } = self.domain;
        let lx = nx as f32 * dx;
        let quantum = (dx as f64).powi(3) / (PER_AXIS * PER_AXIS * PER_AXIS) as f64;
        s.pas.fill(0.);
        let mut k = 0;
        while k < self.n {
            if self.x[k][0] >= lx {
                let j = ((self.x[k][1] / dx) as usize).min(ny - 1);
                s.pas[j] += quantum;
                s.total += quantum;
                s.retirees += 1;
                let last = self.n - 1;
                self.x[k] = self.x[last];
                self.vel[k] = self.vel[last];
                self.c[k] = self.c[last];
                self.n = last;
            } else {
                k += 1;
            }
        }
        self.sortie_droite = Some(s);
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
    /// **S639 — le fond** : une hauteur par colonne (`ny × nx`, `x` le plus rapide, m), mise en escalier (les mailles dont le centre est
    /// sous elle deviennent solides) ; `None` l'ôte. Refus : une longueur fausse (`Domain`), une valeur non finie (`NotFinite`), hors de
    /// `[0, nz·dx]` (`Domain`). Le réglage alloue, le pas non.
    pub fn set_seabed(&mut self, fond: Option<&[f32]>) -> Result<(), Error> {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let Some(f) = fond else {
            self.seabed = None;
            return Ok(());
        };
        if f.len() != nx * ny {
            return Err(Error::Domain);
        }
        if f.iter().any(|v| !v.is_finite()) {
            return Err(Error::NotFinite);
        }
        if f.iter().any(|&v| !(0. ..=nz as f32 * dx).contains(&v)) {
            return Err(Error::Domain);
        }
        self.seabed = Some(f.iter().map(|&h| (0..nz).filter(|&k| (k as f32 + 0.5) * dx < h).count() as u16).collect());
        Ok(())
    }

    /// **S645 — le fond glissant** : les faces `u`/`v` de la maille solide du dessus d'une colonne, quand elles séparent deux mailles
    /// solides, prennent la vitesse de la face de même position juste au-dessus — l'interpolation vers une particule de la moitié basse
    /// de la première maille d'eau ne mêle plus une vitesse nulle (A333). Une face de contremarche et la face `w` du dessus restent nulles.
    pub fn set_seabed_slip(&mut self, on: bool) {
        self.seabed_slip = on;
    }

    /// **S645 — l'air balistique** : une face d'air alimentée par une particule est tenue pour connue par l'extrapolation — le film
    /// mince, étiqueté d'air, garde sa vitesse et la gravité au lieu de recevoir celle de l'eau derrière lui (A333).
    pub fn set_ballistic_air(&mut self, on: bool) {
        self.ballistic_air = on;
    }

    /// **S740 — le plafond d'itérations du gradient conjugué** de la projection (le diagnostic de l'onde solitaire, ONDE-SOLITAIRE-3D-S739) ;
    /// le défaut, `PRESSURE_MAX_ITERATIONS`, au bit.
    pub fn set_pressure_max_iterations(&mut self, n: u32) {
        self.pression_max_iterations = n.max(1);
    }

    /// **S740 — les passes de la séparation des particules** (0 : aucune) ; le défaut, `SEPARATION_PASSES`, au bit.
    pub fn set_separation_passes(&mut self, n: usize) {
        self.separation_passes = n;
    }

    /// **S639** — la hauteur du fond en escalier de la colonne `(i, j)` (m) ; 0 sans fond.
    pub fn seabed_height(&self, i: usize, j: usize) -> f32 {
        self.seabed.as_ref().map_or(0., |s| s[j * self.domain.nx + i] as f32 * self.domain.dx)
    }

    /// **S653 — le corps libre** : avec une masse (kg), la sphère reçoit à chaque pas la force de pression de l'eau
    /// ([`Apic3::body_force`]) et son poids (`g_eff`), et suit sa vitesse ; le pas suivant impose cette vitesse à l'eau — un couplage
    /// **explicite**, stable tant que le corps pèse plus que sa masse ajoutée (½ρV pour une sphère). Un contact simple : le corps ne
    /// descend pas sous le fond de sa colonne ni ne sort du domaine. `None` : la sphère imposée, au bit. Refus : une masse nulle ou
    /// négative (`Domain`), non finie (`NotFinite`).
    pub fn set_body_mass(&mut self, mass: Option<f32>) -> Result<(), Error> {
        if let Some(m) = mass {
            if !m.is_finite() {
                return Err(Error::NotFinite);
            }
            if m <= 0. {
                return Err(Error::Domain);
            }
        }
        self.body_mass = mass;
        self.body_accel = [0.; 3];
        Ok(())
    }

    /// **S652–S653 — la force de pression de l'eau sur la sphère**, N : sur les faces entre une maille du corps (solide, centre dans
    /// la sphère) et une maille d'eau, la pression à la face — extrapolée linéairement des deux mailles d'eau le long de la normale
    /// (lue au centre de la voisine, elle est une demi-maille trop loin : 1,38 × Archimède au repos ; extrapolée, 1,13, le biais d'un
    /// corps de 32 mailles en escalier) — fois `dx²`, dirigée vers le corps. Zéro sans corps.
    pub fn body_force(&self) -> [f64; 3] {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let Some(b) = self.body else { return [0.; 3] };
        let (l, p) = (&self.label, &self.p);
        let mut f = [0f64; 3];
        let surf = (dx * dx) as f64;
        let pas = [1isize, nx as isize, (nx * ny) as isize];
        let dims = [nx, ny, nz];
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let q = [(i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx];
                    let e = [q[0] - b.center[0], q[1] - b.center[1], q[2] - b.center[2]];
                    let c = (k * ny + j) * nx + i;
                    if l[c] != SOLID || e[0] * e[0] + e[1] * e[1] + e[2] * e[2] >= b.radius * b.radius {
                        continue;
                    }
                    let pos = [i, j, k];
                    for axe in 0..3 {
                        for (d, signe) in [(-1isize, 1f64), (1, -1.)] {
                            let (n1, n2) = (pos[axe] as isize + d, pos[axe] as isize + 2 * d);
                            if n1 < 0 || n1 >= dims[axe] as isize {
                                continue;
                            }
                            let v = (c as isize + d * pas[axe]) as usize;
                            if l[v] != WATER {
                                continue;
                            }
                            let v2 = (c as isize + 2 * d * pas[axe]) as usize;
                            let pf = if n2 >= 0 && n2 < dims[axe] as isize && l[v2] == WATER {
                                1.5 * p[v] as f64 - 0.5 * p[v2] as f64
                            } else {
                                p[v] as f64
                            };
                            f[axe] += signe * pf * surf;
                        }
                    }
                }
            }
        }
        f
    }

    /// **S655** — le volume immergé de la sphère, m³ : ses mailles (solides, centre dans la sphère) où `φ < 0` — la reconstruction
    /// reflète l'eau à travers le corps (S393).
    pub fn immersed_body_volume(&self) -> f32 {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let Some(b) = self.body else { return 0. };
        let mut n = 0usize;
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let q = [(i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx];
                    let e = [q[0] - b.center[0], q[1] - b.center[1], q[2] - b.center[2]];
                    let c = (k * ny + j) * nx + i;
                    if self.label[c] == SOLID && e[0] * e[0] + e[1] * e[1] + e[2] * e[2] < b.radius * b.radius && self.phi[c] < 0. {
                        n += 1;
                    }
                }
            }
        }
        n as f32 * dx * dx * dx
    }

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

/// S682 — le compte de la sortie à droite ; S683, celui de l'entrée.
pub(crate) struct SortieDroite {
    pub(crate) pas: Vec<f64>,
    pub(crate) total: f64,
    pub(crate) retirees: u64,
    /// S683 — par rangée, le volume reçu qui n'a pas encore fait un quantum (m³).
    pub(crate) reservoir: Vec<f64>,
    pub(crate) entre: f64,
    pub(crate) posees: u64,
    pub(crate) refusees: u64,
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
        // S483 (ADR-222 D2) : chaque face **collecte** les particules des mailles qui la touchent (en z, y, x ; dans l'ordre du tri)
        // — l'ordre de `p2g` de la carte —, au lieu que chaque particule disperse sur ses huit nœuds : une écriture disjointe, en
        // parallèle avec un système de tâches, au bit de la boucle séquentielle. Le tri se fait ici ; la reconstruction le reprend
        // (les particules n'ont pas bougé entre les deux).
        self.bin();
        self.bin_fresh = true;
        let jobs = self.jobs.clone();
        for axis in 0..3 {
            let (mut field, mut weight) = match axis {
                0 => (core::mem::take(&mut self.u), core::mem::take(&mut self.wu)),
                1 => (core::mem::take(&mut self.v), core::mem::take(&mut self.wv)),
                _ => (core::mem::take(&mut self.w), core::mem::take(&mut self.ww)),
            };
            {
                let this = &*self;
                match &jobs {
                    Some(jobs) => {
                        let grain = 4096;
                        jobs.parallel_fill_f32(&mut weight, grain, &|start, out: &mut [f32]| {
                            for (o, w) in out.iter_mut().enumerate() {
                                *w = this.gather_face(axis, start + o).1;
                            }
                        });
                        let wref = &weight;
                        jobs.parallel_fill_f32(&mut field, grain, &|start, out: &mut [f32]| {
                            for (o, f) in out.iter_mut().enumerate() {
                                let w = wref[start + o];
                                *f = if w > 0. { this.gather_face(axis, start + o).0 / w } else { 0. };
                            }
                        });
                    }
                    None => {
                        for f in 0..field.len() {
                            let (sum, w) = this.gather_face(axis, f);
                            weight[f] = w;
                            field[f] = if w > 0. { sum / w } else { 0. };
                        }
                    }
                }
            }
            match axis {
                0 => (self.u, self.wu) = (field, weight),
                1 => (self.v, self.wv) = (field, weight),
                _ => (self.w, self.ww) = (field, weight),
            }
        }
    }

    /// La face `f` de la grille `axis` : `Σ wt·(v + C·(x_f − x_p))` et `Σ wt` sur les particules dont elle est un des huit
    /// nœuds (les mailles `idx − 1 … idx` le long de l'axe, `idx − 1 … idx + 1` en travers), dans l'ordre des mailles et du tri.
    fn gather_face(&self, axis: usize, f: usize) -> (f32, f32) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let (origin, dims) = staggered(self.domain, axis);
        let idx = [f % dims[0], (f / dims[0]) % dims[1], f / (dims[0] * dims[1])];
        let xf = self.node(axis, f);
        let n = [nx, ny, nz];
        let mut lo = [0usize; 3];
        let mut hi = [0usize; 3];
        for a in 0..3 {
            lo[a] = idx[a].saturating_sub(1);
            hi[a] = if a == axis { idx[a] } else { idx[a] + 1 }.min(n[a] - 1);
        }
        let (mut sum, mut wsum) = (0f32, 0f32);
        for c in lo[2]..=hi[2] {
            for b in lo[1]..=hi[1] {
                for a in lo[0]..=hi[0] {
                    let cell = self.cell(a, b, c);
                    for s in self.bin_start[cell]..self.bin_start[cell + 1] {
                        let k = self.order[s as usize] as usize;
                        if self.is_droplet(k) {
                            continue;
                        }
                        let p = self.x[k];
                        // Le poids de ce seul nœud, axe par axe, avec les bornes de `weights` et son ordre de multiplication.
                        let mut wa = [0f32; 3];
                        for d in 0..3 {
                            let top = (dims[d] - 1) as f32;
                            let fr = (p[d] / dx - origin[d]).clamp(0., (top - 1e-4).max(0.));
                            let base = fr.floor() as usize;
                            let frac = fr - base as f32;
                            let next = (base + 1).min(dims[d] - 1);
                            wa[d] = if idx[d] == base { 1. - frac } else { 0. } + if idx[d] == next { frac } else { 0. };
                        }
                        let wt = wa[0] * wa[1] * wa[2];
                        if wt == 0. {
                            continue;
                        }
                        let cr = self.c[k][axis];
                        let affine = cr[0] * (xf[0] - p[0]) + cr[1] * (xf[1] - p[1]) + cr[2] * (xf[2] - p[2]);
                        sum += wt * (self.vel[k][axis] + affine);
                        wsum += wt;
                    }
                }
            }
        }
        (sum, wsum)
    }

    /// **Grille → particules**, APIC : la vitesse interpolée et la matrice affine `C_ab = Σ ∂_b w · u_a`.
    pub(crate) fn grid_to_particles(&mut self) {
        // S483 (ADR-222 D2) : la vitesse et la matrice affine de chaque particule, deux écritures disjointes (trois puis neuf
        // flottants par particule), en parallèle avec un système de tâches, au bit de la boucle séquentielle.
        let n = self.n;
        let (mut vel, mut c) = (core::mem::take(&mut self.vel), core::mem::take(&mut self.c));
        {
            let this = &*self;
            let fill_v = |start: usize, out: &mut [f32]| {
                for (o, v) in out.chunks_exact_mut(3).enumerate() {
                    if this.is_droplet(start / 3 + o) {
                        continue;
                    }
                    let (vk, _) = this.gather_particle(start / 3 + o);
                    v.copy_from_slice(&vk);
                }
            };
            let fill_c = |start: usize, out: &mut [f32]| {
                for (o, m) in out.chunks_exact_mut(9).enumerate() {
                    if this.is_droplet(start / 9 + o) {
                        m.fill(0.);
                        continue;
                    }
                    let (_, ck) = this.gather_particle(start / 9 + o);
                    for a in 0..3 {
                        m[3 * a..3 * a + 3].copy_from_slice(&ck[a]);
                    }
                }
            };
            match &self.jobs {
                Some(jobs) => {
                    jobs.parallel_fill_f32(vel[..n].as_flattened_mut(), 3 * 4096, &fill_v);
                    jobs.parallel_fill_f32(c[..n].as_flattened_mut().as_flattened_mut(), 9 * 4096, &fill_c);
                }
                None => {
                    // Séquentiel : un seul passage (les poids une fois), les mêmes valeurs.
                    for k in 0..n {
                        if this.is_droplet(k) {
                            c[k] = [[0.; 3]; 3];
                            continue;
                        }
                        let (vk, ck) = this.gather_particle(k);
                        vel[k] = vk;
                        c[k] = ck;
                    }
                }
            }
        }
        self.vel = vel;
        self.c = c;
    }

    /// La vitesse et la matrice affine que la grille donne à la particule `k` (`grid_to_particles`).
    fn gather_particle(&self, k: usize) -> ([f32; 3], [[f32; 3]; 3]) {
        let dx = self.domain.dx;
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
        (v, c)
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
        // S483 : le tri de `particles_to_grid`, si les particules n'ont pas bougé depuis.
        if !core::mem::take(&mut self.bin_fresh) {
            self.bin();
        }
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
        // S640 : sous le fond lisse, `φ` étendu horizontalement depuis l'eau et l'air.
        self.extend_phi_smooth();
        // S678 : le film du rivage, par sa dernière couche.
        self.film_smooth();
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
        // S639 : le fond reflète aussi — l'image `2·z_b − z` des particules proches, cherchée seulement près du fond de la colonne.
        // S640 : avec le fond lisse, les images de l'escalier sont celles de chaque particule (`stair_images`), non de la maille.
        let lisse = self.lisse.is_some();
        let fond = self.seabed.as_ref().map(|_| self.seabed_height(i, j)).filter(|&zb| zb > 0. && q[2] - zb < radius);
        // S639 : les contremarches — une voisine latérale dont le fond monte plus haut que celui de la colonne, à portée du noyau,
        // reflète à travers la face commune, comme une paroi, les particules sous son sommet.
        let zb_ici = self.seabed_height(i, j);
        let marche = |a: isize, b: isize| -> Option<f32> {
            let (ia, jb) = (i as isize + a, j as isize + b);
            if ia < 0 || jb < 0 || ia as usize >= nx || jb as usize >= ny || self.seabed.is_none() {
                return None;
            }
            let zn = self.seabed_height(ia as usize, jb as usize);
            (zn > zb_ici && zn > q[2] - radius).then_some(zn)
        };
        let contremarches = [
            marche(-1, 0).map(|zn| (0usize, i as f32 * dx, 1f32, zn)),
            marche(1, 0).map(|zn| (0usize, (i + 1) as f32 * dx, -1f32, zn)),
            marche(0, -1).map(|zn| (1usize, j as f32 * dx, 1f32, zn)),
            marche(0, 1).map(|zn| (1usize, (j + 1) as f32 * dx, -1f32, zn)),
        ];
        let (mut sw, mut sx) = (0f32, [0f32; 3]);
        for c in k.saturating_sub(reach)..(k + reach + 1).min(nz) {
            for b in j.saturating_sub(reach)..(j + reach + 1).min(ny) {
                for a in i.saturating_sub(reach)..(i + reach + 1).min(nx) {
                    let cell = self.cell(a, b, c);
                    for s in self.bin_start[cell]..self.bin_start[cell + 1] {
                        let k = self.order[s as usize] as usize;
                    if self.is_droplet(k) {
                        continue;
                    }
                    let p0 = self.x[k];
                        // S640 : les images de l'escalier du fond lisse, puis celles des parois pour chacune.
                        let escalier = if lisse { self.stair_images(p0, radius) } else { [None; 5] };
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
                                    for im in escalier.iter().flatten() {
                                        add([mirror(im[0], *mx, lx), mirror(im[1], *my, ly), mirror(im[2], *mz, 0.)]);
                                    }
                                    if let Some(zb) = fond {
                                        if p[2] >= zb && p[2] - zb < radius {
                                            add([p[0], p[1], 2. * zb - p[2]]);
                                        }
                                    }
                                    // `(axe, plan, côté de l'eau)` : l'image d'une particule du côté de l'eau, à moins d'un rayon du plan.
                                    for &(axe, plan, cote, zn) in contremarches.iter().flatten() {
                                        let e = (p[axe] - plan) * cote;
                                        if e >= 0. && e < radius && p[2] < zn {
                                            let mut im = p;
                                            im[axe] = 2. * plan - p[axe];
                                            add(im);
                                        }
                                    }
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
        self.label_seabed();
        self.label_smooth();
        self.label_body();
        // S488 : les gouttes (rien sans `enable_droplets`).
        self.droplets_classify();
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
        // S653 : le corps libre — la force de pression et le poids changent sa vitesse. S655 : la masse ajoutée implicite,
        // `(m + m_a)·aₙ₊₁ = F + m·g + m_a·aₙ` — le couplage explicite lançait le corps plus vite que l'eau (S654).
        if let (Some(m), Some(mut b)) = (self.body_mass, self.body) {
            let f = self.body_force();
            let ma = 0.5 * self.rho * self.immersed_body_volume();
            let mut acc = [0f32; 3];
            for a in 0..3 {
                let poids = if a == 2 { -m * self.g_eff } else { 0. };
                acc[a] = (f[a] as f32 + poids + ma * self.body_accel[a]) / (m + ma);
                b.velocity[a] += dt * acc[a];
            }
            self.body_accel = acc;
            self.body = Some(b);
        }
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
        self.drain_right();
        self.drain_left();
        self.drain_y();
        mark("advection");
        if upto == ApicStage::Advect {
            return Ok(partial);
        }
        if self.separation {
            self.separate();
        }
        // S709 : la projection de densité (rien sans `enable_density_projection`).
        self.density_project();
        self.move_body(dt);
        self.push_seabed();
        self.push_smooth();
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
        // S724 : les bords en y par particules imposent leurs vitesses normales.
        self.walls_y();
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

    /// **S639** — les mailles sous le fond deviennent solides.
    fn label_seabed(&mut self) {
        let Some(sb) = self.seabed.as_ref() else { return };
        let Domain3 { nx, ny, .. } = self.domain;
        for j in 0..ny {
            for i in 0..nx {
                for k in 0..sb[j * nx + i] as usize {
                    self.label[(k * ny + j) * nx + i] = SOLID;
                }
            }
        }
    }

    /// **S639** — une particule sous le fond de sa colonne est reposée au-dessus (0,05 maille), sa vitesse descendante annulée.
    fn push_seabed(&mut self) {
        if self.seabed.is_none() {
            return;
        }
        let dx = self.domain.dx;
        for k in 0..self.n {
            let (i, j, _) = self.cell_of(self.x[k]);
            let zb = self.seabed_height(i, j);
            if self.x[k][2] < zb + 0.05 * dx {
                self.x[k][2] = zb + 0.05 * dx;
                if self.vel[k][2] < 0. {
                    self.vel[k][2] = 0.;
                }
            }
        }
    }

    /// Toute face qui touche une maille solide prend la vitesse du corps — et, sous le fond (S639), une vitesse nulle. Appelé après
    /// chaque opération qui écrit les faces — sans quoi une extrapolation réécrirait la paroi (S320).
    fn impose_body(&mut self) {
        // S640 : les faces fermées par le fond lisse.
        self.impose_smooth();
        let Domain3 { nx, ny, nz, dx } = self.domain;
        if let Some(sb) = self.seabed.as_ref() {
            for j in 0..ny {
                for i in 0..nx {
                    for k in 0..sb[j * nx + i] as usize {
                        self.u[(k * ny + j) * (nx + 1) + i] = 0.;
                        self.u[(k * ny + j) * (nx + 1) + i + 1] = 0.;
                        self.v[(k * (ny + 1) + j) * nx + i] = 0.;
                        self.v[(k * (ny + 1) + j + 1) * nx + i] = 0.;
                        self.w[(k * ny + j) * nx + i] = 0.;
                        self.w[((k + 1) * ny + j) * nx + i] = 0.;
                    }
                }
            }
            // S645 : le fond glissant — les faces tangentielles du dessus des marches, recopiées d'au-dessus.
            if self.seabed_slip {
                for j in 0..ny {
                    for i in 0..nx {
                        let c = sb[j * nx + i] as usize;
                        if c == 0 || c >= nz {
                            continue;
                        }
                        let (k, k0) = (c - 1, c);
                        if i > 0 && sb[j * nx + i - 1] as usize >= c {
                            self.u[(k * ny + j) * (nx + 1) + i] = self.u[(k0 * ny + j) * (nx + 1) + i];
                        }
                        if i + 1 < nx && sb[j * nx + i + 1] as usize >= c {
                            self.u[(k * ny + j) * (nx + 1) + i + 1] = self.u[(k0 * ny + j) * (nx + 1) + i + 1];
                        }
                        if j > 0 && sb[(j - 1) * nx + i] as usize >= c {
                            self.v[(k * (ny + 1) + j) * nx + i] = self.v[(k0 * (ny + 1) + j) * nx + i];
                        }
                        if j + 1 < ny && sb[(j + 1) * nx + i] as usize >= c {
                            self.v[(k * (ny + 1) + j + 1) * nx + i] = self.v[(k0 * (ny + 1) + j + 1) * nx + i];
                        }
                    }
                }
            }
        }
        let Some(b) = self.body else {
            self.walls();
            return;
        };
        let r2 = b.radius * b.radius;
        // S639 : la vitesse du corps aux seules mailles de la sphère (le fond est aussi `SOLID`).
        let solid = |a: usize, j: usize, k: usize| {
            let q = [(a as f32 + 0.5) * dx, (j as f32 + 0.5) * dx, (k as f32 + 0.5) * dx];
            let d = [q[0] - b.center[0], q[1] - b.center[1], q[2] - b.center[2]];
            self.label[(k * ny + j) * nx + a] == SOLID && d[0] * d[0] + d[1] * d[1] + d[2] * d[2] < r2
        };
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
        let Domain3 { nx, ny, nz, dx } = self.domain;
        // S653 : le contact du corps libre — ni sous le fond de sa colonne, ni hors du domaine ; la vitesse normale annulée.
        if self.body_mass.is_some() {
            let hauts = [nx as f32 * dx - b.radius, ny as f32 * dx - b.radius, nz as f32 * dx - b.radius];
            let (ic, jc) = (((b.center[0] / dx).max(0.) as usize).min(nx - 1), ((b.center[1] / dx).max(0.) as usize).min(ny - 1));
            let bas = [b.radius, b.radius, b.radius + self.seabed_height(ic, jc)];
            for a in 0..3 {
                if b.center[a] < bas[a] {
                    b.center[a] = bas[a];
                    b.velocity[a] = b.velocity[a].max(0.);
                }
                if b.center[a] > hauts[a] {
                    b.center[a] = hauts[a];
                    b.velocity[a] = b.velocity[a].min(0.);
                }
            }
        }
        self.body = Some(b);
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

    /// `y = A·x` sur les mailles d'eau : `Σ (x_c − x_n)` vers l'eau, `x_c/θ` vers l'air, rien vers une paroi (sans `1/dx²`) ; chaque
    /// terme pondéré par la fraction ouverte de la face (S640).
    fn apply(&self, x: &[f32], y: &mut [f32]) {
        // S483 (ADR-222 D2) : une écriture par maille, en parallèle avec un système de tâches, au bit.
        let Domain3 { nx, ny, .. } = self.domain;
        let fill = |start: usize, out: &mut [f32]| {
            for (o, yc) in out.iter_mut().enumerate() {
                let c = start + o;
                let (i, j, k) = (c % nx, (c / nx) % ny, c / (nx * ny));
                if self.label[c] != WATER {
                    *yc = 0.;
                    continue;
                }
                let mut s = 0f32;
                for (n, f, axis) in self.neighbours(i, j, k).into_iter().flatten() {
                    // S640 : pondéré par la fraction ouverte de la face (1 sans fond lisse — au bit).
                    let a = self.fraction(axis, f);
                    match self.label[n] {
                        WATER => s += a * (x[c] - x[n]),
                        AIR => s += a * x[c] / self.theta(c, n),
                        // Le corps : paroi mobile, flux imposé, pas de pression.
                        _ => {}
                    }
                }
                *yc = s;
            }
        };
        match &self.jobs {
            Some(jobs) => jobs.parallel_fill_f32(y, nx * ny * 4, &fill),
            None => fill(0, y),
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
                        let (axis, f) = match m {
                            0 => (0, (k * ny + j) * (nx + 1) + i),
                            1 => (0, (k * ny + j) * (nx + 1) + i + 1),
                            2 => (1, (k * (ny + 1) + j) * nx + i),
                            3 => (1, (k * (ny + 1) + j + 1) * nx + i),
                            4 => (2, (k * ny + j) * nx + i),
                            _ => (2, ((k + 1) * ny + j) * nx + i),
                        };
                        let face = match axis {
                            0 => self.u[f],
                            1 => self.v[f],
                            _ => self.w[f],
                        };
                        // S640 : la divergence et le laplacien pondérés par la fraction ouverte (1 sans fond lisse — au bit).
                        let a = self.fraction(axis, f);
                        div += sign * a * face;
                        if let Some((n, _, _)) = nb {
                            match self.label[n] {
                                WATER => diag += a,
                                AIR => {
                                    let t = self.theta(c, n);
                                    diag += a / t;
                                    // S444 : la valeur de `p′` à la surface, en mode relatif (nulle sans fond).
                                    ghost += a * self.surface_pressure(c, n) / t;
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
        // Gradient conjugué préconditionné par la diagonale (S709 : sorti dans `pcg`, au bit).
        let (it, rr, b2) = self.pcg();
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
                        // S640 : une face fermée par le fond lisse n'est pas corrigée (elle reste nulle).
                        if self.label[c] == SOLID || self.label[n] == SOLID || self.fraction(axis, f) == 0. {
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

    /// **S709 — le gradient conjugué préconditionné par la diagonale** : `A·p = rhs` sur les mailles d'eau, `diag` donnée ; rend
    /// `(itérations, ‖r‖², ‖b‖²)`. Sorti de `project` tel quel (la projection de densité le partage).
    pub(crate) fn pcg(&mut self) -> (u32, f64, f64) {
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
        while b2 > 0. && rr > PRESSURE_TOLERANCE2 * b2 && it < self.pression_max_iterations {
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
        (it, rr, b2)
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
                    // S640 : pondérée par les fractions ouvertes (1 sans fond lisse — au bit).
                    let (fu, fv, fw) = ((k * ny + j) * (nx + 1) + i, (k * (ny + 1) + j) * nx + i, (k * ny + j) * nx + i);
                    let div = self.fraction(0, fu + 1) * self.u[fu + 1] - self.fraction(0, fu) * self.u[fu]
                        + self.fraction(1, fv + nx) * self.v[fv + nx] - self.fraction(1, fv) * self.v[fv]
                        + self.fraction(2, fw + nx * ny) * self.w[fw + nx * ny] - self.fraction(2, fw) * self.w[fw];
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
                // S645 : l'air balistique — une face qu'une particule a alimentée est connue.
                let fed = self.ballistic_air && match axis {
                    0 => self.wu[f],
                    1 => self.wv[f],
                    _ => self.ww[f],
                } > 0.;
                let ok = ok || fed;
                // S678 : une face à peine ouverte par le fond lisse est extrapolée, non lue.
                let ok = ok && self.lisse.as_ref().is_none_or(|l| {
                    let a = self.fraction(axis, f);
                    a == 0. || a >= l.seuil_face
                });
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
        // S682 : avec la sortie à droite, le bord droit ne retient plus.
        let x_haut = if self.sortie_droite.is_some() { f32::MAX } else { lx - margin };
        // S698 : avec le bord gauche par particules, le bord gauche ne retient plus.
        let x_bas = if self.gauche.is_some() { f32::MIN } else { margin };
        // S724 : avec les bords en y par particules, le devant et le derrière ne retiennent plus.
        let (y_bas, y_haut) = if self.bords_y.is_some() { (f32::MIN, f32::MAX) } else { (margin, ly - margin) };
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
        // S483 (ADR-222 D2) : chaque particule n'écrit que sa position — une écriture disjointe (`as_flattened_mut`, trois flottants
        // par particule), en parallèle avec un système de tâches, au bit de la boucle séquentielle.
        let n = self.n;
        let mut x = core::mem::take(&mut self.x);
        {
            let this = &*self;
            let fill = |start: usize, out: &mut [f32]| {
                for (o, q) in out.chunks_exact_mut(3).enumerate() {
                    let k = start / 3 + o;
                    // S488 : une goutte suit sa trajectoire balistique (plus bas).
                    if this.is_droplet(k) {
                        continue;
                    }
                    let p = [q[0], q[1], q[2]]; // la position avant le pas (le tableau pris contient encore les positions)
                    let v1 = with_b(this, this.grid_velocity(p), p, t0);
                    let mid = [p[0] + 0.5 * dt * v1[0], p[1] + 0.5 * dt * v1[1], p[2] + 0.5 * dt * v1[2]];
                    let v2 = with_b(this, this.grid_velocity(mid), mid, t0 + 0.5 * dt as f64);
                    q[0] = (p[0] + dt * v2[0]).clamp(x_bas, x_haut);
                    q[1] = (p[1] + dt * v2[1]).clamp(y_bas, y_haut);
                    q[2] = (p[2] + dt * v2[2]).clamp(margin, lz - margin);
                }
            };
            let flat = x[..n].as_flattened_mut();
            match &self.jobs {
                Some(jobs) => jobs.parallel_fill_f32(flat, 3 * 4096, &fill),
                None => fill(0, flat),
            }
        }
        self.x = x;
        // S488 : les gouttes, balistiques ; bornées au domaine, la composante normale annulée à la paroi.
        if self.gouttes.is_some() {
            let (g, d, rho) = (self.droplet_gravity(), droplet_diameter(dx as f64), self.rho as f64);
            let (lo, hi) = ([margin; 3], [lx - margin, ly - margin, lz - margin]);
            for k in 0..n {
                if !self.is_droplet(k) {
                    continue;
                }
                let (mut q, mut v) = ballistic_step(self.x[k], self.vel[k], g, d, rho, dt);
                for a in 0..3 {
                    if q[a] < lo[a] || q[a] > hi[a] {
                        q[a] = q[a].clamp(lo[a], hi[a]);
                        v[a] = 0.;
                    }
                }
                self.x[k] = q;
                self.vel[k] = v;
            }
        }
    }

    /// **Séparation** : deux particules plus proches que 0,4 maille s'écartent chacune du quart de leur recouvrement,
    /// deux passes ; les vitesses ne changent pas, une eau au repos (à une demi-maille d'écart) non plus (S320 P3).
    fn separate(&mut self) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let (lx, ly, lz) = (nx as f32 * dx, ny as f32 * dx, nz as f32 * dx);
        let (d_min, margin) = (SEPARATION * dx, 1e-3 * dx);
        let n = self.n;
        for _ in 0..self.separation_passes {
            self.bin();
            // S483 (ADR-222 D2) : chaque particule **collecte** sa poussée sur ses voisines (mailles en z, y, x ; particules dans
            // l'ordre du tri) — l'ordre de `separate_shift` de la carte —, au lieu de l'accumuler par paires : une écriture disjointe,
            // en parallèle avec un système de tâches, au bit de la boucle séquentielle. (Avant S483, l'accumulation par paires
            // donnait la même poussée dans un autre ordre de somme.)
            let mut shift = core::mem::take(&mut self.shift);
            {
                let this = &*self;
                let fill = |start: usize, out: &mut [f32]| {
                    for (o, sh) in out.chunks_exact_mut(3).enumerate() {
                        let a = start / 3 + o;
                        if this.is_droplet(a) {
                            sh.copy_from_slice(&[0.; 3]);
                            continue;
                        }
                        let xa = this.x[a];
                        let (i, j, k) = this.cell_of(xa);
                        let mut acc = [0f32; 3];
                        for c in k.saturating_sub(1)..(k + 2).min(nz) {
                            for b_ in j.saturating_sub(1)..(j + 2).min(ny) {
                                for a_ in i.saturating_sub(1)..(i + 2).min(nx) {
                                    let other = this.cell(a_, b_, c);
                                    for sb in this.bin_start[other]..this.bin_start[other + 1] {
                                        let b = this.order[sb as usize] as usize;
                                        if b == a || this.is_droplet(b) {
                                            continue;
                                        }
                                        let e = [this.x[b][0] - xa[0], this.x[b][1] - xa[1], this.x[b][2] - xa[2]];
                                        let d = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt();
                                        if d < d_min && d > 1e-6 * dx {
                                            let m = 0.25 * (d_min - d) / d;
                                            for t in 0..3 {
                                                acc[t] -= m * e[t];
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        sh.copy_from_slice(&acc);
                    }
                };
                let flat = shift[..n].as_flattened_mut();
                match &self.jobs {
                    Some(jobs) => jobs.parallel_fill_f32(flat, 3 * 4096, &fill),
                    None => fill(0, flat),
                }
            }
            self.shift = shift;
            let mut x = core::mem::take(&mut self.x);
            {
                let this = &*self;
                let fill = |start: usize, out: &mut [f32]| {
                    for (o, q) in out.chunks_exact_mut(3).enumerate() {
                        let k = start / 3 + o;
                        let (p, d) = ([q[0], q[1], q[2]], this.shift[k]);
                        let mut r = [
                            (p[0] + d[0]).clamp(margin, lx - margin),
                            (p[1] + d[1]).clamp(margin, ly - margin),
                            (p[2] + d[2]).clamp(margin, lz - margin),
                        ];
                        // S400 : la séparation est tenue du côté de la bande — une particule qu'elle pousserait dans une colonne de
                        // la zone garde sa position horizontale (l'échange ne passe que par le flux de la face).
                        if this.columns.is_some() {
                            let (a, b) = (this.cell_of(r), this.cell_of(p));
                            if this.column_of(a.0, a.1) && !this.column_of(b.0, b.1) {
                                r[0] = p[0];
                                r[1] = p[1];
                            }
                        }
                        q.copy_from_slice(&r);
                    }
                };
                let flat = x[..n].as_flattened_mut();
                match &self.jobs {
                    Some(jobs) => jobs.parallel_fill_f32(flat, 3 * 4096, &fill),
                    None => fill(0, flat),
                }
            }
            self.x = x;
        }
    }
}

#[path = "apic3d_columns.rs"]
mod columns;
#[path = "apic3d_gouttes.rs"]
mod gouttes;
pub use gouttes::{ballistic_step, droplet_diameter, CD_GOUTTE, RHO_AIR, SIGMA_EAU, WEBER_RUPTURE};
#[path = "apic3d_poches.rs"]
mod poches;
#[path = "apic3d_lisse.rs"]
mod lisse;
#[path = "apic3d_gauche.rs"]
mod gauche;
#[path = "apic3d_naissance.rs"]
mod naissance;
#[path = "apic3d_densite.rs"]
mod densite;
#[path = "apic3d_bords_y.rs"]
mod bords_y;
#[path = "apic3d_deplacement.rs"]
mod deplacement;
pub use densite::DensityVariant;
pub use poches::{pockets_reserved_bytes, AirPocket, AirPocketState, GAMMA_AIR, MAX_POCKETS, POCHE_MAILLES_MIN, P_ATM, RAPPEL_VOLUME_S};
pub use columns::{columns_reserved_bytes, ColumnsChange, ColumnsSwitch, FloorChange, LinearSwell};

#[cfg(test)]
#[path = "tests_apic3d.rs"]
pub(crate) mod tests;
