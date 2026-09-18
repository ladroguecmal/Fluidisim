//! S272 : résidu progressif réel, voir RESIDU-TEMPOREL-S272.
#[path="../../water-harness/src/host_impl.rs"]
#[allow(dead_code)] mod host_impl;
#[path="support/progressive_background.rs"] mod wave;
use water_core::{background::BackgroundSample,delta_projection::{BackgroundFaces,Domain,Sponge,Volume},
    host::{HostServices,MonotonicClock},SimTime};
struct Frozen;
impl MonotonicClock for Frozen {fn now_ns(&self)->u64{0}}
fn run()->Result<(),String> {
    let args:Vec<_>=std::env::args().collect();
    let dx:f32=args.get(1).ok_or("dx requis")?.parse().map_err(|_|"dx")?;
    let a:f64=args.get(2).ok_or("amplitude requise")?.parse().map_err(|_|"a")?;
    let dt:u64=args.get(3).ok_or("dt_us requis")?.parse().map_err(|_|"dt")?;
    if ![0.125,0.0625,0.03125].contains(&dx) || ![0.01,0.005].contains(&a) || ![2000,1000].contains(&dt) {return Err("fixture non declaree".into());}
    let domain=Domain{nx:(4./dx) as usize,nz:(1.5/dx) as usize,dx};
    let jobs=host_impl::SequentialJobs;let sink=host_impl::StderrSink;
    let mut arena=host_impl::ArenaAllocator::with_capacity(1<<27);
    let mut v=Volume::configure(&mut HostServices{alloc:&mut arena,jobs:&jobs,sink:&sink},domain,1025.,9.81,&vec![0.;domain.nx]).map_err(|e|format!("{e:?}"))?;
    v.set_free_surface(&vec![1.;domain.nx],1.).map_err(|e|format!("{e:?}"))?;
    let mut u=vec![BackgroundSample::default();v.velocity_u().len()];
    let mut w=vec![BackgroundSample::default();v.velocity_w().len()];
    println!("CAS dx={dx} a={a} dt_us={dt}");
    let mut worst=0;
    for n in 0..2_000_000/dt {
        let time=SimTime(n*dt);let t=time.0 as f64*1e-6;
        for k in 0..domain.nz {for i in 0..=domain.nx {u[k*(domain.nx+1)+i]=wave::sample(a,i as f64*dx as f64,(k as f64+0.5)*dx as f64-1.,t);}}
        for k in 0..=domain.nz {for i in 0..domain.nx {w[k*domain.nx+i]=wave::sample(a,(i as f64+0.5)*dx as f64,k as f64*dx as f64-1.,t);}}
        let bg=BackgroundFaces{domain,time,density:1025.,gravity:9.81,u:&u,w:&w};
        let r=v.step_perturbation_mobile(time,dt,6000,1_000_000,&bg,Sponge::default(),&jobs,&Frozen)
            .map_err(|e|format!("REFUS pas={n} t={t} erreur={e:?}"))?;
        if r.advanced_us!=dt {return Err("pas incomplet".into());}
        worst=worst.max(r.report.unwrap().iterations);
        if (n+1)*dt%50_000==0 {
            print!("TRACE t={:.6}",(n+1) as f64*dt as f64*1e-6);
            for h in v.surface() {print!(" {:.12}",*h as f64-1.);}
            println!();
        }
    }
    println!("FIN iterations_max={worst}");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
