//! S190 : réception du budget de source à 2 %, ADR-120 / B4-TOLERANCE-S190.
//! Véhicule de banc sans projection ni surface libre, aucun profil runtime adopté.
#[path = "support/perturbative_block.rs"]
#[allow(dead_code)]
mod block;
#[path = "support/water_montage.rs"]
#[allow(dead_code)]
mod montage;
#[path = "support/reuse_mode.rs"]
#[allow(dead_code)]
mod reuse;
#[path = "support/source_snapshots.rs"]
#[allow(dead_code)]
mod snaps;

use block::{
    anchored_indices, d2z_profile, graded_indices, interior_points, lattice_points_indexed,
    load_direct, scatter_indexed, Block,
};
use reuse::{build_source, Mode};
use snaps::snapshots;
use water_core::Hasher64;

const SIDE: usize = 16;
const STEPS: usize = 100;
const DT: f32 = 0.01;
const LIMIT: f64 = 0.02; // Arbitrage utilisateur, ADR-120 ; pas une constante physique.
const CADENCES: [usize; 7] = [1, 2, 4, 8, 16, 32, 64];
const MODES: [Mode; 2] = [Mode::Hold, Mode::Extrapolate];
type Field = Vec<[f32; 3]>;

fn finite(a: &[[f32; 3]]) -> bool {
    a.iter().flatten().all(|v| v.is_finite())
}

fn gap(a: &[[f32; 3]], b: &[[f32; 3]]) -> f64 {
    assert_eq!(a.len(), SIDE.pow(3));
    assert_eq!(a.len(), b.len());
    assert!(finite(a) && finite(b), "champ non fini avant maximum");
    let mut m = 0.0_f64;
    for i in 1..SIDE - 1 {
        for j in 1..SIDE - 1 {
            for k in 1..SIDE - 1 {
                let p = (i * SIDE + j) * SIDE + k;
                for d in 0..3 {
                    m = m.max((a[p][d] as f64 - b[p][d] as f64).abs());
                }
            }
        }
    }
    m
}

fn accepted(m: f64, spatial: f64, temporal: f64, q: f64, e: f64, e2: f64) -> bool {
    m.is_finite()
        && m > 0.0
        && [spatial, temporal, q, e, e2]
            .iter()
            .all(|v| v.is_finite() && *v >= 0.0)
        && spatial + temporal + q <= LIMIT
        && e + q <= LIMIT
        && e2 <= LIMIT
}

fn evolve(
    cache: &[Field],
    idx: &[Vec<usize>; 3],
    mode: Mode,
    c: usize,
    steps: usize,
    dt: f32,
    direct: bool,
) -> Field {
    let mut blk = Block::at_rest(SIDE);
    let mut used = vec![[0.0; 3]; cache[0].len()];
    for s in 0..steps {
        build_source(cache, mode, c, s, &mut used);
        assert!(finite(&used), "source non finie");
        if direct {
            load_direct(SIDE, &used, &mut blk);
        } else {
            scatter_indexed(SIDE, idx, &used, &mut blk);
        }
        blk.step(dt, true);
        assert!(finite(&blk.u), "etat non fini au pas {s}");
    }
    blk.u
}

/// Les nœuds sont des mailles intérieures : sous-échantillonner les mêmes instantanés
/// évite de recalculer le fournisseur dans le banc, sans changer ses valeurs.
fn subset(cache: &[Field], idx: &[Vec<usize>; 3]) -> Vec<Field> {
    let points = interior_points(SIDE);
    let mut ids = Vec::new();
    for &i in &idx[0] {
        for &j in &idx[1] {
            for &k in &idx[2] {
                ids.push(((i - 1) * (SIDE - 2) + j - 1) * (SIDE - 2) + k - 1);
            }
        }
    }
    assert_eq!(
        ids.iter().map(|&p| points[p]).collect::<Vec<_>>(),
        lattice_points_indexed(SIDE, idx)
    );
    cache
        .iter()
        .map(|f| ids.iter().map(|&p| f[p]).collect())
        .collect()
}

fn main() {
    let full = [
        anchored_indices(SIDE, 14),
        anchored_indices(SIDE, 14),
        anchored_indices(SIDE, 14),
    ];
    let cache = snapshots(&interior_points(SIDE), STEPS - 1, 10_000);
    let fine_cache = snapshots(&interior_points(SIDE), 2 * STEPS - 1, 5_000);
    // Le contrôleur ne doit pas dépendre des instants intermédiaires visités.
    for (s, f) in cache.iter().enumerate() {
        assert!(f
            .iter()
            .flatten()
            .zip(fine_cache[2 * s].iter().flatten())
            .all(|(a, b)| a.to_bits() == b.to_bits()));
    }
    let reference = evolve(&cache, &full, Mode::Hold, 1, STEPS, DT, true);
    let refined = evolve(&fine_cache, &full, Mode::Hold, 1, 2 * STEPS, DT / 2.0, true);
    let zero = vec![[0.0; 3]; SIDE.pow(3)];
    let m = gap(&reference, &zero);
    assert!(m.is_finite() && m > 0.0);
    let q = gap(&reference, &refined) / m;
    assert!((100.0 * q - 0.386).abs() < 0.001, "reference S185 deplacee");

    assert!(accepted(m, LIMIT, 0.0, 0.0, LIMIT, LIMIT));
    assert!(!accepted(m, LIMIT + f64::EPSILON, 0.0, 0.0, LIMIT, LIMIT));
    assert!(!accepted(0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
    for bad in [f64::NAN, f64::INFINITY, -1.0] {
        for axis in 0..5 {
            let mut a = [0.0; 5];
            a[axis] = bad;
            assert!(!accepted(m, a[0], a[1], a[2], a[3], a[4]));
        }
    }
    let omitted = evolve(
        &vec![vec![[0.0; 3]; 2744]; STEPS],
        &full,
        Mode::Hold,
        1,
        STEPS,
        DT,
        true,
    );
    let omission = gap(&omitted, &reference) / m;
    assert_eq!(omission, 1.0);
    assert!(!accepted(
        m,
        0.0,
        0.0,
        q,
        omission,
        gap(&omitted, &refined) / m
    ));

    let profile = d2z_profile(SIDE, &cache[0]);
    let mut lattices = vec![("plein".to_string(), full.clone())];
    for n in [8, 5] {
        lattices.push((
            format!("ancre-{n}x{n}x{n}"),
            [
                anchored_indices(SIDE, n),
                anchored_indices(SIDE, n),
                anchored_indices(SIDE, n),
            ],
        ));
    }
    for nz in [3, 4, 5, 8] {
        lattices.push((
            format!("gradue-14x14x{nz}"),
            [
                full[0].clone(),
                full[1].clone(),
                graded_indices(SIDE, &profile, nz),
            ],
        ));
    }
    for nz in [6, 8] {
        lattices.push((
            format!("mixte-8x8x{nz}"),
            [
                anchored_indices(SIDE, 8),
                anchored_indices(SIDE, 8),
                graded_indices(SIDE, &profile, nz),
            ],
        ));
    }
    let temporal: Vec<Vec<f64>> = MODES
        .iter()
        .map(|&mode| {
            CADENCES
                .iter()
                .map(|&c| {
                    let u = evolve(&cache, &full, mode, c, STEPS, DT, true);
                    if c == 1 {
                        assert!(u
                            .iter()
                            .flatten()
                            .zip(reference.iter().flatten())
                            .all(|(a, b)| a.to_bits() == b.to_bits()));
                    }
                    gap(&u, &reference) / m
                })
                .collect()
        })
        .collect();

    let mut hash = Hasher64::new();
    hash.write_f32(m as f32);
    hash.write_f32(q as f32);
    println!(
        "M = {m:.9e} m/s ; reserve reference = {:.6} % ; seuil = 2 %",
        100.0 * q
    );
    println!("Omission source = {:.6} % : REFUSE", 100.0 * omission);
    println!("reseau | mode | c | noeuds | evaluations | spatial % | temporel % | budget+q % | compose+q % | contre R2 % | verdict");
    let mut counts = [0usize; 2];
    let mut best: Option<(usize, f64, String)> = None;
    for (label, idx) in lattices {
        let nodes: usize = idx.iter().map(Vec::len).product();
        println!("indices {label} : {idx:?}");
        let local = subset(&cache, &idx);
        let direct = label == "plein";
        let spatial = gap(
            &evolve(&local, &idx, Mode::Hold, 1, STEPS, DT, direct),
            &reference,
        ) / m;
        if label == "ancre-8x8x8" {
            assert!((100.0 * spatial - 1.7160).abs() < 0.0001);
        }
        if label == "ancre-5x5x5" {
            assert!((100.0 * spatial - 3.6805).abs() < 0.0001);
        }
        for (mi, &mode) in MODES.iter().enumerate() {
            for (ci, &c) in CADENCES.iter().enumerate() {
                let u = evolve(&local, &idx, mode, c, STEPS, DT, direct);
                let e = gap(&u, &reference) / m;
                let e2 = gap(&u, &refined) / m;
                let t = temporal[mi][ci];
                assert!(e2 <= e + q + 1e-14, "triangulaire incoherente");
                let ok = accepted(m, spatial, t, q, e, e2);
                counts[usize::from(!ok)] += 1;
                let calls = nodes * STEPS.div_ceil(c);
                println!("{label} | {} | {c} | {nodes} | {calls} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {}",
                    mode.short(),100.0*spatial,100.0*t,100.0*(spatial+t+q),100.0*(e+q),
                    100.0*e2,if ok {"RECU"} else {"REFUSE"});
                for v in [spatial, t, e, e2] {
                    hash.write_f32(v as f32);
                }
                if ok
                    && best
                        .as_ref()
                        .map_or(true, |(n, b, _)| calls < *n || (calls == *n && e + q < *b))
                {
                    best = Some((
                        calls,
                        e + q,
                        format!("{label} / {} / c={c} / indices={idx:?}", mode.short()),
                    ));
                }
            }
        }
    }
    assert!(counts[0] > 0 && counts[1] > 0);
    let (calls, budget, label) = best.expect("aucun profil recu");
    assert!(calls < 274400, "aucun profil degrade recu");
    println!(
        "RECEPTION : {} recus / {} refuses ; meilleur profil : {label}",
        counts[0], counts[1]
    );
    println!(
        "Evaluations {calls} contre 274400 ; reduction {:.6} ; compose+reserve {:.6} %",
        274400.0 / calls as f64,
        100.0 * budget
    );
    println!("empreinte : {:#018x}", hash.finish());
}
