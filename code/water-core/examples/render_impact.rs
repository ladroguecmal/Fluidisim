//! S203 : un impact porté par W, rendu sur B, avec emprise et observateur explicites.
//! CPU, sans dépendance, image locale seulement (ADR-124). Rien n'est ajouté à la bibliothèque :
//! l'hôte déclare la scène, la bibliothèque compose B+W dans l'emprise (ADR-062/063).
//!
//! Modes : `scene` (constats de construction), `seams` (coutures d'emprise),
//! `render <dir> <âge>` (règle de budget S203, reproduction), `cost` (coût par point),
//! `render-s205 <dir> <âge> [hs]` (S205, ADR-128 : budget d'impact π/7, mer S201 par défaut).
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
#[path = "support/ray_view.rs"]
mod ray_view;
use ray_view::*;
use std::{
    fs::File,
    io::{BufWriter, Write},
    time::Instant,
};
use water_core::{
    background::Background,
    background_spectrum::{self, Cooked, Recipe},
    impact_field::{Medium, BREAKING_SLOPE},
    impact_generator::{self, Entry},
    prepared_water::{BoundBackground, Context, Prepared},
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    wave_journal::{Cause, Journal},
    FrameId, HostServices, SeaState, SimTime, WaterSample, WorldPos,
};

/// Emprise retenue en S203 (P3/P3b) : premier candidat qui passe les deux coutures au critère
/// déclaré, accord N256/N512 à 0,1 µm, homothétie reçue à λ×2.
pub const EMPRISE_N: usize = 256;
pub const EMPRISE_RADIUS_M: f32 = 52.0;
pub const EMPRISE_AGE_US: u64 = 56_000_000;
/// Observateur S201, inchangé : la comparaison des images reste possible.
pub const CAMERA_ORIGIN: V = [0., -18., 7.];
pub const CAMERA_TARGET: V = [0., 35., 0.];
pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 360;

/// Recette S201 inchangée sauf Hs : à 1,5 m le plancher L1 de B vaut 0,6082 > π/7 et la
/// composition refuse chaque point avant tout impact (constat S203, EN-COURS).
pub const HS: f32 = 0.5;
/// Instant de l'image S201, repris comme naissance : la mer y est déjà inspectée.
pub const BIRTH: SimTime = SimTime(12_000_000);
/// Point d'entrée, local à l'ancre de B (0,0,0) — même couple frame/cell que W.
pub const IMPACT_XY: [f32; 2] = [0.0, 10.0];
pub const FRAME: FrameId = FrameId(0);
pub const CELL: u64 = 0;
/// Objet de banc : demi-largeur 1 m, entrée à 8 m/s. **Choix de banc**, pas un cas gameplay.
pub const ENTRY_HALF_WIDTH_M: f32 = 1.0;
pub const ENTRY_SPEED_MS: f32 = 8.0;
/// Fraction transférée aux ondes : **à calibrer B2** (impact_generator). Choisie sous la borne
/// que la marge de pente laisse, pour que l'impact soit admis ; elle ne dit rien de la physique.
pub const TRANSFERRED_FRACTION: f32 = 0.005;
/// Milieu profond : le régime radial exige profondeur > π/k_lo = λ.
pub const DEPTH_M: f32 = 20.0;

pub fn recipe(hs: f32) -> Recipe {
    Recipe {
        sea: SeaState {
            hs,
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
    }
}

/// Plancher de pente de B tel que `compose` le forme : `steepness·π`, même ordre de sommation
/// que `Background::eval_local` (ADR-117). C'est une borne L1, indépendante du point.
pub fn slope_floor(cooked: &Cooked) -> f32 {
    let mut steep = 0.0f32;
    for c in cooked.components() {
        steep += 2.0 * c.amplitude * c.k_turns_per_m;
    }
    steep * core::f32::consts::PI
}
/// Borne de hauteur de B, `Σ|a|` : sert à la marche de rayon, jamais au budget de pente.
pub fn height_bound(cooked: &Cooked) -> f64 {
    cooked
        .components()
        .iter()
        .map(|c| c.amplitude.abs() as f64)
        .sum()
}

/// Allocation du budget de pente de l'impact par l'hôte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlopeRule {
    /// S203, ADR-126 règle 3 : π/7 moins le plancher L1 de B. Conservée pour reproduire S203.
    MinusBackgroundS203,
    /// S205, ADR-128 : B n'est plus au budget de refus ; un impact seul dispose de π/7.
    PerturbationsAdr128,
}

pub struct Scene {
    pub cooked: Cooked,
    pub background: Background,
    pub floor: f32,
    pub medium: Medium,
    pub event: WaveEvent,
    pub energy_j: f32,
    pub wavelength_m: f32,
    pub max_fraction: f32,
}

/// Le budget de pente laissé à l'impact est ce que B n'a pas consommé : l'hôte le déclare dans
/// `Medium::max_slope`, pour que la construction refuse plutôt que chaque point de la requête.
pub fn scene(hs: f32, ttl_us: u64) -> Result<Scene, String> {
    scene_with(hs, ttl_us, ENTRY_HALF_WIDTH_M, SlopeRule::MinusBackgroundS203)
}
/// S205 : même scène, budget d'impact selon ADR-128.
pub fn scene_s205(hs: f32, ttl_us: u64) -> Result<Scene, String> {
    scene_with(hs, ttl_us, ENTRY_HALF_WIDTH_M, SlopeRule::PerturbationsAdr128)
}
pub fn scene_with(hs: f32, ttl_us: u64, half_width_m: f32, rule: SlopeRule) -> Result<Scene, String> {
    let cooked = background_spectrum::bake(recipe(hs)).map_err(|e| format!("recette {e:?}"))?;
    let floor = slope_floor(&cooked);
    let max_slope = match rule {
        SlopeRule::MinusBackgroundS203 => {
            if !(floor < BREAKING_SLOPE) {
                return Err(format!(
                    "plancher L1 de B {floor} >= pi/7 {BREAKING_SLOPE} : aucune composition possible"
                ));
            }
            BREAKING_SLOPE - floor
        }
        SlopeRule::PerturbationsAdr128 => BREAKING_SLOPE,
    };
    let medium = Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: DEPTH_M,
        max_slope,
    };
    let entry = Entry {
        half_width_m,
        speed_ms: ENTRY_SPEED_MS,
        transferred_fraction: TRANSFERRED_FRACTION,
    };
    let max_fraction = impact_generator::max_transferred_fraction(&entry, &medium);
    let (energy_j, wavelength_m) =
        impact_generator::impact_from_entry(&entry, &medium).map_err(|e| format!("entrée {e:?}"))?;
    let event = WaveEvent::impact(Impact {
        id: 203,
        frame: FRAME,
        cell: CELL,
        birth: BIRTH,
        ttl_us,
        position: [IMPACT_XY[0], IMPACT_XY[1], 0.0],
        energy_j,
        wavelength_m,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l: 0.0,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .map_err(|e| format!("événement {e:?}"))?;
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let background = Background::from_spectrum(
        &mut HostServices {
            alloc: &mut alloc,
            jobs: &jobs,
            sink: &sink,
        },
        &cooked,
        WorldPos::from_units(0, 0, 0),
    )
    .map_err(|e| format!("fond {e:?}"))?;
    Ok(Scene {
        cooked,
        background,
        floor,
        medium,
        event,
        energy_j,
        wavelength_m,
        max_fraction,
    })
}

/// Hauteur maximale d'un champ radial : à la naissance et au centre, `J0 = 1` et chaque
/// coefficient est positif (forme `x²(1−x)²`), donc `η(0, naissance) = Σ coefficients`, borne L1
/// exacte. La bibliothèque ne publie pas cette borne ; l'échantillon la rend.
pub fn radial_height_bound<const N: usize>(field: &RadialImpact<N>) -> Result<f64, String> {
    let v = field.event().data();
    let s = field
        .sample(v.frame, v.cell, [v.position[0], v.position[1]], v.birth)
        .map_err(|e| format!("centre {e:?}"))?;
    Ok(s.eta as f64)
}

/// Majorant **directionnel** de la pente de B, indépendant du point et du temps :
/// `|∇η| = |Σ cᵢ dᵢ cos φᵢ| = max_u Σ cᵢ (dᵢ·u) cos φᵢ ≤ max_u Σ cᵢ |dᵢ·u|`, avec `cᵢ = aᵢkᵢ`.
/// Strictement sous la borne L1 `Σ cᵢ` dès que les directions ne sont pas colinéaires. Le
/// maximum sur `u` est pris sur 3600 directions puis majoré par la pente de la fonction en `u`
/// (`Σ cᵢ` par radian) fois le demi-pas : c'est une borne, pas un échantillon.
pub fn directional_slope_bound(cooked: &Cooked) -> f64 {
    let steps = 3600;
    let l1: f64 = cooked
        .components()
        .iter()
        .map(|c| (c.amplitude.abs() * c.k_turns_per_m) as f64 * core::f64::consts::TAU)
        .sum();
    let mut best = 0.0f64;
    for s in 0..steps {
        let a = s as f64 / steps as f64 * core::f64::consts::PI;
        let u = [a.cos(), a.sin()];
        let v: f64 = cooked
            .components()
            .iter()
            .map(|c| {
                (c.amplitude.abs() * c.k_turns_per_m) as f64
                    * core::f64::consts::TAU
                    * (c.dir[0] as f64 * u[0] + c.dir[1] as f64 * u[1]).abs()
            })
            .sum();
        best = best.max(v);
    }
    best + l1 * core::f64::consts::PI / steps as f64 / 2.0
}

/// Pente réelle maximale de B **échantillonnée** : borne inférieure du maximum, pas le maximum.
/// Grille carrée centrée sur l'impact, pas `step_m`, instants `0..=seconds` au pas d'une seconde.
pub fn sampled_real_slope(bg: &Background, half_m: f64, step_m: f64, seconds: u64) -> f64 {
    let n = (2.0 * half_m / step_m).round() as i64;
    let mut max = 0.0f64;
    for s in 0..=seconds {
        let t = SimTime(s * 1_000_000);
        for i in 0..=n {
            for j in 0..=n {
                let x = IMPACT_XY[0] as f64 - half_m + i as f64 * step_m;
                let y = IMPACT_XY[1] as f64 - half_m + j as f64 * step_m;
                if let Some(w) = bg.eval(WorldPos::from_metres(x, y, 0.0), t) {
                    let nz = w.normal[2] as f64;
                    let slope = (w.normal[0] as f64).hypot(w.normal[1] as f64) / nz;
                    max = max.max(slope);
                }
            }
        }
    }
    max
}

/// Tolérance verticale de la marche de rayon S201 : une couture sous ce seuil ne déplace aucune
/// intersection au-delà de ce que le rendu admet déjà.
pub const RAY_TOLERANCE_M: f64 = 0.003;
/// Seuil relatif ADR-120, **emprunté comme choix de banc** : il n'a pas été dérivé pour une
/// couture visuelle, et ne le devient pas en étant employé ici.
pub const RELATIVE_SEAM: f64 = 0.02;

#[derive(Clone, Copy, Debug)]
pub struct Seams {
    pub radius: f32,
    pub eta_centre: f64,
    /// Couture spatiale : r = R − 1 mm, t ∈ [0 ; A].
    pub edge_eta: f64,
    pub edge_slope: f64,
    pub edge_eta_time_s: f64,
    /// Couture temporelle : t = A, r ∈ [0 ; R).
    pub horizon_eta: f64,
    pub horizon_slope: f64,
    pub horizon_eta_radius: f64,
}
impl Seams {
    pub fn limit(&self) -> f64 {
        RAY_TOLERANCE_M.min(RELATIVE_SEAM * self.eta_centre)
    }
    pub fn passes(&self) -> bool {
        self.edge_eta <= self.limit() && self.horizon_eta <= self.limit()
    }
}

/// Rayon maximal admis par `RadialImpact::new` pour (N, A) : reproduit le contrôle de
/// résolution en f64 puis **vérifie par construction**, en reculant de 0,5 m jusqu'à admission.
pub fn largest_radius<const N: usize>(sc: &Scene, age_us: u64) -> Option<(f32, RadialImpact<N>)> {
    let k0 = core::f64::consts::TAU / sc.wavelength_m as f64;
    let dk = 1.5 * k0 / N as f64;
    let cg = 0.5 * (sc.medium.gravity as f64 / (k0 / 2.0)).sqrt();
    let reach = 2048.0 / (2.0 * k0);
    let mut radius =
        ((core::f64::consts::FRAC_PI_2 / dk - cg * age_us as f64 / 1e6).min(reach) * 2.0).floor() / 2.0;
    while radius > 0.0 {
        let domain = Domain {
            radius: radius as f32,
            age_us,
        };
        if let Ok(f) = RadialImpact::<N>::new(sc.event, sc.medium, domain) {
            return Some((radius as f32, f));
        }
        radius -= 0.5;
    }
    None
}

pub fn seams<const N: usize>(field: &RadialImpact<N>, radius: f32, age_us: u64) -> Result<Seams, String> {
    let v = *field.event().data();
    let centre = [v.position[0], v.position[1]];
    let eta_centre = radial_height_bound(field)?;
    let mut out = Seams {
        radius,
        eta_centre,
        edge_eta: 0.0,
        edge_slope: 0.0,
        edge_eta_time_s: 0.0,
        horizon_eta: 0.0,
        horizon_slope: 0.0,
        horizon_eta_radius: 0.0,
    };
    let edge = [centre[0] + radius - 0.001, centre[1]];
    let steps = age_us / 10_000;
    for i in 0..=steps {
        let t = SimTime(v.birth.0 + (i * 10_000).min(age_us));
        let s = field
            .sample(v.frame, v.cell, edge, t)
            .map_err(|e| format!("couture spatiale {e:?}"))?;
        let slope = (s.slope[0] as f64).hypot(s.slope[1] as f64);
        if (s.eta as f64).abs() > out.edge_eta {
            out.edge_eta = (s.eta as f64).abs();
            out.edge_eta_time_s = (t.0 - v.birth.0) as f64 / 1e6;
        }
        out.edge_slope = out.edge_slope.max(slope);
    }
    let horizon = SimTime(v.birth.0 + age_us);
    let count = (radius / 0.02) as u32;
    for i in 0..count {
        let r = i as f32 * 0.02;
        let s = field
            .sample(v.frame, v.cell, [centre[0] + r, centre[1]], horizon)
            .map_err(|e| format!("couture temporelle {e:?}"))?;
        let slope = (s.slope[0] as f64).hypot(s.slope[1] as f64);
        if (s.eta as f64).abs() > out.horizon_eta {
            out.horizon_eta = (s.eta as f64).abs();
            out.horizon_eta_radius = r as f64;
        }
        out.horizon_slope = out.horizon_slope.max(slope);
    }
    Ok(out)
}

fn seam_line<const N: usize>(sc: &Scene, age_s: u64) -> Result<(), String> {
    let age_us = age_s * 1_000_000;
    match largest_radius::<N>(sc, age_us) {
        None => println!("N={N} A={age_s}s aucun rayon admis"),
        Some((radius, field)) => {
            let s = seams(&field, radius, age_us)?;
            println!(
                "N={N} A={age_s}s R={radius:.1}m bord_eta={:.3}mm (t={:.2}s) bord_pente={:.5} horizon_eta={:.3}mm (r={:.2}m) horizon_pente={:.5} seuil={:.3}mm passe={}",
                s.edge_eta * 1e3,
                s.edge_eta_time_s,
                s.edge_slope,
                s.horizon_eta * 1e3,
                s.horizon_eta_radius,
                s.horizon_slope,
                s.limit() * 1e3,
                s.passes()
            );
        }
    }
    Ok(())
}

fn seams_report() -> Result<(), String> {
    let sc = scene(HS, 60_000_000)?;
    println!("# coutures d'emprise S203, lambda={:.4} E={:.1}", sc.wavelength_m, sc.energy_j);
    for age_s in [2u64, 4, 6, 8, 12, 16, 24, 32] {
        seam_line::<64>(&sc, age_s)?;
        seam_line::<128>(&sc, age_s)?;
        seam_line::<256>(&sc, age_s)?;
        seam_line::<512>(&sc, age_s)?;
    }
    // Extension déclarée après la première série : aucun candidat ≤ 32 s ne passe, la couture
    // temporelle décroissant comme ~t^-0,72. Horizons longs, N256/N512 seuls admissibles.
    println!("# extension horizons longs");
    for age_s in [48u64, 56, 64, 72, 96] {
        seam_line::<256>(&sc, age_s)?;
        seam_line::<512>(&sc, age_s)?;
    }
    // Plus petit rayon qui passe, N512 : balayage au pas de 5 m sous le rayon maximal.
    println!("# balayage du rayon, N512");
    for age_s in [56u64, 64, 72] {
        let age_us = age_s * 1_000_000;
        let Some((r_max, _)) = largest_radius::<512>(&sc, age_us) else {
            continue;
        };
        let mut radius = 30.0f32;
        while radius <= r_max {
            let field = RadialImpact::<512>::new(sc.event, sc.medium, Domain { radius, age_us })
                .map_err(|e| format!("{e:?}"))?;
            let s = seams(&field, radius, age_us)?;
            println!(
                "N=512 A={age_s}s R={radius:.1}m bord_eta={:.3}mm (t={:.2}s) horizon_eta={:.3}mm (r={:.2}m) passe={}",
                s.edge_eta * 1e3,
                s.edge_eta_time_s,
                s.horizon_eta * 1e3,
                s.horizon_eta_radius,
                s.passes()
            );
            radius += 5.0;
        }
    }
    Ok(())
}

/// Écart maximal entre deux discrétisations du même événement, sur un rayon, à un âge donné.
pub fn radial_disagreement<const A: usize, const B: usize>(
    a: &RadialImpact<A>,
    b: &RadialImpact<B>,
    radius: f32,
    age_us: u64,
) -> Result<(f64, f64), String> {
    let v = *a.event().data();
    let t = SimTime(v.birth.0 + age_us);
    let (mut eta, mut slope) = (0.0f64, 0.0f64);
    for i in 0..(radius / 0.02) as u32 {
        let p = [v.position[0] + i as f32 * 0.02, v.position[1]];
        let sa = a.sample(v.frame, v.cell, p, t).map_err(|e| format!("{e:?}"))?;
        let sb = b.sample(v.frame, v.cell, p, t).map_err(|e| format!("{e:?}"))?;
        eta = eta.max((sa.eta as f64 - sb.eta as f64).abs());
        slope = slope.max((sa.slope[0] as f64 - sb.slope[0] as f64).abs());
    }
    Ok((eta, slope))
}

fn controls_report() -> Result<(), String> {
    let sc = scene(HS, 120_000_000)?;
    let a56 = 56_000_000;
    let n256 = RadialImpact::<256>::new(sc.event, sc.medium, Domain { radius: 52.0, age_us: a56 })
        .map_err(|e| format!("{e:?}"))?;
    let n512 = RadialImpact::<512>::new(sc.event, sc.medium, Domain { radius: 52.0, age_us: a56 })
        .map_err(|e| format!("{e:?}"))?;
    println!("# P3b-1 accord N256/N512, A56 R52");
    for age in [1_000_000u64, 3_000_000, 6_000_000, 30_000_000, 56_000_000] {
        let (eta, slope) = radial_disagreement(&n256, &n512, 52.0, age)?;
        println!(
            "age={:.0}s max|d_eta|={:.4}mm max|d_pente_x|={:.6} accord={}",
            age as f64 / 1e6,
            eta * 1e3,
            slope,
            eta <= RAY_TOLERANCE_M
        );
    }
    println!("# P3b-2 homothetie lambda x2 (b=2 m), coutures relatives");
    let big = scene_with(HS, 200_000_000, 2.0 * ENTRY_HALF_WIDTH_M, SlopeRule::MinusBackgroundS203)?;
    let reference = [
        (256usize, 52.0f32, a56, seams(&n256, 52.0, a56)?),
        (512, 55.0, 64_000_000, {
            let f = RadialImpact::<512>::new(sc.event, sc.medium, Domain { radius: 55.0, age_us: 64_000_000 })
                .map_err(|e| format!("{e:?}"))?;
            seams(&f, 55.0, 64_000_000)?
        }),
    ];
    println!(
        "lambda'={:.4}m E'={:.1}J (lambda={:.4}m E={:.1}J)",
        big.wavelength_m, big.energy_j, sc.wavelength_m, sc.energy_j
    );
    for (n, radius, age_us, s) in reference {
        let scale = big.wavelength_m / sc.wavelength_m;
        let r2 = radius * scale;
        let a2 = (age_us as f64 * (scale as f64).sqrt()).round() as u64;
        let s2 = match n {
            256 => {
                let f = RadialImpact::<256>::new(big.event, big.medium, Domain { radius: r2, age_us: a2 })
                    .map_err(|e| format!("N256 {e:?}"))?;
                seams(&f, r2, a2)?
            }
            _ => {
                let f = RadialImpact::<512>::new(big.event, big.medium, Domain { radius: r2, age_us: a2 })
                    .map_err(|e| format!("N512 {e:?}"))?;
                seams(&f, r2, a2)?
            }
        };
        let rel = |x: f64, c: f64| 100.0 * x / c;
        let d_edge = rel(s2.edge_eta, s2.eta_centre) - rel(s.edge_eta, s.eta_centre);
        let d_hor = rel(s2.horizon_eta, s2.eta_centre) - rel(s.horizon_eta, s.eta_centre);
        println!(
            "N={n} R={radius}->{r2:.2}m A={:.3}->{:.3}s bord {:.4}%->{:.4}% horizon {:.4}%->{:.4}% eta0 {:.5}->{:.5}m homothetie={}",
            age_us as f64 / 1e6,
            a2 as f64 / 1e6,
            rel(s.edge_eta, s.eta_centre),
            rel(s2.edge_eta, s2.eta_centre),
            rel(s.horizon_eta, s.eta_centre),
            rel(s2.horizon_eta, s2.eta_centre),
            s.eta_centre,
            s2.eta_centre,
            d_edge.abs() < 0.1 && d_hor.abs() < 0.1
        );
    }
    Ok(())
}

#[derive(Default, Debug)]
pub struct RenderStats {
    pub water: u64,
    pub unresolved: u64,
    pub refused: u64,
    pub evaluations: u64,
    pub w_evaluations: u64,
    pub max_residual: f64,
    pub touched_pixels: u64,
    pub render_ms: f64,
}

/// Rend l'observateur S201 à `naissance + age`. Dans l'emprise, B+W par le chemin hôte
/// `Prepared::sample_world_batch` ; hors emprise, B seul. `with_w = false` rend le **témoin** :
/// même marche, mêmes bornes, même prédicat d'emprise, mais B partout. Hors emprise, les deux
/// rendus exécutent donc exactement les mêmes opérations.
/// Chemin d'évaluation de W dans l'emprise.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPath {
    /// Témoin : B partout, même marche et même prédicat.
    Temoin,
    /// Composition directe par `Prepared::sample_world_batch` (S203, S205).
    Direct,
    /// S208, ADR-129 : table de Bessel au pas λ/diviseur, B + W composés par l'hôte.
    Table(f32),
}
pub fn render(sc: &Scene, age_us: u64, with_w: bool) -> Result<(Vec<u8>, Vec<bool>, RenderStats), String> {
    render_path(sc, age_us, if with_w { WPath::Direct } else { WPath::Temoin })
}
pub fn render_path(sc: &Scene, age_us: u64, path: WPath) -> Result<(Vec<u8>, Vec<bool>, RenderStats), String> {
    let with_w = path != WPath::Temoin;
    if age_us > EMPRISE_AGE_US {
        return Err("instant hors de l'horizon de l'emprise".into());
    }
    let domain = Domain {
        radius: EMPRISE_RADIUS_M,
        age_us: EMPRISE_AGE_US,
    };
    let field = RadialImpact::<EMPRISE_N>::new(sc.event, sc.medium, domain).map_err(|e| format!("champ {e:?}"))?;
    let mut slots = vec![None; 1];
    let mut journal = Journal::new(0, &mut slots);
    journal
        .confirm(
            0,
            Cause {
                entity: 0,
                command: 0,
                emission: 0,
            },
            sc.event,
        )
        .map_err(|e| format!("journal {e:?}"))?;
    let mut pool: Vec<Option<RadialImpact<EMPRISE_N>>> = (0..1).map(|_| None).collect();
    let context = Context {
        frame: FRAME,
        cell: CELL,
        medium: sc.medium,
        domain,
    };
    let prepared = Prepared::build(&journal, &mut pool, context).map_err(|e| format!("préparation {e:?}"))?;
    let bound = BoundBackground::new(&sc.background, FRAME, CELL);
    let time = SimTime(BIRTH.0 + age_us);
    // S208 : table et profil construits avant l'image, jamais dans la marche.
    let divisor = match path {
        WPath::Table(d) => d,
        _ => 16.0,
    };
    let step = sc.wavelength_m / divisor;
    let len = field.table_len(step).map_err(|e| format!("table {e:?}"))?;
    let mut storage = vec![[0.0f32; 2]; EMPRISE_N * len];
    let table = field.bake_table(step, &mut storage).map_err(|e| format!("table {e:?}"))?;
    let mut profile = vec![(0.0f32, 0.0f32); len];
    table.profile(time, &mut profile).map_err(|e| format!("profil {e:?}"))?;
    // Bornes de marche B+W dans les deux rendus : c'est ce qui rend le témoin comparable.
    let height = height_bound(&sc.cooked) + radial_height_bound(&field)?;
    let slope = sc
        .cooked
        .components()
        .iter()
        .map(|c| c.amplitude.abs() as f64 * c.k_turns_per_m as f64 * std::f64::consts::TAU)
        .sum::<f64>()
        + field.slope_bound() as f64;
    let camera = Camera::new(CAMERA_ORIGIN, CAMERA_TARGET);
    let mut stats = RenderStats::default();
    let mut rgb = Vec::with_capacity(WIDTH * HEIGHT * 3);
    let mut touched_map = Vec::with_capacity(WIDTH * HEIGHT);
    let mut out = [WaterSample::default()];
    let mut scratch = [WaterSample::default()];
    let start = Instant::now();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut color = [0.; 3];
            let mut touched = false;
            for (ox, oy) in [(0.25, 0.25), (0.75, 0.75)] {
                let dir = camera.ray(x as f64 + ox, y as f64 + oy, WIDTH, HEIGHT);
                let hit = trace(camera.origin, dir, height, slope, |p| {
                    stats.evaluations += 1;
                    let world = WorldPos::from_metres(p[0], p[1], 0.);
                    let local = sc.background.local_point(world).expect("rayon dans le référentiel");
                    let inside = field.admits(FRAME, CELL, [local[0], local[1]]);
                    touched |= inside;
                    if with_w && inside && matches!(path, WPath::Table(_)) {
                        stats.w_evaluations += 1;
                        let b = sc.background.eval(world, time).expect("rayon dans le domaine de B");
                        match table.eval(&profile, FRAME, CELL, [local[0], local[1]]) {
                            Ok((eta, w)) => {
                                // Même normalisation que `compose` : pentes sommées, puis normale.
                                let sx = -b.normal[0] / b.normal[2] + w[0];
                                let sy = -b.normal[1] / b.normal[2] + w[1];
                                let norm = (1.0 + sx * sx + sy * sy).sqrt();
                                ((b.eta + eta) as f64, [-sx / norm, -sy / norm, 1.0 / norm].map(|v| v as f64))
                            }
                            Err(_) => {
                                stats.refused += 1;
                                (p[2] + 1.0, [0., 0., 1.])
                            }
                        }
                    } else if with_w && inside {
                        stats.w_evaluations += 1;
                        match prepared.sample_world_batch(&bound, &[world], time, BREAKING_SLOPE, &mut out, &mut scratch) {
                            Ok(_) => (out[0].eta as f64, out[0].normal.map(|v| v as f64)),
                            Err(_) => {
                                // Refus de composition : le rayon est déclaré non résolu, jamais
                                // remplacé par B en silence.
                                stats.refused += 1;
                                (p[2] + 1.0, [0., 0., 1.])
                            }
                        }
                    } else {
                        let s = sc.background.eval(world, time).expect("rayon dans le domaine de B");
                        (s.eta as f64, s.normal.map(|v| v as f64))
                    }
                });
                let c = match hit {
                    Trace::Water {
                        distance,
                        normal,
                        residual,
                    } => {
                        stats.water += 1;
                        stats.max_residual = stats.max_residual.max(residual);
                        shade(dir, normal, distance)
                    }
                    Trace::Sky => sky(dir),
                    Trace::Unresolved => {
                        stats.unresolved += 1;
                        [1., 0., 1.]
                    }
                };
                color = add(color, mul(c, 0.5));
            }
            stats.touched_pixels += touched as u64;
            touched_map.push(touched);
            rgb.extend(color.map(byte));
        }
    }
    stats.render_ms = start.elapsed().as_secs_f64() * 1000.;
    Ok((rgb, touched_map, stats))
}

fn write_ppm(path: &std::path::Path, rgb: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut file = BufWriter::new(File::create(path).map_err(|e| e.to_string())?);
    ppm(&mut file, WIDTH, HEIGHT, rgb).map_err(|e| e.to_string())?;
    file.flush().map_err(|e| e.to_string())
}

/// Pixels différents du témoin dont aucun échantillon n'est tombé dans l'emprise : doit valoir 0.
pub fn differing_outside(a: &[u8], b: &[u8], touched: &[bool]) -> (u64, u64) {
    let mut differing = 0;
    let mut outside = 0;
    for (i, t) in touched.iter().enumerate() {
        if a[3 * i..3 * i + 3] != b[3 * i..3 * i + 3] {
            differing += 1;
            outside += !*t as u64;
        }
    }
    (differing, outside)
}

/// Lecture d'un PPM P6 écrit par ce banc ; refuse tout autre format.
fn read_ppm(path: &std::path::Path) -> Result<Vec<u8>, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let header = format!("P6\n{WIDTH} {HEIGHT}\n255\n");
    if !bytes.starts_with(header.as_bytes()) || bytes.len() != header.len() + WIDTH * HEIGHT * 3 {
        return Err(format!("{} : PPM inattendu", path.display()));
    }
    Ok(bytes[header.len()..].to_vec())
}

/// S208, critère (e) : image par la table contre l'image directe S205 du même instant.
fn render_table_report(dir: &str, age_s: f64, hs: f32, divisor: f32) -> Result<(), String> {
    let age_us = (age_s * 1e6).round() as u64;
    let sc = scene_with(hs, EMPRISE_AGE_US, ENTRY_HALF_WIDTH_M, SlopeRule::PerturbationsAdr128)?;
    let base = std::path::Path::new(dir);
    let tag = format!("{:05.1}", age_s).replace('.', "_");
    let hs_tag = hs.to_string().replace('.', "_");
    let direct_path = base.join(format!("impact-s205-hs{hs_tag}-a{tag}.ppm"));
    let direct = read_ppm(&direct_path).map_err(|e| format!("{e} — rendre d'abord render-s205"))?;
    let (rgb, touched, st) = render_path(&sc, age_us, WPath::Table(divisor))?;
    write_ppm(&base.join(format!("impact-s208-table{divisor}-hs{hs_tag}-a{tag}.ppm")), &rgb)?;
    let (differing, outside) = differing_outside(&rgb, &direct, &touched);
    let max_channel = rgb.iter().zip(&direct).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
    let over2 = rgb
        .chunks(3)
        .zip(direct.chunks(3))
        .filter(|(a, b)| a.iter().zip(b.iter()).any(|(x, y)| x.abs_diff(*y) > 2))
        .count();
    println!(
        "table λ/{divisor} age={age_s}s hs={hs} water={} unresolved={} refused={} evals={} w_evals={} render_ms={:.3} rgb_fnv=0x{:016x} direct_fnv=0x{:016x}",
        st.water, st.unresolved, st.refused, st.evaluations, st.w_evaluations, st.render_ms, fnv(&rgb), fnv(&direct)
    );
    println!("pixels_differents_du_direct={differing} dont_hors_emprise={outside} ecart_max_canal={max_channel} pixels_ecart_sup_2={over2}");
    if st.unresolved + st.refused > 0 || outside > 0 {
        return Err("réception refusée : rayon non résolu, refus, ou différence hors emprise".into());
    }
    Ok(())
}

fn render_report(dir: &str, age_s: f64, hs: f32, rule: SlopeRule) -> Result<(), String> {
    if !age_s.is_finite() || age_s < 0.0 {
        return Err("âge invalide".into());
    }
    let age_us = (age_s * 1e6).round() as u64;
    let sc = scene_with(hs, EMPRISE_AGE_US, ENTRY_HALF_WIDTH_M, rule)?;
    let (impact, touched, s) = render(&sc, age_us, true)?;
    let (temoin, touched_t, st) = render(&sc, age_us, false)?;
    if touched != touched_t {
        return Err("prédicat d'emprise différent entre rendu et témoin".into());
    }
    let (differing, outside) = differing_outside(&impact, &temoin, &touched);
    let base = std::path::Path::new(dir);
    let tag = format!("{:05.1}", age_s).replace('.', "_");
    let prefix = match rule {
        SlopeRule::MinusBackgroundS203 => "s203".to_string(),
        SlopeRule::PerturbationsAdr128 => format!("s205-hs{}", hs.to_string().replace('.', "_")),
    };
    write_ppm(&base.join(format!("impact-{prefix}-a{tag}.ppm")), &impact)?;
    write_ppm(&base.join(format!("temoin-{prefix}-a{tag}.ppm")), &temoin)?;
    println!(
        "age={age_s}s t={:.3}s hs={hs} regle={rule:?} budget_impact={:.6} plancher_B={:.6} N={EMPRISE_N} R={EMPRISE_RADIUS_M}m A={}s lambda={:.3}m E={:.1}J",
        (BIRTH.0 + age_us) as f64 / 1e6,
        sc.medium.max_slope,
        sc.floor,
        EMPRISE_AGE_US / 1_000_000,
        sc.wavelength_m,
        sc.energy_j
    );
    for (name, rgb, st) in [("impact", &impact, &s), ("temoin", &temoin, &st)] {
        println!(
            "{name} water={} unresolved={} refused={} evals={} w_evals={} touched_px={} residual_max={:.6}m render_ms={:.3} rgb_fnv=0x{:016x}",
            st.water, st.unresolved, st.refused, st.evaluations, st.w_evaluations, st.touched_pixels, st.max_residual, st.render_ms, fnv(rgb)
        );
    }
    println!("pixels_differents={differing} dont_hors_emprise={outside}");
    if s.unresolved + st.unresolved + s.refused > 0 || outside > 0 {
        return Err("réception image refusée : rayon non résolu, refus de composition ou pixel hors emprise".into());
    }
    Ok(())
}

/// Projection d'un point monde sur l'image de l'observateur, en pixels (même convention que
/// `Camera::ray`). `None` derrière la caméra.
pub fn project(camera: &Camera, p: V) -> Option<[f64; 2]> {
    let d = add(p, mul(camera.origin, -1.));
    let z = dot(d, camera.forward);
    if z <= 0. {
        return None;
    }
    let half = (FIELD_OF_VIEW_DEG.to_radians() / 2.).tan();
    let aspect = WIDTH as f64 / HEIGHT as f64;
    let x = (dot(d, camera.right) / z / (aspect * half) + 1.) * WIDTH as f64 / 2.;
    let y = (1. - dot(d, camera.up) / z / half) * HEIGHT as f64 / 2.;
    Some([x, y])
}

/// Longueur apparente en pixels d'un segment horizontal centré en `centre` : orienté vers
/// l'observateur (le long de la visée), puis perpendiculairement (en travers).
pub fn apparent_pixels(camera: &Camera, centre: [f64; 2], length: f64) -> Option<(f64, f64)> {
    let to_camera = unit([camera.origin[0] - centre[0], camera.origin[1] - centre[1], 0.]);
    let across = [-to_camera[1], to_camera[0], 0.];
    let c = [centre[0], centre[1], 0.];
    let span = |axis: V| -> Option<f64> {
        let a = project(camera, add(c, mul(axis, length / 2.)))?;
        let b = project(camera, add(c, mul(axis, -length / 2.)))?;
        Some((a[0] - b[0]).hypot(a[1] - b[1]))
    };
    Some((span(to_camera)?, span(across)?))
}

fn observer_report() -> Result<(), String> {
    let sc = scene(HS, EMPRISE_AGE_US)?;
    let camera = Camera::new(CAMERA_ORIGIN, CAMERA_TARGET);
    let centre = [IMPACT_XY[0] as f64, IMPACT_XY[1] as f64];
    let lambda = sc.wavelength_m as f64;
    let p = project(&camera, [centre[0], centre[1], 0.]).ok_or("impact derrière la caméra")?;
    let dist = ((centre[0] - CAMERA_ORIGIN[0]).powi(2) + (centre[1] - CAMERA_ORIGIN[1]).powi(2) + CAMERA_ORIGIN[2].powi(2)).sqrt();
    println!("# observateur S203 : camera {CAMERA_ORIGIN:?} -> {CAMERA_TARGET:?}, {WIDTH}x{HEIGHT}, champ vertical {FIELD_OF_VIEW_DEG} deg");
    println!("impact pixel=({:.1},{:.1}) distance={dist:.2}m", p[0], p[1]);
    for (name, l) in [("lambda", lambda), ("lambda/2", lambda / 2.)] {
        let (along, across) = apparent_pixels(&camera, centre, l).ok_or("projection")?;
        println!("{name}={l:.4}m au point d'impact : le_long={along:.2}px en_travers={across:.2}px");
    }
    // Même azimut que l'impact, distance horizontale croissante depuis la caméra.
    let azimuth = unit([centre[0] - CAMERA_ORIGIN[0], centre[1] - CAMERA_ORIGIN[1], 0.]);
    for (name, l) in [("lambda", lambda), ("lambda/2", lambda / 2.)] {
        let mut along_limit = None;
        let mut across_limit = None;
        let mut d = 5.0;
        while d <= 600.0 {
            let c = [CAMERA_ORIGIN[0] + azimuth[0] * d, CAMERA_ORIGIN[1] + azimuth[1] * d];
            if let Some((along, across)) = apparent_pixels(&camera, c, l) {
                if along_limit.is_none() && along < 2.0 {
                    along_limit = Some(d);
                }
                if across_limit.is_none() && across < 2.0 {
                    across_limit = Some(d);
                }
            }
            d += 0.5;
        }
        println!("{name} sous 2 px : le_long a partir de {along_limit:?} m, en_travers a partir de {across_limit:?} m (horizontal depuis la camera, marche limitee a 600 m)");
    }
    // L'emprise se projette sans borne quand l'observateur est dedans en plan : on le dit
    // plutôt que de publier une boîte englobante de points derrière le plan image.
    let horizontal = (centre[0] - CAMERA_ORIGIN[0]).hypot(centre[1] - CAMERA_ORIGIN[1]);
    println!(
        "emprise R={EMPRISE_RADIUS_M} m, observateur a {horizontal:.2} m du centre en plan : observateur_dans_emprise={}",
        horizontal < EMPRISE_RADIUS_M as f64
    );
    // Part non portée par W — grandeurs dérivées, provenance citée.
    let v = ENTRY_SPEED_MS as f64;
    let b = ENTRY_HALF_WIDTH_M as f64;
    let g = sc.medium.gravity as f64;
    let e_ref = impact_generator::reference_energy_j(
        &Entry { half_width_m: ENTRY_HALF_WIDTH_M, speed_ms: ENTRY_SPEED_MS, transferred_fraction: TRANSFERRED_FRACTION },
        &sc.medium,
    ) as f64;
    println!("# part non portee par W");
    println!("Froude d'entree v/sqrt(g*2b)={:.3} (cavite franche au-dela de ~5, SPEC-002 §3)", v / (g * 2. * b).sqrt());
    println!("energie hors ondes (1-f)*E_ref={:.0}J sur E_ref={e_ref:.0}J (f a calibrer B2)", (1. - TRANSFERRED_FRACTION as f64) * e_ref);
    println!("majorant balistique d'une projection a v : hauteur v^2/2g={:.3}m, duree 2v/g={:.3}s", v * v / (2. * g), 2. * v / g);
    println!("demi-largeur mouillee b={b}m = 0,2985 lambda (impact_generator::ALPHA)");
    Ok(())
}

/// Table radiale η(r), η'(r) d'un champ à un instant : piste hôte pour le coût par point.
pub fn radial_table<const N: usize>(field: &RadialImpact<N>, time: SimTime, step: f32, table: &mut Vec<(f32, f32)>) -> Result<(), String> {
    let v = *field.event().data();
    table.clear();
    let count = (EMPRISE_RADIUS_M / step) as usize + 1;
    for i in 0..count {
        let r = (i as f32 * step).min(EMPRISE_RADIUS_M - 1e-3);
        let s = field.sample(v.frame, v.cell, [v.position[0] + r, v.position[1]], time).map_err(|e| format!("{e:?}"))?;
        table.push((s.eta, s.slope[0]));
    }
    Ok(())
}
/// Hermite cubique sur la table ; au-delà du dernier nœud, refus (hors emprise).
pub fn table_eta(table: &[(f32, f32)], step: f32, r: f32) -> Option<f32> {
    let x = r / step;
    let i = x as usize;
    if i + 1 >= table.len() {
        return None;
    }
    let t = x - i as f32;
    let (y0, d0) = table[i];
    let (y1, d1) = table[i + 1];
    let (t2, t3) = (t * t, t * t * t);
    Some((2. * t3 - 3. * t2 + 1.) * y0 + (t3 - 2. * t2 + t) * step * d0 + (-2. * t3 + 3. * t2) * y1 + (t3 - t2) * step * d1)
}

fn timed(mut action: impl FnMut(), repeats: usize) -> (f64, f64) {
    for _ in 0..3 {
        action();
    }
    let mut v = Vec::with_capacity(repeats);
    for _ in 0..repeats {
        let t = Instant::now();
        action();
        v.push(t.elapsed().as_secs_f64() * 1e6);
    }
    v.sort_by(f64::total_cmp);
    (v[repeats / 2], v[repeats - 1])
}

fn cost_report() -> Result<(), String> {
    use std::hint::black_box;
    let sc = scene(HS, EMPRISE_AGE_US)?;
    let domain = Domain { radius: EMPRISE_RADIUS_M, age_us: EMPRISE_AGE_US };
    let field = RadialImpact::<EMPRISE_N>::new(sc.event, sc.medium, domain).map_err(|e| format!("{e:?}"))?;
    let mut slots = vec![None; 1];
    let mut journal = Journal::new(0, &mut slots);
    journal.confirm(0, Cause { entity: 0, command: 0, emission: 0 }, sc.event).map_err(|e| format!("{e:?}"))?;
    let mut pool: Vec<Option<RadialImpact<EMPRISE_N>>> = (0..1).map(|_| None).collect();
    let prepared = Prepared::build(&journal, &mut pool, Context { frame: FRAME, cell: CELL, medium: sc.medium, domain })
        .map_err(|e| format!("{e:?}"))?;
    let bound = BoundBackground::new(&sc.background, FRAME, CELL);
    // 4096 points déterministes dans le disque de rayon 0,99 R (spirale de Vogel).
    let n = 4096usize;
    let golden = std::f64::consts::PI * (3. - 5f64.sqrt());
    let local: Vec<[f32; 2]> = (0..n)
        .map(|i| {
            let r = 0.99 * EMPRISE_RADIUS_M as f64 * ((i as f64 + 0.5) / n as f64).sqrt();
            let a = i as f64 * golden;
            [IMPACT_XY[0] + (r * a.cos()) as f32, IMPACT_XY[1] + (r * a.sin()) as f32]
        })
        .collect();
    let world: Vec<WorldPos> = local.iter().map(|p| WorldPos::from_metres(p[0] as f64, p[1] as f64, 0.)).collect();
    let mut out = vec![WaterSample::default(); n];
    let mut scratch = out.clone();
    println!("# cout par point S203 : {n} points dans R, 3 chauffes / 11 mesures, release, un fil");
    println!("age_s B_med_us B_max_us W_med_us W_max_us BW_med_us BW_max_us BW_us_par_point points_BW_dans_2ms table_build_med_us table_M table_eval_med_us table_us_par_point max_err_table_mm");
    let step = sc.wavelength_m / 2. / 8.;
    let mut table = Vec::with_capacity((EMPRISE_RADIUS_M / step) as usize + 2);
    for age_s in [1u64, 3, 6, 30] {
        let t = SimTime(BIRTH.0 + age_s * 1_000_000);
        let b = timed(|| { for p in &world { black_box(sc.background.eval(*p, t)); } }, 11);
        let w = timed(|| { for p in &local { black_box(field.sample(FRAME, CELL, *p, t).ok()); } }, 11);
        let bw = timed(|| { black_box(prepared.sample_world_batch(&bound, &world, t, BREAKING_SLOPE, &mut out, &mut scratch).unwrap()); }, 11);
        let build = timed(|| { radial_table(&field, t, step, &mut table).unwrap(); black_box(&table); }, 11);
        radial_table(&field, t, step, &mut table)?;
        let eval = timed(|| {
            for p in &local {
                let r = (p[0] - IMPACT_XY[0]).hypot(p[1] - IMPACT_XY[1]);
                black_box(table_eta(&table, step, r));
            }
        }, 11);
        let mut max_err = 0.0f64;
        for i in 0..20_000u32 {
            let r = (i as f32 + 0.37) / 20_000. * (EMPRISE_RADIUS_M - step - 1e-3);
            let direct = field.sample(FRAME, CELL, [IMPACT_XY[0] + r, IMPACT_XY[1]], t).map_err(|e| format!("{e:?}"))?;
            let interp = table_eta(&table, step, r).ok_or("table")?;
            max_err = max_err.max((direct.eta as f64 - interp as f64).abs());
        }
        let per_point = bw.0 / n as f64;
        println!(
            "{age_s} {:.1} {:.1} {:.1} {:.1} {:.1} {:.1} {:.3} {:.0} {:.1} {} {:.1} {:.4} {:.4}",
            b.0, b.1, w.0, w.1, bw.0, bw.1, per_point, 2000. / per_point, build.0, table.len(), eval.0, eval.0 / n as f64, max_err * 1e3
        );
    }
    Ok(())
}

fn scene_report() -> Result<(), String> {
    println!("# constats de construction S203");
    for hs in [0.25f32, 0.5, 1.0, 1.5] {
        let cooked = background_spectrum::bake(recipe(hs)).map_err(|e| format!("{e:?}"))?;
        println!(
            "hs={hs} recipe=0x{:016x} floor_L1={:.6} directionnel={:.6} pi/7={:.6} marge={:.6} hauteur_L1={:.4}m",
            cooked.hash(),
            slope_floor(&cooked),
            directional_slope_bound(&cooked),
            BREAKING_SLOPE,
            BREAKING_SLOPE - slope_floor(&cooked),
            height_bound(&cooked)
        );
    }
    let sc = scene(HS, 60_000_000)?;
    println!(
        "scene hs={HS} floor={:.6} budget_impact={:.6} lambda={:.4}m E={:.3}J fraction={} fraction_max={:.6} E_ref={:.1}J",
        sc.floor,
        sc.medium.max_slope,
        sc.wavelength_m,
        sc.energy_j,
        TRANSFERRED_FRACTION,
        sc.max_fraction,
        impact_generator::reference_energy_j(
            &Entry {
                half_width_m: ENTRY_HALF_WIDTH_M,
                speed_ms: ENTRY_SPEED_MS,
                transferred_fraction: TRANSFERRED_FRACTION
            },
            &sc.medium
        )
    );
    for (n, radius) in [(64usize, 8.0f32), (128, 8.0), (256, 8.0)] {
        let domain = Domain {
            radius,
            age_us: 4_000_000,
        };
        let line = match n {
            64 => RadialImpact::<64>::new(sc.event, sc.medium, domain)
                .map(|f| (f.slope_max(), f.slope_bound(), radial_height_bound(&f))),
            128 => RadialImpact::<128>::new(sc.event, sc.medium, domain)
                .map(|f| (f.slope_max(), f.slope_bound(), radial_height_bound(&f))),
            _ => RadialImpact::<256>::new(sc.event, sc.medium, domain)
                .map(|f| (f.slope_max(), f.slope_bound(), radial_height_bound(&f))),
        };
        match line {
            Ok((slope_max, bound, h)) => println!(
                "N={n} slope_max={slope_max:.6} slope_L1={bound:.6} floor+slope_max={:.6} <= pi/7 {} eta_centre={:.5}m",
                sc.floor + slope_max,
                sc.floor + slope_max <= BREAKING_SLOPE,
                h?
            ),
            Err(e) => println!("N={n} refus {e:?}"),
        }
    }
    for hs in [0.5f32, 1.5] {
        let sc = match scene(hs, 60_000_000) {
            Ok(sc) => sc,
            Err(e) => {
                // La scène refuse à 1,5 m ; B seul se construit pour la mesure de pente.
                println!("hs={hs} scene refusee : {e}");
                let cooked = background_spectrum::bake(recipe(hs)).map_err(|e| format!("{e:?}"))?;
                let floor = slope_floor(&cooked);
                let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 22);
                let jobs = host_impl::SequentialJobs;
                let sink = host_impl::StderrSink;
                let bg = Background::from_spectrum(
                    &mut HostServices {
                        alloc: &mut alloc,
                        jobs: &jobs,
                        sink: &sink,
                    },
                    &cooked,
                    WorldPos::from_units(0, 0, 0),
                )
                .map_err(|e| format!("{e:?}"))?;
                let real = sampled_real_slope(&bg, 256.0, 1.0, 120);
                println!(
                    "hs={hs} pente_reelle_echantillonnee={real:.6} floor_L1={floor:.6} rapport_L1/echantillon={:.4} (512x512 m pas 1 m, t=0..120 s)",
                    floor as f64 / real
                );
                continue;
            }
        };
        let real = sampled_real_slope(&sc.background, 256.0, 1.0, 120);
        println!(
            "hs={hs} pente_reelle_echantillonnee={real:.6} floor_L1={:.6} rapport_L1/echantillon={:.4} (512x512 m pas 1 m, t=0..120 s)",
            sc.floor,
            sc.floor as f64 / real
        );
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("scene") => scene_report()?,
        Some("seams") => seams_report()?,
        Some("controls") => controls_report()?,
        Some("observer") => observer_report()?,
        Some("cost") => cost_report()?,
        Some("render") => {
            let dir = args.get(2).map(String::as_str).unwrap_or("../captures");
            let age: f64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(3.0);
            render_report(dir, age, HS, SlopeRule::MinusBackgroundS203)?
        }
        Some("render-table") => {
            let dir = args.get(2).map(String::as_str).unwrap_or("../captures");
            let age: f64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(3.0);
            let hs: f32 = args.get(4).map(|s| s.parse()).transpose()?.unwrap_or(1.5);
            let divisor: f32 = args.get(5).map(|s| s.parse()).transpose()?.unwrap_or(16.0);
            render_table_report(dir, age, hs, divisor)?
        }
        Some("render-s205") => {
            let dir = args.get(2).map(String::as_str).unwrap_or("../captures");
            let age: f64 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(3.0);
            let hs: f32 = args.get(4).map(|s| s.parse()).transpose()?.unwrap_or(1.5);
            render_report(dir, age, hs, SlopeRule::PerturbationsAdr128)?
        }
        _ => return Err("mode : scene | seams | controls | observer | cost | render <dir> <age_s> | render-s205 <dir> <age_s> [hs]".into()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn floor_matches_the_composition_bound() {
        // Le plancher de l'hôte est celui que `compose` forme : même somme, même ordre.
        let sc = scene(HS, 10_000_000).unwrap();
        let s = sc
            .background
            .eval(WorldPos::from_metres(3.0, -2.0, 0.0), SimTime(5_000_000))
            .unwrap();
        assert_eq!((s.steepness * core::f32::consts::PI).to_bits(), sc.floor.to_bits());
        assert!(scene(1.5, 10_000_000).is_err());
    }
    #[test]
    fn adr128_rule_composes_the_reference_sea_s205() {
        // S205 : la règle S203 refuse la mer S201 ; celle d'ADR-128 la compose, impact compris.
        let sc = scene_s205(1.5, EMPRISE_AGE_US).unwrap();
        assert!(sc.floor > BREAKING_SLOPE);
        assert_eq!(sc.medium.max_slope.to_bits(), BREAKING_SLOPE.to_bits());
        let domain = Domain { radius: EMPRISE_RADIUS_M, age_us: EMPRISE_AGE_US };
        let mut slots = vec![None; 1];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, Cause { entity: 0, command: 0, emission: 0 }, sc.event).unwrap();
        let mut pool: Vec<Option<RadialImpact<EMPRISE_N>>> = (0..1).map(|_| None).collect();
        let prepared = Prepared::build(&journal, &mut pool, Context { frame: FRAME, cell: CELL, medium: sc.medium, domain }).unwrap();
        let bound = BoundBackground::new(&sc.background, FRAME, CELL);
        let (mut out, mut scratch) = ([WaterSample::default()], [WaterSample::default()]);
        let p = WorldPos::from_metres(IMPACT_XY[0] as f64 + 0.5, IMPACT_XY[1] as f64, 0.);
        prepared
            .sample_world_batch(&bound, &[p], SimTime(BIRTH.0 + 3_000_000), BREAKING_SLOPE, &mut out, &mut scratch)
            .unwrap();
        // Même champ W que S203 : l'énergie et la longueur d'onde ne dépendent pas de la règle.
        let s203 = scene(HS, EMPRISE_AGE_US).unwrap();
        assert_eq!(sc.energy_j.to_bits(), s203.energy_j.to_bits());
        assert_eq!(sc.wavelength_m.to_bits(), s203.wavelength_m.to_bits());
    }
    #[test]
    fn directional_bound_sits_between_sample_and_l1() {
        let sc = scene(HS, 10_000_000).unwrap();
        let d = directional_slope_bound(&sc.cooked);
        let sampled = sampled_real_slope(&sc.background, 16.0, 0.5, 4);
        assert!(sampled <= d, "{sampled} > {d}");
        assert!(d < sc.floor as f64, "{d} >= {}", sc.floor);
    }
    #[test]
    fn projection_agrees_with_the_camera_rays() {
        // Un point de la surface plane touché par le rayon du pixel (x, y) se projette sur (x, y).
        let camera = Camera::new(CAMERA_ORIGIN, CAMERA_TARGET);
        for (x, y) in [(320.5, 250.5), (10.25, 359.75), (600.0, 200.0)] {
            let dir = camera.ray(x, y, WIDTH, HEIGHT);
            let s = -camera.origin[2] / dir[2];
            let p = project(&camera, add(camera.origin, mul(dir, s))).unwrap();
            assert!((p[0] - x).abs() < 1e-9 && (p[1] - y).abs() < 1e-9, "{p:?}");
        }
    }
    #[test]
    fn hermite_table_is_exact_on_nodes_and_refuses_outside() {
        let table = vec![(0.0f32, 1.0f32), (1.0, 0.0), (0.5, -1.0)];
        assert_eq!(table_eta(&table, 0.5, 0.0), Some(0.0));
        assert_eq!(table_eta(&table, 0.5, 0.5), Some(1.0));
        assert!(table_eta(&table, 0.5, 1.0).is_none());
    }
    #[test]
    fn a_differing_pixel_outside_the_footprint_is_counted() {
        let a = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut b = a;
        b[4] = 0; // pixel 1, dans l'emprise
        b[8] = 0; // pixel 2, hors emprise
        assert_eq!(differing_outside(&a, &b, &[false, true, false]), (2, 1));
        assert_eq!(differing_outside(&a, &a, &[false, false, false]), (0, 0));
    }
    #[test]
    fn composed_sample_inside_footprint_differs_from_b_and_matches_outside() {
        // Le chemin hôte rend B+W dans l'emprise ; le prédicat `admits` borne exactement où.
        let sc = scene(HS, EMPRISE_AGE_US).unwrap();
        let domain = Domain { radius: EMPRISE_RADIUS_M, age_us: EMPRISE_AGE_US };
        let field = RadialImpact::<EMPRISE_N>::new(sc.event, sc.medium, domain).unwrap();
        let mut slots = vec![None; 1];
        let mut journal = Journal::new(0, &mut slots);
        journal.confirm(0, Cause { entity: 0, command: 0, emission: 0 }, sc.event).unwrap();
        let mut pool: Vec<Option<RadialImpact<EMPRISE_N>>> = (0..1).map(|_| None).collect();
        let prepared = Prepared::build(&journal, &mut pool, Context { frame: FRAME, cell: CELL, medium: sc.medium, domain }).unwrap();
        let bound = BoundBackground::new(&sc.background, FRAME, CELL);
        let t = SimTime(BIRTH.0 + 1_000_000);
        let (mut out, mut scratch) = ([WaterSample::default()], [WaterSample::default()]);
        let inside = WorldPos::from_metres(IMPACT_XY[0] as f64 + 0.5, IMPACT_XY[1] as f64, 0.);
        prepared.sample_world_batch(&bound, &[inside], t, BREAKING_SLOPE, &mut out, &mut scratch).unwrap();
        let b = sc.background.eval(inside, t).unwrap();
        let w = field.sample(FRAME, CELL, [IMPACT_XY[0] + 0.5, IMPACT_XY[1]], t).unwrap();
        assert_eq!(out[0].eta.to_bits(), (b.eta + w.eta).to_bits());
        assert!(w.eta.abs() > 1e-3);
        let outside = WorldPos::from_metres(IMPACT_XY[0] as f64 + 53.0, IMPACT_XY[1] as f64, 0.);
        assert!(!field.admits(FRAME, CELL, [IMPACT_XY[0] + 53.0, IMPACT_XY[1]]));
        assert!(prepared.sample_world_batch(&bound, &[outside], t, BREAKING_SLOPE, &mut out, &mut scratch).is_err());
    }
    #[test]
    fn seams_see_a_truncated_field_and_the_radius_is_maximal() {
        let sc = scene(HS, 60_000_000).unwrap();
        // Emprise volontairement courte : la couture doit être vue, pas lissée.
        let f = RadialImpact::<64>::new(
            sc.event,
            sc.medium,
            Domain {
                radius: 2.0,
                age_us: 2_000_000,
            },
        )
        .unwrap();
        let s = seams(&f, 2.0, 2_000_000).unwrap();
        assert!(!s.passes(), "{s:?}");
        assert!(s.edge_eta > RAY_TOLERANCE_M);
        // Le rayon rendu est le plus grand au pas de 0,5 m : le suivant est refusé.
        let (radius, _) = largest_radius::<64>(&sc, 8_000_000).unwrap();
        let next = RadialImpact::<64>::new(
            sc.event,
            sc.medium,
            Domain {
                radius: radius + 0.5,
                age_us: 8_000_000,
            },
        );
        assert!(next.is_err());
    }
    #[test]
    fn radial_height_bound_is_the_centre_at_birth() {
        let sc = scene(HS, 10_000_000).unwrap();
        let f = RadialImpact::<64>::new(
            sc.event,
            sc.medium,
            Domain {
                radius: 8.0,
                age_us: 4_000_000,
            },
        )
        .unwrap();
        let h = radial_height_bound(&f).unwrap();
        assert!(h > 0.0);
        // Aucun point échantillonné ne dépasse la borne, à aucun instant.
        for step in 0..40 {
            let t = SimTime(BIRTH.0 + step * 100_000);
            for i in 0..80 {
                let p = [IMPACT_XY[0] + i as f32 * 0.1, IMPACT_XY[1]];
                let s = f.sample(FRAME, CELL, p, t).unwrap();
                assert!((s.eta as f64).abs() <= h * (1.0 + 1e-5), "{} > {h}", s.eta);
            }
        }
    }
}
