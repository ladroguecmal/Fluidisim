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
