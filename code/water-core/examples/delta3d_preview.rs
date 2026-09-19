//! Aperçu S297 : deux ondes de fond croisées, impulsion locale et témoin sans impulsion.
//! Seules des images sont écrites ; les états δ ne quittent jamais la mémoire (I-17).
#[path="../../water-harness/src/host_impl.rs"]
#[allow(dead_code)] mod host_impl;
#[path="support/standing_background.rs"]
#[allow(dead_code)] mod standing;
#[path="support/delta3d_background.rs"]
#[allow(dead_code)] mod background3;
#[path="support/delta3d_render.rs"] mod render;
use water_core::{background::BackgroundSample,delta3d::{Domain3,Volume3,Sponge3},host::HostServices,SimTime};
use std::f64::consts::PI;
fn background(x:f64,y:f64,z:f64,t:f64)->BackgroundSample {
    let a=standing::StandingWave{a:0.10,k:PI/4.,h:2.,g:9.81,rho:1025.};
    let b=standing::StandingWave{a:0.07,k:PI/3.,h:2.,g:9.81,rho:1025.};
    background3::sum(a.sample(x,z,t),background3::rotate(b.sample(0.6*x+0.8*y,z,t),[0.6,0.8]))
}
fn main()->Result<(),String> {
    let output=std::env::args().nth(1).unwrap_or_else(||"viewer/captures/s297".into());
    std::fs::create_dir_all(&output).map_err(|e|e.to_string())?;
    let d=Domain3{nx:32,ny:24,nz:12,dx:0.25};
    let make=|| {let mut arena=host_impl::ArenaAllocator::with_capacity(1<<26);
        Volume3::configure(&mut HostServices{alloc:&mut arena,jobs:&host_impl::SequentialJobs,sink:&host_impl::StderrSink},d,1025.,9.81).unwrap()};
    let (mut pulse,mut control)=(make(),make());let mut eta=vec![2.;d.columns()];
    for j in 0..d.ny {for i in 0..d.nx {
        let x=(i as f32+0.5)*d.dx-3.;let y=(j as f32+0.5)*d.dx-2.5;let r=(x*x+y*y)/(2.*0.55*0.55);
        eta[j*d.nx+i]+=0.18*(1.-r)*(-r).exp();
    }}
    pulse.set_free_surface(&eta,2.).unwrap();control.set_free_surface(&vec![2.;d.columns()],2.).unwrap();
    let mut samples=background3::Samples3::new(d);let sponge=Sponge3{width_x:1.,width_y:1.,rate_per_s:2.};
    let mut total=vec![0.;d.columns()];let mut difference=total.clone();
    let (mut imax,mut refinements,mut dmax)=(0,0,0f64);
    for n in 0..=1200 {
        let time=SimTime(n*5000);let t=time.0 as f64*1e-6;
        if n%10==0 {
            for j in 0..d.ny {for i in 0..d.nx {let c=j*d.nx+i;
                let x=(i as f64+0.5)*d.dx as f64;let y=(j as f64+0.5)*d.dx as f64;
                total[c]=pulse.surface()[c]-2.+background(x,y,0.,t).eta;
                difference[c]=pulse.surface()[c]-control.surface()[c];
            }}
            let frame=n/10;let hash=render::render(&format!("{output}/frame_{frame:04}.ppm"),d.nx,d.ny,d.dx,&total,&difference).map_err(|e|e.to_string())?;
            let peak=difference.iter().fold(0f32,|m,x|m.max(x.abs()));
            println!("FRAME {frame} t={t:.2} peak_m={peak:.6} fnv={hash:016x}");
        }
        if n==1200 {break;}
        samples.fill(2.,|x,y,z|background(x,y,z,t));
        for (name,v) in [("pulse",&mut pulse),("control",&mut control)] {
            let r=v.step_perturbation_mobile(time,5000,4000,&samples.view(time),sponge,&host_impl::SequentialJobs)
                .map_err(|e|format!("{name} step {n}: {e:?}"))?;
            imax=imax.max(r.iterations);refinements+=r.refinements;dmax=dmax.max(r.divergence_plain);
        }
    }
    println!("PREVIEW nx=32 ny=24 nz=12 dt_us=5000 duration=6s frames=121 it_max={imax} refinements={refinements} divergence_plain_max={dmax:e}");
    Ok(())
}
