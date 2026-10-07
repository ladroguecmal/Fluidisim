//! S608 — les niveaux d'activité des cellules. Références écrites au plan par son script (couvertures exactes en rationnels).

use super::*;

fn domaine(frame: u32, origine: [f64; 3], dx: f64, n: [i32; 3]) -> DomaineMeta {
    let mut blocs = Vec::new();
    for i in 0..n[0] {
        for j in 0..n[1] {
            for k in 0..n[2] {
                blocs.push([i, j, k]);
            }
        }
    }
    DomaineMeta { frame, origine, dx, blocs }
}

fn cellule(i: i64, j: i64, k: i64) -> CellId {
    CellId::cellule(1, 0, [(i as f64 + 0.5) * 64.0, (j as f64 + 0.5) * 64.0, (k as f64 + 0.5) * 64.0]).unwrap()
}

/// (1) les cellules de A, quatre fractions ; (2) les volumes ; (3) les niveaux ; (4) refus.
#[test]
fn cells_take_five_activity_levels_from_unaligned_domains_s608() {
    let a = domaine(1, [3.3, -1.7, -65.7], 0.5, [40, 40, 17]);
    let b = domaine(1, [70.1, 10.2, -1.3], 0.05, [50, 50, 5]);
    let plein = 64.0f64.powi(3);
    let cov_a = couverture(&a).unwrap();
    let pleines: Vec<CellId> = cov_a.iter().filter(|(_, &v)| v >= (1.0 - 1e-9) * plein).map(|(&c, _)| c).collect();
    println!("S608 : A touche {} cellules, {} pleines", cov_a.len(), pleines.len());
    assert_eq!(cov_a.len(), 36, "critère 1 : cellules touchées");
    let mut attendues = vec![cellule(1, 0, -1), cellule(1, 1, -1)];
    attendues.sort_unstable();
    assert_eq!(pleines, attendues, "critère 1 : cellules pleines");
    for ((i, j, k), r) in [((0, 0, -2), 0.02519287109375), ((1, 0, -1), 1.0), ((2, 2, -1), 0.26113037109375), ((1, 0, 0), 0.0359375)] {
        let f = cov_a[&cellule(i, j, k)] / plein;
        println!("S608 : fraction de ({i}, {j}, {k}) : {f}");
        assert!((f - r).abs() < 1e-12, "critère 1 : fraction de ({i}, {j}, {k})");
    }
    for (d, nom) in [(&a, "A"), (&b, "B")] {
        let total: f64 = couverture(d).unwrap().values().sum();
        let attendu = d.blocs.len() as f64 * (8.0 * d.dx).powi(3);
        println!("S608 : volume couvert par {nom} : {total} m³ (blocs : {attendu})");
        assert!(((total - attendu) / attendu).abs() < 1e-9, "critère 2 : le volume de {nom}");
    }

    let w: Vec<CellId> = (-3..6).map(|i| cellule(i, 0, -1)).collect();
    let n = niveaux(1, &w, &[a.clone(), b.clone()], 0.10).unwrap();
    let mut compte = [0usize; 5];
    for v in n.values() {
        compte[*v as usize] += 1;
    }
    println!("S608 : niveaux (inactive, simplifiée, partielle, active, détail) : {compte:?}");
    assert_eq!(compte, [0, 6, 33, 1, 2], "critère 3 : le compte");
    assert_eq!((n[&cellule(1, 0, -1)], n[&cellule(1, 0, 0)]), (Activite::Detail, Activite::Detail), "critère 3 : les cellules de B");

    assert_eq!(couverture(&DomaineMeta { dx: 0.3, ..a.clone() }), Err(Refus), "critère 4 : dx");
    let ailleurs = DomaineMeta { frame: 2, ..a };
    assert!(niveaux(1, &[], &[ailleurs], 0.10).unwrap().is_empty(), "critère 4 : un autre référentiel");
}
