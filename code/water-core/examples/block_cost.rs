//! S202 : coût complet du domaine δ, un domaine = un bloc de banc déclaré.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::time::Instant;
use water_core::{
    delta_projection::{Domain, Volume},
    host::{Allocator, HostServices, MonotonicClock},
};
struct Clock(Instant);
impl MonotonicClock for Clock {
    fn now_ns(&self) -> u64 {
        self.0.elapsed().as_nanos().min(u64::MAX as u128) as u64
    }
}
fn main() {
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let clock = Clock(Instant::now());
    println!("S202 release CPU sequentiel ; profil60Hz eau2ms ; dt=1/60s ; domaine8x4m fond plat0.4m eta=z0+0.02sin(2pi*x/8)");
    println!("3 chauffes +11 mesures, vitesses reinitialisees avant chaque pas ; aucune I/O/configuration incluse ; un domaine=un bloc");
    println!("nx nz dx limite iterations degrade residu median_ms max_ms max_sur_2ms cout_seul_compatible");
    for nx in [16, 32, 64] {
        for limit in [1, 64, 512] {
            let nz = nx / 2;
            let dx = 8. / nx as f32;
            let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 24);
            let mut v = Volume::configure(
                &mut HostServices {
                    alloc: &mut arena,
                    jobs: &jobs,
                    sink: &sink,
                },
                Domain { nx, nz, dx },
                1025.,
                9.81,
                &vec![0.4; nx],
            )
            .unwrap();
            let eta: Vec<_> = (0..nx)
                .map(|i| {
                    v.domain().z0()
                        + 0.02 * (std::f32::consts::TAU * (i as f32 + 0.5) / nx as f32).sin()
                })
                .collect();
            v.set_surface(&eta).unwrap();
            arena.seal();
            let u = vec![0.; v.velocity_u().len()];
            let w = vec![0.; v.velocity_w().len()];
            let mut costs = Vec::new();
            let mut report = None;
            for rep in 0..14 {
                v.set_velocity(&u, &w).unwrap();
                let r = v.step_measured(1. / 60., limit, &jobs, &clock).unwrap();
                if let Some(previous) = report {
                    assert_eq!(r, previous, "la fixture change entre répétitions");
                }
                report = Some(r);
                if rep >= 3 {
                    costs.push(v.caps().cost_per_block_ms.expect("horloge resolue"));
                }
            }
            costs.sort_by(f32::total_cmp);
            let r = report.unwrap();
            let max = costs[10];
            println!(
                "{nx} {nz} {dx:.3} {limit} {} {} {:.5e} {:.6} {:.6} {:.3} {}",
                r.iterations,
                r.degraded,
                r.residual,
                costs[5],
                max,
                max / 2.,
                max <= 2.
            );
        }
    }
    println!("Compatible en cout seul != admissible physiquement : filtre fond coupe, surface mobile et precision non recus ; cout B/W/rendu absent du tableau.");
}
