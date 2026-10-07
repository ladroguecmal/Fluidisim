//! S603 — la portée d'une modification de bathymétrie. Références écrites au plan par son script (un traceur numpy indépendant).

use super::*;

const PENTE: f64 = 1.0 / 200.0;
const TOL: f64 = 0.25;

fn scene() -> Scene {
    let mut departs = Vec::new();
    for i in 0..121 {
        for th in [core::f64::consts::PI - 0.35, core::f64::consts::PI, core::f64::consts::PI + 0.35] {
            departs.push(([24_000.0, -12_000.0 + 200.0 * i as f64], th));
        }
    }
    let bornes_y = (0..9).map(|i| -8000.0 + 2000.0 * i as f64).collect();
    Scene { periode_s: 8.0, g: 9.81, dt_s: 2.0, h_arrivee_m: 5.0, h_coupe_m: 2.5, departs, bornes_y, points_max: 2000 }
}

fn plateau(x: f64, _y: f64) -> (f64, [f64; 2]) {
    (x * PENTE, [PENTE, 0.0])
}

/// Le plateau moins une bosse `A·cos²(π·r/2R)` (un haut-fond).
fn bosse(centre: [f64; 2], r_max: f64, a: f64) -> impl Fn(f64, f64) -> (f64, [f64; 2]) {
    move |x, y| {
        let (dx, dy) = (x - centre[0], y - centre[1]);
        let r = dx.hypot(dy);
        let (h, g) = plateau(x, y);
        if r >= r_max {
            return (h, g);
        }
        let ang = core::f64::consts::PI * r / (2.0 * r_max);
        let db = -a * (2.0 * ang).sin() * core::f64::consts::PI / (2.0 * r_max);
        let rs = if r > 0.0 { r } else { 1.0 };
        (h - a * ang.cos().powi(2), [g[0] - db * dx / rs, g[1] - db * dy / rs])
    }
}

/// Les plages dont une arrivée change de plus d'un demi-texel (les deux plages du rayon), et le plus grand décalage.
fn brut(s: &Scene, a: &[Arrivee], b: &[Arrivee]) -> (Vec<usize>, f64) {
    let (mut plages, mut max) = (Vec::new(), 0.0f64);
    for (u, v) in a.iter().zip(b) {
        let (ya, yb) = (u.y.unwrap(), v.y);
        let d = yb.map_or(f64::INFINITY, |y| (y - ya).abs());
        if yb.is_some() {
            max = max.max(d);
        }
        if d > TOL {
            plages.extend(s.plage(ya));
            plages.extend(yb.and_then(|y| s.plage(y)));
        }
    }
    plages.sort_unstable();
    plages.dedup();
    (plages, max)
}

/// (1) l'isobathe limite, `dc/dh` ; (2) les neuf arrivées ; (3) la bosse profonde ; (4) entre λ/2 et λ ; (5) la bosse côtière ; (6) refus.
#[test]
fn a_bathymetry_edit_rebakes_only_the_beaches_its_rays_reach_s603() {
    let h_lim = isobathe_limite_m(8.0, 9.81).unwrap();
    println!("S603 : isobathe limite {h_lim} m");
    assert!((h_lim - 99.923142536).abs() < 1e-9, "critère 1 : l'isobathe");
    for h in [5.0, 30.0, 90.0] {
        let (_, dc) = celerite(h, 8.0, 9.81);
        let e = 1e-4 * h;
        let fd = (celerite(h + e, 8.0, 9.81).0 - celerite(h - e, 8.0, 9.81).0) / (2.0 * e);
        println!("S603 : dc/dh à {h} m : {dc} (différence centrée {fd})");
        assert!(((dc - fd) / dc).abs() < 1e-6, "critère 1 : dc/dh à {h} m");
    }

    let s = scene();
    let base = s.arrivees(&plateau, None).unwrap();
    assert!(base.iter().all(|a| a.y.is_some()), "critère 2 : les 363 rayons arrivent");
    let refs = [4017.6787176703274, -4000.0, -12017.678717670313, 8017.67871767032, 0.0, -8017.678717670314, 12017.678717670315, 4000.0,
        -4017.6787176703187];
    let mut k = 0;
    for (d, a) in s.departs.iter().zip(&base) {
        if [-4000.0, 0.0, 4000.0].contains(&d.0[1]) {
            println!("S603 : départ y = {} θ = {} : arrivée {:?}", d.0[1], d.1, a.y);
            assert!((a.y.unwrap() - refs[k]).abs() < 1e-3, "critère 2 : arrivée {k}");
            k += 1;
        }
    }
    assert_eq!(k, 9);

    let cas = [("profonde", [23_000.0, 1000.0], 7.0, 107.14), ("entre", [14_000.0, 1000.0], 7.0, 62.14), ("cote", [6000.0, 1000.0], 12.0, 17.51)];
    for (nom, centre, a, hmin) in cas {
        let nouveau = bosse(centre, 1500.0, a);
        let support = Support { centre, rayon_m: 1500.0, profondeur_min_m: hmin };
        let portee = portee_bathymetrie(&s, &plateau, &nouveau, &support).unwrap();
        let apres = s.arrivees(&nouveau, None).unwrap();
        let (changees, max) = brut(&s, &base, &apres);
        println!("S603 : bosse {nom} : portée {portee:?}, plages changées {changees:?}, décalage max {max} m");
        match nom {
            "profonde" => {
                assert!(portee.is_empty(), "critère 3 : portée vide");
                assert!(max < TOL, "critère 3 : le fond n'est plus senti");
            }
            "entre" => {
                assert!(!portee.is_empty(), "critère 4 : portée non vide");
                assert!(max > 10.0 * TOL, "critère 4 : la règle λ/2 l'aurait manquée");
            }
            _ => {
                assert_eq!(portee, vec![2, 3, 4, 5, 6], "critère 5 : la portée");
                assert!(changees.iter().all(|p| portee.contains(p)), "critère 5 : toute plage changée est dans la portée");
            }
        }
    }

    let refus = |f: &dyn Fn(&mut Scene)| {
        let mut t = scene();
        t.departs.truncate(3);
        f(&mut t);
        t.arrivees(&plateau, None)
    };
    assert_eq!(refus(&|t| t.periode_s = 0.0), Err(Refus), "critère 6 : période");
    assert_eq!(refus(&|t| t.g = -9.81), Err(Refus), "critère 6 : gravité");
    assert_eq!(refus(&|t| t.dt_s = 0.0), Err(Refus), "critère 6 : pas");
    assert_eq!(refus(&|t| t.bornes_y.swap(0, 1)), Err(Refus), "critère 6 : bornes");
    assert_eq!(isobathe_limite_m(-8.0, 9.81), Err(Refus), "critère 6 : période de l'isobathe");
}
