//! Réception S296 : référence mobile 3D contre HOS à ny=1 et onde oblique linéaire.
//! `cargo run -p water-core --release --offline --example delta3d_mobile -- hos`
//! `cargo run -p water-core --release --offline --example delta3d_mobile -- oblique`
//! S340 : `-- coupled-b` — le cas couplé sur **le fond de B** : la même onde stationnaire faite de deux
//! composantes de B opposées, celle que la production peut recevoir ; chaînon au fond analytique, puis HOS.
#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;
// S321 : ce banc n'appelle pas `dispersion_error`, que le module demande à tout banc (L277) —
// point « Bancs sur l'oracle HOS » de la file active.
#[allow(dead_code)]
#[path = "support/nl_surface.rs"]
#[allow(dead_code)]
mod nl;
use nl::NlSurface;
use water_core::{background::BackgroundSample,delta3d::Sponge3,SimTime};
#[path="support/standing_background.rs"]
#[allow(dead_code)]
mod standing;
#[path="support/delta3d_background.rs"]
#[allow(dead_code)]
mod background3;
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
fn reception_hos(coupled: bool) {
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
            if coupled {v.set_free_surface(&vec![H as f32;nx],H as f32).unwrap();}
            let mut samples=background3::Samples3::new(v.domain());
            let wave=standing::StandingWave{a,k,h:H,g:G,rho:1025.};
            let mut refinements=0;
            let mut oracle = hos(a);
            let mean0 = eta.iter().map(|x| *x as f64).sum::<f64>() / nx as f64;
            let (mut profile, mut b2err, mut b2ref, mut drift) = (0f64, 0f64, 0f64, 0f64);
            let (mut imax, mut floor) = (0, 0);
            let (mut wetmin, mut wetmax) = (v.wet_cells(), v.wet_cells());
            for n in 1..=steps {
                oracle.step(0.001).unwrap();
                let r = if coupled {
                    let time=SimTime((n as u64-1)*1000);
                    samples.fill(H,|x,_,z|wave.sample(x,z,time.0 as f64*1e-6));
                    v.step_perturbation_mobile(time,1000,4000,&samples.view(time),Sponge3::default(),&host_impl::SequentialJobs)
                } else {v.step_surface_mobile(1000,4000,&host_impl::SequentialJobs)}
                    .unwrap_or_else(|e|panic!("a={a} nx={nx} step={n}: {e:?}"));
                refinements+=r.refinements;
                imax = imax.max(r.iterations);
                floor += r.floor as usize;
                let mut b2 = 0.;
                let mut mean = 0.;
                for i in 0..nx {
                    let x = (i as f64 + 0.5) * dx;
                    let h = v.surface()[i] as f64 - H + if coupled {wave.sample(x,0.,n as f64*0.001).eta as f64} else {0.};
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
            println!("HOS coupled={coupled} refinements={refinements} a={a} nx={nx} steps={steps} profile_pct={:.6} b2_pct={:.6} mean_drift_m={drift:.3e} wet={wetmin}..{wetmax} it_max={imax} floor={floor} seconds={:.2}",100.*profile,100.*harmonic,start.elapsed().as_secs_f64());
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
/// S340, porte B, cas 1 d'ADR-175 §4 — **le fond de B**. La production n'évalue que B : l'onde
/// stationnaire de S253 y devient deux composantes de `a/2`, opposées, à la fréquence de profondeur finie
/// `ω² = g·k·tanh(k·h)`, mais à la décroissance d'eau profonde (ADR-154) — `k·h` = π. **Chaînon** : à
/// `t` = 0 et `T/4`, ses échantillons contre ceux de l'analytique, champ par champ, et son flux au fond,
/// que l'analytique n'a pas. **Puis** la référence couplée à ce fond contre HOS, aux tolérances de S253.
fn reception_hos_b() {
    use water_core::{background::Background, delta3d::BackgroundGrid3, phase::freq_hz_to_q32, Component, PhaseQ32, WorldPos};
    let k = PI / L;
    let omega = (G * k * (k * H).tanh()).sqrt();
    let steps = (2. * PI / omega / 0.001).round() as usize;
    for a in [0.05, 0.10] {
        let mut previous = (f64::INFINITY, f64::INFINITY);
        for nx in [32, 64, 128] {
            let start = Instant::now();
            let dx = L / nx as f64;
            let mut v = volume(nx, 1, (2.25 / dx).round() as usize, dx);
            v.set_free_surface(&vec![H as f32; nx], H as f32).unwrap();
            let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 26);
            let mut host = HostServices {
                alloc: &mut arena,
                jobs: &host_impl::SequentialJobs,
                sink: &host_impl::StderrSink,
            };
            let onde = |dir: [f32; 2]| Component {
                amplitude: (a / 2.) as f32,
                k_turns_per_m: (k / (2. * PI)) as f32,
                dir,
                freq_q32: freq_hz_to_q32(omega / (2. * PI)),
                phase0: PhaseQ32(1 << 30),
            };
            let fond = Background::from_components(&mut host, &[onde([1., 0.]), onde([-1., 0.])], WorldPos::from_units(0, 0, 0), G as f32)
                .unwrap();
            let mut grille = BackgroundGrid3::configure(&mut host, v.domain(), [0., 0., -H as f32], 1025.).unwrap();
            let wave = standing::StandingWave { a, k, h: H, g: G, rho: 1025. };
            // Chaînon : écart max et échelle de l'analytique, pour η, u, w, du/dt, p, **dans l'eau** (faces
            // sous le plan moyen) et **au-dessus** — où B prolonge ses champs de façon bornée (ADR-154),
            // l'analytique de façon exacte ; flux de B au fond.
            let mut analytique = background3::Samples3::new(v.domain());
            let (mut ecart, mut echelle, mut flux_fond) = ([[0f64; 5]; 2], [[0f64; 5]; 2], 0f64);
            let d = v.domain();
            for t_us in [0u64, (PI / 2. / omega * 1e6).round() as u64] {
                grille.sample(&fond, SimTime(t_us)).unwrap();
                analytique.fill(H, |x, _, z| wave.sample(x, z, t_us as f64 * 1e-6));
                let vue = grille.view().unwrap();
                for axis in 0..3 {
                    let (bs, ss) = match axis { 0 => (vue.u, &analytique.u), 1 => (vue.v, &analytique.v), _ => (vue.w, &analytique.w) };
                    let plan = (d.nx + usize::from(axis == 0)) * (d.ny + usize::from(axis == 1));
                    for (idx, (b, s)) in bs.iter().zip(ss.iter()).enumerate() {
                        let z = ((idx / plan) as f64 + if axis == 2 { 0. } else { 0.5 }) * dx - H;
                        let zone = usize::from(z >= 0.);
                        let champs = [
                            (b.eta, s.eta),
                            (b.u[0], s.u[0]),
                            (b.u[2], s.u[2]),
                            (b.du_dt[0].abs().max(b.du_dt[2].abs()), s.du_dt[0].abs().max(s.du_dt[2].abs())),
                            (b.p_dyn, s.p_dyn),
                        ];
                        for (n, (x, y)) in champs.iter().enumerate() {
                            ecart[zone][n] = ecart[zone][n].max((x - y).abs() as f64);
                            echelle[zone][n] = echelle[zone][n].max(y.abs() as f64);
                        }
                    }
                }
                // Faces `w` du bas (k = 0) : le fond analytique n'y a aucun flux.
                for i in 0..nx {
                    flux_fond = flux_fond.max(vue.w[i].u[2].abs() as f64);
                }
            }
            for (zone, nom) in ["eau", "air"].iter().enumerate() {
                println!(
                    "HOS_B chainon a={a} nx={nx} zone={nom} eta={:.2e}/{:.2e} u={:.2e}/{:.2e} w={:.2e}/{:.2e} dudt={:.2e}/{:.2e} p={:.2e}/{:.2e}",
                    ecart[zone][0], echelle[zone][0], ecart[zone][1], echelle[zone][1], ecart[zone][2], echelle[zone][2],
                    ecart[zone][3], echelle[zone][3], ecart[zone][4], echelle[zone][4]
                );
            }
            // `CHAINON_SEUL=1` : le chaînon, sans la trajectoire.
            if std::env::var("CHAINON_SEUL").is_ok() {
                println!("HOS_B chainon a={a} nx={nx} flux_fond_B={flux_fond:.2e}");
                continue;
            }
            let mut oracle = hos(a);
            let (mut profile, mut b2err, mut b2ref, mut drift, mut imax, mut refinements) = (0f64, 0f64, 0f64, 0f64, 0u32, 0u32);
            let mean0 = H;
            for n in 1..=steps {
                oracle.step(0.001).unwrap();
                let time = SimTime((n as u64 - 1) * 1000);
                grille.sample(&fond, time).unwrap();
                let r = v
                    .step_perturbation_mobile(time, 1000, 4000, &grille.view().unwrap(), Sponge3::default(), &host_impl::SequentialJobs)
                    .unwrap_or_else(|e| panic!("a={a} nx={nx} step={n}: {e:?}"));
                refinements += r.refinements;
                imax = imax.max(r.iterations);
                let (mut b2, mut mean) = (0., 0.);
                for i in 0..nx {
                    let x = (i as f64 + 0.5) * dx;
                    // Surface totale : δ plus l'élévation du fond, celle de B à l'arrondi de phase près.
                    let h = v.surface()[i] as f64 - H + wave.sample(x, 0., n as f64 * 0.001).eta as f64;
                    profile = profile.max((h - hos_eta(&oracle, x)).abs() / a);
                    b2 += 2. / nx as f64 * h * (2. * k * x).cos();
                    mean += v.surface()[i] as f64 / nx as f64;
                }
                let reference = 2. * oracle.eta_modes()[2][0];
                b2err = b2err.max((b2 - reference).abs());
                b2ref = b2ref.max(reference.abs());
                drift = drift.max((mean - mean0).abs());
            }
            let harmonic = b2err / b2ref;
            println!(
                "HOS_B a={a} nx={nx} steps={steps} flux_fond_B={flux_fond:.2e} profile_pct={:.6} b2_pct={:.6} mean_drift_m={drift:.3e} it_max={imax} refinements={refinements} seconds={:.2}",
                100. * profile, 100. * harmonic, start.elapsed().as_secs_f64()
            );
            println!(
                "HOS_B a={a} nx={nx} decroissant={} tolerance_128={}",
                profile < previous.0 && harmonic < previous.1,
                if nx == 128 { format!("{}", profile < 0.02 && harmonic < 0.20) } else { "-".into() }
            );
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
        Some("hos") => reception_hos(false),
        Some("coupled") => reception_hos(true),
        Some("coupled-b") => reception_hos_b(),
        Some("oblique") => oblique(),
        _ => panic!("choisir hos ou oblique"),
    }
}
