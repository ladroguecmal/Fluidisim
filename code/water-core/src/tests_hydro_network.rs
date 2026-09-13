//! S224 — réception du noyau V (ADR-010), autour du cas canonique **C12**.
use super::*;
use crate::SimTime;

/// Prisme : table de forme linéaire, donc lecture de hauteur **exacte**.
fn prism(height_um: i64) -> [i64; SHAPE_ENTRIES] {
    let mut t = [0i64; SHAPE_ENTRIES];
    for (i, v) in t.iter_mut().enumerate() {
        *v = height_um * i as i64 / (SHAPE_ENTRIES - 1) as i64;
    }
    t
}

/// Montage de C12 : réservoir de 1 m² de section, 1 m d'eau, orifice de 10 cm² au fond.
fn c12() -> ([HydroNode; 1], [Orifice; 1], [i64; SHAPE_ENTRIES]) {
    let table = prism(1_000_000); // 1 m en micromètres
    let node = HydroNode {
        volume_ml: 1_000_000, // 1 m³ = 1e6 ml
        capacity_ml: 1_000_000,
        floor_um: 0,
        shape: 0,
    };
    let edge = Orifice {
        from: 0,
        to: None,
        area_mm2: 1_000, // 10 cm² = 1000 mm²
        sill_um: 0,
        discharge: SHARP_EDGE_DISCHARGE,
        residue_nl: 0,
    };
    ([node], [edge], table)
}

/// C12 — le temps de vidange à ±3 %, référence **analytique**.
///
/// `t = (A/(C_d·a))·√(2 h₀/g) = (1/(0,62·10⁻³))·√(2/9,81) = 728 s`. La charge décroît pendant la
/// vidange ; la valeur à charge constante vaudrait la moitié, et c'est l'erreur qu'ADR-010 §3 a
/// corrigée en S03.
#[test]
fn c12_drain_time_matches_the_variable_head_integral_s224() {
    let (mut nodes, mut edges, table) = c12();
    let shapes = Shapes::new(&table).unwrap();
    let mut scratch = [0i64; 1];
    let reference_s = 728.0f64;
    let mut steps = 0u64;
    let limit = (4.0 * reference_s * 1e6 / STEP_US as f64) as u64;
    while nodes[0].volume_ml > 0 && steps < limit {
        step(&mut nodes, &mut edges, &shapes, 9.81, SimTime(STEP_US), &mut scratch).unwrap();
        steps += 1;
    }
    let drained_s = steps as f64 * STEP_US as f64 * 1e-6;
    let error = (drained_s - reference_s).abs() / reference_s;
    println!(
        "C12 vidange={drained_s} s contre {reference_s} s, ecart={:.4} %",
        error * 100.0
    );
    assert!(
        nodes[0].volume_ml == 0,
        "reservoir non vide : {} ml",
        nodes[0].volume_ml
    );
    assert!(
        error <= 0.03,
        "temps de vidange hors des 3 % : {drained_s} s contre {reference_s} s"
    );
}

/// La masse est conservée **exactement** : un transfert entier retiré d'un nœud et ajouté à l'autre
/// ne peut rien perdre. Réseau fermé, deux contenants.
#[test]
fn closed_network_conserves_volume_exactly_s224() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [
        HydroNode {
            volume_ml: 900_000,
            capacity_ml: 1_000_000,
            floor_um: 500_000,
            shape: 0,
        },
        HydroNode {
            volume_ml: 100_000,
            capacity_ml: 1_000_000,
            floor_um: 0,
            shape: 0,
        },
    ];
    let mut edges = [Orifice {
        from: 0,
        to: Some(1),
        area_mm2: 1_000,
        sill_um: 0,
        discharge: SHARP_EDGE_DISCHARGE,
        residue_nl: 0,
    }];
    let mut scratch = [0i64; 1];
    let total = nodes[0].volume_ml + nodes[1].volume_ml;
    for _ in 0..20_000 {
        step(&mut nodes, &mut edges, &shapes, 9.81, SimTime(STEP_US), &mut scratch).unwrap();
        assert_eq!(
            nodes[0].volume_ml + nodes[1].volume_ml,
            total,
            "masse perdue ou creee"
        );
        for n in &nodes {
            assert!(
                n.volume_ml >= 0 && n.volume_ml <= n.capacity_ml,
                "volume hors bornes : {n:?}"
            );
        }
    }
}

/// Un nœud presque vide alimentant trois fuites ne devient pas négatif : c'est la normalisation
/// d'ADR-010 §4, et sans elle le limiteur par arête ne suffit pas.
#[test]
fn three_leaks_on_a_nearly_empty_node_stay_non_negative_s224() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [
        HydroNode {
            volume_ml: 50,
            capacity_ml: 1_000_000,
            floor_um: 1_000_000,
            shape: 0,
        },
        HydroNode {
            volume_ml: 0,
            capacity_ml: 1_000_000,
            floor_um: 0,
            shape: 0,
        },
        HydroNode {
            volume_ml: 0,
            capacity_ml: 1_000_000,
            floor_um: 0,
            shape: 0,
        },
        HydroNode {
            volume_ml: 0,
            capacity_ml: 1_000_000,
            floor_um: 0,
            shape: 0,
        },
    ];
    let mut edges = [1u16, 2, 3].map(|to| Orifice {
        from: 0,
        to: Some(to),
        area_mm2: 100_000,
        sill_um: 0,
        discharge: SHARP_EDGE_DISCHARGE,
        residue_nl: 0,
    });
    let mut scratch = [0i64; 3];
    let total: i64 = nodes.iter().map(|n| n.volume_ml).sum();
    for _ in 0..100 {
        step(&mut nodes, &mut edges, &shapes, 9.81, SimTime(STEP_US), &mut scratch).unwrap();
        assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>(), total);
        for n in &nodes {
            assert!(n.volume_ml >= 0, "volume negatif : {n:?}");
        }
    }
    assert_eq!(nodes[0].volume_ml, 0, "le noeud amont devrait etre vide");
}

/// Le remplissage s'arrête à la capacité, et rien ne déborde en silence.
#[test]
fn downstream_capacity_is_never_exceeded_s224() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [
        HydroNode {
            volume_ml: 1_000_000,
            capacity_ml: 1_000_000,
            floor_um: 2_000_000,
            shape: 0,
        },
        HydroNode {
            volume_ml: 0,
            capacity_ml: 1_000,
            floor_um: 0,
            shape: 0,
        },
    ];
    let mut edges = [Orifice {
        from: 0,
        to: Some(1),
        area_mm2: 100_000,
        sill_um: 0,
        discharge: SHARP_EDGE_DISCHARGE,
        residue_nl: 0,
    }];
    let mut scratch = [0i64; 1];
    for _ in 0..50 {
        step(&mut nodes, &mut edges, &shapes, 9.81, SimTime(STEP_US), &mut scratch).unwrap();
        assert!(nodes[1].volume_ml <= nodes[1].capacity_ml);
    }
    assert_eq!(nodes[1].volume_ml, 1_000, "le receveur devrait etre plein");
}

/// I-03 : deux exécutions du même réseau donnent la **même** suite d'états, au bit.
#[test]
fn the_step_is_deterministic_s224() {
    let run = || {
        let (mut nodes, mut edges, table) = c12();
        let shapes = Shapes::new(&table).unwrap();
        let mut scratch = [0i64; 1];
        let mut trace = Vec::new();
        for _ in 0..500 {
            step(&mut nodes, &mut edges, &shapes, 9.81, SimTime(STEP_US), &mut scratch).unwrap();
            trace.push((nodes[0].volume_ml, edges[0].residue_nl));
        }
        trace
    };
    assert_eq!(run(), run());
}

/// Refus atomiques : un pas refusé ne modifie aucun volume, et chaque cause a son nom.
#[test]
fn refusals_are_atomic_and_named_s224() {
    let (nodes0, edges0, table) = c12();
    let shapes = Shapes::new(&table).unwrap();
    let mut scratch = [0i64; 1];

    for (g, dt) in [(0.0f32, STEP_US), (f32::NAN, STEP_US), (9.81, 0)] {
        let mut nodes = nodes0;
        let mut edges = edges0;
        assert_eq!(
            step(&mut nodes, &mut edges, &shapes, g, SimTime(dt), &mut scratch),
            Err(Error::Domain)
        );
        assert_eq!(
            nodes[0].volume_ml, nodes0[0].volume_ml,
            "volume touche par un refus"
        );
    }
    let mut nodes = nodes0;
    let mut edges = edges0;
    edges[0].from = 7;
    assert_eq!(
        step(&mut nodes, &mut edges, &shapes, 9.81, SimTime(STEP_US), &mut scratch),
        Err(Error::Capacity)
    );
    assert_eq!(nodes[0].volume_ml, nodes0[0].volume_ml);

    let mut nodes = nodes0;
    let mut edges = edges0;
    assert_eq!(
        step(&mut nodes, &mut edges, &shapes, 9.81, SimTime(STEP_US), &mut []),
        Err(Error::Capacity)
    );
    assert_eq!(nodes[0].volume_ml, nodes0[0].volume_ml);

    let mut bad = prism(1_000_000);
    bad[10] = 0;
    assert_eq!(Shapes::new(&bad).err(), Some(Error::Shape));
    assert_eq!(Shapes::new(&[0i64; 3]).err(), Some(Error::Shape));
}

/// Pourquoi ADR-010 §4 exige le **report de reste** : sans lui, la vidange s'arrête.
///
/// Le débit d'un pas passe sous le millilitre quand la charge descend sous ≈13 µm — soit les
/// derniers ~13 ml d'un réservoir de 1 m². La troncature ne perd aucune masse, mais elle rend
/// **nul** tout transfert inférieur au millilitre : le contenant garde de la charge et cesse de se
/// vider. Le reste reporté est ce qui fait franchir le seuil.
#[test]
fn without_the_residue_carry_the_drain_stalls_s224() {
    let (mut nodes, mut edges, table) = c12();
    let shapes = Shapes::new(&table).unwrap();
    let mut scratch = [0i64; 1];
    let mut steps = 0u64;
    let limit = (4.0 * 728.0 * 1e6 / STEP_US as f64) as u64;
    while nodes[0].volume_ml > 0 && steps < limit {
        step(&mut nodes, &mut edges, &shapes, 9.81, SimTime(STEP_US), &mut scratch).unwrap();
        edges[0].residue_nl = 0; // l'hôte jette le reste : c'est le cas que l'ADR interdit
        steps += 1;
    }
    println!(
        "sans report : reste {} ml apres {} s",
        nodes[0].volume_ml,
        steps as f64 * STEP_US as f64 * 1e-6
    );
    assert!(
        nodes[0].volume_ml > 0,
        "la vidange devrait s'arreter sans report de reste"
    );
    assert!(
        nodes[0].volume_ml < 20,
        "l'arret doit venir du dernier centimetre d'eau, pas d'un defaut plus tot : {} ml",
        nodes[0].volume_ml
    );
}
