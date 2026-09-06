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
/// `delta.rs` déclare une cellule sèche sous `H_SEC = 10⁻⁶ m` et rend alors `u = 0` ; `shallow.rs`
/// fait de même sous `10⁻¹⁰ m`. **Quatre ordres de grandeur les séparent**, et une cellule dont la
/// hauteur tombe entre les deux est sèche pour l'un et mouillée pour l'autre — avec une vitesse
/// `hu/h` qui, sur un film de cette épaisseur, peut valoir plusieurs mètres par seconde.
///
/// Aucun des deux seuils n'est faux. `ADR-031` §5 donne la provenance de `H_SEC` ; le `10⁻¹⁰` de
/// `shallow.rs` est écrit en dur, sans justification. **Ce qui est faux, c'est de comparer les deux
/// vitesses sans le savoir.**
pub const SEC_DELTA: f64 = 1.0e-6;

/// Le seuil de `shallow.rs`, écrit en dur dans `vitesse()` et dans le flux HLL.
pub const SEC_SHALLOW: f64 = 1.0e-10;

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
    let (mut d, mut s) = avec_hote(1 << 22, |h| {
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
