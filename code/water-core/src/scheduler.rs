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
//! rangs 2 à 7 de dégradation d'ADR-012 §4, le régime substitutif et sa restauration depuis graine.
//!
//! **S351 — le rang 1** d'ADR-012 §4 : quand le budget ne tient pas tous les vivants, le domaine
//! **focal** est servi entier et les autres **rétrécissent** au lieu d'être affamés — s'ils l'ont
//! déclaré (`Shrink`). L'ordonnanceur rend une **échelle** ; l'hôte en fait une emprise.
//!
//! **S403 — le rang 4** : si le rang 1, à son minimum, laisse encore un vivant sans budget, un non-focal
//! qui l'a déclaré (`declare_coarsen`) **descend `dx` d'un niveau** — le transfert d'état d'ADR-210 —,
//! d'abord celui qui **perd le moins par milliseconde rendue** (ADR-210 D2). Ce que ni le rang 1 ni le
//! rang 4 ne servent est **déclaré affamé** (`starved`) : l'issue est le rang 5, le repli sur W, à
//! l'hôte. Les rangs 2 et 3 n'existent pas ; le rang 4 suit donc le rang 1.
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

/// S351, ADR-012 §4 rang 1 — ce qu'un candidat accepte de céder : **rétrécir son emprise**. L'échelle
/// est une fraction de sa surface, dans `[min_scale, 1]`, et son coût la suit :
/// `fixed_ms + (cost_ms − fixed_ms)·échelle` — la loi mesurée en S350, où le pas de δ 3D vaut
/// 0,09 ms + 3,57 ms × surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shrink {
    /// La plus petite fraction de surface que le domaine accepte, dans `]0, 1]`.
    pub min_scale: f32,
    /// La part du coût, ms, qui ne suit pas la surface — dans `[0, cost_ms]`.
    pub fixed_ms: f32,
}

/// S403, ADR-012 §4 rang 4 — ce qu'un candidat accepte de céder après le rang 1 : **descendre `dx` d'un niveau**, par le
/// transfert d'état d'[ADR-210](../../../docs/adr/ADR-210-changer-de-niveau-par-transfert-d-etat.md). L'hôte le déclare à chaque
/// pas où il l'accepte (`Scheduler::declare_coarsen`) ; un domaine descendu qui ne le déclare plus remonte.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coarsen {
    /// Le coût du pas un niveau plus bas, ms, à pleine emprise — sous la même loi de rétrécissement que le coût entier.
    pub cost_ms: f32,
    /// Ce que l'image perdrait à ce niveau, m : l'écart d'un aller-retour du contenu présent (ADR-210 D2). Il ordonne la
    /// descente — la moindre perte par milliseconde rendue d'abord — et la remontée — la plus forte d'abord.
    pub loss_m: f32,
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
    /// `C` — coût estimé du pas **à pleine emprise**, **mesuré** par le solveur et réinjecté ici.
    pub cost_ms: f32,
    pub blocks: u32,
    pub regime: Regime,
    /// S351 : `Some` si le domaine accepte le rang 1. Un substitutif ne le peut pas : son emprise est le
    /// champ total qu'il possède, et la rétrécir n'est pas gratuit (ADR-012 §4, I-12).
    pub shrink: Option<Shrink>,
}

impl Bid {
    /// `P = W_gameplay · W_perception · W_urgence` (ADR-012 §2). Les trois poids étant dans
    /// `[0,1]` (ADR-170), le produit l'est aussi : c'est **le score d'activation** d'ADR-013 §5,
    /// sans facteur d'échelle.
    pub fn priority(&self) -> f32 {
        self.gameplay * self.perception * self.urgency
    }

    /// S351 : l'échelle effective quand l'échelle commune vaut `q` — jamais sous le minimum déclaré ; 1 sans
    /// déclaration.
    pub fn scale_for(&self, q: f32) -> f32 {
        self.shrink.map_or(1., |s| q.clamp(s.min_scale, 1.))
    }

    /// S351 : le coût à l'échelle commune `q`.
    pub fn cost_for(&self, q: f32) -> f32 {
        match self.shrink {
            Some(s) => s.fixed_ms + (self.cost_ms - s.fixed_ms) * self.scale_for(q),
            None => self.cost_ms,
        }
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
/// S351 : toute décision de dégradation est **engagée** au moins 30 images à 30 Hz, soit une seconde (ADR-012 §5)
/// — le choix du focal comme l'échelle descendue.
pub const ENGAGE_US: u64 = 1_000_000;
/// S351 : la remontée est **lente et rampée**, ≈ 1 s de bout en bout (ADR-012 §5) : l'échelle commune monte d'au
/// plus 1 par seconde. La descente, elle, est immédiate — elle protège la fréquence d'images.
pub const RAMP_PER_S: f32 = 1.;

/// S403 : le coût entier de `bid` à son niveau — le coût déclaré un niveau plus bas s'il y est descendu (vivant descendu
/// et déclarant), le sien sinon.
fn level_base(bid: &Bid, live: &[Live], coarsen: &[(DomainId, Coarsen)]) -> f32 {
    let descendu = live.iter().any(|l| l.id == bid.id && l.active && l.coarse);
    match coarsen.iter().find(|(d, _)| *d == bid.id) {
        Some((_, c)) if descendu => c.cost_ms,
        _ => bid.cost_ms,
    }
}

/// S403 : la loi du rang 1 sur le coût `base` d'un niveau — `fixe + (base − fixe)·échelle`, la part fixe bornée par `base`.
/// À son propre niveau, `Bid::cost_for` au bit.
fn scaled_cost(bid: &Bid, q: f32, base: f32) -> f32 {
    match bid.shrink {
        Some(s) => {
            let fixe = s.fixed_ms.min(base);
            fixe + (base - fixe) * bid.scale_for(q)
        }
        None => base,
    }
}

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
    /// S403 — descendu d'un niveau (rang 4), et depuis quand.
    coarse: bool,
    coarse_since_us: u64,
}

/// Ce qu'un domaine retenu reçoit : le droit de vivre ce pas, et son budget.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Grant {
    pub id: DomainId,
    pub budget_ms: f32,
    /// S351 : la fraction de sa surface accordée — 1, entier ; moins, rétréci au rang 1. Le budget est le coût
    /// à cette échelle.
    pub scale: f32,
    /// S403 : le niveau accordé — 0, le sien ; 1, un niveau plus bas (rang 4, ADR-210). L'hôte transfère l'état quand il change.
    pub level: u8,
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
    /// S351 : déclaration de rétrécissement invalide — échelle minimale hors de `]0, 1]`, part fixe hors de
    /// `[0, coût]`, ou domaine substitutif.
    Shrink,
    /// S403 : déclaration de rang 4 invalide — coût ou perte non finis ou négatifs, candidat non soumis à ce pas ou
    /// substitutif, déclaration en double.
    Coarsen,
}

/// L'ordonnanceur. Sa capacité est fixée à la construction et sa mémoire demandée à l'hôte :
/// aucune allocation à l'exécution (I-06).
pub struct Scheduler {
    profile: Profile,
    bids: Vec<Bid>,
    grants: Vec<Grant>,
    live: Vec<Live>,
    last_us: Option<u64>,
    /// S351 — le rang 1 : le domaine focal, protégé (ADR-012 §4), et depuis quand ; l'échelle commune des autres,
    /// l'instant de sa dernière descente et celui de sa dernière mise à jour (ADR-012 §5).
    focal: Option<DomainId>,
    focal_since_us: u64,
    scale: f32,
    scale_down_us: Option<u64>,
    scale_us: Option<u64>,
    /// S403 — le rang 4 : les déclarations du pas, et l'instant de la dernière remontée (une par seconde, ADR-012 §5).
    coarsen: Vec<(DomainId, Coarsen)>,
    refine_us: Option<u64>,
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
            * (core::mem::size_of::<Bid>() + core::mem::size_of::<Grant>() + core::mem::size_of::<Live>()
                + core::mem::size_of::<(DomainId, Coarsen)>());
        host.alloc.alloc_persistent(bytes).map_err(|_| Error::Capacity)?;
        Ok(Self {
            profile,
            bids: Vec::with_capacity(capacity),
            grants: Vec::with_capacity(capacity),
            live: Vec::with_capacity(capacity),
            last_us: None,
            focal: None,
            focal_since_us: 0,
            scale: 1.,
            scale_down_us: None,
            scale_us: None,
            coarsen: Vec::with_capacity(capacity),
            refine_us: None,
        })
    }

    pub fn profile(&self) -> Profile {
        self.profile
    }

    /// Change de profil sans rien réallouer — manette de qualité (ADR-012 §5), ou banc qui veut
    /// éprouver un budget serré. Les mêmes refus qu'à la construction s'appliquent : un profil
    /// qu'on n'aurait pas accepté au départ ne s'accepte pas davantage en route.
    pub fn set_profile(&mut self, profile: Profile) -> Result<(), Error> {
        if !(profile.cpu_sim_ms.is_finite() && profile.cpu_sim_ms > 0.) {
            return Err(Error::NotFinite);
        }
        if !(profile.off.is_finite() && profile.on.is_finite() && profile.off < profile.on) {
            return Err(Error::NotFinite);
        }
        self.profile = profile;
        Ok(())
    }

    pub fn capacity(&self) -> usize {
        self.bids.capacity()
    }

    /// Ouvre un pas : les soumissions précédentes ne valent plus rien. Ne rend aucune mémoire.
    pub fn begin(&mut self) {
        self.bids.clear();
        self.grants.clear();
        self.coarsen.clear();
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
        if let Some(s) = bid.shrink {
            let echelle = s.min_scale.is_finite() && s.min_scale > 0. && s.min_scale <= 1.;
            let fixe = s.fixed_ms.is_finite() && s.fixed_ms >= 0. && s.fixed_ms <= bid.cost_ms;
            if !(echelle && fixe) || bid.regime == Regime::Substitutive {
                return Err(Error::Shrink);
            }
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

    /// **S403 — le rang 4 déclaré** : le candidat `id`, soumis à ce pas, accepte de descendre `dx` d'un niveau, au coût et à
    /// la perte qu'il donne. Une déclaration par candidat et par pas. Refus `Coarsen` : coût ou perte non finis ou négatifs,
    /// candidat absent de ce pas ou substitutif — le transfert d'ADR-210 ne vaut que pour un perturbatif —, ou en double.
    pub fn declare_coarsen(&mut self, id: DomainId, coarsen: Coarsen) -> Result<(), Error> {
        let fini = |v: f32| v.is_finite() && v >= 0.;
        if !(fini(coarsen.cost_ms) && fini(coarsen.loss_m)) {
            return Err(Error::Coarsen);
        }
        match self.bids.iter().find(|b| b.id == id) {
            Some(b) if b.regime == Regime::Perturbative => {}
            _ => return Err(Error::Coarsen),
        }
        if self.coarsen.iter().any(|(d, _)| *d == id) {
            return Err(Error::Coarsen);
        }
        if self.coarsen.len() == self.coarsen.capacity() {
            return Err(Error::Capacity);
        }
        self.coarsen.push((id, coarsen));
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
                self.live.push(Live { id: b.id, active: true, born_us: now.0, below_since_us: None, coarse: false,
                    coarse_since_us: 0 });
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
    ///
    /// **S351 — le rang 1** (ADR-012 §4). Si ce sac à dos laisse un vivant sans budget et qu'un non-focal a déclaré
    /// pouvoir rétrécir, le **focal** — la plus forte priorité — est servi entier, puis les autres par `P/C`
    /// décroissant à leur échelle minimale, autant qu'il en tient ; enfin l'**échelle commune** monte au plus haut
    /// que ce budget permet. Sans aucune déclaration, la décision reste exactement celle de S278.
    pub fn allocate(&mut self) {
        self.bids.sort_unstable_by(|a, b| {
            (b.priority() * a.cost_ms)
                .total_cmp(&(a.priority() * b.cost_ms))
                .then(a.id.cmp(&b.id))
        });
        let affame = self.fund(1., false);
        let now = self.last_us.unwrap_or(0);
        self.update_focal(now);
        // S403 — le rang 4 : les niveaux se décident ici, sur le sac à dos du rang 1 à son minimum ; sans aucune
        // déclaration ni domaine descendu, rien ne change et la suite est celle de S351, au bit.
        let rang4 = self.update_levels(now);
        let affame = if rang4 { self.fund(1., false) } else { affame };
        let focal = self.focal;
        let declarent = self.bids.iter().any(|b| {
            b.shrink.is_some() && Some(b.id) != focal && self.live.iter().any(|l| l.id == b.id && l.active)
        });
        if !declarent && !rang4 {
            self.scale = 1.;
            return;
        }
        let cible = if affame {
            self.fund(0., true);
            self.largest_scale()
        } else {
            1.
        };
        let q = self.next_scale(cible, now);
        self.scale = q;
        if !affame && q >= 1. {
            return;
        }
        if !affame {
            self.fund(0., true);
        }
        for g in self.grants.iter_mut() {
            if Some(g.id) == focal {
                continue;
            }
            if let Some(b) = self.bids.iter().find(|b| b.id == g.id) {
                g.scale = b.scale_for(q);
                g.budget_ms = scaled_cost(b, q, level_base(b, &self.live, &self.coarsen));
            }
        }
    }

    /// Remplit `grants` à l'échelle commune `q`, sans jamais dépasser budget ni blocs : avec `rang1`, le focal
    /// d'abord et entier, puis les autres par `P/C` décroissant à leur échelle ; sans, le sac à dos de S278, tout à
    /// pleine emprise. Rend vrai si un vivant reste sans budget.
    fn fund(&mut self, q: f32, rang1: bool) -> bool {
        self.grants.clear();
        let focal = if rang1 { self.focal } else { None };
        let (mut ms, mut blocks, mut affame) = (0f32, 0u32, false);
        for passe in 0..2 {
            for bid in &self.bids {
                let est_focal = focal == Some(bid.id);
                if (passe == 0) != est_focal || !self.live.iter().any(|l| l.id == bid.id && l.active) {
                    continue;
                }
                let (echelle, cout) =
                    if rang1 && !est_focal { (bid.scale_for(q), self.scaled_cost(bid, q)) } else { (1., self.level_cost(bid)) };
                if ms + cout > self.profile.cpu_sim_ms || blocks + bid.blocks > self.profile.blocks {
                    affame = true;
                    continue;
                }
                ms += cout;
                blocks += bid.blocks;
                let level = u8::from(self.is_coarse(bid.id));
                self.grants.push(Grant { id: bid.id, budget_ms: cout, scale: echelle, level });
            }
        }
        affame
    }

    /// Le focal d'ADR-012 §4 : parmi les vivants qui soumissionnent, la plus forte priorité — un domaine qui porte
    /// l'acteur du joueur a `W_gameplay` = 1, qui domine le produit (ADR-012 §2). L'égalité se départage par
    /// identité. **Engagé** (ADR-012 §5) : un focal toujours vivant ne cède sa place qu'à une priorité strictement
    /// plus forte, et pas avant une seconde ; un focal qui ne soumissionne plus, ou mort, est remplacé tout de suite.
    fn update_focal(&mut self, now: u64) {
        let vivant = |id: DomainId| self.live.iter().any(|l| l.id == id && l.active);
        let meilleur = self
            .bids
            .iter()
            .filter(|b| vivant(b.id))
            .max_by(|a, b| a.priority().total_cmp(&b.priority()).then(b.id.cmp(&a.id)))
            .copied();
        let actuel = self.focal.and_then(|f| self.bids.iter().find(|b| b.id == f && vivant(b.id)).copied());
        let garde = match (actuel, meilleur) {
            (Some(a), Some(m)) => {
                m.id == a.id || m.priority() <= a.priority() || now.saturating_sub(self.focal_since_us) < ENGAGE_US
            }
            _ => false,
        };
        if !garde {
            let nouveau = meilleur.map(|b| b.id);
            if nouveau != self.focal {
                self.focal_since_us = now;
            }
            self.focal = nouveau;
        }
    }

    /// La plus grande échelle commune qui tient les servis dans le budget : dichotomie de 24 étapes, la
    /// résolution d'un `f32` sur `[0, 1]`. Le coût croît avec l'échelle, donc 0 — où `fund` les a choisis — tient.
    fn largest_scale(&self) -> f32 {
        let focal = self.focal;
        let total = |q: f32| -> f32 {
            self.grants
                .iter()
                .filter_map(|g| self.bids.iter().find(|b| b.id == g.id))
                .map(|b| if Some(b.id) == focal { self.level_cost(b) } else { self.scaled_cost(b, q) })
                .sum()
        };
        let budget = self.profile.cpu_sim_ms;
        if total(1.) <= budget {
            return 1.;
        }
        let (mut lo, mut hi) = (0f32, 1f32);
        for _ in 0..24 {
            let mid = 0.5 * (lo + hi);
            if total(mid) <= budget {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo
    }

    /// L'échelle commune de ce pas, vers `cible` (ADR-012 §5) : **descente immédiate**, qui réarme l'engagement ;
    /// **remontée** seulement une seconde après la dernière descente, d'au plus `RAMP_PER_S` par seconde écoulée
    /// depuis la fin de l'engagement. `cible` tient toujours le budget : l'échelle rendue ne la dépasse jamais.
    fn next_scale(&mut self, cible: f32, now: u64) -> f32 {
        let q = self.scale;
        let nouveau = if cible < q {
            self.scale_down_us = Some(now);
            cible
        } else {
            let fin = self.scale_down_us.map_or(0, |t| t.saturating_add(ENGAGE_US));
            let debut = self.scale_us.unwrap_or(now).max(fin);
            let monte = RAMP_PER_S * now.saturating_sub(debut) as f32 * 1e-6;
            (q + monte).min(cible)
        };
        self.scale_us = Some(now);
        nouveau
    }

    /// S403 : le candidat `id` est-il descendu d'un niveau ? Un vivant descendu, et qui le déclare encore à ce pas.
    fn is_coarse(&self, id: DomainId) -> bool {
        self.live.iter().any(|l| l.id == id && l.active && l.coarse) && self.coarsen.iter().any(|(d, _)| *d == id)
    }

    /// S403 : le coût entier d'un candidat à son niveau — celui qu'il a déclaré un niveau plus bas s'il y est descendu.
    fn level_cost(&self, bid: &Bid) -> f32 {
        level_base(bid, &self.live, &self.coarsen)
    }

    /// S403 : le coût à l'échelle commune `q`, à son niveau — la loi du rang 1 sur le coût de ce niveau.
    fn scaled_cost(&self, bid: &Bid, q: f32) -> f32 {
        scaled_cost(bid, q, self.level_cost(bid))
    }

    /// **S403 — les niveaux de ce pas** (ADR-012 §4 rang 4, §5). Un vivant descendu qui ne déclare plus remonte ; le focal
    /// n'est jamais descendu. Puis, **tant que le rang 1 à son minimum laisse un vivant sans budget**, le non-focal déclaré
    /// qui perd le moins par milliseconde rendue descend, immédiatement. Sinon, une **remontée** au plus par seconde : le
    /// descendu qui perd le plus, une seconde au moins après sa descente, s'il tient sans affamer personne. Rend faux quand le
    /// rang 4 n'est pas en jeu — ni déclaration, ni domaine descendu.
    fn update_levels(&mut self, now: u64) -> bool {
        let focal = self.focal;
        let declared: &[(DomainId, Coarsen)] = &self.coarsen;
        for l in self.live.iter_mut() {
            if l.coarse && (Some(l.id) == focal || !declared.iter().any(|(d, _)| *d == l.id)) {
                l.coarse = false;
            }
        }
        let vivant = |id: DomainId, live: &[Live]| live.iter().any(|l| l.id == id && l.active);
        let en_jeu = self.coarsen.iter().any(|(d, _)| Some(*d) != focal && vivant(*d, &self.live))
            || self.live.iter().any(|l| l.active && l.coarse);
        if !en_jeu {
            return false;
        }
        let mut descendu = false;
        while self.fund(0., true) {
            let Some(id) = self.coarsen_candidate() else { break };
            if let Some(l) = self.live.iter_mut().find(|l| l.id == id) {
                l.coarse = true;
                l.coarse_since_us = now;
            }
            descendu = true;
        }
        if !descendu && !self.fund(0., true) {
            if let Some(id) = self.refine_candidate(now) {
                self.set_coarse(id, false);
                if self.fund(0., true) {
                    self.set_coarse(id, true);
                } else {
                    self.refine_us = Some(now);
                }
            }
        }
        true
    }

    fn set_coarse(&mut self, id: DomainId, coarse: bool) {
        if let Some(l) = self.live.iter_mut().find(|l| l.id == id) {
            l.coarse = coarse;
        }
    }

    /// S403 : le prochain à descendre — vivant, non focal, déclaré, pas encore descendu, et qui rend du temps : la moindre
    /// perte par milliseconde rendue, comparée en croix ; l'égalité se départage par identité.
    fn coarsen_candidate(&self) -> Option<DomainId> {
        let focal = self.focal;
        self.coarsen
            .iter()
            .filter_map(|(id, c)| {
                let bid = self.bids.iter().find(|b| b.id == *id)?;
                let vivant = self.live.iter().any(|l| l.id == *id && l.active && !l.coarse);
                let rendu = bid.cost_ms - c.cost_ms;
                (vivant && Some(*id) != focal && rendu > 0.).then_some((*id, c.loss_m, rendu))
            })
            .min_by(|a, b| (a.1 * b.2).total_cmp(&(b.1 * a.2)).then(a.0.cmp(&b.0)))
            .map(|(id, _, _)| id)
    }

    /// S403 : le prochain à remonter — descendu depuis une seconde au moins, une seconde au moins après la dernière
    /// remontée : la plus forte perte déclarée d'abord ; l'égalité se départage par identité.
    fn refine_candidate(&self, now: u64) -> Option<DomainId> {
        if self.refine_us.is_some_and(|t| now.saturating_sub(t) < ENGAGE_US) {
            return None;
        }
        self.live
            .iter()
            .filter(|l| l.active && l.coarse && now.saturating_sub(l.coarse_since_us) >= ENGAGE_US)
            .filter_map(|l| self.coarsen.iter().find(|(d, _)| *d == l.id).map(|(d, c)| (*d, c.loss_m)))
            .max_by(|a, b| a.1.total_cmp(&b.1).then(b.0.cmp(&a.0)))
            .map(|(id, _)| id)
    }

    /// **S403 — l'issue de la famine, déclarée** : les vivants que ce pas ne sert pas, ni entiers, ni rétrécis (rang 1), ni
    /// descendus (rang 4). Pour l'hôte, le rang 5 : détruire le perturbatif et laisser la haute mer (W) le remplacer.
    pub fn starved(&self) -> impl Iterator<Item = DomainId> + '_ {
        self.live
            .iter()
            .filter(|l| l.active && !self.grants.iter().any(|g| g.id == l.id))
            .map(|l| l.id)
    }

    /// S351 : le domaine focal du dernier `allocate`, protégé du rang 1.
    pub fn focal(&self) -> Option<DomainId> {
        self.focal
    }

    /// S351 : l'échelle commune des non-focaux au dernier `allocate` — 1 quand rien n'est dégradé.
    pub fn scale(&self) -> f32 {
        self.scale
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
        self.focal = None;
        self.focal_since_us = 0;
        self.scale = 1.;
        self.scale_down_us = None;
        self.scale_us = None;
        self.coarsen.clear();
        self.refine_us = None;
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
            shrink: None,
        }
    }

    /// S351 : un domaine δ 3D de la scène de la porte B — 3,7 ms entier, 0,09 ms de part fixe (S350), et qui
    /// accepte de descendre à `min` de sa surface.
    fn bid3(id: u32, score: f32, min: f32) -> Bid {
        Bid { shrink: Some(Shrink { min_scale: min, fixed_ms: 0.09 }), ..bid(id, score, 1., 1., 3.7) }
    }

    fn profil(budget: f32) -> Scheduler {
        let mut alloc = Hote { used: core::cell::Cell::new(0), limit: 1 << 16 };
        let services = Hote { used: core::cell::Cell::new(0), limit: 0 };
        let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
        Scheduler::with_capacity(&mut host, Profile { cpu_sim_ms: budget, blocks: 64, on: 0.1, off: 0.05 }, 8).unwrap()
    }

    /// Un pas complet : soumissions, décision, allocation.
    fn servir(s: &mut Scheduler, t_us: u64, bids: &[Bid]) {
        s.begin();
        for b in bids {
            s.submit(*b).unwrap();
        }
        s.decide(SimTime(t_us)).unwrap();
        s.allocate();
    }

    fn accorde(s: &Scheduler, id: u32) -> Option<Grant> {
        s.grants().iter().copied().find(|g| g.id == DomainId(id))
    }

    /// Critère 1 (b) : là où S278 affamait le second domaine — 3,7 + 3,7 ms pour 5 —, le rang 1 le sert rétréci.
    /// Le focal reste entier ; l'autre prend ce qui reste, 1,3 ms, soit une échelle de (1,3 − 0,09)/3,61.
    #[test]
    fn le_rang_1_retrecit_le_non_focal_au_lieu_de_l_affamer_s351() {
        let mut s = profil(5.);
        servir(&mut s, 0, &[bid3(1, 0.12, 0.05), bid3(2, 0.11, 0.05)]);
        assert_eq!(s.focal(), Some(DomainId(1)));
        let (a, b) = (accorde(&s, 1).unwrap(), accorde(&s, 2).expect("le rang 1 doit servir le second"));
        assert_eq!((a.scale, a.budget_ms), (1., 3.7), "le focal est entier");
        let attendu = (5. - 3.7 - 0.09) / 3.61;
        assert!((b.scale - attendu).abs() < 1e-5, "échelle {} pour {attendu}", b.scale);
        assert!(s.granted_ms() <= 5., "{} ms", s.granted_ms());
        assert!(s.granted_ms() > 5. - 1e-4, "le budget est employé : {} ms", s.granted_ms());
    }

    /// Critère 1 (b) : le focal est la plus forte **priorité**, même quand son rapport `P/C` est le moins bon — il
    /// est protégé des rangs 1 à 5 (ADR-012 §4) ; c'est l'autre qui rétrécit.
    #[test]
    fn le_focal_n_est_jamais_retreci_s351() {
        let mut s = profil(5.);
        let cher = Bid { cost_ms: 4.5, ..bid3(1, 0.2, 0.05) };
        servir(&mut s, 0, &[cher, bid3(2, 0.15, 0.05)]);
        assert_eq!(s.focal(), Some(DomainId(1)));
        assert_eq!(accorde(&s, 1).map(|g| g.scale), Some(1.));
        let b = accorde(&s, 2).expect("servi à son minimum : 0,09 + 3,61 × 0,05 = 0,27 ms tiennent dans 0,5");
        assert!(b.scale < 0.2 && b.scale >= 0.05);
    }

    /// Critère 1 (b) : une échelle **commune**, jamais sous le minimum de chacun ; un non-déclarant reste entier.
    #[test]
    fn l_echelle_est_commune_et_respecte_chaque_minimum_s351() {
        // 3,7 (focal) + 0,5 (entier) + 0,27 et 1,53 aux minimums = 6,00 : tout tient à 6,5 ms ; l'échelle commune
        // monte alors à (6,5 − 5,824)/3,61 = 0,187, sous le minimum 0,4 du troisième.
        let mut s = profil(6.5);
        let entier = Bid { cost_ms: 0.5, ..bid(4, 0.13, 1., 1., 0.5) };
        servir(&mut s, 0, &[bid3(1, 0.2, 0.05), bid3(2, 0.15, 0.05), bid3(3, 0.14, 0.4), entier]);
        let q = s.scale();
        assert!(q < 0.4, "l'échelle commune {q} doit passer sous le minimum du troisième");
        assert_eq!(accorde(&s, 2).unwrap().scale, q);
        assert_eq!(accorde(&s, 3).unwrap().scale, 0.4, "jamais sous son minimum");
        assert_eq!(accorde(&s, 4).unwrap().scale, 1., "un non-déclarant ne rétrécit pas");
        assert!(s.granted_ms() <= 6.5);
    }

    /// Critère 1 (b) : un candidat qui ne tient pas même à son minimum reste sans budget — le rang 5 le
    /// détruirait, il n'est pas écrit ; le plus mauvais rapport `P/C` cède le premier.
    #[test]
    fn qui_ne_tient_pas_a_son_minimum_reste_affame_s351() {
        let mut s = profil(4.);
        servir(&mut s, 0, &[bid3(1, 0.3, 0.05), bid3(2, 0.2, 0.05), bid3(3, 0.15, 0.05)]);
        // 3,7 pour le focal ; 0,3 restent, un seul minimum (0,27 ms) y tient.
        assert_eq!(s.grants().len(), 2);
        assert!(accorde(&s, 2).is_some() && accorde(&s, 3).is_none());
        assert!(s.granted_ms() <= 4.);
    }

    /// Critère 1 (a) : sans aucune déclaration, rien ne change — le second reste affamé, comme en S278.
    #[test]
    fn sans_declaration_la_decision_de_s278_s351() {
        let mut s = profil(5.);
        servir(&mut s, 0, &[bid(1, 0.12, 1., 1., 3.7), bid(2, 0.11, 1., 1., 3.7)]);
        assert_eq!(s.grants(), &[Grant { id: DomainId(1), budget_ms: 3.7, scale: 1., level: 0 }]);
        assert_eq!(s.scale(), 1.);
    }

    /// Critère 1 (d) : l'ordre de soumission ne change rien au bit — ensemble, échelles et budgets.
    #[test]
    fn le_rang_1_ne_depend_pas_de_l_ordre_de_soumission_s351() {
        let bids = [bid3(1, 0.2, 0.05), bid3(2, 0.15, 0.05), bid3(3, 0.14, 0.4), bid3(5, 0.15, 0.1)];
        let decision = |ordre: [usize; 4]| {
            let mut s = profil(6.);
            let b: Vec<Bid> = ordre.iter().map(|&i| bids[i]).collect();
            servir(&mut s, 0, &b);
            let mut g: Vec<(u32, u32, u32)> =
                s.grants().iter().map(|g| (g.id.0, g.scale.to_bits(), g.budget_ms.to_bits())).collect();
            g.sort();
            g
        };
        let a = decision([0, 1, 2, 3]);
        assert_eq!(a, decision([3, 2, 1, 0]));
        assert_eq!(a, decision([2, 0, 3, 1]));
    }

    /// Critère 1 (c), ADR-012 §5 : la descente est immédiate ; la remontée attend une seconde après elle, puis monte
    /// d'au plus 1 par seconde — jamais au-delà de ce que le budget tient.
    #[test]
    fn la_descente_est_immediate_la_remontee_rampee_apres_une_seconde_s351() {
        let mut s = profil(5.);
        let bids = [bid3(1, 0.2, 0.05), bid3(2, 0.15, 0.05)];
        servir(&mut s, 0, &bids);
        let bas = s.scale();
        assert!((bas - (5. - 3.7 - 0.09) / 3.61).abs() < 1e-5, "descente immédiate : {bas}");
        // Le budget s'élargit : tout tiendrait entier. L'échelle ne doit pas bouger pendant l'engagement.
        s.set_profile(Profile { cpu_sim_ms: 8., blocks: 64, on: 0.1, off: 0.05 }).unwrap();
        let (mut avant, mut t_avant, mut entier_a) = (bas, 0u64, None);
        for n in 1..=150u64 {
            let t = n * 16_667;
            servir(&mut s, t, &bids);
            let q = s.scale();
            assert!(s.granted_ms() <= 8., "budget dépassé à {t} µs");
            if t < ENGAGE_US {
                assert_eq!(q, bas, "remontée pendant l'engagement, à {t} µs");
            }
            let permis = RAMP_PER_S * (t - t_avant) as f32 * 1e-6 + 1e-6;
            assert!(q - avant <= permis, "remontée trop vive à {t} µs : {avant} → {q}");
            assert!(q <= 1.);
            if q == 1. && entier_a.is_none() {
                entier_a = Some(t);
            }
            (avant, t_avant) = (q, t);
        }
        let t1 = entier_a.expect("l'échelle doit revenir à 1");
        // Une seconde d'engagement, puis 0,665 à monter à 1 par seconde.
        assert!(t1 >= ENGAGE_US + 660_000 && t1 <= ENGAGE_US + 700_000, "entier à {t1} µs");
        assert_eq!(accorde(&s, 2).unwrap().scale, 1.);
    }

    /// Critère 1 (c) : une nouvelle descente pendant la remontée est immédiate et **réarme** l'engagement.
    #[test]
    fn une_descente_rearme_l_engagement_s351() {
        let mut s = profil(5.);
        let bids = [bid3(1, 0.2, 0.05), bid3(2, 0.15, 0.05)];
        servir(&mut s, 0, &bids);
        s.set_profile(Profile { cpu_sim_ms: 8., blocks: 64, on: 0.1, off: 0.05 }).unwrap();
        servir(&mut s, 1_200_000, &bids);
        let montee = s.scale();
        assert!(montee > 0.4, "la remontée a commencé : {montee}");
        s.set_profile(Profile { cpu_sim_ms: 5., blocks: 64, on: 0.1, off: 0.05 }).unwrap();
        servir(&mut s, 1_216_667, &bids);
        let bas = s.scale();
        assert!(bas < 0.34, "descente immédiate : {bas}");
        s.set_profile(Profile { cpu_sim_ms: 8., blocks: 64, on: 0.1, off: 0.05 }).unwrap();
        servir(&mut s, 2_000_000, &bids);
        assert_eq!(s.scale(), bas, "engagé jusqu'à 2,217 s");
        servir(&mut s, 2_316_667, &bids);
        assert!(s.scale() > bas && s.scale() <= bas + 0.1 + 1e-6, "{}", s.scale());
    }

    /// Critère 1 (c) : le focal est engagé une seconde — une priorité qui le dépasse ne le détrône qu'après ;
    /// un focal qui ne soumissionne plus est remplacé tout de suite.
    #[test]
    fn le_focal_est_engage_une_seconde_s351() {
        let mut s = profil(5.);
        servir(&mut s, 0, &[bid3(1, 0.2, 0.05), bid3(2, 0.15, 0.05)]);
        assert_eq!(s.focal(), Some(DomainId(1)));
        servir(&mut s, 200_000, &[bid3(1, 0.2, 0.05), bid3(2, 0.25, 0.05)]);
        assert_eq!(s.focal(), Some(DomainId(1)), "engagé : pas de bascule à 0,2 s");
        assert_eq!(accorde(&s, 1).unwrap().scale, 1.);
        servir(&mut s, 1_000_000, &[bid3(1, 0.2, 0.05), bid3(2, 0.25, 0.05)]);
        assert_eq!(s.focal(), Some(DomainId(2)), "une seconde après, la plus forte priorité l'emporte");
        assert_eq!(accorde(&s, 2).unwrap().scale, 1.);
        assert!(accorde(&s, 1).unwrap().scale < 1., "l'ancien focal rétrécit");
        // Le focal cesse de soumissionner : il vit encore (délais d'ADR-013), mais le focal change aussitôt.
        servir(&mut s, 1_016_667, &[bid3(1, 0.2, 0.05)]);
        assert_eq!(s.focal(), Some(DomainId(1)));
    }

    /// Critère 1 (e) : un substitutif ne se déclare pas rétrécissable, et une déclaration invalide est refusée
    /// avant de rien toucher.
    #[test]
    fn une_declaration_invalide_est_refusee_s351() {
        let mut s = profil(5.);
        s.begin();
        let sub = Bid { regime: Regime::Substitutive, ..bid3(1, 0.2, 0.05) };
        assert_eq!(s.submit(sub), Err(Error::Shrink));
        for (min, fixe) in [(0., 0.09), (1.5, 0.09), (0.5, 4.), (f32::NAN, 0.), (0.5, -1.)] {
            let b = Bid { shrink: Some(Shrink { min_scale: min, fixed_ms: fixe }), ..bid(2, 0.2, 1., 1., 3.7) };
            assert_eq!(s.submit(b), Err(Error::Shrink), "min {min}, part fixe {fixe}");
        }
        assert!(s.bids().is_empty());
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
        let jouer = |s: &mut Scheduler, t: u64, score: f32| {
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

    // -----------------------------------------------------------------------------------------------------------------
    // S403 — le rang 4 : descendre `dx` d'un niveau (ADR-210), après le rang 1 ; l'issue de la famine, déclarée.

    /// Un pas complet avec des déclarations de rang 4 : `(identité, coût un niveau plus bas, perte)`.
    fn servir4(s: &mut Scheduler, t_us: u64, bids: &[Bid], decl: &[(u32, f32, f32)]) {
        s.begin();
        for b in bids {
            s.submit(*b).unwrap();
        }
        for (id, cost, loss) in decl {
            s.declare_coarsen(DomainId(*id), Coarsen { cost_ms: *cost, loss_m: *loss }).unwrap();
        }
        s.decide(SimTime(t_us)).unwrap();
        s.allocate();
    }

    /// Le banc de S403 en miniature : un focal (0,9) et deux non-focaux égaux (0,5), 0,44 ms chacun à 25 cm, 0,134 ms à 50 cm.
    fn trois() -> [Bid; 3] {
        [bid(0, 1., 0.9, 1., 0.44), bid(1, 1., 0.5, 1., 0.44), bid(2, 1., 0.5, 1., 0.44)]
    }

    fn niveau(s: &Scheduler, id: u32) -> Option<u8> {
        accorde(s, id).map(|g| g.level)
    }

    /// Critère 2 : le rang 4 ne sert qu'après le rang 1 à son minimum — là où le rang 1 suffit (le cas de S351), personne ne
    /// descend, et la décision est celle de S351.
    #[test]
    fn le_rang_4_n_agit_pas_quand_le_rang_1_suffit_s403() {
        let mut s = profil(5.);
        servir4(&mut s, 0, &[bid3(1, 0.12, 0.05), bid3(2, 0.11, 0.05)], &[(2, 0.5, 0.001)]);
        let (a, b) = (accorde(&s, 1).unwrap(), accorde(&s, 2).unwrap());
        assert_eq!((a.level, b.level), (0, 0));
        let attendu = (5. - 3.7 - 0.09) / 3.61;
        assert!((b.scale - attendu).abs() < 1e-5, "échelle {} pour {attendu}", b.scale);
        assert_eq!(s.starved().count(), 0);
    }

    /// Critère 3 : il descend le non-focal qui perd le moins par milliseconde rendue — la bosse, pas la source ; échangées, les
    /// pertes échangent le choix ; le focal n'est jamais descendu, même déclaré ; le budget n'est jamais dépassé.
    #[test]
    fn le_rang_4_descend_celui_qui_perd_le_moins_s403() {
        for (perte1, perte2, attendu) in [(0.0003f32, 0.015f32, 1u32), (0.015, 0.0003, 2)] {
            let mut s = profil(1.05);
            servir4(&mut s, 0, &trois(), &[(0, 0.134, 0.), (1, 0.134, perte1), (2, 0.134, perte2)]);
            assert_eq!(s.focal(), Some(DomainId(0)));
            assert_eq!(niveau(&s, 0), Some(0), "le focal n'est jamais descendu");
            let autre = 3 - attendu;
            assert_eq!((niveau(&s, attendu), niveau(&s, autre)), (Some(1), Some(0)), "pertes {perte1} / {perte2}");
            assert!((accorde(&s, attendu).unwrap().budget_ms - 0.134).abs() < 1e-6);
            assert!(s.granted_ms() <= 1.05, "{} ms", s.granted_ms());
            assert_eq!(s.starved().count(), 0);
        }
    }

    /// Critère 4 : la famine a une issue déclarée — à 0,6 ms, le focal et une descente seulement ; l'autre est rendu affamé.
    #[test]
    fn la_famine_a_une_issue_declaree_s403() {
        let mut s = profil(0.6);
        servir4(&mut s, 0, &trois(), &[(1, 0.134, 0.0003), (2, 0.134, 0.015)]);
        assert_eq!((niveau(&s, 0), niveau(&s, 1)), (Some(0), Some(1)));
        assert_eq!(s.starved().collect::<Vec<_>>(), vec![DomainId(2)]);
        assert!(s.granted_ms() <= 0.6, "{} ms", s.granted_ms());
        // Sans déclaration, S278 : affamé aussi, et dit de même.
        let mut t = profil(0.6);
        servir4(&mut t, 0, &trois(), &[]);
        assert_eq!(t.starved().count(), 2);
    }

    /// Critère 5 : descente immédiate ; remontée engagée une seconde après la descente, un domaine par seconde, la plus forte
    /// perte d'abord ; une nouvelle famine redescend aussitôt.
    #[test]
    fn la_remontee_est_engagee_une_seconde_et_une_par_seconde_s403() {
        let decl = [(1, 0.134, 0.0003), (2, 0.134, 0.015)];
        let mut s = profil(0.75);
        s.set_profile(Profile { cpu_sim_ms: 0.75, blocks: 64, on: 0.1, off: 0.05 }).unwrap();
        servir4(&mut s, 0, &trois(), &decl);
        assert_eq!((niveau(&s, 1), niveau(&s, 2)), (Some(1), Some(1)), "0,44 + 0,134 + 0,134 : les deux descendent");
        // Le budget revient : rien ne remonte avant une seconde.
        s.set_profile(Profile { cpu_sim_ms: 2., blocks: 64, on: 0.1, off: 0.05 }).unwrap();
        servir4(&mut s, 999_999, &trois(), &decl);
        assert_eq!((niveau(&s, 1), niveau(&s, 2)), (Some(1), Some(1)));
        // À une seconde, la plus forte perte remonte d'abord ; l'autre attend la seconde suivante.
        servir4(&mut s, 1_000_000, &trois(), &decl);
        assert_eq!((niveau(&s, 1), niveau(&s, 2)), (Some(1), Some(0)));
        servir4(&mut s, 1_999_999, &trois(), &decl);
        assert_eq!(niveau(&s, 1), Some(1));
        servir4(&mut s, 2_000_000, &trois(), &decl);
        assert_eq!((niveau(&s, 1), niveau(&s, 2)), (Some(0), Some(0)));
        // Nouvelle famine : la descente est immédiate.
        s.set_profile(Profile { cpu_sim_ms: 1.05, blocks: 64, on: 0.1, off: 0.05 }).unwrap();
        servir4(&mut s, 2_000_001, &trois(), &decl);
        assert_eq!((niveau(&s, 1), niveau(&s, 2)), (Some(1), Some(0)));
        assert!(s.granted_ms() <= 1.05);
    }

    /// Un descendu qui ne déclare plus remonte à son niveau — l'hôte l'a décidé ; s'il n'y tient pas, il est affamé et le dit.
    #[test]
    fn un_descendu_qui_ne_declare_plus_remonte_s403() {
        let mut s = profil(1.05);
        servir4(&mut s, 0, &trois(), &[(1, 0.134, 0.0003), (2, 0.134, 0.015)]);
        assert_eq!(niveau(&s, 1), Some(1));
        servir4(&mut s, 20_000, &trois(), &[(2, 0.134, 0.015)]);
        assert_eq!(niveau(&s, 1), Some(0), "1 ne déclare plus : il remonte");
        assert_eq!(niveau(&s, 2), Some(1), "et 2 descend à sa place");
    }

    /// Une déclaration de rang 4 invalide est refusée : coût ou perte non finis ou négatifs, candidat absent ou substitutif,
    /// déclaration en double.
    #[test]
    fn une_declaration_de_rang_4_invalide_est_refusee_s403() {
        let mut s = profil(1.);
        s.begin();
        s.submit(bid(1, 1., 0.5, 1., 0.44)).unwrap();
        s.submit(Bid { regime: Regime::Substitutive, ..bid(2, 1., 0.5, 1., 0.44) }).unwrap();
        let c = |cost: f32, loss: f32| Coarsen { cost_ms: cost, loss_m: loss };
        assert_eq!(s.declare_coarsen(DomainId(1), c(f32::NAN, 0.)), Err(Error::Coarsen));
        assert_eq!(s.declare_coarsen(DomainId(1), c(0.1, -1e-3)), Err(Error::Coarsen));
        assert_eq!(s.declare_coarsen(DomainId(3), c(0.1, 0.)), Err(Error::Coarsen));
        assert_eq!(s.declare_coarsen(DomainId(2), c(0.1, 0.)), Err(Error::Coarsen));
        s.declare_coarsen(DomainId(1), c(0.1, 0.)).unwrap();
        assert_eq!(s.declare_coarsen(DomainId(1), c(0.1, 0.)), Err(Error::Coarsen));
    }
}
