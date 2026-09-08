//! Scénario hôte S88. Mémoire persistée en bytes, pas de simulation de crash disque.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::{hint::black_box, time::Instant};
use water_core::*;
use water_core::{
    impact_field::Medium,
    prepared_water::{Admission, AdmissionError, BoundBackground, Context, LiveWater, Prepared},
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::{Cause, Change, Journal, Record},
};
fn event(id: u64, origin: Origin) -> WaveEvent {
    WaveEvent::impact(Impact {
        id,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(0),
        ttl_us: 4_000_000,
        position: [id as f32 * 0.25, 0.0, 0.0],
        energy_j: 0.01,
        wavelength_m: 4.0,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin,
        above_surface: true,
    })
    .unwrap()
}
fn cause(id: u64) -> Cause {
    Cause {
        entity: 1,
        command: id,
        emission: 0,
    }
}
fn confirm(id: u64) -> Admission {
    Admission::Confirm {
        epoch: 0,
        cause: cause(id),
        event: event(id, Origin::Server),
    }
}
fn context() -> Context {
    Context {
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
    }
}
fn fingerprint(samples: &[WaterSample]) -> u64 {
    let mut hash = Hasher64::new();
    for s in samples {
        for f in [
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
            hash.write_f32(f);
        }
    }
    hash.finish()
}
fn elapsed(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e6
}
fn run(bound: &BoundBackground<'_>, points: &[WorldPos; 64]) -> ([f64; 6], u64) {
    let mut times = [0.0; 6];
    let mut a = [None; 3];
    let mut b = [None; 3];
    let mut fa = [const { None }; 3];
    let mut fb = [const { None }; 3];
    let mut source = LiveWater::<128>::build(
        Journal::new(0, &mut a),
        Journal::new(0, &mut b),
        &mut fa,
        &mut fb,
        context(),
    )
    .unwrap();
    let mut out = [WaterSample::default(); 64];
    let mut scratch = out;
    let p1 = event(1, Origin::Prediction);
    let p2 = event(2, Origin::Prediction);
    let t = Instant::now();
    let changes = [
        source.update(
            Some(Admission::Predict {
                epoch: 0,
                cause: cause(1),
                event: p1,
            }),
            SimTime(0),
            4_000_000,
        ),
        source.update(Some(confirm(1)), SimTime(0), 4_000_000),
        source.update(
            Some(Admission::Predict {
                epoch: 0,
                cause: cause(2),
                event: p2,
            }),
            SimTime(0),
            4_000_000,
        ),
        source.update(
            Some(Admission::Reject {
                epoch: 0,
                cause: cause(2),
            }),
            SimTime(0),
            4_000_000,
        ),
        source.update(Some(confirm(3)), SimTime(0), 4_000_000),
    ];
    times[0] = elapsed(t);
    assert_eq!(
        changes,
        [
            Ok(Some(Change::Added)),
            Ok(Some(Change::Retract(p1))),
            Ok(Some(Change::Added)),
            Ok(Some(Change::Retract(p2))),
            Ok(Some(Change::Added))
        ]
    );
    let t = Instant::now();
    source
        .current()
        .unwrap()
        .sample_world_batch(
            bound,
            black_box(points),
            SimTime(1_000_000),
            1.0,
            &mut out,
            &mut scratch,
        )
        .unwrap();
    times[1] = elapsed(t);
    black_box(&out);
    let initial_hash = fingerprint(&out);
    assert_eq!(source.current().unwrap().field_count(), 2);
    let failure = source.update(Some(confirm(4)), SimTime(1_000_000), 4_000_000);
    assert!(matches!(
        failure,
        Err(AdmissionError::Journal(
            water_core::wave_journal::Error::Full
        ))
    ));
    assert!(matches!(source.current(), Err(AdmissionError::Pending)));
    let mut bytes = [0u8; 192 + 97 * 3];
    let t = Instant::now();
    let size = source.save(&mut bytes).unwrap();
    times[2] = elapsed(t);
    black_box(&bytes);
    assert_eq!(size, bytes.len());
    drop(source); // aucune poignée du service source n'est utilisée par la cible.

    let mut a = [None; 4];
    let mut b = [None; 4];
    let mut fa = [const { None }; 4];
    let mut fb = [const { None }; 4];
    let mut target = LiveWater::<128>::build(
        Journal::new(0, &mut a),
        Journal::new(0, &mut b),
        &mut fa,
        &mut fb,
        context(),
    )
    .unwrap();
    let mut records = [None; 4];
    let t = Instant::now();
    target.restore(black_box(&bytes), &mut records).unwrap();
    times[3] = elapsed(t);
    assert_eq!(target.pending(), Some(confirm(4)));
    assert!(target.current().is_err());
    let t = Instant::now();
    let applied = target.update(target.pending(), SimTime(12_000_000), 16_000_000);
    times[4] = elapsed(t);
    assert_eq!(applied, Ok(Some(Change::Added)));
    let t = Instant::now();
    target
        .current()
        .unwrap()
        .sample_world_batch(
            bound,
            black_box(points),
            SimTime(12_000_000),
            1.0,
            &mut out,
            &mut scratch,
        )
        .unwrap();
    times[5] = elapsed(t);
    black_box(&out);
    let final_hash = fingerprint(&out);
    assert_ne!(initial_hash, final_hash);

    // Référence construite directement depuis les faits confirmés, sans LiveWater ni WLIV.
    let mut records = [None; 3];
    let mut journal = Journal::new(0, &mut records);
    for id in [4, 1, 3] {
        journal
            .confirm(0, cause(id), event(id, Origin::Server))
            .unwrap();
    }
    let mut pool = [const { None }; 3];
    let mut ctx = context();
    ctx.domain.age_us = 16_000_000;
    let reference = Prepared::<128>::build(&journal, &mut pool, ctx).unwrap();
    let mut expected = [WaterSample::default(); 64];
    reference
        .sample_world_batch(
            bound,
            points,
            SimTime(12_000_000),
            1.0,
            &mut expected,
            &mut scratch,
        )
        .unwrap();
    assert_eq!(final_hash, fingerprint(&expected));
    for (a, b) in out.iter().zip(expected.iter()) {
        assert_eq!(a.eta.to_bits(), b.eta.to_bits());
        assert_eq!(a.deta_dt.to_bits(), b.deta_dt.to_bits());
        assert_eq!(a.normal, b.normal);
        assert_eq!(a.u_total, b.u_total);
    }
    // Échec tardif d'une requête monde : sortie entière conservée.
    let mut bad = *points;
    bad[63] = WorldPos::from_units(i64::MAX, 0, 0);
    let saved = fingerprint(&out);
    assert!(target
        .current()
        .unwrap()
        .sample_world_batch(
            bound,
            &bad,
            SimTime(12_000_000),
            1.0,
            &mut out,
            &mut scratch
        )
        .is_err());
    assert_eq!(saved, fingerprint(&out));
    (times, final_hash)
}
fn scenario(repeats: usize) {
    let mut allocator = host_impl::ArenaAllocator::with_capacity(1 << 20);
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let anchor = WorldPos::from_metres(1_000_000.0, 0.0, 0.0);
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
        anchor,
    )
    .unwrap();
    allocator.seal();
    let bound = BoundBackground::new(&bg, FrameId(0), 0);
    let points = std::array::from_fn(|i| {
        WorldPos::from_metres(
            1_000_000.0 + (i % 8) as f64 * 0.25,
            (i / 8) as f64 * 0.25,
            0.0,
        )
    });
    let mut samples = Vec::with_capacity(repeats);
    let mut hash = None;
    for i in 0..repeats + 3 {
        let (t, h) = run(&bound, &points);
        if let Some(old) = hash {
            assert_eq!(old, h);
        }
        hash = Some(h);
        if i >= 3 {
            samples.push(t);
        }
    }
    for (k, name) in [
        "admissions_5",
        "query_2x64",
        "save_blocked",
        "restore_2",
        "retry_and_extend_3",
        "query_3x64",
    ]
    .iter()
    .enumerate()
    {
        let mut v: Vec<f64> = samples.iter().map(|s| s[k]).collect();
        v.sort_by(f64::total_cmp);
        println!(
            "{name} us p50={:.3} p90={:.3} max={:.3}",
            v[repeats / 2],
            v[(repeats - 1) * 9 / 10],
            v[repeats - 1]
        );
    }
    println!(
        "hash={:016x} snapshot_bytes=483 N=128 points=64",
        hash.unwrap()
    );
    println!("target_journals_bytes={} target_fields_bytes={} restore_scratch_bytes={} query_buffers_bytes={}",
        8*std::mem::size_of::<Option<Record>>(), 8*std::mem::size_of::<Option<RadialImpact<128>>>(),
        4*std::mem::size_of::<Option<Record>>(), 128*std::mem::size_of::<WaterSample>());
}
fn main() {
    scenario(21);
}
#[test]
fn host_lifecycle_s88() {
    scenario(1);
}
