//! S618 — le banc B7 des modules de la v2 sur la cible (ce PC, ADR-219 D2) et sur la seconde cible bridée (le budget ÷ 3, 9.10).
//!
//! Chaque coût : la médiane de cinq répétitions d'au moins 0,2 s, et l'étalement max/min (au-delà de 1,5 : « instable »). Les capacités par
//! tick suivent I-16 : `⌊budget / coût⌋`.
//!
//! `cargo run --manifest-path code/Cargo.toml --release --offline -p water-core --example b7_cible`
use std::hint::black_box;
use std::time::Instant;
use water_core::changement_solveur::transduire;
use water_core::qualite::{Reglages, Regulateur};
use water_core::saint_venant_2d::SaintVenant2D;
use water_core::substitutif::Domaine1D;
use water_core::tsunami::{niveau, Rayon, Tsunami};

/// Le coût d'une unité (ns) : `travail(n)` fait `n` unités ; médiane et étalement de cinq répétitions d'au moins 0,2 s.
fn mesurer(mut travail: impl FnMut(u64)) -> (f64, f64) {
    let mut n = 1u64;
    loop {
        let t = Instant::now();
        travail(n);
        if t.elapsed().as_secs_f64() >= 0.2 {
            break;
        }
        n *= 2;
    }
    let mut couts: Vec<f64> = (0..5).map(|_| {
        let t = Instant::now();
        travail(n);
        t.elapsed().as_secs_f64() * 1e9 / n as f64
    }).collect();
    couts.sort_by(f64::total_cmp);
    (couts[2], couts[4] / couts[0])
}

fn main() {
    let (budget_ms, bride) = (2.0f64, 3.0f64);
    let g = 9.81;

    // Saint-Venant 2D, 200² : une cuvette mouillée (Thacker à t = 0), un pas = 40 000 mailles.
    let nx = 200;
    let dx = 4.0 / nx as f64;
    let mut z = Vec::new();
    let mut h = Vec::new();
    for i in 0..nx {
        for j in 0..nx {
            let (x, y) = ((i as f64 + 0.5) * dx - 2.0, (j as f64 + 0.5) * dx - 2.0);
            let zz = -0.1 * (1.0 - x * x - y * y);
            z.push(zz);
            h.push((0.05 * (2.0 * x - 0.5) - zz).max(0.0));
        }
    }
    let n2 = nx * nx;
    let mut sv = SaintVenant2D::nouveau(nx, nx, dx, g, z, h, vec![0.0; n2], vec![0.0; n2]).unwrap();
    // L'unité mesurée est le pas ; le coût par maille s'en déduit.
    let (sv_pas, sv_e) = mesurer(|n| {
        for _ in 0..n {
            sv.pas(0.002).unwrap();
        }
    });
    let sv_ns = sv_pas / n2 as f64;

    // Domaine 1D, 400 mailles.
    let bosse: Vec<f64> = (0..400).map(|i| 0.05 * (-(((i as f64 + 0.5) * 0.5 - 100.0) / 5.0).powi(2)).exp()).collect();
    let mut d1 = Domaine1D::depuis_b(g, 2.0, 0.5, 400, 0.05, &|_, _| (0.0, 0.0), &bosse).unwrap();
    let (d1_pas, d1_e) = mesurer(|n| {
        for _ in 0..n {
            d1.avancer(&|_, _| (0.0, 0.0));
        }
    });
    let d1_ns = d1_pas / 400.0;

    // Un échantillon de W (deux trains, S612).
    let w = transduire(Domaine1D::depuis_b(g, 2.0, 0.5, 400, 0.05, &|_, _| (0.0, 0.0), &bosse).unwrap());
    let (w_ns, w_e) = mesurer(|n| {
        let mut s = 0.0;
        for k in 0..n {
            s += w.eta(black_box((k % 800) as f64 * 0.25), 7.0);
        }
        black_box(s);
    });

    // Un échantillon du tsunami (S582).
    let sommets = [[0.0, 4000.0], [50_000.0, 10.0]];
    let rayon = Rayon::new(&sommets, g).unwrap();
    let ts = Tsunami { a0_m: 0.5, t0_s: 0.0, demi_duree_s: 600.0 };
    let (t_ns, t_e) = mesurer(|n| {
        let mut s = 0.0f32;
        for k in 0..n {
            s += niveau(&rayon, &ts, black_box((k % 1000) as f64 * 40.0), 1000.0).unwrap_or(0.0);
        }
        black_box(s);
    });

    // Une image du régulateur (S617).
    let mut reg = Regulateur::nouveau(Reglages::ADR_012).unwrap();
    let (r_ns, r_e) = mesurer(|n| {
        for k in 0..n {
            black_box(reg.image(black_box(1.5 + (k % 7) as f64 * 0.05), 1.6).unwrap());
        }
    });

    println!("S618 — B7 sur ce PC, release, un fil ; budget {budget_ms} ms (cible), {:.4} ms (seconde cible, ÷ {bride})", budget_ms / bride);
    println!("| module | unité | coût médian (ns) | étalement | capacité cible | capacité bridée |");
    println!("|---|---|---|---|---|---|");
    for (nom, unite, ns, e) in [
        ("SaintVenant2D", "maille·pas", sv_ns, sv_e),
        ("Domaine1D", "maille·pas", d1_ns, d1_e),
        ("TrainW1D", "échantillon", w_ns, w_e),
        ("tsunami::niveau", "échantillon", t_ns, t_e),
        ("Regulateur", "image", r_ns, r_e),
    ] {
        assert!(ns.is_finite() && ns > 0.0, "critère 1 : {nom}");
        let cap = |b: f64| (b * 1e6 / ns).floor() as u64;
        let drapeau = if e > 1.5 { " (instable)" } else { "" };
        println!("| {nom} | {unite} | {ns:.2} | {e:.3}{drapeau} | {} | {} |", cap(budget_ms), cap(budget_ms / bride));
    }
    let cote = |b: f64| ((b * 1e6 / sv_ns).floor()).sqrt().floor() as u64;
    println!("côté du domaine 2D carré par tick : {} mailles (cible), {} (bridée)", cote(budget_ms), cote(budget_ms / bride));
}
