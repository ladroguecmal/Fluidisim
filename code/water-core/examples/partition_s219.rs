//! ADR-135 : coût d'une partition complète, sans allocation pendant la mesure.
use std::time::Instant;
use water_core::{
    bound_pressure::{Prepared, Settings},
    gaussian_spectrum::{self, Recipe},
    pressure_journal::Journal,
    pressure_source::Metadata,
    spectral_pressure::{Node, SlopeCell, Slot},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

fn main() {
    println!("S219 CPU release un fil; source gaussienne preparee, borne locale ADR135; tas adaptatif grand cote, borne parent heritee; aucune technique GPU/LOD/visibilite/mutualisation; partition inclut reste et reserve; preparation separee");
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
            id: 219,
            cause: Cause {
                entity: 219,
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
        let mut pool = vec![SlopeCell::default(); 32768];
        for budget in [8191, 32767, 65535] {
            let start = Instant::now();
            let r = f
                .partition_slope_envelope(&ctx, time, settings.min, settings.max, &mut pool, budget)
                .unwrap();
            let ms = start.elapsed().as_secs_f64() * 1e3;
            assert!(r.bound >= reference);
            assert!(r.evaluations <= budget);
            println!("fixture={name} budget={budget} evaluations={} leaves={} bound={:.9} gain={:.6} bound_over_reference={:.6} stop={:?} partition_ms={ms:.3} pool_bytes={}",r.evaluations,r.leaves,r.bound,global/r.bound,r.bound/reference,r.stop,std::mem::size_of_val(pool.as_slice()));
        }
    }
}
