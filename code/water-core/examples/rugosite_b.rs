//! S256 — rugosité de B contre la mer observée (SPEC-001 §1 sexies ; revue visuelle R1).
//! `mss` de la recette cuite de la scène J1, et du même spectre continu coupé à `b·fp`, en f64,
//! par une intégration indépendante de la cuisson du cœur. Comparée à Cox & Munk (1954) au vent
//! minimal qui soutient `Hs` en mer pleinement développée (Pierson & Moskowitz, 1964).
use water_core::{background_spectrum::{bake, Recipe}, SeaState};

fn q(x: f64, gamma: f64) -> f64 {
    let sigma = if x <= 1.0 { 0.07 } else { 0.09 };
    let r = (-(x - 1.0).powi(2) / (2.0 * sigma * sigma)).exp();
    x.powi(-5) * (-1.25 / x.powi(4)).exp() * gamma.powf(r)
}
/// Simpson f64, coupé au pic pour la raideur de γ.
fn integral(lo: f64, hi: f64, p: i32, gamma: f64) -> f64 {
    if lo < 1.0 && hi > 1.0 {
        return integral(lo, 1.0, p, gamma) + integral(1.0, hi, p, gamma);
    }
    let n = 20_000;
    let h = (hi - lo) / n as f64;
    (0..=n).map(|i| {
        let x = lo + i as f64 * h;
        let w = if i == 0 || i == n { 1.0 } else if i % 2 == 1 { 4.0 } else { 2.0 };
        w * x.powi(p) * q(x, gamma)
    }).sum::<f64>() * h / 3.0
}

fn main() {
    let (hs, tp, g, gamma) = (1.5f64, 6.0f64, 9.81f64, 3.3f64);
    // Recette de la scène J1 (viewer/src/scene.rs), inchangée.
    let recipe = Recipe {
        sea: SeaState { hs: hs as f32, tp: tp as f32, theta_turns: 0.12, components: 32, graine: 201 },
        gravity: g as f32, gamma: gamma as f32, min_ratio: 0.5, max_ratio: 4., spread_turns: 0.25,
    };
    let cooked = bake(recipe).expect("recette J1");
    let mss_cooked: f64 = cooked.components().iter().map(|c| {
        let k = c.k_turns_per_m as f64 * std::f64::consts::TAU;
        0.5 * (c.amplitude as f64 * k).powi(2)
    }).sum();
    let fp = 1.0 / tp;
    let band = integral(0.5, 4.0, 0, gamma);
    let m0 = hs * hs / 16.0;
    println!("RUGOSITE_B recette_J1 composantes=32 bande=[0.5,4]fp lambda_min_m={:.3} mss_cuite={:.5} Hs_verif_m={:.4}",
        g * tp * tp / (std::f64::consts::TAU * 16.0), mss_cooked,
        4.0 * (cooked.components().iter().map(|c| 0.5 * (c.amplitude as f64).powi(2)).sum::<f64>()).sqrt());
    for b in [4.0, 8.0, 16.0, 24.0, 32.0, 57.0] {
        // Même densité absolue que la bande représentée : S(f) = Hs² q(x)/(16 fp ∫_{0.5}^{4} q).
        let m4 = m0 * fp.powi(4) * integral(0.5, b, 4, gamma) / band;
        let mss = std::f64::consts::TAU.powi(4) * m4 / (g * g);
        println!("RUGOSITE_B continu bande=[0.5,{b}]fp lambda_min_m={:.4} mss={:.5}",
            g * tp * tp / (std::f64::consts::TAU * b * b), mss);
    }
    let u195 = (hs * g / 0.21).sqrt();
    for w in [0.95 * u195, u195] {
        let cm = 0.003 + 5.12e-3 * w;
        println!("COX_MUNK vent_m_s={w:.3} mss={cm:.5} borne_basse={:.5} seuil_moitie={:.5}", cm - 0.004, (cm - 0.004) / 2.0);
    }
}
