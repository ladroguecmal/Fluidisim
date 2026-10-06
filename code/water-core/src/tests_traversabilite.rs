//! S570 — l'échantillon de traversabilité et le prochain franchissement. Références écrites au plan par son script.

use super::*;

/// (1) Le produit de danger et ses classes ; les classes de profondeur aux bornes.
#[test]
fn the_hazard_product_and_its_classes_follow_adr_018_s570() {
    for (d, v, hr, classe) in [(0.5f32, 2.0f32, 1.25f32, Danger::PourLaPlupart), (0.3, 0.5, 0.3, Danger::Faible),
                               (1.0, 1.0, 1.5, Danger::PourLaPlupart), (1.2, 2.0, 3.0, Danger::PourTous)] {
        let e = echantillon(d, v).unwrap();
        println!("S570 d = {d} m, v = {v} m/s : HR = {} ({:?}), profondeur {:?}", e.hazard, e.danger, e.profondeur);
        assert!((e.hazard - hr).abs() < 1e-6 && e.danger == classe, "critère 1 : {d}, {v}");
    }
    let bornes = [(0.149f32, Profondeur::Negligeable), (0.15, Profondeur::Ralentie), (0.5, Profondeur::Entravee),
                  (0.999, Profondeur::Entravee), (1.0, Profondeur::Precaire), (1.3, Profondeur::Nage)];
    for (d, p) in bornes {
        assert_eq!(echantillon(d, 0.0).unwrap().profondeur, p, "critère 1 : la profondeur {d}");
    }
    // La borne basse incluse : 1,5 m d'eau immobile, HR = 1,5·0,5 = 0,75 — « dangereux pour certains ».
    assert_eq!(echantillon(1.5, 0.0).unwrap().danger, Danger::PourCertains, "critère 1 : la borne 0,75");
    assert_eq!(echantillon(-0.1, 0.0), Err(Refus), "critère 3");
    assert_eq!(echantillon(0.1, f32::NAN), Err(Refus), "critère 3");
}

/// (2) Une marée M2 : les franchissements de seuil, leur délai, leur seuil et leur sens.
#[test]
fn a_tide_announces_its_next_crossing_s570() {
    let periode = 12.42 * 3600.0;
    let maree = |t: f64| 0.8 + 0.4 * (2.0 * std::f64::consts::PI * t / periode).sin();
    let cas = [
        ("mi-marée montante", 0.0, 3726.0, 1.0f32, 1i8),
        ("pleine mer", periode / 4.0, 7452.0, 1.0, -1),
        ("mi-marée descendante", periode / 2.0, 6034.92493402856, 0.5, -1),
    ];
    for (nom, t0, attendu, seuil, sens) in cas {
        let f = prochain_franchissement(&maree, t0, periode, 60.0, CrossCause::Maree).unwrap().unwrap();
        println!("S570 {nom} : {f:?} (attendu {attendu:.3} s, {seuil} m, {sens:+})");
        assert!((f.delai_s - attendu).abs() < 1e-2, "critère 2 : le délai, {nom}");
        assert_eq!((f.seuil_m, f.trend, f.cause), (seuil, sens, CrossCause::Maree), "critère 2 : le seuil et le sens, {nom}");
    }
    // Une eau étale à 0,8 m : aucun franchissement.
    assert_eq!(prochain_franchissement(&|_| 0.8, 0.0, 3600.0, 60.0, CrossCause::Aucune).unwrap(), None, "critère 2 : rien");
    assert_eq!(prochain_franchissement(&maree, 0.0, 0.0, 60.0, CrossCause::Maree), Err(Refus), "critère 3");
    assert_eq!(prochain_franchissement(&maree, 0.0, 3600.0, -1.0, CrossCause::Maree), Err(Refus), "critère 3");
}

// --- S572 — les tuiles. Références écrites au plan par son script.

fn plage_s572(i: usize, t: f64) -> f64 {
    let periode = 12.42 * 3600.0;
    let fond = -2.0 + 4.0 * ((i as f64 + 0.5) * 64.0) / 1024.0;
    (0.4 * (2.0 * std::f64::consts::PI * t / periode).sin() - fond).max(0.0)
}

/// (1) Une plage sous la marée : 80 franchissements entre deux publications ; (2) la prévision de la colonne 6 ; (3) Morton, séquences.
#[test]
fn a_tile_on_a_beach_publishes_its_crossings_s572() {
    let periode = 12.42 * 3600.0;
    let t1 = periode / 12.0;
    let mut tuile = TileDesc { frame: 0, tile_morton: morton(3, 5), cadence: Cadence::Maree, subdivision: 0, sequence: 0 };
    let vide = Cellule { echantillon: echantillon(0.0, 0.0).unwrap(), t_next_cross: 0.0, cause: CrossCause::Aucune };
    let mut cellules = vec![vide; 256];
    let mut evenements = vec![CrossingEvent { cellule: 0, avant: (Profondeur::Negligeable, Danger::Faible),
        apres: (Profondeur::Negligeable, Danger::Faible) }; 256];
    let n0 = publier(&mut tuile, true, &|i, _| (plage_s572(i, 0.0) as f32, 0.0), None, &mut cellules, &mut evenements).unwrap();
    let prevision = Prevision { profondeur: &|i, _, t| plage_s572(i, t), t: t1, horizon_s: periode, pas_s: 60.0, cause: CrossCause::Maree };
    let n1 = publier(&mut tuile, false, &|i, _| (plage_s572(i, t1) as f32, 0.0), Some(&prevision), &mut cellules, &mut evenements).unwrap();
    let mut colonnes: Vec<usize> = evenements[..n1].iter().map(|e| e.cellule as usize % 16).collect();
    colonnes.sort();
    colonnes.dedup();
    println!("S572 : {n0} puis {n1} événements, colonnes {colonnes:?} ; Morton {} ; séquence {} ; colonne 6 : {:?}", tuile.tile_morton,
        tuile.sequence, cellules[6]);
    assert_eq!((n0, n1), (0, 80), "critère 1 : le nombre");
    assert_eq!(colonnes, vec![2, 3, 4, 6, 7], "critère 1 : les colonnes");
    assert!(evenements[..n1].iter().all(|e| e.avant != e.apres), "critère 1 : avant et après diffèrent");
    let c6 = cellules[6];
    assert!((c6.t_next_cross as f64 - 16368.32335745605).abs() < 1e-2 && c6.cause == CrossCause::Maree, "critère 2");
    assert_eq!((tuile.tile_morton, tuile.sequence), (39, 2), "critère 3");
}

/// (3) Une subdivision 1 donne 32 × 32 échantillons ; (4) les refus.
#[test]
fn a_subdivided_tile_has_more_samples_and_short_buffers_are_refused_s572() {
    let mut tuile = TileDesc { frame: 0, tile_morton: 0, cadence: Cadence::Debit, subdivision: 1, sequence: 7 };
    assert_eq!(tuile.echantillons(), 1024, "critère 3");
    let vide = Cellule { echantillon: echantillon(0.0, 0.0).unwrap(), t_next_cross: 0.0, cause: CrossCause::Aucune };
    let ev = CrossingEvent { cellule: 0, avant: (Profondeur::Negligeable, Danger::Faible), apres: (Profondeur::Negligeable, Danger::Faible) };
    let (mut cellules, mut evenements) = (vec![vide; 1024], vec![ev; 1024]);
    assert_eq!(publier(&mut tuile, true, &|_, _| (0.3, 1.0), None, &mut cellules, &mut evenements), Ok(0));
    assert_eq!(tuile.sequence, 8);
    let mut court = vec![vide; 1023];
    assert_eq!(publier(&mut tuile, true, &|_, _| (0.3, 1.0), None, &mut court, &mut evenements), Err(Refus), "critère 4 : tampon");
    let mut trop = TileDesc { subdivision: 5, ..tuile };
    assert_eq!(publier(&mut trop, true, &|_, _| (0.3, 1.0), None, &mut cellules, &mut evenements), Err(Refus), "critère 4 : subdivision");
    assert_eq!(publier(&mut tuile, true, &|_, _| (-1.0, 1.0), None, &mut cellules, &mut evenements), Err(Refus), "critère 4 : profondeur");
    assert_eq!(tuile.sequence, 8, "critère 4 : rien d'écrit");
}

// --- S573 — l'invalidation ; la praticabilité par agent. Références écrites au plan par son script.

/// (1) Le prochain changement d'un véhicule, d'un bateau, d'un humanoïde sous la marée de S570 ; (2) les bornes.
#[test]
fn each_agent_has_its_own_next_change_s573() {
    let periode = 12.42 * 3600.0;
    let maree = |t: f64| 0.8 + 0.4 * (2.0 * std::f64::consts::PI * t / periode).sin();
    let vehicule = Agent::Vehicule { gue_m: 0.6 };
    let bateau = Agent::Bateau { tirant_m: 0.9, marge_m: 0.2 };
    let v = prochain_changement(vehicule, &maree, periode / 2.0, periode, 60.0, CrossCause::Maree).unwrap().unwrap();
    let b = prochain_changement(bateau, &maree, 0.0, periode, 60.0, CrossCause::Maree).unwrap().unwrap();
    let h = prochain_changement(Agent::Humanoide, &maree, 0.0, periode, 60.0, CrossCause::Maree).unwrap();
    println!("S573 véhicule {v:?} (3 726 s) ; bateau {b:?} (6 034,925 s) ; humanoïde {h:?}");
    assert!((v.delai_s - 3726.0).abs() < 1e-2 && v.trend == -1, "critère 1 : le véhicule");
    assert!((b.delai_s - 6034.92493402856).abs() < 1e-2 && b.trend == 1, "critère 1 : le bateau");
    assert_eq!(h, None, "critère 1 : l'humanoïde");
    let e = |d: f32, v: f32| echantillon(d, v).unwrap();
    assert!(praticable(vehicule, &e(0.6, 0.0)) && !praticable(vehicule, &e(0.601, 0.0)), "critère 2 : le gué");
    assert!(!praticable(bateau, &e(1.1, 0.0)) && praticable(bateau, &e(1.101, 0.0)), "critère 2 : le tirant");
    assert!(!praticable(Agent::Humanoide, &e(0.5, 2.0)), "critère 2 : HR = 1,25");
}

/// (3) Une commande de V invalide les prévisions d'une tuile : `Immediat`, `Aucune`, jusqu'à une prévision établie.
#[test]
fn a_command_upstream_invalidates_the_tile_forecasts_s573() {
    let periode = 12.42 * 3600.0;
    let mut tuile = TileDesc { frame: 0, tile_morton: 0, cadence: Cadence::Maree, subdivision: 0, sequence: 0 };
    let vide = Cellule { echantillon: echantillon(0.0, 0.0).unwrap(), t_next_cross: 0.0, cause: CrossCause::Aucune };
    let ev = CrossingEvent { cellule: 0, avant: (Profondeur::Negligeable, Danger::Faible), apres: (Profondeur::Negligeable, Danger::Faible) };
    let (mut cellules, mut evenements) = (vec![vide; 256], vec![ev; 256]);
    let prevision = Prevision { profondeur: &|i, _, t| plage_s572(i, t), t: 0.0, horizon_s: periode, pas_s: 60.0, cause: CrossCause::Maree };
    let echantillons = |i: usize, _: usize| (plage_s572(i, 0.0) as f32, 0.0);
    publier(&mut tuile, true, &echantillons, Some(&prevision), &mut cellules, &mut evenements).unwrap();
    let avec_maree = cellules.iter().filter(|c| c.cause == CrossCause::Maree).count();
    invalider(&mut tuile, &mut cellules);
    let apres = cellules.iter().filter(|c| c.cause == CrossCause::Aucune).count();
    publier(&mut tuile, false, &echantillons, None, &mut cellules, &mut evenements).unwrap();
    let sans_prevision = cellules.iter().filter(|c| c.cause == CrossCause::Aucune).count();
    publier(&mut tuile, false, &echantillons, Some(&prevision), &mut cellules, &mut evenements).unwrap();
    let retablies = cellules.iter().filter(|c| c.cause == CrossCause::Maree).count();
    println!("S573 invalidation : {avec_maree} prévisions Marée, puis {apres} Aucune ({:?}), {sans_prevision} sans prévision, {retablies} rétablies",
        tuile.cadence);
    assert_eq!((tuile.cadence, apres, sans_prevision), (Cadence::Immediat, 256, 256), "critère 3");
    assert_eq!(retablies, avec_maree, "critère 3 : rétablies");
}
