//! S92 : raffinement indépendant du virage, référence f64, pas un budget runtime.
use water_core::{gaussian_pressure::GaussianPressure, pressure_mode::PressureSegment, SimTime};
fn main() {
    let a = PressureSegment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    let b = PressureSegment {
        birth: SimTime(2_000_000),
        origin: [4.0, 0.0],
        velocity: [0.0, 2.0],
        ..a
    };
    let path = [a, b];
    let configs = [
        (128, 128, 6.0),
        (256, 128, 6.0),
        (128, 256, 6.0),
        (192, 128, 9.0),
    ];
    let models: Vec<_> = configs
        .iter()
        .map(|&(r, a, k)| GaussianPressure::new(1.0, k, r, a, 9.81, 1025.0).unwrap())
        .collect();
    let mut max = [[0.0f64; 4]; 3];
    for us in [
        0, 1_000_000, 1_999_999, 2_000_000, 2_000_001, 3_000_000, 4_000_000, 6_000_000, 8_000_000,
    ] {
        let fields: Vec<_> = models
            .iter()
            .map(|m| m.trajectory(&path, SimTime(us)).unwrap())
            .collect();
        for i in 0..3 {
            max[i][2] = max[i][2].max((fields[0].energy_j - fields[i + 1].energy_j).abs());
            max[i][3] = max[i][3].max((fields[0].power_w - fields[i + 1].power_w).abs());
        }
        for iy in 0..11 {
            for ix in 0..11 {
                let p = [-8.0 + 2.0 * ix as f64, -8.0 + 2.0 * iy as f64];
                let base = fields[0].sample(p).unwrap();
                for i in 0..3 {
                    let q = fields[i + 1].sample(p).unwrap();
                    max[i][0] = max[i][0].max((base.eta - q.eta).abs());
                    max[i][1] = max[i][1].max((base.vertical_velocity - q.vertical_velocity).abs());
                }
            }
        }
    }
    for (name, errors) in ["radial", "angular", "cutoff"].iter().zip(max) {
        println!(
            "{name}: eta_m={:.9e} velocity_m_s={:.9e} energy_j={:.9e} power_w={:.9e}",
            errors[0], errors[1], errors[2], errors[3]
        );
        assert!(errors[0] < 1e-6 && errors[1] < 1e-5 && errors[2] < 1e-5 && errors[3] < 1e-5);
    }
    println!("9 times x 121 points = 1089 points per comparison; bounds [-8,12]^2, [0,8]s; sampled evidence only");
}
