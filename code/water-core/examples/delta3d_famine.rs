//! **La famine a une issue, et le rang 4 choisit selon le contenu** — S403, C8d de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)) ; ADR-012 §4 rangs 4 et 5,
//! [ADR-210](../../../docs/adr/ADR-210-changer-de-niveau-par-transfert-d-etat.md) D2.
//!
//! Trois domaines δ réels de 16 × 12 m à 25 cm (2 m d'eau sous 1 m d'air, pas mobile de 20 ms, multigrille), chacun avec son
//! jumeau à 50 cm : **F** (identité 0), focal, une source mobile — le dipôle de S401 à 2 m/s ; **B** (1), non focal, la même
//! source ; **A** (2), non focal, une bosse de 5 cm et σ = 1 m. **La référence** : les trois à 25 cm, sans famine.
//!
//! L'ordonnanceur du cœur (`Scheduler`, S278–S403) reçoit à chaque pas les trois candidats — F à 0,9, A et B à 0,5 de
//! priorité ; coûts estimés par la loi de coût de la production (0,09 ms + 9,49 ns par maille, S350 ramenée à la maille :
//! 0,44 ms à 25 cm, 0,134 ms à 50 cm) — et, pour A et B au niveau fin, la **déclaration du rang 4** : le coût à 50 cm et la
//! **perte**, l'écart d'un aller-retour 25 → 50 → 25 cm de leur surface présente (ADR-210 D2). Le budget : 2 ms jusqu'à 2 s,
//! **1,05 ms** jusqu'à 4 s (un des deux doit descendre), **0,6 ms** jusqu'à 5,5 s (l'un descend, l'autre est affamé), 2 ms
//! ensuite. L'hôte applique : un niveau qui change → le transfert d'ADR-210 ; **un affamé → le rang 5** — détruit, son image
//! s'efface en 0,5 s (ADR-005 §5), et il renaît au repos quand il est de nouveau servi.
//!
//! **`TEMOIN=1`** : les pertes déclarées sont nulles — l'égalité se départage par identité, et c'est B, la source, qui descend.
//!
//! Publié : par domaine et par phase, l'écart de l'image à la référence et le plus grand saut d'un pas ; le budget accordé au pire
//! contre le budget ; descentes, remontées, pas affamés.
//!
//!     cargo run -p water-core --release --offline --example delta3d_famine

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::delta3d::{Domain3, Volume3};
use water_core::host::HostServices;
use water_core::scheduler::{Bid, Coarsen, DomainId, Profile, Regime, Scheduler};
use water_core::types::SimTime;

const LX: f32 = 16.;
const LY: f32 = 12.;
const REST: f32 = 2.0;
const DT_US: u64 = 20_000;
const FINE: f32 = 0.25;
const COARSE: f32 = 0.5;
const END_S: f64 = 8.;
const FADE_S: f64 = 0.5;
/// La loi de coût de la production (S350), ramenée à la maille : part fixe et coût par maille, ms.
const FIXED_MS: f32 = 0.09;
const PER_CELL_MS: f32 = 9.49e-6;

fn domain(host: &mut HostServices, dx: f32) -> Volume3 {
    let (nx, ny, nz) = ((LX / dx).round() as usize, (LY / dx).round() as usize, (3. / dx).round() as usize);
    let mut v = Volume3::configure(host, Domain3 { nx, ny, nz, dx }, 1000., 9.81).expect("domaine");
    v.clear_to_rest(REST).expect("repos");
    v.enable_multigrid(host).expect("multigrille");
    v
}

fn cost(v: &Volume3) -> f32 {
    FIXED_MS + PER_CELL_MS * v.domain().cells() as f32
}

fn budget_at(t: f64) -> f32 {
    match t {
        t if t < 2. => 2.,
        t if t < 4. => 1.05,
        t if t < 5.5 => 0.6,
        _ => 2.,
    }
}

fn phase_of(t: f64) -> usize {
    match t {
        t if t < 2. => 0,
        t if t < 4. => 1,
        t if t < 5.5 => 2,
        _ => 3,
    }
}

/// Le dipôle de S401 sur la grille d'un domaine, à `p`, débit `q` par pas.
fn dipole(v: &Volume3, p: [f32; 2], q: f32, out: &mut [f32]) {
    let Domain3 { nx, ny, dx, .. } = v.domain();
    out.fill(0.);
    for (centre, sign) in [([p[0] + 0.5, p[1]], 1f32), ([p[0] - 0.5, p[1]], -1.)] {
        let mut sum = 0f64;
        for c in 0..nx * ny {
            let (x, y) = (((c % nx) as f32 + 0.5) * dx - centre[0], ((c / nx) as f32 + 0.5) * dx - centre[1]);
            if x * x + y * y <= 1.5 * 1.5 {
                sum += (-(x * x + y * y) / 0.5).exp() as f64;
            }
        }
        let norm = (1. / (sum * (dx as f64).powi(2))) as f32;
        for (c, o) in out.iter_mut().enumerate().take(nx * ny) {
            let (x, y) = (((c % nx) as f32 + 0.5) * dx - centre[0], ((c / nx) as f32 + 0.5) * dx - centre[1]);
            if x * x + y * y <= 1.5 * 1.5 {
                *o += sign * q * (-(x * x + y * y) / 0.5).exp() * norm;
            }
        }
    }
}

/// Un domaine du banc : sa référence, ses deux niveaux, son état pour l'hôte.
struct Site {
    source: bool,
    reference: Volume3,
    fine: Volume3,
    coarse: Volume3,
    /// 0 : fin ; 1 : descendu ; `None` : détruit (rang 5).
    level: Option<u8>,
    /// L'image au moment de la destruction, et les pas de fondu qui restent (ADR-005 §5).
    fading: Vec<f32>,
    fade_left: usize,
    /// La dernière perte mesurée au niveau fin, déclarée aussi quand il est descendu (pour l'ordre des remontées).
    loss: f32,
    prev_image: Vec<f32>,
    prev_ref: Vec<f32>,
    /// Par phase : l'écart max à la référence, le saut max d'un pas.
    gap: [f32; 4],
    pop: [f32; 4],
    starved_steps: u32,
    descents: u32,
    ascents: u32,
}

fn main() {
    let temoin = std::env::var("TEMOIN").is_ok_and(|v| v == "1");
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 31);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut host = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let mut sites: Vec<Site> = (0..3)
        .map(|i| {
            let mut reference = domain(&mut host, FINE);
            let mut fine = domain(&mut host, FINE);
            let coarse = domain(&mut host, COARSE);
            if i == 2 {
                // A : la bosse, au centre de sa fenêtre.
                let Domain3 { nx, ny, dx, .. } = reference.domain();
                let eta: Vec<f32> = (0..nx * ny)
                    .map(|c| {
                        let (x, y) = (((c % nx) as f32 + 0.5) * dx - 8., ((c / nx) as f32 + 0.5) * dx - 6.);
                        REST + 0.05 * (-(x * x + y * y) / 2.).exp()
                    })
                    .collect();
                reference.set_free_surface(&eta, REST).expect("bosse");
                fine.set_free_surface(&eta, REST).expect("bosse");
            }
            let n = reference.surface().len();
            Site {
                source: i != 2,
                prev_image: reference.surface().to_vec(),
                prev_ref: reference.surface().to_vec(),
                reference,
                fine,
                coarse,
                level: Some(0),
                fading: vec![REST; n],
                fade_left: 0,
                loss: 0.,
                gap: [0.; 4],
                pop: [0.; 4],
                starved_steps: 0,
                descents: 0,
                ascents: 0,
            }
        })
        .collect();
    let mut scratch = domain(&mut host, FINE);
    let (cost_fine, cost_coarse) = (cost(&sites[0].fine), cost(&sites[0].coarse));
    let mut scheduler =
        Scheduler::with_capacity(&mut host, Profile { cpu_sim_ms: budget_at(0.), blocks: 64, on: 0.1, off: 0.05 }, 8)
            .expect("ordonnanceur");
    let dt = DT_US as f64 * 1e-6;
    let steps = (END_S / dt).round() as usize;
    let fade_steps = (FADE_S / dt).round() as usize;
    let n = sites[0].reference.surface().len();
    let (mut dh_fine, mut dh_coarse) = (vec![0f32; n], vec![0f32; sites[0].coarse.surface().len()]);
    let mut worst_ratio = 0f32;
    for s in 0..steps {
        let t = s as f64 * dt;
        let budget = budget_at(t);
        scheduler.set_profile(Profile { cpu_sim_ms: budget, blocks: 64, on: 0.1, off: 0.05 }).expect("profil");
        // Les pertes des non-focaux au niveau fin : l'écart d'un aller-retour de la surface présente (ADR-210 D2).
        for site in sites.iter_mut().skip(1) {
            if site.level == Some(0) {
                site.coarse.resample_from(&site.fine).expect("aller");
                scratch.resample_from(&site.coarse).expect("retour");
                site.loss = site.fine.surface().iter().zip(scratch.surface()).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
            }
        }
        scheduler.begin();
        for (i, site) in sites.iter().enumerate() {
            let perception = if i == 0 { 0.9 } else { 0.5 };
            scheduler
                .submit(Bid { id: DomainId(i as u32), gameplay: 1., perception, urgency: 1., cost_ms: cost_fine, blocks: 1,
                    regime: Regime::Perturbative, shrink: None })
                .expect("soumission");
            if i > 0 && site.level.is_some() {
                let loss = if temoin { 0. } else { site.loss };
                scheduler.declare_coarsen(DomainId(i as u32), Coarsen { cost_ms: cost_coarse, loss_m: loss }).expect("rang 4");
            }
        }
        scheduler.decide(SimTime(s as u64 * DT_US)).expect("décision");
        scheduler.allocate();
        worst_ratio = worst_ratio.max(scheduler.granted_ms() / budget);
        let phase = phase_of(t);
        for (i, site) in sites.iter_mut().enumerate() {
            let grant = scheduler.grants().iter().find(|g| g.id == DomainId(i as u32)).copied();
            // Ce que l'hôte fait de la décision : transfert d'ADR-210, destruction (rang 5), renaissance au repos.
            match (site.level, grant.map(|g| g.level)) {
                (Some(0), Some(1)) => {
                    site.coarse.resample_from(&site.fine).expect("25 → 50 cm");
                    site.descents += 1;
                }
                (Some(1), Some(0)) => {
                    site.fine.resample_from(&site.coarse).expect("50 → 25 cm");
                    site.ascents += 1;
                }
                (Some(l), None) => {
                    // Rang 5 : l'image présente s'efface en 0,5 s ; le domaine est détruit.
                    if l == 0 {
                        site.fading.copy_from_slice(site.fine.surface());
                    } else {
                        scratch.resample_from(&site.coarse).expect("image");
                        site.fading.copy_from_slice(scratch.surface());
                    }
                    site.fade_left = fade_steps;
                    site.fine.clear_to_rest(REST).expect("repos");
                    site.coarse.clear_to_rest(REST).expect("repos");
                }
                _ => {}
            }
            site.level = grant.map(|g| g.level);
            if grant.is_none() {
                site.starved_steps += 1;
            }
            // La source, dans la référence et dans le domaine vivant.
            if site.source {
                let p = [3. + 2. * (t + 0.5 * dt) as f32, 6.];
                let ramp = (t / 1.).min(1.);
                let q = 0.05 * (ramp * ramp * (3. - 2. * ramp)) as f32 * dt as f32;
                dipole(&site.reference, p, q, &mut dh_fine);
                site.reference.add_column_volume(&dh_fine).expect("source");
                match site.level {
                    Some(0) => site.fine.add_column_volume(&dh_fine).expect("source"),
                    Some(_) => {
                        dipole(&site.coarse, p, q, &mut dh_coarse);
                        site.coarse.add_column_volume(&dh_coarse).expect("source");
                    }
                    None => {}
                }
            }
            site.reference.step_surface_mobile(DT_US, 20_000, &jobs).expect("pas");
            match site.level {
                Some(0) => {
                    site.fine.step_surface_mobile(DT_US, 20_000, &jobs).expect("pas");
                }
                Some(_) => {
                    site.coarse.step_surface_mobile(DT_US, 20_000, &jobs).expect("pas");
                }
                None => {}
            }
            // L'image rendue, sur la grille fine.
            let mut image: Vec<f32> = match site.level {
                Some(0) => site.fine.surface().to_vec(),
                Some(_) => {
                    scratch.resample_from(&site.coarse).expect("image");
                    scratch.surface().to_vec()
                }
                None => vec![REST; n],
            };
            if site.fade_left > 0 {
                let w = (site.fade_left - 1) as f32 / fade_steps as f32;
                for (im, f) in image.iter_mut().zip(&site.fading) {
                    *im += w * (f - REST);
                }
                site.fade_left -= 1;
            }
            let r = site.reference.surface();
            let (mut pop, mut gap) = (0f32, 0f32);
            for c in 0..n {
                pop = pop.max(((image[c] - site.prev_image[c]) - (r[c] - site.prev_ref[c])).abs());
                gap = gap.max((image[c] - r[c]).abs());
            }
            site.gap[phase] = site.gap[phase].max(gap);
            site.pop[phase] = site.pop[phase].max(pop);
            site.prev_image.copy_from_slice(&image);
            site.prev_ref.copy_from_slice(r);
        }
        if s % 50 == 49 {
            eprintln!(
                "t = {:.1} s, budget {budget} ms, accordé {:.3} ms ; niveaux F/B/A {:?}/{:?}/{:?} ; écarts {:.1e}/{:.1e}/{:.1e} m",
                (s + 1) as f64 * dt,
                scheduler.granted_ms(),
                sites[0].level,
                sites[1].level,
                sites[2].level,
                sites[0].gap[phase],
                sites[1].gap[phase],
                sites[2].gap[phase]
            );
        }
    }
    let fmt = |a: &[f32; 4]| a.iter().map(|v| format!("{:.2e}", v)).collect::<Vec<_>>().join("/");
    for (i, site) in sites.iter().enumerate() {
        println!(
            "FAMINE_S403 temoin={} domaine={} contenu={} ecart_par_phase_m={} saut_par_phase_m={} descentes={} remontees={} pas_affames={}",
            u8::from(temoin),
            ["F", "B", "A"][i],
            if site.source { "source" } else { "bosse" },
            fmt(&site.gap),
            fmt(&site.pop),
            site.descents,
            site.ascents,
            site.starved_steps
        );
    }
    println!("FAMINE_S403 temoin={} accorde_sur_budget_max={worst_ratio:.4} couts_ms={cost_fine:.3}/{cost_coarse:.3}", u8::from(temoin));
}
