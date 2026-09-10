use super::*;
use crate::background_spectrum::{bake, Recipe, Error};
use crate::host::{Allocator, AllocStats, JobSystem, Sink};
struct Host;
impl Allocator for Host {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> { Ok(0) }
    fn seal(&mut self) {} fn is_sealed(&self) -> bool { false }
    fn stats(&self) -> AllocStats { AllocStats::default() }
}
impl Sink for Host { fn warn(&self,_:&str) {} fn metric(&self,_:&str,_:f64) {} }
impl JobSystem for Host {
    fn worker_count(&self)->u32 {1}
    fn parallel_reduce_ordered_f64(&self,n:usize,_:usize,r:&dyn Fn(usize,usize)->f64,m:&dyn Fn(f64,f64)->f64,v:f64)->f64 {m(v,r(0,n))}
}
fn recipe() -> Recipe { Recipe { sea:SeaState {hs:0.2,tp:6.0,theta_turns:0.0,components:32,graine:42},gravity:9.81,gamma:3.3,min_ratio:0.5,max_ratio:4.0,spread_turns:30.0/360.0} }
fn bg(r:Recipe) -> Background {
    let c=bake(r).unwrap(); let mut a=Host; let h=Host;
    Background::from_spectrum(&mut HostServices {alloc:&mut a,jobs:&h,sink:&h},&c,WorldPos::from_metres(0.,0.,0.)).unwrap()
}
#[test]
fn spectral_coefficients_match_independent_reference_s148() {
    let mut worst=0.0_f64;
    for gamma in [1.0,3.3,7.0] { for n in [32,64,128,256] {
        let mut r=recipe();r.gamma=gamma;r.sea.components=n;
        let c=bake(r).unwrap(); let mut m=[0.0;5];
        let mut peak=(0.0,0.0);
        for (i,node) in c.components().iter().enumerate() {
            let x=node.freq_q32 as f64/4294967296.0*r.sea.tp as f64;
            let lo=0.5*8.0_f64.powf(i as f64/n as f64);
            let hi=0.5*8.0_f64.powf((i+1) as f64/n as f64);
            let density=(node.amplitude as f64).powi(2)/(hi-lo);
            if density>peak.1 {peak=(x,density);}
            for p in [0,1,2,4] { m[p]+=0.5*(node.amplitude as f64).powi(2)*x.powi(p as i32); }
        }
        assert!((peak.0.ln()).abs()<8.0_f64.ln()/n as f64);
        assert!((4.0*m[0].sqrt()/r.sea.hs as f64-1.0).abs()<2e-6);
        let oracle=|p| spectrum_reference_s147::moment(gamma as f64,0.5,4.0,p,16384);
        for p in [1,2,4] { let error=(m[p]/m[0]/(oracle(p as i32)/oracle(0))-1.0).abs();worst=worst.max(error);assert!(error<0.002); }
        let d=c.diagnostics();
        for (p,v) in [(0,d.retained_m0),(2,d.retained_m2)] {
            let full=spectrum_reference_s147::moment(gamma as f64,0.125,1024.0,p,32768);
            assert!((v as f64-oracle(p)/full).abs()<2e-5);
        }
        assert_eq!(c.hash(),bake(r).unwrap().hash());
        println!("S148 gamma={gamma} N={n} hash={:016x}",c.hash());
    }}
    println!("S148 worst moment relative={worst:.9}");
}

#[test]
fn spectral_allocation_refusal_s148() {
    struct Sealed;
    impl Allocator for Sealed {
        fn alloc_persistent(&mut self,_:usize)->Result<usize,AllocError>{Err(AllocError::Sealed)}
        fn seal(&mut self){} fn is_sealed(&self)->bool{true}
        fn stats(&self)->AllocStats{AllocStats::default()}
    }
    let c=bake(recipe()).unwrap();let mut a=Sealed;let h=Host;
    assert!(matches!(Background::from_spectrum(&mut HostServices {alloc:&mut a,jobs:&h,sink:&h},&c,WorldPos::from_metres(0.,0.,0.)),Err(AllocError::Sealed)));
    assert_eq!(c.hash(),0x26695af7314e21db);
}
#[test]
fn spectral_refusals_and_replay_s148() {
    for v in [f32::NAN,f32::INFINITY,-1.0] {
        let mut r=recipe();r.sea.hs=v;assert!(matches!(bake(r),Err(Error::SeaState)));
        r=recipe();r.gravity=v;assert!(matches!(bake(r),Err(Error::Gravity)));
    }
    let mut r=recipe();r.gamma=8.0;assert!(matches!(bake(r),Err(Error::Gamma)));
    r=recipe();r.min_ratio=1.0;assert!(matches!(bake(r),Err(Error::Band)));
    r=recipe();r.sea.components=0;assert!(matches!(bake(r),Err(Error::Components)));
    r=recipe();r.sea.theta_turns=1.0;assert!(matches!(bake(r),Err(Error::Direction)));
    r=recipe();r.sea.tp=f32::MIN_POSITIVE;assert!(matches!(bake(r),Err(Error::NotRepresentable)));
    let original=bg(recipe());let restored=bg(recipe());
    let hash=original.conformance_hash(SimTime(123456789),12,2.0);
    assert_eq!(hash,0x2f32c548a0ff89d2);
    assert_eq!(hash,restored.conformance_hash(SimTime(123456789),12,2.0));
    r=recipe();r.sea.graine+=1;assert_ne!(hash,bg(r).conformance_hash(SimTime(123456789),12,2.0));
    r=recipe();r.sea.hs=0.0;assert_eq!(bg(r).eval(WorldPos::from_metres(0.,0.,0.),SimTime(0)).unwrap().eta,0.0);
    println!("S148 field hash={hash:016x}");
}

#[test]
fn spectral_wave_gravity_and_nonzero_impact_s148() {
    use crate::{FrameId, wave_event::{WaveEvent,Impact,Origin}, wave_journal::{Journal,Cause},
        radial_impact::{RadialImpact,Domain},impact_field::Medium,
        prepared_water::{Prepared,Context,BoundBackground,BatchError}};
    let e=WaveEvent::impact(Impact {id:1,frame:FrameId(0),cell:0,birth:SimTime(0),ttl_us:4000000,
        position:[0.;3],energy_j:0.01,wavelength_m:4.,direction_turns:0.,anisotropy:0.,
        displaced_l:0.,material:0,origin:Origin::Server,above_surface:true}).unwrap();
    let mut slots=[None;1];let mut j=Journal::new(1,&mut slots);
    j.confirm(1,Cause {entity:0,command:1,emission:0},e).unwrap();
    for gravity in [9.81,3.0] {
        let mut r=recipe();r.gravity=gravity;let b=bg(r);
        let medium=Medium {gravity,density:1025.,depth:20.,max_slope:1.};
        let domain=Domain {radius:16.,age_us:4000000};
        let f=RadialImpact::<64>::new(e,medium,domain).unwrap();
        let mut pool=[None];let p=Prepared::<64>::build(&j,&mut pool,Context {frame:FrameId(0),cell:0,medium,domain}).unwrap();
        let time=SimTime(1000000);let point=WorldPos::from_metres(1.,2.,0.);
        let mut out=[WaterSample::default()];let mut scratch=out;
        p.sample_world_batch(&BoundBackground::new(&b,FrameId(0),0),&[point],time,1.,&mut out,&mut scratch).unwrap();
        let base=b.eval(point,time).unwrap();let w=f.sample(FrameId(0),0,[1.,2.],time).unwrap();
        assert_eq!(out[0].eta.to_bits(),(base.eta+w.eta).to_bits());
        assert_ne!(out[0].eta.to_bits(),base.eta.to_bits());
        r.gravity=if gravity==3.0 {9.81} else {3.0};let incompatible=bg(r);
        let previous=out[0].eta.to_bits();
        assert_eq!(p.sample_world_batch(&BoundBackground::new(&incompatible,FrameId(0),0),&[point],time,1.,&mut out,&mut scratch),Err(BatchError::Context));
        assert_eq!(previous,out[0].eta.to_bits());
    }
}
#[test]
fn spectral_derivatives_and_composition_s148() {
    use crate::{wave_journal::Journal,composition,FrameId};
    let b=bg(recipe()); let mut storage=[];let journal=Journal::new(1,&mut storage);

    for time in [1000000,2000000,3000000] {
        let p=WorldPos::from_metres(1.,2.,0.);let s=b.eval(p,SimTime(time)).unwrap();
        let fd=(b.eval(p,SimTime(time+1000)).unwrap().eta-b.eval(p,SimTime(time-1000)).unwrap().eta)/0.002;
        assert!((fd-s.deta_dt).abs()<2e-5);
        assert_eq!(s.deta_dt.to_bits(),s.u_total[2].to_bits());
        let slope=[-s.normal[0]/s.normal[2],-s.normal[1]/s.normal[2]];
        assert!(slope[0].hypot(slope[1])<=s.steepness*core::f32::consts::PI+1e-6);
        let c=composition::compose::<64>(s,&journal,[],FrameId(0),0,[1.,2.],SimTime(time),1.0).unwrap();
        assert_eq!(c.eta.to_bits(),s.eta.to_bits());
    }
}
