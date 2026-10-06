//! S559 — la pression d'un nœud stratifié contre des formes fermées (ADR-241 D3). Les références sont écrites au plan, avant la mesure.

use super::*;
use crate::hydro_network::geometry::{Tetrahedron, VolumeShape};
use crate::hydro_network::liquids::{self, Liquid};

const EAU: Liquid = Liquid { density_kg_m3: 1000.0 };
const HUILE: Liquid = Liquid { density_kg_m3: 850.0 };

fn box_cells(lo: [i64; 3], hi: [i64; 3]) -> [Tetrahedron; 6] {
    let v: [[i64; 3]; 8] = std::array::from_fn(|bits|
        std::array::from_fn(|axis| if bits & (1 << axis) == 0 { lo[axis] } else { hi[axis] }));
    [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].map(|p| {
        Tetrahedron::new([v[0], v[1 << p[0]], v[(1 << p[0]) | (1 << p[1])], v[7]]).unwrap()
    })
}

/// La carène en V des essais de géométrie : section `|x| ≤ z`, `z` de 0 à 2 m, 1 m de long ; `V(h) = h²`.
fn hull_cells() -> [Tetrahedron; 3] {
    let v = [[0, -500_000, 0], [-2_000_000, -500_000, 2_000_000], [2_000_000, -500_000, 2_000_000],
             [0, 500_000, 0], [-2_000_000, 500_000, 2_000_000], [2_000_000, 500_000, 2_000_000]];
    [[0, 1, 2, 3], [1, 2, 3, 4], [2, 3, 4, 5]].map(|ids| Tetrahedron::new(ids.map(|i| v[i])).unwrap())
}

fn noeud(volume_ml: i64, capacity_ml: i64) -> HydroNode {
    HydroNode { volume_ml, capacity_ml, origin_um: [0; 3], shape: 0 }
}

fn proche(mesure: f64, attendu: f64) -> bool {
    ((mesure - attendu) / attendu).abs() < 1e-5
}

/// (1) La cuve droite ; (6) l'ordre de la table n'y fait rien.
#[test]
fn an_upright_tank_of_water_under_oil_has_the_layered_pressure_s559() {
    let cells = box_cells([-2_000_000, -500_000, 0], [2_000_000, 500_000, 2_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let n = noeud(6_000_000, 8_000_000);
    let g = [0.0, 0.0, -9.81];
    let fond = liquids::pressure_at(&n, &[4_000_000, 2_000_000], &[EAU, HUILE], &shapes, g, [0, 0, 0]).unwrap();
    let dans_l_huile = liquids::pressure_at(&n, &[4_000_000, 2_000_000], &[EAU, HUILE], &shapes, g, [0, 0, 1_200_000]).unwrap();
    let a_l_envers = liquids::pressure_at(&n, &[2_000_000, 4_000_000], &[HUILE, EAU], &shapes, g, [0, 0, 0]).unwrap();
    let dehors = liquids::pressure_at(&n, &[4_000_000, 2_000_000], &[EAU, HUILE], &shapes, g, [0, 0, 1_600_000]).unwrap();
    println!("S559 cuve droite : au fond {fond:.4} Pa (13 979,25), à 1,2 m {dans_l_huile:.4} Pa (2 501,55), table inversée {a_l_envers:.4} Pa, \
              au-dessus {dehors}");
    assert!(proche(fond, 9.81 * (1000.0 * 1.0 + 850.0 * 0.5)), "critère 1, au fond");
    assert!(proche(dans_l_huile, 9.81 * 850.0 * 0.3), "critère 1, dans l'huile");
    assert_eq!(dehors, 0.0);
    assert!(proche(a_l_envers, fond), "critère 6");
}

/// (2) La même cuve sous une gravité inclinée : chaque interface passe par la colonne centrale à `V/A`.
#[test]
fn a_tilted_tank_keeps_its_layers_perpendicular_to_gravity_s559() {
    let cells = box_cells([-2_000_000, -500_000, 0], [2_000_000, 500_000, 2_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let n = noeud(6_000_000, 8_000_000);
    let g = [1.0f32, 0.0, -9.759];
    let module = (1.0f64 + 9.759f64 * 9.759).sqrt();
    let up = [-1.0 / module, 0.0, 9.759 / module];
    // Les plans passent par (0, 0, 1) et (0, 0, 1,5) ; le coin bas en (−2, 0, 0).
    let cote = |p: [f64; 3]| p[0] * up[0] + p[2] * up[2];
    let (interface, surface, coin) = (cote([0.0, 0.0, 1.0]), cote([0.0, 0.0, 1.5]), cote([-2.0, 0.0, 0.0]));
    let attendu = module * (1000.0 * (interface - coin) + 850.0 * (surface - interface));
    let mesure = liquids::pressure_at(&n, &[4_000_000, 2_000_000], &[EAU, HUILE], &shapes, g, [-2_000_000, 0, 0]).unwrap();
    println!("S559 cuve inclinée : au coin bas {mesure:.4} Pa pour {attendu:.4} Pa ({:+.2e})", mesure / attendu - 1.0);
    assert!(proche(mesure, attendu), "critère 2");
}

/// (3) La carène en V : les interfaces à `√V`.
#[test]
fn a_v_hull_has_its_interfaces_at_the_square_root_of_volume_s559() {
    let cells = hull_cells();
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let n = noeud(1_690_000, formes[0].capacity_ml());
    let g = [0.0, 0.0, -9.81];
    let quille = liquids::pressure_at(&n, &[1_000_000, 690_000], &[EAU, HUILE], &shapes, g, [0, 0, 0]).unwrap();
    let a_1_2 = liquids::pressure_at(&n, &[1_000_000, 690_000], &[EAU, HUILE], &shapes, g, [0, 0, 1_200_000]).unwrap();
    println!("S559 carène en V : à la quille {quille:.4} Pa (12 311,55), à 1,2 m {a_1_2:.4} Pa (833,85)");
    assert!(proche(quille, 9.81 * (1000.0 + 850.0 * 0.3)), "critère 3, à la quille");
    assert!(proche(a_1_2, 9.81 * 850.0 * 0.1), "critère 3, dans l'huile");
}

/// (4) Un seul liquide : `ρ·|g|·(surface − z)`, la surface du pas présent ; l'ancienne table +Z aussi.
#[test]
fn one_liquid_gives_the_head_of_the_present_step_s559() {
    let cells = hull_cells();
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let n = noeud(1_440_000, formes[0].capacity_ml());
    let g = [0.0, 0.0, -9.81];
    let surface = shapes.surface_plane(&n, g).unwrap().offset_um;
    let mesure = liquids::pressure_at(&n, &[1_440_000], &[EAU], &shapes, g, [0, 0, 300_000]).unwrap();
    println!("S559 un liquide : {mesure:.4} Pa, surface à {surface:.3} µm");
    assert!(proche(mesure, 1000.0 * 9.81 * (surface - 300_000.0) * 1e-6), "critère 4");
    assert!(proche(mesure, 1000.0 * 9.81 * 0.9), "critère 4, la forme fermée (1,2 m)");
}

/// (5) Les refus.
#[test]
fn a_composition_that_does_not_add_up_is_refused_s559() {
    let cells = box_cells([-2_000_000, -500_000, 0], [2_000_000, 500_000, 2_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let n = noeud(6_000_000, 8_000_000);
    let g = [0.0, 0.0, -9.81];
    let p = |c: &[i64], l: &[Liquid]| liquids::pressure_at(&n, c, l, &shapes, g, [0; 3]).err();
    assert_eq!(p(&[4_000_000, 1_999_999], &[EAU, HUILE]), Some(Error::Capacity), "la somme");
    assert_eq!(p(&[4_000_000], &[EAU, HUILE]), Some(Error::Capacity), "les longueurs");
    assert_eq!(p(&[7_000_000, -1_000_000], &[EAU, HUILE]), Some(Error::Capacity), "un volume négatif");
    assert_eq!(p(&[4_000_000, 2_000_000], &[EAU, Liquid { density_kg_m3: 0.0 }]), Some(Error::Domain), "une densité nulle");
    assert_eq!(p(&[6_000_000, 0, 0, 0, 0, 0, 0, 0, 0], &[EAU; 9]), Some(Error::Capacity), "plus de huit liquides");
}
