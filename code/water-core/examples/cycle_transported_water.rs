//! S128 : cycle hôte du montage transporté S127 ; persistance en mémoire uniquement.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::{hint::black_box, mem::size_of, time::Instant};
use water_core::{
    composition,
    impact_field::{Error, Medium},
    prepared_water::{
        Admission, AdmissionError, BatchError, BoundBackground, Context, LiveWater, PrepareError,
        ServiceSnapshotError,
    },
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::{Cause, Change, Journal, Record},
    *,
};
const T4: u64 = 4_000_000;
const T24: u64 = 24_000_000;
const T48: u64 = 48_000_000;
const BYTES: usize = 192 + 97;
fn event() -> WaveEvent {
    WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(7),
        cell: 9,
        birth: SimTime(0),
        ttl_us: T4,
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
fn cause() -> Cause {
    Cause {
        entity: 1,
        command: 1,
        emission: 0,
    }
}
fn context(age_us: u64) -> Context {
    Context {
        frame: FrameId(7),
        cell: 9,
        medium: Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 20.0,
            max_slope: 0.1,
        },
        domain: Domain {
            radius: 80.0,
            age_us,
        },
    }
}
struct Pools<const N: usize> {
    journals: [[Option<Record>; 1]; 2],
    fields: [[Option<RadialImpact<N>>; 1]; 2],
}
impl<const N: usize> Pools<N> {
    fn new() -> Self {
        Self {
            journals: [[None; 1]; 2],
            fields: [const { [const { None }; 1] }; 2],
        }
    }
    fn service(&mut self) -> LiveWater<'_, N> {
        let [a, b] = &mut self.journals;
        let [fa, fb] = &mut self.fields;
        LiveWater::build(Journal::new(0, a), Journal::new(0, b), fa, fb, context(T4)).unwrap()
    }
}
fn bits(s: WaterSample) -> [u32; 10] {
    let v = [
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
    ];
    assert!(v.iter().all(|x| x.is_finite()));
    v.map(f32::to_bits)
}
fn equal(a: &[WaterSample], b: &[WaterSample]) {
    assert_eq!(a.len(), b.len());
    for (i, (x, y)) in a.iter().zip(b).enumerate() {
        assert_eq!(bits(*x), bits(*y), "point {i}");
    }
}
fn fingerprint(out: &[WaterSample]) -> u64 {
    let mut h = Hasher64::new();
    for s in out {
        for b in bits(*s) {
            h.write_f32(f32::from_bits(b));
        }
    }
    h.finish()
}
fn query(
    service: &LiveWater<'_, 256>,
    bound: &BoundBackground<'_>,
    points: &[WorldPos],
    us: u64,
    out: &mut [WaterSample],
    scratch: &mut [WaterSample],
) -> Result<usize, BatchError> {
    service
        .current()
        .unwrap()
        .sample_world_batch(bound, points, SimTime(us), 0.1, out, scratch)
}
fn snapshot(service: &LiveWater<'_, 256>) -> [u8; BYTES] {
    let mut bytes = [0; BYTES];
    assert_eq!(service.save(&mut bytes), Ok(BYTES));
    bytes
}
fn direct(background: &Background, points: &[WorldPos; 64], us: u64) -> [WaterSample; 64] {
    let mut records = [None];
    let mut journal = Journal::new(0, &mut records);
    journal.confirm(0, cause(), event()).unwrap();
    let f = RadialImpact::<256>::new(event(), context(T48).medium, context(T48).domain).unwrap();
    std::array::from_fn(|i| {
        let local = background.local_point(points[i]).unwrap();
        composition::compose(
            background.eval(points[i], SimTime(us)).unwrap(),
            &journal,
            [&f],
            FrameId(7),
            9,
            [local[0], local[1]],
            SimTime(us),
            0.1,
        )
        .unwrap()
    })
}
fn check(
    service: &LiveWater<'_, 256>,
    bg: &Background,
    bound: &BoundBackground<'_>,
    points: &[WorldPos; 64],
    us: u64,
) -> u64 {
    let sentinel = WaterSample {
        eta: 123.0,
        ..WaterSample::default()
    };
    let mut out = [sentinel; 65];
    let mut scratch = [WaterSample::default(); 64];
    assert_eq!(
        query(service, bound, points, us, &mut out, &mut scratch),
        Ok(64)
    );
    assert_eq!(bits(out[64]), bits(sentinel));
    equal(&out[..64], &direct(bg, points, us));
    fingerprint(&out[..64])
}
fn receive(
    bg: &Background,
    bound: &BoundBackground<'_>,
    points: &[WorldPos; 64],
) -> [[u8; BYTES]; 3] {
    let mut pools = Pools::<256>::new();
    let mut source = pools.service();
    assert_eq!(
        source.update(
            Some(Admission::Confirm {
                epoch: 0,
                cause: cause(),
                event: event()
            }),
            SimTime(0),
            T4
        ),
        Ok(Some(Change::Added))
    );
    let mut compared = 0;
    for t in [0, T4] {
        check(&source, bg, bound, points, t);
        compared += 64;
    }
    let s4 = snapshot(&source);
    let mut out = direct(bg, points, T4);
    let mut scratch = out;
    let saved = out;
    assert_eq!(
        query(&source, bound, points, T4 + 1, &mut out, &mut scratch),
        Err(BatchError::Point {
            index: 0,
            error: composition::Error::Domain
        })
    );
    equal(&out, &saved);
    assert_eq!(source.update(None, SimTime(T4 + 1), T24), Ok(None));
    for t in [T4, T4 + 1, 12_137_119, T24] {
        check(&source, bg, bound, points, t);
        compared += 64;
    }
    assert_eq!(
        source.current().unwrap().renewal_deadline(),
        Some((1, SimTime(T24)))
    );
    let s24 = snapshot(&source);
    // Détruire le service source : la reprise ne peut pas relire ses champs.
    drop(source);
    let mut restored_pools = Pools::<256>::new();
    let mut restored = restored_pools.service();
    let mut restore_scratch = [None];
    restored.restore(&s24, &mut restore_scratch).unwrap();
    assert_eq!(snapshot(&restored), s24);
    for t in [0, T4, T4 + 1, 12_137_119, T24] {
        check(&restored, bg, bound, points, t);
        compared += 64;
    }
    assert_eq!(restored.update(None, SimTime(T24 + 1), T48), Ok(None));
    let mut hashes = [0; 3];
    for (i, t) in [T24 + 1, 36_271_829, T48].iter().copied().enumerate() {
        hashes[i] = check(&restored, bg, bound, points, t);
        compared += 64;
    }
    let s48 = snapshot(&restored);
    assert_eq!(u32::from_le_bytes(s48[8..12].try_into().unwrap()), 256);
    assert_eq!(u64::from_le_bytes(s48[48..56].try_into().unwrap()), T48);
    // Le refus numérique d'extension garde exactement la sauvegarde publiée.
    assert_eq!(
        restored.update(None, SimTime(T48), 64_000_000),
        Err(AdmissionError::Prepare(PrepareError::Field {
            id: 1,
            error: Error::Resolution
        }))
    );
    assert_eq!(snapshot(&restored), s48);
    assert_eq!(restored.pending(), None);
    assert_eq!(
        restored.update(None, SimTime(T48 + 1), T48),
        Err(AdmissionError::Horizon)
    );
    assert_eq!(snapshot(&restored), s48);
    out = direct(bg, points, T48);
    let saved = out;
    let mut invalid = *points;
    invalid[63] = WorldPos::from_metres(1_000_081.0, 0.0, 0.0);
    assert_eq!(
        query(&restored, bound, &invalid, T48, &mut out, &mut scratch),
        Err(BatchError::Point {
            index: 63,
            error: composition::Error::Domain
        })
    );
    equal(&out, &saved);
    assert_eq!(
        query(&restored, bound, points, T48 + 1, &mut out, &mut scratch),
        Err(BatchError::Point {
            index: 0,
            error: composition::Error::Domain
        })
    );
    equal(&out, &saved);
    assert!(restored
        .restore(&s48[..BYTES - 1], &mut restore_scratch)
        .is_err());
    assert_eq!(snapshot(&restored), s48);
    check(&restored, bg, bound, points, T48);
    compared += 64;
    // N différent : refus même avant la reconstruction, service cible vide inchangé.
    let mut wrong_pools = Pools::<128>::new();
    let mut wrong = wrong_pools.service();
    let mut before = [0; 192];
    wrong.save(&mut before).unwrap();
    assert_eq!(
        wrong.restore(&s48, &mut restore_scratch),
        Err(ServiceSnapshotError::Context)
    );
    let mut after = [0; 192];
    wrong.save(&mut after).unwrap();
    assert_eq!(before, after);
    drop(restored);
    let mut final_pools = Pools::<256>::new();
    let mut final_service = final_pools.service();
    final_service.restore(&s48, &mut restore_scratch).unwrap();
    assert_eq!(snapshot(&final_service), s48);
    for t in [T48, T24, T4, 0, T48] {
        check(&final_service, bg, bound, points, t);
        compared += 64;
    }
    assert_ne!(hashes[0], hashes[2]);
    println!("reception: {compared} points-temps, dix composantes identiques en bits ; refus atomiques recus");
    println!(
        "hash t24+1={:016x} t36.271829={:016x} t48={:016x}; WLIV={BYTES} octets",
        hashes[0], hashes[1], hashes[2]
    );
    // Témoin : la présence de W est visible en bits après le TTL.
    let at48 = direct(bg, points, T48);
    let changed = points
        .iter()
        .enumerate()
        .filter(|(i, p)| bits(at48[*i]) != bits(bg.eval(**p, SimTime(T48)).unwrap()))
        .count();
    assert!(changed > 0);
    println!("points differents de B seul a48 s: {changed}/64");
    [s4, s24, s48]
}
fn micros(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1e6
}
fn measure(
    op: usize,
    source: &mut LiveWater<'_, 256>,
    target: &mut LiveWater<'_, 256>,
    bound: &BoundBackground<'_>,
    points: &[WorldPos; 64],
    snapshots: &[[u8; BYTES]; 3],
) -> f64 {
    let mut records = [None];
    let mut out = [WaterSample::default(); 64];
    let mut scratch = out;
    let mut bytes = [0; BYTES];
    // Réinitialisation hors chrono des opérations isolées. Pools et B déjà construits.
    source
        .restore(
            &snapshots[if op == 0 || op == 4 { 0 } else { 2 }],
            &mut records,
        )
        .unwrap();
    let start = Instant::now();
    match op {
        0 => {
            source
                .update(None, black_box(SimTime(T24)), black_box(T24))
                .unwrap();
        }
        1 => {
            query(
                source,
                bound,
                black_box(points),
                black_box(T48),
                &mut out,
                &mut scratch,
            )
            .unwrap();
        }
        2 => {
            source.save(black_box(&mut bytes)).unwrap();
        }
        3 => {
            target
                .restore(black_box(&snapshots[2]), &mut records)
                .unwrap();
        }
        4 => {
            // Cycle de trois requêtes : renouvellement24, puis48, sauvegarde et reprise48.
            source.update(None, SimTime(T24), T24).unwrap();
            query(
                source,
                bound,
                black_box(points),
                T24,
                &mut out,
                &mut scratch,
            )
            .unwrap();
            source.update(None, SimTime(T48), T48).unwrap();
            query(
                source,
                bound,
                black_box(points),
                T48,
                &mut out,
                &mut scratch,
            )
            .unwrap();
            source.save(&mut bytes).unwrap();
            target.restore(&bytes, &mut records).unwrap();
            query(
                target,
                bound,
                black_box(points),
                T48,
                &mut out,
                &mut scratch,
            )
            .unwrap();
        }
        _ => unreachable!(),
    }
    let elapsed = micros(start);
    black_box(out);
    black_box(bytes);
    black_box(source.current().unwrap().field_count());
    elapsed
}
fn main() {
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
        WorldPos::from_metres(1_000_000.0, 0.0, 0.0),
    )
    .unwrap();
    allocator.seal();
    let bound = BoundBackground::new(&bg, FrameId(7), 9);
    let points = std::array::from_fn(|i| {
        let r = if i >= 62 {
            80.0
        } else {
            80.0 * i as f64 / 62.0
        };
        let d = if i == 62 {
            [1.0, 0.0]
        } else if i == 63 {
            [0.0, -1.0]
        } else {
            [[1.0, 0.0], [0.0, -1.0], [-0.6, 0.8], [-0.8, -0.6]][i % 4]
        };
        WorldPos::from_metres(1_000_000.0 + r * d[0], r * d[1], 0.0)
    });
    let snapshots = receive(&bg, &bound, &points);
    let mut source_pools = Pools::<256>::new();
    let mut source = source_pools.service();
    let mut target_pools = Pools::<256>::new();
    let mut target = target_pools.service();
    let warm = Instant::now();
    while warm.elapsed().as_secs_f64() < 1.0 {
        for op in 0..5 {
            black_box(measure(
                op,
                &mut source,
                &mut target,
                &bound,
                &points,
                &snapshots,
            ));
        }
    }
    let mut results = [[0.0; 15]; 5];
    for block in 0..15 {
        for offset in 0..5 {
            let op = if block % 2 == 0 { offset } else { 4 - offset };
            for _ in 0..32 {
                results[op][block] +=
                    measure(op, &mut source, &mut target, &bound, &points, &snapshots) / 32.0;
            }
        }
    }
    for (op, name) in [
        "renouvellement4vers24",
        "requete64_a48",
        "sauvegarde48",
        "restauration48",
        "cycle24_48_reprise48",
    ]
    .iter()
    .enumerate()
    {
        results[op].sort_by(f64::total_cmp);
        println!(
            "{name} us min/med/max = {:.4}/{:.4}/{:.4}",
            results[op][0], results[op][7], results[op][14]
        );
    }
    println!("octets: pools_service={} champs_service={} journaux_service={} scratch_restore={} deux_buffers_requete={}",
        size_of::<Pools<256>>(),2*size_of::<Option<RadialImpact<256>>>(),2*size_of::<Option<Record>>(),
        size_of::<Option<Record>>(),128*size_of::<WaterSample>());
}
