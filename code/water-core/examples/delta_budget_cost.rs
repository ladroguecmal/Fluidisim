//! S230 : coût complet de l'arrêt coopératif, retards observés et témoin sans limite.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::time::Instant;
use water_core::{delta_projection::{Domain, Volume}, host::{Allocator, HostServices, MonotonicClock}};
struct Clock(Instant);
impl MonotonicClock for Clock {
    fn now_ns(&self) -> u64 { self.0.elapsed().as_nanos().min(u64::MAX as u128) as u64 }
}
fn main() {
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let clock = Clock(Instant::now());
    println!("S230 CPU sequentiel release, 8x4m, fond plat 0.4m, rho1025 g9.81 dt1/60, eta=z0+0.02sin(2pi*(i+0.5)/nx), plafond512");
    println!("3 chauffes + 101 mesures par cas ; un pas depuis vitesses nulles, etat public verifie au refus ; temps externe incluant retour");
    println!("nx mode budget_ms avances median_ms p95_ms max_ms retard_max_ms ecart_compteur_max_ms");
    for nx in [16, 32, 64] {
        let nz = nx / 2;
        let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 24);
        let mut v = Volume::configure(&mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink },
            Domain { nx, nz, dx: 8. / nx as f32 }, 1025., 9.81, &vec![0.4; nx]).unwrap();
        let eta: Vec<_> = (0..nx).map(|i| v.domain().z0()+0.02*(std::f32::consts::TAU*(i as f32+0.5)/nx as f32).sin()).collect();
        v.set_surface(&eta).unwrap(); arena.seal();
        let u = vec![0.; v.velocity_u().len()]; let w = vec![0.; v.velocity_w().len()];
        let reference = v.step(1./60., 512, &jobs).unwrap();
        let expected_u = v.velocity_u().to_vec(); let expected_w = v.velocity_w().to_vec(); let expected_p = v.pressure().to_vec();
        let mut costs = [0f64; 101];
        for (mode, budget) in [("plain", 0.), ("cooperative", 1000.), ("cooperative", 0.),
            ("cooperative", 0.05), ("cooperative", 0.5), ("cooperative", 2.)] {
            let (mut advanced, mut lag, mut gap) = (0, 0f64, 0f64);
            for rep in 0..104 {
                v.set_velocity(&u, &w).unwrap();
                let previous_p = v.pressure().to_vec(); // hors fenêtre ; état non nul après échauffement.
                let start = Instant::now();
                let (did_advance, reported_ns, report) = if mode == "plain" {
                    let report = v.step(1./60., 512, &jobs).unwrap(); (true, None, Some(report))
                } else {
                    let result = v.step_budgeted(1./60., 512, budget, &jobs, &clock).unwrap();
                    assert_eq!(result.advanced_dt + result.remaining_dt, 1./60.);
                    (result.advanced_dt > 0., Some(result.elapsed_ns), result.report)
                };
                let elapsed = start.elapsed().as_secs_f64()*1000.;
                if did_advance {
                    assert_eq!(report, Some(reference));
                    assert_eq!(v.velocity_u(), expected_u); assert_eq!(v.velocity_w(), expected_w); assert_eq!(v.pressure(), expected_p);
                } else {
                    assert_eq!(v.velocity_u(), u); assert_eq!(v.velocity_w(), w); assert_eq!(v.pressure(), previous_p);
                }
                if rep >= 3 {
                    costs[rep-3] = elapsed; advanced += usize::from(did_advance);
                    if mode != "plain" { lag = lag.max((elapsed-budget as f64).max(0.)); }
                    if let Some(ns) = reported_ns { gap = gap.max((elapsed - ns as f64 / 1e6).max(0.)); }
                }
            }
            costs.sort_by(f64::total_cmp);
            println!("{nx} {mode} {budget:.3} {advanced}/101 {:.6} {:.6} {:.6} {lag:.6} {gap:.6}", costs[50], costs[95], costs[100]);
        }
    }
    println!("Controles64, rollbackO(1), reductions64 presents ; ni preemption, marge WCET, ordonnanceur, f32 pression, surface mobile, 3D, GPU ; retard observe != borne I05");
}
