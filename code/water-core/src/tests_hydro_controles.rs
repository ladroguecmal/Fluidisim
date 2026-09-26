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

fn pump(from: u16, to: Option<u16>, max_flow_mlps: i64, shutoff_head_um: i64, intake_um: [i64; 3], outlet_um: [i64; 3]) -> Opening {
    Opening {
        from, to, flow: Flow::Pump { max_flow_mlps, shutoff_head_um, outlet_um }, position_um: intake_um,
        discharge: 0.0, residue_nl: 0, ..Default::default()
    }
}

/// **Critère 3** — une pompe vide A (1 m², 1 m d'eau) vers B, **refoulement libre** à 3 m (au-dessus de B) ; prise à
/// 0,1 m. `Qmax` = 5 l/s, `H0` = 10 m. Avec `u = H0 − z_ref + h`, `du/dt = −(Qmax/(A·√H0))·√u` : la prise se dénoie à
/// `t = 2·A·√H0·(√u0 − √u1)/Qmax`, `u0` = 8 m, `u1` = 7,1 m — **207,2 s**, à ±1 %. Puis plus rien ne passe : la cuve
/// garde ses 10 cm, à un pas de débit près.
#[test]
fn a_pump_empties_to_its_intake_on_the_analytic_curve_s372() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(1_000_000, 1_000_000, 0), node(0, 1_000_000, 2_000_000)];
    let mut edges = [pump(0, Some(1), 5_000, 10_000_000, [0, 0, 100_000], [0, 0, 3_000_000])];
    let mut scratch = [0i64; 1];
    let mut steps = 0u64;
    let mut dernier = 0u64;
    while steps < 10_000 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        steps += 1;
        if scratch[0] > 0 {
            dernier = steps;
        }
    }
    let (a, h0, qmax) = (1.0f64, 10.0f64, 0.005f64);
    let attendu = 2.0 * a * h0.sqrt() * (8.0f64.sqrt() - 7.1f64.sqrt()) / qmax;
    let t = dernier as f64 * STEP_US as f64 * 1e-6;
    let ecart = (t - attendu) / attendu;
    println!("POMPE_S372 a_sec t={t:.1} s attendu={attendu:.2} s ecart={:.3} % reste_A={} ml B={} ml", ecart * 100.0, nodes[0].volume_ml, nodes[1].volume_ml);
    assert!(ecart.abs() < 0.01, "la prise se dénoie à {t} s contre {attendu} s");
    assert!((99_000..=100_000).contains(&nodes[0].volume_ml), "A garde {} ml sous sa prise", nodes[0].volume_ml);
    assert_eq!(nodes[0].volume_ml + nodes[1].volume_ml, 1_000_000, "masse");
}

/// **Critère 4** — la pompe **noyée** : refoulement au fond de B, dont le fond est à 2 m ; `H0` = 2,5 m. La hauteur
/// statique `2 + h_B − h_A` monte jusqu'à la hauteur de barrage : l'équilibre est `h_B − h_A = 0,5 m` avec
/// `h_A + h_B = 1 m`, soit **A à 25 cm, B à 75 cm**, à ±1 %.
#[test]
fn a_drowned_pump_stops_at_its_shutoff_head_s372() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(1_000_000, 1_000_000, 0), node(0, 1_000_000, 2_000_000)];
    let mut edges = [pump(0, Some(1), 5_000, 2_500_000, [0, 0, 0], [0, 0, 2_000_000])];
    let mut scratch = [0i64; 1];
    for _ in 0..20_000 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
    }
    println!("POMPE_S372 barrage A={} ml B={} ml (250 000 / 750 000 attendus)", nodes[0].volume_ml, nodes[1].volume_ml);
    assert!((nodes[0].volume_ml - 250_000).abs() <= 2_500, "A = {}", nodes[0].volume_ml);
    assert_eq!(nodes[0].volume_ml + nodes[1].volume_ml, 1_000_000, "masse");
}

/// **Critère 4, vitesse** — lois de similitude : à hauteur statique nulle, le débit vaut `n·Qmax` (5 l/s à pleine
/// vitesse, **2,5 l/s à mi-vitesse**, à ±1 % sur une seconde) ; et à mi-vitesse la hauteur de barrage tombe au quart —
/// `H0` = 10 m ne monte plus 3 m (`n²·H0` = 2,5 m), quand la pleine vitesse le fait.
#[test]
fn pump_speed_follows_the_affinity_laws_s372() {
    let table = prism(1_000_000);
    // A : 10 m² sur 1 m (capacité 10 m³), pleine ; refoulement hors réseau à sa surface — Δh ≈ 0 pendant une seconde.
    let shapes = Shapes::new(&table).unwrap();
    let debit = |c: i64, sortie_z: i64| {
        let mut nodes = [node(10_000_000, 10_000_000, 0)];
        let mut edges = [Opening { control_pm: c, ..pump(0, None, 5_000, 10_000_000, [0, 0, 0], [0, 0, sortie_z]) }];
        let mut scratch = [0i64; 1];
        for _ in 0..10 {
            step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        }
        (10_000_000 - nodes[0].volume_ml) as f64
    };
    let plein = debit(CONTROL_FULL, 1_000_000);
    let moitie = debit(500, 1_000_000);
    println!("POMPE_S372 similitude plein={plein} ml/s moitie={moitie} ml/s");
    assert!((plein - 5_000.0).abs() / 5_000.0 < 0.01, "pleine vitesse : {plein}");
    assert!((moitie - 2_500.0).abs() / 2_500.0 < 0.01, "mi-vitesse : {moitie}");
    assert!(debit(CONTROL_FULL, 4_000_000) > 0.0, "à pleine vitesse, 3 m se montent");
    assert_eq!(debit(500, 4_000_000), 0.0, "à mi-vitesse, 3 m dépassent n²·H0 = 2,5 m");
    assert_eq!(debit(0, 1_000_000), 0.0, "arrêtée");
}

/// À sec : une prise au-dessus de la surface ne débite rien, quelle que soit la vitesse.
#[test]
fn a_pump_above_the_surface_runs_dry_s372() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(400_000, 1_000_000, 0)];
    let mut edges = [pump(0, None, 5_000, 10_000_000, [0, 0, 500_000], [0, 0, 0])];
    let mut scratch = [0i64; 1];
    for _ in 0..1_000 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
    }
    assert_eq!(nodes[0].volume_ml, 400_000);
}

/// **Critère 5** — un réseau d'avarie : un compartiment inondé par un orifice depuis la mer (un grand nœud), une vanne
/// vers le compartiment voisin, une pompe de cale qui rejette par-dessus bord, une seconde qui refoule dans une citerne
/// sur le pont ; les commandes changent en cours de route. La masse est **exacte** à chaque pas (rejet compté), les
/// volumes bornés, et deux exécutions donnent **la même trajectoire au bit**.
#[test]
fn a_damage_network_with_valves_and_pumps_conserves_and_repeats_s372() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let run = || {
        let mut nodes = [
            node(90_000_000, 100_000_000, -500_000), // la mer, 100 m² sur 1 m
            node(0, 4_000_000, -2_000_000),          // compartiment inondé, 4 m²
            node(0, 4_000_000, -2_000_000),          // compartiment voisin
            node(200_000, 1_000_000, 1_000_000),     // citerne sur le pont, 1 m²
        ];
        let mut edges = [
            orifice(0, Some(1), 20_000, [0, 0, -1_500_000]),
            Opening { control_pm: 0, ..orifice(1, Some(2), 50_000, [0, 0, -2_000_000]) },
            pump(1, None, 20_000, 8_000_000, [0, 0, -2_000_000], [0, 0, 1_000_000]),
            pump(2, Some(3), 3_000, 6_000_000, [0, 0, -2_000_000], [0, 0, 1_000_000]),
        ];
        let mut scratch = [0i64; 4];
        let total: i64 = nodes.iter().map(|n| n.volume_ml).sum();
        let mut rejete = 0i64;
        let mut h = Hasher64::new();
        for k in 0..6_000u64 {
            edges[1].control_pm = if k >= 1_500 { 700 } else { 0 };
            edges[2].control_pm = if (2_000..4_000).contains(&k) { 1_000 } else { 400 };
            edges[0].control_pm = if k >= 5_000 { 0 } else { 1_000 };
            step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
            rejete += edges.iter().zip(scratch).filter(|(e, _)| e.to.is_none()).map(|(_, ml)| ml).sum::<i64>();
            assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>() + rejete, total, "masse au pas {k}");
            assert!(nodes.iter().all(|n| (0..=n.capacity_ml).contains(&n.volume_ml)), "bornes au pas {k}");
            hash_state(&mut h, &nodes, &edges);
        }
        (h.finish(), nodes.map(|n| n.volume_ml), rejete)
    };
    let (a, fin, rejete) = run();
    let (b, _, _) = run();
    println!("POMPE_S372 avarie fin={fin:?} rejete={rejete} ml empreinte={a:#018x}");
    assert_eq!(a, b, "deux exécutions divergent");
    assert!(rejete > 0 && fin[2] > 0 && fin[3] > 200_000, "chaque arête doit avoir servi : {fin:?}, {rejete}");
}

/// Une pompe sans hauteur de barrage, ou de débit négatif, est refusée atomiquement.
#[test]
fn an_unrepresentable_pump_is_refused_atomically_s372() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    for (q, h0) in [(5_000, 0), (5_000, -1), (-1, 10_000_000)] {
        let mut nodes = [node(900_000, 1_000_000, 0)];
        let mut edges = [pump(0, None, q, h0, [0, 0, 0], [0, 0, 2_000_000])];
        let mut scratch = [0i64; 1];
        assert_eq!(step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch), Err(Error::Capacity));
        assert_eq!(nodes[0].volume_ml, 900_000);
    }
}

/// Le montage des essais d'instantané : une citerne qui se vide par une vanne dans une cale, et une pompe de cale qui
/// rejette hors réseau.
fn montage_commande() -> ([HydroNode; 2], [Opening; 2]) {
    (
        [node(800_000, 1_000_000, 1_000_000), node(100_000, 1_000_000, 0)],
        [
            orifice(0, Some(1), 2_000, [0, 0, 1_000_000]),
            Opening { control_pm: 400, ..pump(1, None, 3_000, 6_000_000, [0, 0, 0], [0, 0, 2_000_000]) },
        ],
    )
}

/// **Critère 6** — WVST version 2 (ADR-199 D5). Des commandes changées en cours de partie — la vanne à 300, la pompe à
/// pleine vitesse — traversent la capture et la restauration dans une destination sale ; la suite est **identique au
/// bit** à celle du graphe jamais sauvegardé. Témoin d'omission : la même restauration, commandes remises à celles de
/// l'auteur, diverge — ce que la version 1 aurait perdu.
#[test]
fn changed_controls_survive_the_snapshot_bit_for_bit_s372() {
    use super::snapshot::{Baseline, Context};
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let (auteur_n, auteur_e) = montage_commande();
    let base = Baseline::new(41, 2, &auteur_n, &auteur_e, &shapes).unwrap();
    let (mut n, mut e) = montage_commande();
    let mut scratch = [0i64; 2];
    for k in 0..300 {
        e[0].control_pm = if k < 150 { 1_000 } else { 300 };
        e[1].control_pm = if k < 200 { 400 } else { 1_000 };
        step(&mut n, &mut e, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
    }
    let contexte = Context { time: SimTime(300 * STEP_US), dt: SimTime(STEP_US), g_eff: DOWN };
    let mut tampon = vec![0u8; base.snapshot_len(&n, &e).unwrap()];
    base.snapshot_into(&n, &e, contexte, &mut tampon).unwrap();
    let mut rn = [node(7, 1_000_000, 0); 2];
    let mut re = [Opening { control_pm: 13, residue_nl: 99, ..orifice(0, None, 1, [0; 3]) }; 2];
    assert_eq!(base.restore_into(&tampon, &mut rn, &mut re).unwrap(), contexte);
    assert_eq!(re.map(|x| x.control_pm), [300, 1_000], "commandes restaurées");
    let suite = |mut n: [HydroNode; 2], mut e: [Opening; 2]| {
        let mut h = Hasher64::new();
        let mut scratch = [0i64; 2];
        for _ in 0..1_000 {
            step(&mut n, &mut e, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
            hash_state(&mut h, &n, &e);
        }
        h.finish()
    };
    let jamais = suite(n, e);
    let restauree = suite(rn, re);
    let mut omise = re;
    omise[0].control_pm = auteur_e[0].control_pm;
    omise[1].control_pm = auteur_e[1].control_pm;
    let temoin = suite(rn, omise);
    println!("INSTANTANE_S372 taille={} octets suite={jamais:#018x} restauree={restauree:#018x} temoin_omission={temoin:#018x}", tampon.len());
    assert_eq!(restauree, jamais, "la suite restaurée diverge");
    assert_ne!(temoin, jamais, "le témoin d'omission doit discriminer");
    // Deux écarts de commande, deux de volume, restes : 88 + 12 × n.
    assert_eq!((tampon.len() - 88) % 12, 0);
}

/// Refus de la version 2 : une commande égale à celle de l'auteur (non canonique), hors de 0..=1 000, ou un en-tête de
/// version 1 ; aucune destination touchée. Sans commande changée, la taille est celle de la version 1.
#[test]
fn control_records_are_validated_and_version_one_is_refused_s372() {
    use super::snapshot::{Baseline, Context, SnapshotError};
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let (auteur_n, auteur_e) = montage_commande();
    let base = Baseline::new(41, 2, &auteur_n, &auteur_e, &shapes).unwrap();
    let contexte = Context { time: SimTime(0), dt: SimTime(STEP_US), g_eff: DOWN };
    // Sans écart : 88 octets, comme la version 1 le faisait.
    assert_eq!(base.snapshot_len(&auteur_n, &auteur_e).unwrap(), 88);
    let mut e = auteur_e;
    e[1].control_pm = 900;
    let mut bon = vec![0u8; base.snapshot_len(&auteur_n, &e).unwrap()];
    base.snapshot_into(&auteur_n, &e, contexte, &mut bon).unwrap();
    assert_eq!(bon.len(), 100);
    let refait = |b: &mut Vec<u8>| {
        let fin = b.len() - 8;
        let mut h = Hasher64::new();
        for &x in &b[..fin] { h.write_u8(x); }
        let v = h.finish();
        b[fin..].copy_from_slice(&v.to_le_bytes());
    };
    let refuse = |b: &[u8], attendu: SnapshotError| {
        let mut dn = auteur_n;
        let mut de = auteur_e;
        assert_eq!(base.restore_into(b, &mut dn, &mut de), Err(attendu));
        assert_eq!((dn, de.map(|x| x.control_pm)), (auteur_n, auteur_e.map(|x| x.control_pm)));
    };
    for (valeur, attendu) in [(400i64, SnapshotError::Record), (1_001, SnapshotError::Record), (-1, SnapshotError::Record)] {
        let mut b = bon.clone();
        b[84..92].copy_from_slice(&valeur.to_le_bytes());
        refait(&mut b);
        refuse(&b, attendu);
    }
    let mut v1 = bon.clone();
    v1[4] = 1;
    refait(&mut v1);
    refuse(&v1, SnapshotError::Version);
    // Et une commande hors bornes ne se capture pas.
    let mut hors = auteur_e;
    hors[0].control_pm = 1_001;
    assert_eq!(base.snapshot_len(&auteur_n, &hors), Err(SnapshotError::State(Error::Capacity)));
}

/// Une arête de pluie sur le nœud `n`, ouverture `catchment_mm2`, exposition `control_pm`.
fn pluie(n: u16, catchment_mm2: i64, control_pm: i64) -> Opening {
    Opening {
        from: n, to: Some(n), flow: Flow::Rain { catchment_mm2 }, position_um: [0; 3],
        discharge: 0.0, residue_nl: 0, control_pm
    }
}

/// Une heure de pluie (36 000 pas) sur un bassin de 32 m² (prisme de 1,5 m, 48 m³, 40 m³ au départ) ; l'exposition peut
/// changer au pas `bascule` (vers `apres`). Rend le volume gagné, en ml.
fn une_heure_de_pluie(pluie_mm_h: f32, exposition: i64, bascule: u64, apres: i64) -> i64 {
    let table = prism(1_500_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(40_000_000, 48_000_000, 0)];
    let mut edges = [pluie(0, 32_000_000, exposition)];
    let mut scratch = [0i64; 1];
    for k in 0..36_000u64 {
        if k == bascule {
            edges[0].control_pm = apres;
        }
        step_meteo(&mut nodes, &mut edges, &shapes, DOWN, Meteo { pluie_mm_h }, SimTime(STEP_US), &mut scratch).unwrap();
    }
    nodes[0].volume_ml - 40_000_000
}

/// **Critère 2 de S378** (ADR-204) — 10 mm/h sur 32 m² pendant une heure : **320 000 ml** (0,32 m³) à 1 ml près ; sous une
/// demi-bâche (exposition 500) : 160 000 ; sous une bâche entière : **0** ; bâche entière posée à 30 min : 160 000 ;
/// demi-bâche posée à 30 min : 240 000. *Le critère écrit avant donnait 240 000 pour la bâche entière posée à 30 min :
/// erreur d'arithmétique (c'est la valeur de la demi-bâche) ; les deux cas sont éprouvés.*
#[test]
fn an_hour_of_rain_fills_by_opening_times_exposure_s378() {
    let cas = [
        (1_000, u64::MAX, 1_000, 320_000),
        (500, u64::MAX, 500, 160_000),
        (0, u64::MAX, 0, 0),
        (1_000, 18_000, 0, 160_000),
        (1_000, 18_000, 500, 240_000),
    ];
    for (expo, bascule, apres, attendu) in cas {
        let gagne = une_heure_de_pluie(10.0, expo, bascule, apres);
        println!("PLUIE_S378 exposition={expo} bascule={bascule} gagne_ml={gagne} attendu_ml={attendu}");
        assert!((gagne - attendu).abs() <= 1, "{gagne} contre {attendu}");
    }
    assert_eq!(une_heure_de_pluie(0.0, 1_000, u64::MAX, 1_000), 0, "temps sec");
}

/// **Critère 4 de S378** — un contenant plein ne reçoit plus rien par la pluie (la place libre la borne), et un réseau sous
/// la pluie tient son bilan **exactement** : pluie entrée = volume gagné + rejeté hors réseau, à chaque pas.
#[test]
fn rain_respects_capacity_and_the_budget_closes_exactly_s378() {
    let table = prism(1_500_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut plein = [node(48_000_000, 48_000_000, 0)];
    let mut arete = [pluie(0, 32_000_000, 1_000)];
    let mut scratch = [0i64; 4];
    for _ in 0..1_000 {
        step_meteo(&mut plein, &mut arete, &shapes, DOWN, Meteo { pluie_mm_h: 50.0 }, SimTime(STEP_US), &mut scratch[..1]).unwrap();
    }
    assert_eq!(plein[0].volume_ml, 48_000_000);
    // Deux bacs sous la pluie, un orifice de l'un à l'autre, une fuite hors réseau ; bilan à chaque pas.
    let mut nodes = [node(500_000, 1_000_000, 1_000_000), node(0, 1_000_000, 0)];
    let mut edges = [
        pluie(0, 1_000_000, 1_000),
        orifice(0, Some(1), 500, [0, 0, 1_000_000]),
        pluie(1, 1_000_000, 400),
        orifice(1, None, 200, [0, 0, 0]),
    ];
    let depart: i64 = nodes.iter().map(|n| n.volume_ml).sum();
    let (mut entre, mut sorti) = (0i64, 0i64);
    for _ in 0..20_000 {
        step_meteo(&mut nodes, &mut edges, &shapes, DOWN, Meteo { pluie_mm_h: 80.0 }, SimTime(STEP_US), &mut scratch).unwrap();
        entre += scratch[0] + scratch[2];
        sorti += scratch[3];
        assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>(), depart + entre - sorti, "bilan");
        assert!(nodes.iter().all(|n| (0..=n.capacity_ml).contains(&n.volume_ml)));
    }
    println!("PLUIE_S378 bilan : entre={entre} ml sorti={sorti} ml");
    assert!(entre > 0 && sorti > 0);
}

/// Refus atomiques : une intensité négative ou non finie (`Domain`) ; une arête de pluie dont `from` et `to` diffèrent
/// (`Capacity`). Et la base de sauvegarde accepte une arête de pluie, dont l'exposition changée survit à la restauration.
#[test]
fn rain_refusals_and_snapshot_s378() {
    use super::snapshot::{Baseline, Context};
    let table = prism(1_500_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(40_000_000, 48_000_000, 0), node(0, 48_000_000, 0)];
    let mut edges = [pluie(0, 32_000_000, 1_000)];
    let mut scratch = [0i64; 1];
    for p in [-1.0f32, f32::NAN, f32::INFINITY] {
        assert_eq!(step_meteo(&mut nodes, &mut edges, &shapes, DOWN, Meteo { pluie_mm_h: p }, SimTime(STEP_US), &mut scratch), Err(Error::Domain));
    }
    let mut mal = [Opening { to: Some(1), ..pluie(0, 32_000_000, 1_000) }];
    assert_eq!(step_meteo(&mut nodes, &mut mal, &shapes, DOWN, Meteo { pluie_mm_h: 10.0 }, SimTime(STEP_US), &mut scratch), Err(Error::Capacity));
    assert_eq!(nodes[0].volume_ml, 40_000_000);
    let base = Baseline::new(7, 1, &nodes, &edges, &shapes).unwrap();
    let mut e = edges;
    e[0].control_pm = 500;
    let ctx = Context { time: SimTime(0), dt: SimTime(STEP_US), g_eff: DOWN };
    let mut tampon = vec![0u8; base.snapshot_len(&nodes, &e).unwrap()];
    base.snapshot_into(&nodes, &e, ctx, &mut tampon).unwrap();
    let (mut rn, mut re) = (nodes, edges);
    base.restore_into(&tampon, &mut rn, &mut re).unwrap();
    assert_eq!(re[0].control_pm, 500, "la demi-bâche survit à la sauvegarde");
}

/// **Critère 3 de S378** — la piscine à débordement de S374 sous la pluie, pompe arrêtée : bassin 8 × 4 m (prisme de
/// 1,5 m), déversoir de 4 m au seuil de 1,40 m vers un bac en contrebas, 20 mm/h sur l'ouverture du bassin (32 m²), deux
/// heures depuis le seuil. En régime, le déversoir débite la pluie : `Q = 20 mm/h × 32 m²` = 1,78·10⁻⁴ m³/s, et la charge
/// sur le seuil vaut `(Q/k)^⅔`, `k = ⅔·C_d·b·√(2g)` — **0,857 mm, à ±1 %**. La constante de temps `A/(dQ/dH)` ≈ 110 s :
/// deux heures en font plus de soixante.
#[test]
fn the_overflow_pool_under_rain_spills_the_rain_s378() {
    let table = prism(1_500_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(44_800_000, 48_000_000, 0), node(0, 4_800_000, -1_300_000)];
    let mut edges = [pluie(0, 32_000_000, 1_000), weir(0, Some(1), 4_000, [4_000_000, 0, 1_400_000])];
    let mut scratch = [0i64; 2];
    let mut deverse = 0i64;
    for k in 0..72_000u64 {
        step_meteo(&mut nodes, &mut edges, &shapes, DOWN, Meteo { pluie_mm_h: 20.0 }, SimTime(STEP_US), &mut scratch).unwrap();
        if k >= 72_000 - 6_000 {
            deverse += scratch[1];
        }
    }
    let q = 20e-3 / 3600.0 * 32.0;
    let k = (2.0 / 3.0) * WEIR_DISCHARGE as f64 * 4.0 * (2.0f64 * 9.81).sqrt();
    let h_attendu = (q / k).powf(2.0 / 3.0);
    let surface = shapes.surface_plane(&nodes[0], DOWN).unwrap().offset_um * 1e-6;
    let h = surface - 1.4;
    let q_mesure = deverse as f64 * 1e-6 / 600.0;
    println!(
        "PLUIE_S378 debordement charge_mm={:.4} attendue_mm={:.4} ecart={:.2}% debit_deversoir={:.4e} pluie={:.4e} m3/s",
        h * 1e3, h_attendu * 1e3, (h - h_attendu) / h_attendu * 100.0, q_mesure, q
    );
    assert!(((h - h_attendu) / h_attendu).abs() < 0.01, "charge {h} contre {h_attendu}");
    assert!(((q_mesure - q) / q).abs() < 0.01, "débit {q_mesure} contre {q}");
}
