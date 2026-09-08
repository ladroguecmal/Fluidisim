//! S102 : composants isolés, pas extrapolation du coût du champ.
use std::{hint::black_box, time::Instant};
use water_core::PhaseQ32;
fn measure(name: &str, mut f: impl FnMut()) {
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
    println!(
        "{name} us min/median/max={}/{}/{}",
        times[0], times[10], times[20]
    );
}
fn main() {
    let mut state = 123456789u32;
    let phases: Vec<_> = (0..65536)
        .map(|_| {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            PhaseQ32(state)
        })
        .collect();
    let points: Vec<_> = (0..65536)
        .map(|i| [(i % 256) as f32 / 16.0 - 8.0, (i / 256) as f32 / 16.0 - 8.0])
        .collect();
    let mut pairs = vec![(0.0f32, 0.0f32); phases.len()];
    let mut output = vec![PhaseQ32(0); phases.len()];
    measure("spatial_phase_65536", || {
        for (p, o) in black_box(&points).iter().zip(output.iter_mut()) {
            *o = PhaseQ32::from_distance(black_box(0.37), p[0])
                .wrapping_add(PhaseQ32::from_distance(black_box(-0.23), p[1]));
        }
        black_box(&output);
    });
    measure("separate_65536", || {
        for (p, o) in black_box(&phases).iter().zip(pairs.iter_mut()) {
            *o = (p.sin(), p.cos());
        }
        black_box(&pairs);
    });
    let expected = pairs.clone();
    measure("joint_65536", || {
        for (p, o) in black_box(&phases).iter().zip(pairs.iter_mut()) {
            *o = p.sin_cos();
        }
        black_box(&pairs);
    });
    for (a, b) in pairs.iter().zip(expected) {
        assert_eq!(
            (a.0.to_bits(), a.1.to_bits()),
            (b.0.to_bits(), b.1.to_bits())
        );
    }
}
