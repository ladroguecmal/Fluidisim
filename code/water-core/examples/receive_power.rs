//! S104 : bilan candidat, intégration temporelle de mesure en f64 hors runtime.
use water_core::{
    gaussian_pressure::GaussianPressure,
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::Segment,
    pressure_mode::PressureSegment,
    spectral_pressure::{prepare, Node, Slot},
    SimTime,
};
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
    let rp = path.map(|s| PressureSegment {
        birth: s.birth,
        duration_us: s.duration_us,
        origin: s.origin.map(f64::from),
        velocity: s.velocity.map(f64::from),
        pressure_pa: s.pressure_pa as f64,
    });
    for (radial, angular) in [(128, 128), (112, 80)] {
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
        let model =
            GaussianPressure::new(1.0, 6.0, radial, angular, 9.81f32 as f64, 1025.0).unwrap();
        let mut max_power = 0.0f64;
        for us in [
            0, 1, 1_000_000, 1_999_999, 2_000_000, 2_000_001, 3_000_000, 3_999_999, 4_000_000,
            8_000_000,
        ] {
            let f = prepare(
                half.nodes(),
                &path,
                9.81,
                1025.0,
                SimTime(us),
                SimTime(8_000_000),
                [-8.0; 2],
                [12.0; 2],
                &mut slots,
            )
            .unwrap();
            let r = model.trajectory(&rp, SimTime(us)).unwrap();
            max_power = max_power.max((f.power_w as f64 - r.power_w).abs());
            if us >= 4_000_000 {
                assert_eq!(f.power_w, 0.0);
            }
        }
        assert!(max_power < 1e-7, "power {max_power}");
        let f = prepare(
            half.nodes(),
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
        let mut spatial_power = 0.0f64;
        for iy in 0..40 {
            for ix in 0..40 {
                let x = -6.0 + (ix as f64 + 0.5) * 0.3;
                let y = -6.0 + (iy as f64 + 0.5) * 0.3;
                let w = f
                    .sample([(x + 4.0) as f32, (y + 2.0) as f32])
                    .unwrap()
                    .vertical_velocity;
                spatial_power -= 10.0 * (-0.5 * (x * x + y * y)).exp() * w as f64 * 0.09;
            }
        }
        let spatial_error = (spatial_power - f.power_w as f64).abs();
        assert!(spatial_error < 1e-7, "spatial power {spatial_error}");
        println!("{radial}x{angular} spatial_power={spatial_power:.12e} spatial_error={spatial_error:.12e}");
        let mut energies = [0.0f64; 2];
        for (i, us) in [4_000_000, 8_000_000].into_iter().enumerate() {
            energies[i] = prepare(
                half.nodes(),
                &path,
                9.81,
                1025.0,
                SimTime(us),
                SimTime(8_000_000),
                [-8.0; 2],
                [12.0; 2],
                &mut slots,
            )
            .unwrap()
            .energy_j as f64;
        }
        assert!((energies[1] - energies[0]).abs() < 1e-7);
        let mut previous = f64::INFINITY;
        for count in [200u64, 400, 800] {
            let step = 4_000_000 / count;
            let mut work = 0.0f64;
            for i in 0..count {
                let f = prepare(
                    half.nodes(),
                    &path,
                    9.81,
                    1025.0,
                    SimTime(i * step + step / 2),
                    SimTime(8_000_000),
                    [-8.0; 2],
                    [12.0; 2],
                    &mut slots,
                )
                .unwrap();
                work += f.power_w as f64 * (step as f64 / 1e6);
            }
            let residual = (work - energies[0]).abs();
            assert!(residual < 3e-6, "work {residual}");
            assert!(
                residual < previous * 0.35,
                "temporal convergence {residual}"
            );
            previous = residual;
            println!("{radial}x{angular} n={count} E={:.12e} work={work:.12e} residual={residual:.12e} power_error={max_power:.12e} free_drift={:.12e}",energies[0],(energies[1]-energies[0]).abs());
        }
    }
}
