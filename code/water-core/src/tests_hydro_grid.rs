//! S607 — la grille d'adressage `HydroGrid`. Références écrites au plan par son script (une implémentation Python indépendante).

use super::*;
use crate::types::{WorldPos, WORLD_UNITS_PER_METRE};

fn interets(t: usize) -> Vec<([f64; 3], f64)> {
    let t = t as f64;
    vec![([100.0 + 30.0 * t, 50.0, -10.0], 100.0), ([-5000.0, 2000.0, 0.0], 200.0), ([40_000.0, -300.0 + 25.0 * t, 5.0], 64.0)]
}

/// (1) les clés, l'aller-retour ; (2) parents et enfants ; (3) voisins ; (4) l'échange ; (5) le rebasage ; (6) refus.
#[test]
fn the_hydro_grid_addresses_cells_and_replays_active_zones_on_the_client_s607() {
    let refs: [(u8, [f64; 3], u64); 5] = [
        (0, [0.0, 0.0, 0.0], 0xe00000000000000),
        (0, [100.0, -50.0, 7.0], 0xa92492492492493),
        (1, [-5000.0, 2000.0, 0.0], 0x624924924905a),
        (2, [40_000.0, -300.0, 5.0], 0x2a492492693),
        (0, [-33_554_432.0, 33_554_431.0, -1.0], 0x5b6db6db6db6db6),
    ];
    for (n, p, k) in refs {
        assert_eq!(CellId::cellule(7, n, p).unwrap().morton, k, "critère 1 : la clé de {p:?}");
    }
    let mut graine = 0x2545_f491_4f6c_dd1du64;
    let mut hasard = || {
        graine = graine.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        ((graine >> 11) as f64 / (1u64 << 53) as f64 - 0.5) * 6.0e7
    };
    for _ in 0..100_000 {
        let p = [hasard(), hasard(), hasard()];
        let c0 = CellId::cellule(3, 0, p).unwrap();
        let u = c0.coordonnees();
        assert_eq!(CellId::depuis_coordonnees(3, 0, u), c0, "critère 1 : l'aller-retour");
        for a in 0..3 {
            assert_eq!(u[a] as i64, (p[a] / 64.0).floor() as i64 + (1 << 19), "critère 1 : la coordonnée");
        }
        let c1 = CellId::cellule(3, 1, p).unwrap();
        assert_eq!(c0.parent(), Some(c1), "critère 2 : parent 0 → 1");
        assert_eq!(c1.parent(), Some(CellId::cellule(3, 2, p).unwrap()), "critère 2 : parent 1 → 2");
    }
    let p1 = CellId::cellule(7, 1, [100.0, -50.0, 7.0]).unwrap();
    let enfants: Vec<CellId> = p1.enfants().collect();
    assert_eq!(enfants.len(), 512);
    assert!(enfants.iter().enumerate().all(|(i, e)| e.morton == p1.morton * 512 + i as u64 && e.parent() == Some(p1)), "critère 2 : enfants");

    let c = CellId::cellule(7, 0, [100.0, -50.0, 7.0]).unwrap();
    let v = c.voisins();
    let u = c.coordonnees();
    let mut tri = v.clone();
    tri.sort_unstable();
    tri.dedup();
    assert!(v.len() == 26 && tri.len() == 26, "critère 3 : 26 voisins distincts");
    assert!(v.iter().all(|w| {
        let q = w.coordonnees();
        (0..3).map(|a| (q[a] as i64 - u[a] as i64).abs()).max() == Some(1)
    }), "critère 3 : Chebyshev 1");

    let (tailles, ajouts, retraits, octets) = ([305, 309, 308, 307, 306, 308, 305, 308, 306, 308], [305, 10, 11, 7, 7, 13, 8, 14, 6, 11],
        [0, 6, 12, 8, 8, 11, 11, 11, 8, 9], [3668, 200, 284, 188, 188, 296, 236, 308, 176, 248]);
    let (mut serveur, mut client) = (ZonesActives::default(), ZonesActives::default());
    for t in 0..10 {
        let nouveau = ZonesActives::depuis_interets(7, &interets(t)).unwrap();
        let e = nouveau.ecart(&serveur);
        let message = e.encoder();
        println!("S607 : pas {t} : {} cellules, +{} −{}, {} octets", nouveau.cellules.len(), e.ajouts.len(), e.retraits.len(), message.len());
        assert_eq!((nouveau.cellules.len(), e.ajouts.len(), e.retraits.len(), message.len()), (tailles[t], ajouts[t], retraits[t], octets[t]),
            "critère 4 : le pas {t}");
        client.appliquer(&Echange::decoder(&message).unwrap());
        serveur = nouveau;
        assert_eq!(client, serveur, "critère 4 : le client au bit");
    }
    assert_eq!((serveur.ancetres(1).len(), serveur.ancetres(2).len()), (14, 8), "critère 4 : les ancêtres");

    assert_eq!(TAILLES_M[2], 4096.0, "critère 5");
    let ancre = WorldPos::from_units(0, 0, 0);
    let juste = WorldPos::from_units(4096 * WORLD_UNITS_PER_METRE - 1, 0, 0);
    assert!(juste.to_local(ancre).is_some() && WorldPos::from_units(4096 * WORLD_UNITS_PER_METRE, 0, 0).to_local(ancre).is_none(),
        "critère 5 : le rebasage à 4 096 m, la taille d'une cellule de niveau 2");

    assert_eq!(CellId::cellule(7, 3, [0.0; 3]), Err(Refus), "critère 6 : niveau");
    assert_eq!(CellId::cellule(7, 0, [3.4e7, 0.0, 0.0]), Err(Refus), "critère 6 : hors de portée");
    assert_eq!(CellId::cellule(7, 0, [f64::NAN, 0.0, 0.0]), Err(Refus), "critère 6 : non finie");
    let message = Echange { ajouts: vec![c], retraits: vec![] }.encoder();
    assert_eq!(Echange::decoder(&message[..message.len() - 1]), Err(Refus), "critère 6 : message tronqué");
}
