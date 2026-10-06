//! S530 — **l'infiltration dans le sol, Green–Ampt** (liste 5.5) : une arête de V de la flaque vers le sol, intégrée exactement sur le pas.
use super::*;
use crate::SimTime;

const DOWN: [f32; 3] = [0.0, 0.0, -9.81];
/// Limon sableux : K = 1,09 cm/h (3 028 nm/s), ψ = 11 cm, Δθ = 0,3.
const K_NM_S: i64 = 3028;

fn prism(height_um: i64) -> [i64; SHAPE_ENTRIES] {
    let mut t = [0i64; SHAPE_ENTRIES];
    for (i, v) in t.iter_mut().enumerate() {
        *v = height_um * i as i64 / (SHAPE_ENTRIES - 1) as i64;
    }
    t
}

fn node(volume_ml: i64, capacity_ml: i64) -> HydroNode {
    HydroNode { volume_ml, capacity_ml, origin_um: [0, 0, 0], shape: 0 }
}

fn infiltration(from: u16, to: u16) -> Opening {
    Opening {
        from,
        to: Some(to),
        flow: Flow::Infiltration { area_mm2: 1_000_000, conductivity_nm_s: K_NM_S, suction_um: 110_000, deficit_pm: 300 },
        position_um: [0, 0, 0],
        ..Default::default()
    }
}

/// La solution implicite de Green–Ampt : `F` tel que `F − M ln(1 + F/M) = K t` (Newton en f64).
fn analytique(k: f64, m: f64, t: f64) -> f64 {
    let mut f = k * t + (2. * m * k * t).sqrt();
    for _ in 0..100 {
        let g = f - m * (1. + f / m).ln() - k * t;
        f -= g / (f / (m + f));
    }
    f
}

/// **S530, critère 2 — Green–Ampt à charge quasi constante.** Une flaque de 1 000 m² et 1 cm sur 1 m² de sol : la lame infiltrée à
/// 60 s, 10 min et 1 h à 10⁻³ de la solution implicite ; la masse exacte à chaque pas.
#[test]
fn infiltration_follows_green_ampt_s530() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(10_000_000, 1_000_000_000), node(0, 1_000_000)];
    let mut edges = [infiltration(0, 1)];
    let mut scratch = [0i64; 1];
    let total = nodes[0].volume_ml + nodes[1].volume_ml;
    let (k, m) = (K_NM_S as f64 * 1e-9, (0.11 + 0.01) * 0.3);
    for n in 1..=36_000u64 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
        assert_eq!(nodes[0].volume_ml + nodes[1].volume_ml, total, "masse, pas {n}");
        if [600u64, 6_000, 36_000].contains(&n) {
            let t = n as f64 * STEP_US as f64 * 1e-6;
            let f = nodes[1].volume_ml as f64 * 1e-6;
            let attendu = analytique(k, m, t);
            println!("S530 Green–Ampt à {t:.0} s : F = {:.4} mm, analytique {:.4} mm, écart {:.2e}", 1e3 * f, 1e3 * attendu, f / attendu - 1.);
            assert!((f / attendu - 1.).abs() <= 1e-3, "critère 2 à {t} s");
        }
    }
}

/// **S530, critère 3 — le sol plein, la flaque à sec.** Un sol de 10 L sur 1 m² se remplit puis arrête l'infiltration, au millilitre ;
/// une flaque vide n'infiltre rien.
#[test]
fn a_full_soil_and_a_dry_puddle_stop_infiltration_s530() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(10_000_000, 1_000_000_000), node(0, 10_000)];
    let mut edges = [infiltration(0, 1)];
    let mut scratch = [0i64; 1];
    for _ in 0..36_000 {
        step(&mut nodes, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
    }
    assert_eq!(nodes[1].volume_ml, 10_000, "le sol plein");
    assert_eq!(nodes[0].volume_ml, 10_000_000 - 10_000, "la flaque a donné exactement le plein du sol");
    let mut sec = [node(0, 1_000_000), node(0, 1_000_000)];
    for _ in 0..100 {
        step(&mut sec, &mut edges, &shapes, DOWN, SimTime(STEP_US), &mut scratch).unwrap();
    }
    assert_eq!(sec[1].volume_ml, 0, "une flaque à sec n'infiltre rien");
}

/// **S530, critère 4 — la pluie sur le sol.** 5 mm/h sur 1 m² de sol (K = 10,9 mm/h) pendant 1 h : tout entre — 5 L au millilitre près —,
/// la flaque reste sous 2 ml.
#[test]
fn rain_on_soil_soaks_in_s530() {
    let table = prism(1_000_000);
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [node(0, 1_000_000), node(0, 1_000_000)];
    let mut edges = [
        Opening { from: 0, to: Some(0), flow: Flow::Rain { catchment_mm2: 1_000_000 }, ..Default::default() },
        infiltration(0, 1),
    ];
    let mut scratch = [0i64; 2];
    let mut flaque_max = 0;
    for _ in 0..36_000 {
        step_meteo(&mut nodes, &mut edges, &shapes, DOWN, Meteo { pluie_mm_h: 5.0 }, SimTime(STEP_US), &mut scratch).unwrap();
        flaque_max = flaque_max.max(nodes[0].volume_ml);
    }
    println!("S530 pluie : sol {} ml, flaque {} ml (au plus {flaque_max} ml)", nodes[1].volume_ml, nodes[0].volume_ml);
    assert!((nodes[0].volume_ml + nodes[1].volume_ml - 5_000).abs() <= 1, "5 L de pluie");
    assert!((nodes[1].volume_ml - 5_000).abs() <= 1, "tout est entré");
    assert!(flaque_max < 2, "la flaque : {flaque_max} ml");
}
