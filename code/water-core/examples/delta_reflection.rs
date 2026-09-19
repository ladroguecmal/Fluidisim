//! S269 : paquet progressif, jauge et garde de réflexion. Voir REFLEXION-PAQUET-S269.
#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;
use std::{f64::consts::PI, time::Instant};
use water_core::{background::BackgroundSample,delta_projection::{BackgroundFaces,Domain,Sponge,Volume},
    host::{HostServices,MonotonicClock},SimTime};
struct Frozen;
impl MonotonicClock for Frozen {fn now_ns(&self)->u64{0}}
#[path="support/reflection_packet.rs"]
mod packet;
use packet::{H,G,modes,sample};
#[cfg(test)] use packet::A;
fn gauge(v:&Volume,x:f64)->f64 {
    let q=x/v.domain().dx as f64-0.5;
    let i=q.floor() as usize;let f=q-i as f64;
    (1.-f)*(v.surface()[i] as f64-H)+f*(v.surface()[i+1] as f64-H)
}
fn run()->Result<(),String> {
    let args:Vec<_>=std::env::args().collect();
    let case=args.get(1).map(String::as_str).unwrap_or("garde");
    if !["garde","garde-gauche","garde-longue","garde-eponge","garde-longue-eponge","mur","eponge"].contains(&case) {return Err("cas : garde, garde-gauche, garde-longue, garde-eponge, garde-longue-eponge, mur, eponge".into());}
    let dx:f32=args.get(2).map(|s|s.parse()).transpose().map_err(|_|"dx invalide")?.unwrap_or(0.25);
    if ![0.25,0.125].contains(&dx) {return Err("dx : 0.25 ou 0.125".into());}
    let shift=if case=="garde-gauche" {24.} else {0.};
    let length=if case.contains("longue") {72.} else if case.starts_with("garde") {48.+shift} else {24.};
    let domain=Domain{nx:(length/dx) as usize,nz:(1.5/dx) as usize,dx};
    let jobs=host_impl::SequentialJobs;let sink=host_impl::StderrSink;
    let mut arena=host_impl::ArenaAllocator::with_capacity(1<<27);
    let mut v=Volume::configure(&mut HostServices{alloc:&mut arena,jobs:&jobs,sink:&sink},
        domain,1025.,G as f32,&vec![0.;domain.nx]).map_err(|e|format!("configure {e:?}"))?;
    let modes=modes();
    let eta:Vec<_>=(0..domain.nx).map(|i|(H+sample(&modes,(i as f64+0.5)*dx as f64-shift as f64,0.,0.)[0]) as f32).collect();
    v.set_free_surface(&eta,H as f32).map_err(|e|format!("surface {e:?}"))?;
    let mut u=vec![0.;v.velocity_u().len()];let mut w=vec![0.;v.velocity_w().len()];
    for k in 0..domain.nz {for i in 1..domain.nx {
        u[k*(domain.nx+1)+i]=sample(&modes,i as f64*dx as f64-shift as f64,(k as f64+0.5)*dx as f64-H,0.)[1] as f32;
    }}
    for k in 1..=domain.nz {for i in 0..domain.nx {
        w[k*domain.nx+i]=sample(&modes,(i as f64+0.5)*dx as f64-shift as f64,k as f64*dx as f64-H,0.)[2] as f32;
    }}
    v.set_velocity(&u,&w).map_err(|e|format!("vitesse {e:?}"))?;
    let bu=vec![BackgroundSample::default();u.len()];let bw=vec![BackgroundSample::default();w.len()];
    let omega=(G*PI*(PI*H).tanh()).sqrt();
    let cg=0.5*omega/PI*(1.+2.*PI*H/(2.*PI*H).sinh());
    let sponge=if case.contains("eponge") {Sponge{width_m:4.,rate_per_s:(10.*cg/4.) as f32}} else {Sponge::default()};
    println!("PAQUET cas={case} dx={dx} nx={} nz={} dt_us=5000 cg={cg} sigma={} fond=nul",domain.nx,domain.nz,sponge.rate_per_s);
    let (mut incident,mut returned,mut analytic_incident)=(0.,0.,0.);
    let start=Instant::now();let mut iters=0;
    for n in 0..7200u64 {
        let bg=BackgroundFaces{domain,time:SimTime(n*5000),density:1025.,gravity:G as f32,u:&bu,w:&bw};
        let r=v.step_perturbation_mobile(bg.time,5000,6000,1_000_000,&bg,sponge,&jobs,&Frozen)
            .map_err(|e|format!("REFUS cas={case} dx={dx} pas={n} t={} erreur={e:?}",n as f64*0.005))?;
        if r.advanced_us!=5000 {return Err("pas non avance".into());}
        iters=iters.max(r.report.unwrap().iterations);
        let t=(n+1) as f64*0.005;let y=gauge(&v,12.+shift as f64);
        if !y.is_finite(){return Err("jauge non finie".into());}
        if t<=12. {incident+=y*y*0.005;let a=sample(&modes,12.,0.,t)[0];analytic_incident+=a*a*0.005;}
        if t>=14. {returned+=y*y*0.005;}
        if args.iter().any(|a| a=="--trace-complete") || (n+1)%50==0 {println!("TRACE t={t:.3} eta={y:.10}");}
    }
    if incident<=0. {return Err("incident nul".into());}
    println!("MESURE cas={case} dx={dx} E_inc={incident:e} E_retour={returned:e} rapport_brut={} energie_inc_sur_lineaire={} iterations_max={iters} temps_s={}",
        (returned/incident).sqrt(),incident/analytic_incident,start.elapsed().as_secs_f64());
    if case.starts_with("garde") && (returned/incident).sqrt()>0.001 {
        return Err("GARDE_CONTAMINEE : mesure brute non recevable, controle differentiel separe requis".into());
    }
    Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packet_is_right_going_incompressible_and_kinematic_s269() {
        let m=modes();let e=1e-5;
        assert!((sample(&m,8.,0.,0.)[0]-A).abs()<1e-15);
        for x in [4.,8.,12.] {for t in [0.,1.,5.] {
            let eta_t=(sample(&m,x,0.,t+e)[0]-sample(&m,x,0.,t-e)[0])/(2.*e);
            assert!((eta_t-sample(&m,x,0.,t)[2]).abs()<1e-10);
            assert_eq!(sample(&m,x,-H,t)[2],0.);
            for z in [-0.8,-0.2,0.] {
                let ux=(sample(&m,x+e,z,t)[1]-sample(&m,x-e,z,t)[1])/(2.*e);
                let wz=(sample(&m,x,z+e,t)[2]-sample(&m,x,z-e,t)[2])/(2.*e);
                assert!((ux+wz).abs()<1e-10);
            }
        }}
        // Déplacement vers +x : le maximum du paquet est à droite après une courte durée.
        assert!(sample(&m,8.1,0.,0.05)[0]>sample(&m,7.9,0.,0.05)[0]);
    }
}
