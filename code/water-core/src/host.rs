//! Services d'hôte — SPEC-004 §8, ADR-020.
//!
//! Le cœur ne connaît ni le moteur, ni le système de fichiers, ni l'horloge. Tout ce dont il a
//! besoin lui est **injecté**. H1 n'en implémente que trois des six : `IGpuBackend` est nul en mode
//! `check` (SPEC-004 §8), et les fournisseurs de bathymétrie et d'échantillons hydrographiques
//! n'ont pas d'objet tant que `B` est seul.

/// Journal, métriques et compteurs — SPEC-004 §8.4. Ne renvoie rien, ne bloque jamais.
pub trait Sink {
    fn warn(&self, message: &str);
    fn metric(&self, name: &str, value: f64);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AllocStats {
    /// Octets alloués pendant la phase d'initialisation.
    pub persistent_bytes: usize,
    /// Nombre d'allocations persistantes.
    pub persistent_calls: u32,
    /// Tentatives d'allocation **après** `seal()`. Doit valoir zéro — I-06.
    pub refused_after_seal: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AllocError {
    /// `seal()` a été appelé : l'allocation générale n'est plus possible. SPEC-004 §8.1.
    Sealed,
    /// L'arène du profil est pleine.
    OutOfArena,
}

/// Allocateur d'hôte — SPEC-004 §8.1.
///
/// `seal()` transforme l'invariant I-06 en **propriété mécanique** : après scellement, toute
/// allocation générale **échoue** au lieu d'être comptée. Le défaut ne peut plus se glisser en
/// production derrière un compteur que personne ne regarde.
pub trait Allocator {
    fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AllocError>;
    fn seal(&mut self);
    fn is_sealed(&self) -> bool;
    fn stats(&self) -> AllocStats;
}

/// Système de tâches — SPEC-004 §8.2.
///
/// `parallel_reduce_ordered` fusionne les résultats partiels dans **l'ordre des indices**, jamais
/// dans l'ordre d'arrivée. C'est la seule façon d'atteindre le régime D2, et donc la seule façon de
/// reproduire un bogue qui survient une fois sur cinquante. **Toute accumulation flottante du
/// système passe par cette primitive.**
///
/// Corollaire imposé : *changer `worker_count` change la vitesse, jamais le résultat.* H1 fournit
/// une implémentation séquentielle qui est, par construction, la référence de cet ordre.
pub trait JobSystem {
    fn worker_count(&self) -> u32;

    /// Découpe `n` éléments en tranches de `grain`, applique `reduce` à chacune, puis fusionne les
    /// résultats **dans l'ordre croissant des tranches**.
    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        grain: usize,
        reduce: &dyn Fn(usize, usize) -> f64,
        merge: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64;
}

/// Agrégat des services d'hôte — SPEC-004 §8.
///
/// `gpu` est absent de H1 : le mode `check` ne demande ni GPU ni rendu (SPEC-003 §4), et
/// SPEC-004 §8 prévoit explicitement que le service soit nul pour un hôte serveur ou un harnais.
pub struct HostServices<'a> {
    pub alloc: &'a mut dyn Allocator,
    pub jobs: &'a dyn JobSystem,
    pub sink: &'a dyn Sink,
}
