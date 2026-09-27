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

// ---------------------------------------------------------------------------------------------------------------------
// Critère 2 : des oracles indépendants — le bord de la boîte d'un domaine dense contre le bord de l'ensemble.

/// Un domaine de `nx × ny`, avec la multigrille si `mg`, l'ensemble réservé si `sparse`.
fn configured(nx: usize, ny: usize, sparse: bool, mg: bool) -> Volume3 {
    let mut arena = host_arena();
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let mut v = Volume3::configure(&mut host, Domain3 { nx, ny, nz: NZ, dx: DX }, 1000., 9.81).unwrap();
    v.clear_to_rest(REST).unwrap();
    if mg {
        v.enable_multigrid(&mut host).unwrap();
    }
    if sparse {
        v.enable_sparse(&mut host).unwrap();
    }
    v
}

/// Le rectangle `[i0, i1) × [j0, j1)` d'une fenêtre de `nx` colonnes.
fn rect_mask(nx: usize, ny: usize, rects: &[[usize; 4]]) -> Vec<u8> {
    (0..nx * ny)
        .map(|c| {
            let (i, j) = (c % nx, c / nx);
            u8::from(rects.iter().any(|r| i >= r[0] && i < r[1] && j >= r[2] && j < r[3]))
        })
        .collect()
}

/// La surface d'une fenêtre, bosses données dans ses colonnes, au repos hors de l'ensemble.
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

/// Le plus grand écart de surface entre la fenêtre (colonnes `[i0, i0 + n.nx) × [j0, j0 + n.ny)`) et un domaine dense.
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

#[test]
fn a_rectangle_of_the_set_is_the_dense_rectangle_s401() {
    // (a) Un rectangle de 16 × 8 colonnes dans une fenêtre de 32 × 24, contre le domaine dense de 16 × 8 : bosse de 5 cm, 5 s.
    // Critère : ≤ 10 µm ; prédiction : à l'arrondi du solveur, ≤ 1 µm. Avec et sans la multigrille.
    for mg in [false, true] {
        let (nx, ny) = (32, 24);
        let mask = rect_mask(nx, ny, &[[8, 24, 8, 16]]);
        let mut window = configured(nx, ny, true, mg);
        window.set_active_columns(&mask).unwrap();
        window.set_free_surface(&bumps_in(nx, ny, &mask, &[(14., 11.)]), REST).unwrap();
        let mut dense = configured(16, 8, false, mg);
        dense.set_free_surface(&bumps_in(16, 8, &[1; 128], &[(6., 3.)]), REST).unwrap();
        let (mut worst, mut its) = (0f32, (0u32, 0u32));
        for _ in 0..250 {
            its.0 += window.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap().iterations;
            its.1 += dense.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap().iterations;
            worst = worst.max(gap(&window, &dense, 8, 8));
        }
        println!("S401 oracle (a) rectangle, multigrille {mg} : écart max {worst:.3e} m sur 5 s ; itérations {} contre {}", its.0, its.1);
        assert!(worst <= 1e-5, "écart {worst}");
    }
}

#[test]
fn two_disjoint_rectangles_are_two_dense_domains_s401() {
    // (b) Deux rectangles de 16 × 16 séparés de 8 colonnes hors de l'ensemble, contre deux domaines denses : l'ensemble les
    // sépare sans recopie (la partition d'ADR-006 §3). Critère : ≤ 10 µm sur 5 s.
    let (nx, ny) = (40, 16);
    let mask = rect_mask(nx, ny, &[[0, 16, 0, 16], [24, 40, 0, 16]]);
    let mut window = configured(nx, ny, true, false);
    window.set_active_columns(&mask).unwrap();
    window.set_free_surface(&bumps_in(nx, ny, &mask, &[(12., 6.), (29., 9.)]), REST).unwrap();
    let (mut a, mut b) = (configured(16, 16, false, false), configured(16, 16, false, false));
    a.set_free_surface(&bumps_in(16, 16, &[1; 256], &[(12., 6.)]), REST).unwrap();
    b.set_free_surface(&bumps_in(16, 16, &[1; 256], &[(5., 9.)]), REST).unwrap();
    let mut worst = 0f32;
    for _ in 0..250 {
        window.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
        a.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
        b.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
        worst = worst.max(gap(&window, &a, 0, 0)).max(gap(&window, &b, 24, 0));
    }
    // L'écart entre les deux : le repos au bit.
    for j in 0..ny {
        for i in 16..24 {
            assert_eq!(window.eta[j * nx + i].to_bits(), REST.to_bits());
        }
    }
    println!("S401 oracle (b) deux rectangles : écart max {worst:.3e} m sur 5 s");
    assert!(worst <= 1e-5, "écart {worst}");
}

#[test]
fn an_l_shaped_set_keeps_its_volume_s401() {
    // (c) Un L de blocs (un coin rentrant) : le volume de perturbation, au plancher de la cuve dense de même fenêtre.
    let (nx, ny) = (24, 24);
    let mask = rect_mask(nx, ny, &[[0, 24, 0, 8], [0, 8, 8, 24]]);
    let mut l = configured(nx, ny, true, false);
    l.set_active_columns(&mask).unwrap();
    l.set_free_surface(&bumps_in(nx, ny, &mask, &[(6., 6.)]), REST).unwrap();
    let mut dense = configured(nx, ny, false, false);
    dense.set_free_surface(&bumps_in(nx, ny, &[1; 576], &[(6., 6.)]), REST).unwrap();
    let (v0, d0) = (l.perturbation_volume(), dense.perturbation_volume());
    let (mut drift, mut floor) = (0f64, 0f64);
    for _ in 0..250 {
        l.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
        dense.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
        drift = drift.max((l.perturbation_volume() - v0).abs());
        floor = floor.max((dense.perturbation_volume() - d0).abs());
    }
    println!("S401 oracle (c) L : dérive du volume {drift:.3e} m³ (cuve dense : {floor:.3e}) sur 5 s, volume {v0:.4e} m³");
    assert!(drift <= 10. * floor.max(1e-15), "dérive {drift} contre {floor}");
}

// ---------------------------------------------------------------------------------------------------------------------
// Critère 3, en petit : le domaine qui suit une source mobile, contre le domaine entier (le banc `delta3d_epars` en grand).

/// La gaussienne tronquée à 1,5 m, d'écart type 0,5 m, normalisée sur la grille : `Σ g·dx²` = 1.
fn gaussian(nx: usize, ny: usize, c: [f32; 2], out: &mut [f32]) {
    out.fill(0.);
    let mut sum = 0f64;
    for j in 0..ny {
        for i in 0..nx {
            let (x, y) = ((i as f32 + 0.5) * DX - c[0], (j as f32 + 0.5) * DX - c[1]);
            if x * x + y * y <= 1.5 * 1.5 {
                let g = (-(x * x + y * y) / 0.5).exp();
                out[j * nx + i] = g;
                sum += g as f64;
            }
        }
    }
    let norm = (1. / (sum * (DX as f64).powi(2))) as f32;
    out.iter_mut().for_each(|g| *g *= norm);
}

#[test]
#[ignore = "lent (≈ 30 s) : le banc delta3d_epars le mesure en grand"]
fn a_domain_that_follows_a_moving_source_stays_within_the_image_tolerance_s401() {
    use crate::domain_blocks::{useful_horizon, Follow, Tracked};
    // 32 × 8 m, une source dipôle (0,05 m³/s, 1 m) qui avance à 2 m/s pendant 3 s, balistique (`a_max` nul, horizon 1 s) ; le
    // domaine épars la suit — 4 m autour de l'activité et de l'enveloppe, libération après 0,25 s. Critère 3 : ≤ 3 mm du
    // domaine entier, la source toujours dedans, et l'ensemble plus petit que la fenêtre.
    let (nx, ny) = (128, 32);
    let mut full = configured(nx, ny, false, true);
    let mut sparse = configured(nx, ny, true, true);
    let mut arena = host_arena();
    let mut follow = Follow::with_capacity(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, nx, ny, DX, 2,
        1e-3, 250_000)
    .unwrap();
    let track = |t: f64| Tracked {
        position: [3. + 2. * t as f32, 4.],
        velocity: [2., 0.],
        a_max: 0.,
        horizon: useful_horizon(4., 0., 1.),
        radius: 2.,
    };
    let (mut front, mut back, mut dh, mut mask) = (vec![0f32; nx * ny], vec![0f32; nx * ny], vec![0f32; nx * ny], vec![0u8; nx * ny]);
    follow.update(0, sparse.surface(), REST, nx, &[track(0.)]);
    follow.columns(nx, ny, &mut mask);
    sparse.set_active_columns(&mask).unwrap();
    let (mut gap, mut part, mut amplitude) = (0f32, 0f64, 0f32);
    for s in 0..150u64 {
        let t = s as f64 * 0.02;
        let x = 3. + 2. * (t + 0.01) as f32;
        gaussian(nx, ny, [x + 0.5, 4.], &mut front);
        gaussian(nx, ny, [x - 0.5, 4.], &mut back);
        let ramp = t.min(1.);
        let q = 0.05 * (ramp * ramp * (3. - 2. * ramp)) as f32 * 0.02;
        for c in 0..nx * ny {
            dh[c] = q * (front[c] - back[c]);
        }
        full.add_column_volume(&dh).unwrap();
        sparse.add_column_volume(&dh).expect("la source hors du domaine épars");
        full.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
        sparse.step_surface_mobile(DT_US, 20_000, &Jobs).unwrap();
        follow.update((s + 1) * DT_US, sparse.surface(), REST, nx, &[track((s + 1) as f64 * 0.02)]);
        follow.columns(nx, ny, &mut mask);
        sparse.set_active_columns(&mask).unwrap();
        for (a, b) in full.surface().iter().zip(sparse.surface()) {
            gap = gap.max((a - b).abs());
            amplitude = amplitude.max((a - REST).abs());
        }
        part += sparse.sparse_cells() as f64 / full.domain().cells() as f64;
    }
    println!("S401 suivi (essai) : écart {gap:.3e} m, amplitude {amplitude:.4} m, part moyenne des mailles {:.3}", part / 150.);
    assert!(gap <= 3e-3, "écart {gap}");
    assert!(part / 150. < 0.9, "part {}", part / 150.);
}
