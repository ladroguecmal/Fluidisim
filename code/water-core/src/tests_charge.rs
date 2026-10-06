//! S565 — la solution d'un réseau en charge contre deux références indépendantes, écrites au plan (bissection ; Hardy Cross).

use super::*;
use crate::hydro_network::charge::{resoudre, tampon, Conduite, Organe, Sommet};

const TOL: f64 = 1e-13;

/// (1) Trois réservoirs reliés à une jonction.
#[test]
fn three_reservoirs_meet_at_the_head_of_the_bisection_s565() {
    let fixes = [100.0, 80.0, 50.0];
    let conduites = [
        Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 2000.0, organe: Organe::Aucun },
        Conduite { a: Sommet::Fixe(1), b: Sommet::Jonction(0), resistance: 3000.0, organe: Organe::Aucun },
        Conduite { a: Sommet::Fixe(2), b: Sommet::Jonction(0), resistance: 1500.0, organe: Organe::Aucun },
    ];
    let (mut h, mut q, mut w) = ([75.0], [0.0; 3], vec![0.0; tampon(1)]);
    let rapport = resoudre(&fixes, &[0.0], &conduites, &mut h, &mut q, &mut w, TOL, 30).unwrap();
    println!("S565 trois réservoirs : h_j = {:.9} m (77,455794994), débits {q:?}, {rapport:?}", h[0]);
    assert!((h[0] - 77.455794994).abs() < 1e-8, "critère 1 : la charge");
    for (m, r) in q.iter().zip([0.106170158, 0.029121613, -0.135291771]) {
        assert!((m - r).abs() < 1e-9, "critère 1 : un débit {m} pour {r}");
    }
    assert!(rapport.residu_m3s < 1e-12 && rapport.iterations < 30);
}

/// (2) Une maille, contre Hardy Cross.
#[test]
fn a_loop_matches_hardy_cross_s565() {
    let fixes = [60.0];
    let j = Sommet::Jonction;
    let conduites = [
        Conduite { a: Sommet::Fixe(0), b: j(0), resistance: 500.0, organe: Organe::Aucun },
        Conduite { a: j(0), b: j(1), resistance: 1000.0, organe: Organe::Aucun },
        Conduite { a: j(1), b: j(2), resistance: 1500.0, organe: Organe::Aucun },
        Conduite { a: j(0), b: j(3), resistance: 1200.0, organe: Organe::Aucun },
        Conduite { a: j(3), b: j(2), resistance: 800.0, organe: Organe::Aucun },
    ];
    let demandes = [0.0, 0.06, 0.08, 0.04];
    let (mut h, mut q, mut w) = ([60.0; 4], [0.0; 5], vec![0.0; tampon(4)]);
    let rapport = resoudre(&fixes, &demandes, &conduites, &mut h, &mut q, &mut w, TOL, 30).unwrap();
    println!("S565 maille : charges {h:?}, débits {q:?}, {rapport:?}");
    for (m, r) in h.iter().zip([43.8, 34.964659639, 33.231017516, 34.924075772]) {
        assert!((m - r).abs() < 1e-8, "critère 2 : une charge {m} pour {r}");
    }
    for (m, r) in q.iter().zip([0.18, 0.093996491217, 0.033996491217, 0.086003508783, 0.046003508783]) {
        assert!((m - r).abs() < 1e-9, "critère 2 : un débit {m} pour {r}");
    }
    assert!(rapport.residu_m3s < 1e-12 && rapport.iterations < 30);
}

/// (3) Les refus.
#[test]
fn an_isolated_junction_or_a_bad_pipe_is_refused_s565() {
    let ok = Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 100.0, organe: Organe::Aucun };
    let ile = Conduite { a: Sommet::Jonction(1), b: Sommet::Jonction(2), resistance: 100.0, organe: Organe::Aucun };
    let (mut h, mut q, mut w) = ([0.0; 3], [0.0; 2], vec![0.0; tampon(3)]);
    assert_eq!(resoudre(&[10.0], &[0.0; 3], &[ok, ile], &mut h, &mut q, &mut w, TOL, 30).err(), Some(Error::Domain), "une île");
    let nulle = Conduite { resistance: 0.0, ..ok };
    let (mut h1, mut q1, mut w1) = ([0.0], [0.0], vec![0.0; tampon(1)]);
    assert_eq!(resoudre(&[10.0], &[0.0], &[nulle], &mut h1, &mut q1, &mut w1, TOL, 30).err(), Some(Error::Domain), "une résistance nulle");
    let mut court = vec![0.0; tampon(1) - 1];
    assert_eq!(resoudre(&[10.0], &[0.0], &[ok], &mut h1, &mut q1, &mut court, TOL, 30).err(), Some(Error::Capacity), "un tampon court");
}

// --- S567 — le réseau en charge couplé au pas de V. Références écrites au plan par son script.

use crate::hydro_network::charge::{pas_reseau, Raccord};
use crate::hydro_network::geometry::{Tetrahedron, VolumeShape};
use crate::SimTime;

fn box_cells(lo: [i64; 3], hi: [i64; 3]) -> [Tetrahedron; 6] {
    let v: [[i64; 3]; 8] = std::array::from_fn(|bits|
        std::array::from_fn(|axis| if bits & (1 << axis) == 0 { lo[axis] } else { hi[axis] }));
    [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].map(|p| {
        Tetrahedron::new([v[0], v[1 << p[0]], v[(1 << p[0]) | (1 << p[1])], v[7]]).unwrap()
    })
}

/// Deux cuves de 1 m² (l'eau à 1,5 et 0,5 m) reliées au fond par A–J0–J1–B ; `pas` pas de 100 ms, la demande `robinet` à J0. Rend les
/// volumes après chaque pas et la sortie cumulée.
fn deux_cuves(pas: usize, robinet: f64) -> (Vec<(i64, i64)>, i64) {
    let cells = box_cells([0, 0, 0], [1_000_000, 1_000_000, 2_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let mut nodes = [
        HydroNode { volume_ml: 1_500_000, capacity_ml: 2_000_000, origin_um: [0; 3], shape: 0 },
        HydroNode { volume_ml: 500_000, capacity_ml: 2_000_000, origin_um: [5_000_000, 0, 0], shape: 0 },
    ];
    let raccords = [Raccord { noeud: 0, position_um: [500_000, 500_000, 0] }, Raccord { noeud: 1, position_um: [5_500_000, 500_000, 0] }];
    let conduites = [
        Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 1e4, organe: Organe::Aucun },
        Conduite { a: Sommet::Jonction(0), b: Sommet::Jonction(1), resistance: 2e4, organe: Organe::Aucun },
        Conduite { a: Sommet::Jonction(1), b: Sommet::Fixe(1), resistance: 1e4, organe: Organe::Aucun },
    ];
    let (mut charges, mut debits, mut fixes, mut restes, mut w) = ([1.0; 2], [0.0; 3], [0.0; 2], [0i64; 2], vec![0.0; tampon(2)]);
    let mut sortie = 0i64;
    let mut volumes = Vec::new();
    for _ in 0..pas {
        pas_reseau(&mut nodes, &shapes, [0.0, 0.0, -9.81], SimTime(100_000), &raccords, &[robinet, 0.0], &conduites, &mut charges,
            &mut debits, &mut fixes, &mut restes, &mut w, &mut sortie).unwrap();
        volumes.push((nodes[0].volume_ml, nodes[1].volume_ml));
        assert_eq!(nodes[0].volume_ml + nodes[1].volume_ml + sortie, 2_000_000, "critère 2 : la masse, à l'entier");
    }
    (volumes, sortie)
}

/// (1) Deux cuves s'égalisent par le réseau selon `√Δh = √Δh₀ − (1/A + 1/B)·t/(2√R)` ; (2) la masse à l'entier, la sortie bornée.
#[test]
fn two_tanks_level_through_a_pressurised_network_s567() {
    let (volumes, sortie) = deux_cuves(3_000, 0.0);
    // Un Euler f64 indépendant, au même pas.
    let (mut dh, mut euler) = (1.0f64, Vec::new());
    for _ in 0..3_000 {
        dh -= 0.1 * 2.0 * (dh.max(0.0) / 4e4).sqrt();
        euler.push(dh);
    }
    for (t, ferme) in [(50usize, 0.5625f64), (100, 0.25), (150, 0.0625)] {
        let (a, b) = volumes[t * 10 - 1];
        let mesure = (a - b) as f64 * 1e-6;
        println!("S567 à {t} s : Δh = {mesure:.6} m (loi fermée {ferme}, Euler {:.6})", euler[t * 10 - 1]);
        assert!((mesure - ferme).abs() < 1e-3, "critère 1 : la loi fermée à {t} s");
        assert!((mesure - euler[t * 10 - 1]).abs() < 1e-5, "critère 1 : l'Euler indépendant à {t} s");
    }
    let (a, b) = volumes[2_999];
    println!("S567 à 300 s : {a} et {b} ml ; sortie {sortie} ml");
    assert!(sortie.abs() <= 2, "critère 2 : la sortie sans demande");
}

/// (3) Le robinet : 1 L/s soutiré à J0 pendant 100 s ; (4) les refus.
#[test]
fn a_tap_on_the_network_draws_exactly_its_demand_s567() {
    let (_, sortie) = deux_cuves(1_000, 0.001);
    println!("S567 robinet : {sortie} ml sortis (100 000)");
    assert!((sortie - 100_000).abs() <= 2, "critère 3");
    // Un raccord hors de l'eau.
    let cells = box_cells([0, 0, 0], [1_000_000, 1_000_000, 2_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let mut nodes = [HydroNode { volume_ml: 500_000, capacity_ml: 2_000_000, origin_um: [0; 3], shape: 0 }];
    let avant = nodes;
    let conduites = [Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 1e4, organe: Organe::Aucun }];
    let (mut charges, mut debits, mut fixes, mut restes, mut w, mut sortie) = ([0.0], [0.0], [0.0], [0i64], vec![0.0; tampon(1)], 0i64);
    let sec = [Raccord { noeud: 0, position_um: [500_000, 500_000, 1_900_000] }];
    // S569 : un raccord à sec n'est plus refusé (S567 le refusait) : exutoire à l'air libre, il ne débite rien ici.
    pas_reseau(&mut nodes, &shapes, [0.0, 0.0, -9.81], SimTime(100_000), &sec, &[0.0], &conduites, &mut charges, &mut debits,
        &mut fixes, &mut restes, &mut w, &mut sortie).unwrap();
    let ailleurs = [Raccord { noeud: 3, position_um: [0; 3] }];
    assert_eq!(pas_reseau(&mut nodes, &shapes, [0.0, 0.0, -9.81], SimTime(100_000), &ailleurs, &[0.0], &conduites, &mut charges,
        &mut debits, &mut fixes, &mut restes, &mut w, &mut sortie).err(), Some(Error::Capacity), "critère 4 : un nœud absent");
    assert_eq!((nodes, restes, sortie), (avant, [0], 0), "critère 4 : rien d'écrit");
}

// --- S568 — les pompes et les clapets. Références écrites au plan par son script.

/// (1) Une pompe refoule d'un réservoir bas vers un haut par une jonction.
#[test]
fn a_pump_lifts_water_to_the_operating_point_s568() {
    let conduites = [
        Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 2000.0, organe: Organe::Pompe { h0_m: 30.0, qmax_m3s: 0.05, vitesse: 1.0 } },
        Conduite { a: Sommet::Jonction(0), b: Sommet::Fixe(1), resistance: 3000.0, organe: Organe::Aucun },
    ];
    let (mut h, mut q, mut w) = ([10.0], [0.0; 2], vec![0.0; tampon(1)]);
    let rapport = resoudre(&[0.0, 20.0], &[0.0], &conduites, &mut h, &mut q, &mut w, TOL, 50).unwrap();
    println!("S568 pompe : h_j = {:.9} m (21,764705882), Q = {:.9} m³/s (0,024253563), {rapport:?}", h[0], q[0]);
    assert!((h[0] - 21.764705882).abs() < 1e-8, "critère 1 : la charge");
    assert!((q[0] - 0.024253563).abs() < 1e-8 && (q[1] - q[0]).abs() < 1e-12, "critère 1 : le débit");
}

/// (2) Un clapet se ferme : la branche de 80 m ne laisse passer que vers son réservoir, et la jonction ne voit plus que 100 et 50 m.
#[test]
fn a_check_valve_closes_against_the_flow_s568() {
    let conduites = [
        Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 2000.0, organe: Organe::Aucun },
        Conduite { a: Sommet::Jonction(0), b: Sommet::Fixe(1), resistance: 3000.0, organe: Organe::Clapet },
        Conduite { a: Sommet::Fixe(2), b: Sommet::Jonction(0), resistance: 1500.0, organe: Organe::Aucun },
    ];
    let (mut h, mut q, mut w) = ([75.0], [0.0; 3], vec![0.0; tampon(1)]);
    let rapport = resoudre(&[100.0, 80.0, 50.0], &[0.0], &conduites, &mut h, &mut q, &mut w, TOL, 50).unwrap();
    println!("S568 clapet : h_j = {:.9} m (71,428571429), débits {q:?}, {rapport:?}", h[0]);
    assert!((h[0] - 71.428571429).abs() < 1e-8, "critère 2 : la charge");
    assert!(q[1].abs() < 1e-10, "critère 2 : le clapet fermé ne laisse que sa fuite");
}

/// (3) Couplé : une pompe remplit la cuve haute jusqu'à son refoulement nul, puis rien ne revient.
#[test]
fn a_pump_fills_a_tank_until_shutoff_and_the_valve_holds_s568() {
    let cells = box_cells([0, 0, 0], [1_000_000, 1_000_000, 2_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let mut nodes = [
        HydroNode { volume_ml: 1_500_000, capacity_ml: 2_000_000, origin_um: [0; 3], shape: 0 },
        HydroNode { volume_ml: 200_000, capacity_ml: 2_000_000, origin_um: [5_000_000, 0, 0], shape: 0 },
    ];
    let raccords = [Raccord { noeud: 0, position_um: [500_000, 500_000, 0] }, Raccord { noeud: 1, position_um: [5_500_000, 500_000, 0] }];
    let conduites = [
        Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 1e4, organe: Organe::Pompe { h0_m: 1.0, qmax_m3s: 0.01, vitesse: 1.0 } },
        Conduite { a: Sommet::Jonction(0), b: Sommet::Fixe(1), resistance: 1e-6, organe: Organe::Aucun },
    ];
    let (mut charges, mut debits, mut fixes, mut restes, mut w) = ([1.0], [0.0; 2], [0.0; 2], [0i64; 2], vec![0.0; tampon(1)]);
    let mut sortie = 0i64;
    let mut haut_max = 0i64;
    for pas in 0..4_300 {
        pas_reseau(&mut nodes, &shapes, [0.0, 0.0, -9.81], SimTime(100_000), &raccords, &[0.0], &conduites, &mut charges, &mut debits,
            &mut fixes, &mut restes, &mut w, &mut sortie).unwrap();
        assert_eq!(nodes[0].volume_ml + nodes[1].volume_ml + sortie, 1_700_000, "critère 3 : la masse");
        if pas >= 2_200 {
            assert!(nodes[1].volume_ml >= haut_max - 1, "critère 3 : rien ne revient (pas {pas})");
        }
        haut_max = haut_max.max(nodes[1].volume_ml);
    }
    let (bas, haut) = (nodes[0].volume_ml as f64 * 1e-6, nodes[1].volume_ml as f64 * 1e-6);
    println!("S568 pompe couplée : basse {bas:.6} m (0,35), haute {haut:.6} m (1,35) ; sortie {sortie} ml");
    assert!((bas - 0.35).abs() < 2e-4 && (haut - 1.35).abs() < 2e-4, "critère 3 : l'équilibre au refoulement nul");
}

// --- S569 — la vitesse des pompes ; l'exutoire à l'air libre. Références écrites au plan par son script.

/// (1) La pompe de S568 à `n` = 0,9 (similitude).
#[test]
fn a_slowed_pump_follows_the_affinity_laws_s569() {
    let conduites = [
        Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 2000.0,
            organe: Organe::Pompe { h0_m: 30.0, qmax_m3s: 0.05, vitesse: 0.9 } },
        Conduite { a: Sommet::Jonction(0), b: Sommet::Fixe(1), resistance: 3000.0, organe: Organe::Aucun },
    ];
    let (mut h, mut q, mut w) = ([10.0], [0.0; 2], vec![0.0; tampon(1)]);
    let rapport = resoudre(&[0.0, 20.0], &[0.0], &conduites, &mut h, &mut q, &mut w, TOL, 50).unwrap();
    println!("S569 pompe à 0,9 : h_j = {:.9} m (20,758823529), Q = {:.9} m³/s (0,015904125), {rapport:?}", h[0], q[0]);
    assert!((h[0] - 20.758823529).abs() < 1e-8 && (q[0] - 0.015904125).abs() < 1e-8, "critère 1");
}

/// Deux cuves de 1 m² (A à 1,5 m, B à 0,2 m), A–J0–J1–B (`R` total 4·10⁴), les raccords aux cotes données ; `pas` pas de 100 ms.
fn vidange_s569(cote_a: i64, cote_b: i64, pas: usize) -> Vec<(i64, i64)> {
    let cells = box_cells([0, 0, 0], [1_000_000, 1_000_000, 2_000_000]);
    let formes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&formes).unwrap();
    let mut nodes = [
        HydroNode { volume_ml: 1_500_000, capacity_ml: 2_000_000, origin_um: [0; 3], shape: 0 },
        HydroNode { volume_ml: 200_000, capacity_ml: 2_000_000, origin_um: [5_000_000, 0, 0], shape: 0 },
    ];
    let raccords = [Raccord { noeud: 0, position_um: [1_000_000, 500_000, cote_a] }, Raccord { noeud: 1, position_um: [5_000_000, 500_000, cote_b] }];
    let conduites = [
        Conduite { a: Sommet::Fixe(0), b: Sommet::Jonction(0), resistance: 1e4, organe: Organe::Aucun },
        Conduite { a: Sommet::Jonction(0), b: Sommet::Jonction(1), resistance: 2e4, organe: Organe::Aucun },
        Conduite { a: Sommet::Jonction(1), b: Sommet::Fixe(1), resistance: 1e4, organe: Organe::Aucun },
    ];
    let (mut charges, mut debits, mut fixes, mut restes, mut w) = ([1.0; 2], [0.0; 3], [0.0; 2], [0i64; 2], vec![0.0; tampon(2)]);
    let mut sortie = 0i64;
    let mut volumes = Vec::new();
    for _ in 0..pas {
        pas_reseau(&mut nodes, &shapes, [0.0, 0.0, -9.81], SimTime(100_000), &raccords, &[0.0, 0.0], &conduites, &mut charges,
            &mut debits, &mut fixes, &mut restes, &mut w, &mut sortie).unwrap();
        assert_eq!(nodes[0].volume_ml + nodes[1].volume_ml + sortie, 1_700_000, "la masse, à l'entier");
        volumes.push((nodes[0].volume_ml, nodes[1].volume_ml));
    }
    volumes
}

/// (2) L'exutoire : l'arrivée de B à 1,0 m, au-dessus de son eau ; A se vide par le fond selon `√(h_A − 1) = √0,5 − t/(2√R)`.
#[test]
fn a_free_outlet_above_the_water_drains_to_its_own_level_s569() {
    let v = vidange_s569(0, 1_000_000, 6_000);
    for (t, ferme) in [(100usize, 1.208946609f64), (200, 1.042893219)] {
        let h = v[t * 10 - 1].0 as f64 * 1e-6;
        println!("S569 exutoire à {t} s : h_A = {h:.6} m (loi fermée {ferme:.6})");
        assert!((h - ferme).abs() < 1e-3, "critère 2 à {t} s");
    }
    let (a, b) = v[5_999];
    println!("S569 exutoire, final : A {a} ml (1 000 000), B {b} ml (700 000)");
    assert!((a as f64 * 1e-6 - 1.0).abs() < 2e-4 && (b as f64 * 1e-6 - 0.7).abs() < 2e-4, "critère 2 : l'état final");
}

/// (3) Un raccord qui se dénoie : la sortie de A en paroi à 1,0 m ; A se vide jusqu'à ce que sa surface passe sous sa sortie, puis plus rien.
#[test]
fn an_outlet_that_uncovers_stops_the_flow_s569() {
    let v = vidange_s569(1_000_000, 0, 6_000);
    let (a, _) = v[5_999];
    let arret = v.iter().position(|x| *x == v[5_999]).unwrap();
    println!("S569 dénoyé : A final {a} ml (entre 999 726 et 1 000 000), immobile depuis le pas {arret}");
    assert!((999_726..=1_000_000).contains(&a), "critère 3 : l'arrêt à la sortie");
    assert!(v[arret..].iter().all(|x| *x == v[5_999]), "critère 3 : puis immobile");
}
