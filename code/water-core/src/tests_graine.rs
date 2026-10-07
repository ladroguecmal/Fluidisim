//! S623 — la graine d'un domaine substitutif. Références écrites au plan par son script (`s623_ref.py`, numpy).

use super::*;
use crate::precalcul::etablissement;

const G: f64 = 9.81;
const T_CUISSON: f64 = 120.0;

fn b(x: f64, t: f64) -> (f64, f64) {
    let k = 2.0 * core::f64::consts::PI / 40.0;
    let e = 0.1 * (k * x - k * (G * 2.0f64).sqrt() * t).cos();
    (e, (G / 2.0f64).sqrt() * e)
}

fn parametres(hs: f64) -> Parametres {
    Parametres { hs_m: hs, tp_s: 9.0305, theta_rad: 0.0, phase_maree: 2, liquide: 0 }
}

const TOL: Tolerance = Tolerance { hs_relatif: 0.1, tp_s: 0.5, theta_rad: 0.1 };

/// (1) le volume ; (2) l'établissement ; (3) la tolérance ; (4) le choix ; (5) l'empreinte ; (6) refus.
#[test]
fn a_seeded_substitutive_domain_is_established_at_once_s623() {
    let mut d = Domaine1D::au_repos(G, 2.0, 0.5, 400, 0.05).unwrap();
    for _ in 0..2400 {
        d.avancer(&b);
    }
    let graine = condense(&d, Genre::Bassin, parametres(0.2));
    let ml = 399_998_810i64;
    let r = graine.restaurer(&parametres(0.2), &TOL, ml, G, 0.05).unwrap();
    let v = r.eta().iter().map(|e| e + 2.0).sum::<f64>() * 0.5;
    println!("S623 : volume restauré {v} m³ (nœud {ml} ml)");
    assert!((v - ml as f64 * 1e-6).abs() < 1e-6, "critère 1");

    let decale = |x: f64, t: f64| b(x, t + T_CUISSON);
    let ecart = r.eta().iter().enumerate().map(|(i, e)| (e - decale((i as f64 + 0.5) * 0.5, 0.0).0).abs()).fold(0.0, f64::max);
    let mut r2 = r.clone();
    let t = etablissement(&mut r2, &decale, 0.05 * 0.1, 2400, 0.5).unwrap();
    println!("S623 : écart initial {ecart} m ; établi en {t} s (depuis le repos : 60,65 s, S610)");
    assert!((ecart - 0.002478421580659841).abs() < 1e-9 && ecart < 0.005 && t == 0.05, "critère 2");

    assert!(graine.restaurer(&parametres(0.219), &TOL, ml, G, 0.05).is_ok(), "critère 3 : dedans");
    assert!(graine.restaurer(&parametres(0.23), &TOL, ml, G, 0.05).is_err(), "critère 3 : hs");
    assert!(graine.restaurer(&Parametres { tp_s: 9.6, ..parametres(0.2) }, &TOL, ml, G, 0.05).is_err(), "critère 3 : tp");
    assert!(graine.restaurer(&Parametres { theta_rad: 0.11, ..parametres(0.2) }, &TOL, ml, G, 0.05).is_err(), "critère 3 : θ");
    assert!(graine.restaurer(&Parametres { phase_maree: 3, ..parametres(0.2) }, &TOL, ml, G, 0.05).is_err(), "critère 3 : phase");

    let graines: Vec<SeedState> = [0.1, 0.2, 0.4, 0.8].iter().map(|&hs| SeedState { parametres: parametres(hs), ..graine.clone() }).collect();
    assert_eq!(choisir(&graines, &parametres(0.21), &TOL).map(|g| g.parametres.hs_m), Some(0.2), "critère 4 : la plus proche");
    assert!(choisir(&graines, &parametres(0.3), &TOL).is_none(), "critère 4 : aucune");

    assert_eq!(condense(&d, Genre::Bassin, parametres(0.2)).id, graine.id, "critère 5 : le même contenu");
    assert_eq!(graine.empreinte(), graine.id, "critère 5 : l'empreinte recalculée");
    let mut autre = graine.clone();
    autre.h[17] ^= 1;
    assert_ne!(autre.empreinte(), graine.id, "critère 5 : un f16 changé");

    assert!(graine.restaurer(&parametres(0.2), &TOL, 0, G, 0.05).is_err(), "critère 6 : volume");
    let mut tronquee = graine.clone();
    tronquee.u.pop();
    assert!(tronquee.restaurer(&parametres(0.2), &TOL, ml, G, 0.05).is_err(), "critère 6 : tailles");
}
