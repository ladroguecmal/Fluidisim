//! Réceptions du corps rigide (S331) : proxy exact, C10 — tirant, période, masse ajoutée —, roulis d'un
//! pavé stable, déterminisme.

use super::*;

const MER: Milieu = Milieu::MER;

/// Le cube de C10 : 0,5 m, 500 kg/m³, proxy de 8 points par axe, centre à l'altitude `z`.
fn cube(z: f64) -> RigidBody {
    RigidBody::cuboid([0.5; 3], 500., [0., 0., z], [8, 8, 8])
}

/// **Critère 1.** Le volume immergé du cube droit est `A·d` exact, pour tout tirant.
#[test]
fn the_proxy_immerses_the_exact_volume_of_an_upright_box_s331() {
    let calme = CalmWater { level: 0.3 };
    for z in [-1.0, 0.05, 0.12, 0.2987, 0.43, 0.55, 0.9] {
        let c = cube(z);
        let d = (0.3 - (z - 0.25)).clamp(0., 0.5);
        let v = c.forces(&calme, MER).immersed_volume;
        assert!((v - 0.25 * d).abs() <= 1e-12, "z {z} : {v} contre {}", 0.25 * d);
    }
}

/// Une trajectoire de pilonnement : le cube lâché la base à la surface, `pas` pas de `dt`, l'altitude
/// du centre à chaque pas.
fn pilonnement(added: f64, pas: usize, dt: f64) -> Vec<f64> {
    let mut c = cube(0.25);
    c.added_mass = [added; 3];
    let calme = CalmWater { level: 0. };
    (0..pas).map(|_| {
        c.step(dt, &calme, MER);
        c.position[2]
    }).collect()
}

/// Les instants des maxima locaux d'une série, interpolés parabolique­ment.
fn maxima(z: &[f64], dt: f64) -> Vec<f64> {
    (1..z.len() - 1)
        .filter(|&i| z[i] > z[i - 1] && z[i] >= z[i + 1])
        .map(|i| {
            let (a, b, c) = (z[i - 1], z[i], z[i + 1]);
            let d = a - 2. * b + c;
            let off = if d != 0. { 0.5 * (a - c) / d } else { 0. };
            (i as f64 + 1. + off) * dt
        })
        .collect()
}

/// **Critères 2 et 3 (C10).** Tirant moyen sur des périodes entières à ± 1 % de `(ρ_c/ρ)·H`, période à
/// ± 5 % de `2π√(ρ_c·H/(ρ·g))` — sans masse ajoutée.
#[test]
fn the_c10_cube_floats_at_its_draft_and_heaves_at_its_period_s331() {
    let dt = 1e-3;
    let z = pilonnement(0., 3500, dt);
    let t = maxima(&z, dt);
    assert!(t.len() >= 3, "{} maxima", t.len());
    let periode = (t[t.len() - 1] - t[0]) / (t.len() - 1) as f64;
    let (debut, fin) = ((t[0] / dt) as usize, (t[t.len() - 1] / dt) as usize);
    let moyenne = z[debut..fin].iter().sum::<f64>() / (fin - debut) as f64;
    let tirant = 0. - (moyenne - 0.25);
    let (d_ref, t_ref) = (500. / 1025. * 0.5, 2. * core::f64::consts::PI * (500. * 0.5 / (1025. * G)).sqrt());
    println!("S331 : tirant {tirant:.6} m (réf. {d_ref:.6}), période {periode:.6} s (réf. {t_ref:.6})");
    assert!((tirant / d_ref - 1.).abs() <= 0.01, "tirant {tirant}");
    assert!((periode / t_ref - 1.).abs() <= 0.05, "période {periode}");
}

/// **Critère 4 (C10).** Masse ajoutée du disque équivalent : le rapport des périodes à 1,414 ± 15 %, et à
/// 1 % de `√(1 + m_a/m)`.
#[test]
fn the_added_mass_lengthens_the_c10_period_by_root_two_s331() {
    let dt = 1e-3;
    let rayon = (0.25f64 / core::f64::consts::PI).sqrt();
    let m_a = 8. / 3. * 1025. * rayon.powi(3);
    let periode = |added: f64| {
        let z = pilonnement(added, 5000, dt);
        let t = maxima(&z, dt);
        (t[t.len() - 1] - t[0]) / (t.len() - 1) as f64
    };
    let (sans, avec) = (periode(0.), periode(m_a));
    let rapport = avec / sans;
    let modele = (1. + m_a / 62.5).sqrt();
    println!("S331 : m_a {m_a:.3} kg, rapport {rapport:.5} (modèle {modele:.5}, C10 : 1,414 ± 15 %)");
    assert!((rapport / 2f64.sqrt() - 1.).abs() <= 0.15, "{rapport}");
    assert!((rapport / modele - 1.).abs() <= 0.01, "{rapport} contre {modele}");
}

/// **Critère 6.** Deux trajectoires identiques au bit — six degrés de liberté, traînée comprise.
#[test]
fn two_trajectories_are_identical_to_the_bit_s331() {
    let run = || {
        let mut c = RigidBody::cuboid([1., 1., 0.3], 500., [0.1, -0.2, 0.2], [6, 6, 4]);
        c.drag = 1.;
        c.angular_velocity = [0.3, -0.1, 0.05];
        let calme = CalmWater { level: 0. };
        for _ in 0..2000 {
            c.step(1e-3, &calme, MER);
        }
        (c.position, c.orientation, c.velocity, c.angular_velocity)
    };
    let (a, b) = (run(), run());
    assert_eq!(format!("{:?}", a.0.map(f64::to_bits)), format!("{:?}", b.0.map(f64::to_bits)));
    assert_eq!(format!("{:?}", a.1.map(f64::to_bits)), format!("{:?}", b.1.map(f64::to_bits)));
    assert_eq!(format!("{:?}", a.2.map(f64::to_bits)), format!("{:?}", b.2.map(f64::to_bits)));
    assert_eq!(format!("{:?}", a.3.map(f64::to_bits)), format!("{:?}", b.3.map(f64::to_bits)));
}

/// **Critère 5.** Un pavé plat, 1 × 1 × 0,3 m à 500 kg/m³, stable — `GM = d/2 + b²/(12d) − c/2` ≈ 0,49 m —,
/// lâché à son tirant et incliné de 5° autour de `x` : période de roulis à ± 5 % de `2π√(I/(m·g·GM))`.
#[test]
fn a_flat_box_rolls_at_its_metacentric_period_s331() {
    let (b, c, rho_c) = (1.0f64, 0.3f64, 500.);
    let d = c * rho_c / 1025.;
    let mut p = RigidBody::cuboid([1., b, c], rho_c, [0., 0., c / 2. - d], [16, 16, 8]);
    let demi = 2.5f64.to_radians();
    p.orientation = [demi.cos(), demi.sin(), 0., 0.];
    let calme = CalmWater { level: 0. };
    let dt = 1e-3;
    let angles: Vec<f64> = (0..5000).map(|_| {
        p.step(dt, &calme, MER);
        2. * p.orientation[1].atan2(p.orientation[0])
    }).collect();
    let t = maxima(&angles, dt);
    assert!(t.len() >= 3, "{} maxima", t.len());
    let periode = (t[t.len() - 1] - t[0]) / (t.len() - 1) as f64;
    let gm = d / 2. + b * b / (12. * d) - c / 2.;
    let reference = 2. * core::f64::consts::PI * (p.inertia[0] / (p.mass * G * gm)).sqrt();
    let amplitude = angles.iter().fold(0f64, |m, a| m.max(a.abs())).to_degrees();
    println!("S331 : roulis {periode:.5} s (réf. {reference:.5}, GM {gm:.4} m), amplitude {amplitude:.3}°");
    assert!((periode / reference - 1.).abs() <= 0.05, "{periode} contre {reference}");
}

// ——— S333 : la houle de B derrière la requête du corps ———

use crate::background::{Background, SeaState};
use crate::host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};
use crate::types::{SimTime, WorldPos};

struct Arena;
impl Allocator for Arena {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> {
        Ok(0)
    }
    fn seal(&mut self) {}
    fn is_sealed(&self) -> bool {
        false
    }
    fn stats(&self) -> AllocStats {
        AllocStats::default()
    }
}
struct Jobs;
impl Sink for Jobs {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Jobs {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(&self, n: usize, _: usize, r: &dyn Fn(usize, usize) -> f64, m: &dyn Fn(f64, f64) -> f64, init: f64) -> f64 {
        m(init, r(0, n))
    }
}

/// Une houle monochromatique de B, amplitude `a`, période `periode`, vers +x : une seule composante, dont
/// `configure` pose l'amplitude `Hs/(2√2)` et la période `Tp/2`.
fn houle(a: f64, periode: f64) -> Background {
    let sea = SeaState { hs: (2. * 2f64.sqrt() * a) as f32, tp: (2. * periode) as f32, theta_turns: 0., components: 1, graine: 333 };
    Background::configure(&mut HostServices { alloc: &mut Arena, jobs: &Jobs, sink: &Jobs }, sea, WorldPos::default()).unwrap()
}

/// **Le pilonnement forcé** de la coque de la porte D — 4 × 1,6 × 1 m à 500 kg/m³, proxy 16 × 8 × 4 — sur
/// la houle `b`, lâchée **sur le régime forcé linéaire** `ζ = Z·η(0, t)/a`, `Z = a·S/(1 − ω²/ωₙ²)`, où `S` est
/// la moyenne de `cos(k·x)` sur les points du proxy — la houle vue par la flottaison, 1 pour une houle
/// infiniment longue. Pas de 2 ms sur `periodes` périodes. Rend `(écart ponctuel au régime forcé / Z,
/// demi-excursion mesurée / Z, Z / a, 1/(1 − ω²/ωₙ²))`.
fn pilonnement_force(b: &Background, periodes: usize) -> (f64, f64, f64, f64) {
    let c = b.components()[0];
    let (a, k) = (c.amplitude as f64, c.k_turns_per_m as f64 * core::f64::consts::TAU);
    let omega = c.freq_q32 as f64 / 4_294_967_296. * core::f64::consts::TAU;
    let mut coque = RigidBody::cuboid([4., 1.6, 1.], 500., [0.; 3], [16, 8, 4]);
    let omega_n2 = MER.rho * G * 6.4 / coque.mass;
    let s = coque.proxy.iter().map(|p| (k * p.body[0]).cos()).sum::<f64>() / coque.proxy.len() as f64;
    let amplification = 1. / (1. - omega * omega / omega_n2);
    let rapport = s * amplification;
    let z_eq = 0.5 - 500. / 1025.;
    let centre = |t: u64| b.eval_local([0.; 3], SimTime(t)).unwrap();
    coque.position[2] = z_eq + rapport * centre(0).eta as f64;
    coque.velocity[2] = rapport * centre(0).deta_dt as f64;
    let pas = (periodes as f64 * core::f64::consts::TAU / omega / 0.002).round() as u64;
    let (mut ecart, mut haut, mut bas) = (0f64, f64::MIN, f64::MAX);
    for n in 0..pas {
        coque.step(0.002, &BackgroundWater { background: b, time: SimTime(n * 2000) }, MER);
        let zeta = coque.position[2] - z_eq;
        ecart = ecart.max((zeta - rapport * centre((n + 1) * 2000).eta as f64).abs());
        (haut, bas) = (haut.max(zeta), bas.min(zeta));
    }
    let z = rapport * a;
    (ecart / z, 0.5 * (haut - bas) / z, rapport, amplification)
}

/// **S333, critère 1 — B derrière la requête du corps.** Sur une houle longue de B — 6 s, λ = 56 m, 25 cm —,
/// la coque pilonne à l'amplitude `a/(1 − ω²/ωₙ²)` à ± 5 %, et suit le régime forcé pas à pas. Sur une houle
/// de 3 s, que la coque n'égale plus (λ = 14 m), l'amplification 1,28 n'est tenue qu'avec la houle vue par
/// la flottaison, `S` = 0,87 : c'est elle que le proxy intègre, à ± 1 % sur les deux houles.
#[test]
fn a_hull_heaves_on_a_swell_of_b_at_its_forced_response_s333() {
    for (a, periode, critere) in [(0.25, 6., true), (0.05, 3., false)] {
        let b = houle(a, periode);
        let (ecart, excursion, rapport, amplification) = pilonnement_force(&b, 4);
        let mesure = excursion * rapport;
        println!("S333 : houle {periode} s, pilonnement {mesure:.5}·a (forcé {rapport:.5}·a, 1/(1 − ω²/ωₙ²) = {amplification:.5}), écart au régime forcé {ecart:.2e}·Z");
        assert!((excursion - 1.).abs() <= 0.01, "{periode} s : {excursion}");
        assert!(ecart <= 0.01, "{periode} s : {ecart}");
        if critere {
            assert!((mesure / amplification - 1.).abs() <= 0.05, "critère 1 : {mesure} contre {amplification}");
        }
    }
}
