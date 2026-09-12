//! S189 : la composition sur réseau **gradué**, là où les deux pics d'erreur se séparent —
//! et l'épreuve de l'hypothèse d'**additivité locale**. Véhicule d'essai, pas le solveur du
//! projet (ADR-007 §5). Conditions publiées avant exécution :
//! docs/validation/COMPOSITION-GRADUEE-S189.md.
//!
//! S188 expliquait la loi du maximum par la coïncidence des deux maxima, localisés à la
//! **tranche**. Si les deux erreurs culminaient sur la même **maille**, elles s'y
//! ajouteraient et la loi serait l'additive. Ce programme localise à la maille, et mesure
//! l'écart à `Δu(r,c) = Δu(r,1) + Δu(1,c)` point par point.
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
    anchored_indices, base, d2z_profile, graded_indices, interior_points,
    lattice_points_indexed, load_direct, scatter_indexed, Block, DX,
};
use reuse::{build_source, Mode, MODES};
use snaps::snapshots;
use water_core::Hasher64;

const SIDE: usize = 16;
const STEPS: usize = 100;
const DT: f32 = 1.0e-2;
const DT_US: u64 = 10_000;
const CADENCES: [usize; 7] = [1, 2, 4, 8, 16, 32, 64];
/// Nœuds verticaux demandés à la graduation, à horizontale pleine.
const NZ: [usize; 4] = [3, 4, 5, 8];
/// Témoins ancrés uniformes, deux lignes de S188.
const WITNESS: [usize; 2] = [8, 5];

fn at(i: usize, j: usize, k: usize) -> usize {
    (i * SIDE + j) * SIDE + k
}

/// Champ d'écart à la référence, en f64, indexé comme le bloc. Les mailles fantômes
/// restent nulles : elles ne sont jamais évoluées.
fn diff(a: &[[f32; 3]], r: &[[f32; 3]]) -> Vec<[f64; 3]> {
    let mut d = vec![[0.0f64; 3]; a.len()];
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            for k in 1..SIDE - 1 {
                let c = at(i, j, k);
                for x in 0..3 {
                    d[c][x] = a[c][x] as f64 - r[c][x] as f64;
                }
            }
        }
    }
    d
}
/// Maximum d'un champ d'écart, et **la maille** qui le porte, avec la composante.
fn peak(d: &[[f64; 3]]) -> (f64, usize, usize, usize, usize) {
    let mut best = (0.0f64, 0usize, 0usize, 0usize, 0usize);
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            for k in 1..SIDE - 1 {
                let c = at(i, j, k);
                for x in 0..3 {
                    if d[c][x].abs() > best.0 {
                        best = (d[c][x].abs(), i, j, k, x);
                    }
                }
            }
        }
    }
    best
}
/// `max |a − b − c|` sur les mailles intérieures : le résidu d'additivité locale.
fn residual(a: &[[f64; 3]], b: &[[f64; 3]], c: &[[f64; 3]]) -> f64 {
    let mut m = 0.0f64;
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            for k in 1..SIDE - 1 {
                let t = at(i, j, k);
                for x in 0..3 {
                    m = m.max((a[t][x] - b[t][x] - c[t][x]).abs());
                }
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
fn compact_max(a: &[[f32; 3]]) -> f64 {
    a.iter()
        .flatten()
        .fold(0.0f64, |m, v| m.max((*v as f64).abs()))
}
/// Écart maximal restreint à une tranche, pour la continuité avec S187 et S188.
fn layer_peak(d: &[[f64; 3]], k: usize) -> f64 {
    let mut m = 0.0f64;
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            for x in 0..3 {
                m = m.max(d[at(i, j, k)][x].abs());
            }
        }
    }
    m
}

/// Réemploi temporel sur les valeurs de nœuds, puis répandage. `direct` charge sans
/// interpoler, ce qui n'a de sens qu'au réseau plein.
fn evolve(
    cache: &[Vec<[f32; 3]>],
    idx: &[Vec<usize>; 3],
    mode: Mode,
    c: usize,
    direct: bool,
) -> Vec<[f32; 3]> {
    let n = SIDE;
    let mut blk = Block::at_rest(n);
    let mut used = vec![[0.0f32; 3]; cache[0].len()];
    for s in 0..STEPS {
        build_source(cache, mode, c, s, &mut used);
        if direct {
            load_direct(n, &used, &mut blk);
        } else {
            scatter_indexed(n, idx, &used, &mut blk);
        }
        blk.step(DT, true);
    }
    blk.u
}

struct Lattice {
    label: String,
    idx: [Vec<usize>; 3],
    nodes: usize,
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

    // Référence unique, et le profil qui nourrit la graduation, mesuré sur elle.
    let full_cache = snapshots(&interior_points(SIDE), last, DT_US);
    let full_idx = [
        anchored_indices(SIDE, 14),
        anchored_indices(SIDE, 14),
        anchored_indices(SIDE, 14),
    ];
    assert_eq!(
        lattice_points_indexed(SIDE, &full_idx),
        interior_points(SIDE),
        "le reseau plein ancre n'est pas la liste des mailles interieures"
    );
    let mut s_max = 0.0f64;
    for f in full_cache.iter().take(STEPS) {
        s_max = s_max.max(compact_max(f));
    }
    let reference = evolve(&full_cache, &full_idx, Mode::Hold, 1, true);
    let u_max = field_max(&reference);
    println!("max |S| = {s_max:.6e} m/s2 ; max |u'(T)| de reference = {u_max:.6e} m/s");
    h.write_f32(s_max as f32);
    h.write_f32(u_max as f32);

    let profile = d2z_profile(SIDE, &full_cache[0]);

    // -- Les familles ------------------------------------------------------------------
    let mut lattices: Vec<Lattice> = Vec::new();
    lattices.push(Lattice {
        label: "plein".into(),
        idx: full_idx.clone(),
        nodes: 2744,
    });
    for nz in NZ {
        let zi = graded_indices(SIDE, &profile, nz);
        let idx = [
            anchored_indices(SIDE, 14),
            anchored_indices(SIDE, 14),
            zi.clone(),
        ];
        let nodes = idx[0].len() * idx[1].len() * idx[2].len();
        lattices.push(Lattice {
            label: format!("graduee Nz={nz} {:?}", zi),
            idx,
            nodes,
        });
    }
    for m in WITNESS {
        let idx = [
            anchored_indices(SIDE, m),
            anchored_indices(SIDE, m),
            anchored_indices(SIDE, m),
        ];
        let nodes = idx[0].len() * idx[1].len() * idx[2].len();
        lattices.push(Lattice {
            label: format!("ancree {m}^3"),
            idx,
            nodes,
        });
    }
    println!("\nreseaux eprouves :");
    for l in &lattices {
        println!("  {} — {} noeuds", l.label, l.nodes);
    }

    let caches: Vec<Vec<Vec<[f32; 3]>>> = lattices
        .iter()
        .map(|l| {
            if l.label == "plein" {
                full_cache.clone()
            } else {
                snapshots(&lattice_points_indexed(SIDE, &l.idx), last, DT_US)
            }
        })
        .collect();

    // -- Axe spatial seul, et où vit son pic -------------------------------------------
    println!("\n-- axe spatial seul (c=1) : erreur, maille du pic, erreur de la tranche haute");
    println!("reseau | noeuds | eU % | maille du pic (i,j,k,comp) | eU tranche 14 %");
    let mut d_sp: Vec<Vec<[f64; 3]>> = Vec::new();
    let mut sp_peak: Vec<(f64, usize, usize, usize, usize)> = Vec::new();
    for (t, l) in lattices.iter().enumerate() {
        let u = evolve(&caches[t], &l.idx, Mode::Hold, 1, l.label == "plein");
        let d = diff(&u, &reference);
        let p = peak(&d);
        println!(
            "{} | {} | {:.4} | {} | {:.4}",
            l.label,
            l.nodes,
            100.0 * p.0 / u_max,
            if p.0 > 0.0 {
                format!("({},{},{},{})", p.1, p.2, p.3, p.4)
            } else {
                "-".into()
            },
            100.0 * layer_peak(&d, SIDE - 2) / u_max
        );
        h.write_f32(p.0 as f32);
        d_sp.push(d);
        sp_peak.push(p);
    }

    // -- Axe temporel seul --------------------------------------------------------------
    println!("\n-- axe temporel seul (reseau plein) : erreur et maille du pic");
    println!("mode | c | eU % | maille du pic");
    let mut d_tm: Vec<Vec<Vec<[f64; 3]>>> = Vec::new();
    let mut tm_peak = vec![vec![(0.0f64, 0usize, 0usize, 0usize, 0usize); 7]; 3];
    for (mi, mode) in MODES.iter().enumerate() {
        let mut row = Vec::new();
        for (ci, c) in CADENCES.iter().enumerate() {
            let u = evolve(&full_cache, &full_idx, *mode, *c, true);
            let d = diff(&u, &reference);
            let p = peak(&d);
            tm_peak[mi][ci] = p;
            println!(
                "{} | {c} | {:.4} | {}",
                mode.short(),
                100.0 * p.0 / u_max,
                if p.0 > 0.0 {
                    format!("({},{},{},{})", p.1, p.2, p.3, p.4)
                } else {
                    "-".into()
                }
            );
            h.write_f32(p.0 as f32);
            row.push(d);
        }
        d_tm.push(row);
    }

    // -- La grille : additivité locale, géométrie des pics, lois de norme ---------------
    println!("\n-- grille : residu d'additivite locale, puis decomposition au pic composé");
    println!(
        "mode | reseau | c | eU % | pic (i,j,k) | residu % | distance des pics | \
         spatial au pic % | temporel au pic % | add | quad | max | juge"
    );
    let floor = 0.386_f64; // plancher de la référence, S185 §6.1 — même référence.
    let mut worst_res = 0.0f64;
    let mut worst_res_judged = 0.0f64;
    let mut same_cell = (0usize, 0usize);
    let mut law = [[(f64::INFINITY, 0.0f64); 3]; 3];
    // Le même rapport, séparé par **famille** de réseau : attribuer un changement de loi à
    // la géométrie demande de ne pas mélanger deux géométries dans un seul verdict.
    let mut law_fam = vec![[[(f64::INFINITY, 0.0f64); 3]; 3]; 2];
    // Résidu rapporté à l'erreur de sa propre case : un résidu absolu ne dit rien seul.
    let mut rel_res = 0.0f64;
    // Où tombe le pic composé : sur le pic spatial, sur le pic temporel, ou ailleurs ?
    let mut where_peak = [0usize; 3];
    let mut exact_zero = true;
    for (mi, mode) in MODES.iter().enumerate() {
        for (t, l) in lattices.iter().enumerate() {
            for (ci, c) in CADENCES.iter().enumerate() {
                let u = evolve(&caches[t], &l.idx, *mode, *c, l.label == "plein");
                let d = diff(&u, &reference);
                let p = peak(&d);
                let res = residual(&d, &d_sp[t], &d_tm[mi][ci]);
                worst_res = worst_res.max(res / u_max);
                // Réception 4 : à c = 1 ou au réseau plein, l'un des deux termes est nul,
                // donc le résidu doit être exactement zéro.
                if *c == 1 || l.label == "plein" {
                    exact_zero &= res == 0.0;
                }
                let (es, et) = (sp_peak[t].0 / u_max, tm_peak[mi][ci].0 / u_max);
                let eu = p.0 / u_max;
                let laws = [es + et, (es * es + et * et).sqrt(), es.max(et)];
                let judged =
                    100.0 * eu > floor && 100.0 * es > floor && 100.0 * et > floor;
                let mut cells = String::new();
                for (li, q) in laws.iter().enumerate() {
                    let ratio = if *q > 0.0 { eu / q } else { 0.0 };
                    cells.push_str(&format!(" | {ratio:.3}"));
                    if judged {
                        law[mi][li].0 = law[mi][li].0.min(ratio);
                        law[mi][li].1 = law[mi][li].1.max(ratio);
                        let f = usize::from(l.label.starts_with("ancree"));
                        law_fam[f][mi][li].0 = law_fam[f][mi][li].0.min(ratio);
                        law_fam[f][mi][li].1 = law_fam[f][mi][li].1.max(ratio);
                    }
                }
                // Distance entre les deux pics, en mailles, et valeurs locales au pic composé.
                let (a, b) = (sp_peak[t], tm_peak[mi][ci]);
                let dist = if a.0 > 0.0 && b.0 > 0.0 {
                    let (dx, dy, dz) = (
                        a.1 as f64 - b.1 as f64,
                        a.2 as f64 - b.2 as f64,
                        a.3 as f64 - b.3 as f64,
                    );
                    format!("{:.2}", (dx * dx + dy * dy + dz * dz).sqrt())
                } else {
                    "-".into()
                };
                let pc = at(p.1, p.2, p.3);
                let (sp_here, tm_here) = (
                    d_sp[t][pc][p.4].abs() / u_max,
                    d_tm[mi][ci][pc][p.4].abs() / u_max,
                );
                if judged {
                    same_cell.1 += 1;
                    worst_res_judged = worst_res_judged.max(res / u_max);
                    if (a.1, a.2, a.3) == (b.1, b.2, b.3) {
                        same_cell.0 += 1;
                    }
                    rel_res = rel_res.max(res / p.0);
                    let cell = (p.1, p.2, p.3);
                    if cell == (a.1, a.2, a.3) {
                        where_peak[0] += 1;
                    } else if cell == (b.1, b.2, b.3) {
                        where_peak[1] += 1;
                    } else {
                        where_peak[2] += 1;
                    }
                }
                println!(
                    "{} | {} | {c} | {:.4} | ({},{},{}) | {:.4} | {dist} | {:.4} | {:.4}{cells} | {}",
                    mode.short(),
                    l.label.split(' ').take(2).collect::<Vec<_>>().join(" "),
                    100.0 * eu,
                    p.1,
                    p.2,
                    p.3,
                    100.0 * res / u_max,
                    100.0 * sp_here,
                    100.0 * tm_here,
                    if judged { "oui" } else { "plancher" }
                );
                h.write_f32(p.0 as f32);
                h.write_f32(res as f32);
            }
        }
    }

    println!("\n-- reception 4 : residu exactement nul quand un des deux termes est nul : {exact_zero}");
    assert!(exact_zero);
    println!(
        "-- residu d'additivite locale : au plus {:.4} % sur toutes les cases, \
         {:.4} % sur les cases jugees ; plancher de la reference {floor} %",
        100.0 * worst_res,
        100.0 * worst_res_judged
    );
    println!(
        "-- residu rapporte a l'erreur de sa propre case : au plus {:.1} %",
        100.0 * rel_res
    );
    println!(
        "-- les deux pics tombent-ils sur la MEME maille ? {} cases jugees sur {}",
        same_cell.0, same_cell.1
    );
    println!(
        "-- ou tombe le pic compose ? sur le pic spatial {} fois, sur le pic temporel {} fois, ailleurs {} fois (sur {} cases jugees)",
        where_peak[0], where_peak[1], where_peak[2], same_cell.1
    );

    println!("\n-- lois de norme, mode par mode ; entre parentheses ce que S188 retenait");
    println!("mode | additive | quadratique | maximum | retenue S189 | retenue S188");
    const S188_KEPT: [&str; 3] = ["maximum", "maximum", "additive, quadratique"];
    for (mi, mode) in MODES.iter().enumerate() {
        let mut kept = String::new();
        for (li, nom) in ["additive", "quadratique", "maximum"].iter().enumerate() {
            let (lo, hi) = law[mi][li];
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
            law[mi][0].0,
            law[mi][0].1,
            law[mi][1].0,
            law[mi][1].1,
            law[mi][2].0,
            law[mi][2].1,
            if kept.is_empty() { "aucune".into() } else { kept },
            S188_KEPT[mi]
        );
    }


    println!("\n-- lois de norme, FAMILLE par famille : c'est la geometrie qui est en cause");
    println!("famille | mode | additive | quadratique | maximum | retenue");
    for (f, fam) in ["graduee (pics separes)", "ancree (pics au meme etage)"]
        .iter()
        .enumerate()
    {
        for (mi, mode) in MODES.iter().enumerate() {
            let mut kept = String::new();
            for (li, nom) in ["additive", "quadratique", "maximum"].iter().enumerate() {
                let (lo, hi) = law_fam[f][mi][li];
                if lo.is_finite() && lo >= 0.8 && hi <= 1.25 {
                    if !kept.is_empty() {
                        kept.push_str(", ");
                    }
                    kept.push_str(nom);
                }
            }
            println!(
                "{fam} | {} | {:.3}-{:.3} | {:.3}-{:.3} | {:.3}-{:.3} | {}",
                mode.label(),
                law_fam[f][mi][0].0,
                law_fam[f][mi][0].1,
                law_fam[f][mi][1].0,
                law_fam[f][mi][1].1,
                law_fam[f][mi][2].0,
                law_fam[f][mi][2].1,
                if kept.is_empty() { "aucune".into() } else { kept }
            );
        }
    }

    println!(
        "\nempreinte des resultats (reception 1, doit se reproduire) : {:#018x}",
        h.finish()
    );
}
