//! **Le ballottement sur colonne graduée, au pas mobile** — S387, C2b de la campagne du solveur volumique 3D
//! ([ADR-208](../../../docs/adr/ADR-208-la-colonne-graduee.md)).
//!
//! Un bassin de 8 × 4 m, 4 m d'eau sous 2 m d'air (24 couches de 25 cm), le mode fondamental le long de `x`,
//! `η = 4 + a·cos(πx/8)`, `a` = 1 cm, pas de 2 ms, pendant `DUREE` secondes (5 par défaut). Deux domaines : la grille fine,
//! et la colonne graduée des essais de S387 — nœuds 0, 5, 9, puis cubiques de 12 au haut (15 inconnues sur 24).
//!
//! **Prédiction** : les deux ballottent à la fréquence de leur schéma (le problème vertical de `scheme_frequency`, S295,
//! restreint par `P` pour la colonne graduée) ; l'écart de hauteur croît comme `a·|Ω_gradué − Ω_fin|·t`. **Critère écrit
//! avant la mesure** (plan de S387) : l'écart mesuré vaut de 0,5 à 2 fois l'écart prédit à la fin.
//!
//!     cargo run -p water-core --release --offline --example delta3d_ballottement_gradue

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::delta3d::{Domain3, Volume3};
use water_core::host::HostServices;

const G: f64 = 9.81;
const DT_US: u64 = 2000;
const A: f64 = 0.01;

/// Ω du schéma vertical, restreint par `P` (linéaire par morceaux entre `nodes`) ; tous les nœuds : le schéma fin.
fn frequency(kappa2: f64, dx: f64, n: usize, nodes: &[usize], dt: f64) -> f64 {
    let mut a = vec![vec![0f64; n]; n];
    let mut d = vec![0f64; n];
    for k in 0..n {
        a[k][k] = kappa2 * dx * dx;
        if k > 0 {
            a[k][k] += 1.;
            a[k][k - 1] = -1.;
        }
        if k + 1 < n {
            a[k][k] += 1.;
            a[k][k + 1] = -1.;
        } else {
            a[k][k] += 2.;
            d[k] = 2.;
        }
    }
    let r = nodes.len();
    let mut p = vec![vec![0f64; r]; n];
    for s in 0..r - 1 {
        for k in nodes[s]..nodes[s + 1] {
            let t = (k - nodes[s]) as f64 / (nodes[s + 1] - nodes[s]) as f64;
            p[k][s] = 1. - t;
            p[k][s + 1] = t;
        }
    }
    p[nodes[r - 1]][r - 1] = 1.;
    let mut m = vec![vec![0f64; r + 1]; r];
    for i in 0..r {
        for j in 0..r {
            m[i][j] = (0..n).map(|k| (0..n).map(|l| p[k][i] * a[k][l] * p[l][j]).sum::<f64>()).sum();
        }
        m[i][r] = (0..n).map(|k| p[k][i] * d[k]).sum();
    }
    for col in 0..r {
        let piv = (col..r).max_by(|x, y| m[*x][col].abs().total_cmp(&m[*y][col].abs())).unwrap();
        m.swap(col, piv);
        for row in col + 1..r {
            let f = m[row][col] / m[col][col];
            for c in col..=r {
                m[row][c] -= f * m[col][c];
            }
        }
    }
    let mut x = vec![0f64; r];
    for row in (0..r).rev() {
        x[row] = (m[row][r] - (row + 1..r).map(|c| m[row][c] * x[c]).sum::<f64>()) / m[row][row];
    }
    let s: f64 = (0..n).map(|k| (0..r).map(|j| p[k][j] * x[j]).sum::<f64>() * dx).sum();
    (1. - G * kappa2 * s * dt * dt / 2.).acos() / dt
}

fn main() {
    let duree: f64 = std::env::var("DUREE").ok().and_then(|v| v.parse().ok()).unwrap_or(5.);
    let (nx, ny, nz, dx) = (32usize, 16usize, 24usize, 0.25f64);
    let water = 16usize;
    let mut nodes = vec![0usize, 5, 9];
    nodes.extend(12..nz);
    let water_nodes: Vec<usize> = nodes.iter().copied().filter(|k| *k < water).collect();
    let kx = std::f64::consts::PI / 8.;
    let kappa2 = 4. / (dx * dx) * (kx * dx / 2.).sin().powi(2);
    let dt = DT_US as f64 * 1e-6;
    let fine = frequency(kappa2, dx, water, &(0..water).collect::<Vec<_>>(), dt);
    let graded = frequency(kappa2, dx, water, &water_nodes, dt);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut volumes = Vec::new();
    for gradue in [false, true] {
        let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 30);
        let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
        let mut v = Volume3::configure(&mut hote, Domain3 { nx, ny, nz, dx: dx as f32 }, 1025., G as f32).expect("domaine");
        if gradue {
            v.enable_graded(&mut hote, &nodes).expect("colonne graduée");
        }
        let eta: Vec<f32> = (0..ny)
            .flat_map(|_| (0..nx).map(move |i| (4. + A * (kx * (i as f64 + 0.5) * dx).cos()) as f32))
            .collect();
        v.set_free_surface(&eta, 4.).expect("surface");
        volumes.push(v);
    }
    let steps = (duree / dt).round() as u64;
    let (mut worst, mut iterations) = (0f64, [0u64; 2]);
    for _ in 0..steps {
        for (n, v) in volumes.iter_mut().enumerate() {
            let r = v.step_surface_mobile(DT_US, 20_000, &jobs).expect("pas");
            assert!(!r.degraded);
            iterations[n] += r.iterations as u64;
        }
        let e = volumes[0].surface().iter().zip(volumes[1].surface()).fold(0f64, |m, (a, b)| m.max((a - b).abs() as f64));
        worst = worst.max(e);
    }
    let predicted = A * (graded - fine).abs() * duree;
    let ratio = worst / predicted;
    println!(
        "BALLOTTEMENT_S387 duree_s={duree} omega_fin={fine:.6} omega_gradue={graded:.6} ecart_relatif={:+.3e} \
         ecart_predit_m={predicted:.3e} ecart_mesure_m={worst:.3e} rapport={ratio:.3} critere_0_5_a_2={} \
         iterations_moyennes_fin={:.1} iterations_moyennes_gradue={:.1} inconnues_par_colonne={}/{}",
        graded / fine - 1.,
        if (0.5..=2.).contains(&ratio) { "tenu" } else { "manque" },
        iterations[0] as f64 / steps as f64,
        iterations[1] as f64 / steps as f64,
        volumes[1].pressure_unknowns_per_column(),
        nz
    );
}
