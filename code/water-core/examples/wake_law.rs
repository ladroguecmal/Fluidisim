//! S157 — la loi en durée d'un sillage, au lieu de deux encadrements (S156-1, A214).
//!
//! S156 a établi le mécanisme — le pas radial rend le champ périodique, de période
//! `2*pi*radial/cutoff` — et mesuré deux seuils seulement, avec un critère qui n'était pas
//! monotone en temps. Ici : un écart global entre deux résolutions voisines, sa monotonie
//! vérifiée avant toute dichotomie, puis trois seuils et la dépendance à `sigma`.
//!
//! `cargo run -p water-core --release --example wake_law`

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

const FIN_US: u64 = 64_000_000;
const DIRECTIONS: usize = 32;

/// Rayons d'échantillonnage, mis à l'échelle de la source : une source large fait tout plus grand.
fn rayons(sigma: f32) -> Vec<f32> {
    (1..=25).map(|i| i as f32 * 8.0 * sigma).collect()
}

/// Élévation efficace sur des cercles concentriques, à l'instant donné.
fn profil(radial: usize, angular: usize, sigma: f32, cutoff: f32, us: u64) -> Vec<f64> {
    let bord = 8.0 * 25.0 * sigma + 16.0;
    let settings = Settings {
        frame: FrameId(7),
        cell: 9,
        gravity: 9.81,
        density: 1025.0,
        min: [-bord; 2],
        max: [bord; 2],
        start: SimTime(0),
        end: SimTime(FIN_US),
    };
    let recipe = Recipe {
        sigma,
        cutoff,
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

    let liste = rayons(sigma);
    let mut points = Vec::new();
    for r in &liste {
        for d in 0..DIRECTIONS {
            let a = core::f32::consts::TAU * d as f32 / DIRECTIONS as f32;
            points.push([r * a.cos(), r * a.sin()]);
        }
    }
    let mut sortie = vec![Surface::default(); points.len()];
    let mut scratch = sortie.clone();
    p.sample_batch(&context, time, &points, &mut scratch, &mut sortie)
        .unwrap();
    (0..liste.len())
        .map(|i| {
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

/// Écart **global** entre une résolution et sa voisine deux fois plus fine : norme quadratique de
/// la différence des profils, rapportée au maximum du profil fin. Un seul nombre par instant, et
/// aucun rayon particulier ne peut le faire sauter — ce que le critère de S156 ne garantissait pas.
fn ecart(radial: usize, sigma: f32, cutoff: f32, us: u64) -> f64 {
    ecart_contre(radial, 512, sigma, cutoff, us)
}

/// Comparer chaque résolution à **la même** référence, et non à sa voisine : un écart entre R et
/// 2R saute aussi quand c'est 2R qui défaille, et on attribue alors la panne à la mauvaise.
fn ecart_contre(radial: usize, reference: usize, sigma: f32, cutoff: f32, us: u64) -> f64 {
    let grossier = profil(radial, 512, sigma, cutoff, us);
    let fin = profil(reference, 512, sigma, cutoff, us);
    let echelle = fin.iter().cloned().fold(0.0f64, f64::max);
    let carre: f64 = grossier
        .iter()
        .zip(&fin)
        .map(|(a, b)| (a - b) * (a - b))
        .sum();
    (carre / fin.len() as f64).sqrt() / echelle
}

fn main() {
    monotonie();
    retour_proche();
}

/// Observable propre au mécanisme, et non à la fenêtre d'échantillonnage : la périodicité fait
/// **revenir** au centre l'énergie qui aurait dû partir. On compare donc l'élévation efficace en
/// champ proche — les cinq premiers cercles — à celle de la référence, en **rapport** et non en
/// écart relatif : c'est un excès, il vaut 1 tant que rien n'est revenu.
///
/// L'écart global de `monotonie` mêle tous les rayons ; quand le paquet quitte la fenêtre, il
/// bouge pour une raison qui n'est pas une perte de résolution. Celui-ci ne bouge que si du
/// signal apparaît là où il ne devrait plus y en avoir.
fn retour_proche() {
    println!();
    println!("# S157 — excès d'élévation en champ proche, rapporté à la référence radial 512");
    println!("Cinq premiers cercles (8 à 40 m pour sigma 1 m). Vaut 1 tant que rien n'est revenu.");
    println!("| instant | radial 64 | radial 128 | radial 256 |");
    println!("|---:|---:|---:|---:|");
    for us in (1..=16).map(|i| i * 4_000_000u64) {
        let reference = proche(512, 1.0, 6.0, us);
        print!("| {} s |", us / 1_000_000);
        for radial in [64usize, 128, 256] {
            print!(" {:.3} |", proche(radial, 1.0, 6.0, us) / reference);
        }
        println!();
    }
}

/// Élévation efficace sur les cinq premiers cercles seulement.
fn proche(radial: usize, sigma: f32, cutoff: f32, us: u64) -> f64 {
    let p = profil(radial, 512, sigma, cutoff, us);
    let carre: f64 = p[..5].iter().map(|v| v * v).sum();
    (carre / 5.0).sqrt()
}

/// Avant toute dichotomie : l'écart croît-il vraiment avec le temps ? Une dichotomie posée sur une
/// quantité non monotone rend un chiffre faux avec l'apparence d'un chiffre précis.
fn monotonie() {
    println!("# S157 — l'écart global croît-il avec le temps ? sigma 1 m, cutoff 6, angulaire 512");
    println!("Chaque colonne compare la résolution indiquée à la référence radial 512.");
    println!("| instant | radial 64 | radial 128 | radial 256 |");
    println!("|---:|---:|---:|---:|");
    let instants: Vec<u64> = (1..=16).map(|i| i * 4_000_000).collect();
    let mut colonnes = vec![Vec::new(); 3];
    for &us in &instants {
        let mut ligne = Vec::new();
        for (c, radial) in [64usize, 128, 256].into_iter().enumerate() {
            let d = ecart(radial, 1.0, 6.0, us);
            colonnes[c].push(d);
            ligne.push(d);
        }
        println!(
            "| {} s | {:.4e} | {:.4e} | {:.4e} |",
            us / 1_000_000,
            ligne[0],
            ligne[1],
            ligne[2]
        );
    }
    for (c, radial) in [64usize, 128, 256].into_iter().enumerate() {
        let mut reculs = 0;
        for paire in colonnes[c].windows(2) {
            if paire[1] < paire[0] {
                reculs += 1;
            }
        }
        println!(
            "radial {radial} : {reculs} reculs sur {} intervalles, maximum {:.4e}",
            colonnes[c].len() - 1,
            colonnes[c].iter().cloned().fold(0.0f64, f64::max)
        );
    }
}
