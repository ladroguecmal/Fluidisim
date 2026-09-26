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
        position_um: [0; 3], discharge: SHARP_EDGE_DISCHARGE, residue_nl: 0, control_pm: CONTROL_FULL }];
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

#[test]
fn c19_oriented_snapshot_continues_real_network_without_allocation_s229() {
    use water_core::hydro_network::snapshot::{Baseline, Context, SnapshotError};
    let (_, witness) = measured(|| std::hint::black_box(vec![0u8; 64]));
    assert!(witness > 0);
    let v: [[i64; 3]; 8] = std::array::from_fn(|bits|
        [if bits & 1 == 0 { -2_000_000 } else { 2_000_000 },
         if bits & 2 == 0 { -500_000 } else { 500_000 },
         if bits & 4 == 0 { 0 } else { 2_000_000 }]);
    let cells = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].map(|p|
        Tetrahedron::new([v[0], v[1 << p[0]], v[(1 << p[0]) | (1 << p[1])], v[7]]).unwrap());
    let volumes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&volumes).unwrap();
    let original = [4_000_000, 1_000_000, 250_000].map(|volume_ml| HydroNode {
        volume_ml, capacity_ml: 8_000_000, origin_um: [0; 3], shape: 0,
    });
    let opening = Opening { from: 0, to: Some(1), flow: Flow::Orifice { area_mm2: 1 },
        position_um: [0; 3], discharge: SHARP_EDGE_DISCHARGE, residue_nl: 0, control_pm: CONTROL_FULL };
    let initial_edges = [opening,
        Opening { to: Some(2), flow: Flow::Weir { width_mm: 1 }, ..opening },
        Opening { from: 1, to: Some(2), ..opening },
        Opening { from: 2, to: None, flow: Flow::Orifice { area_mm2: 1_000 }, ..opening }];
    let base = Baseline::new(31, 9, &original, &initial_edges, &shapes).unwrap();
    let (mut nodes, mut edges) = (original, initial_edges);
    let mut ctx = Context { time: SimTime(0), dt: SimTime(STEP_US), g_eff: [2.943, 0., -9.81] };
    for _ in 0..3 {
        step(&mut nodes, &mut edges, &shapes, ctx.g_eff, ctx.dt, &mut [0; 4]).unwrap();
        ctx.time.0 += ctx.dt.0;
    }
    assert!(edges.iter().any(|e| e.residue_nl != 0));
    let mut wire = [0xaa; 256];
    let (capture, allocs) = measured(|| base.snapshot_into(&nodes, &edges, ctx, &mut wire));
    let len = capture.unwrap(); assert_eq!(allocs, 0);
    let (mut resumed, mut resumed_edges) = ([HydroNode::default(); 3], [Opening::default(); 4]);
    let (restoration, allocs) = measured(|| base.restore_into(&wire[..len], &mut resumed, &mut resumed_edges));
    let mut resumed_ctx = restoration.unwrap(); assert_eq!(allocs, 0);
    let mass_at_capture = nodes.iter().map(|n| n.volume_ml).sum::<i64>();
    let (mut discharged, mut restarted_discharged) = (0, 0);
    let mut next_wire = [0; 256];
    let ((), allocs) = measured(|| {
        for k in 0..200 {
            // Même programme d'entrées futures, à partir du contexte réellement restauré.
            if k % 40 == 0 {
                let g = [[2.943, 0., -9.81], [0., -3., -9.81], [9.81, 0., 0.],
                    [0., 0., 9.81], [-1., 2., -9.]][k / 40];
                ctx.g_eff = g; resumed_ctx.g_eff = g;
            }
            let mut transfers = [0; 4]; let mut restarted_transfers = [0; 4];
            step(&mut nodes, &mut edges, &shapes, ctx.g_eff, ctx.dt, &mut transfers).unwrap();
            step(&mut resumed, &mut resumed_edges, &shapes, resumed_ctx.g_eff, resumed_ctx.dt, &mut restarted_transfers).unwrap();
            ctx.time.0 += ctx.dt.0; resumed_ctx.time.0 += resumed_ctx.dt.0;
            assert_eq!(nodes, resumed); assert_eq!(transfers, restarted_transfers);
            assert_eq!(edges.map(|e| e.residue_nl), resumed_edges.map(|e| e.residue_nl));
            discharged += transfers[3]; restarted_discharged += restarted_transfers[3];
            for (a, b) in nodes.iter().zip(&resumed) {
                let a = shapes.surface_plane(a, ctx.g_eff).unwrap();
                let b = shapes.surface_plane(b, resumed_ctx.g_eff).unwrap();
                assert_eq!(a.offset_um.to_bits(), b.offset_um.to_bits());
                assert_eq!(a.up.map(f64::to_bits), b.up.map(f64::to_bits));
            }
            let n = base.snapshot_into(&nodes, &edges, ctx, &mut wire).unwrap();
            let m = base.snapshot_into(&resumed, &resumed_edges, resumed_ctx, &mut next_wire).unwrap();
            assert_eq!(n, m); assert_eq!(wire[..n], next_wire[..m]);
            if k == 49 || k == 99 {
                resumed.fill(HydroNode::default()); resumed_edges.fill(Opening::default());
                resumed_ctx = base.restore_into(&next_wire[..m], &mut resumed, &mut resumed_edges).unwrap();
            }
        }
    });
    assert_eq!(allocs, 0);
    assert!(discharged > 0, "le rejet exterieur doit etre consomme");
    assert_eq!(discharged, restarted_discharged);
    assert_eq!(nodes.iter().map(|n| n.volume_ml).sum::<i64>() + discharged, mass_at_capture);
    let before = resumed; let rests = resumed_edges.map(|e| e.residue_nl);
    let (error, allocs) = measured(|| base.restore_into(&wire[..87], &mut resumed, &mut resumed_edges));
    assert_eq!(error, Err(SnapshotError::Length)); assert_eq!(allocs, 0);
    assert_eq!(resumed, before); assert_eq!(resumed_edges.map(|e| e.residue_nl), rests);
    let (error, allocs) = measured(|| base.snapshot_into(&nodes, &edges, ctx, &mut [0; 1]));
    assert_eq!(error, Err(SnapshotError::Capacity)); assert_eq!(allocs, 0);
    println!("S229 C19-V oriente: 200 pas, 3 restaurations, {len} octets initiaux, {discharged} ml rejetes; zero allocation");
}
