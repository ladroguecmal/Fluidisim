//! S103 : deux recettes, fixture reçue par receive_resolution --dense.
use std::{hint::black_box, mem::size_of, time::Instant};
use water_core::{
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::Segment,
    spectral_pressure::{prepare, Node, Slot, Surface},
    SimTime,
};
fn measure(mut f: impl FnMut()) -> f64 {
    for _ in 0..3 {
        f();
    }
    let mut t = [0.0f64; 21];
    for v in &mut t {
        let start = Instant::now();
        f();
        *v = start.elapsed().as_secs_f64() * 1e6;
    }
    t.sort_by(f64::total_cmp);
    t[10]
}
fn main() {
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
    let points: Vec<_> = (0..64)
        .map(|i| [-8.0 + (i % 11) as f32 * 2.0, -8.0 + (i / 11) as f32 * 2.0])
        .collect();
    for (r, a) in [(128, 128), (112, 80)] {
        let mut nodes = vec![Node::default(); r * a];
        let mut half = vec![Node::default(); r * a / 2];
        let mut slots = vec![Slot::default(); r * a / 2];
        let mut scratch = [Surface::default(); 64];
        let mut output = [Surface::default(); 64];
        let spectrum = bake(
            Recipe {
                sigma: 1.0,
                cutoff: 6.0,
                radial: r,
                angular: a,
            },
            &mut nodes,
        )
        .unwrap();
        let reduced = spectrum.half_into(&mut half).unwrap();
        let prep = measure(|| {
            let f = prepare(
                black_box(reduced.nodes()),
                &path,
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
        let field = prepare(
            reduced.nodes(),
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
        let query = measure(|| {
            field
                .sample_batch(
                    black_box(&points),
                    black_box(&mut scratch),
                    black_box(&mut output),
                )
                .unwrap();
            black_box(&output);
        });
        println!("{r}x{a}: modes={} recipe_hash={:016x} prepare_us={prep:.1} batch64_us={query:.1} full_nodes_bytes={} reduced_nodes_bytes={} field_bytes={}",r*a/2,spectrum.hash(),r*a*size_of::<Node>(),r*a/2*size_of::<Node>(),r*a/2*size_of::<Slot>());
    }
}
