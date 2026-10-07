//! S582 — la propagation macroscopique d'un tsunami. Références écrites au plan par son script (Simpson pour la formule).

use super::*;

const PROFIL: [[f64; 2]; 3] = [[0.0, 4000.0], [1.0e6, 4000.0], [1.1e6, 10.0]];

/// (1) Le temps de parcours ; (2) Green et le flux.
#[test]
fn travel_time_and_greens_law_s582() {
    let r = Rayon::new(&PROFIL, 9.81).unwrap();
    let (t_plat, t_cote) = (r.temps(1.0e6).unwrap(), r.temps(1.1e6).unwrap());
    println!("S582 τ : {t_plat:.6} s (5 048,187773), {t_cote:.6} s (6 009,747349) ; la pente {:.6} s (961,559576)", t_cote - t_plat);
    assert!((t_plat - 5048.187773461523).abs() < 1e-6, "critère 1 : le plateau");
    assert!((t_cote - 6009.747349358956).abs() < 1e-6, "critère 1 : la côte");
    assert!((t_cote - t_plat - 961.5595758974329).abs() < 1e-6, "critère 1 : la pente");
    let green = r.amplitude(1.0, 1.1e6).unwrap();
    assert!((green / 4.47213595499958 - 1.0).abs() < 1e-9, "critère 2 : Green, {green}");
    let flux = |s: f64| r.amplitude(1.0, s).unwrap().powi(2) * r.profondeur(s).unwrap().sqrt();
    let f0 = flux(0.0);
    for s in [5.0e5, 1.02e6, 1.05e6, 1.1e6] {
        assert!((flux(s) / f0 - 1.0).abs() < 1e-12, "critère 2 : le flux en {s}");
    }
}

/// (3) Le pic arrive à `t₀ + τ(s)`, d'amplitude `A(s)` à l'instant exact ; rien avant ; (4) au bit, refus.
#[test]
fn the_pulse_arrives_when_and_as_high_as_predicted_s582() {
    let r = Rayon::new(&PROFIL, 9.81).unwrap();
    let tsu = Tsunami { a0_m: 0.5, t0_s: 100.0, demi_duree_s: 600.0 };
    let s = 1.05e6;
    let tau = r.temps(s).unwrap();
    let mut pic = (f32::MIN, 0.0);
    let debut = (100.0 + tau - 600.0).floor() as i64 - 5;
    for t in debut..(debut + 1220) {
        let e = niveau(&r, &tsu, s, t as f64).unwrap();
        if (t as f64) < 100.0 + tau - 600.0 {
            assert_eq!(e, 0.0, "critère 3 : rien avant l'onde (t = {t})");
        }
        if e > pic.0 {
            pic = (e, t as f64);
        }
    }
    let exact = niveau(&r, &tsu, s, 100.0 + tau).unwrap() as f64;
    let attendu = r.amplitude(0.5, s).unwrap();
    println!("S582 arrivée en {s} m : pic à {} s (t₀ + τ = {:.3} s), amplitude exacte {exact:.7} m ({attendu:.7})", pic.1, 100.0 + tau);
    assert!((pic.1 - (100.0 + tau)).abs() <= 1.0, "critère 3 : l'instant");
    assert!((exact / attendu - 1.0).abs() < 1e-6, "critère 3 : l'amplitude à l'instant exact");
    assert_eq!(niveau(&r, &tsu, s, 5_432.1).unwrap().to_bits(), niveau(&r, &tsu, s, 5_432.1).unwrap().to_bits(), "critère 4 : au bit");
    assert_eq!(Rayon::new(&[[0.0, 10.0]], 9.81).err(), Some(Refus), "critère 4 : un sommet");
    assert_eq!(Rayon::new(&[[0.0, 10.0], [0.0, 5.0]], 9.81).err(), Some(Refus), "critère 4 : s non croissant");
    assert_eq!(Rayon::new(&[[0.0, 10.0], [1.0, 0.0]], 9.81).err(), Some(Refus), "critère 4 : profondeur nulle");
    assert_eq!(r.temps(1.2e6), Err(Refus), "critère 4 : hors du profil");
}

// --- S584 — le tsunami sur un rayon courbe. Références écrites au plan par son script.

/// (1) L'arrivée ; (2) l'amplitude, Green et la réfraction ensemble ; (3) le flux dans le tube ; (4) refus.
#[test]
fn a_tsunami_on_a_bent_ray_keeps_its_energy_flux_s584() {
    use crate::refraction::{tracer, Point};
    let plage = |x: f64, _y: f64| (4000.0 - 0.0195 * x, [-0.0195, 0.0]);
    let th0 = 30f64.to_radians();
    let (mut a, mut b) = (vec![Point::default(); 2_000], vec![Point::default(); 2_000]);
    tracer([0.0, 0.0], th0, 9.81, &plage, 1.0, &mut a).unwrap();
    let delta = 100.0;
    let xb = -delta * th0.sin();
    let thb = (th0.sin() * ((4000.0 - 0.0195 * xb) / 4000.0f64).sqrt()).asin();
    tracer([xb, delta * th0.cos()], thb, 9.81, &plage, 1.0, &mut b).unwrap();
    let k = a.iter().position(|p| p.x >= 190e3).unwrap();
    let f = (190e3 - a[k - 1].x) / (a[k].x - a[k - 1].x);
    let (t0, _) = sur_rayon(0.5, 4000.0, &a[k - 1], &b[k - 1], delta, plage(a[k - 1].x, 0.0).0).unwrap();
    let (t1, amp) = sur_rayon(0.5, 4000.0, &a[k], &b[k], delta, plage(a[k].x, 0.0).0).unwrap();
    let arrivee = t0 + f * (t1 - t0);
    // La valeur attendue au point a[k] lui-même, calculée dans l'essai (la formule du plan, à son abscisse) : Green × √(cos θ₀/cos θ).
    let h = plage(a[k].x, 0.0).0;
    let sin_t = th0.sin() * (h / 4000.0f64).sqrt();
    let attendu = 0.5 * (4000.0 / h).sqrt().sqrt() * (th0.cos() / (1.0 - sin_t * sin_t).sqrt()).sqrt();
    println!("S584 : arrivée {arrivee:.4} s (1 604,6227) ; amplitude {amp:.6} m pour {attendu:.6} (à 190 km : 0,897047)");
    assert!((arrivee - 1604.622718335518).abs() < 0.01, "critère 1");
    assert!((amp / attendu - 1.0).abs() < 1e-4, "critère 2");
    let flux = |i: usize| {
        let (_, aa) = sur_rayon(0.5, 4000.0, &a[i], &b[i], delta, plage(a[i].x, 0.0).0).unwrap();
        let bb = delta / crate::refraction::coefficient(&a[i], &b[i], delta).powi(2);
        aa * aa * plage(a[i].x, 0.0).0.sqrt() * bb
    };
    let f0 = flux(1);
    for i in [300, 700, 1100, k] {
        assert!((flux(i) / f0 - 1.0).abs() < 1e-4, "critère 3 : le flux au pas {i}");
    }
    assert_eq!(sur_rayon(0.5, 4000.0, &a[k], &b[k], delta, 0.0), Err(Refus), "critère 4 : profondeur");
    assert_eq!(sur_rayon(0.5, 4000.0, &a[k], &b[k], 0.0, 100.0), Err(Refus), "critère 4 : écart nul");
}
