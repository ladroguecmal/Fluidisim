//! S402 — changer un domaine de niveau (C8c) : le transfert d'état entre deux `dx`.
use super::*;
use crate::apic3d::tests::{Arena, Jobs};
use crate::host::AllocStats;

const REST: f32 = 2.0;
const DT_US: u64 = 20_000;

/// Un domaine de `lx × ly × 3 m` à `dx`, au repos à 2 m.
fn domain(lx: f32, ly: f32, dx: f32) -> Volume3 {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let (nx, ny, nz) = ((lx / dx).round() as usize, (ly / dx).round() as usize, (3. / dx).round() as usize);
    let mut v = Volume3::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, Domain3 { nx, ny, nz, dx },
        1000., 9.81)
    .unwrap();
    v.clear_to_rest(REST).unwrap();
    v
}

/// La surface `rest + f(x, y)` aux centres des colonnes.
fn surface(v: &Volume3, f: impl Fn(f64, f64) -> f64) -> Vec<f32> {
    let Domain3 { nx, ny, dx, .. } = v.domain();
    (0..ny)
        .flat_map(|j| (0..nx).map(move |i| (i, j)))
        .map(|(i, j)| REST + f((i as f64 + 0.5) * dx as f64, (j as f64 + 0.5) * dx as f64) as f32)
        .collect()
}

#[test]
fn a_uniform_state_crosses_levels_unchanged_s402() {
    // T1 : surface uniforme au bit dans les deux sens ; vitesse uniforme `u` au bit loin des murs.
    let (mut fine, mut coarse) = (domain(8., 4., 0.25), domain(8., 4., 0.5));
    fine.set_free_surface(&vec![REST + 0.0125; 32 * 16], REST).unwrap();
    let (nx, ny, nz) = (32, 16, 12);
    for k in 0..nz {
        for j in 0..ny {
            for i in 1..nx {
                let f = fine.fu(i, j, k);
                fine.u[f] = 0.1;
            }
        }
    }
    coarse.resample_from(&fine).unwrap();
    assert!(coarse.eta.iter().all(|e| e.to_bits() == (REST + 0.0125).to_bits()));
    assert!(coarse.eta_roundoff.iter().all(|r| *r == 0.));
    for k in 0..6 {
        for j in 0..8 {
            for i in 1..16 {
                assert_eq!(coarse.u[coarse.fu(i, j, k)].to_bits(), 0.1f32.to_bits(), "face grossière ({i}, {j}, {k})");
            }
        }
    }
    let mut back = domain(8., 4., 0.25);
    back.resample_from(&coarse).unwrap();
    assert!(back.eta.iter().all(|e| e.to_bits() == (REST + 0.0125).to_bits()));
    for k in 0..nz {
        for j in 0..ny {
            for i in 2..nx - 1 {
                assert_eq!(back.u[back.fu(i, j, k)].to_bits(), 0.1f32.to_bits(), "face fine ({i}, {j}, {k})");
            }
        }
    }
}

#[test]
fn the_perturbation_volume_is_kept_across_levels_s402() {
    // T2 : une bosse et une pente quelconques, 25 → 50 → 25 cm, et 10 → 25 cm (rapport 2,5) : le volume à l'arrondi f64.
    let bump = |x: f64, y: f64| 0.05 * (-((x - 3.1) * (x - 3.1) + (y - 1.7) * (y - 1.7)) / 0.8).exp() + 0.002 * x;
    let mut fine = domain(8., 4., 0.25);
    fine.set_free_surface(&surface(&fine, bump), REST).unwrap();
    let mut coarse = domain(8., 4., 0.5);
    let down = coarse.resample_from(&fine).unwrap();
    let mut back = domain(8., 4., 0.25);
    let up = back.resample_from(&coarse).unwrap();
    let mut finer = domain(8., 4., 0.1);
    finer.set_free_surface(&surface(&finer, bump), REST).unwrap();
    let mut mid = domain(8., 4., 0.25);
    let odd = mid.resample_from(&finer).unwrap();
    for (name, c) in [("25 → 50", down), ("50 → 25", up)] {
        println!("S402 volume {name} : {:.6e} → {:.6e} m³ (écart {:.1e})", c.volume_before, c.volume_after,
            c.volume_after - c.volume_before);
        assert!((c.volume_after - c.volume_before).abs() <= 1e-10, "{name}");
    }
    // Au rapport 2,5, `dx` = 0,1 m ne s'écrit pas exactement en f32 : les deux fenêtres diffèrent de 3·10⁻⁸ en aire, et le volume
    // de même. Ce que le transfert conserve exactement, c'est la hauteur moyenne de perturbation — volume sur aire.
    let area = |v: &Volume3| {
        let d = v.domain();
        (d.nx as f64 * d.dx as f64) * (d.ny as f64 * d.dx as f64)
    };
    let (before, after) = (odd.volume_before / area(&finer), odd.volume_after / area(&mid));
    println!("S402 volume 10 → 25 : {:.6e} → {:.6e} m³ (écart {:.1e}) ; hauteur moyenne {before:.9e} → {after:.9e} m",
        odd.volume_before, odd.volume_after, odd.volume_after - odd.volume_before);
    assert!((after - before).abs() <= 1e-12 * before.abs(), "10 → 25");
}

/// L'erreur relative d'un aller-retour 25 → 50 → 25 cm d'une surface sinusoïdale de longueur d'onde `lambda` selon x et y.
fn round_trip(lambda: f64) -> f64 {
    let k = std::f64::consts::TAU / lambda;
    let wave = |x: f64, y: f64| 0.05 * (k * x).cos() * (k * y).cos();
    let mut fine = domain(16., 16., 0.25);
    fine.set_free_surface(&surface(&fine, wave), REST).unwrap();
    let mut coarse = domain(16., 16., 0.5);
    coarse.resample_from(&fine).unwrap();
    let mut back = domain(16., 16., 0.25);
    back.resample_from(&coarse).unwrap();
    // L'intérieur : les colonnes à plus d'une maille grossière du bord, où la pente est centrée.
    let (n, mut worst) = (64usize, 0f64);
    for j in 2..n - 2 {
        for i in 2..n - 2 {
            let c = j * n + i;
            worst = worst.max((back.eta[c] as f64 - fine.eta[c] as f64).abs());
        }
    }
    worst / 0.05
}

#[test]
fn a_round_trip_of_a_smooth_wave_loses_little_s402() {
    // T3 : λ = 8 m, seize mailles grossières — critère 1 % ; prédiction 0,3 % (reconstruction linéaire, calcul 1D).
    let e8 = round_trip(8.);
    let e16 = round_trip(16.);
    println!("S402 aller-retour 25 → 50 → 25 cm : λ = 8 m, {:.3} % ; λ = 16 m, {:.3} % de l'amplitude", 100. * e8, 100. * e16);
    assert!(e8 <= 0.01, "λ = 8 m : {e8}");
    // Au moins l'ordre deux : à λ double, l'erreur au moins trois fois moindre. (Mesuré : 8,2 — l'ordre trois ; aux centres des
    // demi-mailles, le terme d'ordre deux de la reconstruction s'annule. Sans le terme croisé : 4,2, l'ordre deux.)
    assert!(e8 / e16 >= 3., "ordre : {}", e8 / e16);
}

#[test]
fn different_frames_are_refused_s402() {
    // T4 : étendue, repos, gravité ; rien n'est écrit. S404 : l'ensemble épars, refusé ici en S402, est désormais porté — ses
    // essais sont les `_s404` plus bas.
    let fine = domain(8., 4., 0.25);
    let mut wider = domain(10., 4., 0.5);
    assert_eq!(wider.resample_from(&fine).unwrap_err(), Error::Domain);
    let mut other_rest = domain(8., 4., 0.5);
    other_rest.clear_to_rest(1.5).unwrap();
    assert_eq!(other_rest.resample_from(&fine).unwrap_err(), Error::Domain);
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let mut moon = Volume3::configure(&mut host, Domain3 { nx: 16, ny: 8, nz: 6, dx: 0.5 }, 1000., 1.62).unwrap();
    moon.clear_to_rest(REST).unwrap();
    assert_eq!(moon.resample_from(&fine).unwrap_err(), Error::Domain);
    assert!(moon.eta.iter().all(|e| *e == REST));
}

#[test]
fn a_transferred_state_steps_on_s402() {
    // Le domaine d'arrivée repart : une bosse transférée au pas suivant, sans refus, masse gardée par le pas.
    let mut fine = domain(8., 4., 0.25);
    fine.set_free_surface(&surface(&fine, |x, y| 0.05 * (-((x - 4.) * (x - 4.) + (y - 2.) * (y - 2.)) / 1.).exp()), REST)
        .unwrap();
    for _ in 0..10 {
        fine.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
    }
    let mut coarse = domain(8., 4., 0.5);
    let c = coarse.resample_from(&fine).unwrap();
    for _ in 0..10 {
        coarse.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
    }
    assert!((coarse.perturbation_volume() - c.volume_after).abs() < 1e-9);
}

// ---------------------------------------------------------------------------------------------------------------------
// S404 — le transfert d'un domaine épars (C8e) : le bord de l'ensemble contre le bord de la boîte.

/// Un domaine épars de `lx × ly × 3 m` à `dx`, l'ensemble fait des rectangles de colonnes `[i0, i1) × [j0, j1)`.
fn sparse_domain(lx: f32, ly: f32, dx: f32, rects: &[[usize; 4]]) -> Volume3 {
    let mut v = domain(lx, ly, dx);
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    v.enable_sparse(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
    let Domain3 { nx, ny, .. } = v.domain();
    let mask: Vec<u8> = (0..nx * ny)
        .map(|c| {
            let (i, j) = (c % nx, c / nx);
            u8::from(rects.iter().any(|r| i >= r[0] && i < r[1] && j >= r[2] && j < r[3]))
        })
        .collect();
    v.set_active_columns(&mask).unwrap();
    v
}

/// Un état lisse — une bosse, une onde oblique, trois champs de vitesse — rapporté au coin `corner` (m) : l'ensemble d'une
/// fenêtre et le domaine dense du même rectangle reçoivent le même aux mêmes points. Dehors, `set_velocity` ferme les faces.
fn smooth_state(v: &mut Volume3, corner: [f64; 2]) {
    let Domain3 { nx, ny, nz, dx } = v.domain();
    let d = dx as f64;
    let h = |x: f64, y: f64| {
        0.05 * (-((x - 1.5) * (x - 1.5) + (y - 0.8) * (y - 0.8)) / 0.5).exp() + 0.01 * (core::f64::consts::TAU * (x + 0.5 * y) / 1.7).sin()
    };
    let active = v.active_columns().map(|a| a.to_vec());
    let eta: Vec<f32> = (0..nx * ny)
        .map(|c| {
            let (i, j) = (c % nx, c / nx);
            if active.as_ref().is_some_and(|a| a[c] == 0) {
                return REST;
            }
            REST + h((i as f64 + 0.5) * d - corner[0], (j as f64 + 0.5) * d - corner[1]) as f32
        })
        .collect();
    v.set_free_surface(&eta, REST).unwrap();
    let field = |axis: usize, i: usize, j: usize, k: usize| {
        let x = (i as f64 + if axis == 0 { 0. } else { 0.5 }) * d - corner[0];
        let y = (j as f64 + if axis == 1 { 0. } else { 0.5 }) * d - corner[1];
        let z = (k as f64 + if axis == 2 { 0. } else { 0.5 }) * d;
        (match axis {
            0 => 0.1 * (1.3 * x + 0.7 * y + 0.4 * z).sin(),
            1 => 0.08 * (0.9 * x - 1.1 * y + 0.3 * z).cos(),
            _ => 0.05 * (0.5 * x + 0.6 * y - 0.8 * z).sin(),
        }) as f32
    };
    let (mut u, mut vv, mut w) = (v.u.clone(), v.v.clone(), v.w.clone());
    for k in 0..=nz {
        for j in 0..=ny {
            for i in 0..=nx {
                if k < nz && j < ny {
                    u[v.fu(i, j, k)] = field(0, i, j, k);
                }
                if k < nz && i < nx {
                    vv[v.fv(i, j, k)] = field(1, i, j, k);
                }
                if i < nx && j < ny {
                    w[v.fw(i, j, k)] = field(2, i, j, k);
                }
            }
        }
    }
    v.set_velocity(&u, &vv, &w).unwrap();
}

/// Le plus grand écart entre la fenêtre, colonnes décalées de `(i0, j0)`, et le dense : surface, vitesses.
fn window_gap(window: &Volume3, dense: &Volume3, i0: usize, j0: usize) -> (f32, f32) {
    let Domain3 { nx, ny, nz, .. } = dense.domain();
    let (mut h, mut vel) = (0f32, 0f32);
    for j in 0..ny {
        for i in 0..nx {
            h = h.max((window.eta[window.col(i + i0, j + j0)] - dense.eta[dense.col(i, j)]).abs());
        }
    }
    for k in 0..=nz {
        for j in 0..=ny {
            for i in 0..=nx {
                if k < nz && j < ny {
                    vel = vel.max((window.u[window.fu(i + i0, j + j0, k)] - dense.u[dense.fu(i, j, k)]).abs());
                }
                if k < nz && i < nx {
                    vel = vel.max((window.v[window.fv(i + i0, j + j0, k)] - dense.v[dense.fv(i, j, k)]).abs());
                }
                if i < nx && j < ny {
                    vel = vel.max((window.w[window.fw(i + i0, j + j0, k)] - dense.w[dense.fw(i, j, k)]).abs());
                }
            }
        }
    }
    (h, vel)
}

/// Hors de l'ensemble : le repos au bit, les faces des colonnes dehors nulles.
fn outside_at_rest(v: &Volume3) -> bool {
    let Domain3 { nx, ny, nz, .. } = v.domain();
    let active = v.active_columns().unwrap();
    (0..ny).all(|j| {
        (0..nx).all(|i| {
            active[j * nx + i] != 0
                || (v.eta[v.col(i, j)].to_bits() == REST.to_bits()
                    && v.eta_roundoff[v.col(i, j)] == 0.
                    && (0..nz).all(|k| v.u[v.fu(i, j, k)] == 0. && v.u[v.fu(i + 1, j, k)] == 0. && v.v[v.fv(i, j, k)] == 0.
                        && v.v[v.fv(i, j + 1, k)] == 0.)
                    && (0..=nz).all(|k| v.w[v.fw(i, j, k)] == 0.))
        })
    })
}

#[test]
fn a_rectangle_of_the_set_crosses_levels_as_its_dense_rectangle_s404() {
    // Critère 3 : un rectangle de 4 × 2 m (colonnes [8, 24) × [8, 16) à 25 cm) dans une fenêtre de 8 × 6 m, 25 → 50 → 25 cm,
    // contre le domaine dense du rectangle ; ≤ 1 µm, prédiction : la surface au bit (`dx` dyadiques). Volume exact.
    let mut fine = sparse_domain(8., 6., 0.25, &[[8, 24, 8, 16]]);
    let mut dense_fine = domain(4., 2., 0.25);
    smooth_state(&mut fine, [2., 2.]);
    smooth_state(&mut dense_fine, [0., 0.]);
    assert_eq!(window_gap(&fine, &dense_fine, 8, 8), (0., 0.));
    let mut coarse = sparse_domain(8., 6., 0.5, &[[4, 12, 4, 8]]);
    let mut dense_coarse = domain(4., 2., 0.5);
    let (a, b) = (coarse.resample_from(&fine).unwrap(), dense_coarse.resample_from(&dense_fine).unwrap());
    let down = window_gap(&coarse, &dense_coarse, 4, 4);
    let mut back = sparse_domain(8., 6., 0.25, &[[8, 24, 8, 16]]);
    let mut dense_back = domain(4., 2., 0.25);
    let (c, d) = (back.resample_from(&coarse).unwrap(), dense_back.resample_from(&dense_coarse).unwrap());
    let up = window_gap(&back, &dense_back, 8, 8);
    println!(
        "S404 niveaux, rectangle contre dense : 25 → 50 cm surface {:.3e} m, vitesses {:.3e} m/s ; 50 → 25 cm {:.3e} m, {:.3e} m/s ; \
         volumes {:.6e} → {:.6e} → {:.6e} m³ (dense {:.6e} → {:.6e} → {:.6e})",
        down.0, down.1, up.0, up.1, a.volume_before, a.volume_after, c.volume_after, b.volume_before, b.volume_after, d.volume_after
    );
    assert!(down.0 <= 1e-6 && up.0 <= 1e-6 && down.1 <= 1e-6 && up.1 <= 1e-6, "{down:?} {up:?}");
    for ch in [a, b, c, d] {
        assert!((ch.volume_after - ch.volume_before).abs() <= 1e-10, "{ch:?}");
    }
    assert!(outside_at_rest(&coarse) && outside_at_rest(&back));
}

#[test]
fn a_set_that_does_not_cover_loses_what_it_leaves_out_and_says_so_s404() {
    // L'ensemble d'arrivée ne couvre que la moitié gauche du départ : ce que portait l'autre moitié est perdu, exactement — chaque
    // colonne de 25 cm tombe entière dans une de 50 cm —, et `LevelChange` le dit ; dehors, le repos.
    let mut fine = sparse_domain(8., 6., 0.25, &[[8, 24, 8, 16]]);
    smooth_state(&mut fine, [2., 2.]);
    let mut half = sparse_domain(8., 6., 0.5, &[[4, 8, 4, 8]]);
    let ch = half.resample_from(&fine).unwrap();
    let mut right = 0f64;
    for j in 8..16 {
        for i in 16..24 {
            let c = fine.col(i, j);
            right += ((fine.eta[c] as f64 - fine.eta_roundoff[c] as f64) - REST as f64) * 0.0625;
        }
    }
    println!("S404 niveaux, arrivée qui ne couvre pas : perdu {:.6e} m³, attendu {right:.6e}", ch.volume_before - ch.volume_after);
    assert!(((ch.volume_before - ch.volume_after) - right).abs() <= 1e-12, "{ch:?} {right}");
    assert!(right.abs() > 1e-4);
    assert!(outside_at_rest(&half));
    // Un domaine dense reçoit tout d'un épars : le volume, exact.
    let mut dense = domain(8., 6., 0.5);
    let all = dense.resample_from(&fine).unwrap();
    assert!((all.volume_after - all.volume_before).abs() <= 1e-10, "{all:?}");
}
