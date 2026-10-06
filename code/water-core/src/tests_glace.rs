//! S574 — la glace : Stefan, Gold, l'échéance d'une charge. Références écrites au plan par son script.

use super::*;
use crate::traversabilite::{echantillon_glace, prochain_franchissement_de, CrossCause};

/// (1) Stefan en degrés-jours ; (2) Gold et la table de SPEC-002 ; (4) l'émergé, la plaque ; (5) les refus.
#[test]
fn stefan_and_gold_reproduce_the_spec_tables_s574() {
    for (fdd, h) in [(10.0, 0.11141029090954438), (50.0, 0.2491209838667681), (100.0, 0.3523102740561124), (200.0, 0.4982419677335362)] {
        let m = epaisseur_fdd(fdd).unwrap();
        println!("S574 Stefan : {fdd} K·jour → {m:.6} m");
        assert!((m - h).abs() < 1e-6, "critère 1 : {fdd}");
    }
    let masses = [("personne", 100.0), ("groupe ou motoneige", 400.0), ("voiture légère", 1500.0), ("camion léger", 5000.0)];
    let attendu = [(0.05, 87.5, "rien"), (0.10, 350.0, "personne"), (0.20, 1400.0, "groupe ou motoneige"), (0.30, 3150.0, "voiture légère"),
                   (0.50, 8750.0, "camion léger")];
    for (h, p, porte) in attendu {
        let c = charge_admissible_kg(h).unwrap();
        let classe = masses.iter().filter(|(_, m)| *m <= c).last().map_or("rien", |(n, _)| *n);
        println!("S574 Gold : {h} m → {c:.1} kg, porte : {classe}");
        assert!((c / p - 1.0).abs() < 1e-6 && classe == porte, "critère 2 : {h}");
    }
    assert!((fraction_emergee(1000.0).unwrap() - 0.083).abs() < 1e-6, "critère 4 : l'émergé");
    assert_eq!((prend_en_plaque(0.15), prend_en_plaque(0.149)), (Ok(false), Ok(true)), "critère 4 : la plaque");
    assert_eq!(epaisseur(-0.1, 10.0, 1.0), Err(Refus), "critère 5");
    assert_eq!(epaisseur(0.1, -1.0, 1.0), Err(Refus), "critère 5");
    assert_eq!(charge_admissible_kg(f64::NAN), Err(Refus), "critère 5");
}

/// (3) L'échéance : depuis 10 cm sous 10 K de gel, quand la glace portera-t-elle une voiture légère ? Et l'échantillon publié.
#[test]
fn a_lake_announces_when_its_ice_will_bear_a_car_s574() {
    let h_min = epaisseur_requise(1500.0).unwrap();
    let croissance = |t: f64| epaisseur(0.10, 10.0, t).unwrap();
    let f = prochain_franchissement_de(&[h_min as f32], &croissance, 0.0, 10.0 * 86_400.0, 3_600.0, CrossCause::Aucune).unwrap().unwrap();
    println!("S574 échéance : h_min = {h_min:.5} m, portée dans {:.1} s (228 714,1)", f.delai_s);
    assert!((f.delai_s - 228_714.09090909085).abs() < 1.0 && f.trend == 1, "critère 3");
    let e = echantillon_glace(0.0, 0.0, 0.30).unwrap();
    println!("S574 échantillon : {e:?}");
    assert!((e.ice_capacity_kg as f64 - 3150.0).abs() < 1e-2 && e.ice_h == 0.30, "l'échantillon porte la charge dérivée");
    // La vérification ajoutée en route, écrite d'abord aux notes (ADR-244 D1).
    use crate::traversabilite::{echantillon, porte_par_la_glace};
    assert!(porte_par_la_glace(1500.0, &e) && !porte_par_la_glace(5000.0, &e), "la voiture oui, le camion non");
    assert!(!porte_par_la_glace(1.0, &echantillon(0.0, 0.0).unwrap()), "sans glace, rien");
}
