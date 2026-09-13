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

#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;

use water_core::{
    background::Background,
    background_spectrum,
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
    prepared_water::{self, mixed, BoundBackground},
    wave_journal::{self, Cause},
    FrameId, HostServices, SeaState, SimTime, WorldPos,
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

/// Le meme champ, a une profondeur imposee : la relation de dispersion en depend, donc la
/// decroissance peut en dependre aussi. L'effondrement en `tau` a ete mesure a 20 m, ou tous les
/// membres de la famille sont en eau profonde (`kh > pi`). Ce controle le met a l'epreuve.
fn field_at_depth(wavelength_m: f32, energy_j: f32, depth: f32) -> Option<RadialImpact<256>> {
    let m = Medium {
        gravity: 9.81,
        density: 1025.,
        depth,
        max_slope: BREAKING_SLOPE,
    };
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

fn depth_control(samples: u32) {
    const TAUS: [f32; 8] = [0., 1., 4., 8., 16., 27.4, 48., 96.];
    println!("PROFONDEUR lambda=3.35 ; kh = 2*pi*h/lambda ; eau profonde si kh > pi = 3.1416");
    for depth in [20.0f32, 8.0, 4.0, 2.0, 1.0, 0.6] {
        let Some(field) = field_at_depth(3.35, 164., depth) else {
            println!("PROFONDEUR h={depth} refusee_a_la_construction");
            continue;
        };
        let scale = (3.35f32 / 9.81).sqrt();
        let annonce = field.slope_max();
        let mut line = String::new();
        for tau in TAUS {
            let t = SimTime(BIRTH + (tau * scale * 1e6) as u64);
            let peak = peak_within(&field, [0., 0.], t, samples, 15.5 * 3.35);
            line.push_str(&format!(" {:.3}", annonce / peak.max(f32::MIN_POSITIVE)));
        }
        println!(
            "PROFONDEUR h={depth} kh={:.4} slope_max={annonce:.6} pessimisme_par_tau ={line}",
            2. * core::f32::consts::PI * depth / 3.35
        );
    }
}

/// Majorant resserre du champ a un age donne, par la table.
fn tightened(annonce: f32, wavelength_m: f32, age_s: f32) -> f32 {
    let tau = age_s / (wavelength_m / 9.81f32).sqrt();
    annonce / RHO[(tau as usize).min(RHO.len() - 1)]
}

/// S215 P5 — la scene J1 avec **une source de plus**, et ce que le coeur en dit.
///
/// Le budget de composition est une somme (ADR-128, ADR-119 regle 1). Cette fonction l'exerce :
/// un, deux, puis trois impacts, chacun avec le sillage prescrit, et elle demande au coeur —
/// `mixed_water` — de composer. Le verdict n'est pas deduit d'une addition faite ici : c'est
/// celui que la bibliotheque rend.
fn two_source_refusal() {
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
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
    let recipe_b = background_spectrum::Recipe {
        sea: SeaState {
            hs: 1.5,
            tp: 6.,
            theta_turns: 0.12,
            components: 32,
            graine: 201,
        },
        gravity: 9.81,
        gamma: 3.3,
        min_ratio: 0.5,
        max_ratio: 4.,
        spread_turns: 0.25,
    };
    let cooked = background_spectrum::bake(recipe_b).expect("recette S201");
    let background = Background::from_spectrum(
        &mut HostServices {
            alloc: &mut alloc,
            jobs: &host_impl::SequentialJobs,
            sink: &host_impl::StderrSink,
        },
        &cooked,
        WorldPos::from_units(0, 0, 0),
    )
    .unwrap();
    let bound = BoundBackground::new(&background, FrameId(0), 0);

    let recipe = Recipe {
        sigma: 2.,
        cutoff: 3.,
        radial: 64,
        angular: 128,
    };
    let source = wake(recipe);
    let mut precords = [None];
    let mut pjournal = Journal::new(1, &mut precords);
    pjournal.admit_authenticated(source.source()).unwrap();
    let n = recipe.radial * recipe.angular;
    let mut full = vec![Node::default(); n];
    let mut half = vec![Node::default(); n / 2];
    let spectrum = gaussian_spectrum::bake(recipe, &mut full)
        .unwrap()
        .half_into(&mut half)
        .unwrap();
    let context = source.source().context();
    let mut slots = vec![Slot::default(); spectrum.nodes().len()];

    // Trois impacts voisins : l'intersection des disques de 52 m reste large, donc c'est bien le
    // budget qui decide et non la geometrie.
    let positions = [[0., 10., 0.], [3., 10., 0.], [-3., 10., 0.]];
    let age = 16.0f32;
    let t = SimTime(BIRTH + (age * 1e6) as u64);
    let prepared =
        bound_pressure::Prepared::from_journal(context, &spectrum, &pjournal, t, &mut slots)
            .unwrap();
    let wake_envelope = prepared.slope_envelope();
    let point = WorldPos::from_metres(0., 10., 0.);
    for count in 1..=3usize {
        let mut records = [None; 3];
        let mut journal = wave_journal::Journal::new(1, &mut records);
        for (i, p) in positions.iter().take(count).enumerate() {
            let v = Impact {
                id: 203 + i as u64,
                frame: FrameId(0),
                cell: 0,
                birth: SimTime(BIRTH),
                ttl_us: 56_000_000,
                position: *p,
                energy_j,
                wavelength_m,
                direction_turns: 0.,
                anisotropy: 0.,
                displaced_l: 0.,
                material: 0,
                origin: Origin::Server,
                above_surface: true,
            };
            journal
                .confirm(
                    1,
                    Cause {
                        entity: 203 + i as u64,
                        command: 1,
                        emission: 0,
                    },
                    WaveEvent::impact(v).unwrap(),
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
                medium: m,
                domain: Domain {
                    radius: RADIUS,
                    age_us: 56_000_000,
                },
            },
        )
        .unwrap();
        let floor = mixed::slope_floor(&impacts, Some(&prepared), t);
        let mut scratch = [Default::default(); 1];
        let mut out = [Default::default(); 1];
        let verdict = mixed::sample_world_batch(
            &bound,
            &impacts,
            Some(&prepared),
            t,
            &[point],
            BREAKING_SLOPE,
            &mut scratch,
            &mut out,
        );
        let serre = count as f32 * tightened(0.212607, wavelength_m, age) + wake_envelope;
        println!(
            "SOURCES n_impacts={count} age={age} budget={floor:.6} part={:.4} verdict={verdict:?} | budget_resserre={serre:.6} part_resserree={:.4} admis_resserre={}",
            floor / BREAKING_SLOPE,
            serre / BREAKING_SLOPE,
            serre <= BREAKING_SLOPE
        );
    }
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

/// Génère la table sûre `ρ(τ)` : un intervalle par unité d'âge adimensionné, et dans chaque
/// intervalle le **minimum** de ρ sur `sub` sous-échantillons. Le minimum, parce que diviser le
/// majorant par un ρ trop grand le ferait cesser d'en être un — et parce que ρ n'est pas
/// monotone (elle plonge puis remonte entre τ = 0 et τ = 1).
/// Longueurs d'onde **génératrices**. La table est le minimum sur elles toutes : l'effondrement
/// en `τ` est exact à trois décimales, mais l'âge transite en microsecondes entières, et cette
/// quantification suffit à faire varier le rapport de quelques ppm d'un λ à l'autre. Prendre le
/// minimum sur la famille mesurée borne ce résidu par construction, au lieu d'inventer une marge
/// sans provenance (I-14). Le contrôle se fait ensuite sur des λ **hors** de cette liste.
/// Table rho(tau) engendree par `--table` (S215). Elle vit ici tant qu'elle n'est pas dans
/// la bibliotheque ; P6 l'y deplace.
const RHO: [f32; 96] = [
    1.0000, 1.0464, 1.1979, 1.4944, 2.0209, 2.9092, 3.8726, 4.4430,
    4.8234, 5.1032, 5.5618, 5.6788, 6.0734, 6.2731, 6.5697, 6.9068,
    7.0707, 7.4902, 7.5702, 8.0718, 8.0913, 8.4915, 8.6388, 8.9340,
    9.1950, 9.3914, 9.7438, 9.8680, 10.2953, 10.3682, 10.8369, 10.8802,
    11.3674, 11.3980, 11.8112, 11.9242, 12.2721, 12.4596, 12.7465, 12.9957,
    13.2255, 13.5293, 13.7156, 14.0661, 14.2129, 14.6001, 14.7193, 15.1364,
    15.2264, 15.6757, 15.7429, 16.2142, 16.2675, 16.7551, 16.7932, 17.2649,
    17.3239, 17.7614, 17.8609, 18.2648, 18.3995, 18.7723, 18.9392, 19.2866,
    19.4818, 19.8048, 20.0288, 20.3266, 20.5791, 20.8533, 21.1294, 21.3852,
    21.6811, 21.9188, 22.2368, 22.4563, 22.7945, 22.9980, 23.3531, 23.5440,
    23.9135, 24.0949, 24.4763, 24.6468, 25.0398, 25.1992, 25.6043, 25.7548,
    26.1716, 26.3141, 26.7403, 26.8759, 27.3100, 27.4401, 27.8952, 28.7706,
];

const GENERATRICES: [(f32, f32); 4] = [(0.5, 0.05), (1.0, 0.5), (3.35, 164.0), (8.0, 4_000.0)];

fn generate_table(samples: u32, sub: u32, upto: u32) -> Vec<f32> {
    let fields: Vec<_> = GENERATRICES
        .iter()
        .map(|&(l, e)| {
            let f = family_field(l, e).expect("champ générateur");
            let annonce = f.slope_max();
            (f, (l / 9.81f32).sqrt(), annonce, 15.5 * l)
        })
        .collect();
    (0..upto)
        .map(|k| {
            let mut lowest = f32::INFINITY;
            for j in 0..=sub {
                let tau = k as f32 + j as f32 / sub as f32;
                for (field, scale, annonce, radius) in &fields {
                    let t = SimTime(BIRTH + (tau * scale * 1e6) as u64);
                    let peak = peak_within(field, [0., 0.], t, samples, *radius);
                    lowest = lowest.min(annonce / peak.max(f32::MIN_POSITIVE));
                }
            }
            // Garde. L'effondrement en `τ` est exact, mais l'âge transite en microsecondes
            // entières : sur trois λ **hors** famille génératrice, le majorant resserré est
            // dépassé d'au plus **3,0e-6**. La garde vaut `1e-4`, soit trente-trois fois ce
            // dépassement mesuré — c'est sa provenance (I-14), et le banc qui la fixe est cet
            // exemple. Elle ne mord pas à `τ = 0`, où `ρ = 1` et où l'annonce d'origine n'est
            // pas dépassée (0,999983).
            (lowest / 1.0001).max(1.0)
        })
        .collect()
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--table") {
        let table = generate_table(4_000, 20, 96);
        println!("// ρ(τ) sûre, S215 : minimum sur chaque intervalle unité d'âge adimensionné.");
        for (k, chunk) in table.chunks(8).enumerate() {
            let row: Vec<String> = chunk.iter().map(|v| format!("{v:.4}")).collect();
            println!("    /* τ {:>2}.. */ {},", k * 8, row.join(", "));
        }
        // Contre-vérification sur des longueurs d'onde **hors** de la famille génératrice, à des
        // τ qui ne tombent pas sur la grille : la table doit rester un majorant.
        for (l, e) in [(2.0f32, 20.0f32), (5.0, 900.0), (0.75, 0.2)] {
            let Some(other) = family_field(l, e) else {
                println!("TABLE_CONTROLE lambda={l} E={e} refusee");
                continue;
            };
            let scale = (l / 9.81f32).sqrt();
            let annonce = other.slope_max();
            let (mut worst, mut worst_plain, mut worst_tau) = (0.0f32, 0.0f32, 0.0f32);
            for i in 0..=960 {
                let tau = i as f32 * 0.1;
                let t = SimTime(BIRTH + (tau * scale * 1e6) as u64);
                let peak = peak_within(&other, [0., 0.], t, 4_000, 15.5 * l);
                let serre = annonce / table[(tau as usize).min(95)];
                if peak / serre > worst {
                    worst = peak / serre;
                    worst_tau = tau;
                }
                worst_plain = worst_plain.max(peak / annonce);
            }
            println!("TABLE_CONTROLE lambda={l} E={e} max(reelle/resserre)={worst:.6} a tau={worst_tau:.1} ; max(reelle/origine)={worst_plain:.6}");
        }
        let other = family_field(1.0, 0.5).expect("champ de contrôle");
        let scale = (1.0f32 / 9.81).sqrt();
        let annonce = other.slope_max();
        let (mut worst, mut worst_plain, mut worst_tau) = (0.0f32, 0.0f32, 0.0f32);
        for i in 0..=960 {
            let tau = i as f32 * 0.1;
            let t = SimTime(BIRTH + (tau * scale * 1e6) as u64);
            let peak = peak_within(&other, [0., 0.], t, 4_000, 15.5);
            let serre = annonce / table[(tau as usize).min(95)];
            if peak / serre > worst {
                worst = peak / serre;
                worst_tau = tau;
            }
            worst_plain = worst_plain.max(peak / annonce);
        }
        // Le témoin qui décide : le majorant **non resserré** est-il lui-même dépassé, et de
        // combien ? Si les deux excès sont du même ordre, le resserrement n'ajoute aucun risque
        // — il hérite de la précision de l'annonce d'origine (ADR-094, « atteinte à 1e-3 »).
        println!("TABLE_CONTROLE lambda=1 E=0.5 max(reelle/majorant_resserre)={worst:.6} a tau={worst_tau:.1} ; max(reelle/majorant_origine)={worst_plain:.6}");
        return;
    }
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
    depth_control(samples);

    // --- P5 : le refus a plusieurs sources, exerce et non deduit.
    two_source_refusal();

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
