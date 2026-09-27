//! **L'impact prévu** — S405, liste 9.3 : la prédiction balistique (`ballistic`) consommée par le domaine épars en mer
//! ([ADR-013](../../../docs/adr/ADR-013-prediction-activation-precalcul.md) §2 ; source §8.2 : « le temps de vol devient alors
//! une fenêtre de calcul permettant de préparer le domaine d'eau »).
//!
//! La mer de S404 : fenêtre de 32 × 16 m à 25 cm, 2 m d'eau sous 1 m d'air, houle de S369 oblique (5 cm, `T` = 3,2 s), pas couplé
//! relatif de 20 ms, éponge de 2 m à 4 s⁻¹, au bord de la boîte et de l'ensemble. **L'objet** : une sphère de 0,25 m lancée à
//! 12 m/s de 6 m au-dessus du plan moyen, traînée vraie `k` = 0,012 m⁻¹ ; le prédicteur ne connaît que `k` = 0,01 ± 30 %.
//! **L'entrée** : le volume de la calotte immergée, ajouté à la surface à chaque pas (gaussienne de 0,5 m tronquée à 1,5 m) —
//! l'objet poursuit sa course jusqu'à être immergé, puis s'arrête. **L'ensemble** : `Follow` de S401 (4 m, 1 mm, 0,25 s), revu
//! toutes les 0,5 s — la préparation d'un domaine n'est pas instantanée.
//!
//! - **`prevu`** : à chaque revue, l'impact est prédit depuis l'état courant (`predict_region`, sur la houle de B) ; en palier T2
//!   ou T1, sa région est suivie ; l'objet l'est aussi, là où il est.
//! - **`temoin`** : l'objet suivi là où il est, sans prédiction.
//!
//! Publié : l'instant et le point d'impact vrais, l'erreur de la prédiction à chaque revue et si la région la couvre, l'avance
//! avec laquelle la région d'impact est dans l'ensemble, si la source d'entrée en est jamais sortie, l'écart au domaine entier.
//!
//!     cargo run -p water-core --release --offline --example delta3d_impact_prevu -- <prevu|temoin>
//!
//! `IMPACT_PHASE_S=<s>` décale les revues : ce qui sépare la dernière revue de l'impact en dépend.

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::background::{Background, SeaState};
use water_core::ballistic::{advance, predict, predict_region, tier, Ballistic, Tier};
use water_core::delta3d::{BackgroundGrid3, Domain3, Sponge3, Volume3};
use water_core::domain_blocks::{Follow, Tracked};
use water_core::host::HostServices;
use water_core::{SimTime, WorldPos};

const DX: f32 = 0.25;
const LX: f32 = 32.;
const LY: f32 = 16.;
const REST: f32 = 2.0;
const RHO: f32 = 1025.;
const DT_US: u64 = 20_000;
const G: f64 = 9.81;
const RADIUS: f64 = 0.25;
const DRAG_TRUE: f64 = 0.012;
const DRAG_NOMINAL: f64 = 0.01;
const DRAG_BOUNDS: [f64; 2] = [0.007, 0.013];
const SIGMA: f32 = 0.5;
const CUT: f32 = 1.5;
const SPONGE: Sponge3 = Sponge3 { width_x: 2., width_y: 2., rate_per_s: 4. };
/// Rayon de couplage, seuil d'activité, délai de libération (S396, S401) ; revue de l'ensemble.
const R_C: i32 = 2;
const ACTIVE: f32 = 1e-3;
const RELEASE_US: u64 = 250_000;
const REVIEW_STEPS: u64 = 25;
/// Le domaine d'ADR-013 §2 pour le palier : la région suivie tient dans `r_c`.
const R_DOMAIN: f64 = 4.;
const DURATION_S: f64 = 4.;

fn volume(host: &mut HostServices, d: Domain3, g: f32, sparse: bool) -> Volume3 {
    let mut v = Volume3::configure(host, d, RHO, g).expect("domaine");
    v.set_free_surface(&vec![REST; d.nx * d.ny], REST).expect("repos");
    v.set_relative_background(Volume3::RELATIVE_ALL).expect("relatif");
    v.enable_multigrid(host).expect("multigrille");
    if sparse {
        v.enable_sparse(host).expect("ensemble");
    }
    v
}

/// `volume` m³ en gaussienne normalisée sur la grille, centrée en `p`.
fn spread(d: Domain3, p: [f32; 2], volume: f32, out: &mut [f32]) {
    out.fill(0.);
    let mut sum = 0f64;
    for (c, o) in out.iter_mut().enumerate() {
        let (x, y) = (((c % d.nx) as f32 + 0.5) * d.dx - p[0], ((c / d.nx) as f32 + 0.5) * d.dx - p[1]);
        let r2 = x * x + y * y;
        if r2 <= CUT * CUT {
            *o = (-r2 / (2. * SIGMA * SIGMA)).exp();
            sum += *o as f64;
        }
    }
    let norm = (volume as f64 / (sum * (d.dx as f64).powi(2))) as f32;
    for o in out.iter_mut() {
        *o *= norm;
    }
}

/// Le volume d'une calotte de hauteur `h` d'une sphère de rayon `RADIUS`, m³.
fn cap(h: f64) -> f64 {
    let h = h.clamp(0., 2. * RADIUS);
    core::f64::consts::PI * h * h * (RADIUS - h / 3.)
}

fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "prevu".into());
    let predicting = match name.as_str() {
        "prevu" => true,
        "temoin" => false,
        other => panic!("cas inconnu : {other}"),
    };
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut host = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let sea = SeaState { hs: 0.05 * 2. * 2f32.sqrt(), tp: 3.2, theta_turns: 0.1, components: 1, graine: 7 };
    let houle = Background::configure(&mut host, sea, WorldPos::from_units(0, 0, 0)).expect("houle");
    let g = houle.gravity();
    let d = Domain3 { nx: (LX / DX) as usize, ny: (LY / DX) as usize, nz: (3. / DX) as usize, dx: DX };
    let mut grid = BackgroundGrid3::configure(&mut host, d, [0., 0., -REST], RHO).expect("grille");
    let mut full = volume(&mut host, d, g, false);
    let mut sparse = volume(&mut host, d, g, true);
    let mut follow = Follow::with_capacity(&mut host, d.nx, d.ny, d.dx, R_C, ACTIVE, RELEASE_US).expect("suivi");
    let (mut dh, mut mask) = (vec![0f32; d.nx * d.ny], vec![0u8; d.nx * d.ny]);
    // La surface de B à l'instant `now + t`, m.
    let eta = |p: [f64; 2], t: f64, now_us: u64| -> f64 {
        let time = SimTime(now_us + (t.max(0.) * 1e6).round() as u64);
        houle.eval(WorldPos::from_metres(p[0], p[1], 0.), time).map_or(0., |s| s.eta as f64)
    };
    // L'objet vrai, et son impact vrai.
    let start = Ballistic {
        position: [3., 8., 6.],
        velocity: [12., 0., 0.],
        orientation: [1., 0., 0., 0.],
        omega: [0.4, 0., 1.5],
        inertia: [1., 1., 1.],
        drag: DRAG_TRUE,
        radius: RADIUS,
    };
    let truth = predict(&start, G, 1e-3, 10., |p, t| eta(p, t, 0)).expect("impact vrai");
    eprintln!("impact vrai à {:.4} s en ({:.3}, {:.3}) m, vitesse {:?}", truth.time, truth.position[0], truth.position[1], truth.velocity);
    let dt = DT_US as f64 * 1e-6;
    let steps = (DURATION_S / dt).round() as u64;
    let n_impact = (truth.time / dt).ceil() as u64;
    let (mut gap, mut amplitude, mut part_max, mut part_sum) = (0f32, 0f32, 0f64, 0f64);
    let mut outside: Option<f64> = None;
    let mut covered_since: Option<f64> = None;
    let mut reviews: Vec<String> = Vec::new();
    let (mut obj, mut submerged) = (start, 0f64);
    let (mut stopped, mut in_water) = (false, false);
    // `IMPACT_PHASE_S=<s>` : les revues décalées de cette durée — la dernière revue avant l'impact en dépend (la première, à
    // l'instant zéro, reste).
    let phase = std::env::var("IMPACT_PHASE_S").ok().and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.);
    let phase_steps = (phase / dt).round() as u64;
    for n in 0..steps {
        let t = n as f64 * dt;
        let now_us = n * DT_US;
        // La revue de l'ensemble, toutes les 0,5 s : l'objet là où il est ; en `prevu`, la région de l'impact prédit.
        if n == 0 || (n >= phase_steps && (n - phase_steps) % REVIEW_STEPS == 0) {
            let mut tracked = vec![Tracked {
                position: [obj.position[0] as f32, obj.position[1] as f32],
                velocity: [0., 0.],
                a_max: 0.,
                horizon: 0.,
                radius: RADIUS as f32,
            }];
            if predicting && n < n_impact {
                let nominal = Ballistic { drag: DRAG_NOMINAL, ..obj };
                if let Some(imp) = predict_region(&nominal, DRAG_BOUNDS, G, 0.01, 10., |p, s| eta(p, s, now_us)) {
                    let err = (imp.position[0] - truth.position[0]).hypot(imp.position[1] - truth.position[1]);
                    let level = tier(imp.time, 0., R_DOMAIN);
                    reviews.push(format!(
                        "{t:.1}s:T{}:{:.3}m/{:.3}m:{:.1}ms",
                        level as u8,
                        err,
                        imp.region,
                        1e3 * (t + imp.time - truth.time)
                    ));
                    if level <= Tier::Build {
                        tracked.push(Tracked {
                            position: [imp.position[0] as f32, imp.position[1] as f32],
                            velocity: [0., 0.],
                            a_max: 0.,
                            horizon: 0.,
                            radius: imp.region as f32,
                        });
                    }
                }
            }
            follow.update(now_us, sparse.surface(), REST, d.nx, &tracked);
            follow.columns(d.nx, d.ny, &mut mask);
            sparse.set_active_columns(&mask).expect("ensemble");
        }
        // La région d'impact vraie — le disque de la sphère — est-elle dans l'ensemble ?
        let active = sparse.active_columns().expect("ensemble");
        let covered = (0..d.nx * d.ny).all(|c| {
            let (x, y) = (((c % d.nx) as f64 + 0.5) * DX as f64 - truth.position[0], ((c / d.nx) as f64 + 0.5) * DX as f64 - truth.position[1]);
            x * x + y * y > RADIUS * RADIUS * 4. || active[c] != 0
        });
        if covered && covered_since.is_none() {
            covered_since = Some(t);
        } else if !covered && t < truth.time {
            covered_since = None;
        }
        // L'objet : vol, puis entrée — dans l'eau, il poursuit sa course en ligne droite jusqu'à être immergé, et s'arrête.
        if !stopped {
            obj = if in_water {
                let p = obj.position;
                let v = obj.velocity;
                Ballistic { position: [p[0] + v[0] * dt, p[1] + v[1] * dt, p[2] + v[2] * dt], ..obj }
            } else {
                advance(&obj, G, dt, 1e-3)
            };
        }
        let h = eta([obj.position[0], obj.position[1]], 0., now_us + DT_US) - (obj.position[2] - RADIUS);
        let depth = h.clamp(0., 2. * RADIUS);
        let dv = cap(depth) - submerged;
        in_water = in_water || h > 0.;
        stopped = stopped || depth >= 2. * RADIUS;
        submerged += dv;
        grid.sample(&houle, SimTime(now_us)).expect("houle");
        let bg = grid.view().expect("fond");
        if dv != 0. {
            spread(d, [obj.position[0] as f32, obj.position[1] as f32], dv as f32, &mut dh);
            full.add_column_volume(&dh).expect("entrée, domaine entier");
            if sparse.add_column_volume(&dh).is_err() {
                outside = Some(t);
                break;
            }
        }
        full.step_perturbation_mobile(SimTime(now_us), DT_US, 20_000, &bg, SPONGE, &jobs).expect("pas, domaine entier");
        sparse.step_perturbation_mobile(SimTime(now_us), DT_US, 20_000, &bg, SPONGE, &jobs).expect("pas, domaine épars");
        let (a, b) = (full.surface(), sparse.surface());
        for c in 0..d.nx * d.ny {
            gap = gap.max((a[c] - b[c]).abs());
            amplitude = amplitude.max((a[c] - REST).abs());
        }
        let part = sparse.sparse_cells() as f64 / d.cells() as f64;
        part_sum += part;
        part_max = part_max.max(part);
        if n % 25 == 24 {
            eprintln!("t = {:.1} s : écart {gap:.2e} m, part {part:.3}, amplitude {amplitude:.4} m, immergé {submerged:.4} m³", (n + 1) as f64 * dt);
        }
    }
    let done = outside.map_or(steps, |t| (t / dt) as u64).max(1);
    println!(
        "IMPACT_S405 cas={name} phase_s={phase:.2} impact_vrai_s={:.4} point_vrai_m={:.3}/{:.3} revues={} region_dans_ensemble_avant_impact_s={} \
         source_dehors={} ecart_max_m={gap:.3e} amplitude_m={amplitude:.4} critere_3mm={} part_moy={:.3} part_max={:.3} \
         volume_entre_m3={submerged:.4}",
        truth.time,
        truth.position[0],
        truth.position[1],
        reviews.join(","),
        covered_since.map_or("jamais".to_string(), |s| format!("{:.2}", truth.time - s)),
        outside.map_or("non".to_string(), |t| format!("{t:.2}")),
        if outside.is_none() && gap <= 3e-3 { "tenu" } else { "manqué" },
        part_sum / done as f64,
        part_max,
    );
}
