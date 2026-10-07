//! S615 — deux clients, deux δ, une grande forme. Références écrites au plan par son script (`s615_ref.py`, numpy).

use super::*;
use crate::changement_solveur::transduire;

const G: f64 = 9.81;
const H: f64 = 2.0;

fn client(dx: f64, n: usize, dt: f64, pas: usize, rides: bool) -> Domaine1D {
    let bosse: Vec<f64> = (0..n).map(|i| {
        let x = (i as f64 + 0.5) * dx;
        let r = if rides && (x - 100.0).abs() < 20.0 { 0.001 * (2.0 * core::f64::consts::PI * x / 1.0 + 0.7).sin() } else { 0.0 };
        0.05 * (-((x - 100.0) / 5.0).powi(2)).exp() + r
    }).collect();
    let mut d = Domaine1D::depuis_b(G, H, dx, n, dt, &|_, _| (0.0, 0.0), &bosse).unwrap();
    for _ in 0..pas {
        d.avancer(&|_, _| (0.0, 0.0));
    }
    d
}

fn ecart(a: &TrainW1D, b: &TrainW1D) -> f64 {
    (0..=800).map(|i| (a.eta(0.25 * i as f64, 15.0) - b.eta(0.25 * i as f64, 15.0)).abs()).fold(0.0, f64::max)
}

/// L'interpolation linéaire d'un profil aux centres `(i + ½)·dx`, à l'intérieur.
fn interp(v: &[f64], dx: f64, x: f64) -> f64 {
    let s = x / dx - 0.5;
    let j = (s.floor() as usize).min(v.len() - 2);
    v[j] + (s - j as f64) * (v[j + 1] - v[j])
}

/// (1) les W des deux clients ; (2) les restes ; (3) la fuite des rides ; (4) sans coupure ; (5) refus.
#[test]
fn two_clients_with_different_deltas_share_one_large_form_through_w_s615() {
    let (a, b) = (client(0.5, 400, 0.05, 100, false), client(0.25, 800, 0.025, 200, true));
    let (wa, reste_a) = transduire_coupe(&a, 1.0).unwrap();
    let (wb, reste_b) = transduire_coupe(&b, 1.0).unwrap();
    let e = ecart(&wa, &wb);
    let crete = (0..=800).map(|i| wa.eta(0.25 * i as f64, 15.0).abs()).fold(0.0, f64::max);
    println!("S615 : les W à 15 s diffèrent de {e:e} m (crête {crete} m)");
    assert!((e - 1.1164202309026809e-4).abs() < 1e-9 && e < 0.01 * crete, "critère 1");

    let er = (0..400).map(|i| (reste_a[i] - interp(&reste_b, 0.25, (i as f64 + 0.5) * 0.5)).abs()).fold(0.0, f64::max);
    println!("S615 : les restes locaux diffèrent de {er:e} m");
    assert!((er - 4.328374884210416e-4).abs() < 1e-9 && er > 0.25e-3, "critère 2");

    let (wb0, _) = transduire_coupe(&client(0.25, 800, 0.025, 200, false), 1.0).unwrap();
    let fuite = ecart(&wb, &wb0);
    println!("S615 : les rides fuient dans W de {fuite:e} m");
    assert!((fuite - 2.7345379616957257e-5).abs() < 1e-9 && fuite < 0.05e-3, "critère 3");

    let brut = ecart(&transduire(client(0.5, 400, 0.05, 100, false)), &transduire(client(0.25, 800, 0.025, 200, true)));
    println!("S615 : sans coupure, les W diffèrent de {brut:e} m");
    assert!((brut - 4.335598842766788e-4).abs() < 1e-9 && brut > 3.0 * e, "critère 4");

    assert_eq!(filtre_gaussien(&[1.0], 0.5, 0.0), Err(Refus), "critère 5 : σ");
    assert_eq!(filtre_gaussien(&[1.0], 0.0, 1.0), Err(Refus), "critère 5 : dx");
}
