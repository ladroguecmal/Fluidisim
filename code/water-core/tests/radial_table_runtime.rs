//! S208, ADR-129, critère (c) : ni `profile` ni `eval` n'allouent. Compteur global d'allocations,
//! actif seulement pendant la section mesurée, comme `delta_runtime.rs` (S200).
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use water_core::impact_field::{Medium, BREAKING_SLOPE};
use water_core::radial_impact::{Domain, RadialImpact};
use water_core::wave_event::{Impact, Origin, WaveEvent};
use water_core::{FrameId, SimTime};
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

#[test]
fn profile_and_eval_do_not_allocate_s208() {
    let event = WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(0),
        ttl_us: 60_000_000,
        position: [0.0, 10.0, 0.0],
        energy_j: 164.0,
        wavelength_m: 3.35,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap();
    let field = RadialImpact::<256>::new(
        event,
        Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 20.0,
            max_slope: BREAKING_SLOPE,
        },
        Domain {
            radius: 52.0,
            age_us: 56_000_000,
        },
    )
    .unwrap();
    let step = 3.35 / 8.0;
    let len = field.table_len(step).unwrap();
    // Allocations de l'hôte, avant la section mesurée : stockage de matrice et profil.
    let mut storage = vec![[0.0f32; 2]; 256 * len];
    let mut profile = vec![(0.0f32, 0.0f32); len];
    let table = field.bake_table(step, &mut storage).unwrap();
    let (sum, calls) = measured(|| {
        let mut sum = 0.0f64;
        for age in [0u64, 3_000_000, 30_000_000, 56_000_000] {
            table.profile(SimTime(age), &mut profile).unwrap();
            for k in 0..500 {
                let r = k as f32 * 0.1;
                let (eta, slope) = table
                    .eval(&profile, FrameId(0), 0, [r * 0.6, 10.0 + r * 0.8])
                    .unwrap();
                sum += eta as f64 + slope[0] as f64;
            }
        }
        sum
    });
    assert!(sum.is_finite());
    assert_eq!(calls, 0, "profil et évaluation doivent rester sans allocation");
    // Témoin : le compteur voit bien une allocation quand il y en a une.
    let (_, witness) = measured(|| vec![0u8; 16]);
    assert!(witness > 0);
}
