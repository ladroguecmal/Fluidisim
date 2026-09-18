//! S278, [ADR-012](../../../docs/adr/ADR-012-ordonnanceur-budget-degradation.md) — **l'ordonnanceur** :
//! ce qui décide qu'une zone est simulée, laissée à la haute mer analytique, ou tenue en
//! transition.
//!
//! Le système décrit ici est conçu depuis S01 et n'avait jamais été écrit. La bande δ que
//! l'afficheur montre depuis S275 est câblée en dur : rien ne la décide, ne la déplace ni ne
//! l'éteint. C'est exactement la pièce que ce module commence.
//!
//! # La forme du problème
//!
//! ADR-012 §1 la tranche en cinq lignes : **l'activation des domaines est un sac à dos sous
//! budget, résolu chaque pas**. Il n'y a ni arbre de décision distribué ni règle locale — il y a
//! des **candidats qui soumissionnent** `(priorité P, coût C)` et un ordonnanceur qui alloue par
//! `P/C` décroissant jusqu'à épuisement, puis distribue à chaque retenu son `budget_ms`.
//!
//! La sûreté ne vient pas de l'ordonnanceur seul : elle vient de ce que **chaque solveur respecte
//! le budget qu'il reçoit** (contrat d'ADR-007, écrit ici par `delta_budget`). C'est ce qui ferme
//! tout chemin par lequel l'eau provoquerait un pic d'image, et c'est la propriété à défendre
//! avant toutes les autres.
//!
//! # Ce que ce module ne fait pas
//!
//! Il décide **qu'un** domaine vit et avec quel budget. Il ne décide pas encore de sa **forme** :
//! grille de référence, blocs épars, fusion et séparation géométriques (liste 1.5 et 1.6), les
//! sept rangs de dégradation d'ADR-012 §4, le régime substitutif et sa restauration depuis graine.
//!
//! # Ce que l'hôte fournit, et qu'on ne calcule pas ici
//!
//! `W_gameplay` vient du jeu, `W_perception` du rendu — **surface à l'écran**, jamais distance
//! (ADR-012 §2) — et `C` est mesuré en continu par le solveur, jamais théorique.
use crate::host::HostServices;

/// Identité stable d'un domaine entre deux pas. Sert aussi à départager deux candidats de même
/// rapport `P/C` : sans elle, la décision dépendrait de l'ordre de soumission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainId(pub u32);

/// ADR-001 §3.3. Le perturbatif ajoute son écart à `B+W` ; le substitutif devient propriétaire du
/// champ total dans son emprise. Porté ici parce que les deux ne se dégradent pas de la même
/// façon : détruire un perturbatif est gratuit (I-12), un substitutif ne renaît pas gratuitement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Regime {
    Perturbative,
    Substitutive,
}

/// Profil de ressources, déclaré et non découvert (ADR-012 §3). **Des ressources seulement** :
/// une capacité dérivée inscrite dans un profil finit par contredire les ressources qui
/// l'entourent — c'est l'écart R04 de S05, qui portait sur `domaines_max`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Profile {
    /// Budget de simulation par pas, hors fil de rendu.
    pub cpu_sim_ms: f32,
    /// Blocs disponibles dans le pool. Le pool est préalloué : le battement coûte des pointeurs,
    /// jamais de la mémoire (ADR-006 §4).
    pub blocks: u32,
}

/// Ce qu'un candidat soumissionne à chaque pas — domaine déjà vivant ou simple prétendant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bid {
    pub id: DomainId,
    /// `W_gameplay` : conséquence sur un acteur, un objet joueur, un objectif. Domine le reste.
    pub gameplay: f32,
    /// `W_perception` : fraction d'écran × visibilité × facteur de regard.
    pub perception: f32,
    /// `W_urgence` : `1/(temps avant que l'absence du domaine devienne visible)`.
    pub urgency: f32,
    /// `C` — coût estimé du pas, **mesuré** par le solveur et réinjecté ici.
    pub cost_ms: f32,
    pub blocks: u32,
    pub regime: Regime,
}

impl Bid {
    /// `P = W_gameplay · W_perception · W_urgence` (ADR-012 §2).
    pub fn priority(&self) -> f32 {
        self.gameplay * self.perception * self.urgency
    }
}

/// Ce qu'un domaine retenu reçoit : le droit de vivre ce pas, et son budget.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Grant {
    pub id: DomainId,
    pub budget_ms: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Poids, coût ou budget non fini, ou négatif.
    NotFinite,
    /// Plus de candidats que le pool n'en accepte.
    Capacity,
    /// Deux soumissions portent la même identité dans le même pas.
    Duplicate,
}

/// L'ordonnanceur. Sa capacité est fixée à la construction et sa mémoire demandée à l'hôte :
/// aucune allocation à l'exécution (I-06).
pub struct Scheduler {
    profile: Profile,
    bids: Vec<Bid>,
    grants: Vec<Grant>,
}

impl Scheduler {
    pub fn with_capacity(
        host: &mut HostServices,
        profile: Profile,
        capacity: usize,
    ) -> Result<Self, Error> {
        if !(profile.cpu_sim_ms.is_finite() && profile.cpu_sim_ms > 0.) {
            return Err(Error::NotFinite);
        }
        let bytes = capacity * (core::mem::size_of::<Bid>() + core::mem::size_of::<Grant>());
        host.alloc.alloc_persistent(bytes).map_err(|_| Error::Capacity)?;
        Ok(Self {
            profile,
            bids: Vec::with_capacity(capacity),
            grants: Vec::with_capacity(capacity),
        })
    }

    pub fn profile(&self) -> Profile {
        self.profile
    }

    pub fn capacity(&self) -> usize {
        self.bids.capacity()
    }

    /// Ouvre un pas : les soumissions précédentes ne valent plus rien. Ne rend aucune mémoire.
    pub fn begin(&mut self) {
        self.bids.clear();
        self.grants.clear();
    }

    /// Soumissionne. Refuse tout poids non fini ou négatif : un `NaN` qui remonterait jusqu'au tri
    /// y vaudrait n'importe quel rang, et le plus dangereux est **le meilleur** (L161).
    pub fn submit(&mut self, bid: Bid) -> Result<(), Error> {
        let fini = |v: f32| v.is_finite() && v >= 0.;
        if !(fini(bid.gameplay) && fini(bid.perception) && fini(bid.urgency) && fini(bid.cost_ms)) {
            return Err(Error::NotFinite);
        }
        if self.bids.iter().any(|b| b.id == bid.id) {
            return Err(Error::Duplicate);
        }
        if self.bids.len() == self.bids.capacity() {
            return Err(Error::Capacity);
        }
        self.bids.push(bid);
        Ok(())
    }

    pub fn bids(&self) -> &[Bid] {
        &self.bids
    }

    pub fn grants(&self) -> &[Grant] {
        &self.grants
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};

    /// Hôte d'essai : l'allocation est bornée pour que le refus se teste, et rien d'autre ne sert.
    struct Hote {
        used: core::cell::Cell<usize>,
        limit: usize,
    }
    impl Allocator for Hote {
        fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AllocError> {
            if self.used.get() + bytes > self.limit {
                return Err(AllocError::OutOfArena);
            }
            self.used.set(self.used.get() + bytes);
            Ok(bytes)
        }
        fn seal(&mut self) {}
        fn is_sealed(&self) -> bool {
            false
        }
        fn stats(&self) -> AllocStats {
            AllocStats::default()
        }
    }
    impl Sink for Hote {
        fn warn(&self, _: &str) {}
        fn metric(&self, _: &str, _: f64) {}
    }
    impl JobSystem for Hote {
        fn worker_count(&self) -> u32 {
            1
        }
        fn parallel_reduce_ordered_f64(&self, n: usize, _: usize,
            reduce: &dyn Fn(usize, usize) -> f64, merge: &dyn Fn(f64, f64) -> f64,
            init: f64) -> f64 {
            merge(init, reduce(0, n))
        }
    }

    fn bid(id: u32, g: f32, p: f32, u: f32, cost: f32) -> Bid {
        Bid {
            id: DomainId(id),
            gameplay: g,
            perception: p,
            urgency: u,
            cost_ms: cost,
            blocks: 1,
            regime: Regime::Perturbative,
        }
    }

    fn scheduler(capacity: usize) -> Scheduler {
        let mut alloc = Hote { used: core::cell::Cell::new(0), limit: 1 << 16 };
        let services = Hote { used: core::cell::Cell::new(0), limit: 0 };
        let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
        Scheduler::with_capacity(&mut host, Profile { cpu_sim_ms: 2., blocks: 64 }, capacity).unwrap()
    }

    #[test]
    fn la_priorite_est_le_produit_des_trois_poids_s278() {
        assert_eq!(bid(1, 0.5, 0.4, 2., 1.).priority(), 0.4);
        // Un gameplay nul annule tout : une eau qui ne touche personne et qu'on ne voit pas.
        assert_eq!(bid(2, 0., 1., 1., 1.).priority(), 0.);
    }

    #[test]
    fn un_poids_non_fini_est_refuse_avant_le_tri_s278() {
        let mut s = scheduler(4);
        s.begin();
        assert_eq!(s.submit(bid(1, f32::NAN, 1., 1., 1.)), Err(Error::NotFinite));
        assert_eq!(s.submit(bid(2, 1., 1., 1., f32::INFINITY)), Err(Error::NotFinite));
        assert_eq!(s.submit(bid(3, -1., 1., 1., 1.)), Err(Error::NotFinite));
        assert!(s.bids().is_empty(), "un refus ne laisse rien derrière lui");
    }

    #[test]
    fn la_capacite_est_fixee_a_la_construction_s278() {
        let mut s = scheduler(2);
        s.begin();
        assert!(s.submit(bid(1, 1., 1., 1., 1.)).is_ok());
        assert!(s.submit(bid(2, 1., 1., 1., 1.)).is_ok());
        assert_eq!(s.submit(bid(3, 1., 1., 1., 1.)), Err(Error::Capacity));
        // Le pas suivant retrouve la place : `begin` vide sans rendre la mémoire.
        s.begin();
        assert_eq!(s.capacity(), 2);
        assert!(s.submit(bid(3, 1., 1., 1., 1.)).is_ok());
    }

    #[test]
    fn deux_soumissions_de_meme_identite_sont_refusees_s278() {
        let mut s = scheduler(4);
        s.begin();
        assert!(s.submit(bid(7, 1., 1., 1., 1.)).is_ok());
        assert_eq!(s.submit(bid(7, 2., 1., 1., 1.)), Err(Error::Duplicate));
    }
}
