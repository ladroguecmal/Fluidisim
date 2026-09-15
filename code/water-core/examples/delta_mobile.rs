//! S237 — surface géométriquement mobile du candidat δ, contre l'onde stationnaire d'amplitude
//! finie. Voir `docs/validation/SURFACE-MOBILE-S237.md`.
//!
//! `cargo run -p water-core --release --offline --example delta_mobile -- oracle`
#[path = "support/nl_surface.rs"]
mod nl;
use nl::{NlSurface, C};
use std::f64::consts::PI;

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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("oracle") => oracle(),
        _ => eprintln!("usage : delta_mobile oracle"),
    }
}
