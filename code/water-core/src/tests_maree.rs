//! S577 — la marée harmonique. Références écrites au plan par son script.

use super::*;
use crate::traversabilite::{prochain_franchissement, CrossCause};

fn m2_s2() -> Maree {
    Maree::new(&[Composante { periode_h: M2, amplitude_m: 1.0, phase_tours: 0.0 }, Composante { periode_h: S2, amplitude_m: 0.46, phase_tours: 0.0 }],
        0.0).unwrap()
}

/// (1) M2 seule contre le cosinus idéal sur 15 jours ; (4) le déterminisme et un an.
#[test]
fn a_single_m2_tide_follows_the_ideal_cosine_s577() {
    let maree = Maree::new(&[Composante { periode_h: M2, amplitude_m: 1.0, phase_tours: 0.0 }], 0.0).unwrap();
    let mut pire = 0f64;
    for i in 0..(15 * 24 * 60) {
        let t = i as f64 * 60.0;
        let ideal = (2.0 * std::f64::consts::PI * t / (M2 * 3600.0)).cos();
        pire = pire.max((maree.niveau(SimTime((t * 1e6) as u64)) as f64 - ideal).abs());
    }
    println!("S577 M2 : l'écart au cosinus idéal sur 15 jours, au pire {pire:.2e} m");
    assert!(pire < 1e-3, "critère 1");
    let t = SimTime(123_456_789_012);
    assert_eq!(maree.niveau(t).to_bits(), maree.niveau(t).to_bits(), "critère 4 : au bit");
    let an = maree.niveau(SimTime(365 * 86_400 * 1_000_000));
    assert!(an.is_finite() && an.abs() <= 1.0 + 1e-6, "critère 4 : un an");
}

/// (2) Vives-eaux et mortes-eaux sur 30 jours ; (3) un gué annoncé ; (5) les refus.
#[test]
fn spring_and_neap_tides_and_a_ford_s577() {
    let maree = m2_s2();
    let (mut haut, mut bas) = (f32::MIN, f32::MAX);
    for i in 0..(30 * 24 * 60) {
        let eta = maree.niveau(SimTime(i as u64 * 60_000_000));
        haut = haut.max(eta);
        bas = bas.min(eta);
    }
    println!("S577 M2 + S2 sur 30 jours : de {bas:.4} à {haut:.4} m (−1,4600 et 1,4600)");
    assert!((haut as f64 - 1.46).abs() < 1e-3 && (bas as f64 + 1.4599806361512861).abs() < 1e-3, "critère 2");
    let gue = |t: f64| maree.niveau(SimTime((t * 1e6) as u64)) as f64 + 0.5;
    let f = prochain_franchissement(&gue, 0.0, 86_400.0, 60.0, CrossCause::Maree).unwrap().unwrap();
    println!("S577 gué : {f:?} (6 974,05 s)");
    assert!((f.delai_s - 6974.05).abs() < 1.0 && f.seuil_m == 1.30 && f.trend == -1, "critère 3");
    let trop = [Composante { periode_h: M2, amplitude_m: 0.1, phase_tours: 0.0 }; 9];
    assert_eq!(Maree::new(&trop, 0.0), Err(Refus), "critère 5 : neuf composantes");
    assert_eq!(Maree::new(&[Composante { periode_h: 0.0, amplitude_m: 0.1, phase_tours: 0.0 }], 0.0), Err(Refus), "critère 5 : période");
}

// --- S578 — la carte cotidale. Références écrites au plan par son script.

/// Une onde M2 progressive dans un chenal de 20 m, une carte de 10 km de pas sur 100 km.
fn chenal_s578() -> Vec<[f32; 2]> {
    let k = 2.0 * std::f64::consts::PI / (14.007141035914502 * M2 * 3600.0);
    let mut h = Vec::new();
    for _j in 0..2 {
        for i in 0..11 {
            let x = i as f64 * 10_000.0;
            h.push([(k * x).cos() as f32, -(k * x).sin() as f32]);
        }
    }
    h
}

/// (1) Aux nœuds, l'onde exacte ; (2) au milieu d'une maille, le creux de la corde ; (3) le retard de la pleine mer ; (4) refus, au bit.
#[test]
fn a_cotidal_map_carries_a_progressive_tide_s578() {
    let h = chenal_s578();
    let carte = CarteCotidale::new(&[M2], [0.0, 0.0], 10_000.0, 11, 2, &h, 0.0).unwrap();
    let k = 2.0 * std::f64::consts::PI / (14.007141035914502 * M2 * 3600.0);
    let omega = 2.0 * std::f64::consts::PI / (M2 * 3600.0);
    let (mut pire_noeud, mut max_milieu) = (0f64, 0f64);
    let (mut pic0, mut pic50) = ((f32::MIN, 0.0), (f32::MIN, 0.0));
    for m in 0..(25 * 60) {
        let t = m as f64 * 60.0;
        let st = SimTime((t * 1e6) as u64);
        for i in [0usize, 3, 7, 10] {
            let x = i as f64 * 10_000.0;
            let e = carte.niveau(x, 5_000.0, st).unwrap() as f64;
            pire_noeud = pire_noeud.max((e - (omega * t - k * x).cos()).abs());
        }
        max_milieu = max_milieu.max(carte.niveau(55_000.0, 0.0, st).unwrap() as f64);
        if t < M2 * 3600.0 {
            let (e0, e50) = (carte.niveau(0.0, 0.0, st).unwrap(), carte.niveau(50_000.0, 0.0, st).unwrap());
            if e0 > pic0.0 { pic0 = (e0, t); }
            if e50 > pic50.0 { pic50 = (e50, t); }
        }
    }
    let retard = pic50.1 - pic0.1;
    println!("S578 carte : aux nœuds au pire {pire_noeud:.2e} m ; au milieu, l'amplitude {max_milieu:.6} (1 − 1,258·10⁻³ = 0,998742) ; \
              retard de la pleine mer {retard:.0} s (3 569,6)");
    assert!(pire_noeud < 1e-4, "critère 1");
    assert!((max_milieu - (1.0 - 0.0012577358810847983)).abs() < 1e-4, "critère 2");
    assert!((retard - 3569.6078073176614).abs() <= 60.0, "critère 3");
    assert_eq!(carte.niveau(-1.0, 0.0, SimTime(0)), Err(Refus), "critère 4 : hors de la grille");
    let st = SimTime(987_654_321_000);
    assert_eq!(carte.niveau(12_345.0, 6_789.0, st).unwrap().to_bits(), carte.niveau(12_345.0, 6_789.0, st).unwrap().to_bits(), "critère 4");
}

// --- S579 — la marée dans la surface de B. Références écrites au plan par son script.

use crate::host::{AllocStats, Allocator, JobSystem, Sink};
use crate::{AllocError, HostServices, SeaState, WaterSample, WorldPos};

struct Hote;
impl Allocator for Hote {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> { Ok(0) }
    fn seal(&mut self) {}
    fn is_sealed(&self) -> bool { false }
    fn stats(&self) -> AllocStats { AllocStats::default() }
}
impl Sink for Hote { fn warn(&self, _: &str) {} fn metric(&self, _: &str, _: f64) {} }
impl JobSystem for Hote {
    fn worker_count(&self) -> u32 { 1 }
    fn parallel_reduce_ordered_f64(&self, n: usize, _: usize, r: &dyn Fn(usize, usize) -> f64, m: &dyn Fn(f64, f64) -> f64, v: f64) -> f64 {
        m(v, r(0, n))
    }
}

fn mer_s579() -> crate::Background {
    use crate::background_spectrum::{bake, Recipe};
    let r = Recipe { sea: SeaState { hs: 0.2, tp: 6.0, theta_turns: 0.0, components: 32, graine: 42 }, gravity: 9.81, gamma: 3.3,
        min_ratio: 0.5, max_ratio: 4.0, spread_turns: 30.0 / 360.0 };
    let c = bake(r).unwrap();
    let (mut a, h) = (Hote, Hote);
    crate::Background::from_spectrum(&mut HostServices { alloc: &mut a, jobs: &h, sink: &h }, &c, WorldPos::from_metres(0., 0., 0.)).unwrap()
}

/// (1) La vitesse du niveau contre `−A·ω·sin(ωt)` ; la carte au nœud ; (3) une marée nulle.
#[test]
fn the_tide_level_rate_is_the_derivative_s579() {
    let m2 = Maree::new(&[Composante { periode_h: M2, amplitude_m: 1.0, phase_tours: 0.0 }], 0.0).unwrap();
    let h = chenal_s578();
    let carte = CarteCotidale::new(&[M2], [0.0, 0.0], 10_000.0, 11, 2, &h, 0.0).unwrap();
    let omega = 96054.0 / 4_294_967_296.0 * std::f64::consts::TAU;
    let k = 2.0 * std::f64::consts::PI / (14.007141035914502 * M2 * 3600.0);
    let (mut pire, mut pire_carte) = (0f64, 0f64);
    for m in 0..(25 * 60) {
        let t = m as f64 * 60.0;
        let st = SimTime((t * 1e6) as u64);
        pire = pire.max((m2.vitesse(st) as f64 + omega * (omega * t).sin()).abs());
        let x = 30_000.0;
        let (_, v) = carte.niveau_et_vitesse(x, 0.0, st).unwrap();
        pire_carte = pire_carte.max((v as f64 + omega * (omega * t - k * x).sin()).abs());
    }
    println!("S579 vitesse : au pire {pire:.2e} m/s ; la carte au nœud {pire_carte:.2e} m/s (|dη/dt| ≤ 1,405·10⁻⁴)");
    assert!(pire < 1e-9 && pire_carte < 1e-9, "critère 1");
    let s = WaterSample { eta: 0.3, deta_dt: -0.2, u_total: [0.1, 0.2, -0.2], ..WaterSample::default() };
    let nulle = Maree::new(&[], 0.0).unwrap();
    let t = SimTime(5_000_000);
    // `WaterSample` n'est pas comparable : champ à champ, par leurs bits.
    let champs = |w: WaterSample| [w.eta, w.deta_dt, w.steepness, w.aeration, w.u_total[0], w.u_total[1], w.u_total[2], w.normal[0],
        w.normal[1], w.normal[2]].map(f32::to_bits);
    assert_eq!(champs(avec_maree(s, nulle.niveau(t), nulle.vitesse(t))), champs(s), "critère 3");
}

/// (2) Sur une mer de B réelle, la marée s'ajoute ; (4) la composition B + W l'accepte.
#[test]
fn the_tide_enters_the_sample_of_b_and_its_composition_s579() {
    let mer = mer_s579();
    let m2 = Maree::new(&[Composante { periode_h: M2, amplitude_m: 1.2, phase_tours: 0.1 }], 0.05).unwrap();
    let mut slots = [None; 1];
    let journal = crate::wave_journal::Journal::new(0, &mut slots);
    let (mut pire_eta, mut pire_w, mut pire_c) = (0f32, 0f32, 0f32);
    for m in 0..200 {
        let t = SimTime(m * 3_600_000_000 / 8 + 123_456);
        let b = mer.eval(WorldPos::from_metres(3.0, 1.0, 0.0), t).unwrap();
        let (tau, tau_dot) = (m2.niveau(t), m2.vitesse(t));
        let s = avec_maree(b, tau, tau_dot);
        let ulp = |x: f32| f32::from_bits(x.abs().to_bits() + 1) - x.abs();
        pire_eta = pire_eta.max(((s.eta - b.eta) - tau).abs() / ulp(s.eta));
        pire_w = pire_w.max(((s.u_total[2] - b.u_total[2]) - tau_dot).abs() / ulp(s.u_total[2]));
        let c = crate::composition::compose(s, &journal, core::iter::empty::<&crate::radial_impact::RadialImpact<64>>(), crate::FrameId(0), 0,
            [3.0, 1.0], t, 1.0).unwrap();
        pire_c = pire_c.max((c.eta - (b.eta + tau)).abs() / ulp(c.eta));
    }
    println!("S579 B + marée : η au pire {pire_eta} ulp, w {pire_w} ulp ; composée {pire_c} ulp");
    assert!(pire_eta <= 2.0 && pire_w <= 2.0, "critère 2");
    assert!(pire_c <= 2.0, "critère 4");
}

// --- S580 — le courant de marée. Références écrites au plan par son script.

/// (1) Au milieu d'une maille, l'amplitude du courant et `v` nul ; (2) le courant en phase avec le niveau ; (3) `avec_courant` ; (4) refus.
#[test]
fn the_tidal_current_follows_the_progressive_wave_s580() {
    let h = chenal_s578();
    let carte = CarteCotidale::new(&[M2], [0.0, 0.0], 10_000.0, 11, 2, &h, 0.0).unwrap();
    let (mut u_max, mut v_max) = (0f32, 0f32);
    let (mut pic_u, mut pic_eta) = ((f32::MIN, 0.0), (f32::MIN, 0.0));
    for m in 0..(25 * 60) {
        let t = m as f64 * 60.0;
        let st = SimTime((t * 1e6) as u64);
        let c = carte.courant(55_000.0, 5_000.0, st, 9.81).unwrap();
        u_max = u_max.max(c[0]);
        v_max = v_max.max(c[1].abs());
        if t < M2 * 3600.0 {
            let e = carte.niveau(55_000.0, 5_000.0, st).unwrap();
            if c[0] > pic_u.0 { pic_u = (c[0], t); }
            if e > pic_eta.0 { pic_eta = (e, t); }
        }
    }
    println!("S580 courant au milieu d'une maille : u max {u_max:.6} m/s (0,700062), |v| max {v_max:.2e} ; pics du courant et du niveau à {} et {} s",
        pic_u.1, pic_eta.1);
    assert!((u_max as f64 - 0.70006225769345).abs() < 1e-4, "critère 1 : l'amplitude");
    assert!(v_max < 1e-6, "critère 1 : v nul");
    assert!((pic_u.1 - pic_eta.1).abs() <= 60.0, "critère 2 : en phase");
    let s = WaterSample { eta: 0.3, deta_dt: -0.2, u_total: [0.1, 0.2, -0.2], steepness: 0.05, ..WaterSample::default() };
    let r = avec_courant(s, [0.5, -0.25]);
    assert_eq!((r.u_total[0], r.u_total[1]), (0.1 + 0.5, 0.2 - 0.25), "critère 3");
    assert_eq!([r.eta, r.deta_dt, r.u_total[2], r.steepness].map(f32::to_bits), [s.eta, s.deta_dt, s.u_total[2], s.steepness].map(f32::to_bits),
        "critère 3 : le reste au bit");
    assert_eq!(carte.courant(-1.0, 0.0, SimTime(0), 9.81), Err(Refus), "critère 4");
}

/// **S634** — (1) contre S580 ; (2) contre une intégration RK4 ; (3) l'atténuation ; (4) l'ellipse ; (5) refus.
#[test]
fn tidal_current_with_friction_and_coriolis_matches_its_integration_s634() {
    let m2 = 12.4206012f64;
    let k = 2.0 * core::f64::consts::PI / 400_000.0;
    let mut h = Vec::new();
    for _j in 0..3 {
        for i in 0..3 {
            let x = 10_000.0 * i as f64;
            h.push([(k * x).cos() as f32, -(k * x).sin() as f32]);
        }
    }
    let carte = CarteCotidale::new(&[m2], [0.0, 0.0], 10_000.0, 3, 3, &h, 0.0).unwrap();
    let (x, y, g) = (13_000.0, 7_000.0, 9.81f32);
    let st = |t: f64| SimTime((t * 1e6).round() as u64);
    let periode = m2 * 3600.0;

    let mut pire0 = 0.0f32;
    for n in 0..200 {
        let t = st(n as f64 * periode / 200.0);
        let (a, b) = (carte.courant(x, y, t, g).unwrap(), carte.courant_amorti(x, y, t, g, 0.0, 0.0).unwrap());
        pire0 = pire0.max((a[0] - b[0]).abs()).max((a[1] - b[1]).abs());
    }
    println!("S634 : f = r = 0, écart à S580 {pire0:e} m/s");
    assert!(pire0 < 1e-6, "critère 1");

    let (f, r) = (1e-4f64, 1e-4f64);
    let u0 = |t: f64| { let c = carte.courant(x, y, st(t), g).unwrap(); [c[0] as f64, c[1] as f64] };
    let deriv = |t: f64, w: [f64; 2]| { let b = u0(t); let (u, v) = (b[0] + w[0], b[1] + w[1]); [-r * u + f * v, -r * v - f * u] };
    let (dt, npas) = (60.0, 14_400usize);
    let mut w = { let b = u0(0.0); [-b[0], -b[1]] };
    let mut pire = 0.0f64;
    for n in 0..npas {
        let t = n as f64 * dt;
        let k1 = deriv(t, w);
        let k2 = deriv(t + dt / 2.0, [w[0] + dt / 2.0 * k1[0], w[1] + dt / 2.0 * k1[1]]);
        let k3 = deriv(t + dt / 2.0, [w[0] + dt / 2.0 * k2[0], w[1] + dt / 2.0 * k2[1]]);
        let k4 = deriv(t + dt, [w[0] + dt * k3[0], w[1] + dt * k3[1]]);
        for c in 0..2 {
            w[c] += dt / 6.0 * (k1[c] + 2.0 * k2[c] + 2.0 * k3[c] + k4[c]);
        }
        let t1 = t + dt;
        if t1 >= npas as f64 * dt - periode {
            let a = carte.courant_amorti(x, y, st(t1), g, f as f32, r as f32).unwrap();
            let b = u0(t1);
            pire = pire.max((b[0] + w[0] - a[0] as f64).abs()).max((b[1] + w[1] - a[1] as f64).abs());
        }
    }
    println!("S634 : contre l'intégration RK4, écart max sur la dernière période {pire:e} m/s");
    assert!(pire < 1e-6, "critère 2");

    let maxi = |ff: f32, rr: f32, axe: usize| (0..1000).map(|n| {
        let t = st(5.0 * 86_400.0 + n as f64 * periode / 1000.0);
        carte.courant_amorti(x, y, t, g, ff, rr).unwrap()[axe].abs()
    }).fold(0.0f32, f32::max) as f64;
    let att = maxi(0.0, 1e-4, 0) / maxi(0.0, 0.0, 0);
    let ell = maxi(1e-4, 0.0, 1) / maxi(1e-4, 0.0, 0);
    println!("S634 : atténuation {att} (attendue 0,814749) ; ellipse {ell} (attendue 0,711648)");
    assert!((att - 0.8147486702618919).abs() < 1e-4, "critère 3");
    assert!((ell - 0.7116480277751258).abs() < 1e-4, "critère 4");

    assert_eq!(carte.courant_amorti(x, y, st(0.0), g, 0.0, -1e-4), Err(Refus), "critère 5 : r");
    assert_eq!(carte.courant_amorti(x, y, st(0.0), 0.0, 0.0, 0.0), Err(Refus), "critère 5 : g");
}
