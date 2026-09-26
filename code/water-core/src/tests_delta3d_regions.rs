//! Essais de la recopie d'état entre domaines (S396, C8a).
use super::*;
use crate::apic3d::tests::{Arena, Jobs};
use crate::host::AllocStats;

const DX: f32 = 0.25;
const NZ: usize = 12;
const REST: f32 = 2.0;
const DT_US: u64 = 20_000;

fn domain(nx: usize, ny: usize) -> Volume3 {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut v = Volume3::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx, ny, nz: NZ, dx: DX }, 1000., 9.81).unwrap();
    v.clear_to_rest(REST).unwrap();
    v
}

/// Une bosse gaussienne de 5 cm centrée en `(cx, cy)` mailles, dans un domaine de `nx × ny`.
fn with_bump(nx: usize, ny: usize, cx: f32, cy: f32) -> Volume3 {
    let mut v = domain(nx, ny);
    let eta: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| {
            let (x, y) = (i as f32 + 0.5 - cx, j as f32 + 0.5 - cy);
            REST + 0.05 * (-(x * x + y * y) / 8.).exp()
        }))
        .collect();
    v.set_free_surface(&eta, REST).unwrap();
    v
}

fn bits(a: &[f32]) -> Vec<u32> {
    a.iter().map(|x| x.to_bits()).collect()
}

#[test]
fn a_split_then_fused_domain_comes_back_at_bit_but_for_the_cut_s396() {
    // Critère 2 de S396 : C → (A, B) → C', au bit partout sauf les faces `u` de la coupure, qui deviennent des murs.
    let mut c = with_bump(32, 16, 8., 8.);
    for _ in 0..20 {
        c.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
    }
    let (mut a, mut b) = (domain(16, 16), domain(16, 16));
    assert_eq!(a.transplant(&c, [0, 0]).unwrap(), 256);
    assert_eq!(b.transplant(&c, [-16, 0]).unwrap(), 256);
    let mut back = domain(32, 16);
    back.transplant(&a, [0, 0]).unwrap();
    back.transplant(&b, [16, 0]).unwrap();
    assert_eq!(bits(&back.eta), bits(&c.eta));
    assert_eq!(bits(&back.eta_roundoff), bits(&c.eta_roundoff));
    assert_eq!(bits(&back.v), bits(&c.v));
    assert_eq!(bits(&back.w), bits(&c.w));
    assert_eq!(bits(&back.p), bits(&c.p));
    // Les faces `u` : toutes égales, sauf la coupure (i = 16), nulle ; les murs de A et de B, nuls.
    let (mut lost, mut flux) = (0f32, 0f64);
    for k in 0..NZ {
        for j in 0..16 {
            for i in 0..=32 {
                let (x, y) = (back.u[back.fu(i, j, k)], c.u[c.fu(i, j, k)]);
                if i == 16 {
                    assert_eq!(x, 0.);
                    lost = lost.max(y.abs());
                    flux += (y as f64).abs() * (DX as f64).powi(2);
                } else {
                    assert_eq!(x.to_bits(), y.to_bits(), "face ({i}, {j}, {k})");
                }
            }
            assert_eq!(a.u[a.fu(16, j, k)], 0.);
            assert_eq!(b.u[b.fu(0, j, k)], 0.);
        }
    }
    println!("S396 aller-retour : au bit hors coupure ; coupure en x = 4 m, vitesse perdue max {lost:.3e} m/s, débit {flux:.3e} m³/s");
}

#[test]
fn the_transplanted_state_is_all_the_step_reads_s396() {
    // La coupure rendue, C' fait les mêmes vingt pas que C, au bit : la recopie porte tout ce que le pas mobile lit.
    let mut c = with_bump(32, 16, 8., 8.);
    for _ in 0..10 {
        c.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
    }
    let (mut a, mut b) = (domain(16, 16), domain(16, 16));
    a.transplant(&c, [0, 0]).unwrap();
    b.transplant(&c, [-16, 0]).unwrap();
    let mut back = domain(32, 16);
    back.transplant(&a, [0, 0]).unwrap();
    back.transplant(&b, [16, 0]).unwrap();
    for k in 0..NZ {
        for j in 0..16 {
            let f = c.fu(16, j, k);
            back.u[f] = c.u[f];
        }
    }
    for _ in 0..20 {
        let (r1, r2) = (c.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap(), back.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap());
        assert_eq!(r1.iterations, r2.iterations);
    }
    assert_eq!(bits(&back.eta), bits(&c.eta));
    assert_eq!(bits(&back.u), bits(&c.u));
    assert_eq!(bits(&back.w), bits(&c.w));
}

#[test]
fn a_transplant_between_different_lattices_is_refused_s396() {
    let src = with_bump(16, 16, 8., 8.);
    let mut other_rest = domain(16, 16);
    other_rest.clear_to_rest(REST + 0.25).unwrap();
    assert_eq!(other_rest.transplant(&src, [0, 0]), Err(Error::Domain));
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut other_dx = Volume3::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx: 16, ny: 16, nz: NZ, dx: 0.1 }, 1000., 9.81).unwrap();
    other_dx.clear_to_rest(REST).unwrap();
    assert_eq!(other_dx.transplant(&src, [0, 0]), Err(Error::Domain));
    // Sans recouvrement : rien n'est recopié.
    let mut far = domain(16, 16);
    assert_eq!(far.transplant(&src, [40, 0]).unwrap(), 0);
    assert!(far.eta.iter().all(|x| *x == REST));
}
