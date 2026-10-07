//! S588 — la polyligne de déferlement. Références écrites au plan par son script (des formules indépendantes de `bathymetrie.rs`).

use super::*;

/// Une plage `h = 0,02·x`, une grille de 5 m, 60 × 8 nœuds.
fn plage() -> Vec<f64> {
    let (nx, ny) = (60usize, 8usize);
    (0..nx * ny).map(|c| 0.02 * ((c % nx) as f64 * 5.0)).collect()
}

/// (1) Les sommets ; (2) le flux et la direction ; (3) une houle qui ne déferle pas dans la grille ; (4) les refus.
#[test]
fn a_plane_beach_breaks_along_a_straight_line_s588() {
    let h = plage();
    let houle = Houle { omega: 2.0 * std::f64::consts::PI / 8.0, theta0: 20f64.to_radians(), hauteur0: 2.0 };
    let (mut ecart, mut sortie) = (vec![0.0; h.len()], vec![Sommet::default(); 8]);
    let n = polyligne([0.0, 0.0], 5.0, 60, 8, &h, houle, 9.81, 1025.0, &mut ecart, &mut sortie).unwrap();
    let s = sortie[0];
    let angle = s.direction_crete[1].atan2(-s.direction_crete[0]).to_degrees();
    println!("S588 : {n} sommets ; x = {:.4} m (algorithme 142,1033 ; racine 142,0982) ; flux {:.6} kW/m (29,799753) ; crête à {angle:.4}° (8,0632)",
        s.pos[0], s.dissipe_kw_par_m);
    assert_eq!(n, 8, "critère 1 : un sommet par ligne");
    for (j, v) in sortie.iter().enumerate() {
        assert!((v.pos[0] - 142.10331625045018).abs() < 1e-3 && v.pos[1] == j as f64 * 5.0, "critère 1 : l'algorithme, ligne {j}");
        assert!((v.pos[0] - 142.09815451635566).abs() < 0.0103, "critère 1 : la racine, ligne {j}");
    }
    assert!((s.dissipe_kw_par_m / 29.799752611303084 - 1.0).abs() < 1e-3, "critère 2 : le flux");
    assert!((angle - 8.06321398083961).abs() < 0.01, "critère 2 : la direction");
    // (3) Une houle de 1 cm déferle près du bord, hors de la grille (sous le premier nœud mouillé, à 0,1 m) : aucun sommet.
    let petite = Houle { hauteur0: 0.01, ..houle };
    let m = polyligne([0.0, 0.0], 5.0, 60, 8, &h, petite, 9.81, 1025.0, &mut ecart, &mut sortie).unwrap();
    println!("S588 : une houle de 1 cm, {m} sommets");
    assert_eq!(m, 0, "critère 3");
    assert_eq!(polyligne([0.0, 0.0], 5.0, 1, 8, &h[..8], houle, 9.81, 1025.0, &mut ecart, &mut sortie), Err(Refus), "critère 4 : grille");
    assert_eq!(polyligne([0.0, 0.0], 5.0, 60, 8, &h, houle, 9.81, 1025.0, &mut ecart, &mut sortie[..7]), Err(Refus), "critère 4 : tampon");
}

/// **S630** — une grille d'îles coniques (pente 0,02, rivage à 100 m) de 201 × 201 nœuds à 5 m, origine (−500, −500).
fn iles(centres: &[[f64; 2]]) -> Vec<f64> {
    let mut p = Vec::with_capacity(201 * 201);
    for j in 0..201 {
        for i in 0..201 {
            let (x, y) = (-500.0 + 5.0 * i as f64, -500.0 + 5.0 * j as f64);
            let r = centres.iter().map(|c| (x - c[0]).hypot(y - c[1])).fold(f64::INFINITY, f64::min);
            p.push(if r > 100.0 { 0.02 * (r - 100.0) } else { -1.0 });
        }
    }
    p
}

/// **S630** — (1) la côte droite ; (2) l'île ; (3) deux îles ; (4) refus.
#[test]
fn the_breaking_contour_follows_any_coast_s630() {
    let g = 9.81;
    let omega = 2.0 * core::f64::consts::PI / 8.0;
    let oblique = Houle { omega, theta0: 0.2, hauteur0: 1.5 };
    let (nx, ny) = (121usize, 11usize);
    let fond: Vec<f64> = (0..nx * ny).map(|k| { let x = 5.0 * (k % nx) as f64; if x > 100.0 { 0.02 * (x - 100.0) } else { -1.0 } }).collect();
    let mut ecart = vec![0.0; nx * ny];
    let mut sommets = vec![Sommet::default(); ny];
    let n = polyligne([0.0, 0.0], 5.0, nx, ny, &fond, oblique, g, 1025.0, &mut ecart, &mut sommets).unwrap();
    let lignes = contours([0.0, 0.0], 5.0, nx, ny, &fond, oblique, g, &mut ecart).unwrap();
    assert_eq!(lignes.len(), 1, "critère 1 : une polyligne");
    let mut pts = lignes[0].clone();
    pts.sort_by(|a, b| a[1].total_cmp(&b[1]));
    println!("S630 : côte droite — {} sommets (S588 : {n})", pts.len());
    assert_eq!(pts.len(), n, "critère 1 : autant de sommets");
    for (p, s) in pts.iter().zip(&sommets[..n]) {
        assert!((p[0] - s.pos[0]).abs() < 1e-12 && (p[1] - s.pos[1]).abs() < 1e-12, "critère 1 : {p:?} contre {:?}", s.pos);
    }

    let normale = Houle { omega, theta0: 0.0, hauteur0: 1.5 };
    let r_b = 214.45894698009943;
    let fond1 = iles(&[[0.0, 0.0]]);
    let mut e1 = vec![0.0; 201 * 201];
    let l1 = contours([-500.0, -500.0], 5.0, 201, 201, &fond1, normale, g, &mut e1).unwrap();
    let ferme = |l: &Vec<[f64; 2]>| l.first() == l.last() && l.len() > 3;
    let err = l1[0].iter().map(|p| (p[0].hypot(p[1]) - r_b).abs()).fold(0.0, f64::max);
    println!("S630 : île — {} polyligne(s), {} sommets distincts, écart radial max {err} m", l1.len(), l1[0].len() - 1);
    assert!(l1.len() == 1 && ferme(&l1[0]) && l1[0].len() - 1 == 340, "critère 2 : une polyligne fermée de 340 sommets");
    assert!((err - 0.012357133934585818).abs() < 1e-9, "critère 2 : l'écart radial");

    let fond2 = iles(&[[-200.0, 0.0], [250.0, 0.0]]);
    let l2 = contours([-500.0, -500.0], 5.0, 201, 201, &fond2, normale, g, &mut e1).unwrap();
    let total: usize = l2.iter().map(|l| l.len() - 1).sum();
    println!("S630 : deux îles — {} polylignes, {total} sommets", l2.len());
    assert!(l2.len() == 2 && l2.iter().all(ferme) && total == 680, "critère 3");

    assert_eq!(contours([0.0, 0.0], 5.0, 1, 4, &[1.0; 4], normale, g, &mut [0.0; 4]), Err(Refus), "critère 4 : grille");
    assert_eq!(contours([0.0, 0.0], 0.0, 2, 2, &[1.0; 4], normale, g, &mut [0.0; 4]), Err(Refus), "critère 4 : pas");
    assert_eq!(contours([0.0, 0.0], 5.0, 2, 2, &[1.0; 4], normale, g, &mut [0.0; 3]), Err(Refus), "critère 4 : tampon");
}

/// **S632** — (1)–(2) la côte droite contre numpy et l'analytique ; (3) l'île en miroir ; (4) refus.
#[test]
fn breaking_along_wave_rays_matches_snell_and_bends_around_an_island_s632() {
    use crate::portee::tracer_houle;
    use crate::refraction::Point;
    let (g, periode) = (9.81, 8.0);
    let omega = 2.0 * core::f64::consts::PI / periode;
    let droite = |x: f64, _y: f64| (0.02 * x, [0.02, 0.0]);
    let th = core::f64::consts::PI - 0.3;
    let xb = 112.5994530931984;
    let mut errs = Vec::new();
    for (dt, r) in [(2.0, 112.61847330202292), (1.0, 112.60477253802767), (0.5, 112.6004762350802)] {
        let (mut a, mut b) = (vec![Point::default(); 20_001], vec![Point::default(); 20_001]);
        let na = tracer_houle([6000.0, 0.0], th, periode, g, &droite, 0.5, dt, &mut a).unwrap();
        let nb = tracer_houle([6000.0, 10.0], th, periode, g, &droite, 0.5, dt, &mut b).unwrap();
        let p = sur_rayons(&a[..na], &b[..nb], 10.0 * th.cos().abs(), omega, 1.5, &droite, g).unwrap().unwrap();
        println!("S632 : côte droite, pas {dt} s — déferlement en x = {} (analytique {xb}, écart {:e})", p[0], (p[0] - xb).abs());
        assert!((p[0] - r).abs() < 1e-9, "critère 1 : {dt}");
        errs.push((p[0] - xb).abs());
    }
    assert!(errs[0] > errs[1] && errs[1] > errs[2] && errs[2] < 2e-3, "critère 2");

    let ile = |x: f64, y: f64| {
        let r = x.hypot(y);
        if r > 100.0 { (0.02 * (r - 100.0), [0.02 * x / r, 0.02 * y / r]) } else { (-1.0, [0.0, 0.0]) }
    };
    let trace = |y0: f64| {
        let mut v = vec![Point::default(); 20_001];
        let n = tracer_houle([5200.0, y0], core::f64::consts::PI, periode, g, &ile, 0.5, 1.0, &mut v).unwrap();
        v.truncate(n);
        v
    };
    for y in [200.0, 300.0] {
        let haut = sur_rayons(&trace(y), &trace(y + 10.0), 10.0, omega, 1.5, &ile, g).unwrap().unwrap();
        let bas = sur_rayons(&trace(-y), &trace(-y - 10.0), 10.0, omega, 1.5, &ile, g).unwrap().unwrap();
        println!("S632 : île, rayon {y} m — déferle en {haut:?} ; miroir {bas:?}");
        assert!((haut[0] - bas[0]).abs() < 1e-9 && (haut[1] + bas[1]).abs() < 1e-9, "critère 3 : {y}");
    }
    let un = [Point::default()];
    assert_eq!(sur_rayons(&un, &un, 0.0, omega, 1.5, &droite, g), Err(Refus), "critère 4 : b0");
    assert_eq!(sur_rayons(&[], &un, 10.0, omega, 1.5, &droite, g), Err(Refus), "critère 4 : rayon vide");
}

/// **S633** — (1) cinq sommets dans l'ordre, le premier contre numpy ; (2) contre l'analytique ; (3) refus.
#[test]
fn vertices_along_a_ray_bundle_carry_flux_and_crest_direction_s633() {
    use crate::portee::tracer_houle;
    use crate::refraction::Point;
    let (g, periode) = (9.81, 8.0);
    let omega = 2.0 * core::f64::consts::PI / periode;
    let droite = |x: f64, _y: f64| (0.02 * x, [0.02, 0.0]);
    let th = core::f64::consts::PI - 0.3;
    let rayons: Vec<Vec<Point>> = (0..5).map(|i| {
        let mut v = vec![Point::default(); 20_001];
        let n = tracer_houle([6000.0, 10.0 * i as f64], th, periode, g, &droite, 0.5, 0.5, &mut v).unwrap();
        v.truncate(n);
        v
    }).collect();
    let refs: Vec<&[Point]> = rayons.iter().map(|v| v.as_slice()).collect();
    let s = sommets_sur_rayons(&refs, 10.0 * th.cos().abs(), omega, 1.5, &droite, g, 1025.0).unwrap();
    assert!(s.len() == 5 && s.iter().all(|v| v.is_some()), "critère 1 : cinq sommets");
    let s: Vec<Sommet> = s.into_iter().map(|v| v.unwrap()).collect();
    println!("S633 : premier sommet {:?}", s[0]);
    assert!((s[0].pos[0] - 112.6004762350802).abs() < 1e-9 && (s[0].dissipe_kw_par_m - 16.973662175104796).abs() < 1e-9
        && (s[0].direction_crete[0] + 0.9940882843963825).abs() < 1e-9 && (s[0].direction_crete[1] - 0.1085747798793847).abs() < 1e-9,
        "critère 1 : le premier sommet");
    assert!(s.windows(2).all(|w| w[0].pos[1] < w[1].pos[1]), "critère 1 : dans l'ordre");
    assert!((s[0].dissipe_kw_par_m / 16.973287676322084 - 1.0).abs() < 1e-4, "critère 2 : le flux");
    assert!((s[0].direction_crete[0] + 0.994088221713009).abs() < 1e-5 && (s[0].direction_crete[1] - 0.10857535379388544).abs() < 1e-5,
        "critère 2 : la direction");
    assert!(s.iter().all(|v| (v.pos[0] - s[0].pos[0]).abs() < 1e-9), "critère 2 : la côte droite");
    assert_eq!(sommets_sur_rayons(&refs[..1], 10.0, omega, 1.5, &droite, g, 1025.0), Err(Refus), "critère 3");
}
