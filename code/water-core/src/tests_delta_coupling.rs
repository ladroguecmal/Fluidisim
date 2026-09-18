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
    let r=v.refine_divergence(-1_025_000.,(0.001f64/1025.) as f32,2000,false,&Jobs,&mut Control::unlimited()).unwrap();
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

#[path = "../examples/support/standing_background.rs"]
#[allow(dead_code)]
mod standing;

/// S253 (ADR-152) — le fond de l'oracle couplé avant usage : incompressible, linéaire (`U_t + ∇P/ρ = 0`)
/// et à cinématique linéarisée (`ζ_t = W(0)`), sous **et au-dessus** du plan moyen ; le résidu
/// contracté se réduit à `(U·∇)U`. Arrondis f32 seulement.
#[test]
fn standing_background_is_incompressible_linear_and_kinematic_s253(){
    let wave=standing::StandingWave{a:0.1,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho:1025.};
    let eps=f32::EPSILON;
    for t in [0.,0.31,0.77,1.13] {
        for x in [0.1,0.55,1.3,1.9] {
            for z in [-1.9,-0.7,-0.05,0.,0.08,0.15] {
                let s=wave.sample(x,z,t);
                let div=s.grad_u[0][0]+s.grad_u[2][2];
                assert!(div.abs()<=4.*eps*s.grad_u[0][0].abs(),"div {div} en x={x} z={z} t={t}");
                for (axis,scale) in [(0,s.du_dt[0].abs()),(2,s.du_dt[2].abs())] {
                    let linear=s.du_dt[axis]+s.grad_p_dyn[axis]/1025.;
                    assert!(linear.abs()<=4.*eps*scale+f32::MIN_POSITIVE,"quantité de mouvement linéaire {linear}");
                }
                let r=s.momentum_residual(1025.,0.).unwrap();
                let adv=wave.advection(x,z,t);
                for (axis,i) in [(0,0),(2,1)] {
                    let bound=1e-5*adv[i].abs()+8.*(eps*s.du_dt[axis].abs()) as f64;
                    assert!((r[axis] as f64-adv[i]).abs()<=bound,"S {} contre {}",r[axis],adv[i]);
                }
            }
            let rate=wave.eta_rate(x,t);
            let w0=wave.sample(x,0.,t).u[2] as f64;
            assert!((w0-rate).abs()<=1e-6*rate.abs().max(1e-6),"ζ_t {rate} contre W(0) {w0}");
        }
    }
}

/// Volume du banc S237 (L = h = 2 m, domaine jusqu'à 2,25 m, repos à 2 m) et échantillons du fond
/// aux faces MAC, `z` compté depuis le repos.
fn standing_case(nx:usize,wave:&standing::StandingWave,eta:&[f32])->Volume {
    let dx=2./nx as f32;
    let mut v=Volume::configure(&mut HostServices{alloc:&mut Arena,jobs:&Jobs,sink:&Jobs},
        Domain{nx,nz:(2.25/dx).round() as usize,dx},wave.rho as f32,wave.g as f32,&vec![0.;nx]).unwrap();
    v.set_free_surface(eta,wave.h as f32).unwrap();
    v
}
fn standing_faces(v:&Volume,wave:&standing::StandingWave,t:f64)->(Vec<BackgroundSample>,Vec<BackgroundSample>) {
    let (nx,nz,dx,rest)=(v.domain.nx,v.domain.nz,v.domain.dx as f64,v.rest as f64);
    let (mut u,mut w)=fields(v);
    for k in 0..nz {for i in 0..=nx {u[v.fu(i,k)]=wave.sample(i as f64*dx,(k as f64+0.5)*dx-rest,t);}}
    for k in 0..=nz {for i in 0..nx {w[v.fw(i,k)]=wave.sample((i as f64+0.5)*dx,k as f64*dx-rest,t);}}
    (u,w)
}

/// S253 (ADR-152) — valeurs fantômes du fond contre la pression analytique à l'interface. Le fantôme
/// vertical vaut `ρg·ζ_fond − P(x_i, ζ_i)`, le latéral `−P(x_Γ, z_c)` ; l'écart tient au Taylor
/// d'ordre un sur au plus une demi-maille, `(dx/2)²·|∂²P|/2`, plus l'arrondi f32.
#[test]
fn background_ghost_values_match_interface_pressure_s253(){
    let wave=standing::StandingWave{a:0.1,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho:1025.};
    let nx=32;let dx=2./nx as f64;let t=0.37;
    // η' non nul, pour que la géométrie totale diffère de ζ_fond et traverse des centres.
    let eta:Vec<f32>=(0..nx).map(|i|(2.+0.08*(5.*wave.k*(i as f64+0.5)*dx).sin()) as f32).collect();
    let mut v=standing_case(nx,&wave,&eta);
    let (u,w)=standing_faces(&v,&wave,t);
    let bg=BackgroundFaces{domain:v.domain,time:SimTime(0),density:1025.,gravity:9.81,u:&u,w:&w};
    v.prepare_surface_background(&bg,&mut Control::unlimited()).unwrap();
    assert!(v.surface_coupled);
    let (rho,g,k)=(1025f64,9.81f64,wave.k);
    let pmax=rho*g*wave.a;
    // |∂²P| ≤ ρga·k²·C(z) en x comme en z, C croissant : majoré au sommet du domaine (0,25 m).
    let c_top=(k*(0.25+wave.h)).cosh()/(k*wave.h).cosh();
    let taylor=0.5*(0.5*dx).powi(2)*k*k*pmax*c_top;
    let mut sides=0;
    for i in 0..nx {
        let x=(i as f64+0.5)*dx;
        let zeta=v.surface_total[i] as f64;
        let exact=rho*g*wave.sample(x,0.,t).eta as f64-wave.sample(x,zeta-2.,t).p_dyn as f64;
        assert!((v.ghost_bg_up[i] as f64-exact).abs()<=taylor+1e-3,"colonne {i} : {} contre {exact}",v.ghost_bg_up[i]);
        for kk in 0..v.domain.nz {
            if i+1<nx && v.wet(i,kk)!=v.wet(i+1,kk) {
                let (wet,air)=if v.wet(i,kk){(i,i+1)}else{(i+1,i)};
                let zc=(kk as f64+0.5)*dx;
                let (hw,ha)=(v.surface_total[wet] as f64,v.surface_total[air] as f64);
                let theta=((hw-zc)/(hw-ha)).max(1e-3);
                let xw=(wet as f64+0.5)*dx;
                let xg=if air>wet {xw+theta*dx} else {xw-theta*dx};
                let exact=-(wave.sample(xg,zc-2.,t).p_dyn as f64);
                let got=v.ghost_bg_side[v.fu(i+1,kk)] as f64;
                assert!((got-exact).abs()<=taylor+1e-3,"face ({},{kk}) : {got} contre {exact}",i+1);
                sides+=1;
            }
        }
    }
    assert!(sides>=4,"l'essai doit exercer des fantômes latéraux : {sides}");
    println!("S253 fantomes : borne_taylor={taylor:e} Pa, lateraux={sides}");
}

fn same_bits(a:&[f32],b:&[f32])->bool {a.len()==b.len()&&a.iter().zip(b).all(|(x,y)|x.to_bits()==y.to_bits())}

/// S253 (ADR-152, critère 1) — fond nul : le pas perturbatif mobile rend le pas S237 **au bit**,
/// champs, hauteur, restes et rapport, sur cinquante pas d'une onde stationnaire de 5 cm.
#[test]
fn zero_background_mobile_step_is_s237_to_the_bit_s253(){
    let nx=32;let dx=2./nx as f64;let k=std::f64::consts::PI/2.;
    let eta:Vec<f32>=(0..nx).map(|i|(2.+0.05*(k*(i as f64+0.5)*dx).cos()) as f32).collect();
    let wave=standing::StandingWave{a:0.,k,h:2.,g:9.81,rho:1025.};
    let mut total=standing_case(nx,&wave,&eta);
    let mut coupled=standing_case(nx,&wave,&eta);
    let (u,w)=fields(&coupled);
    for n in 0..50u64 {
        let t=SimTime(n*1000);
        let bg=BackgroundFaces{domain:coupled.domain,time:t,density:1025.,gravity:9.81,u:&u,w:&w};
        let a=total.step_surface_mobile(1000,4000,1_000_000,&Jobs,&Clock).unwrap();
        let b=coupled.step_perturbation_mobile(t,1000,4000,1_000_000,&bg,Sponge::default(),&Jobs,&Clock).unwrap();
        assert_eq!(a.report,b.report,"rapport au pas {n}");
        assert!(same_bits(&total.u,&coupled.u)&&same_bits(&total.w,&coupled.w)&&same_bits(&total.p,&coupled.p),"champs au pas {n}");
        assert!(same_bits(&total.eta,&coupled.eta)&&same_bits(&total.eta_roundoff,&coupled.eta_roundoff),"hauteur au pas {n}");
        assert!(!coupled.surface_coupled&&!coupled.mobile);
    }
}

/// Une période de l'onde stationnaire S237 par le pas couplé, fond = ordre un : écart maximal de `b₂`
/// à l'ordre deux fermé, relatif à son maximum, et nombre de pas avancés.
fn coupled_b2_error(nx:usize,a:f64)->(f64,usize) {
    let wave=standing::StandingWave{a,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho:1025.};
    let mut v=standing_case(nx,&wave,&vec![2.;nx]);
    let dx=2./nx as f64;
    let steps=(2.*std::f64::consts::PI/wave.omega()/1e-3).round() as usize;
    let (mut worst,mut peak)=(0f64,0f64);
    for n in 0..steps {
        let t=n as f64*1e-3;
        let (u,w)=standing_faces(&v,&wave,t);
        let bg=BackgroundFaces{domain:v.domain,time:SimTime(n as u64*1000),density:1025.,gravity:9.81,u:&u,w:&w};
        match v.step_perturbation_mobile(bg.time,1000,4000,1_000_000,&bg,Sponge::default(),&Jobs,&Clock) {
            Ok(r) => assert_eq!(r.advanced_us,1000),
            Err(e) => {println!("S253 refus nx={nx} a={a} pas={n} {e:?}");return (f64::INFINITY,n);}
        }
        let t1=t+1e-3;
        let mut b2=0.;
        for i in 0..nx {
            let x=(i as f64+0.5)*dx;
            let total=(v.eta[i] as f64-2.)+a*(wave.k*x).cos()*(wave.omega()*t1).cos();
            b2+=2./2.*total*(2.*wave.k*x).cos()*dx;
        }
        let reference=a*a*wave.second_order_b2(t1);
        worst=worst.max((b2-reference).abs());
        peak=peak.max(reference.abs());
    }
    (worst/peak,steps)
}

/// S253 (ADR-152, critère 5) — **témoin discriminant**. Même montage, une période, 32 colonnes,
/// 5 cm : le pas couplé tient `b₂` à 20 % de l'ordre deux fermé ; sans résidus de surface, non.
#[test]
#[cfg_attr(debug_assertions, ignore = "une période : release")]
fn surface_residuals_carry_the_second_harmonic_s253(){
    let (with,steps)=coupled_b2_error(32,0.05);
    SURFACE_RESIDUALS_OFF.with(|c|c.set(true));
    let (without,_)=coupled_b2_error(32,0.05);
    SURFACE_RESIDUALS_OFF.with(|c|c.set(false));
    println!("S253 temoin nx=32 a=0.05 pas={steps} b2_avec={with:.4} b2_sans={without:.4}");
    assert!(with<=0.20,"couplé : b₂ à {with} de l'ordre deux");
    assert!(without>0.20,"témoin sans résidus : b₂ à {without}, non discriminant");
}

/// S253 (critère 2) — refus sans rien modifier : contexte, forme, non-planarité, `eta` du fond
/// incohérent dans une colonne, surface totale hors gardes. Le mode couplé reste éteint.
#[test]
fn coupled_mobile_refusals_are_atomic_s253(){
    let wave=standing::StandingWave{a:0.05,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho:1025.};
    let nx=32;
    let mut v=standing_case(nx,&wave,&vec![2.;nx]);
    let (u,w)=standing_faces(&v,&wave,0.2);
    let snapshot=|v:&Volume|(v.u.clone(),v.w.clone(),v.p.clone(),v.eta.clone(),v.eta_roundoff.clone());
    let before=snapshot(&v);
    let run=|v:&mut Volume,u:&[BackgroundSample],w:&[BackgroundSample],density:f32|{
        let bg=BackgroundFaces{domain:v.domain,time:SimTime(0),density,gravity:9.81,u,w};
        v.step_perturbation_mobile(SimTime(0),1000,4000,1_000_000,&bg,Sponge::default(),&Jobs,&Clock)
    };
    assert_eq!(run(&mut v,&u,&w,1000.),Err(Error::BackgroundContext));
    assert_eq!(run(&mut v,&u[1..],&w,1025.),Err(Error::Shape));
    let mut bad=u.clone();bad[40].grad_u[1][0]=0.1;
    assert_eq!(run(&mut v,&bad,&w,1025.),Err(Error::NonPlanar));
    let mut bad=w.clone();bad[v.fw(5,7)].eta+=1e-3;
    assert_eq!(run(&mut v,&u,&bad,1025.),Err(Error::BackgroundContext));
    let mut high=w.clone();
    for i in 0..nx {for k in 0..=v.domain.nz {high[v.fw(i,k)].eta=0.3;}}
    assert_eq!(run(&mut v,&u,&high,1025.),Err(Error::Domain));
    assert_eq!(snapshot(&v),before);
    assert!(!v.surface_coupled&&!v.mobile);
    assert!(run(&mut v,&u,&w,1025.).is_ok());
}

/// S253 (critère 2) — expiration à plusieurs points du pas : aucune avancée, état intact, mode
/// couplé éteint ; la reprise rend le pas de référence au bit.
#[test]
fn coupled_mobile_expiration_restores_and_resumes_s253(){
    struct Counter(std::cell::Cell<u64>,u64);
    impl MonotonicClock for Counter {fn now_ns(&self)->u64 {
        let n=self.0.get();self.0.set(n+1);if n>=self.1 {1_000_000} else {0}
    }}
    let wave=standing::StandingWave{a:0.05,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho:1025.};
    let nx=32;
    let build=||{
        let mut v=standing_case(nx,&wave,&vec![2.;nx]);
        let (u,w)=standing_faces(&v,&wave,0.);
        let bg=BackgroundFaces{domain:v.domain,time:SimTime(0),density:1025.,gravity:9.81,u:&u,w:&w};
        v.step_perturbation_mobile(SimTime(0),1000,4000,1_000_000,&bg,Sponge::default(),&Jobs,&Clock).unwrap();
        v
    };
    let (u,w)=standing_faces(&build(),&wave,1e-3);
    let mut reference=build();
    let count=Counter(std::cell::Cell::new(0),u64::MAX);
    let bg=BackgroundFaces{domain:reference.domain,time:SimTime(1000),density:1025.,gravity:9.81,u:&u,w:&w};
    reference.step_perturbation_mobile(SimTime(1000),1000,4000,1000,&bg,Sponge::default(),&Jobs,&count).unwrap();
    let calls=count.0.get();
    for cutoff in [1,calls/5,calls/2,calls*4/5,calls-3] {
        let mut v=build();
        let before=(v.u.clone(),v.w.clone(),v.p.clone(),v.eta.clone(),v.eta_roundoff.clone());
        let r=v.step_perturbation_mobile(SimTime(1000),1000,4000,1000,&bg,Sponge::default(),&Jobs,
            &Counter(std::cell::Cell::new(0),cutoff)).unwrap();
        assert_eq!(r.advanced_us,0,"coupure {cutoff}");
        assert_eq!((v.u.clone(),v.w.clone(),v.p.clone(),v.eta.clone(),v.eta_roundoff.clone()),before);
        assert!(!v.surface_coupled&&!v.mobile);
        v.step_perturbation_mobile(SimTime(1000),1000,4000,1000,&bg,Sponge::default(),&Jobs,&Clock).unwrap();
        assert!(same_bits(&v.u,&reference.u)&&same_bits(&v.p,&reference.p)&&same_bits(&v.eta,&reference.eta));
    }
}

/// S253 — diagnostic du refus `Convergence` au premier pas couplé mobile (64 colonnes à 10 cm,
/// 128 colonnes à 5 et 10 cm) : mêmes étapes que le pas, rapport de projection complet, échelles
/// du prédicteur et des valeurs fantômes, vrais résidus successifs. Aucune assertion.
#[test]
#[ignore = "diagnostic S253, release"]
fn coupled_mobile_first_step_refusal_diagnosis_s253(){
    for (nx,a) in [(32usize,0.10f64),(64,0.05),(64,0.10),(128,0.05)] {
        let wave=standing::StandingWave{a,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho:1025.};
        let mut v=standing_case(nx,&wave,&vec![2.;nx]);
        let (u,w)=standing_faces(&v,&wave,0.);
        let bg=BackgroundFaces{domain:v.domain,time:SimTime(0),density:1025.,gravity:9.81,u:&u,w:&w};
        let mut ctl=Control::unlimited();
        v.prepare_surface_background(&bg,&mut ctl).unwrap();
        v.coupled_predict(&bg,0.001,Sponge::default(),&mut ctl).unwrap();
        let predicted=v.us.iter().chain(&v.ws).fold(0f32,|m,x|m.max(x.abs()));
        let ghost_up=v.ghost_bg_up.iter().fold(0f32,|m,x|m.max(x.abs()));
        let ghost_side=v.ghost_bg_side.iter().fold(0f32,|m,x|m.max(x.abs()));
        super::super::PRESSURE_TRACE.with(|t|t.borrow_mut().clear());
        v.mobile=true;
        let r=v.project(-1_025_000.,(0.001f64/1025.) as f32,4000,false,&Jobs,&mut ctl);
        v.mobile=false;v.surface_coupled=false;
        let corrected=v.u.iter().chain(&v.w).fold(0f32,|m,x|m.max(x.abs()));
        let trace=super::super::PRESSURE_TRACE.with(|t|t.borrow().clone());
        println!("S253_DIAG nx={nx} a={a} predit_max={predicted:e} fantome_haut_max={ghost_up:e} fantome_lateral_max={ghost_side:e} corrige_max={corrected:e}");
        println!("S253_DIAG   rapport={r:?}");
        let head:Vec<_>=trace.iter().take(6).collect();
        let tail:Vec<_>=trace.iter().rev().take(3).collect();
        println!("S253_DIAG   vrais_residus={} premiers={head:?} derniers={tail:?}",trace.len());
    }
}

/// S253, ADR-153 — **premier pas couplé mobile à 128 colonnes, 5 cm** : sans affinage, la projection
/// s'arrête au plancher au-dessus de la tolérance (témoin) ; par l'API, le pas est reçu par un
/// affinage à valeurs fantômes homogènes. Une expiration tardive laisse tout intact. Release.
#[test]
#[cfg_attr(debug_assertions, ignore = "128 colonnes : release")]
fn coupled_mobile_first_step_is_received_by_refinement_s253(){
    let wave=standing::StandingWave{a:0.05,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho:1025.};
    let nx=128;
    let build=||standing_case(nx,&wave,&vec![2.;nx]);
    let probe=build();
    let (u,w)=standing_faces(&probe,&wave,0.);
    let bg=BackgroundFaces{domain:probe.domain,time:SimTime(0),density:1025.,gravity:9.81,u:&u,w:&w};
    let mut witness=build();
    let mut ctl=Control::unlimited();
    witness.prepare_surface_background(&bg,&mut ctl).unwrap();
    witness.coupled_predict(&bg,0.001,Sponge::default(),&mut ctl).unwrap();
    witness.mobile=true;
    let r0=witness.project(-1_025_000.,(0.001f64/1025.) as f32,4000,false,&Jobs,&mut ctl).unwrap();
    assert!(r0.degraded&&r0.floor,"témoin : refusé au plancher attendu {r0:?}");
    struct Counter(std::cell::Cell<u64>,u64);
    impl MonotonicClock for Counter {fn now_ns(&self)->u64 {
        let n=self.0.get();self.0.set(n+1);if n>=self.1 {1_000_000_000} else {0}
    }}
    let mut v=build();
    let count=Counter(std::cell::Cell::new(0),u64::MAX);
    let r=v.step_perturbation_mobile(SimTime(0),1000,4000,1_000_000,&bg,Sponge::default(),&Jobs,&count).unwrap();
    let report=r.report.unwrap();
    println!("S253_128 temoin={r0:?}\nS253_128 pas={report:?}");
    assert_eq!(r.advanced_us,1000);
    assert_eq!(report.refinements,1);
    assert!(!report.degraded&&report.divergence_plain<=super::super::PROJECTION_DIVERGENCE_TOLERANCE,"{report:?}");
    assert!(!v.homogeneous_ghost&&!v.surface_coupled&&!v.mobile);
    let calls=count.0.get();
    let mut late=build();
    let before=(late.u.clone(),late.w.clone(),late.p.clone(),late.eta.clone());
    let r=late.step_perturbation_mobile(SimTime(0),1000,4000,1_000_000,&bg,Sponge::default(),&Jobs,
        &Counter(std::cell::Cell::new(0),calls*9/10)).unwrap();
    assert_eq!(r.advanced_us,0);
    assert_eq!((late.u.clone(),late.w.clone(),late.p.clone(),late.eta.clone()),before);
    assert!(!late.homogeneous_ghost&&!late.surface_coupled&&!late.mobile);
}

/// S254 (ADR-154) — le prolongement borné de l'oracle avant usage, au-dessus du plan moyen :
/// continuité en `z = 0`, divergence nulle, dérivées publiées contre différences finies du champ
/// publié, et résidu `S` recalculé en f64, d'ordre deux quand `z` suit l'amplitude.
#[test]
fn standing_bounded_extension_controls_s254(){
    let eps=f32::EPSILON as f64;
    let rho=1025.;
    let close=|got:f32,want:f64,what:&str|{
        let tol=4.*eps*want.abs()+1e-9;
        assert!((got as f64-want).abs()<=tol,"{what} : {got} contre {want}");
    };
    for a in [0.05,0.1] {
        let wave=standing::StandingWave{a,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho};
        for t in [0.,0.31,0.77,1.13] {
            for x in [0.1,0.55,1.3,1.9] {
                // 1. Continuité : les formules bornées en z = 0 rendent le fond analytique.
                let (s0,f0)=(wave.sample(x,0.,t),wave.bounded_fields(x,0.,t));
                for i in [0,2] {
                    close(s0.u[i],f0.u[i],"U(0)");
                    close(s0.du_dt[i],f0.du_dt[i],"U_t(0)");
                    close(s0.grad_p_dyn[i],f0.grad_p[i],"grad P(0)");
                }
                close(s0.p_dyn,f0.p,"P(0)");
                for z in [1e-4,0.02,0.08,0.15] {
                    let s=wave.sample_bounded(x,z,t);
                    let f=wave.bounded_fields(x,z,t);
                    // 2. Divergence nulle à l'arrondi.
                    let div=s.grad_u[0][0]+s.grad_u[2][2];
                    assert!((div as f64).abs()<=4.*eps*(s.grad_u[0][0] as f64).abs()+1e-12,"div {div}");
                    // 3. Dérivées contre différences finies centrées du champ publié.
                    let h=1e-4;
                    let fx=|dx:f64,dz:f64,dt:f64|wave.bounded_fields(x+dx,z+dz,t+dt);
                    let (xp,xm,zp,zm,tp,tm)=(fx(h,0.,0.),fx(-h,0.,0.),fx(0.,h,0.),fx(0.,-h,0.),fx(0.,0.,h),fx(0.,0.,-h));
                    let d=|p:f64,m:f64|(p-m)/(2.*h);
                    let trunc=|scale:f64|1e-6*scale.abs()+1e-9;
                    for i in [0,2] {
                        let (ux,uz,ut)=(d(xp.u[i],xm.u[i]),d(zp.u[i],zm.u[i]),d(tp.u[i],tm.u[i]));
                        assert!((f.grad_u[i][0]-ux).abs()<=trunc(ux),"dx u{i} {} contre {ux}",f.grad_u[i][0]);
                        assert!((f.grad_u[i][2]-uz).abs()<=trunc(uz),"dz u{i} {} contre {uz}",f.grad_u[i][2]);
                        assert!((f.du_dt[i]-ut).abs()<=trunc(ut),"dt u{i} {} contre {ut}",f.du_dt[i]);
                        close(s.u[i],f.u[i],"u f32");
                        close(s.du_dt[i],f.du_dt[i],"u_t f32");
                        for j in [0,2] {close(s.grad_u[i][j],f.grad_u[i][j],"grad u f32");}
                        // Laplacien : différences secondes, pas plus grand pour la troncature.
                        let hl=1e-3;
                        let g=|dx:f64,dz:f64|wave.bounded_fields(x+dx,z+dz,t).u[i];
                        let lap=(g(hl,0.)+g(-hl,0.)+g(0.,hl)+g(0.,-hl)-4.*g(0.,0.))/(hl*hl);
                        assert!((f.laplacian[i]-lap).abs()<=1e-5*lap.abs()+1e-6,"lap u{i} {} contre {lap}",f.laplacian[i]);
                        close(s.laplacian_u[i],f.laplacian[i],"lap f32");
                    }
                    let (px,pz)=(d(xp.p,xm.p),d(zp.p,zm.p));
                    assert!((f.grad_p[0]-px).abs()<=trunc(px),"dxP {} contre {px}",f.grad_p[0]);
                    assert!((f.grad_p[2]-pz).abs()<=trunc(pz),"dzP {} contre {pz}",f.grad_p[2]);
                    close(s.p_dyn,f.p,"P f32");
                    // 5. Résidu contracté publié contre sa recomposition f64.
                    let r=s.momentum_residual(rho as f32,0.).unwrap();
                    for i in [0,2] {
                        let want=f.du_dt[i]+f.u[0]*f.grad_u[i][0]+f.u[2]*f.grad_u[i][2]+f.grad_p[i]/rho;
                        let tol=1e-5*want.abs()+16.*eps*f.du_dt[i].abs();
                        assert!((r[i] as f64-want).abs()<=tol,"S{i} {} contre {want}",r[i]);
                    }
                }
            }
        }
    }
    // 5 bis. Ordre deux : `z = s·a` sous la crête, `S/a²` stable entre 5 et 10 cm, à la dérive du
    // facteur `T + kz` près. À `z` fixe, le résidu linéaire serait d'ordre un.
    let (small,large)=(standing::StandingWave{a:0.05,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho},
        standing::StandingWave{a:0.1,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho});
    let residual=|w:&standing::StandingWave,x:f64,s:f64,t:f64|{
        let f=w.bounded_fields(x,s*w.a,t);
        [0,2].map(|i|f.du_dt[i]+f.u[0]*f.grad_u[i][0]+f.u[2]*f.grad_u[i][2]+f.grad_p[i]/rho)
    };
    let mut compared=0;
    for t in [0.,0.31,0.77] {for x in [0.1,0.55,1.3] {for s in [0.3,0.8,1.] {
        let (r1,r2)=(residual(&small,x,s,t),residual(&large,x,s,t));
        for i in 0..2 {
            if r2[i].abs()>1e-3 {
                let ratio=r2[i]/(4.*r1[i]);
                println!("S254 ordre_deux x={x} s={s} t={t} axe={i} rapport={ratio:.4}");
                assert!((0.75..=1.3).contains(&ratio),"S/a2 : rapport {ratio} (x={x} s={s} t={t})");
                compared+=1;
            }
        }
    }}}
    assert!(compared>=20,"trop peu de résidus comparés : {compared}");
}

/// S254 (ADR-154, protocole §1.3) — **intégration** : `BackgroundFaces` rempli par le fournisseur B de
/// production prolongé, faces au-dessus du plan moyen comprises, consommé par des pas couplés mobiles
/// depuis le repos. Aucune précision n'est revendiquée sous B réel : bassin à murs, `W(fond) ≠ 0`.
#[test]
fn production_background_extended_feeds_coupled_mobile_steps_s254(){
    use crate::{background::{Background,SeaState},types::WorldPos};
    let sea=SeaState{hs:0.3,tp:4.,theta_turns:0.,components:1,graine:254};
    let b=Background::configure(&mut HostServices{alloc:&mut Arena,jobs:&Jobs,sink:&Jobs},sea,
        WorldPos::from_units(0,0,0)).unwrap();
    let nx=32;let dx=2./nx as f32;
    let mut v=Volume::configure(&mut HostServices{alloc:&mut Arena,jobs:&Jobs,sink:&Jobs},
        Domain{nx,nz:(2.25/dx).round() as usize,dx},1025.,b.gravity(),&vec![0.;nx]).unwrap();
    v.set_free_surface(&vec![2.;nx],2.).unwrap();
    let (mut u,mut w)=fields(&v);
    let (nz,rest)=(v.domain.nz,v.rest);
    let mut above_wet=0usize;
    for n in 0..50u64 {
        let t=SimTime(n*1000);
        for k in 0..nz {for i in 0..=nx {
            u[v.fu(i,k)]=b.differential_local_extended([i as f32*dx,0.,(k as f32+0.5)*dx-rest],t,1025.).unwrap();
        }}
        for k in 0..=nz {for i in 0..nx {
            w[v.fw(i,k)]=b.differential_local_extended([(i as f32+0.5)*dx,0.,k as f32*dx-rest],t,1025.).unwrap();
        }}
        for k in 0..nz {for i in 0..=nx {
            let z=(k as f32+0.5)*dx-rest;
            if z>0.&&z<u[v.fu(i,k)].eta {above_wet+=1;}
        }}
        let bg=BackgroundFaces{domain:v.domain,time:t,density:1025.,gravity:b.gravity(),u:&u,w:&w};
        let r=v.step_perturbation_mobile(t,1000,4000,1_000_000_000,&bg,Sponge::default(),&Jobs,&Clock)
            .unwrap_or_else(|e|panic!("pas {n} : {e:?}"));
        assert_eq!(r.advanced_us,1000);
    }
    // Où vit la perturbation : les murs imposent `v·n = −U·n`, qu'une onde progressive ne satisfait pas.
    let (mut wall,mut inner)=(0f32,0f32);
    for k in 0..nz {for i in 0..=nx {
        let x=v.u[v.fu(i,k)].abs();
        if i<4||i>nx-4 {wall=wall.max(x)} else {inner=inner.max(x)}
    }}
    println!("S254 integration faces_mouillees_au_dessus_du_plan_moyen={above_wet} u_prime_max_murs={wall:.3e} u_prime_max_interieur={inner:.3e} U_amplitude={:.3e}",
        u.iter().fold(0f32,|m,s|m.max(s.u[0].abs())));
    assert!(above_wet>0,"aucune face mouillée au-dessus du plan moyen : le prolongement n'est pas consommé");
    assert!(v.u.iter().chain(&v.w).chain(&v.p).chain(&v.eta).all(|x|x.is_finite()));
    assert!(b.differential_local([0.,0.,0.03],SimTime(0),1025.).is_err(),"ADR-113 refuse toujours z > 0");
}


#[test]
fn surface_sponge_exact_decay_compensation_and_interior_s268() {
    let sponge=Sponge{width_m:2.,rate_per_s:4.};
    for sign in [-1.,1.] {
        for steps in [1,1000] {
            let mut v=volume();
            let rest=3.;
            v.set_free_surface(&[rest;16],rest).unwrap();
            for i in 0..16 {v.eta[i]+=sign*0.05;v.eta_roundoff[i]=sign*3e-8;}
            let initial=v.eta.clone();let remainders=v.eta_roundoff.clone();
            let dt=0.25/steps as f64;
            for _ in 0..steps {v.relax_surface(sponge,dt,&mut Control::unlimited()).unwrap();}
            for i in 0..16 {
                let x=(i as f64+0.5)*0.5;
                let ramp=(1.-x.min(8.-x)/2.).max(0.);
                let amplitude=initial[i] as f64-rest as f64-remainders[i] as f64;
                let expected=rest as f64+amplitude*(-4.*ramp*ramp*0.25).exp();
                let got=v.eta[i] as f64-v.eta_roundoff[i] as f64;
                let bound=8.*f32::EPSILON as f64*rest as f64
                    +8.*f32::EPSILON as f64*steps as f64*amplitude.abs();
                assert!((got-expected).abs()<=bound,"{steps} {i}: {got} / {expected}");
                assert!((got-rest as f64).abs()<=amplitude.abs()+bound);
                if ramp==0. {
                    assert_eq!(v.eta[i].to_bits(),initial[i].to_bits());
                    assert_eq!(v.eta_roundoff[i].to_bits(),remainders[i].to_bits());
                }
            }
            let before=(v.eta.clone(),v.eta_roundoff.clone());
            v.relax_surface(Sponge::default(),0.25,&mut Control::unlimited()).unwrap();
            assert!(same_bits(&v.eta,&before.0)&&same_bits(&v.eta_roundoff,&before.1));
        }
    }
    // Un reliquat sub-ulp doit lui aussi décroître, pas revenir intégralement au pas suivant.
    let mut v=volume();v.rest=3.;v.eta.fill(3.);v.eta_roundoff.fill(1e-7);
    let f=sponge.factor(0.25,8.,0.25);
    v.relax_surface(sponge,0.25,&mut Control::unlimited()).unwrap();
    assert!((v.eta_roundoff[0]-f*1e-7).abs()<1e-14);
}

#[test]
fn mobile_step_consumes_surface_sponge_without_damping_background_s268() {
    let wave=standing::StandingWave{a:0.05,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho:1025.};
    let nx=16;
    let mut a=standing_case(nx,&wave,&vec![2.;nx]);
    let mut b=standing_case(nx,&wave,&vec![2.;nx]);
    let sponge=Sponge{width_m:0.5,rate_per_s:4.};
    let mut changed=0;
    for n in 0..20 {
        // Témoin depuis exactement le même état, nouvelle préparation à chaque pas.
        b.u.copy_from_slice(&a.u);b.w.copy_from_slice(&a.w);b.p.copy_from_slice(&a.p);
        b.eta.copy_from_slice(&a.eta);b.eta_roundoff.copy_from_slice(&a.eta_roundoff);
        let (u,w)=standing_faces(&a,&wave,n as f64*0.001);
        let snapshots=(format!("{u:?}"),format!("{w:?}"));
        let bg=BackgroundFaces{domain:a.domain,time:SimTime(n*1000),density:1025.,gravity:9.81,u:&u,w:&w};
        HEIGHT_RELAXATION_OFF.with(|v|v.set(true));
        let witness=b.step_perturbation_mobile(bg.time,1000,4000,1_000_000,&bg,sponge,&Jobs,&Clock);
        HEIGHT_RELAXATION_OFF.with(|v|v.set(false));
        assert_eq!(witness.unwrap().advanced_us,1000);
        let r=a.step_perturbation_mobile(bg.time,1000,4000,1_000_000,&bg,sponge,&Jobs,&Clock).unwrap();
        assert_eq!(r.advanced_us,1000);
        assert!(same_bits(&a.u,&b.u)&&same_bits(&a.w,&b.w)&&same_bits(&a.p,&b.p));
        for i in 0..nx {
            let x=(i as f64+0.5)*2./nx as f64;
            let ramp=(1.-x.min(2.-x)/0.5).max(0.);
            let factor=(-4.*ramp*ramp*0.001).exp();
            let expected=2.+(b.eta[i] as f64-2.-b.eta_roundoff[i] as f64)*factor;
            let got=a.eta[i] as f64-a.eta_roundoff[i] as f64;
            assert!((got-expected).abs()<8.*f32::EPSILON as f64*2.);
            if ramp==0. {assert!(same_bits(&a.eta[i..i+1],&b.eta[i..i+1]));}
            changed+=usize::from(a.eta[i].to_bits()!=b.eta[i].to_bits()
                || a.eta_roundoff[i].to_bits()!=b.eta_roundoff[i].to_bits());
        }
        assert_eq!((format!("{u:?}"),format!("{w:?}")),snapshots);
    }
    assert!(changed>0,"le témoin doit distinguer la relaxation");
    println!("S268 integration : 20 pas, {changed} hauteurs/restes modifies, fond intact");
}


/// État exact stationnaire : courant et élévation constants, pression hydrostatique.
#[test]
fn uniform_through_background_preserves_flat_surface_s270() {
    for dx in [0.25f32,0.125] { for speed in [-0.5f32,0.5] { for elevation in [-0.125f32,0.125] {
        let nx=(4./dx) as usize;
        let mut v=Volume::configure(&mut HostServices{alloc:&mut Arena,jobs:&Jobs,sink:&Jobs},
            Domain{nx,nz:(3./dx) as usize,dx},1025.,9.81,&vec![0.;nx]).unwrap();
        v.set_free_surface(&vec![2.;nx],2.).unwrap();
        let (mut u,mut w)=fields(&v);
        for s in u.iter_mut().chain(&mut w) {
            s.u[0]=speed;s.eta=elevation;s.p_dyn=(1025f32*9.81)*elevation;
        }
        for n in 0..20 {
            let bg=BackgroundFaces{domain:v.domain,time:SimTime(n*1000),density:1025.,gravity:9.81,u:&u,w:&w};
            let r=v.step_perturbation_mobile(bg.time,1000,4000,1_000_000,&bg,Sponge::default(),&Jobs,&Clock).unwrap();
            assert_eq!(r.advanced_us,1000);
            let error=v.eta.iter().map(|h|(h-2.).abs()).fold(0f32,f32::max);
            if n==19 { println!("S270 uniforme dx={dx} U={speed} a={elevation} erreur={error:e}"); }
            assert!(error<=8.*f32::EPSILON,"surface artificielle {error}");
            assert!(v.u.iter().chain(&v.w).all(|x|x.abs()<=8.*f32::EPSILON));
        }
    }}}
}


#[test]
fn boundary_band_integral_and_global_balance_s270() {
    let mut v=volume();v.set_free_surface(&vec![2.;16],2.).unwrap();
    let (mut u,w)=fields(&v);
    for (bottom,rest,surface,speed) in [(0.,2.,2.375,0.7),(0.,2.,1.625,-0.7),
        (1.9,2.,1.8,0.7),(2.1,2.,2.375,-0.7)] {
        v.bottom[0]=bottom;v.rest=rest;v.eta[0]=surface;
        for sample in &mut u {sample.u[0]=speed;}
        let bg=input(&v,&u,&w);
        let got=v.boundary_background_band(0,&bg,&mut Control::unlimited()).unwrap();
        let exact=speed as f64*((surface as f64-bottom as f64).max(0.)
            -(rest as f64-bottom as f64).max(0.));
        assert!((got as f64-exact).abs()<2e-7,"integrale {got} / {exact}");
    }
    v.bottom[0]=0.;v.rest=2.;v.eta.fill(2.);v.surface_total.fill(2.);
    // Flux différents aux deux extrémités ; profil intérieur arbitraire, télescopage exact.
    for k in 0..v.domain.nz {for i in 0..=16 {
        let f=v.fu(i,k);u[f].eta=if i==0 {0.125} else if i==16 {-0.25} else {0.};
        u[f].u[0]=0.5;
    }}
    let bg=input(&v,&u,&w);
    v.transport_coupled(0.002,&bg,&mut Control::unlimited()).unwrap();
    let mass:f64=v.eta.iter().zip(&v.eta_roundoff).map(|(h,r)|(*h as f64-2.-*r as f64)*0.5).sum();
    let exact=-0.001*((-0.25*0.5)-(0.125*0.5));
    assert!((mass-exact).abs()<1e-10,"bilan {mass} / {exact}");
    assert!((v.eta[0]-2.000125).abs()<3e-7);
    assert!((v.eta[15]-2.00025).abs()<3e-7);
}

#[test]
fn incoherent_boundary_elevation_refuses_atomically_s270() {
    let mut v=volume();v.set_free_surface(&vec![2.;16],2.).unwrap();
    let (mut u,w)=fields(&v);let f=v.fu(0,1);u[f].eta=0.125;
    let bg=input(&v,&u,&w);
    let before=(v.u.clone(),v.w.clone(),v.p.clone(),v.eta.clone(),v.eta_roundoff.clone());
    let r=v.step_perturbation_mobile(bg.time,1000,4000,1_000_000,&bg,Sponge::default(),&Jobs,&Clock);
    assert_eq!(r,Err(Error::BackgroundContext));
    assert_eq!((v.u.clone(),v.w.clone(),v.p.clone(),v.eta.clone(),v.eta_roundoff.clone()),before);
    assert!(!v.surface_coupled&&!v.mobile);
}


#[test]
fn boundary_surface_outside_domain_refuses_s270() {
    let mut v=volume();v.set_free_surface(&vec![2.;16],2.).unwrap();
    let (mut u,w)=fields(&v);
    for k in 0..v.domain.nz {let f=v.fu(16,k);u[f].eta=2.;}
    let bg=input(&v,&u,&w);
    let before=v.eta.clone();
    assert_eq!(v.step_perturbation_mobile(bg.time,1000,4000,1_000_000,&bg,Sponge::default(),&Jobs,&Clock),Err(Error::Domain));
    assert_eq!(v.eta,before);assert!(!v.surface_coupled);
}
