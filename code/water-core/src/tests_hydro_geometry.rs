use super::*;

fn box_cells(lo: [i64; 3], hi: [i64; 3]) -> [Tetrahedron; 6] {
    let v: [[i64; 3]; 8] = std::array::from_fn(|bits|
        std::array::from_fn(|axis| if bits & (1 << axis) == 0 { lo[axis] } else { hi[axis] }));
    [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].map(|p| {
        Tetrahedron::new([v[0], v[1 << p[0]], v[(1 << p[0]) | (1 << p[1])], v[7]]).unwrap()
    })
}

fn hull_cells() -> [Tetrahedron; 3] {
    let v = [[0, -500_000, 0], [-2_000_000, -500_000, 2_000_000], [2_000_000, -500_000, 2_000_000],
             [0, 500_000, 0], [-2_000_000, 500_000, 2_000_000], [2_000_000, 500_000, 2_000_000]];
    [[0, 1, 2, 3], [1, 2, 3, 4], [2, 3, 4, 5]].map(|ids|
        Tetrahedron::new(ids.map(|i| v[i])).unwrap())
}

// Oracle indépendant de la partition : intégrales successives sur un pavé. La convolution
// de n segments uniformes donne Σ(-1)^|S| (s-Σw_S)_+^n / (n! Πw). Symétrie près du plein
// pour éviter une soustraction de grands termes presque égaux dans l'oracle.
fn box_oracle(lo: [i64; 3], hi: [i64; 3], plane: SurfacePlane) -> f64 {
    let mut weights = Vec::new();
    let mut bottom = 0.0;
    let mut total = 1.0;
    for axis in 0..3 {
        let width = (hi[axis] - lo[axis]) as f64;
        total *= width;
        bottom += plane.up[axis] * if plane.up[axis] >= 0.0 { lo[axis] } else { hi[axis] } as f64;
        let weight = width * plane.up[axis].abs();
        if weight != 0.0 { weights.push(weight); }
    }
    let span = weights.iter().sum::<f64>();
    let h = plane.offset_um - bottom;
    if h <= 0.0 { return 0.0; }
    if h >= span { return total / 1e12; }
    let complement = h > span * 0.5;
    let h = if complement { span - h } else { h };
    let n = weights.len();
    let mut numerator = 0.0;
    for mask in 0usize..1 << n {
        let shift = weights.iter().enumerate().filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, w)| *w).sum::<f64>();
        let term = (h - shift).max(0.0).powi(n as i32);
        numerator += if mask.count_ones() % 2 == 0 { term } else { -term };
    }
    let factorial = (1..=n).product::<usize>() as f64;
    let f = numerator / (factorial * weights.iter().product::<f64>());
    total / 1e12 * if complement { 1.0 - f } else { f }
}

// Oracle de cale extrudée de longueur 1 m : intégrer la longueur verticale mouillée en x.
// Ses changements de pente sont explicites ; l'intégrale trapézoïdale est exacte entre eux.
fn hull_oracle(plane: SurfacePlane) -> f64 {
    assert_eq!(plane.up[1], 0.0);
    assert!(plane.up[2] > 0.0);
    let h = plane.offset_um * 1e-6 / plane.up[2];
    let slope = -plane.up[0] / plane.up[2];
    let mut cuts = vec![-2.0, 0.0, 2.0];
    for (numerator, denominator) in [(2.0 - h, slope), (h, 1.0 - slope), (h, -1.0 - slope)] {
        if denominator != 0.0 {
            let x = numerator / denominator;
            if (-2.0..2.0).contains(&x) { cuts.push(x); }
        }
    }
    cuts.sort_by(f64::total_cmp);
    let wet = |x: f64| ((h + slope * x).min(2.0) - x.abs()).max(0.0);
    cuts.windows(2).map(|w| (wet(w[0]) + wet(w[1])) * (w[1] - w[0]) * 0.5).sum::<f64>() * 1e6
}

#[test]
fn oriented_box_matches_separable_integrals_s228() {
    let lo = [-2_000_000, -500_000, 0];
    let hi = [2_000_000, 500_000, 2_000_000];
    let cells = box_cells(lo, hi);
    let shape = VolumeShape::new(&cells).unwrap();
    assert_eq!(shape.capacity_ml(), 8_000_000);
    let mut worst: f64 = 0.0;
    for g in [[0., 0., -9.81], [0., 0., 9.81], [9.81, 0., 0.], [-9.81, 0., 0.],
              [0., 9.81, 0.], [0., -9.81, 0.], [2.943, 0., -9.81],
              [2.943, -1.962, -9.81], [1., 1., 1.], [3e-20, -2e-20, -1e-19]] {
        for ml in [0, 1, 8_000, 800_000, 4_000_000, 7_992_000, 7_999_999, 8_000_000] {
            let plane = shape.plane(ml, g).unwrap();
            let error = (box_oracle(lo, hi, plane) - ml as f64).abs();
            worst = worst.max(error);
            assert!(error <= 0.5, "g={g:?}, ml={ml}, erreur={error} ml, plan={plane:?}");
        }
    }
    println!("BOX_S228 max_error_ml={worst:.9}");
}

#[test]
fn oriented_hull_preserves_volume_at_floor_and_roof_s228() {
    let cells = hull_cells();
    let shape = VolumeShape::new(&cells).unwrap();
    assert_eq!(shape.capacity_ml(), 4_000_000);
    let mut worst: f64 = 0.0;
    for gx in [-19.62, -9.81, -4.905, -2.943, 0.0, 2.943, 4.905, 9.81, 19.62] {
        for ml in [0, 1, 1_000, 160_000, 1_000_000, 2_560_000, 3_999_999, 4_000_000] {
            let plane = shape.plane(ml, [gx, 0., -9.81]).unwrap();
            let got = hull_oracle(plane);
            let error = (got - ml as f64).abs();
            worst = worst.max(error);
            assert!(error <= 0.5, "gx={gx}, volume={ml}, obtenu={got}, plan={plane:?}");
        }
    }
    println!("HULL_S228 max_error_ml={worst:.9}");
}

#[test]
fn nonconvex_shape_is_a_volume_not_a_convex_hull_s228() {
    let a = ([0, 0, 0], [2_000_000, 1_000_000, 1_000_000]);
    let b = ([0, 1_000_000, 0], [1_000_000, 2_000_000, 1_000_000]);
    let cells: Vec<_> = box_cells(a.0, a.1).into_iter().chain(box_cells(b.0, b.1)).collect();
    let shape = VolumeShape::new(&cells).unwrap();
    assert_eq!(shape.capacity_ml(), 3_000_000);
    for g in [[2., -3., -9.], [-3., 2., 9.], [0., -9., 0.]] {
        for ml in [1, 100_000, 1_500_000, 2_999_999] {
            let p = shape.plane(ml, g).unwrap();
            let expected = box_oracle(a.0, a.1, p) + box_oracle(b.0, b.1, p);
            assert!((expected - ml as f64).abs() <= 0.5, "{expected} au lieu de {ml}");
        }
    }
}

#[test]
fn vertex_order_and_equal_projections_do_not_change_geometry_s228() {
    let lo = [0; 3];
    let hi = [1_000_000; 3];
    let original = box_cells(lo, hi);
    let mut count = 0;
    for a in 0..4 { for b in 0..4 { for c in 0..4 { for d in 0..4 {
        let p = [a, b, c, d];
        if (0..4).any(|i| p[..i].contains(&p[i])) { continue; }
        count += 1;
        let cells = original.map(|t| Tetrahedron::new(p.map(|i| t.vertices[i])).unwrap());
        let shape = VolumeShape::new(&cells).unwrap();
        for g in [[0., 0., -9.], [1., 1., 1.], [1., 1., 0.]] {
            for ml in [1, 250_000, 500_000, 999_999] {
                let plane = shape.plane(ml, g).unwrap();
                assert!((box_oracle(lo, hi, plane) - ml as f64).abs() <= 0.5);
            }
        }
    }}}}
    assert_eq!(count, 24);
}

#[test]
fn geometry_refuses_degeneracy_overlap_and_out_of_domain_s228() {
    assert_eq!(Tetrahedron::new([[0; 3]; 4]).err(), Some(Error::Shape));
    let outside = [[i64::MIN, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1]];
    assert_eq!(Tetrahedron::new(outside).err(), Some(Error::Domain));
    let at_limit = [[4_096_000_000, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1]];
    assert_eq!(Tetrahedron::new(at_limit).err(), Some(Error::Domain));
    assert_eq!(VolumeShape::new(&[]).err(), Some(Error::Shape));
    let t = Tetrahedron::new([[0; 3], [1_000_000, 0, 0], [0, 1_000_000, 0], [0, 0, 1_000_000]]).unwrap();
    assert_eq!(VolumeShape::new(&[t, t]).err(), Some(Error::Shape));
    // Deux tétraèdres entrelacés sans sommet contenu : tester seulement les sommets échouerait.
    let points = [[1, 1, 1], [-1, -1, 1], [-1, 1, -1], [1, -1, -1]];
    let a = Tetrahedron::new(points.map(|p| p.map(|x| x * 1_000_000))).unwrap();
    let b = Tetrahedron::new(points.map(|p| p.map(|x| -x * 1_000_000))).unwrap();
    assert_eq!(VolumeShape::new(&[a, b]).err(), Some(Error::Shape));
    let cells = [t];
    let shape = VolumeShape::new(&cells).unwrap();
    assert_eq!(shape.capacity_ml(), 166_667); // volume exact = 1/6 m³.
    assert_eq!(shape.plane(-1, [0., 0., -1.]).err(), Some(Error::Capacity));
    assert_eq!(shape.plane(166_668, [0., 0., -1.]).err(), Some(Error::Capacity));
    for g in [[0.; 3], [f32::NAN, 0., -1.], [0., f32::INFINITY, 0.]] {
        assert_eq!(shape.plane(10, g).err(), Some(Error::Domain));
    }
}

#[test]
fn local_translation_thin_geometry_and_resolution_refusal_s228() {
    let lo = [3_000_000_000; 3];
    let hi = [3_001_000_000, 3_001_000_000, 3_000_001_000]; // dalle de 1 mm à 3 km de l'origine.
    let cells = box_cells(lo, hi);
    let shape = VolumeShape::new(&cells).unwrap();
    assert_eq!(shape.capacity_ml(), 1_000);
    for ml in [1, 250, 500, 999] {
        let p = shape.plane(ml, [2., -3., -9.]).unwrap();
        assert!((box_oracle(lo, hi, p) - ml as f64).abs() <= 0.5);
    }
    // À ce volume, f64 n'a plus tous les millilitres : refuser le résidu, sans arrondir la
    // consigne entière vers un volume représentable. Aucun seuil physique n'est assoupli.
    let large = box_cells([0; 3], [3_000_000_000; 3]);
    let shape = VolumeShape::new(&large).unwrap();
    assert_eq!(shape.plane((1i64 << 53) + 1, [0., 0., -9.81]).err(), Some(Error::Resolution));
}
