//! **S446 — c2 de C7d-3c : le raccord bande ↔ mer** ([ADR-214](../../../docs/adr/ADR-214-b-entre-dans-la-bande.md), note de S445).
//!
//! Un domaine `Apic3` en **eau totale** (ici sa seule zone de colonnes, aux bords ouverts en `x`) posé dans un domaine `Volume3` en
//! **δ relatif** (le défaut depuis S443), sur la même grille : 25 cm, 2 rangées, 2,5 m d'eau ; la mer fait 16 m, la bande 4 m au
//! milieu. B : la houle de S369 (une composante, 4 m, `T` = 1,6 s). À chaque pas : (1) la mer reçoit, à l'intérieur de la bande (à
//! `MARGE` colonnes de ses bords), `δ = total − B` — hauteurs et vitesses ; (2) la bande reçoit à ses bords la vitesse normale
//! `B + δ` ; (3) la bande avance ; (4) la mer avance.
//!
//!     cargo run -p water-core --release --offline --example raccord_bande_mer -- [houle_m] [durée_s]
//!
//! Publié : le δ créé dans la mer **hors** de la bande (son maximum, et quand), la dérive du volume de δ de la mer, et l'écart de la
//! bande à B en son milieu. Critères de c2 : `notes/EN-COURS.md`, S446.

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::apic3d::Apic3;
use water_core::background::{Background, SeaState};
use water_core::delta3d::{BackgroundGrid3, Domain3, Sponge3, Volume3};
use water_core::host::HostServices;
use water_core::{SimTime, WorldPos};

const DX: f32 = 0.25;
const NX: usize = 64;
const NY: usize = 2;
const H0: f32 = 2.5;
const I0: usize = 24;
const NA: usize = 16;
/// `RACCORD_MARGE=<n>` remplace la marge (deux colonnes par défaut).
const MARGE_DEFAUT: usize = 2;
const RHO: f32 = 1025.;
const DT_US: u64 = 10_000;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let a_houle: f32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.05);
    let duree: f64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10.);
    #[allow(non_snake_case)]
    let MARGE: usize = std::env::var("RACCORD_MARGE").ok().and_then(|v| v.parse().ok()).unwrap_or(MARGE_DEFAUT);
    let nz = (H0 / DX) as usize + 4;
    let dv = Domain3 { nx: NX, ny: NY, nz, dx: DX };
    let da = Domain3 { nx: NA, ny: NY, nz, dx: DX };
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let houle = Background::configure(
        &mut hote,
        SeaState { hs: a_houle * 2. * 2f32.sqrt(), tp: 3.2, theta_turns: 0., components: 1, graine: 7 },
        WorldPos::from_units(0, 0, 0),
    )
    .map_err(|e| format!("houle {e:?}"))?;
    let g = houle.gravity();
    let mut grille = BackgroundGrid3::configure(&mut hote, dv, [0., 0., -H0], RHO).map_err(|e| format!("{e:?}"))?;
    let mut mer = Volume3::configure(&mut hote, dv, RHO, g).map_err(|e| format!("{e:?}"))?;
    mer.set_free_surface(&vec![H0; dv.columns()], H0).map_err(|e| format!("{e:?}"))?;
    let mut bande = Apic3::configure(&mut hote, da, RHO, g, 16).map_err(|e| format!("{e:?}"))?;
    bande.enable_columns(&mut hote, &vec![1u8; NA * NY]).map_err(|e| format!("{e:?}"))?;
    bande.enable_open_boundaries(&mut hote).map_err(|e| format!("{e:?}"))?;
    let eponge = Sponge3 { width_x: 3., width_y: 0., rate_per_s: 2. };

    // Les indices de faces de la mer.
    let fu = |i: usize, j: usize, k: usize| (k * NY + j) * (NX + 1) + i;
    let fv = |i: usize, j: usize, k: usize| (k * (NY + 1) + j) * NX + i;
    let fw = |i: usize, j: usize, k: usize| (k * NY + j) * NX + i;
    // Et ceux de la bande.
    let au = |i: usize, j: usize, k: usize| (k * NY + j) * (NA + 1) + i;
    let av = |i: usize, j: usize, k: usize| (k * (NY + 1) + j) * NA + i;
    let aw = |i: usize, j: usize, k: usize| (k * NY + j) * NA + i;

    // L'état initial de la bande : l'eau totale de B à t = 0 (δ nul).
    grille.sample(&houle, SimTime(0)).map_err(|e| format!("{e:?}"))?;
    {
        let bg = grille.view().ok_or("fond")?;
        let eta: Vec<f32> = (0..NA * NY).map(|c| H0 + bg.w[fw(I0 + c % NA, c / NA, 0)].eta).collect();
        bande.set_columns_surface(&eta).map_err(|e| format!("{e:?}"))?;
        let mut u = vec![0f32; (NA + 1) * NY * nz];
        let mut v = vec![0f32; NA * (NY + 1) * nz];
        let mut w = vec![0f32; NA * NY * (nz + 1)];
        for k in 0..nz {
            for j in 0..NY {
                for i in 0..=NA {
                    u[au(i, j, k)] = bg.u[fu(I0 + i, j, k)].u[0];
                }
            }
            for j in 1..NY {
                for i in 0..NA {
                    v[av(i, j, k)] = bg.v[fv(I0 + i, j, k)].u[1];
                }
            }
        }
        for k in 0..=nz {
            for j in 0..NY {
                for i in 0..NA {
                    w[aw(i, j, k)] = bg.w[fw(I0 + i, j, k)].u[2];
                }
            }
        }
        bande.set_grid_velocities(&u, &v, &w).map_err(|e| format!("{e:?}"))?;
    }

    let pas = (duree * 1e6 / DT_US as f64).round() as u64;
    let volume = |m: &Volume3| m.surface().iter().map(|e| (*e - H0) as f64).sum::<f64>() * (DX * DX) as f64;
    let (mut dehors_max, mut quand, mut derive_max, mut milieu_max) = (0f32, 0f64, 0f64, 0f32);
    for n in 0..pas {
        let t = SimTime(n * DT_US);
        grille.sample(&houle, t).map_err(|e| format!("{e:?}"))?;
        let bg = grille.view().ok_or("fond")?;
        // (1) La mer reçoit `δ = total − B` à l'intérieur de la bande.
        if n > 0 {
            let mut eta = mer.surface().to_vec();
            let (mut u, mut v, mut w) = (mer.velocity_u().to_vec(), mer.velocity_v().to_vec(), mer.velocity_w().to_vec());
            let colonnes = bande.columns_surface().ok_or("zone")?;
            let (ua, va, wa) = (bande.velocity_u(), bande.velocity_v(), bande.velocity_w());
            for j in 0..NY {
                for i in MARGE..NA - MARGE {
                    eta[j * NX + I0 + i] = colonnes[j * NA + i] - bg.w[fw(I0 + i, j, 0)].eta;
                }
            }
            for k in 0..nz {
                for j in 0..NY {
                    for i in MARGE..=NA - MARGE {
                        u[fu(I0 + i, j, k)] = ua[au(i, j, k)] - bg.u[fu(I0 + i, j, k)].u[0];
                    }
                }
                for j in 1..NY {
                    for i in MARGE..NA - MARGE {
                        v[fv(I0 + i, j, k)] = va[av(i, j, k)] - bg.v[fv(I0 + i, j, k)].u[1];
                    }
                }
            }
            for k in 1..nz {
                for j in 0..NY {
                    for i in MARGE..NA - MARGE {
                        w[fw(I0 + i, j, k)] = wa[aw(i, j, k)] - bg.w[fw(I0 + i, j, k)].u[2];
                    }
                }
            }
            mer.set_surface(&eta).map_err(|e| format!("pas {n} : {e:?}"))?;
            mer.set_velocity(&u, &v, &w).map_err(|e| format!("pas {n} : {e:?}"))?;
        }
        // (2) La bande reçoit à ses bords la vitesse normale `B + δ`.
        let (mu, mut gauche, mut droite) = (mer.velocity_u(), vec![0f32; NY * nz], vec![0f32; NY * nz]);
        for k in 0..nz {
            for j in 0..NY {
                gauche[k * NY + j] = bg.u[fu(I0, j, k)].u[0] + mu[fu(I0, j, k)];
                droite[k * NY + j] = bg.u[fu(I0 + NA, j, k)].u[0] + mu[fu(I0 + NA, j, k)];
            }
        }
        bande.set_open_boundaries(&gauche, &droite).map_err(|e| format!("{e:?}"))?;
        // (3) La bande avance ; (4) la mer avance.
        bande.step(DT_US).map_err(|e| format!("bande, pas {n} : {e:?}"))?;
        mer.step_perturbation_mobile(t, DT_US, 4000, &bg, eponge, &jobs).map_err(|e| format!("mer, pas {n} : {e:?}"))?;
        // Mesures.
        let s = mer.surface();
        let tn = (n + 1) as f64 * DT_US as f64 * 1e-6;
        for j in 0..NY {
            for i in (0..I0).chain(I0 + NA..NX) {
                let d = (s[j * NX + i] - H0).abs();
                if d > dehors_max {
                    dehors_max = d;
                    quand = tn;
                }
            }
        }
        derive_max = derive_max.max(volume(&mer).abs());
        // L'écart de la bande à B en son milieu, au début du pas suivant : lu après le prochain échantillon ; ici, au pas courant.
        let colonnes = bande.columns_surface().ok_or("zone")?;
        grille.sample(&houle, SimTime((n + 1) * DT_US)).map_err(|e| format!("{e:?}"))?;
        let bg2 = grille.view().ok_or("fond")?;
        for j in 0..NY {
            let i = NA / 2;
            let e = (colonnes[j * NA + i] - (H0 + bg2.w[fw(I0 + i, j, 0)].eta)).abs();
            milieu_max = milieu_max.max(e);
        }
        if std::env::var("RACCORD_TRACE").is_ok() && (n + 1) % 100 == 0 {
            eprintln!("TRACE t={tn:.1} dehors_max={dehors_max:.4e} volume={:.3e} milieu={milieu_max:.4e}", volume(&mer));
        }
    }
    // Le volume que la houle fait passer par une face de la bande en une demi-période, sur la largeur : `2a/k · largeur`.
    let k = 2. * core::f64::consts::PI / 4.;
    let demi = 2. * a_houle as f64 / k * (NY as f64 * DX as f64);
    println!(
        "RACCORD_S446 houle_m={a_houle} duree_s={duree} dx={DX} mer={NX}x{NY}x{nz} bande={NA} colonnes en {I0} marge={MARGE} \
         delta_hors_bande_max_m={dehors_max:.4e} a_t={quand:.2} derive_volume_max_m3={derive_max:.3e} demi_periode_m3={demi:.3e} \
         part={:.3} bande_contre_b_milieu_max_m={milieu_max:.4e}",
        derive_max / demi
    );
    Ok(())
}
