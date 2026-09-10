//! S156 — bilan énergétique d'un sillage prolongé, jusqu'à 60 s.
//!
//! Ce que ADR-106 vient d'ouvrir : personne n'avait observé un sillage au-delà de 8 s (S150).
//! Forçage de 16 s en huit tronçons de 2 s à 2 m/s, puis propagation libre jusqu'à 60 s.
//!
//! `cargo run -p water-core --release --example wake_long`

use water_core::{
    bound_pressure::{Prepared, Settings},
    gaussian_spectrum::{bake, Recipe},
    pressure_journal::Journal,
    pressure_source::Metadata,
    spectral_pressure::{Node, Slot},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

const FORCAGE_US: u64 = 16_000_000;
const FIN_US: u64 = 60_000_000;

fn main() {
    for (radial, angular) in [(128usize, 128usize), (256, 256)] {
        bilan(radial, angular);
        println!();
    }
    cout();
    enveloppe();
}

/// S158 — ce que consomment les consommateurs qui lisent une **borne** et non un échantillon.
/// La récurrence rephase les modes ; elle ne change pas l'amplitude des coefficients. Si c'est
/// vrai, l'enveloppe de pente et l'énergie ne bougent pas d'une résolution à l'autre ni dans le
/// temps, alors que le champ échantillonné, lui, se trompe d'un facteur cinquante.
fn enveloppe() {
    println!();
    println!("# S158 — enveloppe de pente et énergie, les deux grandeurs bornées");
    println!("| recette | instant | enveloppe de pente | énergie (J) |");
    println!("|---|---:|---:|---:|");
    for (radial, angular) in [(128usize, 512usize), (512, 512)] {
        for us in [8_000_000u64, 60_000_000] {
            let (enveloppe, energie) = bornes(radial, angular, us);
            println!(
                "| {radial}x{angular} | {} s | {enveloppe:.9e} | {energie:.9e} |",
                us / 1_000_000
            );
        }
    }
}

fn bornes(radial: usize, angular: usize, us: u64) -> (f32, f32) {
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
    let p = Prepared::from_journal(context, &moitie, &journal, SimTime(us), &mut slots).unwrap();
    (p.slope_envelope(), p.energy_j())
}

/// Prix des résolutions que S156 montre nécessaires à 60 s. Médiane sur cent répétitions, après
/// un bloc de chauffe séparé — mesurer avant la chauffe fait mentir la médiane (A195).
fn cout() {
    println!("# S156 — prix d'une préparation et d'un lot de 64 points");
    println!("| recette | nœuds du demi-spectre | préparation p50 | lot 64 points p50 |");
    println!("|---|---:|---:|---:|");
    for (radial, angular) in [(128usize, 128usize), (256, 256), (512, 256), (512, 512)] {
        let (nodes, preparation, lot) = mesure_cout(radial, angular);
        println!("| {radial}x{angular} | {nodes} | {preparation} µs | {lot} µs |");
    }
}

fn mesure_cout(radial: usize, angular: usize) -> (usize, u128, u128) {
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
    let mut nodes = vec![water_core::spectral_pressure::Node::default(); radial * angular];
    let mut demi = vec![water_core::spectral_pressure::Node::default(); radial * angular / 2];
    let complet = bake(recipe, &mut nodes).unwrap();
    let moitie = complet.half_into(&mut demi).unwrap();
    let mut slots = vec![Slot::default(); moitie.nodes().len()];
    let context = wake.source().context();
    let time = SimTime(FIN_US);
    let points: Vec<[f32; 2]> = (0..64)
        .map(|i| [i as f32 * 0.5 - 16.0, i as f32 * 0.25 - 8.0])
        .collect();
    let mut sortie = vec![water_core::spectral_pressure::Surface::default(); points.len()];
    let mut scratch = sortie.clone();

    // Chauffe, sans aucune mesure retenue.
    for _ in 0..20 {
        let p = Prepared::from_journal(context, &moitie, &journal, time, &mut slots).unwrap();
        p.sample_batch(&context, time, &points, &mut scratch, &mut sortie)
            .unwrap();
    }

    let mut preparations = Vec::new();
    let mut lots = Vec::new();
    for _ in 0..100 {
        let debut = std::time::Instant::now();
        let p = Prepared::from_journal(context, &moitie, &journal, time, &mut slots).unwrap();
        preparations.push(debut.elapsed().as_micros());
        let debut = std::time::Instant::now();
        p.sample_batch(&context, time, &points, &mut scratch, &mut sortie)
            .unwrap();
        lots.push(debut.elapsed().as_micros());
    }
    preparations.sort_unstable();
    lots.sort_unstable();
    (moitie.nodes().len(), preparations[50], lots[50])
}

fn bilan(radial: usize, angular: usize) {
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
    // Huit tronçons de 2 s à 2 m/s : 16 s de forçage, 32 m parcourus.
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

    println!("# S156 — sillage 16 s de forçage, radial {radial} / angulaire {angular}");
    println!("| instant | énergie (J) | puissance (W) | énergie/référence |");
    println!("|---:|---:|---:|---:|");
    let mut reference = 0.0f32;
    for us in [
        0u64,
        1_000_000,
        4_000_000,
        8_000_000,
        15_999_999,
        FORCAGE_US,
        16_000_001,
        20_000_000,
        30_000_000,
        45_000_000,
        FIN_US,
    ] {
        let time = SimTime(us);
        let p = Prepared::from_journal(context, &moitie, &journal, time, &mut slots).unwrap();
        let energie = p.energy_j();
        if us == FORCAGE_US {
            reference = energie;
        }
        let rapport = if reference != 0.0 {
            format!("{:.9}", energie / reference)
        } else {
            "—".into()
        };
        println!(
            "| {:.6} s | {energie:.9e} | {:.6e} | {rapport} |",
            us as f64 / 1e6,
            p.power_w()
        );
    }
}
