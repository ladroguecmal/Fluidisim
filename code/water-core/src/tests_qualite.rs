//! S617 — le régulateur de qualité et les capacités dérivées. Références écrites au plan par son script (`s617_ref.py`) ; la trajectoire
//! de référence dans `tests_qualite_reference.txt`.

use super::*;

const BUDGET: f64 = 1.6;

fn regler(charges: &[f64]) -> Vec<f64> {
    let mut reg = Regulateur::nouveau(Reglages::ADR_012).unwrap();
    charges.iter().map(|c| {
        let conso = c * (0.4 + 1.6 * reg.q());
        reg.image(conso, BUDGET).unwrap()
    }).collect()
}

fn equilibre(charge: f64) -> f64 {
    ((BUDGET / charge - 0.4) / 1.6).clamp(0.0, 1.0)
}

fn inversions(qs: &[f64]) -> usize {
    let d: Vec<f64> = qs.windows(2).map(|w| w[1] - w[0]).filter(|x| x.abs() > 1e-3).collect();
    d.windows(2).filter(|w| (w[0] > 0.0) != (w[1] > 0.0)).count()
}

/// (1) la trajectoire, les équilibres ; (2) la descente ; (3) le pompage, le retour ; (4) la rampe, l'engagement ; (5) les capacités ; (6) refus.
#[test]
fn the_quality_regulator_follows_the_load_without_pumping_s617() {
    let trace: Vec<f64> = (0..360).map(|i| if (60..150).contains(&i) { 1.8 } else { 1.0 }).collect();
    let (qs, qf) = (regler(&trace), regler(&[3.0; 360]));
    let reference: Vec<f64> = include_str!("tests_qualite_reference.txt").lines().filter(|l| !l.starts_with('#')).map(|l| l.parse().unwrap()).collect();
    assert_eq!(reference.len(), 720);
    let pire = qs.iter().chain(&qf).zip(&reference).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);
    println!("S617 : trajectoire, pire écart à la référence {pire:e} ; q {} → {} ; à 5 s {} ; à 12 s {} ; faible {}", qs[59], qs[60], qs[149], qs[359], qf[359]);
    assert!(pire < 1e-12, "critère 1 : la trajectoire");
    assert!((qs[149] - equilibre(1.8)).abs() < 1e-6 && (qs[359] - equilibre(1.0)).abs() < 1e-6 && (qf[359] - equilibre(3.0)).abs() < 1e-6,
        "critère 1 : les équilibres");
    assert!(qs[60] < qs[59], "critère 2 : la descente à la première image");

    let retour = (150..360).find(|&i| qs[i] >= 0.99 * equilibre(1.0)).unwrap();
    let retour_s = (retour - 150 + 1) as f64 / 30.0;
    println!("S617 : {} inversions (faible : {}), retour à 99 % en {retour_s} s", inversions(&qs), inversions(&qf));
    assert!(inversions(&qs) <= 3 && inversions(&qf) == 0 && retour_s >= 1.0, "critère 3");

    let mut derniere_descente = None;
    for i in 1..qs.len() {
        let d = qs[i] - qs[i - 1];
        assert!(d <= 1.0 / 30.0 + 1e-15, "critère 4 : la rampe");
        if d < 0.0 {
            derniere_descente = Some(i);
        } else if d > 0.0 {
            assert!(derniere_descente.is_none_or(|j| i - j >= 30), "critère 4 : l'engagement");
        }
    }

    let profil = Profil { cpu_sim_ms: 2.0, memoire_blocs_octets: 384 * 1024 * 1024 };
    let mesure = Couts { ms_par_impact: 109.0 / 4096.0, octets_par_bloc: 8 * 8 * 8 * 16 };
    let lent = Couts { ms_par_impact: 3.0 * 109.0 / 4096.0, ..mesure };
    assert_eq!(Capacites::depuis(profil, mesure).unwrap(), Capacites { paquets_w: 75, blocs: 49152 }, "critère 5");
    assert_eq!(Capacites::depuis(profil, lent).unwrap().paquets_w, 25, "critère 5 : le matériel faible");

    assert!(Regulateur::nouveau(Reglages { tau_s: 0.0, ..Reglages::ADR_012 }).is_err(), "critère 6 : τ");
    assert!(Regulateur::nouveau(Reglages { dt_s: 0.0, ..Reglages::ADR_012 }).is_err(), "critère 6 : pas");
    assert_eq!(Regulateur::nouveau(Reglages::ADR_012).unwrap().image(1.0, 0.0), Err(Refus), "critère 6 : budget");
    assert_eq!(Capacites::depuis(profil, Couts { ms_par_impact: 0.0, ..mesure }), Err(Refus), "critère 6 : coût");
}
