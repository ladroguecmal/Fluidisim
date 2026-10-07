//! S590 — le niveau moyen d'un lac par son bilan d'eau (liste 2.3). Références écrites au plan par son script.

use super::*;
use crate::hydro_network::geometry::{Tetrahedron, VolumeShape};
use crate::SimTime;

fn box_cells(lo: [i64; 3], hi: [i64; 3]) -> [Tetrahedron; 6] {
    let v: [[i64; 3]; 8] = std::array::from_fn(|bits|
        std::array::from_fn(|axis| if bits & (1 << axis) == 0 { lo[axis] } else { hi[axis] }));
    [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].map(|p| {
        Tetrahedron::new([v[0], v[1 << p[0]], v[(1 << p[0]) | (1 << p[1])], v[7]]).unwrap()
    })
}

/// (1) Le niveau d'équilibre ; (2) le bilan au millilitre ; (3) le niveau à 49,8 h.
#[test]
fn a_lake_settles_at_the_level_its_outlet_allows_s590() {
    let cells = box_cells([0, 0, 0], [1_000_000_000, 1_000_000_000, 12_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let mut nodes = [HydroNode { volume_ml: 10_000_000_000_000, capacity_ml: formes[0].capacity_ml(), origin_um: [0; 3], shape: 0 }];
    let mut edges = [
        Opening { from: 0, to: Some(0), flow: Flow::Rain { catchment_mm2: 3_600_000_000_000 }, ..Default::default() },
        // L'évaporation, entière en nm/s : 58 (le plan avait écrit 57,87 ; la référence est recalculée ci-dessous, ADR-237 D1).
        Opening { from: 0, to: None, flow: Flow::Evaporation { area_mm2: 1_000_000_000_000, rate_nm_s: 58 }, ..Default::default() },
        Opening { from: 0, to: None, flow: Flow::Weir { width_mm: 20_000 }, position_um: [1_000_000_000, 500_000_000, 10_000_000],
            discharge: 0.62, ..Default::default() },
    ];
    let mut scratch = [0i64; 3];
    let meteo = Meteo { pluie_mm_h: 10.0 };
    let (q_in, q_e) = (10.0 / 1000.0 / 3600.0 * 3.6e6, 58e-9 * 1e6);
    let kw = 2.0 / 3.0 * 0.62 * 20.0 * (2.0f64 * 9.81).sqrt();
    let h_eq = ((q_in - q_e) / kw).powf(2.0 / 3.0);
    let v0 = nodes[0].volume_ml;
    let (mut recu, mut sorti) = (0i64, 0i64);
    let mut a_49h8 = 0.0;
    let niveau = |n: &HydroNode| shapes.surface_plane(n, [0.0, 0.0, -9.81]).unwrap().offset_um * 1e-6;
    for pas in 1..=25_920 {
        step_meteo(&mut nodes, &mut edges, &shapes, [0.0, 0.0, -9.81], meteo, SimTime(10_000_000), &mut scratch).unwrap();
        recu += scratch[0];
        sorti += scratch[1] + scratch[2];
        if pas == 17_921 {
            a_49h8 = niveau(&nodes[0]);
        }
    }
    let fin = niveau(&nodes[0]);
    println!("S590 lac : H d'équilibre {h_eq:.6} m (le plan, à 57,87 nm/s : 0,419308) ; à 49,8 h {:.6} m ; à 72 h {:.6} m ; reçu {recu} ml, sorti {sorti} ml",
        a_49h8 - 10.0, fin - 10.0);
    assert!((fin - (10.0 + h_eq)).abs() < 1e-3, "critère 1");
    assert_eq!(nodes[0].volume_ml - v0, recu - sorti, "critère 2 : le bilan au millilitre");
    assert!((a_49h8 - (10.0 + h_eq)).abs() < 2e-3, "critère 3");
}
