//! S559 — la pression d'un nœud stratifié contre des formes fermées (ADR-241 D3). Les références sont écrites au plan, avant la mesure.

use super::*;
use crate::hydro_network::geometry::{Tetrahedron, VolumeShape};
use crate::hydro_network::liquids::{self, Liquid};
use crate::SimTime;

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

// --- S560 — le débit par couches (ADR-241 D4). Références écrites au plan.

const PAS_US: u64 = 100_000;

fn orifice(from: u16, to: Option<u16>, position_um: [i64; 3]) -> Opening {
    Opening { from, to, flow: Flow::Orifice { area_mm2: 1_000 }, position_um, discharge: 0.62, ..Default::default() }
}

/// (1) Le manomètre en U : l'eau passe de gauche à droite jusqu'à `1000·h_g = 1000·h_d + 850·0,4` — `h_g` = 1,17 m, la surface de droite
/// (huile comprise) 6 cm plus haut.
#[test]
fn a_u_tube_with_oil_on_one_side_balances_pressures_not_surfaces_s560() {
    let cells = box_cells([0, 0, 0], [1_000_000, 1_000_000, 2_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let mut nodes = [
        HydroNode { volume_ml: 1_500_000, capacity_ml: 2_000_000, origin_um: [0; 3], shape: 0 },
        HydroNode { volume_ml: 900_000, capacity_ml: 2_000_000, origin_um: [1_000_000, 0, 0], shape: 0 },
    ];
    let seuil = [1_000_000, 500_000, 0];
    let mut edges = [orifice(0, Some(1), seuil), orifice(1, Some(0), seuil)];
    let mut composition = [1_500_000i64, 0, 500_000, 400_000];
    let mut scratch = [0i64; 2];
    let g = [0.0, 0.0, -9.81];
    let mut trace = Vec::new();
    for pas in 1..=6_000 {
        liquids::step_liquids(&mut nodes, &mut edges, &shapes, g, Meteo::SEC, SimTime(PAS_US), &mut scratch, &mut composition,
            &[EAU, HUILE], 0).unwrap();
        if pas % 1_000 == 0 {
            trace.push(composition[0] as f64 * 1e-6);
        }
    }
    let surface_droite = shapes.surface_plane(&nodes[1], g).unwrap().offset_um * 1e-6;
    println!("S560 manomètre : h_g toutes les 100 s {trace:?} ; final {:.6} m (1,17), eau à droite {:.6} m (0,83), surface de droite \
              {surface_droite:.6} m (1,23) ; composition {composition:?}", composition[0] as f64 * 1e-6, composition[2] as f64 * 1e-6);
    assert!((composition[0] as f64 * 1e-6 - 1.17).abs() < 1e-4, "critère 1 : h_g");
    assert_eq!((composition[1], composition[3]), (0, 400_000), "critère 1 : l'huile reste à droite");
    assert_eq!(composition[0] + composition[2], 2_000_000, "critère 1 : l'eau conservée");
    assert_eq!(nodes[0].volume_ml + nodes[1].volume_ml, 2_400_000);
}

/// (2) La vidange stratifiée : l'eau sort seule, sous la charge `h_e + 0,425`, épuisée en `2·(√0,925 − √0,425)/(C_d·a·√(2g))`.
#[test]
fn a_layered_tank_drains_its_bottom_layer_first_s560() {
    let cells = box_cells([0, 0, 0], [1_000_000, 1_000_000, 2_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let mut nodes = [HydroNode { volume_ml: 1_000_000, capacity_ml: 2_000_000, origin_um: [0; 3], shape: 0 }];
    let mut edges = [orifice(0, None, [500_000, 500_000, 0])];
    let mut composition = [500_000i64, 500_000];
    let mut scratch = [0i64; 1];
    let g = [0.0, 0.0, -9.81];
    let attendu = 2.0 * (0.925f64.sqrt() - 0.425f64.sqrt()) / (0.62 * 1e-3 * (2.0 * 9.81f64).sqrt());
    let mut epuisee = None;
    for pas in 1..=4_000 {
        liquids::step_liquids(&mut nodes, &mut edges, &shapes, g, Meteo::SEC, SimTime(PAS_US), &mut scratch, &mut composition,
            &[EAU, HUILE], 0).unwrap();
        if composition[0] > 0 {
            assert_eq!(composition[1], 500_000, "critère 2 : l'huile intacte tant que l'eau reste (pas {pas})");
        } else if epuisee.is_none() {
            epuisee = Some(pas as f64 * 0.1);
        }
    }
    let t = epuisee.expect("l'eau s'épuise");
    println!("S560 vidange : l'eau épuisée à {t:.1} s pour {attendu:.2} s ({:+.3} %) ; il reste {} ml d'huile", 100. * (t / attendu - 1.),
        composition[1]);
    assert!((t / attendu - 1.).abs() < 5e-3, "critère 2 : la durée");
}

/// (3) Un seul liquide : `step_liquids` suit `step` à 2 ml près ; (4) les refus.
#[test]
fn one_liquid_follows_the_present_step_and_bad_inputs_are_refused_s560() {
    let cells = box_cells([0, 0, 0], [1_000_000, 1_000_000, 2_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let g = [0.0, 0.0, -9.81];
    let mut a = [HydroNode { volume_ml: 1_000_000, capacity_ml: 2_000_000, origin_um: [0; 3], shape: 0 }];
    let mut b = a;
    let (mut ea, mut eb) = ([orifice(0, None, [500_000, 500_000, 0])], [orifice(0, None, [500_000, 500_000, 0])]);
    let (mut sa, mut sb) = ([0i64; 1], [0i64; 1]);
    let mut composition = [1_000_000i64];
    let mut pire = 0i64;
    for _ in 0..4_000 {
        step(&mut a, &mut ea, &shapes, g, SimTime(PAS_US), &mut sa).unwrap();
        liquids::step_liquids(&mut b, &mut eb, &shapes, g, Meteo::SEC, SimTime(PAS_US), &mut sb, &mut composition, &[EAU], 0).unwrap();
        pire = pire.max((a[0].volume_ml - b[0].volume_ml).abs());
    }
    println!("S560 un liquide : l'écart à `step` au pire {pire} ml ; final {} et {} ml", a[0].volume_ml, b[0].volume_ml);
    assert!(pire <= 2, "critère 3");
    assert_eq!(composition[0], b[0].volume_ml);
    // (4) Une table +Z ; une composition qui ne somme pas ; la pluie hors de la table.
    let v = b[0].volume_ml;
    let table: [i64; super::SHAPE_ENTRIES] = std::array::from_fn(|i| i as i64 * 30_000);
    let plus_z = Shapes::new(&table).unwrap();
    assert_eq!(liquids::step_liquids(&mut b, &mut eb, &plus_z, g, Meteo::SEC, SimTime(PAS_US), &mut sb, &mut [v], &[EAU], 0)
        .err(), Some(Error::Shape), "critère 4 : une table +Z");
    let avant = b;
    assert_eq!(liquids::step_liquids(&mut b, &mut eb, &shapes, g, Meteo::SEC, SimTime(PAS_US), &mut sb, &mut [v + 1], &[EAU], 0)
        .err(), Some(Error::Capacity), "critère 4 : la somme");
    assert_eq!(liquids::step_liquids(&mut b, &mut eb, &shapes, g, Meteo::SEC, SimTime(PAS_US), &mut sb, &mut [v], &[EAU], 1)
        .err(), Some(Error::Capacity), "critère 4 : la pluie");
    assert_eq!(b, avant, "critère 4 : rien d'écrit");
}
