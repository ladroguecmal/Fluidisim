//! S233 : coût complet de la surface linéarisée, sans rendu ni I/O dans la fenêtre.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::time::Instant;
use water_core::{delta_projection::{Domain,Volume},host::{Allocator,HostServices,MonotonicClock}};
struct Clock(Instant);
impl MonotonicClock for Clock {
    fn now_ns(&self)->u64 {self.0.elapsed().as_nanos().min(u64::MAX as u128) as u64}
}
fn main() {
    let jobs=host_impl::SequentialJobs;
    let sink=host_impl::StderrSink;
    println!("S233 CPU sequentiel, 8x4m fond0, eta=4+0.01cos(pi*x/8), g9.81 dt2000us");
    println!("3 chauffes puis11 mesures, etat reinitialise, plafond2000 iterations, budget1s");
    for nx in [16,32,64] {
        let mut arena=host_impl::ArenaAllocator::with_capacity(1<<24);
        let mut v=Volume::configure(&mut HostServices{alloc:&mut arena,jobs:&jobs,sink:&sink},
            Domain{nx,nz:nx/2,dx:8./nx as f32},1025.,9.81,&vec![0.;nx]).unwrap();
        let eta:Vec<_>=(0..nx).map(|i|4.+0.01*(std::f64::consts::PI*(i as f64+0.5)/nx as f64).cos() as f32).collect();
        let u=vec![0.;v.velocity_u().len()]; let w=vec![0.;v.velocity_w().len()];
        arena.seal();
        let mut samples=Vec::new();
        let mut iterations=0;
        for rep in 0..14 {
            v.set_surface(&eta).unwrap(); v.set_velocity(&u,&w).unwrap();
            let clock=Clock(Instant::now());
            let start=Instant::now();
            let r=v.step_surface_linear(2000,2000,1_000_000,&jobs,&clock).unwrap();
            let ms=start.elapsed().as_secs_f64()*1000.;
            assert_eq!(r.advanced_us,2000); iterations=r.report.unwrap().iterations;
            if rep>=3 {samples.push(ms);}
        }
        samples.sort_by(f64::total_cmp);
        println!("n={nx} bytes={} iterations={iterations} median_ms={:.6} max_ms={:.6}",
            arena.stats().persistent_bytes,samples[5],samples[10]);
    }
    println!("Inclut sauvegarde/projection/flux/hauteur/controles et retour. Pas de B/W/rendu, ni reception I-05.");
}
