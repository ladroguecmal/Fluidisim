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
    assert!((s.factor(1.,8.,0.2)-(-0.2f32).exp()).abs()<1e-7);
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
fn context_shape_sponge_and_overflow_refuse_s250(){
    let mut v=volume();let (u,w)=fields(&v);
    let mut bg=input(&v,&u,&w);
    bg.density=1000.;assert_eq!(run(&mut v,&bg,Sponge::default()),Err(Error::BackgroundContext));
    bg.density=1025.;bg.gravity=1.;assert_eq!(run(&mut v,&bg,Sponge::default()),Err(Error::BackgroundContext));
    bg.gravity=9.81;bg.u=&u[1..];assert_eq!(run(&mut v,&bg,Sponge::default()),Err(Error::Shape));
    bg.u=&u;
    for s in [Sponge{width_m:0.,rate_per_s:1.},Sponge{width_m:5.,rate_per_s:1.},
        Sponge{width_m:1.,rate_per_s:f32::NAN}] {
        assert_eq!(run(&mut v,&bg,s),Err(Error::Domain));
    }
    for (t,dt,budget) in [(0,0,1000),(u64::MAX,1,1000),(0,1,u64::MAX),(0,(1<<53)+1,1000)] {
        assert_eq!(v.step_perturbation(SimTime(t),dt,2000,budget,&bg,Sponge::default(),&Jobs,&Clock),Err(Error::NotFinite));
    }
    v.rest-=0.5;
    assert_eq!(run(&mut v,&bg,Sponge::default()),Err(Error::BackgroundContext));
    assert!(v.u.iter().chain(&v.w).chain(&v.p).all(|x|*x==0.));
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

fn real_case(nx:usize)->(Volume,Vec<BackgroundSample>,Vec<BackgroundSample>) {
    let v=Volume::configure(&mut HostServices{alloc:&mut Arena,jobs:&Jobs,sink:&Jobs},
        Domain{nx,nz:nx/2,dx:8./nx as f32},1025.,9.81,&vec![0.;nx]).unwrap();
    let (u,w)=real_fields(&v,SimTime(1_000_000));
    (v,u,w)
}

/// B et W du banc `delta_coupling` (S250) à l'instant `t`, sommés avant contraction.
fn real_fields(v:&Volume,t:SimTime)->(Vec<BackgroundSample>,Vec<BackgroundSample>) {
    use crate::{Background,SeaState,WorldPos,modal_pressure::Segment,spectral_pressure::{self,Node,Slot}};
    let nx=v.domain.nx;
    let mut host=HostServices{alloc:&mut Arena,jobs:&Jobs,sink:&Jobs};
    let b=Background::configure(&mut host,SeaState{hs:0.1,tp:4.,theta_turns:0.,components:1,graine:7},WorldPos::default()).unwrap();
    let nodes=[Node{k:[0.7,0.],transform:1.,weight:0.5},Node{k:[1.2,0.],transform:1.,weight:0.5}];
    let path=[Segment{birth:SimTime(0),duration_us:2_000_000,origin:[0.;2],velocity:[0.5,0.],pressure_pa:80.}];
    let mut slots=[Slot::default();2];
    let field=spectral_pressure::prepare(&nodes,&path,9.81,1025.,t,SimTime(2_000_000),[-16.;2],[16.;2],&mut slots).unwrap();
    let sample=|p|{let mut s=b.differential_local(p,t,1025.).unwrap();s.add(&field.differential(p).unwrap().water);s};
    let (mut u,mut w)=fields(&v);let dx=v.domain.dx;
    for k in 0..nx/2 {for i in 0..=nx {u[v.fu(i,k)]=sample([i as f32*dx,0.,(k as f32+0.5)*dx-4.]);}}
    for k in 0..=nx/2 {for i in 0..nx {w[v.fw(i,k)]=sample([(i as f32+0.5)*dx,0.,k as f32*dx-4.]);}}
    (u,w)
}

/// Oracle de banc : assemblage par arêtes, CG f64 ; aucun appel à apply/project du candidat.
fn flat_oracle(v:&Volume)->(Vec<f64>,Vec<f64>,Vec<f64>) {
    let (nx,nz,dx)=(v.domain.nx,v.domain.nz,v.domain.dx as f64);
    let n=nx*nz;let c=|i:usize,k:usize|k*nx+i;
    let mut edges=Vec::new();
    for k in 0..nz {for i in 0..nx {
        if i+1<nx {edges.push((c(i,k),c(i+1,k)));}
        if k+1<nz {edges.push((c(i,k),c(i,k+1)));}
    }}
    let apply=|p:&[f64]| {
        let mut a=vec![0.;n];
        for &(i,j) in &edges {let d=(p[i]-p[j])/(dx*dx);a[i]+=d;a[j]-=d;}
        for i in 0..nx {a[c(i,nz-1)]+=2.*p[c(i,nz-1)]/(dx*dx);}
        a
    };
    let mut b=vec![0.;n];
    for k in 0..nz {for i in 0..nx {
        b[c(i,k)]=-1025./0.001/dx*(v.us[v.fu(i+1,k)] as f64-v.us[v.fu(i,k)] as f64
            +v.ws[v.fw(i,k+1)] as f64-v.ws[v.fw(i,k)] as f64);
    }}
    let dot=|x:&[f64],y:&[f64]|x.iter().zip(y).map(|(a,b)|a*b).sum::<f64>();
    let mut p=vec![0.;n];let mut r=b.clone();let mut d=r.clone();let b2=dot(&b,&b);let mut rr=b2;
    for _ in 0..4*n {
        if rr<1e-26*b2 {break;}
        let q=apply(&d);let alpha=rr/dot(&d,&q);
        for i in 0..n {p[i]+=alpha*d[i];r[i]-=alpha*q[i];}
        let next=dot(&r,&r);for i in 0..n {d[i]=r[i]+next/rr*d[i];}rr=next;
    }
    let residual=apply(&p).iter().zip(&b).map(|(a,b)|(a-b).powi(2)).sum::<f64>();
    assert!(residual<1e-22*b2);
    let mut u:Vec<f64>=v.us.iter().map(|x|*x as f64).collect();
    let mut w:Vec<f64>=v.ws.iter().map(|x|*x as f64).collect();
    for k in 0..nz {for i in 1..nx {u[v.fu(i,k)]-=0.001/1025./dx*(p[c(i,k)]-p[c(i-1,k)]);}}
    for k in 1..=nz {for i in 0..nx {
        let grad=if k==nz {-2.*p[c(i,k-1)]} else {p[c(i,k)]-p[c(i,k-1)]};
        w[v.fw(i,k)]-=0.001/1025./dx*grad;
    }}
    (p,u,w)
}

#[test]
fn diagnose_flat_projection_s251(){
    for nx in [16,32] {
        let (mut v,u,w)=real_case(nx);let bg=input(&v,&u,&w);
        v.coupled_predict(&bg,0.001,Sponge{width_m:1.,rate_per_s:2.},&mut Control::unlimited()).unwrap();
        let (_,u64,w64)=flat_oracle(&v);
        let norm=u64.iter().chain(&w64).fold(0f64,|m,x|m.max(x.abs()));
        let predicted=v.us.iter().chain(&v.ws).fold(0f32,|m,x|m.max(x.abs()));
        let error=|v:&Volume|v.u.iter().chain(&v.w).zip(u64.iter().chain(&w64))
            .fold(0f64,|m,(a,b)|m.max((*a as f64-b).abs()))/norm;
        let mut r=v.project(-1_025_000.,(0.001f64/1025.) as f32,2000,true,&Jobs,&mut Control::unlimited()).unwrap();
        assert!(r.degraded);
        println!("S251 nx={nx} before={r:?} pred={predicted:e} oracle_vmax={norm:e} velocity_error={:e}",error(&v));
        for pass in 1..=3 {
            v.us.copy_from_slice(&v.u);v.ws.copy_from_slice(&v.w);
            r=v.project(-1_025_000.,(0.001f64/1025.) as f32,2000,false,&Jobs,&mut Control::unlimited()).unwrap();
            println!("S251 nx={nx} correction={pass} {r:?} velocity_error={:e}",error(&v));
            if !r.degraded {break;}
        }
        assert!(!r.degraded);
        assert!(error(&v)<1e-4);
    }
}

#[test]
fn flat_coupled_step_received_against_oracle_s251(){
    for nx in [16,32] {
        let (mut v,u,w)=real_case(nx);let mut bg=input(&v,&u,&w);bg.time=SimTime(1_000_000);
        let sponge=Sponge{width_m:1.,rate_per_s:2.};
        v.coupled_predict(&bg,0.001,sponge,&mut Control::unlimited()).unwrap();
        let (p64,u64,w64)=flat_oracle(&v);
        let r=v.step_perturbation(bg.time,1000,2000,1_000_000,&bg,sponge,&Jobs,&Clock).unwrap().report.unwrap();
        assert_eq!(r.refinements,1);assert!(!r.degraded && r.divergence<=1e-5);
        let norm=u64.iter().chain(&w64).fold(0f64,|m,x|m.max(x.abs()));
        let error=v.u.iter().chain(&v.w).zip(u64.iter().chain(&w64))
            .fold(0f64,|m,(a,b)|m.max((*a as f64-b).abs()))/norm;
        let pn=p64.iter().fold(0f64,|m,x|m.max(x.abs()));
        let pe=v.p.iter().zip(&p64).fold(0f64,|m,(a,b)|m.max((*a as f64-b).abs()))/pn;
        println!("S251 received nx={nx} {r:?} error_velocity={error:e} error_pressure={pe:e}");
        assert!(error<1e-4 && pe<1e-4);
    }
}

#[test]
fn refinement_restores_lid_and_state_on_expiration_s251(){
    struct Counter(std::cell::Cell<u64>,u64);
    impl MonotonicClock for Counter {fn now_ns(&self)->u64 {
        let n=self.0.get();self.0.set(n+1);if n>=self.1 {1_000_000} else {0}
    }}
    let (mut reference,u,w)=real_case(16);let bg=input(&reference,&u,&w);
    let sponge=Sponge{width_m:1.,rate_per_s:2.};
    let count=Counter(std::cell::Cell::new(0),u64::MAX);
    let r=reference.step_perturbation(bg.time,1000,2000,1000,&bg,sponge,&Jobs,&count).unwrap();
    assert_eq!(r.report.unwrap().refinements,1);
    let calls=count.0.get();
    for cutoff in [calls/4,calls/2,calls*3/4,calls-10,calls-2] {
        let (mut v,_,_)=real_case(16);
        let r=v.step_perturbation(bg.time,1000,2000,1000,&bg,sponge,&Jobs,
            &Counter(std::cell::Cell::new(0),cutoff)).unwrap();
        assert_eq!(r.advanced_us,0);
        assert!(!v.homogeneous_lid);
        assert!(v.u.iter().chain(&v.w).chain(&v.p).all(|x|*x==0.));
        assert!(v.eta.iter().all(|x|*x==4.));
        v.step_perturbation(bg.time,1000,2000,1000,&bg,sponge,&Jobs,&Clock).unwrap();
        assert_eq!(v.u,reference.u);assert_eq!(v.w,reference.w);assert_eq!(v.p,reference.p);
    }
}

#[test]
fn incremental_projection_does_not_reapply_imposed_pressure_s251(){
    let mut v=volume();
    for i in 0..16 {v.eta[i]+=0.01*(i as f32*0.2).cos();}
    v.step(0.001,2000,&Jobs).unwrap();
    let old=(v.u.clone(),v.w.clone(),v.eta.clone());
    let r=v.refine_coupled_pressure(-1_025_000.,(0.001f64/1025.) as f32,2000,&Jobs,&mut Control::unlimited()).unwrap();
    assert!(!r.degraded);
    let change=v.u.iter().chain(&v.w).zip(old.0.iter().chain(&old.1))
        .fold(0f32,|m,(a,b)|m.max((a-b).abs()));
    assert!(change<1e-8,"couvercle appliqué deux fois : {change}");
    assert_eq!(v.eta,old.2);assert!(!v.homogeneous_lid);
}

/// S252, A284 : où vont les itérations du démarrage plat 32×16 sur la trajectoire du banc
/// `delta_coupling --flat --fine` — prédiction, projection ordinaire, repli, affinage.
/// Diagnostic, aucune assertion de coût : `cargo test --release -p water-core -- --ignored
/// flat_start_cost_attribution_s252 --nocapture`.
#[test]
#[ignore]
fn flat_start_cost_attribution_s252(){
    let (mut v,_,_)=real_case(32);
    let sponge=Sponge{width_m:1.,rate_per_s:2.};
    COUPLED_TRACE.with(|t|*t.borrow_mut()=Some(Vec::new()));
    for step in 0..20u64 {
        let t=SimTime(1_000_000+step*1000);
        let (u,w)=real_fields(&v,t);
        let mut bg=input(&v,&u,&w);bg.time=t;
        super::super::PRESSURE_TRACE.with(|t|t.borrow_mut().clear());
        let start=std::time::Instant::now();
        let r=v.step_perturbation(t,1000,2000,1_000_000,&bg,sponge,&Jobs,&Clock).unwrap().report.unwrap();
        let total=start.elapsed().as_nanos();
        if step==0 || step==7 {
            // Chaque vrai résidu recalculé : (itérations cumulées de la projection, résidu relatif,
            // erreur inverse) — les relances de la boucle d'acceptation ADR-144.
            let restarts=super::super::PRESSURE_TRACE.with(|t|t.borrow().clone());
            println!("A284 pas={step} relances={} {:?}",restarts.len(),restarts);
        }
        let trace=COUPLED_TRACE.with(|t|t.borrow_mut().as_mut().map(core::mem::take)).unwrap();
        let parts:Vec<String>=trace.iter().map(|(l,it,floor,deg,div,ns)|
            format!("{l}:it={it},plancher={floor},degrade={deg},D={div:.3e},ms={:.3}",*ns as f64/1e6)).collect();
        println!("A284 pas={step} total_ms={:.3} iterations={} affinages={} | {}",total as f64/1e6,r.iterations,r.refinements,parts.join(" | "));
    }
    COUPLED_TRACE.with(|t|*t.borrow_mut()=None);
}

/// S252, A285 : le gradient conjugué préconditionné par la multigrille doit rester un gradient
/// conjugué. Sa récurrence est exacte à l'arrondi près ; son premier vrai résidu ne peut donc pas
/// être au-dessus du plancher que le chemin ordinaire atteint **sur le même système**. Avant
/// correction : 0,616 contre 1,04·10⁻⁵ — β employait ‖r₊‖² au lieu de ⟨r₊, z₊⟩.
#[test]
fn multigrid_conjugate_gradient_keeps_its_recursion_s252(){
    let (mut v,u,w)=real_case(32);let bg=input(&v,&u,&w);
    assert!(!v.levels.is_empty());
    v.coupled_predict(&bg,0.001,Sponge{width_m:1.,rate_per_s:2.},&mut Control::unlimited()).unwrap();
    let (us,ws)=(v.us.clone(),v.ws.clone());
    let (scale,k1)=(-1_025_000f32,(0.001f64/1025.) as f32);
    let trace=|| super::super::PRESSURE_TRACE.with(|t|t.borrow().clone());
    super::super::PRESSURE_TRACE.with(|t|t.borrow_mut().clear());
    let ordinary=v.project(scale,k1,2000,false,&Jobs,&mut Control::unlimited()).unwrap();
    let floor=trace().last().unwrap().1;
    assert_eq!((v.us.clone(),v.ws.clone()),(us,ws),"la projection ne touche pas au prédicteur");
    super::super::PRESSURE_TRACE.with(|t|t.borrow_mut().clear());
    let multigrid=v.project(scale,k1,2000,true,&Jobs,&mut Control::unlimited()).unwrap();
    let first=trace()[0];
    println!("A285 ordinaire={ordinary:?} plancher={floor:e}
A285 multigrille={multigrid:?} premier_vrai_residu={first:?} relances={}",trace().len());
    assert!(first.1<=floor,"premier vrai résidu multigrille {} au-dessus du plancher ordinaire {floor}",first.1);
}
