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
