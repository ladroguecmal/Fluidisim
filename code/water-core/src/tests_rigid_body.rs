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
