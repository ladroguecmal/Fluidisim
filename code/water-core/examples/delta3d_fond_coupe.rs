//! **Le fond coupé en trois dimensions, éprouvé par son ordre** — S324, lot 3, critère 3.
//!
//! Le banc de S232 (`delta_filters`) porté à la grille x-y-z : un pas depuis le repos sous un
//! couvercle `η = z₀ + A·sin(2πx/L)`, et le **débit ouvert** à travers le plan `x = L/2`,
//! `Σ ouverture·u·dx²`, à trois mailles. Deux fonds :
//!
//! - **le fond lisse de S232**, qui ne dépend pas de `y` : le débit par unité de largeur doit redonner
//!   celui de la 2D — le témoin ;
//! - **une bosse vraiment tridimensionnelle**, décentrée en `y` : l'écoulement contourne l'obstacle
//!   et l'ordre de convergence se lit comme en 2D, avec la garde de S197 — incréments de même signe
//!   et décroissants.
//!
//! Critère écrit avant la mesure : ordre **≥ 1,8** sur la bosse, comme S232 en 2D.
//!
//!     cargo run -p water-core --release --offline --example delta3d_fond_coupe

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::delta3d::{Domain3, Volume3};
use water_core::host::HostServices;

const LX: f32 = 8.0;
const LY: f32 = 4.0;
const LZ: f32 = 4.0;
const RHO: f32 = 1025.0;
const G: f32 = 9.81;
/// Pas de S232, 2 ms, en microsecondes.
const DT_US: u64 = 2000;
const A: f32 = 0.01;

/// Le fond lisse de S232, qui ne dépend pas de `y`.
fn lisse(x: f32, _y: f32) -> f32 {
    let d = (x - 3.0) / 1.2;
    0.4 + 0.6 * (-(d * d)).exp()
}

/// Une bosse décentrée en `y` : l'eau doit la contourner.
fn bosse(x: f32, y: f32) -> f32 {
    let (a, b) = ((x - 3.0) / 1.2, (y - 1.3) / 0.9);
    0.4 + 0.6 * (-(a * a + b * b)).exp()
}

/// Un pas depuis le repos ; rend le débit ouvert à travers `x = L/2` (m³/s), les itérations, la
/// divergence et la durée du pas.
fn debit(fond: fn(f32, f32) -> f32, nx: usize) -> (f64, u32, f64, f64, usize) {
    let (ny, nz) = (nx / 2, nx / 2);
    let dx = LX / nx as f32;
    assert_eq!(ny as f32 * dx, LY);
    assert_eq!(nz as f32 * dx, LZ);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let b: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| fond((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)))
        .collect();
    let mut v = Volume3::configure_with_bottom(&mut hote, Domain3 { nx, ny, nz, dx }, RHO, G, &b).expect("configuration");
    let z0 = v.domain().z0();
    let eta: Vec<f32> = (0..ny)
        .flat_map(|_| (0..nx).map(move |i| z0 + A * (core::f32::consts::TAU * (i as f32 + 0.5) * dx / LX).sin()))
        .collect();
    v.set_surface(&eta).expect("surface");
    let debut = Instant::now();
    let r = v.step_surface_linear(DT_US, 20_000, &jobs).expect("pas");
    let duree = debut.elapsed().as_secs_f64();
    assert!(!r.degraded, "pas dégradé");
    let (ou, _, _) = v.apertures().expect("fond coupé");
    let i = nx / 2;
    let mut q = 0.0f64;
    for k in 0..nz {
        for j in 0..ny {
            let f = (k * ny + j) * (nx + 1) + i;
            q += ou[f] as f64 * v.velocity_u()[f] as f64 * dx as f64 * dx as f64;
        }
    }
    (q, r.iterations, r.divergence, duree, nx * ny * nz)
}

/// **Les petites cellules**, sans rien résoudre : la plus petite fraction et la plus petite ouverture
/// non nulles, et combien passent sous 10⁻³ — ce qui conditionne l'opérateur (`--geometrie`).
fn petites_cellules(fond: fn(f32, f32) -> f32, nx: usize) {
    let (ny, nz) = (nx / 2, nx / 2);
    let dx = LX / nx as f32;
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let b: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| fond((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)))
        .collect();
    let v = Volume3::configure_with_bottom(&mut hote, Domain3 { nx, ny, nz, dx }, RHO, G, &b).expect("configuration");
    let frac = v.fluid_fraction().expect("fond coupé");
    let (ou, ov, ow) = v.apertures().expect("fond coupé");
    let min_pos = |x: &[f32]| x.iter().filter(|a| **a > 0.).fold(f32::INFINITY, |m, a| m.min(*a));
    let sous = |x: &[f32], s: f32| x.iter().filter(|a| **a > 0. && **a < s).count();
    let faces: Vec<f32> = ou.iter().chain(ov).chain(ow).copied().collect();
    println!(
        "FOND3D_S324 geometrie nx={nx} fraction_min={:e} fractions_sous_1e-3={} ouverture_min={:e} ouvertures_sous_1e-3={} coupees={}",
        min_pos(frac),
        sous(frac, 1e-3),
        min_pos(&faces),
        sous(&faces, 1e-3),
        frac.iter().filter(|a| **a > 0. && **a < 1.).count()
    );
}

fn main() {
    if std::env::args().any(|a| a == "--geometrie") {
        for (nom, fond) in [("lisse_s232", lisse as fn(f32, f32) -> f32), ("bosse_3d", bosse)] {
            println!("FOND3D_S324 geometrie fond={nom}");
            for nx in [32usize, 64, 128] {
                petites_cellules(fond, nx);
            }
        }
        return;
    }
    println!("S324 — fond coupé 3D, débit ouvert à x = L/2, un pas depuis le repos");
    println!("domaine {LX} x {LY} x {LZ} m, rho={RHO}, g={G}, dt={} ms, eta = z0 + {A}·sin(2πx/L)", DT_US as f64 / 1000.);
    // Débits publiés par S232 en 2D, fond lisse, après correction, par unité de largeur (m²/s).
    let s232 = [2.252495855e-4, 2.258485786e-4, 2.260028664e-4];
    let mut sortie = Vec::new();
    for (nom, fond) in [("lisse_s232", lisse as fn(f32, f32) -> f32), ("bosse_3d", bosse)] {
        let mut q = Vec::new();
        for (n, nx) in [32usize, 64, 128].into_iter().enumerate() {
            let (v, it, div, duree, mailles) = debit(fond, nx);
            let temoin = if nom == "lisse_s232" { format!(" q_par_largeur={:.9e} ecart_s232={:+e}", v / LY as f64, v / LY as f64 / s232[n] - 1.) } else { String::new() };
            println!("FOND3D_S324 fond={nom} nx={nx} mailles={mailles} debit_m3_s={v:+.9e} iterations={it} divergence={div:.3e} duree_pas_s={duree:.3}{temoin}");
            q.push(v);
        }
        let (d1, d2) = (q[1] - q[0], q[2] - q[1]);
        if d1 * d2 > 0. && d1.abs() > d2.abs() {
            let ordre = (d1 / d2).abs().log2();
            let residu = (d2 / (2f64.powf(ordre) - 1.)).abs() / q[2].abs();
            println!("FOND3D_S324 fond={nom} ordre={ordre:.3} residu_richardson={:.3}%", 100. * residu);
            sortie.push((nom, Some(ordre)));
        } else {
            println!("FOND3D_S324 fond={nom} TRIPLET_INUTILISABLE increments={d1:+.3e},{d2:+.3e}");
            sortie.push((nom, None));
        }
    }
    for (nom, ordre) in &sortie {
        let verdict = match ordre {
            Some(o) if *o >= 1.8 => format!("TENU, ordre {o:.3}"),
            Some(o) => format!("MANQUÉ, ordre {o:.3}"),
            None => "SANS ORDRE".to_string(),
        };
        println!("FOND3D_S324 verdict fond={nom} critere_ordre_1_8={verdict}");
    }
}
