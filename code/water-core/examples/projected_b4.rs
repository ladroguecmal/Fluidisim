//! S191 : réception du profil B4 sous projection discrète, protocole PROJECTION-B4-S191.
#[path = "support/perturbative_block.rs"]
#[allow(dead_code)]
mod block;
#[path = "support/water_montage.rs"]
#[allow(dead_code)]
mod montage;
#[path = "support/pressure_projection.rs"]
#[allow(dead_code)]
mod projection;
#[path = "support/reuse_mode.rs"]
#[allow(dead_code)]
mod reuse;
#[path = "support/source_snapshots.rs"]
#[allow(dead_code)]
mod snaps;
use block::{
    anchored_indices, d2z_profile, graded_indices, interior_points, load_direct, scatter_indexed,
    Block, DX,
};
use projection::{Projector, Stats};
use reuse::{build_source, Mode};
use snaps::snapshots;
use water_core::Hasher64;
const SIDE: usize = 16;
type Field = Vec<[f32; 3]>;

fn gap(a: &[[f32; 3]], b: &[[f32; 3]]) -> f64 {
    assert_eq!(a.len(), b.len());
    assert!(a.iter().chain(b).flatten().all(|x| x.is_finite()));
    a.iter()
        .flatten()
        .zip(b.iter().flatten())
        .fold(0.0_f64, |m, (x, y)| m.max((*x as f64 - *y as f64).abs()))
}
fn evolve(
    cache: &[Field],
    idx: &[Vec<usize>; 3],
    c: usize,
    steps: usize,
    dt: f32,
    tol: Option<f64>,
) -> (Field, Stats) {
    let mut blk = Block::at_rest(SIDE);
    let mut projector = Projector::new(SIDE, DX).unwrap();
    let mut used = vec![[0.0; 3]; cache[0].len()];
    let mut stats = Stats::default();
    for s in 0..steps {
        build_source(cache, Mode::Extrapolate, c, s, &mut used);
        assert!(used.iter().flatten().all(|x| x.is_finite()));
        if used.len() == 2744 {
            load_direct(SIDE, &used, &mut blk);
        } else {
            scatter_indexed(SIDE, idx, &used, &mut blk);
        }
        blk.step(dt, true);
        if let Some(t) = tol {
            let p = projector.project(&mut blk.u, t, 256).unwrap();
            stats.iterations = stats.iterations.max(p.iterations);
            stats.relative_residual = stats.relative_residual.max(p.relative_residual);
            stats.scaled_divergence = stats.scaled_divergence.max(p.scaled_divergence);
        }
        assert!(blk.u.iter().flatten().all(|x| x.is_finite()));
    }
    (blk.u, stats)
}
fn subset(cache: &[Field], idx: &[Vec<usize>; 3]) -> Vec<Field> {
    cache
        .iter()
        .map(|f| {
            let mut out = Vec::new();
            for &i in &idx[0] {
                for &j in &idx[1] {
                    for &k in &idx[2] {
                        out.push(f[((i - 1) * 14 + j - 1) * 14 + k - 1]);
                    }
                }
            }
            out
        })
        .collect()
}
fn main() {
    let cache = snapshots(&interior_points(SIDE), 99, 10_000);
    let fine = snapshots(&interior_points(SIDE), 199, 5_000);
    let full = [
        anchored_indices(SIDE, 14),
        anchored_indices(SIDE, 14),
        anchored_indices(SIDE, 14),
    ];
    let profile = d2z_profile(SIDE, &cache[0]);
    let original = [
        full[0].clone(),
        full[1].clone(),
        graded_indices(SIDE, &profile, 8),
    ];
    let local = subset(&cache, &original);
    let (old, _) = evolve(&cache, &full, 1, 100, 0.01, None);
    let (old_candidate, _) = evolve(&local, &original, 8, 100, 0.01, None);
    let zero = vec![[0.0; 3]; SIDE.pow(3)];
    let old_m = gap(&old, &zero);
    let old_e = gap(&old_candidate, &old) / old_m;
    assert!((100.0 * old_e - 0.775379).abs() < 0.00001);
    let (old_spatial, _) = evolve(&local, &original, 1, 100, 0.01, None);
    let (old_temporal, _) = evolve(&cache, &full, 8, 100, 0.01, None);
    assert!((100.0 * gap(&old_spatial, &old) / old_m - 0.639282).abs() < 0.00001);
    assert!((100.0 * gap(&old_temporal, &old) / old_m - 0.775379).abs() < 0.00001);
    let (r, sr) = evolve(&cache, &full, 1, 100, 0.01, Some(1e-9));
    let (r2, sr2) = evolve(&fine, &full, 1, 200, 0.005, Some(1e-9));
    let (tight, st) = evolve(&cache, &full, 1, 100, 0.01, Some(1e-12));
    let m = gap(&r, &zero);
    assert!(m.is_finite() && m > 0.0);
    let q = gap(&r, &r2) / m;
    let qp = gap(&r, &tight) / m;
    assert!(qp <= 1e-5);
    println!(
        "temoin S190 e={:.6} % ; M non projete={old_m:.9e}",
        100.0 * old_e
    );
    println!(
        "M projete={m:.9e} ; rapport projete/non projete={:.9}; q={:.9} % ; qP={:.9} %",
        m / old_m,
        100.0 * q,
        100.0 * qp
    );
    let mut hash = Hasher64::new();
    for v in [m, q, qp] {
        hash.write_f32(v as f32);
    }
    let cadences = [1usize, 2, 4, 8, 16];
    let temporal: Vec<_> = cadences
        .iter()
        .map(|&c| evolve(&cache, &full, c, 100, 0.01, Some(1e-9)))
        .collect();
    let mut worst_div = sr
        .scaled_divergence
        .max(sr2.scaled_divergence)
        .max(st.scaled_divergence);
    let mut worst_iter = sr.iterations.max(sr2.iterations).max(st.iterations);
    let mut counts = [0usize; 2];
    let mut best: Option<(usize, f64, String)> = None;
    println!("Nz | c | s % | t % | e % | q+qP % | budget % | e+reserve % | e2 % | residu additif % | e/(s+t) | verdict");
    for nz in [8, 10, 12, 14] {
        let idx = [
            full[0].clone(),
            full[1].clone(),
            if nz == 14 {
                full[2].clone()
            } else {
                graded_indices(SIDE, &profile, nz)
            },
        ];
        println!("indices Nz={nz} : {:?}", idx[2]);
        let sub = subset(&cache, &idx);
        let (sp, _) = evolve(&sub, &idx, 1, 100, 0.01, Some(1e-9));
        let es = gap(&sp, &r) / m;
        for (ci, &c) in cadences.iter().enumerate() {
            let (u, stats) = evolve(&sub, &idx, c, 100, 0.01, Some(1e-9));
            let (tm, ts) = &temporal[ci];
            worst_div = worst_div
                .max(stats.scaled_divergence)
                .max(ts.scaled_divergence);
            worst_iter = worst_iter.max(stats.iterations).max(ts.iterations);
            let et = gap(tm, &r) / m;
            let e = gap(&u, &r) / m;
            let e2 = gap(&u, &r2) / m;
            let res = u
                .iter()
                .flatten()
                .zip(sp.iter().flatten())
                .zip(tm.iter().flatten())
                .zip(r.iter().flatten())
                .fold(0.0_f64, |mx, (((u, s), t), r)| {
                    mx.max((*u as f64 - *s as f64 - *t as f64 + *r as f64).abs())
                })
                / m;
            assert!(e <= es + et + res + 1e-14);
            if c == 1 || nz == 14 {
                assert_eq!(res, 0.0);
            }
            if c == 1 && nz == 14 {
                assert_eq!(gap(&u, &r), 0.0);
            }
            let budget = es + et + q + qp;
            let ok = budget <= 0.02 && e + q + qp <= 0.02 && e2 + qp <= 0.02;
            counts[usize::from(!ok)] += 1;
            let calls = 14 * 14 * idx[2].len() * 100usize.div_ceil(c);
            if ok
                && best.as_ref().map_or(true, |(n, b, _)| {
                    calls < *n || (calls == *n && e + q + qp < *b)
                })
            {
                best = Some((
                    calls,
                    e + q + qp,
                    format!("Nz={nz}, c={c}, indices={:?}", idx[2]),
                ));
            }
            println!("{nz} | {c} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {:.9} | {:.6} | {}",
                100.0*es,100.0*et,100.0*e,100.0*(q+qp),100.0*budget,100.0*(e+q+qp),100.0*e2,100.0*res,
                if es+et>0.0 {e/(es+et)} else {0.0},if ok {"RECU"} else {"REFUSE"});
            for v in [es, et, e, e2, res] {
                hash.write_f32(v as f32);
            }
        }
    }
    println!(
        "projection : pire divergence normalisee={worst_div:.9e} ; iterations max={worst_iter}"
    );
    println!("{} recus / {} refuses", counts[0], counts[1]);
    match best {
        Some((n, b, label)) => println!(
            "profil retenu {label} ; evaluations={n} ; reduction={:.6} ; e+reserve={:.6} %",
            274400.0 / n as f64,
            100.0 * b
        ),
        None => println!("aucun profil recu sous 2 %"),
    };
    println!("empreinte : {:#018x}", hash.finish());
}
