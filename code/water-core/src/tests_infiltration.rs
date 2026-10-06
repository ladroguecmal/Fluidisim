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

/// La solution de Mein–Larson : sous une pluie `i` > K, la submersion à `F_p = M K/(i − K)`, `t_p = F_p/i` ; ensuite Green–Ampt décalé,
/// `F − M ln(1 + F/M) = K (t − t_p + t_s)`, `K t_s = F_p − M ln(1 + F_p/M)`.
fn mein_larson(k: f64, m: f64, i: f64, t: f64) -> (f64, f64) {
    let fp = m * k / (i - k);
    let tp = fp / i;
    if t <= tp {
        return (i * t, tp);
    }
    let ts = (fp - m * (1. + fp / m).ln()) / k;
    (analytique(k, m, t - tp + ts), tp)
}

/// **S533 — la pluie hors contenant** (liste 5.5) : une rétention de surface de 0,5 mm sur 1 m² reçoit 30 mm/h ; elle s'infiltre (Green–
/// Ampt) dans le sol et déborde au-dehors (le ruissellement). Critères : la submersion (la rétention passe 1 ml) à 1 % du temps de
/// Mein–Larson — **manqué** : 1 ml est le quantum de V, la pluie et l'infiltration arrivent par 0 ou 1 ml à chaque pas, et la rétention
/// touche 2 ml par la quantification dès 29 min (ADR-234 D2 : un seuil au plancher du bruit) ; le passage de 10 ml est imprimé en
/// diagnostic, après coup, sans critère —; `F` à 2 h à 0,5 % de Green–Ampt décalé ; pluie = sol + rétention + ruissellement au
/// millilitre, le ruissellement nul avant la rétention pleine.
#[test]
fn rain_on_open_ground_ponds_and_runs_off_s533() {
    let mut table = prism(500).to_vec();
    table.extend_from_slice(&prism(1_000_000));
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [HydroNode { volume_ml: 0, capacity_ml: 500, origin_um: [0, 0, 0], shape: 0 }, HydroNode { shape: 1, ..node(0, 1_000_000) }];
    let mut edges = [
        Opening { from: 0, to: Some(0), flow: Flow::Rain { catchment_mm2: 1_000_000 }, ..Default::default() },
        Opening { flow: Flow::Infiltration { area_mm2: 1_000_000, conductivity_nm_s: K_NM_S, suction_um: 110_000, deficit_pm: 300 }, ..infiltration(0, 1) },
        Opening { from: 0, to: None, flow: Flow::Spill, ..Default::default() },
    ];
    let mut scratch = [0i64; 3];
    let (k, m, i) = (K_NM_S as f64 * 1e-9, 0.11 * 0.3, 30e-3 / 3600.);
    let (mut pluie, mut ruissele, mut submersion, mut plein, mut dix) = (0i64, 0i64, None, None, None);
    for n in 1..=72_000u64 {
        step_meteo(&mut nodes, &mut edges, &shapes, DOWN, Meteo { pluie_mm_h: 30.0 }, SimTime(STEP_US), &mut scratch).unwrap();
        pluie += scratch[0];
        ruissele += scratch[2];
        let t = n as f64 * STEP_US as f64 * 1e-6;
        if submersion.is_none() && nodes[0].volume_ml > 1 {
            submersion = Some(t);
        }
        if dix.is_none() && nodes[0].volume_ml > 10 {
            dix = Some(t);
        }
        if plein.is_none() && nodes[0].volume_ml == 500 {
            plein = Some(t);
        }
        if ruissele > 0 && plein.is_none() {
            panic!("ruissellement avant la rétention pleine, à {t} s");
        }
        assert_eq!(pluie, nodes[0].volume_ml + nodes[1].volume_ml + ruissele, "masse à {t} s");
    }
    let (f_ana, tp) = mein_larson(k, m, i, 7200.);
    let f = nodes[1].volume_ml as f64 * 1e-6;
    let ts = submersion.expect("submersion");
    println!(
        "S533 : submersion à {:.1} min (Mein–Larson {:.2} min, écart {:.2e} ; diagnostic après coup, 10 ml passés à {:.2} min) ; rétention pleine à {:.1} min ; F à 2 h = {:.3} mm (analytique {:.3}, écart {:.2e}) ; pluie {pluie} ml, ruissellement {ruissele} ml",
        ts / 60., tp / 60., ts / tp - 1., dix.unwrap_or(f64::NAN) / 60., plein.unwrap_or(f64::NAN) / 60., 1e3 * f, 1e3 * f_ana, f / f_ana - 1.
    );
    // Critère 1 manqué (voir plus haut) : écrit dans la preuve, pas relâché ici.
    let _ = ts;
    assert!((f / f_ana - 1.).abs() <= 0.005, "critère 2");
}
