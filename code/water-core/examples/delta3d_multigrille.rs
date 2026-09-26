//! **La multigrille 3D de la référence, mesurée** — S385, campagne du solveur volumique 3D, C1
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)).
//!
//! Le cas mobile de S328 (`delta3d_fond_coupe --mobile`) : un domaine de 8 × 4 m, surface au repos à 4 m dans 6 m de
//! hauteur, perturbée d'une sinusoïde de 1 cm le long de `x` ; fond **plat** ou la **bosse** de S324, décentrée en `y`.
//! À trois mailles (`nx` = 32, 64, 128), le **premier pas** depuis le repos (pression de départ nulle, le plus dur), puis
//! **dix pas** à départ chaud, comme en usage. Le préconditionneur est celui du pas mobile — Jacobi, ou le cycle en V de
//! S385 avec `--multigrille`.
//!
//! Deux divergences sont publiées : sur toutes les lignes (`divergence`) et sur les lignes franches seulement
//! (`divergence_plain`), la seule que la tolérance d'ADR-144 juge.
//!
//! Critère 3 du plan de S385, écrit avant la mesure : avec la multigrille, les itérations ne croissent pas de plus de
//! 50 % de la maille la plus grossière à la plus fine ; avec Jacobi, elles croissent au moins du double.
//!
//!     cargo run -p water-core --release --offline --example delta3d_multigrille [-- --multigrille] [-- --fin]
//!
//! Sans `--fin`, `nx` = 32 et 64 seulement (une minute) ; `--fin` ajoute 128 (plusieurs minutes).

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::delta3d::{Domain3, Volume3};
use water_core::host::HostServices;

const LX: f32 = 8.0;
const LZ: f32 = 4.0;
const RHO: f32 = 1025.0;
const G: f32 = 9.81;
/// Pas de S232 et de S328, 2 ms, en microsecondes.
const DT_US: u64 = 2000;
const A: f32 = 0.01;
const PAS_CHAUDS: usize = 10;
const ITERATIONS_MAX: u32 = 20_000;

fn plat(_x: f32, _y: f32) -> f32 {
    0.
}

/// La bosse de S324, décentrée en `y`.
fn bosse(x: f32, y: f32) -> f32 {
    let (a, b) = ((x - 3.0) / 1.2, (y - 1.3) / 0.9);
    0.4 + 0.6 * (-(a * a + b * b)).exp()
}

struct Mesure {
    mailles: usize,
    it_premier: u32,
    it_moyen: f64,
    duree_premier: f64,
    duree_moyenne: f64,
    divergence: f64,
    divergence_franche: f64,
}

fn mesure(fond: fn(f32, f32) -> f32, coupe: bool, nx: usize, multigrille: bool) -> Mesure {
    let (ny, nz) = (nx / 2, 3 * nx / 4);
    let dx = LX / nx as f32;
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let domaine = Domain3 { nx, ny, nz, dx };
    let mut v = if coupe {
        let b: Vec<f32> = (0..ny)
            .flat_map(|j| (0..nx).map(move |i| fond((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)))
            .collect();
        Volume3::configure_with_bottom(&mut hote, domaine, RHO, G, &b).expect("configuration")
    } else {
        Volume3::configure(&mut hote, domaine, RHO, G).expect("configuration")
    };
    if multigrille {
        v.enable_multigrid(&mut hote).expect("multigrille");
    }
    let eta: Vec<f32> = (0..ny)
        .flat_map(|_| (0..nx).map(move |i| LZ + A * (core::f32::consts::TAU * (i as f32 + 0.5) * dx / LX).sin()))
        .collect();
    v.set_free_surface(&eta, LZ).expect("surface");
    let debut = Instant::now();
    let r = v.step_surface_mobile(DT_US, ITERATIONS_MAX, &jobs).expect("premier pas");
    let duree_premier = debut.elapsed().as_secs_f64();
    assert!(!r.degraded, "premier pas dégradé");
    let (mut total, mut divergence, mut franche) = (0u64, r.divergence, r.divergence_plain);
    let debut = Instant::now();
    for _ in 0..PAS_CHAUDS {
        let r = v.step_surface_mobile(DT_US, ITERATIONS_MAX, &jobs).expect("pas chaud");
        assert!(!r.degraded, "pas chaud dégradé");
        total += r.iterations as u64;
        divergence = divergence.max(r.divergence);
        franche = franche.max(r.divergence_plain);
    }
    let duree_moyenne = debut.elapsed().as_secs_f64() / PAS_CHAUDS as f64;
    Mesure {
        mailles: nx * ny * nz,
        it_premier: r.iterations,
        it_moyen: total as f64 / PAS_CHAUDS as f64,
        duree_premier,
        duree_moyenne,
        divergence,
        divergence_franche: franche,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let multigrille = args.iter().any(|a| a == "--multigrille");
    let mailles: &[usize] = if args.iter().any(|a| a == "--fin") { &[32, 64, 128] } else { &[32, 64] };
    let methode = if multigrille { "multigrille" } else { "jacobi" };
    println!("S385 — pas mobile 3D, préconditionneur {methode} : itérations et durée à trois mailles");
    for (nom, fond, coupe) in [("plat", plat as fn(f32, f32) -> f32, false), ("bosse", bosse, true)] {
        let mut premiers = Vec::new();
        for &nx in mailles {
            let m = mesure(fond, coupe, nx, multigrille);
            println!(
                "MG3D_S385 fond={nom} methode={methode} nx={nx} mailles={} it_premier={} it_moyen={:.1} \
                 duree_premier_s={:.3} duree_moyenne_s={:.3} divergence_max={:.3e} divergence_franche_max={:.3e}",
                m.mailles, m.it_premier, m.it_moyen, m.duree_premier, m.duree_moyenne, m.divergence, m.divergence_franche
            );
            premiers.push((m.it_premier as f64, m.it_moyen));
        }
        let (a, b) = (premiers[0], premiers[premiers.len() - 1]);
        println!(
            "MG3D_S385 fond={nom} methode={methode} croissance_premier={:.2} croissance_moyen={:.2}",
            b.0 / a.0,
            b.1 / a.1.max(1.)
        );
    }
}
