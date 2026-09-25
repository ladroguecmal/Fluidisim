//! S364, critères 4 et 5 — **le coût et la mémoire de B sur la côte** (`Cote`, liste 2.7).
//!
//! Même méthode que `banc_b1` : 4 096 points par campagne, 64 campagnes, le **minimum** est le chiffre à lire. B à
//! 32 composantes (B1), Hs = 1 m, Tp = 8 s — périodes de 4 à 16 s ; la plage commence donc à λ₀ de la composante de
//! 16 s (400 m de fond) et descend au 1/30 jusqu'à 2 m. Deux dispositions : points **groupés** (grille de 3 m, comme
//! B1 — les tables restent en cache) et points **dispersés** sur toute la plage (le pire des accès aux tables).
//!
//! `cargo run -p water-core --release --offline --example bathymetrie_cote_cout`

use std::{hint::black_box, time::Instant};

use water_core::background::{Background, SeaState, COMPOSANTES_B1};
use water_core::bathymetrie_cote::Cote;
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

/// La plage : 400 m (λ₀ de 16 s) à 2 m, au 1/30 ; la table commence à `y_local = −8 000 m`, et les points évalués
/// restent dans le domaine local de B (I-08).
const ORIGINE: f64 = -8000.0;
const LONGUEUR: f64 = 11_940.0;
fn plage(y: f64) -> f64 { 400.0 - y / 30.0 }

fn cout_ns(eval: &dyn Fn(WorldPos, SimTime) -> Option<f32>, points: &[WorldPos]) -> f64 {
    for p in points.iter().take(64) {
        black_box(eval(*p, SimTime(0)));
    }
    let mut mesures = Vec::with_capacity(64);
    for r in 0..64u64 {
        let t = SimTime(r * 1_000);
        let debut = Instant::now();
        for p in points {
            black_box(eval(*p, t));
        }
        mesures.push(debut.elapsed().as_secs_f64() * 1e9 / points.len() as f64);
    }
    mesures.sort_by(|a, b| a.partial_cmp(b).unwrap());
    mesures[0]
}

fn main() {
    let mut alloc = Hote;
    let services = Hote;
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    let sea = SeaState { hs: 1.0, tp: 8.0, theta_turns: 0.25, components: COMPOSANTES_B1, graine: 7 };
    let b = Background::configure(&mut host, sea, WorldPos::from_metres(0.0, 0.0, 0.0)).unwrap();
    let n_comp = b.component_count() as f64;
    let groupes: Vec<WorldPos> = (0..4096)
        .map(|i| WorldPos::from_metres((i % 64) as f64 * 3.0 - 96.0, (i / 64) as f64 * 3.0 + 1000.0, 0.0))
        .collect();
    // Dispersés : un pas premier avec 4 096, sur 7 800 m de plage, de −3 900 à +3 900 m.
    let disperses: Vec<WorldPos> = (0..4096u64)
        .map(|i| {
            let j = (i * 2657) % 4096;
            WorldPos::from_metres(((i * 37) % 200) as f64 - 100.0, -3900.0 + 7800.0 * j as f64 / 4096.0, 0.0)
        })
        .collect();

    println!("=== S364 — B sur la côte : coût et mémoire ===");
    println!("B : {} composantes, Hs = 1 m, Tp = 8 s ; plage 400 → 2 m au 1/30 ({LONGUEUR} m).", b.component_count());
    let fond_seul = |p: WorldPos, t: SimTime| b.eval(p, t).map(|s| s.eta);
    let b_g = cout_ns(&fond_seul, &groupes);
    let b_d = cout_ns(&fond_seul, &disperses);
    println!("COUT_S364 B        groupes={b_g:.1} ns ({:.2} ns/composante)  disperses={b_d:.1} ns ({:.2} ns/composante)",
        b_g / n_comp, b_d / n_comp);
    for pas in [1.0, 2.0, 5.0] {
        let cote = Cote::cuire(&mut host, &b, [0.0, 1.0], ORIGINE, LONGUEUR, pas, &plage).unwrap();
        let cotiere = |p: WorldPos, t: SimTime| cote.eval(&b, p, t).map(|s| s.eta);
        let c_g = cout_ns(&cotiere, &groupes);
        let c_d = cout_ns(&cotiere, &disperses);
        let (n, _) = cote.echantillons();
        println!("COUT_S364 cote pas={pas} m  groupes={c_g:.1} ns ({:.2} ns/composante, x{:.2})  disperses={c_d:.1} ns \
            ({:.2} ns/composante, x{:.2})  echantillons={n} tables={:.2} Mo  par_km_de_profil={:.1} Ko",
            c_g / n_comp, c_g / b_g, c_d / n_comp, c_d / b_d, cote.octets() as f64 / 1e6,
            cote.octets() as f64 / (LONGUEUR / 1000.0) / 1e3);
    }
    // Ce qu'une bathymétrie 2D coûterait avec des tables régulières du même contenu : arithmétique, pas mesure.
    for pas in [2.0f64, 5.0, 10.0, 50.0] {
        let par_km2 = (1000.0 / pas).powi(2) * n_comp * 16.0;
        println!("MEMOIRE_2D_S364 pas={pas} m  {:.1} Mo/km² pour {} composantes", par_km2 / 1e6, b.component_count());
    }
}
