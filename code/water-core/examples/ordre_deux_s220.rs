//! ADR-136 : borne d'ordre deux sur les rectangles S218 et dans la partition S219.
use std::time::Instant;
use water_core::{
    bound_pressure::{Prepared, Settings},
    gaussian_spectrum::{self, Recipe},
    pressure_journal::Journal,
    pressure_source::Metadata,
    spectral_pressure::{Node, SlopeCell, SlopeOrder, Slot},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

fn main() {
    println!("S220 CPU release un fil; source gaussienne preparee; ordre un ADR135 et ordre deux ADR136 (une passe, branche ADR135 incluse); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee");
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
        // Mêmes identifiants que S219 : champ préparé identique au bit.
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
        println!("fixture={name} age={age:.6} reference_s217={reference:.9} global={global:.9} preparation_ms={prep_ms:.3} image_time_admitted={} modes={}",age<=18.5313,f.component_count());
        // Chauffe des deux chemins avant le premier chronométrage ; aucun résultat escamoté.
        for _ in 0..100 {
            std::hint::black_box(f.local_slope_envelope(&ctx, time, [0.; 2], [0.5; 2]).unwrap());
            std::hint::black_box(
                f.local_slope_envelope_second_order(&ctx, time, [0.; 2], [0.5; 2])
                    .unwrap(),
            );
        }
        // Rectangles S218 : l'ordre un publié dans la même passe doit redonner S218 au bit.
        for step in [2.0f32, 1., 0.5] {
            let nx = (128. / step) as usize;
            let ny = (96. / step) as usize;
            let start = Instant::now();
            let (mut bound, mut first, mut second) = (0.0f32, 0.0f32, 0.0f32);
            let (mut modes, mut modes_max, mut tighter) = (0usize, 0usize, 0usize);
            for iy in 0..ny {
                for ix in 0..nx {
                    let lo = [-64. + ix as f32 * step, -48. + iy as f32 * step];
                    let hi = [lo[0] + step, lo[1] + step];
                    let b = f
                        .local_slope_envelope_second_order(&ctx, time, lo, hi)
                        .unwrap();
                    assert!(b.bound <= b.first_order.bound);
                    bound = bound.max(b.bound);
                    first = first.max(b.first_order.bound);
                    second = second.max(b.second_order_bound);
                    modes += b.hessian_modes;
                    modes_max = modes_max.max(b.hessian_modes);
                    tighter += (b.second_order_bound < b.first_order.bound) as usize;
                }
            }
            let ms = start.elapsed().as_secs_f64() * 1e3;
            let count = nx * ny;
            assert!(bound >= reference);
            println!("fixture={name} grid_step={step} rectangles={count} bound={bound:.9} first_order_bound={first:.9} second_order_branch_max={second:.9} gain_over_first={:.6} gain_over_global={:.6} bound_over_reference={:.6} tighter_rectangles={tighter} hessian_modes_mean={:.1} hessian_modes_max={modes_max} grid_ms={ms:.3}",first/bound,global/bound,bound/reference,modes as f64/count as f64);
        }
        let mut pool = vec![SlopeCell::default(); 32768];
        for order in [SlopeOrder::First, SlopeOrder::Second] {
            for budget in [2047, 8191, 32767, 65535] {
                let start = Instant::now();
                let r = f
                    .partition_slope_envelope_order(&ctx, time, settings.min, settings.max, &mut pool, budget, order)
                    .unwrap();
                let ms = start.elapsed().as_secs_f64() * 1e3;
                assert!(r.bound >= reference);
                assert!(r.evaluations <= budget);
                println!("fixture={name} order={order:?} budget={budget} evaluations={} leaves={} bound={:.9} gain={:.6} bound_over_reference={:.6} stop={:?} partition_ms={ms:.3} pool_bytes={}",r.evaluations,r.leaves,r.bound,global/r.bound,r.bound/reference,r.stop,std::mem::size_of_val(pool.as_slice()));
                // Hors chronométrage : quelle branche plafonne la feuille maximale, et à quelle taille.
                let (lo, hi) = pool[0].rectangle();
                let d = f.local_slope_envelope_second_order(&ctx, time, lo, hi).unwrap();
                println!("fixture={name} order={order:?} budget={budget} max_leaf=[{:.6},{:.6}]x[{:.6},{:.6}] width={:.6} height={:.6} cell_bound={:.9} center_slope={:.9} first_branch={:.9} first_reserve={:.9} second_branch={:.9} second_reserve={:.9} corner_slope={:.9} second_remainder={:.9} hessian_modes={}",lo[0],hi[0],lo[1],hi[1],hi[0]-lo[0],hi[1]-lo[1],pool[0].bound(),d.first_order.center_slope,d.first_order.bound,d.first_order.numerical_reserve,d.second_order_bound,d.second_order_reserve,d.corner_slope,d.second_order_remainder,d.hessian_modes);
            }
        }
    }
}
