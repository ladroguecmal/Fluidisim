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

// --- S575 — C15, la glace d'un lac dans V. Références écrites au plan par son script.

use crate::hydro_network::HydroNode;

/// Un lac de 10 × 10 m, 2 m d'eau ; `jours` de gel à 10 K sous la houle `hs`, au pas d'une heure. Rend le nœud, la composition, et
/// vérifie la masse et la somme à chaque pas.
fn lac_s575(jours: usize, hs: f64) -> (HydroNode, [i64; 2]) {
    let mut node = HydroNode { volume_ml: 200_000_000, capacity_ml: 300_000_000, origin_um: [0; 3], shape: 0 };
    let mut ligne = [200_000_000i64, 0];
    let masse = |l: &[i64; 2]| 1000 * l[0] + 917 * l[1];
    let m0 = masse(&ligne);
    // L'état exact est le gel cumulé (s), pas la glace arrondie au quantum : repartir chaque heure de l'épaisseur quantifiée accumulait
    // la troncature (jusqu'à un quantum par pas, 2,4 mm en 720 pas — la première mesure).
    let mut gel_s = 0.0;
    for _ in 0..jours * 24 {
        if prend_en_plaque(hs).unwrap() {
            gel_s += 3_600.0;
            let visee = epaisseur(0.0, 10.0, gel_s).unwrap();
            ajuster_glace(&mut node, &mut ligne, 0, 1, 100.0, visee).unwrap();
        }
        assert_eq!(masse(&ligne), m0, "critère 2 : la masse");
        assert_eq!(ligne[0] + ligne[1], node.volume_ml, "critère 4 : la somme");
    }
    (node, ligne)
}

/// C15 : (1) l'épaisseur après 30 jours ; (2) le dégel rend l'eau ; (3) pas de plaque sous la houle.
#[test]
fn a_sheltered_lake_freezes_and_thaws_without_losing_water_s575() {
    let (mut node, mut ligne) = lac_s575(30, 0.0);
    let h = ligne[1] as f64 / 1e8;
    let spec = 0.035 * 300f64.sqrt();
    println!("S575 C15 : glace {h:.6} m (Stefan 0,610219 ; C15 {spec:.6}, {:+.2} %) ; {} quanta ; eau {} ml ; nœud {} ml", 100.0 * (h / spec - 1.0),
        ligne[1] / 1000, ligne[0], node.volume_ml);
    assert!((h / spec - 1.0).abs() < 0.10, "critère 1 : à ± 10 % de C15");
    assert!((h - 0.6102192946937021).abs() < 1e-5, "critère 1 : à un quantum de Stefan");
    // Le dégel, sur dix jours : l'épaisseur visée décroît jusqu'à zéro.
    for jour in (0..10).rev() {
        ajuster_glace(&mut node, &mut ligne, 0, 1, 100.0, h * jour as f64 / 10.0).unwrap();
        assert_eq!(1000 * ligne[0] + 917 * ligne[1], 1000 * 200_000_000, "critère 2 : la masse pendant le dégel");
    }
    println!("S575 C15 : après le dégel, eau {} ml, glace {} ml, nœud {} ml", ligne[0], ligne[1], node.volume_ml);
    assert_eq!((ligne, node.volume_ml), ([200_000_000, 0], 200_000_000), "critère 2 : l'eau rendue au millilitre");
    let (_, sous_la_houle) = lac_s575(30, 0.2);
    assert_eq!(sous_la_houle, [200_000_000, 0], "critère 3 : pas de plaque sous Hs = 0,2 m");
}
