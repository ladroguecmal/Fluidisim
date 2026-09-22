//! **L'oracle croisé** — deux implémentations du même modèle, confrontées champ à champ.
//!
//! # Ce que cet objet est
//!
//! `delta.rs` et `shallow.rs` résolvent les mêmes équations — Saint-Venant 1D, volumes finis, flux
//! de Rusanov — et ont été écrites indépendamment, dans deux histoires parallèles du dépôt qui
//! s'ignoraient (`ADR-043`). Ce module les fait tourner sur le **même montage** et compare leurs
//! **sorties**, et non leurs verdicts : c'est ce que `ADR-043` §3 promet, et que ni S35 ni S36
//! n'avaient fait.
//!
//! Il n'appartient à aucun des deux véhicules. C'est pourquoi il vit ici et non dans `physics.rs`
//! ni dans `physics_shallow.rs`.
//!
//! # Ce qu'un désaccord veut dire — et le plancher qui en décide
//!
//! Les deux partagent le modèle, donc **tous ses angles morts** : une dimension, `c = √(g·h)`, non
//! dispersif (**L138**). Une concordance ne valide pas la physique. Ce qu'un désaccord peut
//! désigner, c'est une **faute d'implémentation** : indice décalé, signe inversé, condition de bord
//! mal posée.
//!
//! **Mais les deux ne calculent pas dans la même précision.** `delta.rs` est en `f32`,
//! `shallow.rs` en `f64` — un fait qu'aucun document du corpus ne mentionnait avant S37, et qui
//! décide de tout ce que cet oracle peut dire. `ε_f32 ≈ 1,19·10⁻⁷` ; sur une hauteur de 3 m,
//! l'arrondi vaut déjà `≈ 3,6·10⁻⁷ m`. **Sous ce plancher, un désaccord ne dit rien** : il mesure
//! l'arithmétique, pas le schéma.
//!
//! Calibrer l'instrument avant de s'en servir n'est pas une précaution de style — c'est **L131**,
//! *avant de corriger, vérifier qu'on mesure la bonne chose*, et **A157**, *un seuil posé sans
//! fondement est reproductible et dénué de sens*.
//!
//! # Ce qui n'est pas comparable
//!
//! **Les pas de temps.** Les deux avancent sous le même nombre de Courant, mais `dt_cfl` est
//! calculé dans deux précisions et diverge dès le premier pas. Seuls les **états à un même temps
//! final** se comparent.

use water_core::{Bassin, Delta1D, HostServices, Shallow1D};

/// Pesanteur, pour les grandeurs de référence des confrontations.
const G: f64 = 9.81;

use crate::host_impl::{ArenaAllocator, SequentialJobs, StderrSink};
use crate::physics_shallow::montage_c01;

/// Épsilon de la simple précision — `2⁻²³`. Le plancher de l'oracle en dérive.
pub const EPS_F32: f64 = 1.192_092_9e-7;

/// L'écart entre deux champs discrets, sous les deux normes qui disent des choses différentes.
///
/// `linf` répond à *quel est le pire point ?* et `l1` à *combien y en a-t-il ?* Une faute
/// d'implémentation locale — un indice décalé, une condition de bord — se voit en `L∞` et se dilue
/// en `L¹` ; une divergence de schéma se voit dans les deux. Les rapporter ensemble est la seule
/// façon de les distinguer (**A156** : *un cas qui réduit une solution à un scalaire peut classer
/// deux candidats à l'envers*).
#[derive(Debug, Clone, Copy)]
pub struct Ecart {
    /// Plus grand écart absolu, dans l'unité du champ.
    pub linf: f64,
    /// Écart moyen absolu — la norme `L¹` divisée par le nombre de cellules.
    pub l1: f64,
    /// Indice de la cellule qui porte `linf`. Un désaccord groupé sur un bord se lit ici.
    pub i_max: usize,
}

impl Ecart {
    /// Compare deux champs cellule à cellule. Les deux tranches doivent avoir la même longueur —
    /// c'est vérifié, parce qu'une comparaison sur des grilles décalées est un résultat faux qui a
    /// l'air d'un résultat.
    pub fn entre(a: &[f64], b: &[f64]) -> Ecart {
        assert_eq!(a.len(), b.len(), "grilles de tailles différentes : rien à comparer");
        let (mut linf, mut somme, mut i_max) = (0.0f64, 0.0f64, 0usize);
        for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
            let e = (x - y).abs();
            somme += e;
            if e > linf {
                linf = e;
                i_max = i;
            }
        }
        Ecart {
            linf,
            l1: somme / a.len() as f64,
            i_max,
        }
    }
}

/// Les champs d'un état, extraits sous une forme comparable.
pub struct Champs {
    pub fond: Vec<f64>,
    pub hauteur: Vec<f64>,
    pub surface: Vec<f64>,
    pub vitesse: Vec<f64>,
    pub x: Vec<f64>,
}

/// Relève les champs de `delta.rs`, en élargissant les `f32` vers `f64`.
///
/// L'élargissement est exact — tout `f32` est un `f64` — donc il n'ajoute aucune erreur. Ce qui
/// reste est celle que `delta.rs` a déjà commise en calculant.
pub fn champs_delta(d: &Delta1D) -> Champs {
    let n = d.nx();
    Champs {
        fond: (0..n).map(|i| (d.eta(i) - d.h(i)) as f64).collect(),
        hauteur: (0..n).map(|i| d.h(i) as f64).collect(),
        surface: (0..n).map(|i| d.eta(i) as f64).collect(),
        vitesse: (0..n).map(|i| d.u(i) as f64).collect(),
        x: (0..n).map(|i| d.x(i) as f64).collect(),
    }
}

/// Relève les champs de `shallow.rs`.
pub fn champs_shallow(s: &Shallow1D) -> Champs {
    let n = s.cellules();
    Champs {
        fond: (0..n).map(|i| s.fond(i)).collect(),
        hauteur: (0..n).map(|i| s.hauteur(i)).collect(),
        surface: (0..n).map(|i| s.surface(i)).collect(),
        vitesse: (0..n).map(|i| s.vitesse(i)).collect(),
        x: (0..n).map(|i| s.x(i)).collect(),
    }
}

/// Le montage de C01 sur les deux véhicules, à `t = 0`.
///
/// Les deux descriptions sont écrites séparément et se trouvent coïncider : 40 m, 160 cellules,
/// `dx = 0,25 m`, fond `−3 → −1` de pente 1:20, `η₀ = 0`, au repos. Rien n'a été adapté pour cette
/// session — c'est le même `CAS-CANONIQUES` qui a dicté les deux.
pub fn montages_c01(host: &mut HostServices) -> (Delta1D, Shallow1D) {
    let d = Delta1D::configure(host, Bassin::c01()).expect("configuration de delta.rs");
    let s = montage_c01();
    (d, s)
}

/// Un hôte jetable pour les montages de comparaison.
pub fn avec_hote<T>(octets: usize, f: impl FnOnce(&mut HostServices) -> T) -> T {
    let mut alloc = ArenaAllocator::with_capacity(octets);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };
    f(&mut host)
}

/// **Le plancher de l'oracle** — ce que la seule différence de précision produit comme écart.
///
/// C01 au repos est le seul montage où la solution exacte est connue **et triviale** : `u ≡ 0`,
/// `η ≡ η₀`. Chaque solveur y a donc une erreur **mesurable contre la vérité**, sans oracle ni
/// référence intermédiaire. Le plancher de la comparaison croisée est borné par la somme des deux :
/// deux solveurs exacts à `ε₁` et `ε₂` près ne peuvent pas différer de plus de `ε₁ + ε₂`.
///
/// **Au-dessous de ce plancher, un désaccord ne dit rien** — il mesure l'arithmétique. Au-dessus,
/// il dit quelque chose sur les schémas. C'est la seule ligne qui rend cet oracle lisible, et elle
/// se **mesure** au lieu de se supposer.
pub struct Plancher {
    /// Erreur de `delta.rs` (`f32`) contre la solution exacte : `max|u|`.
    pub delta_u: f64,
    /// Erreur de `shallow.rs` (`f64`) contre la solution exacte : `max|u|`.
    pub shallow_u: f64,
    /// Erreur de `delta.rs` sur la surface libre : `max|η − η₀|`.
    pub delta_eta: f64,
    /// Erreur de `shallow.rs` sur la surface libre.
    pub shallow_eta: f64,
    /// Durée simulée, en secondes.
    pub duree_s: f64,
}

impl Plancher {
    /// La borne sur la vitesse : aucun désaccord en `u` sous cette valeur n'est interprétable.
    pub fn borne_u(&self) -> f64 {
        self.delta_u + self.shallow_u
    }
    /// La même, sur la surface libre.
    pub fn borne_eta(&self) -> f64 {
        self.delta_eta + self.shallow_eta
    }
}

/// Mesure le plancher sur C01 au repos, après `duree_s` de simulation.
///
/// Les deux schémas sont réglés **équilibrés** — c'est la propriété que C01 teste, et le seul
/// réglage où la solution exacte est censée être reproduite. Le schéma naïf, lui, s'écarte de 19,5 mm/s
/// (`ADR-038` §2) : il ne mesurerait pas un plancher, il mesurerait son propre défaut.
pub fn plancher_c01(duree_s: f64) -> Plancher {
    let (mut d, mut s) = avec_hote(1 << 20, |h| montages_c01(h));
    s.regler_equilibrage(true);
    d.avancer_equilibre(duree_s);
    s.avancer_jusqu_a(duree_s, 0.45);
    Plancher {
        delta_u: d.max_abs_u(),
        shallow_u: s.vitesse_max(),
        delta_eta: d.max_ecart_eta(),
        shallow_eta: s.ecart_surface_max(0.0),
        duree_s,
    }
}

/// Le résultat d'une confrontation croisée sur un montage donné.
pub struct Confrontation {
    pub nom: &'static str,
    pub duree_s: f64,
    pub hauteur: Ecart,
    pub surface: Ecart,
    pub vitesse: Ecart,
    /// Nombre de pas effectués de chaque côté — ils diffèrent, et c'est attendu.
    pub pas: (u64, u64),
}

impl Confrontation {
    /// Imprime les trois écarts sur une ligne chacun.
    pub fn rapporter(&self) {
        println!(
            "  {} à t = {:.1} s — pas : {} (f32) contre {} (f64)",
            self.nom, self.duree_s, self.pas.0, self.pas.1
        );
        for (champ, e) in [
            ("hauteur", &self.hauteur),
            ("surface", &self.surface),
            ("vitesse", &self.vitesse),
        ] {
            println!(
                "      {champ:<8} L∞ = {:>11.4e}   L¹ = {:>11.4e}   (cellule {})",
                e.linf, e.l1, e.i_max
            );
        }
    }
}

/// **Confronte les deux véhicules sur C01**, au même temps final.
///
/// Les deux sont réglés **équilibrés** : c'est la propriété que C01 teste. Chacun avance à son
/// propre pas de temps — `dt_cfl` est calculé dans deux précisions et diverge dès le premier pas —
/// et seuls les **états à `duree_s`** sont comparés.
pub fn confronter_c01(duree_s: f64) -> Confrontation {
    let (mut d, mut s) = avec_hote(1 << 20, |h| montages_c01(h));
    s.regler_equilibrage(true);
    let pas_d = d.avancer_equilibre(duree_s);
    let pas_s = s.avancer_jusqu_a(duree_s, 0.45);
    let (cd, cs) = (champs_delta(&d), champs_shallow(&s));
    Confrontation {
        nom: "C01",
        duree_s,
        hauteur: Ecart::entre(&cd.hauteur, &cs.hauteur),
        surface: Ecart::entre(&cd.surface, &cs.surface),
        vitesse: Ecart::entre(&cd.vitesse, &cs.vitesse),
        pas: (pas_d, pas_s),
    }
}

/// **Confronte les deux véhicules sur C04** — rupture de barrage, montage aligné.
///
/// # L'alignement, et ce qu'il a fallu regarder
///
/// `Bassin::c04()` et `Shallow1D::configure_barrage` décrivent **le même montage** : 40 m, 800
/// cellules de 5 cm, `h₀ = 1 m` à gauche, lit sec à droite, barrage entre les cellules 399 et 400.
/// Les origines diffèrent — `[−20, +20]` d'un côté, `[0, 40]` de l'autre — mais **les indices se
/// correspondent**, ce que le test vérifie à `t = 0`.
///
/// **Un réglage a dû être changé, et c'est le point délicat de cette confrontation.** La lignée B
/// monte C04 avec le flux **HLL** (`physics_shallow::barrage`), choisi en B-S22 pour les états secs ;
/// `delta.rs` n'a que **Rusanov**. Comparer les deux tels quels mesurerait la différence entre deux
/// **flux**, pas entre deux implémentations du même schéma — et l'oracle ne dirait rien de ce qu'il
/// prétend dire. `shallow.rs` est donc remis sur **Rusanov**, à l'ordre un, comme `delta.rs`.
///
/// Le paramètre `hll` permet l'autre comparaison, qui n'est pas un oracle mais une mesure : *ce que
/// le changement de flux déplace*.
pub fn confronter_c04(duree_s: f64, hll: bool) -> Confrontation {
    use water_core::Flux;
    let (mut d, mut s) = avec_hote(1 << 22, |h| {
        let d = Delta1D::configure(h, Bassin::c04()).expect("configuration de delta.rs");
        let s = Shallow1D::configure_barrage(h, 800, 0.05, 1.0).expect("configuration de shallow.rs");
        (d, s)
    });
    s.regler_flux(if hll { Flux::Hll } else { Flux::Rusanov });
    let pas_d = d.avancer_equilibre(duree_s);
    let pas_s = s.avancer_jusqu_a(duree_s, 0.45);
    let (cd, cs) = (champs_delta(&d), champs_shallow(&s));
    Confrontation {
        nom: if hll { "C04 (HLL contre Rusanov)" } else { "C04" },
        duree_s,
        hauteur: Ecart::entre(&cd.hauteur, &cs.hauteur),
        surface: Ecart::entre(&cd.surface, &cs.surface),
        vitesse: Ecart::entre(&cd.vitesse, &cs.vitesse),
        pas: (pas_d, pas_s),
    }
}

/// **La zone où les deux véhicules ne s'accordent pas sur le sens du mot « sec ».**
///
/// `delta.rs` déclare une cellule sèche sous [`water_core::delta::H_SEC`], `shallow.rs` sous
/// [`water_core::shallow::H_SEC_DEFAUT`]. **Quatre ordres de grandeur les séparent**, et une cellule
/// dont la hauteur tombe entre les deux est sèche pour l'un et mouillée pour l'autre.
///
/// **S40 a mesuré ce que cet écart déplace : rien de publiable.** Le front bouge de 0,148 % sur
/// sept décades, le volume pas du tout. Ce qui diverge est `hu/h` dans le film — une quantité qui
/// dépasse la vitesse du front de Ritter et varie d'un facteur 2,5 avec le seuil. `ADR-047` D3 :
/// **`u` sous le seuil n'est pas une grandeur publiable**, et D4 fait de l'exclusion ci-dessous la
/// règle plutôt qu'un correctif d'enquête.
///
/// Les deux valeurs sont **lues chez leurs propriétaires** et non recopiées : deux copies d'une
/// constante se périment en silence (**L141**), et celle de `shallow.rs` est réglable depuis S40.
pub const SEC_DELTA: f64 = water_core::delta::H_SEC as f64;

/// Le seuil par défaut de `shallow.rs`. Réglable par cellule d'essai depuis S40 — c'est ce qui a
/// rendu le balayage possible.
pub const SEC_SHALLOW: f64 = water_core::shallow::H_SEC_DEFAUT;

/// Écart de vitesse restreint aux cellules **franchement mouillées des deux côtés**.
///
/// « Franchement » veut dire au-dessus du plus grand des deux seuils : là, les deux véhicules
/// s'accordent à dire que la cellule porte de l'eau, et leur vitesse est comparable. Rend aussi le
/// nombre de cellules **litigieuses** — celles dont la hauteur tombe entre les deux seuils.
pub fn ecart_vitesse_hors_zone_seche(cd: &Champs, cs: &Champs) -> (Ecart, usize) {
    let (mut a, mut b, mut litigieuses) = (Vec::new(), Vec::new(), 0usize);
    for i in 0..cd.hauteur.len() {
        let (hd, hs) = (cd.hauteur[i], cs.hauteur[i]);
        let mouille = hd > SEC_DELTA && hs > SEC_DELTA;
        if hd.min(hs) > SEC_SHALLOW && !mouille {
            litigieuses += 1;
        }
        if mouille {
            a.push(cd.vitesse[i]);
            b.push(cs.vitesse[i]);
        }
    }
    (Ecart::entre(&a, &b), litigieuses)
}

/// Les champs des deux véhicules sur C04, après `duree_s`, à flux et ordre égaux.
pub fn champs_c04(duree_s: f64) -> (Champs, Champs) {
    use water_core::Flux;
    let (mut d, mut s) = avec_hote(1 << 22, |h| {
        let d = Delta1D::configure(h, Bassin::c04()).expect("configuration de delta.rs");
        let s = Shallow1D::configure_barrage(h, 800, 0.05, 1.0).expect("configuration de shallow.rs");
        (d, s)
    });
    s.regler_flux(Flux::Rusanov);
    d.avancer_equilibre(duree_s);
    s.avancer_jusqu_a(duree_s, 0.45);
    (champs_delta(&d), champs_shallow(&s))
}

/// Ce que les deux véhicules ont saturé sur un même cas, avec le volume qui donne l'échelle.
pub struct Bilan {
    pub cas: &'static str,
    pub delta: water_core::Saturations,
    pub shallow: water_core::Saturations,
    /// Volume initial, en m² — la référence à laquelle rapporter la masse créée.
    pub volume0: f64,
    pub pas: (u64, u64),
    pub cellules: usize,
}

impl Bilan {
    pub fn rapporter(&self) {
        println!("  {} — {} cellules, {} / {} pas", self.cas, self.cellules, self.pas.0, self.pas.1);
        for (nom, s, pas) in [
            ("delta.rs  (f32)", &self.delta, self.pas.0),
            ("shallow.rs (f64)", &self.shallow, self.pas.1),
        ] {
            let occasions = (pas as f64) * (self.cellules as f64);
            println!(
                "      {nom} — état {:>8}  ({:>9.3e} des occasions)   masse créée {:>11.4e} m² ({:>9.3e} du volume)   qdm {:>11.4e}   étage RK2 {:>6}   bord {:>8}   h<0 en entrée {}",
                s.etat,
                if occasions > 0.0 { s.etat as f64 / occasions } else { 0.0 },
                s.masse_creee,
                s.masse_creee / self.volume0,
                s.qdm_detruite,
                s.etage_rk2,
                s.bord,
                s.h_negatif_en_entree
            );
        }
    }
}

/// Bilan des saturations sur **C01** — repos sur pente, domaine entièrement mouillé.
pub fn bilan_c01(duree_s: f64) -> Bilan {
    let (mut d, mut s) = avec_hote(1 << 20, |h| montages_c01(h));
    s.regler_equilibrage(true);
    let v0 = d.volume();
    let pas_d = d.avancer_equilibre(duree_s);
    let pas_s = s.avancer_jusqu_a(duree_s, 0.45);
    Bilan {
        cas: "C01 — repos sur pente",
        delta: d.saturations(),
        shallow: s.saturations(),
        volume0: v0,
        pas: (pas_d, pas_s),
        cellules: d.nx(),
    }
}

/// Bilan des saturations sur **C03** — seiche en bassin clos, fond plat, régime linéaire.
pub fn bilan_c03(periodes: f64) -> Bilan {
    let (mut d, mut s) = avec_hote(1 << 22, |h| {
        let d = Delta1D::configure(h, Bassin::c03(false)).expect("delta");
        let s = Shallow1D::configure_seiche(h, 400, 0.05, 2.0, 0.02).expect("shallow");
        (d, s)
    });
    s.regler_equilibrage(true);
    let v0 = d.volume();
    let t = periodes * 2.0 * 20.0 / (G * 2.0f64).sqrt();
    let pas_d = d.avancer_equilibre(t);
    let pas_s = s.avancer_jusqu_a(t, 0.45);
    Bilan {
        cas: "C03 — seiche",
        delta: d.saturations(),
        shallow: s.saturations(),
        volume0: v0,
        pas: (pas_d, pas_s),
        cellules: d.nx(),
    }
}

/// Bilan des saturations sur **C04** — rupture de barrage sur lit sec. Le seul où la saturation
/// d'état est **attendue**.
pub fn bilan_c04(duree_s: f64) -> Bilan {
    use water_core::Flux;
    let (mut d, mut s) = avec_hote(1 << 22, |h| {
        let d = Delta1D::configure(h, Bassin::c04()).expect("delta");
        let s = Shallow1D::configure_barrage(h, 800, 0.05, 1.0).expect("shallow");
        (d, s)
    });
    s.regler_flux(Flux::Rusanov);
    let v0 = d.volume();
    let pas_d = d.avancer_equilibre(duree_s);
    let pas_s = s.avancer_jusqu_a(duree_s, 0.45);
    Bilan {
        cas: "C04 — rupture sur lit sec",
        delta: d.saturations(),
        shallow: s.saturations(),
        volume0: v0,
        pas: (pas_d, pas_s),
        cellules: d.nx(),
    }
}

/// **Le cas que la saturation doit attraper** — C04 mené au-delà de sa condition de stabilité.
///
/// Les mesures de P5 ne trouvent **aucun** déclenchement en régime nominal, sur aucun des trois cas.
/// C'est rassurant — et c'est exactement la situation que S34 décrit comme la plus dangereuse :
/// *un garde-fou qu'on n'a jamais vu déclencher n'a pas été testé* (**L118**). Rien ne dit qu'il
/// protège de ce qu'il prétend protéger, ni même qu'il puisse s'activer.
///
/// Le flux de Rusanov préserve la positivité **sous** sa condition de Courant. Au-delà, il ne la
/// préserve plus : c'est le levier, et il est physique plutôt qu'artificiel. `cfl` monte au-dessus
/// de 1 ; la saturation doit alors mordre, et **elle doit être vue mordre**.
pub fn declenchement_c04(cfl: f64, duree_s: f64) -> Bilan {
    use water_core::Flux;
    let (d, mut s) = avec_hote(1 << 22, |h| {
        let d = Delta1D::configure(h, Bassin::c04()).expect("delta");
        let s = Shallow1D::configure_barrage(h, 800, 0.05, 1.0).expect("shallow");
        (d, s)
    });
    let mut d = d.avec_cfl(cfl as f32);
    s.regler_flux(Flux::Rusanov);
    let v0 = d.volume();
    let pas_d = d.avancer_equilibre(duree_s);
    let pas_s = s.avancer_jusqu_a(duree_s, cfl);
    Bilan {
        cas: "C04 hors CFL",
        delta: d.saturations(),
        shallow: s.saturations(),
        volume0: v0,
        pas: (pas_d, pas_s),
        cellules: 800,
    }
}

/// **Ce qu'un seuil de sec déplace, grandeur par grandeur** — S40, angle mort **A163**.
///
/// `ADR-031` §4 avait balayé `H_SEC` sur six décades côté `delta.rs` et conclu que **la valeur est
/// libre** : 0,25 point d'effet sur seize, sur la position du front. S37 a mesuré, sur le même cas,
/// **6,16 m/s** d'écart de vitesse imputable à ce même seuil.
///
/// **Les deux sont vraies et ne parlent pas de la même grandeur.** Ce relevé les met côte à côte :
/// pour un seuil donné, ce que voient les grandeurs **publiées** de C04, et ce que voit `max|u|`,
/// que rien ne publie.
pub struct Sensibilite {
    pub seuil: f64,
    /// Position du front, mesurée au seuil **de mesure** de C04 — `10⁻²·h₀`, sans rapport avec le
    /// seuil du solveur. Deux objets, deux noms (**L148**).
    pub front: f64,
    /// Hauteur au droit du barrage.
    pub h0: f64,
    /// Volume total — la conservation, que rien ne devrait toucher.
    pub volume: f64,
    /// `max|u|` sur tout le domaine. **Aucun cas canonique ne la publie.**
    pub u_max: f64,
    /// Longueur du film : cellules à `0 < h < 10⁻⁶`.
    pub film: usize,
}

/// Balaie le seuil de sec sur `delta.rs`, montage C04.
pub fn sensibilite_delta(seuils: &[f64], duree_s: f64) -> Vec<Sensibilite> {
    seuils
        .iter()
        .map(|&sec| {
            let mut d = avec_hote(1 << 22, |h| {
                Delta1D::configure(h, Bassin::c04()).expect("delta")
            })
            .avec_h_sec(sec as f32);
            d.avancer_equilibre(duree_s);
            let c = champs_delta(&d);
            let x_barrage = c.x[400] - 0.5 * 0.05;
            Sensibilite {
                seuil: sec,
                front: d.front(1e-2).map(|x| x as f64 - x_barrage).unwrap_or(f64::NAN),
                h0: c.hauteur[400],
                volume: d.volume(),
                u_max: c.vitesse.iter().fold(0.0f64, |a, b| a.max(b.abs())),
                film: c.hauteur.iter().filter(|&&h| h > 0.0 && h < SEC_DELTA).count(),
            }
        })
        .collect()
}

/// Le même balayage sur `shallow.rs`, seuil paramétrable depuis S40.
pub fn sensibilite_shallow(seuils: &[f64], duree_s: f64) -> Vec<Sensibilite> {
    use water_core::Flux;
    seuils
        .iter()
        .map(|&sec| {
            let mut s = avec_hote(1 << 22, |h| {
                Shallow1D::configure_barrage(h, 800, 0.05, 1.0).expect("shallow")
            });
            s.regler_flux(Flux::Rusanov);
            s.regler_h_sec(sec);
            s.avancer_jusqu_a(duree_s, 0.45);
            let c = champs_shallow(&s);
            let x_barrage = c.x[400] - 0.5 * 0.05;
            Sensibilite {
                seuil: sec,
                front: s.front_mouille(1e-2).map(|x| x - x_barrage).unwrap_or(f64::NAN),
                h0: c.hauteur[400],
                volume: s.volume(),
                u_max: c.vitesse.iter().fold(0.0f64, |a, b| a.max(b.abs())),
                film: c.hauteur.iter().filter(|&&h| h > 0.0 && h < SEC_DELTA).count(),
            }
        })
        .collect()
}

/// Imprime un balayage, avec l'écart relatif de chaque grandeur à sa valeur au seuil de référence.
pub fn rapporter_sensibilite(nom: &str, v: &[Sensibilite]) {
    println!("  {nom} — C04 à t = 2 s, balayage du seuil de sec du solveur :");
    println!("      seuil        front (m)    Δfront     h(0) (m)     volume       Δvolume      max|u| (m/s)   film");
    let r = &v[0];
    for s in v {
        println!(
            "      {:>8.0e}   {:>9.5}   {:>8.4} %   {:>8.5}   {:>10.6}   {:>9.2e}   {:>11.4}   {:>4}",
            s.seuil,
            s.front,
            (s.front - r.front).abs() / r.front.abs() * 100.0,
            s.h0,
            s.volume,
            (s.volume - r.volume).abs() / r.volume.abs(),
            s.u_max,
            s.film
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **P5 — les saturations de modèle, comptées pour la première fois** (S34-1, **A146**).
    ///
    /// Trois cas, deux véhicules, et les prédictions écrites en P2 avant toute mesure :
    /// C01 et C03 ne doivent **jamais** déclencher la saturation d'état ; C04 doit la déclencher,
    /// mais de façon **localisée** au front.
    #[test]
    fn les_saturations_de_modele_comptees() {
        let c01 = bilan_c01(60.0);
        c01.rapporter();
        let c03 = bilan_c03(20.0);
        c03.rapporter();
        let c04 = bilan_c04(2.0);
        c04.rapporter();

        // **Volet 1** : domaine entièrement mouillé, régime linéaire. Un seul déclenchement serait
        // un défaut, pas un filet.
        for b in [&c01, &c03] {
            assert_eq!(b.delta.etat, 0, "{} : delta.rs a saturé {} fois", b.cas, b.delta.etat);
            assert_eq!(b.shallow.etat, 0, "{} : shallow.rs a saturé {} fois", b.cas, b.shallow.etat);
        }

        // **Le témoin de la protection de racine (S6)** : si une cellule interne arrivait à `h < 0`
        // en entrée de pas, le `(g·h)⁺` des flux mordrait. Il ne le doit jamais.
        for b in [&c01, &c03, &c04] {
            assert_eq!(
                b.delta.h_negatif_en_entree, 0,
                "{} : la protection de racine a mordu — elle n'est donc pas morte, et elle masque",
                b.cas
            );
        }
    }

    /// **La saturation peut-elle seulement se déclencher ?**
    ///
    /// P5 n'a trouvé **aucun** déclenchement en régime nominal. Deux lectures sont alors possibles
    /// et **indiscernables sans ce test** : soit le schéma ne produit jamais d'état impossible — et
    /// la saturation est un filet qui ne sert jamais — soit elle est écrite avec une condition qui
    /// ne peut pas être vraie, et elle ne protège de rien.
    ///
    /// Le test d'une saturation est **le cas qu'elle doit attraper**, jamais le cas nominal (L119).
    /// Ici : au-delà de la condition de Courant, Rusanov cesse de préserver la positivité.
    #[test]
    fn la_saturation_est_vue_mordre_hors_cfl() {
        let mut vue_delta = false;
        let mut vue_shallow = false;
        for cfl in [0.45f64, 0.95, 1.2, 1.8] {
            let b = declenchement_c04(cfl, 0.5);
            println!("  CFL = {cfl:.2}");
            b.rapporter();
            if cfl < 1.0 {
                // **Le témoin.** Sous la condition de Courant, Rusanov préserve la positivité :
                // aucun déclenchement ne doit avoir lieu. Sans ce volet, une saturation écrite
                // « toujours vraie » passerait le test de déclenchement (L119).
                assert_eq!(b.delta.etat, 0, "delta.rs sature à CFL = {cfl}, sous la condition");
                assert_eq!(b.shallow.etat, 0, "shallow.rs sature à CFL = {cfl}, sous la condition");
            } else {
                vue_delta |= b.delta.etat > 0;
                vue_shallow |= b.shallow.etat > 0;
                // **Et ce que la saturation ne fait pas.** Elle mord, mais elle ne rattrape rien :
                // la masse créée dépasse le volume initial de plusieurs ordres de grandeur. Un
                // filet qui laisse passer `10¹⁷` fois le volume n'est pas un filet — c'est un
                // témoin de divergence, et il est muet tant que personne ne le lit.
                assert!(
                    b.delta.masse_creee > b.volume0,
                    "à CFL = {cfl}, la masse créée devrait dépasser le volume initial"
                );
            }
        }
        assert!(
            vue_delta,
            "la saturation de delta.rs n'a jamais été vue mordre, même hors CFL : elle est soit inatteignable, soit mal conditionnée"
        );
        assert!(
            vue_shallow,
            "la saturation de shallow.rs n'a jamais été vue mordre, même hors CFL"
        );
    }

    /// **P6 — le résidu de `10⁻¹⁰ m` vient-il de la saturation ?** (action **S37-3**)
    ///
    /// S37 a trouvé qu'à la cellule fautive de C04, `delta.rs` porte **zéro exactement** et
    /// `shallow.rs` un film de `1,05·10⁻¹⁰ m`. La saturation était le suspect naturel : c'est elle qui
    /// écrit des zéros.
    ///
    /// **La réponse est non, et elle est nette** : les compteurs de S38 montrent **zéro
    /// déclenchement** sur C04 en régime nominal, des deux côtés. Aucune des deux valeurs n'a été
    /// écrite par une saturation — ni le zéro de `delta.rs`, ni le film de `shallow.rs`.
    ///
    /// Ce test établit la vraie cause à la place : **le seuil de sec gouverne aussi le flux**. Une
    /// cellule à `h = 5·10⁻⁷` est sèche pour `delta.rs`, qui lui donne `u = 0` et cesse donc de
    /// transporter ; elle est mouillée pour `shallow.rs`, qui continue à pousser de la matière dans
    /// le film. Le résidu n'est pas un déchet laissé derrière le front : **c'est du transport que
    /// l'autre véhicule a déjà arrêté**. Encore **A163**.
    #[test]
    fn le_residu_ne_vient_pas_de_la_saturation_mais_du_seuil() {
        let b = bilan_c04(2.0);
        assert_eq!(b.delta.etat, 0, "C04 nominal : delta.rs ne doit pas saturer");
        assert_eq!(b.shallow.etat, 0, "C04 nominal : shallow.rs ne doit pas saturer");

        let (cd, cs) = champs_c04(2.0);
        let compte = |c: &Champs, bas: f64, haut: f64| {
            c.hauteur.iter().filter(|&&h| h > bas && h < haut).count()
        };
        let film_delta = compte(&cd, 0.0, SEC_DELTA);
        let film_shallow = compte(&cs, 0.0, SEC_DELTA);
        let secs_delta = cd.hauteur.iter().filter(|&&h| h == 0.0).count();
        let secs_shallow = cs.hauteur.iter().filter(|&&h| h == 0.0).count();
        println!(
            "C04 à t=2 s — cellules à `0 < h < 10⁻⁶` : delta.rs {film_delta}, shallow.rs {film_shallow}  |  cellules à h = 0 exactement : delta.rs {secs_delta}, shallow.rs {secs_shallow}"
        );

        // **Le film existe des deux côtés, et c'est un troisième résultat que la session n'attendait
        // pas.** La prédiction était que `delta.rs` n'aurait **aucune** cellule sous son propre
        // seuil : il en a **trois**. Le seuil de sec ne coupe que la **vitesse** — `u = 0` sous
        // `h_sec` — et non le **flux de masse** : la diffusion de Rusanov, `α·(h_R − h_L)`, continue
        // à déposer de la matière dans une cellule déclarée sèche, même quand les deux vitesses sont
        // nulles.
        //
        // *Un seuil de sec ne assèche pas une cellule : il l'empêche seulement de bouger.*
        //
        // Ce qui reste vrai, et qui suffit à établir la cause : le film de `shallow.rs` est **six
        // fois plus long**, dans le rapport qu'on attend de seuils séparés par quatre ordres de
        // grandeur.
        assert!(
            film_shallow > 3 * film_delta,
            "le film de shallow.rs devrait être nettement plus long : {film_shallow} contre {film_delta}"
        );
        assert!(
            film_delta > 0,
            "delta.rs porte lui aussi un film : son seuil coupe la vitesse, pas le flux de masse"
        );
    }

    /// **Ce que l'oracle a trouvé : les deux véhicules ne définissent pas « sec » pareil.**
    ///
    /// Sur C04 à flux et ordre égaux, la **hauteur** concorde à `6,5·10⁻⁴ m` sur `h₀ = 1 m` — six
    /// pour dix mille. La **vitesse**, elle, diverge de plus de **6 m/s** sur une cellule, alors que
    /// son écart moyen reste à `7·10⁻²`. Un écart aussi localisé n'est pas une divergence de schéma :
    /// c'est un point où les deux codes ne répondent pas à la même question.
    ///
    /// La cause est un seuil : `H_SEC = 10⁻⁶` pour `delta.rs`, `10⁻¹⁰` pour `shallow.rs`. Une cellule
    /// entre les deux est sèche pour l'un, mouillée pour l'autre, et `hu/h` sur un film pareil rend
    /// n'importe quoi.
    ///
    /// **Ce test le démontre plutôt que de l'affirmer** : en écartant les cellules litigieuses,
    /// l'écart de vitesse doit s'effondrer de plusieurs ordres de grandeur. S'il ne s'effondrait
    /// pas, l'explication serait fausse et il faudrait chercher une vraie divergence de schéma.
    #[test]
    fn c04_le_desaccord_de_vitesse_vient_du_seuil_de_sec() {
        let (cd, cs) = champs_c04(2.0);
        let brut = Ecart::entre(&cd.vitesse, &cs.vitesse);
        let (hors_zone, litigieuses) = ecart_vitesse_hors_zone_seche(&cd, &cs);
        println!(
            "C04 vitesse — toutes cellules : L∞ = {:.4e} (cellule {})  |  hors zone sèche : L∞ = {:.4e}  |  {litigieuses} cellule(s) litigieuse(s)",
            brut.linf, brut.i_max, hors_zone.linf
        );
        println!(
            "C04 hauteur — à la cellule {} : delta.rs = {:.4e} m, shallow.rs = {:.4e} m",
            brut.i_max, cd.hauteur[brut.i_max], cs.hauteur[brut.i_max]
        );
        // Le critère a une échelle physique et non un facteur choisi : `2c₀ = 2√(g·h₀)` est la
        // vitesse du front de Ritter, la plus grande que ce montage puisse produire. Un **écart**
        // entre deux solveurs qui atteint presque cette valeur ne décrit pas une divergence de
        // schéma : il décrit un point où l'un des deux répond à une autre question.
        let c2 = 2.0 * (G * 1.0f64).sqrt();
        println!(
            "C04 vitesse — rapporté à 2c₀ = {c2:.3} m/s : brut {:.1} %, hors zone sèche {:.1} %",
            brut.linf / c2 * 100.0,
            hors_zone.linf / c2 * 100.0
        );
        assert!(
            brut.linf > 0.9 * c2,
            "le désaccord brut devrait être du même ordre que la vitesse maximale du montage"
        );
        assert!(
            hors_zone.linf < 0.05 * c2,
            "hors de la zone litigieuse, l'écart devrait retomber sous 5 % de 2c₀ : {:.4e} m/s",
            hors_zone.linf
        );
        assert!(
            (1..=8).contains(&litigieuses),
            "le désaccord doit être localisé sur quelques cellules, pas diffus : {litigieuses}"
        );

        // **Et le fait le plus net de toute la confrontation.** À la cellule fautive, `delta.rs`
        // porte **zéro exactement** et `shallow.rs` un film de `10⁻¹⁰ m` — un dixième de nanomètre,
        // soit moins qu'un atome. Ce n'est pas le même désaccord que le précédent : celui-là porte
        // sur la **lecture** de la vitesse, celui-ci sur ce que le schéma **laisse** derrière le
        // front. `shallow.rs` n'écrase jamais ce résidu ; `delta.rs` le remet à zéro.
        assert_eq!(
            cd.hauteur[brut.i_max], 0.0,
            "delta.rs devrait porter zéro exactement à la cellule litigieuse"
        );
        assert!(
            cs.hauteur[brut.i_max] > 0.0 && cs.hauteur[brut.i_max] < SEC_DELTA,
            "shallow.rs devrait y porter un film sous le seuil de delta.rs : {:.4e}",
            cs.hauteur[brut.i_max]
        );
    }

    /// **P6 — C04 confronté : le cas où un désaccord serait physique.**
    ///
    /// Contrairement à C01, la solution du **schéma** n'est pas connue ici : Ritter est la solution
    /// exacte de l'équation, pas de sa discrétisation. Chaque véhicule a donc une erreur de
    /// troncature de quelques pour cent, et **l'oracle croisé est le seul instrument qui puisse dire
    /// si les deux commettent la même**. C'est exactement l'usage annoncé par `ADR-043` §3.
    ///
    /// D'abord la vérification préalable : à `t = 0`, les deux montages doivent être identiques
    /// **index à index**, malgré des origines qui diffèrent de 20 m.
    #[test]
    fn c04_les_deux_montages_sont_alignes_a_t_zero() {
        let c = confronter_c04(0.0, false);
        c.rapporter();
        assert_eq!(c.hauteur.linf, 0.0, "le barrage n'est pas au même endroit");
        assert_eq!(c.vitesse.linf, 0.0, "l'état initial n'est pas au repos des deux côtés");
    }

    /// **P6 — et après deux secondes ?**
    ///
    /// Le seuil est celui de la troncature, pas celui de l'arithmétique : `h₀ = 1 m`, et deux
    /// schémas d'ordre un sur un front discontinu s'écartent de la solution exacte de quelques
    /// pour cent. **S'ils s'écartent l'un de l'autre du même ordre, c'est qu'ils ne calculent pas
    /// la même chose** — et ce serait le résultat que l'oracle est fait pour trouver.
    ///
    /// Le second appel mesure ce que le changement de **flux** déplace. Ce n'est pas un oracle :
    /// c'est la référence qui donne son échelle au premier.
    #[test]
    fn c04_les_deux_vehicules_confrontes() {
        let meme_flux = confronter_c04(2.0, false);
        meme_flux.rapporter();
        let autre_flux = confronter_c04(2.0, true);
        autre_flux.rapporter();
        println!(
            "      rapport hauteur — même flux : {:.4e} | flux différents : {:.4e} | facteur {:.2}",
            meme_flux.hauteur.linf,
            autre_flux.hauteur.linf,
            autre_flux.hauteur.linf / meme_flux.hauteur.linf.max(1e-30)
        );
        assert!(
            meme_flux.hauteur.linf < autre_flux.hauteur.linf,
            "à flux égal les deux devraient concorder mieux qu'à flux différent : {:.4e} contre {:.4e}",
            meme_flux.hauteur.linf,
            autre_flux.hauteur.linf
        );
    }

    /// **P2 — l'essai à zéro de C03** (**A167**, action S41-4).
    ///
    /// *Tout montage de mesure doit venir avec un essai dont le résultat attendu est zéro.* Pour
    /// C03, il s'écrit sans une ligne de montage nouvelle : **le même bassin, sans excitation**
    /// (`eta_bord = 0`). L'eau est plate et au repos ; il n'y a pas de seiche, donc **pas de
    /// demi-vie à mesurer**.
    ///
    /// Ce test **constate** ce que la mesure rend aujourd'hui, avant toute correction.
    #[test]
    fn c03_essai_a_zero_ce_que_la_mesure_rend_sur_un_bassin_au_repos() {
        use crate::physics_shallow::{demi_vie_seiche, SCHEMAS, SCHEMA_RETENU};
        println!("C03 — essai à zéro : bassin plat, `eta_bord = 0`, aucune seiche");
        for (nom, sc) in [("ordre 1", SCHEMAS[0]), ("ordre 2", SCHEMA_RETENU)] {
            let dv = demi_vie_seiche(400, 0.05, 2.0, 0.0, 20.0, sc);
            println!("      {nom:<8} — demi-vie mesurée sur le néant : {dv:>12.3} périodes");
        }
        // Aucune assertion ici : ce test est un relevé, et sa valeur est dans ce qu'il imprime.
        // Le refus, s'il en faut un, est écrit en P3 avec son témoin.
    }

    /// **P2 (suite) — et ce que le CAS en fait.**
    ///
    /// `demi_vie_seiche` rend `INFINITY` sur un bassin au repos — non par refus délibéré, mais
    /// parce que le filtre `pic > 0` ne laisse passer aucun point et que la régression sur zéro
    /// point rend `NaN`, dont la comparaison `< 0` est fausse.
    ///
    /// **La question n'est pas là.** Elle est : que devient cette valeur en traversant le cas ?
    #[test]
    fn c03_essai_a_zero_le_cas_declare_t_il_le_neant_conforme() {
        use crate::physics_shallow::c03_seiche;
        let cas = c03_seiche(400, 0.05, 2.0, 0.0, 20.0);
        println!("C03 — essai à zéro : verdicts du CAS sur un bassin sans seiche");
        for c in &cas {
            println!(
                "      {:<14} {:<50} mesuré {:>14.4}  référence {:>10.4}  {}",
                c.id,
                c.grandeur,
                c.mesure,
                c.reference,
                if c.passe() { "PASSE" } else { "échoue" }
            );
        }
        let dv = cas.iter().find(|c| c.id == "C03-demi-vie").expect("C03-demi-vie");
        println!(
            "      → sur un montage VIDE, C03-demi-vie rend {:.0} et {}",
            dv.mesure,
            if dv.passe() { "**PASSE**" } else { "échoue" }
        );
    }

    /// **P3 — le refus, et son témoin** (**L119**).
    ///
    /// *Le test d'un garde-fou est le cas qu'il doit refuser, jamais le cas nominal.* Et il lui
    /// faut aussi son **témoin**, le cas sain qu'il ne doit pas refuser — sans quoi un refus écrit
    /// « toujours vrai » passerait le premier volet.
    ///
    /// | | attendu |
    /// |---|---|
    /// | bassin **sans seiche** (`eta_bord = 0`) | **refus** — `NaN` |
    /// | bassin excité (`eta_bord = 0,02`) | **pas de refus**, et la valeur publiée |
    #[test]
    fn c03_la_demi_vie_refuse_un_bassin_sans_seiche() {
        use crate::physics_shallow::{demi_vie_seiche, SCHEMAS, SCHEMA_RETENU};

        // **Le cas refusé.** Aucune seiche : il n'y a rien à mesurer, et la fonction doit le dire.
        for (nom, sc) in [("ordre 1", SCHEMAS[0]), ("ordre 2", SCHEMA_RETENU)] {
            let dv = demi_vie_seiche(400, 0.05, 2.0, 0.0, 20.0, sc);
            println!("C03 essai à zéro — {nom} : {dv}");
            assert!(
                dv.is_nan(),
                "un bassin sans seiche doit être refusé, pas mesuré : {nom} rend {dv}"
            );
        }

        // **Le témoin.** Le montage nominal doit passer, et rendre exactement la valeur publiée par
        // `ADR-040` §5 — un refus mal placé la ferait disparaître sans que rien ne le dise.
        let dv = demi_vie_seiche(400, 0.05, 2.0, 0.02, 20.0, SCHEMA_RETENU);
        println!("C03 témoin — bassin excité, ordre 2 : {dv:.2} périodes (publié 161,14)");
        assert!(dv.is_finite(), "le montage nominal ne doit pas être refusé : {dv}");
        assert!(
            (dv - 161.14).abs() / 161.14 < 0.001,
            "le refus ne doit déplacer aucun chiffre publié : {dv:.3} contre 161,14"
        );
    }

    /// **P3 (suite) — et le cas déclare-t-il encore le néant conforme ?**
    ///
    /// C'était le défaut : `C03-demi-vie` rendait `10⁶` sur un montage vide et **passait**. Les deux
    /// autres assertions du cas échouaient déjà — le cas entier était rouge — mais l'assertion qui
    /// **porte le résultat publié** déclarait le néant excellent.
    #[test]
    fn c03_le_neant_n_est_plus_declare_conforme() {
        use crate::physics_shallow::c03_seiche;
        let cas = c03_seiche(400, 0.05, 2.0, 0.0, 20.0);
        for c in &cas {
            println!(
                "C03 essai à zéro — {:<14} mesuré {:>12.4}  {}",
                c.id,
                c.mesure,
                if c.passe() { "PASSE" } else { "échoue" }
            );
        }
        assert!(
            cas.iter().all(|c| !c.passe()),
            "aucune assertion de C03 ne doit passer sur un bassin sans seiche"
        );
    }

    /// **P5 — l'essai à zéro de C06** (**A167**).
    ///
    /// C06 mesure l'invariance galiléenne : une bosse advectée à `u₀` doit être, au décalage près,
    /// la même que la bosse au repos. Son essai à zéro s'écrit en changeant **un** paramètre :
    /// `u₀ = 0`. Les deux simulations sont alors **le même calcul**, et l'écart doit être
    /// exactement nul — pas petit, **nul**.
    ///
    /// C'est le meilleur genre d'essai à zéro : il n'exerce pas une tolérance, il exerce
    /// l'**identité**. Un écart non nul y dirait que le montage compare deux choses qui diffèrent
    /// avant même qu'on ait bougé.
    #[test]
    fn c06_essai_a_zero_sans_boost_l_ecart_est_nul() {
        use crate::physics_shallow::c06_galilee;
        let cas = c06_galilee(2000, 0.05, 2.0, 0.1, 0.0, 1.0);
        for c in &cas {
            println!(
                "C06 essai à zéro — {:<10} mesuré {:>14.6e}  {}",
                c.id,
                c.mesure,
                if c.passe() { "passe" } else { "ÉCHOUE" }
            );
        }
        for c in &cas {
            assert_eq!(
                c.mesure, 0.0,
                "{} : sans boost, les deux simulations sont le même calcul — l'écart doit être \
exactement nul, pas {:.3e}",
                c.id, c.mesure
            );
        }
    }

    /// **P2 — l'essai à zéro de C08** (**A167**, action S42-1).
    ///
    /// S42 laissait la question ouverte : *que veut dire « résultat attendu zéro » pour une mesure
    /// d'ordre de convergence ?* La réponse est que l'essai ne porte pas sur le solveur mais sur
    /// **l'estimateur**, et qu'un montage sans objet à mesurer est ici une suite d'erreurs qui **ne
    /// converge pas** — trois grilles, trois erreurs identiques. Il n'y a pas d'ordre.
    ///
    /// L'estimateur se teste alors sur des suites **synthétiques**, sans lancer une simulation :
    /// c'est le sens de l'extraction faite en S34.
    ///
    /// Ce test **constate**, avant toute correction.
    #[test]
    fn c08_essai_a_zero_ce_que_l_estimateur_rend() {
        use crate::physics::ordre_grossier_estime;
        let dire = |o: Option<f64>| match o {
            Some(p) => format!("{p:>8.4}"),
            None => "  REFUS ".to_string(),
        };
        // `e_k = C·dx_k^p` sur trois grilles en raffinement ×2 : l'estimateur doit rendre `p`.
        let suite = |p: f64| -> Vec<(usize, f64)> {
            (0..3).map(|k| (100usize << k, 1.0 / (2.0f64.powi(k)).powf(p))).collect()
        };
        println!("C08 — l'estimateur d'ordre, sur des suites dont la réponse est connue :");
        for p in [1.0f64, 2.0, 0.5] {
            let (brut, borne) = ordre_grossier_estime(&suite(p));
            println!("      e ∝ dx^{p:.1}  → brut {}, borné {borne:>8.4}   (attendu {p:.1})", dire(brut));
            assert!(
                brut.map(|b| (b - p).abs() < 1e-9).unwrap_or(false),
                "l'estimateur doit rendre {p} exactement sur une suite construite pour"
            );
        }

        // **L'essai à zéro**, sous ses trois formes. Aucune n'a d'ordre à mesurer, et **toutes
        // rendaient `1.0`** avant S43 — l'ordre nominal du schéma, dans les bornes de G10, donc
        // silencieux.
        let cas: [(&str, Vec<(usize, f64)>); 3] = [
            (
                "e constante (le solveur NE CONVERGE PAS)",
                (0..3).map(|k| (100usize << k, 1.0e-3)).collect(),
            ),
            ("deux grilles seulement (triplet incomplet)", vec![(100, 1e-3), (200, 5e-4)]),
            ("erreurs toutes nulles", (0..3).map(|k| (100usize << k, 0.0)).collect()),
        ];
        for (nom, e) in &cas {
            let (brut, borne) = ordre_grossier_estime(e);
            println!("      {nom:<44} → brut {}, borné {borne:>8.4}", dire(brut));
            assert!(
                brut.is_none(),
                "{nom} : il n'y a aucun ordre à mesurer, l'estimateur doit refuser"
            );
            assert_eq!(borne, 1.0, "{nom} : le borné reste conservateur pour que le filtre marche");
        }
    }

    /// **P2–P3 — les deux assertions en « déficit » avalent-elles un refus ?**
    ///
    /// `physics.rs` exprime les deux minorants de C03 sous forme de **déficit** — une formulation
    /// juste, et bien meilleure qu'une tolérance relative sur un minorant :
    ///
    /// ```text
    /// déficit de demi-vie = max(0, 15 − mesure) / 15      référence 0, tolérance 0
    /// déficit de R²       = max(0, 0,9 − mesure)          référence 0, tolérance 0
    /// ```
    ///
    /// Le `max(0, …)` est ce qui rend le déficit nul dès que le minorant est tenu. **Et c'est aussi
    /// ce qui avale les `NaN`** : `f64::max` propage le non-`NaN`, donc `(15 − NaN).max(0)` vaut
    /// `0` — le **déficit nul**, c'est-à-dire le meilleur score possible face à une tolérance de
    /// zéro.
    ///
    /// Ce test sépare les deux cas que le `max` confond :
    ///
    /// | entrée | ce que ça veut dire | attendu |
    /// |---|---|---|
    /// | `+∞` | le schéma **n'amortit pas** | déficit nul — **le cas passe**, et c'est juste |
    /// | `NaN` | il n'y avait **rien à mesurer** | le cas doit **échouer** |
    #[test]
    fn les_deux_deficits_de_c03_ne_doivent_pas_avaler_un_refus() {
        // Les deux expressions telles que `physics.rs` les calcule désormais — par `deficit()`,
        // qui sépare le minorant tenu du refus.
        use crate::physics::deficit;
        let deficit_demi_vie = |x: f64| deficit(15.0, x, 15.0);
        let deficit_r2 = |x: f64| deficit(0.9, x, 1.0);
        // `passe()` de `Cas` avec référence 0 et tolérance 0 : la mesure elle-même doit valoir 0.
        let passe = |m: f64| m.abs() <= 0.0;

        for (nom, f) in [
            ("déficit de demi-vie", &deficit_demi_vie as &dyn Fn(f64) -> f64),
            ("déficit de R²", &deficit_r2),
        ] {
            let sur_inf = f(f64::INFINITY);
            let sur_nan = f(f64::NAN);
            println!(
                "{nom:<22} — sur +∞ : {sur_inf:>6.3} ({}) | sur NaN : {sur_nan:>6.3} ({})",
                if passe(sur_inf) { "passe" } else { "échoue" },
                if passe(sur_nan) { "PASSE" } else { "échoue" }
            );
            assert!(
                passe(sur_inf),
                "{nom} : une valeur infinie satisfait le minorant, le cas doit passer"
            );
            assert!(
                !passe(sur_nan),
                "{nom} : un refus ne doit pas devenir un déficit nul — c'est le meilleur score \
possible, rendu à une mesure qui n'a rien mesuré"
            );
        }
    }

    /// **A157 — un seuil reproductible peut être dénué de sens, et deux seuils incomparables
    /// peuvent être mis côte à côte.**
    ///
    /// La fiche importée critique le seuil de mouillage de `10⁻⁶ m` — *un micron d'eau*, épaisseur
    /// à laquelle ni le modèle moyenné sur la hauteur, ni la rugosité d'un fond, ni le rendu n'ont
    /// de sens. Elle porte sur le **seuil de mesure du front**, et non sur le seuil du solveur que
    /// `ADR-047` a tranché : deux objets, deux noms (**L148**).
    ///
    /// # Ce que la relecture trouve ici, et qui n'était pas dans la fiche
    ///
    /// Les deux véhicules ne mesurent pas le front de la même façon — **ni au même seuil, ni contre
    /// la même référence** :
    ///
    /// | | seuil | référence |
    /// |---|---|---|
    /// | `physics.rs` *(accueil)* | `10⁻³ m` | front **ponctuel** de Ritter au même seuil |
    /// | `physics_shallow.rs` *(lignée B)* | `10⁻²·h₀` | front de Ritter **moyenné sur la maille** |
    ///
    /// Et `CAS-CANONIQUES` les met **côte à côte** depuis S36, dans le tableau « deux véhicules,
    /// deux colonnes », sans dire que ce ne sont pas les mêmes mesures.
    #[test]
    fn a157_les_deux_fronts_ne_sont_pas_comparables() {
        let (cd, cs) = champs_c04(2.0);
        // **Chaque véhicule a son barrage.** Les origines diffèrent de 20 m — `[−20, +20]` d'un
        // côté, `[0, 40]` de l'autre — et seuls les **indices** se correspondent. Prendre une seule
        // abscisse de barrage pour les deux décale l'un de 20 m ; l'en-tête de ce module le dit, et
        // ce test l'a fait quand même à sa première écriture.
        let front = |c: &Champs, seuil: f64| {
            let x_barrage = c.x[400] - 0.5 * 0.05;
            c.hauteur
                .iter()
                .enumerate()
                .filter(|(_, &h)| h > seuil)
                .map(|(i, _)| c.x[i] - x_barrage)
                .fold(f64::NEG_INFINITY, f64::max)
        };
        // Front mathématique de Ritter au même seuil : `h(x,t) = (2c₀ − x/t)²/(9g)`.
        let c0 = (G * 1.0f64).sqrt();
        let ritter = |seuil: f64| {
            let mut x = 2.0 * c0 * 2.0;
            let mut pas = 1e-4;
            while pas > 1e-9 {
                let h = (2.0 * c0 - x / 2.0).max(0.0).powi(2) / (9.0 * G);
                if h > seuil {
                    x += pas;
                } else {
                    x -= pas;
                    pas *= 0.5;
                }
            }
            x
        };

        println!("C04 à t = 2 s — position du front, par seuil et par véhicule :");
        for seuil in [1e-3f64, 1e-2] {
            let (fd, fs, fr) = (front(&cd, seuil), front(&cs, seuil), ritter(seuil));
            println!(
                "      seuil {seuil:>8.0e} m : delta.rs {fd:>7.4} m ({:>7.2} %) | shallow.rs {fs:>7.4} m ({:>7.2} %) | Ritter ponctuel {fr:>7.4} m",
                (fd - fr) / fr * 100.0,
                (fs - fr) / fr * 100.0
            );
        }

        // **Ce qui est vérifié** : à seuil égal et contre la même référence, les deux véhicules
        // s'accordent. Le désaccord de seize points qui traverse le corpus vient de la **méthode de
        // mesure**, pas des solveurs.
        for seuil in [1e-3f64, 1e-2] {
            let (fd, fs) = (front(&cd, seuil), front(&cs, seuil));
            let ecart = (fd - fs).abs() / fs * 100.0;
            assert!(
                ecart < 1.0,
                "à seuil {seuil:.0e} les deux fronts devraient s'accorder : {fd:.4} contre {fs:.4} ({ecart:.2} %)"
            );
        }

        // **Et ce qui ne l'est pas** : changer de seuil déplace le front bien plus que le désaccord
        // entre véhicules. C'est A157 — le seuil ne décrit pas le solveur, il décrit la mesure.
        let d3 = front(&cd, 1e-3);
        let d2 = front(&cd, 1e-2);
        println!(
            "      → changer de seuil déplace le front de {:.2} % ; changer de véhicule, de moins de 1 %",
            (d3 - d2).abs() / d2 * 100.0
        );
        assert!(
            (d3 - d2).abs() / d2 * 100.0 > 1.0,
            "le seuil devrait déplacer le front plus que le choix du véhicule"
        );
    }

    /// **Ce que S36 avait attribué à l'ordre deux, et qui vient surtout d'ailleurs.**
    ///
    /// `CAS-CANONIQUES` porte depuis S36 un tableau « deux véhicules, deux colonnes » où C04
    /// **échoue** d'un côté et vaut **0,74 %** de l'autre, avec cette explication : *la lignée B est
    /// passée à l'ordre deux, et C04 comme C08 sont passés au vert ensemble.*
    ///
    /// **Trois choses changent en même temps entre les deux colonnes**, et le tableau n'en nomme
    /// qu'une :
    ///
    /// | | `physics.rs` | `physics_shallow.rs` |
    /// |---|---|---|
    /// | schéma | ordre un | **ordre deux** ← la seule que S36 nomme |
    /// | seuil de mesure | `10⁻³ m` | `10⁻²·h₀` |
    /// | **référence** | front **ponctuel** de Ritter | front **moyenné sur la maille** |
    ///
    /// Ce test décompose. Il tourne **à ordre un des deux côtés** : tout écart qui subsiste vient
    /// donc de la mesure, pas du schéma.
    #[test]
    fn a157_ce_que_l_ordre_deux_explique_vraiment() {
        use crate::physics_shallow::front_exact;
        let (cd, _) = champs_c04(2.0);
        let x_barrage = cd.x[400] - 0.5 * 0.05;
        let dx = 0.05f64;
        let front = |seuil: f64| {
            cd.hauteur
                .iter()
                .enumerate()
                .filter(|(_, &h)| h > seuil)
                .map(|(i, _)| cd.x[i] - x_barrage)
                .fold(f64::NEG_INFINITY, f64::max)
        };
        let c0 = (G * 1.0f64).sqrt();
        let ritter_ponctuel = |seuil: f64| {
            let (mut x, mut pas) = (2.0 * c0 * 2.0, 1e-4);
            while pas > 1e-9 {
                let h = (2.0 * c0 - x / 2.0).max(0.0).powi(2) / (9.0 * G);
                if h > seuil { x += pas } else { x -= pas; pas *= 0.5 }
            }
            x
        };

        println!("C04, `delta.rs` à l'ORDRE UN — le même front, contre deux références :");
        for seuil in [1e-3f64, 1e-2] {
            let f = front(seuil);
            let (rp, rm) = (ritter_ponctuel(seuil), front_exact(dx, 2.0, 1.0, seuil));
            println!(
                "      seuil {seuil:>8.0e} : front {f:.4} m | ponctuel {rp:.4} ({:>7.2} %) | moyenné maille {rm:.4} ({:>6.2} %)",
                (f - rp) / rp * 100.0,
                (f - rm) / rm * 100.0
            );
        }

        // **Le résultat, et il dément la thèse écrite avant de mesurer.** Je m'attendais à ce que
        // la seule révision de la mesure fasse passer C04 sous les 3 %. Elle n'y suffit pas.
        //
        // | même solveur, ordre un | écart au front |
        // |---|---|
        // | mesure d'accueil — seuil `10⁻³`, référence ponctuelle | **20,4 %** |
        // | mesure de la lignée B — seuil `10⁻²·h₀`, référence moyennée | **10,2 %** |
        // | *(publié à l'ordre deux, mesure de la lignée B)* | *0,74 %* |
        //
        // **La révision de la mesure retire dix points sur vingt ; l'ordre deux retire les neuf et
        // demi qui restent.** Aucun des deux seul ne fait franchir la tolérance de 3 %.
        //
        // Ce que S36 écrivait — *l'ordre deux explique tout l'écart de verdicts* — était donc une
        // attribution **non démontrée**, et elle est fausse de moitié. Ce qui reste vrai : c'est
        // bien l'ordre deux qui fait franchir le seuil, parce qu'il agit en dernier.
        let f = front(1e-2);
        let rm = front_exact(dx, 2.0, 1.0, 1e-2);
        let ecart_mesure_b = (f - rm).abs() / rm * 100.0;
        let rp = ritter_ponctuel(1e-3);
        let ecart_mesure_accueil = (front(1e-3) - rp).abs() / rp * 100.0;
        println!(
            "      → même solveur, même pas de temps : {ecart_mesure_accueil:.2} % avec la mesure d'accueil, \
{ecart_mesure_b:.2} % avec celle de la lignée B — la mesure explique {:.0} % de l'écart, le schéma le reste",
            (ecart_mesure_accueil - ecart_mesure_b) / (ecart_mesure_accueil - 0.74) * 100.0
        );
        assert!(
            ecart_mesure_b < ecart_mesure_accueil / 1.5,
            "la mesure de la lignée B devrait retirer une part notable de l'écart : \
{ecart_mesure_accueil:.2} % → {ecart_mesure_b:.2} %"
        );
        assert!(
            ecart_mesure_b > 3.0,
            "mais elle ne devrait pas suffire à passer la tolérance de C04 : {ecart_mesure_b:.2} %"
        );
    }

    /// **A159 — refaire une formule publiée, avec ses constantes.**
    ///
    /// L'angle mort importé dit : *une formule énoncée avec ses constantes n'invite pas à être
    /// recalculée ; elle a l'apparence d'un résultat, et on ne vérifie pas un résultat, on le cite.*
    /// Son remède : **repérer les formules dont dépend une décision et les refaire, une fois.**
    /// Personne ne l'avait fait dans cette lignée — c'est Q4 de la relecture de S41.
    ///
    /// Ce test refait `ADR-037` §3, dont dépend le dimensionnement de `δ` pour les transitoires :
    ///
    /// ```text
    /// t_phys = √(2L/g)                     temps de chute gravitaire
    /// t_num  = K·L²/(dx·c),  K = ln2/(2π²(1−ν)),  c = √(gh)
    /// t_num ≥ t_phys   ⟺   dx ≤ K·L^1,5/√(2h)      — `g` disparaît
    /// ```
    ///
    /// **La dérivation est juste** — `g` disparaît bien — et les huit valeurs du tableau se
    /// recalculent, à `h = 2 m`.
    ///
    /// # Le critère, et pourquoi il n'a pas de tolérance
    ///
    /// Comparer à « 1 % près » ou « 0,05 cm près » demanderait de choisir un nombre, et le choisir
    /// trop large reviendrait à ajuster l'instrument sur ce qu'il mesure (**L152**). La question
    /// exacte est : **la valeur publiée est-elle l'arrondi correct de la valeur recalculée, à la
    /// précision où elle est écrite ?** Elle se pose sans tolérance, et elle se répond par oui ou
    /// par non.
    #[test]
    fn adr_037_le_critere_de_dimensionnement_se_recalcule() {
        let k = |nu: f64| 2.0f64.ln() / (2.0 * std::f64::consts::PI.powi(2) * (1.0 - nu));
        let dx_max = |nu: f64, l: f64, h: f64| k(nu) * l.powf(1.5) / (2.0 * h).sqrt();
        let h = 2.0f64;

        // `dx` en centimètres, tel que le tableau d'ADR-037 §3 les publie — une décimale.
        let publie = [
            (0.45f64, 0.5f64, 1.1f64),
            (0.45, 1.0, 3.2),
            (0.45, 2.0, 9.0),
            (0.45, 5.0, 35.7),
            (0.70, 0.5, 2.1),
            (0.70, 1.0, 5.9),
            (0.70, 2.0, 16.6),
            (0.70, 5.0, 65.5),
        ];
        let mut mal_arrondis = Vec::new();
        for (nu, l, cm) in publie {
            let calcule = dx_max(nu, l, h) * 100.0;
            let arrondi = (calcule * 10.0).round() / 10.0;
            let verdict = if (arrondi - cm).abs() < 1e-9 { "ok" } else { "MAL ARRONDI" };
            println!(
                "ADR-037 §3 — ν = {nu:.2}, L = {l:.1} m : publié {cm:.1} | recalculé {calcule:.3} \
→ arrondi {arrondi:.1}   {verdict}"
            );
            if verdict != "ok" {
                mal_arrondis.push((nu, l, cm, arrondi, calcule));
            }
            // Le garde-fou de fond : au-delà de 5 %, ce n'est plus un arrondi, c'est une erreur.
            assert!(
                (calcule - cm).abs() / cm < 0.05,
                "ADR-037 §3 : {cm} cm publié pour ν = {nu}, L = {l} m, recalculé {calcule:.3} — \
ce n'est plus un arrondi"
            );
        }

        // Le levier du nombre de Courant : ×1,83 sur `dx`, donc ÷6,1 sur les cellules 3D.
        let levier = k(0.70) / k(0.45);
        println!(
            "ADR-037 §3.1 — levier de ν : publié ×1,83 | recalculé ×{levier:.4} ; cellules 3D ÷{:.2} (publié 6,1)",
            levier.powi(3)
        );
        assert!((levier - 1.83).abs() < 0.005, "levier recalculé {levier:.4}");

        // **Et une seconde valeur mal arrondie**, trouvée par le même critère : `1,8333³ = 6,163`,
        // dont l'arrondi au dixième est **6,2** et non 6,1. Le §3.1 publie 6,1.
        let cellules = levier.powi(3);
        let arrondi_cellules = (cellules * 10.0).round() / 10.0;
        if (arrondi_cellules - 6.1).abs() > 1e-9 {
            mal_arrondis.push((0.70, 0.0, 6.1, arrondi_cellules, cellules));
        }
        assert!(
            (cellules - 6.1).abs() / 6.1 < 0.05,
            "cellules 3D ÷{cellules:.3} — ce n'est plus un arrondi"
        );

        // « À `dx = 0,25 m`, une éclaboussure d'un mètre s'éteint huit fois trop tôt. »
        let rapport = |l: f64, dx: f64| k(0.45) * l.powf(1.5) / (dx * (2.0 * h).sqrt());
        println!(
            "ADR-037 §3 — à dx = 0,25 m : L = 1 m s'éteint {:.1}× trop tôt (publié : huit), \
L = 0,5 m {:.1}× (publié : vingt)",
            1.0 / rapport(1.0, 0.25),
            1.0 / rapport(0.5, 0.25)
        );
        assert!((1.0 / rapport(1.0, 0.25) - 8.0).abs() < 0.5);

        // **Le résultat de cette vérification, et il n'est pas nul : deux chiffres sur neuf sont
        // mal arrondis** — `65,5` pour 65,4 et `÷6,1` pour 6,2. C'est minuscule, et c'est
        // exactement ce qu'A159 demande de trouver : *les formules dont dépend une décision se
        // refont une fois, avec leurs constantes.* La décision, ici, est le dimensionnement de δ
        // pour les transitoires.
        //
        // Ni l'une ni l'autre ne change quoi que ce soit à la conclusion d'ADR-037. C'est bien le
        // sujet : **une vérification qui ne trouve que des broutilles est une vérification qui a
        // réussi**, et elle ne pouvait pas le dire avant d'avoir été faite.
        for (nu, l, cm, arrondi, calcule) in &mal_arrondis {
            if *l > 0.0 {
                println!(
                    "  → §3 publie {cm:.1} cm pour ν = {nu:.2}, L = {l:.1} m ; \
l'arrondi correct de {calcule:.3} est {arrondi:.1}"
                );
            } else {
                println!(
                    "  → §3.1 publie ÷{cm:.1} ; l'arrondi correct de {calcule:.3} est {arrondi:.1}"
                );
            }
        }
        assert_eq!(
            mal_arrondis.len(),
            2,
            "**deux** chiffres mal arrondis sont connus et documentés par la note S41 d'ADR-037 : \
`65,5` pour 65,4 et `÷6,1` pour 6,2. Un autre compte veut dire que le §3 a changé."
        );
    }

    /// **P3–P5 — de quoi ce seuil décide-t-il ?**
    ///
    /// Sept décades, de `10⁻³` à `10⁻¹⁰`, sur les **deux** véhicules. `ADR-031` §4 s'arrêtait à
    /// `10⁻⁹` et ne regardait que le front ; ici, chaque grandeur publiée de C04 est relevée à côté
    /// de `max|u|`, que **rien ne publie**.
    #[test]
    fn ce_que_le_seuil_de_sec_deplace() {
        let seuils = [1e-3f64, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10];
        let d = sensibilite_delta(&seuils, 2.0);
        rapporter_sensibilite("delta.rs  (f32)", &d);
        let s = sensibilite_shallow(&seuils, 2.0);
        rapporter_sensibilite("shallow.rs (f64)", &s);

        // **Volet 1 — les grandeurs publiées sont insensibles.**
        //
        // La tolérance sur le volume est à `10⁻⁶` relatif et non à l'arrondi : `delta.rs` calcule en
        // `f32`, où la somme de 800 cellules porte déjà quelques `10⁻⁸` de bruit (**A164**). Une
        // borne à `10⁻⁹` mesurerait la précision, pas la conservation — c'est le plancher de S37,
        // sur un autre objet.
        for (nom, v) in [("delta.rs", &d), ("shallow.rs", &s)] {
            let r = &v[0];
            for x in v.iter() {
                let e = (x.front - r.front).abs() / r.front.abs() * 100.0;
                assert!(
                    e < 1.0,
                    "{nom} : le front bouge de {e:.3} % entre 10⁻³ et {:.0e} — la thèse tombe",
                    x.seuil
                );
                assert!(
                    (x.volume - r.volume).abs() / r.volume < 1e-6,
                    "{nom} : le volume n'est pas conservé à travers le balayage"
                );
                assert!(
                    (x.h0 - r.h0).abs() / r.h0 < 1e-3,
                    "{nom} : h(0) bouge avec le seuil"
                );
            }
        }

        // **Volet 2 — et `max|u|` n'est pas une grandeur, c'est du bruit.**
        //
        // Le critère n'est pas une tolérance : `2c₀ = 2√(g·h₀) = 6,264 m/s` est **la vitesse
        // maximale que la solution de Ritter contient**. Une valeur au-dessus ne décrit aucun
        // écoulement — elle décrit `hu/h` sur un film dont l'épaisseur est un réglage.
        let c2 = 2.0 * (G * 1.0f64).sqrt();
        for (nom, v) in [("delta.rs", &d), ("shallow.rs", &s)] {
            let umax = v.iter().fold(0.0f64, |a, x| a.max(x.u_max));
            let umin = v.iter().fold(f64::INFINITY, |a, x| a.min(x.u_max));
            println!(
                "{nom} — max|u| sur le balayage : de {umin:.4} à {umax:.4} m/s, pour une borne physique de {c2:.3}"
            );
            assert!(
                umax > c2,
                "{nom} : max|u| devrait dépasser la vitesse du front de Ritter — c'est ce qui montre que la grandeur n'est pas physique"
            );
            assert!(
                umax / umin > 1.5,
                "{nom} : max|u| devrait varier d'un facteur notable avec le seuil — mesuré {:.2}",
                umax / umin
            );
        }

        // **Volet 3 — le film s'allonge quand le seuil baisse**, régulièrement, des deux côtés.
        // C'est **A165** : le seuil coupe la vitesse, pas le flux de masse.
        for (nom, v) in [("delta.rs", &d), ("shallow.rs", &s)] {
            assert!(
                v.last().unwrap().film > v[0].film,
                "{nom} : le film devrait s'allonger quand le seuil baisse"
            );
        }
    }

    /// **P5 — C01 confronté : les deux véhicules disent-ils la même chose ?**
    ///
    /// C'est le premier usage réel de l'oracle croisé promis par `ADR-043` §3. Le cas est choisi
    /// pour son absence d'ambiguïté : la solution exacte est `u ≡ 0`, `η ≡ η₀`, et les deux
    /// schémas sont censés la reproduire **exactement** — à leur précision près.
    ///
    /// Le critère n'est donc pas une tolérance choisie : c'est le **plancher mesuré** par
    /// [`plancher_c01`], majoré d'un facteur 2 pour ne pas transformer un test de concordance en
    /// test de reproductibilité bit à bit. Au-dessus, un désaccord désignerait une faute
    /// d'implémentation ; au-dessous, il ne dirait rien.
    #[test]
    fn c01_les_deux_vehicules_concordent_au_plancher() {
        for duree in [1.0f64, 10.0, 60.0] {
            let p = plancher_c01(duree);
            let c = confronter_c01(duree);
            c.rapporter();
            println!(
                "      plancher — borne_u = {:>11.4e}   borne_η = {:>11.4e}",
                p.borne_u(),
                p.borne_eta()
            );
            assert!(
                c.vitesse.linf <= 2.0 * p.borne_u().max(1e-12),
                "C01 à t={duree} s : les deux véhicules diffèrent de {:.4e} m/s en vitesse, au-dessus du plancher {:.4e}",
                c.vitesse.linf,
                p.borne_u()
            );
            assert!(
                c.surface.linf <= 2.0 * p.borne_eta().max(1e-12),
                "C01 à t={duree} s : écart de surface {:.4e} m, au-dessus du plancher {:.4e}",
                c.surface.linf,
                p.borne_eta()
            );
        }
    }

    /// **P3 — quel est le plancher de cet oracle ?**
    ///
    /// Sur C01 au repos, chaque solveur est confronté à la **solution exacte** — `u ≡ 0`,
    /// `η ≡ η₀` — et non l'un à l'autre. La somme des deux erreurs borne ce que leur comparaison
    /// croisée peut distinguer.
    ///
    /// Ce test n'affirme pas une valeur : il **la rapporte**, et vérifie seulement que le plancher
    /// est dominé par le côté `f32` — sans quoi la lecture « l'écart vient de la précision » serait
    /// fausse, et il faudrait chercher ailleurs.
    #[test]
    fn le_plancher_de_l_oracle_est_domine_par_le_f32() {
        for duree in [1.0f64, 10.0, 60.0] {
            let p = plancher_c01(duree);
            println!(
                "plancher C01 à t={:>5.1} s — u : f32 {:>11.4e} | f64 {:>11.4e}   η : f32 {:>11.4e} | f64 {:>11.4e}   borne_u {:>11.4e}",
                p.duree_s, p.delta_u, p.shallow_u, p.delta_eta, p.shallow_eta, p.borne_u()
            );
            assert!(
                p.shallow_u <= p.delta_u,
                "le côté f64 devrait être le plus exact : f32 {:.4e}, f64 {:.4e}",
                p.delta_u, p.shallow_u
            );
            assert!(
                p.delta_u < 1e-3,
                "C01 exige max|u| < 1 mm/s ; delta.rs rend {:.4e} à t={duree} s",
                p.delta_u
            );
        }
    }

    /// **P2 — les deux montages de C01 sont-ils le même ?**
    ///
    /// C'est la question à régler avant toute comparaison : deux solveurs qui ne partent pas du
    /// même état ne se comparent pas, et leurs écarts mesureraient le montage au lieu du schéma.
    ///
    /// Le fond et la grille sont des données **exactes** — `delta.rs` les calcule en `f32`, donc
    /// l'accord attendu est au plancher `f32` et non au bit près. La hauteur initiale en découle.
    #[test]
    fn les_deux_montages_de_c01_sont_le_meme() {
        let (d, s) = avec_hote(1 << 20, |h| montages_c01(h));
        let (cd, cs) = (champs_delta(&d), champs_shallow(&s));

        assert_eq!(cd.x.len(), 160, "delta.rs : 160 cellules internes attendues");
        assert_eq!(cs.x.len(), 160, "shallow.rs : 160 cellules attendues");

        // Le plancher : `ε_f32` rapporté à la plus grande grandeur en jeu — 40 m d'abscisse.
        let plancher = 40.0 * EPS_F32;
        for (nom, a, b) in [
            ("x", &cd.x, &cs.x),
            ("fond", &cd.fond, &cs.fond),
            ("hauteur", &cd.hauteur, &cs.hauteur),
            ("surface", &cd.surface, &cs.surface),
            ("vitesse", &cd.vitesse, &cs.vitesse),
        ] {
            let e = Ecart::entre(a, b);
            println!(
                "C01 à t=0 — {nom:<8} L∞ = {:>12.4e}  L¹ = {:>12.4e}  (cellule {})",
                e.linf, e.l1, e.i_max
            );
            assert!(
                e.linf <= plancher,
                "{nom} : les deux montages diffèrent de {:.4e}, au-dessus du plancher f32 ({plancher:.4e})",
                e.linf
            );
        }
    }
}
