//! S155 — la fenêtre de 16 s d'ADR-071 est-elle une limite numérique ou un périmètre déclaré ?
//!
//! Compare le noyau modal f32/Q32 à l'oracle f64 `PressureMode` pour des âges très au-delà de
//! 16 s, puis pour des durées actives très au-delà de 16 s. Les deux ne suivent pas le même
//! chemin : l'âge après extinction n'entre que dans une rotation entière, la durée active entre
//! dans une accumulation f32.
//!
//! Convention d'oracle, celle de S95 : le même f32 de g est converti en f64. On mesure l'erreur
//! d'implémentation, pas la quantification de l'entrée.
//!
//! **Cette sonde mesure au-delà de l'horizon en vigueur.** Tant que `ModalPressure::new` refuse
//! un horizon supérieur à celui d'ADR-071, les lignes concernées affichent `couples = 0` : rien
//! n'a été comparé, et ce n'est donc pas un écart nul. La sonde annonce d'abord l'horizon
//! réellement accepté, pour qu'on ne lise jamais un tableau sans savoir ce qu'il a pu mesurer.
//!
//! `cargo run -p water-core --release --example horizon_modal`

use water_core::modal_pressure::{ModalPressure, Response, Segment};
use water_core::pressure_mode::{PressureMode, PressureSegment};
use water_core::SimTime;

const K: [[f32; 2]; 5] = [
    [0.0234375, 0.0],
    [1.0, 0.0],
    [0.6, 0.8],
    [6.0, 0.0],
    [9.0, 0.0],
];
const RATIOS: [f32; 6] = [-2.0, -1.0, -0.999999, 0.0, 0.5, 2.0];

fn reference(s: Segment) -> PressureSegment {
    PressureSegment {
        birth: s.birth,
        duration_us: s.duration_us,
        origin: s.origin.map(f64::from),
        velocity: s.velocity.map(f64::from),
        pressure_pa: s.pressure_pa as f64,
    }
}

fn parts(r: Response) -> [f32; 4] {
    [r.eta.re, r.eta.im, r.velocity.re, r.velocity.im]
}

/// Écart maximal (élévation, vitesse), amplitude d'élévation de l'oracle, et **nombre de
/// couples effectivement comparés** : un écart nul sans comparaison est un artefact, pas une
/// mesure, et rien ne le distingue d'un excellent résultat si on ne le compte pas.
fn ecart(age_us: u64, duration_us: u64, horizon_us: u64) -> (f64, f64, f64, u32) {
    let mut worst = [0.0f64; 2];
    let mut amplitude = 0.0f64;
    let mut compares = 0u32;
    for k in K {
        let omega = (9.81f32 * (k[0] * k[0] + k[1] * k[1]).sqrt()).sqrt();
        let oracle = PressureMode::new(k.map(f64::from), 9.81f32 as f64, 1025.0).unwrap();
        for ratio in RATIOS {
            let s = Segment {
                birth: SimTime(0),
                duration_us,
                origin: [0.7, -0.3],
                velocity: [ratio * omega / k[0], 0.0],
                pressure_pa: 10.0,
            };
            // Origine et extrémité restent dans ±4096 m : borner la vitesse par la durée.
            if (s.velocity[0] as f64).abs() * duration_us as f64 / 1e6 >= 4000.0 {
                continue;
            }
            let Ok(m) = ModalPressure::new(k, 9.81, 1025.0, s, horizon_us) else {
                continue;
            };
            let candidat = parts(m.sample(SimTime(age_us)).unwrap());
            let q = oracle.sample(reference(s), SimTime(age_us)).unwrap();
            let attendu = [q.eta.re, q.eta.im, q.velocity.re, q.velocity.im];
            for i in 0..4 {
                worst[i / 2] = worst[i / 2].max((candidat[i] as f64 - attendu[i]).abs());
            }
            amplitude = amplitude.max((q.eta.re * q.eta.re + q.eta.im * q.eta.im).sqrt());
            compares += 1;
        }
    }
    (worst[0], worst[1], amplitude, compares)
}

/// Le plus grand horizon que le constructeur accepte aujourd'hui, par essais croissants.
fn horizon_accepte() -> u64 {
    let s = Segment {
        birth: SimTime(0),
        duration_us: 1_000_000,
        origin: [0.7, -0.3],
        velocity: [0.0, 0.0],
        pressure_pa: 10.0,
    };
    let mut best = 0;
    for essai in [1_000_000u64, 16_000_000, 32_000_000, 64_000_000, 128_000_000] {
        if ModalPressure::new([1.0, 0.0], 9.81, 1025.0, s, essai).is_ok() {
            best = essai;
        }
    }
    best
}

fn main() {
    let accepte = horizon_accepte();
    println!(
        "Horizon accepté par le constructeur : {} s. Toute ligne d'âge supérieur affiche 0 couple.",
        accepte / 1_000_000
    );
    println!();
    println!("# S155 — âge croissant, durée active fixée à 4 s (celle de S95)");
    println!("Seuils de régression S95 : 2e-7 m, 2e-6 m/s.");
    println!("| âge | écart élévation (m) | écart vitesse (m/s) | |eta| oracle (m) | relatif | couples |");
    println!("|---:|---:|---:|---:|---:|---:|");
    for secondes in [1u64, 4, 8, 16, 24, 32, 48, 60, 64] {
        let age = secondes * 1_000_000;
        let (eta, v, a, n) = ecart(age, 4_000_000, age.max(16_000_000));
        println!("| {secondes} s | {eta:.6e} | {v:.6e} | {a:.6e} | {:.3e} | {n} |", eta / a);
    }

    println!();
    println!("# S155 — durée active croissante, mesurée à l'extinction (âge = durée)");
    println!("| durée active | écart élévation (m) | écart vitesse (m/s) | |eta| oracle (m) | relatif | couples |");
    println!("|---:|---:|---:|---:|---:|---:|");
    for secondes in [1u64, 4, 8, 16, 24, 32, 48, 60, 64] {
        let duree = secondes * 1_000_000;
        let (eta, v, a, n) = ecart(duree, duree, duree.max(16_000_000));
        println!("| {secondes} s | {eta:.6e} | {v:.6e} | {a:.6e} | {:.3e} | {n} |", eta / a);
    }

    println!();
    println!("# S155 — durée active croissante, observée tard (âge = durée + 4 s)");
    println!("| durée active | écart élévation (m) | écart vitesse (m/s) | |eta| oracle (m) | relatif | couples |");
    println!("|---:|---:|---:|---:|---:|---:|");
    for secondes in [1u64, 4, 8, 16, 24, 32, 48, 56, 60] {
        let duree = secondes * 1_000_000;
        let age = duree + 4_000_000;
        let (eta, v, a, n) = ecart(age, duree, age.max(16_000_000));
        println!("| {secondes} s | {eta:.6e} | {v:.6e} | {a:.6e} | {:.3e} | {n} |", eta / a);
    }
}
