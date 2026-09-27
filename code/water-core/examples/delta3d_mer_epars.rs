//! **L'épars et les niveaux sous le pas couplé** — S404, C8e de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)) : le domaine épars de S401 et le transfert de
//! niveau d'[ADR-210](../../../docs/adr/ADR-210-changer-de-niveau-par-transfert-d-etat.md), en mer — δ sous B, en mode relatif
//! ([ADR-198](../../../docs/adr/ADR-198-la-voie-d-a289.md) D1).
//!
//! Une fenêtre de 32 × 16 m à 25 cm (128 × 64 × 12), 2 m d'eau sous 1 m d'air, pas couplé de 20 ms, multigrille (C1). **B** : la
//! houle de S369 — une composante de 5 cm, `T` = 3,2 s —, oblique (36°). **L'éponge** : 2 m sur les deux axes, au taux
//! `10·c_g/largeur` (S315) pour les ondes de 2 m du dipôle (`c_g` ≈ 0,9 m/s en 2 m d'eau) : 4 s⁻¹ ; au bord de la boîte, et, dans
//! le domaine épars, au bord de l'ensemble (S404). **La source** : le dipôle de S401 — 0,05 m³/s, ajouté 0,5 m devant, retiré 0,5 m
//! derrière, gaussiennes d'écart type 0,5 m tronquées à 1,5 m et normalisées sur la grille —, débit monté en 1 s, en ligne droite
//! selon x. **Le suivi** : `Follow` de S401 (4 m, 1 mm, 0,25 s ; prévision, `a_max` = 1 m/s²), l'ensemble changé à chaque pas.
//!
//! - **`suivi`** : 2 m/s, 10 s ; le domaine entier contre l'épars qui suit.
//! - **`murs`** : le même, l'éponge de l'épars au seul bord de la boîte — les murs de l'ensemble réfléchissent : le témoin.
//! - **`long`** : 0,8 m/s, 30 s (L369 : un comportement s'éprouve sur sa durée d'usage).
//! - **`niveaux`** : 2 m/s, 10 s ; le rang 4 à 3 s — 25 → 50 cm —, le retour à 6 s ; le domaine entier qui fait ces passages et
//!   l'épars qui les fait en suivant (`Follow::require_cover` : l'ensemble d'arrivée couvre celui de départ), contre le domaine
//!   entier à 25 cm tenu tout du long. L'image est ce que le rendu montrerait, sur la grille fine (S402).
//!
//! Publié : l'écart de surface au domaine entier (partout, et dans l'ensemble), l'amplitude, la part des mailles de l'ensemble au
//! cours du temps, ce que les blocs libérés ont rendu, les sauts et volumes des passages, itérations et durée d'un pas.
//!
//!     cargo run -p water-core --release --offline --example delta3d_mer_epars -- <suivi|murs|long|niveaux>
//!
//! `MER_DUREE_S=<s>` raccourcit un cas (essai de fumée).

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::time::Instant;
use water_core::background::{Background, SeaState};
use water_core::delta3d::{BackgroundGrid3, Domain3, Sponge3, Volume3};
use water_core::domain_blocks::{useful_horizon, Follow, Tracked};
use water_core::host::HostServices;
use water_core::{SimTime, WorldPos};

const LX: f32 = 32.;
const LY: f32 = 16.;
const REST: f32 = 2.0;
const RHO: f32 = 1025.;
const DT_US: u64 = 20_000;
/// Rayon de couplage (ADR-006 §4, S396), m : deux blocs de 2 m à 25 cm, un bloc de 4 m à 50 cm.
const R_C_M: f32 = 4.;
/// Seuil d'activité (S396) et délai de libération d'un bloc (ADR-006 §4).
const ACTIVE: f32 = 1e-3;
const RELEASE_US: u64 = 250_000;
/// Demi-écart du dipôle, écart type et troncature de ses gaussiennes, m ; débit, m³/s.
const HALF_GAP: f32 = 0.5;
const SIGMA: f32 = 0.5;
const CUT: f32 = 1.5;
const FLOW: f32 = 0.05;
const A_MAX: f32 = 1.;
const SPONGE: Sponge3 = Sponge3 { width_x: 2., width_y: 2., rate_per_s: 4. };
/// Le rang 4 et le retour, cas `niveaux`.
const SWITCH_S: f64 = 3.;
const RETURN_S: f64 = 6.;

fn grid_of(dx: f32) -> Domain3 {
    Domain3 { nx: (LX / dx).round() as usize, ny: (LY / dx).round() as usize, nz: (3. / dx).round() as usize, dx }
}

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

/// Le dipôle sur la grille de `d` : `q` m³ ajoutés en `p + ½·e`, retirés en `p − ½·e`, gaussiennes normalisées sur la grille.
fn dipole(d: Domain3, p: [f32; 2], e: [f32; 2], q: f32, out: &mut Vec<f32>) {
    out.clear();
    out.resize(d.nx * d.ny, 0.);
    for (centre, sign) in [([p[0] + HALF_GAP * e[0], p[1] + HALF_GAP * e[1]], 1f32), ([p[0] - HALF_GAP * e[0], p[1] - HALF_GAP * e[1]], -1.)]
    {
        let mut sum = 0f64;
        let mut g = vec![0f32; d.nx * d.ny];
        for (c, gc) in g.iter_mut().enumerate() {
            let (x, y) = (((c % d.nx) as f32 + 0.5) * d.dx - centre[0], ((c / d.nx) as f32 + 0.5) * d.dx - centre[1]);
            let r2 = x * x + y * y;
            if r2 <= CUT * CUT {
                *gc = (-r2 / (2. * SIGMA * SIGMA)).exp();
                sum += *gc as f64;
            }
        }
        let norm = (1. / (sum * (d.dx as f64).powi(2))) as f32;
        for (o, gc) in out.iter_mut().zip(&g) {
            *o += sign * q * gc * norm;
        }
    }
}

struct Case {
    speed: f32,
    duration: f64,
    start: [f32; 2],
}

/// Ce qu'un pas couplé coûte et fait, cumulé.
#[derive(Default)]
struct Cost {
    iterations: u64,
    ms: f64,
}

fn step(v: &mut Volume3, grid: &BackgroundGrid3, time: SimTime, cost: &mut Cost) {
    let bg = grid.view().expect("fond");
    let start = Instant::now();
    let report = v.step_perturbation_mobile(time, DT_US, 20_000, &bg, SPONGE, &host_impl::SequentialJobs).expect("pas couplé");
    cost.ms += start.elapsed().as_secs_f64() * 1e3;
    cost.iterations += report.iterations as u64;
}

fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "suivi".into());
    let case = match name.as_str() {
        "suivi" | "murs" | "niveaux" => Case { speed: 2., duration: 10., start: [6., 8.] },
        "long" => Case { speed: 0.8, duration: 30., start: [4., 8.] },
        other => panic!("cas inconnu : {other}"),
    };
    // `MER_DUREE_S=<s>` : une durée plus courte, pour un essai de fumée.
    let case = match std::env::var("MER_DUREE_S").ok().and_then(|v| v.parse::<f64>().ok()) {
        Some(d) => Case { duration: d, ..case },
        None => case,
    };
    let levels = name == "niveaux";
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut host = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let sea = SeaState { hs: 0.05 * 2. * 2f32.sqrt(), tp: 3.2, theta_turns: 0.1, components: 1, graine: 7 };
    let houle = Background::configure(&mut host, sea, WorldPos::from_units(0, 0, 0)).expect("houle");
    let g = houle.gravity();
    let (fd, cd) = (grid_of(0.25), grid_of(0.5));
    let mut grid_f = BackgroundGrid3::configure(&mut host, fd, [0., 0., -REST], RHO).expect("grille fine");
    let mut grid_c = BackgroundGrid3::configure(&mut host, cd, [0., 0., -REST], RHO).expect("grille grossière");
    // La référence ; l'épars qui suit ; pour `niveaux`, l'entier qui change de niveau, les deux à 50 cm, l'image fine.
    let mut reference = volume(&mut host, fd, g, false);
    let mut sparse = volume(&mut host, fd, g, true);
    if name == "murs" {
        sparse.set_sparse_edge_sponge_for_trials(false).expect("témoin");
    }
    let mut dense_lvl = volume(&mut host, fd, g, false);
    let mut dense_c = volume(&mut host, cd, g, false);
    let mut sparse_c = volume(&mut host, cd, g, true);
    let mut image = volume(&mut host, fd, g, false);
    let mut follow_f = Follow::with_capacity(&mut host, fd.nx, fd.ny, fd.dx, (R_C_M / (8. * fd.dx)).round() as i32, ACTIVE, RELEASE_US)
        .expect("suivi fin");
    let mut follow_c = Follow::with_capacity(&mut host, cd.nx, cd.ny, cd.dx, (R_C_M / (8. * cd.dx)).round() as i32, ACTIVE, RELEASE_US)
        .expect("suivi grossier");
    let (mut dh_f, mut dh_c) = (Vec::new(), Vec::new());
    let (mut mask_f, mut mask_c) = (vec![0u8; fd.nx * fd.ny], vec![0u8; cd.nx * cd.ny]);
    let dt = DT_US as f64 * 1e-6;
    let steps = (case.duration / dt).round() as u64;
    let (n_switch, n_return) = ((SWITCH_S / dt).round() as u64, (RETURN_S / dt).round() as u64);
    let horizon = useful_horizon(R_C_M, A_MAX, 5.);
    let kin = |t: f64| ([case.start[0] + case.speed * t as f32, case.start[1]], [case.speed, 0f32]);
    let tracked = |t: f64| {
        let (p, v) = kin(t);
        Tracked { position: p, velocity: v, a_max: A_MAX, horizon, radius: HALF_GAP + CUT }
    };
    follow_f.update(0, sparse.surface(), REST, fd.nx, &[tracked(0.)]);
    follow_f.columns(fd.nx, fd.ny, &mut mask_f);
    sparse.set_active_columns(&mask_f).expect("ensemble");
    let cells_f = fd.cells() as f64;
    let (mut gap, mut gap_in, mut gap_lvl, mut amplitude) = (0f32, 0f32, 0f32, 0f32);
    // Pour `niveaux` : l'écart de l'entier qui change de niveau à la référence, avant, pendant, après ; les sauts.
    let (mut price, mut pops) = ([0f32; 3], [[0f32; 2]; 2]);
    let (mut part_sum, mut part_max, mut parts) = (0f64, 0f64, Vec::new());
    let (mut removed, mut height_removed) = (0f64, 0f32);
    let mut volumes: Vec<(f64, f64)> = Vec::new();
    let (mut cost_ref, mut cost_sparse) = (Cost::default(), Cost::default());
    let mut outside: Option<f64> = None;
    let (mut prev_img, mut prev_img_s, mut prev_ref) = (reference.surface().to_vec(), reference.surface().to_vec(), reference.surface().to_vec());
    let mut img_s = vec![REST; fd.nx * fd.ny];
    for n in 0..steps {
        let t = n as f64 * dt;
        let time = SimTime(n * DT_US);
        let now = n * DT_US;
        // Les passages, au début du pas.
        if levels && n == n_switch {
            volumes.push(dense_c.resample_from(&dense_lvl).map(|c| (c.volume_before, c.volume_after)).expect("25 → 50 cm"));
            follow_c.require_cover(now, sparse.active_columns().expect("ensemble"), fd.nx, fd.ny, fd.dx);
            follow_c.columns(cd.nx, cd.ny, &mut mask_c);
            sparse_c.set_active_columns(&mask_c).expect("ensemble grossier");
            volumes.push(sparse_c.resample_from(&sparse).map(|c| (c.volume_before, c.volume_after)).expect("épars 25 → 50 cm"));
        }
        if levels && n == n_return {
            volumes.push(dense_lvl.resample_from(&dense_c).map(|c| (c.volume_before, c.volume_after)).expect("50 → 25 cm"));
            follow_f.require_cover(now, sparse_c.active_columns().expect("ensemble"), cd.nx, cd.ny, cd.dx);
            follow_f.columns(fd.nx, fd.ny, &mut mask_f);
            sparse.set_active_columns(&mask_f).expect("ensemble fin");
            volumes.push(sparse.resample_from(&sparse_c).map(|c| (c.volume_before, c.volume_after)).expect("épars 50 → 25 cm"));
        }
        let coarse = levels && n >= n_switch && n < n_return;
        // La source à mi-pas, le débit monté en une seconde.
        let (p, v) = kin(t + 0.5 * dt);
        let e = [v[0] / v[0].hypot(v[1]), v[1] / v[0].hypot(v[1])];
        let ramp = t.min(1.);
        let q = FLOW * (ramp * ramp * (3. - 2. * ramp)) as f32 * dt as f32;
        dipole(fd, p, e, q, &mut dh_f);
        grid_f.sample(&houle, time).expect("houle fine");
        reference.add_column_volume(&dh_f).expect("source, référence");
        step(&mut reference, &grid_f, time, &mut cost_ref);
        if coarse {
            dipole(cd, p, e, q, &mut dh_c);
            grid_c.sample(&houle, time).expect("houle grossière");
            dense_c.add_column_volume(&dh_c).expect("source, entier à 50 cm");
            step(&mut dense_c, &grid_c, time, &mut Cost::default());
            if sparse_c.add_column_volume(&dh_c).is_err() {
                outside = Some(t);
                break;
            }
            step(&mut sparse_c, &grid_c, time, &mut cost_sparse);
        } else {
            if levels {
                dense_lvl.add_column_volume(&dh_f).expect("source, entier");
                step(&mut dense_lvl, &grid_f, time, &mut Cost::default());
            }
            if sparse.add_column_volume(&dh_f).is_err() {
                outside = Some(t);
                break;
            }
            step(&mut sparse, &grid_f, time, &mut cost_sparse);
        }
        // Le suivi, à chaque pas, au niveau courant.
        let now = (n + 1) * DT_US;
        let change = if coarse {
            follow_c.update(now, sparse_c.surface(), REST, cd.nx, &[tracked((n + 1) as f64 * dt)]);
            follow_c.columns(cd.nx, cd.ny, &mut mask_c);
            sparse_c.set_active_columns(&mask_c).expect("ensemble grossier")
        } else {
            follow_f.update(now, sparse.surface(), REST, fd.nx, &[tracked((n + 1) as f64 * dt)]);
            follow_f.columns(fd.nx, fd.ny, &mut mask_f);
            sparse.set_active_columns(&mask_f).expect("ensemble")
        };
        removed += change.volume_removed;
        height_removed = height_removed.max(change.height_removed);
        // Les images, sur la grille fine.
        let r = reference.surface().to_vec();
        let (img, active): (Vec<f32>, Vec<u8>) = if coarse {
            image.resample_from(&sparse_c).expect("image de l'épars");
            img_s.copy_from_slice(image.surface());
            image.resample_from(&dense_c).expect("image de l'entier");
            let mut a = vec![0u8; fd.nx * fd.ny];
            let ac = sparse_c.active_columns().expect("ensemble");
            for (c, x) in a.iter_mut().enumerate() {
                let (i, j) = (c % fd.nx, c / fd.nx);
                *x = ac[(j / 2) * cd.nx + i / 2];
            }
            (image.surface().to_vec(), a)
        } else {
            img_s.copy_from_slice(sparse.surface());
            let d = if levels { dense_lvl.surface().to_vec() } else { r.clone() };
            (d, sparse.active_columns().expect("ensemble").to_vec())
        };
        let phase = if !levels || n < n_switch { 0 } else if n < n_return { 1 } else { 2 };
        let (mut pop, mut pop_s) = (0f32, 0f32);
        for c in 0..fd.nx * fd.ny {
            let d = (img_s[c] - img[c]).abs();
            if levels {
                gap_lvl = gap_lvl.max(d);
                price[phase] = price[phase].max((img[c] - r[c]).abs());
                pop = pop.max(((img[c] - prev_img[c]) - (r[c] - prev_ref[c])).abs());
                pop_s = pop_s.max(((img_s[c] - prev_img_s[c]) - (r[c] - prev_ref[c])).abs());
            }
            let dr = (img_s[c] - r[c]).abs();
            gap = gap.max(dr);
            if active[c] != 0 {
                gap_in = gap_in.max(dr);
            }
            amplitude = amplitude.max((r[c] - REST).abs());
        }
        if levels && (n == n_switch || n == n_return) {
            let k = usize::from(n == n_return);
            pops[k] = [pop, pop_s];
        }
        prev_img.copy_from_slice(&img);
        prev_img_s.copy_from_slice(&img_s);
        prev_ref.copy_from_slice(&r);
        let part = if coarse {
            sparse_c.sparse_cells() as f64 / cd.cells() as f64
        } else {
            sparse.sparse_cells() as f64 / cells_f
        };
        part_sum += part;
        part_max = part_max.max(part);
        if (n + 1) % 100 == 0 {
            parts.push(part);
        }
        if n % 50 == 49 {
            eprintln!("t = {:.1} s : écart {gap:.2e} m, part {part:.3}, amplitude {amplitude:.4} m", (n + 1) as f64 * dt);
        }
    }
    let done = outside.map_or(steps, |t| (t / dt) as u64).max(1);
    let fmt = |v: &[f64]| v.iter().map(|x| format!("{x:.3}")).collect::<Vec<_>>().join("/");
    println!(
        "MER_EPARS_S404 cas={name} vitesse={} duree_s={:.2} source_dehors={} ecart_max_m={gap:.3e} ecart_dans_ensemble_m={gap_in:.3e} \
         amplitude_m={amplitude:.4} critere_3mm={} part_moy={:.3} part_max={:.3} part_toutes_2s={} volume_rendu_m3={removed:.3e} \
         hauteur_rendue_max_m={height_removed:.2e} iterations_moy={:.1}/{:.1} duree_pas_ms={:.1}/{:.1}",
        case.speed,
        done as f64 * dt,
        outside.map_or("non".to_string(), |t| format!("{t:.2}")),
        if outside.is_none() && gap <= 3e-3 { "tenu" } else { "manqué" },
        part_sum / done as f64,
        part_max,
        fmt(&parts),
        cost_ref.iterations as f64 / done as f64,
        cost_sparse.iterations as f64 / done as f64,
        cost_ref.ms / done as f64,
        cost_sparse.ms / done as f64,
    );
    if levels {
        println!(
            "MER_EPARS_S404_NIVEAUX epars_contre_entier_m={gap_lvl:.3e} critere_3mm={} prix_avant/pendant/apres_m={:.3e}/{:.3e}/{:.3e} \
             sauts_passage_entier/epars_m={:.3e}/{:.3e} sauts_retour_entier/epars_m={:.3e}/{:.3e} volumes_m3={}",
            if gap_lvl <= 3e-3 { "tenu" } else { "manqué" },
            price[0],
            price[1],
            price[2],
            pops[0][0],
            pops[0][1],
            pops[1][0],
            pops[1][1],
            volumes.iter().map(|(a, b)| format!("{a:.6e}->{b:.6e}")).collect::<Vec<_>>().join(";"),
        );
    }
}
