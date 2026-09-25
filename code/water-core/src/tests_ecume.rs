//! S367 — la référence du champ d'écume (critères écrits avant le code, EN-COURS S367).
use super::*;
use crate::background::{Background, Component};
use crate::hash::Hasher64;
use crate::host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};
use crate::phase::{freq_hz_to_q32, PhaseQ32};
use crate::types::WorldPos;

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

const G: f32 = 9.81;

/// Une onde seule en eau profonde, de cambrure `ak`, période `t` s, direction `+x`.
fn onde(ak: f64, periode: f64, phase0: u32) -> Component {
    let hz = 1.0 / periode;
    let omega = core::f64::consts::TAU * hz;
    let k = omega * omega / G as f64;
    Component { amplitude: (ak / k) as f32, k_turns_per_m: (k / core::f64::consts::TAU) as f32, dir: [1.0, 0.0],
        freq_q32: freq_hz_to_q32(hz), phase0: PhaseQ32(phase0) }
}

fn fond(c: &[Component]) -> Background {
    let mut alloc = Hote;
    let services = Hote;
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    Background::from_components(&mut host, c, WorldPos::from_metres(0.0, 0.0, 0.0), G).unwrap()
}

/// **Critère 1a — la décroissance.** Deux texels, `(Fa, Fr) = (1, 0)` et `(0,3 ; 0,5)`, quarante pas de 0,25 s sans
/// source ni advection, contre la solution fermée du système en f64 à 10 s.
#[test]
fn decroissance_exacte_s367() {
    let mut f = ChampEcume::nouveau(2, 1.0, [0.0, 0.0]);
    f.actif_mut()[0] = 1.0;
    f.actif_mut()[1] = 0.3;
    f.residuel_mut()[1] = 0.5;
    for _ in 0..40 {
        f.decroitre(0.25);
    }
    let (la, lr) = (core::f64::consts::LN_2 / 3.0, core::f64::consts::LN_2 / 30.0);
    let t = 10.0f64;
    let attendu = |a0: f64, r0: f64| {
        (a0 * (-la * t).exp(), r0 * (-lr * t).exp() + a0 * la / (la - lr) * ((-lr * t).exp() - (-la * t).exp()))
    };
    let mut pire = 0f64;
    for (k, (a0, r0)) in [(1.0, 0.0), (0.3, 0.5)].iter().enumerate() {
        let (a, r) = attendu(*a0, *r0);
        pire = pire.max((f.actif()[k] as f64 - a).abs()).max((f.residuel()[k] as f64 - r).abs());
        println!("S367 decroissance texel={k} actif={:.7}/{a:.7} residuel={:.7}/{r:.7}", f.actif()[k], f.residuel()[k]);
    }
    println!("S367 decroissance pire_ecart={pire:.2e}");
    assert!(pire <= 1e-6, "{pire}");
}

/// **Critère 1b — l'advection.** Une bosse gaussienne (σ = 2 m) sur 32 m au pas de 0,25 m, vitesse uniforme
/// (0,37 ; −0,21) m/s, cent pas de 0,1 s : le centre de masse se déplace de (3,7 ; −2,1) m. Publiés : l'écart du centre,
/// la perte au sommet (diffusion numérique de l'interpolation), la masse.
#[test]
fn advection_translation_s367() {
    let (n, pas) = (128usize, 0.25f32);
    let mut f = ChampEcume::nouveau(n, pas, [0.0, 0.0]);
    let (x0, y0, s) = (10.0f32, 20.0f32, 2.0f32);
    for j in 0..n {
        for i in 0..n {
            let c = f.centre(i, j);
            let r2 = (c[0] - x0).powi(2) + (c[1] - y0).powi(2);
            f.actif_mut()[j * n + i] = (-0.5 * r2 / (s * s)).exp();
        }
    }
    let masse = |f: &ChampEcume| {
        let (mut m, mut mx, mut my) = (0f64, 0f64, 0f64);
        for j in 0..n {
            for i in 0..n {
                let v = f.actif()[j * n + i] as f64;
                let c = f.centre(i, j);
                m += v;
                mx += v * c[0] as f64;
                my += v * c[1] as f64;
            }
        }
        (m, mx / m, my / m)
    };
    let (m0, _, _) = masse(&f);
    let u = [0.37f32, -0.21f32];
    for _ in 0..100 {
        f.advecter(&|_| u, 0.1);
    }
    let (m1, cx, cy) = masse(&f);
    let ecart = ((cx - (x0 as f64 + 3.7)).powi(2) + (cy - (y0 as f64 - 2.1)).powi(2)).sqrt();
    let sommet = f.actif().iter().cloned().fold(0f32, f32::max);
    println!("S367 advection centre=({cx:.4}, {cy:.4}) attendu=({:.4}, {:.4}) ecart_m={ecart:.2e} sommet={sommet:.4} \
        masse_relative={:.2e}", x0 + 3.7, y0 - 2.1, m1 / m0 - 1.0);
    assert!(ecart <= 1e-3, "{ecart}");
    assert!((m1 / m0 - 1.0).abs() <= 1e-4, "{}", m1 / m0);
}

/// Le seuil, sur une onde seule : à la crête, l'accélération descendante vaut `a·ω² = a·k·g` ; l'indicateur y vaut 0 à
/// `a·k` = 0,30, ½ à 0,45, 1 à 0,60.
#[test]
fn seuil_sur_une_onde_s367() {
    for (ak, attendu) in [(0.30, 0.0f32), (0.45, 0.5), (0.60, 1.0)] {
        let b = fond(&[onde(ak, 8.0, 0)]);
        let f = ChampEcume::nouveau(1, 1.0, [0.0, 0.0]);
        let lambda = G as f64 * 64.0 / core::f64::consts::TAU;
        let mut max = 0f32;
        for i in 0..2000 {
            max = max.max(f.deferlement(&b, [(i as f64 * lambda / 2000.0) as f32, 0.0], SimTime::from_micros(0)));
        }
        println!("S367 seuil ak={ak} indicateur_max={max:.4} attendu={attendu}");
        assert!((max - attendu).abs() <= 0.01, "{ak} {max}");
    }
}

/// **Critère 1c — le déterminisme.** Une mer de trois ondes, 64 × 64 texels, vingt pas : deux passes, même hash.
#[test]
fn determinisme_s367() {
    let b = fond(&[onde(0.35, 6.0, 1), onde(0.30, 4.5, 7_000_000), onde(0.25, 3.0, 99_000_000)]);
    let passe = || {
        let mut f = ChampEcume::nouveau(64, 0.5, [-16.0, -16.0]);
        for k in 0..20u64 {
            f.pas_de_temps(&b, SimTime::from_micros(k * 100_000), 100_000, true);
        }
        let mut h = Hasher64::new();
        for (a, r) in f.actif().iter().zip(f.residuel()) {
            h.write_f32(*a);
            h.write_f32(*r);
        }
        (h.finish(), ChampEcume::couverture(f.actif(), 0.5))
    };
    let (h1, c1) = passe();
    let (h2, _) = passe();
    println!("S367 determinisme hash={h1:016x} couverture_active={c1:.4}");
    assert_eq!(h1, h2);
}
