//! S565 — la solution d'un réseau en charge contre deux références indépendantes, écrites au plan (bissection ; Hardy Cross).

use super::*;
use crate::hydro_network::charge::{resoudre, tampon, Conduite, Sommet};

const TOL: f64 = 1e-13;

/// (1) Trois réservoirs reliés à une jonction.
#[test]
fn three_reservoirs_meet_at_the_head_of_the_bisection_s565() {
    let fixes = [100.0, 80.0, 50.0];
    let conduites = [
        Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 2000.0 },
        Conduite { a: Sommet::Fixe(1), b: Sommet::Jonction(0), resistance: 3000.0 },
        Conduite { a: Sommet::Fixe(2), b: Sommet::Jonction(0), resistance: 1500.0 },
    ];
    let (mut h, mut q, mut w) = ([75.0], [0.0; 3], vec![0.0; tampon(1)]);
    let rapport = resoudre(&fixes, &[0.0], &conduites, &mut h, &mut q, &mut w, TOL, 30).unwrap();
    println!("S565 trois réservoirs : h_j = {:.9} m (77,455794994), débits {q:?}, {rapport:?}", h[0]);
    assert!((h[0] - 77.455794994).abs() < 1e-8, "critère 1 : la charge");
    for (m, r) in q.iter().zip([0.106170158, 0.029121613, -0.135291771]) {
        assert!((m - r).abs() < 1e-9, "critère 1 : un débit {m} pour {r}");
    }
    assert!(rapport.residu_m3s < 1e-12 && rapport.iterations < 30);
}

/// (2) Une maille, contre Hardy Cross.
#[test]
fn a_loop_matches_hardy_cross_s565() {
    let fixes = [60.0];
    let j = Sommet::Jonction;
    let conduites = [
        Conduite { a: Sommet::Fixe(0), b: j(0), resistance: 500.0 },
        Conduite { a: j(0), b: j(1), resistance: 1000.0 },
        Conduite { a: j(1), b: j(2), resistance: 1500.0 },
        Conduite { a: j(0), b: j(3), resistance: 1200.0 },
        Conduite { a: j(3), b: j(2), resistance: 800.0 },
    ];
    let demandes = [0.0, 0.06, 0.08, 0.04];
    let (mut h, mut q, mut w) = ([60.0; 4], [0.0; 5], vec![0.0; tampon(4)]);
    let rapport = resoudre(&fixes, &demandes, &conduites, &mut h, &mut q, &mut w, TOL, 30).unwrap();
    println!("S565 maille : charges {h:?}, débits {q:?}, {rapport:?}");
    for (m, r) in h.iter().zip([43.8, 34.964659639, 33.231017516, 34.924075772]) {
        assert!((m - r).abs() < 1e-8, "critère 2 : une charge {m} pour {r}");
    }
    for (m, r) in q.iter().zip([0.18, 0.093996491217, 0.033996491217, 0.086003508783, 0.046003508783]) {
        assert!((m - r).abs() < 1e-9, "critère 2 : un débit {m} pour {r}");
    }
    assert!(rapport.residu_m3s < 1e-12 && rapport.iterations < 30);
}

/// (3) Les refus.
#[test]
fn an_isolated_junction_or_a_bad_pipe_is_refused_s565() {
    let ok = Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 100.0 };
    let ile = Conduite { a: Sommet::Jonction(1), b: Sommet::Jonction(2), resistance: 100.0 };
    let (mut h, mut q, mut w) = ([0.0; 3], [0.0; 2], vec![0.0; tampon(3)]);
    assert_eq!(resoudre(&[10.0], &[0.0; 3], &[ok, ile], &mut h, &mut q, &mut w, TOL, 30).err(), Some(Error::Domain), "une île");
    let nulle = Conduite { resistance: 0.0, ..ok };
    let (mut h1, mut q1, mut w1) = ([0.0], [0.0], vec![0.0; tampon(1)]);
    assert_eq!(resoudre(&[10.0], &[0.0], &[nulle], &mut h1, &mut q1, &mut w1, TOL, 30).err(), Some(Error::Domain), "une résistance nulle");
    let mut court = vec![0.0; tampon(1) - 1];
    assert_eq!(resoudre(&[10.0], &[0.0], &[ok], &mut h1, &mut q1, &mut court, TOL, 30).err(), Some(Error::Capacity), "un tampon court");
}
