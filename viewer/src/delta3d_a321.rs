//! S391 — **A321** : à 30 Hz, la scène de la porte B explose en 24 à 40 s, quel que soit le solveur de pression.
//!
//! Le banc de l'attribution : la scène de `Config::review`, un pas `PAS_US=` (33 333), `SECONDES=` (60), `CYCLES=` (32,
//! Jacobi) ; `COMMUTATEURS=` éteint des termes du pas (`Step::switches` : 1 `u'·∇u'`, 2 `U·∇u'`, 4 `u'·∇U`, 8 le résidu du
//! fond, 16 la bande de B, 64 le terme de second ordre d'ADR-209, 32 rien) ; `EPONGE=0` retire l'éponge ; `PAQUET=0` le paquet ;
//! S440 : `RELATIF=1`, le mode relatif de δ. Chaque seconde : la plus grande hauteur
//! publiée et sa colonne, la plus grande vitesse de δ et sa face, et la **part de l'échelle de la maille** dans les
//! vitesses — `Σ(Δ²u)² / (16·Σu²)` selon chaque axe, 1 pour un damier pur, ≈ 0 pour un champ lisse. Lignes `A321_S391`.
use crate::delta3d_step::{face_total, Step3, Upto};
use water_core::delta3d::{Domain3, Sponge3};
use water_core::SimTime;

/// Part de l'échelle de la maille dans un champ de faces, selon un axe : `Σ(u[n−1] − 2u[n] + u[n+1])² / (16·Σu²)`.
fn part_maille(v: &[f32], dims: [usize; 3], axe: usize) -> f64 {
    let [nx, ny, nz] = dims;
    let at = |i: usize, j: usize, k: usize| v[(k * ny + j) * nx + i] as f64;
    let (mut num, mut den) = (0f64, 0f64);
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let c = at(i, j, k);
                den += c * c;
                let (lo, hi) = match axe {
                    0 if i > 0 && i + 1 < nx => (at(i - 1, j, k), at(i + 1, j, k)),
                    1 if j > 0 && j + 1 < ny => (at(i, j - 1, k), at(i, j + 1, k)),
                    2 if k > 0 && k + 1 < nz => (at(i, j, k - 1), at(i, j, k + 1)),
                    _ => continue,
                };
                let d2 = lo - 2. * c + hi;
                num += d2 * d2;
            }
        }
    }
    if den > 0. { num / (16. * den) } else { 0. }
}

pub fn banc() -> Result<(), String> {
    let env = |k: &str| std::env::var(k).ok();
    let pas_us: u64 = env("PAS_US").and_then(|v| v.parse().ok()).unwrap_or(33_333);
    let secondes: f64 = env("SECONDES").and_then(|v| v.parse().ok()).unwrap_or(60.);
    let commutateurs: u32 = env("COMMUTATEURS").and_then(|v| v.parse().ok()).unwrap_or(0);
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        // S409 / C3b : `MAILLE=` et `EMPRISE=`, la même scène à une autre maille (`Config::at_mesh`).
        let mut config = crate::delta3d_scene::Config::review_from_env()?;
        config.step_us = pas_us;
        if let Some(c) = env("CYCLES").and_then(|v| v.parse().ok()) {
            config.cycles = c;
        }
        if env("EPONGE").is_some_and(|v| v == "0") {
            config.sponge = Sponge3::default();
        }
        if env("PAQUET").is_some_and(|v| v == "0") {
            config = config.without_packet();
        }
        let (u, v, w, eta) = config.initial_state();
        let mut carte =
            Step3::new(background, config.domain, config.origin, crate::delta3d_scene::RHO, crate::delta3d_scene::G).await?;
        carte.set_step(config.step_us, config.rest, config.sponge)?;
        carte.set_state(&u, &v, &w, &eta)?;
        if !crate::delta3d_mg::cycles_du_banc().is_empty() {
            carte.enable_multigrid();
        }
        carte.set_switches_for_bench(commutateurs);
        // S440 (A322) : `RELATIF=1`, le mode relatif de δ (S369, A324, `Step3::set_relative`).
        // S443 : le défaut ; `RELATIF=0`, le pas de S297.
        let relatif = env("RELATIF").is_none_or(|v| v != "0");
        // S441 : la bande relative sous Lax-Wendroff — le défaut depuis S442 ; `BANDE_LW=0`, la bande centrée.
        carte.set_relative_band_lax_wendroff(env("BANDE_LW").is_none_or(|v| v != "0"));
        carte.set_relative(relatif);
        let Domain3 { nx, ny, nz, .. } = config.domain;
        let pas = (secondes * 1e6 / pas_us as f64).round() as u64;
        let par_seconde = (1e6 / pas_us as f64).round().max(1.) as u64;
        println!(
            "A321_S391 pas_us={pas_us} pas={pas} cycles={} multigrille={} commutateurs={commutateurs} relatif={relatif} eponge={:?} paquet={} m faces={}",
            config.cycles,
            carte.multigrid().is_some(),
            config.sponge,
            config.packet.amplitude,
            face_total(config.domain)
        );
        let nu = (nx + 1) * ny * nz;
        let nv = nx * (ny + 1) * nz;
        for n in 0..pas {
            carte.publish_time(background, SimTime(n * pas_us))?;
            carte.run_for_bench(config.cycles, Upto::Full)?;
            if (n + 1) % par_seconde != 0 && n + 1 != pas {
                continue;
            }
            let h = carte.published()?;
            let vel = carte.velocities()?;
            let t = (n + 1) as f64 * pas_us as f64 * 1e-6;
            let fini = h.iter().all(|x| x.is_finite()) && vel.iter().all(|x| x.is_finite());
            if !fini {
                println!("A321_S391 explose t={t:.2} pas={}", n + 1);
                return Ok(());
            }
            let (ch, hmax) = h.iter().enumerate().fold((0, 0f32), |(c, m), (i, x)| if x.abs() > m { (i, x.abs()) } else { (c, m) });
            let (fmax, umax) = vel.iter().enumerate().fold((0, 0f32), |(c, m), (i, x)| if x.abs() > m { (i, x.abs()) } else { (c, m) });
            let lieu = if fmax < nu {
                ("u", fmax % (nx + 1), (fmax / (nx + 1)) % ny, fmax / ((nx + 1) * ny))
            } else if fmax < nu + nv {
                let f = fmax - nu;
                ("v", f % nx, (f / nx) % (ny + 1), f / (nx * (ny + 1)))
            } else {
                let f = fmax - nu - nv;
                ("w", f % nx, (f / nx) % ny, f / (nx * ny))
            };
            let pu = part_maille(&vel[..nu], [nx + 1, ny, nz], 0).max(part_maille(&vel[..nu], [nx + 1, ny, nz], 1));
            let pz = part_maille(&vel[..nu], [nx + 1, ny, nz], 2);
            let pw = part_maille(&vel[nu + nv..], [nx, ny, nz + 1], 0).max(part_maille(&vel[nu + nv..], [nx, ny, nz + 1], 1));
            let d = carte.diagnostics_now()?;
            println!(
                "A321_S391 t={t:.1} max_eta={hmax:.4e} colonne=({},{}) max_u={umax:.4e} face={}({},{},{}) maille_u_horiz={pu:.3e} maille_u_vert={pz:.3e} maille_w_horiz={pw:.3e} divergence={:.2e} residu={:.2e}",
                ch % nx, ch / nx, lieu.0, lieu.1, lieu.2, lieu.3, d.divergence_plain, d.residual_relative
            );
        }
        println!("A321_S391 tient pas={pas}");
        Ok(())
    })
}
