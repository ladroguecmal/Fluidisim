//! Essais de la référence APIC 3D (S388, C4a).
use super::*;
use crate::host::{AllocStats, Allocator, HostServices, JobSystem, Sink};

pub(crate) struct Arena {
    pub stats: AllocStats,
    pub sealed: bool,
}
impl Allocator for Arena {
    fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AllocError> {
        if self.sealed {
            return Err(AllocError::Sealed);
        }
        self.stats.persistent_bytes += bytes;
        self.stats.persistent_calls += 1;
        Ok(0)
    }
    fn seal(&mut self) {
        self.sealed = true;
    }
    fn is_sealed(&self) -> bool {
        self.sealed
    }
    fn stats(&self) -> AllocStats {
        self.stats
    }
}
pub(crate) struct Jobs;
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
        let g = grain.max(1);
        let (mut acc, mut s) = (init, 0);
        while s < n {
            let e = (s + g).min(n);
            acc = m(acc, r(s, e));
            s = e;
        }
        acc
    }
}

pub(crate) fn apic(nx: usize, ny: usize, nz: usize, dx: f32, capacity: usize) -> (Apic3, Arena) {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let a = Apic3::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx, ny, nz, dx }, 1000., 9.81, capacity).unwrap();
    (a, arena)
}

#[test]
fn configuration_counts_what_it_holds_s388() {
    let (a, arena) = apic(6, 5, 4, 0.1, 1000);
    let floats = a.u.len() + a.v.len() + a.w.len() + a.wu.len() + a.wv.len() + a.ww.len()
        + a.old_u.len()
        + a.p.len() + a.phi.len() + a.rhs.len() + a.r.len() + a.z.len() + a.d.len() + a.q.len() + a.diag.len()
        + 18 * a.x.len();
    let bytes = a.valid_u.len() + a.valid_v.len() + a.valid_w.len() + a.old_valid.len() + a.label.len();
    let words = a.bin_count.len() + a.bin_start.len() + a.order.len();
    assert_eq!(arena.stats.persistent_bytes, 4 * floats + bytes + 4 * words);
    assert_eq!(reserved_bytes(a.domain(), 1000), Some(arena.stats.persistent_bytes));
    let mut sealed = Arena { stats: AllocStats::default(), sealed: true };
    assert!(Apic3::configure(&mut HostServices { alloc: &mut sealed, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx: 2, ny: 2, nz: 2, dx: 0.1 }, 1000., 9.81, 10).is_err());
}

#[test]
fn seeding_places_eight_per_cell_and_refuses_beyond_capacity_s388() {
    let (mut a, _) = apic(4, 3, 5, 0.1, 480);
    // L'eau sous z = 0,25 m : deux mailles et demie ; les positions au quart de maille donnent 5 rangées de 2 × 2.
    assert_eq!(a.seed(&|p| p[2] < 0.25).unwrap(), 4 * 3 * 4 * 5);
    assert_eq!(a.seed(&|_| true).unwrap(), 480);
    let (mut b, _) = apic(4, 3, 5, 0.1, 100);
    assert_eq!(b.seed(&|_| true), Err(Error::Domain));
    assert_eq!(b.particle_count(), 0);
    a.bin();
    for c in 0..a.bin_count.len() {
        assert_eq!(a.bin_start[c + 1] - a.bin_start[c], 8);
        for s in a.bin_start[c]..a.bin_start[c + 1] {
            let p = a.x[a.order[s as usize] as usize];
            let (i, j, k) = a.cell_of(p);
            assert_eq!(a.cell(i, j, k), c);
        }
    }
}

#[test]
fn apic_transfers_keep_an_affine_field_s388() {
    // Critère 1 : v = a + B·x, C = B, grille entièrement couverte ; aller et retour, particules à une demi-maille des parois.
    let (mut s, _) = apic(8, 7, 6, 0.1, 8 * 8 * 7 * 6);
    s.seed(&|_| true).unwrap();
    let a0 = [0.3f32, -0.2, 0.1];
    let b = [[0.5f32, -1.0, 0.25], [0.75, 0.2, -0.6], [-0.4, 0.9, 0.1]];
    for k in 0..s.n {
        let p = s.x[k];
        for r in 0..3 {
            s.vel[k][r] = a0[r] + b[r][0] * p[0] + b[r][1] * p[1] + b[r][2] * p[2];
        }
        s.c[k] = b;
    }
    let before: Vec<_> = (0..s.n).map(|k| (s.vel[k], s.c[k])).collect();
    s.particles_to_grid();
    s.grid_to_particles();
    let dx = 0.1f32;
    let (lx, ly, lz) = (0.8f32, 0.7f32, 0.6f32);
    let mut checked = 0;
    for k in 0..s.n {
        let p = s.x[k];
        let interior = p[0] > 0.5 * dx && p[0] < lx - 0.5 * dx && p[1] > 0.5 * dx && p[1] < ly - 0.5 * dx
            && p[2] > 0.5 * dx && p[2] < lz - 0.5 * dx;
        if !interior {
            continue;
        }
        checked += 1;
        let (v0, c0) = before[k];
        for r in 0..3 {
            assert!((s.vel[k][r] - v0[r]).abs() <= 1e-5 * (1. + v0[r].abs()), "v {k} {r} : {} contre {}", s.vel[k][r], v0[r]);
            for q in 0..3 {
                assert!((s.c[k][r][q] - c0[r][q]).abs() <= 1e-4, "C {k} {r}{q} : {} contre {}", s.c[k][r][q], c0[r][q]);
            }
        }
    }
    assert!(checked > s.n / 3, "{checked}");
}

/// La hauteur que lit la pression dans la colonne `(i, j)` : l'iso-zéro interpolée entre deux centres.
fn read_height(a: &Apic3, i: usize, j: usize) -> f32 {
    let nz = a.domain().nz;
    let f = |k: usize| a.phi[a.cell(i, j, k)];
    let k = (0..nz - 1).find(|&k| f(k) < 0. && f(k + 1) >= 0.).expect("une surface");
    let dx = a.domain().dx;
    (k as f32 + 0.5) * dx + dx * f(k) / (f(k) - f(k + 1))
}

#[test]
fn surface_reading_follows_its_model_at_every_position_s389() {
    // Critères 1 et 2 de S389 : la nappe entière glissée sur huit positions d'une maille ; la lecture 3D contre le modèle
    // f64 à 0,1 % de maille, noyaux d'une et de deux mailles ; et, au noyau retenu, l'écart sous 2,5 % de maille partout.
    let dx = 0.1f32;
    for kernel in [1f32, KERNEL_CELLS] {
        let r = minimax_radius(dx as f64, kernel as f64).0;
        let mut worst = 0f32;
        for m in 0..READ_POSITIONS {
            let offset = (m as f32 + 0.5) / READ_POSITIONS as f32;
            let (mut a, _) = apic(8, 8, 12, dx, 8 * 8 * 8 * 12);
            a.seed(&|p| p[2] < 0.5).unwrap();
            let shift = (offset - 0.5) * dx;
            for k in 0..a.n {
                a.x[k][2] = (a.x[k][2] + shift).max(1e-3);
            }
            a.set_reconstruction_kernel(kernel);
            a.reconstruct();
            let surface = 0.5 + shift;
            let lue = read_height(&a, 4, 4) - surface;
            let modele = lattice_read_error(dx as f64, kernel as f64, r, offset as f64) as f32;
            println!("S389 noyau={kernel} position={offset:.4} : lue {:+.3} %, modèle {:+.3} % de maille",
                100. * lue / dx, 100. * modele / dx);
            assert!((lue - modele).abs() <= 1e-3 * dx, "noyau {kernel}, position {offset} : {lue} contre {modele}");
            worst = worst.max(lue.abs());
        }
        println!("S389 noyau={kernel} rayon={r:.5} m : pire lecture {:.2} % de maille", 100. * worst / dx);
        if kernel == KERNEL_CELLS {
            assert!(worst <= 0.025 * dx, "{worst}");
        }
    }
    // Pour mémoire, le rayon de S318 (noyau d'une maille, au point de la surface).
    println!("S389 rayon de S318 : {:.5} m ; retenu : {:.5} m", rest_radius_at_the_plane(dx), rest_radius(dx));
}

#[test]
fn a_tank_at_rest_stays_at_rest_s388() {
    // Critère 3 de S388 (et de S389) : cuve de 1 × 1 × 1 m à 5 cm, 0,5 m d'eau, 2 s ; masse exacte, vitesse parasite ≤ 1 cm/s.
    // S388, noyau d'une maille sans images aux parois : 5,6 mm/s. S389 : le témoin au noyau d'une maille, parois reflétées.
    for kernel in [KERNEL_CELLS, 1.] {
        let (mut a, _) = apic(20, 20, 20, 0.05, 20 * 20 * 20 * 8);
        a.set_reconstruction_kernel(kernel);
        let n = a.seed(&|p| p[2] < 0.5).unwrap();
        let (mut t, mut worst, mut steps) = (0u64, 0f32, 0);
        let mut last = ApicReport::default();
        while t < 2_000_000 {
            let us = a.stable_step_us(20_000).min(2_000_000 - t);
            last = a.step(us).unwrap();
            worst = worst.max(last.max_speed);
            t += us;
            steps += 1;
        }
        println!("S388 repos noyau={kernel} : {steps} pas, vitesse parasite max {:.3e} m/s, dernière {:.3e} ; itérations {} ; \
                  résidu {:.1e} ; divergence {:.1e}", worst, last.max_speed, last.iterations, last.residual, last.divergence);
        assert_eq!(a.particle_count(), n);
        assert!(worst <= 0.01, "{worst}");
    }
}

/// Une sphère de 0,4 m au coin d'un quart de cuve à 5 cm (`D/dx` = 8) : l'axe au coin, les deux parois sont deux plans de
/// symétrie (S393).
fn quarter_with_sphere(nz: usize, water: f32, center_z: f32, velocity: f32) -> (Apic3, usize) {
    let (mut a, _) = apic(16, 16, nz, 0.05, 16 * 16 * nz * 8);
    let body = Sphere3 { center: [0., 0., center_z], radius: 0.2, velocity: [0., 0., velocity] };
    a.set_body(Some(body)).unwrap();
    let n = a
        .seed(&|p| p[2] < water && p[0] * p[0] + p[1] * p[1] + (p[2] - center_z) * (p[2] - center_z) >= 0.2 * 0.2)
        .unwrap();
    (a, n)
}

#[test]
fn a_body_is_refused_when_not_finite_or_empty_s393() {
    let (mut a, _) = apic(4, 4, 4, 0.1, 100);
    let ok = Sphere3 { center: [0.2, 0.2, 0.2], radius: 0.1, velocity: [0.; 3] };
    assert_eq!(a.set_body(Some(Sphere3 { radius: 0., ..ok })), Err(Error::Domain));
    assert_eq!(a.set_body(Some(Sphere3 { center: [f32::NAN, 0., 0.], ..ok })), Err(Error::NotFinite));
    assert_eq!(a.body(), None);
    a.set_body(Some(ok)).unwrap();
    assert_eq!(a.body(), Some(ok));
}

#[test]
fn a_sphere_at_rest_half_immersed_stays_at_rest_s393() {
    // Critère 2 de S393 : à demi immergée, au repos, 2 s ; masse exacte, vitesse parasite ≤ 1 cm/s (2D, S320 : 5,4 mm/s).
    let (mut a, n) = quarter_with_sphere(16, 0.5, 0.5, 0.);
    let (mut t, mut worst, mut steps) = (0u64, 0f32, 0);
    let mut last = ApicReport::default();
    while t < 2_000_000 {
        let us = a.stable_step_us(20_000).min(2_000_000 - t);
        last = a.step(us).unwrap();
        worst = worst.max(last.max_speed);
        t += us;
        steps += 1;
    }
    let solids = a.labels().iter().filter(|l| **l == SOLID).count();
    println!("S393 sphère au repos : {steps} pas, vitesse parasite max {:.3e} m/s, dernière {:.3e} ; itérations {} ; \
              {solids} mailles solides", worst, last.max_speed, last.iterations);
    assert_eq!(a.particle_count(), n);
    assert!(solids > 0);
    assert!(worst <= 0.01, "{worst}");
}

#[test]
fn an_entering_sphere_keeps_mass_and_pushes_particles_out_s393() {
    // `Fr` = 2 : la sphère part la base au ras de l'eau (0,8 m) et descend à 2·√(g·D) ; vingt pas.
    let u = 2. * (9.81f32 * 0.4).sqrt();
    let (mut a, n) = quarter_with_sphere(24, 0.8, 1.0, -u);
    let mut max_speed = 0f32;
    for _ in 0..20 {
        let us = a.stable_step_us(20_000);
        max_speed = max_speed.max(a.step(us).unwrap().max_speed);
        let b = a.body().unwrap();
        for p in a.particles() {
            let d = ((p[0] - b.center[0]).powi(2) + (p[1] - b.center[1]).powi(2) + (p[2] - b.center[2]).powi(2)).sqrt();
            assert!(d >= b.radius, "une particule dans le corps : {d}");
        }
    }
    let b = a.body().unwrap();
    println!("S393 entrée : centre {:.4} m après vingt pas, vitesse max {max_speed:.3} m/s", b.center[2]);
    assert_eq!(a.particle_count(), n);
    assert!(b.center[2] < 1.0 - 0.1);
}

/// Une cuve toute en colonnes (S398) : aucune particule, surface posée.
fn all_columns(n: usize, eta: &dyn Fn(usize, usize) -> f32) -> Apic3 {
    let (mut a, mut arena) = apic(n, n, n, 0.05, 8);
    a.enable_columns(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, &vec![1u8; n * n]).unwrap();
    assert_eq!(arena.stats.persistent_bytes, reserved_bytes(a.domain(), 8).unwrap() + columns_reserved_bytes(a.domain()).unwrap());
    let surface: Vec<f32> = (0..n * n).map(|c| eta(c % n, c / n)).collect();
    a.set_columns_surface(&surface).unwrap();
    a
}

#[test]
fn a_column_zone_is_refused_twice_or_misshapen_s398() {
    let (mut a, mut arena) = apic(4, 4, 4, 0.1, 8);
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    assert_eq!(a.enable_columns(&mut host, &[1; 15]), Err(Error::Shape));
    assert_eq!(a.set_columns_surface(&[0.2; 16]), Err(Error::Domain));
    a.enable_columns(&mut host, &[1; 16]).unwrap();
    assert_eq!(a.enable_columns(&mut host, &[1; 16]), Err(Error::Domain));
    assert_eq!(a.set_columns_surface(&[f32::NAN; 16]), Err(Error::NotFinite));
    assert!(a.is_column(3, 3));
}

#[test]
fn columns_at_rest_stay_at_rest_s398() {
    // Critère 2 de S398 : toutes colonnes, 1 m de côté à 5 cm, 0,5 m d'eau, 2 s ; vitesse ≤ 1 mm/s, volume à 10⁻⁶.
    let mut a = all_columns(20, &|_, _| 0.5);
    let v0 = a.columns_volume();
    let (mut t, mut worst, mut steps) = (0u64, 0f32, 0);
    while t < 2_000_000 {
        let us = a.stable_step_us(20_000).min(2_000_000 - t);
        a.step(us).unwrap();
        worst = worst.max(a.columns_max_speed());
        t += us;
        steps += 1;
    }
    let drift = a.columns_volume() / v0 - 1.;
    println!("S398 colonnes au repos : {steps} pas, vitesse max {worst:.3e} m/s, volume {drift:+.2e}");
    assert!(worst <= 1e-3, "{worst}");
    assert!(drift.abs() <= 1e-6, "{drift}");
}

#[test]
fn a_column_bump_keeps_its_volume_s398() {
    // Une bosse de 5 cm se déploie une seconde : le transport par débits mouillés garde le volume, à l'arrondi près.
    let mut a = all_columns(20, &|i, j| {
        let (x, y) = (i as f32 - 9.5, j as f32 - 9.5);
        0.5 + 0.05 * (-(x * x + y * y) / 8.).exp()
    });
    let v0 = a.columns_volume();
    let mut t = 0u64;
    while t < 1_000_000 {
        let us = a.stable_step_us(20_000).min(1_000_000 - t);
        a.step(us).unwrap();
        t += us;
    }
    let drift = a.columns_volume() / v0 - 1.;
    let eta = a.columns_surface().unwrap();
    let spread = eta.iter().fold(0f32, |m, e| m.max((e - 0.5).abs()));
    println!("S398 bosse en colonnes : volume {drift:+.2e}, écart max au repos {spread:.4} m (départ 0,05)");
    assert!(drift.abs() <= 1e-6, "{drift}");
    assert!(spread < 0.05);
}
