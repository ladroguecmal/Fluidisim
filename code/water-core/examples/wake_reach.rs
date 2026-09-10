//! S156 — où est l'énergie d'un sillage prolongé, et jusqu'où la quadrature la représente.
//!
//! L'énergie spectrale se conserve après extinction **par construction** : chaque mode tourne, et
//! la rotation laisse `g|eta|^2 + |v|^2/k` invariant. Cela ne dit rien de la validité spatiale du
//! champ. Ce que mesure cette sonde : le profil radial d'élévation efficace à plusieurs instants,
//! pour plusieurs résolutions de quadrature. Là où deux résolutions divergent, le champ n'est
//! plus représenté — et aucun oracle partageant la même discrétisation ne le dirait.
//!
//! `cargo run -p water-core --release --example wake_reach`

use water_core::{
    bound_pressure::{Prepared, Settings},
    gaussian_spectrum::{bake, Recipe},
    pressure_journal::Journal,
    pressure_source::Metadata,
    spectral_pressure::{Node, Slot, Surface},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

const FIN_US: u64 = 60_000_000;
const RAYONS: [f32; 13] = [
    2.0, 5.0, 10.0, 15.0, 20.0, 30.0, 45.0, 60.0, 80.0, 100.0, 130.0, 160.0, 200.0,
];
const DIRECTIONS: usize = 32;

/// Élévation efficace sur le cercle de rayon r, à l'instant donné.
fn profil(radial: usize, angular: usize, us: u64) -> Vec<f64> {
    let settings = Settings {
        frame: FrameId(7),
        cell: 9,
        gravity: 9.81,
        density: 1025.0,
        min: [-256.0; 2],
        max: [256.0; 2],
        start: SimTime(0),
        end: SimTime(FIN_US),
    };
    let recipe = Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial,
        angular,
    };
    let metadata = Metadata {
        epoch: 1,
        id: 7,
        cause: Cause {
            entity: 7,
            command: 1,
            emission: 0,
        },
        settings,
        recipe,
    };
    let legs = [Leg {
        duration_us: 2_000_000,
        velocity: [2.0, 0.0],
        downward_force_n: 100.0,
    }; 8];
    let wake = Wake::build(metadata, SimTime(0), [0.0; 2], &legs).unwrap();
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal.admit_authenticated(wake.source()).unwrap();

    let mut nodes = vec![Node::default(); radial * angular];
    let mut demi = vec![Node::default(); radial * angular / 2];
    let complet = bake(recipe, &mut nodes).unwrap();
    let moitie = complet.half_into(&mut demi).unwrap();
    let mut slots = vec![Slot::default(); moitie.nodes().len()];
    let context = wake.source().context();
    let time = SimTime(us);
    let p = Prepared::from_journal(context, &moitie, &journal, time, &mut slots).unwrap();

    let mut points = Vec::new();
    for r in RAYONS {
        for d in 0..DIRECTIONS {
            let a = core::f32::consts::TAU * d as f32 / DIRECTIONS as f32;
            points.push([r * a.cos(), r * a.sin()]);
        }
    }
    let mut sortie = vec![Surface::default(); points.len()];
    let mut scratch = sortie.clone();
    p.sample_batch(&context, time, &points, &mut scratch, &mut sortie)
        .unwrap();
    RAYONS
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let somme: f64 = (0..DIRECTIONS)
                .map(|d| {
                    let e = sortie[i * DIRECTIONS + d].eta as f64;
                    e * e
                })
                .sum();
            (somme / DIRECTIONS as f64).sqrt()
        })
        .collect()
}

/// Rayon au-dela duquel deux profils cessent de se ressembler, l'ecart etant rapporte au
/// **maximum du profil de reference** a cet instant et non a la valeur locale : rapporter a une
/// valeur qui s'annule fait exploser la mesure sans qu'aucune precision ne soit perdue (L232).
fn rayon_honnete(essai: &[f64], reference: &[f64], tolerance: f64) -> String {
    let echelle = reference.iter().cloned().fold(0.0f64, f64::max);
    let mut dernier = "aucun".to_string();
    for (i, r) in RAYONS.iter().enumerate() {
        if (essai[i] - reference[i]).abs() / echelle > tolerance {
            return dernier;
        }
        dernier = format!("{r} m");
    }
    format!("au-dela de {} m", RAYONS[RAYONS.len() - 1])
}

fn main() {
    lois_d_echelle();
    attribution();
    let resolutions = [(128usize, 128usize), (256, 256), (512, 512)];
    for us in [8_000_000u64, 30_000_000, FIN_US] {
        println!("# S156 — élévation efficace (m) sur le cercle, t = {} s", us / 1_000_000);
        print!("| rayon |");
        for (r, a) in resolutions {
            print!(" {r}x{a} |");
        }
        println!(" écart 256x256 / 512x512 |");
        print!("|---:|");
        for _ in resolutions {
            print!("---:|");
        }
        println!("---:|");
        let profils: Vec<Vec<f64>> = resolutions
            .iter()
            .map(|&(r, a)| profil(r, a, us))
            .collect();
        for (i, r) in RAYONS.iter().enumerate() {
            print!("| {r} m |");
            for prof in &profils {
                print!(" {:.4e} |", prof[i]);
            }
            let a = profils[1][i];
            let b = profils[2][i];
            let ecart = if b > 0.0 { (a - b).abs() / b } else { f64::NAN };
            println!(" {ecart:.3e} |");
        }
        println!(
            "Rayon honnete a 10 % du maximum : 128x128 {}, 256x256 {}.",
            rayon_honnete(&profils[0], &profils[2], 0.10),
            rayon_honnete(&profils[1], &profils[2], 0.10)
        );
        println!();
    }
}

/// Laquelle des deux resolutions borne la portee ? Le pas radial rend le champ periodique de
/// periode 2*pi*radial/cutoff ; le pas angulaire ne resout plus exp(i k.x) quand k*r*da depasse
/// un tour, soit r au-dela de angular/(2*k_max). Les deux se corrigent au meme prix par noeud,
/// mais pas au meme endroit, et le plafond de la grammaire de recette est 512 sur chaque axe.
fn attribution() {
    let essais = [(512usize, 128usize), (128, 512), (512, 512)];
    for us in [8_000_000u64, 30_000_000, FIN_US] {
        let profils: Vec<Vec<f64>> = essais.iter().map(|&(r, a)| profil(r, a, us)).collect();
        println!(
            "# S156 — t = {} s : periodicite radiale 2*pi*radial/cutoff, alias angulaire angular/(2*k_max)",
            us / 1_000_000
        );
        println!("| rayon | 512 radial, 128 angulaire | 128 radial, 512 angulaire | 512x512 |");
        println!("|---:|---:|---:|---:|");
        for (i, r) in RAYONS.iter().enumerate() {
            println!(
                "| {r} m | {:.4e} | {:.4e} | {:.4e} |",
                profils[0][i], profils[1][i], profils[2][i]
            );
        }
        println!(
            "Rayon honnete a 10 % : radial seul {}, angulaire seul {}.",
            rayon_honnete(&profils[0], &profils[2], 0.10),
            rayon_honnete(&profils[1], &profils[2], 0.10)
        );
        println!();
    }
}

/// Les deux bornes doivent suivre des lois differentes si l'attribution est juste.
/// Angulaire : rayon honnete proportionnel a `angular`. Radial : duree honnete en `sqrt(radial)`,
/// parce que le mode le plus lent a resoudre est le plus rapide a voyager — k_min vaut
/// `cutoff/(2*radial)` et sa vitesse de groupe `0,5*sqrt(g/k_min)` croit comme `sqrt(radial)`.
fn lois_d_echelle() {
    println!("# S156 — loi angulaire : rayon honnete a 10 %, radial fixe a 512, t = 8 s");
    let reference = profil(512, 512, 8_000_000);
    println!("| angulaire | rayon honnete | attendu si proportionnel |");
    println!("|---:|---:|---:|");
    for angular in [64usize, 128, 256] {
        let essai = profil(512, angular, 8_000_000);
        println!(
            "| {angular} | {} | {:.0} m |",
            rayon_honnete(&essai, &reference, 0.10),
            45.0 * angular as f64 / 128.0
        );
    }
    println!();
    println!("# S156 — loi radiale : a partir de quand deux resolutions cessent de s'accorder ?");
    println!("Angulaire fixe a 512, pour que seul le pas radial varie. Rayon d'accord a 10 %.");
    println!("| instant | 64 contre 128 | 128 contre 256 | 256 contre 512 |");
    println!("|---:|---:|---:|---:|");
    for us in [
        4_000_000u64, 6_000_000, 8_000_000, 10_000_000, 15_000_000, 20_000_000, 30_000_000,
        40_000_000, 45_000_000, 50_000_000, FIN_US,
    ] {
        let p64 = profil(64, 512, us);
        let p128 = profil(128, 512, us);
        let p256 = profil(256, 512, us);
        let p512 = profil(512, 512, us);
        println!(
            "| {} s | {} | {} | {} |",
            us / 1_000_000,
            rayon_honnete(&p64, &p128, 0.10),
            rayon_honnete(&p128, &p256, 0.10),
            rayon_honnete(&p256, &p512, 0.10)
        );
    }
    println!(
        "Duree predite par la recurrence 2*pi/(c_g,max * dk) : {:.1} / {:.1} / {:.1} s pour 64 / 128 / 256.",
        duree_honnete(64),
        duree_honnete(128),
        duree_honnete(256)
    );
    println!();
}

/// Duree au bout de laquelle le mode le plus rapide a parcouru la periode spatiale `2*pi/dk`.
fn duree_honnete(radial: usize) -> f64 {
    let cutoff = 6.0f64;
    let dk = cutoff / radial as f64;
    let k_min = 0.5 * dk;
    let vitesse = 0.5 * (9.81f64 / k_min).sqrt();
    core::f64::consts::TAU / (dk * vitesse)
}
