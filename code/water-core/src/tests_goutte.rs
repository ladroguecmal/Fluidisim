//! S595 — le vol d'une goutte. Références écrites au plan par son script (son propre code : bissection, RK4 au pas de 10 µs).

use super::*;

/// (1) Les vitesses terminales ; (2) le vol ; (3) la goutte lourde et le vide ; (4) les refus.
#[test]
fn a_droplet_falls_at_its_terminal_speed_and_lands_where_predicted_s595() {
    let m = Milieu::AIR;
    let (v1, v2) = (vitesse_terminale(10e-6, &m).unwrap(), vitesse_terminale(1e-3, &m).unwrap());
    let c = vol(0.5e-3, 10.0, std::f64::consts::FRAC_PI_4, &m, 1e-4, 10.0).unwrap().unwrap();
    println!("S595 : terminale 10 µm {v1:.6} m/s (0,011992), 1 mm {v2:.4} m/s (6,9556) ; vol {:.5} s (0,90367), portée {:.4} m (2,3030)",
        c.temps_s, c.portee_m);
    assert!((v1 / 0.01199162559889631 - 1.0).abs() < 1e-3 && (v2 / 6.955609011930068 - 1.0).abs() < 1e-3, "critère 1");
    assert!((c.temps_s / 0.9036691913229765 - 1.0).abs() < 1e-3 && (c.portee_m / 2.302979431734419 - 1.0).abs() < 1e-3, "critère 2");
    let vide = Milieu { rho_air: 0.0, ..m };
    let p = vol(0.5e-3, 10.0, std::f64::consts::FRAC_PI_4, &vide, 1e-4, 10.0).unwrap().unwrap();
    println!("S595 : sans air, la portée {:.9} m (v²/g = 10,193679918)", p.portee_m);
    assert!((p.portee_m / (100.0 / 9.81) - 1.0).abs() < 1e-6, "critère 3");
    assert_eq!(vitesse_terminale(0.0, &m), Err(Refus), "critère 4");
    assert_eq!(vol(1e-3, 0.0, 0.5, &m, 1e-4, 10.0), Err(Refus), "critère 4");
    assert_eq!(vol(1e-3, 1.0, 0.5, &m, 0.0, 10.0), Err(Refus), "critère 4");
}
