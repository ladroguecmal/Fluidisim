//! S401 — le domaine épars (C8b) : l'ensemble de colonnes dans `Volume3`.
use super::*;
use crate::apic3d::tests::{Arena, Jobs};
use crate::host::AllocStats;

const DX: f32 = 0.25;
const NZ: usize = 12;
const REST: f32 = 2.0;
const DT_US: u64 = 20_000;

fn host_arena() -> Arena {
    Arena { stats: AllocStats::default(), sealed: false }
}

fn domain(nx: usize, ny: usize, sparse: bool) -> Volume3 {
    let mut arena = host_arena();
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let mut v = Volume3::configure(&mut host, Domain3 { nx, ny, nz: NZ, dx: DX }, 1000., 9.81).unwrap();
    v.clear_to_rest(REST).unwrap();
    if sparse {
        v.enable_sparse(&mut host).unwrap();
    }
    v
}

/// Une bosse gaussienne de 5 cm centrée en `(cx, cy)` mailles.
fn bump(nx: usize, ny: usize, cx: f32, cy: f32) -> Vec<f32> {
    (0..ny)
        .flat_map(|j| {
            (0..nx).map(move |i| {
                let (x, y) = (i as f32 + 0.5 - cx, j as f32 + 0.5 - cy);
                REST + 0.05 * (-(x * x + y * y) / 8.).exp()
            })
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

#[test]
fn a_full_set_is_the_box_at_bit_s401() {
    // Critère 1 : l'ensemble réservé, toutes les colonnes y sont — cinquante pas mobiles au bit du pas sans ensemble, avec et
    // sans le terme de second ordre de l'advection (ADR-209), qui lit lui aussi le bord de l'ensemble.
    for correction in [false, true] {
        let (mut plain, mut sparse) = (domain(24, 16, false), domain(24, 16, true));
        for v in [&mut plain, &mut sparse] {
            v.set_free_surface(&bump(24, 16, 9., 7.), REST).unwrap();
            if correction {
                v.enable_advection_correction();
            }
        }
        assert_eq!(sparse.sparse_cells(), plain.domain().cells());
        for s in 0..50 {
            let (a, b) = (plain.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap(), sparse.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap());
            assert_eq!(a.iterations, b.iterations, "pas {s}");
            assert!(same_state(&plain, &sparse), "pas {s}, correction {correction}");
        }
    }
}

#[test]
fn a_column_that_leaves_goes_to_rest_and_what_it_carried_is_published_s401() {
    let (nx, ny) = (24, 16);
    let mut v = domain(nx, ny, true);
    v.set_free_surface(&bump(nx, ny, 9., 7.), REST).unwrap();
    for _ in 0..10 {
        v.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
    }
    // Les colonnes i ≥ 16 sortent : ce qu'elles portaient est compté, puis elles sont au repos, faces nulles.
    let before = v.perturbation_volume();
    let mut carried = 0f64;
    let mut high = 0f32;
    for j in 0..ny {
        for i in 16..nx {
            let c = v.col(i, j);
            carried += ((v.eta[c] as f64 - v.eta_roundoff[c] as f64) - REST as f64) * (DX as f64).powi(2);
            high = high.max((v.eta[c] - REST).abs());
        }
    }
    let mask: Vec<u8> = (0..nx * ny).map(|c| u8::from(c % nx < 16)).collect();
    let change = v.set_active_columns(&mask).unwrap();
    assert_eq!((change.activated, change.deactivated, change.active_columns), (0, 8 * ny, 16 * ny));
    assert_eq!(change.volume_removed.to_bits(), carried.to_bits());
    assert_eq!(change.height_removed, high);
    assert!(change.velocity_removed > 0.);
    assert!((v.perturbation_volume() - (before - carried)).abs() < 1e-12);
    assert_eq!(v.sparse_cells(), 16 * ny * NZ);
    // Hors de l'ensemble, après dix pas encore : le repos au bit, toutes les faces nulles.
    for _ in 0..10 {
        v.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
    }
    for j in 0..ny {
        for i in 16..nx {
            assert_eq!(v.eta[v.col(i, j)].to_bits(), REST.to_bits());
            for k in 0..NZ {
                assert_eq!(v.u[v.fu(i + 1, j, k)], 0.);
                assert_eq!(v.v[v.fv(i, j, k)], 0.);
                assert_eq!(v.p[v.c(i, j, k)], 0.);
            }
            for k in 0..=NZ {
                assert_eq!(v.w[v.fw(i, j, k)], 0.);
            }
        }
        // Le bord de l'ensemble est un mur.
        for k in 0..NZ {
            assert_eq!(v.u[v.fu(16, j, k)], 0.);
        }
    }
    // Elles reviennent au repos : rien n'est compté.
    let back = v.set_active_columns(&vec![1u8; nx * ny]).unwrap();
    assert_eq!((back.activated, back.deactivated, back.volume_removed, back.velocity_removed), (8 * ny, 0, 0., 0.));
}

#[test]
fn paths_that_do_not_carry_the_set_refuse_it_s401() {
    let (nx, ny) = (16, 8);
    let mut v = domain(nx, ny, true);
    let mut out: Vec<u8> = vec![1; nx * ny];
    out[0] = 0;
    v.set_active_columns(&out).unwrap();
    // Le pas linéaire, la colonne graduée, la recopie d'état.
    assert_eq!(v.step_surface_linear(DT_US, 100, &Jobs).unwrap_err(), Error::Domain);
    let mut arena = host_arena();
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    assert_eq!(v.enable_graded(&mut host, &[0, 4, NZ - 1]).unwrap_err(), Error::Domain);
    assert_eq!(v.enable_sparse(&mut host).unwrap_err(), Error::Domain);
    let other = domain(nx, ny, false);
    assert_eq!(v.transplant(&other, [0, 0]).unwrap_err(), Error::Domain);
    let mut plain = domain(nx, ny, false);
    assert_eq!(plain.transplant(&v, [0, 0]).unwrap_err(), Error::Domain);
    // Hors de l'ensemble : ni surface écartée du repos, ni source ; rien n'est écrit.
    let mut eta = vec![REST; nx * ny];
    eta[0] = REST + 0.01;
    assert_eq!(v.set_free_surface(&eta, REST).unwrap_err(), Error::Domain);
    let mut dh = vec![0f32; nx * ny];
    dh[0] = 1e-3;
    assert_eq!(v.add_column_volume(&dh).unwrap_err(), Error::Domain);
    assert_eq!(v.eta[0].to_bits(), REST.to_bits());
    dh[0] = 0.;
    dh[1] = 1e-3;
    v.add_column_volume(&dh).unwrap();
    // Sans ensemble réservé, ou de mauvaise longueur.
    assert_eq!(plain.set_active_columns(&out).unwrap_err(), Error::Domain);
    assert_eq!(v.set_active_columns(&out[1..]).unwrap_err(), Error::Shape);
    // Le repos qui se déplace emmène les colonnes hors de l'ensemble.
    v.shift_rest(REST + 0.1).unwrap();
    assert_eq!(v.eta[0].to_bits(), (REST + 0.1).to_bits());
}

#[test]
fn blocks_cover_their_columns_clipped_to_the_window_s401() {
    use crate::domain_blocks::{Block, BLOCK};
    let (nx, ny) = (20, 12);
    let mut v = domain(nx, ny, true);
    let origin = Block { i: 10, j: 0 };
    // Le bloc (10, −1) couvre les colonnes [0, 8) × [−8, 0) : hors de la fenêtre. (11, 0) : [8, 16) × [0, 8). (12, 0) : [16, 20)
    // × [0, 8), coupé au bord. (10, 1) : [0, 8) × [8, 12), coupé.
    let change = v
        .set_active_blocks(&[Block { i: 10, j: -1 }, Block { i: 11, j: 0 }, Block { i: 12, j: 0 }, Block { i: 10, j: 1 }], origin)
        .unwrap();
    let active = v.active_columns().unwrap();
    for j in 0..ny {
        for i in 0..nx {
            let want = (j < BLOCK && i >= BLOCK) || (j >= BLOCK && i < BLOCK);
            assert_eq!(active[j * nx + i] != 0, want, "colonne ({i}, {j})");
        }
    }
    assert_eq!(change.active_columns, 8 * 8 + 4 * 8 + 8 * 4);
}
