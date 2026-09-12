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
    // Quatre tableaux f32 par famille de faces, fond+surface, fraction ; six f64 par cellule.
    let bytes = (4 * ((n + 1) * k + n * (k + 1)) + 2 * n + n * k) * 4 + 6 * n * k * 8;
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
