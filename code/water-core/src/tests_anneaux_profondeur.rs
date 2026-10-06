// ══ S528 — les anneaux d'impact en profondeur finie ═════════════════════════════════════════════════════════════════════════
use super::*;
use crate::wave_event::{Impact, Origin};

fn evenement() -> WaveEvent {
    WaveEvent::impact(Impact {
        id: 528,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(0),
        ttl_us: 10_000_000,
        position: [0.0; 3],
        energy_j: 0.01,
        wavelength_m: 4.0,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap()
}

fn milieu(depth: f32) -> Medium {
    Medium { gravity: 9.81, density: 1025.0, depth, max_slope: 0.1 }
}

fn bits(s: Sample) -> [u32; 7] {
    [s.eta, s.deta_dt, s.potential, s.slope[0], s.slope[1], s.horizontal_velocity[0], s.horizontal_velocity[1]].map(f32::to_bits)
}

/// **S528, critère 1 — l'eau profonde au bit.** `new_in_depth` avec `2 k_min h` > 32 (k_min = 0,785 rad/m à λ = 4 m : h = 30 m) rend les
/// échantillons de `new` au bit, à plusieurs âges et rayons ; par 1 m de fond, `new` refuse le régime et `new_in_depth` l'accepte.
#[test]
fn a_deep_enough_finite_depth_ring_is_the_deep_ring_bit_for_bit_s528() {
    let domaine = Domain { radius: 40.0, age_us: 10_000_000 };
    let profond = RadialImpact::<128>::new(evenement(), milieu(30.0), domaine).unwrap();
    let fini = RadialImpact::<128>::new_in_depth(evenement(), milieu(30.0), domaine).unwrap();
    for t in [1u64, 500_000, 5_000_000, 10_000_000] {
        for p in [[0.0f32, 0.0], [3.0, 1.5], [-12.0, 20.0], [30.0, -25.0]] {
            let a = profond.sample(FrameId(0), 0, p, SimTime(t)).unwrap();
            let b = fini.sample(FrameId(0), 0, p, SimTime(t)).unwrap();
            assert_eq!(bits(a), bits(b), "t {t}, p {p:?}");
        }
    }
    assert!(matches!(RadialImpact::<128>::new(evenement(), milieu(1.0), domaine), Err(Error::Regime)));
    assert!(RadialImpact::<128>::new_in_depth(evenement(), milieu(1.0), domaine).is_ok());
}

/// **S528, critère 3 — les bornes de pente en eau peu profonde.** Par 1 m de fond, la pente réelle du champ (sur une couronne de rayons
/// et d'angles, toutes les 0,25 s jusqu'à 10 s) reste sous `slope_max_at` et sous `slope_max_beyond` au rayon du point.
#[test]
fn the_slope_bounds_hold_in_shallow_water_s528() {
    let domaine = Domain { radius: 40.0, age_us: 10_000_000 };
    let anneau = RadialImpact::<128>::new_in_depth(evenement(), milieu(1.0), domaine).unwrap();
    let mut pire = 0f32;
    for n in 0..=40u64 {
        let t = SimTime(250_000 * n);
        let globale = anneau.slope_max_at(t);
        for i in 0..160 {
            let r = 0.25 * i as f32;
            let p = [r * 0.6, r * 0.8];
            let s = anneau.sample(FrameId(0), 0, p, t).unwrap();
            let pente = (s.slope[0] * s.slope[0] + s.slope[1] * s.slope[1]).sqrt();
            assert!(pente <= globale, "t {} s, r {r} : {pente} > {globale}", n as f32 * 0.25);
            assert!(pente <= anneau.slope_max_beyond(t, r), "couronne, t {} s, r {r}", n as f32 * 0.25);
            pire = pire.max(pente / globale);
        }
    }
    println!("S528 : la pente réelle au plus {pire:.3} de slope_max_at, par 1 m de fond, 0–10 s");
}
