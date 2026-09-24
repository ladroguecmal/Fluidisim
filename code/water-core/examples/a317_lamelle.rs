//! **S334 — A317 hors du jeu** : une coque qui perce le couvercle de δ, en **pilonnement imposé** — 5 cm à
//! 4,48 rad/s, démarrage progressif sur une période —, δ linéaire sur 2 m de fond, pas de 10 ms.
//!
//! **Scène 3D** (par défaut) : coque 4 × 1,6 × 1 m dans un δ de 16 × 16 m. **Tranche** (`--tranche`) : la même
//! section, 1,6 × 1 m, infiniment longue — `ny` = 2, la coque traverse le domaine en `y` —, dans une tranche de
//! 32 m : le même mécanisme de paroi, cent fois moins cher, donc plusieurs résolutions.
//!
//! Placements sous la maille : `φ`, la part d'eau que la première paroi laisse dans sa maille de bord ; la
//! seconde, une largeur de coque plus loin, en laisse `1 − frac(φ + 1,6/dx)`. La mesure : l'amplitude
//! quadratique moyenne de δ sur une bande à 2,5–3,5 m de chaque flanc, entre 3 et 6 s — avant que les murs ne
//! renvoient les ondes. Une coque symétrique rayonne pareil de ses deux flancs, quel que soit son placement.
//!
//! `cargo run -p water-core --release --offline --example a317_lamelle [-- options]` — lignes `A317` :
//! `--tranche`, `--dx <m>` (0,25), `--phi <a,b,…>` (0,05 ; 0,08 ; 0,30 ; 0,55 ; 0,80), `--couvercle-partiel`
//! (la surface des colonnes en partie couvertes de S334, `set_partial_lid`).
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
/// Pilonnement imposé : amplitude, pulsation — la pulsation propre de la coque 3D, `√(ρgA/m)`.
const AMPLITUDE: f64 = 0.05;
const OMEGA: f64 = 4.484;
const LARGEUR: f64 = 1.6;

#[derive(Clone, Copy)]
struct Banc {
    tranche: bool,
    dx: f64,
    partiel: bool,
}

impl Banc {
    fn domaine(&self) -> Domain3 {
        let n = |l: f64| (l / self.dx).round() as usize;
        if self.tranche {
            Domain3 { nx: n(32.), ny: 2, nz: n(2.), dx: self.dx as f32 }
        } else {
            Domain3 { nx: n(16.), ny: n(16.), nz: n(2.), dx: self.dx as f32 }
        }
    }
    /// La première paroi, dans la maille qui porte le placement : 15,2 m sur la tranche, 7 m en 3D.
    fn paroi(&self, phi: f64) -> f64 {
        let maille = ((if self.tranche { 15.2 } else { 7. }) / self.dx).round();
        (maille + phi) * self.dx
    }
    fn demi(&self) -> [f64; 3] {
        if self.tranche { [0.5 * LARGEUR, 1e3, 0.5] } else { [2., 0.5 * LARGEUR, 0.5] }
    }
    /// Centre à l'altitude relative `h`, arrondi en f32 comme δ le reçoit ; la 3D garde 40 et 60 % d'eau aux
    /// parois ±x à 25 cm.
    fn centre(&self, phi: f64, h: f64, z0: f64) -> [f32; 3] {
        let z = z0 + 0.5 - 500. / Milieu::MER.rho + h;
        let travers = self.paroi(phi) + 0.5 * LARGEUR;
        if self.tranche {
            [travers as f32, self.dx as f32, z as f32]
        } else {
            [8.1, travers as f32, z as f32]
        }
    }
}

fn noeuds(b: Banc, d: Domain3, c: [f32; 3], out: &mut Vec<f32>) {
    let c = c.map(|v| v as f64);
    out.clear();
    for k in 0..=d.nz {
        for j in 0..=d.ny {
            for i in 0..=d.nx {
                out.push(oriented_box_distance(c, [1., 0., 0., 0.], b.demi(), [i as f64 * b.dx, j as f64 * b.dx, k as f64 * b.dx]) as f32);
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

/// Une scène : rend les amplitudes quadratiques moyennes des bandes des deux flancs, m, et les itérations
/// moyennes du gradient conjugué.
fn scene(b: Banc, phi: f64) -> Result<(f64, f64, f64), String> {
    let d = b.domaine();
    let z0 = d.z0() as f64;
    let mut c = b.centre(phi, 0., z0);
    let mut n = Vec::new();
    noeuds(b, d, c, &mut n);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 28);
    let mut v = Volume3::configure_with_floating_solid(
        &mut HostServices { alloc: &mut arena, jobs: &host_impl::SequentialJobs, sink: &host_impl::StderrSink },
        d, Milieu::MER.rho as f32, 9.81, &vec![0.; d.columns()], &n,
    ).map_err(|e| format!("configuration : {e:?}"))?;
    v.set_surface(&vec![d.z0(); d.columns()]).map_err(|e| format!("{e:?}"))?;
    v.set_partial_lid(b.partiel);
    let paroi = b.paroi(phi);
    let bande = |t: f64| -> Option<usize> {
        if t > paroi - 3.5 && t < paroi - 2.5 {
            Some(0)
        } else if t > paroi + LARGEUR + 2.5 && t < paroi + LARGEUR + 3.5 {
            Some(1)
        } else {
            None
        }
    };
    let (mut somme, mut compte, mut iterations) = ([0f64; 2], [0usize; 2], 0u64);
    for pas in 0..PAS {
        let t = (pas + 1) as f64 * DT;
        let suivant = b.centre(phi, pilonnement(t), z0);
        let vitesse = [0., 0., (suivant[2] - c[2]) / DT as f32];
        c = suivant;
        noeuds(b, d, c, &mut n);
        v.set_solid_rigid(&n, vitesse, [0.; 3], c).map_err(|e| format!("pas {pas} : paroi {e:?}"))?;
        let r = v.step_surface_linear(DT_US, 8000, &host_impl::SequentialJobs).map_err(|e| format!("pas {pas} : δ {e:?}"))?;
        iterations += r.iterations as u64;
        if r.divergence > PROJECTION_DIVERGENCE_TOLERANCE {
            return Err(format!("pas {pas} : divergence {}", r.divergence));
        }
        if t > 3. {
            for j in 0..d.ny {
                for i in 0..d.nx {
                    let (x, y) = ((i as f64 + 0.5) * b.dx, (j as f64 + 0.5) * b.dx);
                    let (travers, long) = if b.tranche { (x, 0.) } else { (y, x - 8.1) };
                    let Some(k) = bande(travers) else { continue };
                    if long.abs() <= 2. {
                        let h = (v.surface()[j * d.nx + i] - d.z0()) as f64;
                        somme[k] += h * h;
                        compte[k] += 1;
                    }
                }
            }
        }
    }
    Ok(((somme[0] / compte[0] as f64).sqrt(), (somme[1] / compte[1] as f64).sqrt(), iterations as f64 / PAS as f64))
}

fn main() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().collect();
    let valeur = |nom: &str| arguments.iter().position(|a| a == nom).and_then(|i| arguments.get(i + 1)).cloned();
    let b = Banc {
        tranche: arguments.iter().any(|a| a == "--tranche"),
        dx: valeur("--dx").and_then(|v| v.parse().ok()).unwrap_or(0.25),
        partiel: arguments.iter().any(|a| a == "--couvercle-partiel"),
    };
    let placements: Vec<f64> = valeur("--phi")
        .map(|v| v.split(',').filter_map(|x| x.parse().ok()).collect())
        .unwrap_or_else(|| vec![0.05, 0.08, 0.30, 0.55, 0.80]);
    let d = b.domaine();
    println!("A317 scene {} couvercle_partiel={} dx={} delta={:?} pilonnement_m={AMPLITUDE} omega_rad_s={OMEGA} bandes=2.5-3.5m fenetre=3-6s",
        if b.tranche { "tranche" } else { "3d" }, b.partiel, b.dx, [d.nx, d.ny, d.nz]);
    let mut toutes = Vec::new();
    for phi in placements {
        let (moins, plus, iterations) = scene(b, phi)?;
        let phi_plus = 1. - (phi + LARGEUR / b.dx).fract();
        println!("A317 placement dx={} phi_moins={phi:.2} phi_plus={phi_plus:.2} amplitude_moins_m={moins:.6} amplitude_plus_m={plus:.6} rapport={:.4} iterations={iterations:.0}",
            b.dx, moins / plus);
        toutes.extend([moins, plus]);
    }
    let moyenne = toutes.iter().sum::<f64>() / toutes.len() as f64;
    let ecart = toutes.iter().fold(0f64, |m, a| m.max((a / moyenne - 1.).abs()));
    println!("A317 dispersion dx={} amplitude_moyenne_m={moyenne:.6} ecart_max_relatif={ecart:.4}", b.dx);
    Ok(())
}
