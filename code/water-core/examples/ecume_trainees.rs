//! S367, critère 3 — **les traînées de l'écume résiduelle** (banc B9 ; ADR-014 §2.1 ; liste 7.1).
//!
//! ADR-014 affirme qu'advectée par la vitesse orbitale **complète**, l'écume s'accumule dans les zones de convergence et
//! forme d'elle-même des traînées alignées au vent. Mesure : même mer de vent pleinement développée (U10 = 10 m/s, vent
//! et vagues vers +y), même seuil, 90 s ; le champ résiduel advecté par la vitesse orbitale de B, contre le témoin sans
//! advection (B n'a ni courant ni vent). Longueurs de corrélation du résiduel le long du vent (y) et en travers (x) :
//! première distance où l'autocorrélation, moyenne retirée, tombe sous 1/e.
//!
//! `cargo run -p water-core --release --offline --example ecume_trainees -- <seuil_g>`

use water_core::background::Background;
use water_core::background_spectrum::{bake_directional, fully_developed_wind_sea};
use water_core::ecume::ChampEcume;
use water_core::host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};
use water_core::types::{SimTime, WorldPos};

struct Hote;
impl Allocator for Hote {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> { Ok(0) }
    fn seal(&mut self) {}
    fn is_sealed(&self) -> bool { false }
    fn stats(&self) -> AllocStats { AllocStats::default() }
}
impl Sink for Hote {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Hote {
    fn worker_count(&self) -> u32 { 1 }
    fn parallel_reduce_ordered_f64(&self, n: usize, _: usize, reduce: &dyn Fn(usize, usize) -> f64,
        merge: &dyn Fn(f64, f64) -> f64, init: f64) -> f64 { merge(init, reduce(0, n)) }
}

/// Longueur de corrélation (texels) du champ `c` (n × n) selon `dx, dy` : première distance où l'autocorrélation normée
/// passe sous 1/e ; interpolation linéaire entre les deux décalages qui l'encadrent.
fn correlation(c: &[f32], n: usize, dx: usize, dy: usize) -> f32 {
    let m = c.iter().map(|v| *v as f64).sum::<f64>() / c.len() as f64;
    let var = c.iter().map(|v| (*v as f64 - m).powi(2)).sum::<f64>() / c.len() as f64;
    if var <= 0.0 {
        return 0.0;
    }
    let mut precedent = 1.0f64;
    for d in 1..n / 2 {
        let (mut s, mut k) = (0f64, 0usize);
        for j in 0..n - d * dy {
            for i in 0..n - d * dx {
                s += (c[j * n + i] as f64 - m) * (c[(j + d * dy) * n + i + d * dx] as f64 - m);
                k += 1;
            }
        }
        let r = s / k as f64 / var;
        if r < (-1.0f64).exp() {
            return (d as f64 - 1.0 + (precedent - (-1.0f64).exp()) / (precedent - r)) as f32;
        }
        precedent = r;
    }
    (n / 2) as f32
}

fn main() {
    let seuil: f32 = std::env::args().nth(1).and_then(|a| a.parse().ok()).expect("seuil en fractions de g");
    let mut alloc = Hote;
    let services = Hote;
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    let cuite = bake_directional(fully_developed_wind_sea(10.0, 0.25, 64, 7, 9.81), 10.0).expect("spectre");
    let b = Background::from_spectrum(&mut host, &cuite, WorldPos::from_metres(0.0, 0.0, 0.0)).unwrap();
    // `periodique` : l'écume qui sort par un bord rentre par l'autre (sans quoi le bord d'entrée, vide, fabrique un
    // gradient le long des vagues). Deux résolutions : un allongement dû à la diffusion de l'interpolation baisse avec
    // le texel ; un allongement physique, non. Longueurs en mètres.
    let periodique = std::env::args().nth(2).as_deref() == Some("periodique");
    for (n, pas) in [(128usize, 1.0f32), (256, 0.5)] {
        for orbitale in [true, false] {
            let demi = n as f32 * pas / 2.0;
            let mut f = ChampEcume::nouveau(n, pas, [-demi, -demi]);
            f.seuil_g = seuil;
            f.periodique = periodique;
            for k in 0..450u64 {
                f.pas_de_temps(&b, SimTime::from_micros(60_000_000 + k * 200_000), 200_000, orbitale);
            }
            let (lx, ly) = (correlation(f.residuel(), n, 1, 0) * pas, correlation(f.residuel(), n, 0, 1) * pas);
            let (ax, ay) = (correlation(f.actif(), n, 1, 0) * pas, correlation(f.actif(), n, 0, 1) * pas);
            let moyen = f.residuel().iter().sum::<f32>() / f.residuel().len() as f32;
            let nom = if orbitale { "orbitale" } else { "aucune" };
            println!("TRAINEES_S367 periodique={periodique} pas={pas} advection={nom} residuel_Lx={lx:.2}m Ly={ly:.2}m Ly/Lx={:.2} actif_Lx={ax:.2}m Ly={ay:.2}m Ly/Lx={:.2} residuel_moyen={moyen:.3}", ly / lx, ay / ax);
        }
    }
}
