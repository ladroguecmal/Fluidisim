//! S114 : reprise multisource, refus transactionnels et cycle B+pression.
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
fn segment_bits(s: Segment) -> [u64; 7] {
    [
        s.birth.0,
        s.duration_us,
        s.origin[0].to_bits() as u64,
        s.origin[1].to_bits() as u64,
        s.velocity[0].to_bits() as u64,
        s.velocity[1].to_bits() as u64,
        s.pressure_pa.to_bits() as u64,
    ]
}
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
    let paths = [
        [a],
        [Segment {
            birth: SimTime(500_000),
            origin: [1.0, 1.0],
            velocity: [0.0, 2.0],
            pressure_pa: 7.0,
            ..a
        }],
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
    let mut times: Vec<u64> = (0..=16).map(|i| i * 500_000).collect();
    for t in [500_000, 2_000_000, 2_500_000] {
        times.extend([t - 1, t + 1]);
    }
    times.sort_unstable();
    times.dedup();
    for (radial, angular) in [(224, 128), (256, 128)] {
        let recipe = Recipe {
            sigma: 1.0,
            cutoff: 6.0,
            radial,
            angular,
        };
        let settings = Settings {
            frame: FrameId(7),
            cell: 9,
            gravity: 9.81,
            density: 1025.0,
            min: [-8.0; 2],
            max: [12.0; 2],
            start: SimTime(0),
            end: SimTime(8_000_000),
        };
        let meta = |id| Metadata {
            epoch: 11,
            id,
            cause: Cause {
                entity: id,
                command: 44,
                emission: 0,
            },
            settings,
            recipe,
        };
        let direct = [
            Source::new(meta(1), &paths[0]).unwrap(),
            Source::new(meta(2), &paths[1]).unwrap(),
        ];
        let mut wire = [[0; 156]; 2];
        for i in 0..2 {
            assert_eq!(direct[i].encode_into(&mut wire[i]), Ok(156));
        }
        let mut decoded_a = [a];
        let mut decoded_b = [a];
        let decoded = [
            Source::decode_into(&wire[0], &mut decoded_a).unwrap(),
            Source::decode_into(&wire[1], &mut decoded_b).unwrap(),
        ];
        let mut limited = [None; 1];
        let mut journal = Journal::new(11, &mut limited);
        assert_eq!(journal.admit_authenticated(decoded[1]), Ok(Change::Added));
        assert_eq!(
            journal.admit_authenticated(decoded[0]),
            Err(JournalError::Full)
        );
        assert_eq!(journal.published().next().unwrap().metadata().id, 2);
        assert_eq!(journal.pending().unwrap().metadata().id, 1);
        let mut snapshot = [0; 344];
        assert_eq!(journal.snapshot_into(&mut snapshot), Ok(344));
        // Reconstruction sur un pool encore plein : aucune admission implicite.
        let mut tight_slots = [None; 1];
        let mut tight_segments = [a; 2];
        let mut tight =
            Journal::restore_into(&snapshot, 11, &mut tight_slots, &mut tight_segments).unwrap();
        assert_eq!(tight.retry(), Err(JournalError::Full));
        let mut roundtrip = [0; 344];
        tight.snapshot_into(&mut roundtrip).unwrap();
        assert_eq!(snapshot, roundtrip);
        let mut slots = [None; 2];
        let mut restored_segments = [a; 2];
        let mut restored =
            Journal::restore_into(&snapshot, 11, &mut slots, &mut restored_segments).unwrap();
        assert_eq!(restored.published().count(), 1);
        assert!(restored.current().is_err());
        restored.snapshot_into(&mut roundtrip).unwrap();
        assert_eq!(snapshot, roundtrip);
        // Cuisson reconstruite depuis la source restaurée, non depuis un état modal sauvé.
        let received = restored.published().next().unwrap();
        let mut nodes = vec![Node::default(); radial * angular];
        let mut reduced = vec![Node::default(); radial * angular / 2];
        let spectrum = bake(received.metadata().recipe, &mut nodes).unwrap();
        let half = spectrum.half_into(&mut reduced).unwrap();
        let ctx = Context::new(received.metadata().settings, &half).unwrap();
        let mut field_pool = vec![Slot::default(); half.nodes().len()];
        let mut direct_pool = field_pool.clone();
        let mut candidate_pool = field_pool.clone();
        assert!(matches!(
            Prepared::from_journal(ctx, &half, &restored, SimTime(1_500_000), &mut field_pool),
            Err(bound_pressure::Error::Pending)
        ));
        assert_eq!(restored.retry(), Ok(Change::Added));
        assert_eq!(restored.retry(), Ok(Change::Unchanged));
        let sources: Vec<_> = restored.current().unwrap().collect();
        assert_eq!(
            sources.iter().map(|s| s.metadata().id).collect::<Vec<_>>(),
            [1, 2]
        );
        for i in 0..2 {
            assert!(sources[i].same_content(&direct[i]));
        }
        // Arrivée directe dans l'ordre contraire à la publication restaurée.
        let mut direct_slots = [None; 2];
        let mut direct_journal = Journal::new(11, &mut direct_slots);
        for s in direct.into_iter().rev() {
            direct_journal.admit_authenticated(s).unwrap();
        }
        let mut confirmed = [0; 344];
        restored.snapshot_into(&mut confirmed).unwrap();
        direct_journal.snapshot_into(&mut roundtrip).unwrap();
        assert_eq!(confirmed, roundtrip);
        let marker = Segment {
            pressure_pa: -123.0,
            ..a
        };
        // Toute troncature, puis corruption tardive de la source en attente : deux pools intacts.
        let rejected = |bytes: &[u8], epoch| {
            let mut cs = [Some(direct[0]); 2];
            let mut cp = [marker; 2];
            assert!(Journal::restore_into(bytes, epoch, &mut cs, &mut cp).is_err());
            assert!(cs.iter().all(|s| s.unwrap().same_content(&direct[0])));
            assert_eq!(cp.map(segment_bits), [marker; 2].map(segment_bits));
        };
        for end in 0..snapshot.len() {
            rejected(&snapshot[..end], 11);
        }
        let mut corrupt = snapshot;
        corrupt[343] = 1; // réservé du dernier segment WPRS, après le premier événement valide.
        rejected(&corrupt, 11);
        rejected(&snapshot, 12);
        let mut cs = [Some(direct[0]); 2];
        let mut cp = [marker; 1];
        assert!(Journal::restore_into(&snapshot, 11, &mut cs, &mut cp).is_err());
        assert!(cs.iter().all(|s| s.unwrap().same_content(&direct[0])));
        assert_eq!(cp.map(segment_bits), [marker].map(segment_bits));
        let mut no_slots = [];
        let mut cp = [marker; 2];
        assert!(Journal::restore_into(&snapshot, 11, &mut no_slots, &mut cp).is_err());
        assert_eq!(cp.map(segment_bits), [marker; 2].map(segment_bits));
        let mut scratch = [WaterSample::default(); 64];
        let mut output = scratch;
        let mut reference = scratch;
        let mut all_times = Hasher64::new();
        let mut midpoint = None;
        for &us in &times {
            let t = SimTime(us);
            let f = Prepared::from_journal(ctx, &half, &restored, t, &mut field_pool).unwrap();
            let r =
                Prepared::from_journal(ctx, &half, &direct_journal, t, &mut direct_pool).unwrap();
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
            assert_eq!(f.slope_envelope().to_bits(), r.slope_envelope().to_bits());
            for s in output {
                for v in values(s) {
                    assert!(v.is_finite());
                    all_times.write_f32(v);
                }
            }
            all_times.write_f32(f.energy_j());
            all_times.write_f32(f.power_w());
            if us == 1_500_000 {
                let before = output.map(|s| values(s).map(f32::to_bits));
                let mut invalid = points.clone();
                invalid[63] = WorldPos::from_units(i64::MIN, 0, 0);
                assert!(f
                    .sample_world_batch(&bound, &ctx, t, &invalid, 0.1, &mut scratch, &mut output)
                    .is_err());
                assert_eq!(output.map(|s| values(s).map(f32::to_bits)), before);
                assert!(f
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
                assert_eq!(output.map(|s| values(s).map(f32::to_bits)), before);
                assert!(f
                    .sample_world_batch(&bound, &ctx, t, &points, 1e-8, &mut scratch, &mut output)
                    .is_err());
                assert_eq!(output.map(|s| values(s).map(f32::to_bits)), before);
                // Refus de préparation d'un autre journal restauré : contexte valide mais incompatible.
                let mut changed = snapshot;
                changed[240..244].copy_from_slice(&8u32.to_le_bytes()); // frame de la source en attente.
                let mut ss = [None; 2];
                let mut sp = [a; 2];
                let mut wrong = Journal::restore_into(&changed, 11, &mut ss, &mut sp).unwrap();
                wrong.retry().unwrap();
                assert!(matches!(
                    Prepared::from_journal(ctx, &half, &wrong, t, &mut candidate_pool),
                    Err(bound_pressure::Error::Context)
                ));
                let mut huge = snapshot;
                huge[336..340].copy_from_slice(&f32::MAX.to_le_bytes());
                let mut hs = [None; 2];
                let mut hp = [a; 2];
                let mut overflow = Journal::restore_into(&huge, 11, &mut hs, &mut hp).unwrap();
                overflow.retry().unwrap();
                assert!(matches!(
                    Prepared::from_journal(ctx, &half, &overflow, t, &mut candidate_pool),
                    Err(bound_pressure::Error::Preparation(_))
                ));
                rejected(&corrupt, 11);
                // Témoin nominal : la publication active reste utilisable après les refus.
                f.sample_world_batch(&bound, &ctx, t, &points, 0.1, &mut scratch, &mut output)
                    .unwrap();
                assert_eq!(output.map(|s| values(s).map(f32::to_bits)), before);
                println!("{radial}x{angular} hash1_5s={:016x}", digest(&output));
                midpoint = Some(output.map(|s| values(s).map(f32::to_bits)));
            }
        }
        println!(
            "{radial}x{angular} points_times={} all_times_hash={:016x} snapshot_bytes={}",
            points.len() * times.len(),
            all_times.finish(),
            snapshot.len()
        );
        let save = measure_small(|| {
            black_box(journal.snapshot_into(black_box(&mut roundtrip)).unwrap());
        });
        let restore = measure_small(|| {
            let mut ss = [None; 2];
            let mut sp = [a; 2];
            let j = Journal::restore_into(black_box(&snapshot), 11, &mut ss, &mut sp).unwrap();
            black_box(j.pending());
        });
        let time = SimTime(1_500_000);
        let prepare = measure(|| {
            let f = Prepared::from_journal(
                ctx,
                &half,
                &restored,
                black_box(time),
                black_box(&mut field_pool),
            )
            .unwrap();
            black_box((f.energy_j(), f.power_w()));
        });
        let resume = measure(|| {
            let mut ss = [None; 2];
            let mut sp = [a; 2];
            let mut j = Journal::restore_into(black_box(&snapshot), 11, &mut ss, &mut sp).unwrap();
            j.retry().unwrap();
            let f = Prepared::from_journal(ctx, &half, &j, time, &mut field_pool).unwrap();
            f.sample_world_batch(
                &bound,
                &ctx,
                time,
                black_box(&points),
                0.1,
                &mut scratch,
                &mut output,
            )
            .unwrap();
            black_box((&output, f.energy_j(), f.power_w()));
        });
        assert_eq!(
            output.map(|s| values(s).map(f32::to_bits)),
            midpoint.unwrap()
        );
        let end_to_end = measure(|| {
            for i in 0..2 {
                direct[i].encode_into(black_box(&mut wire[i])).unwrap();
            }
            let mut da = [a];
            let mut db = [a];
            let sa = Source::decode_into(black_box(&wire[0]), &mut da).unwrap();
            let sb = Source::decode_into(black_box(&wire[1]), &mut db).unwrap();
            let mut ls = [None; 1];
            let mut j = Journal::new(11, &mut ls);
            j.admit_authenticated(sb).unwrap();
            assert_eq!(j.admit_authenticated(sa), Err(JournalError::Full));
            let mut bytes = [0; 344];
            j.snapshot_into(&mut bytes).unwrap();
            let mut ss = [None; 2];
            let mut sp = [a; 2];
            let mut j = Journal::restore_into(black_box(&bytes), 11, &mut ss, &mut sp).unwrap();
            j.retry().unwrap();
            let f = Prepared::from_journal(ctx, &half, &j, time, &mut field_pool).unwrap();
            f.sample_world_batch(
                &bound,
                &ctx,
                time,
                black_box(&points),
                0.1,
                &mut scratch,
                &mut output,
            )
            .unwrap();
            black_box((&output, f.energy_j(), f.power_w()));
        });
        assert_eq!(
            output.map(|s| values(s).map(f32::to_bits)),
            midpoint.unwrap()
        );
        println!("{radial}x{angular} snapshot_us={save:?} restore_us={restore:?} prepare_us={prepare:?} resume_query_us={resume:?} end_to_end_us={end_to_end:?}");
    }
    assert_eq!(alloc.stats().refused_after_seal, 0);
}
