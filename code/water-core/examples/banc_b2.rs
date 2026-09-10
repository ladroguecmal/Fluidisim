//! S152 : volet impact de B2, profil local ; ni lambda_cut ni choix de technologie.
#[allow(dead_code)]
#[path="support/radial_reference.rs"] mod reference;
#[path="../../water-harness/src/host_impl.rs"] mod host_impl;
use water_core::{radial_impact::{Domain,RadialImpact},wave_event::{WaveEvent,Impact},FrameId,SimTime};
const TIMES: [u64;6]=[0,1_000_000,10_000_000,30_000_000,45_000_000,60_000_000];
fn event(lambda:f32)->WaveEvent {let v=*reference::event().data();WaveEvent::impact(Impact {wavelength_m:lambda,..v}).unwrap()}
fn domain()->Domain {Domain {radius:80.,age_us:60_000_000}}
fn points()->[[f32;2];17] {std::array::from_fn(|i| {
    let r=if i==0 {0.} else if i==16 {80.} else {80.*(i as f32-0.381966)/16.};
    [r,0.]
})}
fn admissible<const N:usize>(lambda:f32) {
    println!("admission lambda={lambda} N={N} {:?}",RadialImpact::<N>::new(event(lambda),reference::medium(),domain()).err());
}
fn receive<const N:usize>(lambda:f32, refs:&[([f64;7],[f64;7])]) {
    let f=match RadialImpact::<N>::new(event(lambda),reference::medium(),domain()) {Ok(f)=>f,Err(_)=>return};
    let mut error=0.0f64;let mut h=water_core::Hasher64::new();
    for (pi,p) in points().iter().enumerate() {for (ti,t) in TIMES.iter().enumerate() {
        let actual=reference::components(f.sample(FrameId(7),9,*p,SimTime(*t)).unwrap());
        let (expected,scale)=refs[pi*TIMES.len()+ti];
        for k in 0..7 {assert!(actual[k].is_finite());error=error.max((actual[k]-expected[k]).abs()/scale[k]);h.write_f32(actual[k] as f32);}
    }}
    println!("quality lambda={lambda} N={N} error={error:.9e} pass={} hash={:016x}",error<=1e-4,h.finish());
    assert!(error<=1e-4);
}
fn main() {
    if std::env::args().any(|a|a=="--cost-only") {for lambda in [2.,3.,4.,5.,6.] {cost::<256>(lambda);cost::<512>(lambda);}return;}
    let admission_only=std::env::args().any(|a|a=="--admission-only");
    let dirs=reference::directions(1024);let fine=reference::directions(2048);
    for lambda in [2.,3.,4.,5.,6.] {
        admissible::<64>(lambda);admissible::<128>(lambda);admissible::<256>(lambda);
        admissible::<512>(lambda);
        if admission_only {continue;}
        let mut refs=Vec::new();let mut radial=0.0f64;let mut angular=0.0f64;
        for p in points() {
            let a=reference::Reference::with_wavelength(p,512,&dirs,lambda);
            let b=reference::Reference::with_wavelength(p,1024,&dirs,lambda);
            let c=reference::Reference::with_wavelength(p,1024,&fine,lambda);
            for t in TIMES {let aa=a.at(t);let bb=b.at(t);let cc=c.at(t);
                for k in 0..7 {radial=radial.max((aa[k]-bb[k]).abs()/c.scales[k]);angular=angular.max((bb[k]-cc[k]).abs()/c.scales[k]);}
                refs.push((cc,c.scales));
            }
        }
        println!("oracle lambda={lambda} radial={radial:.9e} angular={angular:.9e}");
        assert!(radial<=1e-6 && angular<=1e-6);
        receive::<64>(lambda,&refs);receive::<128>(lambda,&refs);receive::<256>(lambda,&refs);
        receive::<512>(lambda,&refs);
    }
}

fn cost<const N:usize>(lambda:f32) {
    use std::{hint::black_box,time::Instant,mem::size_of};
    let field=match RadialImpact::<N>::new(event(lambda),reference::medium(),domain()) {Ok(f)=>f,Err(_)=>return};
    let points:[[f32;2];64]=std::array::from_fn(|i|[80.*i as f32/63.,0.]);
    let query=|| {for p in points {black_box(field.sample(FrameId(7),9,black_box(p),SimTime(30_000_000)).unwrap());}};
    for _ in 0..20 {query();}
    let mut medians=[0.;5];let mut tails=[0.;5];
    for run in 0..5 {let mut samples=[0.;101];for sample in &mut samples {
        let start=Instant::now();query();*sample=start.elapsed().as_secs_f64()*1e6;
    }samples.sort_by(f64::total_cmp);medians[run]=samples[50];tails[run]=samples[99];}
    let mean=medians.iter().sum::<f64>()/5.;let sd=(medians.iter().map(|v|(v-mean).powi(2)).sum::<f64>()/4.).sqrt();
    println!("cost lambda={lambda} N={N} bytes_field={} query64_p50_us={medians:?} query64_p99_us={tails:?} p50_sd={sd:.3}",size_of::<RadialImpact<N>>());
    restore_cost::<N>(lambda);
}
fn restore_cost<const N:usize>(lambda:f32) {
    use water_core::{wave_journal::{Journal,Cause},prepared_water::{LiveWater,Context}};
    use std::{hint::black_box,time::Instant};
    let context=Context {frame:FrameId(7),cell:9,medium:reference::medium(),domain:domain()};
    let mut a=[None];let mut b=[None];let mut fa=[const {None}];let mut fb=[const {None}];
    let mut j=Journal::new(1,&mut a);j.confirm(1,Cause {entity:1,command:1,emission:0},event(lambda)).unwrap();
    let source=LiveWater::<N>::build(j,Journal::new(1,&mut b),&mut fa,&mut fb,context).unwrap();
    let mut bytes=[0;289];assert_eq!(source.save(&mut bytes).unwrap(),289);
    let mut c=[None];let mut d=[None];let mut fc=[const {None}];let mut fd=[const {None}];
    let mut target=LiveWater::<N>::build(Journal::new(1,&mut c),Journal::new(1,&mut d),&mut fc,&mut fd,context).unwrap();
    let mut records=[None];target.restore(&bytes,&mut records).unwrap();assert_eq!(target.current().unwrap().field_count(),1);
    let mut check=[0;289];target.save(&mut check).unwrap();assert_eq!(check,bytes);
    // L'hôte restaure aussi T_sim=30s ; WLIV ne stocke pas de réalisation de B.
    let mut alloc=host_impl::ArenaAllocator::with_capacity(1<<20);
    let jobs=host_impl::SequentialJobs;let sink=host_impl::StderrSink;
    let bg=water_core::Background::configure(&mut water_core::HostServices {alloc:&mut alloc,jobs:&jobs,sink:&sink},
        water_core::SeaState {hs:0.,tp:6.,theta_turns:0.,components:32,graine:42},water_core::WorldPos::from_metres(0.,0.,0.)).unwrap();
    let bound=water_core::prepared_water::BoundBackground::new(&bg,FrameId(7),9);
    let points=points().map(|p|water_core::WorldPos::from_metres(p[0] as f64,p[1] as f64,0.));
    let mut expected=[water_core::WaterSample::default();17];let mut actual=expected;let mut scratch=expected;
    source.current().unwrap().sample_world_batch(&bound,&points,SimTime(30_000_000),0.1,&mut expected,&mut scratch).unwrap();
    target.current().unwrap().sample_world_batch(&bound,&points,SimTime(30_000_000),0.1,&mut actual,&mut scratch).unwrap();
    let bits=|s:water_core::WaterSample|[s.eta,s.deta_dt,s.steepness,s.aeration,s.normal[0],s.normal[1],s.normal[2],s.u_total[0],s.u_total[1],s.u_total[2]].map(f32::to_bits);
    assert_eq!(actual.map(bits),expected.map(bits));
    for _ in 0..20 {target.restore(&bytes,&mut records).unwrap();}
    let mut samples=[0.;505];for sample in &mut samples {let start=Instant::now();target.restore(black_box(&bytes),&mut records).unwrap();*sample=start.elapsed().as_secs_f64()*1e6;}
    samples.sort_by(f64::total_cmp);
    println!("restore lambda={lambda} N={N} WLIV_bytes=289 p50_us={:.3} p99_us={:.3} fields=1 snapshot_equal=true",samples[252],samples[499]);
}
