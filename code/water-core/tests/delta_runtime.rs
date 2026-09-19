use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use water_core::delta_projection::{Domain, Error, Volume};
use water_core::host::MonotonicClock;
thread_local! {
    static TRACK: Cell<bool> = const { Cell::new(false) };
    static CALLS: Cell<usize> = const { Cell::new(0) };
}
struct Counter;
fn allocation() {
    let _ = TRACK.try_with(|track| {
        if track.get() {
            let _ = CALLS.try_with(|c| c.set(c.get() + 1));
        }
    });
}
// Instrument uniquement : le cœur conserve son interdiction de code unsafe.
unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        allocation();
        unsafe { System.alloc(l) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        allocation();
        unsafe { System.alloc_zeroed(l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        allocation();
        unsafe { System.realloc(p, l, n) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
}
#[global_allocator]
static ALLOCATOR: Counter = Counter;
fn measured<T>(f: impl FnOnce() -> T) -> (T, usize) {
    struct Stop;
    impl Drop for Stop {
        fn drop(&mut self) {
            TRACK.with(|t| t.set(false));
        }
    }
    CALLS.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    let stop = Stop;
    let result = f();
    drop(stop);
    (result, CALLS.with(Cell::get))
}
use water_core::host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};

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
struct Jobs;
impl Sink for Jobs {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Jobs {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        grain: usize,
        r: &dyn Fn(usize, usize) -> f64,
        m: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        // Découpe réelle, fusion **dans l'ordre des indices** : c'est la propriété que
        // SPEC-004 §8.2 exige, et la tester ici vaut mieux que la supposer.
        let g = grain.max(1);
        let mut acc = init;
        let mut s = 0;
        while s < n {
            let e = (s + g).min(n);
            acc = m(acc, r(s, e));
            s = e;
        }
        acc
    }
}

/// Fond non plat : une bosse continue et une marche, pour que le découpage ait du travail.
fn bottom(nx: usize, dx: f32) -> Vec<f32> {
    (0..nx)
        .map(|i| {
            let x = i as f32 * dx;
            let bump = 0.6 * (-(((x - 3.0) / 1.2) * ((x - 3.0) / 1.2))).exp();
            let step = if i > 3 * nx / 4 { 0.35 } else { 0. };
            0.4 + bump + step
        })
        .collect()
}

fn build(nx: usize, nz: usize, dx: f32, g: f32) -> (Volume, Arena) {
    let mut arena = Arena {
        stats: AllocStats::default(),
        sealed: false,
    };
    let jobs = Jobs;
    let b = bottom(nx, dx);
    let v = Volume::configure(
        &mut HostServices {
            alloc: &mut arena,
            jobs: &jobs,
            sink: &jobs,
        },
        Domain { nx, nz, dx },
        1025.0,
        g,
        &b,
    )
    .unwrap();
    (v, arena)
}

#[test]
fn perturbation_step_has_no_runtime_allocation_s250() {
    use water_core::{background::BackgroundSample, delta_projection::{BackgroundFaces,Sponge}, SimTime};
    struct Frozen;
    impl MonotonicClock for Frozen { fn now_ns(&self)->u64 {0} }
    let (mut v,mut arena)=build(32,16,0.25,9.81);
    let mut u=vec![BackgroundSample::default();v.velocity_u().len()];
    let w=vec![BackgroundSample::default();v.velocity_w().len()];
    for (i,s) in u.iter_mut().enumerate() {s.du_dt[0]=(i as f32*0.17).sin();}
    let bg=BackgroundFaces{domain:v.domain(),time:SimTime(0),density:1025.,gravity:9.81,u:&u,w:&w};
    arena.seal();
    let (result,allocations)=measured(||v.step_perturbation(SimTime(0),1000,2000,1_000_000,
        &bg,Sponge{width_m:1.,rate_per_s:2.},&Jobs,&Frozen));
    assert_eq!(result.unwrap().advanced_us,1000);
    assert_eq!(allocations,0);
    assert_eq!(arena.stats().refused_after_seal,0);
    let (result,allocations)=measured(||v.step_perturbation(SimTime(0),1000,2000,0,
        &bg,Sponge::default(),&Jobs,&Frozen));
    assert_eq!(result.unwrap().advanced_us,0);
    assert_eq!(allocations,0);
}

#[test]
fn velocity_refinement_has_no_runtime_allocation_s251() {
    use water_core::{background::BackgroundSample,delta_projection::{BackgroundFaces,Sponge},SimTime};
    struct Frozen;
    impl MonotonicClock for Frozen {fn now_ns(&self)->u64 {0}}
    let mut arena=Arena{stats:AllocStats::default(),sealed:false};
    let domain=Domain{nx:16,nz:8,dx:0.5};
    let mut v=Volume::configure(&mut HostServices{alloc:&mut arena,jobs:&Jobs,sink:&Jobs},
        domain,1025.,9.81,&[0.;16]).unwrap();
    let u=vec![BackgroundSample::default();v.velocity_u().len()];
    let mut w=vec![BackgroundSample::default();v.velocity_w().len()];
    // Gradient vertical presque entièrement annulé par pression ; l'éponge laisse un
    // petit champ solénoïdal. Contre-épreuve du chemin d'affinage, sans fournisseur caché.
    for k in 0..=8 {for i in 0..16 {w[k*16+i].du_dt[2]=0.014*(2.*(k as f32*0.5-4.)).exp();}}
    let bg=BackgroundFaces{domain,time:SimTime(0),density:1025.,gravity:9.81,u:&u,w:&w};
    arena.seal();
    let (r,allocations)=measured(||v.step_perturbation(SimTime(0),1000,2000,1_000_000,
        &bg,Sponge{width_m:1.,rate_per_s:2.},&Jobs,&Frozen));
    let report=r.unwrap().report.unwrap();
    assert_eq!(report.refinements,1,"{report:?}");
    assert!(!report.degraded && report.divergence<=1e-5);
    assert_eq!(allocations,0);
    assert_eq!(arena.stats().refused_after_seal,0);
}

use water_core::background::BackgroundSample;
#[path = "../examples/support/standing_background.rs"]
#[allow(dead_code)]
mod standing;

/// S253 (ADR-152, critère 2) — le pas perturbatif mobile n'alloue rien, sur un pas complet comme
/// sur une expiration, les échantillons du fond étant préparés par l'hôte avant le scellement.
#[test]
fn coupled_mobile_step_has_no_runtime_allocation_s253() {
    use water_core::{delta_projection::{BackgroundFaces,Sponge},SimTime};
    struct Frozen;
    impl MonotonicClock for Frozen {fn now_ns(&self)->u64 {0}}
    let wave=standing::StandingWave{a:0.05,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho:1025.};
    let (nx,dx)=(32usize,2./32.);
    let mut arena=Arena{stats:AllocStats::default(),sealed:false};
    let domain=Domain{nx,nz:36,dx:dx as f32};
    let mut v=Volume::configure(&mut HostServices{alloc:&mut arena,jobs:&Jobs,sink:&Jobs},
        domain,1025.,9.81,&vec![0.;nx]).unwrap();
    v.set_free_surface(&vec![2.;nx],2.).unwrap();
    let mut u=vec![BackgroundSample::default();v.velocity_u().len()];
    let mut w=vec![BackgroundSample::default();v.velocity_w().len()];
    for k in 0..36 {for i in 0..=nx {u[k*(nx+1)+i]=wave.sample(i as f64*dx,(k as f64+0.5)*dx-2.,0.2);}}
    for k in 0..=36 {for i in 0..nx {w[k*nx+i]=wave.sample((i as f64+0.5)*dx,k as f64*dx-2.,0.2);}}
    let bg=BackgroundFaces{domain,time:SimTime(200_000),density:1025.,gravity:9.81,u:&u,w:&w};
    arena.seal();
    let (r,allocations)=measured(||v.step_perturbation_mobile(SimTime(200_000),1000,4000,1_000_000,
        &bg,Sponge::default(),&Jobs,&Frozen));
    assert_eq!(r.unwrap().advanced_us,1000);
    assert_eq!(allocations,0);
    let (r,allocations)=measured(||v.step_perturbation_mobile(SimTime(200_000),1000,4000,0,
        &bg,Sponge::default(),&Jobs,&Frozen));
    assert_eq!(r.unwrap().advanced_us,0);
    assert_eq!(allocations,0);
    assert_eq!(arena.stats().refused_after_seal,0);
}

#[test]
fn global_allocator_sees_counterexample_but_no_step_allocations() {
    let (_, count) = measured(|| {
        let mut v = Vec::with_capacity(16);
        v.push(3u64);
        std::hint::black_box(v);
    });
    assert!(count > 0, "le compteur doit voir une vraie allocation");
    let (mut v, mut arena) = build(32, 16, 0.25, 9.81);
    let eta: Vec<_> = (0..32)
        .map(|i| v.domain().z0() + 0.02 * (i as f32 * 0.3).sin())
        .collect();
    v.set_surface(&eta).unwrap();
    arena.seal();
    // Premier pas aussi : aucun échauffement susceptible de cacher une allocation.
    for limit in [500, 1, 0, 500] {
        let (r, allocs) = measured(|| v.step(0.002, limit, &Jobs));
        assert_eq!(allocs, 0, "limite {limit}");
        let r = r.unwrap();
        // S252 (ADR-151) : le plafond vaut par projection, et `iterations` compte tout le pas —
        // au plus l'ordinaire, son repli multigrille et un affinage. Jusqu'à S251, la dernière seule.
        assert!(r.iterations <= 3 * limit, "limite {limit} : {r:?}");
        if limit == 1 || limit == 0 {
            assert!(r.degraded);
        }
    }
}

#[test]
fn failed_calculation_restores_all_published_state_and_can_recover() {
    let (mut v, mut arena) = build(32, 16, 0.25, 9.81);
    let eta: Vec<_> = (0..32)
        .map(|i| v.domain().z0() + 0.02 * (i as f32 * 0.3).sin())
        .collect();
    v.set_surface(&eta).unwrap();
    v.step(0.002, 500, &Jobs).unwrap();
    let normal_u = v.velocity_u().to_vec();
    let normal_w = v.velocity_w().to_vec();
    let p = v.pressure().to_vec();
    let extreme: Vec<_> = (0..normal_u.len())
        .map(|i| if i % 3 == 0 { f32::MAX } else { 0. })
        .collect();
    v.set_velocity(&extreme, &normal_w).unwrap();
    arena.seal();
    let (result, allocs) = measured(|| v.step(1., 500, &Jobs));
    assert_eq!(result, Err(Error::NotFinite));
    assert_eq!(allocs, 0);
    assert_eq!(v.velocity_u(), extreme);
    assert_eq!(v.velocity_w(), normal_w);
    assert_eq!(v.pressure(), p);
    v.set_velocity(&normal_u, &normal_w).unwrap();
    let (r, count) = measured(|| v.step(0.002, 500, &Jobs));
    assert!(r.is_ok());
    assert_eq!(count, 0);
    // Le pas refusé ne doit pas polluer les tampons employés par le pas suivant.
    let (mut fresh, _) = build(32, 16, 0.25, 9.81);
    fresh.set_surface(&eta).unwrap();
    fresh.set_velocity(&normal_u, &normal_w).unwrap();
    fresh.step(0.002, 500, &Jobs).unwrap();
    assert_eq!(v.velocity_u(), fresh.velocity_u());
    assert_eq!(v.velocity_w(), fresh.velocity_w());
    assert_eq!(v.pressure(), fresh.pressure());
}

#[test]
fn allocation_accounting_matches_requested_typed_storage() {
    let (v, arena) = build(32, 16, 0.25, 9.81);
    let (n, k) = (v.domain().nx, v.domain().nz);
    // Quatre tableaux f32 par famille de faces, fond+surface, fraction ; huit f32 par cellule
    // depuis S251 (pression principale pendant l'affinage de vitesse). S253 : un tableau de plus
    // sur les faces u (fantôme latéral du fond), sept au lieu de cinq par colonne (surface totale,
    // fantôme vertical du fond).
    let bytes = (4 * ((n + 1) * k + n * (k + 1)) + (n + 1) * k + 7 * n + n * k) * 4 + 8 * n * k * 4;
    // S245 : la hiérarchie multigrille. Le compte est **refait ici**, indépendamment du cœur : un
    // essai de comptabilité qui appellerait la même fonction que le code ne vérifierait rien.
    // Division par deux tant que les deux dimensions sont paires et au moins huit ; par niveau,
    // cinq tableaux de mailles (fraction, diagonale, correction, second membre, temporaire) et les
    // deux familles d'ouvertures. S274 (ADR-167) : deux tableaux mobiles de plus par niveau
    // (fractions, diagonale), et une correction fine du cycle mobile dès qu'un niveau existe.
    let (mut hx, mut hz, mut hierarchy) = (n, k, 0usize);
    while hx % 2 == 0 && hz % 2 == 0 && hx >= 8 && hz >= 8 {
        let (cx, cz) = (hx / 2, hz / 2);
        hierarchy += 7 * cx * cz + (cx + 1) * cz + cx * (cz + 1);
        (hx, hz) = (cx, cz);
    }
    assert_eq!(hierarchy, 1156 + 2 * (16 * 8 + 8 * 4), "32×16 donne 16×8 puis 8×4");
    let hierarchy = hierarchy + n * k;
    let bytes = bytes + hierarchy * 4;
    assert_eq!(arena.stats().persistent_bytes, bytes);
    assert_eq!(arena.stats().persistent_calls, 1);
    let mut arena = Arena {
        stats: AllocStats::default(),
        sealed: false,
    };
    let result = Volume::configure(
        &mut HostServices {
            alloc: &mut arena,
            jobs: &Jobs,
            sink: &Jobs,
        },
        Domain {
            nx: 1,
            nz: usize::MAX,
            dx: 1.,
        },
        1025.,
        9.81,
        &[0.],
    );
    assert_eq!(result.err(), Some(Error::Domain));
    assert_eq!(arena.stats().persistent_calls, 0);
}

struct Clock { first:u64, second:u64, calls:Cell<u32> }
impl Clock { fn new(first:u64,second:u64)->Self {Self{first,second,calls:Cell::new(0)}} }
impl MonotonicClock for Clock {
    fn now_ns(&self)->u64 { let call=self.calls.get(); self.calls.set(call+1); if call==0 {self.first}else{self.second} }
}
#[test]
fn measured_cost_is_scoped_and_does_not_change_the_step() {
    let (mut timed,mut arena)=build(32,16,0.25,9.81);
    let (mut plain,_)=build(32,16,0.25,9.81);
    let eta:Vec<_>=(0..32).map(|i|timed.domain().z0()+0.02*(i as f32*0.3).sin()).collect();
    assert_eq!(timed.caps().cost_per_block_ms,None);
    timed.set_surface(&eta).unwrap(); plain.set_surface(&eta).unwrap(); arena.seal();
    let clock=Clock::new(10_000,1_510_000);
    let (r,count)=measured(||timed.step_measured(0.002,500,&Jobs,&clock));
    assert_eq!(count,0); assert_eq!(clock.calls.get(),2);
    assert_eq!(r.unwrap(),plain.step(0.002,500,&Jobs).unwrap());
    assert_eq!(timed.caps().cost_per_block_ms,Some(1.5));
    assert_eq!(timed.velocity_u(),plain.velocity_u());
    assert_eq!(timed.velocity_w(),plain.velocity_w()); assert_eq!(timed.pressure(),plain.pressure());
    timed.set_surface(&eta).unwrap(); assert_eq!(timed.caps().cost_per_block_ms,None);
    for (first,second) in [(1,1),(2,1)] {
        timed.step_measured(0.002,1,&Jobs,&Clock::new(first,second)).unwrap();
        assert_eq!(timed.caps().cost_per_block_ms,None);
    }
    let r=timed.step_measured(0.002,1,&Jobs,&Clock::new(0,500_000)).unwrap();
    assert!(r.degraded); assert_eq!(timed.caps().cost_per_block_ms,Some(0.5));
    assert!(timed.step_measured(0.,1,&Jobs,&Clock::new(0,100)).is_err());
    assert_eq!(timed.caps().cost_per_block_ms,None);
    timed.step_measured(0.002,1,&Jobs,&Clock::new(0,100)).unwrap();
    timed.step(0.002,1,&Jobs).unwrap(); assert_eq!(timed.caps().cost_per_block_ms,None);
}

struct DeadlineClock { calls: Cell<usize>, cutoff: usize, backward: bool }
impl DeadlineClock {
    fn new(cutoff: usize) -> Self { Self { calls: Cell::new(0), cutoff, backward: false } }
}
impl MonotonicClock for DeadlineClock {
    fn now_ns(&self) -> u64 {
        let i = self.calls.get(); self.calls.set(i + 1);
        if self.backward { if i < self.cutoff { 100 } else { 99 } }
        else if i < self.cutoff { 0 } else { 1_000_000 }
    }
}
fn prepared() -> Volume {
    let (mut v, mut arena) = build(8, 4, 1., 9.81);
    v.set_surface(&(0..8).map(|i| v.domain().z0() + 0.02 * (i as f32 * 0.3).sin()).collect::<Vec<_>>()).unwrap();
    arena.seal(); v.step(0.002, 100, &Jobs).unwrap(); v
}
fn bits(v: &Volume) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    (v.velocity_u().iter().map(|x| x.to_bits()).collect(),
     v.velocity_w().iter().map(|x| x.to_bits()).collect(),
     v.pressure().iter().map(|x| x.to_bits()).collect())
}

#[test]
fn every_budget_checkpoint_preserves_state_and_allows_identical_retry_s230() {
    use water_core::delta_projection::Phase;
    let (_, positive) = measured(|| std::hint::black_box(vec![0u8; 8])); assert!(positive > 0);
    let mut reference = prepared(); let before = bits(&reference);
    assert!(reference.pressure().iter().any(|x| *x != 0.));
    let normal_report = reference.step(0.002, 100, &Jobs).unwrap(); let after = bits(&reference);
    let mut probe = prepared(); let clock = DeadlineClock::new(usize::MAX);
    let (r, allocations) = measured(|| probe.step_budgeted(0.002, 100, 1., &Jobs, &clock));
    assert_eq!(allocations, 0); let r = r.unwrap();
    assert_eq!(r.report, Some(normal_report)); assert_eq!(r.advanced_dt, 0.002);
    assert_eq!(r.remaining_dt, 0.); assert_eq!(r.stopped_at, None); assert_eq!(bits(&probe), after);
    let mut phases = [false; 8];
    // Chaque lecture après l'origine est un point d'expiration, y compris les tampons extraits
    // temporairement et la dernière publication. Aucun seuil choisi pour seulement le début.
    for cutoff in 1..clock.calls.get() {
        let mut v = prepared(); let timer = DeadlineClock::new(cutoff);
        let (result, allocations) = measured(|| v.step_budgeted(0.002, 100, 1., &Jobs, &timer));
        assert_eq!(allocations, 0); let result = result.unwrap();
        assert_eq!(result.advanced_dt, 0.); assert_eq!(result.remaining_dt, 0.002);
        assert_eq!(result.report, None); assert_eq!(result.elapsed_ns, 1_000_000);
        assert_eq!(v.caps().cost_per_block_ms, None); assert_eq!(bits(&v), before);
        let index = match result.stopped_at.unwrap() {
            Phase::Prepare => 0, Phase::Advect => 1, Phase::Rhs => 2, Phase::Pressure => 3,
            Phase::Correct => 4, Phase::Diagnostics => 5, Phase::Validate => 6, Phase::Publish => 7,
        };
        phases[index] = true;
        let (retry, allocations) = measured(|| v.step(0.002, 100, &Jobs));
        assert_eq!(allocations, 0); assert_eq!(retry.unwrap(), normal_report); assert_eq!(bits(&v), after);
    }
    assert_eq!(phases, [true; 8]);
    println!("S230: {} points d'expiration, huit phases, reprise identique et zero allocation", clock.calls.get()-1);
}

#[test]
fn budget_zero_invalid_clock_and_numeric_error_do_not_publish_s230() {
    let mut v = prepared(); let before = bits(&v);
    let timer = DeadlineClock::new(usize::MAX);
    let r = v.step_budgeted(0.002, 100, 0., &Jobs, &timer).unwrap();
    assert_eq!(r.advanced_dt, 0.); assert_eq!(r.remaining_dt, 0.002); assert_eq!(bits(&v), before);
    for budget in [-1., f32::NAN, f32::INFINITY, f32::MAX] {
        assert_eq!(v.step_budgeted(0.002, 100, budget, &Jobs, &timer), Err(Error::NotFinite));
        assert_eq!(bits(&v), before);
    }
    for cutoff in [1, 15, 30] {
        let backward = DeadlineClock { backward: true, ..DeadlineClock::new(cutoff) };
        let (result, allocations) = measured(|| v.step_budgeted(0.002, 100, 1., &Jobs, &backward));
        assert_eq!(result, Err(Error::Clock)); assert_eq!(allocations, 0); assert_eq!(bits(&v), before);
    }
    let extreme = vec![f32::MAX; v.velocity_u().len()]; let w = v.velocity_w().to_vec();
    v.set_velocity(&extreme, &w).unwrap(); let extreme_bits = bits(&v);
    let (r, allocs) = measured(|| v.step_budgeted(1., 100, 1., &Jobs, &timer));
    assert_eq!(r, Err(Error::NotFinite)); assert_eq!(allocs, 0); assert_eq!(bits(&v), extreme_bits);
}

#[test]
fn budgeted_reductions_are_bounded_and_keep_the_unlimited_bits_s230() {
    struct CappedJobs(Cell<usize>);
    impl JobSystem for CappedJobs {
        fn worker_count(&self) -> u32 { 1 }
        fn parallel_reduce_ordered_f64(&self, n: usize, grain: usize,
            r: &dyn Fn(usize,usize)->f64, m: &dyn Fn(f64,f64)->f64, init: f64) -> f64 {
            assert!(n <= 64); self.0.set(self.0.get()+1);
            Jobs.parallel_reduce_ordered_f64(n, grain, r, m, init)
        }
    }
    let (mut limited, _) = build(32, 16, 0.25, 9.81);
    let (mut plain, _) = build(32, 16, 0.25, 9.81);
    let eta: Vec<_> = (0..32).map(|i| 4. + 0.02 * (i as f32 * 0.3).sin()).collect();
    limited.set_surface(&eta).unwrap(); plain.set_surface(&eta).unwrap();
    let jobs = CappedJobs(Cell::new(0)); let clock = DeadlineClock::new(usize::MAX);
    for limit in [500, 1, 0, 500] {
        let reference = plain.step(0.002, limit, &Jobs).unwrap();
        let (result, allocations) = measured(|| limited.step_budgeted(0.002, limit, 1., &jobs, &clock));
        assert_eq!(allocations, 0); let result = result.unwrap();
        assert_eq!(result.report, Some(reference)); assert_eq!(result.advanced_dt, 0.002);
        assert_eq!(result.remaining_dt, 0.); assert_eq!(bits(&limited), bits(&plain));
        if limit <= 1 { assert!(reference.degraded); }
    }
    assert!(jobs.0.get() > 100, "le consommateur doit passer par plusieurs petits appels");
}


fn surface_bits(v:&Volume)->Vec<u32> {v.surface().iter().map(|v|v.to_bits()).collect()}
#[test]
fn evolving_surface_is_atomic_at_every_checkpoint_and_allocation_free_s233() {
    let mut reference=prepared();
    reference.step_surface_linear(2000,100,1000,&Jobs,&DeadlineClock::new(usize::MAX)).unwrap();
    let make=|| {
        let mut v=prepared();
        v.step_surface_linear(2000,100,1000,&Jobs,&DeadlineClock::new(usize::MAX)).unwrap(); v
    };
    let before=bits(&reference); let eta_before=surface_bits(&reference);
    let clock=DeadlineClock::new(usize::MAX);
    let (r,allocs)=measured(||reference.step_surface_linear(2000,100,1000,&Jobs,&clock));
    assert_eq!(allocs,0); let report=r.unwrap();
    let after=bits(&reference); let eta_after=surface_bits(&reference);
    assert_ne!(eta_before,eta_after);
    let (_,positive)=measured(||std::hint::black_box(vec![0u8;8])); assert!(positive>0);
    for cutoff in 1..clock.calls.get() {
        let mut v=make();
        let (r,allocs)=measured(||v.step_surface_linear(2000,100,1000,&Jobs,&DeadlineClock::new(cutoff)));
        assert_eq!(allocs,0); let r=r.unwrap();
        assert_eq!((r.advanced_us,r.remaining_us),(0,2000)); assert!(r.report.is_none());
        assert_eq!(bits(&v),before); assert_eq!(surface_bits(&v),eta_before);
        let (r,allocs)=measured(||v.step_surface_linear(2000,100,1000,&Jobs,&DeadlineClock::new(usize::MAX)));
        assert_eq!(allocs,0); assert_eq!(r.unwrap(),report);
        assert_eq!(bits(&v),after); assert_eq!(surface_bits(&v),eta_after);
    }
    for (duration,iterations,budget,error) in [(0,100,1000,Error::NotFinite),
        (1u64<<54,100,1000,Error::NotFinite),(1_000_000,100,1000,Error::Domain),
        (2000,0,1000,Error::Convergence),(2000,100,u64::MAX,Error::NotFinite)] {
        let mut v=make();
        let (r,allocs)=measured(||v.step_surface_linear(duration,iterations,budget,&Jobs,&DeadlineClock::new(usize::MAX)));
        assert_eq!(r,Err(error)); assert_eq!(allocs,0);
        assert_eq!(bits(&v),before); assert_eq!(surface_bits(&v),eta_before);
    }
    let mut v=make();
    let backward=DeadlineClock{backward:true,..DeadlineClock::new(clock.calls.get()-1)};
    assert_eq!(v.step_surface_linear(2000,100,1000,&Jobs,&backward),Err(Error::Clock));
    assert_eq!(bits(&v),before); assert_eq!(surface_bits(&v),eta_before);
    let r=v.step_surface_linear(2000,100,0,&Jobs,&DeadlineClock::new(usize::MAX)).unwrap();
    assert_eq!(r.remaining_us,2000); assert_eq!(surface_bits(&v),eta_before);
    // Défaut numérique après entrée valide : rollback des champs et du reste de hauteur.
    let huge = vec![f32::MAX;v.surface().len()];
    v.set_surface(&huge).unwrap(); let huge_bits=surface_bits(&v);
    let (r,allocs)=measured(||v.step_surface_linear(2000,100,1000,&Jobs,&DeadlineClock::new(usize::MAX)));
    assert_eq!(r,Err(Error::NotFinite)); assert_eq!(allocs,0);
    assert_eq!(bits(&v),before); assert_eq!(surface_bits(&v),huge_bits);
    println!("S233: {} expirations, surface incluse, zero allocation et reprise identique",clock.calls.get()-1);
}

/// S237 : bassin 2×3 m, surface mobile ondulée, déjà avancée d'un pas.
fn prepared_mobile() -> Volume {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut v = Volume::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain { nx: 8, nz: 12, dx: 0.25 }, 1025., 9.81, &[0.; 8]).unwrap();
    let eta: Vec<f32> = (0..8).map(|i| 2.0 + 0.2 * (std::f32::consts::PI * (i as f32 + 0.5) / 8.).cos()).collect();
    v.set_free_surface(&eta, 2.0).unwrap();
    arena.seal();
    v.step_surface_mobile(1000, 500, 1000, &Jobs, &DeadlineClock::new(usize::MAX)).unwrap();
    v
}

#[test]
fn mobile_surface_is_atomic_at_every_checkpoint_and_allocation_free_s237() {
    let mut reference = prepared_mobile();
    let before = bits(&reference); let eta_before = surface_bits(&reference);
    let clock = DeadlineClock::new(usize::MAX);
    let (r, allocs) = measured(|| reference.step_surface_mobile(1000, 500, 1000, &Jobs, &clock));
    assert_eq!(allocs, 0); let report = r.unwrap();
    let after = bits(&reference); let eta_after = surface_bits(&reference);
    assert_ne!(eta_before, eta_after);
    let (_, positive) = measured(|| std::hint::black_box(vec![0u8; 8])); assert!(positive > 0);
    for cutoff in 1..clock.calls.get() {
        let mut v = prepared_mobile();
        let (r, allocs) = measured(|| v.step_surface_mobile(1000, 500, 1000, &Jobs, &DeadlineClock::new(cutoff)));
        assert_eq!(allocs, 0); let r = r.unwrap();
        assert_eq!((r.advanced_us, r.remaining_us), (0, 1000)); assert!(r.report.is_none());
        assert_eq!(bits(&v), before); assert_eq!(surface_bits(&v), eta_before);
        let (r, allocs) = measured(|| v.step_surface_mobile(1000, 500, 1000, &Jobs, &DeadlineClock::new(usize::MAX)));
        assert_eq!(allocs, 0); assert_eq!(r.unwrap(), report);
        assert_eq!(bits(&v), after); assert_eq!(surface_bits(&v), eta_after);
    }
    for (duration, iterations, budget, error) in [(0, 500, 1000, Error::NotFinite),
        (1u64 << 54, 500, 1000, Error::NotFinite), (1_000_000, 500, 1000, Error::Domain),
        (1000, 0, 1000, Error::Convergence), (1000, 500, u64::MAX, Error::NotFinite)] {
        let mut v = prepared_mobile();
        let (r, allocs) = measured(|| v.step_surface_mobile(duration, iterations, budget, &Jobs, &DeadlineClock::new(usize::MAX)));
        assert_eq!(r, Err(error)); assert_eq!(allocs, 0);
        assert_eq!(bits(&v), before); assert_eq!(surface_bits(&v), eta_before);
    }
    let mut v = prepared_mobile();
    let backward = DeadlineClock { backward: true, ..DeadlineClock::new(clock.calls.get() - 1) };
    assert_eq!(v.step_surface_mobile(1000, 500, 1000, &Jobs, &backward), Err(Error::Clock));
    assert_eq!(bits(&v), before); assert_eq!(surface_bits(&v), eta_before);
    println!("S237: {} expirations, surface mobile incluse, zero allocation et reprise identique", clock.calls.get() - 1);
}

#[test]
fn coupled_surface_sponge_is_atomic_and_allocation_free_s268() {
    use water_core::{background::BackgroundSample,delta_projection::{BackgroundFaces,Sponge},SimTime};
    let mut reference=prepared_mobile();
    let u=vec![BackgroundSample::default();reference.velocity_u().len()];
    let w=vec![BackgroundSample::default();reference.velocity_w().len()];
    let bg=BackgroundFaces{domain:reference.domain(),time:SimTime(0),density:1025.,gravity:9.81,u:&u,w:&w};
    let sponge=Sponge{width_m:0.5,rate_per_s:4.};
    let before=bits(&reference);let eta_before=surface_bits(&reference);
    let clock=DeadlineClock::new(usize::MAX);
    let (result,allocs)=measured(||reference.step_perturbation_mobile(SimTime(0),1000,2000,1000,
        &bg,sponge,&Jobs,&clock));
    assert_eq!(allocs,0);let report=result.unwrap();assert_eq!(report.advanced_us,1000);
    let after=bits(&reference);let eta_after=surface_bits(&reference);
    assert_ne!(eta_before,eta_after);
    for cutoff in 1..clock.calls.get() {
        let mut v=prepared_mobile();
        let (r,allocs)=measured(||v.step_perturbation_mobile(SimTime(0),1000,2000,1000,
            &bg,sponge,&Jobs,&DeadlineClock::new(cutoff)));
        assert_eq!(allocs,0);let r=r.unwrap();
        assert_eq!((r.advanced_us,r.remaining_us),(0,1000));
        assert_eq!(bits(&v),before);assert_eq!(surface_bits(&v),eta_before);
        let (r,allocs)=measured(||v.step_perturbation_mobile(SimTime(0),1000,2000,1000,
            &bg,sponge,&Jobs,&DeadlineClock::new(usize::MAX)));
        assert_eq!(allocs,0);assert_eq!(r.unwrap(),report);
        assert_eq!(bits(&v),after);assert_eq!(surface_bits(&v),eta_after);
    }
    println!("S268 : {} expirations, reprise identique et zero allocation",clock.calls.get()-1);
}

#[test]
fn through_background_is_atomic_and_allocation_free_s270() {
    use water_core::{background::BackgroundSample,delta_projection::{BackgroundFaces,Sponge},SimTime};
    let mut reference=prepared_mobile();
    let sample=BackgroundSample{u:[0.5,0.,0.],eta:0.125,p_dyn:(1025f32*9.81)*0.125,..Default::default()};
    let u=vec![sample;reference.velocity_u().len()];
    let w=vec![sample;reference.velocity_w().len()];
    let bg=BackgroundFaces{domain:reference.domain(),time:SimTime(0),density:1025.,gravity:9.81,u:&u,w:&w};
    let sponge=Sponge{width_m:0.5,rate_per_s:4.};
    let before=bits(&reference);let eta_before=surface_bits(&reference);
    let clock=DeadlineClock::new(usize::MAX);
    let (result,allocs)=measured(||reference.step_perturbation_mobile(SimTime(0),1000,2000,1000,
        &bg,sponge,&Jobs,&clock));
    assert_eq!(allocs,0);let report=result.unwrap();assert_eq!(report.advanced_us,1000);
    let after=bits(&reference);let eta_after=surface_bits(&reference);
    assert_ne!(eta_before,eta_after);
    for cutoff in 1..clock.calls.get() {
        let mut v=prepared_mobile();
        let (r,allocs)=measured(||v.step_perturbation_mobile(SimTime(0),1000,2000,1000,
            &bg,sponge,&Jobs,&DeadlineClock::new(cutoff)));
        assert_eq!(allocs,0);let r=r.unwrap();
        assert_eq!((r.advanced_us,r.remaining_us),(0,1000));
        assert_eq!(bits(&v),before);assert_eq!(surface_bits(&v),eta_before);
        let (r,allocs)=measured(||v.step_perturbation_mobile(SimTime(0),1000,2000,1000,
            &bg,sponge,&Jobs,&DeadlineClock::new(usize::MAX)));
        assert_eq!(allocs,0);assert_eq!(r.unwrap(),report);
        assert_eq!(bits(&v),after);assert_eq!(surface_bits(&v),eta_after);
    }
    println!("S270 : {} expirations, reprise identique et zero allocation",clock.calls.get()-1);
}

#[test]
fn shrink_transfers_state_and_runs_without_allocation_s283() {
    use water_core::{background::BackgroundSample, delta_projection::{BackgroundFaces, Sponge}, SimTime};
    let make = |nx| {
        let mut arena = Arena { stats: AllocStats::default(), sealed: false };
        let mut v = Volume::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
            Domain { nx, nz: 12, dx: 0.25 }, 1025., 9.81, &vec![0.; nx]).unwrap();
        let eta: Vec<_> = (0..nx).map(|i| {
            let x = (i as f32 + 0.5 - nx as f32 * 0.5) * 0.25;
            2. + 0.02 * (-x * x / 0.25).exp()
        }).collect();
        v.set_free_surface(&eta, 2.).unwrap();
        v
    };
    let mut source = make(32);
    source.step_surface_mobile(1000, 2000, 1000, &Jobs, &DeadlineClock::new(usize::MAX)).unwrap();
    let mut small = make(16);
    let source_bits = bits(&source);
    let source_eta = surface_bits(&source);
    let (result, allocs) = measured(|| small.shrink_perturbation_from(&source, 8, 1.));
    result.unwrap(); assert_eq!(allocs, 0);
    assert_eq!(bits(&source), source_bits); assert_eq!(surface_bits(&source), source_eta);
    for i in 4..12 {
        assert_eq!(small.surface()[i].to_bits(), source.surface()[8+i].to_bits());
        for k in 0..=12 {
            assert_eq!(small.velocity_w()[k*16+i].to_bits(), source.velocity_w()[k*32+8+i].to_bits());
        }
    }
    for k in 0..12 {
        assert_eq!(small.velocity_u()[k*17], 0.);
        assert_eq!(small.velocity_u()[k*17+16], 0.);
        for i in 4..=12 {
            assert_eq!(small.velocity_u()[k*17+i].to_bits(), source.velocity_u()[k*33+8+i].to_bits());
        }
    }
    assert!(small.pressure().iter().all(|p| *p == 0.));
    let before = (bits(&small), surface_bits(&small));
    for (start, band) in [(usize::MAX, 1.), (17, 1.), (8, f32::NAN), (8, 0.), (8, 2.)] {
        let (r, allocations) = measured(|| small.shrink_perturbation_from(&source, start, band));
        assert!(r.is_err()); assert_eq!(allocations, 0);
        assert_eq!((bits(&small), surface_bits(&small)), before);
    }
    // Même fournisseur zéro, mais faces aux dimensions du NOUVEAU domaine ; vrai pas couplé.
    let u = vec![BackgroundSample::default(); small.velocity_u().len()];
    let w = vec![BackgroundSample::default(); small.velocity_w().len()];
    let bg = BackgroundFaces { domain: small.domain(), time: SimTime(1000), density: 1025.,
        gravity: 9.81, u: &u, w: &w };
    let (r, allocations) = measured(|| small.step_perturbation_mobile(SimTime(1000), 1000,
        2000, 1000, &bg, Sponge { width_m: 1., rate_per_s: 0.5 }, &Jobs,
        &DeadlineClock::new(usize::MAX)));
    assert_eq!(allocations, 0); assert_eq!(r.unwrap().advanced_us, 1000);
    assert_ne!(surface_bits(&small), before.1);
}
#[test]
fn progressive_shrink_damping_is_bounded_and_allocation_free_s284() {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut v = Volume::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain { nx: 32, nz: 12, dx: 0.25 }, 1025., 9.81, &[0.; 32]).unwrap();
    v.set_free_surface(&[2.125; 32], 2.).unwrap();
    let u = vec![0.25; v.velocity_u().len()];
    let w = vec![0.5; v.velocity_w().len()];
    v.set_velocity(&u, &w).unwrap();
    arena.seal();
    let before = (bits(&v), surface_bits(&v));
    for (start, count, band, decay) in [(usize::MAX,16,1.,0.5), (8,16,1.,f32::NAN),
        (8,16,0.,0.5), (8,16,1.,1.1)] {
        let (r, n) = measured(||v.prepare_shrink(start,count,band,decay));
        assert!(r.is_err()); assert_eq!(n,0);
        assert_eq!((bits(&v),surface_bits(&v)),before);
    }
    assert_eq!(v.prepare_shrink(8,16,1.,1.).unwrap(),0.);
    assert_eq!((bits(&v),surface_bits(&v)),before);
    for step in 1..=4 {
        let (r,n) = measured(||v.prepare_shrink(8,16,1.,0.5));
        assert_eq!(n,0);
        assert!(r.unwrap() <= 0.0625);
        assert_eq!(v.surface()[0],2. + 0.125 * 0.5f32.powi(step));
        assert_eq!(v.velocity_w()[0],0.5 * 0.5f32.powi(step));
        for i in 12..20 { assert_eq!(v.surface()[i],2.125); }
        for k in 0..12 { assert_eq!(v.velocity_u()[k*33+16],0.25); }
    }
}
#[test]
fn reference_3d_linear_step_has_no_runtime_allocation_s295() {
    use water_core::delta3d::{Domain3, Volume3};
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut v = Volume3::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx: 12, ny: 8, nz: 6, dx: 0.5 }, 1025., 9.81).unwrap();
    arena.seal();
    let eta: Vec<f32> = (0..12 * 8).map(|c| 3. + 0.02 * ((c % 12) as f32 * 0.7).sin() * ((c / 12) as f32 * 0.4).cos()).collect();
    v.set_surface(&eta).unwrap();
    for _ in 0..20 {
        let (r, allocs) = measured(|| v.step_surface_linear(2000, 2000, &Jobs));
        assert!(r.unwrap().iterations > 0);
        assert_eq!(allocs, 0);
    }
    // Le refus atomique n'alloue pas davantage.
    let (r, allocs) = measured(|| v.step_surface_linear(2000, 1, &Jobs));
    assert_eq!(r.err(), Some(Error::Convergence));
    assert_eq!(allocs, 0);
    assert_eq!(arena.stats.refused_after_seal, 0);
}

#[test]
fn reference_3d_mobile_step_has_no_runtime_allocation_s296() {
    use water_core::delta3d::{Domain3,Volume3};
    let mut arena=Arena{stats:AllocStats::default(),sealed:false};
    let mut v=Volume3::configure(&mut HostServices{alloc:&mut arena,jobs:&Jobs,sink:&Jobs},
        Domain3{nx:12,ny:8,nz:8,dx:0.5},1025.,9.81).unwrap();
    let eta:Vec<_>=(0..96).map(|c|3.+0.02*(c as f32*0.7).sin()).collect();
    v.set_free_surface(&eta,3.).unwrap();arena.seal();
    for _ in 0..20 {
        let (r,n)=measured(||v.step_surface_mobile(1000,4000,&Jobs));
        assert!(!r.unwrap().degraded);assert_eq!(n,0);
    }
    let (r,n)=measured(||v.step_surface_mobile(1000,0,&Jobs));
    assert_eq!(r.err(),Some(Error::Convergence));assert_eq!(n,0);
    assert_eq!(arena.stats.refused_after_seal,0);
}

#[test]
fn coupled_3d_has_no_runtime_allocation_s297() {
    use water_core::{delta3d::{Domain3,Volume3,BackgroundFaces3,Sponge3},background::BackgroundSample,SimTime};
    let mut arena=Arena{stats:AllocStats::default(),sealed:false};
    let mut v=Volume3::configure(&mut HostServices{alloc:&mut arena,jobs:&Jobs,sink:&Jobs},Domain3{nx:8,ny:6,nz:12,dx:0.25},1025.,9.81).unwrap();
    let eta:Vec<_>=(0..48).map(|c|2.+0.05*(c as f32).sin()).collect();v.set_free_surface(&eta,2.).unwrap();
    let u=vec![BackgroundSample::default();v.velocity_u().len()];let vv=vec![BackgroundSample::default();v.velocity_v().len()];let w=vec![BackgroundSample::default();v.velocity_w().len()];
    arena.seal();
    let bg=BackgroundFaces3{domain:v.domain(),time:SimTime(0),density:1025.,gravity:9.81,u:&u,v:&vv,w:&w};
    for _ in 0..20 {
        let (r,n)=measured(||v.step_perturbation_mobile(SimTime(0),1000,4000,&bg,Sponge3::default(),&Jobs));
        r.unwrap();assert_eq!(n,0);
    }
    let (r,n)=measured(||v.step_perturbation_mobile(SimTime(0),1000,0,&bg,Sponge3::default(),&Jobs));
    assert_eq!(r.err(),Some(Error::Convergence));assert_eq!(n,0);
}
