//! L'hôte harnais — SPEC-003 §10, étage H1.
//!
//! Un hôte au sens d'ADR-020, au même titre que le moteur de jeu ou l'outil de cuisson. Il ne rend
//! aucun pixel, ne lit aucun asset lourd, et n'ouvre aucun GPU : ce sont les conditions du mode
//! `check` (SPEC-003 §4), et c'est ce qui lui permet de tourner à chaque commit.

use water_core::{AllocError, AllocStats, Allocator, JobSystem, Sink};

/// Allocateur à arène, compteur et scellement — SPEC-004 §8.1.
///
/// Après `seal()`, toute allocation **échoue** au lieu d'être comptée : I-06 devient une propriété
/// mécanique et non une consigne. Les tentatives refusées sont comptées séparément, parce qu'une
/// tentative refusée est un défaut à corriger, pas un incident à ignorer.
pub struct ArenaAllocator {
    capacity: usize,
    used: usize,
    sealed: bool,
    stats: AllocStats,
}

impl ArenaAllocator {
    pub fn with_capacity(bytes: usize) -> Self {
        ArenaAllocator {
            capacity: bytes,
            used: 0,
            sealed: false,
            stats: AllocStats::default(),
        }
    }
}

impl Allocator for ArenaAllocator {
    fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AllocError> {
        if self.sealed {
            self.stats.refused_after_seal += 1;
            return Err(AllocError::Sealed);
        }
        if self.used + bytes > self.capacity {
            return Err(AllocError::OutOfArena);
        }
        let offset = self.used;
        self.used += bytes;
        self.stats.persistent_bytes += bytes;
        self.stats.persistent_calls += 1;
        Ok(offset)
    }

    fn seal(&mut self) {
        self.sealed = true;
    }

    fn is_sealed(&self) -> bool {
        self.sealed
    }

    fn stats(&self) -> AllocStats {
        self.stats
    }
}

/// Système de tâches séquentiel — SPEC-004 §8.2.
///
/// Un seul fil, et c'est **volontaire pour H1** : cette implémentation *est* la référence de
/// l'ordre de fusion. Le jour où une version parallèle existera, l'assertion « changer
/// `worker_count` change la vitesse, jamais le résultat » se vérifiera contre celle-ci.
pub struct SequentialJobs;

impl JobSystem for SequentialJobs {
    fn worker_count(&self) -> u32 {
        1
    }

    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        grain: usize,
        reduce: &dyn Fn(usize, usize) -> f64,
        merge: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        let g = grain.max(1);
        let mut acc = init;
        let mut start = 0usize;
        while start < n {
            let end = (start + g).min(n);
            acc = merge(acc, reduce(start, end));
            start = end;
        }
        acc
    }
}

/// Journal du harnais. Écrit sur la sortie d'erreur pour ne pas polluer le rapport.
pub struct StderrSink;

impl Sink for StderrSink {
    fn warn(&self, message: &str) {
        eprintln!("[avertissement] {message}");
    }
    fn metric(&self, name: &str, value: f64) {
        eprintln!("[métrique] {name} = {value}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seal_fait_echouer_lallocation() {
        // SPEC-004 §8.1 : après scellement, l'allocation échoue au lieu d'être comptée.
        let mut a = ArenaAllocator::with_capacity(1024);
        assert!(a.alloc_persistent(64).is_ok());
        a.seal();
        assert_eq!(a.alloc_persistent(1), Err(AllocError::Sealed));
        assert_eq!(a.stats().refused_after_seal, 1);
        assert_eq!(a.stats().persistent_calls, 1);
    }

    #[test]
    fn reduction_reproductible_a_grain_fixe() {
        // Le résultat est bit à bit stable pour un `grain` donné : c'est ce qu'exige le régime D2.
        let jobs = SequentialJobs;
        let f = |a: usize, b: usize| (a..b).map(|i| (i as f64) * 0.1).sum::<f64>();
        let m = |x: f64, y: f64| x + y;
        let a = jobs.parallel_reduce_ordered_f64(1000, 64, &f, &m, 0.0);
        let b = jobs.parallel_reduce_ordered_f64(1000, 64, &f, &m, 0.0);
        assert_eq!(a.to_bits(), b.to_bits());
    }

    #[test]
    fn le_grain_fait_partie_du_contrat_car_il_change_le_resultat() {
        // **Découverte de H1, consignée sous forme exécutable.**
        //
        // SPEC-004 §8.2 énonce : « changer `worker_count` change la vitesse, jamais le résultat ».
        // C'est vrai *à condition que le découpage soit fixé*. L'addition flottante n'étant pas
        // associative, deux grains différents donnent deux sommes différentes — ici sur un cas
        // aussi banal qu'une suite arithmétique.
        //
        // Conséquence pour l'interface : `grain` est **une donnée du contrat**, fixée par
        // l'appelant, et jamais dérivée du nombre de fils de la machine. Sans cela le corollaire
        // d'I-03 est faux, et il l'est silencieusement.
        // Quatre valeurs suffisent, à condition qu'elles soient mal conditionnées : une somme
        // d'ordres de grandeur voisins ne révèle rien, et c'est ce qui rend le piège coûteux.
        const V: [f64; 4] = [1.0, 1e16, -1e16, 1.0];
        let jobs = SequentialJobs;
        let f = |a: usize, b: usize| {
            let mut s = 0.0f64;
            for i in a..b {
                s += V[i];
            }
            s
        };
        let m = |x: f64, y: f64| x + y;
        let g1 = jobs.parallel_reduce_ordered_f64(4, 1, &f, &m, 0.0);
        let g2 = jobs.parallel_reduce_ordered_f64(4, 2, &f, &m, 0.0);
        assert_eq!(g1, 1.0, "grain 1 : ((1 + 1e16) − 1e16) + 1");
        assert_eq!(g2, 0.0, "grain 2 : (1 + 1e16) + (−1e16 + 1)");
    }
}
