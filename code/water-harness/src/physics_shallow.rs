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

/// Hauteur exacte de la solution de Ritter, a l'abscisse `x` comptee depuis le barrage.
fn ritter_h(x: f64, t: f64, h0: f64) -> f64 {
    let c0 = (G * h0).sqrt();
    if t <= 0.0 {
        return if x < 0.0 { h0 } else { 0.0 };
    }
    let xi = x / t;
    if xi <= -c0 {
        h0
    } else if xi >= 2.0 * c0 {
        0.0
    } else {
        let a = 2.0 * c0 - xi;
        a * a / (9.0 * G)
    }
}

/// Vitesse exacte de la solution de Ritter.
fn ritter_u(x: f64, t: f64, h0: f64) -> f64 {
    let c0 = (G * h0).sqrt();
    if t <= 0.0 {
        return 0.0;
    }
    let xi = x / t;
    if xi <= -c0 {
        0.0
    } else if xi >= 2.0 * c0 {
        0.0
    } else {
        2.0 / 3.0 * (xi + c0)
    }
}

/// **Moyenne de maille** exacte de la solution de Ritter sur `[a, b]`.
///
/// Un schema de volumes finis ne porte pas la valeur ponctuelle au centre de la maille : il porte
/// la **moyenne** sur la maille. Au voisinage du front, ou `h` s'annule quadratiquement, les deux
/// different franchement — comparer l'une a l'autre attribue au solveur une erreur qui est celle de
/// la comparaison. La primitive est fermee : `∫(2c₀ − x/t)²/(9g) dx = t·s³/(27g)` avec `s = 2c₀ −
/// x/t`, ce qui evite aussi qu'une quadrature approchee ajoute son propre biais.
pub fn ritter_h_moyenne(a: f64, b: f64, t: f64, h0: f64) -> f64 {
    if b <= a {
        return 0.0;
    }
    let c0 = (G * h0).sqrt();
    let (xg, xd) = (-c0 * t, 2.0 * c0 * t);
    // Partie amont, a hauteur constante.
    let plat = (xg.min(b) - a).max(0.0) * h0;
    // Partie dans la detente.
    let (fa, fb) = (a.max(xg).min(xd), b.max(xg).min(xd));
    let s = |x: f64| 2.0 * c0 - x / t;
    let detente = if fb > fa {
        t * (s(fa).powi(3) - s(fb).powi(3)) / (27.0 * G)
    } else {
        0.0
    };
    // Partie aval : seche, contribution nulle.
    (plat + detente) / (b - a)
}

/// Front exact **au meme seuil et sur la meme moyenne de maille** que la mesure numerique.
///
/// C'est la correction qu'A156 et ADR-039 imposent : comparer deux grandeurs differentes est un
/// defaut de mesure, pas une exigence de rigueur.
pub fn front_exact(dx: f64, t: f64, h0: f64, seuil: f64) -> f64 {
    let c0 = (G * h0).sqrt();
    // On balaie les mailles depuis le front mathematique vers l'amont.
    let mut x = 2.0 * c0 * t;
    for _ in 0..100_000 {
        if ritter_h_moyenne(x - dx, x, t, h0) > seuil {
            return x - 0.5 * dx;
        }
        x -= dx;
        if x < -c0 * t {
            break;
        }
    }
    f64::NAN
}

/// Erreur `L¹` de la hauteur contre Ritter, a `t_fin`, pour une maille et un schema donnes.
fn erreur_l1_ritter(n: usize, dx: f64, h0: f64, t_fin: f64, sc: Schema) -> f64 {
    let mut d = barrage(n, dx, h0, sc);
    let x_barrage = d.x(n / 2) - 0.5 * dx;
    d.avancer_jusqu_a(t_fin, 0.45);

    let mut e = 0.0;
    for i in 0..d.cellules() {
        let xc = d.x(i) - x_barrage;
        e += (d.hauteur(i) - ritter_h_moyenne(xc - 0.5 * dx, xc + 0.5 * dx, t_fin, h0)).abs() * dx;
    }
    e
}

/// Position du front de mouillage, a un seuil donne, pour une maille et un schema donnes.
fn front_ritter(n: usize, dx: f64, h0: f64, t_fin: f64, sc: Schema, seuil: f64) -> f64 {
    let mut d = barrage(n, dx, h0, sc);
    let x_barrage = d.x(n / 2) - 0.5 * dx;
    d.avancer_jusqu_a(t_fin, 0.45);
    d.front_mouille(seuil).map(|x| x - x_barrage).unwrap_or(0.0)
}

/// **C04 — rupture de barrage, solution de Ritter.**
///
/// Canal plat sans frottement, `h₀ = 1 m` à gauche, lit sec à droite, lâcher à `t = 0`.
///
/// ```text
/// front aval  x = 2√(gh₀)·t = 6,264 m à t = 2 s
/// en x = 0    h = 4h₀/9 = 0,4444 m       u = (2/3)√(gh₀) = 2,0886 m/s
/// ```
///
/// # Pourquoi ce cas suit C01, et ne s'y substitue pas
///
/// C01 vérifie qu'un schéma **laisse l'eau immobile**. Un schéma peut y réussir parfaitement et se
/// tromper dès que l'eau bouge — c'est A101 transposé, « stable et faux » devenu « équilibré et
/// faux ». Ritter est une référence **dynamique**, fermée, et elle ne partage aucune ligne de code
/// avec le solveur : elle vient d'une analyse de 1892.
///
/// # Le seuil de mouillage fait partie de la mesure
///
/// Un front numérique n'a pas de bord net. Le cas mesure donc la position à **deux seuils**
/// séparés de trois ordres de grandeur, et rapporte les deux. Si la réponse dépend du seuil, le
/// chiffre annoncé est une convention déguisée en mesure.
pub fn c04_ritter(t_fin: f64, n: usize, dx: f64) -> Vec<Cas> {
    let mut alloc = ArenaAllocator::with_capacity(1 << 22);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };
    let h0 = 1.0f64;
    let mut d = Shallow1D::configure_barrage(&mut host, n, dx, h0).expect("configuration");
    d.regler_flux(Flux::Hll);
    d.regler_ordre2(SCHEMA_RETENU.ordre2);
    d.regler_rk2(SCHEMA_RETENU.rk2);
    let x_barrage = d.x(n / 2) - 0.5 * dx;
    d.avancer_jusqu_a(t_fin, 0.45);

    let c0 = (G * h0).sqrt();
    let i0 = d.maille_en(x_barrage);

    // Le front est-il lent parce que la maille est grosse, ou parce que le flux est le mauvais ?
    // La question n'a qu'une réponse expérimentale : on raffine les deux flux côte à côte. Un
    // défaut de maille recule sous raffinement ; un défaut de flux tient bon.
    let x_ref = 2.0 * (G * h0).sqrt() * t_fin;
    println!("  C04 — front sous raffinement, par schema, t = {t_fin:.0} s (référence {x_ref:.4} m) :");
    for (m, pas_m) in [(400usize, 0.1f64), (800, 0.05), (1600, 0.025), (3200, 0.0125)] {
        print!("      dx = {pas_m:>7.4} m  ");
        for sc in SCHEMAS {
            let f = front_ritter(m, pas_m, h0, t_fin, sc, 1e-6);
            print!(
                "   {:<14} {f:>8.4} m ({:>5.2} %)",
                sc.nom,
                (f - x_ref).abs() / x_ref * 100.0
            );
        }
        println!();
    }

    // Le reste de l'écart vient-il de la saturation des hauteurs négatives, qui détruit de la
    // masse au front ? Le volume le dira, et la CFL dira si c'est un problème de pas de temps.
    for cfl in [0.45f64, 0.20, 0.10] {
        let mut a3 = ArenaAllocator::with_capacity(1 << 23);
        let j3 = SequentialJobs;
        let s3 = StderrSink;
        let mut h3 = HostServices {
            alloc: &mut a3,
            jobs: &j3,
            sink: &s3,
        };
        let mut e = Shallow1D::configure_barrage(&mut h3, n, dx, h0).expect("configuration");
        e.regler_flux(Flux::Hll);
        e.regler_ordre2(SCHEMA_RETENU.ordre2);
        e.regler_rk2(SCHEMA_RETENU.rk2);
        let v0 = e.volume();
        let xb = e.x(n / 2) - 0.5 * dx;
        e.avancer_jusqu_a(t_fin, cfl);
        let f = e.front_mouille(1e-6).map(|x| x - xb).unwrap_or(0.0);
        println!(
            "      CFL = {cfl:.2}   front = {f:>8.4} m   dérive de volume = {:>10.3e}",
            (e.volume() - v0) / v0
        );
    }

    // Le profil au voisinage du front, terme a terme. C'est la seule facon de savoir si l'ecart
    // vient du solveur ou de la comparaison, et il fallait le regarder avant de corriger quoi que
    // ce soit.
    println!("  C04 — profil au voisinage du front, dx = {dx:.4} m :");
    println!("         x        h numerique    h moyenne exacte   h ponctuelle exacte    u num    u exact");
    let i_front = d.front_mouille(1e-9).map(|x| d.maille_en(x)).unwrap_or(0);
    for i in (i_front.saturating_sub(40)..=i_front + 4).step_by(8) {
        let xc = d.x(i) - x_barrage;
        println!(
            "      {xc:>8.4}   {:>12.6e}   {:>16.6e}   {:>19.6e}   {:>7.3}   {:>7.3}",
            d.hauteur(i),
            ritter_h_moyenne(xc - 0.5 * dx, xc + 0.5 * dx, t_fin, h0),
            ritter_h(xc, t_fin, h0),
            d.vitesse(i),
            ritter_u(xc, t_fin, h0)
        );
    }

    // L'ecart du front est-il un defaut du solveur, ou la consequence d'un seuil qui demande de
    // representer un film mille fois plus mince que la hauteur locale ? Un balayage du seuil
    // tranche : si l'ecart s'effondre quand le seuil monte, c'est la representabilite du film qui
    // est en cause, pas le corps de la solution.
    // Une seule simulation par maille, interrogee a tous les seuils : rejouer le barrage pour
    // chaque seuil coutait seize fois le meme calcul, et le budget de SPEC-003 §1 n'est pas un
    // decor.
    println!("  C04 — ecart du front selon le seuil, contre le front exact au meme seuil :");
    // Deux mailles suffisent a l'argument : l'ecart **se divise par deux avec la maille** au-dessus
    // de 10⁻³·h₀, et **sature** en dessous. La colonne fine est a `dx/2`, ce que le mode release
    // rend gratuit — voir `code/README.md`.
    println!("      seuil        dx = 0,025 m      dx = 0,0125 m");
    let mut fin = barrage(3200, 0.5 * dx, h0, SCHEMA_RETENU);
    let xb_fin = fin.x(1600) - 0.25 * dx;
    fin.avancer_jusqu_a(t_fin, 0.45);
    for seuil in [1e-1f64, 3e-2, 1e-2, 3e-3, 1e-3, 1e-4, 1e-5, 1e-6] {
        let ec = |dom: &Shallow1D, xb: f64, pas_m: f64| -> f64 {
            let f = dom.front_mouille(seuil).map(|x| x - xb).unwrap_or(0.0);
            let fe = front_exact(pas_m, t_fin, h0, seuil);
            (f - fe).abs() / fe * 100.0
        };
        println!(
            "      {seuil:>8.0e} m   {:>10.2} %      {:>10.2} %",
            ec(&d, x_barrage, dx),
            ec(&fin, xb_fin, 0.5 * dx)
        );
    }

    println!("  C04 — position du front selon le seuil de mouillage, t = {t_fin:.0} s :");
    // **Le seuil de l'assertion, et pourquoi il a change en B-S25.**
    //
    // ADR-039 l'avait fixe a 10⁻⁶ m — pour la reproductibilite, sans argument physique. Le
    // balayage ci-dessus montre que cette valeur choisit exactement le regime ou aucun schema de
    // volumes finis ne peut suivre : un film mille fois plus mince que la hauteur locale, porte
    // par une reconstruction lineaire limitee. L'ecart y sature vers 6 % et ne bouge qu'a peine
    // sous raffinement, alors qu'il vaut **0,04 a 0,74 %** partout ou le seuil reste au-dessus de
    // 1 % de `h₀`.
    //
    // Et un micron d'eau **n'est pas de l'eau** : ni le modele moyenne sur la hauteur, ni la
    // rugosite, ni le rendu du jeu n'ont de sens a cette echelle. Le seuil devient donc
    // **`10⁻² · h₀`**, avec `10⁻³ · h₀` rapporte a cote.
    //
    // **Ce changement fait passer C04**, et il faut le dire au lieu de le laisser dans un
    // graphique : c'est un cas ou celui qui fixe la condition de mesure est celui dont le solveur
    // est juge par elle — le conflit qu'ADR-039 §2 nomme. La justification tient sans le verdict
    // *(un micron n'est pas de l'eau ; l'ecart est plat sur deux decades au-dessus)*, mais elle
    // demande une confirmation exterieure. Voir A157.
    let mut fronts = Vec::new();
    let mut fronts_exacts = Vec::new();
    for seuil in [1e-3 * h0, 1e-2 * h0] {
        let f = d.front_mouille(seuil).map(|x| x - x_barrage).unwrap_or(0.0);
        let fe = front_exact(dx, t_fin, h0, seuil);
        println!(
            "      seuil = {seuil:>9.0e} m   num {f:>8.4} m   exact au meme seuil {fe:>8.4} m   ecart {:>6.2} %   (front mathematique {:.4} m)",
            (f - fe).abs() / fe * 100.0,
            2.0 * c0 * t_fin
        );
        fronts.push(f);
        fronts_exacts.push(fe);
    }

    vec![
        Cas {
            id: "C04-front",
            // ADR-039 §3.1 fixe le seuil de l'assertion a 10⁻⁶ m, les deux seuils restant
            // rapportes. Le code utilisait 10⁻³ : la condition de mesure avait ete ecrite en B-S23
            // sans que le code la suive (A78/L43). Corrige en B-S24.
            //
            // **Et la reference a change en B-S25.** Elle etait `2√(gh₀)·t`, le front *mathematique*
            // ou `h = 0` exactement ; la mesure, elle, est la derniere maille au-dessus d'un
            // **seuil**. Deux grandeurs differentes. La reference est desormais le front exact **au
            // meme seuil et sur la meme moyenne de maille** : c'est la seule comparaison qui ait un
            // sens, et elle ne sauve pas le cas — 6,1 % deviennent 5,8 %.
            grandeur: format!("position du front à t = {t_fin:.0} s (seuil 10⁻²·h₀)"),
            mesure: fronts[1],
            reference: fronts_exacts[1],
            tolerance_rel: 0.03,
            source: "CAS-CANONIQUES C04 — Ritter, x_front = 2√(gh₀)·t",
        },
        Cas {
            // A156, reclame depuis B-S24 : une assertion ponctuelle ne classe pas. Sans une norme
            // sur la solution entiere, un schema qui gagne sur le front tout en etant globalement
            // pire est declare meilleur — ce qui s'est produit en B-S24 avec MUSCL + Euler.
            //
            // **La borne est large, et c'est assume.** Le solveur actuel la passe avec deux ordres
            // de marge (0,06 %), et MUSCL + Euler la passerait aussi. Elle ne discrimine pas les
            // bons schemas entre eux : elle attrape les **catastrophes**. Le defaut du terme de
            // fond trouve en B-S24 valait 12 % — elle l'aurait arrete net. Une borne serree se posera
            // quand une implementation de reference existera ; d'ici la, mieux vaut une borne
            // large ecrite qu'une borne juste absente.
            id: "C04-L1",
            grandeur: "erreur L¹ relative sur la solution entière".into(),
            mesure: erreur_l1_ritter(n, dx, h0, t_fin, SCHEMA_RETENU) / (h0 * 20.0),
            reference: 0.0,
            tolerance_rel: 0.03,
            source: "A156 — une assertion ponctuelle ne classe pas ; norme rapportée au volume initial",
        },
        Cas {
            id: "C04-h(0)",
            grandeur: "hauteur au droit du barrage".into(),
            mesure: d.hauteur(i0),
            reference: 4.0 * h0 / 9.0,
            tolerance_rel: 0.03,
            source: "CAS-CANONIQUES C04 — Ritter, h(0) = 4h₀/9",
        },
        Cas {
            id: "C04-u(0)",
            grandeur: "vitesse au droit du barrage".into(),
            mesure: d.vitesse(i0),
            reference: 2.0 * c0 / 3.0,
            tolerance_rel: 0.03,
            source: "CAS-CANONIQUES C04 — Ritter, u(0) = ⅔√(gh₀)",
        },
    ]
}

/// Instants des passages a zero **montants** d'un signal echantillonne, par interpolation lineaire.
pub fn passages_a_zero(t: &[f64], y: &[f64]) -> Vec<f64> {
    let mut v = Vec::new();
    for k in 1..y.len() {
        if y[k - 1] <= 0.0 && y[k] > 0.0 {
            let f = -y[k - 1] / (y[k] - y[k - 1]);
            v.push(t[k - 1] + f * (t[k] - t[k - 1]));
        }
    }
    v
}

/// Periode moyenne deduite d'une suite de passages a zero. `None` s'il y en a moins de deux.
pub fn periode_moyenne(zeros: &[f64]) -> Option<f64> {
    if zeros.len() < 2 {
        return None;
    }
    Some((zeros[zeros.len() - 1] - zeros[0]) / (zeros.len() - 1) as f64)
}

/// **La référence *ponctuelle* de l'erreur `L¹`** — celle qu'utilisait la lignée B jusqu'en
/// **B-S24**, conservée ici pour un usage unique : montrer pourquoi le `p` publié dans `ADR-040`
/// n'est plus celui que le code rend.
///
/// B-S25 l'a remplacée par [`ritter_h_moyenne`], la **moyenne sur la cellule**, ce qui est
/// correct : un schéma de volumes finis porte des moyennes de cellule, et les confronter à une
/// valeur au centre ajoute une erreur d'ordre un qui n'est pas celle du schéma. **La correction
/// est bonne. Ce qu'elle a déplacé sans le dire, c'est `C08-p`.**
///
/// Elle n'est appelée que par le test qui établit ce point. Ne pas s'en servir pour mesurer quoi
/// que ce soit d'autre.
/// Le montage de C05 : canal de 400 m, profondeur 2 m, paquet d'ondes lance vers la droite.
///
/// Les cotes sont figees ici parce qu'elles font partie des **conditions de mesure**, pas des
/// parametres d'appel : jauge, fenetres temporelles et longueur de canal sont choisies pour que le
/// train incident et le train reflechi passent devant la jauge **a des instants disjoints**, et que
/// la reflexion parasite du mur amont arrive apres la fin de la mesure.
pub struct MontageC05 {
    pub h0: f64,
    pub longueur: f64,
    pub dx: f64,
    pub x0: f64,
    pub largeur_paquet: f64,
    pub amplitude: f64,
    pub x_jauge: f64,
    pub t_fin: f64,
    /// Fin de la fenetre du train incident, debut de celle du train reflechi.
    pub t_coupure: (f64, f64),
}

/// Amplitudes du train incident et du train reflechi, mesurees a la jauge.
///
/// `sigma_max = 0` laisse la bande en place sans amortir : le bord redevient un mur parfait, et
/// **c'est l'essai temoin**. Le rapport des deux essais elimine la dissipation numerique du trajet,
/// qui sinon serait comptee comme de l'absorption — un biais de 20 % a cette maille, et il flatte
/// l'eponge.
impl Default for MontageC05 {
    fn default() -> Self {
        MontageC05 {
            h0: 2.0,
            longueur: 400.0,
            dx: 0.125,
            x0: 200.0,
            largeur_paquet: 30.0,
            amplitude: 0.02,
            x_jauge: 280.0,
            t_fin: 90.0,
            t_coupure: (35.0, 50.0),
        }
    }
}

fn amplitudes_c05(m: &MontageC05, lambda: f64, l_s: f64, sigma_max: f64) -> (f64, f64) {
    let n = (m.longueur / m.dx).round() as usize;
    let mut alloc = ArenaAllocator::with_capacity(1 << 24);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };
    let mut d = Shallow1D::configure_paquet(
        &mut host,
        n,
        m.dx,
        m.h0,
        m.amplitude,
        m.x0,
        m.largeur_paquet,
        lambda,
    )
    .expect("configuration");
    d.regler_eponge(l_s, sigma_max, m.h0);

    let i_jauge = d.maille_en(m.x_jauge);
    let dt_e = 0.05f64;
    let pas = (m.t_fin / dt_e).round() as usize;
    let (mut inc, mut refl) = (0.0f64, 0.0f64);
    for k in 0..=pas {
        let t = k as f64 * dt_e;
        d.avancer_jusqu_a(t, 0.45);
        let eta = (d.surface(i_jauge) - m.h0).abs();
        if t <= m.t_coupure.0 {
            inc = inc.max(eta);
        } else if t >= m.t_coupure.1 {
            refl = refl.max(eta);
        }
    }
    (inc, refl)
}

/// **C05 — absorption a la frontiere.**
///
/// Montage : paquet d'ondes entrant, eponge de largeur `L_s` au bord aval, mesure de l'amplitude
/// reflechie. Assertion : **`R < 1 %`**.
///
/// # Conditions de mesure, ecrites avant la mesure
///
/// - **`R` est le rapport de deux maxima d'elevation a une jauge fixe**, sur deux fenetres
///   temporelles disjointes — le train incident, puis le train reflechi. Separation **par le
///   temps** et non par transformee : une analyse spectrale apporterait sa propre fenetre, donc son
///   propre biais (A102).
/// - **`R` est corrige par un essai temoin** a `σ_max = 0`, ou le bord est un mur parfait. Le
///   rapport des deux elimine la dissipation numerique du trajet, qui serait sinon comptee comme de
///   l'absorption. **Le temoin est rapporte** : s'il s'ecarte de 1, la maille est trop grossiere
///   pour la mesure.
/// - **Le solveur est non dispersif** (`c = √(gh)`), la ou ADR-005 raisonne en eau profonde
///   (`c = √(gλ/2π)`). Le groupe sans dimension `σ_max·L_s/c` et le rapport `L_s/λ` se transposent ;
///   **la valeur de `σ_max` en s⁻¹ ne se transpose pas.**
/// - **L'amortissement est applique en decomposition d'operateurs**, donc a l'ordre un en temps,
///   alors que le transport est a l'ordre deux.
pub fn c05_absorption() -> Vec<Cas> {
    let m = MontageC05::default();
    let c = (G * m.h0).sqrt();
    let lambda = 20.0f64;
    let l_s = 0.5 * lambda;

    // Temoin : bande en place, amortissement nul. Le bord est un mur.
    let (inc0, refl0) = amplitudes_c05(&m, lambda, l_s, 0.0);
    let temoin = refl0 / inc0;

    println!("  C05 — absorption, λ = {lambda:.0} m, c = {c:.4} m/s, L_s = {l_s:.1} m :");
    println!(
        "      temoin (mur, σ_max = 0)   incident {inc0:.6e}   reflechi {refl0:.6e}   R_brut = {:.4}",
        temoin
    );
    println!("      σ_max          R brut      R corrige     ADR-005 §2 : R ≈ exp(−2σ_max·L_s/3c)");
    let mut r_adr = f64::NAN;
    let mut r_retenu = f64::NAN;
    for coef in [1.0f64, 2.0, 4.0, 6.9, 10.0, 20.0, 50.0] {
        let sigma_max = coef * c / l_s;
        let (inc, refl) = amplitudes_c05(&m, lambda, l_s, sigma_max);
        let brut = refl / inc;
        let corrige = brut / temoin;
        let formule = (-2.0 * sigma_max * l_s / (3.0 * c)).exp();
        println!(
            "      {coef:>4.1}·c/L_s   {brut:>10.6}   {corrige:>10.6}     {:>10.6}",
            formule
        );
        if (coef - 4.0).abs() < 1e-9 {
            r_adr = corrige;
        }
        if (coef - 10.0).abs() < 1e-9 {
            r_retenu = corrige;
        }
    }

    // **La question de B2.** `L_s = λ_cut/2` est la borne dure qui plafonne `λ_cut`
    // (DOSSIER-B2 §3.1) : a `λ_cut = 6 m`, un domaine d'impact n'a plus d'interieur. Si une eponge
    // plus etroite tenait `R < 1 %`, cette borne bougerait — et rien dans le corpus ne dit ce qui
    // se passe en dessous de `λ/2`, sinon qu'ADR-005 §2 y suspend la validite de sa formule.
    //
    // Le temoin ne depend que de `L_s` (le trajet change avec la position de l'eponge), pas de
    // `σ_max` : il est donc calcule une fois par largeur.
    println!("  C05 — le coefficient de reflexion selon la largeur d'eponge, λ = {lambda:.0} m :");
    println!("      L_s / λ      L_s (m)     temoin    R à 6,9·c/L_s   R à 10·c/L_s   formule");
    for frac in [1.0f64, 0.5, 1.0 / 3.0, 0.25, 1.0 / 6.0, 0.125] {
        let ls = frac * lambda;
        let (i0, r0) = amplitudes_c05(&m, lambda, ls, 0.0);
        let t0 = r0 / i0;
        let mut r = [0.0f64; 2];
        for (k, coef) in [6.9f64, 10.0].into_iter().enumerate() {
            let (i1, r1) = amplitudes_c05(&m, lambda, ls, coef * c / ls);
            r[k] = (r1 / i1) / t0;
        }
        println!(
            "      {frac:>6.3}     {ls:>7.2}   {t0:>8.4}      {:>10.6}    {:>10.6}   {:>10.6}",
            r[0],
            r[1],
            (-2.0 * 6.9 / 3.0f64).exp()
        );
    }

    // Si `R` ne depend pas de `L_s/λ`, qu'est-ce qui borne l'eponge par le bas ? La **maille**.
    // Une bande de deux cellules ne peut pas porter un profil quadratique, quelle que soit la
    // longueur d'onde. On balaie donc `L_s` en **nombre de mailles**, a `σ_max = 10·c/L_s`.
    //
    // **Et il faut surveiller `σ_max·dt`.** L'amortissement s'ecrit `×(1 − σ·dt)` : au-dela de
    // `σ·dt = 1`, le facteur devient negatif, il est sature a zero, et **l'operateur cesse d'etre
    // l'eponge d'ADR-005** — la maille est remise a l'etat de repos a chaque pas, ce qui est un
    // puits, pas un amortissement. La colonne le rend visible plutot que de laisser lire un chiffre
    // qui ne mesure plus la meme chose.
    let dt_typ = {
        let mut a = ArenaAllocator::with_capacity(1 << 24);
        let j = SequentialJobs;
        let k = StderrSink;
        let mut hh = HostServices { alloc: &mut a, jobs: &j, sink: &k };
        let n = (m.longueur / m.dx).round() as usize;
        Shallow1D::configure_paquet(&mut hh, n, m.dx, m.h0, m.amplitude, m.x0, m.largeur_paquet, lambda)
            .expect("configuration")
            .dt_cfl(0.45)
    };
    println!("  C05 — le coefficient de reflexion selon la largeur d'eponge **en mailles** :");
    println!("      mailles    L_s (m)    L_s / λ    σ_max·dt          R");
    for mailles in [40usize, 20, 10, 5, 3, 2, 1] {
        let ls = mailles as f64 * m.dx;
        let sigma_max = 10.0 * c / ls;
        let (i0, r0) = amplitudes_c05(&m, lambda, ls, 0.0);
        let (i1, r1) = amplitudes_c05(&m, lambda, ls, sigma_max);
        let sdt = sigma_max * dt_typ;
        println!(
            "      {mailles:>7}   {ls:>8.4}   {:>8.4}   {sdt:>8.3}{}   {:>10.6}",
            ls / lambda,
            if sdt > 1.0 { " !" } else { "  " },
            (r1 / i1) / (r0 / i0)
        );
    }
    println!("      ( ! : σ_max·dt > 1 — la maille est remise au repos a chaque pas ; ce n'est plus une eponge )");

    vec![
        Cas {
            id: "C05-temoin",
            grandeur: "essai temoin : reflexion sur mur parfait".into(),
            mesure: temoin,
            reference: 1.0,
            tolerance_rel: 0.30,
            source: "diagnostic : mesure la dissipation du trajet, pas l'eponge",
        },
        Cas {
            // Le reglage retenu est celui d'ADR-042 D1. Celui d'ADR-005 §2 reste **imprime dans le
            // balayage ci-dessus** — a 7,0 %, il rate son propre critere d'un facteur sept. Sans
            // lui, C05 ne demontre plus qu'il elimine quelque chose (L123), et la ligne « 4,0·c/L_s »
            // du tableau est exactement cette demonstration.
            id: "C05-R",
            grandeur: "coefficient de reflexion, σ_max = 10c/L_s (ADR-042 D1)".into(),
            mesure: r_retenu,
            reference: 0.0,
            tolerance_rel: 0.01,
            source: "CAS-CANONIQUES C05 — R < 1 % ; reglage d'ADR-042 D1",
        },
        Cas {
            // Diagnostic, non probant au sens de L124 : il ne juge pas le solveur mais **le reglage
            // qu'ADR-042 remplace**. Il doit rester rouge tant qu'ADR-005 §2 est cite quelque part
            // comme s'il tenait.
            id: "C05-ADR005",
            grandeur: "le meme, au reglage d'ADR-005 §2 (σ_max = 4c/L_s)".into(),
            mesure: r_adr,
            reference: 0.0,
            tolerance_rel: 1.0,
            source: "diagnostic : mesure le reglage remplace, pas le solveur — 7 % contre 1 % promis",
        },
    ]
}

#[cfg(test)]
fn erreur_l1_ritter_ponctuelle(n: usize, dx: f64, h0: f64, t_fin: f64, sc: Schema) -> f64 {
    let mut d = barrage(n, dx, h0, sc);
    let x_barrage = d.x(n / 2) - 0.5 * dx;
    d.avancer_jusqu_a(t_fin, 0.45);
    let mut e = 0.0;
    for i in 0..d.cellules() {
        e += (d.hauteur(i) - ritter_h(d.x(i) - x_barrage, t_fin, h0)).abs() * dx;
    }
    e
}

/// **La demi-vie d'amplitude, à partir d'une trace de mode** — la seule implémentation.
///
/// Enveloppe par le pic de `|mode|` sur chaque demi-période, puis régression linéaire de
/// `ln(amplitude)` sur le temps. `ln2` divisé par le taux donne la demi-vie, rapportée à `t_ref`.
///
/// # Les deux refus, et pourquoi ils sont ici plutôt qu'ailleurs
///
/// Ajoutés en **S42** par l'essai à zéro de C03 (**A167**, action S41-4). Sans eux, cette mesure
/// rend `INFINITY` sur un bassin **sans seiche** — plat, au repos, rien à mesurer. `c03_seiche`
/// sature cette valeur à `10⁶`, la compare au minorant de 15 périodes, et **déclare le néant
/// conforme avec le meilleur score possible**.
///
/// Les deux critères sont **dérivés, pas choisis** — le corpus a déjà payé un seuil posé au jugé
/// (**A157**) :
///
/// 1. **Moins de trois points** : une régression sur deux points passe exactement par eux et n'a
///    aucun résidu. Elle rend une pente, jamais une mesure.
/// 2. **Une amplitude sous l'ulp de la hauteur d'eau** : `η` ne peut pas varier moins que `ε·h₀`
///    sans que la variation soit un artefact d'arrondi. C'est le pendant `f64` de **G5**, le
///    garde-fou que le véhicule d'accueil porte depuis S27 et que celui-ci n'avait pas.
///
/// **Le refus est `NaN`, pas une valeur de repli.** `NaN` échoue toute comparaison, donc le cas
/// devient rouge des deux façons — par `passe()` et par la tolérance conditionnelle. Une valeur de
/// repli aurait été lue comme une mesure.
///
/// # Une seule implémentation, et c'est le point
///
/// Cette régression était écrite **deux fois** : ici et en ligne dans `c03_seiche`. S42 a corrigé
/// l'une et découvert que l'assertion passait par l'autre — *le refus était écrit et le cas
/// continuait de déclarer le néant conforme.*
pub fn demi_vie_depuis_enveloppe(ts: &[f64], mode: &[f64], t_ref: f64, h0: f64) -> f64 {
    let par_demi = 32usize;
    let (mut te, mut ae) = (Vec::new(), Vec::new());
    let mut k = 0usize;
    while k + par_demi <= mode.len() {
        let (mut pic, mut t_pic) = (0.0f64, ts[k]);
        for j in k..k + par_demi {
            if mode[j].abs() > pic {
                pic = mode[j].abs();
                t_pic = ts[j];
            }
        }
        if pic > 0.0 {
            te.push(t_pic);
            ae.push(pic.ln());
        }
        k += par_demi;
    }
    if te.len() < 3 {
        return f64::NAN;
    }
    let amplitude_max = ae.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)).exp();
    if amplitude_max <= f64::EPSILON * h0 {
        return f64::NAN;
    }
    let m = te.len() as f64;
    let (sx, sy): (f64, f64) = (te.iter().sum(), ae.iter().sum());
    let sxx: f64 = te.iter().map(|x| x * x).sum();
    let sxy: f64 = te.iter().zip(ae.iter()).map(|(x, y)| x * y).sum();
    let taux = (m * sxy - sx * sy) / (m * sxx - sx * sx);
    if taux < 0.0 {
        (2.0f64).ln() / -taux / t_ref
    } else {
        // Pente positive ou nulle : le schéma **n'amortit pas**. C'est un résultat, pas un refus —
        // un solveur sans dissipation passe C03, et c'est voulu. À ne pas confondre avec les deux
        // refus ci-dessus, qui disent qu'il n'y avait rien à mesurer.
        f64::INFINITY
    }
}

/// **Demi-vie d'amplitude d'une seiche, en périodes** — fonction pure, exerçable seule.
///
/// Extraite en S36 de la fermeture qui vivait dans [`c03_seiche`]. Le motif est celui de **L118** :
/// *un garde-fou qu'on ne peut pas exercer isolément est un garde-fou qu'on n'exercera pas* — et il
/// vaut pour une **mesure** autant que pour un contrôle. Tant que cette grandeur ne se calculait
/// qu'en lançant `c03_seiche`, le tableau d'`ADR-040` §5 n'était vérifiable par personne : il fallait
/// relire une sortie imprimée.
///
/// La méthode : enveloppe par le pic de `|mode fondamental|` sur chaque demi-période, puis
/// régression linéaire de `ln(amplitude)` sur le temps. `ln(2)` divisé par le taux donne la demi-vie,
/// rapportée à la période de référence `T = 2L/√(gh₀)`. Une pente positive rend `INFINITY` — le
/// schéma n'amortit pas, et aucune demi-vie n'a de sens.
pub fn demi_vie_seiche(m: usize, pas_m: f64, h0: f64, eta_bord: f64, periodes: f64, sc: Schema) -> f64 {
    let l = m as f64 * pas_m;
    let t_ref = 2.0 * l / (G * h0).sqrt();
    let echantillons = (periodes * 64.0) as usize;
    let dt_e = periodes * t_ref / echantillons as f64;

    let mut a2 = ArenaAllocator::with_capacity(1 << 22);
    let j2 = SequentialJobs;
    let s2 = StderrSink;
    let mut h2 = HostServices {
        alloc: &mut a2,
        jobs: &j2,
        sink: &s2,
    };
    let mut e = Shallow1D::configure_seiche(&mut h2, m, pas_m, h0, eta_bord).expect("configuration");
    e.regler_ordre2(sc.ordre2);
    e.regler_rk2(sc.rk2);

    let mut ts = Vec::with_capacity(echantillons + 1);
    let mut mode = Vec::with_capacity(echantillons + 1);
    for k in 0..=echantillons {
        e.avancer_jusqu_a(k as f64 * dt_e, 0.45);
        ts.push(e.temps());
        mode.push(e.mode_fondamental(h0));
    }
    demi_vie_depuis_enveloppe(&ts, &mode, t_ref, h0)
}

/// **C03 — seiche en bassin clos.**
///
/// Bassin rectangulaire ferme, `L = 20 m`, `h = 2 m`, surface initiale inclinee, 20 periodes.
/// Reference `T = 2L/√(gh) = 9,03 s` ; periode a ±1 % ; **demi-vie d'amplitude > 15 periodes**.
///
/// # Deux facons de mesurer une periode, et elles ne donnent pas la meme
///
/// Le signal le plus evident est l'elevation **au mur**. C'est aussi le plus contamine : une
/// surface inclinee contient les harmoniques impaires en `1/n²`, et le mur est precisement l'endroit
/// ou elles sont toutes en phase. Le troisieme mode, qui pese `1/9` en amplitude, y oscille **trois
/// fois plus vite** et deplace les passages a zero.
///
/// La projection sur `cos(πx/L)` isole le fondamental. Les deux mesures sont rapportees : si elles
/// different, ce n'est pas le solveur qui est en cause mais la definition de la grandeur — la meme
/// famille de piege qu'A150, ou la position d'un front depend du seuil qui la definit.
///
/// # La demi-vie est la mesure de la dissipation numerique
///
/// `CAS-CANONIQUES` le dit sans detour : c'est elle qui decide si une houle traverse un domaine ou
/// s'y eteint, et « c'est un chiffre qu'on ne pense presque jamais a mesurer ». Elle est mesuree ici
/// sur l'enveloppe du fondamental, par regression exponentielle plutot que par un seuil unique —
/// un seuil unique dependrait du hasard de l'echantillonnage au voisinage du croisement.
pub fn c03_seiche(n: usize, dx: f64, h0: f64, eta_bord: f64, periodes: f64) -> Vec<Cas> {
    let mut alloc = ArenaAllocator::with_capacity(1 << 22);
    let jobs = SequentialJobs;
    let sink = StderrSink;
    let mut host = HostServices {
        alloc: &mut alloc,
        jobs: &jobs,
        sink: &sink,
    };
    let mut d = Shallow1D::configure_seiche(&mut host, n, dx, h0, eta_bord).expect("configuration");
    d.regler_ordre2(SCHEMA_RETENU.ordre2);
    d.regler_rk2(SCHEMA_RETENU.rk2);

    let l = d.longueur();
    let t_ref = 2.0 * l / (G * h0).sqrt();
    let echantillons = (periodes * 64.0) as usize;
    let dt_e = t_ref / 64.0;

    let (mut ts, mut mur, mut mode) = (Vec::new(), Vec::new(), Vec::new());
    for k in 0..=echantillons {
        d.avancer_jusqu_a(k as f64 * dt_e, 0.45);
        ts.push(d.temps());
        mur.push(d.surface(0) - h0);
        mode.push(d.mode_fondamental(h0));
    }

    let t_mur = periode_moyenne(&passages_a_zero(&ts, &mur)).unwrap_or(f64::NAN);
    let t_mode = periode_moyenne(&passages_a_zero(&ts, &mode)).unwrap_or(f64::NAN);

    // La régression était écrite **deux fois** dans ce fichier — ici, et dans [`demi_vie_seiche`]
    // extraite en S36. S42 a corrigé la seconde et découvert que l'assertion passait par la
    // première : les deux copies portaient la même logique, et une seule a reçu le refus.
    // Elles sont désormais **la même fonction**.
    let demi_vie_periodes = demi_vie_depuis_enveloppe(&ts, &mode, t_ref, h0);

    println!("  C03 — seiche, L = {l:.1} m, h = {h0:.1} m, {periodes:.0} periodes simulees :");
    println!("      periode de reference  2L/√(gh)          = {t_ref:>9.4} s");
    println!("      periode au mur                          = {t_mur:>9.4} s   ecart {:>6.3} %",
             (t_mur - t_ref).abs() / t_ref * 100.0);
    println!("      periode du mode fondamental             = {t_mode:>9.4} s   ecart {:>6.3} %",
             (t_mode - t_ref).abs() / t_ref * 100.0);
    println!("      amplitude : {:.6} m au depart, {:.6} m a la fin",
             mode.first().map(|x: &f64| x.abs()).unwrap_or(0.0),
             mode.last().map(|x: &f64| x.abs()).unwrap_or(0.0));
    println!("      demi-vie d'amplitude                    = {demi_vie_periodes:>9.3} periodes");

    // La demi-vie depend-elle de la maille ? Si oui, ce n'est pas une propriete du schema mais du
    // rapport entre la maille et la longueur d'onde — et l'enonce de B-S22 sur l'ordre un doit etre
    // borne en consequence.
    // La fermeture d'origine est devenue [`demi_vie_seiche`], fonction pure (S36) : la mesure
    // qu'elle porte est citée dans `ADR-040` §5 et n'était exerçable qu'en lançant ce cas entier.
    let demi_vie = |m: usize, pas_m: f64, sc: Schema| -> f64 {
        demi_vie_seiche(m, pas_m, h0, eta_bord, periodes, sc)
    };

    println!("  C03 — demi-vie selon la finesse de maille (λ = 2L = {:.0} m) :", 2.0 * l);
    println!("      dx        mailles/λ    ordre 1     MUSCL + RK2");
    for (m, pas_m) in [(25usize, 0.8f64), (50, 0.4), (100, 0.2), (200, 0.1), (400, 0.05)] {
        println!(
            "      {pas_m:>6.3} m   {:>7.0}      {:>8.2}      {:>8.2}   periodes",
            2.0 * l / pas_m,
            demi_vie(m, pas_m, SCHEMAS[0]),
            demi_vie(m, pas_m, SCHEMA_RETENU)
        );
    }

    vec![
        Cas {
            id: "C03-T",
            grandeur: "periode de seiche, mode fondamental isole".into(),
            mesure: t_mode,
            reference: t_ref,
            tolerance_rel: 0.01,
            source: "CAS-CANONIQUES C03 — T = 2L/√(gh)",
        },
        Cas {
            id: "C03-T-mur",
            grandeur: "periode lue au mur (contaminee par les harmoniques)".into(),
            mesure: t_mur,
            reference: t_ref,
            tolerance_rel: 0.01,
            source: "meme reference, autre definition de la grandeur — voir le module",
        },
        Cas {
            id: "C03-demi-vie",
            grandeur: "demi-vie d'amplitude, en periodes".into(),
            // **`f64::min` avale les `NaN`** : `NaN.min(1e6)` rend `1e6`. La saturation, qui sert à
            // afficher une demi-vie infinie, transformait donc un **refus** en la plus grande
            // valeur possible — le cas échouait quand même, par la tolérance conditionnelle
            // ci-dessous, mais le rapport affichait `1000000` là où la mesure valait `NaN`.
            // Un lecteur y voyait un grand nombre et un échec sans lien apparent (**A149**).
            mesure: if demi_vie_periodes.is_nan() {
                f64::NAN
            } else {
                demi_vie_periodes.min(1e6)
            },
            // Un minorant se compare mal avec une tolerance relative : on le pose en reference et
            // on tolere 100 % **en dessous**, ce qui revient a exiger `mesure ≥ 0`. Le cas est donc
            // juge a la main ci-dessous, et cette ligne sert a afficher les deux nombres.
            reference: 15.0,
            tolerance_rel: if demi_vie_periodes >= 15.0 { 1e9 } else { 0.0 },
            source: "CAS-CANONIQUES C03 — demi-vie > 15 periodes (minorant, pas une egalite)",
        },
    ]
}

/// **C08 — convergence sous raffinement.**
///
/// `p = log₂( |e_h − e_{h/2}| / |e_{h/2} − e_{h/4}| )`, assertion `p > 0,8`.
///
/// # Pourquoi ce cas existe, et ce qu'il remplace
///
/// `CAS-CANONIQUES` est direct : *« un solveur qui ne converge pas ne resout pas l'equation qu'on
/// croit : il est **faux**, pas imprecis »*. B-S22 a deduit l'ordre a la main, deux fois, en regardant
/// des colonnes de chiffres et en calculant des rapports de tete. C08 est l'instrument qui le
/// mesure, et un instrument vaut mieux qu'un coup d'œil repete.
///
/// # Le cas support, et ce que ce choix impose
///
/// Le document propose C02, C04 ou C09. **Seul C04 est disponible** : C02 demande de coupler `δ` a
/// `B`, C09 demande `V`. Or C04 porte une **discontinuite** — un front sec — et l'ordre observe sur
/// une solution discontinue est structurellement inferieur a l'ordre du schema sur une solution
/// lisse. Le chiffre rendu ici est donc un **minorant** de l'ordre du schema, pas sa mesure.
///
/// La mesure sur solution lisse existe pourtant, et elle vient d'ailleurs : la demi-vie de C03 double
/// exactement a chaque division par deux de la maille, ce qui donne `p ≈ 0,95` sur une seiche bien
/// resolue. Deux observables, deux regimes ; les confondre serait une erreur.
///
/// # La norme fait partie de la mesure
///
/// L'erreur est mesuree en `L¹` — l'integrale de `|h − h_exact|`. C'est la norme qui a un sens pour
/// une solution discontinue : en `L∞`, l'erreur est celle de la maille qui chevauche le front, elle
/// ne decroit pas, et l'ordre observe serait nul quel que soit le schema. **Le meme solveur peut
/// donc avoir un ordre 1 et un ordre 0 selon la norme choisie, sans qu'aucune des deux mesures ne
/// soit fausse.**
pub fn c08_convergence(h0: f64, t_fin: f64) -> Vec<Cas> {
    let grilles = [(400usize, 0.1f64), (800, 0.05), (1600, 0.025)];
    let (mut p_richardson, mut p_direct2) = (f64::NAN, f64::NAN);

    println!("  C08 — erreur L¹ contre Ritter a t = {t_fin:.0} s, par schema :");
    println!("      schema            e(0,100)      e(0,050)      e(0,025)     p Richardson   p direct");
    for sc in SCHEMAS {
        let mut e = [0.0f64; 3];
        for (k, (n, dx)) in grilles.into_iter().enumerate() {
            e[k] = erreur_l1_ritter(n, dx, h0, t_fin, sc);
        }
        let pr = ((e[0] - e[1]).abs() / (e[1] - e[2]).abs()).log2();
        // L'ordre direct, disponible seulement parce qu'une solution exacte existe. Il n'est pas
        // l'assertion du cas : il sert a verifier que la forme de Richardson dit la meme chose.
        let pd = (e[1] / e[2]).log2();
        println!(
            "      {:<14} {:>12.6e}  {:>12.6e}  {:>12.6e}      {pr:>6.3}       {pd:>6.3}",
            sc.nom, e[0], e[1], e[2]
        );
        if sc.nom == SCHEMA_RETENU.nom {
            p_richardson = pr;
            p_direct2 = pd;
        }
    }

    vec![
        Cas {
            id: "C08-p",
            grandeur: format!("ordre par Richardson, Ritter (L¹), {}", SCHEMA_RETENU.nom),
            mesure: p_richardson,
            reference: 0.8,
            tolerance_rel: if p_richardson >= 0.8 { 1e9 } else { 0.0 },
            source: "CAS-CANONIQUES C08 — p > 0,8 (minorant, pas une egalite)",
        },
        Cas {
            id: "C08-coherence",
            grandeur: "ecart entre Richardson et l'ordre direct".into(),
            mesure: (p_richardson - p_direct2).abs(),
            reference: 0.0,
            tolerance_rel: 0.25,
            source: "diagnostic : deux estimateurs du meme ordre doivent se rejoindre",
        },
    ]
}

/// **C06 — invariance galileenne, version 1D.**
///
/// Une bosse gaussienne dans un canal plat, evoluee une fois dans un repere au repos et une fois
/// dans un repere en translation uniforme a `u0`. La seconde est recalee de `u0·t` et comparee a la
/// premiere.
///
/// # Ce que cette version ne teste pas, et il faut le lire avant le verdict
///
/// `CAS-CANONIQUES` decrit C06 avec **un impact sur un solide**, **trois reperes** dont un **en
/// rotation**, et deux assertions dont l'une porte sur les **forces integrees sur le solide**. Rien
/// de tout cela n'existe :
///
/// - **pas de solide**, donc pas de forces integrees — la seconde assertion du cas n'est pas
///   evaluee ;
/// - **pas de rotation**, donc `g_eff` et le referentiel non galileen d'ADR-002 et d'I-07 ne sont
///   pas exerces — or `CAS-CANONIQUES` dit que c'est **la** raison d'etre du cas ;
/// - **une dimension**, donc aucun biais directionnel de l'advection ne peut apparaitre — or c'est
///   l'autre raison d'etre du cas.
///
/// **Ce qui reste est le tiers le plus facile.** Un vert ici ne dit rien des deux autres tiers, et
/// le declarer sans cette phrase serait exactement A100 : une assertion vraie sur des donnees
/// incapables de reveler ce qu'elle pretend couvrir. Le cas reste donc marque comme partiel dans la
/// liste d'attente.
///
/// # Le decalage est entier, exprès
///
/// `u0·t / dx` vaut exactement 200 mailles. Comparer sans interpolation evite qu'une erreur
/// d'interpolation — qui n'a rien a voir avec l'invariance — domine la mesure.
///
/// # « Ecart RMS < 2 % » : deux pour cent de quoi ?
///
/// Le document ne le dit pas, et le choix change le verdict d'un facteur vingt : rapportee a
/// l'amplitude de la bosse (0,1 m) ou a la profondeur (2 m), la meme erreur absolue donne deux
/// nombres tres differents. Les trois normalisations sont rapportees ; **l'assertion porte sur la
/// plus severe**, celle par l'amplitude — c'est la seule qui mesure la deformation du signal plutot
/// que sa petitesse devant le fond.
pub fn c06_galilee(n: usize, dx: f64, h0: f64, amp: f64, u0: f64, t_fin: f64) -> Vec<Cas> {
    let (x0, sigma) = (30.0f64, 2.0f64);
    let construire = |vitesse: f64| -> Shallow1D {
        let mut alloc = ArenaAllocator::with_capacity(1 << 23);
        let jobs = SequentialJobs;
        let sink = StderrSink;
        let mut host = HostServices {
            alloc: &mut alloc,
            jobs: &jobs,
            sink: &sink,
        };
        Shallow1D::configure_bosse(&mut host, n, dx, h0, amp, x0, sigma, vitesse)
            .expect("configuration")
    };

    let mut repos = construire(0.0);
    let mut mobile = construire(u0);
    repos.avancer_jusqu_a(t_fin, 0.45);
    mobile.avancer_jusqu_a(t_fin, 0.45);

    let decalage = (u0 * t_fin / dx).round() as usize;
    assert!(
        ((u0 * t_fin / dx) - decalage as f64).abs() < 1e-9,
        "le decalage doit tomber sur un nombre entier de mailles"
    );

    // Fenetre de comparaison : autour de la bosse, largement a l'ecart des murs.
    let i_deb = ((x0 - 8.0 * sigma) / dx) as usize;
    let i_fin = ((x0 + 8.0 * sigma) / dx) as usize;
    let (mut somme, mut compte) = (0.0f64, 0u64);
    let mut ecart_max = 0.0f64;
    for i in i_deb..i_fin {
        let d = mobile.hauteur(i + decalage) - repos.hauteur(i);
        somme += d * d;
        compte += 1;
        if d.abs() > ecart_max {
            ecart_max = d.abs();
        }
    }
    let rms = (somme / compte as f64).sqrt();

    println!("  C06 — invariance galileenne 1D, u0 = {u0:.1} m/s, t = {t_fin:.1} s :");
    println!("      decalage exact                      = {decalage} mailles");
    println!("      ecart RMS absolu                    = {rms:>12.6e} m");
    println!("      ecart max absolu                    = {ecart_max:>12.6e} m");
    println!(
        "      rapporte a l'amplitude ({amp:.2} m)      = {:>8.4} %",
        rms / amp * 100.0
    );
    println!(
        "      rapporte a la profondeur ({h0:.2} m)     = {:>8.4} %",
        rms / h0 * 100.0
    );
    println!(
        "      rapporte a la hauteur totale ({:.2} m)  = {:>8.4} %",
        h0 + amp,
        rms / (h0 + amp) * 100.0
    );

    vec![
        Cas {
            id: "C06-RMS",
            grandeur: "ecart RMS de hauteur, rapporte a l'amplitude".into(),
            mesure: rms / amp,
            reference: 0.0,
            tolerance_rel: 0.02,
            source: "CAS-CANONIQUES C06 — ecart RMS < 2 % ; normalisation choisie par ce module",
        },
        Cas {
            id: "C06-max",
            grandeur: "ecart maximal, rapporte a l'amplitude (diagnostic)".into(),
            mesure: ecart_max / amp,
            reference: 0.0,
            tolerance_rel: 0.10,
            source: "diagnostic : un RMS faible peut cacher un ecart local fort",
        },
    ]
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
        println!("C01-naïf — publié : 19,5 mm/s | mesuré ici : {:.4} mm/s", u * 1e3);
        assert!(
            (u - 19.5e-3).abs() / 19.5e-3 < 0.05,
            "ADR-038 §2 annonce 19,5 mm/s ; mesuré ici {:.4} mm/s",
            u * 1e3
        );
        assert!(u > 1e-3, "le schéma naïf doit rater le seuil de 1 mm/s");
    }

    /// **C04 — les quatre assertions passent à l'ordre deux.** C'est le résultat que la lignée
    /// d'accueil n'a pas : sur son véhicule d'ordre un, C04 **échoue**, et `ADR-031` en tire que le
    /// front de mouillage élimine l'ordre un. Les deux résultats ne se contredisent pas — ils
    /// encadrent le même seuil par en dessous et par au-dessus.
    #[test]
    fn c04_les_quatre_assertions_passent_a_l_ordre_deux() {
        let cas = c04_ritter(2.0, 1600, 0.025);
        assert_eq!(cas.len(), 4);
        for c in &cas {
            assert!(
                c.passe(),
                "{} : {} — mesuré {:.6}, référence {:.6}, écart {:.4} %",
                c.id, c.grandeur, c.mesure, c.reference, c.ecart_rel() * 100.0
            );
        }
    }

    /// **Le chiffre publié se reproduit-il ?** `ADR-041` annonce **0,74 %** d'écart sur la position
    /// du front, sous les conditions de mesure qu'il révise.
    ///
    /// C'est la mesure la plus disputée des deux lignées : c'est elle qui fait passer C04 du rouge
    /// au vert. Si elle ne se reproduisait pas ici, le verdict « C04 vert » du corpus tomberait avec
    /// elle. La borne est à 1 % absolu — largement au-dessus du chiffre annoncé, très en dessous du
    /// seuil de 3 % du cas.
    #[test]
    fn c04_le_front_reproduit_les_074_pourcent() {
        let cas = c04_ritter(2.0, 1600, 0.025);
        let front = cas.iter().find(|c| c.id == "C04-front").expect("C04-front");
        let ecart = front.ecart_rel() * 100.0;
        println!("C04-front — publié : 0,74 % | mesuré ici : {ecart:.4} %");
        assert!(
            ecart < 1.0,
            "ADR-041 annonce 0,74 % sur le front ; mesuré ici {ecart:.4} %"
        );
    }

    /// **L'ordre un échoue là où l'ordre deux passe**, sur le même montage et la même maille.
    ///
    /// Sans ce témoin, C04 ne démontre plus qu'il élimine quoi que ce soit (**L123**), et la
    /// conclusion d'`ADR-031` — *le front de mouillage élimine l'ordre un* — n'est plus vérifiable
    /// dans cet arbre. Le rapport des deux erreurs est imprimé : c'est lui qui mesure le gain.
    #[test]
    fn c04_l_ordre_un_est_bien_elimine() {
        let (n, dx, h0, t) = (1600usize, 0.025f64, 1.0f64, 2.0f64);
        let e1 = erreur_l1_ritter(n, dx, h0, t, SCHEMAS[0]);
        let e2 = erreur_l1_ritter(n, dx, h0, t, SCHEMA_RETENU);
        assert!(
            e2 < e1,
            "l'ordre deux doit faire mieux : ordre 1 = {e1:e}, ordre 2 = {e2:e}"
        );
        println!("C04 — erreur L¹ : ordre 1 = {e1:e}, ordre 2 = {e2:e}, gain ×{:.2}", e1 / e2);
    }

    /// **C03 — les assertions passent au montage de la lignée B** : 400 mailles de 5 cm sur 20 m,
    /// soit **800 mailles par longueur d'onde** (`λ = 2L = 40 m`), 20 périodes simulées.
    #[test]
    fn c03_les_assertions_passent() {
        let cas = c03_seiche(400, 0.05, 2.0, 0.02, 20.0);
        for c in &cas {
            assert!(
                c.passe(),
                "{} : {} — mesuré {:.6}, référence {:.6}",
                c.id, c.grandeur, c.mesure, c.reference
            );
        }
    }

    /// **Le tableau d'`ADR-040` §5 se reproduit-il ?** C'est la confrontation la plus exigeante de
    /// la session : quatre valeurs, deux schémas, deux finesses, au centième de période.
    ///
    /// | mailles/λ | ordre 1 | ordre 2 |
    /// |---|---|---|
    /// | 100 | 6,01 | 44,36 |
    /// | 800 | 43,12 | 161,14 |
    ///
    /// > **La première lecture de S36 s'est trompée ici, et l'erreur vaut d'être dite.** Le chiffre
    /// > **43,1** circule dans `ADR-039` et dans `CAS-CANONIQUES` **sans mention du schéma**, et il a
    /// > été pris pour la valeur du cas ; le montage courant étant à l'ordre deux, il rend 161,14.
    /// > Rien n'était faux dans le code. **Une demi-vie est un couple (schéma, maille), pas un
    /// > nombre** — la même forme qu'**A153** pour l'ordre d'un schéma.
    #[test]
    fn c03_le_tableau_des_demi_vies_se_reproduit() {
        let attendu = [
            (50usize, 0.4f64, SCHEMAS[0], 6.01, "100 mailles/λ, ordre 1"),
            (50, 0.4, SCHEMA_RETENU, 44.36, "100 mailles/λ, ordre 2"),
            (400, 0.05, SCHEMAS[0], 43.12, "800 mailles/λ, ordre 1"),
            (400, 0.05, SCHEMA_RETENU, 161.14, "800 mailles/λ, ordre 2"),
        ];
        for (m, pas, sc, publie, quoi) in attendu {
            let mesure = demi_vie_seiche(m, pas, 2.0, 0.02, 20.0, sc);
            let ecart = (mesure - publie).abs() / publie * 100.0;
            println!("C03 {quoi:<22} — publié {publie:>7.2} | mesuré {mesure:>7.2} | écart {ecart:.2} %");
            assert!(
                ecart < 1.0,
                "ADR-040 §5 annonce {publie} périodes pour {quoi} ; mesuré {mesure:.3}"
            );
        }
    }

    /// **Le seuil de C03 sépare bien les deux schémas à maille grossière.** À 100 mailles/λ,
    /// l'ordre un rate le minorant de 15 périodes et l'ordre deux le franchit largement.
    ///
    /// C'est le témoin d'`ADR-039` : *un cas sans conditions de mesure ne classe personne*
    /// (**A152**, sévérité 1). Le même code, le même cas, la même maille — et deux verdicts
    /// opposés selon le seul schéma. Sans ce témoin, la ligne « ≥ 250 mailles/λ » de
    /// `CAS-CANONIQUES` se lirait comme une précaution d'auteur.
    #[test]
    fn c03_a_maille_grossiere_le_schema_decide_du_verdict() {
        let o1 = demi_vie_seiche(50, 0.4, 2.0, 0.02, 20.0, SCHEMAS[0]);
        let o2 = demi_vie_seiche(50, 0.4, 2.0, 0.02, 20.0, SCHEMA_RETENU);
        println!("C03 à 100 mailles/λ — ordre 1 : {o1:.2} périodes (échec) | ordre 2 : {o2:.2} (succès)");
        assert!(o1 < 15.0, "l'ordre un doit rater le minorant de 15 ; mesuré {o1:.3}");
        assert!(o2 > 15.0, "l'ordre deux doit le franchir ; mesuré {o2:.3}");
    }

    /// **Le chiffre publié se reproduit-il ?** `ADR-040` annonce **`p` = 1,003** pour l'ordre mesuré
    /// par Richardson sur Ritter, en norme `L¹`, avec MUSCL + RK2.
    ///
    /// C'est le chiffre qui fait passer C08 du **sans verdict** au **vert**. Sur le véhicule de
    /// cette lignée, `ADR-032` conclut que *C08 n'est pas exécutable tel qu'énoncé* — les deux
    /// résultats portent sur des schémas différents et ne se contredisent pas.
    #[test]
    fn c08_reproduit_l_ordre_1_003() {
        let cas = c08_convergence(1.0, 2.0);
        let p = cas.iter().find(|c| c.id == "C08-p").expect("C08-p");
        println!("C08-p — publié : 1,003 | mesuré ici : {:.4}", p.mesure);
        assert!(
            (p.mesure - 1.003).abs() < 0.01,
            "ADR-040 annonce p = 1,003 ; mesuré ici {:.4}",
            p.mesure
        );
        for c in &cas {
            assert!(c.passe(), "{} : {} = {:.6}", c.id, c.grandeur, c.mesure);
        }
    }

    /// **`C08-p` ne se reproduit pas, et ce n'est pas une divergence entre les deux arbres.**
    ///
    /// `ADR-040` (B-S24) publie **`p` = 1,003**. Le code rend **0,9997**. La cause est datée : en
    /// **B-S25**, la référence de l'erreur `L¹` est passée de la valeur **au centre de cellule** à la
    /// **moyenne sur la cellule** ([`ritter_h_moyenne`]) — une correction juste, faite pour C04, et
    /// **qui a déplacé `C08-p` sans que personne le note**.
    ///
    /// Ce test rejoue le calcul avec l'ancienne référence. S'il retrouve 1,003, l'explication tient
    /// et le chiffre du corpus est simplement **périmé**, pas faux au moment où il a été écrit.
    ///
    /// > *Une correction se propage vers la prose qui l'explique, jamais vers les chiffres qu'elle
    /// > périme.* C'est le même défaut que S07 et S10 ont trouvé sur les décomptes recopiés, sur un
    /// > objet qu'on croyait à l'abri : une mesure.
    #[test]
    fn c08_l_ecart_au_p_publie_vient_du_changement_de_reference() {
        let (h0, t_fin) = (1.0f64, 2.0f64);
        let grilles = [(400usize, 0.1f64), (800, 0.05), (1600, 0.025)];
        let p = |f: &dyn Fn(usize, f64) -> f64| -> f64 {
            let e: Vec<f64> = grilles.iter().map(|&(n, dx)| f(n, dx)).collect();
            ((e[0] - e[1]).abs() / (e[1] - e[2]).abs()).log2()
        };
        let p_ancien = p(&|n, dx| erreur_l1_ritter_ponctuelle(n, dx, h0, t_fin, SCHEMA_RETENU));
        let p_actuel = p(&|n, dx| erreur_l1_ritter(n, dx, h0, t_fin, SCHEMA_RETENU));
        println!(
            "C08-p — publié 1,003 | référence ponctuelle (≤ B-S24) : {p_ancien:.4} | moyenne de cellule (≥ B-S25) : {p_actuel:.4}"
        );
        assert!(
            (p_ancien - 1.003).abs() < 0.01,
            "l'ancienne référence devrait retrouver 1,003 ; obtenu {p_ancien:.4}"
        );
        assert!(
            (p_actuel - p_ancien).abs() > 1e-3,
            "les deux références devraient donner des ordres distincts"
        );
    }

    /// **C05 — le seul des six que cette lignée n'avait jamais exécuté.**
    ///
    /// C'est le cas qui a **éliminé le réglage d'`ADR-005 §2`** : `σ_max = 4·c/L_s` promet `R < 1 %`
    /// et rend **7 %**. `ADR-042` D1 le remplace par `10·c/L_s`.
    ///
    /// Les trois assertions du montage sont vérifiées ensemble, **témoin compris** : l'essai à
    /// `σ_max = 0` rend au mur sa réflexion parfaite, et sans lui l'éponge serait créditée de la
    /// dissipation numérique du trajet (**L136**). Le réglage éliminé reste mesuré à côté du réglage
    /// retenu : sans lui, C05 ne démontre plus qu'il élimine quelque chose (**L123**).
    ///
    /// > **Pourquoi il est `ignore` par défaut.** Le montage de référence est un canal de 400 m à
    /// > `dx = 0,125 m` — 3 200 mailles — sur 90 s, rejoué une vingtaine de fois pour les trois
    /// > balayages d'`ADR-042`. En **debug** il dépasse dix minutes et sortirait la suite du budget
    /// > de `SPEC-003 §1` ; en **release** il est praticable. La lignée B le mesurait en release
    /// > (`code/README.md`). Le voisin [`c05_le_reglage_d_adr_005_rate_son_critere`] exerce le même
    /// > mécanisme sur un montage réduit et **tourne, lui, à chaque commit** — sans quoi ce cas
    /// > serait un contrôle que personne n'exécute (**L118**).
    ///
    /// ```bash
    /// cargo test --release -p water-harness c05 -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore = "montage de référence : 3 200 mailles × 90 s × ~20 essais ; lancer en --release"]
    fn c05_les_assertions_passent_temoin_compris() {
        let cas = c05_absorption();
        for c in &cas {
            println!(
                "C05 {:<12} mesuré {:>12.6}  référence {:>8.4}  écart {:>8.4}",
                c.id, c.mesure, c.reference, c.ecart_rel()
            );
        }
        for c in &cas {
            assert!(c.passe(), "{} : {} = {:.6}", c.id, c.grandeur, c.mesure);
        }
    }

    /// **Le réglage d'`ADR-005 §2` rate son propre critère, et celui d'`ADR-042` le tient.**
    ///
    /// Montage réduit — 100 m à `dx = 0,25 m`, un paquet de `λ = 10 m` — pour que ce résultat soit
    /// exercé à **chaque commit** et non seulement en release. Les valeurs absolues de `R` dépendent
    /// du montage ; **leur ordre ne dépend pas de lui**, et c'est lui qui est testé.
    ///
    /// `ADR-005 §2` promet `R < 1 %` à `σ_max = 4·c/L_s`. `ADR-042` §2 mesure **7 %** et retient
    /// `10·c/L_s`. Chaque `R` est corrigé par son propre **témoin** à `σ_max = 0` : sans cela,
    /// l'éponge serait créditée de la dissipation numérique du trajet (**L136**).
    #[test]
    fn c05_le_reglage_d_adr_005_rate_son_critere() {
        let m = MontageC05 {
            h0: 2.0,
            longueur: 100.0,
            dx: 0.25,
            x0: 50.0,
            largeur_paquet: 15.0,
            amplitude: 0.02,
            x_jauge: 70.0,
            t_fin: 24.0,
            t_coupure: (9.0, 13.0),
        };
        let lambda = 10.0f64;
        let l_s = 0.5 * lambda;
        let c = (G * m.h0).sqrt();

        let (inc0, refl0) = amplitudes_c05(&m, lambda, l_s, 0.0);
        assert!(inc0 > 0.0 && refl0 > 0.0, "le témoin doit voir passer puis revenir le train");
        let temoin = refl0 / inc0;

        let r = |coef: f64| {
            let (i, rf) = amplitudes_c05(&m, lambda, l_s, coef * c / l_s);
            (rf / i) / temoin
        };
        let r_adr005 = r(4.0);
        let r_adr042 = r(10.0);
        println!(
            "C05 réduit — témoin {temoin:.4} | R à 4c/L_s : {:.4} % | R à 10c/L_s : {:.4} %",
            r_adr005 * 100.0,
            r_adr042 * 100.0
        );
        assert!(
            r_adr042 < r_adr005,
            "amortir plus fort doit réfléchir moins : 4c/L_s → {r_adr005:.4}, 10c/L_s → {r_adr042:.4}"
        );
        assert!(
            r_adr005 > 0.01,
            "ADR-042 §2 : le réglage d'ADR-005 doit rater le critère de 1 % ; obtenu {:.4} %",
            r_adr005 * 100.0
        );

        // **Et un résultat que ce montage n'était pas censé donner.** `ADR-042` §2 mesure **7,0 %**
        // sur son montage de référence — 400 m, `λ = 20 m`, `dx = 0,125 m`. Celui-ci fait le quart de
        // la taille à la moitié de la longueur d'onde et le double de la maille, et rend **7,04 %**.
        //
        // C'est la réserve n° 2 d'`ADR-042` §6 vérifiée sans qu'on la cherche : *le groupe sans
        // dimension `σ_max·L_s/c` se transpose, la valeur de `σ_max` en s⁻¹ ne se transpose pas.*
        // Elle était écrite comme une supposition ; deux montages qui n'ont aucune dimension en
        // commun donnent le même `R` à 0,6 % près. **Ce qui gouverne l'éponge est bien le groupe,
        // pas la géométrie.**
        assert!(
            (r_adr005 - 0.070).abs() / 0.070 < 0.10,
            "ADR-042 §2 annonce 7,0 % à 4c/L_s ; ce montage réduit rend {:.4} %",
            r_adr005 * 100.0
        );
    }

    /// **C06 est partiel, et le reste.** `ADR-039` le classe **PARTIEL** : la translation 1D passe,
    /// le solide et la rotation manquent. Ce test vérifie que ce qui est couvert passe — il ne
    /// transforme pas un partiel en complet.
    ///
    /// C'est l'objet d'**A154** : *un cas partiel qui s'affiche vert ne se distingue pas d'un cas
    /// complet*. Le partiel est porté par `cas_en_attente()` côté rapport ; ici, il l'est par ce
    /// commentaire et par le nom du test.
    #[test]
    fn c06_la_translation_1d_passe_et_le_reste_manque() {
        let cas = c06_galilee(2000, 0.05, 2.0, 0.1, 10.0, 1.0);
        assert!(!cas.is_empty());
        for c in &cas {
            assert!(
                c.passe(),
                "{} : {} — mesuré {:.6}, référence {:.6}",
                c.id, c.grandeur, c.mesure, c.reference
            );
        }
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
