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

/// Moindres carrés sur trois fonctions : équations normales accumulées `m`, `r`, résolues par Cramer.
fn moindres_carres(m: [[f64; 3]; 3], r: [f64; 3]) -> [f64; 3] {
    let det = |m: [[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let colonne = |j: usize| {
        let mut n = m;
        for i in 0..3 {
            n[i][j] = r[i];
        }
        det(n) / det(m)
    };
    [colonne(0), colonne(1), colonne(2)]
}

/// Accumule une ligne `v`, de valeur `y`, dans les équations normales.
fn accumule(m: &mut [[f64; 3]; 3], r: &mut [f64; 3], v: [f64; 3], y: f64) {
    for i in 0..3 {
        for j in 0..3 {
            m[i][j] += v[i] * v[j];
        }
        r[i] += v[i] * y;
    }
}

/// **Le pilonnement forcé** de la coque de la porte D — 4 × 1,6 × 1 m à 500 kg/m³, proxy 16 × 8 × 4 — sur
/// la houle `b`, lâchée **sur le régime forcé linéaire** `ζ = Z·η/a`, `Z = a·S/(1 − ω²/ωₙ²)`, où `S` est la
/// moyenne de `cos(k·x)` sur les points du proxy — la houle vue par la flottaison, 1 pour une houle
/// infiniment longue —, et à la vitesse horizontale de l'eau qui la porte. Pas de 2 ms sur `periodes`
/// périodes. Le pilonnement se décompose par moindres carrés sur la houle **sous la coque**, là où l'eau
/// l'a portée : `ζ = α·η + β·(∂η/∂t)/ω + γ`. Le cavalement déphase le départ et excite un pilonnement libre,
/// non amorti, que la projection écarte. Rend `(√(α² + β²), résidu quadratique / Z, S/(1 − ω²/ωₙ²),
/// 1/(1 − ω²/ωₙ²))`, les deux premiers en unités de `a`.
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
    let sous = |x: f64, t: u64| b.eval_local([x as f32, 0., 0.], SimTime(t)).unwrap();
    coque.position[2] = z_eq + rapport * sous(0., 0).eta as f64;
    coque.velocity = [s * sous(0., 0).u_total[0] as f64, 0., rapport * sous(0., 0).deta_dt as f64];
    let pas = (periodes as f64 * core::f64::consts::TAU / omega / 0.002).round() as u64;
    let (mut m, mut r, mut serie) = ([[0f64; 3]; 3], [0f64; 3], Vec::with_capacity(pas as usize));
    for n in 0..pas {
        coque.step(0.002, &BackgroundWater { background: b, time: SimTime(n * 2000) }, MER);
        let e = sous(coque.position[0], (n + 1) * 2000);
        let v = [e.eta as f64, e.deta_dt as f64 / omega, 1.];
        accumule(&mut m, &mut r, v, coque.position[2] - z_eq);
        serie.push((v, coque.position[2] - z_eq));
    }
    let [alpha, beta, gamma] = moindres_carres(m, r);
    let residu = (serie.iter().map(|(v, y)| (y - alpha * v[0] - beta * v[1] - gamma).powi(2)).sum::<f64>() / pas as f64).sqrt();
    ((alpha * alpha + beta * beta).sqrt(), residu / (rapport * a), rapport, amplification)
}

/// **S333, critère 1 — B derrière la requête du corps.** Sur une houle longue de B — 6 s, λ = 56 m, 25 cm —,
/// la coque pilonne à l'amplitude `a/(1 − ω²/ωₙ²)` à ± 5 %. Sur une houle de 3 s, que la coque n'égale
/// plus (λ = 14 m), l'amplification 1,28 n'est tenue qu'avec la houle vue par la flottaison, `S` = 0,87 :
/// c'est elle que le proxy intègre, à ± 1 % sur les deux houles. Le pilonnement libre que le départ excite,
/// que rien n'amortit — le proxy n'a pas de rayonnement —, reste sous 10 % de `Z` : 0,35 % à 6 s, 5,2 % à 3 s.
#[test]
fn a_hull_heaves_on_a_swell_of_b_at_its_forced_response_s333() {
    for (a, periode, critere) in [(0.25, 6., true), (0.05, 3., false)] {
        let b = houle(a, periode);
        let (mesure, residu, rapport, amplification) = pilonnement_force(&b, 4);
        println!("S333 : houle {periode} s, pilonnement forcé {mesure:.5}·a (prédit {rapport:.5}·a, 1/(1 − ω²/ωₙ²) = {amplification:.5}), résidu libre {residu:.2e}·Z");
        assert!((mesure / rapport - 1.).abs() <= 0.01, "{periode} s : {mesure} contre {rapport}");
        assert!(residu <= 0.1, "{periode} s : {residu}");
        if critere {
            assert!((mesure / amplification - 1.).abs() <= 0.05, "critère 1 : {mesure} contre {amplification}");
        }
    }
}

/// **S333, critère 1 bis — la houle entraîne la coque.** Sur la houle longue du critère 1, la coque libre,
/// lâchée à la vitesse horizontale de l'eau qu'elle suit, `S·u(0)`, cavale avec l'eau. Sa trajectoire se
/// décompose par moindres carrés en `x = x₀ + U·t + c·ξ(t)`, `ξ` l'excursion de la particule de surface au
/// centre : **`c` à ± 5 % de 1**, et à 1 % de `S`, la houle vue par la flottaison. `U` est une **dérive** du
/// second ordre, de l'ordre de la dérive de Stokes `a²ωk` ; elle est publiée, pas jugée. Sans la part
/// horizontale de la pression, `c` serait nul.
#[test]
fn the_swell_carries_the_hull_along_s333() {
    let b = houle(0.25, 6.);
    let c = b.components()[0];
    let (a, k) = (c.amplitude as f64, c.k_turns_per_m as f64 * core::f64::consts::TAU);
    let omega = c.freq_q32 as f64 / 4_294_967_296. * core::f64::consts::TAU;
    let mut coque = RigidBody::cuboid([4., 1.6, 1.], 500., [0.; 3], [16, 8, 4]);
    let s = coque.proxy.iter().map(|p| (k * p.body[0]).cos()).sum::<f64>() / coque.proxy.len() as f64;
    let rapport = s / (1. - omega * omega / (MER.rho * G * 6.4 / coque.mass));
    let z_eq = 0.5 - 500. / 1025.;
    // L'excursion horizontale de la particule de surface au centre, `a·cos φ = −(∂η/∂t)/ω` pour une houle
    // monochromatique vers +x.
    let centre = |t: u64| b.eval_local([0.; 3], SimTime(t)).unwrap();
    let xi = |t: u64| -(centre(t).deta_dt as f64) / omega;
    coque.position[2] = z_eq + rapport * centre(0).eta as f64;
    coque.velocity = [s * centre(0).u_total[0] as f64, 0., rapport * centre(0).deta_dt as f64];
    let pas = (4. * core::f64::consts::TAU / omega / 0.002).round() as u64;
    let (mut m, mut r) = ([[0f64; 3]; 3], [0f64; 3]);
    for n in 0..pas {
        coque.step(0.002, &BackgroundWater { background: &b, time: SimTime(n * 2000) }, MER);
        accumule(&mut m, &mut r, [1., (n + 1) as f64 * 0.002, xi((n + 1) * 2000)], coque.position[0]);
    }
    let [_, derive, cavalement] = moindres_carres(m, r);
    println!("S333 : cavalement {cavalement:.5}·ξ (S = {s:.5}), dérive {derive:.2e} m/s (Stokes a²ωk = {:.2e})", a * a * omega * k);
    assert!((cavalement - 1.).abs() <= 0.05, "critère 1 bis : {cavalement}");
    assert!((cavalement / s - 1.).abs() <= 0.01, "{cavalement} contre S = {s}");
}

/// **S336, critère 2 — la coque amortie.** La coque de la porte D, avec les constantes que δ lui mesure — masse
/// ajoutée 3 200 kg, amortissement 6 400 N·s/m —, lâchée 10 cm au-dessus de son tirant en eau calme : elle
/// pilonne à `ω'·√(1 − ζ²)`, `ω'² = K/(m + A)`, et ses crêtes décroissent du décrément `2πζ/√(1 − ζ²)`,
/// `ζ = B/(2√(K(m + A)))`, les deux à ± 2 %. Sans amortissement, ses crêtes ne décroissent pas.
#[test]
fn the_damped_hull_rings_down_at_its_radiation_rate_s336() {
    let (masse_ajoutee, amortissement) = (3200., 6400.);
    let z_eq = 0.5 - 500. / 1025.;
    let lance = |b: f64| -> Vec<f64> {
        let mut c = RigidBody::cuboid([4., 1.6, 1.], 500., [0., 0., z_eq + 0.1], [16, 8, 4]);
        c.added_mass = [0., 0., masse_ajoutee];
        c.radiation_damping = [0., 0., b];
        let calme = CalmWater { level: 0. };
        (0..5000).map(|_| {
            c.step(0.002, &calme, MER);
            c.position[2] - z_eq
        }).collect()
    };
    let z = lance(amortissement);
    let t = maxima(&z, 0.002);
    assert!(t.len() >= 3, "{} maxima", t.len());
    let periode = t[1] - t[0];
    let pics: Vec<f64> = t.iter().map(|x| z[(x / 0.002).round() as usize - 1]).collect();
    let decrement = (pics[0] / pics[1]).ln();
    let (m, k) = (3200., 1025. * G * 6.4);
    let omega = (k / (m + masse_ajoutee)).sqrt();
    let zeta = amortissement / (2. * (k * (m + masse_ajoutee)).sqrt());
    let (periode_ref, decrement_ref) = (2. * core::f64::consts::PI / (omega * (1. - zeta * zeta).sqrt()),
        2. * core::f64::consts::PI * zeta / (1. - zeta * zeta).sqrt());
    let libre = lance(0.);
    let t0 = maxima(&libre, 0.002);
    let (p0, p1) = (libre[(t0[0] / 0.002).round() as usize - 1], libre[(t0[1] / 0.002).round() as usize - 1]);
    println!("S336 : période {periode:.4} s (réf. {periode_ref:.4}), décrément {decrement:.4} (réf. {decrement_ref:.4}, ζ = {zeta:.4}) ; sans amortissement, crêtes {p0:.4} puis {p1:.4} m");
    assert!((periode / periode_ref - 1.).abs() <= 0.02, "{periode} contre {periode_ref}");
    assert!((decrement / decrement_ref - 1.).abs() <= 0.02, "{decrement} contre {decrement_ref}");
    assert!((p1 / p0 - 1.).abs() <= 1e-3, "{p0} {p1}");
}

/// **S336, critère 2 bis — la masse ajoutée voit l'eau accélérée.** La coque de la porte D avec A = 3 200 kg, sur
/// les houles de 6 et 3 s : son pilonnement forcé vaut `(K·S − A·ω²)/(K − (m + A)·ω²)` — la poussée de la houle
/// vue par la flottaison, moins l'inertie de l'eau qu'elle entraîne, qui l'accompagne —, à ± 1 %, par moindres
/// carrés sur la houle sous la coque. Une masse ajoutée sur l'accélération absolue donnerait `K·S/(K − (m + A)·ω²)`.
#[test]
fn the_added_mass_follows_the_accelerating_water_s336() {
    let masse_ajoutee = 3200.;
    for (a_houle, periode) in [(0.25, 6.), (0.05, 3.)] {
        let b = houle(a_houle, periode);
        let c = b.components()[0];
        let (a, k) = (c.amplitude as f64, c.k_turns_per_m as f64 * core::f64::consts::TAU);
        let omega = c.freq_q32 as f64 / 4_294_967_296. * core::f64::consts::TAU;
        let mut coque = RigidBody::cuboid([4., 1.6, 1.], 500., [0.; 3], [16, 8, 4]);
        coque.added_mass = [0., 0., masse_ajoutee];
        let (m, raideur) = (coque.mass, MER.rho * G * 6.4);
        let s = coque.proxy.iter().map(|p| (k * p.body[0]).cos()).sum::<f64>() / coque.proxy.len() as f64;
        let relatif = (raideur * s - masse_ajoutee * omega * omega) / (raideur - (m + masse_ajoutee) * omega * omega);
        let absolu = raideur * s / (raideur - (m + masse_ajoutee) * omega * omega);
        let z_eq = 0.5 - 500. / 1025.;
        let sous = |x: f64, t: u64| b.eval_local([x as f32, 0., 0.], SimTime(t)).unwrap();
        coque.position[2] = z_eq + relatif * sous(0., 0).eta as f64;
        coque.velocity = [s * sous(0., 0).u_total[0] as f64, 0., relatif * sous(0., 0).deta_dt as f64];
        let pas = (4. * core::f64::consts::TAU / omega / 0.002).round() as u64;
        let (mut mm, mut r) = ([[0f64; 3]; 3], [0f64; 3]);
        for n in 0..pas {
            coque.step(0.002, &BackgroundWater { background: &b, time: SimTime(n * 2000) }, MER);
            let e = sous(coque.position[0], (n + 1) * 2000);
            accumule(&mut mm, &mut r, [e.eta as f64, e.deta_dt as f64 / omega, 1.], coque.position[2] - z_eq);
        }
        let [alpha, beta, _] = moindres_carres(mm, r);
        let mesure = (alpha * alpha + beta * beta).sqrt();
        println!("S336 : houle {periode} s, masse ajoutée {masse_ajoutee} kg — pilonnement {mesure:.5}·a (relatif {relatif:.5}, absolu {absolu:.5}) ; a = {a:.3} m");
        assert!((mesure / relatif - 1.).abs() <= 0.01, "{periode} s : {mesure} contre {relatif}");
    }
}


/// **S494 — W derrière la requête du corps.** Le montage : la houle de B (`a`, 6 s) et, si `energie > 0`, un impact
/// confirmé de W à l'origine — `energie` J, λ = 4 m, né à 0 —, domaine de 32 m sur 12 s, 256 modes ; la bouée — 0,5 × 0,5 × 0,4 m,
/// 500 kg/m³, proxy 4 × 4 × 4 — lâchée à son tirant en `(x0, 0)`, à la vitesse horizontale de B (l'impact naît au repos), `pas` pas de 2 ms sous `MixedWater` (`mixte`) ou
/// sous `BackgroundWater`. `visite` reçoit l'instant (µs), la bouée et la requête à cet instant : avant le premier pas, puis
/// après chacun. Rend le nombre de refus de la composition.
fn bouee_sous_w(
    a: f64,
    energie: f32,
    cote: f64,
    x0: f64,
    pas: u64,
    mixte: bool,
    visite: &mut dyn FnMut(u64, &RigidBody, &dyn WaterQuery),
) -> u64 {
    use crate::prepared_water::{BoundBackground, Context, Prepared};
    use crate::radial_impact::{Domain, RadialImpact};
    use crate::wave_event::{Impact, Origin, WaveEvent};
    use crate::wave_journal::{Cause, Journal};
    use crate::FrameId;
    let b = houle(a, 6.);
    let bound = BoundBackground::new(&b, FrameId(0), 0);
    let mut records = [None; 1];
    let mut journal = Journal::new(0, &mut records);
    if energie > 0. {
        let event = WaveEvent::impact(Impact {
            id: 1,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(0),
            ttl_us: 12_000_000,
            position: [0.; 3],
            energy_j: energie,
            wavelength_m: 4.,
            direction_turns: 0.,
            anisotropy: 0.,
            displaced_l: 0.,
            material: 0,
            origin: Origin::Server,
            above_surface: true,
        })
        .unwrap();
        journal.confirm(0, Cause { entity: 1, command: 1, emission: 0 }, event).unwrap();
    }
    let ctx = Context {
        frame: FrameId(0),
        cell: 0,
        medium: crate::impact_field::Medium { gravity: b.gravity(), density: 1025., depth: 50., max_slope: 0.5 },
        domain: Domain { radius: 32., age_us: 12_000_000 },
    };
    let mut pool: [Option<RadialImpact<256>>; 1] = [const { None }; 1];
    let impacts = Prepared::<256>::build(&journal, &mut pool, ctx).unwrap();
    let haut = 0.8 * cote;
    let z_eq = haut * (0.5 - 500. / MER.rho);
    let mut bouee = RigidBody::cuboid([cote, cote, haut], 500., [x0, 0., z_eq], [4, 4, 4]);
    let mut refus = 0;
    let mixte_a = |t: u64| MixedWater {
        bound: &bound,
        impacts: &impacts,
        pressure: None,
        time: SimTime(t),
        max_slope: 0.5,
        refusals: core::cell::Cell::new(0),
    };
    // Lâchée à la vitesse horizontale de l'eau qui la porte (S333) : au repos, elle prendrait du retard sur la houle de B.
    let u0 = BackgroundWater { background: &b, time: SimTime(0) }.velocity([x0, 0., 0.]);
    bouee.velocity = [u0[0], u0[1], 0.];
    for n in 0..=pas {
        let t = n * 2000;
        if mixte {
            let eau = mixte_a(t);
            visite(t, &bouee, &eau);
            if n < pas {
                bouee.step(0.002, &eau, MER);
            }
            refus += eau.refusals.get();
        } else {
            let eau = BackgroundWater { background: &b, time: SimTime(t) };
            visite(t, &bouee, &eau);
            if n < pas {
                bouee.step(0.002, &eau, MER);
            }
        }
    }
    refus
}

/// **S494, critère 1 — sans impact, la requête mixte est celle de B.** Sur 20 s de houle de B (5 cm, 6 s), la bouée sous
/// `MixedWater` (journal vide) suit sa trajectoire sous `BackgroundWater` à 10⁻⁶ m : seule la normale est renormalisée.
#[test]
fn without_impact_the_mixed_query_is_the_background_s494() {
    let mut seule = Vec::new();
    bouee_sous_w(0.05, 0., 0.5, 5., 10_000, false, &mut |_, c, _| seule.push(c.position));
    let mut mixte = Vec::new();
    let refus = bouee_sous_w(0.05, 0., 0.5, 5., 10_000, true, &mut |_, c, _| mixte.push(c.position));
    let ecart = seule.iter().zip(&mixte).flat_map(|(p, q)| (0..3).map(move |k| (p[k] - q[k]).abs())).fold(0f64, f64::max);
    println!("S494 critère 1 : écart {ecart:.2e} m, refus {refus}");
    assert_eq!(refus, 0);
    assert!(ecart <= 1e-6, "{ecart}");
}

/// Le relevé d'un instant : `(t, x, y, z, η̄, u_x, u_y)` — `η̄` la surface moyenne sous les seize colonnes du proxy à la position de
/// la bouée de côté `cote`, `u` la vitesse horizontale de l'eau en son centre.
fn releve_s494(t: u64, c: &RigidBody, eau: &dyn WaterQuery, cote: f64) -> [f64; 7] {
    let mut eta = 0.;
    for j in 0..4 {
        for i in 0..4 {
            eta += eau.surface(c.position[0] + ((i as f64 + 0.5) / 4. - 0.5) * cote, c.position[1] + ((j as f64 + 0.5) / 4. - 0.5) * cote);
        }
    }
    let u = eau.velocity([c.position[0], c.position[1], 0.]);
    [t as f64 * 1e-6, c.position[0], c.position[1], c.position[2], eta / 16., u[0], u[1]]
}

/// La mesure de S494 : la bouée de côté `cote` sous B (`a`) et un impact de `energie` J à 5 m, 10 s. Rend ce que rend
/// [`analyse_s494`].
fn mesure_s494(a: f64, energie: f32, cote: f64) -> [f64; 7] {
    let serie_de = |energie: f32| {
        let mut serie = Vec::new();
        let refus = bouee_sous_w(a, energie, cote, 5., 5000, true, &mut |t, c, eau| serie.push(releve_s494(t, c, eau, cote)));
        (serie, refus)
    };
    let (serie, refus) = serie_de(energie);
    let (sans, _) = serie_de(0.);
    analyse_s494(&serie, &sans, 0.8 * cote, refus)
}

/// L'analyse d'une série de relevés au pas de 2 ms contre la même scène sans la part de W étudiée (`sans`) ; `haut` la hauteur de
/// la bouée. Rend `[max|η̄_W|, max|z_W|, écart du pilonnement à l'oscillateur, max|∫u dt|, écart horizontal,
/// refus, max|η̄ − η̄(0)|]` — `_W` : la différence avec `sans` ; l'oscillateur `z'' = (ρgA/m)(η̄ − z + h/2) − g`, forcé par `η̄`
/// (interpolée linéairement entre les pas, ω·dt ≈ 0,008), intégré en RK4 à 0,1 ms depuis le même état ; `∫u dt` l'excursion de
/// la particule de surface au centre de la bouée, l'écart horizontal la norme de la différence des deux déplacements.
fn analyse_s494(serie: &[[f64; 7]], sans: &[[f64; 7]], haut: f64, refus: u64) -> [f64; 7] {
    let dt = 0.002;
    let k_raideur = MER.rho * G / (haut * 500.);
    let eta_a = |t: f64| -> f64 {
        let s = t / dt;
        let i = (s.floor() as usize).min(serie.len() - 2);
        let f = s - i as f64;
        serie[i][4] * (1. - f) + serie[i + 1][4] * f
    };
    let acc = |t: f64, z: f64| k_raideur * (eta_a(t) - z + 0.5 * haut) - G;
    let (mut z, mut v, sous) = (serie[0][3], 0., 20usize);
    let h = dt / sous as f64;
    let (mut eta_w, mut z_w, mut pire, mut ampl_x, mut pire_x, mut ampl) = (0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
    let mut excursion = [0f64; 2];
    for n in 1..serie.len() {
        for q in 0..sous {
            let t = (n - 1) as f64 * dt + q as f64 * h;
            let (k1z, k1v) = (v, acc(t, z));
            let (k2z, k2v) = (v + 0.5 * h * k1v, acc(t + 0.5 * h, z + 0.5 * h * k1z));
            let (k3z, k3v) = (v + 0.5 * h * k2v, acc(t + 0.5 * h, z + 0.5 * h * k2z));
            let (k4z, k4v) = (v + h * k3v, acc(t + h, z + h * k3z));
            z += h / 6. * (k1z + 2. * k2z + 2. * k3z + k4z);
            v += h / 6. * (k1v + 2. * k2v + 2. * k3v + k4v);
        }
        let s = serie[n];
        for k in 0..2 {
            excursion[k] += 0.5 * (serie[n - 1][5 + k] + s[5 + k]) * dt;
        }
        eta_w = eta_w.max((s[4] - sans[n][4]).abs());
        z_w = z_w.max((s[3] - sans[n][3]).abs());
        pire = pire.max((s[3] - z).abs());
        ampl = ampl.max((s[4] - serie[0][4]).abs());
        ampl_x = ampl_x.max(excursion[0].hypot(excursion[1]));
        pire_x = pire_x.max((s[1] - serie[0][1] - excursion[0]).hypot(s[2] - serie[0][2] - excursion[1]));
    }
    [eta_w, z_w, pire, ampl_x, pire_x, refus as f64, ampl]
}

/// **S494, critères 2 à 4 — un impact de W fait pilonner et cavaler la bouée.** La bouée de 0,5 m à 5 m d'un impact d'1 kJ
/// (λ = 4 m), sur une houle de B de 2 cm. (2) Le pilonnement suit l'oscillateur forcé par la surface sous l'empreinte à 3 % de
/// max|η̄| ; l'impact la fait bouger d'au moins 30 % de max|η̄_W|. (3) **Manqué d'abord** : l'écart horizontal y vaut
/// l'excursion — la bouée prend une **dérive du second ordre** vers l'extérieur (le corps suit `x'' = −g·∂η/∂x`, la particule
/// `x'' = −g·∂η/∂x + u·∂u/∂x`), que l'ordre de grandeur n'avait pas prévue. Écrit après le manqué : l'écart croît comme
/// l'énergie (`a²`) de 10 J à 1 kJ — ×≥ 30 pour ×100, le premier ordre donnerait ×10 ; au régime linéaire (0,1 J, B à 10 µm), ce
/// qui reste est **l'empreinte** — l'anneau vu à travers 0,5 m — : il décroît comme le carré du côté (÷ ≥ 3 de 0,5 à 0,25 m) et
/// tient 2 % de l'excursion à 0,25 m. (4) Aucun refus.
#[test]
fn an_impact_of_w_heaves_and_carries_the_buoy_s494() {
    let m = mesure_s494(0.02, 1000., 0.5);
    let ampl = m[6];
    println!(
        "S494 1 kJ sur 2 cm : max|η̄| {ampl:.4} m, max|η̄_W| {:.4} m, pilonnement dû à W {:.4} m, écart à l'oscillateur {:.2e} m ({:.2} % de max|η̄|), excursion {:.4} m, écart horizontal {:.2e} m ({:.1} %), refus {}",
        m[0], m[1], m[2], 100. * m[2] / ampl, m[3], m[4], 100. * m[4] / m[3], m[5]
    );
    assert_eq!(m[5], 0., "critère 4");
    assert!(m[2] <= 0.03 * ampl, "critère 2 : {} contre {ampl}", m[2]);
    assert!(m[1] >= 0.3 * m[0], "critère 2 : {} contre {}", m[1], m[0]);
    let (dix, mille) = (mesure_s494(1e-5, 10., 0.5), mesure_s494(1e-5, 1000., 0.5));
    let echelle = mille[4] / dix[4];
    let (l50, l25) = (mesure_s494(1e-5, 0.1, 0.5), mesure_s494(1e-5, 0.1, 0.25));
    println!(
        "S494 : écart horizontal ×{echelle:.1} de 10 J à 1 kJ (second ordre ≈ 100, premier 10) ; à 0,1 J, {:.2} % de l'excursion à 0,5 m, {:.2} % à 0,25 m (÷ {:.2}) ; pilonnement {:.2} % et {:.2} %",
        100. * l50[4] / l50[3], 100. * l25[4] / l25[3], (l50[4] / l50[3]) / (l25[4] / l25[3]),
        100. * l50[2] / l50[6], 100. * l25[2] / l25[6]
    );
    assert!(echelle >= 30., "critère 3, second ordre : {echelle}");
    assert!((l50[4] / l50[3]) / (l25[4] / l25[3]) >= 3., "critère 3, l'empreinte");
    assert!(l25[4] <= 0.02 * l25[3], "critère 3 : {} contre {}", l25[4], l25[3]);
    for r in [dix, mille, l50, l25] {
        assert_eq!(r[5], 0., "critère 4");
        assert!(r[2] <= 0.03 * r[6], "critère 2 : {} contre {}", r[2], r[6]);
    }
}

/// **S495 — le sillage d'un objet en marche derrière la requête du corps.** Une houle de B presque nulle (10 µm, 6 s) ; une source
/// de pression gaussienne (σ = 1 m, recette 64 × 128, coupure 6), `p0` Pa, en marche à 3 m/s de (−15, 0) à (15, 0) sur 10 s, publiée
/// par son contrôleur à chaque pas ; la bouée plate de côté `cote` (hauteur 0,4 × le côté) lâchée à son tirant en `depart`, à la vitesse de l'eau, `pas` pas de 2 ms sous `MixedWater`.
/// `visite` reçoit l'instant (µs), la bouée et la requête à cet instant, avant le premier pas puis après chacun. Rend les refus.
fn bouee_dans_sillage(
    p0: f32,
    cote: f64,
    depart: [f64; 2],
    pas: u64,
    visite: &mut dyn FnMut(u64, &RigidBody, &dyn WaterQuery),
) -> u64 {
    use crate::gaussian_spectrum::{bake, Recipe};
    use crate::modal_pressure::Segment;
    use crate::prepared_water::{BoundBackground, Context, Prepared};
    use crate::radial_impact::{Domain, RadialImpact};
    use crate::spectral_pressure::{Node, Slot};
    use crate::wave_journal::{Cause, Journal};
    use crate::{bound_pressure, pressure_journal, pressure_source, FrameId};
    let b = houle(1e-5, 6.);
    let bound = BoundBackground::new(&b, FrameId(0), 0);
    let mut records = [None; 1];
    let journal = Journal::new(0, &mut records);
    let ctx = Context {
        frame: FrameId(0),
        cell: 0,
        medium: crate::impact_field::Medium { gravity: b.gravity(), density: 1025., depth: 50., max_slope: 0.5 },
        domain: Domain { radius: 32., age_us: 12_000_000 },
    };
    let mut pool: [Option<RadialImpact<64>>; 1] = [const { None }; 1];
    let impacts = Prepared::<64>::build(&journal, &mut pool, ctx).unwrap();
    // Le nombre de nœuds du sillage de production (S212, 64 × 128) et une coupure à 6σ⁻¹ : la gaussienne reconstruite sans pression
    // parasite loin de la source (16 × 24 ne la reconstruit qu'à `r ≲ 24/k_max` ≈ 4 m ; une pression qui pousse l'eau et pas le corps).
    let recipe = Recipe { sigma: 1.0, cutoff: 6.0, radial: 64, angular: 128 };
    let mut nodes = vec![Node::default(); 8192];
    let mut hn = vec![Node::default(); 4096];
    let full = bake(recipe, &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let settings = bound_pressure::Settings {
        frame: FrameId(0),
        cell: 0,
        gravity: b.gravity(),
        density: 1025.,
        min: [-16., -8.],
        max: [16., 8.],
        start: SimTime(0),
        end: SimTime(14_000_000),
    };
    let pc = bound_pressure::Context::new(settings, &half).unwrap();
    let route = [Segment { birth: SimTime(0), duration_us: 10_000_000, origin: [-15., 0.], velocity: [3., 0.], pressure_pa: p0 }];
    let mut ps = [None; 1];
    let mut pj = pressure_journal::Journal::new(0, &mut ps);
    let metadata = pressure_source::Metadata { epoch: 0, id: 0, cause: Cause { entity: 2, command: 0, emission: 0 }, settings, recipe };
    pj.admit_authenticated(pressure_source::Source::new(metadata, &route).unwrap()).unwrap();
    let mut pp = vec![Slot::default(); 4096];
    let mut spare = pp.clone();
    let mut controleur = bound_pressure::Controller::new(pc, &half, &mut pj, SimTime(0), &mut pp, &mut spare).unwrap();
    // Plate — hauteur 0,4 × le côté : sa stabilité de forme est nette (à 0,8, avec quatre points par axe, elle roule et chavire).
    let haut = 0.4 * cote;
    let mut bouee = RigidBody::cuboid([cote, cote, haut], 500., [depart[0], depart[1], haut * (0.5 - 500. / MER.rho)], [4, 4, 4]);
    let mut refus = 0;
    for n in 0..=pas {
        let t = SimTime(n * 2000);
        controleur.update(t).unwrap();
        let pression = controleur.current(t).unwrap();
        let eau = MixedWater { bound: &bound, impacts: &impacts, pressure: Some(&pression), time: t, max_slope: 0.5, refusals: core::cell::Cell::new(0) };
        if n == 0 {
            // Lâchée à la vitesse horizontale de l'eau qui la porte : la source naît en marche, l'eau a déjà une vitesse.
            let u = eau.velocity([depart[0], depart[1], 0.]);
            bouee.velocity = [u[0], u[1], 0.];
        }
        visite(t.0, &bouee, &eau);
        if n < pas {
            bouee.step(0.002, &eau, MER);
        }
        refus += eau.refusals.get();
    }
    refus
}

/// La mesure de S495 : la bouée de côté `cote` en (−6, 6), à 6σ de la route d'une source de `p0` Pa — à 3σ, la queue de la
/// gaussienne pousse l'eau plus que la pente du sillage, et pas le corps —, 12 s, le bras de Kelvin y passe vers 9 s ; la scène
/// sans sillage est la même à 0 Pa. Rend ce que rend [`analyse_s494`].
fn mesure_s495(p0: f32, cote: f64) -> [f64; 7] {
    let serie_de = |p0: f32| {
        let mut serie = Vec::new();
        let refus = bouee_dans_sillage(p0, cote, [-6., 6.], 6000, &mut |t, c, eau| serie.push(releve_s494(t, c, eau, cote)));
        (serie, refus)
    };
    let (serie, refus) = serie_de(p0);
    let (sans, _) = serie_de(0.);
    analyse_s494(&serie, &sans, 0.4 * cote, refus)
}

/// **S495, critère 1 — l'extraction au bit.** Sans pression, la composition mixte au point local (`mixed::sample_local`) rend, au
/// bit, celle des impacts seuls (`Prepared::sample_local`, S494) — la même opération dans le même ordre —, sur une grille de points
/// et d'instants autour d'un impact d'1 kJ ; les essais de `tests_mixed_water` tiennent l'extraction pour le chemin monde.
#[test]
fn the_mixed_composition_at_a_local_point_is_the_impacts_one_s495() {
    use crate::prepared_water::{mixed, BoundBackground, Context, Prepared};
    use crate::radial_impact::{Domain, RadialImpact};
    use crate::wave_event::{Impact, Origin, WaveEvent};
    use crate::wave_journal::{Cause, Journal};
    use crate::FrameId;
    let b = houle(0.02, 6.);
    let bound = BoundBackground::new(&b, FrameId(0), 0);
    let mut records = [None; 1];
    let mut journal = Journal::new(0, &mut records);
    let event = WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(0),
        ttl_us: 12_000_000,
        position: [0.; 3],
        energy_j: 1000.,
        wavelength_m: 4.,
        direction_turns: 0.,
        anisotropy: 0.,
        displaced_l: 0.,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap();
    journal.confirm(0, Cause { entity: 1, command: 1, emission: 0 }, event).unwrap();
    let ctx = Context {
        frame: FrameId(0),
        cell: 0,
        medium: crate::impact_field::Medium { gravity: b.gravity(), density: 1025., depth: 50., max_slope: 0.5 },
        domain: Domain { radius: 32., age_us: 12_000_000 },
    };
    let mut pool: [Option<RadialImpact<256>>; 1] = [const { None }; 1];
    let impacts = Prepared::<256>::build(&journal, &mut pool, ctx).unwrap();
    let mut n = 0;
    for t in (0..10_000_000u64).step_by(250_000) {
        for i in -8..=8 {
            for j in -8..=8 {
                let p = [i as f32 * 0.75, j as f32 * 0.75];
                let a = mixed::sample_local(&bound, &impacts, None, SimTime(t), p, 0.5).unwrap();
                let c = impacts.sample_local(&bound, p, SimTime(t), 0.5).unwrap();
                assert_eq!(
                    [a.eta, a.deta_dt, a.u_total[0], a.u_total[1], a.u_total[2], a.normal[0], a.normal[1], a.normal[2]].map(f32::to_bits),
                    [c.eta, c.deta_dt, c.u_total[0], c.u_total[1], c.u_total[2], c.normal[0], c.normal[1], c.normal[2]].map(f32::to_bits),
                    "{p:?} à {t} µs"
                );
                n += 1;
            }
        }
    }
    println!("S495 critère 1 : {n} points au bit");
}

/// **S495, critères 2 à 4 — le sillage d'un objet en marche fait pilonner et dériver la bouée.** Une source de pression gaussienne
/// (σ = 1 m) à 3 m/s, la bouée posée à 6 m de sa route. (2) À 200 Pa (bouée de 0,5 m), le pilonnement suit l'oscillateur forcé par
/// la surface sous l'empreinte à 3 % de max|η̄| ; le sillage la fait bouger d'au moins 30 % de max|η̄_P|. (3) À 2 Pa — le régime
/// linéaire, le second ordre cumulé `k·a·ω·t` ≈ 0,005 (leçon de S494) —, bouée de 0,25 m : le déplacement horizontal suit `∫u dt`
/// à 5 % de son maximum. (4) Aucun refus. Mesuré : 0,05 % ; 0,85 % ; à 200 Pa l'écart horizontal vaut 12,5 % — la dérive du second
/// ordre, publiée, non jugée.
#[test]
#[ignore = "≈ 8 min : 4 096 modes de pression, ≈ 150 échantillons par pas — `cargo test … s495 -- --ignored`"]
fn the_wake_of_a_moving_pressure_heaves_and_carries_the_buoy_s495() {
    let fort = mesure_s495(200., 0.5);
    let faible = mesure_s495(2., 0.25);
    for (nom, m) in [("200 Pa, 0,5 m", fort), ("2 Pa, 0,25 m", faible)] {
        println!(
            "S495 {nom} : max|η̄| {:.4} m, max|η̄_P| {:.4} m, pilonnement dû au sillage {:.4} m, écart à l'oscillateur {:.2e} m ({:.2} %), \
             excursion {:.2e} m, écart horizontal {:.2e} m ({:.2} %), refus {}",
            m[6], m[0], m[1], m[2], 100. * m[2] / m[6], m[3], m[4], 100. * m[4] / m[3], m[5]
        );
        assert_eq!(m[5], 0., "critère 4, {nom}");
        assert!(m[2] <= 0.03 * m[6], "critère 2, {nom} : {} contre {}", m[2], m[6]);
    }
    assert!(fort[1] >= 0.3 * fort[0], "critère 2 : {} contre {}", fort[1], fort[0]);
    assert!(faible[4] <= 0.05 * faible[3], "critère 3 : {} contre {}", faible[4], faible[3]);
}

/// **S497 — un corps en marche produit son sillage.** La coque de la porte D (4 × 1,6 × 1 m, 500 kg/m³) menée par le jeu sur un cercle
/// de 20 m de rayon à 3 m/s (sa vitesse horizontale imposée à chaque pas de 2 ms, l'eau calme la porte), 12 s. La source de son sillage
/// est émise tronçon par tronçon, `Δ` µs chacun, par `RigidBody::wake_leg` ; la référence est la même trajectoire **déclarée** d'avance —
/// les positions du corps toutes les 0,2 s, soixante tronçons —, sous la même charge. Rend `[écart max curseur/corps aux fins de tronçon,
/// prédiction ½·(U²/R)·Δ², max|η_émis − η_déclaré|, max|η_déclaré|, P₀ publiée, m·g/(2πσ²), refus]`, η relevée sur une grille de 15 × 15
/// points à 4, 8 et 12 s.
fn sillage_emis_s497(delta_us: u64) -> [f64; 7] {
    use crate::bound_pressure::{Context, Controller, Settings};
    use crate::gaussian_spectrum::{bake, Recipe};
    use crate::pressure_journal::Journal;
    use crate::pressure_source::Metadata;
    use crate::spectral_pressure::{Node, Slot};
    use crate::wake_source::{Emitter, Leg, Wake};
    use crate::wave_journal::Cause;
    use crate::FrameId;
    let (rayon, u, fin_us) = (20f64, 3f64, 12_000_000u64);
    let settings = Settings {
        frame: FrameId(0),
        cell: 0,
        gravity: G as f32,
        density: MER.rho as f32,
        min: [-30.; 2],
        max: [30.; 2],
        start: SimTime(0),
        end: SimTime(14_000_000),
    };
    let recipe = Recipe { sigma: 2., cutoff: 3., radial: 64, angular: 128 };
    let meta = |id: u64| Metadata { epoch: 1, id, cause: Cause { entity: 9, command: id, emission: 0 }, settings, recipe };
    let mut nodes = vec![Node::default(); 8192];
    let mut hn = vec![Node::default(); 4096];
    let full = bake(recipe, &mut nodes).unwrap();
    let half = full.half_into(&mut hn).unwrap();
    let context = Context::new(settings, &half).unwrap();
    // Le corps, mené sur le cercle ; ses positions toutes les 0,2 s pour la référence.
    let mut coque = RigidBody::cuboid([4., 1.6, 1.], 500., [rayon, 0., 0.5 - 500. / MER.rho], [8, 4, 4]);
    let calme = CalmWater { level: 0. };
    let mene = |c: &mut RigidBody, t: f64| {
        let th = u * t / rayon;
        c.velocity[0] = -u * th.sin();
        c.velocity[1] = u * th.cos();
    };
    let poids = (coque.mass * G) as f32;
    // Les émissions restent chez l'hôte tant que le journal les retient (ADR-104) : préparées et acquittées tronçon par tronçon (un
    // journal d'un emplacement par acquittement), admises ensuite au contrôleur — le champ, calculé à l'instant demandé, est le même ;
    // l'admission en direct est celle de S151.
    let mut emissions = Vec::new();
    let mut emetteur = Emitter::new(meta(100), SimTime(0), [rayon as f32, 0.]).unwrap();
    let (mut refus, mut pire_curseur) = (0f64, 0f64);
    let mut releves = vec![[coque.position[0], coque.position[1]]];
    let pas = fin_us / 2000;
    for n in 0..=pas {
        let t = n * 2000;
        mene(&mut coque, t as f64 * 1e-6);
        if t % delta_us == 0 {
            if t > 0 {
                let c = emetteur.cursor().position;
                pire_curseur = pire_curseur.max((c[0] as f64 - coque.position[0]).hypot(c[1] as f64 - coque.position[1]));
            }
            if t < fin_us {
                let leg = coque.wake_leg(emetteur.cursor(), delta_us);
                let emission = emetteur.prepare(emetteur.cursor(), leg).unwrap();
                {
                    let mut un = [None];
                    let mut j1 = Journal::new(1, &mut un);
                    if j1.admit_authenticated(emission.source()).is_err() || emetteur.acknowledge(&emission, &j1).is_err() {
                        refus += 1.;
                    }
                }
                emissions.push(emission);
            }
        }
        if n < pas {
            coque.step(0.002, &calme, MER);
            if (n + 1) % 100 == 0 {
                releves.push([coque.position[0], coque.position[1]]);
            }
        }
    }
    let mut slots = vec![None; 64];
    let mut journal = Journal::new(1, &mut slots);
    // Le contrôleur se construit sur un journal non vide.
    journal.admit_authenticated(emissions[0].source()).unwrap();
    let (mut active, mut spare) = (vec![Slot::default(); 4096], vec![Slot::default(); 4096]);
    let mut emis = Controller::new(context, &half, &mut journal, SimTime(0), &mut active, &mut spare).unwrap();
    for e in &emissions[1..] {
        if emis.admit(e.source()).is_err() {
            refus += 1.;
        }
    }
    // La référence : la trajectoire déclarée, tronçons de 0,2 s entre les positions relevées.
    let legs: Vec<Leg> = releves
        .windows(2)
        .map(|w| Leg { duration_us: 200_000, velocity: [((w[1][0] - w[0][0]) / 0.2) as f32, ((w[1][1] - w[0][1]) / 0.2) as f32], downward_force_n: poids })
        .collect();
    let declaree = Wake::build(meta(1), SimTime(0), [releves[0][0] as f32, releves[0][1] as f32], &legs).unwrap();
    let p0 = declaree.source().segments()[0].pressure_pa as f64;
    let mut slots_r = vec![None; 2];
    let mut journal_r = Journal::new(1, &mut slots_r);
    journal_r.admit_authenticated(declaree.source()).unwrap();
    let (mut active_r, mut spare_r) = (vec![Slot::default(); 4096], vec![Slot::default(); 4096]);
    let mut reference = Controller::new(context, &half, &mut journal_r, SimTime(0), &mut active_r, &mut spare_r).unwrap();
    let (mut ecart, mut ampl) = (0f64, 0f64);
    for t in [4_000_000u64, 8_000_000, 12_000_000] {
        emis.update(SimTime(t)).unwrap();
        reference.update(SimTime(t)).unwrap();
        let (e, r) = (emis.current(SimTime(t)).unwrap(), reference.current(SimTime(t)).unwrap());
        for i in 0..15 {
            for j in 0..15 {
                let p = [-28. + 4. * i as f32, -28. + 4. * j as f32];
                let (a, b) = (e.sample_local(p).unwrap().eta as f64, r.sample_local(p).unwrap().eta as f64);
                ecart = ecart.max((a - b).abs());
                ampl = ampl.max(b.abs());
            }
        }
    }
    let d = delta_us as f64 * 1e-6;
    [pire_curseur, 0.5 * u * u / rayon * d * d, ecart, ampl, p0, poids as f64 / (core::f64::consts::TAU * 4.), refus]
}

/// **S497, critères 1 à 4 — le sillage émis par le corps en marche contre la trajectoire déclarée.** (1) À chaque fin de tronçon, le
/// curseur de la source est à moins de 1,2 × `½·(U²/R)·Δ²` du corps ; (2) `P₀ = m·g/(2πσ²)` à 10⁻⁶ près ; (3) à Δ = 0,5 s, le sillage
/// émis à 10 % de max|η| du sillage déclaré (prévu 2 à 6 %), et un ordre ≥ 1,7 sur Δ = 1 / 0,5 / 0,25 s ; (4) aucun refus.
/// Mesuré : 0,0560 m (prédit 0,0563) ; P₀ au bit ; 1,20 % ; ordres 1,89 et 1,83 ; 0.
#[test]
fn a_moving_body_emits_its_wake_s497() {
    let r: Vec<[f64; 7]> = [1_000_000u64, 500_000, 250_000].iter().map(|d| sillage_emis_s497(*d)).collect();
    for (d, m) in [1., 0.5, 0.25].iter().zip(&r) {
        println!(
            "S497 Δ = {d} s : curseur/corps {:.4} m (prédit {:.4}), sillage émis/déclaré {:.2e} m sur {:.2e} ({:.2} %), P₀ {:.4} Pa (m·g/2πσ² = {:.4}), refus {}",
            m[0], m[1], m[2], m[3], 100. * m[2] / m[3], m[4], m[5], m[6]
        );
        assert_eq!(m[6], 0., "critère 4");
        assert!(m[0] <= 1.2 * m[1], "critère 1 : {} contre {}", m[0], m[1]);
        assert!((m[4] / m[5] - 1.).abs() <= 1e-6, "critère 2 : {} contre {}", m[4], m[5]);
    }
    let ordre = |a: f64, b: f64| (a / b).log2();
    let (o1, o2) = (ordre(r[0][2], r[1][2]), ordre(r[1][2], r[2][2]));
    println!("S497 : ordres {o1:.2} et {o2:.2}");
    assert!(r[1][2] <= 0.1 * r[1][3], "critère 3 : {} contre {}", r[1][2], r[1][3]);
    assert!(o1 >= 1.7 && o2 >= 1.7, "critère 3, ordre : {o1} {o2}");
}

/// Les archétypes d'ADR-008 §3 en pavés droits — aire de flottaison et masse du tableau : navire de 60 m, barque, caisse flottante,
/// balle de ping-pong (un cube de même aire et de même masse).
fn archetypes_s498() -> [(&'static str, RigidBody, f64, f64); 4] {
    let pave = |l: f64, b: f64, h: f64, m: f64, couches: [usize; 3]| RigidBody::cuboid([l, b, h], m / (l * b * h), [0.; 3], couches);
    let cote = 1.3e-3f64.sqrt();
    [
        ("navire", pave(60., 10., 8., 1.2e6, [8, 4, 4]), 600., 1.2e6),
        ("barque", pave(3., 2., 0.6, 400., [6, 4, 4]), 6., 400.),
        ("caisse", pave(1., 1., 0.2, 50., [4, 4, 4]), 1., 50.),
        ("ping-pong", pave(cote, cote, cote, 2.7e-3, [4, 4, 4]), 1.3e-3, 2.7e-3),
    ]
}

/// **S498, critère 1 — les régimes d'ADR-008 §3.** La pulsation que le corps mesure sur son proxy à l'équilibre est `√(ρgA/(m + m_a))`
/// à 10⁻⁹ près, et le régime bascule aux seuils `ω·dt` = 0,3 et 1, de part et d'autre (à 10⁻⁶ près). À 30 Hz : le navire normal, la
/// barque et la caisse sous-cyclées (2 sous-pas : leur `m_a` est nul ici, plus raides que le tableau d'ADR-008), la balle contrainte.
#[test]
fn the_floating_regimes_follow_omega_dt_s498() {
    for (nom, corps, aire, masse) in archetypes_s498() {
        let f = corps.floating(1. / 30., MER);
        let attendu = (MER.rho * G * aire / masse).sqrt();
        println!("S498 {nom} : ω {:.4} rad/s (attendu {attendu:.4}), ω·dt à 30 Hz {:.3}, régime {:?}, c {:.5} m", f.omega, f.omega / 30., f.regime, f.offset);
        assert!((f.omega / attendu - 1.).abs() <= 1e-9, "{nom} : {} contre {attendu}", f.omega);
        for (x, regime) in [(0.3 * (1. - 1e-6), Regime::Normal), (0.3 * (1. + 1e-6), Regime::Subcycled(2)), (1. - 1e-6, Regime::Subcycled(4)), (1. + 1e-6, Regime::Constrained)] {
            assert_eq!(corps.floating(x / attendu, MER).regime, regime, "{nom} à ω·dt = {x}");
        }
    }
    let r: Vec<Regime> = archetypes_s498().iter().map(|a| a.1.floating(1. / 30., MER).regime).collect();
    assert_eq!(r, [Regime::Normal, Regime::Subcycled(2), Regime::Subcycled(2), Regime::Constrained]);
}

/// L'écart d'un corps contraint à la surface de `eau` : `|z − (η + c)|`, et l'angle entre son axe et la normale (rad).
fn ecart_contraint(c: &RigidBody, f: &Floating, eau: &dyn WaterQuery) -> (f64, f64) {
    let [x, y, z] = c.position;
    let s = eau.slope(x, y);
    let n = (1. + s[0] * s[0] + s[1] * s[1]).sqrt();
    let normale = [-s[0] / n, -s[1] / n, 1. / n];
    let axe = rotate(c.orientation, [0., 0., 1.]);
    let croix = [axe[1] * normale[2] - axe[2] * normale[1], axe[2] * normale[0] - axe[0] * normale[2], axe[0] * normale[1] - axe[1] * normale[0]];
    ((z - (eau.surface(x, y) + f.offset)).abs(), norm(croix).asin())
}

/// **S498, critère 2 — le mode contraint.** La balle de ping-pong à 30 Hz : 120 s sur une houle de B (5 cm, 6 s), puis 10 s près d'un
/// impact de W d'1 kJ (λ = 4 m) — `max|z − (η + c)| = 0` et son axe sur la normale à 10⁻¹² rad ; sa vitesse horizontale suit celle de
/// l'eau. **Critère 3 — le témoin** : la même balle au pas normal diverge, `|G|` > 1 par pas (prédit ≈ 2,7).
#[test]
fn a_small_light_object_rides_the_surface_exactly_s498() {
    let cote = 1.3e-3f64.sqrt();
    let balle = |x: f64| RigidBody::cuboid([cote; 3], 2.7e-3 / cote.powi(3), [x, 0., 0.], [4, 4, 4]);
    let f = balle(0.).floating(1. / 30., MER);
    assert_eq!(f.regime, Regime::Constrained);
    // 120 s sur B.
    let b = houle(0.05, 6.);
    let mut c = balle(5.);
    c.position[2] = f.offset;
    let (mut pire_z, mut pire_angle, mut pire_u) = (0f64, 0f64, 0f64);
    for n in 0..3600u64 {
        let t = |k: u64| SimTime(k * 1_000_000 / 30);
        let (eau, suivante) = (BackgroundWater { background: &b, time: t(n) }, BackgroundWater { background: &b, time: t(n + 1) });
        c.step_floating(&f, 1. / 30., &eau, &suivante, MER);
        let (dz, angle) = ecart_contraint(&c, &f, &suivante);
        pire_z = pire_z.max(dz);
        pire_angle = pire_angle.max(angle);
        if n > 30 {
            let u = suivante.velocity([c.position[0], c.position[1], 0.]);
            pire_u = pire_u.max((c.velocity[0] - u[0]).hypot(c.velocity[1] - u[1]));
        }
    }
    println!("S498 balle sur B, 120 s : max|z − (η + c)| {pire_z:.1e} m, angle à la normale {pire_angle:.1e} rad, |v − u| {pire_u:.1e} m/s");
    assert_eq!(pire_z, 0., "critère 2");
    assert!(pire_angle <= 1e-12, "critère 2 : {pire_angle}");
    // 10 s près d'un impact de W.
    let mut sous_w = (0f64, 0f64);
    let mut deplacement = 0f64;
    {
        use crate::prepared_water::{BoundBackground, Context, Prepared};
        use crate::radial_impact::{Domain, RadialImpact};
        use crate::wave_event::{Impact, Origin, WaveEvent};
        use crate::wave_journal::{Cause, Journal};
        use crate::FrameId;
        let b = houle(0.02, 6.);
        let bound = BoundBackground::new(&b, FrameId(0), 0);
        let mut records = [None; 1];
        let mut journal = Journal::new(0, &mut records);
        let event = WaveEvent::impact(Impact {
            id: 1,
            frame: FrameId(0),
            cell: 0,
            birth: SimTime(0),
            ttl_us: 12_000_000,
            position: [0.; 3],
            energy_j: 1000.,
            wavelength_m: 4.,
            direction_turns: 0.,
            anisotropy: 0.,
            displaced_l: 0.,
            material: 0,
            origin: Origin::Server,
            above_surface: true,
        })
        .unwrap();
        journal.confirm(0, Cause { entity: 1, command: 1, emission: 0 }, event).unwrap();
        let ctx = Context {
            frame: FrameId(0),
            cell: 0,
            medium: crate::impact_field::Medium { gravity: b.gravity(), density: 1025., depth: 50., max_slope: 0.5 },
            domain: Domain { radius: 32., age_us: 12_000_000 },
        };
        let mut pool: [Option<RadialImpact<256>>; 1] = [const { None }; 1];
        let impacts = Prepared::<256>::build(&journal, &mut pool, ctx).unwrap();
        let eau_a = |k: u64| MixedWater {
            bound: &bound,
            impacts: &impacts,
            pressure: None,
            time: SimTime(k * 1_000_000 / 30),
            max_slope: 0.5,
            refusals: core::cell::Cell::new(0),
        };
        let mut c = balle(5.);
        c.position[2] = f.offset;
        for n in 0..300u64 {
            let (eau, suivante) = (eau_a(n), eau_a(n + 1));
            c.step_floating(&f, 1. / 30., &eau, &suivante, MER);
            let (dz, angle) = ecart_contraint(&c, &f, &suivante);
            sous_w = (sous_w.0.max(dz), sous_w.1.max(angle));
            assert_eq!(eau.refusals.get() + suivante.refusals.get(), 0);
        }
        deplacement = deplacement.max((c.position[0] - 5.).abs());
    }
    println!("S498 balle près d'un impact, 10 s : max|z − (η + c)| {:.1e} m, angle {:.1e} rad, déplacement radial {deplacement:.3} m", sous_w.0, sous_w.1);
    assert_eq!(sous_w.0, 0., "critère 2, W");
    assert!(sous_w.1 <= 1e-12, "critère 2, W : {}", sous_w.1);
    // Le témoin : au pas normal, 0,1 µm sous l'équilibre — la rampe des points reste linéaire sur huit pas (±4,5 mm).
    let mut t = balle(0.);
    t.position[2] = f.offset - 1e-7;
    let calme = CalmWater { level: 0. };
    let mut ecarts = vec![1e-7];
    for _ in 0..8 {
        t.step(1. / 30., &calme, MER);
        ecarts.push((t.position[2] - f.offset).abs());
    }
    let facteur = (ecarts[8] / ecarts[4]).powf(0.25);
    let x: f64 = f.omega / 30.;
    let predit = (1. - x * x / 2. - ((1. - x * x / 2.).powi(2) - 1.).sqrt()).abs();
    println!("S498 témoin au pas normal : écarts {ecarts:?} m, |G| par pas {facteur:.3} (prédit {predit:.3})");
    assert!(facteur > 1.5, "critère 3 : {facteur}");
    assert!((facteur / predit - 1.).abs() <= 0.05, "critère 3 : {facteur} contre {predit}");
}

/// **S498, critère 4 — hors mode contraint, rien n'amplifie.** Le navire (normal), la barque et la caisse (sous-cyclées) à 30 Hz, lâchés
/// 1 cm au-dessus de leur équilibre en eau calme, 120 s : le facteur par période `|G|`, tiré de l'enveloppe ajustée à la pulsation
/// exacte du pas, ≤ 1 + 10⁻⁹. (Les maxima interpolés par une parabole, essayés d'abord, portent un bruit d'échantillonnage de 6·10⁻⁴.)
#[test]
fn nothing_amplifies_outside_the_constrained_mode_s498() {
    for (nom, corps, _, _) in archetypes_s498().into_iter().take(3) {
        let f = corps.floating(1. / 30., MER);
        let mut c = corps.clone();
        c.position[2] = f.offset + 0.01;
        let calme = CalmWater { level: 0. };
        let z: Vec<f64> = (0..3600)
            .map(|_| {
                c.step_floating(&f, 1. / 30., &calme, &calme, MER);
                c.position[2] - f.offset
            })
            .collect();
        // Une application linéaire de module 1 rend une sinusoïde exacte aux pas, de pulsation `ω'` : `cos(ω'·dt/n) = 1 − (ω·dt/n)²/2`
        // par sous-pas. Ajuster `z = (a + b·t)·cos ω't + (c + d·t)·sin ω't` ; le facteur par période est `1 + T·(ab + cd)/(a² + c²)` —
        // une erreur de pulsation y entre au second ordre seulement.
        let sous = match f.regime { Regime::Subcycled(n) => n as f64, _ => 1. };
        let h = 1. / 30. / sous;
        let w = (1. - (f.omega * h).powi(2) / 2.).acos() / h;
        let mut m = [[0f64; 4]; 4];
        let mut r = [0f64; 4];
        for (i, zi) in z.iter().enumerate() {
            let t = (i + 1) as f64 / 30.;
            let v = [(w * t).cos(), t * (w * t).cos(), (w * t).sin(), t * (w * t).sin()];
            for p in 0..4 {
                r[p] += v[p] * zi;
                for q in 0..4 {
                    m[p][q] += v[p] * v[q];
                }
            }
        }
        // Gauss avec pivot partiel, 4 × 4.
        for col in 0..4 {
            let piv = (col..4).max_by(|&i, &j| m[i][col].abs().total_cmp(&m[j][col].abs())).unwrap();
            m.swap(col, piv);
            r.swap(col, piv);
            for row in col + 1..4 {
                let k = m[row][col] / m[col][col];
                for q in col..4 {
                    m[row][q] -= k * m[col][q];
                }
                r[row] -= k * r[col];
            }
        }
        let mut sol = [0f64; 4];
        for row in (0..4).rev() {
            sol[row] = (r[row] - (row + 1..4).map(|q| m[row][q] * sol[q]).sum::<f64>()) / m[row][row];
        }
        let [a0, b0, c0, d0] = sol;
        let periode = core::f64::consts::TAU / w;
        let g = 1. + periode * (a0 * b0 + c0 * d0) / (a0 * a0 + c0 * c0);
        let residu = z.iter().enumerate().map(|(i, zi)| {
            let t = (i + 1) as f64 / 30.;
            (zi - (a0 + b0 * t) * (w * t).cos() - (c0 + d0 * t) * (w * t).sin()).abs()
        }).fold(0f64, f64::max);
        println!("S498 {nom} ({:?}) : {:.0} périodes, |G| {g:.12}, amplitude {:.4} m, résidu de l'ajustement {residu:.1e} m", f.regime, 120. / periode, a0.hypot(c0));
        assert!(g <= 1. + 1e-9, "critère 4, {nom} : {g}");
    }
}
