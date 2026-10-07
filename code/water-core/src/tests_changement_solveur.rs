//! S612 — le changement de solveur par W. Références écrites au plan par son script (`s612_ref.py`, numpy indépendant).

use super::*;

const G: f64 = 9.81;
const H: f64 = 2.0;
const DX: f64 = 0.5;
const N: usize = 400;
const DT: f64 = 0.05;

fn gauss(x: f64) -> f64 {
    0.05 * (-((x - 100.0) / 5.0).powi(2)).exp()
}

fn exact(x: f64, t: f64) -> f64 {
    let c = (G * H).sqrt();
    0.5 * gauss(x - c * t) + 0.5 * gauss(x + c * t)
}

fn domaine(pas: usize) -> Domaine1D {
    let bosse: Vec<f64> = (0..N).map(|i| gauss((i as f64 + 0.5) * DX)).collect();
    let mut d = Domaine1D::depuis_b(G, H, DX, N, DT, &|_, _| (0.0, 0.0), &bosse).unwrap();
    for _ in 0..pas {
        d.avancer(&|_, _| (0.0, 0.0));
    }
    d
}

/// (1) la continuité ; (2) l'énergie ; (3) l'écart à l'exact ; (4) le nouveau solveur ; (5) hors du domaine.
#[test]
fn a_solver_change_goes_through_w_without_any_state_conversion_s612() {
    let a = domaine(100);
    let eta_s = a.eta().to_vec();
    let e_delta = energie_delta(&a, 1000.0);
    let w = transduire(a);
    let continuite = eta_s.iter().enumerate().map(|(i, e)| (w.eta((i as f64 + 0.5) * DX, 5.0) - e).abs()).fold(0.0, f64::max);
    println!("S612 : continuité {continuite:e} m");
    assert!(continuite < 1e-15, "critère 1 (assemblage)");
    let rel = (w.energie(1000.0) - e_delta) / e_delta;
    println!("S612 : énergie δ {e_delta} J/m, W {} J/m ({} %)", w.energie(1000.0), 100.0 * rel);
    assert!((e_delta - 76.88147600444304).abs() < 1e-9 * 76.9 && (w.energie(1000.0) - 76.78552044815758).abs() < 1e-9 * 76.8, "critère 2");
    assert!(rel.abs() < 0.005, "critère 2 : sous 0,5 %");

    let continue_a = domaine(300);
    let err_a = continue_a.eta().iter().enumerate().map(|(i, e)| (e - exact((i as f64 + 0.5) * DX, 15.0)).abs()).fold(0.0, f64::max);
    let err_w = (0..N).map(|i| (w.eta((i as f64 + 0.5) * DX, 15.0) - exact((i as f64 + 0.5) * DX, 15.0)).abs()).fold(0.0, f64::max);
    println!("S612 : à 15 s, solveur continué {err_a:e} m, W {err_w:e} m");
    assert!((err_a - 3.1500193083892035e-4).abs() < 1e-9 && (err_w - 2.3824942983781207e-4).abs() < 1e-9, "critère 3 : les références");
    assert!(err_w <= err_a, "critère 3 : la bascule n'ajoute rien");

    let ext = |x: f64, t: f64| (w.eta(x + 150.0, t + 5.0), w.u(x + 150.0, t + 5.0));
    let mut nouveau = Domaine1D::depuis_b(G, H, 0.25, 400, 0.025, &ext, &[]).unwrap();
    for _ in 0..400 {
        nouveau.avancer(&ext);
    }
    let ecart = nouveau.eta().iter().enumerate().map(|(i, e)| (e - w.eta(150.0 + (i as f64 + 0.5) * 0.25, 15.0)).abs()).fold(0.0, f64::max);
    let crete = nouveau.eta().iter().fold(0.0f64, |m, e| m.max(e.abs()));
    println!("S612 : le nouveau solveur, crête {crete} m, écart à W {ecart:e} m");
    assert!((ecart - 3.4298479105587115e-5).abs() < 1e-9 && ecart < 0.01 * 0.025, "critère 4");

    assert_eq!((w.eta(-1000.0, 5.0), w.eta(1.0e4, 5.0)), (0.0, 0.0), "critère 5");
}
