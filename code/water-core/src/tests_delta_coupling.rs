use super::*;
use crate::host::{AllocError, AllocStats, Allocator, HostServices, Sink};
#[derive(Default)]
struct Arena;
impl Allocator for Arena {
    fn alloc_persistent(&mut self,_:usize)->Result<usize,AllocError>{Ok(0)}
    fn seal(&mut self){}
    fn is_sealed(&self)->bool{false}
    fn stats(&self)->AllocStats{AllocStats::default()}
}
struct Jobs;
impl Sink for Jobs {fn warn(&self,_:&str){} fn metric(&self,_:&str,_:f64){}}
impl JobSystem for Jobs {
    fn worker_count(&self)->u32{1}
    fn parallel_reduce_ordered_f64(&self,n:usize,_:usize,r:&dyn Fn(usize,usize)->f64,
        m:&dyn Fn(f64,f64)->f64,init:f64)->f64{m(init,r(0,n))}
}
struct Clock;
impl MonotonicClock for Clock {fn now_ns(&self)->u64{0}}
fn volume()->Volume {
    Volume::configure(&mut HostServices{alloc:&mut Arena,jobs:&Jobs,sink:&Jobs},
        Domain{nx:16,nz:8,dx:0.5},1025.,9.81,&[0.;16]).unwrap()
}
fn fields(v:&Volume)->(Vec<BackgroundSample>,Vec<BackgroundSample>){
    (vec![BackgroundSample::default();v.u.len()],vec![BackgroundSample::default();v.w.len()])
}
fn input<'a>(v:&Volume,u:&'a [BackgroundSample],w:&'a [BackgroundSample])->BackgroundFaces<'a>{
    BackgroundFaces{domain:v.domain,time:SimTime(0),density:1025.,gravity:9.81,u,w}
}
fn run(v:&mut Volume,bg:&BackgroundFaces<'_>,s:Sponge)->Result<SurfaceReport,Error>{
    v.step_perturbation(SimTime(0),1000,2000,1_000_000,bg,s,&Jobs,&Clock)
}

#[test]
fn zero_background_recovers_old_step_s250(){
    let mut a=volume(); let mut b=volume();
    for i in 0..16 {a.eta[i]+=0.01*(i as f32*0.2).cos();}
    b.eta.copy_from_slice(&a.eta);
    let (u,w)=fields(&a);let bg=input(&a,&u,&w);
    let old=a.step(0.001,2000,&Jobs).unwrap();
    let new=run(&mut b,&bg,Sponge::default()).unwrap().report.unwrap();
    assert!(!old.degraded && !new.degraded);
    for (x,y) in a.u.iter().chain(&a.w).zip(b.u.iter().chain(&b.w)){
        assert!((x-y).abs()<1e-7,"{x} {y}");
    }
    let mut v=volume();run(&mut v,&bg,Sponge::default()).unwrap();
    assert!(v.u.iter().chain(&v.w).all(|x|*x==0.));
}

#[test]
fn crossed_acceleration_has_both_terms_and_negative_source_s250(){
    let s=BackgroundSample{u:[2.,0.,3.],du_dt:[7.,0.,11.],
        grad_u:[[5.,0.,7.],[0.;3],[11.,0.,13.]],grad_p_dyn:[1025.,0.,2050.],..Default::default()};
    // axis x: U·Dv=2*17+3*19 ; v·gradU=23*5+29*7 ; S=7+1+2*5+3*7.
    let expected=2.*17.+3.*19.+23.*5.+29.*7.+7.+1.+2.*5.+3.*7.;
    assert_eq!(extra(&s,0,[23.,29.],[17.,19.],1025.).unwrap(),expected);
    let mut v=volume();let (mut u,mut w)=fields(&v);
    for s in u.iter_mut().chain(&mut w){s.du_dt=[2.,0.,-3.];}
    let bg=input(&v,&u,&w);
    v.coupled_predict(&bg,0.001,Sponge::default(),&mut Control::unlimited()).unwrap();
    assert!((v.us[v.fu(4,4)]+0.002).abs()<1e-8);
    assert!((v.ws[v.fw(4,4)]-0.003).abs()<1e-8);
    assert_eq!(v.u[v.fu(4,4)],0.); // predictor does not publish
}

#[test]
fn predictor_cross_terms_on_affine_fields_s250(){
    let mut v=volume();let (mut u,mut w)=fields(&v);
    // v=(2x+3z, 5x-2z), divergence-free affine interior; U=(7,11).
    for k in 0..8 {for i in 0..=16 {let f=v.fu(i,k);
        v.u[f]=2.*i as f32*0.5+3.*(k as f32+0.5)*0.5;u[f].u=[7.,0.,11.];}}
    for k in 0..=8 {for i in 0..16 {let f=v.fw(i,k);
        v.w[f]=5.*(i as f32+0.5)*0.5-2.*k as f32*0.5;w[f].u=[7.,0.,11.];}}
    let bg=input(&v,&u,&w);let f=v.fu(4,4);let old=v.u[f];
    let wc=5.*2.-2.*2.25;
    v.coupled_predict(&bg,0.001,Sponge::default(),&mut Control::unlimited()).unwrap();
    let expected=old-0.001*(old*2.+wc*3.+7.*2.+11.*3.);
    assert!((v.us[f]-expected).abs()<2e-6);
    assert!((v.us[f]-(old-0.001*(old*2.+wc*3.))).abs()>0.04);
}

#[test]
fn sponge_is_exact_exponential_and_preserves_interior_s250(){
    let s=Sponge{width_m:2.,rate_per_s:4.};
    assert_eq!(s.factor(4.,8.,0.2),1.);
    assert!((s.factor(0.,8.,0.2)-(-0.8f32).exp()).abs()<1e-7);
    assert_eq!(s.factor(1.,8.,0.2),s.factor(7.,8.,0.2));
    let mut v=volume();let (u,w)=fields(&v);let bg=input(&v,&u,&w);
    // vertical shear w(x), independent of z: solenoidal, zero self-advection.
    for k in 0..=8 {for i in 0..16 {let f=v.fw(i,k);v.w[f]=(i as f32*0.2).sin();}}
    let before:f64=v.w.iter().map(|x|(*x as f64).powi(2)).sum();
    let r=run(&mut v,&bg,s).unwrap().report.unwrap();
    assert!(!r.degraded);
    let after:f64=v.u.iter().chain(&v.w).map(|x|(*x as f64).powi(2)).sum();
    assert!(after<before,"{before} -> {after}");
}

#[test]
fn refusals_and_expiration_are_atomic_s250(){
    let mut v=volume();let (mut u,w)=fields(&v);
    let original=(v.u.clone(),v.w.clone(),v.p.clone(),v.eta.clone());
    u[3].grad_u[1][1]=0.1;
    assert_eq!(run(&mut v,&input(&volume(),&u,&w),Sponge::default()),Err(Error::NonPlanar));
    u[3]=BackgroundSample::default();u[2].u[0]=f32::NAN;
    assert_eq!(run(&mut v,&input(&volume(),&u,&w),Sponge::default()),Err(Error::NotFinite));
    u[2]=BackgroundSample::default();
    for (i,s) in u.iter_mut().enumerate(){s.du_dt[0]=(i as f32*0.1).sin();}
    let bg=input(&v,&u,&w);
    assert_eq!(v.step_perturbation(SimTime(1),1000,100,1000,&bg,Sponge::default(),&Jobs,&Clock),Err(Error::BackgroundContext));
    assert_eq!(v.step_perturbation(SimTime(0),1000,0,1000,&bg,Sponge::default(),&Jobs,&Clock),Err(Error::Convergence));
    struct Tick(std::cell::Cell<u64>);
    impl MonotonicClock for Tick {fn now_ns(&self)->u64{let n=self.0.get();self.0.set(n+1000);n}}
    let mut completions=0;
    for limit in [0,5,20,50,100,500,2000] {
        let r=v.step_perturbation(SimTime(0),1000,2000,limit,&bg,Sponge::default(),&Jobs,&Tick(std::cell::Cell::new(0))).unwrap();
        if r.advanced_us==0 {
            assert_eq!((&v.u,&v.w,&v.p,&v.eta),(&original.0,&original.1,&original.2,&original.3));
        } else {completions+=1;break;}
    }
    if completions==0 {run(&mut v,&bg,Sponge::default()).unwrap();}
    let mut clean=volume();run(&mut clean,&bg,Sponge::default()).unwrap();
    assert_eq!(v.u,clean.u);assert_eq!(v.w,clean.w);assert_eq!(v.p,clean.p);
}
