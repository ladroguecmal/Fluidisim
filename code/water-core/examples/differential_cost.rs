//! S183 : coût complet du consommateur différentiel, contre le chemin de surface aux mêmes
//! entrées. Paramètres de banc, pas paramètres gameplay ; aucun budget cible.
//! Conditions de mesure publiées avant exécution : docs/validation/COUT-DIFFERENTIEL-S183.md.
use std::{hint::black_box, mem::size_of, time::Instant};
use water_core::{
    bound_pressure::{self, Settings},
    gaussian_spectrum::{bake, Recipe},
    impact_field::Medium,
    modal_pressure::Segment,
    prepared_water::{
        mixed::{differential_world_batch, sample_world_batch, DifferentialSample},
        BoundBackground, Context as ImpactContext, Prepared,
    },
    pressure_journal, pressure_source,
    radial_impact::{Domain, RadialImpact},
    spectral_pressure::{Node, Slot},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::{Cause, Journal},
    AllocError, AllocStats, Allocator, Background, FrameId, HostServices, JobSystem, SeaState,
    SimTime, Sink, WaterSample, WorldPos,
};

// ------------------------------------------------------------------ hôte instrumenté

/// I-06 devient mécanique par `seal()` : après scellement, toute demande est refusée
/// **et comptée**, au lieu d'être servie en silence (`host.rs`).
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
const NODES_MAX: usize = 24 * 32;
const SLOTS_MAX: usize = NODES_MAX / 2;
const IMPACTS_MAX: usize = 4;
const LOT_MAX: usize = 256;
/// Aucun montage ne doit refuser pour la pente pendant un chronométrage : le contrôle est
/// une comparaison O(1) identique sur les deux chemins, et les refus ont leur propre table.
const MAX_SLOPE: f32 = 2.0;
/// Instant publié de référence, dans la fenêtre 0 → 8 s et sous l'horizon d'impact 4 s.
const T0: SimTime = SimTime(1_500_000);
const T1: SimTime = SimTime(2_500_000);
const LOTS: [usize; 4] = [1, 8, 64, 256];
const BLOCKS: usize = 15;

#[derive(Clone, Copy)]
struct Case {
    label: &'static str,
    components: usize,
    radial: usize,
    angular: usize,
    impacts: usize,
}
const REFERENCE: Case = Case {
    label: "reference",
    components: 16,
    radial: 16,
    angular: 24,
    impacts: 1,
};

fn medium() -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: 20.0,
        max_slope: 0.1,
    }
}
fn impact_context() -> ImpactContext {
    ImpactContext {
        frame: FRAME,
        cell: CELL,
        medium: medium(),
        domain: Domain {
            radius: 16.0,
            age_us: 4_000_000,
        },
    }
}
/// Impacts distincts mais de même énergie : c'est leur nombre qu'on mesure, pas leur variété.
fn event(id: u64) -> WaveEvent {
    let k = id as f32;
    WaveEvent::impact(Impact {
        id,
        frame: FRAME,
        cell: CELL,
        birth: SimTime(100_000 * id),
        ttl_us: 4_000_000,
        position: [1.5 * (k - 2.0), -1.5 * (k - 2.0), 0.0],
        energy_j: 0.01,
        wavelength_m: 4.0,
        direction_turns: 0.125 * k,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap()
}
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
fn sea(components: usize) -> SeaState {
    SeaState {
        hs: 0.1,
        tp: 6.0,
        theta_turns: 0.125,
        components,
        graine: 42,
    }
}
fn anchor() -> WorldPos {
    WorldPos::from_metres(1e9, -1e9, 0.0)
}
/// Séquence fixe ; un lot de `n` points en prend le préfixe, pour que l'axe « lot » ne
/// change que le nombre de points et jamais lesquels.
fn all_points() -> [WorldPos; LOT_MAX] {
    std::array::from_fn(|i| {
        let r = 6.0 * ((i % 64) as f64 + 0.5) / 64.0;
        let a = i as f64 * 2.399_963;
        WorldPos::from_metres(1e9 + r * a.cos(), -1e9 + r * a.sin(), -0.5 * (i % 4) as f64)
    })
}

#[derive(Clone, Copy, Default)]
struct Times {
    impact_new: f64,
    prepared_build: f64,
    bake: f64,
    context_new: f64,
    controller_new: f64,
    update: f64,
    surface: [f64; 4],
    differential: [f64; 4],
    refuse_time: f64,
    refuse_context: f64,
    refuse_capacity: f64,
    refuse_point_last: f64,
    refuse_point_first: f64,
    /// Lot 64 sans pression : attribue le coût entre les couches.
    surface_np: f64,
    differential_np: f64,
}

fn us(start: Instant, n: f64) -> f64 {
    start.elapsed().as_secs_f64() * 1e6 / n
}

fn run(case: Case, reps: u32) -> (Times, AllocStats, usize) {
    let mut arena = Arena::default();
    let services = Services;
    let b = Background::configure(
        &mut HostServices {
            alloc: &mut arena,
            jobs: &services,
            sink: &services,
        },
        sea(case.components),
        anchor(),
    )
    .unwrap();
    // Tout ce qui suit se passe système scellé : plus une seule allocation d'hôte n'est due.
    arena.seal();

    let mut records = [None; IMPACTS_MAX];
    let mut journal = Journal::new(0, &mut records);
    for id in 1..=case.impacts as u64 {
        journal
            .confirm(
                0,
                Cause {
                    entity: 1,
                    command: id,
                    emission: 0,
                },
                event(id),
            )
            .unwrap();
    }
    let ctx = impact_context();
    let recipe = Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial: case.radial,
        angular: case.angular,
    };
    let mut nodes = [Node::default(); NODES_MAX];
    let mut half_nodes = [Node::default(); SLOTS_MAX];
    let full = bake(recipe, &mut nodes).unwrap();
    let half = full.half_into(&mut half_nodes).unwrap();
    let slots = half.nodes().len();

    let mut sources = [None; 2];
    let mut pj = pressure_journal::Journal::new(0, &mut sources);
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

    let mut t = Times::default();
    let points = all_points();
    let bound = BoundBackground::new(&b, FRAME, CELL);

    // -- préparation ----------------------------------------------------------------
    {
        let start = Instant::now();
        for _ in 0..reps {
            black_box(
                RadialImpact::<64>::new(black_box(event(1)), black_box(ctx.medium), ctx.domain)
                    .unwrap(),
            );
        }
        t.impact_new = us(start, reps as f64);
    }
    {
        let mut pool = [const { None }; IMPACTS_MAX];
        let start = Instant::now();
        for _ in 0..reps {
            black_box(Prepared::<64>::build(&journal, &mut pool, ctx).unwrap());
        }
        t.prepared_build = us(start, reps as f64);
    }
    {
        let mut scratch = [Node::default(); NODES_MAX];
        let mut scratch_half = [Node::default(); SLOTS_MAX];
        let start = Instant::now();
        for _ in 0..reps {
            let f = bake(black_box(recipe), &mut scratch).unwrap();
            black_box(f.half_into(&mut scratch_half).unwrap().nodes().len());
        }
        t.bake = us(start, reps as f64);
    }
    {
        let start = Instant::now();
        for _ in 0..reps {
            black_box(bound_pressure::Context::new(black_box(settings()), &half).unwrap());
        }
        t.context_new = us(start, reps as f64);
    }
    let mut active = [Slot::default(); SLOTS_MAX];
    let mut spare = [Slot::default(); SLOTS_MAX];
    {
        let start = Instant::now();
        for _ in 0..reps {
            let c = bound_pressure::Controller::new(pc, &half, &mut pj, T0, &mut active, &mut spare)
                .unwrap();
            black_box(c.published_time());
        }
        t.controller_new = us(start, reps as f64);
    }

    let mut controller =
        bound_pressure::Controller::new(pc, &half, &mut pj, T0, &mut active, &mut spare).unwrap();

    // -- actualisation --------------------------------------------------------------
    {
        let start = Instant::now();
        for i in 0..reps {
            // Deux instants alternés : jamais `Unchanged`, toujours une vraie republication.
            black_box(controller.update(if i % 2 == 0 { T1 } else { T0 }).unwrap());
        }
        t.update = us(start, reps as f64);
        if controller.published_time() != T0 {
            controller.update(T0).unwrap();
        }
    }

    // -- évaluation -----------------------------------------------------------------
    let mut pool = [const { None }; IMPACTS_MAX];
    let impacts = Prepared::<64>::build(&journal, &mut pool, ctx).unwrap();
    let pressure = controller.current(T0).unwrap();
    let mut surface_scratch = [WaterSample::default(); LOT_MAX];
    let mut surface_out = [WaterSample::default(); LOT_MAX];
    let mut diff_scratch = [DifferentialSample::default(); LOT_MAX];
    let mut diff_out = [DifferentialSample::default(); LOT_MAX];
    for (k, lot) in LOTS.iter().enumerate() {
        let passes = (reps as usize / lot).max(4);
        let start = Instant::now();
        for _ in 0..passes {
            sample_world_batch(
                &bound,
                &impacts,
                Some(&pressure),
                T0,
                black_box(&points[..*lot]),
                MAX_SLOPE,
                &mut surface_scratch,
                &mut surface_out,
            )
            .unwrap();
            black_box(surface_out[0].eta);
        }
        t.surface[k] = us(start, (passes * lot) as f64);
        let start = Instant::now();
        for _ in 0..passes {
            differential_world_batch(
                &bound,
                &impacts,
                Some(&pressure),
                T0,
                black_box(&points[..*lot]),
                MAX_SLOPE,
                &mut diff_scratch,
                &mut diff_out,
            )
            .unwrap();
            black_box(diff_out[0].water.eta);
        }
        t.differential[k] = us(start, (passes * lot) as f64);
    }

    // -- attribution par couche : le même lot 64, pression retirée --------------------
    {
        let passes = 8;
        let start = Instant::now();
        for _ in 0..passes {
            sample_world_batch(
                &bound,
                &impacts,
                None,
                T0,
                black_box(&points[..64]),
                MAX_SLOPE,
                &mut surface_scratch,
                &mut surface_out,
            )
            .unwrap();
            black_box(surface_out[0].eta);
        }
        t.surface_np = us(start, (passes * 64) as f64);
        let start = Instant::now();
        for _ in 0..passes {
            differential_world_batch(
                &bound,
                &impacts,
                None,
                T0,
                black_box(&points[..64]),
                MAX_SLOPE,
                &mut diff_scratch,
                &mut diff_out,
            )
            .unwrap();
            black_box(diff_out[0].water.eta);
        }
        t.differential_np = us(start, (passes * 64) as f64);
    }

    // -- refus ----------------------------------------------------------------------
    // Un refus doit coûter le contrôle qui l'a produit, pas le lot qu'il n'a pas calculé.
    let lot = 64;
    {
        let start = Instant::now();
        for _ in 0..reps {
            black_box(differential_world_batch(
                &bound,
                &impacts,
                Some(&pressure),
                SimTime(7_900_000),
                black_box(&points[..lot]),
                MAX_SLOPE,
                &mut diff_scratch,
                &mut diff_out,
            ))
            .unwrap_err();
        }
        t.refuse_time = us(start, reps as f64);
    }
    {
        let wrong = BoundBackground::new(&b, FrameId(8), CELL);
        let start = Instant::now();
        for _ in 0..reps {
            black_box(differential_world_batch(
                &wrong,
                &impacts,
                Some(&pressure),
                T0,
                black_box(&points[..lot]),
                MAX_SLOPE,
                &mut diff_scratch,
                &mut diff_out,
            ))
            .unwrap_err();
        }
        t.refuse_context = us(start, reps as f64);
    }
    {
        let start = Instant::now();
        for _ in 0..reps {
            black_box(differential_world_batch(
                &bound,
                &impacts,
                Some(&pressure),
                T0,
                black_box(&points[..lot]),
                MAX_SLOPE,
                &mut diff_scratch[..lot - 1],
                &mut diff_out,
            ))
            .unwrap_err();
        }
        t.refuse_capacity = us(start, reps as f64);
    }
    // Hors domaine au dernier point, puis au premier. L'écart entre les deux **est** le
    // prix du travail déjà fait quand le refus tombe : le lot est atomique, rien n'est publié.
    for (index, slot) in [(lot - 1, 0usize), (0, 1)] {
        let mut far = points;
        far[index] = WorldPos::from_metres(1e9 + 1000.0, -1e9, 0.0);
        let passes = reps / 8;
        let start = Instant::now();
        for _ in 0..passes {
            black_box(differential_world_batch(
                &bound,
                &impacts,
                Some(&pressure),
                T0,
                black_box(&far[..lot]),
                MAX_SLOPE,
                &mut diff_scratch,
                &mut diff_out,
            ))
            .unwrap_err();
        }
        let v = us(start, passes as f64);
        if slot == 0 {
            t.refuse_point_last = v;
        } else {
            t.refuse_point_first = v;
        }
    }
    (t, arena.stats(), slots)
}

// ------------------------------------------------------------------ réceptions

/// Les deux chemins doivent refuser pour la même cause aux mêmes entrées : sans cela,
/// comparer leurs durées comparerait deux contrats différents.
fn receive_refusals() {
    let mut arena = Arena::default();
    let services = Services;
    let b = Background::configure(
        &mut HostServices {
            alloc: &mut arena,
            jobs: &services,
            sink: &services,
        },
        sea(REFERENCE.components),
        anchor(),
    )
    .unwrap();
    arena.seal();
    let mut records = [None; IMPACTS_MAX];
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
    let mut pool = [const { None }; IMPACTS_MAX];
    let impacts = Prepared::<64>::build(&journal, &mut pool, impact_context()).unwrap();
    let bound = BoundBackground::new(&b, FRAME, CELL);
    let wrong = BoundBackground::new(&b, FrameId(8), CELL);
    let points = all_points();
    let lot = 8;

    let mut names = Vec::new();
    let mut surface = Vec::new();
    let mut differential = Vec::new();
    let mut record = |name: &'static str, bd: &BoundBackground<'_>, time: SimTime, slope: f32, cut: bool| {
        let mut ws = [WaterSample::default(); LOT_MAX];
        let mut wo = ws;
        let mut ds = [DifferentialSample::default(); LOT_MAX];
        let mut d_o = ds;
        let n = if cut { lot - 1 } else { LOT_MAX };
        names.push(name);
        surface.push(
            sample_world_batch(
                bd,
                &impacts,
                None,
                time,
                &points[..lot],
                slope,
                &mut ws[..n],
                &mut wo,
            )
            .unwrap_err(),
        );
        differential.push(
            differential_world_batch(
                bd,
                &impacts,
                None,
                time,
                &points[..lot],
                slope,
                &mut ds[..n],
                &mut d_o,
            )
            .unwrap_err(),
        );
    };
    // L'impact 1 naît à 100 ms et vit 4 s : la fenêtre se ferme à 4,1 s.
    record("temps", &bound, SimTime(4_200_000), MAX_SLOPE, false);
    record("contexte", &wrong, T0, MAX_SLOPE, false);
    record("max_slope", &bound, T0, f32::NAN, false);
    record("capacite", &bound, T0, MAX_SLOPE, true);
    record("pente", &bound, T0, 1e-6, false);
    drop(record);
    for i in 0..names.len() {
        assert_eq!(
            surface[i], differential[i],
            "refus divergents pour {}: surface {:?}, differentiel {:?}",
            names[i], surface[i], differential[i]
        );
    }
    let shown: Vec<String> = names
        .iter()
        .zip(&surface)
        .map(|(n, e)| format!("{n}={e:?}"))
        .collect();
    println!("refus communs aux deux chemins: {}", shown.join(" "));

    // Sortie préservée sur refus : le lot est atomique des deux côtés.
    let mut keep = [DifferentialSample::default(); 2];
    keep[0].applied_pressure = 123.0;
    let before = keep;
    let mut scratch = keep;
    assert!(differential_world_batch(
        &bound,
        &impacts,
        None,
        SimTime(4_200_000),
        &points[..2],
        MAX_SLOPE,
        &mut scratch,
        &mut keep
    )
    .is_err());
    assert_eq!(keep, before);
    let mut wkeep = [WaterSample::default(); 2];
    wkeep[0].eta = 123.0;
    let wbefore = wkeep;
    let mut wscratch = wkeep;
    assert!(sample_world_batch(
        &bound,
        &impacts,
        None,
        SimTime(4_200_000),
        &points[..2],
        MAX_SLOPE,
        &mut wscratch,
        &mut wkeep
    )
    .is_err());
    assert_eq!(wkeep[0].eta.to_bits(), wbefore[0].eta.to_bits());
    println!("sortie preservee sur refus, deux chemins: oui");
}

fn footprint() {
    println!(
        "octets: WaterSample={} DifferentialSample={} Slot={} Node={} OptionRadialImpact64={}",
        size_of::<WaterSample>(),
        size_of::<DifferentialSample>(),
        size_of::<Slot>(),
        size_of::<Node>(),
        size_of::<Option<RadialImpact<64>>>()
    );
    println!("montage | cuisson | pools pression | champs impacts | tampons surface | tampons differentiels | total surface | total differentiel");
    for (label, radial, angular, impacts, lot) in [
        ("reference r16a24 i1 lot64", 16usize, 24usize, 1usize, 64usize),
        ("recette r8a12 i1 lot64", 8, 12, 1, 64),
        ("recette r24a32 i1 lot64", 24, 32, 1, 64),
        ("4 impacts lot64", 16, 24, 4, 64),
        ("reference lot256", 16, 24, 1, 256),
    ] {
        let slots = radial * angular / 2;
        let cook = (radial * angular + slots) * size_of::<Node>();
        let pools = 2 * slots * size_of::<Slot>();
        let fields = impacts * size_of::<Option<RadialImpact<64>>>();
        let s = 2 * lot * size_of::<WaterSample>();
        let d = 2 * lot * size_of::<DifferentialSample>();
        println!(
            "{label} | {cook} | {pools} | {fields} | {s} | {d} | {} | {}",
            cook + pools + fields + s,
            cook + pools + fields + d
        );
    }
}

fn median(v: &mut [f64]) -> (f64, f64, f64) {
    v.sort_by(f64::total_cmp);
    (v[0], v[v.len() / 2], v[v.len() - 1])
}

fn main() {
    receive_refusals();
    footprint();

    let cases = [
        REFERENCE,
        Case {
            label: "recette r8a12",
            radial: 8,
            angular: 12,
            ..REFERENCE
        },
        Case {
            label: "recette r24a32",
            radial: 24,
            angular: 32,
            ..REFERENCE
        },
        Case {
            label: "B 64 composantes",
            components: 64,
            ..REFERENCE
        },
        Case {
            label: "0 impact",
            impacts: 0,
            ..REFERENCE
        },
        Case {
            label: "4 impacts",
            impacts: 4,
            ..REFERENCE
        },
    ];
    // Mise en régime : tous les montages, une seconde pleine, avant la première mesure.
    let warm = Instant::now();
    while warm.elapsed().as_secs_f64() < 1.0 {
        for c in cases {
            black_box(run(c, 32).0.update);
        }
    }
    let mut results = vec![[Times::default(); BLOCKS]; cases.len()];
    let mut stats = vec![AllocStats::default(); cases.len()];
    let mut slots = vec![0usize; cases.len()];
    for block in 0..BLOCKS {
        for offset in 0..cases.len() {
            // Ordre renversé un bloc sur deux : la dérive ne se range pas d'un seul côté.
            let i = if block % 2 == 0 {
                offset
            } else {
                cases.len() - 1 - offset
            };
            let (t, s, n) = run(cases[i], 512);
            results[i][block] = t;
            stats[i] = s;
            slots[i] = n;
        }
    }
    println!("\n-- preparation, us par operation (min/med/max sur {BLOCKS} blocs)");
    println!("montage | creneaux | RadialImpact::new | Prepared::build | bake+half | Context::new | Controller::new | update");
    for (i, c) in cases.iter().enumerate() {
        let pick = |f: fn(&Times) -> f64| {
            let mut v: Vec<f64> = results[i].iter().map(f).collect();
            median(&mut v)
        };
        let a = pick(|t| t.impact_new);
        let b = pick(|t| t.prepared_build);
        let d = pick(|t| t.bake);
        let e = pick(|t| t.context_new);
        let f = pick(|t| t.controller_new);
        let g = pick(|t| t.update);
        println!(
            "{} | {} | {:.4}/{:.4}/{:.4} | {:.4}/{:.4}/{:.4} | {:.3}/{:.3}/{:.3} | {:.4}/{:.4}/{:.4} | {:.3}/{:.3}/{:.3} | {:.3}/{:.3}/{:.3}",
            c.label, slots[i], a.0, a.1, a.2, b.0, b.1, b.2, d.0, d.1, d.2, e.0, e.1, e.2, f.0,
            f.1, f.2, g.0, g.1, g.2
        );
    }
    println!("\n-- evaluation, us par point (mediane), surface / differentiel / rapport");
    print!("montage");
    for lot in LOTS {
        print!(" | lot {lot}");
    }
    println!();
    for (i, c) in cases.iter().enumerate() {
        print!("{}", c.label);
        for k in 0..LOTS.len() {
            let mut s: Vec<f64> = results[i].iter().map(|t| t.surface[k]).collect();
            let mut d: Vec<f64> = results[i].iter().map(|t| t.differential[k]).collect();
            let sm = median(&mut s).1;
            let dm = median(&mut d).1;
            print!(" | {sm:.4} / {dm:.4} / {:.2}", dm / sm);
        }
        println!();
    }
    println!("\n-- attribution, us par point, lot 64, avec et sans pression (mediane)");
    println!("montage | surface avec | surface sans | differentiel avec | differentiel sans");
    for (i, c) in cases.iter().enumerate() {
        let pick = |f: fn(&Times) -> f64| {
            let mut v: Vec<f64> = results[i].iter().map(f).collect();
            median(&mut v).1
        };
        println!(
            "{} | {:.4} | {:.4} | {:.4} | {:.4}",
            c.label,
            pick(|t| t.surface[2]),
            pick(|t| t.surface_np),
            pick(|t| t.differential[2]),
            pick(|t| t.differential_np)
        );
    }
    println!("\n-- refus, us par appel refuse, lot 64 demande (mediane)");
    println!("montage | temps | contexte | capacite | hors domaine au point 63 | au point 0");
    for (i, c) in cases.iter().enumerate() {
        let pick = |f: fn(&Times) -> f64| {
            let mut v: Vec<f64> = results[i].iter().map(f).collect();
            median(&mut v).1
        };
        println!(
            "{} | {:.4} | {:.4} | {:.4} | {:.2} | {:.4}",
            c.label,
            pick(|t| t.refuse_time),
            pick(|t| t.refuse_context),
            pick(|t| t.refuse_capacity),
            pick(|t| t.refuse_point_last),
            pick(|t| t.refuse_point_first)
        );
    }
    println!("\n-- allocations hote par montage, Background::configure puis seal()");
    println!("montage | octets | appels | refusees apres seal");
    for (i, c) in cases.iter().enumerate() {
        println!(
            "{} | {} | {} | {}",
            c.label,
            stats[i].persistent_bytes,
            stats[i].persistent_calls,
            stats[i].refused_after_seal
        );
    }
}
