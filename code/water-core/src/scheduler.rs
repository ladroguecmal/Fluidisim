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
    /// `P = W_gameplay · W_perception · W_urgence` (ADR-012 §2). Les trois poids étant dans
    /// `[0,1]` (ADR-170), le produit l'est aussi : c'est **le score d'activation** d'ADR-013 §5,
    /// sans facteur d'échelle.
    pub fn priority(&self) -> f32 {
        self.gameplay * self.perception * self.urgency
    }
}

/// Hystérésis d'ADR-013 §5 : on allume au-dessus de `ON`, on éteint en dessous de `OFF`, et
/// entre les deux **on ne change rien**. C'est l'intervalle qui empêche le battement, pas les
/// bornes ; les valeurs restent celles du départ, toutes à calibrer (banc B8).
pub const ON: f32 = 0.60;
pub const OFF: f32 = 0.40;

/// L'état d'un domaine d'un pas à l'autre. C'est la seule mémoire de l'ordonnanceur : sans elle,
/// l'hystérésis n'existe pas, puisqu'elle porte sur la décision précédente.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Live {
    id: DomainId,
    active: bool,
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
    live: Vec<Live>,
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
        let bytes = capacity
            * (core::mem::size_of::<Bid>() + core::mem::size_of::<Grant>() + core::mem::size_of::<Live>());
        host.alloc.alloc_persistent(bytes).map_err(|_| Error::Capacity)?;
        Ok(Self {
            profile,
            bids: Vec::with_capacity(capacity),
            grants: Vec::with_capacity(capacity),
            live: Vec::with_capacity(capacity),
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
        // ADR-170 : les trois poids sont des fractions. Le coût, lui, est une durée : il est
        // seulement fini et positif.
        let poids = |v: f32| v.is_finite() && (0. ..=1.).contains(&v);
        if !(poids(bid.gameplay) && poids(bid.perception) && poids(bid.urgency)) {
            return Err(Error::NotFinite);
        }
        if !(bid.cost_ms.is_finite() && bid.cost_ms >= 0.) {
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

    /// Décision d'un pas, hystérésis seule : chaque soumission allume au-dessus de `ON`, éteint
    /// en dessous de `OFF`, et **garde son état entre les deux**. Un candidat qu'on ne revoit pas
    /// s'éteint — ne pas soumissionner, c'est renoncer.
    ///
    /// Le budget n'intervient pas encore (P4) : ce que rend cette étape est l'ensemble des
    /// domaines qui *veulent* vivre, pas celui qui vivra.
    pub fn decide(&mut self) {
        for l in self.live.iter_mut() {
            if !self.bids.iter().any(|b| b.id == l.id) {
                l.active = false;
            }
        }
        for b in &self.bids {
            let s = b.priority();
            match self.live.iter_mut().find(|l| l.id == b.id) {
                Some(l) => {
                    if l.active {
                        if s < OFF {
                            l.active = false;
                        }
                    } else if s > ON {
                        l.active = true;
                    }
                }
                // Inconnu : il n'entre dans la mémoire que s'il franchit le seuil d'allumage.
                None if s > ON => self.live.push(Live { id: b.id, active: true }),
                None => {}
            }
        }
        self.live.retain(|l| l.active);
    }

    /// Les domaines que la décision laisse vivants, dans l'ordre où ils se sont allumés.
    pub fn active(&self) -> impl Iterator<Item = DomainId> + '_ {
        self.live.iter().filter(|l| l.active).map(|l| l.id)
    }

    pub fn is_active(&self, id: DomainId) -> bool {
        self.live.iter().any(|l| l.id == id && l.active)
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

    /// Un poids hors de `[0,1]` est refusé (ADR-170) : sans cette borne, les seuils d'ADR-013
    /// n'ont pas de sens, et le défaut ne se voit qu'au moment où quelqu'un calcule.
    #[test]
    fn un_poids_hors_de_zero_un_est_refuse_s278() {
        let mut s = scheduler(4);
        s.begin();
        assert_eq!(s.submit(bid(1, 1.5, 1., 1., 1.)), Err(Error::NotFinite));
        // Le coût est une durée, pas une fraction : au-delà de 1 ms, il reste légitime.
        assert!(s.submit(bid(2, 1., 1., 1., 40.)).is_ok());
    }

    /// L'hystérésis d'ADR-013 §5 : ce qui empêche le battement est l'**intervalle**, pas les
    /// bornes. Un candidat qui oscille dedans garde l'état qu'il avait en y entrant.
    #[test]
    fn un_candidat_qui_oscille_dans_l_intervalle_ne_bat_pas_s278() {
        let mut s = scheduler(4);
        let pas = |s: &mut Scheduler, score: f32| {
            s.begin();
            s.submit(bid(1, score, 1., 1., 1.)).unwrap();
            s.decide();
            s.is_active(DomainId(1))
        };
        // Sous le seuil d'allumage, rien ne s'allume — même à un cheveu.
        assert!(!pas(&mut s, 0.59));
        assert!(pas(&mut s, 0.61), "franchi ON, le domaine vit");
        // Entre OFF et ON, dix oscillations ne changent rien : c'est exactement le battement
        // qu'un seuil unique produirait.
        for score in [0.45, 0.55, 0.41, 0.59, 0.42, 0.58, 0.44, 0.56, 0.43, 0.57] {
            assert!(pas(&mut s, score), "le domaine ne doit pas s'éteindre dans l'intervalle");
        }
        assert!(!pas(&mut s, 0.39), "sous OFF, il s'éteint");
        // Et il ne se rallume pas en remontant dans l'intervalle : l'hystérésis est symétrique.
        for score in [0.41, 0.5, 0.59] {
            assert!(!pas(&mut s, score));
        }
        assert!(pas(&mut s, 0.61));
    }

    /// Ne pas soumissionner, c'est renoncer : un domaine qu'on ne revoit pas s'éteint, sans quoi
    /// une source disparue laisserait son domaine vivre indéfiniment.
    #[test]
    fn un_candidat_qui_ne_soumissionne_plus_s_eteint_s278() {
        let mut s = scheduler(4);
        s.begin();
        s.submit(bid(1, 0.9, 1., 1., 1.)).unwrap();
        s.submit(bid(2, 0.9, 1., 1., 1.)).unwrap();
        s.decide();
        assert_eq!(s.active().count(), 2);
        s.begin();
        s.submit(bid(1, 0.9, 1., 1., 1.)).unwrap();
        s.decide();
        assert!(s.is_active(DomainId(1)) && !s.is_active(DomainId(2)));
    }

    #[test]
    fn deux_soumissions_de_meme_identite_sont_refusees_s278() {
        let mut s = scheduler(4);
        s.begin();
        assert!(s.submit(bid(7, 1., 1., 1., 1.)).is_ok());
        assert_eq!(s.submit(bid(7, 0.9, 1., 1., 1.)), Err(Error::Duplicate));
    }
}
