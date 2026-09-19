//! S298 : les cas limites progressifs/absorbants 3D contre le chemin 2D reçu.
//! Aucun champ de simulation sérialisé : seulement les maxima et énergies de jauge.
#[path="../../water-harness/src/host_impl.rs"] #[allow(dead_code)] mod host_impl;
#[path="support/progressive_background.rs"] mod wave;
#[path="support/reflection_packet.rs"] mod packet;
#[path="support/delta3d_background.rs"] #[allow(dead_code)] mod sample3;
use water_core::{background::BackgroundSample,delta_projection::{Volume,Domain,BackgroundFaces,Sponge},
    delta3d::{Volume3,Domain3,Sponge3},host::{HostServices,MonotonicClock},SimTime};
struct Frozen;
impl MonotonicClock for Frozen {fn now_ns(&self)->u64{0}}
fn run(case:&str,dx:f32,axis:usize)->Result<(),String> {
    let progressive=case=="progressive";
    let length=if progressive {4.} else {24.};
    let (nx,nz)=((length/dx) as usize,(1.5/dx) as usize);
    let d=Domain3{nx:if axis==0 {nx} else {1},ny:if axis==0 {1} else {nx},nz,dx};
    let d2=Domain{nx,nz,dx};
    let mut arena=host_impl::ArenaAllocator::with_capacity(1<<28);
    let jobs=host_impl::SequentialJobs;let sink=host_impl::StderrSink;
    let mut host=HostServices{alloc:&mut arena,jobs:&jobs,sink:&sink};
    let mut v2=Volume::configure(&mut host,d2,1025.,9.81,&vec![0.;nx]).unwrap();
    let mut v3=Volume3::configure(&mut host,d,1025.,9.81).unwrap();
    let modes=packet::modes();
    let eta:Vec<_>=(0..nx).map(|i| (1.+if progressive {0.} else {packet::sample(&modes,(i as f64+0.5)*dx as f64,0.,0.)[0]}) as f32).collect();
    v2.set_free_surface(&eta,1.).unwrap();v3.set_free_surface(&eta,1.).unwrap();
    if !progressive {
        let mut u=vec![0.;v2.velocity_u().len()];let mut w=vec![0.;v2.velocity_w().len()];
        for k in 0..nz {for i in 1..nx {u[k*(nx+1)+i]=packet::sample(&modes,i as f64*dx as f64,(k as f64+0.5)*dx as f64-1.,0.)[1] as f32;}}
        for k in 1..=nz {for i in 0..nx {w[k*nx+i]=packet::sample(&modes,(i as f64+0.5)*dx as f64,k as f64*dx as f64-1.,0.)[2] as f32;}}
        v2.set_velocity(&u,&w).unwrap();
        if axis==0 {v3.set_velocity(&u,&vec![0.;v3.velocity_v().len()],&w).unwrap();}
        else {v3.set_velocity(&vec![0.;v3.velocity_u().len()],&u,&w).unwrap();}
    }
    let mut samples=sample3::Samples3::new(d);
    let mut u=vec![BackgroundSample::default();v2.velocity_u().len()];
    let mut w=vec![BackgroundSample::default();v2.velocity_w().len()];
    let k=std::f64::consts::PI;let omega=(9.81*k*k.tanh()).sqrt();
    let cg=0.5*omega/k*(1.+2.*k/(2.*k).sinh());
    let s=if case=="eponge" {Sponge{width_m:4.,rate_per_s:(10.*cg/4.) as f32}} else {Sponge::default()};
    let s3=Sponge3{width_x:if axis==0 {s.width_m} else {0.},width_y:if axis==1 {s.width_m} else {0.},rate_per_s:s.rate_per_s};
    let dt=if progressive {1000} else {5000};let steps=if progressive {2000} else {7200};
    let (mut max_h,mut max_u,mut max_slope)=(0f32,0f32,0f32);
    let (mut gauge_error,mut incident,mut returned2,mut returned3)=(0f64,0f64,0f64,0f64);
    let (mut it2,mut it3)=(0,0);
    for n in 0..steps {
        let time=SimTime(n*dt);let t=time.0 as f64*1e-6;
        if progressive {
            samples.fill(1.,|x,y,z|if axis==0 {wave::sample(0.01,x,z,t)} else {sample3::rotate(wave::sample(0.01,y,z,t),[0.,1.])});
            for k in 0..nz {for i in 0..=nx {u[k*(nx+1)+i]=wave::sample(0.01,i as f64*dx as f64,(k as f64+0.5)*dx as f64-1.,t);}}
            for k in 0..=nz {for i in 0..nx {w[k*nx+i]=wave::sample(0.01,(i as f64+0.5)*dx as f64,k as f64*dx as f64-1.,t);}}
        }
        let bg=BackgroundFaces{domain:d2,time,density:1025.,gravity:9.81,u:&u,w:&w};
        let r2=v2.step_perturbation_mobile(time,dt,6000,1_000_000,&bg,s,&jobs,&Frozen).map_err(|e|format!("2D {case} dx={dx} n={n}: {e:?}"))?;
        if r2.advanced_us!=dt {return Err("2D pas incomplet".into());}
        let r3=v3.step_perturbation_mobile(time,dt,6000,&samples.view(time),s3,&jobs).map_err(|e|format!("3D {case} dx={dx} axe={axis} n={n}: {e:?}"))?;
        it2=it2.max(r2.report.unwrap().iterations);it3=it3.max(r3.iterations);
        for (a,b) in v2.surface().iter().zip(v3.surface()) {max_h=max_h.max((a-b).abs());}
        for (a,b) in v2.velocity_u().iter().zip(if axis==0 {v3.velocity_u()} else {v3.velocity_v()}).chain(v2.velocity_w().iter().zip(v3.velocity_w())) {max_u=max_u.max((a-b).abs());}
        for i in 1..nx {let a=v2.surface();let b=v3.surface();max_slope=max_slope.max(((a[i]-a[i-1])-(b[i]-b[i-1])).abs()/dx);}
        if !progressive {
            let q=12./dx as f64-0.5;let i=q.floor() as usize;let f=q-i as f64;
            let gauge=|h:&[f32]|(1.-f)*(h[i] as f64-1.)+f*(h[i+1] as f64-1.);
            let (a,b)=(gauge(v2.surface()),gauge(v3.surface()));let dt_s=dt as f64*1e-6;
            gauge_error+=(a-b).powi(2)*dt_s;
            if (n+1)*dt<=12_000_000 {incident+=a*a*dt_s;}
            if (n+1)*dt>=14_000_000 {returned2+=a*a*dt_s;returned3+=b*b*dt_s;}
        }
    }
    let gauge=if progressive {0.} else {(gauge_error/incident).sqrt()};
    println!("S298 cas={case} dx={dx} axe={axis} pas={steps} hauteur_max_m={max_h:e} pente_max={max_slope:e} vitesse_max={max_u:e} jauge_ecart={gauge:e} retour2={returned2:e} retour3={returned3:e} iterations2={it2} iterations3={it3}");
    if max_h>0.003 || !max_h.is_finite() || gauge>0.001 || !gauge.is_finite() {return Err("equivalence hors seuil S201 / garde S269".into());}
    Ok(())
}
fn main()->Result<(),String> {
    let selected=std::env::args().nth(1).unwrap_or_else(||"all".into());
    if !["all","progressive","packet"].contains(&selected.as_str()) {return Err("all | progressive | packet".into());}
    if selected!="packet" {for dx in [0.125,0.0625,0.03125] {for axis in [0,1] {run("progressive",dx,axis)?;}}}
    if selected!="progressive" {for case in ["mur","eponge"] {for dx in [0.25,0.125] {for axis in [0,1] {run(case,dx,axis)?;}}}}
    Ok(())
}
