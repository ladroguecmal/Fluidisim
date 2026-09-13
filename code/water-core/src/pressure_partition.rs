//! S219 : partition complète, tas maximal dans la mémoire de l'appelant.
use super::{local_bound::SlopeOrder, Error, Field};

#[derive(Clone, Copy, Debug, Default)]
pub struct SlopeCell {
    min: [f32; 2],
    max: [f32; 2],
    bound: f32,
}
impl SlopeCell {
    pub fn rectangle(&self) -> ([f32; 2], [f32; 2]) {
        (self.min, self.max)
    }
    pub fn bound(&self) -> f32 {
        self.bound
    }
    fn width(&self) -> f32 {
        (self.max[0] - self.min[0]).max(self.max[1] - self.min[1])
    }
    fn above(&self, other: &Self) -> bool {
        self.bound > other.bound || (self.bound == other.bound && self.width() > other.width())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PartitionStop {
    Evaluations,
    Capacity,
    Precision,
    Zero,
}
#[derive(Debug, PartialEq, Eq)]
pub enum PartitionError {
    EmptyPool,
    ZeroBudget,
    Evaluation(Error),
}
#[derive(Clone, Copy, Debug)]
pub struct SlopePartition {
    pub bound: f32,
    pub leaves: usize,
    pub evaluations: usize,
    pub stop: PartitionStop,
}
fn down(cells: &mut [SlopeCell], mut i: usize) {
    loop {
        let left = i * 2 + 1;
        if left >= cells.len() {
            break;
        }
        let right = left + 1;
        let child = if right < cells.len() && cells[right].above(&cells[left]) {
            right
        } else {
            left
        };
        if !cells[child].above(&cells[i]) {
            break;
        }
        cells.swap(i, child);
        i = child;
    }
}
impl Field<'_> {
    /// Recalcule une partition à cet instant. Au succès, pool[..leaves] couvre le
    /// rectangle fermé. Budget en appels locaux (1 puis 2 par division), pas en ms.
    /// Arrêt normal avec couverture sur capacité/budget/précision ; pool vide ou
    /// budget nul refusés. Une erreur numérique invalide le résultat de cet appel.
    /// Pas de reprise inter-appels, de cache temporel ni de migration d'admission.
    /// O(evaluations*(N+log(pool.len()))), mémoire O(pool.len()), sans allocation.
    /// Hérite des limites numériques d'ADR-135, sans certificat f32 supplémentaire.
    pub fn partition_slope_envelope(
        &self,
        min: [f32; 2],
        max: [f32; 2],
        pool: &mut [SlopeCell],
        budget: usize,
    ) -> Result<SlopePartition, PartitionError> {
        self.partition_slope_envelope_order(min, max, pool, budget, SlopeOrder::First)
    }
    /// ADR-136 : même parcours, borne locale de l'ordre demandé. `First` est l'appel S219.
    pub fn partition_slope_envelope_order(
        &self,
        min: [f32; 2],
        max: [f32; 2],
        pool: &mut [SlopeCell],
        budget: usize,
        order: SlopeOrder,
    ) -> Result<SlopePartition, PartitionError> {
        if pool.is_empty() {
            return Err(PartitionError::EmptyPool);
        }
        if budget == 0 {
            return Err(PartitionError::ZeroBudget);
        }
        let root = self
            .local_bound(min, max, order)
            .map_err(PartitionError::Evaluation)?;
        pool[0] = SlopeCell {
            min,
            max,
            bound: root,
        };
        let (mut count, mut evaluations) = (1, 1);
        let stop = loop {
            let parent = pool[0];
            if parent.bound == 0.0 {
                break PartitionStop::Zero;
            }
            if budget - evaluations < 2 {
                break PartitionStop::Evaluations;
            }
            if count == pool.len() {
                break PartitionStop::Capacity;
            }
            let axis = if parent.max[0] - parent.min[0] >= parent.max[1] - parent.min[1] {
                0
            } else {
                1
            };
            let middle = parent.min[axis] + (parent.max[axis] - parent.min[axis]) * 0.5;
            if middle <= parent.min[axis] || middle >= parent.max[axis] {
                break PartitionStop::Precision;
            }
            let (mut a, mut b) = (parent, parent);
            a.max[axis] = middle;
            b.min[axis] = middle;
            a.bound = self
                .local_bound(a.min, a.max, order)
                .map_err(PartitionError::Evaluation)?
                .min(parent.bound);
            b.bound = self
                .local_bound(b.min, b.max, order)
                .map_err(PartitionError::Evaluation)?
                .min(parent.bound);
            // Les deux évaluations réussissent avant toute mutation de la couverture.
            pool[0] = a;
            down(&mut pool[..count], 0);
            pool[count] = b;
            let mut i = count;
            while i > 0 && pool[i].above(&pool[(i - 1) / 2]) {
                let p = (i - 1) / 2;
                pool.swap(i, p);
                i = p;
            }
            count += 1;
            evaluations += 2;
        };
        Ok(SlopePartition {
            bound: pool[0].bound,
            leaves: count,
            evaluations,
            stop,
        })
    }
}
