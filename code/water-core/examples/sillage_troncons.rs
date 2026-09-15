//! S242 — coût par image de la préparation du sillage **contre le nombre de tronçons**.
//!
//! On ne pose aucune horloge dans `render_components` : on ferait bouger ce qu'on mesure. On fait
//! varier le paramètre — un, deux, trois sillages dans le même journal, soit 8, 16 et 24 segments,
//! à nombre de nœuds **inchangé** (4 096). La pente contre le nombre de segments *est* la part de la
//! boucle interne. Voir `docs/validation/PREPARATION-SILLAGE-S242.md`.
//!
//! Fixture de S212/S235 inchangée : σ 2 m, coupure 3 rad/m, huit tronçons de 2 s à [3, 0] m/s sous
//! 19 620 N, départs (−24, y) avec y ∈ {4, −26, 34}, contexte 40 s, caméra S201 (0, −18).
//!
//! `cargo run -p water-core --release --example sillage_troncons`
use std::{hint::black_box, time::Instant};
use water_core::{
    bound_pressure::Settings,
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::ModalPressure,
    pressure_journal::Journal,
    pressure_source::Metadata,
    pressure_timeline::{NodeState, Timeline},
    spectral_pressure::Node,
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

const BIRTH: u64 = 12_000_000;
const SPAN: u64 = 40_000_000;
const ORIGIN: [f32; 2] = [0.0, -18.0];
const OFFSETS: [f32; 3] = [4.0, -26.0, 34.0];
const LEG_US: u64 = 2_000_000;
const LEGS: u64 = 8;

fn recipe() -> Recipe {
    Recipe { sigma: 2.0, cutoff: 3.0, radial: 64, angular: 128 }
}

fn wake(index: usize) -> Wake {
    let settings = Settings {
        frame: FrameId(0),
        cell: 0,
        gravity: 9.81,
        density: 1025.0,
        min: [-64.0, -48.0],
        max: [64.0, 56.0],
        start: SimTime(BIRTH),
        end: SimTime(BIRTH + SPAN),
    };
    let id = 212 + index as u64;
    let metadata = Metadata {
        epoch: 1,
        id,
        cause: Cause { entity: id, command: 1, emission: 0 },
        settings,
        recipe: recipe(),
    };
    let legs = [Leg { duration_us: LEG_US, velocity: [3.0, 0.0], downward_force_n: 19_620.0 }; 8];
    Wake::build(metadata, SimTime(BIRTH), [-24.0, OFFSETS[index]], &legs).unwrap()
}

/// Instant de l'image `i` à 60 images/s depuis l'âge `age0` (s).
fn frame(age0: f64, i: u64) -> SimTime {
    SimTime(BIRTH + (age0 * 1e6) as u64 + i * 1_000_000 / 60)
}

fn stats(mut v: Vec<f64>) -> (f64, f64, f64) {
    v.sort_by(f64::total_cmp);
    let q = |p: f64| v[((v.len() - 1) as f64 * p).round() as usize];
    (q(0.5), q(0.95), v[v.len() - 1])
}

/// Empreinte FNV-1a 64 bits des coefficients publiés : l'identité au bit, sans les imprimer.
fn fingerprint(out: &[[f32; 4]]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for row in out {
        for value in row {
            for byte in value.to_bits().to_le_bytes() {
                h = (h ^ byte as u64).wrapping_mul(0x100_0000_01b3);
            }
        }
    }
    h
}

/// Segments en forçage à cet instant, selon la seule géométrie des tronçons.
fn active(time: SimTime, wakes: usize) -> usize {
    (0..LEGS)
        .filter(|j| {
            let birth = BIRTH + j * LEG_US;
            birth < time.0 && time.0 < birth + LEG_US
        })
        .count()
        * wakes
}

fn main() {
    println!("# S242 — préparation du sillage contre le nombre de tronçons");
    println!();
    println!("Techniques présentes : repli des tronçons achevés, modes préconstruits, publication");
    println!("rebasée à la caméra. Techniques absentes : parallélisme (un fil), SIMD explicite, LOD");
    println!("temporel, mutualisation entre journaux.");
    println!("Domaine : fixture S212/S235, recette 64×128 (4 096 nœuds), 60 images/s simulées,");
    println!("release, un fil, machine locale ; ni GPU, ni transfert, ni LOD spatial.");
    println!();
    println!("| sillages | segments | fenêtre | actifs | images | médiane (ms) | p95 (ms) | max (ms) | empreinte |");
    println!("|---:|---:|---|---:|---:|---:|---:|---:|---|");
    for wakes in 1..=3usize {
        measure(wakes);
    }
}

fn measure(wakes: usize) {
    let r = recipe();
    let sources: Vec<Wake> = (0..wakes).map(wake).collect();
    let mut records = [None; 3];
    let mut journal = Journal::new(1, &mut records[..wakes]);
    for w in &sources {
        journal.admit_authenticated(w.source()).unwrap();
    }
    let context = sources[0].source().context();
    let mut full = vec![Node::default(); 64 * 128];
    let mut half = vec![Node::default(); 64 * 128 / 2];
    let spectrum = bake(r, &mut full).unwrap().half_into(&mut half).unwrap();
    let count = spectrum.nodes().len();
    let capacity = Timeline::mode_capacity(&spectrum, &journal);
    let mut nodes = vec![NodeState::default(); count];
    let mut modes: Vec<Option<ModalPressure>> = vec![None; capacity];
    let mut out = vec![[0.0f32; 4]; count];
    let mut timeline =
        Timeline::build(context, &spectrum, &journal, &mut nodes, &mut modes).unwrap();
    let segments = capacity / count;

    // Chauffe séparée (A195).
    for i in 0..60 {
        timeline.render_components(&context, frame(1.0, i), ORIGIN, &mut out).unwrap();
    }
    for (name, age0) in [("forçage", 3.0), ("après forçage", 24.0)] {
        let mut ms = Vec::new();
        timeline.render_components(&context, frame(age0, 9), ORIGIN, &mut out).unwrap();
        let mut hash = 0u64;
        for i in 10..130u64 {
            let t = frame(age0, i);
            let start = Instant::now();
            timeline.render_components(&context, t, ORIGIN, &mut out).unwrap();
            ms.push(start.elapsed().as_secs_f64() * 1e3);
            black_box(&out);
            hash ^= fingerprint(&out).rotate_left((i % 64) as u32);
        }
        let n = ms.len();
        let (m, p, x) = stats(ms);
        let a = active(frame(age0, 70), wakes);
        println!(
            "| {wakes} | {segments} | {name} | {a} | {n} | {m:.4} | {p:.4} | {x:.4} | `{hash:#018x}` |"
        );
    }
}
