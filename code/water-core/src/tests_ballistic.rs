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
