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

#[cfg(test)]
#[path = "tests_domain_blocks.rs"]
mod tests;
