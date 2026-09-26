//! S374 — **la piscine de V**, pour la voir dans Godot (demande de l'utilisateur). Une piscine à débordement : un bassin de
//! 8 × 4 m, un déversoir de 4 m sur son petit côté est, qui tombe dans un bac tampon plus bas, et une pompe qui rend l'eau au
//! bassin par une buse au-dessus de sa surface, côté ouest. V calcule (ADR-010, ADR-199), et publie chaque pas — les
//! surfaces par `Shapes::surface_plane`, les débits d'arête, la commande — dans `godot/donnees/piscine_v.json`, que
//! `godot/piscine.gd` rejoue. Rien n'est reconstruit par le consommateur (I-01).
//!
//! Scénario : la pompe arrêtée 5 s, lancée, arrêtée à 240 s ; 330 s en tout, au pas de V (100 ms).
//!
//! Lancer, depuis la racine du dépôt :
//! `cargo run --manifest-path code/Cargo.toml --release --offline -p water-core --example piscine_v [sortie.json]`.
//! Lignes `PISCINE_V_S374` : bilan de volume à chaque pas (critère 1), régime établi contre le point de fonctionnement
//! analytique (critère 2).
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use water_core::hydro_network::*;
use water_core::SimTime;

const G: [f32; 3] = [0.0, 0.0, -9.81];

/// Le bassin : 8 × 4 m au sol, 1,5 m de haut ; le fond au niveau 0.
const BASSIN_TAILLE_M: [f64; 3] = [8.0, 4.0, 1.5];
const BASSIN_FOND_M: [f64; 3] = [0.0, 0.0, 0.0];
/// Le bac tampon : 1 × 4 m, 1,2 m de haut, contre la face extérieure du mur est (épais de 0,2 m), fond à −1,3 m.
const TAMPON_TAILLE_M: [f64; 3] = [1.0, 4.0, 1.2];
const TAMPON_FOND_M: [f64; 3] = [4.7, 0.0, -1.3];
/// Le déversoir : toute la largeur du mur est, seuil à 1,40 m.
const SEUIL_M: [f64; 3] = [4.0, 0.0, 1.4];
const DEVERSOIR_LARGEUR_M: f64 = 4.0;
/// La pompe : prise à 5 cm du fond du bac tampon, buse sur le mur ouest à 1,7 m ; 12 l/s, hauteur de barrage 8 m (données
/// d'auteur, ADR-199 D3). La buse, 5 cm de diamètre, n'existe pas dans V : elle ne sert qu'au dessin du jet.
const PRISE_M: [f64; 3] = [4.7, 0.0, -1.25];
const SORTIE_M: [f64; 3] = [-4.0, 0.0, 1.7];
const POMPE_QMAX_MLPS: i64 = 12_000;
const POMPE_H0_M: f64 = 8.0;
const BUSE_DIAMETRE_M: f64 = 0.05;
/// Niveaux de départ : le bassin 5 mm sous le seuil, le bac tampon à 0,80 m.
const BASSIN_DEPART_M: f64 = 1.395;
const TAMPON_DEPART_M: f64 = 0.80;
const POMPE_MARCHE_S: f64 = 5.0;
const POMPE_ARRET_S: f64 = 240.0;
const DUREE_S: f64 = 330.0;

fn um(m: f64) -> i64 {
    (m * 1e6).round() as i64
}
fn um3(p: [f64; 3]) -> [i64; 3] {
    p.map(um)
}
fn prisme(hauteur_um: i64) -> [i64; SHAPE_ENTRIES] {
    let mut t = [0i64; SHAPE_ENTRIES];
    for (i, v) in t.iter_mut().enumerate() {
        *v = hauteur_um * i as i64 / (SHAPE_ENTRIES - 1) as i64;
    }
    t
}

/// Le point de fonctionnement du régime établi, analytique : la charge `H` sur le seuil telle que le déversoir débite ce
/// que la pompe refoule, le volume total fixant le niveau du bac tampon. Point fixe sur `H`, en double.
fn regime_analytique(volume_total_m3: f64) -> (f64, f64, f64) {
    let aire_b = BASSIN_TAILLE_M[0] * BASSIN_TAILLE_M[1];
    let aire_t = TAMPON_TAILLE_M[0] * TAMPON_TAILLE_M[1];
    let k = (2.0 / 3.0) * WEIR_DISCHARGE as f64 * DEVERSOIR_LARGEUR_M * (2.0 * 9.81f64).sqrt();
    let mut h = 0.01;
    let mut q = 0.0;
    let mut hb = 0.0;
    for _ in 0..200 {
        hb = (volume_total_m3 - aire_b * (SEUIL_M[2] + h)) / aire_t;
        let dh = SORTIE_M[2] - (TAMPON_FOND_M[2] + hb);
        q = POMPE_QMAX_MLPS as f64 * 1e-6 * (1.0 - dh / POMPE_H0_M).sqrt();
        h = (q / k).powf(2.0 / 3.0);
    }
    (h, hb, q)
}

fn main() -> std::io::Result<()> {
    let sortie = std::env::args().nth(1).unwrap_or_else(|| "godot/donnees/piscine_v.json".into());
    let tables = [prisme(um(BASSIN_TAILLE_M[2])), prisme(um(TAMPON_TAILLE_M[2]))].concat();
    let shapes = Shapes::new(&tables).expect("tables");
    let aire_b = BASSIN_TAILLE_M[0] * BASSIN_TAILLE_M[1];
    let aire_t = TAMPON_TAILLE_M[0] * TAMPON_TAILLE_M[1];
    let ml = |m3: f64| (m3 * 1e6).round() as i64;
    let mut nodes = [
        HydroNode {
            volume_ml: ml(aire_b * BASSIN_DEPART_M),
            capacity_ml: ml(aire_b * BASSIN_TAILLE_M[2]),
            origin_um: um3(BASSIN_FOND_M),
            shape: 0,
        },
        HydroNode {
            volume_ml: ml(aire_t * TAMPON_DEPART_M),
            capacity_ml: ml(aire_t * TAMPON_TAILLE_M[2]),
            origin_um: um3(TAMPON_FOND_M),
            shape: 1,
        },
    ];
    let mut edges = [
        Opening {
            from: 0,
            to: Some(1),
            flow: Flow::Weir { width_mm: (DEVERSOIR_LARGEUR_M * 1e3).round() as i64 },
            position_um: um3(SEUIL_M),
            discharge: WEIR_DISCHARGE,
            residue_nl: 0,
            control_pm: CONTROL_FULL,
        },
        Opening {
            from: 1,
            to: Some(0),
            flow: Flow::Pump { max_flow_mlps: POMPE_QMAX_MLPS, shutoff_head_um: um(POMPE_H0_M), outlet_um: um3(SORTIE_M) },
            position_um: um3(PRISE_M),
            discharge: 0.0,
            residue_nl: 0,
            control_pm: 0,
        },
    ];
    let total: i64 = nodes.iter().map(|n| n.volume_ml).sum();
    let mut scratch = [0i64; 2];
    let dt_s = STEP_US as f64 * 1e-6;
    let pas = (DUREE_S / dt_s).round() as usize;
    let surface = |n: &HydroNode| -> f64 {
        n.origin_um[2] as f64 * 1e-6 + shapes.surface_plane(n, G).expect("surface").offset_um * 1e-6
    };
    let mut lignes: Vec<String> = Vec::with_capacity(pas + 1);
    let mut ligne = |t: f64, nodes: &[HydroNode; 2], q: [i64; 2], c: i64| {
        lignes.push(format!(
            "[{t:.1},{:.7},{:.7},{},{},{:.4},{:.4},{c}]",
            surface(&nodes[0]),
            surface(&nodes[1]),
            nodes[0].volume_ml,
            nodes[1].volume_ml,
            q[0] as f64 * 1e-3 / dt_s,
            q[1] as f64 * 1e-3 / dt_s
        ));
    };
    ligne(0.0, &nodes, [0, 0], 0);
    let mut ecart_max = 0i64;
    let mut regime = (0.0f64, 0.0f64, 0usize, 0.0f64);
    for k in 0..pas {
        let t = k as f64 * dt_s;
        edges[1].control_pm = if (POMPE_MARCHE_S..POMPE_ARRET_S).contains(&t) { CONTROL_FULL } else { 0 };
        step(&mut nodes, &mut edges, &shapes, G, SimTime(STEP_US), &mut scratch).expect("pas de V");
        ecart_max = ecart_max.max((nodes.iter().map(|n| n.volume_ml).sum::<i64>() - total).abs());
        // Le régime établi : la dernière minute avant l'arrêt de la pompe, débits moyens.
        if t >= POMPE_ARRET_S - 60.0 && t < POMPE_ARRET_S {
            regime.0 += scratch[0] as f64;
            regime.1 += scratch[1] as f64;
            regime.2 += 1;
            regime.3 = surface(&nodes[0]);
        }
        ligne(t + dt_s, &nodes, scratch, edges[1].control_pm);
    }
    let n = regime.2 as f64;
    let (q_dev, q_pompe) = (regime.0 / n * 1e-3 / dt_s, regime.1 / n * 1e-3 / dt_s);
    let (h, hb, q) = regime_analytique(total as f64 * 1e-6);
    let charge = regime.3 - SEUIL_M[2];
    println!("PISCINE_V_S374 critere=1 pas={pas} ecart_volume_max_ml={ecart_max} {}", if ecart_max == 0 { "tenu" } else { "manque" });
    let e_dev = (q_dev - q * 1e3) / (q * 1e3);
    let e_pompe = (q_pompe - q * 1e3) / (q * 1e3);
    let e_h = (charge - h) / h;
    println!(
        "PISCINE_V_S374 critere=2 deversoir_ls={q_dev:.4} pompe_ls={q_pompe:.4} analytique_ls={:.4} ecarts={:.3}%/{:.3}% \
         charge_m={charge:.5} analytique_m={h:.5} ecart={:.3}% tampon_analytique_m={hb:.4} {}",
        q * 1e3,
        e_dev * 100.0,
        e_pompe * 100.0,
        e_h * 100.0,
        if e_dev.abs() < 0.01 && e_pompe.abs() < 0.01 && e_h.abs() < 0.01 { "tenu" } else { "manque" }
    );
    // L'export.
    if let Some(dossier) = Path::new(&sortie).parent() {
        create_dir_all(dossier)?;
    }
    let mut f = BufWriter::new(File::create(&sortie)?);
    let v = |p: [f64; 3]| format!("[{},{},{}]", p[0], p[1], p[2]);
    writeln!(f, "{{")?;
    writeln!(f, "\"source\": \"code/water-core/examples/piscine_v.rs (S374)\",")?;
    writeln!(f, "\"pas_s\": {:.3},", dt_s)?;
    writeln!(f, "\"bassin\": {{\"fond_m\": {}, \"taille_m\": {}}},", v(BASSIN_FOND_M), v(BASSIN_TAILLE_M))?;
    writeln!(f, "\"tampon\": {{\"fond_m\": {}, \"taille_m\": {}}},", v(TAMPON_FOND_M), v(TAMPON_TAILLE_M))?;
    writeln!(f, "\"deversoir\": {{\"seuil_m\": {}, \"largeur_m\": {DEVERSOIR_LARGEUR_M}}},", v(SEUIL_M))?;
    writeln!(
        f,
        "\"pompe\": {{\"prise_m\": {}, \"sortie_m\": {}, \"qmax_ls\": {}, \"h0_m\": {POMPE_H0_M}, \"buse_diametre_m\": {BUSE_DIAMETRE_M}}},",
        v(PRISE_M),
        v(SORTIE_M),
        POMPE_QMAX_MLPS as f64 * 1e-3
    )?;
    writeln!(f, "\"colonnes\": [\"t_s\", \"surface_bassin_m\", \"surface_tampon_m\", \"volume_bassin_ml\", \"volume_tampon_ml\", \"deversoir_ls\", \"pompe_ls\", \"commande_pompe\"],")?;
    writeln!(f, "\"pas\": [")?;
    for (i, l) in lignes.iter().enumerate() {
        writeln!(f, "{l}{}", if i + 1 < lignes.len() { "," } else { "" })?;
    }
    writeln!(f, "]}}")?;
    f.flush()?;
    println!("PISCINE_V_S374 export={sortie} lignes={}", lignes.len());
    Ok(())
}
