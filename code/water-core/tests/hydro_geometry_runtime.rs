//! Compteur réel du tas : plan et pas orientés, avec contre-épreuve positive (I-06).
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use water_core::hydro_network::{geometry::{Tetrahedron, VolumeShape}, *};
use water_core::SimTime;

thread_local! {
    static TRACK: Cell<bool> = const { Cell::new(false) };
    static CALLS: Cell<usize> = const { Cell::new(0) };
}
struct Counter;
fn allocation() {
    let _ = TRACK.try_with(|t| if t.get() { let _ = CALLS.try_with(|c| c.set(c.get() + 1)); });
}
unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        allocation(); unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        allocation(); unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn realloc(&self, p: *mut u8, layout: Layout, n: usize) -> *mut u8 {
        allocation(); unsafe { System.realloc(p, layout, n) }
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) { unsafe { System.dealloc(p, layout) } }
}
#[global_allocator]
static ALLOCATOR: Counter = Counter;
fn measured<T>(f: impl FnOnce() -> T) -> (T, usize) {
    struct Stop;
    impl Drop for Stop { fn drop(&mut self) { TRACK.with(|t| t.set(false)); } }
    CALLS.with(|c| c.set(0));
    TRACK.with(|t| t.set(true));
    let stop = Stop;
    let result = f();
    drop(stop);
    (result, CALLS.with(Cell::get))
}

#[test]
fn oriented_planes_and_real_steps_do_not_allocate_s228() {
    let (_, witness) = measured(|| std::hint::black_box(vec![0u8; 64]));
    assert!(witness > 0, "le compteur doit voir une allocation reelle");
    let v: [[i64; 3]; 8] = std::array::from_fn(|bits|
        [if bits & 1 == 0 { -2_000_000 } else { 2_000_000 },
         if bits & 2 == 0 { -500_000 } else { 500_000 },
         if bits & 4 == 0 { 0 } else { 2_000_000 }]);
    let cells = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].map(|p|
        Tetrahedron::new([v[0], v[1 << p[0]], v[(1 << p[0]) | (1 << p[1])], v[7]]).unwrap());
    let volumes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&volumes).unwrap();
    let mut nodes = [4_080_000, 4_000_000].map(|volume_ml| HydroNode {
        volume_ml, capacity_ml: 8_000_000, origin_um: [0; 3], shape: 0,
    });
    let mut edges = [Opening { from: 0, to: Some(1), flow: Flow::Orifice { area_mm2: 1_000 },
        position_um: [0; 3], discharge: SHARP_EDGE_DISCHARGE, residue_nl: 0 }];
    let (result, calls) = measured(|| {
        for g in [[0., 0., -9.81], [2.943, 0., -9.81], [9.81, 0., 0.],
                  [0., 0., 9.81], [3., -2., -9.]] {
            for _ in 0..20 {
                let plane = shapes.surface_plane(&nodes[0], g)?;
                std::hint::black_box(plane);
                step(&mut nodes, &mut edges, &shapes, g, SimTime(STEP_US), &mut [0])?;
            }
        }
        Ok::<_, Error>(())
    });
    assert_eq!(result, Ok(()));
    assert_eq!(calls, 0);
    assert!(nodes[1].volume_ml > 4_000_000, "temoin de transfert effectif");
    assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>(), 8_080_000);
    let before = nodes;
    let rest = edges[0].residue_nl;
    let (refusal, calls) = measured(||
        step(&mut nodes, &mut edges, &shapes, [0.; 3], SimTime(STEP_US), &mut [0]));
    assert_eq!(refusal, Err(Error::Domain));
    assert_eq!(calls, 0);
    assert_eq!(nodes, before);
    assert_eq!(edges[0].residue_nl, rest);
}
