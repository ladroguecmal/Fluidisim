//! S184 P5 : ce qu'une évaluation **par réseau** pourrait retirer au coût par point.
//! Mesure la part « phase + trigonométrie » de B et ce qu'une récurrence la rendrait.
//! **Ce n'est pas une proposition** : une récurrence ne rend pas les mêmes bits (I-03),
//! et elle dérive le long d'une ligne. On chiffre une occasion, on ne l'adopte pas.
use std::{hint::black_box, time::Instant};
use water_core::{
    background_spectrum::{bake, Recipe},
    impact_field::Medium,
    prepared_water::{
        mixed::{differential_world_batch, DifferentialSample},
        BoundBackground, Context as ImpactContext, Prepared,
    },
    radial_impact::Domain,
    wave_journal::Journal,
    AllocError, AllocStats, Allocator, Background, Component, FrameId, HostServices, JobSystem,
    PhaseQ32, SeaState, SimTime, Sink, WorldPos,
};

#[derive(Default)]
struct Arena {
    stats: AllocStats,
    sealed: bool,
}
impl Allocator for Arena {
    fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AllocError> {
        if self.sealed {
            self.stats.refused_after_seal += 1;
            return Err(AllocError::Sealed);
        }
        self.stats.persistent_bytes += bytes;
        self.stats.persistent_calls += 1;
        Ok(0)
    }
    fn seal(&mut self) {
        self.sealed = true;
    }
    fn is_sealed(&self) -> bool {
        self.sealed
    }
    fn stats(&self) -> AllocStats {
        self.stats
    }
}
struct Services;
impl Sink for Services {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Services {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        _: usize,
        r: &dyn Fn(usize, usize) -> f64,
        m: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        m(init, r(0, n))
    }
}

const FRAME: FrameId = FrameId(7);
const CELL: u64 = 9;
const M: usize = 14; // côté du réseau, 2744 nœuds — celui du bloc n=16 de P3
const DX: f64 = 0.25;
const T: SimTime = SimTime(1_500_000);
const BLOCKS: usize = 9;

fn recipe(components: usize) -> Recipe {
    Recipe {
        sea: SeaState {
            hs: 0.1,
            tp: 6.0,
            theta_turns: 0.125,
            components,
            graine: 42,
        },
        gravity: 9.81,
        gamma: 3.3,
        min_ratio: 0.5,
        max_ratio: 4.0,
        spread_turns: 0.08,
    }
}
fn node(i: usize, j: usize, k: usize) -> WorldPos {
    WorldPos::from_metres(
        1e9 + i as f64 * DX,
        -1e9 + j as f64 * DX,
        -1.0 - k as f64 * DX,
    )
}
fn lattice() -> Vec<WorldPos> {
    let mut v = Vec::with_capacity(M * M * M);
    for k in 0..M {
        for j in 0..M {
            for i in 0..M {
                v.push(node(i, j, k));
            }
        }
    }
    v
}

/// Phase et trigonométrie **par point**, comme les évalue une requête ponctuelle :
/// la phase temporelle est sortie de la boucle, le reste est payé à chaque nœud.
fn per_point(components: &[Component], acc: &mut [f32; 2]) {
    let temporal: Vec<PhaseQ32> = components
        .iter()
        .map(|c| PhaseQ32::from_time(c.freq_q32, T).wrapping_add(c.phase0))
        .collect();
    for k in 0..M {
        for j in 0..M {
            for i in 0..M {
                let x = i as f32 * DX as f32;
                let y = j as f32 * DX as f32;
                let _ = k;
                for (c, t) in components.iter().zip(&temporal) {
                    let d = x * c.dir[0] + y * c.dir[1];
                    let (s, co) = PhaseQ32::from_distance(c.k_turns_per_m, d)
                        .wrapping_add(*t)
                        .sin_cos();
                    acc[0] += c.amplitude * co;
                    acc[1] += c.amplitude * s;
                }
            }
        }
    }
}

/// La même chose par **récurrence le long de l'axe x** : une rotation par nœud et par
/// composante, la phase n'étant recalculée qu'en tête de ligne.
fn per_lattice(components: &[Component], acc: &mut [f32; 2]) {
    let temporal: Vec<PhaseQ32> = components
        .iter()
        .map(|c| PhaseQ32::from_time(c.freq_q32, T).wrapping_add(c.phase0))
        .collect();
    // Rotation d'un pas de réseau, une fois pour tout le réseau.
    let rot: Vec<(f32, f32)> = components
        .iter()
        .map(|c| PhaseQ32::from_distance(c.k_turns_per_m, DX as f32 * c.dir[0]).sin_cos())
        .collect();
    let mut state: Vec<(f32, f32)> = vec![(0.0, 0.0); components.len()];
    for k in 0..M {
        for j in 0..M {
            let y = j as f32 * DX as f32;
            for (n, (c, t)) in components.iter().zip(&temporal).enumerate() {
                let d = y * c.dir[1];
                state[n] = PhaseQ32::from_distance(c.k_turns_per_m, d)
                    .wrapping_add(*t)
                    .sin_cos();
            }
            let _ = k;
            for _i in 0..M {
                for (n, c) in components.iter().enumerate() {
                    let (s, co) = state[n];
                    acc[0] += c.amplitude * co;
                    acc[1] += c.amplitude * s;
                    let (sd, cd) = rot[n];
                    state[n] = (s * cd + co * sd, co * cd - s * sd);
                }
            }
        }
    }
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn main() {
    let points = lattice();
    let nodes = points.len() as f64;
    println!("reseau {M}^3 = {} noeuds, pas {DX} m", points.len());
    println!("composantes | B differentiel ns/noeud | phase+trigo par point ns/noeud | par reseau ns/noeud | part trigo | plafond du gain");

    // 32 est le minimum du profil V1, et la valeur retenue par B1 (COMPOSANTES_B1).
    for count in [32usize, 64, 128] {
        let cooked = bake(recipe(count)).unwrap();
        let mut arena = Arena::default();
        let services = Services;
        let b = Background::from_spectrum(
            &mut HostServices {
                alloc: &mut arena,
                jobs: &services,
                sink: &services,
            },
            &cooked,
            WorldPos::from_metres(1e9, -1e9, 0.0),
        )
        .unwrap();
        arena.seal();
        let components = cooked.components().to_vec();
        assert_eq!(components.len(), count);

        // B seul : journal vide, aucune pression. Même chemin que le montage « 0 impact ».
        let mut records: [Option<_>; 0] = [];
        let journal: Journal<'_> = Journal::new(0, &mut records);
        let mut pool: [Option<water_core::radial_impact::RadialImpact<64>>; 0] = [];
        let ctx = ImpactContext {
            frame: FRAME,
            cell: CELL,
            medium: Medium {
                gravity: 9.81,
                density: 1025.0,
                depth: 20.0,
                max_slope: 0.1,
            },
            domain: Domain {
                radius: 16.0,
                age_us: 4_000_000,
            },
        };
        let impacts = Prepared::<64>::build(&journal, &mut pool, ctx).unwrap();
        let bound = BoundBackground::new(&b, FRAME, CELL);
        let mut scratch = vec![DifferentialSample::default(); points.len()];
        let mut out = scratch.clone();

        let mut full = Vec::new();
        let mut point_k = Vec::new();
        let mut lat_k = Vec::new();
        let warm = Instant::now();
        while warm.elapsed().as_secs_f64() < 0.3 {
            let mut a = [0.0f32; 2];
            per_point(&components, &mut a);
            black_box(a);
        }
        for block in 0..BLOCKS {
            let order = block % 2 == 0;
            let mut run_full = || {
                let start = Instant::now();
                differential_world_batch(
                    &bound,
                    &impacts,
                    None,
                    T,
                    black_box(&points),
                    2.0,
                    &mut scratch,
                    &mut out,
                )
                .unwrap();
                black_box(out[0].water.eta);
                start.elapsed().as_secs_f64() * 1e9 / nodes
            };
            let run_point = || {
                let mut a = [0.0f32; 2];
                let start = Instant::now();
                per_point(black_box(&components), &mut a);
                let d = start.elapsed().as_secs_f64() * 1e9 / nodes;
                black_box(a);
                d
            };
            let run_lat = || {
                let mut a = [0.0f32; 2];
                let start = Instant::now();
                per_lattice(black_box(&components), &mut a);
                let d = start.elapsed().as_secs_f64() * 1e9 / nodes;
                black_box(a);
                d
            };
            if order {
                full.push(run_full());
                point_k.push(run_point());
                lat_k.push(run_lat());
            } else {
                lat_k.push(run_lat());
                point_k.push(run_point());
                full.push(run_full());
            }
        }
        let f = median(&mut full);
        let p = median(&mut point_k);
        let l = median(&mut lat_k);
        println!(
            "{count} | {f:.1} | {p:.1} | {l:.1} | {:.1} % | {:.1} %",
            100.0 * p / f,
            100.0 * (p - l) / f
        );
    }

    // Ce que contient la source, et donc ce que la décimation a le droit d'ignorer.
    let k_max = 6.0f64 * (15.5 / 16.0); // (i+0.5)·cutoff/radial, i=15 — gaussian_spectrum::bake
    let lam = std::f64::consts::TAU / k_max;
    println!("\ncoupure de pression k_max={k_max:.4} rad/m, lambda_min={lam:.4} m");
    print!("points par lambda_min selon le pas de reseau :");
    for r in [1usize, 2, 4, 8] {
        print!(" H={:.2}m -> {:.2}", r as f64 * DX, lam / (r as f64 * DX));
    }
    println!();
    println!(
        "impact lambda=4 m : {}",
        [1usize, 2, 4, 8]
            .map(|r| format!("H={:.2}m -> {:.1}", r as f64 * DX, 4.0 / (r as f64 * DX)))
            .join(" ; ")
    );
}
