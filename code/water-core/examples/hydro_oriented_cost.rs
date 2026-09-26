//! S228 : coût du pas V réellement consommé, géométrie orientée. Aucune réception de budget.
use std::hint::black_box;
use std::time::Instant;
use water_core::hydro_network::{geometry::{Tetrahedron, VolumeShape}, *};
use water_core::SimTime;

fn main() {
    let v: [[i64; 3]; 8] = std::array::from_fn(|bits|
        [if bits & 1 == 0 { -2_000_000 } else { 2_000_000 },
         if bits & 2 == 0 { -500_000 } else { 500_000 },
         if bits & 4 == 0 { 0 } else { 2_000_000 }]);
    let cells = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].map(|p|
        Tetrahedron::new([v[0], v[1 << p[0]], v[(1 << p[0]) | (1 << p[1])], v[7]]).unwrap());
    let volumes = [VolumeShape::new(&cells).unwrap()];
    let shapes = Shapes::from_volumes(&volumes).unwrap();
    println!("S228 present=oriented_tetrahedra,bisection,integer_transfers,collective_limits \
              absent=plane_cache,adjacency_index,budget_interrupt,pressurised_network \
              g=[2.943,0,-9.81] dt_us={STEP_US} cells_per_shape=6 geometry_shared=true \
              warmup_steps=100 measured_steps=500 series_per_case=1 \
              metric=complete_cpu_step_with_timestamp_overhead initialization_excluded=true");
    for (topology, n) in [("leaks", 1usize), ("leaks", 16), ("leaks", 64), ("ring", 16), ("ring", 64)] {
        // Contenants indépendants qui fuient par leur hublot latéral ; pas artificiellement
        // remis à l'état initial entre mesures, volumes et restes continuent leur évolution.
        let mut nodes = vec![HydroNode { volume_ml: 4_000_000, capacity_ml: 8_000_000,
            origin_um: [0; 3], shape: 0 }; n];
        if topology == "ring" {
            // Anneau de contenants à surface libre, sans réseau sous pression. Une différence
            // de cote de 1 mm alimente les arêtes descendantes ; la dernière ne remonte pas.
            for (i, node) in nodes.iter_mut().enumerate() { node.origin_um[2] = ((n - i) * 1_000) as i64; }
        }
        let mut edges: Vec<_> = (0..n).map(|from| Opening {
            from: from as u16, to: if topology == "ring" { Some(((from + 1) % n) as u16) } else { None },
            flow: Flow::Orifice { area_mm2: 1_000 },
            position_um: if topology == "ring" { nodes[from].origin_um } else { [2_000_000, 0, 1_200_000] },
            discharge: SHARP_EDGE_DISCHARGE, residue_nl: 0,
            control_pm: CONTROL_FULL,
        }).collect();
        let mut scratch = vec![0; n];
        let g = black_box([2.943, 0., -9.81]);
        for _ in 0..100 {
            step(&mut nodes, &mut edges, &shapes, g, SimTime(STEP_US), &mut scratch).unwrap();
        }
        let mut ns = Vec::with_capacity(500);
        for _ in 0..500 {
            let start = Instant::now();
            step(black_box(&mut nodes), black_box(&mut edges), black_box(&shapes), g,
                 SimTime(STEP_US), black_box(&mut scratch)).unwrap();
            ns.push(start.elapsed().as_nanos() as u64);
        }
        ns.sort_unstable();
        let total_leaked = 4_000_000 * n as i64 - nodes.iter().map(|v| v.volume_ml).sum::<i64>();
        if topology == "ring" { assert_eq!(total_leaked, 0); }
        assert!(nodes.iter().any(|v| v.volume_ml != 4_000_000));
        println!("STEP_V topology={topology} nodes={n} edges={n} p50_us={:.3} p95_us={:.3} max_us={:.3} \
                  final_first_node_ml={} total_leaked_ml={}", ns[250] as f64 / 1e3,
                 ns[475] as f64 / 1e3, ns[499] as f64 / 1e3, nodes[0].volume_ml,
                 total_leaked);
    }
}
