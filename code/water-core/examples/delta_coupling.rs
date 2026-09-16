//! S250 : B et pression W planaires consommés par le candidat MAC, surface imposée.
#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;
use std::time::Instant;
use water_core::{Background,SeaState,WorldPos,SimTime,background::BackgroundSample,
    delta_projection::{BackgroundFaces,Domain,Sponge,Volume},
    host::{Allocator,HostServices,MonotonicClock},modal_pressure::Segment,spectral_pressure::{self,Node,Slot}};
struct Clock(Instant);
impl MonotonicClock for Clock {fn now_ns(&self)->u64 {self.0.elapsed().as_nanos() as u64}}
// Somme de tous les champs, avant contraction ; aucun résidu par source additionné.
fn sum(mut a:BackgroundSample,b:BackgroundSample)->BackgroundSample {
    a.eta+=b.eta;a.p_dyn+=b.p_dyn;
    for i in 0..3 {
        a.grad_eta[i]+=b.grad_eta[i];a.u[i]+=b.u[i];a.du_dt[i]+=b.du_dt[i];
        a.grad_p_dyn[i]+=b.grad_p_dyn[i];a.laplacian_u[i]+=b.laplacian_u[i];
        for j in 0..3 {a.grad_u[i][j]+=b.grad_u[i][j];}
    }
    a
}
fn main() {
    let jobs=host_impl::SequentialJobs;let sink=host_impl::StderrSink;
    let mut arena=host_impl::ArenaAllocator::with_capacity(1<<24);
    let mut host=HostServices{alloc:&mut arena,jobs:&jobs,sink:&sink};
    let nx=if std::env::args().any(|s|s=="--fine") {32} else {16};
    let flat=std::env::args().any(|s|s=="--flat");
    let domain=Domain{nx,nz:nx/2,dx:8./nx as f32};
    let mut v=Volume::configure(&mut host,domain,1025.,9.81,&vec![0.;nx]).unwrap();
    let mut witness=Volume::configure(&mut host,domain,1025.,9.81,&vec![0.;nx]).unwrap();
    let amplitude=if flat {0.} else {0.01};
    let eta:Vec<_>=(0..domain.nx).map(|i|domain.z0()+amplitude*(std::f32::consts::PI*(i as f32+0.5)/domain.nx as f32).cos()).collect();
    v.set_surface(&eta).unwrap();witness.set_surface(&eta).unwrap();
    let b=Background::configure(&mut host,SeaState{hs:0.1,tp:4.,theta_turns:0.,components:1,graine:7},
        WorldPos::default()).unwrap();
    let nodes=[Node{k:[0.7,0.],transform:1.,weight:0.5},Node{k:[1.2,0.],transform:1.,weight:0.5}];
    let path=[Segment{birth:SimTime(0),duration_us:2_000_000,origin:[0.;2],velocity:[0.5,0.],pressure_pa:80.}];
    let mut slots=[Slot::default();2];
    let mut u=vec![BackgroundSample::default();v.velocity_u().len()];
    let mut w=vec![BackgroundSample::default();v.velocity_w().len()];
    let mut no_source_u=u.clone();let mut no_source_w=w.clone();
    let mut max_cross=0f32;let mut max_div=0f64;let mut refinements=0;
    let mut samples=Vec::with_capacity(20);let mut preparation=Vec::with_capacity(20);
    arena.seal();
    for step in 0..20 {
        let time=SimTime(1_000_000+step*1000);
        let start=Instant::now();
        let field=spectral_pressure::prepare(&nodes,&path,9.81,1025.,time,SimTime(2_000_000),
            [-16.;2],[16.;2],&mut slots).unwrap();
        let mut sample=|p| {
            let a=b.differential_local(p,time,1025.).unwrap();let c=field.differential(p).unwrap().water;
            let both=sum(a,c);
            let sa=a.momentum_residual(1025.,0.).unwrap();let sc=c.momentum_residual(1025.,0.).unwrap();
            let total=both.momentum_residual(1025.,0.).unwrap();
            for axis in [0,2] {max_cross=max_cross.max((total[axis]-sa[axis]-sc[axis]).abs());}
            both
        };
        for k in 0..domain.nz {for i in 0..=domain.nx {
            u[k*(domain.nx+1)+i]=sample([i as f32*domain.dx,0.,(k as f32+0.5)*domain.dx-domain.z0()]);
        }}
        for k in 0..=domain.nz {for i in 0..domain.nx {
            w[k*domain.nx+i]=sample([(i as f32+0.5)*domain.dx,0.,k as f32*domain.dx-domain.z0()]);
        }}
        preparation.push(start.elapsed().as_secs_f64()*1000.);
        let bg=BackgroundFaces{domain,time,density:1025.,gravity:9.81,u:&u,w:&w};
        let start=Instant::now();
        let result=v.step_perturbation(time,1000,2000,1_000_000,&bg,
            Sponge{width_m:1.,rate_per_s:2.},&jobs,&Clock(Instant::now()));
        let report=result.unwrap();
        samples.push(start.elapsed().as_secs_f64()*1000.);
        assert_eq!(report.advanced_us,1000);
        max_div=max_div.max(report.report.unwrap().divergence);
        refinements+=report.report.unwrap().refinements;
        assert_eq!(v.surface(),eta);
        if flat && step==0 {assert_eq!(report.report.unwrap().refinements,1);}
        if step==0 {
            no_source_u.copy_from_slice(&u);no_source_w.copy_from_slice(&w);
            for s in no_source_u.iter_mut().chain(&mut no_source_w) {
                let r=s.momentum_residual(1025.,0.).unwrap();
                for axis in [0,2] {s.du_dt[axis]-=r[axis];}
            }
            let muted=BackgroundFaces{u:&no_source_u,w:&no_source_w,..bg};
            let r=witness.step_perturbation(time,1000,2000,1_000_000,&muted,Sponge{width_m:1.,rate_per_s:2.},
                &jobs,&Clock(Instant::now())).unwrap();
            assert_eq!(r.advanced_us,1000);
            let difference=v.velocity_u().iter().chain(v.velocity_w())
                .zip(witness.velocity_u().iter().chain(witness.velocity_w()))
                .fold(0f32,|m,(x,y)|m.max((x-y).abs()));
            assert!(difference>1e-8,"source sans effet : {difference}");
            println!("first_step_source_difference={difference:e}");
        }
    }
    assert!(max_cross>1e-5,"la contraction séparée doit perdre les termes croisés");
    samples.sort_by(f64::total_cmp);preparation.sort_by(f64::total_cmp);
    println!("S251 nx={nx} nz={} flat={flat} steps=20 dt_us=1000 B=1 W=2 refinements={refinements} CPU sequentiel release",domain.nz);
    println!("cross_acceleration_max={max_cross:e} divergence_max={max_div:e}");
    println!("step_median_ms={:.6} step_max_ms={:.6} prepare_median_ms={:.6}",samples[10],samples[19],preparation[10]);
    println!("Surface imposee, bords de perturbation ; ni B4 global, ni I-05, ni raccordement mobile.");
}
