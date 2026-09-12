//! S188 : la grille de composition de S186, **rejouée sur un réseau ancré** (ADR-118).
//! Une seule variable change — où le dernier nœud se pose — et les nombres de nœuds sont
//! exactement ceux de S186. Véhicule d'essai, pas le solveur du projet (ADR-007 §5).
//! Conditions publiées avant exécution : docs/validation/COMPOSITION-ANCREE-S188.md.
//!
//! La métrique ajoutée est la **tranche qui porte le maximum** : une composition en norme
//! maximum dépend de l'endroit où vivent les deux maxima, et l'ancrage déplace l'un des
//! deux (S187 §8.5). C'est elle qui distingue « la loi tient » de « la loi tient pour une
//! autre raison ».
#[path = "support/perturbative_block.rs"]
#[allow(dead_code)]
mod block;
#[path = "support/reuse_mode.rs"]
#[allow(dead_code)]
mod reuse;
#[path = "support/water_montage.rs"]
#[allow(dead_code)]
mod montage;
#[path = "support/source_snapshots.rs"]
#[allow(dead_code)]
mod snaps;

use block::{
    anchored_indices, base, interior_points, lattice_points_indexed, load_direct,
    scatter_indexed, Block, DX,
};
use montage::{recipe, segments};
use reuse::{build_source, Mode, MODES};
use snaps::snapshots;
use water_core::Hasher64;

const SIDE: usize = 16;
const STEPS: usize = 100;
const DT: f32 = 1.0e-2;
const DT_US: u64 = 10_000;
const CADENCES: [usize; 7] = [1, 2, 4, 8, 16, 32, 64];
/// Nœuds par axe, choisis pour donner **exactement** les nombres de nœuds de la famille
/// isotrope de S186 : 14³ = 2744, 8³ = 512, 5³ = 125, 3³ = 27. Le nom `r` des lignes de
/// S186 leur correspond dans cet ordre.
const PER_AXIS: [usize; 4] = [14, 8, 5, 3];
const S186_NAME: [&str; 4] = ["r=1", "r=2", "r=4", "r=8"];

fn compact(i: usize, j: usize, k: usize) -> usize {
    let m = SIDE - 2;
    ((i - 1) * m + (j - 1)) * m + (k - 1)
}
fn at(i: usize, j: usize, k: usize) -> usize {
    (i * SIDE + j) * SIDE + k
}

/// Écart maximal sur les mailles intérieures, **et la tranche qui le porte**.
fn field_gap_at(a: &[[f32; 3]], b: &[[f32; 3]]) -> (f64, usize) {
    let mut best = (0.0f64, 0usize);
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            for k in 1..SIDE - 1 {
                let c = at(i, j, k);
                for x in 0..3 {
                    let d = (a[c][x] as f64 - b[c][x] as f64).abs();
                    if d > best.0 {
                        best = (d, k);
                    }
                }
            }
        }
    }
    best
}
fn field_max(a: &[[f32; 3]]) -> f64 {
    let mut m = 0.0f64;
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            for k in 1..SIDE - 1 {
                let c = at(i, j, k);
                for x in 0..3 {
                    m = m.max((a[c][x] as f64).abs());
                }
            }
        }
    }
    m
}
fn identical(a: &[[f32; 3]], b: &[[f32; 3]]) -> bool {
    a.iter()
        .zip(b)
        .all(|(p, q)| (0..3).all(|x| p[x].to_bits() == q[x].to_bits()))
}
fn compact_max(a: &[[f32; 3]]) -> f64 {
    a.iter()
        .flatten()
        .fold(0.0f64, |m, v| m.max((*v as f64).abs()))
}

struct Outcome {
    u: Vec<[f32; 3]>,
    es_max: f64,
}

/// Réemploi temporel sur les valeurs de **nœuds**, puis répandage sur les mailles — l'ordre
/// du runtime, celui de S186. `direct` charge sans interpoler, ce qui n'a de sens qu'au
/// réseau plein et sert la référence.
fn evolve(
    cache: &[Vec<[f32; 3]>],
    idx: &[Vec<usize>; 3],
    mode: Mode,
    c: usize,
    steps: usize,
    dt: f32,
    direct: bool,
    reference_cells: Option<&[Vec<[f32; 3]>]>,
) -> Outcome {
    let n = SIDE;
    let mut blk = Block::at_rest(n);
    let mut used = vec![[0.0f32; 3]; cache[0].len()];
    let mut es_max = 0.0f64;
    for s in 0..steps {
        build_source(cache, mode, c, s, &mut used);
        if direct {
            load_direct(n, &used, &mut blk);
        } else {
            scatter_indexed(n, idx, &used, &mut blk);
        }
        if let Some(refs) = reference_cells {
            let exact = &refs[s];
            for i in 1..n - 1 {
                for j in 1..n - 1 {
                    for k in 1..n - 1 {
                        let t = compact(i, j, k);
                        for x in 0..3 {
                            let d = blk.s[at(i, j, k)][x] as f64 - exact[t][x] as f64;
                            es_max = es_max.max(d.abs());
                        }
                    }
                }
            }
        }
        blk.step(dt, true);
    }
    Outcome { u: blk.u, es_max }
}

fn axes(m: usize) -> [Vec<usize>; 3] {
    [
        anchored_indices(SIDE, m),
        anchored_indices(SIDE, m),
        anchored_indices(SIDE, m),
    ]
}

fn main() {
    let mut h = Hasher64::new();
    let last = CADENCES
        .iter()
        .map(|c| ((STEPS - 1) / c) * c + c)
        .max()
        .unwrap();
    println!(
        "bloc {SIDE}^3, {} mailles interieures, dx {DX} m ; dt {DT} s, {STEPS} pas",
        (SIDE - 2).pow(3)
    );
    println!(
        "profondeur des mailles interieures : z de {:.3} a {:.3} m",
        base(SIDE)[2] + DX,
        base(SIDE)[2] + (SIDE - 2) as f64 * DX
    );
    print!("reseaux ancres :");
    for (t, m) in PER_AXIS.iter().enumerate() {
        let idx = axes(*m);
        print!(
            " {} -> {}^3 = {} noeuds {:?} ;",
            S186_NAME[t],
            idx[2].len(),
            idx[0].len() * idx[1].len() * idx[2].len(),
            idx[2]
        );
    }
    println!();

    // Le réseau plein ancré **est** la liste des mailles intérieures : sans cela la
    // référence et la ligne pleine ne parleraient pas des mêmes points.
    assert_eq!(
        lattice_points_indexed(SIDE, &axes(14)),
        interior_points(SIDE),
        "le reseau ancre a 14 noeuds n'est pas le reseau plein"
    );

    let caches: Vec<Vec<Vec<[f32; 3]>>> = PER_AXIS
        .iter()
        .map(|m| snapshots(&lattice_points_indexed(SIDE, &axes(*m)), last, DT_US))
        .collect();
    let full = &caches[0];
    let full_idx = axes(14);

    let mut s_max = 0.0f64;
    for f in full.iter().take(STEPS) {
        s_max = s_max.max(compact_max(f));
    }
    let reference = evolve(full, &full_idx, Mode::Hold, 1, STEPS, DT, true, None);
    let u_max = field_max(&reference.u);
    println!("max |S| = {s_max:.6e} m/s2 ; max |u'(T)| de reference = {u_max:.6e} m/s");
    h.write_f32(s_max as f32);
    h.write_f32(u_max as f32);

    // -- Réceptions -------------------------------------------------------------------
    println!("\n-- receptions");
    let mut full_ok = true;
    for mode in MODES {
        let o = evolve(full, &full_idx, mode, 1, STEPS, DT, false, None);
        full_ok &= identical(&o.u, &reference.u);
    }
    println!("2 — le reseau plein ancre rend la reference en bits, les trois modes : {full_ok}");
    assert!(full_ok);

    let half = snapshots(&interior_points(SIDE), 2 * STEPS, DT_US / 2);
    let ho = evolve(&half, &full_idx, Mode::Hold, 1, 2 * STEPS, DT / 2.0, true, None);
    let (drift, _) = field_gap_at(&reference.u, &ho.u);
    let floor = 100.0 * drift / u_max;
    println!("5 — plancher de la reference a dt/2 : {floor:.3} % de max |u'(T)|");
    h.write_f32(floor as f32);

    let finite = reference.u.iter().flatten().all(|x| x.is_finite())
        && caches
            .iter()
            .all(|c| c.iter().all(|f| f.iter().flatten().all(|x| x.is_finite())));
    println!("6 — source et champ finis : {finite}");
    assert!(finite);

    // -- Les deux axes seuls, et où vit leur maximum -----------------------------------
    let r0 = recipe();
    let k_max = (r0.radial as f64 - 0.5) * r0.cutoff as f64 / r0.radial as f64;
    let lambda_min = std::f64::consts::TAU / k_max;
    let speed = {
        let v = segments()[0][0].velocity;
        ((v[0] * v[0] + v[1] * v[1]) as f64).sqrt()
    };
    let t_content = lambda_min / speed;

    println!("\n-- axe spatial seul (c=1), reseau ancre ; rappel S186 sur reseau debordant");
    println!("ligne | noeuds/axe | noeuds | eS % | eU % | tranche du max | eU de S186 %");
    const S186_SPATIAL: [f64; 4] = [0.0, 2.5401, 13.6043, 32.9593];
    let mut spatial = [0.0f64; 4];
    let mut spatial_layer = [0usize; 4];
    for (t, m) in PER_AXIS.iter().enumerate() {
        let idx = axes(*m);
        let o = evolve(&caches[t], &idx, Mode::Hold, 1, STEPS, DT, false, Some(full));
        let (eu, layer) = field_gap_at(&o.u, &reference.u);
        spatial[t] = eu;
        spatial_layer[t] = layer;
        println!(
            "{} | {m} | {} | {:.4} | {:.4} | {} | {:.4}",
            S186_NAME[t],
            idx[0].len() * idx[1].len() * idx[2].len(),
            100.0 * o.es_max / s_max,
            100.0 * eu / u_max,
            if eu > 0.0 { layer.to_string() } else { "-".into() },
            S186_SPATIAL[t]
        );
        h.write_f32(o.es_max as f32);
        h.write_f32(eu as f32);
    }

    println!("\n-- axe temporel seul (reseau plein) ; doit redonner S186 et S185");
    println!("mode | c | tau/T | eU % | tranche du max");
    let mut temporal = [[0.0f64; 7]; 3];
    let mut temporal_layer = [[0usize; 7]; 3];
    for (mi, mode) in MODES.iter().enumerate() {
        for (ci, c) in CADENCES.iter().enumerate() {
            let o = evolve(full, &full_idx, *mode, *c, STEPS, DT, false, None);
            let (eu, layer) = field_gap_at(&o.u, &reference.u);
            temporal[mi][ci] = eu;
            temporal_layer[mi][ci] = layer;
            println!(
                "{} | {c} | {:.3} | {:.4} | {}",
                mode.short(),
                *c as f64 * DT as f64 / t_content,
                100.0 * eu / u_max,
                if eu > 0.0 { layer.to_string() } else { "-".into() }
            );
            h.write_f32(eu as f32);
        }
    }

    // -- La grille, rejouée -----------------------------------------------------------
    println!("\n-- grille ancree ; eU en % de max|u'(T)|, puis mesure/prediction par loi");
    println!("mode | ligne | c | eS % | eU % | tranche | additive | quadratique | maximum | juge");
    let mut worst = [(f64::INFINITY, 0.0f64); 3];
    let mut per_mode = [[(f64::INFINITY, 0.0f64); 3]; 3];
    let mut coincide = (0usize, 0usize); // (maxima au même endroit, cases jugées)
    for (mi, mode) in MODES.iter().enumerate() {
        for (t, m) in PER_AXIS.iter().enumerate() {
            let idx = axes(*m);
            for (ci, c) in CADENCES.iter().enumerate() {
                let o = evolve(&caches[t], &idx, *mode, *c, STEPS, DT, false, Some(full));
                let (gap, layer) = field_gap_at(&o.u, &reference.u);
                let eu = gap / u_max;
                let (es, et) = (spatial[t] / u_max, temporal[mi][ci] / u_max);
                let laws = [es + et, (es * es + et * et).sqrt(), es.max(et)];
                let judged =
                    100.0 * eu > floor && 100.0 * es > floor && 100.0 * et > floor;
                let mut cells = String::new();
                for (li, p) in laws.iter().enumerate() {
                    let ratio = if *p > 0.0 { eu / p } else { 0.0 };
                    cells.push_str(&format!(" | {ratio:.3}"));
                    if judged {
                        worst[li].0 = worst[li].0.min(ratio);
                        worst[li].1 = worst[li].1.max(ratio);
                        per_mode[mi][li].0 = per_mode[mi][li].0.min(ratio);
                        per_mode[mi][li].1 = per_mode[mi][li].1.max(ratio);
                    }
                }
                if judged {
                    coincide.1 += 1;
                    if spatial_layer[t] == temporal_layer[mi][ci] {
                        coincide.0 += 1;
                    }
                }
                println!(
                    "{} | {} | {c} | {:.4} | {:.4} | {layer}{cells} | {}",
                    mode.short(),
                    S186_NAME[t],
                    100.0 * o.es_max / s_max,
                    100.0 * eu,
                    if judged { "oui" } else { "plancher" }
                );
                h.write_f32(o.es_max as f32);
                h.write_f32(gap as f32);
            }
        }
    }

    println!("\n-- verdict global, cases jugees ; retenue si tout tient dans [0,80 ; 1,25]");
    for (li, nom) in ["additive", "quadratique", "maximum"].iter().enumerate() {
        let (lo, hi) = worst[li];
        println!(
            "{nom} : rapport de {lo:.3} a {hi:.3} -> {}",
            if lo >= 0.8 && hi <= 1.25 {
                "retenue"
            } else {
                "rejetee"
            }
        );
    }
    println!("\n-- verdict mode par mode ; entre parentheses, ce que S186 retenait");
    println!("mode | additive | quadratique | maximum | retenue S188 | retenue S186");
    const S186_KEPT: [&str; 3] = ["maximum", "maximum", "additive, quadratique"];
    for (mi, mode) in MODES.iter().enumerate() {
        let mut kept = String::new();
        for (li, nom) in ["additive", "quadratique", "maximum"].iter().enumerate() {
            let (lo, hi) = per_mode[mi][li];
            if lo.is_finite() && lo >= 0.8 && hi <= 1.25 {
                if !kept.is_empty() {
                    kept.push_str(", ");
                }
                kept.push_str(nom);
            }
        }
        println!(
            "{} | {:.3}-{:.3} | {:.3}-{:.3} | {:.3}-{:.3} | {} | {}",
            mode.label(),
            per_mode[mi][0].0,
            per_mode[mi][0].1,
            per_mode[mi][1].0,
            per_mode[mi][1].1,
            per_mode[mi][2].0,
            per_mode[mi][2].1,
            if kept.is_empty() { "aucune".into() } else { kept },
            S186_KEPT[mi]
        );
    }
    println!(
        "\nles deux maxima vivent-ils sur la meme tranche ? {} cases jugees sur {}",
        coincide.0, coincide.1
    );

    println!(
        "\nempreinte des resultats (reception 1, doit se reproduire) : {:#018x}",
        h.finish()
    );
}
