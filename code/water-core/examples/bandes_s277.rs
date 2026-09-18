//! S277 — R10 : **chiffrer « le motif est trop répétitif »**.
//!
//! L'utilisateur a regardé la scène `--delta` et a rendu deux retours : le motif de surface est
//! trop répétitif pour être réaliste, et les vagues doivent interagir avec l'onde. Le premier se
//! mesure, et cet exemple le mesure — avant d'écrire quoi que ce soit dans la revue.
//!
//! Deux recettes, les mêmes 32 composantes, et tout les sépare :
//!
//! - la houle de `--delta` (`viewer/src/delta.rs`) a un **étalement nul** et une bande étroite
//!   `0,7–1,6 fp`. C'est une contrainte de mesure, pas un choix esthétique : le domaine δ est une
//!   tranche 2D `Domain { nx, nz, dx }`, sans dimension `y`, et ne peut porter qu'une houle
//!   constante le long des crêtes ;
//! - la scène S201 (`viewer/src/scene.rs`) étale sur `0,25` tour et couvre `0,5–4 fp`.
//!
//! Ce qui se voit à l'écran n'est donc pas un défaut de rendu : c'est la houle la plus pauvre que
//! le dépôt sache produire, choisie pour que δ soit mesurable.
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;

use water_core::{
    background::Background, background_spectrum::{self, Recipe}, HostServices, SeaState,
    SimTime, WorldPos,
};

/// Houle de `--delta`, recopiée de `viewer/src/delta.rs::swell_recipe`.
fn delta() -> Recipe {
    Recipe {
        sea: SeaState { hs: 2., tp: 8., theta_turns: 0., components: 32, graine: 275 },
        gravity: 9.81, gamma: 3.3, min_ratio: 0.7, max_ratio: 1.6, spread_turns: 0.,
    }
}

/// Mer de la scène S201, recopiée de `viewer/src/scene.rs::build`.
fn s201() -> Recipe {
    Recipe {
        sea: SeaState { hs: 1.5, tp: 6., theta_turns: 0.12, components: 32, graine: 201 },
        gravity: 9.81, gamma: 3.3, min_ratio: 0.5, max_ratio: 4., spread_turns: 0.25,
    }
}

fn ecart_type(v: &[f64]) -> f64 {
    let m = v.iter().sum::<f64>() / v.len() as f64;
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / v.len() as f64).sqrt()
}

fn mesure(nom: &str, r: Recipe) {
    let cooked = background_spectrum::bake(r).expect("recette");
    let c = cooked.components();
    let (mut lmin, mut lmax) = (f64::MAX, 0f64);
    let (mut amin, mut amax) = (f64::MAX, f64::MIN);
    for k in c {
        let lambda = 1. / k.k_turns_per_m as f64;
        lmin = lmin.min(lambda);
        lmax = lmax.max(lambda);
        let a = (k.dir[1] as f64).atan2(k.dir[0] as f64).to_degrees();
        amin = amin.min(a);
        amax = amax.max(a);
    }
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
    let b = Background::from_spectrum(
        &mut HostServices { alloc: &mut alloc, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink },
        &cooked,
        WorldPos::from_units(0, 0, 0),
    )
    .expect("fond");
    let t = SimTime(20_000_000);
    // Le long des crêtes : 201 points sur 200 m, à x fixé. Une houle à crêtes longues y est plate.
    let long: Vec<f64> = (0..201)
        .map(|i| b.eval(WorldPos::from_metres(0., -100. + i as f64, 0.), t).expect("point").eta as f64)
        .collect();
    // En travers : mêmes 201 points, le long de x. C'est là que la houle vit.
    let travers: Vec<f64> = (0..201)
        .map(|i| b.eval(WorldPos::from_metres(-100. + i as f64, 0., 0.), t).expect("point").eta as f64)
        .collect();
    println!(
        "BANDES_S277 {nom} lambda_m min={lmin:.1} max={lmax:.1} rapport={:.1} \
         directions_deg min={amin:.2} max={amax:.2} etalement_deg={:.2} \
         eta_ecart_type_m le_long_des_cretes={:.4} en_travers={:.4}",
        lmax / lmin, amax - amin, ecart_type(&long), ecart_type(&travers)
    );
}

fn main() {
    mesure("delta_S275", delta());
    mesure("scene_S201", s201());
}
