//! S126 : réception indépendante du champ étendu. Oracle f64/libm hors runtime.
use std::time::Instant;
#[path = "support/radial_reference.rs"]
mod reference;
use reference::{components, directions, event, medium, Reference};
use water_core::{
    impact_field::Error,
    radial_impact::{Domain, RadialImpact},
    FrameId, SimTime,
};
const LIMIT: f64 = 1e-4; // ADR-060, extension aux échelles des sept composantes : banc B2.
const ORACLE_LIMIT: f64 = 1e-6; // 1 % du seuil ; déclaré en P1.
const TIMES: [u64; 9] = [
    0, 1, 137_119, 500_003, 1_333_331, 2_718_281, 3_999_000, 3_999_999, 4_000_000,
];
fn points(radius: f32) -> Vec<[f32; 2]> {
    let mut points = vec![[0.0; 2]];
    for i in 1..=64 {
        let r = radius * (i as f32 - 0.381_966) / 64.0;
        let d = [[1.0, 0.0], [0.0, -1.0], [-0.6, 0.8], [-0.8, -0.6]][i % 4];
        points.push([r * d[0], r * d[1]]);
    }
    // Le raccord entre table et asymptotique traverse la bande pour 64/hi < r < 64/lo.
    for r in [20.3717, 20.37183, 40.74366, 81.4872, 81.4874] {
        if r <= radius {
            points.push([r, 0.0]);
        }
    }
    for r in [radius * 0.99, f32::from_bits(radius.to_bits() - 1), radius] {
        points.push([r, 0.0]);
        points.push([0.0, -r]);
    }
    points
}
fn max_errors(a: [f64; 7], b: [f64; 7], scales: [f64; 7], max: &mut [f64; 7]) {
    for k in 0..7 {
        assert!(a[k].is_finite() && b[k].is_finite() && scales[k] > 0.0);
        max[k] = max[k].max((a[k] - b[k]).abs() / scales[k]);
    }
}
fn campaign<const N: usize>(radius: f32) {
    let field = RadialImpact::<N>::new(
        event(),
        medium(),
        Domain {
            radius,
            age_us: 4_000_000,
        },
    )
    .unwrap();
    let dirs = directions(1024);
    let fine_dirs = directions(2048);
    let points = points(radius);
    let mut spectral = [[0.0; 7]; 2];
    let mut angular_error = [0.0; 7];
    let mut errors = [0.0; 7];
    let mut absolute = [0.0f64; 7];
    let mut faulty = [0.0; 7];
    let mut scale_report = [0.0; 7];
    let mut tail_error = [[0.0f64; 7]; 2];
    let mut tail_peak = [[0.0f64; 7]; 2];
    for (index, &point) in points.iter().enumerate() {
        let a = Reference::new(point, 512, &dirs);
        let b = Reference::new(point, 1024, &dirs);
        let c = Reference::new(point, 2048, &dirs);
        let d = Reference::new(point, 2048, &fine_dirs);
        scale_report = d.scales;
        for us in TIMES {
            let aa = a.at(us);
            let bb = b.at(us);
            let cc = c.at(us);
            let dd = d.at(us);
            max_errors(aa, bb, d.scales, &mut spectral[0]);
            max_errors(bb, cc, d.scales, &mut spectral[1]);
            max_errors(cc, dd, d.scales, &mut angular_error);
            let got = components(field.sample(FrameId(7), 9, point, SimTime(us)).unwrap());
            max_errors(got, dd, d.scales, &mut errors);
            for k in 0..7 {
                absolute[k] = absolute[k].max((got[k] - dd[k]).abs());
                let r = (point[0] as f64).hypot(point[1] as f64);
                for (region, lower) in [16.0, 0.9 * radius as f64].iter().enumerate() {
                    if r >= *lower {
                        tail_error[region][k] = tail_error[region][k].max((got[k] - dd[k]).abs());
                        tail_peak[region][k] = tail_peak[region][k].max(dd[k].abs());
                    }
                }
            }
            let mut wrong = got;
            wrong[5] = -wrong[5];
            wrong[6] = -wrong[6];
            max_errors(wrong, dd, d.scales, &mut faulty);
            if us == 0 {
                assert_eq!(dd[1], 0.0);
                assert_eq!(dd[2], 0.0);
                assert_eq!(dd[5], 0.0);
                assert_eq!(dd[6], 0.0);
            }
        }
        if index % 20 == 0 {
            println!("N{N} oracle {index}/{}", points.len());
        }
    }
    // Refus au premier point/instant hors domaine, témoins acceptés inclus dans la campagne.
    assert_eq!(
        field
            .sample(
                FrameId(7),
                9,
                [f32::from_bits(radius.to_bits() + 1), 0.0],
                SimTime(0)
            )
            .err(),
        Some(Error::Domain)
    );
    assert_eq!(
        field
            .sample(FrameId(7), 9, [0.0; 2], SimTime(4_000_001))
            .err(),
        Some(Error::Time)
    );
    println!(
        "N{N} R{radius}: {} points-temps, echelles={scale_report:?}",
        points.len() * TIMES.len()
    );
    println!(
        "spectral 512/1024={:?}\nspectral 1024/2048={:?}\nangulaire 1024/2048={angular_error:?}",
        spectral[0], spectral[1]
    );
    println!(
        "erreurs normalisees={errors:?}\nerreurs absolues={absolute:?}\ndefaut signe={faulty:?}"
    );
    for region in 0..2 {
        let ratio: [f64; 7] = std::array::from_fn(|k| {
            assert!(tail_peak[region][k] > 0.0);
            tail_error[region][k] / tail_peak[region][k]
        });
        println!(
            "region {region} pics={:?} erreurs={:?} rapports={ratio:?}",
            tail_peak[region], tail_error[region]
        );
    }
    assert!(spectral.iter().flatten().all(|e| *e <= ORACLE_LIMIT));
    assert!(angular_error.iter().all(|e| *e <= ORACLE_LIMIT));
    assert!(errors.iter().all(|e| *e <= LIMIT));
    assert!(faulty[5].max(faulty[6]) > LIMIT);
    println!("N{N} R{radius}: RECU sur grille, refus et contre-epreuve recus");
}
fn main() {
    let start = Instant::now();
    let center = Reference::new([0.0; 2], 2048, &directions(1024));
    let lo = std::f64::consts::PI / 4.0;
    let width = 3.0 * lo;
    let scale = (event().data().energy_j as f64 * 630.0
        / (std::f64::consts::PI
            * medium().density as f64
            * medium().gravity as f64
            * width
            * (lo + width / 2.0)))
        .sqrt();
    let exact = scale * width * (lo + width / 2.0) / 30.0;
    assert!((center.at(0)[0] / exact - 1.0).abs() < 1e-10);
    println!(
        "centre analytique: eta={exact:.12e} relatif={:.3e}",
        center.at(0)[0] / exact - 1.0
    );
    campaign::<128>(64.0);
    campaign::<256>(128.0);
    println!(
        "duree oracle et campagne {:.3} s",
        start.elapsed().as_secs_f64()
    );
}
