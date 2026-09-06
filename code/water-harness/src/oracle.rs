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

#[cfg(test)]
mod tests {
    use super::*;

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
