//! Réceptions de la découpe 3D (S324, critère 1) : identité **au bit** avec la 2D quand le fond ne
//! dépend pas de `y`, exactitude sur un fond plan contre une quadrature indépendante, somme des
//! colonnes, coin étroit, symétrie.

use super::*;
use crate::delta_projection::{Domain, Volume};
use crate::host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};

struct Arena;
impl Allocator for Arena {
    fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> {
        Ok(0)
    }
    fn seal(&mut self) {}
    fn is_sealed(&self) -> bool {
        false
    }
    fn stats(&self) -> AllocStats {
        AllocStats::default()
    }
}
struct Jobs;
impl Sink for Jobs {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Jobs {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(&self, n: usize, grain: usize, r: &dyn Fn(usize, usize) -> f64,
        m: &dyn Fn(f64, f64) -> f64, init: f64) -> f64 {
        let (g, mut acc, mut s) = (grain.max(1), init, 0);
        while s < n {
            let e = (s + g).min(n);
            acc = m(acc, r(s, e));
            s = e;
        }
        acc
    }
}

/// Les trois fonds de S232, en coordonnée physique.
fn fond_s232(forme: usize, x: f32) -> f32 {
    let d = (x - 3.0) / 1.2;
    match forme {
        0 => 0.5,
        1 => 0.4 + 0.6 * (-(d * d)).exp(),
        _ => 0.4 + 0.6 * (-(d * d)).exp() + 0.125 * (1.0 + (2.0 * (x - 6.0)).tanh()),
    }
}

/// **Critère 1a.** Un fond qui ne dépend pas de `y` donne, à `ny = 1`, les fractions et les
/// ouvertures de la 2D **au bit**, sur les trois fonds de S232 ; à `ny = 4`, chaque rangée égale la
/// première au bit.
#[test]
fn y_invariant_bottom_reproduces_the_2d_cut_to_the_bit_s324() {
    let (nx, nz, dx) = (32usize, 16usize, 0.25f32);
    for forme in 0..3 {
        let b2: Vec<f32> = (0..nx).map(|i| fond_s232(forme, (i as f32 + 0.5) * dx)).collect();
        let v2 = Volume::configure(&mut HostServices { alloc: &mut Arena, jobs: &Jobs, sink: &Jobs },
            Domain { nx, nz, dx }, 1025., 9.81, &b2).unwrap();
        let (f2, (u2, w2)) = (v2.fluid_fraction(), v2.apertures());
        let c = cut(Domain3 { nx, ny: 1, nz, dx }, &b2);
        for k in 0..nz {
            for i in 0..nx {
                assert_eq!(c.frac[k * nx + i].to_bits(), f2[k * nx + i].to_bits(), "fraction {forme} {i} {k}");
            }
            for i in 0..=nx {
                assert_eq!(c.open_u[k * (nx + 1) + i].to_bits(), u2[k * (nx + 1) + i].to_bits(), "u {forme} {i} {k}");
            }
            for i in 0..nx {
                assert_eq!(c.open_v[k * 2 * nx + i], 0., "mur avant");
                assert_eq!(c.open_v[(k * 2 + 1) * nx + i], 0., "mur arrière");
            }
        }
        for k in 0..=nz {
            for i in 0..nx {
                assert_eq!(c.open_w[k * nx + i].to_bits(), w2[k * nx + i].to_bits(), "w {forme} {i} {k}");
            }
        }
        let ny = 4;
        let b3: Vec<f32> = (0..ny).flat_map(|_| b2.iter().copied()).collect();
        let c4 = cut(Domain3 { nx, ny, nz, dx }, &b3);
        for k in 0..nz {
            for j in 1..ny {
                for i in 0..nx {
                    assert_eq!(c4.frac[(k * ny + j) * nx + i].to_bits(), c4.frac[k * ny * nx + i].to_bits());
                }
            }
        }
    }
}

/// Fraction fluide exacte d'une maille `[bas, bas + dx]` au-dessus d'un fond **plan**
/// `p0 + α·x' + β·y'` sur une empreinte carrée : quadrature fine en `f64` de la fonction de
/// répartition de `αX + βY`, `X, Y` uniformes — sans passer par les triangles du module.
fn fraction_plane(p0: f64, alpha: f64, beta: f64, dx: f64, bas: f64) -> f64 {
    let (a, b) = ((alpha * dx).abs(), (beta * dx).abs());
    let (a, b) = if a >= b { (a, b) } else { (b, a) };
    // Répartition de U = |α|X + |β|Y sur [0, a + b], trapézoïdale ; le fond est p0 + U à un
    // décalage près quand les pentes sont négatives, pris en compte par `min`.
    let min = p0 + (alpha * dx).min(0.) + (beta * dx).min(0.);
    let cdf = |u: f64| -> f64 {
        if u <= 0. {
            0.
        } else if u >= a + b {
            1.
        } else if b == 0. {
            u / a
        } else if u <= b {
            u * u / (2. * a * b)
        } else if u <= a {
            (2. * u - b) / (2. * a)
        } else {
            1. - (a + b - u) * (a + b - u) / (2. * a * b)
        }
    };
    let n = 200_000;
    let h = dx / n as f64;
    (0..n).map(|m| cdf(bas + (m as f64 + 0.5) * h - min)).sum::<f64>() * h / dx
}

/// **Critère 1b.** Sur un fond plan, les empreintes intérieures — dont les quatre coins sont des
/// moyennes exactes du plan — donnent la fraction exacte à l'arrondi f32 près.
#[test]
fn a_plane_bottom_is_cut_exactly_s324() {
    let (nx, ny, nz, dx) = (7usize, 6usize, 12usize, 0.25f32);
    let (g, alpha, beta) = (0.37f32, 0.21f32, -0.13f32);
    let plan = |x: f32, y: f32| g + alpha * x + beta * y;
    let fond: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| plan((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)))
        .collect();
    let c = cut(Domain3 { nx, ny, nz, dx }, &fond);
    let mut pire = 0f64;
    for k in 0..nz {
        for j in 1..ny - 1 {
            for i in 1..nx - 1 {
                let p0 = plan(i as f32 * dx, j as f32 * dx) as f64;
                let exacte = fraction_plane(p0, alpha as f64, beta as f64, dx as f64, k as f64 * dx as f64);
                pire = pire.max((c.frac[(k * ny + j) * nx + i] as f64 - exacte).abs());
            }
        }
    }
    assert!(pire < 2e-6, "écart au plan {pire:e}");
}

/// **Critère 1c.** La somme des fractions d'une colonne, fois `dx`, est la hauteur d'eau au-dessus
/// du fond moyen de l'empreinte — la moyenne de ses coins pour le modèle à quatre triangles.
#[test]
fn a_column_holds_the_water_above_its_mean_bottom_s324() {
    let (nx, ny, nz, dx) = (9usize, 8usize, 10usize, 0.2f32);
    let fond: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| {
            let (x, y) = ((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx);
            0.8 + 0.35 * (3.1 * x).sin() * (2.3 * y + 0.4).cos()
        }))
        .collect();
    let c = cut(Domain3 { nx, ny, nz, dx }, &fond);
    let k0 = corners(Domain3 { nx, ny, nz, dx }, &fond);
    for j in 0..ny {
        for i in 0..nx {
            let m = 0.25
                * (k0[j * (nx + 1) + i] + k0[j * (nx + 1) + i + 1] + k0[(j + 1) * (nx + 1) + i + 1] + k0[(j + 1) * (nx + 1) + i])
                    as f64;
            let eau: f64 = (0..nz).map(|k| c.frac[(k * ny + j) * nx + i] as f64 * dx as f64).sum();
            assert!((eau - (nz as f64 * dx as f64 - m)).abs() < 2e-6, "colonne {i} {j} : {eau} contre {}", nz as f64 * dx as f64 - m);
        }
    }
}

/// **Critère 1d.** Un coin étroit ne disparaît pas — la faute de S232, où un triangle entier sortait
/// de la découpe : un fond qui ne plonge sous le haut d'une maille qu'en un sommet y laisse une
/// fraction positive, égale à celle d'une quadrature indépendante.
#[test]
fn a_thin_corner_wedge_stays_fluid_s324() {
    // Un triangle dont un seul sommet passe sous la tranche [0, 1].
    let w = [1.2f32, 1.4, 0.98];
    let f = triangle_fraction(w, 1.0) as f64;
    // Quadrature indépendante, barycentrique, en `f64`.
    let n = 1000;
    let mut s = 0.0f64;
    for a in 0..n {
        for b in 0..n - a {
            let (l1, l2) = ((a as f64 + 1. / 3.) / n as f64, (b as f64 + 1. / 3.) / n as f64);
            let fond = w[0] as f64 * l1 + w[1] as f64 * l2 + w[2] as f64 * (1. - l1 - l2);
            s += (1. - fond).clamp(0., 1.);
        }
    }
    let quadrature = s / (n * (n + 1) / 2) as f64;
    assert!(f > 0., "le coin étroit a disparu");
    assert!((f - quadrature).abs() < 2e-6 + 1e-3 * quadrature, "coin {f:e} contre {quadrature:e}");
    // La découpe entière n'échantillonne rien : la somme exacte des colonnes (critère 1c) garantit
    // qu'aucun coin n'en sort.
}

/// **Critère 1e.** Le modèle est symétrique : le fond retourné en `x` donne les fractions
/// retournées, à l'ordre de sommation près.
#[test]
fn the_cut_is_mirror_symmetric_s324() {
    let (nx, ny, nz, dx) = (8usize, 5usize, 9usize, 0.25f32);
    let f = |i: usize, j: usize| 0.9 + 0.3 * ((i * i) as f32 * 0.37).sin() + 0.2 * (j as f32 * 1.3).cos();
    let fond: Vec<f32> = (0..ny).flat_map(|j| (0..nx).map(move |i| f(i, j))).collect();
    let miroir: Vec<f32> = (0..ny).flat_map(|j| (0..nx).map(move |i| f(nx - 1 - i, j))).collect();
    let (a, b) = (cut(Domain3 { nx, ny, nz, dx }, &fond), cut(Domain3 { nx, ny, nz, dx }, &miroir));
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let (x, y) = (a.frac[(k * ny + j) * nx + i], b.frac[(k * ny + j) * nx + nx - 1 - i]);
                assert!((x - y).abs() < 1e-6, "{i} {j} {k} : {x} contre {y}");
            }
        }
    }
}
