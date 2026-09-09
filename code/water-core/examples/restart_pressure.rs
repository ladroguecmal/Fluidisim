//! S111 : reprise du journal vers le champ, identité avec construction directe.
use std::{hint::black_box, time::Instant};
use water_core::{
    bound_pressure::{Context, Prepared, Settings},
    gaussian_spectrum::{bake, Recipe},
    modal_pressure::Segment,
    prepared_water::BoundBackground,
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
fn measure_small(mut f: impl FnMut()) -> [f64; 3] {
    measure(|| {
        for _ in 0..1000 {
            f();
        }
    })
    .map(|v| v / 1000.0)
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
use water_core::{
    pressure_journal::{Change, Error as JournalError, Journal},
    pressure_source::{Metadata, Source},
    wave_journal::Cause,
};
fn main() {
    let mut alloc = Host::default();
    let services = Host::default();
    let anchor = WorldPos::from_metres(1e9, -1e9, 0.0);
    let b = Background::configure(
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
    let bound = BoundBackground::new(&b, FrameId(7), 9);
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
        let recipe = Recipe {
            sigma: 1.0,
            cutoff: 6.0,
            radial,
            angular,
        };
        let metadata = Metadata {
            epoch: 11,
            id: 22,
            cause: Cause {
                entity: 33,
                command: 44,
                emission: 0,
            },
            settings: Settings {
                frame: FrameId(7),
                cell: 9,
                gravity: 9.81,
                density: 1025.0,
                min: [-8.0; 2],
                max: [12.0; 2],
                start: SimTime(0),
                end: SimTime(8_000_000),
            },
            recipe,
        };
        let direct = Source::new(metadata, &path).unwrap();
        let mut wire = [0; 196];
        direct.encode_into(&mut wire).unwrap();
        let mut decoded_segments = path;
        let decoded = Source::decode_into(&wire, &mut decoded_segments).unwrap();
        let mut empty = [];
        let mut journal = Journal::new(11, &mut empty);
        assert_eq!(
            journal.admit_authenticated(decoded),
            Err(JournalError::Full)
        );
        assert!(journal.current().is_err());
        let mut snapshot = [0; 228];
        assert_eq!(journal.snapshot_into(&mut snapshot), Ok(228));
        let mut restored_segments = path;
        let mut slots = [None; 1];
        let mut restored =
            Journal::restore_into(&snapshot, 11, &mut slots, &mut restored_segments).unwrap();
        assert!(restored.current().is_err());
        assert_eq!(restored.published().count(), 0);
        let mut roundtrip = [0; 228];
        restored.snapshot_into(&mut roundtrip).unwrap();
        assert_eq!(roundtrip, snapshot);
        assert_eq!(restored.retry(), Ok(Change::Added));
        let received = restored.current().unwrap().next().unwrap();
        assert!(received.same_content(&direct));
        // Source de calcul reconstruite, jamais le tableau original substitué à la restauration.
        let mut nodes = vec![Node::default(); radial * angular];
        let mut reduced = vec![Node::default(); radial * angular / 2];
        let spectrum = bake(received.metadata().recipe, &mut nodes).unwrap();
        let half = spectrum.half_into(&mut reduced).unwrap();
        let ctx = Context::new(metadata.settings, &half).unwrap();
        assert!(received.context().matches(&ctx));
        let mut field_pool = vec![Slot::default(); half.nodes().len()];
        let mut direct_pool = field_pool.clone();
        let mut scratch = [WaterSample::default(); 64];
        let mut output = scratch;
        let mut reference = scratch;
        for us in [
            0, 1_000_000, 1_999_999, 2_000_000, 2_000_001, 3_000_000, 4_000_000, 6_000_000,
            8_000_000,
        ] {
            let t = SimTime(us);
            let f = Prepared::build(
                received.context(),
                &half,
                received.segments(),
                t,
                &mut field_pool,
            )
            .unwrap();
            let r = Prepared::build(
                direct.context(),
                &half,
                direct.segments(),
                t,
                &mut direct_pool,
            )
            .unwrap();
            f.sample_world_batch(&bound, &ctx, t, &points, 0.1, &mut scratch, &mut output)
                .unwrap();
            r.sample_world_batch(&bound, &ctx, t, &points, 0.1, &mut scratch, &mut reference)
                .unwrap();
            assert_eq!(
                output.map(|s| values(s).map(f32::to_bits)),
                reference.map(|s| values(s).map(f32::to_bits))
            );
            assert_eq!(f.energy_j().to_bits(), r.energy_j().to_bits());
            assert_eq!(f.power_w().to_bits(), r.power_w().to_bits());
            let before = output.map(|s| values(s).map(f32::to_bits));
            let mut invalid = points.clone();
            invalid[63] = WorldPos::from_units(i64::MIN, 0, 0);
            assert!(f
                .sample_world_batch(&bound, &ctx, t, &invalid, 0.1, &mut scratch, &mut output)
                .is_err());
            assert_eq!(output.map(|s| values(s).map(f32::to_bits)), before);
            // Restauration refusée sur d'autres pools ne perturbe ni le journal ni le champ actifs.
            let mut candidate_slots = [None; 1];
            let mut candidate_segments = path;
            assert!(Journal::restore_into(
                &snapshot[..227],
                11,
                &mut candidate_slots,
                &mut candidate_segments
            )
            .is_err());
            f.sample_world_batch(&bound, &ctx, t, &points, 0.1, &mut scratch, &mut output)
                .unwrap();
            assert_eq!(output.map(|s| values(s).map(f32::to_bits)), before);
            if us == 3_000_000 {
                println!("{radial}x{angular} hash3s={:016x}", digest(&output));
            }
        }
        let encode = measure_small(|| {
            black_box(direct.encode_into(black_box(&mut wire)).unwrap());
        });
        let mut decode_pool = path;
        let decode = measure_small(|| {
            let s = Source::decode_into(black_box(&wire), black_box(&mut decode_pool)).unwrap();
            black_box(s.metadata());
        });
        let save = measure_small(|| {
            black_box(journal.snapshot_into(black_box(&mut roundtrip)).unwrap());
        });
        let restore = measure_small(|| {
            let mut slots = [None; 1];
            let mut segments = path;
            let j =
                Journal::restore_into(black_box(&snapshot), 11, &mut slots, &mut segments).unwrap();
            black_box(j.pending());
        });
        let time = SimTime(3_000_000);
        let prepare = measure(|| {
            let f = Prepared::build(
                received.context(),
                &half,
                received.segments(),
                black_box(time),
                black_box(&mut field_pool),
            )
            .unwrap();
            black_box(f.energy_j());
        });
        let total = measure(|| {
            let mut slots = [None; 1];
            let mut segments = path;
            let mut j =
                Journal::restore_into(black_box(&snapshot), 11, &mut slots, &mut segments).unwrap();
            j.retry().unwrap();
            let s = j.current().unwrap().next().unwrap();
            let f = Prepared::build(
                s.context(),
                &half,
                s.segments(),
                black_box(time),
                black_box(&mut field_pool),
            )
            .unwrap();
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
        println!("{radial}x{angular} encode_us={encode:?} decode_us={decode:?} snapshot_us={save:?} restore_us={restore:?} prepare_us={prepare:?} restore_retry_prepare_query_us={total:?}");
    }
    assert_eq!(alloc.stats().refused_after_seal, 0);
}
