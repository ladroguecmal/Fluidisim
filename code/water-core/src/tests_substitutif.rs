//! S609 — le régime substitutif. Références écrites au plan par son script (une implémentation numpy indépendante).

use super::*;

const G: f64 = 9.81;
const H: f64 = 2.0;
const DX: f64 = 0.5;
const N: usize = 400;
const DT: f64 = 0.05;

fn b(x: f64, t: f64) -> (f64, f64) {
    let k = 2.0 * core::f64::consts::PI / 40.0;
    let e = 0.1 * (k * x - k * (G * H).sqrt() * t).cos();
    (e, (G / H).sqrt() * e)
}

fn lancer(bosse: &[f64], pas: usize) -> Domaine1D {
    let mut d = Domaine1D::depuis_b(G, H, DX, N, DT, &b, bosse).unwrap();
    for _ in 0..pas {
        d.avancer(&b);
    }
    d
}

fn ecart(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f64::max)
}

/// (1) B seul ; (2) la bosse qui sort ; (3) la bascule ; (4) refus.
#[test]
fn a_substitutive_domain_owns_the_total_field_and_lets_disturbances_out_s609() {
    let seul = lancer(&[], 1200);
    let attendu: Vec<f64> = (0..N).map(|i| b((i as f64 + 0.5) * DX, seul.temps()).0).collect();
    let err_b = ecart(seul.eta(), &attendu);
    println!("S609 : B seul, écart à 60 s : {err_b:e} m");
    assert!((err_b - 6.285449409718349e-4).abs() < 1e-9 && err_b < 1e-3, "critère 1");

    let bosse: Vec<f64> = (0..N).map(|i| 0.05 * (-(((i as f64 + 0.5) * DX - 100.0) / 5.0).powi(2)).exp()).collect();
    let moitie = ecart(lancer(&bosse, 200).eta(), lancer(&[], 200).eta());
    let reste = ecart(lancer(&bosse, 1200).eta(), seul.eta());
    println!("S609 : la bosse, deux moitiés de {moitie} m à 10 s ; à 60 s, il reste {reste:e} m ({} % d'une moitié)", 100.0 * reste / 0.025);
    assert!((moitie - 0.02500255353597207).abs() < 1e-9, "critère 2 : la moitié");
    assert!((reste - 3.256827165280529e-4).abs() < 1e-9 && reste < 0.02 * 0.025, "critère 2 : le reste");

    // S642 : le seuil est celui de l'appelant (ADR-112 D1 : `0,35·Hs` n'est pas une règle reçue).
    assert_eq!(mode_requis(0.35, 0.35, false), Ok(Mode::Perturbatif), "critère 3 : au seuil");
    assert_eq!(mode_requis(0.3500001, 0.35, false), Ok(Mode::Substitutif), "critère 3 : au-delà");
    assert_eq!(mode_requis(0.0, 0.35, true), Ok(Mode::Substitutif), "critère 3 : par nature");
    assert_eq!(mode_requis(0.1, -1.0, false), Err(Refus), "S642 : seuil négatif");
    assert_eq!(mode_requis(0.1, f64::NAN, false), Err(Refus), "S642 : seuil non fini");

    assert!(Domaine1D::depuis_b(G, 0.0, DX, N, DT, &b, &[]).is_err(), "critère 4 : profondeur");
    assert!(Domaine1D::depuis_b(G, H, 0.0, N, DT, &b, &[]).is_err(), "critère 4 : dx");
    assert!(Domaine1D::depuis_b(G, H, DX, N, 0.0, &b, &[]).is_err(), "critère 4 : pas");
    assert!(Domaine1D::depuis_b(G, H, DX, N, 0.12, &b, &[]).is_err(), "critère 4 : Courant ≥ 1");
}
