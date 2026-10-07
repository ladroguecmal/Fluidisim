//! S592 — un canal de dix biefs (la loi de Manning) trouve sa hauteur normale. Références écrites au plan par son script.

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

const BIEF: Flow = Flow::Manning { width_mm: 5_000, length_mm: 100_000, roughness_e6: 15_000, outlet_slope_e6: 1_000 };

/// Le canal : dix biefs de 100 × 5 m, le lit descendant de 0,1 m de bief en bief ; les arêtes au milieu de chaque marche ; la pluie
/// (5 m³/s) sur le premier ; la sortie en régime uniforme.
fn canal() -> ([HydroNode; 10], [Opening; 11]) {
    let nodes: [HydroNode; 10] = std::array::from_fn(|k| HydroNode {
        volume_ml: 0, capacity_ml: 2_000_000_000, origin_um: [k as i64 * 100_000_000, 0, -(k as i64) * 100_000], shape: 0 });
    let mut edges: [Opening; 11] = std::array::from_fn(|k| Opening {
        from: k as u16, to: Some(k as u16 + 1), flow: BIEF,
        position_um: [(k as i64 + 1) * 100_000_000, 2_500_000, -(k as i64 + 1) * 100_000 + 50_000], ..Default::default() });
    // La sortie : vers dehors, le seuil au fond du dernier bief.
    edges[9] = Opening { from: 9, to: None, flow: BIEF, position_um: [1_000_000_000, 2_500_000, -900_000], ..Default::default() };
    edges[10] = Opening { from: 0, to: Some(0), flow: Flow::Rain { catchment_mm2: 1_800_000_000_000 }, ..Default::default() };
    (nodes, edges)
}

/// (1) La hauteur normale ; (2) le débit de chaque arête ; (3) le bilan ; (4) l'instantané.
#[test]
fn a_canal_of_ten_reaches_finds_its_normal_depth_s592() {
    let cells = box_cells([0, 0, 0], [100_000_000, 5_000_000, 4_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let (auteur_n, auteur_e) = canal();
    let (mut nodes, mut edges) = (auteur_n, auteur_e);
    let mut scratch = [0i64; 11];
    let meteo = Meteo { pluie_mm_h: 10.0 };
    let (mut recu, mut sorti) = (0i64, 0i64);
    let mut moyenne = [0f64; 10];
    for pas in 0..10_800 {
        step_meteo(&mut nodes, &mut edges, &shapes, [0.0, 0.0, -9.81], meteo, SimTime(1_000_000), &mut scratch).unwrap();
        recu += scratch[10];
        sorti += scratch[9];
        if pas >= 10_740 {
            for (m, s) in moyenne.iter_mut().zip(&scratch[..10]) {
                *m += *s as f64 * 1e-6 / 60.0;
            }
        }
    }
    let profondeurs: Vec<f64> = nodes.iter().map(|n| n.volume_ml as f64 * 1e-6 / 500.0).collect();
    println!("S592 canal : profondeurs {profondeurs:.5?} (y_n = 0,706106) ; débits sur la dernière minute {moyenne:.5?} m³/s");
    for (k, y) in profondeurs.iter().enumerate() {
        assert!((y - 0.7061060015424827).abs() < 1e-3, "critère 1 : le bief {k}, {y}");
    }
    for (k, q) in moyenne.iter().enumerate() {
        assert!((q / 5.0 - 1.0).abs() < 1e-3, "critère 2 : l'arête {k}, {q}");
    }
    assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>(), recu - sorti, "critère 3 : le bilan");
    // (4) L'instantané : la loi acceptée, la restauration au bit, la suite identique.
    use super::snapshot::{Baseline, Context};
    let base = Baseline::new(7, 1, &auteur_n, &auteur_e, &shapes).unwrap();
    let contexte = Context { time: SimTime(10_800_000_000), dt: SimTime(1_000_000), g_eff: [0.0, 0.0, -9.81] };
    let mut tampon = vec![0u8; base.snapshot_len(&nodes, &edges).unwrap()];
    base.snapshot_into(&nodes, &edges, contexte, &mut tampon).unwrap();
    let (mut rn, mut re) = (auteur_n, auteur_e);
    assert_eq!(base.restore_into(&tampon, &mut rn, &mut re).unwrap(), contexte);
    assert_eq!(rn, nodes, "critère 4 : les nœuds au bit");
    let (mut a, mut ea, mut b, mut eb) = (nodes, edges, rn, re);
    for _ in 0..100 {
        step_meteo(&mut a, &mut ea, &shapes, [0.0, 0.0, -9.81], meteo, SimTime(1_000_000), &mut scratch).unwrap();
        step_meteo(&mut b, &mut eb, &shapes, [0.0, 0.0, -9.81], meteo, SimTime(1_000_000), &mut scratch).unwrap();
    }
    assert_eq!(a, b, "critère 4 : la suite au bit");
}

// --- S593 — la ligne d'eau d'une rivière et son remous. Références écrites au plan par son script.

/// Les profondeurs de l'état stationnaire exact du découpage (référence 1 du plan), de l'amont à l'aval.
const Y_DISC: [f64; 20] = [0.76181, 0.78607, 0.81805, 0.85849, 0.9075, 0.96464, 1.02908, 1.09975, 1.17562, 1.25572, 1.33922, 1.42547,
    1.51393, 1.60415, 1.69582, 1.78867, 1.88248, 1.97708, 2.07236, 2.16819];

/// (1) La ligne d'eau à 1 mm du découpage exact (référence arrondie au 10⁻⁵ m) ; (2) à 1,5 mm de l'onde diffusive (l'écart du découpage,
/// 0,75 mm, calculé au plan) ; (3) la vitesse moyenne de chaque bief, le bilan.
#[test]
fn a_river_backs_up_behind_its_weir_s593() {
    let cells = box_cells([0, 0, 0], [100_000_000, 5_000_000, 4_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let mut nodes: [HydroNode; 20] = std::array::from_fn(|k| HydroNode {
        volume_ml: 0, capacity_ml: 2_000_000_000, origin_um: [k as i64 * 100_000_000, 0, -(k as i64) * 100_000], shape: 0 });
    let mut edges: [Opening; 21] = std::array::from_fn(|k| Opening {
        from: k as u16, to: Some(k as u16 + 1), flow: BIEF,
        position_um: [(k as i64 + 1) * 100_000_000, 2_500_000, -(k as i64 + 1) * 100_000 + 50_000], ..Default::default() });
    // Le seuil aval : un déversoir de 5 m, sa crête à 1,5 m au-dessus du fond du dernier bief.
    edges[19] = Opening { from: 19, to: None, flow: Flow::Weir { width_mm: 5_000 }, position_um: [2_000_000_000, 2_500_000, -1_900_000 + 1_500_000],
        discharge: 0.62, ..Default::default() };
    edges[20] = Opening { from: 0, to: Some(0), flow: Flow::Rain { catchment_mm2: 1_800_000_000_000 }, ..Default::default() };
    let mut scratch = [0i64; 21];
    let meteo = Meteo { pluie_mm_h: 10.0 };
    let (mut recu, mut sorti) = (0i64, 0i64);
    for _ in 0..216_000 {
        step_meteo(&mut nodes, &mut edges, &shapes, [0.0, 0.0, -9.81], meteo, SimTime(50_000), &mut scratch).unwrap();
        recu += scratch[20];
        sorti += scratch[19];
    }
    let y: Vec<f64> = nodes.iter().map(|n| n.volume_ml as f64 * 1e-6 / 500.0).collect();
    let pire = y.iter().zip(&Y_DISC).map(|(a, b)| (a - b).abs()).fold(0f64, f64::max);
    let vitesses: Vec<f64> = y.iter().map(|y| 5.0 / (5.0 * y)).collect();
    println!("S593 rivière : profondeurs {y:.5?} ; au pire {:.3} mm du découpage exact ; vitesses {vitesses:.4?} m/s", pire * 1e3);
    assert!(pire < 1e-3, "critère 1");
    assert!(pire < 1e-3 + 0.75e-3, "critère 2 : la ligne diffusive (l'écart du découpage, 0,75 mm, plus le critère 1)");
    for (v, yr) in vitesses.iter().zip(&Y_DISC) {
        assert!((v / (1.0 / yr) - 1.0).abs() < 1e-3, "critère 3 : la vitesse moyenne");
    }
    assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>(), recu - sorti, "critère 3 : le bilan");
}
