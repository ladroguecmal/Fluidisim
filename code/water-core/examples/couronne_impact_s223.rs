//! S223 — A262 : une inegalite conjointe pour la somme spatiale des majorants d'impact.
//!
//! P2 — surete de la borne `B(x) = min(0,5819 ; sqrt(2/pi x))` contre `J1` **telle que le code la
//! calcule**. L'inegalite `|J1(x)| <= sqrt(2/pi x)` est vraie en mathematiques pour tout `x > 0` ;
//! elle ne dit rien de l'approximation executee — table de Hermite jusqu'a 64, developpement
//! asymptotique au-dela (ADR-084). C'est cette approximation-la que la borne doit majorer, sinon
//! l'inegalite est fausse pour le programme.

use std::time::Instant;
use water_core::{
    impact_field::{Medium, BREAKING_SLOPE},
    impact_generator::{self, Entry},
    prepared_water::{self, mixed},
    radial_impact::{bessel, Domain, RadialImpact, BESSEL_MAX},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::{self, Cause},
    FrameId, SimTime,
};

const BIRTH: u64 = 12_000_000;
const RADIUS: f32 = 52.;

/// Maximum de `|J1|` : atteint en `x ≈ 1,8412` et vaut `0,581865`.
const J1_PEAK: f32 = 0.581_865_0;

/// Constante de decroissance : `sup_x |J1(x)| * sqrt(x)` sur le domaine **execute**.
///
/// L'asymptote `sqrt(2/pi x)` — soit `0,7979/sqrt(x)` — est la limite en `+inf`, **pas** une borne :
/// `|J1|` la depasse aux `x` moderes (mesure en P2 : 3,4 % a `x = 2,17`). La constante ci-dessous
/// est relevee sur `bessel` executee ; c'est elle qui rend la borne vraie pour le programme.
const J1_DECAY: f32 = 0.0; // relevee en P2, puis figee

/// Borne sur `|J1(x)|` pour `x >= x0`. Plate jusqu'au pic, en `1/sqrt(x)` ensuite.
fn bound_with(decay: f32, x0: f32) -> f32 {
    if x0 <= 0.0 {
        return J1_PEAK;
    }
    J1_PEAK.min(decay / x0.sqrt())
}


/// Champ d'impact de la scene J1, centre en `c`.
fn impact_event(index: u64, c: [f32; 2]) -> WaveEvent {
    let m = medium();
    let (energy_j, wavelength_m) = impact_generator::impact_from_entry(
        &Entry { half_width_m: 1., speed_ms: 8., transferred_fraction: 0.005 },
        &m,
    )
    .unwrap();
    WaveEvent::impact(Impact {
        id: 223 + index,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(BIRTH),
        ttl_us: 56_000_000,
        position: [c[0], c[1], 0.],
        energy_j,
        wavelength_m,
        direction_turns: 0.,
        anisotropy: 0.,
        displaced_l: 0.,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .unwrap()
}

fn medium() -> Medium {
    Medium { gravity: 9.81, density: 1025., depth: 20., max_slope: BREAKING_SLOPE }
}

/// Maximum reel de la pente des perturbations sur **l'intersection des disques** — la seule
/// region que la composition admet (ADR-077, ADR-080).
fn joint_peak<const N: usize>(
    fields: &[RadialImpact<N>],
    centres: &[[f32; 2]],
    time: SimTime,
    step: f32,
) -> f32 {
    let r = RADIUS;
    let (mut lo, mut hi) = ([f32::MAX; 2], [f32::MIN; 2]);
    for c in centres {
        lo[0] = lo[0].min(c[0] - r);
        lo[1] = lo[1].min(c[1] - r);
        hi[0] = hi[0].max(c[0] + r);
        hi[1] = hi[1].max(c[1] + r);
    }
    let nx = ((hi[0] - lo[0]) / step) as usize;
    let ny = ((hi[1] - lo[1]) / step) as usize;
    let mut peak = 0.0f32;
    for iy in 0..=ny {
        for ix in 0..=nx {
            let p = [lo[0] + ix as f32 * step, lo[1] + iy as f32 * step];
            let mut sx = 0.0f32;
            let mut sy = 0.0f32;
            let mut inside = true;
            for f in fields.iter() {
                match f.sample(FrameId(0), 0, p, time) {
                    Ok(s) => {
                        sx += s.slope[0];
                        sy += s.slope[1];
                    }
                    Err(_) => {
                        inside = false;
                        break;
                    }
                }
            }
            if inside {
                peak = peak.max((sx * sx + sy * sy).sqrt());
            }
        }
    }
    peak
}

/// P4/P5 — la somme actuelle, l'inegalite conjointe et le maximum reel, par separation et par age.
fn joint_scan(step: f32) {
    println!("S223 P4/P5 somme actuelle (slope_floor) contre inegalite conjointe (slope_floor_joint) et maximum reel; impacts scene J1, rayon {RADIUS} m; pas_m={step}");
    for count in 2..=3usize {
        for d in [0.0f32, 5.0, 20.0, 50.0, 90.0] {
            let centres: Vec<[f32; 2]> = (0..count)
                .map(|i| [i as f32 * d, 10.0f32])
                .collect();
            let mut records = [None; 3];
            let mut journal = wave_journal::Journal::new(1, &mut records);
            for (i, c) in centres.iter().enumerate() {
                journal
                    .confirm(
                        1,
                        Cause { entity: 223 + i as u64, command: 1, emission: 0 },
                        impact_event(i as u64, *c),
                    )
                    .unwrap();
            }
            let mut pool: [Option<RadialImpact<256>>; 3] = [None, None, None];
            let impacts = prepared_water::Prepared::<256>::build(
                &journal,
                &mut pool,
                prepared_water::Context {
                    frame: FrameId(0),
                    cell: 0,
                    medium: medium(),
                    domain: Domain { radius: RADIUS, age_us: 56_000_000 },
                },
            )
            .unwrap();
            // Champs reconstruits a part pour le maximum reel : `impacts` emprunte `pool`.
            let independent: Vec<RadialImpact<256>> = centres
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    RadialImpact::new(
                        impact_event(i as u64, *c),
                        medium(),
                        Domain { radius: RADIUS, age_us: 56_000_000 },
                    )
                    .unwrap()
                })
                .collect();
            for age in [0.0f32, 2.0, 8.0] {
                let t = SimTime(BIRTH + (age * 1e6) as u64);
                let plain = mixed::slope_floor(&impacts, None, t);
                // Sensibilite au nombre d'echantillons : le cout y est lineaire, la qualite non.
                let mut by_samples = String::new();
                for n in [4usize, 8, 16, 32, 64, 128] {
                    let mut best = f64::INFINITY;
                    let mut v = 0.0f32;
                    for _ in 0..200 {
                        let start = Instant::now();
                        v = mixed::slope_floor_joint(&impacts, None, t, n);
                        best = best.min(start.elapsed().as_secs_f64() * 1e6);
                    }
                    by_samples.push_str(&format!(" n{n}={v:.6}/{best:.1}us"));
                }
                let start = Instant::now();
                let joint = mixed::slope_floor_joint(&impacts, None, t, 64);
                let us = start.elapsed().as_secs_f64() * 1e6;
                let peak = joint_peak(&independent, &centres, t, step);
                assert!(joint >= peak, "borne conjointe {joint} sous le maximum {peak}");
                println!(
                    "JOINT impacts={count} d={d} age={age} somme={plain:.9} conjointe={joint:.9} gain={:.4} maximum={peak:.9} conjointe_sur_max={:.4} somme_sur_max={:.4} us={us:.1} part_somme={:.4} part_conjointe={:.4}",
                    plain / joint.max(f32::MIN_POSITIVE),
                    joint / peak.max(f32::MIN_POSITIVE),
                    plain / peak.max(f32::MIN_POSITIVE),
                    plain / BREAKING_SLOPE,
                    joint / BREAKING_SLOPE
                );
                println!("  SAMPLES impacts={count} d={d} age={age}{by_samples}");
            }
        }
    }
}

fn main() {
    let samples: u32 = std::env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(4_000_000);
    println!("S223 P2 CPU release un fil; borne B(x)=min(0.581865, sqrt(2/pi x)) contre bessel(x).1 executee; domaine [0, {BESSEL_MAX}]; echantillons={samples}");

    // Deux regimes, separes : la table de Hermite jusqu'a 64, l'asymptotique au-dela. Les
    // mecanismes d'erreur n'y sont pas les memes, et les melanger cacherait lequel deborde.
    for (nom, lo, hi) in [
        ("table", 0.0f64, 64.0f64),
        ("asymptotique", 64.0, BESSEL_MAX as f64),
    ] {
        // Ce que l'asymptote ferait si on la prenait pour une borne, et la constante qu'il faut.
        let asym = (2.0f32 / core::f32::consts::PI).sqrt();
        let (mut worst, mut worst_x, mut worst_j1, mut worst_b) = (0.0f64, 0.0f64, 0.0f32, 0.0f32);
        let (mut decay, mut decay_x) = (0.0f32, 0.0f32);
        for i in 0..=samples {
            let x = lo + (hi - lo) * i as f64 / samples as f64;
            let xf = x as f32;
            let Ok((_, j1)) = bessel(xf) else { continue };
            if xf > 0.0 {
                let d = j1.abs() * xf.sqrt();
                if d > decay {
                    decay = d;
                    decay_x = xf;
                }
            }
            let b = bound_with(asym, xf);
            if b <= 0.0 {
                continue;
            }
            let ratio = (j1.abs() / b) as f64;
            if ratio > worst {
                worst = ratio;
                worst_x = x;
                worst_j1 = j1;
                worst_b = b;
            }
        }
        println!("REGIME={nom} asymptote_comme_borne pire_rapport={worst:.9} a x={worst_x:.6} j1={worst_j1:.9} borne={worst_b:.9} depassement_ppm={:.1} | constante_requise sup|J1|sqrt(x)={decay:.9} a x={decay_x:.6} contre asymptote={asym:.9}", (worst - 1.0) * 1e6);
    }

    // Le pic lui-meme : la borne plate doit majorer ce que la table rend autour de x = 1,8412.
    let (mut peak, mut peak_x) = (0.0f32, 0.0f32);
    for i in 0..=200_000u32 {
        let x = 1.5 + 0.7 * i as f32 / 200_000.0;
        if let Ok((_, j1)) = bessel(x) {
            if j1.abs() > peak {
                peak = j1.abs();
                peak_x = x;
            }
        }
    }
    println!("PIC_TABLE max_j1={peak:.9} a x={peak_x:.6} contre J1_PEAK={J1_PEAK:.9} rapport={:.9}", peak / J1_PEAK);
    let _ = J1_DECAY;
    joint_scan(std::env::args().nth(2).and_then(|a| a.parse().ok()).unwrap_or(0.25));
}
