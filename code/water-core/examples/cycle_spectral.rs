//! S149 : recette WSPR + WLIV, ancre et contexte hôte conservés en mémoire.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::{hint::black_box, time::Instant};
use water_core::{
    composition,
    impact_field::Medium,
    prepared_water::{
        Admission, BatchError, BoundBackground, Context, LiveWater,
        ServiceSnapshotError,
    },
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::{Cause, Journal, Record},
    *,
};
const T4: u64 = 4_000_000;


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
    let f = RadialImpact::<256>::new(event(), context(T4).medium, context(T4).domain).unwrap();
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
fn background(c: &background_spectrum::Cooked, anchor: WorldPos) -> Background {
    let mut a=host_impl::ArenaAllocator::with_capacity(1<<20);
    let jobs=host_impl::SequentialJobs;let sink=host_impl::StderrSink;
    Background::from_spectrum(&mut HostServices {alloc:&mut a,jobs:&jobs,sink:&sink},c,anchor).unwrap()
}
fn main() {
    use background_spectrum::{bake,decode,Recipe};
    let anchor=WorldPos::from_metres(1_000_000.,0.,0.);
    let points=std::array::from_fn(|i| WorldPos::from_metres(1_000_000.+i as f64/8.,2.,0.));
    let times=[0,1_000_000,3_000_000,4_000_000];
    // Les objets sources sont détruits à la sortie du bloc. Seules les charges et
    // les métadonnées hôte (ancre, contexte, temps) traversent la restauration.
    let (recipe_bytes,live_bytes,expected)={
        let c=bake(Recipe {sea:SeaState {hs:0.2,tp:6.,theta_turns:0.,components:32,graine:42},
            gravity:9.81,gamma:3.3,min_ratio:0.5,max_ratio:4.,spread_turns:30./360.}).unwrap();
        let bg=background(&c,anchor);let bound=BoundBackground::new(&bg,FrameId(7),9);
        let mut pools=Pools::<256>::new();let mut live=pools.service();
        live.update(Some(Admission::Confirm {epoch:0,cause:cause(),event:event()}),SimTime(0),T4).unwrap();
        let hashes=times.map(|t| check(&live,&bg,&bound,&points,t));
        (c.encode(),snapshot(&live),hashes)
    };
    let c=decode(&recipe_bytes).unwrap();let bg=background(&c,anchor);
    let bound=BoundBackground::new(&bg,FrameId(7),9);
    let mut pools=Pools::<256>::new();let mut live=pools.service();let mut scratch_records=[None];
    live.restore(&live_bytes,&mut scratch_records).unwrap();
    assert_eq!(snapshot(&live),live_bytes);
    for (t,hash) in times.into_iter().zip(expected) { assert_eq!(check(&live,&bg,&bound,&points,t),hash); }
    let mut bad=live_bytes;bad[12]^=1;
    assert_eq!(live.restore(&bad,&mut scratch_records),Err(ServiceSnapshotError::Context));
    assert_eq!(snapshot(&live),live_bytes);
    println!("S149 recipe={} WLIV={} hashes={expected:016x?}",recipe_bytes.len(),live_bytes.len());
    if std::env::args().any(|a| a=="--verify-only") {return;}
    let mut out=[WaterSample::default();64];let mut scratch=out;
    // Échauffement, puis 15 blocs ; opérations mesurées séparément.
    for _ in 0..100 {black_box(decode(black_box(&recipe_bytes)).unwrap());query(&live,&bound,&points,1_000_000,&mut out,&mut scratch).unwrap();}
    for op in 0..3 {
        let mut samples=[0.;15];
        for sample in &mut samples {
            let start=Instant::now();
            for _ in 0..32 {match op {
                0=>{black_box(decode(black_box(&recipe_bytes)).unwrap());},
                1=>{live.restore(black_box(&live_bytes),&mut scratch_records).unwrap();},
                _=>{query(&live,&bound,black_box(&points),1_000_000,&mut out,&mut scratch).unwrap();black_box(&out);}
            }}
            *sample=start.elapsed().as_secs_f64()*1e6/32.;
        }
        samples.sort_by(f64::total_cmp);
        println!("{} median_us={:.3} min={:.3} max={:.3}",["decode_recuisson_B","restore_WLIV","B32_W256_64points"][op],samples[7],samples[0],samples[14]);
    }
}
