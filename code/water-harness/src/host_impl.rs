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

/// Système de tâches **parallèle** — SPEC-004 §8.2, construit en S243.
///
/// Il ne fournit que `parallel_fill_f32`, l'écriture disjointe : la réduction reste celle de
/// `SequentialJobs`, dont l'ordre de fusion **est** la référence (ADR-029 §3). Un parallélisme de
/// somme demanderait de fusionner dans l'ordre des tranches, ce que rien ne consomme aujourd'hui.
///
/// Les fils sont créés par `std::thread::scope` à chaque appel : aucune dépendance, aucun état
/// partagé, et les emprunts restent vérifiés par le compilateur — le harnais n'a pas de `unsafe`.
/// Le prix de cette simplicité est le coût de création des fils, mesuré et publié
/// (`docs/validation/PARALLELISME-S243.md`).
pub struct ScopedJobs {
    workers: u32,
}

impl ScopedJobs {
    /// `workers` fils au plus ; `1` rend exactement le chemin séquentiel, sans créer de fil.
    pub fn with_workers(workers: u32) -> Self {
        Self { workers: workers.max(1) }
    }
    /// Autant de fils que la machine en déclare, `1` si elle ne sait pas le dire.
    pub fn detected() -> Self {
        Self::with_workers(
            std::thread::available_parallelism().map_or(1, |n| n.get() as u32),
        )
    }
}

impl JobSystem for ScopedJobs {
    fn worker_count(&self) -> u32 {
        self.workers
    }

    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        grain: usize,
        reduce: &dyn Fn(usize, usize) -> f64,
        merge: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        SequentialJobs.parallel_reduce_ordered_f64(n, grain, reduce, merge, init)
    }

    fn parallel_fill_f32(
        &self,
        out: &mut [f32],
        grain: usize,
        fill: &(dyn Fn(usize, &mut [f32]) + Sync),
    ) {
        let g = grain.max(1);
        if self.workers <= 1 || out.len() <= g {
            // Un seul fil : le chemin séquentiel, sans le prix d'un `scope`.
            let mut start = 0usize;
            while start < out.len() {
                let end = (start + g).min(out.len());
                fill(start, &mut out[start..end]);
                start = end;
            }
            return;
        }
        // Les tranches sont disjointes : `chunks_mut` le prouve au compilateur, et le découpage
        // ne change aucun résultat (chaque élément est écrit une fois, depuis des lectures seules).
        // Aucun tampon intermédiaire : chaque fil reçoit une **portion contiguë**, multiple du
        // grain, qu'il parcourt lui-même par tranches de `g`.
        let slices = out.len().div_ceil(g);
        let span = slices.div_ceil(self.workers as usize).max(1) * g;
        std::thread::scope(|scope| {
            for (w, share) in out.chunks_mut(span).enumerate() {
                let base = w * span;
                scope.spawn(move || {
                    let mut start = 0usize;
                    while start < share.len() {
                        let end = (start + g).min(share.len());
                        fill(base + start, &mut share[start..end]);
                        start = end;
                    }
                });
            }
        });
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

    /// S243, SPEC-004 §8.2 — l'écriture disjointe rend **les mêmes bits** quel que soit le nombre
    /// de fils **et** quel que soit le grain. C'est la phrase que `SequentialJobs` annonçait depuis
    /// S20, et elle est ici plus forte que pour la réduction : rien ne s'accumule entre tâches.
    #[test]
    fn l_ecriture_disjointe_ne_depend_ni_du_grain_ni_du_nombre_de_fils_s243() {
        // Une charge dont chaque élément dépend de son seul indice, et dont le calcul est assez
        // tordu pour qu'un ordre différent se verrait : trois arrondis f32 enchaînés.
        let fill = |offset: usize, slice: &mut [f32]| {
            for (i, cell) in slice.iter_mut().enumerate() {
                let x = (offset + i) as f32 * 0.1;
                *cell = ((x * x + 1e16) - 1e16) + x.sin() * 1e-7;
            }
        };
        let n = 10_000;
        let mut reference = vec![0f32; n];
        SequentialJobs.parallel_fill_f32(&mut reference, 1, &fill);
        for workers in [1u32, 2, 3, 4, 8, 16] {
            for grain in [1usize, 2, 7, 64, 997, 10_000, 20_000] {
                let mut out = vec![0f32; n];
                ScopedJobs::with_workers(workers).parallel_fill_f32(&mut out, grain, &fill);
                for (k, (a, b)) in out.iter().zip(&reference).enumerate() {
                    assert_eq!(
                        a.to_bits(),
                        b.to_bits(),
                        "élément {k} : {workers} fils, grain {grain}"
                    );
                }
            }
        }
        // Le défaut du trait est la référence : un hôte qui ne surcharge rien la rend déjà.
        struct Nu;
        impl JobSystem for Nu {
            fn worker_count(&self) -> u32 { 1 }
            fn parallel_reduce_ordered_f64(&self, _: usize, _: usize,
                _: &dyn Fn(usize, usize) -> f64, _: &dyn Fn(f64, f64) -> f64, init: f64) -> f64 { init }
        }
        let mut out = vec![0f32; n];
        Nu.parallel_fill_f32(&mut out, 333, &fill);
        assert!(out.iter().zip(&reference).all(|(a, b)| a.to_bits() == b.to_bits()));
    }

    /// Les fils travaillent vraiment : chaque tranche est remplie une fois et une seule.
    #[test]
    fn chaque_tranche_est_ecrite_une_fois_et_une_seule_s243() {
        let n = 5_003usize;
        let mut out = vec![f32::NAN; n];
        ScopedJobs::with_workers(8).parallel_fill_f32(&mut out, 16, &|offset, slice| {
            for (i, cell) in slice.iter_mut().enumerate() {
                *cell = (offset + i) as f32;
            }
        });
        assert!(out.iter().enumerate().all(|(i, v)| *v == i as f32));
        assert!(ScopedJobs::detected().worker_count() >= 1);
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

#[cfg(test)]
mod cout_des_fils {
    use super::*;
    use std::time::Instant;

    /// S244 — **prix d'un appel parallèle**, isolé de tout travail utile. La question qui décide du
    /// lot : une pass de δ vaut 21,7 µs à 2 048 mailles ; si créer les fils coûte davantage, la
    /// primitive de S243 ne peut pas servir une boucle aussi fine.
    #[test]
    #[ignore = "mesure S244, lancée explicitement ; à lancer en release"]
    fn prix_d_un_appel_parallele_s244() {
        let median = |mut v: Vec<f64>| {
            v.sort_by(f64::total_cmp);
            v[v.len() / 2]
        };
        for n in [2_048usize, 16_384, 262_144] {
            let mut out = vec![0f32; n];
            for workers in [1u32, 2, 4, 8] {
                let jobs = ScopedJobs::with_workers(workers);
                let grain = (n / workers.max(1) as usize).max(1);
                // Travail volontairement trivial : ce qu'on mesure est l'appel, pas le calcul.
                let fill = |offset: usize, slice: &mut [f32]| {
                    for (i, c) in slice.iter_mut().enumerate() {
                        *c = (offset + i) as f32;
                    }
                };
                let mut us = Vec::new();
                for _ in 0..41 {
                    let start = Instant::now();
                    jobs.parallel_fill_f32(&mut out, grain, &fill);
                    us.push(start.elapsed().as_secs_f64() * 1e6);
                }
                println!(
                    "PRIX_APPEL_S244 elements={n} fils={workers} median_us={:.2}",
                    median(us)
                );
            }
        }
    }
}
