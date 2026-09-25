//! S364 — la bathymétrie entre dans B : les tables cuites contre la référence de S362 (critères écrits avant le code,
//! EN-COURS S364).
use super::*;
use crate::background::Component;
use crate::bathymetrie::{profondeur_de_deferlement, transformer};
use crate::host::{AllocStats, Allocator, JobSystem, Sink};
use crate::phase::freq_hz_to_q32;

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

const G: f64 = 9.81;
/// La plage des critères : 1/50, de 80 m (la houle de 10 s n'y sent presque pas le fond) à 2 m (au large du
/// déferlement de la houle d'un mètre, 1,69 m à 30° — S362).
const LONGUEUR: f64 = 3900.0;
fn plage(y: f64) -> f64 { 80.0 - y / 50.0 }

/// Une composante : période `periode`, amplitude `a`, direction à `theta0` de la normale `+y` (vers la côte).
fn composante(periode: f64, a: f32, theta0: f64, phase0: u32) -> Component {
    let hz = 1.0 / periode;
    let omega = core::f64::consts::TAU * hz;
    let k = omega * omega / G;
    Component {
        amplitude: a,
        k_turns_per_m: (k / core::f64::consts::TAU) as f32,
        dir: [theta0.sin() as f32, theta0.cos() as f32],
        freq_q32: freq_hz_to_q32(hz),
        phase0: PhaseQ32(phase0),
    }
}

fn fond(composantes: &[Component]) -> Background {
    let mut alloc = Hote;
    let services = Hote;
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    Background::from_components(&mut host, composantes, WorldPos::from_metres(0.0, 0.0, 0.0), G as f32).unwrap()
}

fn cuire(b: &Background, pas: f64) -> Cote {
    let mut alloc = Hote;
    let services = Hote;
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    Cote::cuire(&mut host, b, [0.0, 1.0], 0.0, LONGUEUR, pas, &plage).unwrap()
}

/// La référence de la correction de phase, rad, aux sondes `ys` croissantes : `∫₀^y (k_y − k_y0) dy'` par Simpson à
/// seize sous-intervalles par mètre — indépendante du pas des tables.
fn correction_reference(omega: f64, theta0: f64, ys: &[f64]) -> Vec<f64> {
    let ky0 = omega * omega / G * theta0.cos();
    let f = |y: f64| transformer(omega, theta0, 1.0, plage(y), G).unwrap().ky - ky0;
    let mut sortie = Vec::with_capacity(ys.len());
    let (mut y, mut s) = (0.0f64, 0.0f64);
    for &cible in ys {
        while y < cible {
            let fin = (y + 1.0).min(cible);
            let d = (fin - y) / 16.0;
            let mut somme = 0.0;
            for i in 0..=16 {
                let poids = if i == 0 || i == 16 { 1.0 } else if i % 2 == 1 { 4.0 } else { 2.0 };
                somme += poids * f(y + i as f64 * d);
            }
            s += somme * d / 3.0;
            y = fin;
        }
        sortie.push(s);
    }
    sortie
}

/// **Critère 1** — les tables interpolées contre la référence, houle d'un mètre (a₀ = 0,5 m), 10 s, 30°, plage 1/50,
/// jusqu'à 2 m de fond : erreur de hauteur `a₀·|ΔK| + a·|Δφ|` ≤ **3 mm** (tolérance d'image, S201) au pas de 2 m, facteur
/// d'amplitude à 1 %. Sondes au quart et au milieu de chaque pas, là où l'interpolation se trompe le plus.
/// **Prédiction** : `Δ²/8·dk_y/dy` — 0,4 mm à 2 m, 2 mm à 5 m.
#[test]
fn tables_contre_la_reference_s364() {
    let (periode, a0, theta0) = (10.0, 0.5f32, 30f64.to_radians());
    let b = fond(&[composante(periode, a0, theta0, 0)]);
    let omega = core::f64::consts::TAU / periode;
    // La composante est à −30° de la normale dans le repère de la côte (`t = (−1, 0)`) : même transformation.
    let theta_cote = -theta0;
    let mut au_pas_de_2 = f64::NAN;
    for pas in [1.0, 2.0, 5.0, 10.0] {
        let cote = cuire(&b, pas);
        let n = cote.echantillons().0;
        let ys: Vec<f64> = (0..n - 1).flat_map(|j| [(j as f64 + 0.25) * pas, (j as f64 + 0.5) * pas]).collect();
        let reference = correction_reference(omega, theta_cote, &ys);
        let (mut pire_h, mut pire_phase, mut pire_k) = (0f64, 0f64, 0f64);
        for (y, s_ref) in ys.iter().zip(&reference) {
            let (correction, facteur, _, _) = cote.interpoler(0, *y as f32).unwrap();
            let e = transformer(omega, theta_cote, 1.0, plage(*y), G).unwrap();
            let tours = correction.0 as f64 / 4_294_967_296.0;
            let ref_tours = (s_ref / core::f64::consts::TAU).rem_euclid(1.0);
            let mut dphi = (tours - ref_tours).abs();
            dphi = dphi.min(1.0 - dphi) * core::f64::consts::TAU;
            let dk = (facteur as f64 - e.amplitude).abs();
            let h = a0 as f64 * dk + a0 as f64 * e.amplitude * dphi;
            pire_h = pire_h.max(h);
            pire_phase = pire_phase.max(dphi);
            pire_k = pire_k.max(dk / e.amplitude);
        }
        let bord = cote.interpoler(0, 0.0).unwrap().1 - 1.0;
        println!("S364 tables pas={pas} echantillons={n} octets={} pire_hauteur_mm={:.4} pire_phase_rad={:.2e} \
            pire_facteur_rel={:.2e} facteur_au_large_moins_un={bord:.2e}", cote.octets(), pire_h * 1e3, pire_phase,
            pire_k);
        if pas == 2.0 {
            au_pas_de_2 = pire_h;
            assert!(pire_k <= 0.01, "{pire_k}");
        }
    }
    // **Le bord du large.** La plage commence à λ₀/2 = 78 m — le « fond qui cesse de se sentir » des manuels et de
    // SPEC-005 §8 — et le facteur y vaut déjà 0,991 : une marche de 4,5 mm sur cette houle, B non transformé au large.
    // Prédiction, écrite avant la mesure : un bord à λ₀ (156 m) ramène la marche sous 10⁻⁴ (`K_s − 1 ≈ −4·10⁻⁵`).
    let lambda0 = G * periode * periode / core::f64::consts::TAU;
    for h0 in [0.5 * lambda0, lambda0] {
        let e = transformer(omega, theta_cote, 1.0, h0, G).unwrap();
        println!("S364 bord h0={h0:.1} facteur_moins_un={:.2e} marche_mm={:.3}", e.amplitude - 1.0,
            (e.amplitude - 1.0).abs() * a0 as f64 * 1e3);
        if h0 == lambda0 {
            assert!((e.amplitude - 1.0).abs() <= 1e-4, "{}", e.amplitude);
        }
    }
    let h_b = profondeur_de_deferlement(omega, theta_cote, 1.0, G, 80.0, 0.5).unwrap();
    println!("S364 tables deferlement h_b={h_b:.3} (la plage s'arrête à 2 m)");
    assert!(h_b < 2.0);
    assert!(au_pas_de_2 <= 3e-3, "{au_pas_de_2}");
}

/// La plage de P3 : 1/30, de 160 m (λ₀ de la composante de 10 s, la plus longue) à 2 m ; la table commence à
/// `y_local = −2 400 m` pour que toute la plage tienne dans le domaine local de B (I-08, `|x| < 4 096 m`).
const ORIGINE3: f64 = -2400.0;
const LONGUEUR3: f64 = 4740.0;
fn plage3(y: f64) -> f64 { 160.0 - y / 30.0 }

/// La correction de phase de référence de chaque composante, rad, aux ordonnées locales `ys` croissantes :
/// `∫ (k_y − k_y0)` depuis le bord de la table, seize sous-intervalles par mètre, par composante `[c][i]`.
fn corrections3(b: &Background, ys: &[f64]) -> Vec<Vec<f64>> {
    b.components().iter().map(|c| {
        let omega = c.freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU;
        let (cos0, sin0) = (c.dir[1] as f64, -(c.dir[0] as f64));
        let theta0 = sin0.atan2(cos0);
        let ky0 = c.k_turns_per_m as f64 * core::f64::consts::TAU * cos0;
        let f = |s: f64| transformer(omega, theta0, 1.0, plage3(s), G).unwrap().ky - ky0;
        let (mut s, mut ds) = (0f64, 0f64);
        ys.iter().map(|y| {
            let cible = y - ORIGINE3;
            while s < cible {
                let fin = (s + 1.0).min(cible);
                let d = (fin - s) / 16.0;
                let mut somme = 0.0;
                for i in 0..=16 {
                    let poids = if i == 0 || i == 16 { 1.0 } else if i % 2 == 1 { 4.0 } else { 2.0 };
                    somme += poids * f(s + i as f64 * d);
                }
                ds += somme * d / 3.0;
                s = fin;
            }
            ds
        }).collect()
    }).collect()
}

/// La référence, en f64, de la somme transformée au point local `(x, y)`, `ds[c]` la correction de phase de chaque
/// composante en `y` : phase de B (même phase temporelle entière, phase spatiale en f64) plus la correction ; amplitude
/// `a·K` ; pente et vitesse orbitale au premier ordre WKB. Rend `(η, ∂η/∂x, ∂η/∂y, u_x, u_y)`.
fn reference3(b: &Background, x: f64, y: f64, t: SimTime, ds: &[f64]) -> [f64; 5] {
    let yp = y - ORIGINE3;
    let mut r = [0f64; 5];
    for (c, ds) in b.components().iter().zip(ds) {
        let omega = c.freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU;
        let dir = [c.dir[0] as f64, c.dir[1] as f64];
        let theta0 = (-dir[0]).atan2(dir[1]);
        let temps = c.phase0.0.wrapping_sub(PhaseQ32::from_time(c.freq_q32, t).0) as f64 / 4_294_967_296.0;
        let phi = core::f64::consts::TAU * (c.k_turns_per_m as f64 * (x * dir[0] + y * dir[1]) + temps) + ds;
        let e = transformer(omega, theta0, 1.0, plage3(yp), G).unwrap();
        let a = c.amplitude as f64 * e.amplitude;
        // `k_x` le long de `t = (−1, 0)`, `k_y` le long de `n = (0, 1)`.
        let kv = [-e.kx, e.ky];
        let coth = 1.0 / (e.k * plage3(yp)).tanh();
        r[0] += a * phi.sin();
        r[1] += a * phi.cos() * kv[0];
        r[2] += a * phi.cos() * kv[1];
        r[3] += a * omega * coth * phi.sin() * kv[0] / e.k;
        r[4] += a * omega * coth * phi.sin() * kv[1] / e.k;
    }
    r
}

/// **Critères 2 et 3, et B sur la côte contre la référence.** Huit composantes de 6 à 10 s, de −30° à +30° autour de la
/// normale, Hs ≈ 1 m ; table au pas de 2 m. (2) Au large de la table, l'évaluation côtière est **celle de B, au bit**.
/// (3) Deux passes sur une grille donnent le même hash. Sur la plage, jusqu'à 2 m de fond : η à **3 mm**, la pente et
/// la vitesse horizontale à 1 % de leur amplitude, contre la référence en f64.
#[test]
fn b_sur_la_cote_s364() {
    let composantes: Vec<Component> = (0..8)
        .map(|i| {
            let f = i as f64 / 7.0;
            composante(6.0 + 4.0 * f, 0.177, (-30.0 + 60.0 * f).to_radians(), 0x9E37_79B9u32.wrapping_mul(i + 1))
        })
        .collect();
    let b = fond(&composantes);
    let mut alloc = Hote;
    let services = Hote;
    let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
    let cote = Cote::cuire(&mut host, &b, [0.0, 1.0], ORIGINE3, LONGUEUR3, 2.0, &plage3).unwrap();
    let temps = [SimTime::from_micros(0), SimTime::from_micros(37_250_000), SimTime::from_micros(3_600_123_456)];

    // (2) Au large : au bit.
    let mut identiques = 0;
    for i in 0..40 {
        for t in temps {
            let p = WorldPos::from_metres(-300.0 + 17.0 * i as f64, -2400.5 - 30.0 * i as f64, 0.0);
            let (c, f) = (cote.eval(&b, p, t).unwrap(), b.eval(p, t).unwrap());
            let bits = |s: &WaterSample| [s.eta, s.u_total[0], s.u_total[1], s.u_total[2], s.normal[0], s.normal[1],
                s.normal[2], s.deta_dt, s.steepness].map(f32::to_bits);
            assert_eq!(bits(&c), bits(&f), "au large, point {i}");
            identiques += 1;
        }
    }

    // (3) Deux passes, même hash.
    let passe = || {
        let mut h = crate::hash::Hasher64::new();
        for iy in 0..60 {
            for ix in 0..20 {
                let p = WorldPos::from_metres(-200.0 + 21.0 * ix as f64, -2390.0 + 78.0 * iy as f64, 0.0);
                if let Some(s) = cote.eval(&b, p, temps[1]) {
                    h.write_f32(s.eta);
                    h.write_f32(s.u_total[0]);
                    h.write_f32(s.normal[1]);
                }
            }
        }
        h.finish()
    };
    let (h1, h2) = (passe(), passe());
    assert_eq!(h1, h2);

    // Contre la référence, sur la plage.
    let (mut pire_eta, mut pire_pente, mut pire_u) = (0f64, 0f64, 0f64);
    let (mut max_pente, mut max_u) = (0f64, 0f64);
    let mut points = 0;
    let ys: Vec<f64> = (0..237).map(|iy| ORIGINE3 + 1.0 + 20.0 * iy as f64 + 0.37).collect();
    let ds = corrections3(&b, &ys);
    for (iy, &y) in ys.iter().enumerate() {
        let dsy: Vec<f64> = ds.iter().map(|d| d[iy]).collect();
        for (ix, t) in temps.iter().enumerate() {
            let x = -150.0 + 113.0 * ix as f64;
            let s = cote.eval(&b, WorldPos::from_metres(x, y, 0.0), *t).unwrap();
            let r = reference3(&b, x, y, *t, &dsy);
            let pente = [-s.normal[0] / s.normal[2], -s.normal[1] / s.normal[2]];
            pire_eta = pire_eta.max((s.eta as f64 - r[0]).abs());
            pire_pente = pire_pente.max((pente[0] as f64 - r[1]).abs().max((pente[1] as f64 - r[2]).abs()));
            pire_u = pire_u.max((s.u_total[0] as f64 - r[3]).abs().max((s.u_total[1] as f64 - r[4]).abs()));
            max_pente = max_pente.max(r[1].abs().max(r[2].abs()));
            max_u = max_u.max(r[3].abs().max(r[4].abs()));
            points += 1;
        }
    }
    let y_bord = ORIGINE3 as f32;
    let saut = (cote.eval(&b, WorldPos::from_metres(0.0, y_bord as f64 + 0.01, 0.0), temps[1]).unwrap().eta
        - b.eval(WorldPos::from_metres(0.0, y_bord as f64 + 0.01, 0.0), temps[1]).unwrap().eta).abs();
    println!("S364 cote au_large_identiques={identiques} hash={h1:016x} points={points} pire_eta_mm={:.3} \
        pire_pente={pire_pente:.2e} (max {max_pente:.3}) pire_u={pire_u:.2e} m/s (max {max_u:.3}) \
        ecart_au_bord_mm={:.4} octets={}", pire_eta * 1e3, saut as f64 * 1e3, cote.octets());
    assert!(pire_eta <= 3e-3, "{pire_eta}");
    assert!(pire_pente <= 0.01 * max_pente && pire_u <= 0.01 * max_u, "{pire_pente} {pire_u}");
}
