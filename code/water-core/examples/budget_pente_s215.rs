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

/// Champ radial d'une longueur d'onde et d'une énergie données, avec le domaine que **la
/// fixture elle-même suit** : ADR-126 demande `R ≥ 15,5 λ` et `A ≥ 96 √(λ/g)`, et la scène J1
/// (λ 3,35 m, rayon 52 m, ttl 56 s) est exactement à ces deux bornes. La famille les reprend,
/// pour que chaque membre soit observé sur la même portion de sa propre vie.
fn family_field(wavelength_m: f32, energy_j: f32) -> Option<RadialImpact<256>> {
    let m = medium();
    let scale = (wavelength_m / m.gravity).sqrt();
    let event = WaveEvent::impact(Impact {
        id: 215,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(BIRTH),
        ttl_us: (96. * scale * 1e6) as u64,
        position: [0., 0., 0.],
        energy_j,
        wavelength_m,
        direction_turns: 0.,
        anisotropy: 0.,
        displaced_l: 0.,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .ok()?;
    RadialImpact::new(
        event,
        m,
        Domain {
            radius: 15.5 * wavelength_m,
            age_us: (96. * scale * 1e6) as u64,
        },
    )
    .ok()
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
    peak_within(field, position, t, samples, RADIUS)
}

fn peak_within(
    field: &RadialImpact<256>,
    position: [f32; 2],
    t: SimTime,
    samples: u32,
    radius: f32,
) -> f32 {
    let mut peak = 0.0f32;
    for i in 0..=samples {
        let r = radius * i as f32 / samples as f32;
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
        let mut slope_at = |p: [f32; 2]| {
            prepared
                .sample_batch(&context, t, &[p], &mut scratch, &mut out)
                .ok()
                .map(|()| {
                    let s = out[0].slope;
                    (s[0] * s[0] + s[1] * s[1]).sqrt()
                })
        };
        // Balayage grossier pour localiser, puis **raffinement local** : un balayage global plus
        // fin coûte le carré du gain et ne regarde que le voisinage du maximum pour rien. La
        // fenêtre fait ±1 m — plus que le demi-pas grossier — au pas de 2 cm, soit λ_min/105.
        let (mut peak, mut argmax) = (0.0f32, [0.0f32; 2]);
        for iy in 0..=ny {
            for ix in 0..=nx {
                let p = [
                    WAKE_MIN[0] + ix as f32 * step,
                    WAKE_MIN[1] + iy as f32 * step,
                ];
                if let Some(v) = slope_at(p) {
                    if v > peak {
                        peak = v;
                        argmax = p;
                    }
                }
            }
        }
        let coarse = peak;
        let fine = 0.02f32;
        for iy in -50..=50i32 {
            for ix in -50..=50i32 {
                let p = [
                    argmax[0] + ix as f32 * fine,
                    argmax[1] + iy as f32 * fine,
                ];
                if let Some(v) = slope_at(p) {
                    peak = peak.max(v);
                }
            }
        }
        worst_wake = worst_wake.max(peak / envelope);
        println!(
            "SILLAGE age={age} pente_reelle={peak:.6} grossier={coarse:.6} gain_raffinement={:.4} argmax=[{:.2},{:.2}] majorant={envelope:.6} pessimisme={:.4} points={}",
            peak / coarse.max(f32::MIN_POSITIVE),
            argmax[0],
            argmax[1],
            envelope / peak.max(f32::MIN_POSITIVE),
            (nx + 1) * (ny + 1) + 101 * 101
        );
    }
    println!("SILLAGE_SURETE max(pente_reelle/majorant)={worst_wake:.6} (doit rester <= 1)");

    // --- P3-bis : la décroissance est-elle universelle dans la famille ?
    //
    // Si le pessimisme ne dépend que de l'âge **adimensionné** `τ = t/√(λ/g)`, alors un seul
    // rapport mesuré `ρ(τ)` suffit à resserrer `slope_max` pour toute la famille — exactement
    // ce que `SLOPE_L1_RATIO` est déjà pour `τ = 0` (ADR-094, S141), une dimension plus riche.
    // Deux énergies à λ égal éprouvent au passage la linéarité : le rapport doit être identique.
    println!("FAMILLE tau=t/sqrt(lambda/g) ; domaine ADR-126 (R=15,5λ, A=96√(λ/g))");
    const TAUS: [f32; 11] = [0., 0.5, 1., 2., 4., 8., 16., 27.4, 48., 66.8, 96.];
    for (wavelength_m, energy_j) in [
        (0.5f32, 0.05f32),
        (1.0, 0.5),
        (3.35, 164.0),
        (3.35, 16.4),
        (8.0, 4_000.0),
        (20.0, 100_000.0),
    ] {
        let Some(field) = family_field(wavelength_m, energy_j) else {
            println!("FAMILLE lambda={wavelength_m} E={energy_j} refusee_a_la_construction");
            continue;
        };
        let scale = (wavelength_m / 9.81f32).sqrt();
        let annonce = field.slope_max();
        let radius = 15.5 * wavelength_m;
        let mut line = String::new();
        for tau in TAUS {
            let t = SimTime(BIRTH + (tau * scale * 1e6) as u64);
            let peak = peak_within(&field, [0., 0.], t, samples, radius);
            line.push_str(&format!(" {:.3}", annonce / peak.max(f32::MIN_POSITIVE)));
        }
        println!(
            "FAMILLE lambda={wavelength_m} E={energy_j} echelle_s={scale:.4} slope_max={annonce:.6} pessimisme_par_tau ={line}"
        );
    }
}
