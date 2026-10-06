//! S538 — **l'inondation limitée par l'air** (liste 5.9, C17, ADR-015 T2) : une poche isotherme scellée dans un compartiment de V.
use super::*;
use crate::SimTime;

const DOWN: [f32; 3] = [0.0, 0.0, -9.81];
const RHO: f64 = 1025.0;

fn prism(height_um: i64) -> [i64; SHAPE_ENTRIES] {
    let mut t = [0i64; SHAPE_ENTRIES];
    for (i, v) in t.iter_mut().enumerate() {
        *v = height_um * i as i64 / (SHAPE_ENTRIES - 1) as i64;
    }
    t
}

/// C17 : la mer (5·10⁴ m², sa surface à 0), un compartiment de 10 m³ (5 m² × 2 m, son plafond à la flottaison), une brèche d'1 dm² à son
/// fond (2 m sous la flottaison). Rend la hauteur d'eau dans le compartiment au fil du temps, toutes les 0,1 s, sur `duree` s.
fn c17(air: Air, duree: u64) -> Vec<f64> {
    let mut table = prism(20_000_000).to_vec();
    table.extend_from_slice(&prism(2_000_000));
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [
        HydroNode { volume_ml: 500_000_000_000, capacity_ml: 1_000_000_000_000, origin_um: [0, 0, -10_000_000], shape: 0 },
        HydroNode { volume_ml: 0, capacity_ml: 10_000_000, origin_um: [0, 0, -2_000_000], shape: 1 },
    ];
    let mut edges = [Opening {
        from: 0,
        to: Some(1),
        flow: Flow::Orifice { area_mm2: 10_000 },
        position_um: [0, 0, -2_000_000],
        discharge: SHARP_EDGE_DISCHARGE,
        ..Default::default()
    }];
    let mut scratch = [0i64; 1];
    let airs = [Air::Open, air];
    let mut heads = [0f64; 2];
    let mut h = Vec::new();
    for _ in 0..duree * 10 {
        step_air(&mut nodes, &mut edges, &shapes, DOWN, Meteo::SEC, SimTime(STEP_US), &mut scratch, &airs, RHO, &mut heads).unwrap();
        h.push(nodes[1].volume_ml as f64 * 1e-6 / 5.0);
    }
    h
}

/// **S538, critère 1 — sans air scellé, le pas d'avant au bit.**
#[test]
fn open_air_is_the_previous_step_bit_for_bit_s538() {
    let mut table = prism(20_000_000).to_vec();
    table.extend_from_slice(&prism(2_000_000));
    let shapes = Shapes::new(&table).unwrap();
    let depart = [
        HydroNode { volume_ml: 500_000_000_000, capacity_ml: 1_000_000_000_000, origin_um: [0, 0, -10_000_000], shape: 0 },
        HydroNode { volume_ml: 0, capacity_ml: 10_000_000, origin_um: [0, 0, -2_000_000], shape: 1 },
    ];
    let breche = Opening { from: 0, to: Some(1), flow: Flow::Orifice { area_mm2: 10_000 }, position_um: [0, 0, -2_000_000],
        discharge: SHARP_EDGE_DISCHARGE, ..Default::default() };
    let (mut a, mut b) = (depart, depart);
    let (mut ea, mut eb) = ([breche], [breche]);
    let (mut sa, mut sb) = ([0i64; 1], [0i64; 1]);
    let mut heads = [0f64; 2];
    for _ in 0..5_000 {
        step_meteo(&mut a, &mut ea, &shapes, DOWN, Meteo::SEC, SimTime(STEP_US), &mut sa).unwrap();
        step_air(&mut b, &mut eb, &shapes, DOWN, Meteo::SEC, SimTime(STEP_US), &mut sb, &[Air::Open; 2], RHO, &mut heads).unwrap();
        assert_eq!((a[0].volume_ml, a[1].volume_ml, ea[0].residue_nl), (b[0].volume_ml, b[1].volume_ml, eb[0].residue_nl));
    }
}

/// **S538, critères 2 à 4 — C17.** Sans évent, la hauteur finale à 0,5 % de l'équilibre de Boyle `p_atm·2/u = p_atm + ρg·u` ; avec évent,
/// 99 % du plein à 1 % de Torricelli ; le rapport des temps de remplissage au-delà de 5 (sans évent, jamais plein).
#[test]
fn a_sealed_compartment_floods_only_until_its_air_holds_s538() {
    let ouvert = c17(Air::Open, 700);
    let scelle = c17(Air::Sealed { pv_pa_ml: P_ATM_PA * 10_000_000.0 }, 2_000);
    let (g, a, cd, aire, haut) = (9.81f64, 0.01f64, SHARP_EDGE_DISCHARGE as f64, 5.0f64, 2.0f64);
    let t99 = 2. * aire / (cd * a * (2. * g).sqrt()) * (haut.sqrt() - (0.01 * haut).sqrt());
    let mesure99 = ouvert.iter().position(|h| *h >= 0.99 * haut).map(|n| (n + 1) as f64 * 0.1).expect("99 %");
    let rg = RHO * g;
    let u = (-P_ATM_PA + (P_ATM_PA * P_ATM_PA + 4. * rg * haut * P_ATM_PA).sqrt()) / (2. * rg);
    let attendu = haut - u;
    let fin = *scelle.last().unwrap();
    let plein_scelle = scelle.iter().position(|h| *h >= 0.99 * haut);
    println!(
        "S538 C17 : avec évent, 99 % à {mesure99:.1} s (Torricelli {t99:.1} s, écart {:.2e}) ; sans évent, {fin:.4} m après 2 000 s (Boyle {attendu:.4} m, écart {:.2e}) ; plein sans évent : {plein_scelle:?}",
        mesure99 / t99 - 1., fin / attendu - 1.
    );
    assert!((mesure99 / t99 - 1.).abs() <= 0.01, "critère 3");
    assert!((fin / attendu - 1.).abs() <= 0.005, "critère 2");
    assert!(plein_scelle.is_none(), "critère 4 : sans évent, jamais plein");
}
