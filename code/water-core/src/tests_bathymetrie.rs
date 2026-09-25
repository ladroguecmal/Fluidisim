//! S362 — la référence de la bathymétrie contre les résultats publiés (critères écrits avant le code, EN-COURS S362).
use super::*;

const G: f64 = 9.81;

/// Critère 1 — la dispersion en profondeur finie : résidu relatif ≤ 10⁻¹² de 2 à 20 s et de 1 cm à 1 km ; limites
/// profonde et peu profonde à 10⁻⁶ dans leurs domaines ; écart à l'approximation explicite de Fenton et McKee (1990,
/// *On calculating the lengths of water waves*, Coastal Engineering 14, 499–513), `k = k₀/tanh((k₀h)^¾)^⅔`, au plus 1,7 %
/// — la borne que ses auteurs publient.
#[test]
fn dispersion_en_profondeur_finie_s362() {
    let mut residu = 0f64;
    for i in 0..=36 {
        let periode = 2.0 + 0.5 * i as f64;
        let omega = core::f64::consts::TAU / periode;
        for j in 0..=50 {
            let h = 0.01 * 10f64.powf(5.0 * j as f64 / 50.0);
            let k = nombre_d_onde(omega, h, G).unwrap();
            residu = residu.max((omega * omega - G * k * (k * h).tanh()).abs() / (omega * omega));
        }
    }
    // Profond : T = 5 s au-dessus de 1 km, k·h ≈ 161. Peu profond : T = 20 s sur 0,4 mm, k·h ≈ 2·10⁻³.
    let (w_p, w_s) = (core::f64::consts::TAU / 5.0, core::f64::consts::TAU / 20.0);
    let profond = (nombre_d_onde(w_p, 1000.0, G).unwrap() - w_p * w_p / G).abs() / (w_p * w_p / G);
    let k_s = nombre_d_onde(w_s, 4e-4, G).unwrap();
    let peu_profond = (k_s - w_s / (G * 4e-4).sqrt()).abs() / k_s;
    let omega = core::f64::consts::TAU / 8.0;
    let mut fenton = 0f64;
    for i in 0..=4000 {
        let x = 10f64.powf(-3.0 + 5.0 * i as f64 / 4000.0);
        let h = x * G / (omega * omega);
        let k = nombre_d_onde(omega, h, G).unwrap();
        let approchee = (omega * omega / G) / x.powf(0.75).tanh().powf(2.0 / 3.0);
        fenton = fenton.max((k - approchee).abs() / k);
    }
    println!("S362 dispersion residu={residu:.2e} profond={profond:.2e} peu_profond={peu_profond:.2e} fenton_mckee={fenton:.4}");
    assert!(residu <= 1e-12, "{residu}");
    assert!(profond <= 1e-6 && peu_profond <= 1e-6, "{profond} {peu_profond}");
    assert!(fenton <= 0.017, "{fenton}");
    assert!(nombre_d_onde(0.0, 1.0, G).is_none() && nombre_d_onde(1.0, -1.0, G).is_none());
}

/// Critère 2 — le coefficient de levée passe par son minimum, **0,913 vers k·h ≈ 1,2** (Dean et Dalrymple 1991, §4.4 ;
/// la valeur des tables de levée), et vaut 1 en eau profonde.
#[test]
fn coefficient_de_levee_s362() {
    let omega = core::f64::consts::TAU / 10.0;
    let (mut minimum, mut kh_min) = (f64::INFINITY, 0.0);
    for i in 1..=20000 {
        let kh = i as f64 * 5e-4;
        // h tel que k·h = kh : ω² = g·k·tanh(kh) donne k, donc h.
        let k = omega * omega / (G * kh.tanh());
        let ks = coefficient_de_levee(omega, kh / k, G).unwrap();
        if ks < minimum {
            minimum = ks;
            kh_min = kh;
        }
    }
    let profond = coefficient_de_levee(omega, 5000.0, G).unwrap();
    println!("S362 levee minimum={minimum:.5} kh={kh_min:.4} profond={profond:.12}");
    assert!((minimum - 0.913).abs() < 5e-4, "{minimum}");
    assert!((kh_min - 1.2).abs() < 0.05, "{kh_min}");
    assert!((profond - 1.0).abs() < 1e-12);
}

/// Critère 3 — sur une plage plane (de 60 m à 1 m de fond, pente 1/50), une houle de 8 s arrivant à 30° : Snell
/// (`k·sin θ` conservé), le flux d'énergie `a²·c_g·cos θ` constant à 10⁻¹⁰, et la phase intégrée dont la dérivée est
/// `k_y` à 10⁻⁶ près. L'instrument, deux fois corrigé au premier passage : la dérivée se prend **hors du coin du profil**
/// (y = 2 950 m, où la plage s'arrête à 1 m : la différence centrée s'y trompe de 2,4·10⁻³ par construction), et par une
/// différence de **±5 cm** — à ±0,5 m, sa troncature `k_y''·d²/(6·k_y)` vaut 3·10⁻⁶ par 2 m de fond (mesuré 3,1·10⁻⁶),
/// plus que le critère ; à ±5 cm, 3·10⁻⁸.
#[test]
fn refraction_sur_une_plage_s362() {
    let omega = core::f64::consts::TAU / 8.0;
    let (theta0, a0) = (30f64.to_radians(), 0.5);
    let profil = |y: f64| (60.0 - y / 50.0).max(1.0);
    let k0 = omega * omega / G;
    let flux0 = a0 * a0 * (G / (2.0 * omega)) * theta0.cos();
    let (mut snell, mut flux, mut derivee) = (0f64, 0f64, 0f64);
    let mut theta_cote = 0.0;
    for i in 0..=58 {
        let y = 50.0 * i as f64;
        let e = transformer(omega, theta0, a0, profil(y), G).unwrap();
        snell = snell.max((e.k * e.theta.sin() - k0 * theta0.sin()).abs() / k0);
        flux = flux.max((e.amplitude * e.amplitude * e.cg * e.theta.cos() - flux0).abs() / flux0);
        let d = 0.05;
        let avant = phase_transversale(omega, theta0, &profil, 0.0, y - d, G, 2000).unwrap();
        let apres = phase_transversale(omega, theta0, &profil, 0.0, y + d, G, 2000).unwrap();
        derivee = derivee.max(((apres - avant) / (2.0 * d) - e.ky).abs() / e.ky);
        theta_cote = e.theta;
    }
    println!("S362 refraction snell={snell:.2e} flux={flux:.2e} derivee_phase={derivee:.2e} angle_a_1m={:.2}deg", theta_cote.to_degrees());
    assert!(snell <= 1e-12, "{snell}");
    assert!(flux <= 1e-10, "{flux}");
    assert!(derivee <= 1e-6, "{derivee}");
}

/// Critère 4 — le déferlement borné par la profondeur (McCowan 1894) : une houle de 1 m et 10 s de face, puis à 30° ;
/// à la profondeur trouvée, `H = 0,78·h` à 10⁻⁹ ; publiée. Et une houle trop petite pour le fond minimal ne déferle pas.
#[test]
fn profondeur_de_deferlement_s362() {
    let omega = core::f64::consts::TAU / 10.0;
    for theta0 in [0f64, 30f64.to_radians()] {
        let hb = profondeur_de_deferlement(omega, theta0, 1.0, G, 100.0, 0.05).unwrap();
        let e = transformer(omega, theta0, 0.5, hb, G).unwrap();
        let ecart = (2.0 * e.amplitude - MCCOWAN * hb).abs() / hb;
        println!("S362 deferlement theta0={:.0}deg h_b={hb:.4}m H_b={:.4}m ecart={ecart:.1e}", theta0.to_degrees(), 2.0 * e.amplitude);
        assert!(ecart <= 1e-9, "{ecart}");
        assert!(hb > 0.5 && hb < 3.0, "{hb}");
    }
    assert!(profondeur_de_deferlement(omega, 0.0, 0.001, G, 100.0, 0.5).is_none());
}
