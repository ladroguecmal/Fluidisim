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
// S375 : la définition de la piscine vit dans `support/piscine.rs`, partagée avec `piscine_delta`.
#[path = "support/piscine.rs"]
mod piscine;
use piscine::*;

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
    let (tables, mut nodes, mut edges) = construire();
    let shapes = Shapes::new(&tables).expect("tables");
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
        edges[1].control_pm = commande_pompe(t);
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
