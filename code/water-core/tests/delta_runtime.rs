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
        assert!(r.iterations <= limit);
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
    // Quatre tableaux f32 par famille de faces, fond+surface, fraction ; six f32 par cellule.
    let bytes = (4 * ((n + 1) * k + n * (k + 1)) + 2 * n + n * k) * 4 + 6 * n * k * 4;
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
