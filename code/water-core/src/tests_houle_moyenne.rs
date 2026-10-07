//! Essais de l'effet moyen de la houle (S672) : trois solutions analytiques.

use super::*;
use crate::pente_douce::nombre_d_onde;

const G: f64 = 9.81;
const PENTE: f64 = 1.0 / 50.0;

/// **S672 (1)** — le creux hors du déferlement (Longuet-Higgins et Stewart 1962) : `η̄ = −H²k/(8·sinh 2kh)`. Une houle de 1 m et 10 s de
/// face, levée par le flux, sur la plage 1:50 de 80 m à 5 m ; à chaque rangée, à 0,5 %.
#[test]
fn the_setdown_matches_longuet_higgins_and_stewart_s672() {
    let om = core::f64::consts::TAU / 10.0;
    let cg0 = G / (2. * om);
    let cg = |h: f64| {
        let k = nombre_d_onde(om, h, G);
        0.5 * om / k * (1. + 2. * k * h / (2. * k * h).sinh())
    };
    let hauteur = |h: f64| (cg0 / cg(h)).sqrt();
    let n = ((80.0 - 5.0) / PENTE / 0.5) as usize + 1;
    let s: Vec<f64> = (0..n).map(|i| i as f64 * 0.5).collect();
    let h: Vec<f64> = s.iter().map(|s| 80.0 - PENTE * s).collect();
    let analytique = |h: f64| {
        let k = nombre_d_onde(om, h, G);
        -hauteur(h).powi(2) * k / (8. * (2. * k * h).sinh())
    };
    let sss = |i: usize, _eta: f64| {
        let k = nombre_d_onde(om, h[i], G);
        contrainte(&[Onde { amplitude: 0.5 * hauteur(h[i]), omega: om, k: [k, 0.] }], h[i], G).0
    };
    let eta = niveau_moyen(&s, &h, &sss, analytique(80.0), G).unwrap();
    let mut pire = 0f64;
    for i in 0..n {
        pire = pire.max((eta[i] / analytique(h[i]) - 1.).abs());
    }
    println!("S672 (1) : le creux à {:.3} % de Longuet-Higgins et Stewart au plus ; à 5 m, η̄ = {:.2} cm", 100. * pire, 100. * eta[n - 1]);
    assert!(pire < 0.005, "critère 1");
}

/// **S672 (2)** — la remontée saturée (Bowen, Inman et Simmons 1968) : `H = γ·(h + η̄)`, `dη̄/ds = K·|dh/ds|`, `K = 1/(1 + 8/(3γ²))`.
/// Une houle de 30 s, `γ` = 0,78, de 3 m à 0,3 m ; à chaque rangée, la pente à 2 % de `K`.
#[test]
fn the_saturated_setup_matches_bowen_s672() {
    let om = core::f64::consts::TAU / 30.0;
    let gamma = 0.78;
    let n = ((3.0 - 0.3) / PENTE / 0.5) as usize + 1;
    let s: Vec<f64> = (0..n).map(|i| i as f64 * 0.5).collect();
    let h: Vec<f64> = s.iter().map(|s| 3.0 - PENTE * s).collect();
    let sss = |i: usize, eta: f64| {
        let d = h[i] + eta;
        let k = nombre_d_onde(om, d, G);
        contrainte(&[Onde { amplitude: 0.5 * gamma * d, omega: om, k: [k, 0.] }], d, G).0
    };
    let eta = niveau_moyen(&s, &h, &sss, 0.0, G).unwrap();
    let k_bowen = 1. / (1. + 8. / (3. * gamma * gamma));
    let (mut pire, mut lo, mut hi) = (0f64, f64::MAX, 0f64);
    for i in 1..n {
        let r = (eta[i] - eta[i - 1]) / 0.5 / PENTE / k_bowen;
        pire = pire.max((r - 1.).abs());
        (lo, hi) = (lo.min(r), hi.max(r));
    }
    println!("S672 (2) : K = {k_bowen:.4} ; la pente mesurée de {lo:.4}·K à {hi:.4}·K ; au rivage (0,3 m), η̄ = {:.1} cm", 100. * eta[n - 1]);
    assert!(pire < 0.02, "critère 2");
}

/// **S672 (3)** — le courant de dérive de Longuet-Higgins (1970), sans mélange : `V = (5π/16)·(γ/c_f)·tan β·√(gh)·sin θ`. Une houle de
/// 30 s, `γ` = 0,78 (`H = γh`), `c_f` = 0,01, `θ` = 1° à 2 m (Snell) ; de 1,5 m à 0,5 m, à 3 %. À 5°, l'écart rapporté.
#[test]
fn the_longshore_current_matches_longuet_higgins_s672() {
    let om = core::f64::consts::TAU / 30.0;
    let (gamma, cf) = (0.78, 0.01);
    let n = ((2.5 - 0.3) / PENTE / 0.5) as usize + 1;
    let s: Vec<f64> = (0..n).map(|i| i as f64 * 0.5).collect();
    let h: Vec<f64> = s.iter().map(|s| 2.5 - PENTE * s).collect();
    for theta_b in [1.0f64, 5.0] {
        let kn = nombre_d_onde(om, 2.0, G) * theta_b.to_radians().sin();
        let onde = |i: usize| {
            let k = nombre_d_onde(om, h[i], G);
            Onde { amplitude: 0.5 * gamma * h[i], omega: om, k: [(k * k - kn * kn).sqrt(), kn] }
        };
        let ssn: Vec<f64> = (0..n).map(|i| contrainte(&[onde(i)], h[i], G).1).collect();
        let vitesses: Vec<Vec<([f64; 2], f64)>> = (0..n).map(|i| vec![vitesse_au_fond(&onde(i), h[i])]).collect();
        let v = derive_aux_rangees(&s, &ssn, &vitesses, cf).unwrap();
        let mut pire = 0f64;
        for i in 0..n {
            if h[i] <= 1.5 + 1e-9 && h[i] >= 0.5 - 1e-9 {
                let k = nombre_d_onde(om, h[i], G);
                let lh = 5. * core::f64::consts::PI / 16. * gamma / cf * PENTE * (G * h[i]).sqrt() * (kn / k);
                pire = pire.max((v[i] / lh - 1.).abs());
            }
        }
        let i1 = ((2.5 - 1.0) / PENTE / 0.5) as usize;
        println!("S672 (3) : θ = {theta_b}° à 2 m — à 1 m, V = {:.3} m/s ; écart à Longuet-Higgins au plus {:.2} %", v[i1], 100. * pire);
        if theta_b == 1.0 {
            assert!(pire < 0.03, "critère 3");
        }
    }
}
