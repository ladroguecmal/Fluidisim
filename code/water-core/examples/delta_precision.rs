//! S231 : référence avant/après changement de précision, même exécutable de banc.
#[path = "../../water-harness/src/host_impl.rs"] mod host_impl;
use std::{io::{self, Write}, time::Instant};
use water_core::{delta_projection::{Domain, Volume}, host::{Allocator, HostServices}};
fn main() {
    let jobs = host_impl::SequentialJobs; let sink = host_impl::StderrSink;
    let stdout = io::stdout(); let mut output = io::BufWriter::new(stdout.lock());
    for nx in [16, 32, 64, 128] {
        for cut in [false, true] {
            for steps in if nx == 32 { vec![1,100] } else { vec![1] } {
                let nz = nx/2; let dx = 8./nx as f32;
                let bottom: Vec<_> = (0..nx).map(|i| if cut {
                    let d = ((i as f32+0.5)*dx-3.)/1.2; 0.4+0.6*(-d*d).exp()
                } else { 0.5 }).collect();
                let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 25);
                let mut v = Volume::configure(&mut HostServices { alloc:&mut arena, jobs:&jobs, sink:&sink },
                    Domain{nx,nz,dx},1025.,9.81,&bottom).unwrap();
                arena.seal();
                let eta: Vec<_> = (0..nx).map(|i| 4.+0.01*(std::f32::consts::TAU*(i as f32+0.5)/nx as f32).sin()).collect();
                v.set_surface(&eta).unwrap();
                let mut report = None; let start = Instant::now();
                for _ in 0..steps { report = Some(v.step(0.002,20000,&jobs).unwrap()); }
                let ms=start.elapsed().as_secs_f64()*1000.; let r=report.unwrap();
                writeln!(output,"CASE {nx} {} {steps} {} {} {:.12e} {:.12e} {:.6} {}",u8::from(cut),r.iterations,r.degraded,r.residual,r.divergence,ms,arena.stats().persistent_bytes).unwrap();
                for value in v.velocity_u() { writeln!(output,"u {:.17e}",value).unwrap(); }
                for value in v.velocity_w() { writeln!(output,"w {:.17e}",value).unwrap(); }
                for value in v.pressure() { writeln!(output,"p {:.17e}",value).unwrap(); }
            }
        }
    }
}
