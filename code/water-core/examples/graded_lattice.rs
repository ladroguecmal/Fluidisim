//! S187 : le réseau d'échantillonnage **gradué en profondeur**. Deux questions, dans
//! l'ordre : quelle part de l'erreur du haut vient de l'axe vertical, puis à erreur égale
//! combien de nœuds un réseau gradué économise-t-il. Véhicule d'essai, pas le solveur du
//! projet (ADR-007 §5). Conditions publiées avant exécution :
//! docs/validation/RESEAU-GRADUE-S187.md.
//!
//! Bloc, réseau, `scatter` et reconstruction viennent tous de `support/` : S184 les a
//! mesurés en coût, S186 en composition, S187 en graduation. Un second exemplaire de l'un
//! d'eux rendrait les trois mesures incomparables (L137).
#[path = "support/perturbative_block.rs"]
#[allow(dead_code)]
mod block;
#[path = "support/water_montage.rs"]
#[allow(dead_code)]
mod montage;
#[path = "support/source_snapshots.rs"]
#[allow(dead_code)]
mod snaps;

use block::{
    anchored_indices, axes_indices, axis_indices, indexed_count, interior_points, lattice_points,
    lattice_points_indexed, load_direct, scatter, scatter_indexed, Block, DX, RATIOS,
};
use snaps::snapshots;
use water_core::Hasher64;

const SIDE: usize = 16;
const STEPS: usize = 100;
const DT: f32 = 1.0e-2;
const DT_US: u64 = 10_000;
/// Nombres de nœuds verticaux demandés à la graduation.
const NZ: [usize; 6] = [3, 4, 5, 6, 8, 14];
/// Pas horizontaux éprouvés avec la graduation. `8` a été **ajouté après la première
/// exécution** : le protocole déclarait `{1, 2, 4}`, et la famille graduée n'avait alors
/// aucun point sous 75 nœuds, ce qui laissait l'isotrope `r = 8` (27 nœuds) sans
/// comparaison possible. L'ajout est déclaré dans le document plutôt que tu.
const RH: [usize; 4] = [1, 2, 4, 8];

fn compact(i: usize, j: usize, k: usize) -> usize {
    let m = SIDE - 2;
    ((i - 1) * m + (j - 1)) * m + (k - 1)
}
fn at(i: usize, j: usize, k: usize) -> usize {
    (i * SIDE + j) * SIDE + k
}

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

/// Évolue le bloc en échantillonnant la source sur le réseau `idx` à **chaque** pas —
/// `c = 1`, l'axe temporel est tenu hors de cette session (S186 a montré qu'il se compose
/// selon le maximum). `uniform` emprunte le `scatter` historique au lieu de la forme
/// générale, ce qui sert la réception 2.
fn evolve(
    cache: &[Vec<[f32; 3]>],
    idx: &[Vec<usize>; 3],
    uniform: Option<usize>,
    reference_cells: Option<&[Vec<[f32; 3]>]>,
    direct: bool,
) -> Outcome {
    let n = SIDE;
    let mut blk = Block::at_rest(n);
    let mut es_max = 0.0f64;
    for s in 0..STEPS {
        if direct {
            load_direct(n, &cache[s], &mut blk);
        } else if let Some(r) = uniform {
            scatter(n, r, &cache[s], &mut blk);
        } else {
            scatter_indexed(n, idx, &cache[s], &mut blk);
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
        blk.step(DT, true);
    }
    Outcome { u: blk.u, es_max }
}

/// Profil vertical et graduation : depuis S189 les deux vivent dans `support/`, parce que
/// S189 pose le même réseau et que deux copies divergeraient (L137). L'empreinte de cette
/// session vérifie que le déplacement n'a rien changé.
fn d2z_profile(field: &[[f32; 3]]) -> Vec<f64> {
    block::d2z_profile(SIDE, field)
}
fn graded_indices(profile: &[f64], want: usize) -> Vec<usize> {
    block::graded_indices(SIDE, profile, want)
}

/// Indices verticaux de la convention **historique** : uniformes de pas `r`, avec un
/// dernier nœud qui **déborde** des mailles intérieures. C'est ce que `lattice_points`
/// fait depuis S184, et il faut le mesurer à part : le nœud de débordement se trouve à
/// une profondeur plus faible, où la source est exponentiellement plus grande. Confondre
/// cet effet d'**ancrage** avec l'effet de **graduation** attribuerait à la règle dérivée
/// ce qui vient d'une convention.
fn overshoot_indices(want: usize) -> Option<Vec<usize>> {
    RATIOS
        .iter()
        .map(|r| axis_indices(SIDE, *r))
        .find(|v| v.len() == want)
}

/// Témoin naïf 1 : indices verticaux uniformes **ancrés**. Depuis S188 la définition vit
/// dans `support/` — les deux sessions doivent poser le même réseau, sinon leurs mesures
/// ne se comparent plus (L137). L'empreinte de cette session le vérifie.
fn uniform_indices(want: usize) -> Vec<usize> {
    anchored_indices(SIDE, want)
}

/// Indices verticaux à pas géométrique de raison 2, fin en haut. Témoin naïf 2 : une
/// graduation plausible mais posée à la main, et non dérivée du contenu.
fn geometric_indices(want: usize) -> Vec<usize> {
    let top = SIDE - 2;
    let mut steps = Vec::new();
    let mut h = 1.0f64;
    for _ in 0..want - 1 {
        steps.push(h);
        h *= 2.0;
    }
    let sum: f64 = steps.iter().sum();
    let scale = (top - 1) as f64 / sum;
    let mut out = vec![1usize];
    let mut acc = 1.0f64;
    // Les pas sont parcourus du bas vers le haut en ordre décroissant : le plus fin
    // atterrit en haut, là où le contenu est le plus court.
    for h in steps.iter().rev() {
        acc += h * scale;
        let cell = acc.round().max(1.0) as usize;
        if out.last() != Some(&cell) && cell <= top {
            out.push(cell);
        }
    }
    if *out.last().unwrap() != top {
        out.push(top);
    }
    out
}

fn label(idx: &[Vec<usize>; 3]) -> String {
    format!(
        "{}x{}x{}",
        idx[0].len(),
        idx[1].len(),
        idx[2].len()
    )
}

fn main() {
    let mut h = Hasher64::new();
    println!(
        "bloc {SIDE}^3, {} mailles interieures, dx {DX} m ; dt {DT} s, {STEPS} pas, c=1",
        (SIDE - 2).pow(3)
    );

    // Référence unique : réseau plein, chargement direct, source reconstruite chaque pas.
    let full = snapshots(&interior_points(SIDE), STEPS - 1, DT_US);
    let mut s_max = 0.0f64;
    for f in full.iter() {
        s_max = s_max.max(compact_max(f));
    }
    let empty: [Vec<usize>; 3] = [vec![], vec![], vec![]];
    let reference = evolve(&full, &empty, None, None, true);
    let u_max = field_max(&reference.u);
    println!("max |S| = {s_max:.6e} m/s2 ; max |u'(T)| de reference = {u_max:.6e} m/s");
    h.write_f32(s_max as f32);
    h.write_f32(u_max as f32);

    // -- Réceptions -------------------------------------------------------------------
    println!("\n-- receptions");
    let mut same_points = true;
    let mut same_field = true;
    for r in RATIOS {
        let idx = axes_indices(SIDE, [r, r, r]);
        same_points &= lattice_points_indexed(SIDE, &idx) == lattice_points(SIDE, r);
        let cache = snapshots(&lattice_points(SIDE, r), STEPS - 1, DT_US);
        let a = evolve(&cache, &idx, Some(r), None, false);
        let b = evolve(&cache, &idx, None, None, false);
        same_field &= identical(&a.u, &b.u);
    }
    println!("2 — le reseau general reproduit l'uniforme : points {same_points}, champ en bits {same_field}");
    assert!(same_points && same_field);

    let mut covers = true;
    for want in NZ {
        for v in [uniform_indices(want), geometric_indices(want)] {
            covers &= v[0] == 1 && *v.last().unwrap() >= SIDE - 2;
        }
    }
    println!("4 — les reseaux verticaux couvrent le bloc : {covers}");
    assert!(covers);

    // -- Q1 : attribution par axe -----------------------------------------------------
    println!("\n-- Q1 : d'ou vient l'erreur ? decimation d'un axe a la fois, c=1");
    println!("variante | r | noeuds | eS % | eU % | eU tranche haute %");
    for r in [2usize, 4, 8] {
        for (nom, axes) in [
            ("verticale", [1, 1, r]),
            ("horizontale", [r, r, 1]),
            ("isotrope", [r, r, r]),
        ] {
            let idx = axes_indices(SIDE, axes);
            let pts = lattice_points_indexed(SIDE, &idx);
            let cache = snapshots(&pts, STEPS - 1, DT_US);
            let o = evolve(&cache, &idx, None, Some(&full), false);
            let eu = field_gap(&o.u, &reference.u);
            let top = layer_gap(&o.u, &reference.u, SIDE - 2);
            println!(
                "{nom} | {r} | {} | {:.4} | {:.4} | {:.4}",
                pts.len(),
                100.0 * o.es_max / s_max,
                100.0 * eu / u_max,
                100.0 * top / u_max
            );
            h.write_f32(o.es_max as f32);
            h.write_f32(eu as f32);
        }
    }

    // -- Le profil vertical, remesuré -------------------------------------------------
    let profile = d2z_profile(&full[0]);
    println!("\n-- profil vertical remesure a T0 : |d2S/dz2| par tranche (m/s2 par m2)");
    println!("k | z (m) | |d2z S| | racine | pas relatif h(z)/h_haut");
    let bz = block::base(SIDE)[2];
    let top_w = profile[profile.len() - 1].sqrt();
    for (t, p) in profile.iter().enumerate() {
        println!(
            "{} | {:.2} | {p:.4e} | {:.4e} | {:.3}",
            t + 1,
            bz + (t + 1) as f64 * DX,
            p.sqrt(),
            top_w / p.sqrt()
        );
    }
    println!(
        "rapport extreme du profil : {:.2}, donc pas vertical profond jusqu'a {:.2} fois celui du haut",
        profile.iter().cloned().fold(0.0f64, f64::max)
            / profile.iter().cloned().fold(f64::INFINITY, f64::min),
        top_w / profile.iter().cloned().fold(f64::INFINITY, f64::min).sqrt()
    );

    // -- Q2 : la courbe erreur contre nombre de nœuds ---------------------------------
    println!("\n-- Q2 : erreur contre nombre de noeuds, c=1 ; famille isotrope puis graduee");
    println!("famille | forme | noeuds | eS % | eU % | eU tranche haute % | indices z");
    let mut rows: Vec<(String, usize, f64)> = Vec::new();

    for r in RATIOS {
        let idx = axes_indices(SIDE, [r, r, r]);
        let pts = lattice_points_indexed(SIDE, &idx);
        let cache = snapshots(&pts, STEPS - 1, DT_US);
        let o = evolve(&cache, &idx, None, Some(&full), false);
        let eu = 100.0 * field_gap(&o.u, &reference.u) / u_max;
        println!(
            "isotrope | r={r} {} | {} | {:.4} | {eu:.4} | {:.4} | uniforme",
            label(&idx),
            pts.len(),
            100.0 * o.es_max / s_max,
            100.0 * layer_gap(&o.u, &reference.u, SIDE - 2) / u_max
        );
        rows.push((format!("isotrope r={r}"), pts.len(), eu));
        h.write_f32(eu as f32);
    }

    for rh in RH {
        for want in NZ {
            let zi = graded_indices(&profile, want);
            let idx = [axis_indices(SIDE, rh), axis_indices(SIDE, rh), zi.clone()];
            let pts = lattice_points_indexed(SIDE, &idx);
            let cache = snapshots(&pts, STEPS - 1, DT_US);
            let o = evolve(&cache, &idx, None, Some(&full), false);
            let eu = 100.0 * field_gap(&o.u, &reference.u) / u_max;
            println!(
                "graduee | rh={rh} Nz={want} {} | {} | {:.4} | {eu:.4} | {:.4} | {:?}",
                label(&idx),
                indexed_count(&idx),
                100.0 * o.es_max / s_max,
                100.0 * layer_gap(&o.u, &reference.u, SIDE - 2) / u_max,
                zi
            );
            rows.push((format!("graduee rh={rh} Nz={want}"), pts.len(), eu));
            h.write_f32(eu as f32);
        }
    }

    // -- Les deux témoins naïfs, à nombre de nœuds verticaux égal ---------------------
    println!("\n-- temoins a Nz egal, pas horizontal 2 : derivee contre uniforme contre geometrique");
    println!("Nz | regle | noeuds | eU % | eU tranche haute % | indices z");
    for want in [3usize, 4, 5, 6, 8] {
        for (nom, zi) in [
            ("derivee", graded_indices(&profile, want)),
            ("uniforme", uniform_indices(want)),
            ("geometrique", geometric_indices(want)),
        ] {
            let idx = [axis_indices(SIDE, 2), axis_indices(SIDE, 2), zi.clone()];
            let pts = lattice_points_indexed(SIDE, &idx);
            let cache = snapshots(&pts, STEPS - 1, DT_US);
            let o = evolve(&cache, &idx, None, None, false);
            let eu = 100.0 * field_gap(&o.u, &reference.u) / u_max;
            println!(
                "{want} | {nom} | {} | {eu:.4} | {:.4} | {:?}",
                pts.len(),
                100.0 * layer_gap(&o.u, &reference.u, SIDE - 2) / u_max,
                zi
            );
            h.write_f32(eu as f32);
        }
    }

    // -- Lecture à ordonnée égale ------------------------------------------------------
    println!("\n-- gain a erreur egale : pour chaque isotrope, le gradue le moins couteux qui fait aussi bien");
    println!("cible | eU cible % | noeuds cible | meilleur gradue | noeuds | eU % | gain de noeuds");
    for (nom, nodes, eu) in rows.iter().filter(|r| r.0.starts_with("isotrope")) {
        let best = rows
            .iter()
            .filter(|(n, _, e)| n.starts_with("graduee") && *e <= *eu)
            .min_by_key(|(_, c, _)| *c);
        match best {
            Some((bn, bc, be)) => println!(
                "{nom} | {eu:.4} | {nodes} | {bn} | {bc} | {be:.4} | {:.1} %",
                100.0 * (*bc as f64 - *nodes as f64) / *nodes as f64
            ),
            None => println!("{nom} | {eu:.4} | {nodes} | aucun | - | - | -"),
        }
    }

    // -- Ancrage contre graduation : séparer deux effets qu'on pourrait confondre -----
    println!("
-- ancrage contre graduation, pas horizontal 2, a nombre de noeuds verticaux egal");
    println!("Nz | convention | noeuds | eU % | eU tranche haute % | indices z");
    for want in [3usize, 5, 8, 14] {
        let mut set: Vec<(&str, Vec<usize>)> = Vec::new();
        if let Some(v) = overshoot_indices(want) {
            set.push(("debordante (historique)", v));
        }
        set.push(("ancree uniforme", uniform_indices(want)));
        set.push(("ancree derivee", graded_indices(&profile, want)));
        for (nom, zi) in set {
            let idx = [axis_indices(SIDE, 2), axis_indices(SIDE, 2), zi.clone()];
            let pts = lattice_points_indexed(SIDE, &idx);
            let cache = snapshots(&pts, STEPS - 1, DT_US);
            let o = evolve(&cache, &idx, None, None, false);
            let eu = 100.0 * field_gap(&o.u, &reference.u) / u_max;
            println!(
                "{want} | {nom} | {} | {eu:.4} | {:.4} | {:?}",
                pts.len(),
                100.0 * layer_gap(&o.u, &reference.u, SIDE - 2) / u_max,
                zi
            );
            h.write_f32(eu as f32);
        }
    }

    // -- L'ancrage vaut-il aussi horizontalement ? ------------------------------------
    // Le nœud de débordement vertical tombe plus haut, où la source est exponentiellement
    // plus grande : d'où son coût. Horizontalement il n'y a pas d'exponentielle, mais il y
    // a le contenu advecté. La question se mesure, elle ne se raisonne pas.
    println!("
-- ancrage horizontal, a nombre de noeuds egal ; vertical tenu plein (14)");
    println!("nx=ny | convention | noeuds | eU % | indices x");
    let zfull = axis_indices(SIDE, 1);
    for want in [3usize, 5, 8] {
        let mut set: Vec<(&str, Vec<usize>)> = Vec::new();
        if let Some(v) = overshoot_indices(want) {
            set.push(("debordante (historique)", v));
        }
        set.push(("ancree uniforme", uniform_indices(want)));
        for (nom, hi) in set {
            let idx = [hi.clone(), hi.clone(), zfull.clone()];
            let pts = lattice_points_indexed(SIDE, &idx);
            let cache = snapshots(&pts, STEPS - 1, DT_US);
            let o = evolve(&cache, &idx, None, None, false);
            println!(
                "{want} | {nom} | {} | {:.4} | {:?}",
                pts.len(),
                100.0 * field_gap(&o.u, &reference.u) / u_max,
                hi
            );
            h.write_f32((field_gap(&o.u, &reference.u) / u_max) as f32);
        }
    }

    let finite = reference.u.iter().flatten().all(|x| x.is_finite());
    println!("\n6 — champ de reference fini : {finite}");
    assert!(finite);
    println!(
        "\nempreinte des resultats (reception 1, doit se reproduire) : {:#018x}",
        h.finish()
    );
}
