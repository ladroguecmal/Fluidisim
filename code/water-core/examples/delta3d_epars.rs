//! **Le domaine épars qui suit une source mobile** — S401, C8b de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) ; [ADR-006](../../../docs/adr/ADR-006-cellules-domaines-solveurs.md)
//! §3–4 ; [ADR-013](../../../docs/adr/ADR-013-prediction-activation-precalcul.md) §2).
//!
//! Un bassin de 40 × 20 m à 25 cm, 2 m d'eau (8 couches) sous 1 m d'air, pas mobile de 20 ms, multigrille (C1). **La source** :
//! un dipôle de volume — ajouté devant, retiré derrière, à 1 m l'un de l'autre, en gaussiennes tronquées à 1,5 m et
//! normalisées sur la grille —, le modèle au premier ordre d'un corps qui avance ; son débit monte en une seconde. **La
//! référence** : le domaine entier, toute la fenêtre. **Le domaine épars** : la même fenêtre, dont l'ensemble est celui que
//! `Follow` requiert — blocs à moins de 4 m d'un bloc où la surface s'écarte du repos de plus de 1 mm, ou de l'enveloppe
//! prévue de la source (ADR-013 §2) —, libérés 0,25 s après ; une source qui tomberait hors de l'ensemble est refusée par
//! `add_column_volume`, et le banc le dit.
//!
//! - **`droite`** : 2 m/s en ligne droite, 10 s ; ensemble mis à jour à chaque pas.
//! - **`virage`** : 2 m/s, accélération latérale de 0,5 m/s² (rayon 8 m), 8 s ; `a_max` = 1 m/s².
//! - **`rapide`** : 10 m/s en ligne droite, 3 s ; ensemble mis à jour toutes les 0,5 s. `EPARS_PREVISION=0` : la source
//!   suivie là où elle est, sans enveloppe — le témoin ; `EPARS_CADENCE_S=<s>` change la cadence de mise à jour.
//!
//! Publié : l'écart de surface au domaine entier (partout, et dans l'ensemble), l'amplitude, la part des mailles de
//! l'ensemble, ce que les blocs libérés ont rendu au repos, itérations et durée d'un pas des deux calculs.
//!
//!     cargo run -p water-core --release --offline --example delta3d_epars -- <droite|virage|rapide>

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::delta3d::{Domain3, Volume3};
use water_core::domain_blocks::{useful_horizon, Follow, Tracked};
use water_core::host::HostServices;

const DX: f32 = 0.25;
const NX: usize = 160;
const NY: usize = 80;
const NZ: usize = 12;
const REST: f32 = 2.0;
const DT_US: u64 = 20_000;
/// Rayon de couplage (ADR-006 §4, S396) : 4 m, deux blocs de 2 m.
const R_C: i32 = 2;
/// Seuil d'activité (S396).
const ACTIVE: f32 = 1e-3;
/// Délai de libération d'un bloc (ADR-006 §4).
const RELEASE_US: u64 = 250_000;
/// Demi-écart du dipôle, écart type et troncature de ses gaussiennes, m.
const HALF_GAP: f32 = 0.5;
const SIGMA: f32 = 0.5;
const CUT: f32 = 1.5;

struct Case {
    speed: f32,
    lateral: f32,
    a_max: f32,
    duration: f64,
    cadence_steps: u64,
    start: [f32; 2],
    /// Débit du dipôle, m³/s.
    flow: f32,
}

fn domain(host: &mut HostServices, sparse: bool) -> Volume3 {
    let mut v = Volume3::configure(host, Domain3 { nx: NX, ny: NY, nz: NZ, dx: DX }, 1000., 9.81).expect("domaine");
    v.clear_to_rest(REST).expect("repos");
    v.enable_multigrid(host).expect("multigrille");
    if sparse {
        v.enable_sparse(host).expect("ensemble");
    }
    v
}

/// La gaussienne tronquée de centre `c`, normalisée sur la grille : `Σ g·dx²` = 1.
fn gaussian(c: [f32; 2], out: &mut [f32]) {
    out.fill(0.);
    let mut sum = 0f64;
    for j in 0..NY {
        for i in 0..NX {
            let (x, y) = ((i as f32 + 0.5) * DX - c[0], (j as f32 + 0.5) * DX - c[1]);
            let r2 = x * x + y * y;
            if r2 <= CUT * CUT {
                let g = (-r2 / (2. * SIGMA * SIGMA)).exp();
                out[j * NX + i] = g;
                sum += g as f64;
            }
        }
    }
    let norm = (1. / (sum * (DX as f64).powi(2))) as f32;
    for g in out.iter_mut() {
        *g *= norm;
    }
}

/// La position et la vitesse de la source à l'instant `t` : droite, ou arc de cercle à accélération latérale constante.
fn kinematics(case: &Case, t: f64) -> ([f32; 2], [f32; 2]) {
    let (v, a) = (case.speed as f64, case.lateral as f64);
    if a == 0. {
        return ([case.start[0] + (v * t) as f32, case.start[1]], [case.speed, 0.]);
    }
    let r = v * v / a;
    let th = v * t / r;
    let p = [case.start[0] as f64 + r * th.sin(), case.start[1] as f64 + r * (1. - th.cos())];
    ([p[0] as f32, p[1] as f32], [(v * th.cos()) as f32, (v * th.sin()) as f32])
}

fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "droite".into());
    let prevision = std::env::var("EPARS_PREVISION").map(|v| v != "0").unwrap_or(true);
    let case = match name.as_str() {
        "droite" => Case { speed: 2., lateral: 0., a_max: 1., duration: 10., cadence_steps: 1, start: [6., 10.], flow: 0.05 },
        "virage" => Case { speed: 2., lateral: 0.5, a_max: 1., duration: 8., cadence_steps: 1, start: [8., 4.], flow: 0.05 },
        "rapide" => Case { speed: 10., lateral: 0., a_max: 2., duration: 3., cadence_steps: 25, start: [4., 10.], flow: 0.05 },
        other => panic!("cas inconnu : {other}"),
    };
    // `EPARS_CADENCE_S=<s>` : la cadence de mise à jour de l'ensemble, en secondes (témoins).
    let case = match std::env::var("EPARS_CADENCE_S").ok().and_then(|v| v.parse::<f64>().ok()) {
        Some(c) => Case { cadence_steps: ((c / (DT_US as f64 * 1e-6)).round() as u64).max(1), ..case },
        None => case,
    };
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 30);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut host = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let mut full = domain(&mut host, false);
    let mut sparse = domain(&mut host, true);
    let mut follow = Follow::with_capacity(&mut host, NX, NY, DX, R_C, ACTIVE, RELEASE_US).expect("suivi");
    let (mut front, mut back, mut dh, mut mask) = (vec![0f32; NX * NY], vec![0f32; NX * NY], vec![0f32; NX * NY], vec![0u8; NX * NY]);
    let dt = DT_US as f64 * 1e-6;
    let steps = (case.duration / dt).round() as u64;
    let horizon = if prevision { useful_horizon(2. * R_C as f32, case.a_max, 5.) } else { 0. };
    let tracked = |t: f64| {
        let (p, v) = kinematics(&case, t);
        Tracked { position: p, velocity: v, a_max: case.a_max, horizon, radius: HALF_GAP + CUT }
    };
    // L'ensemble de départ : l'enveloppe seule, aucune activité encore.
    follow.update(0, sparse.surface(), REST, NX, &[tracked(0.)]);
    follow.columns(NX, NY, &mut mask);
    sparse.set_active_columns(&mask).expect("ensemble");
    let (mut gap, mut gap_in, mut amplitude) = (0f32, 0f32, 0f32);
    let (mut part_sum, mut part_max) = (0f64, 0f64);
    let (mut removed, mut height_removed, mut velocity_removed) = (0f64, 0f32, 0f32);
    let (mut its, mut ms) = ([0u64; 2], [0f64; 2]);
    let mut outside: Option<f64> = None;
    for s in 0..steps {
        let t = s as f64 * dt;
        // Le dipôle à mi-pas, le débit monté en une seconde (lissé).
        let (p, v) = kinematics(&case, t + 0.5 * dt);
        let norm = (v[0] * v[0] + v[1] * v[1]).sqrt().max(1e-6);
        let e = [v[0] / norm, v[1] / norm];
        gaussian([p[0] + HALF_GAP * e[0], p[1] + HALF_GAP * e[1]], &mut front);
        gaussian([p[0] - HALF_GAP * e[0], p[1] - HALF_GAP * e[1]], &mut back);
        let ramp = (t / 1.).min(1.);
        let q = case.flow * (ramp * ramp * (3. - 2. * ramp)) as f32 * dt as f32;
        for c in 0..NX * NY {
            dh[c] = q * (front[c] - back[c]);
        }
        full.add_column_volume(&dh).expect("source, domaine entier");
        if sparse.add_column_volume(&dh).is_err() {
            outside = Some(t);
            break;
        }
        let start = Instant::now();
        its[0] += full.step_surface_mobile(DT_US, 20_000, &jobs).expect("pas, domaine entier").iterations as u64;
        ms[0] += start.elapsed().as_secs_f64() * 1e3;
        let start = Instant::now();
        its[1] += sparse.step_surface_mobile(DT_US, 20_000, &jobs).expect("pas, domaine épars").iterations as u64;
        ms[1] += start.elapsed().as_secs_f64() * 1e3;
        let now = (s + 1) * DT_US;
        if (s + 1) % case.cadence_steps == 0 {
            follow.update(now, sparse.surface(), REST, NX, &[tracked((s + 1) as f64 * dt)]);
            follow.columns(NX, NY, &mut mask);
            let change = sparse.set_active_columns(&mask).expect("ensemble");
            removed += change.volume_removed;
            height_removed = height_removed.max(change.height_removed);
            velocity_removed = velocity_removed.max(change.velocity_removed);
        }
        let active = sparse.active_columns().expect("ensemble");
        let (a, b) = (full.surface(), sparse.surface());
        for c in 0..NX * NY {
            let d = (a[c] - b[c]).abs();
            gap = gap.max(d);
            if active[c] != 0 {
                gap_in = gap_in.max(d);
            }
            amplitude = amplitude.max((a[c] - REST).abs());
        }
        let part = sparse.sparse_cells() as f64 / full.domain().cells() as f64;
        part_sum += part;
        part_max = part_max.max(part);
        if s % 50 == 49 {
            eprintln!("t = {:.1} s : écart {gap:.2e} m, part {part:.3}, amplitude {amplitude:.4} m", (s + 1) as f64 * dt);
        }
    }
    let done = match outside {
        Some(t) => (t / dt) as u64,
        None => steps,
    };
    let n = done.max(1) as f64;
    println!(
        "EPARS_S401 cas={name} prevision={} horizon_s={horizon:.2} vitesse={} cadence_s={:.2} duree_s={:.2} source_dehors={} \
         ecart_max_m={gap:.3e} ecart_dans_ensemble_m={gap_in:.3e} amplitude_m={amplitude:.4} critere_3mm={} \
         part_mailles_moy={:.3} part_mailles_max={:.3} volume_rendu_m3={removed:.3e} hauteur_rendue_max_m={height_removed:.2e} \
         vitesse_effacee_max={velocity_removed:.2e} iterations_moy={:.1}/{:.1} duree_pas_ms={:.1}/{:.1}",
        u8::from(prevision),
        case.speed,
        case.cadence_steps as f64 * dt,
        done as f64 * dt,
        outside.map_or("non".to_string(), |t| format!("{t:.2}")),
        if outside.is_none() && gap <= 3e-3 { "tenu" } else { "manqué" },
        part_sum / n,
        part_max,
        its[0] as f64 / n,
        its[1] as f64 / n,
        ms[0] / n,
        ms[1] / n,
    );
}
