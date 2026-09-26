//! **Le ballottement d'APIC en 3D** — S388, C4a de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md), [ADR-186](../../../docs/adr/ADR-186-apic-seconde-representation.md)).
//!
//! Deux cas, contre `ω² = g·k·tanh(k·h)` :
//!
//! - **`10`** — le mode (1, 0) de S318 : cuve de 2 m, 0,5 m d'eau, `η = h + A·cos(πx/Lx)`, `A` = 2 cm, invariant en `y`
//!   (cuve de 0,2 m de large) ; période exacte **1,9765 s** ; 10 s ;
//! - **`11`** — un mode **oblique** (1, 1) : cuve carrée de 1 m, 0,5 m d'eau, `η = h + A·cos(πx)·cos(πy)`, 2 cm ; 5 s.
//!
//! La période se lit sur un **moment** de la masse — `Σ (x − Lx/2)` pour (1, 0), `Σ (x − ½)(y − ½)` pour (1, 1) —, lisse,
//! sans les marches d'une jauge par comptage (S318, faute 3) : passages par zéro, interpolés, période moyenne entre le
//! premier et le dernier. L'énergie **créée** se rapporte à celle de l'onde, `½·ρ·g·∫η'²`.
//!
//! Critères du plan de S388, écrits avant : (1, 0), erreur de période ≤ 7 % à 5 cm, ≤ 1 % à 2,5 cm, décroissante ; énergie
//! jamais créée au-delà de 1 % de celle de l'onde ; (1, 1), ≤ 2 % à la maille la plus fine.
//!
//!     cargo run -p water-core --release --offline --example apic3d_ballottement -- <10|11> <dx>
//!
//! Témoins : `APIC3D_RAYON=plan` (le rayon de S318), `APIC3D_SANS_SEPARATION`, `APIC3D_NOYAU=1` (S389), `APIC3D_TRACE`.
//! S389 : l'amortissement par période, régression de ln(pic) sur les demi-périodes — la mesure qui porte « l'énergie ne
//! croît pas ».
//!
//! S398 : deux autres solveurs sur la même cuve et le même instrument (le moment, lu sur `η`) — `APIC3D_COLONNES=1`, APIC 3D
//! **tout en colonnes** (sa zone de colonnes, sans particule) ; `APIC3D_DELTA=1`, δ (`Volume3`, pas mobile, 20 ms, Jacobi jusqu'à 200 000 itérations). Ligne
//! `APIC3D_S398`.

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::apic3d::Apic3;
use water_core::delta3d::{Domain3, Volume3};
use water_core::host::HostServices;

const G: f64 = 9.81;
const H: f64 = 0.5;
const A: f64 = 0.02;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("10");
    let dx: f64 = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(0.05);
    let (lx, ly, lz, duree) = match mode {
        "11" => (1.0, 1.0, 0.8, 5.0),
        _ => (2.0, 0.2, 1.0, 10.0),
    };
    let (nx, ny, nz) = ((lx / dx).round() as usize, (ly / dx).round() as usize, (lz / dx).round() as usize);
    let (kx, ky) = match mode {
        "11" => (std::f64::consts::PI / lx, std::f64::consts::PI / ly),
        _ => (std::f64::consts::PI / lx, 0.),
    };
    let k = (kx * kx + ky * ky).sqrt();
    let periode_exacte = std::f64::consts::TAU / (G * k * (k * H).tanh()).sqrt();
    let profil = move |x: f64, y: f64| A * (kx * x).cos() * if mode == "11" { (ky * y).cos() } else { 1. };
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    // S398 : les colonnes (APIC 3D sans particule) ou δ, sur la même cuve ; le moment se lit sur `η`.
    let colonnes = std::env::var("APIC3D_COLONNES").is_ok();
    let delta = std::env::var("APIC3D_DELTA").is_ok();
    if colonnes || delta {
        let domain = Domain3 { nx, ny, nz, dx: dx as f32 };
        let eta: Vec<f32> = (0..nx * ny)
            .map(|c| (H + profil(((c % nx) as f64 + 0.5) * dx, ((c / nx) as f64 + 0.5) * dx)) as f32)
            .collect();
        let moment_eta = |e: &[f32]| -> f64 {
            e.iter()
                .enumerate()
                .map(|(c, h)| {
                    let (x, y) = (((c % nx) as f64 + 0.5) * dx, ((c / nx) as f64 + 0.5) * dx);
                    (*h as f64 - H) * if mode == "11" { (x - lx / 2.) * (y - ly / 2.) } else { x - lx / 2. }
                })
                .sum()
        };
        let mut apic_cols = None;
        let mut vol = None;
        if colonnes {
            let mut a = Apic3::configure(&mut hote, domain, 1000., G as f32, 8).expect("configuration");
            a.enable_columns(&mut hote, &vec![1u8; nx * ny]).expect("colonnes");
            a.set_columns_surface(&eta).expect("surface");
            apic_cols = Some(a);
        } else {
            let mut v = Volume3::configure(&mut hote, domain, 1000., G as f32).expect("δ");
            v.set_free_surface(&eta, H as f32).expect("surface");
            vol = Some(v);
        }
        let volume = |a: &Option<Apic3>, v: &Option<Volume3>| -> f64 {
            match (a, v) {
                (Some(a), _) => a.columns_volume(),
                (_, Some(v)) => v.surface().iter().map(|e| *e as f64 * dx * dx).sum(),
                _ => 0.,
            }
        };
        let v0 = volume(&apic_cols, &vol);
        let fin = (duree * 1e6) as u64;
        let (mut t, mut pas, mut passages, mut pics, mut pic) = (0u64, 0u64, Vec::new(), Vec::new(), 0f64);
        let mut precedent = (0f64, moment_eta(&eta));
        let debut = Instant::now();
        while t < fin {
            let us = match &apic_cols {
                Some(a) => a.stable_step_us(20_000),
                None => 20_000,
            }
            .min(fin - t);
            match (&mut apic_cols, &mut vol) {
                (Some(a), _) => {
                    a.step(us).expect("pas");
                }
                (_, Some(v)) => {
                    // Le Jacobi de δ ne converge pas en 20 000 itérations sur la cuve (1, 0) à 2,5 cm ; la multigrille de S385
                    // y refuse la cuve mince (4 et 8 mailles de large) — S398, observé, non étudié ici.
                    v.step_surface_mobile(us, 200_000, &jobs).expect("pas δ");
                }
                _ => unreachable!(),
            }
            t += us;
            pas += 1;
            let e: &[f32] = match (&apic_cols, &vol) {
                (Some(a), _) => a.columns_surface().unwrap(),
                (_, Some(v)) => v.surface(),
                _ => unreachable!(),
            };
            let (s, mo) = (t as f64 * 1e-6, moment_eta(e));
            if precedent.1 != 0. && precedent.1.signum() != mo.signum() {
                passages.push(precedent.0 + (s - precedent.0) * precedent.1 / (precedent.1 - mo));
                if passages.len() >= 2 {
                    pics.push(pic);
                }
                pic = 0.;
            }
            pic = pic.max(mo.abs());
            precedent = (s, mo);
        }
        let periode = if passages.len() >= 3 { 2. * (passages[passages.len() - 1] - passages[0]) / (passages.len() - 1) as f64 } else { f64::NAN };
        let amortissement = if pics.len() >= 3 {
            let n = pics.len() as f64;
            let (sx, sy): (f64, f64) = pics.iter().enumerate().map(|(i, p)| (i as f64, p.ln())).fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1));
            let (mx, my) = (sx / n, sy / n);
            let (sxy, sxx): (f64, f64) = pics.iter().enumerate().map(|(i, p)| ((i as f64 - mx) * (p.ln() - my), (i as f64 - mx).powi(2))).fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1));
            1. - (2. * sxy / sxx).exp()
        } else {
            f64::NAN
        };
        println!(
            "APIC3D_S398 solveur={} mode={mode} dx={dx} pas={pas} periode_s={periode:.4} periode_exacte_s={periode_exacte:.4} \
             erreur={:+.2}% amortissement_par_periode={:+.2}% pics={} volume_relatif={:+.2e} calcul_s={:.0}",
            if colonnes { "colonnes" } else { "delta" },
            100. * (periode / periode_exacte - 1.),
            100. * amortissement,
            pics.len(),
            volume(&apic_cols, &vol) / v0 - 1.,
            debut.elapsed().as_secs_f64()
        );
        return;
    }
    let mut a = Apic3::configure(&mut hote, Domain3 { nx, ny, nz, dx: dx as f32 }, 1000., G as f32, nx * ny * nz * 8)
        .expect("configuration");
    let n = a.seed(&|p| (p[2] as f64) < H + profil(p[0] as f64, p[1] as f64)).expect("ensemencement");
    // Témoins de mesure : le rayon de S318 (`APIC3D_RAYON=plan`), sans séparation (`APIC3D_SANS_SEPARATION`).
    if std::env::var("APIC3D_RAYON").as_deref() == Ok("plan") {
        a.set_reconstruction_radius(water_core::apic3d::rest_radius_at_the_plane(dx as f32));
    }
    if std::env::var("APIC3D_SANS_SEPARATION").is_ok() {
        a.set_separation(false);
    }
    // S389 : le témoin au noyau d'une maille (`APIC3D_NOYAU=1`) ; défaut, deux mailles.
    if let Some(k) = std::env::var("APIC3D_NOYAU").ok().and_then(|v| v.parse::<f32>().ok()) {
        a.set_reconstruction_kernel(k);
    }
    let m = a.particle_mass() as f64;
    let moment = |a: &Apic3| -> f64 {
        a.particles()
            .iter()
            .map(|p| {
                let (x, y) = (p[0] as f64, p[1] as f64);
                if mode == "11" { (x - lx / 2.) * (y - ly / 2.) } else { x - lx / 2. }
            })
            .sum()
    };
    let energie = |a: &Apic3| -> f64 {
        a.particles().iter().zip(a.velocities()).map(|(p, v)| {
            m * (0.5 * (v[0] as f64 * v[0] as f64 + v[1] as f64 * v[1] as f64 + v[2] as f64 * v[2] as f64) + G * p[2] as f64)
        }).sum()
    };
    // Énergie de l'onde : ½·ρ·g·∫η'² ; A²·Lx·Ly/4 pour (1, 0), A²·Lx·Ly/8 pour (1, 1).
    let facteur = if mode == "11" { 0.125 } else { 0.25 };
    let e_onde = 0.5 * 1000. * G * A * A * lx * ly * facteur * 2.;
    let e0 = energie(&a);
    let (mut t, mut e_max, mut passages, mut precedent, mut iterations, mut pas) = (0u64, f64::MIN, Vec::new(), (0f64, moment(&a)), 0u64, 0u64);
    // S389 : les pics du moment, un par demi-période (le plus grand |moment| entre deux passages), pour l'amortissement.
    let (mut pics, mut pic_courant) = (Vec::new(), 0f64);
    let fin = (duree * 1e6) as u64;
    let debut = Instant::now();
    while t < fin {
        let us = a.stable_step_us(20_000).min(fin - t);
        let r = a.step(us).expect("pas");
        iterations += r.iterations as u64;
        pas += 1;
        t += us;
        let (s, mo) = (t as f64 * 1e-6, moment(&a));
        if precedent.1 != 0. && precedent.1.signum() != mo.signum() {
            passages.push(precedent.0 + (s - precedent.0) * precedent.1 / (precedent.1 - mo));
            if passages.len() >= 2 {
                pics.push(pic_courant);
            }
            pic_courant = 0.;
        }
        pic_courant = pic_courant.max(mo.abs());
        precedent = (s, mo);
        e_max = e_max.max(energie(&a) - e0);
        if std::env::var("APIC3D_TRACE").is_ok() && pas % 25 == 0 {
            let (ec, ep): (f64, f64) = a.particles().iter().zip(a.velocities()).map(|(p, v)| (0.5 * m * (v[0] as f64 * v[0] as f64 + v[1] as f64 * v[1] as f64 + v[2] as f64 * v[2] as f64), m * G * p[2] as f64)).fold((0., 0.), |acc, x| (acc.0 + x.0, acc.1 + x.1));
            println!("APIC3D_TRACE t={s:.3} energie_sur_onde={:+.4} cinetique_sur_onde={:.4} moment={mo:+.4e} iterations={}", (energie(&a) - e0) / e_onde, ec / e_onde, r.iterations);
            let _ = ep;
        }
    }
    let periode = if passages.len() >= 3 {
        2. * (passages[passages.len() - 1] - passages[0]) / (passages.len() - 1) as f64
    } else {
        f64::NAN
    };
    let erreur = periode / periode_exacte - 1.;
    // Amortissement par période : régression de ln(pic) sur l'indice de demi-période ; positif, l'onde s'éteint.
    let amortissement = if pics.len() >= 3 {
        let n = pics.len() as f64;
        let (sx, sy): (f64, f64) = pics.iter().enumerate().map(|(i, p)| (i as f64, p.ln())).fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1));
        let (mx, my) = (sx / n, sy / n);
        let (sxy, sxx): (f64, f64) = pics.iter().enumerate().map(|(i, p)| ((i as f64 - mx) * (p.ln() - my), (i as f64 - mx).powi(2))).fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1));
        1. - (2. * sxy / sxx).exp()
    } else {
        f64::NAN
    };
    println!(
        "APIC3D_S388 mode={mode} dx={dx} mailles={} particules={n} duree_s={duree} pas={pas} periode_s={periode:.4} \
         periode_exacte_s={periode_exacte:.4} erreur={:+.2}% passages={} energie_creee_max={:+.2}% \
         amortissement_par_periode={:+.2}% pics={} iterations_moyennes={:.1} calcul_s={:.0}",
        nx * ny * nz,
        100. * erreur,
        passages.len(),
        100. * e_max / e_onde,
        100. * amortissement,
        pics.len(),
        iterations as f64 / pas as f64,
        debut.elapsed().as_secs_f64()
    );
    assert_eq!(a.particle_count(), n, "masse");
}
