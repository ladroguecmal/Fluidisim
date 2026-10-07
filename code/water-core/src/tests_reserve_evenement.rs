//! S600 — la réserve d'événement (ADR-012 §6). Références comptées au plan par son script.

use super::*;

fn renforces(demande: &[bool], critique: bool) -> Vec<bool> {
    let mut r = ReserveEvenement::ADR_012;
    demande.iter().map(|&d| r.budget(2.0, critique, d).unwrap() > 2.0).collect()
}

/// (1) La demande continue ; (2) deux événements ; (3) les bornes ; (4) les refus.
#[test]
fn the_event_reserve_is_bounded_in_size_duration_and_average_s600() {
    let continu = renforces(&[true; 1_800], true);
    let n = continu.iter().filter(|x| **x).count();
    let mut d = vec![true; 60];
    d.extend([false; 300]);
    d.extend([true; 60]);
    let deux = renforces(&d, true);
    let (n1, n2) = (deux[..60].iter().filter(|x| **x).count(), deux[360..].iter().filter(|x| **x).count());
    println!("S600 : continu {n} ticks renforcés sur 1 800 (165) ; deux événements {n1} et {n2} (15, 15)");
    assert_eq!(n, 165, "critère 1");
    assert_eq!((n1, n2), (15, 15), "critère 2");
    assert!(renforces(&[true; 1_800], false).iter().all(|x| !x), "critère 3 : non critique");
    let mut suite = 0;
    for x in &continu {
        suite = if *x { suite + 1 } else { 0 };
        assert!(suite <= 15, "critère 3 : au plus 15 ticks de suite");
    }
    let mut r = ReserveEvenement::ADR_012;
    for _ in 0..1_000 {
        assert!(r.budget(2.0, true, true).unwrap() <= 3.0, "critère 3 : jamais au-dessus de 1,5 × le nominal");
    }
    let mut r = ReserveEvenement::ADR_012;
    assert_eq!(r.budget(0.0, true, true), Err(Refus), "critère 4");
}
