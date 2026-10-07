//! S405 — la prédiction balistique (9.3) : des cas à réponse connue.
use super::*;

const G: f64 = 9.81;
const IDENTITY: [f64; 4] = [1., 0., 0., 0.];

fn thrown(position: [f64; 3], velocity: [f64; 3], drag: f64, radius: f64) -> Ballistic {
    Ballistic { position, velocity, orientation: IDENTITY, omega: [0.; 3], inertia: [1.; 3], drag, radius }
}

#[test]
fn in_vacuum_over_a_plane_the_impact_is_the_parabola_s405() {
    // Critère 1 : RK4 est exact sur une parabole, et la bisection affine l'instant à 10⁻¹² s.
    let obj = thrown([1., 2., 6.], [12., 3., 2.], 0., 0.25);
    let t = (2. + (4f64 + 2. * G * 5.75).sqrt()) / G;
    let imp = predict(&obj, G, 0.01, 10., |_, _| 0.).unwrap();
    let (dt, dx, dy) = (imp.time - t, imp.position[0] - (1. + 12. * t), imp.position[1] - (2. + 3. * t));
    println!("S405 parabole : instant {:.12} s, écarts {dt:.1e} s, {dx:.1e} m, {dy:.1e} m", imp.time);
    assert!(dt.abs() <= 1e-9 && dx.abs() <= 1e-8 && dy.abs() <= 1e-8);
    assert!((imp.position[2] - 0.25).abs() <= 1e-8 && (imp.velocity[2] - (2. - G * t)).abs() <= 1e-8);
    assert_eq!(imp.region, 0.25);
    // `advance` est le même intégrateur : à l'instant de l'impact, le même centre.
    let there = advance(&obj, G, imp.time, 0.01);
    assert!((0..3).all(|i| (there.position[i] - imp.position[i]).abs() <= 1e-9), "{there:?}");
    // Pas de contact dans l'horizon ; objet déjà au contact ; paramètres invalides.
    assert!(predict(&obj, G, 0.01, 0.5, |_, _| 0.).is_none());
    assert!(predict(&thrown([0., 0., 0.2], [1., 0., 0.], 0., 0.25), G, 0.01, 10., |_, _| 0.).is_none());
    assert!(predict(&obj, 0., 0.01, 10., |_, _| 0.).is_none());
    assert!(predict(&obj, G, 0., 10., |_, _| 0.).is_none());
    assert!(predict(&Ballistic { drag: -1., ..obj }, G, 0.01, 10., |_, _| 0.).is_none());
    assert!(predict(&Ballistic { inertia: [1., 0., 1.], ..obj }, G, 0.01, 10., |_, _| 0.).is_none());
}

#[test]
fn a_fall_with_quadratic_drag_converges_at_fourth_order_s405() {
    // Critère 2 : chute verticale depuis le repos, `v_t = √(g/k)` : `z(t) = z₀ − (v_t²/g)·ln cosh(g·t/v_t)`, d'où l'instant
    // exact ; l'ordre se lit sur trois pas.
    let (z0, k) = (20f64, 0.05f64);
    let vt = (G / k).sqrt();
    let exact = vt / G * ((z0 * G / (vt * vt)).exp()).acosh();
    let obj = thrown([0., 0., z0], [0.; 3], k, 0.);
    let err = |h: f64| predict(&obj, G, h, 10., |_, _| 0.).unwrap().time - exact;
    let e: Vec<f64> = [0.2, 0.1, 0.05].iter().map(|h| err(*h)).collect();
    let fine = err(0.01);
    println!(
        "S405 traînée : instant exact {exact:.9} s ; écarts {:.2e}, {:.2e}, {:.2e} s (rapports {:.2}, {:.2}) ; à 10 ms {fine:.1e} s",
        e[0], e[1], e[2], e[0] / e[1], e[1] / e[2]
    );
    for r in [e[0] / e[1], e[1] / e[2]] {
        assert!((12.8..=19.2).contains(&r), "rapport {r}");
    }
    assert!(fine.abs() <= 1e-6);
}

#[test]
fn free_rotation_follows_the_analytic_top_s405() {
    // Critère 3 : toupie symétrique `I = (1, 1, 2)` — dans le repère du corps, `ω⊥` tourne à `Ω = (I₃ − I₁)/I₁·ω₃` ; rotation
    // autour d'un axe principal — `q(t) = (cos(ω·t/2), 0, 0, sin(ω·t/2))`. Une chute d'environ 2 s.
    let mut obj = thrown([0., 0., 20.], [0.; 3], 0., 0.);
    obj.inertia = [1., 1., 2.];
    obj.omega = [0.3, 0., 2.];
    let imp = predict(&obj, G, 0.001, 10., |_, _| 0.).unwrap();
    let (t, big) = (imp.time, 2f64);
    let top = [0.3 * (big * t).cos(), 0.3 * (big * t).sin(), 2.];
    let e_top = (0..3).map(|i| (imp.omega[i] - top[i]).abs()).fold(0f64, f64::max);
    obj.inertia = [1., 2., 3.];
    obj.omega = [0., 0., 3.];
    let imp = predict(&obj, G, 0.001, 10., |_, _| 0.).unwrap();
    let q = [(1.5 * imp.time).cos(), 0., 0., (1.5 * imp.time).sin()];
    let e_axis = (0..4).map(|i| (imp.orientation[i] - q[i]).abs()).fold(0f64, f64::max);
    println!("S405 rotation : toupie {e_top:.1e} rad/s, axe principal {e_axis:.1e} (sur {:.3} s)", imp.time);
    assert!(e_top <= 1e-8 && e_axis <= 1e-8);
}

#[test]
fn a_moving_surface_is_met_where_it_is_s405() {
    // Critère 4 : une houle de 5 cm, λ = 16 m, vue par un objet lancé à 12 m/s de 6 m, traînée faible : l'instant de contact à
    // 10 ms contre la référence à 0,1 ms ; publié, ce que coûterait d'ignorer la houle.
    let k = core::f64::consts::TAU / 16.;
    let w = (G * k).sqrt();
    let swell = |p: [f64; 2], t: f64| 0.05 * (k * p[0] - w * t).sin();
    let obj = thrown([0., 0., 6.], [12., 0., 0.], 0.01, 0.25);
    let reference = predict(&obj, G, 1e-4, 10., swell).unwrap();
    let coarse = predict(&obj, G, 0.01, 10., swell).unwrap();
    let flat = predict(&obj, G, 0.01, 10., |_, _| 0.).unwrap();
    let d = coarse.time - reference.time;
    println!(
        "S405 surface mouvante : contact à {:.6} s, x = {:.4} m ; pas de 10 ms : {d:.1e} s ; sans la houle : {:+.2} ms, {:+.3} m",
        reference.time,
        reference.position[0],
        1e3 * (flat.time - reference.time),
        flat.position[0] - reference.position[0]
    );
    assert!(d.abs() <= 1e-3);
    assert!((reference.position[2] - 0.25 - swell([reference.position[0], reference.position[1]], reference.time)).abs() <= 1e-9);
}

#[test]
fn tiers_follow_adr_013_s405() {
    // Critère 5 : T1 sous 0,3 s ; T2 sous 8 s si `½·a_max·t²` tient dans le domaine ; T3 sous 8 s ; T4 sinon.
    assert_eq!(tier(9., 0., 20.), Tier::Watch);
    assert_eq!(tier(f64::INFINITY, 0., 20.), Tier::Watch);
    assert_eq!(tier(f64::NAN, 0., 20.), Tier::Watch);
    assert_eq!(tier(7.99, 0., 20.), Tier::Build);
    assert_eq!(tier(7.99, 20., 20.), Tier::Reserve);
    // L'avion de chasse d'ADR-013 §2 : 20 m/s², 20 m — T2 jusqu'à √2 s.
    assert_eq!(tier(1.41, 20., 20.), Tier::Build);
    assert_eq!(tier(1.42, 20., 20.), Tier::Reserve);
    assert_eq!(tier(0.29, 20., 20.), Tier::Active);
    assert_eq!(tier(0.3, 0., 20.), Tier::Build);
    assert!(Tier::Active < Tier::Build && Tier::Build < Tier::Reserve && Tier::Reserve < Tier::Watch);
}

#[test]
fn the_useful_region_covers_a_poorly_known_drag_s405() {
    // Critère 6 : une traînée connue à ±30 % près ; toute traînée dans les bornes tombe dans la région.
    let obj = thrown([0., 0., 6.], [12., 3., 2.], 0.02, 0.25);
    let bounds = [0.014, 0.026];
    let imp = predict_region(&obj, bounds, G, 0.01, 10., |_, _| 0.).unwrap();
    let mut worst = 0f64;
    for n in 0..=20 {
        let drag = bounds[0] + (bounds[1] - bounds[0]) * n as f64 / 20.;
        let other = predict(&Ballistic { drag, ..obj }, G, 0.01, 10., |_, _| 0.).unwrap();
        worst = worst.max((other.position[0] - imp.position[0]).hypot(other.position[1] - imp.position[1]));
    }
    println!("S405 région : rayon {:.3} m (sphère 0,25 m) ; la plus éloignée des 21 traînées à {worst:.3} m", imp.region);
    assert!(worst <= imp.region - 0.25 + 1e-9);
    assert!(imp.region > 0.5);
}

/// Les huit sommets d'une boîte de demi-côtés `h`.
fn boite(h: [f64; 3]) -> [[f64; 3]; 8] {
    core::array::from_fn(|n| [if n & 1 == 0 { -h[0] } else { h[0] }, if n & 2 == 0 { -h[1] } else { h[1] }, if n & 4 == 0 { -h[2] } else { h[2] }])
}

/// **S537, critères 1 et 2 — le contact d'un corps quelconque.** Une boîte alignée lâchée de 10 m touche à `√(2(z₀ − h)/g)` à 10⁻⁹ s ;
/// une planche de 4 × 0,2 × 0,2 m tournant à 3 rad/s autour de `x` (sa demi-longueur selon `y`) touche par un coin à la racine de
/// `z₀ − ½gt² − (h_y|sin ωt| + h_z|cos ωt|)`, à 10⁻⁹ s d'une bisection indépendante ; sa sphère englobante, 38,6 ms trop tôt.
#[test]
fn a_tumbling_plank_touches_by_its_lowest_corner_s537() {
    let alignee = Ballistic { position: [0., 0., 10.], velocity: [0.; 3], orientation: IDENTITY, omega: [0.; 3], inertia: [1.; 3], drag: 0., radius: 0. };
    let imp = predict_hull(&alignee, &boite([0.5, 0.5, 0.1]), G, 0.01, 10., |_, _| 0.).unwrap();
    let attendu = (2. * (10. - 0.1) / G).sqrt();
    println!("S537 boîte alignée : {:.12} s (attendu {attendu:.12}), écart {:.1e} s", imp.time, imp.time - attendu);
    assert!((imp.time - attendu).abs() <= 1e-9, "critère 1");
    assert!((imp.region - (0.5f64 * 0.5 + 0.5 * 0.5 + 0.1 * 0.1).sqrt()).abs() <= 1e-12);
    let (hy, hz, w) = (2.0, 0.1, 3.0);
    // Inertie d'une planche : I_x le plus grand ou le plus petit — une rotation autour d'un axe principal reste constante.
    let planche = Ballistic { omega: [w, 0., 0.], inertia: [1.0, 0.0034, 1.0], radius: (hy * hy + hz * hz + 0.01f64).sqrt(), ..alignee };
    let imp = predict_hull(&planche, &boite([0.1, hy, hz]), G, 0.01, 10., |_, _| 0.).unwrap();
    let f = |t: f64| 10. - 0.5 * G * t * t - (hy * (w * t).sin().abs() + hz * (w * t).cos().abs());
    let mut t = 0.;
    while f(t + 1e-4) > 0. {
        t += 1e-4;
    }
    let (mut lo, mut hi) = (t, t + 1e-4);
    for _ in 0..200 {
        let m = 0.5 * (lo + hi);
        if f(m) > 0. { lo = m } else { hi = m }
    }
    let sphere = predict(&planche, G, 0.01, 10., |_, _| 0.).unwrap();
    println!(
        "S537 planche : {:.12} s (bisection indépendante {lo:.12}), écart {:.1e} s ; la sphère englobante {:.6} s ({:.1} ms trop tôt)",
        imp.time, imp.time - lo, sphere.time, 1e3 * (lo - sphere.time)
    );
    assert!((imp.time - lo).abs() <= 1e-9, "critère 2");
}

/// **S602 — 9.4, les paliers de confiance des objets contrôlables.** Références écrites au plan par son script (la table d'ADR-013 au dixième).
#[test]
fn controllable_objects_lose_tiers_when_the_game_lowers_confidence_s602() {
    let pleine = Confiance::PLEINE;
    let h = [horizon_utile(20.0, 20.0, pleine).unwrap(), horizon_utile(3.0, 20.0, pleine).unwrap(), horizon_utile(5.0, 60.0, pleine).unwrap()];
    println!("S602 : horizons {h:?} (1,4142 ; 3,6515 ; 4,8990 — ADR-013 : 1,4 ; 3,7 ; 4,9)");
    for (m, r) in h.iter().zip([1.4142135623730951, 3.6514837167011076, 4.898979485566356]) {
        assert!((m - r).abs() < 1e-12, "critère 1");
    }
    let reduite = Confiance { facteur_jeu: 2.0 };
    assert_eq!(palier_controlable(4.0, 5.0, 60.0, pleine), Some(Tier::Build), "critère 2 : T2 en pleine confiance");
    assert_eq!(palier_controlable(4.0, 5.0, 60.0, reduite), Some(Tier::Reserve), "critère 2 : T3 quand le jeu la divise par deux");
    assert!((horizon_utile(5.0, 60.0, reduite).unwrap() - 3.4641016151377544).abs() < 1e-12, "critère 2 : l'horizon réduit");
    for i in 0..20 {
        for j in 0..10 {
            let (t, a) = (0.05 + 0.5 * i as f64, 2.0 * j as f64);
            assert_eq!(palier_controlable(t, a, 30.0, pleine), Some(tier(t, a, 30.0)), "critère 2 : facteur 1, le palier de tier");
        }
    }
    assert_eq!((reevaluation_s(Tier::Watch, 1.0 / 30.0), reevaluation_s(Tier::Build, 1.0 / 30.0)), (0.5, 1.0 / 30.0), "critère 3");
    assert_eq!(palier_controlable(4.0, 5.0, 60.0, Confiance { facteur_jeu: 0.5 }), None, "critère 4");
    assert_eq!(horizon_utile(5.0, 60.0, Confiance { facteur_jeu: f64::NAN }), None, "critère 4");
}
