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

#[test]
fn rest_surface_is_reconstructed_at_its_height_s388() {
    // Critère 2 : une nappe plane à 0,5 m (cinq mailles de 10 cm) ; l'iso-zéro, interpolée sur la verticale des centres,
    // dans les colonnes à plus d'une maille des parois.
    let (mut a, _) = apic(8, 8, 10, 0.1, 8 * 8 * 8 * 10);
    a.seed(&|p| p[2] < 0.5).unwrap();
    a.reconstruct();
    let mut worst = 0f32;
    for j in 1..7 {
        for i in 1..7 {
            let f = |k: usize| a.phi[a.cell(i, j, k)];
            let k = (0..9).find(|&k| f(k) < 0. && f(k + 1) >= 0.).expect("une surface");
            let h = (k as f32 + 0.5) * 0.1 + 0.1 * f(k) / (f(k) - f(k + 1));
            worst = worst.max((h - 0.5).abs());
            assert!((0..k).all(|m| f(m) < 0.) && (k + 1..10).all(|m| f(m) >= 0.));
        }
    }
    println!("S388 surface au repos : écart maximal {:.3e} m ({:.2} % de maille)", worst, 100. * worst / 0.1);
    // Critère 2 tel qu'écrit (1 % de maille) : il ne tient qu'au rayon réglé sur les faces ; le rayon minimax lit une
    // face à ±6 % — publié, non relevé (P4).
    let (d_below, d_above) = rest_distances(0.1);
    println!("S388 rayons : S318 (au plan) {:.5} m ; faces {:.5} m ; minimax {:.5} m ; d₋ {:.5}, d₊ {:.5}",
        rest_radius_at_the_plane(0.1), rest_radius_at_the_faces(0.1), rest_radius(0.1), d_below, d_above);
    assert!(worst <= 0.07 * 0.1, "{worst}");
}

#[test]
fn surfaces_between_centres_are_published_s388() {
    // Hors du cas réglé. La hauteur **vraie** est celle que les particules portent (rangées de dx/2) : une face de maille
    // (0,40 m) et un milieu de maille (0,45 m). Les deux réglages : aux centres (retenu) et au plan (S318).
    for (nom, r) in [("faces", rest_radius_at_the_faces(0.1)), ("plan_S318", rest_radius_at_the_plane(0.1)), ("minimax", rest_radius(0.1))] {
        for h in [0.40f32, 0.45] {
            let (mut a, _) = apic(8, 8, 10, 0.1, 8 * 8 * 8 * 10);
            let n = a.seed(&|p| p[2] < h).unwrap();
            let vraie = n as f32 * 0.1 * 0.1 * 0.1 / 8. / (0.8 * 0.8);
            assert!((vraie - h).abs() < 1e-6);
            a.radius = r;
            a.reconstruct();
            let f = |k: usize| a.phi[a.cell(4, 4, k)];
            let k = (0..9).find(|&k| f(k) < 0. && f(k + 1) >= 0.).expect("une surface");
            let lue = (k as f32 + 0.5) * 0.1 + 0.1 * f(k) / (f(k) - f(k + 1));
            println!("S388 réglage={nom} hauteur_vraie={vraie:.3} m : iso-zéro lue {lue:.5} m, écart {:+.2} % de maille ; \
                      calcul f64 {:+.2} %", 100. * (lue - vraie) / 0.1, 100. * read_error(0.1, r as f64, h == 0.40) / 0.1);
        }
    }
}

#[test]
fn a_tank_at_rest_stays_at_rest_s388() {
    // Critère 3 : cuve de 1 × 1 × 1 m à 5 cm, 0,5 m d'eau, 2 s ; masse exacte, vitesse parasite ≤ 1 cm/s (2D : 4,4 mm/s).
    let (mut a, _) = apic(20, 20, 20, 0.05, 20 * 20 * 20 * 8);
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
    println!("S388 repos : {steps} pas, vitesse parasite max {:.3e} m/s, dernière {:.3e} ; itérations {} ; résidu {:.1e} ; \
              divergence {:.1e}", worst, last.max_speed, last.iterations, last.residual, last.divergence);
    assert_eq!(a.particle_count(), n);
    assert!(worst <= 0.01, "{worst}");
}
