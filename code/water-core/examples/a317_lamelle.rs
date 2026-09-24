//! **S334 — A317 hors du jeu** : une coque 4 × 1,6 × 1 m qui perce le couvercle de δ, en **pilonnement imposé**
//! — 5 cm à 4,48 rad/s, sa pulsation propre, démarrage progressif sur une période —, dans un δ linéaire de
//! 16 × 16 m sur 2 m, mailles de 25 cm, pas de 10 ms.
//!
//! Cinq placements sous la maille : `φ`, la part d'eau que la paroi −y laisse dans sa maille de bord ; la paroi
//! +y, 1,6 m plus loin, en laisse `1 − frac(φ + 0,4)`. La mesure : l'amplitude quadratique moyenne de δ sur
//! une bande à 2,5–3,5 m de chaque flanc long, `|x − x_c| ≤ 2 m`, entre 3 et 6 s — avant que les murs ne
//! renvoient les anneaux. Une coque symétrique rayonne pareil de ses deux flancs, quel que soit son placement.
//!
//! `cargo run -p water-core --release --offline --example a317_lamelle` — lignes `A317`, ≈ 5 min.
#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::body::Milieu;
use water_core::delta3d::{Domain3, Volume3};
use water_core::delta_projection::PROJECTION_DIVERGENCE_TOLERANCE;
use water_core::host::HostServices;
use water_core::rigid_body::oriented_box_distance;

const DT: f64 = 0.01;
const DT_US: u64 = 10_000;
const PAS: u64 = 600;
const MAILLES: [usize; 3] = [64, 64, 8];
const DX: f64 = 0.25;
const DEMI: [f64; 3] = [2., 0.8, 0.5];
/// Pilonnement imposé : amplitude, pulsation — la pulsation propre de la coque, `√(ρgA/m)`.
const AMPLITUDE: f64 = 0.05;
const OMEGA: f64 = 4.484;
/// La paroi −y dans la maille 28 ; le centre en `x` laisse 40 et 60 % d'eau aux parois ±x.
const MAILLE_PAROI: f64 = 28.;
const CENTRE_X: f64 = 8.1;

/// La coque à l'altitude relative `h`, placée pour que la paroi −y laisse `phi` d'eau dans sa maille : centre
/// arrondi en f32, comme δ le reçoit.
fn centre(phi: f64, h: f64, z0: f64) -> [f32; 3] {
    let z_r = 0.5 - 500. / Milieu::MER.rho;
    [CENTRE_X as f32, ((MAILLE_PAROI + phi) * DX + DEMI[1]) as f32, (z0 + z_r + h) as f32]
}

fn noeuds(c: [f32; 3], out: &mut Vec<f32>) {
    let c = c.map(|v| v as f64);
    out.clear();
    for k in 0..=MAILLES[2] {
        for j in 0..=MAILLES[1] {
            for i in 0..=MAILLES[0] {
                out.push(oriented_box_distance(c, [1., 0., 0., 0.], DEMI, [i as f64 * DX, j as f64 * DX, k as f64 * DX]) as f32);
            }
        }
    }
}

/// Le pilonnement imposé à l'instant `t`, démarré sur une période.
fn pilonnement(t: f64) -> f64 {
    let periode = 2. * std::f64::consts::PI / OMEGA;
    let rampe = if t < periode { 0.5 * (1. - (std::f64::consts::PI * t / periode).cos()) } else { 1. };
    AMPLITUDE * rampe * (OMEGA * t).sin()
}

/// Une scène : rend les amplitudes quadratiques moyennes des bandes −y et +y, m.
fn scene(phi: f64) -> Result<(f64, f64), String> {
    let d = Domain3 { nx: MAILLES[0], ny: MAILLES[1], nz: MAILLES[2], dx: DX as f32 };
    let z0 = d.z0() as f64;
    let mut c = centre(phi, 0., z0);
    let mut n = Vec::new();
    noeuds(c, &mut n);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 27);
    let mut v = Volume3::configure_with_floating_solid(
        &mut HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink },
        d, Milieu::MER.rho as f32, 9.81, &vec![0.; d.columns()], &n,
    ).map_err(|e| format!("configuration : {e:?}"))?;
    v.set_surface(&vec![d.z0(); d.columns()]).map_err(|e| format!("{e:?}"))?;
    let paroi = (MAILLE_PAROI + phi) * DX;
    let bande = |y: f64| -> Option<usize> {
        if y > paroi - 3.5 && y < paroi - 2.5 { Some(0) } else if y > paroi + 1.6 + 2.5 && y < paroi + 1.6 + 3.5 { Some(1) } else { None }
    };
    let (mut somme, mut compte) = ([0f64; 2], [0usize; 2]);
    for pas in 0..PAS {
        let t = (pas + 1) as f64 * DT;
        let suivant = centre(phi, pilonnement(t), z0);
        let vitesse = [0., 0., (suivant[2] - c[2]) / DT as f32];
        c = suivant;
        noeuds(c, &mut n);
        v.set_solid_rigid(&n, vitesse, [0.; 3], c).map_err(|e| format!("pas {pas} : paroi {e:?}"))?;
        let r = v.step_surface_linear(DT_US, 4000, &host_impl::SequentialJobs).map_err(|e| format!("pas {pas} : δ {e:?}"))?;
        if r.divergence > PROJECTION_DIVERGENCE_TOLERANCE {
            return Err(format!("pas {pas} : divergence {}", r.divergence));
        }
        if t > 3. {
            for j in 0..d.ny {
                let y = (j as f64 + 0.5) * DX;
                let Some(b) = bande(y) else { continue };
                for i in 0..d.nx {
                    let x = (i as f64 + 0.5) * DX;
                    if (x - CENTRE_X).abs() <= 2. {
                        let h = (v.surface()[j * d.nx + i] - d.z0()) as f64;
                        somme[b] += h * h;
                        compte[b] += 1;
                    }
                }
            }
        }
    }
    Ok(((somme[0] / compte[0] as f64).sqrt(), (somme[1] / compte[1] as f64).sqrt()))
}

fn main() -> Result<(), String> {
    println!("A317 scene pilonnement_m={AMPLITUDE} omega_rad_s={OMEGA} delta={MAILLES:?} dx={DX} pas_s={DT} duree_s={} bandes=2.5-3.5m fenetre=3-6s",
        PAS as f64 * DT);
    let mut toutes = Vec::new();
    for phi in [0.05, 0.08, 0.30, 0.55, 0.80] {
        let (moins, plus) = scene(phi)?;
        let phi_plus = 1. - (phi + 0.4f64).fract();
        println!("A317 placement phi_moins={phi:.2} phi_plus={phi_plus:.2} amplitude_moins_m={moins:.6} amplitude_plus_m={plus:.6} rapport={:.4}", moins / plus);
        toutes.extend([moins, plus]);
    }
    let moyenne = toutes.iter().sum::<f64>() / toutes.len() as f64;
    let ecart = toutes.iter().fold(0f64, |m, a| m.max((a / moyenne - 1.).abs()));
    println!("A317 dispersion amplitude_moyenne_m={moyenne:.6} ecart_max_relatif={ecart:.4} critere=0.05");
    Ok(())
}
