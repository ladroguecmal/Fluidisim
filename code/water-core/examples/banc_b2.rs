//! S152 : volet impact de B2, profil local ; ni lambda_cut ni choix de technologie.
#[allow(dead_code)]
#[path="support/radial_reference.rs"] mod reference;
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
    let admission_only=std::env::args().any(|a|a=="--admission-only");
    let dirs=reference::directions(1024);let fine=reference::directions(2048);
    for lambda in [2.,3.,4.,5.,6.] {
        admissible::<64>(lambda);admissible::<128>(lambda);admissible::<256>(lambda);
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
    }
}
