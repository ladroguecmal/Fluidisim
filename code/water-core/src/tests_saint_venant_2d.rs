//! S613 — le mouillage et le séchage en 2D, jugés sur Thacker. Références écrites au plan par son script (`s613_ref.py`, numpy).

use super::*;

const G: f64 = 9.81;
const A: f64 = 1.0;
const H0: f64 = 0.1;
const LD: f64 = 4.0;
const ETA: f64 = 0.5;

fn omega() -> f64 {
    (2.0 * G * H0).sqrt() / A
}

fn centre(nx: usize, i: usize) -> f64 {
    (i as f64 + 0.5) * LD / nx as f64
}

fn fond(nx: usize) -> Vec<f64> {
    let mut z = Vec::with_capacity(nx * nx);
    for i in 0..nx {
        for j in 0..nx {
            let r2 = (centre(nx, i) - LD / 2.0).powi(2) + (centre(nx, j) - LD / 2.0).powi(2);
            z.push(-H0 * (1.0 - r2 / (A * A)));
        }
    }
    z
}

/// La solution de Thacker (SWASHES) à l'instant `t` : `(h, u, v)` par maille.
fn exact(nx: usize, t: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let (w, z) = (omega(), fond(nx));
    let (mut h, mut u, mut v) = (Vec::new(), Vec::new(), Vec::new());
    for i in 0..nx {
        for j in 0..nx {
            let k = i * nx + j;
            let e = ETA * H0 / (A * A) * (2.0 * (centre(nx, i) - LD / 2.0) * (w * t).cos() + 2.0 * (centre(nx, j) - LD / 2.0) * (w * t).sin() - ETA)
                - z[k];
            let hk = e.max(0.0);
            h.push(hk);
            u.push(if hk > 0.0 { -ETA * w * (w * t).sin() } else { 0.0 });
            v.push(if hk > 0.0 { ETA * w * (w * t).cos() } else { 0.0 });
        }
    }
    (h, u, v)
}

fn centre_de_masse(nx: usize, h: &[f64]) -> (f64, f64) {
    let (mut m, mut x, mut y) = (0.0, 0.0, 0.0);
    for i in 0..nx {
        for j in 0..nx {
            let hk = h[i * nx + j];
            m += hk;
            x += hk * centre(nx, i);
            y += hk * centre(nx, j);
        }
    }
    (x / m, y / m)
}

/// Une période de Thacker : l'écart L1, la variation de masse, la plus petite hauteur, les centres de masse à T/4, T/2, T.
fn thacker(nx: usize) -> (f64, f64, f64, Vec<(f64, f64)>) {
    let periode = 2.0 * core::f64::consts::PI / omega();
    let npas = 10 * nx;
    let dt = periode / npas as f64;
    let (h, u, v) = exact(nx, 0.0);
    let qx = h.iter().zip(&u).map(|(a, b)| a * b).collect();
    let qy = h.iter().zip(&v).map(|(a, b)| a * b).collect();
    let mut d = SaintVenant2D::nouveau(nx, nx, LD / nx as f64, G, fond(nx), h, qx, qy).unwrap();
    let v0 = d.volume();
    let (mut hmin, mut centres) = (0.0f64, Vec::new());
    for n in 1..=npas {
        d.pas(dt).unwrap();
        hmin = d.h.iter().fold(hmin, |m, &x| m.min(x));
        if n == npas / 4 || n == npas / 2 || n == npas {
            centres.push(centre_de_masse(nx, &d.h));
        }
    }
    let he = exact(nx, periode).0;
    let l1 = d.h.iter().zip(&he).map(|(a, b)| (a - b).abs()).sum::<f64>() / he.iter().sum::<f64>();
    (l1, d.volume() / v0 - 1.0, hmin, centres)
}

/// (1) 100² ; (2) la convergence ; (3) masse, positivité ; (4) le lac au repos ; (5) refus.
#[test]
fn a_planar_surface_rotates_in_a_paraboloid_with_wetting_and_drying_s613() {
    let refs = [(50, 0.4270884943083992), (100, 0.24218466972342975), (200, 0.1286419144167166)];
    let mut l1s = Vec::new();
    for (nx, r) in refs {
        let (l1, dm, hmin, centres) = thacker(nx);
        println!("S613 : {nx}² — L1 {l1}, masse {dm:e}, h min {hmin}, centres {centres:?}");
        assert!((l1 - r).abs() < 1e-9, "critères 1–2 : L1 à {nx}²");
        assert!(dm.abs() < 1e-13 && hmin >= 0.0, "critère 3 à {nx}²");
        if nx == 100 {
            let cref = [(2.017083014344954, 2.4754695487832254), (1.552226476871074, 2.058725468647306), (2.3951028365245266, 1.9028492651276134)];
            for (c, r) in centres.iter().zip(cref) {
                assert!((c.0 - r.0).abs() < 1e-9 && (c.1 - r.1).abs() < 1e-9, "critère 1 : centre de masse");
            }
        }
        l1s.push(l1);
    }
    assert!(l1s[1] / l1s[2] >= 1.8, "critère 2 : l'ordre un converge");

    let nx = 100;
    let z = fond(nx);
    let h: Vec<f64> = z.iter().map(|z| (-0.05 - z).max(0.0)).collect();
    let mut lac = SaintVenant2D::nouveau(nx, nx, LD / nx as f64, G, z, h, vec![0.0; nx * nx], vec![0.0; nx * nx]).unwrap();
    let dt = 2.0 * core::f64::consts::PI / omega() / 1000.0;
    for _ in 0..500 {
        lac.pas(dt).unwrap();
    }
    println!("S613 : lac au repos, vitesse max {:e} m/s", lac.vitesse_max());
    assert!(lac.vitesse_max() < 1e-14, "critère 4");

    assert!(SaintVenant2D::nouveau(1, 4, 0.1, G, vec![0.0; 4], vec![0.0; 4], vec![0.0; 4], vec![0.0; 4]).is_err(), "critère 5 : mailles");
    assert!(SaintVenant2D::nouveau(2, 2, 0.0, G, vec![0.0; 4], vec![0.0; 4], vec![0.0; 4], vec![0.0; 4]).is_err(), "critère 5 : dx");
    assert_eq!(lac.pas(0.0), Err(Refus), "critère 5 : dt");
    assert_eq!(lac.pas(1.0), Err(Refus), "critère 5 : Courant");
}

/// **S619** — (2) les tableaux de travail préalloués : aucune réallocation en 100 pas, et un clone qui fait les mêmes pas reste au bit.
#[test]
fn a_step_reuses_its_work_arrays_s619() {
    let nx = 50;
    let (h, u, v) = exact(nx, 0.0);
    let qx = h.iter().zip(&u).map(|(a, b)| a * b).collect();
    let qy = h.iter().zip(&v).map(|(a, b)| a * b).collect();
    let mut d = SaintVenant2D::nouveau(nx, nx, LD / nx as f64, G, fond(nx), h, qx, qy).unwrap();
    let avant = d.adresses_travail();
    let mut jumeau = d.clone();
    for _ in 0..100 {
        d.pas(0.005).unwrap();
        jumeau.pas(0.005).unwrap();
    }
    assert_eq!(d.adresses_travail(), avant, "critère 2 : aucune réallocation");
    let au_bit = d.h.iter().chain(&d.qx).chain(&d.qy).zip(jumeau.h.iter().chain(&jumeau.qx).chain(&jumeau.qy)).all(|(a, b)| a.to_bits() == b.to_bits());
    println!("S619 : 100 pas sans réallocation ; le clone au bit : {au_bit}");
    assert!(au_bit, "critère 2 : le clone au bit");
}
