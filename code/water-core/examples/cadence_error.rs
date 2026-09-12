//! S185 : l'erreur de la cadence temporelle en 3D, sur le fournisseur réel.
//! Trois modes de réemploi, dont deux seulement sont disponibles au runtime.
//! Conditions publiées avant exécution : docs/validation/CADENCE-3D-S185.md.
#[path = "support/perturbative_block.rs"]
#[allow(dead_code)]
mod block;
#[path = "support/water_montage.rs"]
#[allow(dead_code)]
mod montage;

use block::{interior_points, load_direct, point, Block, DX, NU};
use montage::{
    event, impact_context, recipe, sea, segments, settings, Arena, Services, CELL, FRAME,
    MAX_SLOPE, T0,
};
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
    Allocator, Background, Hasher64, HostServices, SimTime, WorldPos,
};

const SIDE: usize = 16;
const STEPS: usize = 100;
/// Pas de temps de banc, s, et son équivalent en microsecondes de `SimTime`.
const DT: f32 = 1.0e-2;
const DT_US: u64 = 10_000;
const CADENCES: [usize; 7] = [1, 2, 4, 8, 16, 32, 64];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Hold,
    Extrapolate,
    Interpolate,
}
impl Mode {
    fn label(self) -> &'static str {
        match self {
            Mode::Hold => "maintien",
            Mode::Extrapolate => "extrapolation",
            Mode::Interpolate => "interpolation",
        }
    }
}

/// Source réemployée au pas `n` selon le mode et la cadence. À `c = 1` les trois modes
/// donnent `cache[n]` **exactement** : la fraction vaut zéro, et `a + 0·x == a`.
fn build_source(cache: &[Vec<[f32; 3]>], mode: Mode, c: usize, n: usize, out: &mut [[f32; 3]]) {
    let k = (n / c) * c;
    let frac = (n - k) as f32 / c as f32;
    let a = &cache[k];
    match mode {
        Mode::Hold => out.copy_from_slice(a),
        Mode::Extrapolate => {
            if k >= c {
                let b = &cache[k - c];
                for i in 0..out.len() {
                    for x in 0..3 {
                        out[i][x] = a[i][x] + frac * (a[i][x] - b[i][x]);
                    }
                }
            } else {
                // Avant la deuxième reconstruction, il n'y a rien à extrapoler.
                out.copy_from_slice(a);
            }
        }
        Mode::Interpolate => {
            let b = &cache[k + c];
            for i in 0..out.len() {
                for x in 0..3 {
                    out[i][x] = a[i][x] + frac * (b[i][x] - a[i][x]);
                }
            }
        }
    }
}

struct Outcome {
    /// Champ final, indexé comme le bloc.
    u: Vec<[f32; 3]>,
    /// max |S utilisée − S de référence| sur toutes les mailles et tous les pas.
    es_max: f64,
    /// −∫(S utilisée − S référence) dt, accumulé en f64 par maille intérieure.
    predicted: Vec<[f64; 3]>,
}

fn evolve(
    cache: &[Vec<[f32; 3]>],
    mode: Mode,
    c: usize,
    steps: usize,
    dt: f32,
    at_rest: bool,
) -> Outcome {
    let n = SIDE;
    let mut blk = if at_rest {
        Block::at_rest(n)
    } else {
        Block::new(n)
    };
    let cells = cache[0].len();
    let mut used = vec![[0.0f32; 3]; cells];
    let mut es_max = 0.0f64;
    let mut predicted = vec![[0.0f64; 3]; cells];
    for s in 0..steps {
        build_source(cache, mode, c, s, &mut used);
        let reference = &cache[s];
        for i in 0..cells {
            for x in 0..3 {
                let d = used[i][x] as f64 - reference[i][x] as f64;
                es_max = es_max.max(d.abs());
                predicted[i][x] -= dt as f64 * d;
            }
        }
        load_direct(n, &used, &mut blk);
        blk.step(dt, true);
    }
    Outcome {
        u: blk.u,
        es_max,
        predicted,
    }
}

/// Écart maximal entre deux champs, sur les seules mailles intérieures.
fn field_gap(a: &[[f32; 3]], b: &[[f32; 3]]) -> f64 {
    let n = SIDE;
    let mut m = 0.0f64;
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            for k in 1..n - 1 {
                let c = (i * n + j) * n + k;
                for x in 0..3 {
                    m = m.max((a[c][x] as f64 - b[c][x] as f64).abs());
                }
            }
        }
    }
    m
}
/// Maximum sur un tableau compact (mailles intérieures seules).
fn compact_max(a: &[[f32; 3]]) -> f64 {
    a.iter()
        .flatten()
        .fold(0.0f64, |m, v| m.max((*v as f64).abs()))
}
fn field_max(a: &[[f32; 3]]) -> f64 {
    let n = SIDE;
    let mut m = 0.0f64;
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            for k in 1..n - 1 {
                let c = (i * n + j) * n + k;
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

/// Construit les instantanés de source aux instants `T0 + n·step_us`, `n = 0..=last`.
/// Chaque instantané exige une **actualisation du contrôleur** : la publication est liée
/// à son instant, et c'est un coût par instant, indépendant du nombre de points (S183).
fn snapshots(last: usize, step_us: u64) -> Vec<Vec<[f32; 3]>> {
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
    let mut pool = [const { None }; 1];
    let impacts = Prepared::<64>::build(&journal, &mut pool, impact_context()).unwrap();

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
    let mut controller =
        bound_pressure::Controller::new(pc, &half, &mut pj, T0, &mut active, &mut spare).unwrap();
    let bound = BoundBackground::new(&b, FRAME, CELL);
    let points: Vec<WorldPos> = interior_points(SIDE);
    let mut scratch = vec![DifferentialSample::default(); points.len()];
    let mut samples = scratch.clone();

    let mut out = Vec::with_capacity(last + 1);
    for n in 0..=last {
        let t = SimTime(T0.0 + n as u64 * step_us);
        if t != controller.published_time() {
            controller.update(t).unwrap();
        }
        let pressure = controller.current(t).unwrap();
        differential_world_batch(
            &bound,
            &impacts,
            Some(&pressure),
            t,
            &points,
            MAX_SLOPE,
            &mut scratch,
            &mut samples,
        )
        .unwrap();
        // Rangement **compact**, dans l'ordre de `interior_points` : c'est celui
        // qu'attend `load_direct`. Un rangement par indice de bloc y entrerait décalé.
        let field: Vec<[f32; 3]> = samples
            .iter()
            .map(|s| s.momentum_residual(NU).unwrap())
            .collect();
        out.push(field);
    }
    out
}

fn main() {
    let mut h = Hasher64::new();
    let last = CADENCES
        .iter()
        .map(|c| ((STEPS - 1) / c) * c + c)
        .max()
        .unwrap();
    println!(
        "bloc {SIDE}^3, {} mailles interieures, pas {DX} m ; dt {DT} s, {STEPS} pas = {:.2} s",
        (SIDE - 2).pow(3),
        STEPS as f32 * DT
    );
    println!("instantanes de source construits : {}", last + 1);
    let cache = snapshots(last, DT_US);

    // Amplitude de la source, pour que les erreurs relatives aient un sens absolu.
    let mut s_max = 0.0f64;
    for f in cache.iter().take(STEPS) {
        s_max = s_max.max(compact_max(f));
    }
    let reference = evolve(&cache, Mode::Hold, 1, STEPS, DT, true);
    let u_max = field_max(&reference.u);
    println!(
        "max |S| = {s_max:.6e} m/s2 ; max |u'(T)| de reference = {u_max:.6e} m/s ; \
         tout fini = {}",
        reference.u.iter().flatten().all(|x| x.is_finite())
            && cache.iter().all(|f| f.iter().flatten().all(|x| x.is_finite()))
    );
    h.write_f32(s_max as f32);
    h.write_f32(u_max as f32);

    // Réception 3 : la référence est-elle assez convergée pour servir de référence ?
    let half_cache = snapshots(2 * STEPS, DT_US / 2);
    let half = evolve(&half_cache, Mode::Hold, 1, 2 * STEPS, DT / 2.0, true);
    let drift = field_gap(&reference.u, &half.u);
    println!(
        "reception 3 — reference a dt/2 : ecart {:.4e} m/s, soit {:.3} % de max |u'(T)|",
        drift,
        100.0 * drift / u_max
    );
    h.write_f32(drift as f32);

    println!(
        "\n-- erreur de cadence, etat initial au repos ; eS et eU en % de max |S| et max |u'(T)|"
    );
    println!("mode | c | maintien (ms) | eS % | eU % | ecart a la prediction %");
    for mode in [Mode::Hold, Mode::Extrapolate, Mode::Interpolate] {
        for c in CADENCES {
            let o = evolve(&cache, mode, c, STEPS, DT, true);
            if c == 1 {
                // Réception 2 : à cadence 1 les trois modes sont la référence, en bits.
                assert!(identical(&o.u, &reference.u), "cadence 1 differe, {}", mode.label());
            }
            let eu = field_gap(&o.u, &reference.u);
            // Identité de prédiction : le champ doit s'écarter de l'intégrale de l'erreur
            // de source, à l'advection près — qui est d'ordre supérieur ici.
            let n = SIDE;
            let (mut dev, mut pmax) = (0.0f64, 0.0f64);
            let mut t = 0usize;
            for i in 1..n - 1 {
                for j in 1..n - 1 {
                    for k in 1..n - 1 {
                        let cc = (i * n + j) * n + k;
                        for x in 0..3 {
                            let measured = o.u[cc][x] as f64 - reference.u[cc][x] as f64;
                            dev = dev.max((measured - o.predicted[t][x]).abs());
                            pmax = pmax.max(o.predicted[t][x].abs());
                        }
                        t += 1;
                    }
                }
            }
            println!(
                "{} | {c} | {:.0} | {:.4} | {:.4} | {:.3}",
                mode.label(),
                c as f32 * DT * 1000.0,
                100.0 * o.es_max / s_max,
                100.0 * eu / u_max,
                if pmax > 0.0 { 100.0 * dev / pmax } else { 0.0 }
            );
            h.write_f32(o.es_max as f32);
            h.write_f32(eu as f32);
        }
    }

    println!("\n-- controle a etat initial non nul ; eU en % de max |u'(T) - u'(0)| de reference");
    let ref_live = evolve(&cache, Mode::Hold, 1, STEPS, DT, false);
    let start = Block::new(SIDE).u;
    let mut moved = 0.0f64;
    for c in 0..ref_live.u.len() {
        for x in 0..3 {
            moved = moved.max((ref_live.u[c][x] as f64 - start[c][x] as f64).abs());
        }
    }
    println!("deplacement de reference = {moved:.6e} m/s");
    print!("mode");
    for c in CADENCES {
        print!(" | c={c}");
    }
    println!();
    for mode in [Mode::Hold, Mode::Extrapolate, Mode::Interpolate] {
        print!("{}", mode.label());
        for c in CADENCES {
            let o = evolve(&cache, mode, c, STEPS, DT, false);
            let e = field_gap(&o.u, &ref_live.u);
            print!(" | {:.4}", 100.0 * e / moved);
            h.write_f32(e as f32);
        }
        println!();
    }

    // Coût du mode lui-même : ce que le réemploi ajoute par maille et par pas, hors
    // reconstruction. Ordre de grandeur, pas une mesure au protocole de S184.
    {
        use std::hint::black_box;
        use std::time::Instant;
        let cells = cache[0].len();
        let mut used = vec![[0.0f32; 3]; cells];
        println!("\n-- cout du mode, ns par maille et par pas (hors reconstruction)");
        println!("mode | ns/maille | instantanes a conserver | ko");
        for mode in [Mode::Hold, Mode::Extrapolate, Mode::Interpolate] {
            // Pas 11 à cadence 8 : `k = 8`, donc l'extrapolation extrapole vraiment
            // au lieu de tomber sur sa branche de repli du tout premier intervalle.
            let (c, step) = (8usize, 11usize);
            let warm = Instant::now();
            while warm.elapsed().as_secs_f64() < 0.1 {
                build_source(&cache, mode, c, step, &mut used);
                black_box(used[0]);
            }
            let laps = 20_000;
            let start = Instant::now();
            for _ in 0..laps {
                build_source(black_box(&cache), mode, c, step, &mut used);
                black_box(used[0]);
            }
            let keep = if mode == Mode::Hold { 1 } else { 2 };
            println!(
                "{} | {:.2} | {keep} | {}",
                mode.label(),
                start.elapsed().as_secs_f64() * 1e9 / (laps as f64 * cells as f64),
                keep * cells * 12 / 1024
            );
        }
    }

    println!("\nempreinte des resultats (reception 1, doit se reproduire) : {:#018x}", h.finish());
}
