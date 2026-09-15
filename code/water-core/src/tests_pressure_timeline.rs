//! S213 : réception du levier temporel contre `Prepared::from_journal`.
use super::*;
use crate::{
    bound_pressure::{Prepared, Settings},
    gaussian_spectrum::{bake, Recipe},
    pressure_source::Metadata,
    spectral_pressure::{Node, Slot},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId,
};

const START: u64 = 5_000_000;

/// S243 — système de tâches séquentiel des essais. Il **n'implémente pas** `parallel_fill_f32` :
/// c'est le défaut du trait qui sert, c'est-à-dire la référence de bits de SPEC-004 §8.2.
struct Seq;
impl crate::host::JobSystem for Seq {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(
        &self,
        _: usize,
        _: usize,
        _: &dyn Fn(usize, usize) -> f64,
        _: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        init
    }
}

fn settings(start: u64) -> Settings {
    Settings {
        frame: FrameId(3),
        cell: 4,
        gravity: 9.81,
        density: 1025.0,
        min: [-16.0; 2],
        max: [24.0; 2],
        start: SimTime(start),
        end: SimTime(start + 20_000_000),
    }
}
fn recipe() -> Recipe {
    recipe_of(8, 16)
}
fn recipe_of(radial: usize, angular: usize) -> Recipe {
    Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial,
        angular,
    }
}
/// Quatre tronçons : virages, un arrêt sous charge, une charge nulle.
fn wake(birth: u64, start: u64) -> Wake {
    wake_with(recipe(), birth, start)
}
fn wake_with(recipe: Recipe, birth: u64, start: u64) -> Wake {
    let metadata = Metadata {
        epoch: 1,
        id: 213,
        cause: Cause {
            entity: 213,
            command: 1,
            emission: 0,
        },
        settings: settings(start),
        recipe,
    };
    let leg = |ms: u64, v: [f32; 2], f: f32| Leg {
        duration_us: ms * 1000,
        velocity: v,
        downward_force_n: f,
    };
    let legs = [
        leg(2000, [2.0, 0.0], 400.0),
        leg(1500, [0.0, 2.5], 400.0),
        leg(2500, [0.0, 0.0], 250.0),
        leg(2000, [1.5, 1.0], 0.0),
    ];
    Wake::build(metadata, SimTime(birth), [-3.0, 1.0], &legs).unwrap()
}
fn bits(c: &[[f32; 4]]) -> Vec<u32> {
    c.iter().flatten().map(|v| v.to_bits()).collect()
}
/// Écart max de η et des pentes reconstruits, rapporté aux normes L1 des coefficients attendus.
fn relative_gap(expected: &[[f32; 4]], actual: &[[f32; 4]]) -> f64 {
    let eval = |c: &[[f32; 4]], q: [f64; 2]| {
        let mut v = [0.0f64; 3];
        for &[a, b, kx, ky] in c {
            let (a, b, kx, ky) = (a as f64, b as f64, kx as f64, ky as f64);
            let (s, co) = (kx * q[0] + ky * q[1]).sin_cos();
            v[0] += a * co - b * s;
            v[1] -= kx * (a * s + b * co);
            v[2] -= ky * (a * s + b * co);
        }
        v
    };
    let (norm, slope_norm) = expected.iter().fold((0.0f64, 0.0f64), |(n, s), c| {
        let a = (c[0] as f64).hypot(c[1] as f64);
        (n + a, s + a * (c[2] as f64).hypot(c[3] as f64))
    });
    let mut worst = 0.0f64;
    for q in [[0.0, 0.0], [1.5, -2.0], [-4.0, 3.25], [7.0, 0.5], [-9.0, -6.0]] {
        let (e, a) = (eval(expected, q), eval(actual, q));
        worst = worst.max((e[0] - a[0]).abs() / norm.max(f64::MIN_POSITIVE));
        for i in 1..3 {
            worst = worst.max((e[i] - a[i]).abs() / slope_norm.max(f64::MIN_POSITIVE));
        }
    }
    worst
}

struct Fixture {
    wake: Wake,
    full: Vec<Node>,
    half: Vec<Node>,
}
impl Fixture {
    fn new() -> Self {
        let n = recipe().radial * recipe().angular;
        Self {
            wake: wake(START, START),
            full: vec![Node::default(); n],
            half: vec![Node::default(); n / 2],
        }
    }
}

#[test]
fn timeline_matches_prepared_across_instants_and_jumps_s213() {
    for (radial, angular) in [(8, 16), (32, 64)] {
        let worst = matches_prepared(radial, angular);
        println!("S213 recette {radial}x{angular} : écart relatif max {worst:e}");
    }
}
fn matches_prepared(radial: usize, angular: usize) -> f64 {
    let r = recipe_of(radial, angular);
    let mut fx = Fixture {
        wake: wake_with(r, START, START),
        full: vec![Node::default(); radial * angular],
        half: vec![Node::default(); radial * angular / 2],
    };
    let spectrum = bake(r, &mut fx.full)
        .unwrap()
        .half_into(&mut fx.half)
        .unwrap();
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal.admit_authenticated(fx.wake.source()).unwrap();
    let context = fx.wake.source().context();
    let count = spectrum.nodes().len();
    let mut nodes = vec![NodeState::default(); count];
    let mut modes = vec![None; Timeline::mode_capacity(&spectrum, &journal)];
    assert_eq!(modes.len(), count * 4);
    let mut timeline = Timeline::build(context, &spectrum, &journal, &mut nodes, &mut modes).unwrap();
    assert_eq!(timeline.component_count(), count);
    let mut slots = vec![Slot::default(); count];
    let (mut expected, mut actual) = (vec![[0.0f32; 4]; count], vec![[0.0f32; 4]; count]);
    // Naissance, bornes exactes de tronçon à ±1 µs, fin de forçage, fin de contexte, puis
    // retour arrière, répétition et reprise en avant.
    let us = [
        0u64, 1, 1_000_000, 1_999_999, 2_000_000, 2_000_001, 3_500_000, 5_999_999, 6_000_000,
        7_250_000, 8_000_000, 8_000_001, 12_345_678, 20_000_000, 3_000_000, 3_000_000, 0,
        9_000_000, 19_999_999,
    ];
    let mut worst = 0.0f64;
    for (i, &offset) in us.iter().enumerate() {
        let time = SimTime(START + offset);
        let origin = if i % 2 == 0 { [0.0, 0.0] } else { [5.0, -3.0] };
        Prepared::from_journal(context, &spectrum, &journal, time, &mut slots)
            .unwrap()
            .render_components(&context, time, origin, &mut expected)
            .unwrap();
        timeline
            .render_components(&context, time, origin, &mut actual, &Seq)
            .unwrap();
        for (e, a) in expected.iter().zip(&actual) {
            assert_eq!([e[2], e[3]], [a[2], a[3]]);
        }
        let gap = if offset == 0 {
            // Naissance : les deux chemins rendent zéro.
            assert!(expected.iter().chain(&actual).all(|c| c[0] == 0.0 && c[1] == 0.0));
            0.0
        } else {
            relative_gap(&expected, &actual)
        };
        assert!(gap <= 1e-5, "instant {offset} µs : écart relatif {gap:e}");
        worst = worst.max(gap);
    }
    assert!(worst > 0.0, "le chemin replié n'est pas le chemin préparé au bit");
    worst
}

#[test]
fn incremental_fold_equals_full_fold_bitwise_s213() {
    let mut fx = Fixture::new();
    let spectrum = bake(recipe(), &mut fx.full)
        .unwrap()
        .half_into(&mut fx.half)
        .unwrap();
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal.admit_authenticated(fx.wake.source()).unwrap();
    let context = fx.wake.source().context();
    let count = spectrum.nodes().len();
    let cap = Timeline::mode_capacity(&spectrum, &journal);
    let (mut n1, mut m1) = (vec![NodeState::default(); count], vec![None; cap]);
    let (mut n2, mut m2) = (vec![NodeState::default(); count], vec![None; cap]);
    let mut stepped = Timeline::build(context, &spectrum, &journal, &mut n1, &mut m1).unwrap();
    let mut direct = Timeline::build(context, &spectrum, &journal, &mut n2, &mut m2).unwrap();
    let (mut a, mut b) = (vec![[0.0f32; 4]; count], vec![[0.0f32; 4]; count]);
    for ms in (0..=19_000).step_by(250) {
        stepped
            .render_components(&context, SimTime(START + ms * 1000), [2.0, 1.0], &mut a, &Seq)
            .unwrap();
    }
    let last = SimTime(START + 19_000_000);
    direct.render_components(&context, last, [2.0, 1.0], &mut b, &Seq).unwrap();
    assert_eq!(bits(&a), bits(&b));
}

#[test]
fn refusals_leave_output_and_state_usable_s213() {
    let mut fx = Fixture::new();
    let spectrum = bake(recipe(), &mut fx.full)
        .unwrap()
        .half_into(&mut fx.half)
        .unwrap();
    let context = fx.wake.source().context();
    let count = spectrum.nodes().len();
    let mut nodes = vec![NodeState::default(); count];
    let mut modes = vec![None; count * 4];
    {
        let mut empty_records = [None];
        let empty = Journal::new(1, &mut empty_records);
        assert!(matches!(
            Timeline::build(context, &spectrum, &empty, &mut nodes, &mut modes),
            Err(Error::Empty)
        ));
    }
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal.admit_authenticated(fx.wake.source()).unwrap();
    assert!(matches!(
        Timeline::build(context, &spectrum, &journal, &mut nodes[..count - 1], &mut modes),
        Err(Error::Preparation(PrepareError::Capacity))
    ));
    assert!(matches!(
        Timeline::build(context, &spectrum, &journal, &mut nodes, &mut modes[..count * 4 - 1]),
        Err(Error::Preparation(PrepareError::Capacity))
    ));
    let mut other = settings(START);
    other.cell += 1;
    let wrong = Context::new(other, &spectrum).unwrap();
    assert!(matches!(
        Timeline::build(wrong, &spectrum, &journal, &mut nodes, &mut modes),
        Err(Error::Context)
    ));
    let mut timeline = Timeline::build(context, &spectrum, &journal, &mut nodes, &mut modes).unwrap();
    let mut out = vec![[7.0f32; 4]; count + 1];
    let before = bits(&out);
    let t = SimTime(START + 4_000_000);
    assert_eq!(timeline.render_components(&wrong, t, [0.0; 2], &mut out, &Seq), Err(Error::Context));
    for bad in [SimTime(START - 1), SimTime(START + 20_000_001)] {
        assert_eq!(timeline.render_components(&context, bad, [0.0; 2], &mut out, &Seq), Err(Error::Time));
    }
    assert_eq!(
        timeline.render_components(&context, t, [0.0; 2], &mut out[..count - 1], &Seq),
        Err(Error::Preparation(PrepareError::Capacity))
    );
    for origin in [[f32::NAN, 0.0], [0.0, 4096.0]] {
        assert!(matches!(
            timeline.render_components(&context, t, origin, &mut out, &Seq),
            Err(Error::Preparation(_))
        ));
    }
    assert_eq!(bits(&out), before);
    timeline.render_components(&context, t, [0.0; 2], &mut out, &Seq).unwrap();
    assert_eq!(out[count], [7.0; 4]);
    assert!(out[..count].iter().any(|c| c[0] != 7.0));
}

#[test]
fn wake_sources_cannot_start_before_the_reference_s213() {
    // La garde de `Timeline::build` (naissance avant le début du contexte) repose sur ce refus
    // amont : une source de sillage ne naît jamais avant son contexte.
    let metadata = Metadata {
        epoch: 1,
        id: 1,
        cause: Cause { entity: 1, command: 1, emission: 0 },
        settings: settings(START),
        recipe: recipe(),
    };
    let leg = [Leg { duration_us: 1_000_000, velocity: [1.0, 0.0], downward_force_n: 10.0 }];
    assert!(Wake::build(metadata, SimTime(START - 1), [0.0; 2], &leg).is_err());
    assert!(Wake::build(metadata, SimTime(START), [0.0; 2], &leg).is_ok());
}