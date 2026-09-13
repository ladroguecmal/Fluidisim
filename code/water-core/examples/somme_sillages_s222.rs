//! S222 — A254, part somme : ce que coûte une scène à plusieurs sillages dans le budget de pente,
//! et ce qu'une borne locale conjointe lui rendrait.
//!
//! `mixed_water::slope_floor` somme **un majorant par impact** (ADR-133) plus **un seul** terme de
//! pression : plusieurs sillages n'entrent donc au budget qu'en partageant un journal, une recette
//! et une emprise. Dans cette configuration ils partagent aussi les **emplacements** du
//! demi-spectre — leurs amplitudes modales s'additionnent en complexe —, donc le terme de pression
//! est sous-additif par construction. De combien, c'est ce que cet exemple mesure, proches puis
//! éloignés, contre le maximum réel et contre la borne locale partitionnée (ADR-137).

use std::time::Instant;
use water_core::{
    bound_pressure::{self, Prepared, Settings},
    impact_field::{Medium, BREAKING_SLOPE},
    impact_generator::{self, Entry},
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    gaussian_spectrum::{self, Recipe},
    pressure_journal::Journal,
    pressure_source::Metadata,
    spectral_pressure::{Node, SlopeCell, SlopeOrder, Slot},
    wake_source::{Leg, Wake},
    wave_journal::Cause,
    FrameId, SimTime,
};

const START: u64 = 12_000_000;
const MIN: [f32; 2] = [-64., -48.];
const MAX: [f32; 2] = [64., 48.];
/// Recette S219–S221, pour que le champ d'une source soit comparable au bit à leurs relevés.
const RECIPE: Recipe = Recipe {
    sigma: 2.,
    cutoff: 3.,
    radial: 64,
    angular: 128,
};

fn settings() -> Settings {
    Settings {
        frame: FrameId(0),
        cell: 0,
        gravity: 9.81,
        density: 1025.,
        min: MIN,
        max: MAX,
        start: SimTime(START),
        end: SimTime(72_000_000),
    }
}

/// Une source de sillage, identifiée à part, sur la trajectoire `y = offset`.
fn wake(index: u64, offset: f32, speed: f32, duration: f32) -> Option<Wake> {
    let metadata = Metadata {
        epoch: 1,
        id: 222 + index,
        cause: Cause {
            entity: 222 + index,
            command: 1,
            emission: 0,
        },
        settings: settings(),
        recipe: RECIPE,
    };
    let legs = [Leg {
        duration_us: (duration * 1e6 / 4.) as u64,
        velocity: [speed, 0.],
        downward_force_n: 19620.,
    }; 4];
    Wake::build(metadata, SimTime(START), [-12., offset], &legs).ok()
}

/// Maximum réel de la pente dans l'emprise : balayage puis raffinement local (méthode S215/S216).
fn real_peak(
    f: &Prepared<'_>,
    ctx: &bound_pressure::Context,
    time: SimTime,
    step: f32,
) -> (f32, [f32; 2]) {
    let nx = ((MAX[0] - MIN[0]) / step) as usize;
    let ny = ((MAX[1] - MIN[1]) / step) as usize;
    let mut scratch = [Default::default(); 1];
    let mut out = [Default::default(); 1];
    let mut slope_at = |p: [f32; 2]| {
        f.sample_batch(ctx, time, &[p], &mut scratch, &mut out)
            .ok()
            .map(|()| {
                let s = out[0].slope;
                (s[0] * s[0] + s[1] * s[1]).sqrt()
            })
    };
    let (mut peak, mut argmax) = (0.0f32, [0.0f32; 2]);
    for iy in 0..=ny {
        for ix in 0..=nx {
            let p = [MIN[0] + ix as f32 * step, MIN[1] + iy as f32 * step];
            if let Some(v) = slope_at(p) {
                if v > peak {
                    peak = v;
                    argmax = p;
                }
            }
        }
    }
    for iy in -50..=50i32 {
        for ix in -50..=50i32 {
            let p = [argmax[0] + ix as f32 * 0.02, argmax[1] + iy as f32 * 0.02];
            if let Some(v) = slope_at(p) {
                if v > peak {
                    peak = v;
                    argmax = p;
                }
            }
        }
    }
    (peak, argmax)
}


/// Maximum reel dans un rectangle, meme methode que `real_peak` mais borne a la region.
fn peak_in(
    f: &Prepared<'_>,
    ctx: &bound_pressure::Context,
    time: SimTime,
    lo: [f32; 2],
    hi: [f32; 2],
    step: f32,
) -> f32 {
    let nx = ((hi[0] - lo[0]) / step) as usize;
    let ny = ((hi[1] - lo[1]) / step) as usize;
    let mut scratch = [Default::default(); 1];
    let mut out = [Default::default(); 1];
    let mut peak = 0.0f32;
    for iy in 0..=ny {
        for ix in 0..=nx {
            let p = [
                (lo[0] + ix as f32 * step).min(hi[0]),
                (lo[1] + iy as f32 * step).min(hi[1]),
            ];
            if f.sample_batch(ctx, time, &[p], &mut scratch, &mut out).is_ok() {
                let s = out[0].slope;
                peak = peak.max((s[0] * s[0] + s[1] * s[1]).sqrt());
            }
        }
    }
    peak
}

/// S222 P4 — ce qu'une **requete locale** paierait et gagnerait.
///
/// La partition borne toute l'emprise et coute des secondes. Un hote qui interroge une region
/// bornee n'a pas besoin de cela : un seul appel `local_slope_envelope_spectral` sur le rectangle
/// demande suffit, en O(N). Cette fonction mesure les deux cotes — ce que la borne rend contre
/// l'enveloppe globale, et ce que l'appel coute — sur une region centree au pire point et sur une
/// region quelconque.
fn local_query(
    f: &Prepared<'_>,
    ctx: &bound_pressure::Context,
    time: SimTime,
    label: &str,
    count: u64,
    global: f32,
    argmax: [f32; 2],
    step: f32,
) {
    for (nom, centre) in [("au_pire", argmax), ("ailleurs", [-40.0f32, 30.0])] {
        for half in [1.0f32, 4.0, 16.0] {
            let lo = [
                (centre[0] - half).max(MIN[0]),
                (centre[1] - half).max(MIN[1]),
            ];
            let hi = [
                (centre[0] + half).min(MAX[0]),
                (centre[1] + half).min(MAX[1]),
            ];
            let Ok(e) = f.local_slope_envelope_spectral(ctx, time, lo, hi) else {
                println!("  LOCALE={label} sources={count} region={nom} demi={half} refusee");
                continue;
            };
            // Cout : meilleure de plusieurs rafales, l'appel etant court (methode micro S221).
            let mut best = f64::INFINITY;
            for _ in 0..5 {
                let start = Instant::now();
                for _ in 0..2000 {
                    std::hint::black_box(
                        f.local_slope_envelope_spectral(ctx, time, lo, hi).unwrap(),
                    );
                }
                best = best.min(start.elapsed().as_secs_f64() * 1e6 / 2000.);
            }
            let local_max = peak_in(f, ctx, time, lo, hi, step);
            println!(
                "  LOCALE={label} sources={count} region={nom} demi={half} borne={:.9} gain_sur_global={:.4} maximum_local={local_max:.9} borne_sur_maximum_local={:.4} us_par_appel={best:.1}",
                e.bound,
                global / e.bound,
                e.bound / local_max.max(f32::MIN_POSITIVE)
            );
        }
    }
}


/// Terme d'impact du budget a cet instant : `slope_max_at` (ADR-133), pour la scene J1.
fn impact_term(time: SimTime) -> f32 {
    let m = Medium {
        gravity: 9.81,
        density: 1025.,
        depth: 20.,
        max_slope: BREAKING_SLOPE,
    };
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
        birth: SimTime(START),
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
    let field = RadialImpact::<256>::new(
        event,
        m,
        Domain {
            radius: 52.,
            age_us: 56_000_000,
        },
    )
    .unwrap();
    field.slope_max_at(time)
}

/// Combien d'impacts tiennent encore sous `pi/7` une fois le terme de pression paye.
fn impacts_that_fit(pressure: f32, impact: f32) -> i32 {
    if impact <= 0.0 {
        return -1;
    }
    (((BREAKING_SLOPE - pressure) / impact).floor()).max(0.0) as i32
}

/// S222 P6 — le terme d'impact depend fortement de l'age (ADR-133). La conclusion de P5 tient a
/// l'instant mesure ; ce balayage en dit le domaine, au lieu de le supposer.
fn impact_by_age(pressure: f32) {
    println!("AGE_IMPACT pression_3_sillages_eloignes={pressure:.9} pi_sur_7={BREAKING_SLOPE:.9}");
    for age_s in [0.0f64, 0.5, 1., 2., 4., 8., 16., 32., 56.] {
        let t = SimTime(START + (age_s * 1e6) as u64);
        let imp = impact_term(t);
        println!(
            "AGE_IMPACT age_s={age_s} impact_unitaire={imp:.9} part_d_un_impact={:.4} impacts_admis={}",
            imp / BREAKING_SLOPE,
            impacts_that_fit(pressure, imp)
        );
    }
}

fn main() {
    let step: f32 = std::env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(0.5);
    println!("S222 CPU release un fil; sources gaussiennes dans un meme journal, meme recette et meme emprise; enveloppe directionnelle ADR-134 (terme actuel du budget) contre borne locale partitionnee ADR-137 et maximum reel; aucune technique GPU/LOD/visibilite/mutualisation; pas_m={step}");

    // Instant : celui de la fixture « base » de S219–S221 — 8 s de forçage puis tau = 4.
    let age = 8.0f64 + 4.0 * (2.0f64 / 9.81).sqrt();
    let time = SimTime(START + (age * 1e6).round() as u64);

    let mut nodes = vec![Node::default(); 8192];
    let mut half = vec![Node::default(); 4096];
    let spectrum = gaussian_spectrum::bake(RECIPE, &mut nodes)
        .unwrap()
        .half_into(&mut half)
        .unwrap();
    let mut pool = vec![SlopeCell::default(); 32768];
    impact_by_age(0.165356964);

    // Trajectoires : écart en y entre sources. « proches » = deux fois sigma, « eloignes » = 30 m,
    // soit bien au-delà de la largeur du sillage de Kelvin à cette vitesse.
    let mut single = 0.0f32;
    for (label, spacing) in [("proches", 4.0f32), ("eloignes", 30.0)] {
        for count in 1..=3u64 {
            let offsets: Vec<f32> = (0..count)
                .map(|i| (i as f32 - (count as f32 - 1.) / 2.) * spacing)
                .collect();
            let sources: Vec<Wake> = offsets
                .iter()
                .enumerate()
                .filter_map(|(i, &o)| wake(i as u64, o, 3., 8.))
                .collect();
            if sources.len() != count as usize {
                println!("config={label} sources={count} refusee_a_la_construction");
                continue;
            }
            let mut records = [None; 3];
            let mut journal = Journal::new(1, &mut records);
            let mut admitted = 0usize;
            for w in &sources {
                if journal.admit_authenticated(w.source()).is_ok() {
                    admitted += 1;
                }
            }
            if admitted != sources.len() {
                println!("config={label} sources={count} admises={admitted} refus_journal");
                continue;
            }
            let ctx = sources[0].source().context();
            let mut slots = vec![Slot::default(); 4096];
            let start = Instant::now();
            let f = match Prepared::from_journal(ctx, &spectrum, &journal, time, &mut slots) {
                Ok(f) => f,
                Err(e) => {
                    println!("config={label} sources={count} preparation_refusee={e:?}");
                    continue;
                }
            };
            let prep_ms = start.elapsed().as_secs_f64() * 1e3;
            let global = f.slope_envelope();
            if count == 1 && label == "proches" {
                single = global;
            }
            let (peak, argmax) = real_peak(&f, &ctx, time, step);
            println!(
                "CONFIG={label} sources={count} ecart_m={spacing} modes={} global={global:.9} global_sur_une_source={:.4} maximum={peak:.9} argmax=[{:.2},{:.2}] pessimisme_global={:.4} preparation_ms={prep_ms:.3}",
                f.component_count(),
                global / single.max(f32::MIN_POSITIVE),
                argmax[0],
                argmax[1],
                global / peak.max(f32::MIN_POSITIVE)
            );
            local_query(&f, &ctx, time, label, count, global, argmax, step);
            let mut partitioned = global;
            for budget in [2047usize, 8191, 32767] {
                let start = Instant::now();
                let r = f
                    .partition_slope_envelope_order(
                        &ctx,
                        time,
                        MIN,
                        MAX,
                        &mut pool,
                        budget,
                        SlopeOrder::Spectral,
                    )
                    .unwrap();
                let ms = start.elapsed().as_secs_f64() * 1e3;
                assert!(r.bound >= peak, "borne {} sous le maximum {peak}", r.bound);
                println!(
                    "  PARTITION={label} sources={count} budget={budget} evaluations={} feuilles={} borne={:.9} gain_sur_global={:.4} borne_sur_maximum={:.4} arret={:?} ms={ms:.1}",
                    r.evaluations,
                    r.leaves,
                    r.bound,
                    global / r.bound,
                    r.bound / peak.max(f32::MIN_POSITIVE),
                    r.stop
                );
                partitioned = r.bound;
            }
            // P5 : la meme mesure traduite dans le contrat d'admission, impacts sommes (ADR-133).
            let imp = impact_term(time);
            println!(
                "  ADMISSION={label} sources={count} pi_sur_7={BREAKING_SLOPE:.9} impact_unitaire={imp:.9} pression_enveloppe={global:.9} part={:.4} impacts_admis={} | pression_partitionnee={partitioned:.9} part={:.4} impacts_admis={} | maximum_reel={peak:.9} part={:.4} impacts_admis={}",
                global / BREAKING_SLOPE,
                impacts_that_fit(global, imp),
                partitioned / BREAKING_SLOPE,
                impacts_that_fit(partitioned, imp),
                peak / BREAKING_SLOPE,
                impacts_that_fit(peak, imp)
            );
        }
    }
}
