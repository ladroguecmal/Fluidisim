//! Mode `physics`, second véhicule — les montages écrits contre `shallow.rs`.
//!
//! # Pourquoi ce module existe à côté de `physics.rs`
//!
//! Le dépôt a forké en S21, et les deux lignées ont écrit **le même solveur** sans le savoir :
//! `delta.rs` ici, `shallow.rs` là-bas — Saint-Venant 1D, volumes finis, flux de Rusanov (ADR-043).
//! S35 a importé le second solveur ; **ce module importe ses montages**.
//!
//! Ils ne sont **pas fusionnés** avec ceux de `physics.rs`, et c'est délibéré. Deux raisons, dont
//! la seconde est la vraie :
//!
//! 1. Les deux jeux portent des fonctions de **même nom** — `c03_seiche`, `ritter`,
//!    `c08_convergence` — avec des signatures différentes, chacune écrite contre son solveur.
//! 2. **Deux implémentations indépendantes du même modèle forment un oracle** que le projet n'a
//!    nulle part ailleurs (ADR-043 §3). Sur un cas sans solution analytique, leur désaccord désigne
//!    une faute d'implémentation dans l'une des deux. **Fusionner les montages détruirait
//!    exactement ce qu'on cherche à garder.**
//!
//! # Ce que cet oracle ne dit pas
//!
//! Les deux solveurs partagent le modèle, donc **tous ses angles morts** : une dimension,
//! `c = √(g·h)`, non dispersif. Une concordance ne valide pas la physique — elle élimine la faute
//! d'implémentation, et rien de plus (L138).
//!
//! # Provenance
//!
//! Le contenu de ce module vient de la lignée B, sessions **B-B-B-S22** à **B-B-B-S26**, et n'est pas de
//! l'écriture neuve. Les décisions qui le motivent sont `ADR-038` à `ADR-042` ; la carte de
//! renumérotation est dans `docs/registres/FORK-B-B-S22-B-B-S26.md`. **Les références à des sessions dans
//! les commentaires ci-dessous désignent celles de la lignée B**, et sont préfixées `B-`.

use water_core::{Flux, HostServices, Shallow1D};

use crate::host_impl::{ArenaAllocator, SequentialJobs, StderrSink};
use crate::physics::Cas;

/// Accélération de la pesanteur. Identique à `physics::G` — répétée ici pour que ce module se lise
/// seul, comme le fait `shallow.rs` vis-à-vis de `delta.rs`.
const G: f64 = 9.81;

/// Un jeu de reglages du solveur, avec son nom pour le rapport.
///
/// Les trois sont conserves et mesures ensemble. Ce n'est pas de la nostalgie : sans le schema
/// d'ordre un, C04 et C08 ne demontrent plus qu'ils eliminent (L123), et sans l'etage intermediaire
/// — MUSCL avec Euler — rien ne dirait que l'ordre en temps compte autant que l'ordre en espace.
#[derive(Clone, Copy)]
pub struct Schema {
    pub ordre2: bool,
    pub rk2: bool,
    pub nom: &'static str,
}

pub const SCHEMAS: [Schema; 3] = [
    Schema { ordre2: false, rk2: false, nom: "ordre 1" },
    Schema { ordre2: true, rk2: false, nom: "MUSCL + Euler" },
    Schema { ordre2: true, rk2: true, nom: "MUSCL + RK2" },
];

/// Le schema retenu pour les assertions : le meilleur disponible.
pub const SCHEMA_RETENU: Schema = SCHEMAS[2];

/// Monte le barrage de C04 avec un schema donne.
fn barrage(n: usize, dx: f64, h0: f64, sc: Schema) -> Shallow1D {
    let mut alloc = ArenaAllocator::with_capacity(1 << 23);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };
    let mut d = Shallow1D::configure_barrage(&mut host, n, dx, h0).expect("configuration");
    d.regler_flux(Flux::Hll);
    d.regler_ordre2(sc.ordre2);
    d.regler_rk2(sc.rk2);
    d
}

/// Montage de C01 : bassin de 40 m, fond en pente 1:20, eau au repos.
///
/// **Le bassin est entièrement mouillé** — le fond va de −3 m à −1 m sous une surface libre à 0.
/// `CAS-CANONIQUES` ne fixe pas la profondeur ; la choisir sans front sec est délibéré. Un front
/// sec est une difficulté **distincte** — celle d'une plage — et la mêler à C01 rendrait un échec
/// inintérprétable : on ne saurait pas si le schéma déséquilibre l'hydrostatique ou s'il trébuche
/// sur la maille sèche.
pub fn montage_c01() -> Shallow1D {
    montage_c01_maille(160, 0.25)
}

/// Le même montage à une autre finesse de maille — 40 m de bassin dans tous les cas.
pub fn montage_c01_maille(n: usize, dx: f64) -> Shallow1D {
    let mut alloc = ArenaAllocator::with_capacity(1 << 20);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };
    Shallow1D::configure(&mut host, n, dx, -3.0, 0.05, 0.0).expect("configuration")
}

/// **C01 — repos hydrostatique sur fond en pente.**
///
/// Référence : `u ≡ 0` et `η ≡ η₀`, exactement. Assertions de `CAS-CANONIQUES` : `max|u| < 1 mm/s`
/// et `max|η − η₀| < 1 mm` après 60 s.
///
/// **La référence est zéro**, donc la tolérance relative de `Cas` se lit ici comme un seuil
/// **absolu** — c'est le comportement de `ecart_rel` quand la référence est nulle, et c'est
/// exactement ce que C01 demande.
///
/// Un troisième cas accompagne les deux : **le volume**. Le schéma est conservatif par
/// construction, donc il conservera le volume **même en étant faux** — la maille perd ce que sa
/// voisine gagne. Ce cas ne prouve donc rien sur la physique ; il est là pour qu'un échec de C01
/// puisse être attribué : si le volume dérive **aussi**, le défaut n'est pas l'équilibre
/// hydrostatique mais la comptabilité du schéma. Il est classé comme tel, pas compté comme une
/// vérification de plus (A104).
pub fn c01_repos_hydrostatique(t_fin: f64) -> (Vec<Cas>, u64) {
    let mut d = montage_c01();
    d.regler_equilibrage(true);
    d.regler_ordre2(SCHEMA_RETENU.ordre2);
    d.regler_rk2(SCHEMA_RETENU.rk2);
    let v0 = d.volume();

    // Relevé intermédiaire : un courant parasite qui **sature** et un courant parasite qui
    // **croît** ne décrivent pas le même défaut. Le premier est un biais borné, le second est
    // l'eau qui s'écoule indéfiniment vers le bas de la plage — le symptôme que C01 nomme.
    println!("  C01 — croissance du courant parasite, schéma **naïf** :");
    let mut naif = montage_c01();
    for jalon in [1.0, 5.0, 15.0, 30.0, t_fin] {
        naif.avancer_jusqu_a(jalon, 0.45);
        println!(
            "      t = {:>5.1} s   max|u| = {:>10.6} m/s   max|η−η₀| = {:>10.6} m",
            naif.temps(),
            naif.vitesse_max(),
            naif.ecart_surface_max(0.0)
        );
    }
    let pas = d.avancer_jusqu_a(t_fin, 0.45);

    // Le défaut dépend-il de la finesse de maille ? Un schéma **consistant mais non équilibré**
    // voit son courant parasite décroître avec `dx` sans jamais s'annuler ; un schéma **bien
    // équilibré** le tient à l'arrondi machine à toute finesse. Les deux se ressemblent sur une
    // seule maille et se distinguent sur trois.
    // Le cout de l'ordre deux, mesure plutot que suppose : RK2 double les evaluations du residu,
    // MUSCL ajoute le calcul des pentes. Sur le meme nombre de pas et la meme maille.
    println!("  C01 — cout de l'ordre deux, 60 s simulees sur 160 mailles :");
    for sc in [SCHEMAS[0], SCHEMA_RETENU] {
        let t0 = std::time::Instant::now();
        let mut e = montage_c01_maille(160, 0.25);
        e.regler_equilibrage(true);
        e.regler_ordre2(sc.ordre2);
        e.regler_rk2(sc.rk2);
        let pas = e.avancer_jusqu_a(60.0, 0.45);
        println!(
            "      {:<14} {pas} pas   {:>8.1} ms   {:>7.1} µs/pas",
            sc.nom,
            t0.elapsed().as_secs_f64() * 1e3,
            t0.elapsed().as_secs_f64() * 1e6 / pas as f64
        );
    }

    println!("  C01 — dépendance à la maille, à t = 5 s :");
    for (n, dx) in [(80usize, 0.5f64), (160, 0.25), (320, 0.125), (640, 0.0625)] {
        let mut a = montage_c01_maille(n, dx);
        a.avancer_jusqu_a(5.0, 0.45);
        let mut b = montage_c01_maille(n, dx);
        b.regler_equilibrage(true);
        b.avancer_jusqu_a(5.0, 0.45);
        let mut c = montage_c01_maille(n, dx);
        c.regler_equilibrage(true);
        c.regler_ordre2(true);
        c.regler_rk2(true);
        c.avancer_jusqu_a(5.0, 0.45);
        println!(
            "      dx = {dx:>7.4} m   naïf {:>10.6}   équilibré {:>10.3e}   ordre 2 {:>10.3e}",
            a.vitesse_max(),
            b.vitesse_max(),
            c.vitesse_max()
        );
    }

    let cas = vec![
        Cas {
            id: "C01-u",
            grandeur: format!("max|u| après {t_fin:.0} s, pente 1:20, équilibré"),
            mesure: d.vitesse_max(),
            reference: 0.0,
            tolerance_rel: 1e-3,
            source: "CAS-CANONIQUES C01 — u ≡ 0, seuil 1 mm/s",
        },
        Cas {
            id: "C01-η",
            grandeur: format!("max|η − η₀| après {t_fin:.0} s"),
            mesure: d.ecart_surface_max(0.0),
            reference: 0.0,
            tolerance_rel: 1e-3,
            source: "CAS-CANONIQUES C01 — η ≡ η₀, seuil 1 mm",
        },
        Cas {
            id: "C01-volume",
            grandeur: "dérive relative du volume (diagnostic, non probant)".into(),
            mesure: (d.volume() - v0).abs() / v0,
            reference: 0.0,
            tolerance_rel: 1e-12,
            source: "conservativité du schéma — vraie même si la physique est fausse",
        },
    ];
    (cas, pas)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **C01 — les trois assertions passent sur le schéma équilibré.**
    ///
    /// Reprise directe de la lignée B. Ce test ne dit rien de neuf sur la physique ; il dit que le
    /// montage a survécu au transport d'un arbre à l'autre.
    #[test]
    fn c01_les_trois_assertions_passent() {
        let (cas, pas) = c01_repos_hydrostatique(60.0);
        assert_eq!(cas.len(), 3);
        for c in &cas {
            assert!(c.passe(), "{} : {} = {:e}", c.id, c.grandeur, c.mesure);
        }
        assert!(pas > 0);
    }

    /// **Le chiffre publié se reproduit-il ?** `ADR-038` §2 annonce **19,5 mm/s** de courant
    /// parasite pour le schéma naïf, contre un seuil de 1 mm/s.
    ///
    /// Ce test est le premier à confronter une mesure de la lignée B **exécutée dans cet arbre** à
    /// ce que le corpus en dit. Jusqu'ici cette colonne de verdicts était, selon les termes de
    /// `CAS-CANONIQUES`, « un témoignage, pas une mesure ». La borne est large — ±5 % — parce que
    /// l'enjeu n'est pas la troisième décimale mais l'ordre de grandeur : **le schéma naïf rate son
    /// seuil d'un facteur vingt**, et c'est cela qui doit se reproduire.
    #[test]
    fn c01_le_schema_naif_reproduit_19_5_mm_par_seconde() {
        let mut naif = montage_c01();
        naif.avancer_jusqu_a(60.0, 0.45);
        let u = naif.vitesse_max();
        assert!(
            (u - 19.5e-3).abs() / 19.5e-3 < 0.05,
            "ADR-038 §2 annonce 19,5 mm/s ; mesuré ici {:.4} mm/s",
            u * 1e3
        );
        assert!(u > 1e-3, "le schéma naïf doit rater le seuil de 1 mm/s");
    }

    /// **L'exactitude du schéma équilibré ne dépend pas de la maille.** C'est la propriété que
    /// `ADR-038` oppose au raffinement : ce n'est pas une petite erreur, c'est l'absence d'erreur
    /// (L122). Quatre finesses, un facteur huit entre les extrêmes.
    #[test]
    fn c01_equilibre_exact_a_toute_finesse() {
        for (n, dx) in [(80usize, 0.5f64), (160, 0.25), (320, 0.125), (640, 0.0625)] {
            let mut d = montage_c01_maille(n, dx);
            d.regler_equilibrage(true);
            d.avancer_jusqu_a(5.0, 0.45);
            let u = d.vitesse_max();
            assert!(u < 1e-12, "dx = {dx} m : max|u| = {u:e}, attendu à l'arrondi machine");
        }
    }
}
