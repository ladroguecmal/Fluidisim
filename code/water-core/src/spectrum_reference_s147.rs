//! Réception de conception, pas un constructeur de production. ADR-100 / SPEC-001 §1 bis.
//! x=f/fp, q(x)=x^-5 exp(-5/(4x^4)) gamma^r ; intégration dans log(x), Jacobien x.
use super::*;
use crate::host::{AllocStats, Allocator, JobSystem, Sink};

struct Host;
impl Allocator for Host {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> { Ok(0) }
    fn seal(&mut self) {}
    fn is_sealed(&self) -> bool { false }
    fn stats(&self) -> AllocStats { AllocStats::default() }
}
impl Sink for Host {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Host {
    fn worker_count(&self) -> u32 { 1 }
    fn parallel_reduce_ordered_f64(&self, n: usize, _: usize,
        reduce: &dyn Fn(usize, usize) -> f64, merge: &dyn Fn(f64, f64) -> f64,
        init: f64) -> f64 { merge(init, reduce(0, n)) }
}

fn shape(x: f64, gamma: f64) -> f64 {
    let sigma = if x <= 1.0 { 0.07 } else { 0.09 };
    let r = (-0.5 * ((x - 1.0) / sigma).powi(2)).exp();
    x.powi(-5) * (-1.25 * x.powi(-4)).exp() * gamma.powf(r)
}

// Simpson en log-fréquence ; les moments dimensionnels valent fp^p fois ces rapports.
fn moment(gamma: f64, lo: f64, hi: f64, p: i32, n: usize) -> f64 {
    assert!(n > 0 && n % 2 == 0 && lo > 0.0 && hi > lo && gamma >= 1.0);
    // Le changement de sigma au pic casse la régularité requise par Simpson.
    if lo < 1.0 && hi > 1.0 {
        return moment(gamma, lo, 1.0, p, n) + moment(gamma, 1.0, hi, p, n);
    }
    let a = lo.ln();
    let h = (hi.ln() - a) / n as f64;
    let mut sum = 0.0;
    for i in 0..=n {
        let x = (a + i as f64 * h).exp();
        let w = if i == 0 || i == n { 1.0 } else if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += w * shape(x, gamma) * x.powi(p + 1);
    }
    sum * h / 3.0
}

// Prototype de la discrétisation décidée : énergie intégrée par cellule logarithmique,
// fréquence au centre géométrique ; amplitude normalisée seulement après l'intégration.
fn discrete(gamma: f64, hi: f64, n: usize, subdivisions: usize, p: i32) -> f64 {
    let ratio = (hi / 0.5).powf(1.0 / n as f64);
    let (mut sum, mut energy) = (0.0, 0.0);
    for i in 0..n {
        let lo = 0.5 * ratio.powi(i as i32);
        let upper = lo * ratio;
        let weight = moment(gamma, lo, upper, 0, subdivisions);
        sum += weight * (lo * upper).sqrt().powi(p);
        energy += weight;
    }
    sum / energy
}

#[test]
fn spectrum_moments_and_truncation_s147() {
    // Seuils de réception numérique de l'instrument, pas des tolérances de mer.
    // Oracle analytique gamma=1 : intégrale q dx = exp(-1.25/x^4)/5.
    for hi in [2.0_f64, 4.0] {
        let exact = ((-1.25 * hi.powi(-4)).exp() - (-20.0_f64).exp()) / 5.0;
        assert!((moment(1.0, 0.5, hi, 0, 16384) / exact - 1.0).abs() < 1e-10);
    }
    // Total m2 analytique sur (0, inf), obtenu par y=1.25/x^4.
    let total_m2 = core::f64::consts::PI.sqrt() / (4.0 * 1.25_f64.sqrt());
    println!("gamma hi retained_m0 retained_m2 Tz_over_Tp m4_over_m0");
    for gamma in [1.0, 3.3, 7.0] { // 7 est une fixture de pic étroit, pas un maximum prescrit.
        // Queue au-delà de 1024 : m0 <=1/(4*1024^4), m2 <=1/(2*1024^2).
        let full0 = moment(gamma, 0.125, 1024.0, 0, 65536);
        let full2 = moment(gamma, 0.125, 1024.0, 2, 65536);
        if gamma == 1.0 {
            assert!((full0 / 0.2 - 1.0).abs() < 1e-10);
            assert!((full2 / total_m2 - 1.0).abs() < 2e-6);
        }
        for hi in [2.0, 4.0] {
            let m0 = moment(gamma, 0.5, hi, 0, 16384);
            let m2 = moment(gamma, 0.5, hi, 2, 16384);
            println!("{gamma:.1} {hi:.1} {:.9} {:.9} {:.9} {:.9}",
                m0 / full0, m2 / full2, (m0 / m2).sqrt(),
                moment(gamma, 0.5, hi, 4, 16384) / m0);
            for p in [0, 1, 2, 4] {
                let reference = moment(gamma, 0.5, hi, p, 32768);
                assert!((moment(gamma, 0.5, hi, p, 16384) / reference - 1.0).abs() < 1e-8);
                if p == 0 { continue; }
                for n in [32, 64, 128, 256] {
                    let value = discrete(gamma, hi, n, 64, p);
                    assert!((value / discrete(gamma, hi, n, 128, p) - 1.0).abs() < 1e-8);
                    let relative = value / (reference / m0) - 1.0;
                    println!("bins gamma={gamma} hi={hi} N={n} p={p} error={relative:.9}");
                    // Enveloppe de quadrature déduite du rapport max x/centre dans une cellule.
                    let bound = (p as f64 * (hi / 0.5).ln() / (2.0 * n as f64)).exp() - 1.0;
                    assert!(relative.abs() <= bound + 1e-8);
                }
            }
        }
    }
    // Contre-épreuve : oublier le Jacobien donne le moment -1, pas la masse spectrale.
    let right = moment(3.3, 0.5, 2.0, 0, 16384);
    let wrong = moment(3.3, 0.5, 2.0, -1, 16384);
    assert!((wrong / right - 1.0).abs() > 0.01);
}

#[test]
fn actual_background_has_equal_log_energy_s147() {
    let mut allocator = Host;
    let services = Host;
    let mut host = HostServices { alloc: &mut allocator, jobs: &services, sink: &services };
    let sea = SeaState { hs: 2.0, tp: 6.0, theta_turns: 0.0, components: 32, graine: 42 };
    let bg = Background::configure(&mut host, sea, WorldPos::from_metres(0.0, 0.0, 0.0)).unwrap();
    let mut moments = [0.0; 3];
    for c in &bg.components {
        let x = c.freq_q32 as f64 / 4294967296.0 * sea.tp as f64;
        let e = 0.5 * (c.amplitude as f64).powi(2);
        for (p, sum) in moments.iter_mut().enumerate() { *sum += e * x.powi(p as i32); }
        assert_eq!(c.amplitude.to_bits(), bg.components[0].amplitude.to_bits());
    }
    assert!((4.0 * moments[0].sqrt() / sea.hs as f64 - 1.0).abs() < 1e-7);
    // Fréquences géométriques de 2 fp à fp/2 : somme géométrique indépendante du constructeur.
    let q = 4.0_f64.powf(-2.0 / 31.0);
    let exact_m2_ratio = 4.0 * (1.0 - q.powi(32)) / (32.0 * (1.0 - q));
    assert!((moments[2] / moments[0] / exact_m2_ratio - 1.0).abs() < 1e-7);
    let target = moment(3.3, 0.5, 2.0, 2, 16384) / moment(3.3, 0.5, 2.0, 0, 16384);
    println!("actual N32 Tz/Tp={:.9} m2/m0={:.9} JONSWAP_same_band={target:.9} relative={:.9}",
        (moments[0] / moments[2]).sqrt(), moments[2] / moments[0],
        moments[2] / moments[0] / target - 1.0);
    // Différence résolue au-delà de 100 fois le budget arithmétique vérifié ci-dessus.
    assert!((moments[2] / moments[0] / target - 1.0).abs() > 100.0 * 1e-7);
}
