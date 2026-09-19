use super::*;
use crate::host::{AllocStats, Allocator, Sink};

struct Arena {
    stats: AllocStats,
    sealed: bool,
}
impl Allocator for Arena {
    fn alloc_persistent(&mut self, bytes: usize) -> Result<usize, AllocError> {
        if self.sealed {
            self.stats.refused_after_seal += 1;
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

struct Jobs;
impl Sink for Jobs {
    fn warn(&self, _: &str) {}
    fn metric(&self, _: &str, _: f64) {}
}
impl JobSystem for Jobs {
    fn worker_count(&self) -> u32 {
        1
    }
    fn parallel_reduce_ordered_f64(
        &self,
        n: usize,
        grain: usize,
        r: &dyn Fn(usize, usize) -> f64,
        m: &dyn Fn(f64, f64) -> f64,
        init: f64,
    ) -> f64 {
        let g = grain.max(1);
        let mut acc = init;
        let mut s = 0;
        while s < n {
            let e = (s + g).min(n);
            acc = m(acc, r(s, e));
            s = e;
        }
        acc
    }
}

fn volume(nx: usize, ny: usize, nz: usize, dx: f32, g: f32) -> (Volume3, Arena) {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let v = Volume3::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx, ny, nz, dx }, 1025., g).unwrap();
    (v, arena)
}

/// Pseudo-aléatoire déterministe, sans dépendance : essais reproductibles au bit.
fn noise(n: usize, seed: u64) -> Vec<f32> {
    let mut s = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    (0..n).map(|_| {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        ((s >> 40) as f32 / (1u64 << 24) as f32) * 2. - 1.
    }).collect()
}

#[test]
fn configuration_refuses_what_it_cannot_hold_s295() {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    for d in [Domain3 { nx: 0, ny: 2, nz: 2, dx: 1. }, Domain3 { nx: 2, ny: 0, nz: 2, dx: 1. },
              Domain3 { nx: 2, ny: 2, nz: 0, dx: 1. }, Domain3 { nx: 2, ny: 2, nz: 2, dx: 0. },
              Domain3 { nx: 2, ny: 2, nz: 2, dx: f32::NAN }, Domain3 { nx: usize::MAX, ny: 2, nz: 2, dx: 1. }] {
        assert_eq!(Volume3::configure(&mut host, d, 1025., 9.81).err(), Some(Error::Domain), "{d:?}");
    }
    let d = Domain3 { nx: 2, ny: 2, nz: 2, dx: 1. };
    assert_eq!(Volume3::configure(&mut host, d, 0., 9.81).err(), Some(Error::NotFinite));
    assert_eq!(Volume3::configure(&mut host, d, 1025., f32::INFINITY).err(), Some(Error::NotFinite));
    // I-06 : scellé, l'hôte refuse, et la configuration le dit au lieu d'allouer ailleurs.
    host.alloc.seal();
    assert_eq!(Volume3::configure(&mut host, d, 1025., 9.81).err(), Some(Error::Domain));
    assert_eq!(arena.stats.refused_after_seal, 1);
}

#[test]
fn configuration_counts_every_buffer_it_holds_s295() {
    let (v, arena) = volume(5, 3, 4, 0.5, 9.81);
    let floats = 3 * (v.u.len() + v.v.len() + v.w.len())
        + v.p.len() + v.rhs.len() + v.res.len() + v.dir.len() + v.tmp.len() + v.saved_p.len()
        + v.eta.len() + v.eta_roundoff.len() + v.saved_eta.len() + v.saved_eta_roundoff.len()
        + v.flux_x.len() + v.flux_y.len();
    assert_eq!(arena.stats.persistent_bytes, floats * 4);
    assert_eq!(arena.stats.persistent_calls, 1);
    assert!(v.surface().iter().all(|e| *e == 2.));
}

#[test]
fn pressure_operator_is_symmetric_and_positive_s295() {
    for (nx, ny, nz) in [(1usize, 1usize, 1usize), (4, 3, 5), (7, 1, 6), (6, 5, 4)] {
        let (v, _) = volume(nx, ny, nz, 0.37, 9.81);
        let n = v.domain.cells();
        let (x, y) = (noise(n, 3), noise(n, 11));
        let (mut ax, mut ay) = (vec![0.; n], vec![0.; n]);
        v.apply(&x, &mut ax);
        v.apply(&y, &mut ay);
        let dot = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(p, q)| *p as f64 * *q as f64).sum::<f64>();
        let (xay, yax, xax) = (dot(&x, &ay), dot(&y, &ax), dot(&x, &ax));
        let scale = dot(&x, &x).sqrt() * dot(&ay, &ay).sqrt();
        assert!((xay - yax).abs() <= 1e-5 * scale, "{nx}x{ny}x{nz} : {xay} contre {yax}");
        // Dirichlet au couvercle : défini positif, pas seulement semi-défini.
        assert!(xax > 0., "{nx}x{ny}x{nz}");
    }
}

#[test]
fn operator_reduces_to_the_2d_one_at_ny_1_s295() {
    use crate::delta_projection::{Domain, Volume};
    let (nx, nz, dx) = (9usize, 6usize, 0.5f32);
    let (v3, _) = volume(nx, 1, nz, dx, 9.81);
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let v2 = Volume::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain { nx, nz, dx }, 1025., 9.81, &vec![0.; nx]).unwrap();
    let p = noise(nx * nz, 5);
    let mut a3 = vec![0.; nx * nz];
    v3.apply(&p, &mut a3);
    let a2 = v2.apply_for_tests(&p);
    for c in 0..nx * nz {
        assert_eq!(a3[c].to_bits(), a2[c].to_bits(), "maille {c}");
    }
}

fn project_current(v: &mut Volume3, dt: f32, max_iters: u32) -> Report {
    v.us.copy_from_slice(&v.u);
    v.vs.copy_from_slice(&v.v);
    v.ws.copy_from_slice(&v.w);
    v.project(-v.rho / dt, dt / v.rho, max_iters, &Jobs).unwrap()
}

#[test]
fn projection_leaves_a_divergence_free_field_s295() {
    for (nx, ny, nz) in [(8usize, 6usize, 5usize), (12, 1, 7), (5, 9, 4)] {
        let (mut v, _) = volume(nx, ny, nz, 0.25, 9.81);
        let (u, vv, w) = (noise(v.u.len(), 1), noise(v.v.len(), 2), noise(v.w.len(), 3));
        v.set_velocity(&u, &vv, &w).unwrap();
        let r = project_current(&mut v, 0.002, 4000);
        assert!(!r.degraded, "{nx}x{ny}x{nz} : {r:?}");
        assert!(r.iterations > 0 && r.divergence <= PROJECTION_DIVERGENCE_TOLERANCE, "{r:?}");
        // La divergence publiée est celle du champ publié, recalculée ici indépendamment.
        let mut div = vec![0.; v.domain.cells()];
        v.divergence(&v.u, &v.v, &v.w, &mut div);
        let umax = v.u.iter().chain(&v.v).chain(&v.w).fold(0f32, |m, x| m.max(x.abs()));
        let d = div.iter().fold(0f32, |m, x| m.max(x.abs())) as f64 * v.domain.dx as f64 / umax as f64;
        assert!((d - r.divergence).abs() <= 1e-6 * d, "{d} contre {}", r.divergence);
        // Murs et fond : vitesse normale nulle au bit.
        for k in 0..nz {
            for j in 0..ny {
                assert_eq!(v.u[v.fu(0, j, k)].to_bits(), 0);
                assert_eq!(v.u[v.fu(nx, j, k)].to_bits(), 0);
            }
            for i in 0..nx {
                assert_eq!(v.v[v.fv(i, 0, k)].to_bits(), 0);
                assert_eq!(v.v[v.fv(i, ny, k)].to_bits(), 0);
            }
        }
    }
}

#[test]
fn rest_projects_to_exact_zero_s295() {
    let (mut v, _) = volume(6, 4, 5, 0.5, 9.81);
    let r = project_current(&mut v, 0.002, 100);
    assert_eq!((r.iterations, r.degraded, r.residual, r.divergence), (0, false, 0., 0.));
    assert!(v.p.iter().chain(&v.u).chain(&v.v).chain(&v.w).all(|x| x.to_bits() == 0));
}
