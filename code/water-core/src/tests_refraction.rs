//! S583 — la réfraction par tracé de rayons. Références écrites au plan par son script (Snell ; Simpson pour l'arrivée).

use super::*;

fn plage(x: f64, _y: f64) -> (f64, [f64; 2]) {
    (4000.0 - 0.0195 * x, [-0.0195, 0.0])
}

/// (1) Snell le long du rayon ; (2) l'arrivée à 190 km ; (3) `K_r`.
#[test]
fn a_ray_bends_by_snells_law_over_parallel_isobaths_s583() {
    let th0 = 30f64.to_radians();
    let mut a = vec![Point::default(); 2_000];
    let mut b = vec![Point::default(); 2_000];
    let na = tracer([0.0, 0.0], th0, 9.81, &plage, 1.0, &mut a).unwrap();
    let delta = 100.0;
    // Le rayon voisin, de la même famille : son angle par Snell à son abscisse (la première mesure le lançait à 30°, un autre invariant —
    // 0,981 au lieu de 0,935 ; un tracé indépendant l'a confirmé).
    let xb = -delta * th0.sin();
    let thb = (th0.sin() * ((4000.0 - 0.0195 * xb) / 4000.0f64).sqrt()).asin();
    let nb = tracer([xb, delta * th0.cos()], thb, 9.81, &plage, 1.0, &mut b).unwrap();
    let c = |p: &Point| (9.81 * plage(p.x, p.y).0).sqrt();
    let invariant0 = th0.sin() / c(&a[0]);
    let pire = a[..na].iter().map(|p| (p.theta.sin() / c(p) / invariant0 - 1.0).abs()).fold(0f64, f64::max);
    let k = a[..na].iter().position(|p| p.x >= 190e3).expect("le rayon passe 190 km");
    let f = (190e3 - a[k - 1].x) / (a[k].x - a[k - 1].x);
    let (t, y) = (a[k - 1].t + f * (a[k].t - a[k - 1].t), a[k - 1].y + f * (a[k].y - a[k - 1].y));
    let kr = coefficient(&a[k], &b[k], delta);
    println!("S583 : {na} et {nb} points ; sin θ/c au pire {pire:.1e} ; à 190 km : t = {t:.4} s (1 604,6227), y = {y:.3} m (72 949,931), \
              θ = {:.6}° (7,804001), K_r = {kr:.6} (0,934944)", a[k].theta.to_degrees());
    assert!(pire < 1e-9, "critère 1");
    assert!((t - 1604.622718335518).abs() < 0.01 && (y - 72949.93122821147).abs() < 0.1, "critère 2");
    assert!((kr - 0.9349444896626372).abs() < 1e-4, "critère 3");
}

/// (4) Un fond uniforme : le rayon droit, `θ` constant au bit ; les refus.
#[test]
fn a_flat_bottom_keeps_the_ray_straight_s583() {
    let mut a = vec![Point::default(); 100];
    let n = tracer([0.0, 0.0], 0.3, 9.81, &|_, _| (50.0, [0.0, 0.0]), 1.0, &mut a).unwrap();
    assert_eq!(n, 100);
    assert!(a.iter().all(|p| p.theta.to_bits() == 0.3f64.to_bits()), "critère 4 : θ constant au bit");
    assert_eq!(tracer([0.0, 0.0], 0.3, 9.81, &|_, _| (0.0, [0.0, 0.0]), 1.0, &mut a), Err(Refus), "critère 4 : profondeur");
    assert_eq!(tracer([0.0, 0.0], 0.3, 9.81, &|_, _| (50.0, [0.0, 0.0]), 0.0, &mut a), Err(Refus), "critère 4 : pas");
    assert_eq!(tracer([0.0, 0.0], 0.3, 9.81, &|_, _| (50.0, [0.0, 0.0]), 1.0, &mut []), Err(Refus), "critère 4 : tampon");
}
