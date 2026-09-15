//! S237 — surface géométriquement mobile du candidat δ, contre l'onde stationnaire d'amplitude
//! finie. Voir `docs/validation/SURFACE-MOBILE-S237.md`.
//!
//! `cargo run -p water-core --release --offline --example delta_mobile -- oracle`
#[path = "support/nl_surface.rs"]
#[allow(dead_code)]
mod nl;
#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;
use nl::{NlSurface, C};
use std::f64::consts::PI;
use std::time::Instant;
use water_core::delta_projection::{Domain, Volume};
use water_core::host::{HostServices, MonotonicClock};

struct Clock(Instant);
impl MonotonicClock for Clock {
    fn now_ns(&self) -> u64 {
        self.0.elapsed().as_nanos().min(u64::MAX as u128) as u64
    }
}

/// Bassin à murs de longueur `L`, profondeur `H` : mode `cos(kx)`, `k = π/L`, `kh = π`.
const L: f64 = 2.;
const H: f64 = 2.;
const G: f64 = 9.81;
/// Référence : bande et relèvement du véhicule, fixés en S237 P3 (symbole à `K = 64` trop grossier).
const REF_BAND: usize = 16;
const REF_LEVELS: usize = 256;

fn k() -> f64 {
    PI / L
}
fn omega() -> f64 {
    (G * k() * (k() * H).tanh()).sqrt()
}
fn period() -> f64 {
    2. * PI / omega()
}

/// Ordre deux depuis le repos, dérivé en S237 P3 (script sympy, notes de session) :
/// `B₂'' + Ω²B₂ = σ₂D₂ + K₂'`, `σ₂ = 2k·tanh 2kh`, `Ω² = gσ₂`, `B₂(0) = B₂'(0) = 0`, avec
/// `K₂ = −gk²·sin 2ωt/(2ω)` et `D₂ = gk(gk·sin²ωt + ω²cos²ωt·sinh 2kh)/(4ω²cosh²kh)`.
/// Rend `B₂(t)` : `b₂ = a²·B₂`, coefficient de `cos 2kx`.
fn second_order_b2(t: f64) -> f64 {
    let (k, h, g, w) = (k(), H, G, omega());
    let sigma2 = 2. * k * (2. * k * h).tanh();
    let big2 = g * sigma2;
    let c2 = (k * h).cosh().powi(2);
    let s2h = (2. * k * h).sinh();
    let d0 = g * k * (g * k / 2. + w * w * s2h / 2.) / (4. * w * w * c2);
    let dc = g * k * (-g * k / 2. + w * w * s2h / 2.) / (4. * w * w * c2);
    let f0 = sigma2 * d0;
    let f2 = sigma2 * dc - g * k * k;
    let p0 = f0 / big2;
    let p2 = f2 / (big2 - 4. * w * w);
    p0 + p2 * (2. * w * t).cos() - (p0 + p2) * (big2.sqrt() * t).cos()
}

/// Véhicule HOS de S193 sur la période `2L`, condition initiale paire `η̂₁ = a/2`, `ψ = 0`.
fn hos(a: f64, band: usize, levels: usize, order: usize) -> NlSurface {
    let mut eta = vec![[0.; 2]; band + 1];
    eta[1] = [a / 2., 0.];
    NlSurface::new(band, levels, 2. * L, H, G, order, &eta, &vec![[0.; 2]; band + 1]).expect("véhicule HOS")
}

/// `η` du véhicule en `x` quelconque, depuis ses modes (convention de `sample`).
fn hos_eta(s: &NlSurface, x: f64) -> f64 {
    let m: &[C] = s.eta_modes();
    m[0][0]
        + 2. * (1..=s.band)
            .map(|q| {
                let t = s.wave[q] * x;
                m[q][0] * t.cos() - m[q][1] * t.sin()
            })
            .sum::<f64>()
}

/// Coefficient de `cos 2kx` : `2·Re η̂₂` (mode `q = 2` sur la période `2L`).
fn hos_b2(s: &NlSurface) -> f64 {
    2. * s.eta_modes()[2][0]
}

/// P3 : l'oracle contre l'ordre deux, et contre lui-même en `Q`, `K`, `dt`.
fn oracle() {
    println!(
        "ORACLE L={L} h={H} g={G} k={:.9} kh={:.6} omega={:.9} T={:.9}",
        k(),
        k() * H,
        omega(),
        period()
    );
    let steps = |dt: f64| (period() / dt).round() as usize;
    for (a, levels) in [(0.01, 64), (0.01, 256), (0.05, 256), (0.10, 256)] {
        for order in [2, 3] {
            let dt = 1e-3;
            let mut s = hos(a, 16, levels, order);
            let (mut worst, mut peak) = (0f64, 0f64);
            for n in 1..=steps(dt) {
                s.step(dt).expect("pas HOS");
                let t = n as f64 * dt;
                let analytic = a * a * second_order_b2(t);
                worst = worst.max((hos_b2(&s) - analytic).abs());
                peak = peak.max(analytic.abs());
            }
            println!(
                "ORDRE_DEUX a={a} K={levels} ka={:.4} U={:.3} M={order} max|b2_analytique|={peak:.6e} ecart_max={worst:.6e} relatif={:.5} volume={:.3e} dispersion_symbole={:.3e}",
                a * k(),
                a * (2. * L) * (2. * L) / (H * H * H),
                worst / peak,
                s.volume(),
                s.dispersion_error(4).0
            );
        }
    }
    // Précision propre du véhicule sur le profil aux centres d'une grille fine, à a = 0,10.
    let profile = |band: usize, levels: usize, dt: f64| {
        let mut s = hos(0.10, band, levels, 3);
        let mut series = Vec::new();
        for n in 1..=steps(dt) {
            s.step(dt).expect("pas HOS");
            if n % (steps(dt) / 40) == 0 {
                series.push((0..128).map(|i| hos_eta(&s, (i as f64 + 0.5) * L / 128.)).collect::<Vec<_>>());
            }
        }
        series
    };
    let base = profile(REF_BAND, REF_LEVELS, 1e-3);
    for (label, other) in [
        ("Q32", profile(32, REF_LEVELS, 1e-3)),
        ("K512", profile(REF_BAND, 512, 1e-3)),
        ("dt0.5ms", profile(REF_BAND, REF_LEVELS, 5e-4)),
    ] {
        let d = base
            .iter()
            .zip(&other)
            .flat_map(|(a, b)| a.iter().zip(b).map(|(x, y)| (x - y).abs()))
            .fold(0f64, f64::max);
        println!(
            "ORACLE_PROPRE a=0.10 reference=Q{REF_BAND}_K{REF_LEVELS}_dt1ms variante={label} ecart_max_sur_a={:.3e} symbole_reference={:.3e}",
            d / 0.10,
            hos(0.1, REF_BAND, REF_LEVELS, 3).dispersion_error(4).0
        );
    }
}

/// Mode de surface du candidat comparé.
#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Mobile,
    Linear,
}

/// Une trajectoire du candidat sur une période, comparée pas à pas au véhicule HOS.
struct Run {
    /// `max|η − η_HOS|/a` sur tous les points et tous les pas.
    profile: f64,
    /// `max|b₂ − b₂,HOS|` et `max|b₂,HOS|`, `max|b₂|` du candidat.
    b2_error: f64,
    b2_ref: f64,
    b2_max: f64,
    volume_drift: f64,
    wet_min: usize,
    wet_max: usize,
    iterations_max: u32,
    /// S238 : pas acceptés au plancher de la pression (cycle certifié, ADR-143), et leur pire divergence.
    cycles: usize,
    cycle_divergence: f64,
    /// Plus petit `θ` des faces fantômes rencontré (mode mobile) : au-dessus de `SURFACE_THETA_MIN`,
    /// la borne n'a jamais agi et la sensibilité à sa valeur est nulle par construction.
    theta_min: f64,
    step_ms_median: f64,
    step_ms_max: f64,
    bytes: usize,
}

fn candidate(mode: Mode, nx: usize, a: f64, dt_us: u64) -> Result<Run, String> {
    candidate_capped(mode, nx, a, dt_us, 4000)
}

/// S238 : même trajectoire sous un plafond d'itérations choisi par l'appelant (S237 : 4 000).
fn candidate_capped(mode: Mode, nx: usize, a: f64, dt_us: u64, cap: u32) -> Result<Run, String> {
    let dx = L / nx as f64;
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 26);
    // Mobile : domaine jusqu'à 2,25 m, repos à 2 m. Linéaire : couvercle S233 au repos, z₀ = h.
    let nz = match mode {
        Mode::Mobile => (2.25 / dx).round() as usize,
        Mode::Linear => (H / dx).round() as usize,
    };
    let mut v = Volume::configure(
        &mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink },
        Domain { nx, nz, dx: dx as f32 },
        1025.,
        G as f32,
        &vec![0.; nx],
    )
    .map_err(|e| format!("configuration {e:?}"))?;
    let xc = |i: usize| (i as f64 + 0.5) * dx;
    let eta: Vec<f32> = (0..nx).map(|i| (H + a * (k() * xc(i)).cos()) as f32).collect();
    match mode {
        Mode::Mobile => v.set_free_surface(&eta, H as f32),
        Mode::Linear => v.set_surface(&eta),
    }
    .map_err(|e| format!("surface {e:?}"))?;
    use water_core::host::Allocator;
    arena.seal();
    let dt = dt_us as f64 * 1e-6;
    let steps = (period() / dt).round() as usize;
    let mut reference = hos(a, REF_BAND, REF_LEVELS, 3);
    let volume0: f64 = v.surface().iter().map(|h| *h as f64).sum();
    let mut run = Run {
        profile: 0.,
        b2_error: 0.,
        b2_ref: 0.,
        b2_max: 0.,
        volume_drift: 0.,
        wet_min: usize::MAX,
        wet_max: 0,
        iterations_max: 0,
        cycles: 0,
        cycle_divergence: 0.,
        theta_min: f64::INFINITY,
        step_ms_median: 0.,
        step_ms_max: 0.,
        bytes: arena.stats().persistent_bytes,
    };
    let mut times = Vec::with_capacity(steps);
    for n in 1..=steps {
        let clock = Clock(Instant::now());
        let start = Instant::now();
        let report = match mode {
            Mode::Mobile => v.step_surface_mobile(dt_us, cap, 600_000_000, &jobs, &clock),
            Mode::Linear => v.step_surface_linear(dt_us, cap, 600_000_000, &jobs, &clock),
        }
        .map_err(|e| format!("pas {n} : {e:?}"))?;
        times.push(start.elapsed().as_secs_f64() * 1e3);
        if report.advanced_us != dt_us {
            return Err(format!("pas {n} non avancé : {:?}", report.stopped_at));
        }
        run.iterations_max = run.iterations_max.max(report.report.map_or(0, |r| r.iterations));
        if let Some(r) = report.report.filter(|r| r.floor) {
            run.cycles += 1;
            run.cycle_divergence = run.cycle_divergence.max(r.divergence);
            println!("  PLANCHER pas={n} iterations={} residu={:.4e} divergence={:.3e} omega={:.3e}", r.iterations, r.residual, r.divergence, r.backward_error);
        }
        if mode == Mode::Mobile {
            let wet = v.wet_cells();
            run.wet_min = run.wet_min.min(wet);
            run.wet_max = run.wet_max.max(wet);
            // Mêmes définitions que `ghost_up` et `ghost_side`, fond plat.
            let s = v.surface();
            let zc = |k: usize| (k as f64 + 0.5) * dx;
            for i in 0..nx {
                let e = s[i] as f64;
                let top = ((e / dx) - 0.5).ceil() as usize - 1;
                run.theta_min = run.theta_min.min((e - zc(top)) / dx);
                for j in [i.wrapping_sub(1), i + 1] {
                    if j >= nx {
                        continue;
                    }
                    let n = s[j] as f64;
                    for k in 0..=top {
                        if zc(k) >= n {
                            run.theta_min = run.theta_min.min((e - zc(k)) / (e - n));
                        }
                    }
                }
            }
        }
        reference.step(dt).map_err(|e| format!("HOS {e}"))?;
        let (mut b2, mut b2h) = (0f64, 0f64);
        for (i, h) in v.surface().iter().enumerate() {
            let x = xc(i);
            let mac = *h as f64 - H;
            let hosv = hos_eta(&reference, x);
            run.profile = run.profile.max((mac - hosv).abs() / a);
            b2 += 2. / L * mac * (2. * k() * x).cos() * dx;
            b2h += 2. / L * hosv * (2. * k() * x).cos() * dx;
        }
        run.b2_error = run.b2_error.max((b2 - b2h).abs());
        run.b2_ref = run.b2_ref.max(b2h.abs());
        run.b2_max = run.b2_max.max(b2.abs());
    }
    run.volume_drift = (v.surface().iter().map(|h| *h as f64).sum::<f64>() - volume0).abs() / nx as f64;
    times.sort_by(f64::total_cmp);
    run.step_ms_median = times[times.len() / 2];
    run.step_ms_max = times[times.len() - 1];
    Ok(run)
}

fn compare(modes: &[Mode], grids: &[usize], amplitudes: &[f64], dt_us: u64) {
    for &a in amplitudes {
        for &mode in modes {
            for &nx in grids {
                let label = if mode == Mode::Mobile { "mobile" } else { "lineaire" };
                match candidate(mode, nx, a, dt_us) {
                    Ok(r) => println!(
                        "CANDIDAT mode={label} a={a} nx={nx} dt_us={dt_us} profil_sur_a={:.5} b2_ecart={:.4e} b2_ref={:.4e} b2_relatif={:.4} b2_candidat={:.4e} derive_volume_m={:.3e} mailles_fluides={}..{} theta_min={:.4e} iterations_max={} pas_au_plancher={} divergence_plancher_max={:.3e} pas_ms median={:.3} max={:.3} octets={}",
                        r.profile, r.b2_error, r.b2_ref, r.b2_error / r.b2_ref, r.b2_max, r.volume_drift,
                        r.wet_min, r.wet_max, r.theta_min, r.iterations_max, r.cycles, r.cycle_divergence, r.step_ms_median, r.step_ms_max, r.bytes
                    ),
                    Err(e) => println!("CANDIDAT mode={label} a={a} nx={nx} dt_us={dt_us} REFUS {e}"),
                }
            }
        }
    }
}

/// Critère 4 : mobile contre linéaire S233 sur la même grille, `a = 1 mm`, une période.
fn small_amplitude(nx: usize, a: f64, dt_us: u64) -> Result<f64, String> {
    let dx = L / nx as f64;
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 26);
    let mut build = |nz: usize| {
        Volume::configure(
            &mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink },
            Domain { nx, nz, dx: dx as f32 },
            1025.,
            G as f32,
            &vec![0.; nx],
        )
        .map_err(|e| format!("{e:?}"))
    };
    let mut mobile = build((2.25 / dx).round() as usize)?;
    let mut linear = build((H / dx).round() as usize)?;
    let eta: Vec<f32> = (0..nx).map(|i| (H + a * (k() * (i as f64 + 0.5) * dx).cos()) as f32).collect();
    mobile.set_free_surface(&eta, H as f32).map_err(|e| format!("{e:?}"))?;
    linear.set_surface(&eta).map_err(|e| format!("{e:?}"))?;
    let clock = Clock(Instant::now());
    let mut worst = 0f64;
    for n in 1..=(period() / (dt_us as f64 * 1e-6)).round() as usize {
        mobile.step_surface_mobile(dt_us, 4000, 60_000_000, &jobs, &clock).map_err(|e| format!("mobile {n} {e:?}"))?;
        linear.step_surface_linear(dt_us, 4000, 60_000_000, &jobs, &clock).map_err(|e| format!("linéaire {n} {e:?}"))?;
        for (m, l) in mobile.surface().iter().zip(linear.surface()) {
            worst = worst.max((*m as f64 - *l as f64).abs() / a);
        }
    }
    Ok(worst)
}

/// Diagnostic d'un refus `Convergence` : avance jusqu'au pas qui précède, puis rejoue ce pas
/// (atomique) sous plusieurs plafonds d'itérations.
fn diagnose(nx: usize, a: f64, failing: usize) {
    let dx = L / nx as f64;
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 26);
    let mut v = Volume::configure(
        &mut HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink },
        Domain { nx, nz: (2.25 / dx).round() as usize, dx: dx as f32 },
        1025.,
        G as f32,
        &vec![0.; nx],
    )
    .expect("configuration");
    let eta: Vec<f32> = (0..nx).map(|i| (H + a * (k() * (i as f64 + 0.5) * dx).cos()) as f32).collect();
    v.set_free_surface(&eta, H as f32).expect("surface");
    let clock = Clock(Instant::now());
    for n in 1..failing {
        v.step_surface_mobile(1000, 4000, 60_000_000, &jobs, &clock).unwrap_or_else(|e| panic!("pas {n} : {e:?}"));
    }
    for cap in [4000u32, 16000, 64000] {
        let start = Instant::now();
        let r = v.step_surface_mobile(1000, cap, 600_000_000, &jobs, &clock);
        println!(
            "DIAGNOSTIC nx={nx} a={a} pas={failing} plafond={cap} resultat={:?} ms={:.1}",
            r.map(|s| s.report),
            start.elapsed().as_secs_f64() * 1e3
        );
        if r.is_ok() {
            break;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("plancher") {
        // S238 : le cas refusé de S237 et ses deux voisins, qui doivent redonner les chiffres S237.
        return compare(&[Mode::Mobile], &[32, 64, 128], &[0.05], 1000);
    }
    if args.get(1).map(String::as_str) == Some("plancher16000") {
        // S238 : sous plafond 4 000, la certification du cycle au pas 397 n'aboutit pas (7 386
        // itérations nécessaires) ; même trajectoire à 128 colonnes sous plafond 16 000.
        let label = "mobile";
        match candidate_capped(Mode::Mobile, 128, 0.05, 1000, 16_000) {
            Ok(r) => println!(
                "CANDIDAT mode={label} a=0.05 nx=128 plafond=16000 profil_sur_a={:.5} b2_relatif={:.4} b2_candidat={:.4e} derive_volume_m={:.3e} iterations_max={} pas_au_plancher={} divergence_plancher_max={:.3e} pas_ms median={:.3} max={:.3}",
                r.profile, r.b2_error / r.b2_ref, r.b2_max, r.volume_drift, r.iterations_max, r.cycles, r.cycle_divergence, r.step_ms_median, r.step_ms_max
            ),
            Err(e) => println!("CANDIDAT mode={label} a=0.05 nx=128 plafond=16000 REFUS {e}"),
        }
        return;
    }
    if args.get(1).map(String::as_str) == Some("diagnostic") {
        return diagnose(128, 0.05, 397);
    }
    if args.get(1).map(String::as_str) == Some("sensibilite") {
        println!("SENSIBILITE theta_min={}", water_core::delta_projection::SURFACE_THETA_MIN);
        return compare(&[Mode::Mobile], &[32, 128], &[0.05], 1000);
    }
    match args.get(1).map(String::as_str) {
        Some("oracle") => oracle(),
        Some("essai") => compare(&[Mode::Mobile], &[32], &[0.05], 1000),
        Some("petite") => println!("PETITE_AMPLITUDE a=0.001 nx=64 dt_us=1000 ecart_mobile_lineaire_sur_a={:?}", small_amplitude(64, 0.001, 1000)),
        Some("reception") => {
            println!("PETITE_AMPLITUDE a=0.001 nx=64 dt_us=1000 ecart_mobile_lineaire_sur_a={:?}", small_amplitude(64, 0.001, 1000));
            compare(&[Mode::Mobile, Mode::Linear], &[32, 64, 128], &[0.05, 0.10], 1000);
            compare(&[Mode::Mobile], &[64], &[0.01], 1000);
        }
        _ => eprintln!("usage : delta_mobile oracle | essai | reception"),
    }
}
