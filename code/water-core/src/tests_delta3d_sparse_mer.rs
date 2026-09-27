//! S404 — l'ensemble épars sous le pas couplé (C8e) : le bord de l'ensemble contre le bord de la boîte, sous une houle réelle
//! de B, en mode relatif (ADR-198 D1).
use super::*;
use crate::apic3d::tests::{Arena, Jobs};
use crate::background::{Background, SeaState};
use crate::host::AllocStats;
use crate::{SimTime, WorldPos};

const DX: f32 = 0.25;
const NZ: usize = 12;
const REST: f32 = 2.0;
const DT_US: u64 = 20_000;
const RHO: f32 = 1025.;

fn host_arena() -> Arena {
    Arena { stats: AllocStats::default(), sealed: false }
}

/// La houle de S369 — une composante de 5 cm, `T` = 3,2 s —, oblique (36°) : les deux axes portent la bande de B.
fn swell() -> Background {
    let mut arena = host_arena();
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let state = SeaState { hs: 0.05 * 2. * 2f32.sqrt(), tp: 3.2, theta_turns: 0.1, components: 1, graine: 7 };
    Background::configure(&mut host, state, WorldPos::from_units(0, 0, 0)).unwrap()
}

/// Un domaine en mer de `nx × ny` colonnes dont le coin est en `origin` (m, dans le repère de B), et la grille de B à ses
/// faces ; mode relatif ; multigrille si `mg` ; ensemble réservé si `sparse`.
fn at_sea(houle: &Background, nx: usize, ny: usize, origin: [f32; 2], sparse: bool, mg: bool) -> (Volume3, BackgroundGrid3) {
    let mut arena = host_arena();
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let d = Domain3 { nx, ny, nz: NZ, dx: DX };
    let mut v = Volume3::configure(&mut host, d, RHO, houle.gravity()).unwrap();
    v.set_free_surface(&vec![REST; nx * ny], REST).unwrap();
    v.set_relative_background(Volume3::RELATIVE_ALL).unwrap();
    if mg {
        v.enable_multigrid(&mut host).unwrap();
    }
    if sparse {
        v.enable_sparse(&mut host).unwrap();
    }
    let grid = BackgroundGrid3::configure(&mut host, d, [origin[0], origin[1], -REST], RHO).unwrap();
    (v, grid)
}

/// Un pas couplé à l'instant `n·dt`, B échantillonnée à ses faces.
fn step(v: &mut Volume3, grid: &mut BackgroundGrid3, houle: &Background, n: u64, sponge: Sponge3) -> Report {
    let time = SimTime(n * DT_US);
    grid.sample(houle, time).unwrap();
    let bg = grid.view().unwrap();
    v.step_perturbation_mobile(time, DT_US, 20_000, &bg, sponge, &Jobs).unwrap()
}

/// Les bosses de 5 cm des essais de S401, dans les colonnes non nulles de `mask`, au repos ailleurs.
fn bumps_in(nx: usize, ny: usize, mask: &[u8], centres: &[(f32, f32)]) -> Vec<f32> {
    (0..nx * ny)
        .map(|c| {
            if mask[c] == 0 {
                return REST;
            }
            let (i, j) = (c % nx, c / nx);
            REST + centres
                .iter()
                .map(|(cx, cy)| {
                    let (x, y) = (i as f32 + 0.5 - cx, j as f32 + 0.5 - cy);
                    0.05 * (-(x * x + y * y) / 8.).exp()
                })
                .sum::<f32>()
        })
        .collect()
}

fn rect_mask(nx: usize, ny: usize, rects: &[[usize; 4]]) -> Vec<u8> {
    (0..nx * ny)
        .map(|c| {
            let (i, j) = (c % nx, c / nx);
            u8::from(rects.iter().any(|r| i >= r[0] && i < r[1] && j >= r[2] && j < r[3]))
        })
        .collect()
}

fn bits(a: &[f32]) -> Vec<u32> {
    a.iter().map(|x| x.to_bits()).collect()
}

fn same_state(a: &Volume3, b: &Volume3) -> bool {
    bits(&a.eta) == bits(&b.eta)
        && bits(&a.eta_roundoff) == bits(&b.eta_roundoff)
        && bits(&a.u) == bits(&b.u)
        && bits(&a.v) == bits(&b.v)
        && bits(&a.w) == bits(&b.w)
        && bits(&a.p) == bits(&b.p)
}

/// Le plus grand écart de surface entre la fenêtre (colonnes `[i0, i0 + nx) × [j0, j0 + ny)` du dense) et le dense.
fn gap(window: &Volume3, dense: &Volume3, i0: usize, j0: usize) -> f32 {
    let (wn, d) = (window.domain().nx, dense.domain());
    let mut worst = 0f32;
    for j in 0..d.ny {
        for i in 0..d.nx {
            worst = worst.max((window.eta[(j0 + j) * wn + i0 + i] - dense.eta[j * d.nx + i]).abs());
        }
    }
    worst
}

/// Hors de l'ensemble : le repos au bit, toutes les faces des colonnes dehors nulles.
fn outside_at_rest(v: &Volume3) -> bool {
    let Domain3 { nx, ny, nz, .. } = v.domain();
    let active = v.active_columns().unwrap();
    (0..ny).all(|j| {
        (0..nx).all(|i| {
            active[j * nx + i] != 0
                || (v.eta[v.col(i, j)].to_bits() == REST.to_bits()
                    && (0..nz).all(|k| {
                        v.u[v.fu(i, j, k)] == 0. && v.u[v.fu(i + 1, j, k)] == 0. && v.v[v.fv(i, j, k)] == 0.
                            && v.v[v.fv(i, j + 1, k)] == 0. && v.p[v.c(i, j, k)] == 0.
                    })
                    && (0..=nz).all(|k| v.w[v.fw(i, j, k)] == 0.))
        })
    })
}

const SPONGE: Sponge3 = Sponge3 { width_x: 1.0, width_y: 0.5, rate_per_s: 2. };

#[test]
fn a_full_set_under_the_coupled_step_is_the_box_at_bit_s404() {
    // Critère 1 : l'ensemble réservé, plein — cinquante pas couplés relatifs sous la houle, au bit du pas sans ensemble, avec
    // et sans le terme d'ADR-209, éponge comprise.
    let houle = swell();
    for correction in [false, true] {
        let (mut plain, mut gp) = at_sea(&houle, 24, 16, [0., 0.], false, false);
        let (mut full, mut gf) = at_sea(&houle, 24, 16, [0., 0.], true, false);
        for v in [&mut plain, &mut full] {
            v.set_free_surface(&bumps_in(24, 16, &[1; 384], &[(9., 7.)]), REST).unwrap();
            if correction {
                v.enable_advection_correction();
            }
        }
        for n in 0..50 {
            let a = step(&mut plain, &mut gp, &houle, n, SPONGE);
            let b = step(&mut full, &mut gf, &houle, n, SPONGE);
            assert_eq!(a.iterations, b.iterations, "pas {n}");
            assert!(same_state(&plain, &full), "pas {n}, correction {correction}");
            assert_eq!(plain.balance(), full.balance(), "pas {n}");
        }
    }
}

/// L'oracle (a) : un rectangle 16 × 8 dans une fenêtre 32 × 24, contre le dense du rectangle — même houle, échantillonnée aux
/// mêmes points —, 5 s. Rend l'écart max, le bilan de chacun (résidu max rapporté à son échelle), l'écart des bandes entrées, et
/// si le dehors est resté au repos.
fn rectangle_against_dense(mg: bool, edge_sponge: bool) -> (f32, [f64; 2], f64, bool) {
    let houle = swell();
    let (nx, ny) = (32, 24);
    let mask = rect_mask(nx, ny, &[[8, 24, 8, 16]]);
    let (mut window, mut gw) = at_sea(&houle, nx, ny, [0., 0.], true, mg);
    window.set_active_columns(&mask).unwrap();
    window.set_sparse_edge_sponge_for_trials(edge_sponge).unwrap();
    window.set_free_surface(&bumps_in(nx, ny, &mask, &[(14., 11.)]), REST).unwrap();
    let (mut dense, mut gd) = at_sea(&houle, 16, 8, [8. * DX, 8. * DX], false, mg);
    dense.set_free_surface(&bumps_in(16, 8, &[1; 128], &[(6., 3.)]), REST).unwrap();
    // Les échantillons de B aux faces du rectangle sont ceux de la fenêtre, au bit : les coordonnées sont dyadiques.
    gw.sample(&houle, SimTime(0)).unwrap();
    gd.sample(&houle, SimTime(0)).unwrap();
    let (bw, bd) = (gw.view().unwrap(), gd.view().unwrap());
    for k in 0..NZ {
        for j in 0..8 {
            for i in 0..=16 {
                let (a, b) = (&bw.u[window.fu(i + 8, j + 8, k)], &bd.u[dense.fu(i, j, k)]);
                assert_eq!((a.eta.to_bits(), a.u[0].to_bits(), a.p_dyn.to_bits()), (b.eta.to_bits(), b.u[0].to_bits(), b.p_dyn.to_bits()));
            }
        }
    }
    let (mut worst, mut residual, mut scale, mut band) = (0f32, [0f64; 2], [0f64; 2], 0f64);
    for n in 0..250 {
        step(&mut window, &mut gw, &houle, n, SPONGE);
        step(&mut dense, &mut gd, &houle, n, SPONGE);
        worst = worst.max(gap(&window, &dense, 8, 8));
        for (s, v) in [&window, &dense].into_iter().enumerate() {
            let b = v.balance();
            assert_eq!(b.perturbation_in, 0., "δ ne traverse ni le bord ni un mur");
            residual[s] = residual[s].max(b.residual.abs());
            scale[s] = scale[s].max(b.band_in.abs()).max(b.delta.abs());
        }
        band = band.max((window.balance().band_in - dense.balance().band_in).abs() / scale[1]);
    }
    (worst, [residual[0] / scale[0], residual[1] / scale[1]], band, outside_at_rest(&window))
}

#[test]
fn a_rectangle_of_the_set_is_the_dense_rectangle_at_sea_s404() {
    // Critère 2 (a) et (c). Critère : ≤ 10 µm sur 5 s ; prédiction ≤ 1 µm, l'ordre des sommes. Le bilan de la fenêtre se ferme
    // au plancher du dense, les murs comptés au bord.
    for mg in [false, true] {
        let (worst, ratio, band, rest) = rectangle_against_dense(mg, true);
        println!(
            "S404 oracle (a) rectangle en mer, multigrille {mg} : écart max {worst:.3e} m sur 5 s ; bilan, résidu/échelle \
             {:.2e} (épars) contre {:.2e} (dense) ; bandes entrées, écart relatif {band:.2e}",
            ratio[0], ratio[1]
        );
        assert!(rest, "hors de l'ensemble, le repos");
        assert!(worst <= 1e-5, "écart {worst}");
        assert!(ratio[0] <= 1e-6 && ratio[0] <= 10. * ratio[1].max(1e-9), "bilan {ratio:?}");
        assert!(band <= 1e-4, "bandes {band}");
    }
}

#[test]
fn without_the_edge_sponge_the_set_is_not_the_dense_rectangle_s404() {
    // Le témoin : l'éponge mesurée depuis le seul bord de la boîte — le rectangle n'a plus celle de son dense, qui en couvre une
    // bonne part (1 m et 0,5 m sur 4 × 2 m), et s'en écarte bien au-delà du critère : c'est l'éponge du bord de l'ensemble qui en
    // fait le bord de la boîte.
    let (worst, _, _, rest) = rectangle_against_dense(false, false);
    assert!(rest);
    println!("S404 témoin, murs sans éponge : écart max {worst:.3e} m sur 5 s");
    assert!(worst > 1e-4, "écart {worst}");
}

#[test]
fn two_disjoint_rectangles_at_sea_are_two_dense_domains_s404() {
    // (b) Deux rectangles de 16 × 16 séparés de 8 colonnes, contre deux denses : l'écart ≤ 10 µm sur 5 s, et le repos au bit
    // entre les deux.
    let houle = swell();
    let (nx, ny) = (40, 16);
    let mask = rect_mask(nx, ny, &[[0, 16, 0, 16], [24, 40, 0, 16]]);
    let (mut window, mut gw) = at_sea(&houle, nx, ny, [0., 0.], true, false);
    window.set_active_columns(&mask).unwrap();
    window.set_free_surface(&bumps_in(nx, ny, &mask, &[(12., 6.), (29., 9.)]), REST).unwrap();
    let (mut a, mut ga) = at_sea(&houle, 16, 16, [0., 0.], false, false);
    let (mut b, mut gb) = at_sea(&houle, 16, 16, [24. * DX, 0.], false, false);
    a.set_free_surface(&bumps_in(16, 16, &[1; 256], &[(12., 6.)]), REST).unwrap();
    b.set_free_surface(&bumps_in(16, 16, &[1; 256], &[(5., 9.)]), REST).unwrap();
    let mut worst = 0f32;
    for n in 0..250 {
        step(&mut window, &mut gw, &houle, n, SPONGE);
        step(&mut a, &mut ga, &houle, n, SPONGE);
        step(&mut b, &mut gb, &houle, n, SPONGE);
        worst = worst.max(gap(&window, &a, 0, 0)).max(gap(&window, &b, 24, 0));
    }
    assert!(outside_at_rest(&window));
    println!("S404 oracle (b) deux rectangles en mer : écart max {worst:.3e} m sur 5 s");
    assert!(worst <= 1e-5, "écart {worst}");
}

#[test]
fn the_s297_step_refuses_the_set_s404() {
    // Le pas de S297 donne à δ les restes de B partout : un ensemble ne peut pas le porter. Refus `Domain`, rien n'est écrit ;
    // il faut les trois bits relatifs.
    let houle = swell();
    let (mut v, mut g) = at_sea(&houle, 16, 8, [0., 0.], true, false);
    let mask = rect_mask(16, 8, &[[0, 12, 0, 8]]);
    v.set_active_columns(&mask).unwrap();
    v.set_free_surface(&bumps_in(16, 8, &mask, &[(6., 3.)]), REST).unwrap();
    let before = (bits(&v.eta), bits(&v.u), bits(&v.p));
    g.sample(&houle, SimTime(0)).unwrap();
    let bg = g.view().unwrap();
    for terms in [0, Volume3::RELATIVE_RESIDUAL | Volume3::RELATIVE_BAND, Volume3::RELATIVE_BAND | Volume3::RELATIVE_SURFACE] {
        v.set_relative_background(terms).unwrap();
        assert_eq!(v.step_perturbation_mobile(SimTime(0), DT_US, 20_000, &bg, SPONGE, &Jobs).unwrap_err(), Error::Domain);
        assert_eq!((bits(&v.eta), bits(&v.u), bits(&v.p)), before);
    }
    // Les trois bits, et un essai en plus : accepté.
    v.set_relative_background(Volume3::RELATIVE_ALL | Volume3::TRIAL_NO_CROSS_BAND).unwrap();
    v.step_perturbation_mobile(SimTime(0), DT_US, 20_000, &bg, SPONGE, &Jobs).unwrap();
    // Sans ensemble, le témoin d'essai de l'éponge est refusé.
    let (mut plain, _) = at_sea(&houle, 16, 8, [0., 0.], false, false);
    assert_eq!(plain.set_sparse_edge_sponge_for_trials(false).unwrap_err(), Error::Domain);
}
