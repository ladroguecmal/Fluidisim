//! Réception S296 : référence mobile 3D contre HOS à ny=1 et onde oblique linéaire.
//! `cargo run -p water-core --release --offline --example delta3d_mobile -- hos`
//! `cargo run -p water-core --release --offline --example delta3d_mobile -- oblique`
#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;
#[path = "support/nl_surface.rs"]
#[allow(dead_code)]
mod nl;
use nl::NlSurface;
use std::{f64::consts::PI, time::Instant};
use water_core::{
    delta3d::{Domain3, Volume3},
    host::HostServices,
};
const L: f64 = 2.;
const H: f64 = 2.;
const G: f64 = 9.81;
fn hos(a: f64) -> NlSurface {
    let mut eta = vec![[0.; 2]; 17];
    eta[1] = [a / 2., 0.];
    NlSurface::new(16, 256, 2. * L, H, G, 3, &eta, &vec![[0.; 2]; 17]).unwrap()
}
fn hos_eta(s: &NlSurface, x: f64) -> f64 {
    let m = s.eta_modes();
    m[0][0]
        + 2. * (1..=s.band)
            .map(|q| {
                let t = s.wave[q] * x;
                m[q][0] * t.cos() - m[q][1] * t.sin()
            })
            .sum::<f64>()
}
fn volume(nx: usize, ny: usize, nz: usize, dx: f64) -> Volume3 {
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
    Volume3::configure(
        &mut HostServices {
            alloc: &mut arena,
            jobs: &host_impl::SequentialJobs,
            sink: &host_impl::StderrSink,
        },
        Domain3 {
            nx,
            ny,
            nz,
            dx: dx as f32,
        },
        1025.,
        G as f32,
    )
    .unwrap()
}
fn reception_hos() {
    let k = PI / L;
    let omega = (G * k * (k * H).tanh()).sqrt();
    let steps = (2. * PI / omega / 0.001).round() as usize;
    for a in [0.05, 0.10] {
        let mut previous = (f64::INFINITY, f64::INFINITY);
        for nx in [32, 64, 128] {
            let start = Instant::now();
            let dx = L / nx as f64;
            let mut v = volume(nx, 1, (2.25 / dx).round() as usize, dx);
            let eta: Vec<_> = (0..nx)
                .map(|i| (H + a * (k * (i as f64 + 0.5) * dx).cos()) as f32)
                .collect();
            v.set_free_surface(&eta, H as f32).unwrap();
            let mut oracle = hos(a);
            let mean0 = eta.iter().map(|x| *x as f64).sum::<f64>() / nx as f64;
            let (mut profile, mut b2err, mut b2ref, mut drift) = (0f64, 0f64, 0f64, 0f64);
            let (mut imax, mut floor) = (0, 0);
            let (mut wetmin, mut wetmax) = (v.wet_cells(), v.wet_cells());
            for n in 1..=steps {
                oracle.step(0.001).unwrap();
                let r = v
                    .step_surface_mobile(1000, 4000, &host_impl::SequentialJobs)
                    .unwrap_or_else(|e| panic!("a={a} nx={nx} step={n}: {e:?}"));
                imax = imax.max(r.iterations);
                floor += r.floor as usize;
                let mut b2 = 0.;
                let mut mean = 0.;
                for i in 0..nx {
                    let x = (i as f64 + 0.5) * dx;
                    let h = v.surface()[i] as f64 - H;
                    profile = profile.max((h - hos_eta(&oracle, x)).abs() / a);
                    b2 += 2. / nx as f64 * h * (2. * k * x).cos();
                    mean += v.surface()[i] as f64 / nx as f64;
                }
                let reference = 2. * oracle.eta_modes()[2][0];
                b2err = b2err.max((b2 - reference).abs());
                b2ref = b2ref.max(reference.abs());
                drift = drift.max((mean - mean0).abs());
                wetmin = wetmin.min(v.wet_cells());
                wetmax = wetmax.max(v.wet_cells());
            }
            let harmonic = b2err / b2ref;
            println!("HOS a={a} nx={nx} steps={steps} profile_pct={:.6} b2_pct={:.6} mean_drift_m={drift:.3e} wet={wetmin}..{wetmax} it_max={imax} floor={floor} seconds={:.2}",100.*profile,100.*harmonic,start.elapsed().as_secs_f64());
            assert!(
                profile < previous.0 && harmonic < previous.1,
                "raffinement non décroissant"
            );
            if nx == 128 {
                assert!(profile < 0.02 && harmonic < 0.20);
            }
            previous = (profile, harmonic);
        }
    }
}
fn oblique() {
    let (lx, ly, h, a) = (8., 4., 4., 0.001);
    let (kx, ky) = (PI / lx, PI / ly);
    let k = (kx * kx + ky * ky).sqrt();
    let omega = (G * k * (k * h).tanh()).sqrt();
    let mut previous = f64::INFINITY;
    for nx in [16, 32, 48] {
        let start = Instant::now();
        let dx = lx / nx as f64;
        let ny = (ly / dx).round() as usize;
        let nz = (h / dx).round() as usize;
        let mut mobile = volume(nx, ny, nz + 2, dx);
        let mut linear = volume(nx, ny, nz, dx);
        let mode = |i: usize, j: usize| {
            (kx * (i as f64 + 0.5) * dx).cos() * (ky * (j as f64 + 0.5) * dx).cos()
        };
        let eta: Vec<_> = (0..ny)
            .flat_map(|j| (0..nx).map(move |i| (h + a * mode(i, j)) as f32))
            .collect();
        mobile.set_free_surface(&eta, h as f32).unwrap();
        linear.set_surface(&eta).unwrap();
        let (mut gap, mut continuous, mut drift, mut imax) = (0f64, 0f64, 0f64, 0);
        let mean0 = eta.iter().map(|x| *x as f64).sum::<f64>() / eta.len() as f64;
        // Phase du mode : instants du premier passage à zéro, interpolation entre deux pas.
        let (mut last_mode, mut crossing) = (a, None);
        for n in 1..=1000 {
            let r = mobile
                .step_surface_mobile(1000, 4000, &host_impl::SequentialJobs)
                .unwrap();
            imax = imax.max(r.iterations);
            linear
                .step_surface_linear(1000, 4000, &host_impl::SequentialJobs)
                .unwrap();
            let t = n as f64 * 0.001;
            let mut projection = 0.;
            let mut mean = 0.;
            for j in 0..ny {
                for i in 0..nx {
                    let c = j * nx + i;
                    let m = mobile.surface()[c] as f64 - h;
                    gap = gap
                        .max((mobile.surface()[c] as f64 - linear.surface()[c] as f64).abs() / a);
                    continuous = continuous.max((m - a * mode(i, j) * (omega * t).cos()).abs() / a);
                    projection += 4. * m * mode(i, j) / (nx * ny) as f64;
                    mean += mobile.surface()[c] as f64 / (nx * ny) as f64;
                }
            }
            drift = drift.max((mean - mean0).abs());
            if crossing.is_none() && projection < 0. && last_mode >= 0. {
                crossing = Some(t - 0.001 + 0.001 * last_mode / (last_mode - projection));
            }
            last_mode = projection;
        }
        let phase = (crossing.unwrap() * omega - PI / 2.) * 180. / PI;
        println!("OBLIQUE nx={nx} ny={ny} mobile_linear_pct={:.6} continuous_pct={:.6} phase_zero_deg={phase:.6} mean_drift_m={drift:.3e} it_max={imax} seconds={:.2}",100.*gap,100.*continuous,start.elapsed().as_secs_f64());
        assert!(gap < 0.01);
        assert!(continuous < previous);
        if nx == 48 {
            assert!(continuous < 0.01);
        }
        previous = continuous;
    }
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("hos") => reception_hos(),
        Some("oblique") => oblique(),
        _ => panic!("choisir hos ou oblique"),
    }
}
