//! S203 : un impact porté par W, rendu sur B, avec emprise et observateur explicites.
//! CPU, sans dépendance, image locale seulement (ADR-124). Rien n'est ajouté à la bibliothèque :
//! l'hôte déclare la scène, la bibliothèque compose B+W dans l'emprise (ADR-062/063).
//!
//! Modes : `scene` (constats de construction), `seams` (coutures d'emprise),
//! `render <ppm> <secondes après naissance> [temoin]`, `cost` (coût par point).
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use water_core::{
    background::Background,
    background_spectrum::{self, Cooked, Recipe},
    impact_field::{Medium, BREAKING_SLOPE},
    impact_generator::{self, Entry},
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, HostServices, SeaState, SimTime, WorldPos,
};

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
    let cooked = background_spectrum::bake(recipe(hs)).map_err(|e| format!("recette {e:?}"))?;
    let floor = slope_floor(&cooked);
    if !(floor < BREAKING_SLOPE) {
        return Err(format!(
            "plancher L1 de B {floor} >= pi/7 {BREAKING_SLOPE} : aucune composition possible"
        ));
    }
    let medium = Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: DEPTH_M,
        max_slope: BREAKING_SLOPE - floor,
    };
    let entry = Entry {
        half_width_m: ENTRY_HALF_WIDTH_M,
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
        _ => return Err("mode : scene | seams".into()),
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
    fn directional_bound_sits_between_sample_and_l1() {
        let sc = scene(HS, 10_000_000).unwrap();
        let d = directional_slope_bound(&sc.cooked);
        let sampled = sampled_real_slope(&sc.background, 16.0, 0.5, 4);
        assert!(sampled <= d, "{sampled} > {d}");
        assert!(d < sc.floor as f64, "{d} >= {}", sc.floor);
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
