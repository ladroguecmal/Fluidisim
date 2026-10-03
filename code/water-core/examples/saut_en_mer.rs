//! **S459 — C10-2 : le saut dans la mer δ** ([C10-SCENES-S454](../../../docs/validation/C10-SCENES-S454.md) §2), premier pas, au CPU.
//!
//! Une mer `Volume3` relative à B (4,8 m × 1,6 m, 1,4 m d'eau, 1,5 m d'air, 5 cm) ; au milieu, une bande `Apic3` de 1,6 m × 1,6 m en
//! **eau totale**, raccordée par `BandInSea` (S447–S448) ; le saut de B10 au centre de la bande (une sphère de 0,4 m à 4 m/s, arrêtée
//! à 0,6 m sous la surface). **La couronne du raccord** (`ANNEAU` colonnes de chaque bord en `x`) est **épinglée en colonnes**
//! (`ColumnsSwitch::pinned_columns`) : jamais de particules au raccord — la réponse au déclencheur de c3 (APIC-CARTE-S416 §23.6).
//!
//!     cargo run -p water-core --release --offline --example saut_en_mer -- [eps_b] [durée_s]
//!
//! Trois passages : (A) dans la mer, sans houle ; (B) **le témoin** — la même bande seule, à parois, sans houle ; (C) dans la mer, sous
//! une houle calme (`eps_b`, 0,0628 : 2 cm à 2 m) pendant `durée_s` (3 s). Publié : le cratère de (A) contre (B) à `t·√(g/D)` = 1
//! (hauteurs des colonnes, hors de celles qui traversent le corps) ; pour (A) et (C), la masse au raccord (le volume de δ moins ce
//! que son bilan explique, en fraction du volume de la bande), les colonnes épinglées passées en particules, le refus éventuel.

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::apic3d::{Apic3, ColumnsSwitch, LinearSwell, Sphere3};
use water_core::background::{Background, SeaState};
use water_core::band_in_sea::BandInSea;
use water_core::delta3d::{BackgroundGrid3, Domain3, Sponge3, Volume3};
use water_core::host::HostServices;
use water_core::{SimTime, WorldPos};

const G: f64 = 9.81;
const LAMBDA: f64 = 2.;
const DX: f64 = 0.05;
const PROFONDEUR: f64 = 1.4;
const AIR: f64 = 1.5;
const NX_MER: usize = 96;
const NA: usize = 32;
const I0: usize = 32;
const NY: usize = 32;
const ANNEAU: usize = 4;
const MARGE: usize = 3;
const RHO: f32 = 1000.;
const D: f64 = 0.4;
const FR: f64 = 2.;
const ARRET: f64 = 0.6;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Mer,
    Seule,
}

struct Issue {
    /// Les hauteurs de surface des colonnes de la bande à t·√(g/D) = 1.
    hauteurs: Vec<f64>,
    raccord_sur_volume: f64,
    epinglees_en_particules: usize,
    refus: Option<String>,
    duree_atteinte: f64,
    pas: u64,
    calcul_s: f64,
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let eps_b: f64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0628);
    let duree: f64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(3.);
    let echelle = (D / G).sqrt();
    // `SAUT_HOULE_SEULE=1` : le passage (C) seul.
    if std::env::var("SAUT_HOULE_SEULE").is_ok() {
        let c = run(eps_b, duree, Mode::Mer)?;
        println!(
            "SAUT_EN_MER_S459 passage=mer_houle eps_b={eps_b} duree_atteinte_s={:.2} raccord_sur_volume={:.2e} epinglees_en_particules={}              refus={:?} pas={} calcul_s={:.1}",
            c.duree_atteinte, c.raccord_sur_volume, c.epinglees_en_particules, c.refus, c.pas, c.calcul_s
        );
        return Ok(());
    }
    let a = run(1e-4, 1.5 * echelle, Mode::Mer)?;
    let b = run(1e-4, 1.5 * echelle, Mode::Seule)?;
    // Le cratère : (A) contre (B), hors des colonnes dont la surface lue tombe dans le corps.
    let r = 0.5 * D;
    let corps_z = PROFONDEUR + r - (FR * (G * D).sqrt() * echelle).min(ARRET);
    let (mut max, mut ecarts) = (0f64, Vec::new());
    for j in 0..NY {
        for i in 0..NA {
            let c = j * NA + i;
            let (ha, hb) = (a.hauteurs[c], b.hauteurs[c]);
            if ha.is_nan() || hb.is_nan() {
                continue;
            }
            let (x, y) = ((i as f64 + 0.5) * DX - NA as f64 * DX / 2., (j as f64 + 0.5) * DX - NY as f64 * DX / 2.);
            if (x * x + y * y).sqrt() < r + DX && (hb - corps_z).abs() < r + DX {
                continue;
            }
            max = f64::max(max, (ha - hb).abs());
            ecarts.push((ha - hb).abs());
        }
    }
    ecarts.sort_by(|x, y| x.total_cmp(y));
    println!(
        "SAUT_EN_MER_S459 cratere_t1 ecart_max_sur_dx={:.3} ecart_median_sur_dx={:.4} colonnes={}",
        max / DX,
        ecarts.get(ecarts.len() / 2).copied().unwrap_or(f64::NAN) / DX,
        ecarts.len()
    );
    for (nom, x) in [("mer_sans_houle", &a), ("seule", &b)] {
        println!(
            "SAUT_EN_MER_S459 passage={nom} raccord_sur_volume={:.2e} epinglees_en_particules={} refus={:?} pas={} calcul_s={:.1}",
            x.raccord_sur_volume, x.epinglees_en_particules, x.refus, x.pas, x.calcul_s
        );
    }
    // `SAUT_SANS_HOULE=1` : sans le passage (C).
    if std::env::var("SAUT_SANS_HOULE").is_ok() {
        return Ok(());
    }
    let c = run(eps_b, duree, Mode::Mer)?;
    println!(
        "SAUT_EN_MER_S459 passage=mer_houle eps_b={eps_b} duree_atteinte_s={:.2} raccord_sur_volume={:.2e} epinglees_en_particules={} \
         refus={:?} pas={} calcul_s={:.1}",
        c.duree_atteinte, c.raccord_sur_volume, c.epinglees_en_particules, c.refus, c.pas, c.calcul_s
    );
    Ok(())
}

fn run(eps_b: f64, duree: f64, mode: Mode) -> Result<Issue, String> {
    let debut = std::time::Instant::now();
    let en_mer = mode == Mode::Mer;
    let k = std::f64::consts::TAU / LAMBDA;
    let nz = ((PROFONDEUR + AIR) / DX).round() as usize;
    let dv = Domain3 { nx: NX_MER, ny: NY, nz, dx: DX as f32 };
    let da = Domain3 { nx: NA, ny: NY, nz, dx: DX as f32 };
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 32);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let periode = (std::f64::consts::TAU * LAMBDA / G).sqrt();
    let houle = Background::configure(
        &mut hote,
        SeaState { hs: (eps_b / k * 2. * 2f64.sqrt()) as f32, tp: (2. * periode) as f32, theta_turns: 0., components: 1, graine: 7 },
        WorldPos::from_units(0, 0, 0),
    )
    .map_err(|e| format!("houle {e:?}"))?;
    let mut grille = BackgroundGrid3::configure(&mut hote, dv, [0., 0., -PROFONDEUR as f32], RHO).map_err(|e| format!("{e:?}"))?;
    grille.sample(&houle, SimTime(0)).map_err(|e| format!("{e:?}"))?;
    let phi = {
        let bg = grille.view().ok_or("fond")?;
        let (mut c, mut s) = (0f64, 0f64);
        for i in 0..NX_MER {
            let x = (i as f64 + 0.5) * DX;
            let e = bg.w[i].eta as f64;
            c += e * (k * x).cos();
            s += e * (k * x).sin();
        }
        (-s).atan2(c)
    };
    let x0 = I0 as f64 * DX;
    let ab = eps_b / k;
    let omega = (G * k).sqrt();
    let eta = |x: f64| PROFONDEUR + ab * (k * x + phi).cos();
    let vitesse = |x: f64, z: f64| {
        let amp = ab * omega * (k * (z - PROFONDEUR)).exp();
        let (s, c) = (k * x + phi).sin_cos();
        (amp * c, amp * s)
    };
    // Le corps : au centre de la bande, la descente de B10 arrêtée à `ARRET`.
    let (r, u) = (0.5 * D, FR * (G * D).sqrt());
    let (cx, cy, z0) = (NA as f64 * DX / 2., NY as f64 * DX / 2., PROFONDEUR + r);
    let sphere = |t: f64| {
        let descente = (u * t).min(ARRET);
        let v = if u * t < ARRET { -u } else { 0. };
        Sphere3 { center: [cx as f32, cy as f32, (z0 - descente) as f32], radius: r as f32, velocity: [0., 0., v as f32] }
    };
    let capacite = NA * NY * ((PROFONDEUR / DX).ceil() as usize + 4) * 8 * 2;
    let mut a = Apic3::configure(&mut hote, da, RHO, G as f32, capacite).map_err(|e| format!("{e:?}"))?;
    a.seed(&|p| {
        let (x, y, z) = (p[0] as f64 - cx, p[1] as f64 - cy, p[2] as f64 - z0);
        (p[2] as f64) < eta(p[0] as f64 + x0) && x * x + y * y + z * z >= r * r
    })
    .map_err(|e| format!("{e:?}"))?;
    a.set_particle_velocities(&|p| {
        let (x, z) = (p[0] as f64 + x0, p[2] as f64);
        let (uu, ww) = vitesse(x, z);
        let g = [[-k * ww, 0., k * uu], [0.; 3], [k * uu, 0., k * ww]];
        ([uu as f32, 0., ww as f32], g.map(|l| l.map(|v| v as f32)))
    })
    .map_err(|e| format!("{e:?}"))?;
    // La zone : la couronne du raccord ; le reste en particules jusqu'à la première bascule.
    let couronne = |c: usize| c % NA < ANNEAU || c % NA >= NA - ANNEAU;
    let mask: Vec<u8> = (0..NA * NY).map(|c| u8::from(couronne(c))).collect();
    a.enable_columns(&mut hote, &mask).map_err(|e| format!("{e:?}"))?;
    let surface: Vec<f32> = (0..NA * NY).map(|c| eta((c % NA) as f64 * DX + 0.5 * DX + x0) as f32).collect();
    a.set_columns_surface(&surface).map_err(|e| format!("{e:?}"))?;
    {
        let (mut uu, v, mut ww) = (vec![0f32; (NA + 1) * NY * nz], vec![0f32; NA * (NY + 1) * nz], vec![0f32; NA * NY * (nz + 1)]);
        for kk in 0..nz {
            for j in 0..NY {
                for i in 0..=NA {
                    let (x, z) = (i as f64 * DX + x0, (kk as f64 + 0.5) * DX);
                    if z <= eta(x) {
                        uu[(kk * NY + j) * (NA + 1) + i] = vitesse(x, z).0 as f32;
                    }
                }
            }
        }
        for kk in 1..nz {
            for j in 0..NY {
                for i in 0..NA {
                    let (x, z) = ((i as f64 + 0.5) * DX + x0, kk as f64 * DX);
                    if z <= eta(x) {
                        ww[(kk * NY + j) * NA + i] = vitesse(x, z).1 as f32;
                    }
                }
            }
        }
        a.set_grid_velocities(&uu, &v, &ww).map_err(|e| format!("{e:?}"))?;
    }
    a.set_body(Some(sphere(0.))).map_err(|e| format!("{e:?}"))?;
    let mut bascule = ColumnsSwitch::with_capacity(&mut hote, a.domain()).map_err(|e| format!("{e:?}"))?;
    bascule.hold_us = 300_000;
    bascule.floor_cells = Some(4);
    bascule.floor_speed = Some(0.3);
    bascule.floor_speed_release = Some(0.15);
    bascule.background = Some(LinearSwell {
        amplitude: ab as f32,
        wavenumber: k as f32,
        omega: omega as f32,
        phase: (phi + k * x0) as f32,
        mean_level: PROFONDEUR as f32,
    });
    // S459 : la couronne du raccord, épinglée en colonnes (dans la mer comme dans le témoin : la même bascule).
    bascule.pinned_columns = Some((0..NA * NY).map(couronne).collect());
    let mut mer = Volume3::configure(&mut hote, dv, RHO, G as f32).map_err(|e| format!("{e:?}"))?;
    mer.set_free_surface(&vec![PROFONDEUR as f32; dv.columns()], PROFONDEUR as f32).map_err(|e| format!("{e:?}"))?;
    let eponge = Sponge3 { width_x: 0.8, width_y: 0., rate_per_s: 2. };
    if en_mer {
        a.enable_open_boundaries(&mut hote).map_err(|e| format!("{e:?}"))?;
    }
    let mut raccord = if en_mer {
        let mut rc = BandInSea::configure(&mut hote, dv, da, I0, MARGE).map_err(|e| format!("{e:?}"))?;
        // `SAUT_HAUTEUR_LUE=1` : les colonnes de particules de l'intérieur donnent à la mer leur hauteur lue (S449 : instable quand
        // elles touchaient le raccord ; ici la couronne épinglée les en tient loin).
        rc.set_particle_heights(std::env::var("SAUT_HAUTEUR_LUE").is_ok());
        // `SAUT_ANNEAU=<colonnes>` : la mer ne reçoit les vitesses de la bande que sur cet anneau (S459).
        rc.set_velocity_ring(std::env::var("SAUT_ANNEAU").ok().and_then(|v| v.parse().ok()));
        Some(rc)
    } else {
        None
    };
    let volume_bande = NA as f64 * NY as f64 * DX * DX * PROFONDEUR;
    let echelle = (D / G).sqrt();
    let (mut t_us, mut pas) = (0u64, 0u64);
    let (mut apports, mut raccord_max, mut epinglees) = (0f64, 0f64, 0usize);
    let mut deplace_porte = 0f64;
    let mut hauteurs = Vec::new();
    let mut refus = None;
    let volume = |m: &Volume3| m.surface().iter().map(|e| (*e - PROFONDEUR as f32) as f64).sum::<f64>() * DX * DX;
    'pas: while (t_us as f64) * 1e-6 < duree {
        let t = t_us as f64 * 1e-6;
        a.set_body(Some(sphere(t))).map_err(|e| format!("{e:?}"))?;
        let us = a.stable_step_us(10_000);
        let ts = SimTime(t_us);
        grille.sample(&houle, ts).map_err(|e| format!("{e:?}"))?;
        let bg = grille.view().ok_or("fond")?;
        // S459 : le volume que le corps déplace — la calotte de la sphère sous le niveau moyen ; sa variation, un apport de la mer.
        let immerge = {
            let zc = sphere(t).center[2] as f64;
            let d = (PROFONDEUR - (zc - r)).clamp(0., 2. * r);
            std::f64::consts::PI * d * d * (3. * r - d) / 3.
        };
        if let Some(rc) = raccord.as_mut() {
            if std::env::var("SAUT_SANS_DEPLACE").is_err() {
                rc.set_displaced(immerge);
                if pas > 0 {
                    apports += immerge - deplace_porte;
                    deplace_porte = immerge;
                }
            }
            if pas > 0 {
                if let Err(e) = rc.feed_sea(&mut mer, &mut a, &bg) {
                    refus = Some(format!("raccord, pas {pas} : {e:?}"));
                    break 'pas;
                }
            }
            rc.feed_band(&mer, &mut a, &bg).map_err(|e| format!("{e:?}"))?;
        }
        if let Err(e) = a.step(us) {
            refus = Some(format!("bande, pas {pas} : {e:?}"));
            break;
        }
        if en_mer {
            if let Err(e) = mer.step_perturbation_mobile(ts, us, 4000, &bg, eponge, &jobs) {
                refus = Some(format!("mer, pas {pas} : {e:?}"));
                break;
            }
            let b = mer.balance();
            apports += b.band_in + b.perturbation_in - b.sponge_out;
            raccord_max = raccord_max.max((volume(&mer) - apports).abs());
        }
        t_us += us;
        pas += 1;
        bascule.switch(t_us, &mut a).map_err(|e| format!("bascule {e:?}"))?;
        if pas == 1 {
            bascule.clear_counts();
        }
        epinglees = epinglees.max((0..NA * NY).filter(|&c| couronne(c) && !a.is_column(c % NA, c / NA)).count());
        if hauteurs.is_empty() && t_us as f64 * 1e-6 >= echelle {
            let phi_c = a.distance();
            hauteurs = (0..NA * NY)
                .map(|c| {
                    let f = |kk: usize| phi_c[kk * NA * NY + c] as f64;
                    (0..nz - 1)
                        .rev()
                        .find(|&kk| f(kk) < 0. && f(kk + 1) >= 0.)
                        .map_or(f64::NAN, |kk| (kk as f64 + 0.5) * DX + DX * f(kk) / (f(kk) - f(kk + 1)))
                })
                .collect();
        }
    }
    Ok(Issue {
        hauteurs,
        raccord_sur_volume: raccord_max / volume_bande,
        epinglees_en_particules: epinglees,
        refus,
        duree_atteinte: t_us as f64 * 1e-6,
        pas,
        calcul_s: debut.elapsed().as_secs_f64(),
    })
}
