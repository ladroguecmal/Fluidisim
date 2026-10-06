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
