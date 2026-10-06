// ══ S522 — la pression de W en profondeur finie ═════════════════════════════════════════════════════════════════════════════
use super::*;
use crate::spectral_pressure::{self, Node, Slot};

fn tenue(duration_us: u64) -> Segment {
    Segment { birth: SimTime(0), duration_us, origin: [0.0, 0.0], velocity: [0.0, 0.0], pressure_pa: 1000.0 }
}

fn bits(r: Response) -> [u32; 4] {
    [r.eta.re, r.eta.im, r.velocity.re, r.velocity.im].map(f32::to_bits)
}

/// **S522, critère 1 — le chemin profond au bit.** Une profondeur où `2|k|h` > 32 rend les modes de l'eau profonde au bit (pression tenue
/// et source mobile, à plusieurs âges) ; une profondeur nulle, négative ou non finie est refusée.
#[test]
fn a_deep_enough_depth_is_the_deep_water_mode_bit_for_bit_s522() {
    for k in [[0.5f32, 0.0], [0.3, 0.4], [2.0, -1.0]] {
        let m = (k[0] * k[0] + k[1] * k[1]).sqrt();
        let h = 17.0 / m;
        for s in [tenue(4_000_000), Segment { velocity: [3.0, 1.0], ..tenue(4_000_000) }] {
            let a = ModalPressure::new(k, 9.81, 1025.0, s, 16_000_000).unwrap();
            let b = ModalPressure::new_in_depth(k, 9.81, 1025.0, Some(h), s, 16_000_000).unwrap();
            for t in [1u64, 1_000_000, 3_999_999, 4_000_000, 9_000_000, 16_000_000] {
                assert_eq!(bits(a.sample(SimTime(t)).unwrap()), bits(b.sample(SimTime(t)).unwrap()), "k {k:?}, t {t}");
            }
        }
    }
    for h in [0.0f32, -1.0, f32::NAN, f32::INFINITY] {
        assert!(ModalPressure::new_in_depth([1.0, 0.0], 9.81, 1025.0, Some(h), tenue(1_000_000), 8_000_000).is_err());
    }
}

/// **S522, critère 2 — un mode en profondeur finie.** Après le forçage, la pulsation libre mesurée sur le signal du mode (`x(t+τ) + x(t−τ)
/// = 2 cos ωτ · x(t)`, en moindres carrés) est `√(g k tanh kh)` à 10⁻⁴ relatif ; sous la pression tenue, au premier creux (`ωt = π`), la
/// hauteur vaut `−2p/ρg` quelle que soit la profondeur.
#[test]
fn a_mode_in_finite_depth_oscillates_at_the_finite_depth_frequency_s522() {
    let (g, rho, p) = (9.81f64, 1025.0f64, 1000.0f64);
    let k = 0.8f32;
    for kh in [0.1f64, 0.5, 1.0, 3.0] {
        let h = (kh / k as f64) as f32;
        let attendu = (g * k as f64 * (k as f64 * h as f64).tanh()).sqrt();
        // Le forçage : 2 s ; puis 40 échantillons espacés de τ = 0,05 s.
        let m = ModalPressure::new_in_depth([k, 0.0], 9.81, 1025.0, Some(h), tenue(2_000_000), 64_000_000).unwrap();
        let x: Vec<f64> = (0..400).map(|n| m.sample(SimTime(3_000_000 + 50_000 * n)).unwrap().eta.re as f64).collect();
        let (mut num, mut den) = (0f64, 0f64);
        for n in 1..x.len() - 1 {
            num += x[n] * (x[n + 1] + x[n - 1]);
            den += 2. * x[n] * x[n];
        }
        let omega = (num / den).acos() / 0.05;
        let ecart = omega / attendu - 1.;
        // Le premier creux sous la pression tenue (10 s de forçage).
        let longue = ModalPressure::new_in_depth([k, 0.0], 9.81, 1025.0, Some(h), tenue(16_000_000), 64_000_000).unwrap();
        let t_creux = (core::f64::consts::PI / attendu * 1e6).round() as u64;
        let creux = longue.sample(SimTime(t_creux)).unwrap().eta.re as f64;
        let ecart_creux = creux / (-2. * p / (rho * g)) - 1.;
        println!("S522 mode kh = {kh} : ω mesurée {omega:.6} rad/s, attendue {attendu:.6}, écart {ecart:.2e} ; creux {creux:.6} m, écart {ecart_creux:.2e}");
        assert!(ecart.abs() <= 1e-4, "critère 2, pulsation à kh = {kh} : {ecart}");
        assert!(ecart_creux.abs() <= 1e-3, "critère 2, creux à kh = {kh} : {ecart_creux}");
    }
}

/// **S522, critère 1 — le champ.** `prepare_in_depth` sans profondeur, ou avec une profondeur où tous les nœuds ont `2|k|h` > 32, rend
/// le champ de `prepare` au bit ; avec 5 m de fond, il en diffère.
#[test]
fn a_field_in_deep_enough_water_is_the_deep_field_bit_for_bit_s522() {
    use crate::gaussian_spectrum::{bake, Recipe};
    let recette = Recipe { sigma: 1.0, cutoff: 4.0, radial: 32, angular: 32 };
    let mut noeuds = vec![Node::default(); 32 * 32];
    let spectre = bake(recette, &mut noeuds).unwrap();
    let k_min = spectre.nodes().iter().map(|n| (n.k[0] * n.k[0] + n.k[1] * n.k[1]).sqrt()).fold(f32::MAX, f32::min);
    let chemin = [Segment { velocity: [2.0, 0.0], ..tenue(3_000_000) }];
    let champ = |profondeur: Option<f32>, pool: &mut Vec<Slot>| {
        let f = spectral_pressure::prepare_in_depth(spectre.nodes(), &chemin, 9.81, 1025.0, profondeur, SimTime(4_000_000),
            SimTime(8_000_000), [-16.0; 2], [16.0; 2], pool).unwrap();
        [[0.0f32, 0.0], [-3.0, 1.5], [5.0, -2.0]].map(|p| {
            let s = f.sample(p).unwrap();
            [s.eta, s.vertical_velocity, s.potential, s.slope[0], s.horizontal_velocity[0]].map(f32::to_bits)
        })
    };
    let (mut a, mut b, mut c, mut d) = (vec![Slot::default(); 1024], vec![Slot::default(); 1024], vec![Slot::default(); 1024], vec![Slot::default(); 1024]);
    let profond = {
        let f = spectral_pressure::prepare(spectre.nodes(), &chemin, 9.81, 1025.0, SimTime(4_000_000), SimTime(8_000_000), [-16.0; 2],
            [16.0; 2], &mut a).unwrap();
        [[0.0f32, 0.0], [-3.0, 1.5], [5.0, -2.0]].map(|p| {
            let s = f.sample(p).unwrap();
            [s.eta, s.vertical_velocity, s.potential, s.slope[0], s.horizontal_velocity[0]].map(f32::to_bits)
        })
    };
    assert_eq!(profond, champ(None, &mut b));
    assert_eq!(profond, champ(Some(17.0 / k_min), &mut c));
    assert_ne!(profond, champ(Some(5.0), &mut d));
    assert!(spectral_pressure::prepare_in_depth(spectre.nodes(), &chemin, 9.81, 1025.0, Some(0.0), SimTime(4_000_000), SimTime(8_000_000),
        [-16.0; 2], [16.0; 2], &mut d).is_err());
}
