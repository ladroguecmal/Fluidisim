//! S367, critère 2 — **la couverture de moutons du champ d'écume contre Monahan** (banc B9, scénario 1 ; liste 7.1).
//!
//! Mer de vent pleinement développée (Pierson–Moskowitz, `fully_developed_wind_sea`, étalement cos^2s, s = 10), 64
//! composantes ; champ de 128 × 128 texels d'un mètre ; pas de 0,2 s, 10 s de mise en régime puis 20 s de mesure. Couverture
//! = part des texels où l'écume **active** dépasse ½ (les moutons de Monahan : stades A et B, le blanc) ; publiées aussi
//! la part en déferlement à l'instant (indicateur > ½) et celle du résiduel (> 0,2). Monahan et O'Muircheartaigh (1980) :
//! `W = 3,84·10⁻⁶·U10^3,41`.
//!
//! `cargo run -p water-core --release --offline --example ecume_couverture -- [seuil_g …]`

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

fn monahan(u: f32) -> f32 { 3.84e-6 * u.powf(3.41) }

/// Couvertures moyennes (active > ½, déferlement > ½, résiduel > 0,2) pour le vent `u` et le seuil `seuil_g`.
fn mesurer(u: f32, seuil_g: f32) -> (f32, f32, f32, f32) {
    mesurer_graine(u, seuil_g, 7)
}

fn mer(u: f32, graine: u64) -> Background {
    let mut alloc = Hote;
    let services = Hote;
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    let cuite = bake_directional(fully_developed_wind_sea(u, 0.25, 64, graine, 9.81), 10.0).expect("spectre");
    Background::from_spectrum(&mut host, &cuite, WorldPos::from_metres(0.0, 0.0, 0.0)).unwrap()
}

fn mesurer_graine(u: f32, seuil_g: f32, graine: u64) -> (f32, f32, f32, f32) {
    let mut alloc = Hote;
    let services = Hote;
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    let recette = fully_developed_wind_sea(u, 0.25, 64, graine, 9.81);
    let cuite = bake_directional(recette, 10.0).expect("spectre");
    let b = Background::from_spectrum(&mut host, &cuite, WorldPos::from_metres(0.0, 0.0, 0.0)).unwrap();
    let mut f = ChampEcume::nouveau(128, 1.0, [-64.0, -64.0]);
    f.seuil_g = seuil_g;
    let dt_us = 200_000u64;
    let (mut ca, mut cd, mut cr, mut n) = (0f32, 0f32, 0f32, 0f32);
    let t0 = 60_000_000u64;
    for k in 0..150u64 {
        let t = SimTime::from_micros(t0 + k * dt_us);
        f.pas_de_temps(&b, t, dt_us, true);
        if k >= 50 {
            let t1 = SimTime::from_micros(t0 + (k + 1) * dt_us);
            let mut deferle = 0usize;
            for j in 0..f.cote() {
                for i in 0..f.cote() {
                    if f.deferlement(&b, f.centre(i, j), t1) > 0.5 {
                        deferle += 1;
                    }
                }
            }
            ca += ChampEcume::couverture(f.actif(), 0.5);
            cr += ChampEcume::couverture(f.residuel(), 0.2);
            cd += deferle as f32 / (f.cote() * f.cote()) as f32;
            n += 1.0;
        }
    }
    let hs = 4.0 * b.components().iter().map(|c| 0.5 * (c.amplitude as f64).powi(2)).sum::<f64>().sqrt();
    (ca / n, cd / n, cr / n, hs as f32)
}

/// L'écart-type de l'accélération verticale de B, rapporté à g : `σ_a² = Σ ½·(a·ω²)²`.
fn sigma_a(u: f32) -> (f32, f32, f32) {
    let recette = fully_developed_wind_sea(u, 0.25, 64, 7, 9.81);
    let cuite = bake_directional(recette, 10.0).expect("spectre");
    let (mut v, mut fmin, mut fmax) = (0f64, f64::MAX, 0f64);
    for c in cuite.components() {
        let hz = c.freq_q32 as f64 / 4_294_967_296.0;
        let w = core::f64::consts::TAU * hz;
        v += 0.5 * (c.amplitude as f64 * w * w).powi(2);
        fmin = fmin.min(hz);
        fmax = fmax.max(hz);
    }
    ((v.sqrt() / 9.81) as f32, fmin as f32, fmax as f32)
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("sigma") {
        for u in [7.0f32, 10.0, 13.0] {
            let (s, fmin, fmax) = sigma_a(u);
            println!("SIGMA_S367 U10={u} sigma_a_sur_g={s:.4} bande_hz=[{fmin:.3}, {fmax:.3}] rapport={:.2}", fmax / fmin);
        }
        return;
    }
    // `verifier` : le seuil calé (`seuil_pour_couverture`) sur des mers d'une autre graine — des réalisations que le
    // calage n'a pas vues ; critère, à un facteur 1,5 de Monahan.
    if std::env::args().nth(1).as_deref() == Some("verifier") {
        for u in [7.0f32, 10.0, 13.0] {
            let b = mer(u, 11);
            let seuil = water_core::ecume::seuil_pour_couverture(&b, monahan(u));
            let (ca, cd, cr, hs) = mesurer_graine(u, seuil, 11);
            let w = monahan(u);
            println!("VERIFIER_S367 U10={u} graine=11 Hs={hs:.2} seuil_g={seuil:.4} active={:.3}% deferlement={:.3}%                 residuel={:.3}% monahan={:.3}% rapport={:.2}", ca * 100.0, cd * 100.0, cr * 100.0, w * 100.0, ca / w);
        }
        return;
    }
    // `grand U10 graine` : le même seuil calé, sur un champ de 384 m mesuré 40 s — le bruit de réalisation d'un champ
    // de 128 m, qui ne tient qu'une longueur d'onde de pic à 13 m/s, en question.
    if std::env::args().nth(1).as_deref() == Some("grand") {
        let u: f32 = std::env::args().nth(2).and_then(|a| a.parse().ok()).unwrap_or(13.0);
        let graine: u64 = std::env::args().nth(3).and_then(|a| a.parse().ok()).unwrap_or(11);
        let b = mer(u, graine);
        let seuil = water_core::ecume::seuil_pour_couverture(&b, monahan(u));
        let mut f = ChampEcume::nouveau(384, 1.0, [-192.0, -192.0]);
        f.seuil_g = seuil;
        let (mut ca, mut n) = (0f32, 0f32);
        for k in 0..250u64 {
            f.pas_de_temps(&b, SimTime::from_micros(60_000_000 + k * 200_000), 200_000, true);
            if k >= 50 {
                ca += ChampEcume::couverture(f.actif(), 0.5);
                n += 1.0;
            }
        }
        println!("GRAND_S367 U10={u} graine={graine} champ=384m mesure=40s seuil_g={seuil:.4} active={:.3}% monahan={:.3}%             rapport={:.2}", ca / n * 100.0, monahan(u) * 100.0, ca / n / monahan(u));
        return;
    }
    // `kappa k1 k2 …` : le seuil en écarts-types de l'accélération, `κ·σ_a/g`, pour chaque vent.
    if std::env::args().nth(1).as_deref() == Some("kappa") {
        let kappas: Vec<f32> = std::env::args().skip(2).filter_map(|a| a.parse().ok()).collect();
        for u in [7.0f32, 10.0, 13.0] {
            let (sa, _, _) = sigma_a(u);
            for k in &kappas {
                let (ca, cd, cr, hs) = mesurer(u, k * sa);
                let w = monahan(u);
                println!("KAPPA_S367 U10={u} Hs={hs:.2} kappa={k:.2} seuil_g={:.4} active={:.3}% deferlement={:.3}%                     residuel={:.3}% monahan={:.3}% rapport={:.2}", k * sa, ca * 100.0, cd * 100.0, cr * 100.0,
                    w * 100.0, ca / w);
            }
        }
        return;
    }
    let seuils: Vec<f32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let seuils = if seuils.is_empty() { vec![0.45] } else { seuils };
    println!("=== S367 — la couverture du champ d'écume contre Monahan (B9, scénario 1) ===");
    for seuil in seuils {
        for u in [7.0f32, 10.0, 13.0] {
            let (ca, cd, cr, hs) = mesurer(u, seuil);
            let w = monahan(u);
            println!("COUVERTURE_S367 seuil_g={seuil:.3} U10={u} Hs={hs:.2} active={:.3}% deferlement={:.3}% \
                residuel={:.3}% monahan={:.3}% rapport={:.2}", ca * 100.0, cd * 100.0, cr * 100.0, w * 100.0, ca / w);
        }
    }
}
