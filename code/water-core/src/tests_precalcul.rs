//! S610 — le précalcul avant l'impact. Références écrites au plan par son script.

use super::*;

fn b(x: f64, t: f64) -> (f64, f64) {
    let (g, h) = (9.81f64, 2.0f64);
    let k = 2.0 * core::f64::consts::PI / 40.0;
    let e = 0.1 * (k * x - k * (g * h).sqrt() * t).cos();
    (e, (g / h).sqrt() * e)
}

/// (1) l'établissement ; (2) la translation ; (3) rebâtir, réallouer, `dx`, libérer ; (4) refus.
#[test]
fn a_prepared_domain_moves_without_error_and_a_substitutive_one_must_establish_s610() {
    let mut d = Domaine1D::au_repos(9.81, 2.0, 0.5, 400, 0.05).unwrap();
    let t = etablissement(&mut d, &b, 0.05 * 0.1, 2400, 0.5).unwrap();
    let (c, periode) = ((9.81f64 * 2.0).sqrt(), 40.0 / (9.81f64 * 2.0).sqrt());
    println!("S610 : établi à {t} s (L/c = {} s, + 2T = {} s)", 200.0 / c, 200.0 / c + 2.0 * periode);
    assert!((t - 60.65).abs() < 0.5 * 0.05, "critère 1 : la référence");
    assert!(t >= 200.0 / c && t <= 200.0 / c + 2.0 * periode, "critère 1 : la borne d'ADR-013 §4");

    let p0 = [100.3, 50.7];
    let mut prep = Preparation::nouvelle(p0, 9.0, 0.25, 86).unwrap();
    println!("S610 : {} blocs préparés", prep.blocs.len());
    assert_eq!(prep.blocs.len(), 86, "critère 2 : la préparation");
    assert!(prep.delta_initial().all(|v| v.to_bits() == 0) && prep.delta_initial().count() == 86 * 64, "critère 2 : δ = 0 (assemblage)");
    let p1 = [p0[0] + 6.0, p0[1] - 4.0];
    assert_eq!(prep.reviser(Some((p1, 0.25))), Decision::Translater { di: 3, dj: -2 }, "critère 2 : translater");
    let mut translatee = prep.blocs.clone();
    translatee.sort_unstable();
    assert_eq!(translatee, Preparation::nouvelle(p1, 9.0, 0.25, 86).unwrap().blocs, "critère 2 : la préparation directe, au bit");

    let p2 = [p0[0] + 0.3, p0[1] + 0.3];
    let mut a = Preparation::nouvelle(p0, 9.0, 0.25, 90).unwrap();
    assert_eq!(a.reviser(Some((p2, 0.25))), Decision::Rebatir, "critère 3 : rebâtir dans 90");
    assert_eq!(a.blocs.len(), 89, "critère 3 : 89 blocs");
    let mut r = Preparation::nouvelle(p0, 9.0, 0.25, 86).unwrap();
    assert_eq!(r.reviser(Some((p2, 0.25))), Decision::Reallouer { blocs: 89 }, "critère 3 : réallouer");
    let mut x = Preparation::nouvelle(p0, 9.0, 0.25, 86).unwrap();
    assert_eq!(x.reviser(Some((p0, 0.5))), Decision::RebatirDx, "critère 3 : un autre dx");
    let mut l = Preparation::nouvelle(p0, 9.0, 0.25, 86).unwrap();
    assert_eq!(l.reviser(None), Decision::Liberer, "critère 3 : libérer");
    assert!(l.blocs.is_empty());

    assert_eq!(Preparation::nouvelle(p0, 0.0, 0.25, 86), Err(Refus), "critère 4 : rayon");
    assert_eq!(Preparation::nouvelle(p0, 9.0, 0.0, 86), Err(Refus), "critère 4 : dx");
    assert_eq!(Preparation::nouvelle(p0, 9.0, 0.25, 85), Err(Refus), "critère 4 : capacité");
}
