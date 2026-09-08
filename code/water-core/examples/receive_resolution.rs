//! S103 : réception échantillonnée de résolution, jamais certificat continu.
use water_core::{
    gaussian_pressure::GaussianPressure,
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::Segment,
    spectral_pressure::{prepare, Node, Slot},
    SimTime,
};
fn main() {
    let dense = std::env::args().any(|x| x == "--dense");
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
    let rp = path.map(|s| water_core::pressure_mode::PressureSegment {
        birth: s.birth,
        duration_us: s.duration_us,
        origin: s.origin.map(f64::from),
        velocity: s.velocity.map(f64::from),
        pressure_pa: s.pressure_pa as f64,
    });
    let initial_times = [
        0, 1_000_000, 1_999_999, 2_000_000, 2_000_001, 3_000_000, 4_000_000, 6_000_000, 8_000_000,
    ];
    let mut times = initial_times.to_vec();
    if dense {
        times.extend((0..=16).map(|i| i * 500_000));
        times.sort_unstable();
        times.dedup();
    }
    let side = if dense { 21 } else { 11 };
    let points: Vec<_> = (0..side * side)
        .map(|i| {
            [
                -8.0 + (i % side) as f32 * (20.0 / (side - 1) as f32),
                -8.0 + (i / side) as f32 * (20.0 / (side - 1) as f32),
            ]
        })
        .collect();
    let mut references = Vec::new();
    for n in [128, 256] {
        let model = GaussianPressure::new(1.0, 6.0, n, n, 9.81f32 as f64, 1025.0).unwrap();
        let mut values = Vec::new();
        for &us in &times {
            let f = model.trajectory(&rp, SimTime(us)).unwrap();
            values.push((
                f.energy_j,
                points
                    .iter()
                    .map(|p| f.sample(p.map(f64::from)).unwrap())
                    .collect::<Vec<_>>(),
            ));
        }
        references.push(values);
    }
    let configs = [
        (32, 128),
        (64, 128),
        (96, 128),
        (128, 128),
        (128, 16),
        (128, 24),
        (128, 32),
        (128, 48),
        (128, 64),
        (96, 32),
        (96, 48),
        (112, 128),
        (128, 80),
        (112, 80),
    ];
    let thresholds = [1e-6, 1e-5, 1e-5, 1e-6, 1e-5, 1e-5];
    println!("points={} times={}", points.len(), times.len());
    for (radial, angular) in configs {
        if dense && ![(128, 128), (112, 80)].contains(&(radial, angular)) {
            continue;
        }
        let mut nodes = vec![Node::default(); radial * angular];
        let mut reduced = vec![Node::default(); radial * angular / 2];
        let spectrum = bake(
            Recipe {
                sigma: 1.0,
                cutoff: 6.0,
                radial,
                angular,
            },
            &mut nodes,
        )
        .unwrap();
        let half = spectrum.half_into(&mut reduced).unwrap();
        let mut slots = vec![Slot::default(); half.nodes().len()];
        let mut errors = [[0.0f64; 6]; 2];
        for (ti, us) in times.iter().enumerate() {
            let f = prepare(
                half.nodes(),
                &path,
                9.81,
                1025.0,
                SimTime(*us),
                SimTime(8_000_000),
                [-8.0; 2],
                [12.0; 2],
                &mut slots,
            )
            .unwrap();
            for ri in 0..2 {
                errors[ri][5] = errors[ri][5].max((f.energy_j as f64 - references[ri][ti].0).abs());
            }
            for (pi, p) in points.iter().enumerate() {
                let s = f.sample(*p).unwrap();
                for ri in 0..2 {
                    let r = references[ri][ti].1[pi];
                    let e = [
                        (s.eta as f64 - r.eta).abs(),
                        (s.vertical_velocity as f64 - r.vertical_velocity).abs(),
                        (s.potential as f64 - r.potential).abs(),
                        (s.slope[0] as f64 - r.slope[0])
                            .abs()
                            .max((s.slope[1] as f64 - r.slope[1]).abs()),
                        (s.horizontal_velocity[0] as f64 - r.horizontal_velocity[0])
                            .abs()
                            .max(
                                (s.horizontal_velocity[1] as f64 - r.horizontal_velocity[1]).abs(),
                            ),
                    ];
                    for j in 0..5 {
                        errors[ri][j] = errors[ri][j].max(e[j]);
                    }
                }
            }
        }
        for ri in 0..2 {
            let pass = errors[ri]
                .iter()
                .zip(thresholds)
                .all(|(e, t)| e.is_finite() && *e < t);
            if dense {
                assert!(
                    pass,
                    "dense reception failed r={radial} a={angular} ref={ri}"
                );
            }
            println!(
                "r={radial} a={angular} vs={} pass={pass} eta/w/phi/slope/u/E={:?}",
                [128, 256][ri],
                errors[ri]
            );
        }
    }
}
