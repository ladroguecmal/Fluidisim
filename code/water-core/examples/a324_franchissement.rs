//! **A324 — S436 : la surface de B qui franchit un centre de maille.** En mode relatif (masque 7), sous une houle de plus d'une
//! demi-maille, δ est amplifié à l'échelle de la maille ([MER-S369](../../../docs/validation/MER-S369.md) §7). Ce banc réduit
//! (20 m × 2 rangées à 12,5 cm, 2,5 m d'eau, la houle de S369 : `λ` = 4 m, `T` = 1,6 s) suit le saut **pas à pas** : à chaque pas,
//! le maximum de `|η′|` et de `|u′|`, `|w′|`, leur place, l'élévation de B à cette colonne, et si la mouillure totale d'une colonne
//! vient de changer (une maille ouverte ou fermée par la surface totale).
//!
//!     cargo run -p water-core --release --offline --example a324_franchissement -- <houle_m> [pas] [germe_m]
//!
//! `A324_DUREE=<s>` (1 s par défaut) ; `A324_MASQUE` (7 par défaut) ; `A324_PROPRE=1`, le remède de S436. Le témoin, sans germe, tourne à côté : il doit rester nul au bit.

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::background::{Background, SeaState};
use water_core::delta3d::{BackgroundGrid3, Domain3, Sponge3, Volume3};
use water_core::host::HostServices;
use water_core::{SimTime, WorldPos};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let a_houle: f32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.065);
    let dt_us: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10_000);
    let germe: f32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.001);
    let duree: f64 = std::env::var("A324_DUREE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);
    let masque: u8 = std::env::var("A324_MASQUE").ok().and_then(|v| v.parse().ok()).unwrap_or(Volume3::RELATIVE_ALL);
    let (dx, h0) = (0.125f32, 2.5f32);
    let d = Domain3 { nx: 160, ny: 2, nz: (h0 / dx) as usize + 2, dx };
    let (nx, ny, nz) = (d.nx, d.ny, d.nz);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let houle = Background::configure(
        &mut hote,
        SeaState { hs: a_houle * 2. * 2f32.sqrt(), tp: 3.2, theta_turns: 0., components: 1, graine: 7 },
        WorldPos::from_units(0, 0, 0),
    )
    .map_err(|e| format!("houle {e:?}"))?;
    let mut grille = BackgroundGrid3::configure(&mut hote, d, [0., 0., -h0], 1025.).map_err(|e| format!("{e:?}"))?;
    let mut volumes = Vec::new();
    for avec_germe in [true, false] {
        let mut v = Volume3::configure(&mut hote, d, 1025., houle.gravity()).map_err(|e| format!("{e:?}"))?;
        let mut surface = vec![h0; d.columns()];
        if avec_germe {
            for i in 70..90 {
                for j in 0..ny {
                    surface[j * nx + i] = h0 + germe * ((i as f32 - 80.) * 0.3).cos();
                }
            }
        }
        v.set_free_surface(&surface, h0).map_err(|e| format!("{e:?}"))?;
        v.set_relative_background(masque).map_err(|e| format!("{e:?}"))?;
        // `A324_PROPRE=1` : l'erreur de B aux fantômes latéraux prise au point de surface de B (S436).
        v.set_lateral_own_ghost(std::env::var("A324_PROPRE").is_ok());
        volumes.push(v);
    }
    let sponge = Sponge3 { width_x: 3., width_y: 0., rate_per_s: 2. };
    let pas = (duree * 1e6 / dt_us as f64).round() as u64;
    let fw = |i: usize, j: usize, k: usize| (k * ny + j) * nx + i;
    let fu = |i: usize, j: usize, k: usize| (k * ny + j) * (nx + 1) + i;
    // La maille mouillée la plus haute de chaque colonne, sous la surface totale.
    let sommet = |surface_totale: f32| -> i64 { ((surface_totale / dx) - 0.5).ceil() as i64 - 1 };
    let mut sommets_avant: Vec<i64> = vec![-1; nx];
    for n in 0..pas {
        let t = SimTime(n * dt_us);
        grille.sample(&houle, t).map_err(|e| format!("{e:?}"))?;
        let bg = grille.view().ok_or("fond")?;
        let mut sommets = vec![0i64; nx];
        for (i, s) in sommets.iter_mut().enumerate() {
            *s = sommet(volumes[0].surface()[i] + bg.w[fw(i, 0, 0)].eta);
        }
        for v in volumes.iter_mut() {
            v.step_perturbation_mobile(t, dt_us, 200, &bg, sponge, &jobs).map_err(|e| format!("pas {n} : {e:?}"))?;
        }
        let v = &volumes[0];
        let (mut emax, mut ie) = (0f32, 0usize);
        for i in 0..nx {
            let e = (v.surface()[i] - h0).abs();
            if e > emax {
                emax = e;
                ie = i;
            }
        }
        let (mut umax, mut pu) = (0f32, (0usize, 0usize));
        for k in 0..nz {
            for i in 0..=nx {
                let x = v.velocity_u()[fu(i, 0, k)].abs();
                if x > umax {
                    umax = x;
                    pu = (i, k);
                }
            }
        }
        let (mut wmax, mut pw) = (0f32, (0usize, 0usize));
        for k in 0..=nz {
            for i in 0..nx {
                let x = v.velocity_w()[fw(i, 0, k)].abs();
                if x > wmax {
                    wmax = x;
                    pw = (i, k);
                }
            }
        }
        let changes: Vec<String> = (0..nx)
            .filter(|&i| n > 0 && sommets[i] != sommets_avant[i])
            .map(|i| format!("{i}:{}→{}", sommets_avant[i], sommets[i]))
            .take(6)
            .collect();
        let temoin = volumes[1].surface().iter().fold(0f32, |m, e| m.max((e - h0).abs()));
        println!(
            "pas {n:4} t={:.2} eta′max={:.2e}@{ie} (B {:+.4}, sommet {}) u′max={:.2e}@{:?} w′max={:.2e}@{:?} temoin={temoin:.1e} \
             changes[{}] {}",
            (n + 1) as f64 * dt_us as f64 * 1e-6,
            emax,
            bg.w[fw(ie, 0, 0)].eta,
            sommets[ie],
            umax,
            pu,
            wmax,
            pw,
            (0..nx).filter(|&i| n > 0 && sommets[i] != sommets_avant[i]).count(),
            changes.join(" ")
        );
        sommets_avant = sommets;
    }
    Ok(())
}
