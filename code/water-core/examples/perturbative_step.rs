//! S184 : ce que coûte de **consommer** la source, rapporté au pas qu'elle alimente.
//! Véhicule d'essai, pas le solveur du projet (ADR-007 §5). Paramètres de banc.
//! Conditions publiées avant exécution : docs/validation/CONSOMMATION-S184.md.
use std::{hint::black_box, time::Instant};
use water_core::{
    bound_pressure::{self, Settings},
    gaussian_spectrum::{bake, Recipe},
    impact_field::Medium,
    modal_pressure::Segment,
    prepared_water::{
        mixed::{differential_world_batch, DifferentialSample},
        BoundBackground, Context as ImpactContext, Prepared,
    },
    pressure_journal, pressure_source,
    radial_impact::Domain,
    spectral_pressure::{Node, Slot},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::{Cause, Journal},
    AllocError, AllocStats, Allocator, Background, FrameId, HostServices, JobSystem, SeaState,
    SimTime, Sink, WorldPos,
};

// ------------------------------------------------------------------ hôte

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

// ------------------------------------------------------------------ montage

const FRAME: FrameId = FrameId(7);
const CELL: u64 = 9;
const MAX_SLOPE: f32 = 2.0;
const T0: SimTime = SimTime(1_500_000);
/// Viscosité cinématique de banc, m²/s — ADR-114 n'en fixe aucune par défaut.
const NU: f32 = 1.0e-6;
/// Pas de temps de banc, s. Aucune stabilité explicite revendiquée : on mesure un coût.
const DT: f32 = 1.0e-3;
const DX: f64 = 0.25;
const SIDES: [usize; 3] = [10, 16, 20];
const RATIOS: [usize; 4] = [1, 2, 4, 8];
const CADENCES: [usize; 5] = [1, 2, 4, 8, 16];
const BLOCKS: usize = 7;

fn settings() -> Settings {
    Settings {
        frame: FRAME,
        cell: CELL,
        gravity: 9.81,
        density: 1025.0,
        min: [-8.0; 2],
        max: [12.0; 2],
        start: SimTime(0),
        end: SimTime(8_000_000),
    }
}
fn point(local: [f64; 3]) -> WorldPos {
    WorldPos::from_metres(1e9 + local[0], -1e9 + local[1], local[2])
}
/// Le plus grand indice de maille qu'un réseau atteigne : le nœud supérieur d'un
/// réseau grossier déborde du bloc, et il doit rester dans le domaine comme les autres.
fn reach(n: usize) -> usize {
    RATIOS
        .iter()
        .map(|r| 1 + (nodes_per_axis(n, *r) - 1) * r)
        .max()
        .unwrap()
}
/// Coin du bloc en local, m. Placé pour que **tout** nœud de **tout** réseau vérifie
/// `z <= 0` (exigence de `differential_local`) et reste dans la boîte de pression et
/// le rayon d'impact. Le bloc est donc d'autant plus profond qu'il est grand.
fn base(n: usize) -> [f64; 3] {
    let span = reach(n) as f64 * DX;
    [1.0 - 0.5 * span, 1.0 - 0.5 * span, -0.05 - span]
}
fn cell_local(n: usize, i: usize, j: usize, k: usize) -> [f64; 3] {
    let b = base(n);
    [
        b[0] + i as f64 * DX,
        b[1] + j as f64 * DX,
        b[2] + k as f64 * DX,
    ]
}

/// Bloc de mailles. Les mailles intérieures sont `1..n-1` sur chaque axe ; la couche
/// extérieure est fantôme et n'est jamais évoluée.
struct Block {
    n: usize,
    u: Vec<[f32; 3]>,
    next: Vec<[f32; 3]>,
    /// Source par maille intérieure, rangée dans le même indexage que `u`.
    s: Vec<[f32; 3]>,
}
impl Block {
    fn new(n: usize) -> Self {
        let total = n * n * n;
        // État de départ non nul et déterministe : un pas sur zéro ne mesurerait
        // ni l'advection ni le laplacien.
        let u = (0..total)
            .map(|t| {
                let f = t as f32 * 0.001;
                [
                    0.01 * (f).sin(),
                    0.01 * (1.7 * f).cos(),
                    0.005 * (0.3 * f).sin(),
                ]
            })
            .collect();
        Self {
            n,
            u,
            next: vec![[0.0; 3]; total],
            s: vec![[0.0; 3]; total],
        }
    }
    #[inline]
    fn at(&self, i: usize, j: usize, k: usize) -> usize {
        (i * self.n + j) * self.n + k
    }
    fn interior(&self) -> usize {
        (self.n - 2).pow(3)
    }
    /// Un pas explicite. `with_source` décide si le terme `−S` est appliqué ;
    /// c'est la **seule** différence entre les deux chemins comparés.
    fn step(&mut self, with_source: bool) {
        let n = self.n;
        let inv2 = 0.5 / DX as f32;
        let invsq = 1.0 / (DX * DX) as f32;
        for i in 1..n - 1 {
            for j in 1..n - 1 {
                for k in 1..n - 1 {
                    let c = self.at(i, j, k);
                    let u = self.u[c];
                    let px = self.u[self.at(i + 1, j, k)];
                    let mx = self.u[self.at(i - 1, j, k)];
                    let py = self.u[self.at(i, j + 1, k)];
                    let my = self.u[self.at(i, j - 1, k)];
                    let pz = self.u[self.at(i, j, k + 1)];
                    let mz = self.u[self.at(i, j, k - 1)];
                    let mut out = [0.0f32; 3];
                    for a in 0..3 {
                        let dx = (px[a] - mx[a]) * inv2;
                        let dy = (py[a] - my[a]) * inv2;
                        let dz = (pz[a] - mz[a]) * inv2;
                        let adv = u[0] * dx + u[1] * dy + u[2] * dz;
                        let lap = (px[a] + mx[a] + py[a] + my[a] + pz[a] + mz[a]
                            - 6.0 * u[a])
                            * invsq;
                        let src = if with_source { self.s[c][a] } else { 0.0 };
                        out[a] = u[a] + DT * (-adv + NU * lap - src);
                    }
                    self.next[c] = out;
                }
            }
        }
        core::mem::swap(&mut self.u, &mut self.next);
    }
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

/// Nombre de nœuds par axe pour un pas de réseau `r`, couvrant les mailles intérieures.
fn nodes_per_axis(n: usize, r: usize) -> usize {
    let span = n - 3; // indices intérieurs 1..=n-2, donc `span+1` positions
    span / r + if span % r == 0 { 1 } else { 2 }
}
fn lattice_points(n: usize, r: usize) -> Vec<WorldPos> {
    let m = nodes_per_axis(n, r);
    let mut v = Vec::with_capacity(m * m * m);
    for a in 0..m {
        for b in 0..m {
            for c in 0..m {
                v.push(point(cell_local(n, 1 + a * r, 1 + b * r, 1 + c * r)));
            }
        }
    }
    v
}
fn interior_points(n: usize) -> Vec<WorldPos> {
    let mut v = Vec::with_capacity((n - 2).pow(3));
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            for k in 1..n - 1 {
                v.push(point(cell_local(n, i, j, k)));
            }
        }
    }
    v
}
/// Interpolation trilinéaire des nœuds vers les centres de mailles intérieures.
/// Le coin supérieur est borné : à `r = 1` le poids vaut zéro et la valeur du nœud
/// passe telle quelle, ce qui rend la réception 3 exacte sans cas particulier.
fn scatter(n: usize, r: usize, nodes: &[[f32; 3]], block: &mut Block) {
    let m = nodes_per_axis(n, r);
    let idx = |a: usize, b: usize, c: usize| (a * m + b) * m + c;
    let inv = 1.0 / r as f32;
    for i in 1..n - 1 {
        let (ai, ti) = ((i - 1) / r, ((i - 1) % r) as f32 * inv);
        let ai1 = (ai + 1).min(m - 1);
        for j in 1..n - 1 {
            let (aj, tj) = ((j - 1) / r, ((j - 1) % r) as f32 * inv);
            let aj1 = (aj + 1).min(m - 1);
            for k in 1..n - 1 {
                let (ak, tk) = ((k - 1) / r, ((k - 1) % r) as f32 * inv);
                let ak1 = (ak + 1).min(m - 1);
                let c = (i * n + j) * n + k;
                for a in 0..3 {
                    let g = |x: usize, y: usize, z: usize| nodes[idx(x, y, z)][a];
                    let l = |p: f32, q: f32, t: f32| p + t * (q - p);
                    let x00 = l(g(ai, aj, ak), g(ai1, aj, ak), ti);
                    let x10 = l(g(ai, aj1, ak), g(ai1, aj1, ak), ti);
                    let x01 = l(g(ai, aj, ak1), g(ai1, aj, ak1), ti);
                    let x11 = l(g(ai, aj1, ak1), g(ai1, aj1, ak1), ti);
                    let y0 = l(x00, x10, tj);
                    let y1 = l(x01, x11, tj);
                    block.s[c][a] = l(y0, y1, tk);
                }
            }
        }
    }
}
fn load_direct(n: usize, values: &[[f32; 3]], block: &mut Block) {
    let mut t = 0;
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            for k in 1..n - 1 {
                block.s[(i * n + j) * n + k] = values[t];
                t += 1;
            }
        }
    }
}

// ------------------------------------------------------------------ mesures

#[derive(Clone, Copy, Default)]
struct Times {
    /// ns par maille intérieure et par pas.
    step_only: f64,
    step_with: f64,
    /// µs par maille intérieure, une reconstruction complète.
    source_direct: f64,
    /// µs par maille intérieure, réseau `r` compris interpolation.
    source_r: [f64; 4],
    /// µs par maille et par pas, amorti sur seize pas à la cadence donnée.
    cadence: [f64; 5],
}

struct Montage {
    zeros: usize,
}

fn run(side: usize, reps: u32, receive: bool, m: &mut Montage) -> Times {
    let mut arena = Arena::default();
    let services = Services;
    let b = Background::configure(
        &mut HostServices {
            alloc: &mut arena,
            jobs: &services,
            sink: &services,
        },
        SeaState {
            hs: 0.1,
            tp: 6.0,
            theta_turns: 0.125,
            components: 16,
            graine: 42,
        },
        point([0.0, 0.0, 0.0]),
    )
    .unwrap();
    arena.seal();

    let event = WaveEvent::impact(Impact {
        id: 1,
        frame: FRAME,
        cell: CELL,
        birth: SimTime(0),
        ttl_us: 4_000_000,
        position: [0.0; 3],
        energy_j: 0.01,
        wavelength_m: 4.0,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap();
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
            event,
        )
        .unwrap();
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
    let mut pool = [const { None }; 1];
    let impacts = Prepared::<64>::build(&journal, &mut pool, ctx).unwrap();

    let recipe = Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial: 16,
        angular: 24,
    };
    let mut nodes_pool = [Node::default(); 384];
    let mut half_pool = [Node::default(); 192];
    let full = bake(recipe, &mut nodes_pool).unwrap();
    let half = full.half_into(&mut half_pool).unwrap();
    let base = Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 10.0,
    };
    let paths = [
        [base],
        [Segment {
            birth: SimTime(500_000),
            origin: [1.0, 1.0],
            velocity: [0.0, 2.0],
            pressure_pa: 7.0,
            ..base
        }],
    ];
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
                    recipe,
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
    let mut block = Block::new(n);
    let cells = block.interior() as f64;
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
        receptions(n, &direct, &mut block, m);
    }

    let mut t = Times::default();
    load_direct(n, &direct, &mut block);

    // -- le pas lui-même, avec et sans le terme de source --------------------------
    let passes = reps.max(1);
    let start = Instant::now();
    for _ in 0..passes {
        block.step(black_box(false));
        black_box(block.u[0]);
    }
    t.step_only = start.elapsed().as_secs_f64() * 1e9 / (passes as f64 * cells);
    let start = Instant::now();
    for _ in 0..passes {
        block.step(black_box(true));
        black_box(block.u[0]);
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

    for (idx, r) in RATIOS.iter().enumerate() {
        let pts = lattice_points(n, *r);
        let mut values = Vec::new();
        // Plus le réseau est grossier, moins il coûte : compenser pour garder du signal.
        let laps = (refreshes * (r * r * r)).min(96);
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
            scatter(n, *r, &values, &mut block);
            black_box(block.s[0]);
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
                load_direct(n, &direct, &mut block);
            }
            block.step(true);
            black_box(block.u[0]);
        }
        t.cadence[idx] = start.elapsed().as_secs_f64() * 1e6 / (steps as f64 * cells);
    }
    t
}

/// Les quatre contrôles de §5, exécutés avant tout chronométrage.
fn receptions(n: usize, direct: &[[f32; 3]], block: &mut Block, m: &mut Montage) {
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

    load_direct(n, direct, block);
    // 1 — à nu = 0 et u' = 0, un pas avec source laisse exactement −dt·S.
    let saved = block.u.clone();
    for v in block.u.iter_mut() {
        *v = [0.0; 3];
    }
    let mut zero_nu = Block {
        n,
        u: block.u.clone(),
        next: block.next.clone(),
        s: block.s.clone(),
    };
    zero_nu.step_zero_viscosity();
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
    block.u = saved.clone();
    let mut with_zero = Block {
        n,
        u: saved.clone(),
        next: block.next.clone(),
        s: vec![[0.0; 3]; n * n * n],
    };
    let mut without = Block {
        n,
        u: saved.clone(),
        next: block.next.clone(),
        s: block.s.clone(),
    };
    with_zero.step(true);
    without.step(false);
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
    block.u = saved;
    let mut aligned = Block {
        n,
        u: block.u.clone(),
        next: block.next.clone(),
        s: vec![[0.0; 3]; n * n * n],
    };
    scatter(n, 1, direct, &mut aligned);
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            for k in 1..n - 1 {
                let c = (i * n + j) * n + k;
                for a in 0..3 {
                    assert!(
                        same(aligned.s[c][a], block.s[c][a], m),
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

impl Block {
    /// Variante de `step` à viscosité nulle, pour la réception 1 seulement.
    fn step_zero_viscosity(&mut self) {
        let n = self.n;
        let inv2 = 0.5 / DX as f32;
        for i in 1..n - 1 {
            for j in 1..n - 1 {
                for k in 1..n - 1 {
                    let c = self.at(i, j, k);
                    let u = self.u[c];
                    let px = self.u[self.at(i + 1, j, k)];
                    let mx = self.u[self.at(i - 1, j, k)];
                    let py = self.u[self.at(i, j + 1, k)];
                    let my = self.u[self.at(i, j - 1, k)];
                    let pz = self.u[self.at(i, j, k + 1)];
                    let mz = self.u[self.at(i, j, k - 1)];
                    let mut out = [0.0f32; 3];
                    for a in 0..3 {
                        let dx = (px[a] - mx[a]) * inv2;
                        let dy = (py[a] - my[a]) * inv2;
                        let dz = (pz[a] - mz[a]) * inv2;
                        let adv = u[0] * dx + u[1] * dy + u[2] * dz;
                        out[a] = u[a] + DT * (-adv - self.s[c][a]);
                    }
                    self.next[c] = out;
                }
            }
        }
        core::mem::swap(&mut self.u, &mut self.next);
    }
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
    println!(
        "mailles interieures: {:?}",
        SIDES.map(|n| (n - 2).pow(3))
    );
    println!(
        "noeuds par reseau (n=16): {:?}",
        RATIOS.map(|r| nodes_per_axis(16, r).pow(3))
    );

    let warm = Instant::now();
    while warm.elapsed().as_secs_f64() < 1.0 {
        for side in SIDES {
            black_box(run(side, 4, false, &mut m).step_only);
        }
    }
    let mut results = vec![[Times::default(); BLOCKS]; SIDES.len()];
    for block in 0..BLOCKS {
        for offset in 0..SIDES.len() {
            let i = if block % 2 == 0 {
                offset
            } else {
                SIDES.len() - 1 - offset
            };
            results[i][block] = run(SIDES[i], 40, false, &mut m);
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
    println!("\ncomposantes exemptees par +-0 dans les receptions: {}", m.zeros);
}
