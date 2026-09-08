//! S98 : coûts séparés, allocations hôte avant mesure. Aucun budget cible certifié.
use std::{hint::black_box, mem::size_of, time::Instant};
use water_core::{
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::Segment,
    spectral_pressure::{prepare, Node, Slot, Surface},
    Hasher64, SimTime,
};
fn measure(mut f: impl FnMut()) -> (f64, f64, f64) {
    for _ in 0..3 {
        f();
    }
    let mut times = [0.0f64; 21];
    for t in &mut times {
        let start = Instant::now();
        f();
        *t = start.elapsed().as_secs_f64() * 1e6;
    }
    times.sort_by(f64::total_cmp);
    (times[0], times[10], times[20])
}
fn hash(samples: &[Surface]) -> u64 {
    let mut h = Hasher64::new();
    for s in samples {
        for v in [
            s.eta,
            s.vertical_velocity,
            s.potential,
            s.slope[0],
            s.slope[1],
            s.horizontal_velocity[0],
            s.horizontal_velocity[1],
        ] {
            h.write_f32(v);
        }
    }
    h.finish()
}
fn main() {
    let recipe = Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial: 128,
        angular: 128,
    };
    let a = Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    let path = [
        a,
        Segment {
            birth: SimTime(2_000_000),
            origin: [4.0, 0.0],
            velocity: [0.0, 2.0],
            ..a
        },
    ];
    let mut nodes = vec![Node::default(); 16384];
    let paired = std::env::args().any(|s| s == "--half");
    let count = if paired { 8192 } else { 16384 };
    let mut reduced = vec![Node::default(); if paired { 8192 } else { 0 }];
    let mut slots = vec![Slot::default(); count];
    let mut output = [Surface::default(); 121];
    let mut points = [[0.0f32; 2]; 121];
    for (i, p) in points.iter_mut().enumerate() {
        *p = [-8.0 + (i % 11) as f32 * 2.0, -8.0 + (i / 11) as f32 * 2.0];
    }
    println!("us min/median/max, 3 warmups + 21 measurements; 128x128, turn, t=3s, end=8s");
    let times = measure(|| {
        let s = bake(black_box(recipe), black_box(&mut nodes)).unwrap();
        black_box(s.hash());
    });
    println!("bake {times:?}");
    let spectrum = bake(recipe, &mut nodes).unwrap();
    assert_eq!(spectrum.hash(), 0x20e6_4a39_2ae2_37a1);
    if paired {
        let t = measure(|| {
            let h = spectrum.half_into(black_box(&mut reduced)).unwrap();
            black_box(h.nodes());
        });
        println!("reduce {t:?}");
    }
    let half = if paired {
        Some(spectrum.half_into(&mut reduced).unwrap())
    } else {
        None
    };
    let input = half.as_ref().map_or(spectrum.nodes(), |h| h.nodes());
    println!("mode_count={count}");
    let times = measure(|| {
        let f = prepare(
            black_box(input),
            black_box(&path),
            9.81,
            1025.0,
            black_box(SimTime(3_000_000)),
            SimTime(8_000_000),
            [-8.0; 2],
            [12.0; 2],
            black_box(&mut slots),
        )
        .unwrap();
        black_box(f.energy_j);
    });
    println!("prepare_two_segments {times:?}");
    let field = prepare(
        input,
        &path,
        9.81,
        1025.0,
        SimTime(3_000_000),
        SimTime(8_000_000),
        [-8.0; 2],
        [12.0; 2],
        &mut slots,
    )
    .unwrap();
    let mut scratch = [Surface::default(); 121];
    for count in [1, 64, 121] {
        let times = measure(|| {
            for i in 0..count {
                output[i] = field.sample(black_box(points[i])).unwrap();
            }
            black_box(&output[..count]);
        });
        println!(
            "sample_{count} {times:?} hash={:016x}",
            hash(&output[..count])
        );
        let expected = hash(&output[..count]);
        let times = measure(|| {
            field
                .sample_batch(
                    black_box(&points[..count]),
                    black_box(&mut scratch),
                    black_box(&mut output),
                )
                .unwrap();
            black_box(&output[..count]);
        });
        assert_eq!(hash(&output[..count]), expected);
        println!("batch_{count} {times:?} hash={expected:016x}");
    }
    println!("batch scratch bytes={}", size_of_val(&scratch));
    // Comparaison hors chronométrage, indépendante de la cuisson et du calcul modal candidats.
    let model = water_core::gaussian_pressure::GaussianPressure::new(
        1.0,
        6.0,
        128,
        128,
        9.81f32 as f64,
        1025.0,
    )
    .unwrap();
    let reference_path = path.map(|s| water_core::pressure_mode::PressureSegment {
        birth: s.birth,
        duration_us: s.duration_us,
        origin: s.origin.map(f64::from),
        velocity: s.velocity.map(f64::from),
        pressure_pa: s.pressure_pa as f64,
    });
    let reference = model
        .trajectory(&reference_path, SimTime(3_000_000))
        .unwrap();
    let mut max = 0.0f64;
    for (p, s) in points.iter().zip(output) {
        let r = reference.sample(p.map(f64::from)).unwrap();
        for (x, y) in [
            (s.eta, r.eta),
            (s.vertical_velocity, r.vertical_velocity),
            (s.potential, r.potential),
            (s.slope[0], r.slope[0]),
            (s.slope[1], r.slope[1]),
            (s.horizontal_velocity[0], r.horizontal_velocity[0]),
            (s.horizontal_velocity[1], r.horizontal_velocity[1]),
        ] {
            max = max.max((x as f64 - y).abs());
        }
    }
    assert!(max < 1e-7);
    println!("max absolute component error={max:.9e} (mixed units, threshold 1e-7 in each unit)");
    println!("bytes Node={} Slot={} Surface={} nodes={} reduced_nodes={} one_field={} two_fields={} points={} output={} trajectory={}",size_of::<Node>(),size_of::<Slot>(),size_of::<Surface>(),16384*size_of::<Node>(),reduced.len()*size_of::<Node>(),count*size_of::<Slot>(),2*count*size_of::<Slot>(),size_of_val(&points),size_of_val(&output),size_of_val(&path));
    println!("excludes allocator metadata, reference model, stack temporaries and OS; no allocation counter");
}
