//! Banc local S81, pas un budget cible. Sorties consommées hors sections chronométrées.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::{hint::black_box, time::Instant};
use water_core::*;
use water_core::{
    impact_field::Medium,
    prepared_water::{Context, Prepared},
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::{Cause, Journal},
};
fn times(mut action: impl FnMut(), repeats: usize) -> [f64; 3] {
    for _ in 0..3 {
        action();
    }
    let mut v = Vec::with_capacity(repeats);
    for _ in 0..repeats {
        let t = Instant::now();
        action();
        v.push(t.elapsed().as_secs_f64() * 1e6);
    }
    v.sort_by(f64::total_cmp);
    [v[repeats / 2], v[(repeats - 1) * 9 / 10], v[repeats - 1]]
}
fn event(id: u64) -> WaveEvent {
    WaveEvent::impact(Impact {
        id,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(0),
        ttl_us: 4_000_000,
        position: [0.0; 3],
        energy_j: 0.01,
        wavelength_m: 4.0,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap()
}
fn main() {
    if std::env::args().any(|s| s == "--angles") {
        println!("// S81 : PhaseQ32(i << 25).cos(), i=0..127. Généré par bench_water --angles.");
        println!("pub(crate) const DIRECTIONS: [f32; 128] = [");
        for i in 0..128u32 {
            println!(
                "f32::from_bits(0x{:08x}),",
                PhaseQ32(i << 25).cos().to_bits()
            );
        }
        println!("];");
        return;
    }
    let mut allocator = host_impl::ArenaAllocator::with_capacity(1 << 20);
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let bg = Background::configure(
        &mut HostServices {
            alloc: &mut allocator,
            jobs: &jobs,
            sink: &sink,
        },
        SeaState {
            hs: 0.01,
            tp: 6.0,
            theta_turns: 0.0,
            components: 32,
            graine: 0,
        },
        WorldPos::from_units(0, 0, 0),
    )
    .unwrap();
    allocator.seal();
    let context = Context {
        frame: FrameId(0),
        cell: 0,
        medium: Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 20.0,
            max_slope: 0.1,
        },
        domain: Domain {
            radius: 16.0,
            age_us: 4_000_000,
        },
    };
    println!("sources points pool_bytes buffers_bytes prep_us_p50_p90_max B_us_p50_p90_max BW_us_p50_p90_max hash");
    for (sources, points_n) in [(1, 16), (1, 64), (4, 64), (16, 64)] {
        let mut records = vec![None; sources];
        let mut j = Journal::new(0, &mut records);
        for i in 0..sources {
            j.confirm(
                0,
                Cause {
                    entity: 0,
                    command: i as u64,
                    emission: 0,
                },
                event(i as u64),
            )
            .unwrap();
        }
        let mut pool: Vec<Option<RadialImpact<64>>> = (0..sources).map(|_| None).collect();
        let points: Vec<[f32; 2]> = (0..points_n)
            .map(|i| [(i % 8) as f32 * 0.25, (i / 8) as f32 * 0.25])
            .collect();
        let world: Vec<WorldPos> = points
            .iter()
            .map(|p| WorldPos::from_metres(p[0] as f64, p[1] as f64, 0.0))
            .collect();
        let mut bases = vec![WaterSample::default(); points_n];
        let mut scratch = bases.clone();
        let mut output = bases.clone();
        let prep = times(
            || {
                let p = Prepared::build(&j, &mut pool, context).unwrap();
                black_box(p.field_count());
            },
            21,
        );
        let prepared = Prepared::build(&j, &mut pool, context).unwrap();
        let t = SimTime(1_000_000);
        let bt = times(
            || {
                for i in 0..points_n {
                    bases[i] = bg.eval(black_box(world[i]), t).unwrap();
                }
                black_box(&bases);
            },
            21,
        );
        let bw = times(
            || {
                prepared
                    .sample_world_batch(
                        &water_core::prepared_water::BoundBackground::new(&bg, FrameId(0), 0),
                        black_box(&world),
                        t,
                        1.0,
                        &mut output,
                        &mut scratch,
                    )
                    .unwrap();
                black_box(&output);
            },
            21,
        );
        let mut hash = Hasher64::new();
        for s in &output {
            for v in [
                s.eta,
                s.deta_dt,
                s.steepness,
                s.aeration,
                s.normal[0],
                s.normal[1],
                s.normal[2],
                s.u_total[0],
                s.u_total[1],
                s.u_total[2],
            ] {
                hash.write_f32(v);
            }
        }
        let pool_bytes = sources * std::mem::size_of::<Option<RadialImpact<64>>>();
        let buffers = points_n
            * (3 * std::mem::size_of::<WaterSample>()
                + std::mem::size_of::<[f32; 2]>()
                + std::mem::size_of::<WorldPos>());
        println!(
            "{sources} {points_n} {pool_bytes} {buffers} {prep:.3?} {bt:.3?} {bw:.3?} {:016x}",
            hash.finish()
        );
    }
    let bessel = times(
        || {
            for i in 0..1024 {
                black_box(water_core::radial_impact::bessel(black_box(i as f32 / 16.0)).unwrap());
            }
        },
        21,
    );
    println!("bessel_1024_us={bessel:.3?}");
}
