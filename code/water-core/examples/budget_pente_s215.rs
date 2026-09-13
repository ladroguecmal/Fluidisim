//! S215 — A254 : ce que valent réellement les majorants de pente du budget de composition.
//!
//! S214 a mesuré le budget conjoint de la scène J1 à 84 % de `BREAKING_SLOPE`, pour une pente
//! réelle 3,7 à 10,5 fois plus faible — mais sur une grille de 1,3 m, quand le maximum de pente
//! d'un impact radial est atteint en `r = 0,2062 λ` (ADR-094), soit 0,69 m ici. Cet exemple
//! refait la mesure **à échantillonnage fin**, champ par champ, et vérifie que chaque majorant
//! publié en est bien un — la seconde moitié de l'annonce d'ADR-094, jamais vérifiée au-delà
//! de 2 s.
//!
//! Fixture : celle de la scène J1 (S201/S203/S205/S212), redéclarée ici depuis les mêmes entrées
//! de générateur ; λ et E sont imprimés pour qu'une divergence avec `viewer/src/scene.rs` se voie.

use water_core::{
    bound_pressure::{self, Settings},
    gaussian_spectrum::{self, Recipe},
    impact_field::{Medium, BREAKING_SLOPE},
    impact_generator::{self, Entry},
    pressure_journal::Journal,
    pressure_source::Metadata,
    radial_impact::{Domain, RadialImpact},
    spectral_pressure::{Node, Slot},
    wake_source::{Leg, Wake},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::Cause,
    FrameId, SimTime,
};

const BIRTH: u64 = 12_000_000;
const RADIUS: f32 = 52.;
const WAKE_MIN: [f32; 2] = [-64., -48.];
const WAKE_MAX: [f32; 2] = [64., 56.];
/// Âges communs aux deux champs. Au-delà de 40 s le sillage n'a plus de contexte ; l'impact vit
/// jusqu'à 56 s.
const AGES: [f64; 14] = [
    0., 0.5, 1., 2., 4., 8., 12., 16., 18., 20., 24., 30., 39., 56.,
];

fn medium() -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.,
        depth: 20.,
        max_slope: BREAKING_SLOPE,
    }
}

fn impact() -> (RadialImpact<256>, f32, f32) {
    let m = medium();
    let (energy_j, wavelength_m) = impact_generator::impact_from_entry(
        &Entry {
            half_width_m: 1.,
            speed_ms: 8.,
            transferred_fraction: 0.005,
        },
        &m,
    )
    .unwrap();
    let event = WaveEvent::impact(Impact {
        id: 203,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(BIRTH),
        ttl_us: 56_000_000,
        position: [0., 10., 0.],
        energy_j,
        wavelength_m,
        direction_turns: 0.,
        anisotropy: 0.,
        displaced_l: 0.,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap();
    let field = RadialImpact::new(
        event,
        m,
        Domain {
            radius: RADIUS,
            age_us: 56_000_000,
        },
    )
    .unwrap();
    (field, energy_j, wavelength_m)
}

/// Pente réelle maximale du champ radial à un instant, le long d'un rayon.
///
/// Le champ est isotrope — `anisotropy = 0`, `direction_turns = 0` —, donc un rayon suffit et
/// c'est ainsi qu'ADR-094 l'a reçu. `samples` points sur `[0, RADIUS]` : à 20 000, le pas vaut
/// 2,6 mm, soit λ/1288, et le pic de `0,2062 λ` est traversé par plus de 260 points.
fn impact_peak(field: &RadialImpact<256>, position: [f32; 2], t: SimTime, samples: u32) -> f32 {
    let mut peak = 0.0f32;
    for i in 0..=samples {
        let r = RADIUS * i as f32 / samples as f32;
        let p = [position[0] + r, position[1]];
        if let Ok(s) = field.sample(FrameId(0), 0, p, t) {
            peak = peak.max((s.slope[0] * s.slope[0] + s.slope[1] * s.slope[1]).sqrt());
        }
    }
    peak
}

/// Sillage prescrit S212, à la recette demandée.
fn wake(recipe: Recipe) -> Wake {
    let settings = Settings {
        frame: FrameId(0),
        cell: 0,
        gravity: 9.81,
        density: 1025.,
        min: WAKE_MIN,
        max: WAKE_MAX,
        start: SimTime(BIRTH),
        end: SimTime(BIRTH + 40_000_000),
    };
    let metadata = Metadata {
        epoch: 1,
        id: 212,
        cause: Cause {
            entity: 212,
            command: 1,
            emission: 0,
        },
        settings,
        recipe,
    };
    let legs = [Leg {
        duration_us: 2_000_000,
        velocity: [3., 0.],
        downward_force_n: 19_620.,
    }; 8];
    Wake::build(metadata, SimTime(BIRTH), [-24., 4.], &legs).expect("sillage S212")
}

fn main() {
    let samples: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(20_000);
    let (field, energy_j, wavelength_m) = impact();
    println!(
        "FIXTURE lambda_m={wavelength_m:.6} energie_J={energy_j:.6} pic_theorique_r_m={:.4} rayon_m={RADIUS} echantillons={samples} pas_m={:.6}",
        0.2062 * wavelength_m,
        RADIUS / samples as f32
    );
    let annonce = field.slope_max();
    println!(
        "IMPACT_ANNONCE slope_max={annonce:.6} slope_bound_L1={:.6} ratio_L1={:.6}",
        field.slope_bound(),
        field.slope_bound() / annonce
    );

    // --- P2 : pente réelle de l'impact contre son majorant, sur toute sa durée de vie.
    let mut worst_safety = 0.0f32;
    for age in AGES {
        let t = SimTime(BIRTH + (age * 1e6) as u64);
        let peak = impact_peak(&field, [0., 10.], t, samples);
        let ratio = annonce / peak.max(f32::MIN_POSITIVE);
        worst_safety = worst_safety.max(peak / annonce);
        println!("IMPACT age={age} pente_reelle={peak:.6} majorant={annonce:.6} pessimisme={ratio:.4}");
    }
    println!(
        "IMPACT_SURETE max(pente_reelle/majorant)={worst_safety:.6} (doit rester <= 1)"
    );

    // --- P3 : pente réelle du sillage contre son enveloppe, à chaque âge.
    let recipe = Recipe {
        sigma: 2.,
        cutoff: 3.,
        radial: 64,
        angular: 128,
    };
    let source = wake(recipe);
    let mut records = [None];
    let mut journal = Journal::new(1, &mut records);
    journal.admit_authenticated(source.source()).unwrap();
    let n = recipe.radial * recipe.angular;
    let mut full = vec![Node::default(); n];
    let mut half = vec![Node::default(); n / 2];
    let spectrum = gaussian_spectrum::bake(recipe, &mut full)
        .unwrap()
        .half_into(&mut half)
        .unwrap();
    let context = source.source().context();
    let mut slots = vec![Slot::default(); spectrum.nodes().len()];
    // Grille fine dans l'emprise. Le sillage n'a pas de pic ponctuel comme l'impact : sa plus
    // courte longueur d'onde représentée vaut `2π/cutoff = 2,09 m`, donc un pas de 0,25 m la
    // résout à λ/8,4 et un pas de 0,1 m à λ/21. Le pas est un argument pour que la convergence
    // du maximum se mesure au lieu de se supposer.
    let step: f32 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.25);
    let nx = ((WAKE_MAX[0] - WAKE_MIN[0]) / step) as usize;
    let ny = ((WAKE_MAX[1] - WAKE_MIN[1]) / step) as usize;
    let mut scratch = [Default::default(); 1];
    let mut out = [Default::default(); 1];
    let mut worst_wake = 0.0f32;
    for age in AGES.iter().copied().filter(|a| *a <= 40.) {
        let t = SimTime(BIRTH + (age * 1e6) as u64);
        let prepared =
            bound_pressure::Prepared::from_journal(context, &spectrum, &journal, t, &mut slots)
                .unwrap();
        let envelope = prepared.slope_envelope();
        let mut peak = 0.0f32;
        for iy in 0..=ny {
            for ix in 0..=nx {
                let p = [
                    WAKE_MIN[0] + ix as f32 * step,
                    WAKE_MIN[1] + iy as f32 * step,
                ];
                if prepared
                    .sample_batch(&context, t, &[p], &mut scratch, &mut out)
                    .is_ok()
                {
                    let s = out[0].slope;
                    peak = peak.max((s[0] * s[0] + s[1] * s[1]).sqrt());
                }
            }
        }
        worst_wake = worst_wake.max(peak / envelope);
        println!(
            "SILLAGE age={age} pente_reelle={peak:.6} majorant={envelope:.6} pessimisme={:.4} points={}",
            envelope / peak.max(f32::MIN_POSITIVE),
            (nx + 1) * (ny + 1)
        );
    }
    println!("SILLAGE_SURETE max(pente_reelle/majorant)={worst_wake:.6} (doit rester <= 1)");
}
