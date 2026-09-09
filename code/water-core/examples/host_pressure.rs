//! S107 : scénario hôte et mesures locales, oracle f64 hors chronométrie.
use std::{hint::black_box, mem::size_of, time::Instant};
use water_core::{
    bound_pressure::{Context, Prepared, Settings},
    gaussian_pressure::GaussianPressure,
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::Segment,
    prepared_water::BoundBackground,
    pressure_mode::PressureSegment,
    spectral_pressure::{Node, Slot},
    *,
};
#[derive(Default)]
struct Host {
    sealed: bool,
    stats: AllocStats,
}
impl Allocator for Host {
    fn alloc_persistent(&mut self, n: usize) -> Result<usize, AllocError> {
        if self.sealed {
            self.stats.refused_after_seal += 1;
            return Err(AllocError::Sealed);
        }
        self.stats.persistent_bytes += n;
        self.stats.persistent_calls += 1;
        Ok(0)
    }
    fn seal(&mut self) {
        self.sealed = true;
    }
    fn is_sealed(&self) -> bool {
        self.sealed
    }
    fn stats(&self) -> AllocStats {
        self.stats
    }
}
impl Sink for Host {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Host {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        _: usize,
        r: &dyn Fn(usize, usize) -> f64,
        m: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        m(init, r(0, n))
    }
}
fn measure(mut f: impl FnMut()) -> [f64; 3] {
    for _ in 0..3 {
        f();
    }
    let mut times = [0.0f64; 21];
    for t in &mut times {
        let start = Instant::now();
        f();
        *t = start.elapsed().as_secs_f64() * 1e6;
    }
    times.sort_by(f64::total_cmp);
    [times[0], times[10], times[20]]
}
fn values(s: WaterSample) -> [f32; 10] {
    [
        s.eta,
        s.deta_dt,
        s.u_total[0],
        s.u_total[1],
        s.u_total[2],
        s.normal[0],
        s.normal[1],
        s.normal[2],
        s.steepness,
        s.aeration,
    ]
}
fn digest(samples: &[WaterSample]) -> u64 {
    let mut h = Hasher64::new();
    for &s in samples {
        for v in values(s) {
            h.write_f32(v);
        }
    }
    h.finish()
}
fn main() {
    let mut alloc = Host::default();
    let services = Host::default();
    let anchor = WorldPos::from_metres(1e9, -1e9, 0.0);
    let background = Background::configure(
        &mut HostServices {
            alloc: &mut alloc,
            jobs: &services,
            sink: &services,
        },
        SeaState {
            hs: 0.1,
            tp: 6.0,
            theta_turns: 0.125,
            components: 16,
            graine: 42,
        },
        anchor,
    )
    .unwrap();
    alloc.seal();
    let bound = BoundBackground::new(&background, FrameId(7), 9);
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
    let reference_path = path.map(|s| PressureSegment {
        birth: s.birth,
        duration_us: s.duration_us,
        origin: s.origin.map(f64::from),
        velocity: s.velocity.map(f64::from),
        pressure_pa: s.pressure_pa as f64,
    });
    let points: Vec<_> = (0..64)
        .map(|i| {
            WorldPos::from_units(
                anchor.x + (-8 + 2 * (i % 11)) * WORLD_UNITS_PER_METRE,
                anchor.y + (-8 + 2 * (i / 11)) * WORLD_UNITS_PER_METRE,
                anchor.z,
            )
        })
        .collect();
    for (radial, angular) in [(128, 128), (112, 80)] {
        let mut nodes = vec![Node::default(); radial * angular];
        let mut hn = vec![Node::default(); radial * angular / 2];
        let mut slots = vec![Slot::default(); hn.len()];
        let mut spare = vec![Slot::default(); hn.len()];
        let full = bake(
            Recipe {
                sigma: 1.0,
                cutoff: 6.0,
                radial,
                angular,
            },
            &mut nodes,
        )
        .unwrap();
        let half = full.half_into(&mut hn).unwrap();
        let ctx = Context::new(
            Settings {
                frame: FrameId(7),
                cell: 9,
                gravity: 9.81,
                density: 1025.0,
                min: [-8.0; 2],
                max: [12.0; 2],
                start: SimTime(0),
                end: SimTime(8_000_000),
            },
            &half,
        )
        .unwrap();
        let reference =
            GaussianPressure::new(1.0, 6.0, radial, angular, 9.81f32 as f64, 1025.0).unwrap();
        let mut scratch = [WaterSample::default(); 64];
        let mut output = scratch;
        let mut errors = [0.0f64; 8];
        let mut envelope = 0.0f32;
        let mut hashes = Vec::new();
        for us in [
            0, 1_000_000, 1_999_999, 2_000_000, 2_000_001, 3_000_000, 4_000_000, 6_000_000,
            8_000_000,
        ] {
            let time = SimTime(us);
            let field = Prepared::build(ctx, &half, &path, time, &mut slots).unwrap();
            field
                .sample_world_batch(&bound, &ctx, time, &points, 0.1, &mut scratch, &mut output)
                .unwrap();
            envelope = envelope.max(output[0].steepness * std::f32::consts::PI);
            let r = reference.trajectory(&reference_path, time).unwrap();
            for (point, actual) in points.iter().zip(output) {
                let base = background.eval(*point, time).unwrap();
                let local = background.local_point(*point).unwrap();
                let w = r.sample([local[0] as f64, local[1] as f64]).unwrap();
                let sx = -base.normal[0] as f64 / base.normal[2] as f64 + w.slope[0];
                let sy = -base.normal[1] as f64 / base.normal[2] as f64 + w.slope[1];
                let norm = (1.0 + sx * sx + sy * sy).sqrt();
                let expected = [
                    base.eta as f64 + w.eta,
                    base.deta_dt as f64 + w.vertical_velocity,
                    base.u_total[0] as f64 + w.horizontal_velocity[0],
                    base.u_total[1] as f64 + w.horizontal_velocity[1],
                    base.u_total[2] as f64 + w.vertical_velocity,
                    -sx / norm,
                    -sy / norm,
                    1.0 / norm,
                ];
                for i in 0..8 {
                    assert!(expected[i].is_finite() && values(actual)[i].is_finite());
                    errors[i] = errors[i].max((values(actual)[i] as f64 - expected[i]).abs());
                }
                assert_eq!(actual.aeration, base.aeration);
            }
            hashes.push(digest(&output));
            let before = digest(&output);
            let mut invalid = points.clone();
            invalid[63] = WorldPos::from_units(i64::MIN, 0, 0);
            assert!(field
                .sample_world_batch(&bound, &ctx, time, &invalid, 0.1, &mut scratch, &mut output)
                .is_err());
            assert_eq!(digest(&output), before);
            assert!(field
                .sample_world_batch(
                    &bound,
                    &ctx,
                    SimTime(us + 1),
                    &points,
                    0.1,
                    &mut scratch,
                    &mut output
                )
                .is_err());
            assert_eq!(digest(&output), before);
            let mut bad = path;
            for s in &mut bad {
                s.pressure_pa = 1e30;
            }
            // Hors t=0, énergie non finie du candidat ; aucun remplacement de la publication.
            if us != 0 {
                assert!(Prepared::build(ctx, &half, &bad, time, &mut spare).is_err());
            }
            field
                .sample_world_batch(&bound, &ctx, time, &points, 0.1, &mut scratch, &mut output)
                .unwrap();
            assert_eq!(digest(&output), before);
            let retry = Prepared::build(ctx, &half, &path, time, &mut spare).unwrap();
            retry
                .sample_world_batch(&bound, &ctx, time, &points, 0.1, &mut scratch, &mut output)
                .unwrap();
            assert_eq!(digest(&output), before);
        }
        assert!(errors.iter().all(|&e| e < 2e-7), "{errors:?}");
        let time = SimTime(3_000_000);
        let prep = measure(|| {
            let f = Prepared::build(
                black_box(ctx),
                &half,
                &path,
                black_box(time),
                black_box(&mut slots),
            )
            .unwrap();
            black_box((f.energy_j(), f.slope_envelope()));
        });
        let f = Prepared::build(ctx, &half, &path, time, &mut slots).unwrap();
        let query = measure(|| {
            f.sample_world_batch(
                &bound,
                &ctx,
                time,
                black_box(&points),
                0.1,
                black_box(&mut scratch),
                black_box(&mut output),
            )
            .unwrap();
            black_box(&output);
        });
        let total = measure(|| {
            let f =
                Prepared::build(ctx, &half, &path, black_box(time), black_box(&mut spare)).unwrap();
            f.sample_world_batch(
                &bound,
                &ctx,
                time,
                black_box(&points),
                0.1,
                black_box(&mut scratch),
                black_box(&mut output),
            )
            .unwrap();
            black_box(&output);
        });
        println!(
            "{radial}x{angular} errors_eta_dt_uvw_normal={errors:?} max_envelope={envelope:.9e}"
        );
        println!("hashes={hashes:016x?}");
        println!("prepare_us_min_med_max={prep:?} batch64_us_min_med_max={query:?} cycle_us_min_med_max={total:?}");
        println!("nodes_bytes={} half_bytes={} two_fields_bytes={} points_bytes={} scratch_output_bytes={} background_bytes={}",nodes.len()*size_of::<Node>(),half.nodes().len()*size_of::<Node>(),2*half.nodes().len()*size_of::<Slot>(),points.len()*size_of::<WorldPos>(),128*size_of::<WaterSample>(),alloc.stats().persistent_bytes);
    }
    assert_eq!(alloc.stats().refused_after_seal, 0);
}
