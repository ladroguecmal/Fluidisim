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
use crate::types::SimTime;

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
    /// Seuils d'allumage et d'extinction (ADR-171). `Profile::default_thresholds` rend ceux
    /// d'ADR-013 §5 ; un hôte qui les change doit dire sur quelle mesure.
    pub on: f32,
    pub off: f32,
}

impl Profile {
    /// Les seuils de départ d'ADR-013 §5, pour un profil qui n'a rien calibré.
    pub fn default_thresholds(cpu_sim_ms: f32, blocks: u32) -> Self {
        Self { cpu_sim_ms, blocks, on: ON, off: OFF }
    }
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

/// Hystérésis d'ADR-013 §5 : on allume au-dessus de `on`, on éteint en dessous de `off`, et entre
/// les deux **on ne change rien**. C'est l'intervalle qui empêche le battement, pas les bornes.
///
/// Ces deux constantes sont les **valeurs de départ** d'ADR-013 §5, gardées comme défaut. S279 les
/// a éprouvées pour la première fois et elles n'ont pas tenu : la priorité étant un produit de
/// trois fractions, elle est toujours inférieure à la plus petite, et un domaine occupant 55 % du
/// cadre n'atteint jamais 0,60. **Les seuils appartiennent donc au profil** (ADR-171) : chaque hôte
/// calibre les siens et écrit sur quoi.
pub const ON: f32 = 0.60;
pub const OFF: f32 = 0.40;

/// Durée de vie minimale d'un domaine (ADR-013 §5) : au-delà du temps de réaction du joueur, en
/// deçà de sa mémoire perceptuelle. Rien ne meurt avant, quel que soit son score.
pub const LIFETIME_US: u64 = 750_000;
/// Délai avant extinction une fois le score passé sous `OFF` (ADR-013 §5). Remonter au-dessus
/// d'`OFF` l'efface : c'est un séjour continu qui tue, pas un passage.
pub const OFF_DELAY_US: u64 = 1_000_000;

/// L'état d'un domaine d'un pas à l'autre. C'est la seule mémoire de l'ordonnanceur : sans elle,
/// l'hystérésis n'existe pas, puisqu'elle porte sur la décision précédente.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Live {
    id: DomainId,
    active: bool,
    /// Instant d'allumage : porte la durée de vie minimale.
    born_us: u64,
    /// Depuis quand le score est continûment sous `OFF`. `None` dès qu'il repasse au-dessus.
    below_since_us: Option<u64>,
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
    /// Le temps fourni est antérieur à celui du pas précédent. L'horloge n'est jamais implicite,
    /// et une horloge qui recule ferait vivre éternellement ce qui devrait mourir.
    Clock,
}

/// L'ordonnanceur. Sa capacité est fixée à la construction et sa mémoire demandée à l'hôte :
/// aucune allocation à l'exécution (I-06).
pub struct Scheduler {
    profile: Profile,
    bids: Vec<Bid>,
    grants: Vec<Grant>,
    live: Vec<Live>,
    last_us: Option<u64>,
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
        // Un intervalle inversé ou nul supprimerait l'hystérésis sans le dire, et le battement
        // reviendrait par une porte que personne ne surveille.
        if !(profile.off.is_finite() && profile.on.is_finite() && profile.off < profile.on) {
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
            last_us: None,
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

    /// Décision d'un pas : allumage au-dessus de `ON`, extinction sous `OFF` **et** seulement
    /// une fois les deux délais tenus — durée de vie minimale depuis l'allumage, séjour continu
    /// sous le seuil. Entre les deux seuils, rien ne change.
    ///
    /// **Ne pas soumissionner vaut un score nul**, pas une mort immédiate : une source qui
    /// clignote ferait battre son domaine, et le battement est plus visible que ce qu'il évite.
    ///
    /// Le budget n'intervient pas ici : ce que rend cette étape est l'ensemble des domaines qui
    /// *veulent* vivre, pas celui qui vivra — c'est `allocate` qui tranche.
    pub fn decide(&mut self, now: SimTime) -> Result<(), Error> {
        if self.last_us.is_some_and(|t| now.0 < t) {
            return Err(Error::Clock);
        }
        self.last_us = Some(now.0);
        for l in self.live.iter_mut() {
            let s = self.bids.iter().find(|b| b.id == l.id).map_or(0., |b| b.priority());
            if s < self.profile.off {
                l.below_since_us.get_or_insert(now.0);
            } else {
                l.below_since_us = None;
            }
            let vecu = now.0.saturating_sub(l.born_us) >= LIFETIME_US;
            let bas = l.below_since_us.is_some_and(|t| now.0.saturating_sub(t) >= OFF_DELAY_US);
            if vecu && bas {
                l.active = false;
            }
        }
        for b in &self.bids {
            // Inconnu : il n'entre dans la mémoire que s'il franchit le seuil d'allumage.
            if b.priority() > self.profile.on && !self.live.iter().any(|l| l.id == b.id) {
                self.live.push(Live { id: b.id, active: true, born_us: now.0, below_since_us: None });
            }
        }
        self.live.retain(|l| l.active);
        Ok(())
    }

    /// Le sac à dos d'ADR-012 §1 : trier les vivants par `P/C` décroissant, allouer jusqu'à
    /// épuisement du budget, donner à chaque retenu son `budget_ms`.
    ///
    /// **Le budget donné est le coût annoncé**, pas une part du reliquat. Un budget est une
    /// **borne**, pas une enveloppe à consommer : la somme des bornes ne dépasse jamais le profil,
    /// et c'est ce qui ferme tout chemin par lequel l'eau provoquerait un pic d'image. Un solveur
    /// qui déborde son estimation est coupé à sa borne (contrat d'ADR-007) ; ce qui reste non
    /// distribué est une marge, pas une perte.
    ///
    /// **Le rapport se compare en croix**, `P_a·C_b` contre `P_b·C_a` : aucune division, donc rien
    /// à décider pour un coût nul, et l'égalité se départage par identité — sans quoi la décision
    /// dépendrait de l'ordre de soumission.
    ///
    /// Un candidat trop gros est **sauté**, pas bloquant : les suivants remplissent le reliquat.
    /// Un gros domaine peut donc jeûner tant que de petits se présentent — limite connue, à
    /// éprouver au banc B8, qui n'existe pas.
    pub fn allocate(&mut self) {
        self.grants.clear();
        self.bids.sort_unstable_by(|a, b| {
            (b.priority() * a.cost_ms)
                .total_cmp(&(a.priority() * b.cost_ms))
                .then(a.id.cmp(&b.id))
        });
        let (mut ms, mut blocks) = (0f32, 0u32);
        for bid in &self.bids {
            if !self.live.iter().any(|l| l.id == bid.id && l.active) {
                continue;
            }
            if ms + bid.cost_ms > self.profile.cpu_sim_ms || blocks + bid.blocks > self.profile.blocks {
                continue;
            }
            ms += bid.cost_ms;
            blocks += bid.blocks;
            self.grants.push(Grant { id: bid.id, budget_ms: bid.cost_ms });
        }
    }

    /// Somme des budgets distribués. Ne dépasse jamais `profile.cpu_sim_ms` : c'est la propriété
    /// que le banc doit défendre en premier.
    pub fn granted_ms(&self) -> f32 {
        self.grants.iter().map(|g| g.budget_ms).sum()
    }

    /// Oublie tout état : plus aucun domaine vivant, plus d'horloge. C'est ce qu'un hôte appelle
    /// quand son temps recule — retour au début d'une scène, chargement — plutôt que de subir le
    /// refus de `decide`. Ne rend aucune mémoire.
    pub fn rewind(&mut self) {
        self.live.clear();
        self.grants.clear();
        self.last_us = None;
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
        Scheduler::with_capacity(&mut host, Profile::default_thresholds(2., 64), capacity).unwrap()
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
        // Deux secondes par pas : au-delà des deux délais, donc l'essai ne teste que l'hystérésis.
        let mut t = 0u64;
        let pas = |s: &mut Scheduler, t: &mut u64, score: f32| {
            *t += 2_000_000;
            s.begin();
            s.submit(bid(1, score, 1., 1., 1.)).unwrap();
            s.decide(SimTime(*t)).unwrap();
            s.is_active(DomainId(1))
        };
        // Sous le seuil d'allumage, rien ne s'allume — même à un cheveu.
        assert!(!pas(&mut s, &mut t, 0.59));
        assert!(pas(&mut s, &mut t, 0.61), "franchi ON, le domaine vit");
        // Entre OFF et ON, dix oscillations ne changent rien : c'est exactement le battement
        // qu'un seuil unique produirait.
        for score in [0.45, 0.55, 0.41, 0.59, 0.42, 0.58, 0.44, 0.56, 0.43, 0.57] {
            assert!(pas(&mut s, &mut t, score), "le domaine ne doit pas s'éteindre dans l'intervalle");
        }
        // Passer sous `OFF` ne tue pas : il faut y séjourner. Le premier pas ouvre le compte.
        assert!(pas(&mut s, &mut t, 0.39), "le délai d'extinction n'est pas encore écoulé");
        assert!(!pas(&mut s, &mut t, 0.39), "après le délai, il s'éteint");
        // Et il ne se rallume pas en remontant dans l'intervalle : l'hystérésis est symétrique.
        for score in [0.41, 0.5, 0.59] {
            assert!(!pas(&mut s, &mut t, score));
        }
        assert!(pas(&mut s, &mut t, 0.61));
    }

    /// Ne pas soumissionner, c'est renoncer : un domaine qu'on ne revoit pas s'éteint, sans quoi
    /// une source disparue laisserait son domaine vivre indéfiniment.
    #[test]
    fn un_candidat_qui_ne_soumissionne_plus_s_eteint_s278() {
        let mut s = scheduler(4);
        s.begin();
        s.submit(bid(1, 0.9, 1., 1., 1.)).unwrap();
        s.submit(bid(2, 0.9, 1., 1., 1.)).unwrap();
        s.decide(SimTime(0)).unwrap();
        assert_eq!(s.active().count(), 2);
        // Il cesse de soumissionner : son score vaut zéro, mais il ne meurt qu'au bout des délais.
        s.begin();
        s.submit(bid(1, 0.9, 1., 1., 1.)).unwrap();
        s.decide(SimTime(900_000)).unwrap();
        assert!(s.is_active(DomainId(2)), "ni la durée de vie ni le délai ne sont tenus");
        s.begin();
        s.submit(bid(1, 0.9, 1., 1., 1.)).unwrap();
        // Le compte a commencé à 900 ms, quand il a cessé de soumissionner : il meurt une
        // seconde plus tard, pas une seconde après sa dernière soumission.
        s.decide(SimTime(1_900_000)).unwrap();
        assert!(s.is_active(DomainId(1)) && !s.is_active(DomainId(2)));
    }

    /// Rien ne meurt avant sa durée de vie minimale, même tombé à zéro aussitôt né (ADR-013 §5).
    #[test]
    fn rien_ne_meurt_avant_sa_duree_de_vie_s278() {
        let mut s = scheduler(4);
        s.begin();
        s.submit(bid(1, 0.9, 1., 1., 0.5)).unwrap();
        s.decide(SimTime(0)).unwrap();
        // Score nul dès le pas suivant : le délai d'extinction court, la durée de vie aussi.
        for t in [100_000u64, 500_000, 740_000] {
            s.begin();
            s.submit(bid(1, 0., 1., 1., 0.5)).unwrap();
            s.decide(SimTime(t)).unwrap();
            assert!(s.is_active(DomainId(1)), "mort à {t} µs, avant ses 750 ms");
        }
        // Le séjour sous le seuil a commencé à 100 ms — au premier pas à score nul, pas à la
        // naissance. Il meurt donc à 1,1 s, et pas avant.
        s.begin();
        s.submit(bid(1, 0., 1., 1., 0.5)).unwrap();
        s.decide(SimTime(1_099_000)).unwrap();
        assert!(s.is_active(DomainId(1)), "le délai court depuis la chute, pas depuis la naissance");
        s.begin();
        s.submit(bid(1, 0., 1., 1., 0.5)).unwrap();
        s.decide(SimTime(1_100_000)).unwrap();
        assert!(!s.is_active(DomainId(1)));
    }

    /// Un score qui replonge sous `OFF` puis remonte ne cumule pas : c'est un séjour **continu**
    /// qui tue. Sans cette remise à zéro, un domaine sain finirait par mourir de vieux passages.
    #[test]
    fn le_delai_d_extinction_se_remet_a_zero_s278() {
        let mut s = scheduler(4);
        let mut jouer = |s: &mut Scheduler, t: u64, score: f32| {
            s.begin();
            s.submit(bid(1, score, 1., 1., 0.5)).unwrap();
            s.decide(SimTime(t)).unwrap();
        };
        jouer(&mut s, 0, 0.9);
        // Neuf dixièmes de seconde sous le seuil, puis une remontée, puis neuf dixièmes encore :
        // 1,8 s cumulées sous `OFF`, et pourtant il vit.
        jouer(&mut s, 900_000, 0.1);
        jouer(&mut s, 1_000_000, 0.9);
        jouer(&mut s, 1_900_000, 0.1);
        assert!(s.is_active(DomainId(1)), "le cumul ne doit pas tuer");
        jouer(&mut s, 2_950_000, 0.1);
        assert!(!s.is_active(DomainId(1)), "un séjour continu de plus d'une seconde, lui, tue");
    }

    /// L'horloge n'est jamais implicite : un temps qui recule est refusé plutôt que subi.
    #[test]
    fn une_horloge_qui_recule_est_refusee_s278() {
        let mut s = scheduler(4);
        s.begin();
        s.submit(bid(1, 0.9, 1., 1., 0.5)).unwrap();
        assert!(s.decide(SimTime(2_000_000)).is_ok());
        s.begin();
        s.submit(bid(1, 0.9, 1., 1., 0.5)).unwrap();
        assert_eq!(s.decide(SimTime(1_999_999)), Err(Error::Clock));
    }

    /// La propriété à défendre avant toutes les autres (ADR-012 §1) : la somme des budgets
    /// distribués ne dépasse jamais le profil, quelle que soit la demande.
    #[test]
    fn le_budget_n_est_jamais_depasse_s278() {
        let mut s = scheduler(8);
        s.begin();
        // Six domaines qui demandent 0,8 ms chacun, soit 4,8 ms pour un budget de 2 ms.
        for i in 0..6 {
            s.submit(bid(i, 0.9, 1., 1., 0.8)).unwrap();
        }
        s.decide(SimTime(9_000_000)).unwrap();
        s.allocate();
        assert_eq!(s.active().count(), 6, "les six veulent vivre");
        assert_eq!(s.grants().len(), 2, "deux seulement sont financés");
        assert!(s.granted_ms() <= s.profile().cpu_sim_ms, "{} ms", s.granted_ms());
    }

    /// Le tri d'ADR-012 §1 est par `P/C`, pas par `P` : un domaine deux fois moins prioritaire mais
    /// dix fois moins cher passe devant. C'est tout l'intérêt du sac à dos.
    #[test]
    fn le_meilleur_rapport_passe_devant_la_meilleure_priorite_s278() {
        let mut s = scheduler(4);
        s.begin();
        s.submit(bid(1, 1., 1., 1., 1.75)).unwrap();  // P = 1,00  C = 1,75  → 0,57
        s.submit(bid(2, 0.7, 1., 1., 0.25)).unwrap(); // P = 0,70  C = 0,25  → 2,80
        s.decide(SimTime(9_000_000)).unwrap();
        s.allocate();
        assert_eq!(s.grants()[0].id, DomainId(2), "le rapport commande");
        // Et le budget restant sert au plus cher : 0,25 + 1,75 = 2,0 ms, exactement le profil.
        assert_eq!(s.grants().len(), 2);
        assert_eq!(s.granted_ms(), 2.);
    }

    /// Le seuil d'activation est **absolu**, et le rapport ne le rachète pas : un candidat presque
    /// gratuit mais sous `ON` ne s'allume pas. Le score dit qu'un domaine mérite d'exister, le
    /// rapport dit seulement dans quel ordre on sert ceux qui le méritent.
    #[test]
    fn un_rapport_excellent_ne_rachete_pas_un_score_trop_bas_s278() {
        let mut s = scheduler(4);
        s.begin();
        s.submit(bid(1, 0.5, 1., 1., 0.01)).unwrap(); // rapport 50, score 0,50 < ON
        s.decide(SimTime(9_000_000)).unwrap();
        s.allocate();
        assert!(!s.is_active(DomainId(1)));
        assert!(s.grants().is_empty());
    }

    /// Un candidat trop gros est sauté, et les suivants remplissent le reliquat plutôt que de le
    /// perdre.
    #[test]
    fn un_candidat_trop_gros_ne_bloque_pas_la_file_s278() {
        let mut s = scheduler(4);
        s.begin();
        s.submit(bid(1, 1., 1., 1., 3.0)).unwrap(); // seul, il dépasse le budget de 2 ms
        s.submit(bid(2, 0.9, 1., 1., 0.5)).unwrap();
        s.decide(SimTime(9_000_000)).unwrap();
        s.allocate();
        assert_eq!(s.grants().len(), 1);
        assert_eq!(s.grants()[0].id, DomainId(2));
    }

    /// L'ordre de soumission ne doit rien changer : deux candidats de même rapport se départagent
    /// par identité, jamais par l'ordre d'arrivée (I-03).
    #[test]
    fn l_ordre_de_soumission_ne_change_pas_la_decision_s278() {
        let decision = |ordre: [u32; 3]| {
            let mut s = scheduler(4);
            s.begin();
            for id in ordre {
                s.submit(bid(id, 0.9, 1., 1., 0.5)).unwrap();
            }
            s.decide(SimTime(9_000_000)).unwrap();
            s.allocate();
            s.grants().iter().map(|g| g.id).collect::<Vec<_>>()
        };
        assert_eq!(decision([1, 2, 3]), decision([3, 1, 2]));
        assert_eq!(decision([1, 2, 3]), decision([2, 3, 1]));
    }

    /// Un domaine vivant mais non financé ne s'éteint pas : l'hystérésis porte sur le score, pas
    /// sur le budget. Ce qu'il devient après plusieurs pas sans budget relève de la dégradation
    /// (ADR-012 §4), qui n'est pas écrite.
    #[test]
    fn un_vivant_non_finance_reste_vivant_s278() {
        let mut s = scheduler(4);
        s.begin();
        s.submit(bid(1, 1., 1., 1., 1.5)).unwrap();
        s.submit(bid(2, 0.9, 1., 1., 1.5)).unwrap();
        s.decide(SimTime(9_000_000)).unwrap();
        s.allocate();
        assert_eq!(s.grants().len(), 1);
        assert!(s.is_active(DomainId(1)) && s.is_active(DomainId(2)));
    }

    /// ADR-171 : les seuils viennent du profil. Un score de 0,55 — celui d'une bande qui occupe
    /// la moitié du cadre — reste éteint aux valeurs de départ et vit aux seuils calibrés.
    #[test]
    fn les_seuils_viennent_du_profil_s279() {
        let essai = |on: f32, off: f32| {
            let mut alloc = Hote { used: core::cell::Cell::new(0), limit: 1 << 16 };
            let services = Hote { used: core::cell::Cell::new(0), limit: 0 };
            let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
            let profile = Profile { cpu_sim_ms: 2., blocks: 64, on, off };
            let mut s = Scheduler::with_capacity(&mut host, profile, 4).unwrap();
            s.begin();
            s.submit(bid(1, 0.55, 1., 1., 0.5)).unwrap();
            s.decide(SimTime(0)).unwrap();
            s.is_active(DomainId(1))
        };
        assert!(!essai(ON, OFF), "aux valeurs de départ, 0,55 n'allume rien");
        assert!(essai(0.45, 0.35), "aux seuils calibrés de l'afficheur, il vit");
    }

    /// Un intervalle inversé ou nul supprimerait l'hystérésis sans le dire.
    #[test]
    fn un_intervalle_d_hysteresis_vide_est_refuse_s279() {
        let mut alloc = Hote { used: core::cell::Cell::new(0), limit: 1 << 16 };
        let services = Hote { used: core::cell::Cell::new(0), limit: 0 };
        let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
        for (on, off) in [(0.4f32, 0.4f32), (0.3, 0.5), (0.5, f32::NAN)] {
            let profile = Profile { cpu_sim_ms: 2., blocks: 64, on, off };
            assert_eq!(
                Scheduler::with_capacity(&mut host, profile, 4).err(),
                Some(Error::NotFinite),
                "seuils {on} / {off} acceptés"
            );
        }
    }

    #[test]
    fn deux_soumissions_de_meme_identite_sont_refusees_s278() {
        let mut s = scheduler(4);
        s.begin();
        assert!(s.submit(bid(7, 1., 1., 1., 1.)).is_ok());
        assert_eq!(s.submit(bid(7, 0.9, 1., 1., 1.)), Err(Error::Duplicate));
    }
}
