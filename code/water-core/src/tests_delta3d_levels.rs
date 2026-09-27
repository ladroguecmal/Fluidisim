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
    // T4 : étendue, repos, gravité ; ensemble épars ; rien n'est écrit.
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
    let mut sparse = domain(8., 4., 0.5);
    sparse.enable_sparse(&mut host).unwrap();
    assert_eq!(sparse.resample_from(&fine).unwrap_err(), Error::Domain);
    assert!(sparse.eta.iter().all(|e| *e == REST));
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
