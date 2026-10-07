//! S587 — la bulle d'une explosion sous-marine. Références écrites au plan par son script.

use super::*;

/// (1) Le rayon et la période ; (2) les lois d'échelle ; (3) les refus.
#[test]
fn an_underwater_charge_has_its_bubble_period_s587() {
    let b = bulle(1.0, 20.0, 0.4, 1025.0, 9.81).unwrap();
    println!("S587 : 1 kg à 20 m — R_max {:.6} m (1,097268), T {:.8} s (0,11685897)", b.rayon_max_m, b.periode_s);
    assert!((b.rayon_max_m / 1.097268039332142 - 1.0).abs() < 1e-9, "critère 1 : le rayon");
    assert!((b.periode_s / 0.11685896896878312 - 1.0).abs() < 1e-9, "critère 1 : la période");
    let huit = bulle(8.0, 20.0, 0.4, 1025.0, 9.81).unwrap();
    let profond = bulle(1.0, 60.0, 0.4, 1025.0, 9.81).unwrap();
    println!("S587 : ×8 de charge → T ×{:.12} ; à 60 m → T ×{:.12} (0,494175617660)", huit.periode_s / b.periode_s, profond.periode_s / b.periode_s);
    assert!((huit.periode_s / b.periode_s - 2.0).abs() < 1e-12, "critère 2 : T ∝ W^(1/3)");
    assert!((profond.periode_s / b.periode_s / 0.4941756176601117 - 1.0).abs() < 1e-12, "critère 2 : T ∝ p^(−5/6)");
    assert_eq!(bulle(0.0, 20.0, 0.4, 1025.0, 9.81), Err(Refus), "critère 3 : masse");
    assert_eq!(bulle(1.0, -1.0, 0.4, 1025.0, 9.81), Err(Refus), "critère 3 : profondeur");
    assert_eq!(bulle(1.0, 20.0, 0.0, 1025.0, 9.81), Err(Refus), "critère 3 : fraction");
}
