//! S597 — le nuage de microbulles. Références écrites au plan par son script (la taille d'ensemble vérifiée, ADR-250 D1).

use super::*;
use crate::bulle::{Bulle, EAU_DOUCE};

/// (1) La fraction analytique contre une population de bulles intégrées une à une.
#[test]
fn a_microbubble_cloud_clears_as_its_bubbles_rise_s597() {
    let classes = [
        Classe { diametre_m: 50e-6, nombre_par_m2: 1e7 },
        Classe { diametre_m: 100e-6, nombre_par_m2: 5e6 },
        Classe { diametre_m: 200e-6, nombre_par_m2: 1e6 },
    ];
    let nuage = Nuage::new(&classes, 1.0, 9.81, EAU_DOUCE).unwrap();
    let instants = [10.0, 30.0, 60.0, 120.0];
    let n = 10_000usize;
    let mut pire = 0f64;
    for (k, c) in classes.iter().enumerate() {
        // Les profondeurs initiales, uniformes sur [0, 1 m] par une suite déterministe (le milieu de n intervalles, permutés).
        let mut bulles: Vec<Bulle> = (0..n).map(|i| {
            let j = (i * 7_919) % n;
            Bulle { position: [0.0, 0.0, -(j as f64 + 0.5) / n as f64], vitesse: [0.0; 3], diametre: c.diametre_m }
        }).collect();
        let mut t = 0.0;
        let dt = 0.05;
        for &cible in &instants {
            while t < cible - 1e-9 {
                for b in bulles.iter_mut().filter(|b| b.position[2] < 0.0) {
                    b.pas([0.0, 0.0, -9.81], EAU_DOUCE, [0.0; 3], dt);
                }
                t += dt;
            }
            let restantes = bulles.iter().filter(|b| b.position[2] < 0.0).count() as f64 / n as f64;
            let analytique = nuage.fraction(k, cible);
            println!("S597 classe {} µm, t = {cible} s : {restantes:.4} restantes (analytique {analytique:.4}, v = {:.3} mm/s)",
                c.diametre_m * 1e6, nuage.vitesse(k) * 1e3);
            pire = pire.max((restantes - analytique).abs());
        }
    }
    println!("S597 : l'écart au pire {pire:.4}");
    assert!(pire < 0.05, "critère 1");
    // (2) τ(0) et l'opacité décroissante.
    let tau0: f64 = classes.iter().map(|c| c.nombre_par_m2 * 2.0 * std::f64::consts::PI * (c.diametre_m / 2.0).powi(2)).sum();
    assert!((nuage.epaisseur_optique(0.0) / tau0 - 1.0).abs() < 1e-12, "critère 2 : τ(0)");
    let mut precedent = nuage.opacite(0.0);
    for i in 1..=200 {
        let o = nuage.opacite(i as f64 * 5.0);
        assert!(o <= precedent, "critère 2 : l'opacité décroît");
        precedent = o;
    }
    println!("S597 : τ(0) = {tau0:.4}, opacité {:.4} → {:.4} à 120 s → {:.4} à 1000 s", nuage.opacite(0.0), nuage.opacite(120.0), nuage.opacite(1000.0));
    // (3) Les refus.
    assert_eq!(Nuage::new(&[], 1.0, 9.81, EAU_DOUCE), Err(Refus), "critère 3");
    assert_eq!(Nuage::new(&classes, 0.0, 9.81, EAU_DOUCE), Err(Refus), "critère 3");
    assert_eq!(Nuage::new(&[Classe { diametre_m: 0.0, nombre_par_m2: 1.0 }], 1.0, 9.81, EAU_DOUCE), Err(Refus), "critère 3");
}
