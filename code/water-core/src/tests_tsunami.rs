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
