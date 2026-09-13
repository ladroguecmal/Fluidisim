//! S224 — réception du noyau V (ADR-010), autour du cas canonique **C12**.
//! S226 — `g_eff` devient un vecteur ; ces cas-ci le prennent vertical, et leur invariance est le
//! contrôle qui sépare une généralisation d'une réécriture.
use super::*;
use crate::SimTime;

/// Gravité verticale de référence. Sous elle, la verticale locale vaut exactement `(0, 0, 1)` et
/// la projection redevient une soustraction d'altitudes.
const DOWN: [f32; 3] = [0.0, 0.0, -9.81];

/// Prisme : table de forme linéaire, donc lecture de hauteur **exacte**.
fn prism(height_um: i64) -> [i64; SHAPE_ENTRIES] {
    let mut t = [0i64; SHAPE_ENTRIES];
    for (i, v) in t.iter_mut().enumerate() {
        *v = height_um * i as i64 / (SHAPE_ENTRIES - 1) as i64;
    }
    t
}

/// Montage de C12 : réservoir de 1 m² de section, 1 m d'eau, orifice de 10 cm² au fond.
fn c12() -> ([HydroNode; 1], [Opening; 1], [i64; SHAPE_ENTRIES]) {
    let table = prism(1_000_000); // 1 m en micromètres
    let node = HydroNode {
        volume_ml: 1_000_000, // 1 m³ = 1e6 ml
        capacity_ml: 1_000_000,
        origin_um: [0, 0, 0],
        shape: 0,
    };
    let edge = Opening {
        from: 0,
        to: None,
        flow: Flow::Orifice { area_mm2: 1_000 }, // 10 cm²
        position_um: [0, 0, 0],
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
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
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
            origin_um: [0, 0, 500_000],
            shape: 0,
        },
        HydroNode {
            volume_ml: 100_000,
            capacity_ml: 1_000_000,
            origin_um: [0, 0, 0],
            shape: 0,
        },
    ];
    let mut edges = [Opening {
        from: 0,
        to: Some(1),
        flow: Flow::Orifice { area_mm2: 1_000 },
        position_um: [0, 0, 500_000],
        discharge: SHARP_EDGE_DISCHARGE,
        residue_nl: 0,
    }];
    let mut scratch = [0i64; 1];
    let total = nodes[0].volume_ml + nodes[1].volume_ml;
    for _ in 0..20_000 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
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
            origin_um: [0, 0, 1_000_000],
            shape: 0,
        },
        HydroNode {
            volume_ml: 0,
            capacity_ml: 1_000_000,
            origin_um: [0, 0, 0],
            shape: 0,
        },
        HydroNode {
            volume_ml: 0,
            capacity_ml: 1_000_000,
            origin_um: [0, 0, 0],
            shape: 0,
        },
        HydroNode {
            volume_ml: 0,
            capacity_ml: 1_000_000,
            origin_um: [0, 0, 0],
            shape: 0,
        },
    ];
    let mut edges = [1u16, 2, 3].map(|to| Opening {
        from: 0,
        to: Some(to),
        flow: Flow::Orifice { area_mm2: 100_000 },
        position_um: [0, 0, 1_000_000],
        discharge: SHARP_EDGE_DISCHARGE,
        residue_nl: 0,
    });
    let mut scratch = [0i64; 3];
    let total: i64 = nodes.iter().map(|n| n.volume_ml).sum();
    for _ in 0..100 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
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
            origin_um: [0, 0, 2_000_000],
            shape: 0,
        },
        HydroNode {
            volume_ml: 0,
            capacity_ml: 1_000,
            origin_um: [0, 0, 0],
            shape: 0,
        },
    ];
    let mut edges = [Opening {
        from: 0,
        to: Some(1),
        flow: Flow::Orifice { area_mm2: 100_000 },
        position_um: [0, 0, 2_000_000],
        discharge: SHARP_EDGE_DISCHARGE,
        residue_nl: 0,
    }];
    let mut scratch = [0i64; 1];
    for _ in 0..50 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
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
            step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
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

    for (g, dt) in [([0.0f32; 3], STEP_US), ([0., 0., f32::NAN], STEP_US), (DOWN, 0)] {
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
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch),
        Err(Error::Capacity)
    );
    assert_eq!(nodes[0].volume_ml, nodes0[0].volume_ml);

    let mut nodes = nodes0;
    let mut edges = edges0;
    assert_eq!(
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut []),
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
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
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

/// Le déversoir suit `Q = (2/3)·C_d·b·√(2g)·H^{3/2}` (ADR-010 §3), et il se distingue de l'orifice
/// par son exposant : doubler la charge multiplie le débit par `2^{3/2} = 2,83`, contre `√2 = 1,41`.
/// Le test mesure ce rapport sur le premier pas, où la charge est encore celle qu'on a posée.
#[test]
fn the_weir_follows_the_three_halves_law_s224() {
    let table = prism(2_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let transfer = |volume_ml: i64, flow: Flow| {
        let mut nodes = [
            HydroNode { volume_ml, capacity_ml: 2_000_000, origin_um: [0, 0, 0], shape: 0 },
            HydroNode { volume_ml: 0, capacity_ml: 2_000_000, origin_um: [0, 0, 0], shape: 0 },
        ];
        let mut edges = [Opening {
            from: 0,
            to: None,
            flow,
            position_um: [0, 0, 0],
            discharge: WEIR_DISCHARGE,
            residue_nl: 0,
        }];
        let mut scratch = [0i64; 1];
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        volume_ml - nodes[0].volume_ml
    };
    // Prisme de 2 m sur 2e6 ml : 1e6 ml = 1 m de charge, 2e6 ml = 2 m.
    let weir = Flow::Weir { width_mm: 1_000 };
    let (q1, q2) = (transfer(1_000_000, weir), transfer(2_000_000, weir));
    let ratio = q2 as f64 / q1 as f64;
    println!("deversoir : Q(2 m)/Q(1 m) = {ratio:.4}, attendu 2^1.5 = {:.4}", 2f64.powf(1.5));
    assert!((ratio - 2f64.powf(1.5)).abs() < 0.02, "exposant du deversoir : {ratio}");

    let orifice = Flow::Orifice { area_mm2: 1_000 };
    let (o1, o2) = (transfer(1_000_000, orifice), transfer(2_000_000, orifice));
    let oratio = o2 as f64 / o1 as f64;
    println!("orifice : Q(2 m)/Q(1 m) = {oratio:.4}, attendu sqrt(2) = {:.4}", 2f64.sqrt());
    assert!((oratio - 2f64.sqrt()).abs() < 0.02, "exposant de l'orifice : {oratio}");
}

/// Chaîne de trois contenants, et la question qu'ADR-010 §4 laisse ouverte : « 2 à 4 itérations de
/// Gauss-Seidel par pas suffisent pour un réseau ouvert ». Ce module n'en fait **aucune** — un seul
/// passage explicite, depuis l'état du début de pas. Le test ne suppose pas que c'est assez : il
/// compare le pas de 100 ms à une intégration **cent fois plus fine**, et publie l'écart.
#[test]
fn an_open_chain_tracks_a_hundredfold_finer_step_s224() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let build = || {
        (
            [
                HydroNode { volume_ml: 800_000, capacity_ml: 1_000_000, origin_um: [0, 0, 2_000_000], shape: 0 },
                HydroNode { volume_ml: 200_000, capacity_ml: 1_000_000, origin_um: [0, 0, 1_000_000], shape: 0 },
                HydroNode { volume_ml: 0, capacity_ml: 1_000_000, origin_um: [0, 0, 0], shape: 0 },
            ],
            [
                Opening { from: 0, to: Some(1), flow: Flow::Orifice { area_mm2: 2_000 }, position_um: [0, 0, 2_000_000], discharge: SHARP_EDGE_DISCHARGE, residue_nl: 0 },
                Opening { from: 1, to: Some(2), flow: Flow::Orifice { area_mm2: 2_000 }, position_um: [0, 0, 1_000_000], discharge: SHARP_EDGE_DISCHARGE, residue_nl: 0 },
            ],
        )
    };
    let run = |dt_us: u64, steps: u64| {
        let (mut nodes, mut edges) = build();
        let mut scratch = [0i64; 2];
        let total: i64 = nodes.iter().map(|n| n.volume_ml).sum();
        for _ in 0..steps {
            step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(dt_us), &mut scratch).unwrap();
            assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>(), total, "masse perdue");
            for n in &nodes {
                assert!(n.volume_ml >= 0 && n.volume_ml <= n.capacity_ml, "hors bornes : {n:?}");
            }
        }
        nodes.map(|n| n.volume_ml)
    };
    // 60 s des deux côtés : 600 pas de 100 ms contre 60 000 pas de 1 ms.
    let coarse = run(STEP_US, 600);
    let fine = run(1_000, 60_000);
    let worst = (0..3)
        .map(|i| (coarse[i] - fine[i]).abs() as f64 / 1_000_000.0)
        .fold(0.0f64, f64::max);
    println!("chaine a 60 s : 100 ms {coarse:?} contre 1 ms {fine:?}, ecart max {:.4} % de la capacite", worst * 100.0);
    assert!(worst < 0.02, "le pas de 100 ms ne suit pas le pas fin : {worst}");
}

/// Gravité effective d'un vaisseau accéléré : `0,3 g` latéral, le montage de **C16**.
/// Le « bas » penche vers `+X`, donc l'eau s'accumule de ce côté.
const TILTED: [f32; 3] = [0.3 * 9.81, 0.0, -9.81];

/// Cuve de 4 m² de section et 2 m de haut, remplie à 1 m, avec un hublot **latéral** à `x = +2 m`
/// et `z = 1,2 m` — au-dessus de la surface au repos.
fn hull(position_um: [i64; 3]) -> ([HydroNode; 1], [Opening; 1], [i64; SHAPE_ENTRIES]) {
    let table = prism(2_000_000); // 2 m
    let node = HydroNode {
        volume_ml: 4_000_000, // 4 m³ sur 8 de capacité : surface à 1 m
        capacity_ml: 8_000_000,
        origin_um: [0, 0, 0],
        shape: 0,
    };
    let edge = Opening {
        from: 0,
        to: None,
        flow: Flow::Orifice { area_mm2: 1_000 },
        position_um,
        discharge: SHARP_EDGE_DISCHARGE,
        residue_nl: 0,
    };
    ([node], [edge], table)
}

/// Un pas, et le volume qui en sort. Sert à savoir si une ouverture débite, sans exposer la charge.
fn leaked(position_um: [i64; 3], g: [f32; 3]) -> i64 {
    let (mut nodes, mut edges, _) = hull(position_um);
    let cells = oriented_box_cells_s228([-2_000_000, -500_000, 0], [2_000_000, 500_000, 2_000_000]);
    let volumes = [geometry::VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&volumes).unwrap();
    let mut scratch = [0i64; 1];
    let before = nodes[0].volume_ml;
    // Dix pas : le report de reste fait franchir le millilitre même à charge faible.
    for _ in 0..10 {
        step(&mut nodes, &mut edges, &shapes, g, SimTime(STEP_US), &mut scratch).unwrap();
    }
    before - nodes[0].volume_ml
}

/// ADR-010 §2, mot pour mot : *« un vaisseau qui accélère ne verrait pas son réservoir fuir par le
/// hublot latéral qui se retrouve en bas »*. C'est le test que le module de S224 ne pouvait pas
/// passer, quelle que soit sa précision : il ne recevait que le **module** de `g_eff`.
#[test]
fn a_side_port_leaks_only_when_gravity_tilts_s226() {
    let port = [2_000_000, 0, 1_200_000]; // x = +2 m, z = 1,2 m
    let upright = leaked(port, DOWN);
    let tilted = leaked(port, TILTED);
    println!("hublot lateral : vertical={upright} ml, incline={tilted} ml");
    assert_eq!(upright, 0, "le hublot est au-dessus de la surface au repos");
    assert!(tilted > 0, "sous 0,3 g lateral le hublot passe sous la surface");
}

/// C16, part V : *« inclinaison de la surface au repos à ±1° de la normale à `g_eff` »*.
///
/// L'inclinaison n'est pas lue dans le code — elle est **déduite du comportement** : à deux
/// abscisses, on encadre par dichotomie la cote à laquelle une ouverture se met à débiter. La
/// frontière entre « débite » et « ne débite pas » **est** le plan de surface, et sa pente entre
/// les deux abscisses donne l'inclinaison.
#[test]
fn the_free_surface_is_perpendicular_to_g_eff_s226() {
    let threshold = |x_um: i64| {
        let (mut lo, mut hi) = (-2_000_000i64, 4_000_000i64);
        while hi - lo > 100 {
            let mid = (lo + hi) / 2;
            if leaked([x_um, 0, mid], TILTED) > 0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (lo + hi) / 2
    };
    let (xa, xb) = (-2_000_000i64, 2_000_000i64);
    let (za, zb) = (threshold(xa), threshold(xb));
    let slope = (zb - za) as f64 / (xb - xa) as f64;
    let measured_deg = slope.atan().to_degrees();
    // Pente du plan perpendiculaire à `g_eff`, en fonction de `x` : avec `u = −g/‖g‖`, la cote
    // vaut `z = (h − u_x·x)/u_z`, donc la pente est `−u_x/u_z = −g_x/g_z`. Elle est **positive**
    // ici : l'eau s'accumule du côté où le « bas » penche, et la surface y monte. La première
    // écriture de ce test posait `−a/g` et se trompait donc de signe — le code rendait la bonne
    // valeur, l'attendu non.
    let expected_deg = (-(TILTED[0] as f64) / (TILTED[2] as f64)).atan().to_degrees();
    println!(
        "surface : seuils z({xa})={za} um, z({xb})={zb} um, inclinaison={measured_deg:.4}° contre {expected_deg:.4}°"
    );
    assert!(
        (measured_deg - expected_deg).abs() <= 1.0,
        "inclinaison hors du degre exige par C16 : {measured_deg} contre {expected_deg}"
    );
}

/// Ce que la table de forme perd quand `g_eff` s'incline — et la réponse n'est pas celle prédite.
///
/// ADR-010 §2 cuit `shape_lut` « à partir du maillage (**coupes horizontales**) » : la relation
/// volume → hauteur y suppose une orientation. Quand `g_eff` penche, le plan d'eau penche avec lui,
/// et rien ne dit que la table reste valable. Ce test l'intègre numériquement, pour deux sections.
#[test]
fn what_the_shape_table_loses_when_gravity_tilts_s226() {
    // Aire de la section sous la droite `z = zc + x·pente`, dans un contenant décrit par sa
    // demi-largeur `half(z)`. Intégration en `z`, 200 001 tranches : l'erreur d'intégration est
    // très en dessous des écarts qu'on cherche.
    let area_below = |half: &dyn Fn(f64) -> f64, height: f64, zc: f64, slope: f64| {
        let n = 200_000;
        let mut a = 0.0;
        for i in 0..n {
            let z = height * (i as f64 + 0.5) / n as f64;
            let w = half(z);
            if w <= 0.0 {
                continue;
            }
            // Portion de la tranche `[−w, w]` qui est sous la droite : `x` tel que
            // `z < zc + x·slope`, soit `x > (z − zc)/slope` si `slope > 0`.
            let wet = if slope.abs() < 1e-12 {
                if z < zc { 2.0 * w } else { 0.0 }
            } else {
                let x0 = (z - zc) / slope;
                if slope > 0.0 {
                    (w - x0.clamp(-w, w)).max(0.0)
                } else {
                    (x0.clamp(-w, w) + w).max(0.0)
                }
            };
            a += wet * height / n as f64;
        }
        a
    };
    let slope = 0.3f64; // 0,3 g latéral, la pente de C16
    let height = 2.0f64;
    for (nom, half) in [
        ("prisme", &(|_z: f64| 2.0) as &dyn Fn(f64) -> f64),
        ("coque_en_V", &(|z: f64| 2.0 * z / 2.0) as &dyn Fn(f64) -> f64),
    ] {
        for zc in [0.4f64, 1.0, 1.6] {
            let flat = area_below(half, height, zc, 0.0);
            let tilted = area_below(half, height, zc, slope);
            let error = if flat > 0.0 { (tilted - flat).abs() / flat } else { 0.0 };
            println!(
                "TABLE_FORME section={nom} hauteur_centre={zc} aire_plate={flat:.6} aire_inclinee={tilted:.6} ecart={:.4} %",
                error * 100.0
            );
            if nom == "prisme" && zc == 1.0 {
                // Parois verticales, surface qui ne touche ni le fond ni le plafond : le coin gagné
                // d'un côté vaut exactement celui perdu de l'autre. La table reste **exacte**.
                assert!(error < 1e-4, "prisme centre : la table devrait rester exacte, {error}");
            }
        }
    }
}

/// Trois arrivées se partagent la place du même receveur, y compris à un millilitre près.
/// Le témoin non saturé doit aussi débiter : refuser tous les transferts ne suffit pas.
#[test]
fn confluence_never_overfills_the_receiver_s227() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    for free in [1, 2, 53, 1_001, 1_000_000] {
        let source = HydroNode {
            volume_ml: 1_000_000, capacity_ml: 1_000_000,
            origin_um: [0, 0, 2_000_000], shape: 0,
        };
        let mut nodes = [source; 4];
        nodes[3] = HydroNode {
            volume_ml: 0, capacity_ml: free, origin_um: [0; 3], shape: 0,
        };
        let mut edges = [0, 1, 2].map(|from| Opening {
            from, to: Some(3), flow: Flow::Orifice { area_mm2: 100_000 },
            position_um: [0, 0, 2_000_000], discharge: SHARP_EDGE_DISCHARGE,
            residue_nl: 0,
        });
        let mut scratch = [0; 3];
        for tick in 0..4 {
            step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
            assert!(nodes.iter().all(|n| (0..=n.capacity_ml).contains(&n.volume_ml)),
                    "capacite depassee : libre={free}, pas={tick}, noeuds={nodes:?}");
            assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>(), 3_000_000);
            if tick == 0 {
                assert!(nodes[3].volume_ml > 0, "le temoin doit debiter");
                if free <= 1_001 {
                    assert_eq!(nodes[3].volume_ml, free);
                    assert!(scratch.iter().max().unwrap() - scratch.iter().min().unwrap() <= 1);
                }
            }
        }
    }
}

/// Une grande différence sur l'axe horizontal ne change rien à la charge verticale.
/// La différence doit être calculée en entier avant conversion : pas de panique i64.
#[test]
fn extreme_coordinates_preserve_vertical_transfers_s227() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let run = |x_from, x_to| {
        let mut nodes = [
            HydroNode { volume_ml: 1_000_000, capacity_ml: 1_000_000,
                        origin_um: [x_from, 0, 2_000_000], shape: 0 },
            HydroNode { volume_ml: 0, capacity_ml: 1_000_000,
                        origin_um: [x_to, 0, 0], shape: 0 },
        ];
        let mut edges = [Opening {
            from: 0, to: Some(1), flow: Flow::Orifice { area_mm2: 1_000 },
            position_um: [x_to, 0, 2_000_000], discharge: SHARP_EDGE_DISCHARGE,
            residue_nl: 0,
        }];
        let mut scratch = [0];
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        (nodes.map(|n| n.volume_ml), edges[0].residue_nl)
    };
    let reference = run(0, 0);
    assert!(reference.0[1] > 0);
    assert_eq!(run(i64::MAX, i64::MIN), reference);
    assert_eq!(run(i64::MIN, i64::MAX), reference);
}

/// Un nœud peut recevoir, émettre et déborder vers l'extérieur au même pas. L'intégrité ne
/// dépend pas de l'ordre d'application des arêtes, même si l'arrondi peut dépendre de cet ordre.
#[test]
fn mixed_network_keeps_bounds_and_accounts_for_external_loss_s227() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    for reverse in [false, true] {
        let mut nodes = [50, 70, 10, 0].map(|volume_ml| HydroNode {
            volume_ml, capacity_ml: 100, origin_um: [0; 3], shape: 0,
        });
        nodes[0].origin_um[2] = 3_000_000;
        nodes[1].origin_um[2] = 2_000_000;
        nodes[2].origin_um[2] = 1_000_000;
        let mut edges = [(0, Some(2)), (1, Some(2)), (2, Some(3)), (2, None)]
            .map(|(from, to)| Opening {
                from, to, flow: Flow::Orifice { area_mm2: 100_000 },
                position_um: nodes[from as usize].origin_um,
                discharge: SHARP_EDGE_DISCHARGE, residue_nl: 0,
            });
        if reverse { edges.reverse(); }
        let mut scratch = [0; 4];
        let mut lost = 0;
        for _ in 0..10 {
            step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
            lost += edges.iter().zip(scratch).filter(|(e, _)| e.to.is_none())
                .map(|(_, ml)| ml).sum::<i64>();
            assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>() + lost, 130);
            assert!(nodes.iter().all(|n| (0..=n.capacity_ml).contains(&n.volume_ml)));
        }
        assert!(lost > 0, "le temoin doit effectivement rejeter hors reseau");
    }
}

#[test]
fn late_numeric_refusal_keeps_all_nodes_and_residues_s227() {
    let (mut nodes, base, table) = c12();
    let original_nodes = nodes;
    let shapes = Shapes::new(&table).unwrap();
    let mut edges = [base[0]; 2];
    edges[0].residue_nl = 123;
    edges[1].residue_nl = 456;
    edges[1].flow = Flow::Orifice { area_mm2: i64::MAX };
    edges[1].discharge = f32::MAX;
    assert_eq!(step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut [0; 2]),
               Err(Error::NonFinite));
    assert_eq!(nodes, original_nodes);
    assert_eq!(edges.map(|e| e.residue_nl), [123, 456]);
}

/// Régression A266, activée S228. Un prisme symétrique à mi-remplissage garde
/// sa cote centrale à 1 m tant que le plan ne coupe ni fond ni plafond. S226 testait seulement
/// une aire sous une droite imposée, sans passer par la relation hauteur/volume du module.
#[test]
fn tilted_prism_volume_regression_a266_s227() {
    assert!(leaked([0, 0, 990_000], TILTED) > 0, "temoin sous la surface");
    assert_eq!(leaked([0, 0, 1_010_000], TILTED), 0,
               "un hublot central au-dessus de 1 m ne doit pas fuir");
}

fn oriented_box_cells_s228(lo: [i64; 3], hi: [i64; 3]) -> [geometry::Tetrahedron; 6] {
    let v: [[i64; 3]; 8] = std::array::from_fn(|bits|
        std::array::from_fn(|axis| if bits & (1 << axis) == 0 { lo[axis] } else { hi[axis] }));
    [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].map(|p|
        geometry::Tetrahedron::new([v[0], v[1 << p[0]], v[(1 << p[0]) | (1 << p[1])], v[7]]).unwrap())
}

#[test]
fn receiving_surface_and_large_origin_use_the_same_plane_s228() {
    let cells = oriented_box_cells_s228([-2_000_000, -500_000, 0], [2_000_000, 500_000, 2_000_000]);
    let volumes = [geometry::VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&volumes).unwrap();
    let run = |origin_um| {
        let mut nodes = [4_080_000, 4_000_000].map(|volume_ml| HydroNode {
            volume_ml, capacity_ml: 8_000_000, origin_um, shape: 0,
        });
        let mut edges = [Opening {
            from: 0, to: Some(1), flow: Flow::Orifice { area_mm2: 1_000 },
            position_um: origin_um, discharge: SHARP_EDGE_DISCHARGE, residue_nl: 0,
        }];
        step(&mut nodes, &mut edges, &shapes, TILTED, SimTime(STEP_US), &mut [0]).unwrap();
        assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>(), 8_080_000);
        (nodes[1].volume_ml - 4_000_000, edges[0].residue_nl)
    };
    let reference = run([0; 3]);
    // Deux surfaces centrales à 1,02 et 1 m : |g_eff| Δh_normal = |g_z| 0,02 m.
    let expected = (SHARP_EDGE_DISCHARGE as f64 * 1e-3
        * (2.0 * -(TILTED[2] as f64) * 0.02).sqrt() * 0.1 * 1e6) as i64;
    assert_eq!(reference.0, expected);
    assert_eq!(expected, 38, "le temoin doit effectivement transferer");
    assert_eq!(run([i64::MAX, i64::MIN, 0]), reference);
}

#[test]
fn a_real_hull_does_not_leak_above_its_volume_surface_s228() {
    let v = [[0, -500_000, 0], [-2_000_000, -500_000, 2_000_000], [2_000_000, -500_000, 2_000_000],
             [0, 500_000, 0], [-2_000_000, 500_000, 2_000_000], [2_000_000, 500_000, 2_000_000]];
    let cells = [[0, 1, 2, 3], [1, 2, 3, 4], [2, 3, 4, 5]].map(|ids|
        geometry::Tetrahedron::new(ids.map(|i| v[i])).unwrap());
    let volumes = [geometry::VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&volumes).unwrap();
    // Cale |x| <= z <= 2, longueur 1 : sous pente 0,3 et 1 m³, cote centrale sqrt(1-0,3²).
    let run = |z| {
        let mut nodes = [HydroNode { volume_ml: 1_000_000, capacity_ml: 4_000_000,
                                    origin_um: [0; 3], shape: 0 }];
        let mut edges = [Opening { from: 0, to: None, flow: Flow::Orifice { area_mm2: 1_000 },
            position_um: [0, 0, z], discharge: SHARP_EDGE_DISCHARGE, residue_nl: 0 }];
        for _ in 0..10 {
            step(&mut nodes, &mut edges, &shapes, TILTED, SimTime(STEP_US), &mut [0]).unwrap();
        }
        1_000_000 - nodes[0].volume_ml
    };
    assert!(run(940_000) > 0);
    assert_eq!(run(960_000), 0);
}

#[test]
fn geometric_refusals_leave_nodes_and_residues_untouched_s228() {
    let (nodes0, edges0, table) = c12();
    let legacy = Shapes::new(&table).unwrap();
    let mut nodes = nodes0;
    let mut edges = edges0;
    edges[0].residue_nl = 123;
    assert_eq!(step(&mut nodes, &mut edges, &legacy, TILTED, SimTime(STEP_US), &mut [0]),
               Err(Error::Orientation));
    assert_eq!(nodes, nodes0);
    assert_eq!(edges[0].residue_nl, 123);

    let small = oriented_box_cells_s228([0; 3], [1_000_000; 3]);
    let large = oriented_box_cells_s228([0; 3], [3_000_000_000; 3]);
    let volumes = [geometry::VolumeShape::new(&small).unwrap(), geometry::VolumeShape::new(&large).unwrap()];
    let shapes = Shapes::from_volumes(&volumes).unwrap();
    let initial = [nodes0[0], HydroNode {
        volume_ml: (1i64 << 53) + 1, capacity_ml: volumes[1].capacity_ml(),
        origin_um: [0; 3], shape: 1,
    }];
    let mut nodes = initial;
    let mut edges = [edges0[0]; 2];
    edges[0].residue_nl = 123;
    edges[1].from = 1;
    edges[1].residue_nl = 456;
    assert_eq!(step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut [0; 2]),
               Err(Error::Resolution));
    assert_eq!(nodes, initial);
    assert_eq!(edges.map(|e| e.residue_nl), [123, 456]);
    nodes[1].capacity_ml -= 1;
    let invalid = nodes;
    assert_eq!(step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut [0; 2]),
               Err(Error::Capacity));
    assert_eq!(nodes, invalid);
    assert_eq!(edges.map(|e| e.residue_nl), [123, 456]);
}
