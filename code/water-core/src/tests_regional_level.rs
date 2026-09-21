//! Essais du niveau régional local — S317, ordre D, ADR-185.

use super::*;
use crate::delta3d::Ledger3;

/// Une ligne de contrôle verticale en `x = 42`, sur 0,25 m de largeur, sortant vers `+x`, et une
/// région de 20 m derrière elle.
fn spec() -> RegionSpec {
    RegionSpec {
        frame: FrameId(0),
        cell: 0,
        line: [[42.0, 0.0], [42.0, 0.25]],
        outward: [1.0, 0.0],
        depth_m: 20.0,
    }
}

/// **Le niveau est le volume reçu sur l'aire**, et l'aire est celle qu'on a déclarée — pas
/// l'océan. 0,25 m × 20 m = 5 m².
#[test]
fn the_level_is_the_received_volume_over_the_declared_area() {
    let mut r = RegionalLevel::new(spec()).unwrap();
    assert_eq!(r.area(), 5.0);
    let recu = r.receive(1e-4).unwrap();
    assert_eq!(recu.volume(), 1e-4);
    assert_eq!(r.volume(), 1e-4);
    assert!((r.level_m() - 2e-5).abs() < 1e-18);
    assert_eq!(r.receipts(), 1);
    assert_eq!(r.boundary_out(), 0.0);
    drop(recu);
}

/// **Rien ne se restitue sans receveur** : le registre ne baisse que sur présentation d'un reçu,
/// et après restitution pas à pas l'attente vaut **exactement** zéro — les deux sommes sont les
/// mêmes additions dans le même ordre. La représentation est fermée ; le monde, lui, n'est pas
/// revendiqué (ADR-185 §3).
#[test]
fn restitution_closes_the_representation_and_never_claims_the_world() {
    let mut r = RegionalLevel::new(spec()).unwrap();
    let mut l = Ledger3::default();
    let mut volume_interieur = 0f64;
    for i in 0..10_000 {
        // Un flux signé, comme à une ligne réelle : de l'eau sort, puis en rentre.
        let q = 1e-7 * ((i as f64) * 0.013).sin() + 2e-9;
        volume_interieur -= q;
        l.account(q, 0.0, 1e-16).unwrap();
        l.account_restitution(r.receive(q).unwrap()).unwrap();
    }
    assert_eq!(l.pending(), 0.0);
    assert_eq!(l.created(), 0.0);
    assert!(l.representation_closed());
    assert!(!l.global_conservation_claimable());
    assert_eq!(l.restituted(), r.volume());
    // δ + région : ce que l'intérieur a perdu, la région l'a pris.
    assert!((volume_interieur + r.volume()).abs() < 1e-18);
}

/// Sans reçu, l'attente reste le sortant : la région peut avoir pris de l'eau, le registre ne le
/// sait pas tant qu'on ne le lui prouve pas — et ne le suppose pas.
#[test]
fn without_a_receipt_the_ledger_does_not_move() {
    let mut r = RegionalLevel::new(spec()).unwrap();
    let mut l = Ledger3::default();
    l.account(3e-5, 0.0, 0.0).unwrap();
    let recu = r.receive(3e-5).unwrap();
    assert_eq!(l.pending(), 3e-5);
    assert!(!l.representation_closed());
    l.account_restitution(recu).unwrap();
    assert_eq!(l.pending(), 0.0);
}

/// Une région qui rend plus qu'il n'est sorti **crée** de l'eau, et le registre le dit.
#[test]
fn restituting_more_than_went_out_is_creation() {
    let mut r = RegionalLevel::new(spec()).unwrap();
    let mut l = Ledger3::default();
    l.account(1e-5, 0.0, 0.0).unwrap();
    l.account_restitution(r.receive(3e-5).unwrap()).unwrap();
    assert!((l.created() - 2e-5).abs() < 1e-20);
    assert!(!l.representation_closed());
}

/// **Le signe est porté** : une région qui rend de l'eau à l'intérieur — une onde longue qui entre
/// par la ligne — voit son niveau baisser sous le repos.
#[test]
fn a_region_can_give_water_back() {
    let mut r = RegionalLevel::new(spec()).unwrap();
    let recu = r.receive(-1.37e-4).unwrap();
    assert!(r.level_m() < 0.0);
    assert_eq!(recu.volume(), -1.37e-4);
    drop(recu);
}

/// Le niveau n'existe que dans la région : même repère, même cellule, du côté sortant, sur la
/// profondeur déclarée.
#[test]
fn the_level_lives_only_inside_the_region() {
    let mut r = RegionalLevel::new(spec()).unwrap();
    let _ = r.receive(5e-5).unwrap();
    let dedans = r.level_m();
    assert_eq!(r.sample(FrameId(0), 0, [50.0, 0.1]), dedans);
    assert_eq!(r.sample(FrameId(0), 0, [41.0, 0.1]), 0.0); // derrière la ligne
    assert_eq!(r.sample(FrameId(0), 0, [62.5, 0.1]), 0.0); // au-delà de la profondeur
    assert_eq!(r.sample(FrameId(0), 0, [50.0, 0.3]), 0.0); // hors du segment
    assert_eq!(r.sample(FrameId(1), 0, [50.0, 0.1]), 0.0); // autre repère
    assert_eq!(r.sample(FrameId(0), 7, [50.0, 0.1]), 0.0); // autre cellule
    assert_eq!(r.sample(FrameId(0), 0, [f32::NAN, 0.1]), 0.0);
}

/// Les refus, chacun sous son nom.
#[test]
fn every_refusal_has_its_own_name() {
    let s = spec();
    let refus = |f: &dyn Fn(&mut RegionSpec)| {
        let mut t = s;
        f(&mut t);
        RegionalLevel::new(t).unwrap_err()
    };
    assert_eq!(refus(&|t| t.line = [[42.0, 0.0], [42.0, 0.0]]), RegionError::Line);
    assert_eq!(refus(&|t| t.line[0][0] = f32::NAN), RegionError::Line);
    assert_eq!(refus(&|t| t.line = [[5000.0, 0.0], [5000.0, 1.0]]), RegionError::Domain);
    assert_eq!(refus(&|t| t.outward = [0.9, 0.0]), RegionError::Outward);
    assert_eq!(refus(&|t| t.outward = [0.0, 1.0]), RegionError::Outward); // le long de la ligne
    assert_eq!(refus(&|t| t.outward = [f32::INFINITY, 0.0]), RegionError::Outward);
    assert_eq!(refus(&|t| t.depth_m = 0.0), RegionError::Depth);
    assert_eq!(refus(&|t| t.depth_m = -1.0), RegionError::Depth);
    assert_eq!(refus(&|t| t.depth_m = f32::NAN), RegionError::Depth);
    assert_eq!(refus(&|t| t.depth_m = 4096.0), RegionError::Depth);
    // Une région qui déborderait de sa cellule : l'océan ne s'obtient pas en élargissant.
    assert_eq!(refus(&|t| t.depth_m = 4060.0), RegionError::Domain);
    let mut r = RegionalLevel::new(s).unwrap();
    assert_eq!(r.receive(f64::NAN).unwrap_err(), RegionError::NonFinite);
    assert_eq!(r.volume(), 0.0);
    assert_eq!(r.receipts(), 0);
}
