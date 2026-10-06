//! Réceptions de l'acteur (S514, liste 6.7).

use super::*;

/// **S514, critère 1 — les règles d'ADR-018.** Les seuils de profondeur et les classes de `HR`, de part et d'autre de chaque seuil ;
/// « 0,5 m à 2 m/s » emporte un adulte (`HR` = 1,25), « 0,5 m à 1,4 m/s » non (`HR` = 0,95).
#[test]
fn the_traversability_rules_of_adr018_s514() {
    let e = 1e-9;
    for (d, p) in [(0.15, Progress::Slowed), (0.50, Progress::Hampered), (1.00, Progress::Precarious), (1.30, Progress::Swimming)] {
        assert_eq!(progress(d), p, "{d} m");
        assert!(progress(d - e) != p, "{d} m moins un rien");
    }
    assert_eq!(progress(0.), Progress::Free);
    for (h, c) in [(0.75, Danger::Some), (1.25, Danger::Most), (2.50, Danger::All)] {
        assert_eq!(danger(h), c, "HR {h}");
        assert!(danger(h - e) != c, "HR {h} moins un rien");
    }
    assert_eq!(hazard_product(0.5, 2.), 1.25);
    assert!(adult_swept(0.5, 2.));
    assert!(!adult_swept(0.5, 1.4));
}
