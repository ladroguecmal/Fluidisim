//! S184 : ce que coûte de **consommer** la source, rapporté au pas qu'elle alimente.
//! Véhicule d'essai, pas le solveur du projet (ADR-007 §5). Paramètres de banc.
//! Conditions publiées avant exécution : docs/validation/CONSOMMATION-S184.md.
//!
//! S185 a sorti l'hôte, les paramètres de montage et le bloc dans `support/` : le même pas
//! doit servir aux deux sessions, sinon leurs mesures ne se comparent plus (L137).
#[path = "support/perturbative_block.rs"]
#[allow(dead_code)]
mod block;
#[path = "support/water_montage.rs"]
#[allow(dead_code)]
mod montage;

use block::{
    interior_points, lattice_points, load_direct, nodes_per_axis, point, scatter, Block, DX, NU,
    RATIOS,
};
use montage::{event, impact_context, recipe, sea, segments, settings, Arena, Services, CELL,
    FRAME, MAX_SLOPE, T0};
use std::{hint::black_box, time::Instant};
use water_core::{
    bound_pressure,
    gaussian_spectrum::bake,
    prepared_water::{
        mixed::{differential_world_batch, DifferentialSample},
        BoundBackground, Prepared,
    },
    pressure_journal, pressure_source,
    spectral_pressure::{Node, Slot},
    wave_journal::{Cause, Journal},
    AllocStats, Allocator, Background, HostServices, WorldPos,
};

/// Pas de temps de banc, s. Aucune stabilité explicite revendiquée : on mesure un coût.
const DT: f32 = 1.0e-3;
const SIDES: [usize; 3] = [10, 16, 20];
const CADENCES: [usize; 5] = [1, 2, 4, 8, 16];
const BLOCKS: usize = 7;

#[derive(Clone, Copy, Default)]
struct Times {
    step_only: f64,
    step_with: f64,
    source_direct: f64,
    source_r: [f64; 4],
    cadence: [f64; 5],
}

/// Requête de la source aux points donnés, puis contraction en m/s².
/// Une seule transaction : un refus n'écrit rien (ADR-063).
fn fetch(
    bound: &BoundBackground<'_>,
    impacts: &Prepared<'_, '_, 64>,
    pressure: &bound_pressure::Prepared<'_>,
    points: &[WorldPos],
    scratch: &mut Vec<DifferentialSample>,
    samples: &mut Vec<DifferentialSample>,
    out: &mut Vec<[f32; 3]>,
) {
    scratch.resize(points.len(), DifferentialSample::default());
    samples.resize(points.len(), DifferentialSample::default());
    differential_world_batch(
        bound,
        impacts,
        Some(pressure),
        T0,
        points,
        MAX_SLOPE,
        scratch,
        samples,
    )
    .unwrap();
    out.clear();
    for s in samples.iter() {
        out.push(s.momentum_residual(NU).unwrap());
    }
}

struct Montage {
    zeros: usize,
}

fn run(side: usize, reps: u32, receive: bool, m: &mut Montage) -> (Times, AllocStats) {
    let mut arena = Arena::default();
    let services = Services;
    let b = Background::configure(
        &mut HostServices {
            alloc: &mut arena,
            jobs: &services,
            sink: &services,
        },
        sea(16),
        point([0.0, 0.0, 0.0]),
    )
    .unwrap();
    arena.seal();

    let mut records = [None; 1];
    let mut journal = Journal::new(0, &mut records);
    journal
        .confirm(
            0,
            Cause {
                entity: 1,
                command: 1,
                emission: 0,
            },
            event(1),
        )
        .unwrap();
    let ctx = impact_context();
    let mut pool = [const { None }; 1];
    let impacts = Prepared::<64>::build(&journal, &mut pool, ctx).unwrap();

    let r = recipe();
    let mut nodes_pool = [Node::default(); 384];
    let mut half_pool = [Node::default(); 192];
    let full = bake(r, &mut nodes_pool).unwrap();
    let half = full.half_into(&mut half_pool).unwrap();
    let paths = segments();
    let mut sources = [None; 2];
    let mut pj = pressure_journal::Journal::new(0, &mut sources);
    for (i, path) in paths.iter().enumerate() {
        pj.admit_authenticated(
            pressure_source::Source::new(
                pressure_source::Metadata {
                    epoch: 0,
                    id: i as u64,
                    cause: Cause {
                        entity: 2,
                        command: i as u64,
                        emission: 0,
                    },
                    settings: settings(),
                    recipe: r,
                },
                path,
            )
            .unwrap(),
        )
        .unwrap();
    }
    let pc = bound_pressure::Context::new(settings(), &half).unwrap();
    let mut active = [Slot::default(); 192];
    let mut spare = [Slot::default(); 192];
    let controller =
        bound_pressure::Controller::new(pc, &half, &mut pj, T0, &mut active, &mut spare).unwrap();
    let pressure = controller.current(T0).unwrap();
    let bound = BoundBackground::new(&b, FRAME, CELL);

    let n = side;
    let mut blk = Block::new(n);
    let cells = blk.interior() as f64;
    let direct_points = interior_points(n);
    let mut scratch = Vec::new();
    let mut samples = Vec::new();
    let mut direct = Vec::new();
    fetch(
        &bound,
        &impacts,
        &pressure,
        &direct_points,
        &mut scratch,
        &mut samples,
        &mut direct,
    );

    if receive {
        receptions(n, &direct, &mut blk, m);
    }

    let mut t = Times::default();
    load_direct(n, &direct, &mut blk);

    // -- le pas lui-même, avec et sans le terme de source --------------------------
    let passes = reps.max(1);
    let start = Instant::now();
    for _ in 0..passes {
        blk.step(DT, black_box(false));
        black_box(blk.u[0]);
    }
    t.step_only = start.elapsed().as_secs_f64() * 1e9 / (passes as f64 * cells);
    let start = Instant::now();
    for _ in 0..passes {
        blk.step(DT, black_box(true));
        black_box(blk.u[0]);
    }
    t.step_with = start.elapsed().as_secs_f64() * 1e9 / (passes as f64 * cells);

    // -- produire la source, par maille puis décimée --------------------------------
    let refreshes = 3;
    let start = Instant::now();
    for _ in 0..refreshes {
        fetch(
            &bound,
            &impacts,
            &pressure,
            black_box(&direct_points),
            &mut scratch,
            &mut samples,
            &mut direct,
        );
        black_box(direct[0]);
    }
    t.source_direct = start.elapsed().as_secs_f64() * 1e6 / (refreshes as f64 * cells);

    for (idx, ratio) in RATIOS.iter().enumerate() {
        let pts = lattice_points(n, *ratio);
        let mut values = Vec::new();
        // Plus le réseau est grossier, moins il coûte : compenser pour garder du signal.
        let laps = (refreshes * (ratio * ratio * ratio)).min(96);
        let start = Instant::now();
        for _ in 0..laps {
            fetch(
                &bound,
                &impacts,
                &pressure,
                black_box(&pts),
                &mut scratch,
                &mut samples,
                &mut values,
            );
            scatter(n, *ratio, &values, &mut blk);
            black_box(blk.s[0]);
        }
        t.source_r[idx] = start.elapsed().as_secs_f64() * 1e6 / (laps as f64 * cells);
    }

    // -- cadence : seize pas, source reconstruite un pas sur c ----------------------
    for (idx, c) in CADENCES.iter().enumerate() {
        let steps = 16usize;
        let start = Instant::now();
        for s in 0..steps {
            if s % c == 0 {
                fetch(
                    &bound,
                    &impacts,
                    &pressure,
                    black_box(&direct_points),
                    &mut scratch,
                    &mut samples,
                    &mut direct,
                );
                load_direct(n, &direct, &mut blk);
            }
            blk.step(DT, true);
            black_box(blk.u[0]);
        }
        t.cadence[idx] = start.elapsed().as_secs_f64() * 1e6 / (steps as f64 * cells);
    }
    (t, arena.stats())
}

/// Les quatre contrôles de §5, exécutés avant tout chronométrage.
fn receptions(n: usize, direct: &[[f32; 3]], blk: &mut Block, m: &mut Montage) {
    let same = |a: f32, b: f32, m: &mut Montage| {
        if a.to_bits() == b.to_bits() {
            return true;
        }
        if a == 0.0 && b == 0.0 {
            m.zeros += 1;
            return true;
        }
        false
    };
    // 4 — tout est fini.
    assert!(direct.iter().flatten().all(|x| x.is_finite()));

    load_direct(n, direct, blk);
    // 1 — à nu = 0 et u' = 0, un pas avec source laisse exactement −dt·S.
    let saved = blk.u.clone();
    let mut zero_nu = Block {
        n,
        u: vec![[0.0; 3]; n * n * n],
        next: blk.next.clone(),
        s: blk.s.clone(),
    };
    zero_nu.advance(DT, true, 0.0);
    let mut t = 0;
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            for k in 1..n - 1 {
                let c = (i * n + j) * n + k;
                for a in 0..3 {
                    let expected = -(DT * direct[t][a]);
                    assert!(
                        same(zero_nu.u[c][a], expected, m),
                        "reception 1: {} != {}",
                        zero_nu.u[c][a],
                        expected
                    );
                }
                t += 1;
            }
        }
    }
    // 2 — source forcée à zéro : le chemin « avec source » redevient le chemin « sans ».
    let mut with_zero = Block {
        n,
        u: saved.clone(),
        next: blk.next.clone(),
        s: vec![[0.0; 3]; n * n * n],
    };
    let mut without = Block {
        n,
        u: saved.clone(),
        next: blk.next.clone(),
        s: blk.s.clone(),
    };
    with_zero.step(DT, true);
    without.step(DT, false);
    for c in 0..n * n * n {
        for a in 0..3 {
            assert_eq!(
                with_zero.u[c][a].to_bits(),
                without.u[c][a].to_bits(),
                "reception 2"
            );
        }
    }
    // 3 — le réseau à r = 1 reproduit la source par maille, en bits.
    let mut aligned = Block {
        n,
        u: saved,
        next: blk.next.clone(),
        s: vec![[0.0; 3]; n * n * n],
    };
    scatter(n, 1, direct, &mut aligned);
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            for k in 1..n - 1 {
                let c = (i * n + j) * n + k;
                for a in 0..3 {
                    assert!(
                        same(aligned.s[c][a], blk.s[c][a], m),
                        "reception 3 en ({i},{j},{k})"
                    );
                }
            }
        }
    }
    println!(
        "receptions n={n}: source atteint l'etat, source nulle rejoint le chemin sans source, \
         reseau r=1 identique en bits, tout fini ({} composantes exemptees par +-0)",
        m.zeros
    );
}

fn median(v: &mut [f64]) -> (f64, f64, f64) {
    v.sort_by(f64::total_cmp);
    (v[0], v[v.len() / 2], v[v.len() - 1])
}

fn main() {
    let mut m = Montage { zeros: 0 };
    for side in SIDES {
        run(side, 1, true, &mut m);
    }
    println!("mailles interieures: {:?}", SIDES.map(|n| (n - 2).pow(3)));
    println!(
        "noeuds par reseau (n=16): {:?}",
        RATIOS.map(|r| nodes_per_axis(16, r).pow(3))
    );
    println!("pas du bloc {DX} m, viscosite {NU} m2/s, dt {DT} s");

    let warm = Instant::now();
    while warm.elapsed().as_secs_f64() < 1.0 {
        for side in SIDES {
            black_box(run(side, 4, false, &mut m).0.step_only);
        }
    }
    let mut results = vec![[Times::default(); BLOCKS]; SIDES.len()];
    let mut stats = vec![AllocStats::default(); SIDES.len()];
    for block in 0..BLOCKS {
        for offset in 0..SIDES.len() {
            let i = if block % 2 == 0 {
                offset
            } else {
                SIDES.len() - 1 - offset
            };
            let (t, s) = run(SIDES[i], 40, false, &mut m);
            results[i][block] = t;
            stats[i] = s;
        }
    }

    println!("\n-- le pas, ns par maille interieure (min/med/max sur {BLOCKS} blocs)");
    println!("cote | mailles | sans source | avec source");
    for (i, n) in SIDES.iter().enumerate() {
        let pick = |f: fn(&Times) -> f64| {
            let mut v: Vec<f64> = results[i].iter().map(f).collect();
            median(&mut v)
        };
        let a = pick(|t| t.step_only);
        let b = pick(|t| t.step_with);
        println!(
            "{n} | {} | {:.2}/{:.2}/{:.2} | {:.2}/{:.2}/{:.2}",
            (n - 2).pow(3),
            a.0,
            a.1,
            a.2,
            b.0,
            b.1,
            b.2
        );
    }

    println!("\n-- produire la source, us par maille interieure (mediane)");
    print!("cote | par maille");
    for r in RATIOS {
        print!(" | reseau r={r}");
    }
    println!(" | part du pas avec source");
    for (i, n) in SIDES.iter().enumerate() {
        let pick = |f: fn(&Times) -> f64| {
            let mut v: Vec<f64> = results[i].iter().map(f).collect();
            median(&mut v).1
        };
        let direct = pick(|t| t.source_direct);
        let step = pick(|t| t.step_with) / 1000.0;
        print!("{n} | {direct:.3}");
        for k in 0..RATIOS.len() {
            let v = match k {
                0 => pick(|t| t.source_r[0]),
                1 => pick(|t| t.source_r[1]),
                2 => pick(|t| t.source_r[2]),
                _ => pick(|t| t.source_r[3]),
            };
            print!(" | {v:.3}");
        }
        println!(" | {:.4} %", 100.0 * step / (step + direct));
    }

    println!("\n-- cadence, us par maille et par pas, amorti sur seize pas (mediane)");
    print!("cote");
    for c in CADENCES {
        print!(" | c={c}");
    }
    println!();
    for (i, n) in SIDES.iter().enumerate() {
        print!("{n}");
        for k in 0..CADENCES.len() {
            let mut v: Vec<f64> = results[i].iter().map(|t| t.cadence[k]).collect();
            print!(" | {:.3}", median(&mut v).1);
        }
        println!();
    }
    println!("\n-- allocations hote par montage, Background::configure puis seal()");
    for (i, n) in SIDES.iter().enumerate() {
        println!(
            "cote {n} | octets={} appels={} refusees_apres_seal={}",
            stats[i].persistent_bytes, stats[i].persistent_calls, stats[i].refused_after_seal
        );
    }
    println!(
        "\ncomposantes exemptees par +-0 dans les receptions: {}",
        m.zeros
    );
}
