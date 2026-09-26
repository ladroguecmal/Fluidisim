//! **La course de la surface sous la mer de la porte B** — S387, C2b de la campagne du solveur volumique 3D
//! ([ADR-208](../../../docs/adr/ADR-208-la-colonne-graduee.md) D4 : au pas mobile, les couches cubiques d'une colonne
//! graduée doivent contenir toute la course de la surface).
//!
//! La mer `--houle` de l'afficheur (S259, `viewer/src/scene.rs`, `Scene::build`) reconstruite à l'identique : mer de vent de
//! S201 (`Hs` 1,5 m, `Tp` 6 s, 32 composantes, graine 201, étalée) et houle (`Hs` 2 m, `Tp` 12 s, 32 composantes, graine
//! 202), assemblées, ancrées à l'origine du monde. Le domaine de la porte B (`Config::review`) : 30 × 28 m, coin à
//! `(−15, 0)`, repos à 3,5 m au-dessus de son fond, 28 couches de 25 cm. `B` est évalué au centre d'une colonne sur quatre
//! (un mètre), toutes les 0,1 s pendant `DUREE` secondes (600 par défaut).
//!
//! Deux lectures : l'élévation de `B` telle quelle, et son écart à **sa moyenne sur le domaine** au même instant — ce qu'un
//! domaine qui monterait et descendrait avec la mer verrait. Le paquet de la scène (0,65 m) s'y ajoute.
//!
//!     cargo run -p water-core --release --offline --example delta3d_course_surface

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::background::{Background, SeaState};
use water_core::background_spectrum::{self, Recipe};
use water_core::host::HostServices;
use water_core::types::{SimTime, WorldPos};

fn main() {
    let duree: f64 = std::env::var("DUREE").ok().and_then(|v| v.parse().ok()).unwrap_or(600.);
    let wind = Recipe {
        sea: SeaState { hs: 1.5, tp: 6., theta_turns: 0.12, components: 32, graine: 201 },
        gravity: 9.81,
        gamma: 3.3,
        min_ratio: 0.5,
        max_ratio: 4.,
        spread_turns: 0.25,
    };
    let swell = Recipe {
        sea: SeaState { hs: 2., tp: 12., theta_turns: 0., components: 32, graine: 202 },
        gravity: 9.81,
        gamma: 7.,
        min_ratio: 0.7,
        max_ratio: 1.6,
        spread_turns: 0.,
    };
    let wind = background_spectrum::bake_directional(wind, 10.).expect("vent S259");
    let swell = background_spectrum::bake_directional(swell, 75.).expect("houle S259");
    let cooked = background_spectrum::assemble(&[&wind, &swell]).expect("mer S259");
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let b = Background::from_spectrum(
        &mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink },
        &cooked,
        WorldPos::from_units(0, 0, 0),
    )
    .expect("fond");
    let (nx, ny, dx, origin) = (120usize, 112usize, 0.25f64, [-15f64, 0f64]);
    let points: Vec<WorldPos> = (0..ny)
        .step_by(4)
        .flat_map(|j| {
            (0..nx).step_by(4).map(move |i| {
                WorldPos::from_metres(origin[0] + (i as f64 + 0.5) * dx, origin[1] + (j as f64 + 0.5) * dx, 0.)
            })
        })
        .collect();
    let steps = (duree * 10.).round() as u64;
    let (mut lo, mut hi, mut rlo, mut rhi) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    let mut values = vec![0f64; points.len()];
    let mut all = Vec::with_capacity(points.len() * steps as usize);
    let mut residuals = Vec::with_capacity(points.len() * steps as usize);
    for s in 0..=steps {
        let t = SimTime::from_micros(s * 100_000);
        for (v, p) in values.iter_mut().zip(&points) {
            *v = b.eval(*p, t).expect("échantillon").eta as f64;
        }
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        for v in &values {
            lo = lo.min(*v);
            hi = hi.max(*v);
            rlo = rlo.min(v - mean);
            rhi = rhi.max(v - mean);
            all.push(*v);
            residuals.push(v - mean);
        }
    }
    let quantile = |x: &mut Vec<f64>, q: f64| {
        x.sort_by(|a, b| a.total_cmp(b));
        x[((x.len() - 1) as f64 * q).round() as usize]
    };
    let (q_lo, q_hi) = (quantile(&mut all, 0.001), quantile(&mut all, 0.999));
    let (r_lo, r_hi) = (quantile(&mut residuals, 0.001), quantile(&mut residuals, 0.999));
    println!("S387 — la course de B sur le domaine de la porte B, {duree} s, {} points, 0,1 s", points.len());
    println!(
        "COURSE_S387 lecture=B min_m={lo:+.3} max_m={hi:+.3} course_m={:.3} q001_m={q_lo:+.3} q999_m={q_hi:+.3}",
        hi - lo
    );
    println!(
        "COURSE_S387 lecture=B_moins_moyenne_du_domaine min_m={rlo:+.3} max_m={rhi:+.3} course_m={:.3} q001_m={r_lo:+.3} \
         q999_m={r_hi:+.3}",
        rhi - rlo
    );
    // Couches cubiques qu'impose la course : la surface, plus le paquet de 0,65 m de part et d'autre, dans des mailles de
    // 25 cm, et une maille mouillée entière sous le creux.
    for (nom, bas, haut) in [("B", lo, hi), ("B_moins_moyenne_du_domaine", rlo, rhi)] {
        let course = (haut + 0.65) - (bas - 0.65);
        let cubiques = (course / dx).ceil() as usize + 1;
        let sous_le_creux = 3.5 + bas - 0.65 - dx;
        println!(
            "COURSE_S387 lecture={nom} avec_paquet_course_m={course:.3} couches_cubiques_min={cubiques} \
             profondeur_libre_sous_le_creux_m={sous_le_creux:.3}"
        );
    }
}
