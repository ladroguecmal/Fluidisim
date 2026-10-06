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
