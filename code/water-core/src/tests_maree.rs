//! S577 — la marée harmonique. Références écrites au plan par son script.

use super::*;
use crate::traversabilite::{prochain_franchissement, CrossCause};

fn m2_s2() -> Maree {
    Maree::new(&[Composante { periode_h: M2, amplitude_m: 1.0, phase_tours: 0.0 }, Composante { periode_h: S2, amplitude_m: 0.46, phase_tours: 0.0 }],
        0.0).unwrap()
}

/// (1) M2 seule contre le cosinus idéal sur 15 jours ; (4) le déterminisme et un an.
#[test]
fn a_single_m2_tide_follows_the_ideal_cosine_s577() {
    let maree = Maree::new(&[Composante { periode_h: M2, amplitude_m: 1.0, phase_tours: 0.0 }], 0.0).unwrap();
    let mut pire = 0f64;
    for i in 0..(15 * 24 * 60) {
        let t = i as f64 * 60.0;
        let ideal = (2.0 * std::f64::consts::PI * t / (M2 * 3600.0)).cos();
        pire = pire.max((maree.niveau(SimTime((t * 1e6) as u64)) as f64 - ideal).abs());
    }
    println!("S577 M2 : l'écart au cosinus idéal sur 15 jours, au pire {pire:.2e} m");
    assert!(pire < 1e-3, "critère 1");
    let t = SimTime(123_456_789_012);
    assert_eq!(maree.niveau(t).to_bits(), maree.niveau(t).to_bits(), "critère 4 : au bit");
    let an = maree.niveau(SimTime(365 * 86_400 * 1_000_000));
    assert!(an.is_finite() && an.abs() <= 1.0 + 1e-6, "critère 4 : un an");
}

/// (2) Vives-eaux et mortes-eaux sur 30 jours ; (3) un gué annoncé ; (5) les refus.
#[test]
fn spring_and_neap_tides_and_a_ford_s577() {
    let maree = m2_s2();
    let (mut haut, mut bas) = (f32::MIN, f32::MAX);
    for i in 0..(30 * 24 * 60) {
        let eta = maree.niveau(SimTime(i as u64 * 60_000_000));
        haut = haut.max(eta);
        bas = bas.min(eta);
    }
    println!("S577 M2 + S2 sur 30 jours : de {bas:.4} à {haut:.4} m (−1,4600 et 1,4600)");
    assert!((haut as f64 - 1.46).abs() < 1e-3 && (bas as f64 + 1.4599806361512861).abs() < 1e-3, "critère 2");
    let gue = |t: f64| maree.niveau(SimTime((t * 1e6) as u64)) as f64 + 0.5;
    let f = prochain_franchissement(&gue, 0.0, 86_400.0, 60.0, CrossCause::Maree).unwrap().unwrap();
    println!("S577 gué : {f:?} (6 974,05 s)");
    assert!((f.delai_s - 6974.05).abs() < 1.0 && f.seuil_m == 1.30 && f.trend == -1, "critère 3");
    let trop = [Composante { periode_h: M2, amplitude_m: 0.1, phase_tours: 0.0 }; 9];
    assert_eq!(Maree::new(&trop, 0.0), Err(Refus), "critère 5 : neuf composantes");
    assert_eq!(Maree::new(&[Composante { periode_h: 0.0, amplitude_m: 0.1, phase_tours: 0.0 }], 0.0), Err(Refus), "critère 5 : période");
}

// --- S578 — la carte cotidale. Références écrites au plan par son script.

/// Une onde M2 progressive dans un chenal de 20 m, une carte de 10 km de pas sur 100 km.
fn chenal_s578() -> Vec<[f32; 2]> {
    let k = 2.0 * std::f64::consts::PI / (14.007141035914502 * M2 * 3600.0);
    let mut h = Vec::new();
    for _j in 0..2 {
        for i in 0..11 {
            let x = i as f64 * 10_000.0;
            h.push([(k * x).cos() as f32, -(k * x).sin() as f32]);
        }
    }
    h
}

/// (1) Aux nœuds, l'onde exacte ; (2) au milieu d'une maille, le creux de la corde ; (3) le retard de la pleine mer ; (4) refus, au bit.
#[test]
fn a_cotidal_map_carries_a_progressive_tide_s578() {
    let h = chenal_s578();
    let carte = CarteCotidale::new(&[M2], [0.0, 0.0], 10_000.0, 11, 2, &h, 0.0).unwrap();
    let k = 2.0 * std::f64::consts::PI / (14.007141035914502 * M2 * 3600.0);
    let omega = 2.0 * std::f64::consts::PI / (M2 * 3600.0);
    let (mut pire_noeud, mut max_milieu) = (0f64, 0f64);
    let (mut pic0, mut pic50) = ((f32::MIN, 0.0), (f32::MIN, 0.0));
    for m in 0..(25 * 60) {
        let t = m as f64 * 60.0;
        let st = SimTime((t * 1e6) as u64);
        for i in [0usize, 3, 7, 10] {
            let x = i as f64 * 10_000.0;
            let e = carte.niveau(x, 5_000.0, st).unwrap() as f64;
            pire_noeud = pire_noeud.max((e - (omega * t - k * x).cos()).abs());
        }
        max_milieu = max_milieu.max(carte.niveau(55_000.0, 0.0, st).unwrap() as f64);
        if t < M2 * 3600.0 {
            let (e0, e50) = (carte.niveau(0.0, 0.0, st).unwrap(), carte.niveau(50_000.0, 0.0, st).unwrap());
            if e0 > pic0.0 { pic0 = (e0, t); }
            if e50 > pic50.0 { pic50 = (e50, t); }
        }
    }
    let retard = pic50.1 - pic0.1;
    println!("S578 carte : aux nœuds au pire {pire_noeud:.2e} m ; au milieu, l'amplitude {max_milieu:.6} (1 − 1,258·10⁻³ = 0,998742) ; \
              retard de la pleine mer {retard:.0} s (3 569,6)");
    assert!(pire_noeud < 1e-4, "critère 1");
    assert!((max_milieu - (1.0 - 0.0012577358810847983)).abs() < 1e-4, "critère 2");
    assert!((retard - 3569.6078073176614).abs() <= 60.0, "critère 3");
    assert_eq!(carte.niveau(-1.0, 0.0, SimTime(0)), Err(Refus), "critère 4 : hors de la grille");
    let st = SimTime(987_654_321_000);
    assert_eq!(carte.niveau(12_345.0, 6_789.0, st).unwrap().to_bits(), carte.niveau(12_345.0, 6_789.0, st).unwrap().to_bits(), "critère 4");
}
