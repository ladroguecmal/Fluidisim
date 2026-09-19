//! S295 — réception de la référence δ tridimensionnelle à surface linéarisée (ADR-175 §4.1).
//!
//! Onde stationnaire **oblique**, mode (1, 1) d'une cuve de 8 × 4 m, h = 4 m, A = 1 cm, vitesse
//! nulle, 1 s. Deux oracles, tous deux calculés hors du solveur :
//! - la dispersion **continue** `ω² = g·k·tanh(k·h)`, `k = π·√(1/Lx² + 1/Ly²)` — ce que la
//!   porte B demande ;
//! - la fréquence **du schéma** `Ω` (Laplacien discret, structure verticale discrète, Euler
//!   symplectique et son demi-pas) — ce qui sépare une faute d'implémentation de la discrétisation.
//!
//! Lancer : `cargo run --release --offline -p water-core --example delta3d_lineaire`.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use std::time::Instant;
use water_core::delta3d::{Domain3, Volume3};
use water_core::host::{Allocator, HostServices};

fn scheme(kx: f64, ky: f64, dx: f64, nz: usize, g: f64, dt: f64) -> (f64, f64) {
    let kappa2 = 4. / (dx * dx) * ((kx * dx / 2.).sin().powi(2) + (ky * dx / 2.).sin().powi(2));
    let (mut a, mut b, mut c, mut d) = (vec![0f64; nz], vec![0f64; nz], vec![0f64; nz], vec![0f64; nz]);
    for k in 0..nz {
        let mut diag = kappa2 * dx * dx;
        if k > 0 { diag += 1.; a[k] = -1.; }
        if k + 1 < nz { diag += 1.; c[k] = -1.; } else { diag += 2.; d[k] = 2.; }
        b[k] = diag;
    }
    for k in 1..nz {
        let m = a[k] / b[k - 1];
        b[k] -= m * c[k - 1];
        d[k] -= m * d[k - 1];
    }
    let mut phi = vec![0f64; nz];
    phi[nz - 1] = d[nz - 1] / b[nz - 1];
    for k in (0..nz - 1).rev() { phi[k] = (d[k] - c[k] * phi[k + 1]) / b[k]; }
    let ws2 = g * kappa2 * phi.iter().map(|x| x * dx).sum::<f64>();
    let big = (1. - ws2 * dt * dt / 2.).acos() / dt;
    (big, -ws2 * dt * dt / (2. * (big * dt).sin()))
}

fn main() {
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let (lx, ly, h, a) = (8f64, 4f64, 4f64, 0.01f64);
    let pi = std::f64::consts::PI;
    let (kx, ky) = (pi / lx, pi / ly);
    let k = (kx * kx + ky * ky).sqrt();
    println!("S295 onde oblique (1,1), cuve 8 x 4 m, h 4 m, A 1 cm, 1 s ; CPU séquentiel, release");
    println!("g | n | dt µs | mailles | erreur contre ω continue (%) | contre Ω du schéma (%) | Ω/ω−1 | dérive moyenne de η (m) | itérations moyennes | s");
    for g in [9.81f64, 1.62] {
        let omega = (g * k * (k * h).tanh()).sqrt();
        for (n, us) in [(16usize, 2000u64), (32, 2000), (32, 1000), (48, 1000)] {
            let dx = lx / n as f64;
            let (nx, ny, nz) = (n, (ly / dx).round() as usize, (h / dx).round() as usize);
            let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
            let mut v = Volume3::configure(&mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink },
                Domain3 { nx, ny, nz, dx: dx as f32 }, 1025., g as f32).unwrap();
            arena.seal();
            let mode = |i: usize, j: usize| (kx * (i as f64 + 0.5) * dx).cos() * (ky * (j as f64 + 0.5) * dx).cos();
            let eta: Vec<f32> = (0..ny).flat_map(|j| (0..nx).map(move |i| (h + a * mode(i, j)) as f32)).collect();
            v.set_surface(&eta).unwrap();
            let mean0: f64 = eta.iter().map(|x| *x as f64).sum::<f64>() / eta.len() as f64;
            let (big, beta) = scheme(kx, ky, dx, nz, g, us as f64 * 1e-6);
            let (mut e_cont, mut e_scheme, mut iterations) = (0f64, 0f64, 0u64);
            let start = Instant::now();
            let steps = 1_000_000 / us;
            for step in 1..=steps {
                iterations += v.step_surface_linear(us, 8000, &jobs).unwrap().iterations as u64;
                let t = (step * us) as f64 * 1e-6;
                let s = v.surface();
                for j in 0..ny {
                    for i in 0..nx {
                        let hn = s[j * nx + i] as f64 - h;
                        e_cont = e_cont.max((hn - a * mode(i, j) * (omega * t).cos()).abs() / a);
                        let ph = big * t;
                        e_scheme = e_scheme.max((hn - a * mode(i, j) * (ph.cos() + beta * ph.sin())).abs() / a);
                    }
                }
            }
            let seconds = start.elapsed().as_secs_f64();
            let mean: f64 = v.surface().iter().map(|x| *x as f64).sum::<f64>() / eta.len() as f64;
            println!("{g} | {n} | {us} | {} | {:.4} | {:.4} | {:.3e} | {:.2e} | {:.1} | {:.1}",
                nx * ny * nz, 100. * e_cont, 100. * e_scheme, big / omega - 1., (mean - mean0).abs(),
                iterations as f64 / steps as f64, seconds);
        }
    }
}
