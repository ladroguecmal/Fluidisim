//! S637 — V qui déclenche δ et le tient par la masse. Références écrites au plan par son script.

use super::*;
use crate::hydro_network::geometry::{Tetrahedron, VolumeShape};

fn boite(lo: [i64; 3], hi: [i64; 3]) -> [Tetrahedron; 6] {
    let v: [[i64; 3]; 8] = std::array::from_fn(|bits|
        std::array::from_fn(|axis| if bits & (1 << axis) == 0 { lo[axis] } else { hi[axis] }));
    [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].map(|p| {
        Tetrahedron::new([v[0], v[1 << p[0]], v[(1 << p[0]) | (1 << p[1])], v[7]]).unwrap()
    })
}

/// (1) le déclenchement ; (2) l'amorçage ; (3) le retard de masse ; (4) la surface plate ; (5) V intact ; (6) refus.
#[test]
fn the_v_node_triggers_delta_and_holds_its_mass_s637() {
    let t = boite([0, 0, 0], [10_000_000, 5_000_000, 2_000_000]);
    let formes = [VolumeShape::new(&t).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let bas = [0.0f32, 0.0, -9.81];
    let mut node = HydroNode { volume_ml: 50_000_000, capacity_ml: formes[0].capacity_ml(), origin_um: [0; 3], shape: 0 };
    let (q_ml, dt_v, tau, aire) = (2_000i64, 0.1, 1.0, 50.0);
    let publie = node.volume_ml;
    let mut declenche_au = None;
    for pas in 1..=100 {
        node.volume_ml += q_ml;
        if declenche(&node, publie, &shapes, bas, 500.0).unwrap() {
            declenche_au = Some(pas);
            break;
        }
    }
    println!("S637 : V déclenche δ au pas {declenche_au:?}");
    assert_eq!(declenche_au, Some(13), "critère 1");

    let niveau = node.volume_ml as f64 * 1e-6 / aire;
    let (nx, ny, dx) = (20usize, 10usize, 0.5);
    let mut dom = amorcer(nx, ny, dx, 9.81, vec![0.0; nx * ny], niveau).unwrap();
    dom.regler_ordre_deux(1e-16).unwrap();
    assert!((dom.h[0] - 1.0005199999999999).abs() < 1e-12 && dom.h.iter().all(|&h| h == dom.h[0]), "critère 2");

    let (mut v_ref, mut m_ref) = (node.volume_ml as f64 * 1e-6, node.volume_ml as f64 * 1e-6);
    let v_depart = node.volume_ml;
    let mut pire = 0.0f64;
    let mut retard = 0.0;
    for _ in 0..300 {
        node.volume_ml += q_ml;
        for _ in 0..4 {
            dom.pas(0.025).unwrap();
        }
        forcer(&mut dom, node.volume_ml as f64 * 1e-6, dt_v, tau).unwrap();
        v_ref += q_ml as f64 * 1e-6;
        m_ref += (v_ref - m_ref) * dt_v / tau;
        retard = node.volume_ml as f64 * 1e-6 - dom.volume();
        pire = pire.max((retard - (v_ref - m_ref)).abs());
    }
    println!("S637 : retard établi {retard} m³ (Q·(τ − dt) = 0,018), écart pas à pas à la référence {pire:e} m³, vitesse max {:e} m/s",
        dom.vitesse_max());
    assert!(pire < 1e-9 && (retard - 0.018).abs() < 1e-9, "critère 3");
    assert!(dom.vitesse_max() < 1e-12, "critère 4");
    assert_eq!(node.volume_ml, v_depart + 300 * q_ml, "critère 5 : V ne reçoit que le robinet");
    assert_eq!(forcer(&mut dom, 1.0, 0.0, 1.0), Err(Refus), "critère 6 : dt_V");
    assert_eq!(forcer(&mut dom, 1.0, 0.1, 0.0), Err(Refus), "critère 6 : τ");
}
