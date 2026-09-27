//! **Changer un domaine de niveau, sans rupture visible ?** — S402, C8c de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)) ; le rang 4 d'ADR-012 et le retour.
//!
//! Un bassin de 24 × 16 m, 2 m d'eau sous 1 m d'air, pas mobile de 20 ms, multigrille. **La référence** : le domaine à 25 cm,
//! tenu tout du long. **Le domaine qui change de niveau** : à 25 cm jusqu'à 2 s, à 50 cm de 2 à 5 s (le rang 4 d'ADR-012), de
//! nouveau à 25 cm ensuite, par l'un des deux mécanismes :
//!
//! - **`transfert`** — l'état recopié d'un niveau à l'autre (`Volume3::resample_from`) ;
//! - **`adr005`** — le cycle de vie d'[ADR-005](../../../docs/adr/ADR-005-zone-de-transition.md) §5 : le nouveau domaine naît à
//!   δ = 0, l'ancien continue sans la source et s'efface linéairement en τ = 0,5 s (la transduction vers W n'existe pas en
//!   référence : ce qui en sortirait est perdu ici).
//!
//! **L'image** est ce que le rendu montrerait, sur la grille fine : un domaine à 50 cm y est reconstruit par le même transfert.
//! **Le saut** d'un passage : la variation de l'image sur le pas du passage, moins celle de la référence — ce qui change d'un
//! coup. **L'écart** : l'image contre la référence.
//!
//!     cargo run -p water-core --release --offline --example delta3d_niveaux -- <bosse|source> <transfert|adr005>

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::delta3d::{Domain3, Volume3};
use water_core::host::HostServices;

const LX: f32 = 24.;
const LY: f32 = 16.;
const REST: f32 = 2.0;
const DT_US: u64 = 20_000;
const FINE: f32 = 0.25;
const COARSE: f32 = 0.5;
const SWITCH_S: f64 = 2.;
const RETURN_S: f64 = 5.;
const END_S: f64 = 7.;
const TAU_S: f64 = 0.5;

fn domain(host: &mut HostServices, dx: f32) -> Volume3 {
    let (nx, ny, nz) = ((LX / dx).round() as usize, (LY / dx).round() as usize, (3. / dx).round() as usize);
    let mut v = Volume3::configure(host, Domain3 { nx, ny, nz, dx }, 1000., 9.81).expect("domaine");
    v.clear_to_rest(REST).expect("repos");
    v.enable_multigrid(host).expect("multigrille");
    v
}

/// La bosse de départ, aux centres des colonnes d'un domaine.
fn bump(v: &Volume3) -> Vec<f32> {
    let Domain3 { nx, ny, dx, .. } = v.domain();
    (0..nx * ny)
        .map(|c| {
            let (x, y) = (((c % nx) as f32 + 0.5) * dx - 9., ((c / nx) as f32 + 0.5) * dx - 8.);
            REST + 0.05 * (-(x * x + y * y) / (2. * 1. * 1.)).exp()
        })
        .collect()
}

/// Le dipôle de S401 sur la grille d'un domaine : +q devant, −q derrière, gaussiennes tronquées et normalisées, à `p`.
fn dipole(v: &Volume3, p: [f32; 2], q: f32, out: &mut Vec<f32>) {
    let Domain3 { nx, ny, dx, .. } = v.domain();
    out.clear();
    out.resize(nx * ny, 0.);
    for (centre, sign) in [([p[0] + 0.5, p[1]], 1f32), ([p[0] - 0.5, p[1]], -1.)] {
        let mut sum = 0f64;
        let mut g = vec![0f32; nx * ny];
        for (c, gc) in g.iter_mut().enumerate() {
            let (x, y) = (((c % nx) as f32 + 0.5) * dx - centre[0], ((c / nx) as f32 + 0.5) * dx - centre[1]);
            if x * x + y * y <= 1.5 * 1.5 {
                *gc = (-(x * x + y * y) / 0.5).exp();
                sum += *gc as f64;
            }
        }
        let norm = (1. / (sum * (dx as f64).powi(2))) as f32;
        for (o, gc) in out.iter_mut().zip(&g) {
            *o += sign * q * gc * norm;
        }
    }
}

fn step(v: &mut Volume3) {
    v.step_surface_mobile(DT_US, 20_000, &host_impl::SequentialJobs).expect("pas");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let case = args.get(1).map_or("bosse", |s| s.as_str()).to_string();
    let mechanism = args.get(2).map_or("transfert", |s| s.as_str()).to_string();
    let source = case == "source";
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 30);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut host = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let mut reference = domain(&mut host, FINE);
    let mut fine = domain(&mut host, FINE);
    let mut coarse = domain(&mut host, COARSE);
    // L'ancien domaine d'ADR-005 §5, qui s'efface, et la grille de l'image.
    let mut old_fine = domain(&mut host, FINE);
    let mut old_coarse = domain(&mut host, COARSE);
    let mut image = domain(&mut host, FINE);
    let mut scratch = domain(&mut host, FINE);
    if !source {
        let b = bump(&reference);
        reference.set_free_surface(&b, REST).expect("bosse");
        fine.set_free_surface(&b, REST).expect("bosse");
    }
    let dt = DT_US as f64 * 1e-6;
    let steps = (END_S / dt).round() as usize;
    let (n_switch, n_return) = ((SWITCH_S / dt).round() as usize, (RETURN_S / dt).round() as usize);
    let cells = reference.surface().len();
    let (mut prev_image, mut prev_ref) = (reference.surface().to_vec(), reference.surface().to_vec());
    let (mut dh_fine, mut dh_coarse) = (Vec::new(), Vec::new());
    let (mut pop_switch, mut pop_return, mut pop_other) = (0f32, 0f32, 0f32);
    let (mut gap_coarse, mut gap_after, mut amplitude) = (0f32, 0f32, 0f32);
    let mut gap_at_return = 0f32;
    let mut fade_left = 0usize; // pas restants du fondu de l'ancien domaine (ADR-005 §5)
    let fade_steps = (TAU_S / dt).round() as usize;
    for s in 0..steps {
        let t = s as f64 * dt;
        // Les passages, au début du pas `s`.
        if s == n_switch || s == n_return {
            let to_coarse = s == n_switch;
            match mechanism.as_str() {
                "transfert" => {
                    if to_coarse {
                        coarse.resample_from(&fine).expect("25 → 50 cm");
                    } else {
                        fine.resample_from(&coarse).expect("50 → 25 cm");
                    }
                }
                _ => {
                    // ADR-005 §5 : l'ancien continue et s'efface ; le nouveau naît au repos.
                    if to_coarse {
                        old_fine.resample_from(&fine).expect("copie");
                        coarse.clear_to_rest(REST).expect("repos");
                    } else {
                        old_coarse.resample_from(&coarse).expect("copie");
                        fine.clear_to_rest(REST).expect("repos");
                    }
                    fade_left = fade_steps;
                }
            }
        }
        let on_coarse = s >= n_switch && s < n_return;
        // La source, dans le domaine courant et dans la référence : dipôle à 2 m/s, débit monté en 1 s.
        if source {
            let p = [4. + 2. * (t + 0.5 * dt) as f32, 8.];
            let ramp = (t / 1.).min(1.);
            let q = 0.05 * (ramp * ramp * (3. - 2. * ramp)) as f32 * dt as f32;
            dipole(&reference, p, q, &mut dh_fine);
            reference.add_column_volume(&dh_fine).expect("source");
            if on_coarse {
                dipole(&coarse, p, q, &mut dh_coarse);
                coarse.add_column_volume(&dh_coarse).expect("source");
            } else {
                fine.add_column_volume(&dh_fine).expect("source");
            }
        }
        step(&mut reference);
        if on_coarse {
            step(&mut coarse);
        } else {
            step(&mut fine);
        }
        if fade_left > 0 {
            if on_coarse {
                step(&mut old_fine);
            } else {
                step(&mut old_coarse);
            }
        }
        // L'image, sur la grille fine.
        let mut img: Vec<f32> = if on_coarse {
            image.resample_from(&coarse).expect("image");
            image.surface().to_vec()
        } else {
            fine.surface().to_vec()
        };
        if fade_left > 0 {
            // Le fondu linéaire de l'ancien domaine, sur τ.
            let w = (fade_left - 1) as f32 / fade_steps as f32;
            let old: Vec<f32> = if on_coarse {
                old_fine.surface().to_vec()
            } else {
                scratch.resample_from(&old_coarse).expect("image de l'ancien");
                scratch.surface().to_vec()
            };
            for (i, o) in img.iter_mut().zip(&old) {
                *i += w * (o - REST);
            }
            fade_left -= 1;
        }
        let r = reference.surface();
        let (mut pop, mut gap) = (0f32, 0f32);
        for c in 0..cells {
            pop = pop.max(((img[c] - prev_image[c]) - (r[c] - prev_ref[c])).abs());
            gap = gap.max((img[c] - r[c]).abs());
            amplitude = amplitude.max((r[c] - REST).abs());
        }
        if s == n_switch {
            pop_switch = pop;
        } else if s == n_return {
            pop_return = pop;
            gap_at_return = gap;
        } else {
            pop_other = pop_other.max(pop);
        }
        if on_coarse {
            gap_coarse = gap_coarse.max(gap);
        } else if s >= n_return {
            gap_after = gap_after.max(gap);
        }
        prev_image.copy_from_slice(&img);
        prev_ref.copy_from_slice(r);
        if s % 50 == 49 {
            eprintln!("t = {:.1} s : écart {gap:.2e} m, saut du pas {pop:.2e} m", (s + 1) as f64 * dt);
        }
    }
    println!(
        "NIVEAUX_S402 cas={case} mecanisme={mechanism} amplitude_m={amplitude:.4} saut_passage_m={pop_switch:.3e} \
         saut_retour_m={pop_return:.3e} saut_autres_pas_max_m={pop_other:.3e} ecart_periode_50cm_max_m={gap_coarse:.3e} \
         ecart_au_retour_m={gap_at_return:.3e} ecart_apres_retour_max_m={gap_after:.3e} critere_saut_3mm={}",
        if pop_switch <= 3e-3 && pop_return <= 3e-3 { "tenu" } else { "manqué" }
    );
}
