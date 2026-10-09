//! Essais de Serre–Green–Naghdi 1D (S694) : l'onde solitaire exacte.

use super::*;

const G: f64 = 9.81;

/// L'onde solitaire de SGN : `η = a·sech²(κ(x − x0))`, `c = √(g(d+a))`, `κ = √(3a)/(2d√(d+a))`, `u = c·η/(d+η)`.
fn solitaire(a: f64, d: f64, x: f64, x0: f64) -> (f64, f64) {
    let c = (G * (d + a)).sqrt();
    let k = (3. * a).sqrt() / (2. * d * (d + a).sqrt());
    let s = 1. / (k * (x - x0)).cosh();
    let eta = a * s * s;
    (eta, c * eta / (d + eta))
}

/// L'onde de `a/d` sur 40 `d`, maille `d/n`, domaine périodique de 80 `d`. Rend (l'écart de forme au plus, rapporté à `a` ; l'écart
/// relatif de la célérité de la crête ; l'écart de masse relatif).
fn propager(a: f64, n: usize, dispersif: bool) -> (f64, f64, f64) {
    let (d, l) = (1.0f64, 80.0f64);
    let dx = d / n as f64;
    let nx = (l / dx).round() as usize;
    let x0 = 20.0;
    let xc = |i: usize| (i as f64 + 0.5) * dx;
    let (h, q): (Vec<f64>, Vec<f64>) = (0..nx).map(|i| {
        let (e, u) = solitaire(a, d, xc(i), x0);
        (d + e, (d + e) * u)
    }).unzip();
    let mut s = Serre1D::nouveau(dx, G, h, q, dispersif).unwrap();
    let v0 = s.volume();
    let c = (G * (d + a)).sqrt();
    let t_fin = 40. * d / c;
    let mut t = 0.;
    while t < t_fin - 1e-12 {
        let dt = s.pas_stable().min(t_fin - t);
        s.pas(dt).unwrap();
        t += dt;
    }
    // La crête : la maille la plus haute, affinée par une parabole.
    let i = (0..nx).max_by(|&x, &y| s.h[x].total_cmp(&s.h[y])).unwrap();
    let (hm, h0, hp) = (s.h[(i + nx - 1) % nx], s.h[i], s.h[(i + 1) % nx]);
    let decal = 0.5 * (hm - hp) / (hm - 2. * h0 + hp);
    let x_crete = xc(i) + decal * dx;
    let x_attendu = x0 + c * t_fin;
    let c_num = (x_crete - x0) / t_fin;
    let forme = (0..nx).map(|j| (s.h[j] - d - solitaire(a, d, xc(j), x_attendu).0).abs()).fold(0., f64::max) / a;
    (forme, c_num / c - 1., (s.volume() - v0).abs() / v0)
}

/// **S694** — (1) la forme à 2 % de `a` à `d/40`, divisée par 3 au moins depuis `d/20` ; (2) la célérité à 0,2 % ; (3) le témoin (Saint-Venant)
/// au-delà de 10 % ; (4) la masse au bit.
#[test]
fn the_serre_solitary_wave_keeps_its_shape_and_speed_s694() {
    for a in [0.1f64, 0.3] {
        let (f20, c20, m20) = propager(a, 20, true);
        let (f40, c40, m40) = propager(a, 40, true);
        let (ft, ct, _) = propager(a, 40, false);
        println!("S694 a/d = {a} : forme {:.3} % (d/20) → {:.3} % (d/40), ÷ {:.1} ; célérité {:+.3} % → {:+.3} % ; masse {m20:.1e}, {m40:.1e} ; témoin Saint-Venant : forme {:.1} %, célérité {:+.2} %",
            100. * f20, 100. * f40, f20 / f40, 100. * c20, 100. * c40, 100. * ft, 100. * ct);
        assert!(f40 < 0.02 && f20 / f40 >= 3., "critère 1 : {a}");
        assert!(c40.abs() < 0.002, "critère 2 : {a}");
        assert!(ft > 0.10, "critère 3 : {a}");
        assert!(m20 < 1e-12 && m40 < 1e-12, "critère 4 : {a}");
    }
}

/// S733 — la plage de R43, périodique : le fond plat à 0,5 m jusqu'à 5,696 m, la pente de 1:12 jusqu'à 4 cm d'eau, puis le miroir.
fn plage_r43_periodique(dx: f64) -> Vec<f64> {
    let demi = 11.5f64;
    let n = (2. * demi / dx).round() as usize;
    (0..n).map(|i| {
        let x = (i as f64 + 0.5) * dx;
        let x = if x > demi { 2. * demi - x } else { x };
        ((x - 5.696).max(0.) / 12.).min(0.46)
    }).collect()
}

/// **S733 — E1 (1), le lac au repos sur le fond doux** : la plage de R43, 2 s ; la vitesse sous 10⁻¹² m/s, la masse au bit.
#[test]
fn the_serre_lake_at_rest_on_a_sloping_bottom_s733() {
    let dx = 0.025;
    let z = plage_r43_periodique(dx);
    let h: Vec<f64> = z.iter().map(|z| 0.5 - z).collect();
    let n = z.len();
    let mut s = Serre1D::nouveau_fond(dx, G, h, vec![0.; n], z, true).unwrap();
    let v0 = s.volume();
    let mut t = 0.;
    while t < 2. {
        let dt = s.pas_stable().min(2. - t);
        s.pas(dt).unwrap();
        t += dt;
    }
    let umax = (0..n).map(|i| (s.q[i] / s.h[i]).abs()).fold(0f64, f64::max);
    let masse = (s.volume() - v0).abs() / v0;
    println!("S733 E1 (1) : le lac au repos sur la plage de R43, 2 s — la vitesse max {umax:.2e} m/s ; la masse {masse:.1e}");
    assert!(umax < 1e-12, "critère 1 : la vitesse {umax}");
    assert!(masse < 1e-14, "critère 1 : la masse {masse}");
}

/// **S733 — E1 (2), la levée sur le fond doux** : une bosse gaussienne de 1 mm (σ = 0,5 m) partie vers la droite sur 0,5 m d'eau, une pente
/// de 1:50 jusqu'à 0,2 m, un plateau, le miroir. Saint-Venant (sans dispersion : une bosse courte se disperse sous SGN, la loi de Green est
/// celle des ondes longues) ; la hauteur sur le plateau contre `(0,5/0,2)^¼` = 1,2574, à 5 %. SGN rapporté.
#[test]
fn the_serre_shoaling_follows_green_s733() {
    let dx = 0.025;
    let (demi, a, sigma, x0) = (45f64, 1e-3f64, 2f64, 7f64);
    let n = (2. * demi / dx).round() as usize;
    let z: Vec<f64> = (0..n).map(|i| {
        let x = (i as f64 + 0.5) * dx;
        let x = if x > demi { 2. * demi - x } else { x };
        ((x - 15.).max(0.) / 50.).min(0.3)
    }).collect();
    let mut rapports = Vec::new();
    // Le témoin de l'usure numérique : la même bosse sur le fond plat, la même durée (sa hauteur à la fin, rapportée).
    for (dispersif, plat) in [(false, true), (false, false), (true, false)] {
        let z: Vec<f64> = if plat { vec![0.; n] } else { z.clone() };
        let (mut h, mut q) = (vec![0f64; n], vec![0f64; n]);
        for i in 0..n {
            let x = (i as f64 + 0.5) * dx;
            let eta = a * (-((x - x0) / sigma).powi(2)).exp();
            let d = 0.5 - z[i];
            h[i] = d + eta;
            q[i] = eta * (G * d).sqrt();
        }
        let mut s = Serre1D::nouveau_fond(dx, G, h, q, z.clone(), dispersif).unwrap();
        let (mut t, mut haut) = (0f64, 0f64);
        while t < 18. {
            let dt = s.pas_stable().min(18. - t);
            s.pas(dt).unwrap();
            t += dt;
            for i in 0..n {
                let x = (i as f64 + 0.5) * dx;
                if plat || (32. ..38.).contains(&x) {
                    haut = if plat { (0..n).map(|k| s.h[k] - 0.5).fold(f64::MIN, f64::max) } else { haut.max(s.h[i] + s.z[i] - 0.5) };
                }
            }
        }
        let r = haut / a;
        let nom = if plat { "Saint-Venant, fond plat (l'usure)" } else if dispersif { "SGN" } else { "Saint-Venant" };
        println!("S733 E1 (2), {nom} : la hauteur {:.4} mm, rapport {r:.4} (Green sur le plateau : 1,2574)", haut * 1e3);
        rapports.push(r);
    }
    let green = (0.5f64 / 0.2).powf(0.25);
    println!("S733 E1 (2) : Saint-Venant sur la pente {:.4}, rapporté à l'usure {:.4} ; Green {green:.4}", rapports[1], rapports[1] / rapports[0]);
    assert!((rapports[1] / green - 1.).abs() < 0.05, "critère 2 : {}", rapports[1]);
}
