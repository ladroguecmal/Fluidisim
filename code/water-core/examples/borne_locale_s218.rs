//! ADR-135 : coût d'une partition complète, sans allocation pendant la mesure.
use std::time::Instant;
use water_core::{
    bound_pressure::{Prepared, Settings},
    gaussian_spectrum::{self, Recipe},
    pressure_journal::Journal,
    pressure_source::Metadata,
    spectral_pressure::{Node, Slot},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

fn main() {
    println!("S218 CPU release un fil; source gaussienne preparee, borne locale ADR135; aucune technique GPU/LOD/visibilite/mutualisation; partition inclut reste et reserve; preparation separee");
    for (name, speed, duration, tau, reference) in [
        ("base", 3., 8., 4., 0.070323104f32),
        ("lent", 1.5, 8., 4., 0.014352296),
        ("long", 3., 16., 4., 0.066987508),
        ("base_tard", 3., 8., 24., 0.044552853),
    ] {
        if std::env::args().nth(1).is_some_and(|filter| filter != name) {
            continue;
        }
        let recipe = Recipe {
            sigma: 2.,
            cutoff: 3.,
            radial: 64,
            angular: 128,
        };
        let settings = Settings {
            frame: FrameId(0),
            cell: 0,
            gravity: 9.81,
            density: 1025.,
            min: [-64., -48.],
            max: [64., 48.],
            start: SimTime(12_000_000),
            end: SimTime(72_000_000),
        };
        let metadata = Metadata {
            epoch: 1,
            id: 218,
            cause: Cause {
                entity: 218,
                command: 1,
                emission: 0,
            },
            settings,
            recipe,
        };
        let legs = [Leg {
            duration_us: (duration * 1e6 / 4.) as u64,
            velocity: [speed, 0.],
            downward_force_n: 19620.,
        }; 4];
        let wake = Wake::build(metadata, settings.start, [-12., 0.], &legs).unwrap();
        let mut records = [None];
        let mut journal = Journal::new(1, &mut records);
        journal.admit_authenticated(wake.source()).unwrap();
        let mut nodes = vec![Node::default(); 8192];
        let mut half = vec![Node::default(); 4096];
        let spectrum = gaussian_spectrum::bake(recipe, &mut nodes)
            .unwrap()
            .half_into(&mut half)
            .unwrap();
        let mut slots = vec![Slot::default(); 4096];
        let ctx = wake.source().context();
        let age = duration as f64 + tau * (2.0f64 / 9.81).sqrt();
        let time = SimTime(12_000_000 + (age * 1e6).round() as u64);
        let start = Instant::now();
        let f = Prepared::from_journal(ctx, &spectrum, &journal, time, &mut slots).unwrap();
        let prep_ms = start.elapsed().as_secs_f64() * 1e3;
        let global = f.slope_envelope();
        println!("fixture={name} age={age:.6} reference_s217={reference:.9} global={global:.9} preparation_ms={prep_ms:.3} image_time_admitted={}",age<=18.5313);
        // Chauffe du chemin avant le premier chronométrage ; aucun résultat escamoté.
        for _ in 0..100 {
            std::hint::black_box(
                f.local_slope_envelope(&ctx, time, [0.; 2], [0.5; 2])
                    .unwrap(),
            );
        }
        for step in [2.0f32, 1., 0.5] {
            let nx = (128. / step) as usize;
            let ny = (96. / step) as usize;
            let start = Instant::now();
            let (mut bound, mut reserve, mut center) = (0.0f32, 0.0f32, 0.0f32);
            for iy in 0..ny {
                for ix in 0..nx {
                    let lo = [-64. + ix as f32 * step, -48. + iy as f32 * step];
                    let hi = [lo[0] + step, lo[1] + step];
                    let b = f.local_slope_envelope(&ctx, time, lo, hi).unwrap();
                    bound = bound.max(b.bound);
                    reserve = reserve.max(b.numerical_reserve);
                    center = center.max(b.center_slope);
                }
            }
            let ms = start.elapsed().as_secs_f64() * 1e3;
            assert!(bound >= reference && bound >= center);
            println!("fixture={name} step={step} rectangles={} bound={bound:.9} center_max={center:.9} reserve={reserve:.9} bound_over_reference={:.6} gain={:.6} partition_ms={ms:.3}",nx*ny,bound/reference,global/bound);
        }
    }
}
