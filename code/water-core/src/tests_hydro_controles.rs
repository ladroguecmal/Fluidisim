//! S372 — **vannes et pompes dans V** (ADR-199, liste 5.4). La commande d'une arête (`control_pm`), la vanne qui en
//! découle, la pompe en réseau ouvert ; et d'abord ce qui ne doit pas bouger : à commande pleine, le pas d'avant, au bit.
use super::*;
use crate::hash::Hasher64;
use crate::SimTime;

const DOWN: [f32; 3] = [0.0, 0.0, -9.81];

fn prism(height_um: i64) -> [i64; SHAPE_ENTRIES] {
    let mut t = [0i64; SHAPE_ENTRIES];
    for (i, v) in t.iter_mut().enumerate() {
        *v = height_um * i as i64 / (SHAPE_ENTRIES - 1) as i64;
    }
    t
}

fn node(volume_ml: i64, capacity_ml: i64, z_um: i64) -> HydroNode {
    HydroNode { volume_ml, capacity_ml, origin_um: [0, 0, z_um], shape: 0 }
}

/// Une arête d'orifice, commande par défaut.
fn orifice(from: u16, to: Option<u16>, area_mm2: i64, position_um: [i64; 3]) -> Opening {
    Opening {
        from, to, flow: Flow::Orifice { area_mm2 }, position_um,
        discharge: SHARP_EDGE_DISCHARGE, residue_nl: 0, ..Default::default()
    }
}

fn weir(from: u16, to: Option<u16>, width_mm: i64, position_um: [i64; 3]) -> Opening {
    Opening {
        from, to, flow: Flow::Weir { width_mm }, position_um,
        discharge: WEIR_DISCHARGE, residue_nl: 0, ..Default::default()
    }
}

fn hash_state(h: &mut Hasher64, nodes: &[HydroNode], edges: &[Opening]) {
    for n in nodes {
        h.write_u64(n.volume_ml as u64);
    }
    for e in edges {
        h.write_u64(e.residue_nl as u64);
    }
}

/// **Critère 1** : à commande pleine, quatre montages du noyau — C12, une chaîne de trois, un réseau mixte qui rejette hors
/// réseau, un déversoir — donnent **la même trajectoire au bit** qu'avant la
/// commande : volumes et restes de chaque pas, hachés. L'empreinte a été relevée sur le code de S371 (`c04b2974`), avant
/// toute modification du pas.
#[test]
fn full_control_keeps_every_trajectory_bit_for_bit_s372() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut h = Hasher64::new();
    // C12 : 1 m³, orifice de 10 cm² au fond, 2 000 pas.
    let mut nodes = [node(1_000_000, 1_000_000, 0)];
    let mut edges = [orifice(0, None, 1_000, [0, 0, 0])];
    let mut scratch = [0i64; 4];
    for _ in 0..2_000 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        hash_state(&mut h, &nodes, &edges);
    }
    // Chaîne de trois cuves étagées.
    let mut nodes = [node(800_000, 1_000_000, 2_000_000), node(200_000, 1_000_000, 1_000_000), node(0, 1_000_000, 0)];
    let mut edges = [orifice(0, Some(1), 2_000, [0, 0, 2_000_000]), orifice(1, Some(2), 2_000, [0, 0, 1_000_000])];
    for _ in 0..600 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        hash_state(&mut h, &nodes, &edges);
    }
    // Réseau mixte, confluence et rejet hors réseau.
    let mut nodes = [node(50, 100, 3_000_000), node(70, 100, 2_000_000), node(10, 100, 1_000_000), node(0, 100, 0)];
    let mut edges = [(0u16, Some(2u16)), (1, Some(2)), (2, Some(3)), (2, None)]
        .map(|(from, to)| orifice(from, to, 100_000, [0, 0, 3_000_000 - 1_000_000 * from as i64]));
    for _ in 0..200 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        hash_state(&mut h, &nodes, &edges);
    }
    // Déversoir de 20 cm, seuil à 60 cm.
    let mut nodes = [node(1_000_000, 1_000_000, 0), node(0, 1_000_000, -2_000_000)];
    let mut edges = [weir(0, Some(1), 200, [0, 0, 600_000])];
    for _ in 0..1_000 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        hash_state(&mut h, &nodes, &edges);
    }
    let empreinte = h.finish();
    println!("TRAJECTOIRES_V_S372 empreinte={empreinte:#018x}");
    assert_eq!(empreinte, EMPREINTE_S371, "une trajectoire à commande pleine a changé");
}

/// Relevée sur `c04b2974` (S371), avant la commande.
const EMPREINTE_S371: u64 = 0xa02d_e06b_c52b_fd2e;

/// Vide la cuve de C12 (1 m², 1 m d'eau, orifice de 10 cm² au fond) sous la commande `control_pm` ; rend la durée, en s.
fn c12_drain_s(control_pm: i64) -> f64 {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(1_000_000, 1_000_000, 0)];
    let mut edges = [Opening { control_pm, ..orifice(0, None, 1_000, [0, 0, 0]) }];
    let mut scratch = [0i64; 1];
    let mut steps = 0u64;
    while nodes[0].volume_ml > 0 && steps < 100_000 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        steps += 1;
    }
    steps as f64 * STEP_US as f64 * 1e-6
}

/// **Critère 2** — la vanne (ADR-199 D2). La durée de vidange de C12 est inversement proportionnelle à la section :
/// `t = (A/(C_d·a·c))·√(2 h₀/g)`, soit 728 s à commande pleine (S224) et **1 456 s à demi-ouverture**, à ±3 % comme C12.
/// Fermée, **aucun millilitre** ne passe en 10 000 pas, et le reste de l'arête ne bouge pas.
#[test]
fn a_half_open_valve_doubles_the_drain_time_s372() {
    let g = 9.81f64;
    let reference = |c: f64| (1.0 / (0.62 * 1e-3 * c)) * (2.0 / g).sqrt();
    for (c, attendu) in [(1_000, reference(1.0)), (500, reference(0.5)), (250, reference(0.25))] {
        let t = c12_drain_s(c);
        let ecart = (t - attendu) / attendu;
        println!("VANNE_S372 commande={c} vidange={t:.1} s attendu={attendu:.1} s ecart={:.3} %", ecart * 100.0);
        assert!(ecart.abs() < 0.03, "vidange à la commande {c} : {t} s contre {attendu} s");
    }
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(1_000_000, 1_000_000, 0)];
    let mut edges = [Opening { control_pm: 0, residue_nl: 123_456, ..orifice(0, None, 1_000, [0, 0, 0]) }];
    let mut scratch = [0i64; 1];
    for _ in 0..10_000 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
    }
    assert_eq!(nodes[0].volume_ml, 1_000_000, "une vanne fermée a laissé passer de l'eau");
    assert_eq!(edges[0].residue_nl, 123_456, "une vanne fermée a touché au reste de l'arête");
}

/// La vanne rouverte reprend où elle en était : fermée 60 s après 100 s de vidange, puis rouverte — la vidange dure
/// **60 s de plus** qu'à vanne toujours ouverte, à un pas près (la commande est un état, sans mémoire cachée).
#[test]
fn a_valve_closed_then_reopened_resumes_exactly_s372() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let run = |ferme: bool| {
        let mut nodes = [node(1_000_000, 1_000_000, 0)];
        let mut edges = [orifice(0, None, 1_000, [0, 0, 0])];
        let mut scratch = [0i64; 1];
        let mut steps = 0u64;
        while nodes[0].volume_ml > 0 && steps < 100_000 {
            edges[0].control_pm = if ferme && (1_000..1_600).contains(&steps) { 0 } else { CONTROL_FULL };
            step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
            steps += 1;
        }
        steps
    };
    let ouverte = run(false);
    let fermee = run(true);
    println!("VANNE_S372 reouverture : {ouverte} pas contre {fermee} pas");
    assert_eq!(fermee, ouverte + 600, "la vanne fermée 60 s ne décale pas la vidange de 60 s");
}

/// Un déversoir commandé réduit sa lame dans le rapport de la commande : même charge, **un quart** du débit à 250.
#[test]
fn a_weir_gate_scales_its_width_s372() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let passe = |c: i64| {
        let mut nodes = [node(1_000_000, 1_000_000, 0), node(0, 1_000_000, -2_000_000)];
        let mut edges = [Opening { control_pm: c, ..weir(0, Some(1), 1_000, [0, 0, 600_000]) }];
        let mut scratch = [0i64; 1];
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        nodes[1].volume_ml as f64
    };
    let rapport = passe(250) / passe(CONTROL_FULL);
    println!("VANNE_S372 deversoir rapport={rapport:.6}");
    assert!((rapport - 0.25).abs() < 1e-4, "rapport {rapport}");
}

/// Une commande hors de 0..=1 000 est refusée, atomiquement : ni volume ni reste modifiés.
#[test]
fn an_out_of_range_control_is_refused_atomically_s372() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    for c in [-1, 1_001, i64::MAX] {
        let mut nodes = [node(900_000, 1_000_000, 0), node(0, 1_000_000, -2_000_000)];
        let mut edges = [
            Opening { residue_nl: 77, ..orifice(0, Some(1), 1_000, [0, 0, 0]) },
            Opening { control_pm: c, ..orifice(0, None, 1_000, [0, 0, 0]) },
        ];
        let avant = (nodes, edges.map(|e| e.residue_nl));
        let mut scratch = [0i64; 2];
        let r = step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch);
        assert_eq!(r, Err(Error::Capacity));
        assert_eq!((nodes, edges.map(|e| e.residue_nl)), avant);
    }
}
