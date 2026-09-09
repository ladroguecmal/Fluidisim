//! S127 : transport lointain, référence indépendante et domaine réellement admissible.
#[path = "support/radial_reference.rs"]
mod reference;
use reference::{components, directions, event, medium, Reference};
use std::time::Instant;
use water_core::{
    impact_field::Error,
    radial_impact::{Domain, RadialImpact},
    FrameId, SimTime,
};
const RADIUS: f32 = 80.0;
const AGE: u64 = 48_000_000;
const TIMES: [u64; 3] = [0, 24_000_000, AGE];
const RINGS: usize = 640; // 32 m correspond exactement au noeud256 ; coarse320 aussi.
#[derive(Clone, Copy, Default)]
struct Row {
    density: f64,
    potential: f64,
    candidate_potential: f64,
}
fn integrate(rows: &[Row], stride: usize, start: usize) -> [f64; 4] {
    let mut out = [0.0; 4];
    let steps = (RINGS - start) / stride;
    assert_eq!(steps % 2, 0);
    for j in 0..=steps {
        let i = start + j * stride;
        let r = RADIUS as f64 * i as f64 / RINGS as f64;
        let weight = if j == 0 || j == steps {
            1.0
        } else if j % 2 == 0 {
            2.0
        } else {
            4.0
        };
        let area =
            std::f64::consts::TAU * r * weight * (RADIUS as f64 / RINGS as f64 * stride as f64)
                / 3.0;
        let row = rows[i];
        for (k, v) in [
            row.density,
            row.density * r,
            row.potential,
            row.candidate_potential,
        ]
        .iter()
        .enumerate()
        {
            out[k] += v * area;
        }
    }
    out
}
fn update(got: [f64; 7], want: [f64; 7], scales: [f64; 7], errors: &mut [f64; 7]) {
    for k in 0..7 {
        assert!(got[k].is_finite() && want[k].is_finite());
        errors[k] = errors[k].max((got[k] - want[k]).abs() / scales[k]);
    }
}
fn main() {
    let start = Instant::now();
    let cg = 0.5 * (medium().gravity as f64 / (std::f64::consts::PI / 4.0)).sqrt();
    for (n, r) in [(128, 64.0f32), (256, 128.0)] {
        let age = (r as f64 / cg * 1e6).ceil() as u64;
        let domain = Domain {
            radius: r,
            age_us: age,
        };
        let result = if n == 128 {
            RadialImpact::<128>::new(event(), medium(), domain).err()
        } else {
            RadialImpact::<256>::new(event(), medium(), domain).err()
        };
        assert_eq!(result, Some(Error::Resolution));
        println!("N{n} R{r} temps groupe {} s : Resolution", age as f64 / 1e6);
    }
    let field = RadialImpact::<256>::new(
        event(),
        medium(),
        Domain {
            radius: RADIUS,
            age_us: AGE,
        },
    )
    .unwrap();
    assert_eq!(event().data().ttl_us, 4_000_000); // durée source inchangée, ADR-066
    let used = RADIUS as f64 + cg * AGE as f64 / 1e6;
    println!(
        "domaine N256 R80 T48: charge={used:.6} m disponible={:.6} m",
        256.0 * 4.0 / 6.0
    );
    let dirs = directions(1024);
    let fine_dirs = directions(2048);
    let mut rows = [
        vec![Row::default(); RINGS + 1],
        vec![Row::default(); RINGS + 1],
        vec![Row::default(); RINGS + 1],
    ];
    let mut coarse = rows.clone();
    let mut candidate_error = [0.0; 7];
    let mut spectral_error = [0.0; 7];
    let mut angular_error = [0.0; 7];
    let mut min_density = f64::INFINITY;
    let mut surface_count = 0;
    for ir in 0..=RINGS {
        let r = RADIUS * ir as f32 / RINGS as f32;
        let a = Reference::new([r, 0.0], 256, &dirs);
        let b = Reference::new([r, 0.0], 512, &dirs);
        for (it, us) in TIMES.iter().copied().enumerate() {
            let aa = a.at(us);
            let bb = b.at(us);
            let got = components(field.sample(FrameId(7), 9, [r, 0.0], SimTime(us)).unwrap());
            update(got, bb, b.scales, &mut candidate_error);
            update(aa, bb, b.scales, &mut spectral_error);
            let da = a.density(us);
            let db = b.density(us);
            min_density = min_density.min(da.min(db));
            let potential =
                |eta: f64| 0.5 * medium().density as f64 * medium().gravity as f64 * eta * eta;
            rows[it][ir] = Row {
                density: db,
                potential: potential(bb[0]),
                candidate_potential: potential(got[0]),
            };
            coarse[it][ir] = Row {
                density: da,
                potential: potential(aa[0]),
                candidate_potential: potential(got[0]),
            };
            surface_count += 1;
        }
        if ir % 20 == 0 {
            // Directions non axiales et raffinement plus fin indépendant du bilan intégré.
            let p = [-0.6 * r, 0.8 * r];
            let b = Reference::new(p, 512, &dirs);
            let c = Reference::new(p, 1024, &dirs);
            let d = Reference::new(p, 1024, &fine_dirs);
            for us in [
                0, 4_000_001, 12_137_119, 24_000_000, 32_718_281, 40_000_003, 47_999_999, AGE,
            ] {
                let bb = b.at(us);
                let cc = c.at(us);
                let dd = d.at(us);
                let got = components(field.sample(FrameId(7), 9, p, SimTime(us)).unwrap());
                update(got, dd, d.scales, &mut candidate_error);
                update(bb, cc, d.scales, &mut spectral_error);
                update(cc, dd, d.scales, &mut angular_error);
                surface_count += 1;
            }
        }
        if ir % 80 == 0 {
            println!("oracle radial {ir}/{RINGS}");
        }
    }
    let energy = event().data().energy_j as f64;
    let mut mean = [0.0; 3];
    let mut outside = [0.0; 3];
    for it in 0..3 {
        let full = integrate(&rows[it], 1, 0);
        let radial = integrate(&rows[it], 2, 0);
        let spectral = integrate(&coarse[it], 1, 0);
        let outer = integrate(&rows[it], 1, 256);
        let outer_radial = integrate(&rows[it], 2, 256);
        let outer_spectral = integrate(&coarse[it], 1, 256);
        mean[it] = full[1] / full[0];
        outside[it] = outer[0] / energy;
        println!("t={} s: E/E0={:.10} rayon_moyen={:.8} E_hors32/E0={:.10} potentiel_ref/E0={:.10} potentiel_candidat/E0={:.10}",TIMES[it] as f64/1e6,full[0]/energy,mean[it],outside[it],full[2]/energy,full[3]/energy);
        println!(
            "raffinement radial E/E0={:.3e} spectral E/E0={:.3e} potentiel candidat/ref E0={:.3e}",
            (radial[0] - full[0]).abs() / energy,
            (spectral[0] - full[0]).abs() / energy,
            (full[2] - full[3]).abs() / energy
        );
        assert!((full[0] / energy - 1.0).abs() < 0.003);
        assert!((radial[0] - full[0]).abs() / energy < 0.002);
        assert!((spectral[0] - full[0]).abs() / energy < 1e-4);
        println!(
            "raffinement hors32 radial={:.3e} spectral={:.3e} rayon_moyen={:.3e} m",
            (outer_radial[0] - outer[0]).abs() / energy,
            (outer_spectral[0] - outer[0]).abs() / energy,
            (radial[1] / radial[0] - mean[it]).abs()
        );
        assert!((outer_radial[0] - outer[0]).abs() / energy < 0.002);
        assert!((outer_spectral[0] - outer[0]).abs() / energy < 1e-4);
        assert!((full[2] - full[3]).abs() / energy < 1e-4);
    }
    println!("{surface_count} points-temps, erreur candidat={candidate_error:?}\nspectral={spectral_error:?}\nangulaire={angular_error:?}\ndensite minimale={min_density:.3e}");
    assert!(candidate_error.iter().all(|x| *x <= 1e-4));
    assert!(spectral_error.iter().all(|x| *x <= 1e-6));
    assert!(angular_error.iter().all(|x| *x <= 1e-6));
    assert!(min_density >= -1e-12);
    assert!(outside[0] < 0.003 && outside[2] > 0.5);
    assert!(mean[0] < mean[1] && mean[1] < mean[2]);
    // Un champ figé à la naissance échouerait la demande de transport.
    assert!(outside[0] <= 0.5);
    assert_eq!(
        field
            .sample(FrameId(7), 9, [0.0; 2], SimTime(AGE + 1))
            .err(),
        Some(Error::Time)
    );
    println!(
        "TRANSPORT RECU sur fixture, duree {:.3} s",
        start.elapsed().as_secs_f64()
    );
}
