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
///
/// La table est calculée **une fois** : la référence 512x512 coûte quatre secondes par instant,
/// et la recalculer pour chaque seuil rendait la sonde inutilisable.
fn table(sigma: f32, cutoff: f32, pas_us: u64) -> (Vec<u64>, Vec<Vec<f64>>) {
    let mut instants = Vec::new();
    let mut lignes = vec![Vec::new(); 3];
    let mut us = 4_000_000u64;
    while us <= FIN_US {
        let reference = proche(512, sigma, cutoff, us);
        for (c, radial) in [64usize, 128, 256].into_iter().enumerate() {
            lignes[c].push(proche(radial, sigma, cutoff, us) / reference);
        }
        instants.push(us);
        us += pas_us;
    }
    (instants, lignes)
}

fn retour_proche() {
    println!();
    println!("# S157 — excès d'élévation en champ proche, rapporté à la référence radial 512");
    println!("Cinq premiers cercles (8 à 40 m pour sigma 1 m). Vaut 1 tant que rien n'est revenu.");
    let (instants, lignes) = table(1.0, 6.0, 2_000_000);
    println!("| instant | radial 64 | radial 128 | radial 256 |");
    println!("|---:|---:|---:|---:|");
    for (i, us) in instants.iter().enumerate() {
        println!(
            "| {} s | {:.3} | {:.3} | {:.3} |",
            us / 1_000_000,
            lignes[0][i],
            lignes[1][i],
            lignes[2][i]
        );
    }
    seuils(&instants, &lignes);
}

/// Premier instant où l'excès franchit un seuil, lu dans la table : la quantité croît sans être
/// strictement monotone, et une bissection y rendrait un chiffre faux d'apparence précise.
fn seuils(instants: &[u64], lignes: &[Vec<f64>]) {
    println!();
    println!("# S157 — premier franchissement, sigma 1 m, cutoff 6, grille de 2 s");
    println!("| seuil | radial 64 | radial 128 | radial 256 | rapport 64→128 | 128→256 | exposant |");
    println!("|---:|---:|---:|---:|---:|---:|---:|");
    for seuil in [1.10f64, 1.25, 1.50, 2.00] {
        let t: Vec<Option<f64>> = (0..3)
            .map(|c| {
                lisse(&lignes[c])
                    .iter()
                    .position(|v| *v > seuil)
                    .map(|i| instants[i] as f64 / 1e6)
            })
            .collect();
        let mot = |v: Option<f64>| match v {
            Some(s) => format!("{s:.0} s"),
            None => "au-delà de 64 s".into(),
        };
        let rapport = |a: Option<f64>, b: Option<f64>| match (a, b) {
            (Some(x), Some(y)) => format!("{:.2}", y / x),
            _ => "—".into(),
        };
        let exposant = match (t[0], t[2]) {
            (Some(x), Some(y)) => format!("{:.2}", (y / x).log2() / 2.0),
            _ => "—".into(),
        };
        println!(
            "| {seuil:.2} | {} | {} | {} | {} | {} | {exposant} |",
            mot(t[0]),
            mot(t[1]),
            mot(t[2]),
            rapport(t[0], t[1]),
            rapport(t[1], t[2])
        );
    }
    println!("Exposant 0,50 = racine de radial ; 1,00 = proportionnel à radial.");
}

/// Élévation efficace sur les cinq premiers cercles seulement.
/// Médiane glissante à trois points puis maximum courant. Le rapport oscille — la récurrence est
/// un battement — et un franchissement lu sur une pointe isolée n'est pas un franchissement. Le
/// maximum courant rend la courbe monotone par construction, donc l'instant de franchissement
/// bien défini ; la médiane l'empêche d'être déclenché par un seul point.
fn lisse(v: &[f64]) -> Vec<f64> {
    let mut median = v.to_vec();
    for i in 1..v.len() - 1 {
        let mut trois = [v[i - 1], v[i], v[i + 1]];
        trois.sort_by(|a, b| a.partial_cmp(b).unwrap());
        median[i] = trois[1];
    }
    let mut courant = 0.0f64;
    median
        .iter()
        .map(|x| {
            courant = courant.max(*x);
            courant
        })
        .collect()
}

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
