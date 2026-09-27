//! **S396 — les domaines comme ensembles de blocs** ([ADR-006](../../docs/adr/ADR-006-cellules-domaines-solveurs.md) §3–4),
//! C8a de la campagne du solveur volumique 3D ([ADR-207](../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)).
//!
//! Un domaine δ est un **ensemble de blocs** d'un réseau commun : **fusion = union, séparation = partition**, sans remaillage
//! ni interpolation, si `dx` et le repère coïncident (§3.1). Les domaines de δ couvrent toute la profondeur d'eau (ADR-175) :
//! un bloc est ici une **colonne** de `BLOCK × BLOCK` mailles, repérée dans le plan.
//!
//! §4 : deux domaines fusionnent quand leurs ensembles **dilatés** du rayon de couplage `r_c` se touchent ; un domaine se
//! sépare quand la partition de ses blocs dilatés compte plusieurs composantes **pendant plus d'une seconde**. Une seule
//! relation sert aux deux — deux blocs sont **liés** si leurs dilatations de `r` blocs se touchent, Chebyshev ≤ `2r + 1` —,
//! sans quoi une fusion pourrait se défaire au pas suivant.
//!
//! Capacité réservée auprès de l'hôte à la configuration (I-06) : ni l'insertion ni les composantes n'allouent.

use crate::host::{AllocError, HostServices};

/// Côté d'un bloc, en mailles (ADR-006 §3 : blocs de 8³ ; ici des colonnes de 8 × 8, toute la profondeur).
pub const BLOCK: usize = 8;

/// Délai de séparation : une partition en plusieurs composantes doit tenir une seconde entière (ADR-006 §4).
pub const SPLIT_AFTER_US: u64 = 1_000_000;

/// Un bloc du réseau commun, en unités de blocs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Block {
    pub i: i32,
    pub j: i32,
}

/// Refus des ensembles de blocs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockError {
    /// Plus de blocs que la capacité réservée.
    Capacity,
    /// L'hôte refuse la réserve (scellé ou arène pleine).
    Host,
}

/// La distance de Chebyshev entre deux blocs, en blocs.
#[inline]
pub fn chebyshev(a: Block, b: Block) -> i32 {
    (a.i - b.i).abs().max((a.j - b.j).abs())
}

/// Deux blocs sont **liés** au rayon `r` si leurs dilatations de `r` blocs se touchent ou se recouvrent.
#[inline]
pub fn linked(a: Block, b: Block, r: i32) -> bool {
    chebyshev(a, b) <= 2 * r + 1
}

/// Un ensemble de blocs, trié et sans doublon, avec son tampon de composantes ; capacité fixée à la configuration.
pub struct BlockSet {
    blocks: Vec<Block>,
    /// Union-find des composantes, à la taille de la capacité.
    parent: Vec<u32>,
    capacity: usize,
}

impl BlockSet {
    /// Octets réservés pour `capacity` blocs : deux entiers par bloc, un parent.
    pub fn reserved_bytes(capacity: usize) -> Option<usize> {
        capacity.checked_mul(3 * 4)
    }

    /// Réserve `capacity` blocs auprès de l'hôte, **avant `seal()`** (I-06).
    pub fn with_capacity(host: &mut HostServices, capacity: usize) -> Result<Self, BlockError> {
        let bytes = Self::reserved_bytes(capacity).ok_or(BlockError::Capacity)?;
        host.alloc.alloc_persistent(bytes).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => BlockError::Host,
        })?;
        Ok(BlockSet { blocks: Vec::with_capacity(capacity), parent: vec![0; capacity], capacity })
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
    pub fn clear(&mut self) {
        self.blocks.clear();
    }
    pub fn contains(&self, b: Block) -> bool {
        self.blocks.binary_search(&b).is_ok()
    }

    /// Ajoute un bloc ; `false` s'il y était. Refus `Capacity` au-delà de la réserve — rien n'est changé.
    pub fn insert(&mut self, b: Block) -> Result<bool, BlockError> {
        match self.blocks.binary_search(&b) {
            Ok(_) => Ok(false),
            Err(at) => {
                if self.blocks.len() == self.capacity {
                    return Err(BlockError::Capacity);
                }
                self.blocks.insert(at, b);
                Ok(true)
            }
        }
    }

    /// **La fusion** : l'union, dans `self`. Refus `Capacity` si elle ne tient pas — rien n'est changé.
    pub fn union_with(&mut self, other: &BlockSet) -> Result<(), BlockError> {
        let extra = other.blocks.iter().filter(|b| !self.contains(**b)).count();
        if self.blocks.len() + extra > self.capacity {
            return Err(BlockError::Capacity);
        }
        for b in &other.blocks {
            self.insert(*b)?;
        }
        Ok(())
    }

    /// Les bornes de l'ensemble, blocs inclus : `(min, max)` ; `None` s'il est vide.
    pub fn bounds(&self) -> Option<(Block, Block)> {
        let first = *self.blocks.first()?;
        let (mut lo, mut hi) = (first, first);
        for b in &self.blocks {
            lo = Block { i: lo.i.min(b.i), j: lo.j.min(b.j) };
            hi = Block { i: hi.i.max(b.i), j: hi.j.max(b.j) };
        }
        Some((lo, hi))
    }

    /// **Le critère de fusion** (ADR-006 §4) : un bloc de chacun est lié à l'autre au rayon `r`.
    pub fn touches(&self, other: &BlockSet, r: i32) -> bool {
        self.blocks.iter().any(|a| other.blocks.iter().any(|b| linked(*a, *b, r)))
    }

    fn find(&mut self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            let up = self.parent[self.parent[x as usize] as usize];
            self.parent[x as usize] = up;
            x = up;
        }
        x
    }

    /// **La partition** : le nombre de composantes des blocs liés au rayon `r`, et pour chaque bloc (dans l'ordre de
    /// `blocks`) le numéro de sa composante, de 0 à `n − 1` dans l'ordre de première apparition. Aucune allocation.
    pub fn components(&mut self, r: i32, labels: &mut [u32]) -> u32 {
        let n = self.blocks.len();
        assert!(labels.len() >= n, "labels trop court");
        for k in 0..n {
            self.parent[k] = k as u32;
        }
        for a in 0..n {
            for b in a + 1..n {
                if linked(self.blocks[a], self.blocks[b], r) {
                    let (ra, rb) = (self.find(a as u32), self.find(b as u32));
                    if ra != rb {
                        self.parent[ra.max(rb) as usize] = ra.min(rb);
                    }
                }
            }
        }
        // Les racines d'abord, puis numérotées dans l'ordre de première apparition ; la forêt sert ensuite de table.
        for k in 0..n {
            labels[k] = self.find(k as u32);
        }
        for k in 0..n {
            self.parent[k] = u32::MAX;
        }
        let mut count = 0u32;
        for k in 0..n {
            let root = labels[k] as usize;
            if self.parent[root] == u32::MAX {
                self.parent[root] = count;
                count += 1;
            }
            labels[k] = self.parent[root];
        }
        count
    }
}

/// **L'horloge de séparation** d'un domaine (ADR-006 §4) : le temps continu passé à plusieurs composantes. Elle repart de
/// zéro dès que la partition revient à une composante, et à la naissance du domaine — une fusion en fait naître un.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SplitClock {
    split_for_us: u64,
}

impl SplitClock {
    /// Avance de `dt_us` avec la partition courante ; `true` quand la séparation est due.
    pub fn advance(&mut self, dt_us: u64, components: u32) -> bool {
        if components > 1 {
            self.split_for_us = self.split_for_us.saturating_add(dt_us);
        } else {
            self.split_for_us = 0;
        }
        self.split_for_us >= SPLIT_AFTER_US
    }
    /// Le temps continu déjà passé à plusieurs composantes, µs.
    pub fn split_for_us(&self) -> u64 {
        self.split_for_us
    }
}

/// **S401 — l'horizon utile d'une prévision** ([ADR-013](../../docs/adr/ADR-013-prediction-activation-precalcul.md) §2) : un
/// objet de capacité de manœuvre `a_max` déplace son point d'arrivée de `½·a_max·t²` en `t` secondes ; préparer n'a de sens
/// que tant que cet écart tient dans le domaine qu'on allait construire, `R` : `t ≤ √(2R/a_max)`. Un objet sans manœuvre
/// (`a_max` nul, balistique) n'a pas de borne propre : `cap`, en secondes, la donne.
pub fn useful_horizon(r_domain: f32, a_max: f32, cap: f32) -> f32 {
    if a_max > 0. {
        (2. * r_domain / a_max).sqrt().min(cap)
    } else {
        cap
    }
}

/// **S401 — un objet que le domaine suit**, et sa prévision : dans le plan de la fenêtre, en mètres depuis son coin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tracked {
    /// Position, m.
    pub position: [f32; 2],
    /// Vitesse, m/s.
    pub velocity: [f32; 2],
    /// Capacité de manœuvre, m/s² (ADR-013 §2).
    pub a_max: f32,
    /// Horizon de la prévision, s — `useful_horizon` ; zéro : l'objet là où il est, sans prévision.
    pub horizon: f32,
    /// Rayon de l'objet, m.
    pub radius: f32,
}

/// **S401 — le domaine qui suit la perturbation** (C8b ; [ADR-006](../../docs/adr/ADR-006-cellules-domaines-solveurs.md)
/// §4, ADR-013 §2) : quels blocs d'une fenêtre sont du domaine.
///
/// Un bloc est **requis** s'il est à moins de `radius` blocs (Chebyshev, la dilatation de S396) d'un bloc **marqué** : un bloc
/// dont une colonne s'écarte du repos de plus de `threshold`, ou que touche l'**enveloppe prévue** d'un objet —
/// les disques de centre `p + V·t` et de rayon `r + ½·a_max·t²`, pour `t` de 0 à l'horizon : tout ce que l'objet peut
/// atteindre s'il n'accélère pas au-delà de `a_max`. Un bloc que rien ne requiert depuis `release_after_us` sort du domaine
/// (ADR-006 §4 : 0,25 s pour un bloc) — ce délai est aussi sa durée de vie minimale et l'anti-battement.
///
/// Capacité réservée auprès de l'hôte à la configuration (I-06) : la mise à jour n'alloue rien.
pub struct Follow {
    nbx: usize,
    nby: usize,
    /// Côté d'une maille, m.
    dx: f32,
    /// Rayon de couplage `r_c`, en blocs (ADR-006 §4).
    pub radius: i32,
    /// Seuil d'activité, `|η − repos|`, m.
    pub threshold: f32,
    /// Délai de libération d'un bloc que rien ne requiert, µs.
    pub release_after_us: u64,
    /// Par bloc, le dernier instant où il était requis ; `u64::MAX`, jamais.
    required_at: Vec<u64>,
    /// Par bloc : marqué, puis requis — tampon de la dilatation.
    mark: Vec<u8>,
    /// Par bloc : 1, du domaine.
    set: Vec<u8>,
}

impl Follow {
    /// Octets réservés pour une fenêtre de `nbx × nby` blocs : un instant et deux octets par bloc.
    pub fn reserved_bytes(nbx: usize, nby: usize) -> Option<usize> {
        nbx.checked_mul(nby)?.checked_mul(8 + 2)
    }

    /// Une fenêtre de `nx × ny` colonnes de côté `dx`, soit `⌈nx/BLOCK⌉ × ⌈ny/BLOCK⌉` blocs ; réservée **avant `seal()`**.
    pub fn with_capacity(host: &mut HostServices, nx: usize, ny: usize, dx: f32, radius: i32, threshold: f32,
        release_after_us: u64) -> Result<Self, BlockError> {
        let (nbx, nby) = (nx.div_ceil(BLOCK), ny.div_ceil(BLOCK));
        let bytes = Self::reserved_bytes(nbx, nby).ok_or(BlockError::Capacity)?;
        host.alloc.alloc_persistent(bytes).map_err(|e| match e {
            AllocError::Sealed | AllocError::OutOfArena => BlockError::Host,
        })?;
        let n = nbx * nby;
        Ok(Follow { nbx, nby, dx, radius, threshold, release_after_us, required_at: vec![u64::MAX; n], mark: vec![0; n],
            set: vec![0; n] })
    }

    /// Les dimensions de la fenêtre, en blocs.
    pub fn blocks(&self) -> (usize, usize) {
        (self.nbx, self.nby)
    }

    /// Le domaine courant, un octet par bloc (`bj·nbx + bi`) : 1, du domaine.
    pub fn set(&self) -> &[u8] {
        &self.set
    }

    /// **La mise à jour** à l'instant `now_us` : l'activité lue sur la surface `eta` (`nx` colonnes par rangée, repos
    /// `rest`), l'enveloppe de chaque objet, la dilatation, puis la libération. Rend le domaine. Aucune allocation.
    pub fn update(&mut self, now_us: u64, eta: &[f32], rest: f32, nx: usize, tracked: &[Tracked]) -> &[u8] {
        self.mark.fill(0);
        for (c, e) in eta.iter().enumerate() {
            if (e - rest).abs() > self.threshold {
                let (bi, bj) = ((c % nx) / BLOCK, (c / nx) / BLOCK);
                self.mark[bj * self.nbx + bi] = 1;
            }
        }
        for t in tracked {
            self.mark_envelope(t);
        }
        self.dilate();
        for b in 0..self.mark.len() {
            if self.mark[b] != 0 {
                self.required_at[b] = now_us;
            }
            let at = self.required_at[b];
            self.set[b] = u8::from(at != u64::MAX && now_us.saturating_sub(at) < self.release_after_us);
        }
        &self.set
    }

    /// **S404 — un changement de niveau** (ADR-210) : les blocs de cette fenêtre qui recouvrent une colonne non nulle de `mask` —
    /// la même fenêtre à un autre niveau, `nx × ny` colonnes de côté `dx` — sont requis à `now_us`. Le domaine d'arrivée couvre
    /// ainsi celui de départ, et le transfert d'état ne perd rien ; il suit ensuite sa perturbation, et ce que plus rien ne
    /// requiert sort après le délai. Aucune allocation.
    pub fn require_cover(&mut self, now_us: u64, mask: &[u8], nx: usize, ny: usize, dx: f32) {
        let side = BLOCK as f64 * self.dx as f64;
        let span = |k: usize, n: usize| {
            let (a, b) = (k as f64 * dx as f64, (k + 1) as f64 * dx as f64);
            ((a / side).floor() as usize, ((b / side).ceil() as usize).min(n))
        };
        for j in 0..ny {
            for i in 0..nx {
                if mask[j * nx + i] == 0 {
                    continue;
                }
                let ((bi0, bi1), (bj0, bj1)) = (span(i, self.nbx), span(j, self.nby));
                for bj in bj0..bj1 {
                    for bi in bi0..bi1 {
                        self.required_at[bj * self.nbx + bi] = now_us;
                        self.set[bj * self.nbx + bi] = 1;
                    }
                }
            }
        }
    }

    /// Le domaine en colonnes : `out[j·nx + i]` vaut 1 si la colonne est dans un bloc du domaine.
    pub fn columns(&self, nx: usize, ny: usize, out: &mut [u8]) {
        for j in 0..ny {
            for i in 0..nx {
                out[j * nx + i] = self.set[(j / BLOCK) * self.nbx + i / BLOCK];
            }
        }
    }

    /// Marque les blocs que touche l'enveloppe de `t` : sur chaque intervalle de temps, le disque qui contient la part de
    /// l'enveloppe qu'il couvre — centre au milieu du trajet, rayon du bout de l'intervalle plus la moitié du trajet —, les
    /// intervalles assez courts pour que le centre ne parcoure pas plus d'un demi-bloc.
    fn mark_envelope(&mut self, t: &Tracked) {
        let speed = (t.velocity[0] * t.velocity[0] + t.velocity[1] * t.velocity[1]).sqrt();
        let side = BLOCK as f32 * self.dx;
        let horizon = t.horizon.max(0.);
        let steps = ((speed * horizon / (0.5 * side)).ceil() as usize).clamp(1, 1024);
        let dt = horizon / steps as f32;
        for k in 0..steps {
            let mid = (k as f32 + 0.5) * dt;
            let end = (k + 1) as f32 * dt;
            let centre = [t.position[0] + t.velocity[0] * mid, t.position[1] + t.velocity[1] * mid];
            let reach = t.radius + 0.5 * t.a_max.max(0.) * end * end + 0.5 * speed * dt;
            self.mark_disk(centre, reach);
        }
    }

    /// Marque les blocs dont le carré touche le disque.
    fn mark_disk(&mut self, centre: [f32; 2], reach: f32) {
        let side = BLOCK as f32 * self.dx;
        // Les blocs que recouvre le carré englobant le disque, coupés à la fenêtre ; `None` s'il en est dehors.
        let range = |c: f32, n: usize| -> Option<(usize, usize)> {
            let (lo, hi) = (((c - reach) / side).floor(), ((c + reach) / side).floor());
            if !(hi >= 0.) || lo >= n as f32 {
                return None;
            }
            Some((lo.max(0.) as usize, (hi as usize).min(n - 1)))
        };
        let (Some((i0, i1)), Some((j0, j1))) = (range(centre[0], self.nbx), range(centre[1], self.nby)) else {
            return;
        };
        for bj in j0..=j1 {
            for bi in i0..=i1 {
                let (x0, y0) = (bi as f32 * side, bj as f32 * side);
                let dxp = (x0 - centre[0]).max(0.).max(centre[0] - (x0 + side));
                let dyp = (y0 - centre[1]).max(0.).max(centre[1] - (y0 + side));
                if dxp * dxp + dyp * dyp <= reach * reach {
                    self.mark[bj * self.nbx + bi] = 1;
                }
            }
        }
    }

    /// La dilatation de Chebyshev de `radius` blocs, séparable : en `x` vers `set` (tampon), puis en `y` vers `mark`.
    fn dilate(&mut self) {
        let (nbx, nby, r) = (self.nbx as isize, self.nby as isize, self.radius.max(0) as isize);
        for bj in 0..nby {
            for bi in 0..nbx {
                let any = (bi - r..=bi + r).any(|x| x >= 0 && x < nbx && self.mark[(bj * nbx + x) as usize] != 0);
                self.set[(bj * nbx + bi) as usize] = u8::from(any);
            }
        }
        for bj in 0..nby {
            for bi in 0..nbx {
                let any = (bj - r..=bj + r).any(|y| y >= 0 && y < nby && self.set[(y * nbx + bi) as usize] != 0);
                self.mark[(bj * nbx + bi) as usize] = u8::from(any);
            }
        }
    }
}

#[cfg(test)]
#[path = "tests_domain_blocks.rs"]
mod tests;
