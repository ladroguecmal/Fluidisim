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

// ---- P4 : le pas à surface linéarisée -----------------------------------------------------------

/// Onde stationnaire de S233 : `η = z₀ + A·cos(π(i + ½)/n)` sur un bassin de 8 m, `h` = 4 m.
fn s233_surface(n: usize) -> Vec<f32> {
    (0..n).map(|i| 4. + 0.01 * (std::f64::consts::PI * (i as f64 + 0.5) / n as f64).cos() as f32).collect()
}

#[test]
fn ny_1_reproduces_the_2d_linear_trajectory_s295() {
    use crate::delta_projection::{Domain, Volume};
    struct Still;
    impl crate::host::MonotonicClock for Still { fn now_ns(&self) -> u64 { 0 } }
    for (n, us) in [(16usize, 2000u64), (32, 1000)] {
        let dx = 8. / n as f32;
        let (mut v3, _) = volume(n, 1, n / 2, dx, 9.81);
        let mut arena = Arena { stats: AllocStats::default(), sealed: false };
        let mut v2 = Volume::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
            Domain { nx: n, nz: n / 2, dx }, 1025., 9.81, &vec![0.; n]).unwrap();
        let eta = s233_surface(n);
        v3.set_surface(&eta).unwrap();
        v2.set_surface(&eta).unwrap();
        let (mut worst, mut identical) = (0f64, true);
        for step in 0..1_000_000 / us {
            let r3 = v3.step_surface_linear(us, 2000, &Jobs).unwrap();
            let r2 = v2.step_surface_linear(us, 2000, 1_000_000, &Jobs, &Still).unwrap().report.unwrap();
            assert_eq!(r3.iterations, r2.iterations, "n={n} pas {step}");
            for (a, b) in v3.surface().iter().zip(v2.surface()) {
                worst = worst.max((*a as f64 - *b as f64).abs());
                identical &= a.to_bits() == b.to_bits();
            }
        }
        println!("S295 ny=1 contre 2D : n={n} dt={us} µs, écart maximal {worst:e} m, identique au bit : {identical}");
        assert!(identical, "n={n} : écart {worst:e} m");
    }
}

#[test]
fn a_surface_independent_of_y_stays_independent_of_y_s295() {
    let (nx, ny, nz) = (16usize, 5usize, 8usize);
    let (mut v, _) = volume(nx, ny, nz, 0.5, 9.81);
    let line = s233_surface(nx);
    let eta: Vec<f32> = (0..ny).flat_map(|_| line.iter().copied()).collect();
    v.set_surface(&eta).unwrap();
    for _ in 0..200 {
        v.step_surface_linear(2000, 2000, &Jobs).unwrap();
    }
    for j in 1..ny {
        for i in 0..nx {
            assert_eq!(v.eta[v.col(i, j)].to_bits(), v.eta[v.col(i, 0)].to_bits(), "colonne ({i},{j})");
            for k in 0..nz {
                assert_eq!(v.u[v.fu(i, j, k)].to_bits(), v.u[v.fu(i, 0, k)].to_bits());
                assert_eq!(v.w[v.fw(i, j, k)].to_bits(), v.w[v.fw(i, 0, k)].to_bits());
            }
        }
    }
    assert!(v.v.iter().all(|x| *x == 0.), "aucune vitesse transverse ne doit naître");
    assert!(v.surface()[0] < 4.01, "l'onde a bien évolué");
}

#[test]
fn rest_is_exact_and_guards_refuse_s295() {
    let (mut v, _) = volume(8, 6, 4, 1., 9.81);
    for _ in 0..100 {
        let r = v.step_surface_linear(2000, 100, &Jobs).unwrap();
        assert_eq!(r.iterations, 0);
        assert!(v.surface().iter().all(|e| *e == 4.));
        assert!(v.u.iter().chain(&v.v).chain(&v.w).chain(&v.p).all(|x| x.to_bits() == 0));
    }
    assert_eq!(v.step_surface_linear(0, 100, &Jobs).err(), Some(Error::NotFinite));
    // dt²·g/dx = 0,5²·9,81 > 1 : hors de la garde de S233.
    assert_eq!(v.step_surface_linear(500_000, 100, &Jobs).err(), Some(Error::Domain));
    v.g_eff = -1.;
    assert_eq!(v.step_surface_linear(2000, 100, &Jobs).err(), Some(Error::Domain));
}

#[test]
fn refusal_restores_every_published_field_s295() {
    let (mut v, _) = volume(12, 8, 6, 0.5, 9.81);
    let eta: Vec<f32> = (0..12 * 8).map(|c| 3. + 0.02 * ((c % 12) as f32 * 0.7).sin() * ((c / 12) as f32 * 0.4).cos()).collect();
    v.set_surface(&eta).unwrap();
    for _ in 0..5 { v.step_surface_linear(2000, 2000, &Jobs).unwrap(); }
    let before: Vec<Vec<u32>> = [&v.u, &v.v, &v.w, &v.p, &v.eta, &v.eta_roundoff]
        .iter().map(|f| f.iter().map(|x| x.to_bits()).collect()).collect();
    assert_eq!(v.step_surface_linear(2000, 1, &Jobs).err(), Some(Error::Convergence));
    let after: Vec<Vec<u32>> = [&v.u, &v.v, &v.w, &v.p, &v.eta, &v.eta_roundoff]
        .iter().map(|f| f.iter().map(|x| x.to_bits()).collect()).collect();
    assert_eq!(before, after);
    // Et la reprise est celle d'un pas qui n'aurait jamais été tenté.
    v.step_surface_linear(2000, 2000, &Jobs).unwrap();
}

/// Fréquence **du schéma** pour le mode `(kx, ky)`, calculée hors du solveur : valeur propre
/// horizontale discrète `κ² = (4/dx²)(sin²(kx·dx/2) + sin²(ky·dx/2))`, structure verticale `φ` du
/// problème discret à fond de Neumann et couvercle de Dirichlet à une demi-maille, `S = Σ φ·dx`,
/// puis `ω_s² = g·κ²·S` et Euler symplectique : `cos(Ω·dt) = 1 − ω_s²·dt²/2`. Départ au repos
/// cinématique : `ηⁿ = A·(cos(nΩdt) + β·sin(nΩdt))`, `β = −ω_s²·dt²/(2·sin(Ω·dt))` — le
/// demi-pas qui sépare hauteur et flux dans Euler symplectique. Rend `(Ω, β)`. Sépare une faute
/// d'implémentation de l'erreur de discrétisation.
fn scheme_frequency(kx: f64, ky: f64, dx: f64, nz: usize, g: f64, dt: f64) -> (f64, f64) {
    let kappa2 = 4. / (dx * dx) * ((kx * dx / 2.).sin().powi(2) + (ky * dx / 2.).sin().powi(2));
    // Système tridiagonal (Thomas) : diag·φ_k − φ_{k±1} = second membre, le tout multiplié par dx².
    let mut a = vec![0f64; nz]; // sous-diagonale
    let mut b = vec![0f64; nz]; // diagonale
    let mut c = vec![0f64; nz]; // sur-diagonale
    let mut d = vec![0f64; nz];
    for k in 0..nz {
        let mut diag = kappa2 * dx * dx;
        if k > 0 { diag += 1.; a[k] = -1.; }
        if k + 1 < nz { diag += 1.; c[k] = -1.; } else { diag += 2.; d[k] = 2.; }
        b[k] = diag;
    }
    for k in 1..nz {
        let m = a[k] / b[k - 1];
        b[k] -= m * c[k - 1];
        d[k] -= m * d[k - 1];
    }
    let mut phi = vec![0f64; nz];
    phi[nz - 1] = d[nz - 1] / b[nz - 1];
    for k in (0..nz - 1).rev() { phi[k] = (d[k] - c[k] * phi[k + 1]) / b[k]; }
    let s: f64 = phi.iter().map(|x| x * dx).sum();
    let ws2 = g * kappa2 * s;
    let big_omega = (1. - ws2 * dt * dt / 2.).acos() / dt;
    (big_omega, -ws2 * dt * dt / (2. * (big_omega * dt).sin()))
}

#[test]
fn oblique_standing_wave_follows_its_dispersion_s295() {
    // Mode (1, 1) d'une cuve de 8 × 4 m, h = 4 m, A = 1 cm, vitesse nulle, 1 s : critère 3 du plan.
    let (lx, ly, h, a, g) = (8f64, 4f64, 4f64, 0.01f64, 9.81f64);
    let pi = std::f64::consts::PI;
    let (kx, ky) = (pi / lx, pi / ly);
    let k = (kx * kx + ky * ky).sqrt();
    let omega = (g * k * (k * h).tanh()).sqrt();
    let mut continuous = Vec::new();
    for (n, us) in [(16usize, 2000u64), (32, 1000)] {
        let dx = lx / n as f64;
        let (nx, ny, nz) = (n, (ly / dx) as usize, (h / dx) as usize);
        let (mut v, _) = volume(nx, ny, nz, dx as f32, g as f32);
        let mode = |i: usize, j: usize| (kx * (i as f64 + 0.5) * dx).cos() * (ky * (j as f64 + 0.5) * dx).cos();
        let eta: Vec<f32> = (0..ny).flat_map(|j| (0..nx).map(move |i| (h + a * mode(i, j)) as f32)).collect();
        v.set_surface(&eta).unwrap();
        let (big_omega, beta) = scheme_frequency(kx, ky, dx, nz, g, us as f64 * 1e-6);
        let (mut e_cont, mut e_scheme) = (0f64, 0f64);
        for step in 1..=1_000_000 / us {
            v.step_surface_linear(us, 4000, &Jobs).unwrap();
            let t = (step * us) as f64 * 1e-6;
            for j in 0..ny {
                for i in 0..nx {
                    let h_num = v.eta[v.col(i, j)] as f64 - h;
                    e_cont = e_cont.max((h_num - a * mode(i, j) * (omega * t).cos()).abs() / a);
                    let phase = big_omega * t;
                    e_scheme = e_scheme.max((h_num - a * mode(i, j) * (phase.cos() + beta * phase.sin())).abs() / a);
                }
            }
        }
        println!("S295 onde oblique (1,1) n={n} dt={us} µs : erreur {:.4} % contre ω continue, {:.4} % contre Ω du schéma             (Ω/ω − 1 = {:.3e})", 100. * e_cont, 100. * e_scheme, big_omega / omega - 1.);
        // Le solveur est le schéma : l'écart restant est l'arrondi f32 et la tolérance de pression.
        assert!(e_scheme < 1e-3, "n={n} : {e_scheme}");
        continuous.push(e_cont);
    }
    assert!(continuous[1] < continuous[0], "le raffinement doit réduire l'erreur : {continuous:?}");
    assert!(continuous[1] < 0.01, "{continuous:?}");
}
