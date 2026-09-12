//! S186 : composer l'erreur **spatiale** (réseau décimé) et l'erreur **temporelle** (cadence)
//! sur un seul véhicule et contre une seule référence. Véhicule d'essai, pas le solveur du
//! projet (ADR-007 §5). Conditions publiées avant exécution :
//! docs/validation/COMPOSITION-ERREURS-S186.md.
//!
//! Le bloc, le réseau, le `scatter` trilinéaire et les trois modes de réemploi viennent tous
//! de `support/` : S184 les a mesurés en coût, S185 en erreur de cadence, S186 en composition.
//! Un second exemplaire de l'un d'eux rendrait les trois mesures incomparables (L137).
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
    base, interior_points, lattice_points, load_direct, nodes_per_axis, scatter, Block, DX,
    RATIOS,
};
use montage::{recipe, segments};
use reuse::{build_source, Mode, MODES};
use snaps::snapshots;
use water_core::Hasher64;

const SIDE: usize = 16;
const STEPS: usize = 100;
/// Pas de temps de banc, s, et son équivalent en microsecondes de `SimTime`.
const DT: f32 = 1.0e-2;
const DT_US: u64 = 10_000;
const CADENCES: [usize; 7] = [1, 2, 4, 8, 16, 32, 64];

/// Indice compact d'une maille intérieure, dans l'ordre de `interior_points`.
fn compact(i: usize, j: usize, k: usize) -> usize {
    let m = SIDE - 2;
    ((i - 1) * m + (j - 1)) * m + (k - 1)
}
fn at(i: usize, j: usize, k: usize) -> usize {
    (i * SIDE + j) * SIDE + k
}

struct Outcome {
    /// Champ final, indexé comme le bloc.
    u: Vec<[f32; 3]>,
    /// max |S appliquée − S de référence| sur toutes les mailles et tous les pas.
    es_max: f64,
}

/// Évolue le bloc en réemployant `cache` (valeurs de **nœuds** du réseau de pas `r`) selon
/// `mode` et `c`, puis en le répandant sur les mailles. `via_scatter = false` charge les
/// valeurs directement, ce qui n'a de sens qu'à `r = 1` et sert la réception 2.
fn evolve(
    cache: &[Vec<[f32; 3]>],
    r: usize,
    mode: Mode,
    c: usize,
    steps: usize,
    dt: f32,
    via_scatter: bool,
    reference_cells: Option<&[Vec<[f32; 3]>]>,
) -> Outcome {
    let n = SIDE;
    let mut blk = Block::at_rest(n);
    let mut used = vec![[0.0f32; 3]; cache[0].len()];
    let mut es_max = 0.0f64;
    for s in 0..steps {
        build_source(cache, mode, c, s, &mut used);
        if via_scatter {
            scatter(n, r, &used, &mut blk);
        } else {
            load_direct(n, &used, &mut blk);
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

/// Écart maximal entre deux champs, sur les mailles intérieures.
fn field_gap(a: &[[f32; 3]], b: &[[f32; 3]]) -> f64 {
    let mut m = 0.0f64;
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            for k in 1..SIDE - 1 {
                let c = at(i, j, k);
                for x in 0..3 {
                    m = m.max((a[c][x] as f64 - b[c][x] as f64).abs());
                }
            }
        }
    }
    m
}
/// Écart maximal restreint à une tranche de profondeur `k`.
fn layer_gap(a: &[[f32; 3]], b: &[[f32; 3]], k: usize) -> f64 {
    let mut m = 0.0f64;
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            let c = at(i, j, k);
            for x in 0..3 {
                m = m.max((a[c][x] as f64 - b[c][x] as f64).abs());
            }
        }
    }
    m
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
fn layer_max(a: &[[f32; 3]], k: usize) -> f64 {
    let mut m = 0.0f64;
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            let c = at(i, j, k);
            for x in 0..3 {
                m = m.max((a[c][x] as f64).abs());
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

/// Nombre d'onde effectif d'un axe dans une tranche, `√(max|∂²S| / max|S|)`. Estimateur
/// grossier, annoncé comme tel : il dit **quelle échelle est présente**, pas laquelle domine
/// l'énergie. `None` quand la tranche n'a pas de voisin sur cet axe.
fn k_eff(field: &[[f32; 3]], axis: usize, k: usize) -> Option<f64> {
    let inv = 1.0 / (DX * DX);
    let mut d2 = 0.0f64;
    let mut s = 0.0f64;
    let lo = 2;
    let hi = SIDE - 2;
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            let (a, b, c) = (i, j, k);
            // Le voisinage doit rester dans les mailles intérieures sur l'axe sondé.
            let ok = match axis {
                0 => a >= lo && a < hi,
                1 => b >= lo && b < hi,
                _ => c >= lo && c < hi,
            };
            if !ok {
                continue;
            }
            let (p, m) = match axis {
                0 => (compact(a + 1, b, c), compact(a - 1, b, c)),
                1 => (compact(a, b + 1, c), compact(a, b - 1, c)),
                _ => (compact(a, b, c + 1), compact(a, b, c - 1)),
            };
            let o = compact(a, b, c);
            for x in 0..3 {
                let v = field[p][x] as f64 - 2.0 * field[o][x] as f64 + field[m][x] as f64;
                d2 = d2.max((v * inv).abs());
                s = s.max((field[o][x] as f64).abs());
            }
        }
    }
    if s > 0.0 && d2 > 0.0 {
        Some((d2 / s).sqrt())
    } else {
        None
    }
}

fn main() {
    let mut h = Hasher64::new();
    let last = CADENCES
        .iter()
        .map(|c| ((STEPS - 1) / c) * c + c)
        .max()
        .unwrap();
    println!(
        "bloc {SIDE}^3, {} mailles interieures, dx {DX} m ; dt {DT} s, {STEPS} pas = {:.2} s",
        (SIDE - 2).pow(3),
        STEPS as f32 * DT
    );
    let bz = base(SIDE)[2];
    println!(
        "profondeur des mailles interieures : z de {:.3} a {:.3} m",
        bz + DX,
        bz + (SIDE - 2) as f64 * DX
    );
    print!("instantanes par reseau : {} chacun ;", last + 1);
    for r in RATIOS {
        let m = nodes_per_axis(SIDE, r);
        print!(" r={r} -> {m}^3 = {} noeuds", m * m * m);
    }
    println!();

    // Le réseau plein est **exactement** la liste des mailles intérieures : sans cela la
    // référence et la ligne `r = 1` ne parleraient pas des mêmes points.
    assert_eq!(lattice_points(SIDE, 1), interior_points(SIDE), "r=1 n'est pas le reseau plein");

    let caches: Vec<Vec<Vec<[f32; 3]>>> = RATIOS
        .iter()
        .map(|r| snapshots(&lattice_points(SIDE, *r), last, DT_US))
        .collect();
    let full = &caches[0];

    let mut s_max = 0.0f64;
    for f in full.iter().take(STEPS) {
        s_max = s_max.max(compact_max(f));
    }
    let reference = evolve(full, 1, Mode::Hold, 1, STEPS, DT, false, None);
    let u_max = field_max(&reference.u);
    println!(
        "max |S| = {s_max:.6e} m/s2 ; max |u'(T)| de reference = {u_max:.6e} m/s"
    );
    h.write_f32(s_max as f32);
    h.write_f32(u_max as f32);

    // -- Réceptions ---------------------------------------------------------------------
    println!("\n-- receptions");
    let through_scatter = evolve(full, 1, Mode::Hold, 1, STEPS, DT, true, None);
    println!(
        "2 — scatter a r=1 identique au chargement direct, en bits : {}",
        identical(&through_scatter.u, &reference.u)
    );
    assert!(identical(&through_scatter.u, &reference.u));
    let mut same = true;
    for (ri, r) in RATIOS.iter().enumerate() {
        let a = evolve(&caches[ri], *r, Mode::Hold, 1, STEPS, DT, true, None);
        for mode in MODES {
            let b = evolve(&caches[ri], *r, mode, 1, STEPS, DT, true, None);
            same &= identical(&a.u, &b.u);
        }
    }
    println!("4 — a c=1 les trois modes rendent le meme champ, en bits : {same}");
    assert!(same);
    let half_cache = snapshots(&interior_points(SIDE), 2 * STEPS, DT_US / 2);
    let half = evolve(&half_cache, 1, Mode::Hold, 1, 2 * STEPS, DT / 2.0, false, None);
    let floor = 100.0 * field_gap(&reference.u, &half.u) / u_max;
    println!("5 — reference a dt/2 : plancher {floor:.3} % de max |u'(T)|");
    h.write_f32(floor as f32);
    let finite = reference.u.iter().flatten().all(|x| x.is_finite())
        && caches
            .iter()
            .all(|c| c.iter().all(|f| f.iter().flatten().all(|x| x.is_finite())));
    println!("6 — source et champ finis : {finite}");
    assert!(finite);

    // -- Le contenu réellement présent, par tranche de profondeur -----------------------
    let r0 = recipe();
    let k_max = (r0.radial as f64 - 0.5) * r0.cutoff as f64 / r0.radial as f64;
    let lambda_min = std::f64::consts::TAU / k_max;
    let speed = {
        let v = segments()[0][0].velocity;
        ((v[0] * v[0] + v[1] * v[1]) as f64).sqrt()
    };
    let t_content = lambda_min / speed;
    println!(
        "\ncontenu de pression : k_max {k_max:.4} rad/m, lambda_min {lambda_min:.4} m, \
         advecte a {speed} m/s -> T {t_content:.4} s ; decroissance verticale 1/k_max = {:.4} m",
        1.0 / k_max
    );
    println!("\n-- contenu effectif de la source, tranche par tranche (instant T0)");
    println!("k | z (m) | max|S| | k_eff x | k_eff y | k_eff z | lambda_eff z (m)");
    let field0 = &full[0];
    for k in 1..SIDE - 1 {
        let z = bz + k as f64 * DX;
        let mut sm = 0.0f64;
        for i in 1..SIDE - 1 {
            for j in 1..SIDE - 1 {
                for x in 0..3 {
                    sm = sm.max((field0[compact(i, j, k)][x] as f64).abs());
                }
            }
        }
        let f = |o: Option<f64>| o.map(|v| format!("{v:.3}")).unwrap_or_else(|| "-".into());
        let lz = k_eff(field0, 2, k)
            .map(|v| format!("{:.3}", std::f64::consts::TAU / v))
            .unwrap_or_else(|| "-".into());
        println!(
            "{k} | {z:.2} | {sm:.4e} | {} | {} | {} | {}",
            f(k_eff(field0, 0, k)),
            f(k_eff(field0, 1, k)),
            f(k_eff(field0, 2, k)),
            lz
        );
    }

    // -- L'axe spatial seul -------------------------------------------------------------
    println!("\n-- erreur spatiale seule (c=1) ; eS et eU en % de max|S| et max|u'(T)|");
    println!("r | h (m) | h/lambda_min | eS % | eU % | eU/(h/lambda_min)^2");
    let mut spatial = [0.0f64; 4];
    for (ri, r) in RATIOS.iter().enumerate() {
        let o = evolve(&caches[ri], *r, Mode::Hold, 1, STEPS, DT, true, Some(full));
        let eu = field_gap(&o.u, &reference.u);
        spatial[ri] = eu;
        let hl = *r as f64 * DX / lambda_min;
        println!(
            "{r} | {:.2} | {:.4} | {:.4} | {:.4} | {}",
            *r as f64 * DX,
            hl,
            100.0 * o.es_max / s_max,
            100.0 * eu / u_max,
            if *r == 1 {
                "-".into()
            } else {
                format!("{:.4}", (eu / u_max) / (hl * hl))
            }
        );
        h.write_f32(o.es_max as f32);
        h.write_f32(eu as f32);
    }

    println!("\n-- erreur spatiale par tranche de profondeur (c=1), % de max|u'(T)| global");
    println!("k | z (m) | max|u'| local % | r=2 | r=4 | r=8");
    let by_r: Vec<Outcome> = RATIOS
        .iter()
        .enumerate()
        .map(|(ri, r)| evolve(&caches[ri], *r, Mode::Hold, 1, STEPS, DT, true, None))
        .collect();
    for k in 1..SIDE - 1 {
        let z = bz + k as f64 * DX;
        print!(
            "{k} | {z:.2} | {:.2}",
            100.0 * layer_max(&reference.u, k) / u_max
        );
        for ri in 1..RATIOS.len() {
            print!(" | {:.4}", 100.0 * layer_gap(&by_r[ri].u, &reference.u, k) / u_max);
        }
        println!();
    }

    // -- La grille complète, et les trois lois ------------------------------------------
    println!("\n-- grille complete ; eU en % de max|u'(T)|, puis mesure/prediction par loi");
    println!("mode | r | c | tau/T | eS % | eU % | additive | quadratique | maximum | juge");
    let mut worst = [(0.0f64, 0.0f64); 3]; // (min, max) du rapport sur les cases jugées
    for i in 0..3 {
        worst[i] = (f64::INFINITY, 0.0);
    }
    // Le même rapport, séparé par mode de réemploi : une loi peut valoir pour l'un et pas
    // pour l'autre, et le verdict global le cacherait.
    let mut per_mode = [[(f64::INFINITY, 0.0f64); 3]; 3];
    let mut temporal = [[0.0f64; 7]; 3];
    for (mi, mode) in MODES.iter().enumerate() {
        for (ci, c) in CADENCES.iter().enumerate() {
            let o = evolve(full, 1, *mode, *c, STEPS, DT, true, None);
            temporal[mi][ci] = field_gap(&o.u, &reference.u);
        }
    }
    // Grille conservee pour la synthese : les lois se jugent sur l'ensemble, pas ligne
    // par ligne, et la non-monotonie ne se voit qu'en comparant une ligne a son c = 1.
    let mut grid = [[[0.0f64; 7]; 4]; 3];
    for (mi, mode) in MODES.iter().enumerate() {
        for (ri, r) in RATIOS.iter().enumerate() {
            for (ci, c) in CADENCES.iter().enumerate() {
                let o = evolve(&caches[ri], *r, *mode, *c, STEPS, DT, true, Some(full));
                let eu = field_gap(&o.u, &reference.u) / u_max;
                grid[mi][ri][ci] = eu;
                let (es, et) = (spatial[ri] / u_max, temporal[mi][ci] / u_max);
                let laws = [
                    es + et,
                    (es * es + et * et).sqrt(),
                    es.max(et),
                ];
                let judged = 100.0 * eu > floor
                    && 100.0 * es > floor
                    && 100.0 * et > floor;
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
                println!(
                    "{} | {r} | {c} | {:.3} | {:.4} | {:.4}{cells} | {}",
                    mode.short(),
                    *c as f64 * DT as f64 / t_content,
                    100.0 * o.es_max / s_max,
                    100.0 * eu,
                    if judged { "oui" } else { "plancher" }
                );
                h.write_f32(o.es_max as f32);
                h.write_f32((eu * u_max) as f32);
            }
        }
    }
    println!("\n-- verdict, sur les seules cases jugees ; retenue si tout tient dans [0,80 ; 1,25]");
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


    // -- H1 : espace et temps sont-ils le meme operateur ? ------------------------------
    // Les deux erreurs sont du second ordre en un pas sans dimension : `h/lambda_min` pour
    // l'espace, `v*tau/lambda_min = tau/T` pour le temps. Leur **rapport de constantes** ne
    // depend donc pas du normalisateur choisi : si le contenu effectif est plus lisse que
    // `lambda_min`, les deux constantes sont multipliees par le meme facteur. C'est ce
    // rapport, et lui seul, qui juge H1.
    println!("\n-- le meme verdict, mode par mode");
    println!("mode | additive | quadratique | maximum | loi retenue");
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
            "{} | {:.3}-{:.3} | {:.3}-{:.3} | {:.3}-{:.3} | {}",
            mode.label(),
            per_mode[mi][0].0,
            per_mode[mi][0].1,
            per_mode[mi][1].0,
            per_mode[mi][1].1,
            per_mode[mi][2].0,
            per_mode[mi][2].1,
            if kept.is_empty() { "aucune".into() } else { kept }
        );
    }

    println!("\n-- H1 : constantes du second ordre, et leur rapport (independant du normalisateur)");
    // Mode interpolation, meme operateur que `scatter` : lineaire entre deux echantillons.
    // Moyenne sur les cadences au-dessus du plancher et sous la saturation `tau/T < 1`.
    let mut acc = Vec::new();
    for (ci, c) in CADENCES.iter().enumerate() {
        let st = *c as f64 * DT as f64 / t_content;
        if st < 1.0 && 100.0 * temporal[2][ci] / u_max > floor {
            acc.push((temporal[2][ci] / u_max) / (st * st));
        }
    }
    let a_time = acc.iter().sum::<f64>() / acc.len().max(1) as f64;
    println!(
        "  A_temps (interpolation lineaire en temps) = {a_time:.4} sur {} cadences jugees",
        acc.len()
    );
    println!("r | h/lambda_min | A_espace | A_espace / A_temps | axes interpoles");
    for (ri, r) in RATIOS.iter().enumerate() {
        let ss = *r as f64 * DX / lambda_min;
        if *r == 1 || ss >= 1.0 {
            // `r = 1` n'a pas d'erreur ; `h/lambda_min >= 1` est sature, la loi d'ordre
            // deux n'y a plus cours et sa constante ne veut rien dire.
            println!("{r} | {ss:.4} | - | - | 3 (sature ou nul)");
            continue;
        }
        let a_space = (spatial[ri] / u_max) / (ss * ss);
        println!("{r} | {ss:.4} | {a_space:.4} | {:.2} | 3", a_space / a_time);
    }

    // -- Non-monotonie : degrader la cadence peut-il reduire l'erreur totale ? ----------
    println!("\n-- degrader la cadence reduit-elle l'erreur ? (minimum sur c, par mode et r)");
    println!("mode | r | eU(c=1) % | min sur c % | c du minimum | gain %");
    for (mi, mode) in MODES.iter().enumerate() {
        for (ri, r) in RATIOS.iter().enumerate() {
            let start = grid[mi][ri][0];
            let mut best = (start, 1usize);
            for (ci, c) in CADENCES.iter().enumerate() {
                if grid[mi][ri][ci] < best.0 {
                    best = (grid[mi][ri][ci], *c);
                }
            }
            println!(
                "{} | {r} | {:.4} | {:.4} | {} | {:.2}",
                mode.short(),
                100.0 * start,
                100.0 * best.0,
                best.1,
                if start > 0.0 { 100.0 * (best.0 - start) / start } else { 0.0 }
            );
        }
    }

    println!(
        "\nempreinte des resultats (reception 1, doit se reproduire) : {:#018x}",
        h.finish()
    );
}
