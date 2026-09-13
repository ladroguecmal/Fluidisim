//! ADR-137 : décomposition spectrale par taille de maille, puis partition S219 par ordre.
use std::time::Instant;
use water_core::{
    bound_pressure::{Prepared, Settings},
    gaussian_spectrum::{self, Recipe},
    pressure_journal::Journal,
    pressure_source::Metadata,
    spectral_pressure::{Node, SlopeCell, SlopeOrder, Slot, SpectralSlopeEnvelope},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

fn branch(s: &SpectralSlopeEnvelope) -> &'static str {
    if s.bound == s.second_order.first_order.bound {
        "ordre_un"
    } else if s.bound == s.second_order.second_order_bound {
        "ordre_deux"
    } else if s.bound == s.cuts[0].bound {
        "coupure_2"
    } else if s.bound == s.cuts[1].bound {
        "coupure_1"
    } else {
        "coupure_demi"
    }
}

/// Micro-mesure : coût par appel des trois annonces sur un même rectangle, ordre alterné.
fn micro(f: &Prepared<'_>, ctx: &water_core::bound_pressure::Context, time: SimTime) {
    let (lo, hi) = ([8.0f32, -1.5], [10.0f32, 0.0]);
    let calls = 2000;
    let mut best = [f64::INFINITY; 3];
    for round in 0..6 {
        let order: [usize; 3] = if round % 2 == 0 { [0, 1, 2] } else { [2, 1, 0] };
        for which in order {
            let start = Instant::now();
            for _ in 0..calls {
                match which {
                    0 => {
                        std::hint::black_box(f.local_slope_envelope(ctx, time, lo, hi).unwrap());
                    }
                    1 => {
                        std::hint::black_box(f.local_slope_envelope_second_order(ctx, time, lo, hi).unwrap());
                    }
                    _ => {
                        std::hint::black_box(f.local_slope_envelope_spectral(ctx, time, lo, hi).unwrap());
                    }
                }
            }
            let us = start.elapsed().as_secs_f64() * 1e6 / calls as f64;
            best[which] = best[which].min(us);
            println!("micro round={round} which={} us_per_call={us:.1}", ["ordre_un", "ordre_deux", "spectrale"][which]);
        }
    }
    println!("micro best_us ordre_un={:.1} ordre_deux={:.1} spectrale={:.1}", best[0], best[1], best[2]);
}

fn main() {
    let micro_mode = std::env::args().nth(1).is_some_and(|a| a == "micro");
    println!("S221 CPU release un fil; source gaussienne preparee; ordre deux ADR136 et coupure spectrale ADR137 (une passe, classes D<1/2,[1/2,1),[1,2),>=2, coupures D*=2,1,1/2); tas adaptatif S219; aucune technique GPU/LOD/visibilite/mutualisation; bornes avec reste et reserve; preparation separee; detail de feuille hors chronometrage");
    for (name, speed, duration, tau, reference) in [
        ("base", 3., 8., 4., 0.070323104f32),
        ("lent", 1.5, 8., 4., 0.014352296),
        ("long", 3., 16., 4., 0.066987508),
        ("base_tard", 3., 8., 24., 0.044552853),
    ] {
        if std::env::args()
            .nth(1)
            .is_some_and(|filter| filter != name && !(micro_mode && name == "base"))
        {
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
        // Mêmes identifiants que S219/S220 : champ préparé identique au bit.
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
        if micro_mode {
            micro(&f, &ctx, time);
            return;
        }
        for _ in 0..100 {
            std::hint::black_box(f.local_slope_envelope_second_order(&ctx, time, [0.; 2], [0.5; 2]).unwrap());
            std::hint::black_box(f.local_slope_envelope_spectral(&ctx, time, [0.; 2], [0.5; 2]).unwrap());
        }
        // Grilles égales aux partitions uniformes de 1024, 4096 et 16384 feuilles.
        for (w, h) in [(4.0f32, 3.0f32), (2., 1.5), (1., 0.75)] {
            let nx = (128. / w) as usize;
            let ny = (96. / h) as usize;
            let start = Instant::now();
            let (mut bound, mut second) = (0.0f32, 0.0f32);
            let mut cut_max = [0.0f32; 3];
            let mut worst: Option<([f32; 2], SpectralSlopeEnvelope)> = None;
            let mut first_cell: Option<SpectralSlopeEnvelope> = None;
            let mut wins = [0usize; 5];
            for iy in 0..ny {
                for ix in 0..nx {
                    let lo = [-64. + ix as f32 * w, -48. + iy as f32 * h];
                    let hi = [lo[0] + w, lo[1] + h];
                    let s = f.local_slope_envelope_spectral(&ctx, time, lo, hi).unwrap();
                    assert!(s.bound <= s.second_order.bound);
                    second = second.max(s.second_order.bound);
                    for (m, c) in cut_max.iter_mut().zip(&s.cuts) {
                        *m = m.max(c.bound);
                    }
                    wins[match branch(&s) {
                        "ordre_un" => 0,
                        "ordre_deux" => 1,
                        "coupure_2" => 2,
                        "coupure_1" => 3,
                        _ => 4,
                    }] += 1;
                    first_cell.get_or_insert(s);
                    if s.bound > bound {
                        bound = s.bound;
                        worst = Some((lo, s));
                    }
                }
            }
            let ms = start.elapsed().as_secs_f64() * 1e3;
            assert!(bound >= reference);
            let c0 = first_cell.unwrap();
            let total: f32 = c0.class_mass.iter().sum();
            let fr = c0.class_mass.map(|m| m / total);
            println!("fixture={name} grid={w}x{h} rectangles={} bound={bound:.9} second_order_max={second:.9} cut2_max={:.9} cut1_max={:.9} cut_half_max={:.9} gain_over_second={:.6} gain_over_global={:.6} bound_over_reference={:.6} wins_first={} wins_second={} wins_cut2={} wins_cut1={} wins_cut_half={} mass_total={total:.9} mass_fraction_lt_half={:.4} mass_fraction_half_1={:.4} mass_fraction_1_2={:.4} mass_fraction_ge_2={:.4} grid_ms={ms:.3}",nx*ny,cut_max[0],cut_max[1],cut_max[2],second/bound,global/bound,bound/reference,wins[0],wins[1],wins[2],wins[3],wins[4],fr[0],fr[1],fr[2],fr[3]);
            let (lo, s) = worst.unwrap();
            println!("fixture={name} grid={w}x{h} worst_cell=[{:.3},{:.3}] branch={} first={:.9} second={:.9} {}", lo[0], lo[1], branch(&s), s.second_order.first_order.bound, s.second_order.second_order_bound, s.cuts.iter().map(|c| format!("cut{}: bound={:.9} corner={:.9} remainder={:.9} C_U={:.9} G_U={:.9} reserve={:.9}", c.threshold, c.bound, c.resolved_corner_slope, c.resolved_remainder, c.unresolved_mass, c.unresolved_envelope, c.reserve)).collect::<Vec<_>>().join(" | "));
        }
        let mut pool = vec![SlopeCell::default(); 32768];
        for order in [SlopeOrder::Second, SlopeOrder::Spectral] {
            for budget in [2047, 8191, 16383, 32767] {
                let start = Instant::now();
                let r = f
                    .partition_slope_envelope_order(&ctx, time, settings.min, settings.max, &mut pool, budget, order)
                    .unwrap();
                let ms = start.elapsed().as_secs_f64() * 1e3;
                assert!(r.bound >= reference);
                assert!(r.evaluations <= budget);
                println!("fixture={name} order={order:?} budget={budget} evaluations={} leaves={} bound={:.9} gain={:.6} bound_over_reference={:.6} stop={:?} partition_ms={ms:.3} pool_bytes={}",r.evaluations,r.leaves,r.bound,global/r.bound,r.bound/reference,r.stop,std::mem::size_of_val(pool.as_slice()));
                let (lo, hi) = pool[0].rectangle();
                let s = f.local_slope_envelope_spectral(&ctx, time, lo, hi).unwrap();
                println!("fixture={name} order={order:?} budget={budget} max_leaf=[{:.6},{:.6}]x[{:.6},{:.6}] cell_bound={:.9} spectral_branch={} first={:.9} second={:.9} cut2={:.9} cut1={:.9} cut_half={:.9}",lo[0],hi[0],lo[1],hi[1],pool[0].bound(),branch(&s),s.second_order.first_order.bound,s.second_order.second_order_bound,s.cuts[0].bound,s.cuts[1].bound,s.cuts[2].bound);
            }
        }
    }
}
