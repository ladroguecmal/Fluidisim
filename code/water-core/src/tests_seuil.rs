//! S564 — le seuil adaptatif en hauteur de surface. Les références sont écrites au plan, avant la mesure.

use super::*;
use crate::hydro_network::geometry::{Tetrahedron, VolumeShape};
use crate::hydro_network::seuil::{changement_significatif, ecart_hauteur_um};
use crate::SimTime;

fn box_cells(lo: [i64; 3], hi: [i64; 3]) -> [Tetrahedron; 6] {
    let v: [[i64; 3]; 8] = std::array::from_fn(|bits|
        std::array::from_fn(|axis| if bits & (1 << axis) == 0 { lo[axis] } else { hi[axis] }));
    [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].map(|p| {
        Tetrahedron::new([v[0], v[1 << p[0]], v[(1 << p[0]) | (1 << p[1])], v[7]]).unwrap()
    })
}

fn hull_cells() -> [Tetrahedron; 3] {
    let v = [[0, -500_000, 0], [-2_000_000, -500_000, 2_000_000], [2_000_000, -500_000, 2_000_000],
             [0, 500_000, 0], [-2_000_000, 500_000, 2_000_000], [2_000_000, 500_000, 2_000_000]];
    [[0, 1, 2, 3], [1, 2, 3, 4], [2, 3, 4, 5]].map(|ids| Tetrahedron::new(ids.map(|i| v[i])).unwrap())
}

const SEUIL: f64 = 500.0;
const BAS: [f32; 3] = [0.0, 0.0, -9.81];

/// (1) Un litre dans un bidon, dans une piscine, dans une carène ; sous une gravité inclinée.
#[test]
fn a_litre_matters_in_a_can_and_not_in_a_pool_s564() {
    let bidon_t = box_cells([0, 0, 0], [200_000, 100_000, 400_000]);
    let piscine_t = box_cells([0, 0, 0], [10_000_000, 5_000_000, 2_000_000]);
    let carene_t = hull_cells();
    let formes = [VolumeShape::new(&bidon_t).unwrap(), VolumeShape::new(&piscine_t).unwrap(), VolumeShape::new(&carene_t).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let noeud = |forme: u16, volume_ml: i64| HydroNode { volume_ml, capacity_ml: formes[forme as usize].capacity_ml(), origin_um: [0; 3],
        shape: forme };
    let incline = [1.0f32, 0.0, -9.759];
    let cas: [(&str, HydroNode, i64, [f32; 3], f64, bool); 6] = [
        ("bidon +1 L", noeud(0, 5_000), 4_000, BAS, 50_000.0, true),
        ("piscine +1 L", noeud(1, 50_001_000), 50_000_000, BAS, 20.0, false),
        ("piscine +25 L", noeud(1, 50_025_000), 50_000_000, BAS, 500.0, true),
        ("carène +0,99 L", noeud(2, 1_000_990), 1_000_000, BAS, 494.8775, false),
        ("carène +1,01 L", noeud(2, 1_001_010), 1_000_000, BAS, 504.8726, true),
        ("piscine inclinée +25 L", noeud(1, 50_025_000), 50_000_000, incline, 497.3955, false),
    ];
    for (nom, n, publie, g, attendu, verdict) in cas {
        let ecart = ecart_hauteur_um(&n, publie, &shapes, g).unwrap();
        let significatif = changement_significatif(&n, publie, &shapes, g, SEUIL).unwrap();
        println!("S564 {nom} : {ecart:.4} µm (référence {attendu}) → {}", if significatif { "significatif" } else { "ignoré" });
        assert!((ecart - attendu).abs() < 1.0, "critère 1 : {nom}");
        assert_eq!(significatif, verdict, "critère 1 : le verdict, {nom}");
    }
}

/// (2) Une fuite de 1 L par pas dans la piscine : publiée exactement tous les 25 pas ; la masse jamais touchée ; (3) les refus.
#[test]
fn a_slow_leak_accumulates_until_it_is_published_s564() {
    let piscine_t = box_cells([0, 0, 0], [10_000_000, 5_000_000, 2_000_000]);
    let formes = [VolumeShape::new(&piscine_t).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let mut nodes = [HydroNode { volume_ml: 50_000_000, capacity_ml: formes[0].capacity_ml(), origin_um: [0; 3], shape: 0 }];
    // 1 L par pas de 100 ms : 10 L/s, une évaporation d'auteur sur 1 m² à 10⁷ nm/s.
    let mut edges = [Opening { from: 0, to: None, flow: Flow::Evaporation { area_mm2: 1_000_000, rate_nm_s: 10_000_000 },
        ..Default::default() }];
    let mut scratch = [0i64; 1];
    let mut publie = nodes[0].volume_ml;
    let mut publications = Vec::new();
    for pas in 1..=100 {
        step(&mut nodes, &mut edges, &shapes, BAS, SimTime(100_000), &mut scratch).unwrap();
        assert_eq!(nodes[0].volume_ml, 50_000_000 - 1_000 * pas, "critère 2 : la masse, au millilitre");
        if changement_significatif(&nodes[0], publie, &shapes, BAS, SEUIL).unwrap() {
            publications.push(pas);
            publie = nodes[0].volume_ml;
        }
    }
    println!("S564 fuite : publiée aux pas {publications:?}");
    assert_eq!(publications, vec![25, 50, 75, 100], "critère 2");
    assert_eq!(ecart_hauteur_um(&nodes[0], formes[0].capacity_ml() + 1, &shapes, BAS).err(), Some(Error::Capacity), "critère 3");
    assert_eq!(changement_significatif(&nodes[0], publie, &shapes, BAS, -1.0).err(), Some(Error::Domain), "critère 3");
}
