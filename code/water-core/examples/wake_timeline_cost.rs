//! S213 — coût par image du **levier temporel** du sillage, seul, sur CPU (ADR-131).
//!
//! Fixture de S212 inchangée (viewer/src/scene.rs) : σ 2 m, coupure 3 rad/m, huit tronçons de 2 s
//! à [3, 0] m/s sous 19 620 N, départ (−24, 4) m, contexte 40 s. Même publication que l'hôte :
//! coefficients `[A, B, kx, ky]` rebasés à la caméra S201 (0, −18).
//!
//! `cargo run -p water-core --release --example wake_timeline_cost`
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::{hint::black_box, mem::size_of, time::Instant};
use water_core::{
    bound_pressure::{Prepared, Settings},
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::ModalPressure,
    pressure_journal::Journal,
    pressure_source::Metadata,
    pressure_timeline::{NodeState, Timeline},
    spectral_pressure::{Node, Slot},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

const BIRTH: u64 = 12_000_000;
const SPAN: u64 = 40_000_000;
const ORIGIN: [f32; 2] = [0.0, -18.0];

fn recipe(radial: usize, angular: usize) -> Recipe {
    Recipe { sigma: 2.0, cutoff: 3.0, radial, angular }
}
fn wake(recipe: Recipe) -> Wake {
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
    let metadata = Metadata {
        epoch: 1,
        id: 212,
        cause: Cause { entity: 212, command: 1, emission: 0 },
        settings,
        recipe,
    };
    let legs = [Leg { duration_us: 2_000_000, velocity: [3.0, 0.0], downward_force_n: 19_620.0 }; 8];
    Wake::build(metadata, SimTime(BIRTH), [-24.0, 4.0], &legs).unwrap()
}
/// Instant de l'image `i` à 60 images/s depuis l'âge `age0` (s), en entiers de µs.
fn frame(age0: f64, i: u64) -> SimTime {
    SimTime(BIRTH + (age0 * 1e6) as u64 + i * 1_000_000 / 60)
}
fn stats(mut v: Vec<f64>) -> (f64, f64, f64) {
    v.sort_by(f64::total_cmp);
    let q = |p: f64| v[((v.len() - 1) as f64 * p).round() as usize];
    (q(0.5), q(0.95), v[v.len() - 1])
}

fn main() {
    println!("# S213 — levier temporel du sillage, coût CPU par image");
    println!();
    println!("Techniques présentes : repli des tronçons achevés par nœud (rotation entière), modes");
    println!("préconstruits, tronçon en cours par `ModalPressure::sample`, publication rebasée.");
    println!("Techniques absentes : grille et transformée, LOD spatial, LOD spectral, LOD temporel,");
    println!("visibilité, mutualisation, parallélisme (un fil), SIMD explicite.");
    println!("Domaine : fixture S212, une source, 8 tronçons, origine caméra S201, 60 images/s simulées,");
    println!("release, un fil, machine locale ; GPU exclu ; aucun transfert.");
    for (radial, angular) in [(64usize, 128usize), (128, 256)] {
        measure(radial, angular);
    }
}

fn measure(radial: usize, angular: usize) {
    let r = recipe(radial, angular);
    let wake = wake(r);
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal.admit_authenticated(wake.source()).unwrap();
    let context = wake.source().context();
    let mut full = vec![Node::default(); radial * angular];
    let mut half = vec![Node::default(); radial * angular / 2];
    let spectrum = bake(r, &mut full).unwrap().half_into(&mut half).unwrap();
    let count = spectrum.nodes().len();
    let capacity = Timeline::mode_capacity(&spectrum, &journal);
    let mut nodes = vec![NodeState::default(); count];
    let mut modes: Vec<Option<ModalPressure>> = vec![None; capacity];
    let mut out = vec![[0.0f32; 4]; count];
    let mut slots = vec![Slot::default(); count];

    // Construction : médiane de cinq, après une construction de chauffe.
    let mut builds = Vec::new();
    for rep in 0..6 {
        let start = Instant::now();
        let t = Timeline::build(context, &spectrum, &journal, &mut nodes, &mut modes).unwrap();
        black_box(t.component_count());
        if rep > 0 {
            builds.push(start.elapsed().as_secs_f64() * 1e3);
        }
    }
    let memory = count * size_of::<NodeState>() + capacity * size_of::<Option<ModalPressure>>();
    let mut timeline = Timeline::build(context, &spectrum, &journal, &mut nodes, &mut modes).unwrap();

    println!();
    println!("## Recette {radial}×{angular} — {count} nœuds, {capacity} modes");
    println!();
    println!(
        "Construction médiane {:.3} ms ; mémoire hôte {} octets ({} par nœud, {} par mode).",
        stats(builds).0,
        memory,
        size_of::<NodeState>(),
        size_of::<Option<ModalPressure>>()
    );
    println!();
    println!("| scénario | chemin | images | médiane (ms) | p95 (ms) | max (ms) |");
    println!("|---|---|---:|---:|---:|---:|");

    // Chauffe séparée (A195).
    for i in 0..60 {
        timeline.render_components(&context, frame(1.0, i), ORIGIN, &mut out, &host_impl::SequentialJobs).unwrap();
    }
    for (name, age0) in [("fenêtre S212, 3,17–5,15 s, forçage", 3.0), ("après forçage, 24,17–26,15 s", 24.0)] {
        // Même fenêtre que le banc S212 : images 10 à 129.
        let mut lever = Vec::new();
        let mut prepared = Vec::new();
        timeline.render_components(&context, frame(age0, 9), ORIGIN, &mut out, &host_impl::SequentialJobs).unwrap();
        for i in 10..130 {
            let t = frame(age0, i);
            let start = Instant::now();
            timeline.render_components(&context, t, ORIGIN, &mut out, &host_impl::SequentialJobs).unwrap();
            lever.push(start.elapsed().as_secs_f64() * 1e3);
            black_box(&out);
            let start = Instant::now();
            Prepared::from_journal(context, &spectrum, &journal, t, &mut slots)
                .unwrap()
                .render_components(&context, t, ORIGIN, &mut out)
                .unwrap();
            prepared.push(start.elapsed().as_secs_f64() * 1e3);
        }
        for (path, v) in [("repli temporel", lever), ("préparation S212", prepared)] {
            let (m, p, x) = stats(v);
            println!("| {name} | {path} | 120 | {m:.4} | {p:.4} | {x:.4} |");
        }
    }

    // Balayage complet à 60 images/s : images qui franchissent une fin de tronçon séparées.
    let mut ordinary = Vec::new();
    let mut crossing = Vec::new();
    let mut last = frame(0.0, 0);
    timeline.render_components(&context, last, ORIGIN, &mut out, &host_impl::SequentialJobs).unwrap();
    for i in 1..=2400u64 {
        let t = frame(0.0, i);
        let crosses = (1..=8u64).any(|j| {
            let end = BIRTH + j * 2_000_000;
            last.0 < end && end <= t.0
        });
        let start = Instant::now();
        timeline.render_components(&context, t, ORIGIN, &mut out, &host_impl::SequentialJobs).unwrap();
        let ms = start.elapsed().as_secs_f64() * 1e3;
        if crosses { crossing.push(ms) } else { ordinary.push(ms) }
        last = t;
    }
    let n = ordinary.len();
    let (m, p, x) = stats(ordinary);
    println!("| balayage 0–40 s, images ordinaires | repli temporel | {n} | {m:.4} | {p:.4} | {x:.4} |");
    let n = crossing.len();
    let (m, p, x) = stats(crossing);
    println!("| balayage 0–40 s, fin de tronçon franchie | repli temporel | {n} | {m:.4} | {p:.4} | {x:.4} |");

    // Retour arrière (relance R) : repli complet refait.
    let mut jumps = Vec::new();
    for _ in 0..5 {
        timeline.render_components(&context, frame(39.0, 0), ORIGIN, &mut out, &host_impl::SequentialJobs).unwrap();
        let start = Instant::now();
        timeline.render_components(&context, frame(3.0, 0), ORIGIN, &mut out, &host_impl::SequentialJobs).unwrap();
        jumps.push(start.elapsed().as_secs_f64() * 1e3);
        timeline.render_components(&context, frame(39.0, 0), ORIGIN, &mut out, &host_impl::SequentialJobs).unwrap();
    }
    let mut forward = Vec::new();
    for _ in 0..5 {
        timeline.render_components(&context, frame(0.5, 0), ORIGIN, &mut out, &host_impl::SequentialJobs).unwrap();
        let start = Instant::now();
        timeline.render_components(&context, frame(39.0, 0), ORIGIN, &mut out, &host_impl::SequentialJobs).unwrap();
        forward.push(start.elapsed().as_secs_f64() * 1e3);
    }
    let (m, _, x) = stats(jumps);
    println!("| saut 39 s → 3 s (repli refait) | repli temporel | 5 | {m:.4} | — | {x:.4} |");
    let (m, _, x) = stats(forward);
    println!("| saut 0,5 s → 39 s (huit replis) | repli temporel | 5 | {m:.4} | — | {x:.4} |");
}
