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
        + v.prec.len() + v.p.len() + v.rhs.len() + v.res.len() + v.dir.len() + v.tmp.len() + v.saved_p.len()
        + v.eta.len() + v.eta_roundoff.len() + v.saved_eta.len() + v.saved_eta_roundoff.len()
        + v.flux_x.len() + v.flux_y.len() + v.band_x.len() + v.band_y.len()
        + v.surface_total.len() + v.ghost_bg_up.len() + v.ghost_bg_x.len() + v.ghost_bg_y.len() + v.pressure_base.len()
        + v.ghost_bg_error.len();
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

#[test]
fn mobile_operator_has_symmetric_ghost_rows_s296() {
    for ny in [1, 4] {
        let (mut v, _) = volume(7, ny, 8, 0.25, 9.81);
        let eta: Vec<_> = (0..7*ny).map(|c| 1. + 0.21 * (c as f32 * 1.7).sin()).collect();
        v.set_free_surface(&eta, 1.).unwrap();
        let (mut x, mut y) = (noise(v.p.len(), 4), noise(v.p.len(), 8));
        for k in 0..8 { for j in 0..ny { for i in 0..7 {
            if !v.wet3(i,j,k) { x[v.c(i,j,k)]=0.; y[v.c(i,j,k)]=0.; }
        }}}
        let (mut ax,mut ay) = (vec![0.;x.len()],vec![0.;y.len()]);
        v.apply_mobile3(&x,&mut ax); v.apply_mobile3(&y,&mut ay);
        let dot = |a:&[f32],b:&[f32]| a.iter().zip(b).map(|(x,y)|*x as f64 * *y as f64).sum::<f64>();
        assert!((dot(&x,&ay)-dot(&y,&ax)).abs() < 1e-5 * (dot(&x,&x)*dot(&ay,&ay)).sqrt());
        assert!(dot(&x,&ax)>0.);
        if ny == 1 {
            use crate::delta_projection::{Domain,Volume,PressureRow};
            let mut arena = Arena { stats: AllocStats::default(), sealed:false };
            let mut v2 = Volume::configure(&mut HostServices{alloc:&mut arena,jobs:&Jobs,sink:&Jobs},
                Domain{nx:7,nz:8,dx:0.25},1025.,9.81,&[0.;7]).unwrap();
            v2.set_free_surface(&eta,1.).unwrap();
            let mut rows = vec![PressureRow::default();56];
            v2.write_mobile_pressure_rows(&mut rows).unwrap();
            for c in 0..56 {
                let mut a = 0f32;
                for (f,offset) in [-1isize,1,-7,7].into_iter().enumerate() {
                    let row = rows[c];
                    if row.weights[f] == 0. {continue;}
                    if row.ghosts[f] != 0. {a += row.weights[f]*x[c]*row.ghosts[f];}
                    else {a += row.weights[f]*(x[c]-x[(c as isize+offset) as usize]);}
                }
                assert_eq!(ax[c].to_bits(),(a*16.).to_bits(),"cell {c}");
            }
        }
    }
}

#[test]
fn mobile_projection_is_conservative_s296() {
    for ny in [1,4] {
        let (mut v,_) = volume(9,ny,9,0.25,9.81);
        let eta: Vec<_>=(0..9*ny).map(|c|1.25+0.2*(c as f32*1.7).sin()).collect();
        v.set_free_surface(&eta,1.25).unwrap();
        v.set_velocity(&noise(v.u.len(),1),&noise(v.v.len(),2),&noise(v.w.len(),3)).unwrap();
        v.us.copy_from_slice(&v.u); v.vs.copy_from_slice(&v.v); v.ws.copy_from_slice(&v.w);
        let r=v.project_mobile3(-1025./0.002,0.002/1025.,4000,&Jobs).unwrap();
        assert!(!r.degraded,"{r:?}");
        assert!(r.divergence_plain<=PROJECTION_DIVERGENCE_TOLERANCE);
    }
}

#[test]
fn mobile_projection_matches_2d_s296() {
    use crate::delta_projection::{Domain,Volume,MOBILE_MULTIGRID_OFF};
    struct Still;
    impl crate::host::MonotonicClock for Still { fn now_ns(&self)->u64 {0} }
    let (mut v3,_) = volume(32,1,36,0.0625,9.81);
    let mut arena=Arena{stats:AllocStats::default(),sealed:false};
    let mut v2=Volume::configure(&mut HostServices{alloc:&mut arena,jobs:&Jobs,sink:&Jobs},
        Domain{nx:32,nz:36,dx:0.0625},1025.,9.81,&[0.;32]).unwrap();
    let eta:Vec<_>=(0..32).map(|i|2.+0.1*(std::f64::consts::PI*(i as f64+0.5)/32.).cos() as f32).collect();
    v3.set_free_surface(&eta,2.).unwrap();v2.set_free_surface(&eta,2.).unwrap();
    MOBILE_MULTIGRID_OFF.with(|c|c.set(true));
    let r2=v2.step_surface_mobile(1000,4000,1_000_000,&Jobs,&Still).unwrap().report.unwrap();
    MOBILE_MULTIGRID_OFF.with(|c|c.set(false));
    let r3=v3.project_mobile3(-1_025_000.,(0.001f64/1025.) as f32,4000,&Jobs).unwrap();
    assert_eq!(r3.iterations,r2.iterations);
    assert_eq!(v3.pressure(),v2.pressure());
    v3.extrapolate_mobile3();
    assert_eq!(v3.velocity_u(),v2.velocity_u());assert_eq!(v3.velocity_w(),v2.velocity_w());
}

#[test]
fn mobile_trajectory_matches_2d_bits_s296() {
    use crate::delta_projection::{Domain,Volume,MOBILE_MULTIGRID_OFF};
    struct Still;
    impl crate::host::MonotonicClock for Still {fn now_ns(&self)->u64{0}}
    struct Reset;
    impl Drop for Reset {fn drop(&mut self){MOBILE_MULTIGRID_OFF.with(|c|c.set(false));}}
    let (mut v3,_) = volume(32,1,36,0.0625,9.81);
    let mut arena=Arena{stats:AllocStats::default(),sealed:false};
    let mut v2=Volume::configure(&mut HostServices{alloc:&mut arena,jobs:&Jobs,sink:&Jobs},
        Domain{nx:32,nz:36,dx:0.0625},1025.,9.81,&[0.;32]).unwrap();
    MOBILE_MULTIGRID_OFF.with(|c|c.set(true)); let _reset=Reset;
    for a in [0.05,0.1] {
        let eta:Vec<_>=(0..32).map(|i|(2.+a*(std::f64::consts::PI*(i as f64+0.5)/32.).cos()) as f32).collect();
        v3.set_free_surface(&eta,2.).unwrap();v2.set_free_surface(&eta,2.).unwrap();
        v3.set_velocity(&vec![0.;v3.u.len()],&vec![0.;v3.v.len()],&vec![0.;v3.w.len()]).unwrap();
        v2.set_velocity(&vec![0.;v2.velocity_u().len()],&vec![0.;v2.velocity_w().len()]).unwrap();
        for step in 0..1604 {
            let r3=v3.step_surface_mobile(1000,4000,&Jobs).unwrap();
            let r2=v2.step_surface_mobile(1000,4000,1_000_000,&Jobs,&Still).unwrap().report.unwrap();
            assert_eq!(r3.iterations,r2.iterations,"a={a} step {step}");
            for (a,b) in v3.surface().iter().zip(v2.surface()).chain(v3.pressure().iter().zip(v2.pressure()))
                .chain(v3.velocity_u().iter().zip(v2.velocity_u())).chain(v3.velocity_w().iter().zip(v2.velocity_w())) {
                assert_eq!(a.to_bits(),b.to_bits(),"step {step}");
            }
        }
    }
}

#[test]
fn mobile_transposition_and_rest_s296() {
    let (mut x,_) = volume(12,7,12,0.25,9.81);
    let (mut y,_) = volume(7,12,12,0.25,9.81);
    let eta:Vec<_>=(0..84).map(|c|2.+0.05*(std::f32::consts::PI*(c%12) as f32/12.).cos()).collect();
    let mut trans=vec![0.;84];
    for j in 0..7 {for i in 0..12 {trans[i*7+j]=eta[j*12+i];}}
    x.set_free_surface(&eta,2.).unwrap();y.set_free_surface(&trans,2.).unwrap();
    let mut worst=0f32;
    for _ in 0..300 {
        x.step_surface_mobile(1000,4000,&Jobs).unwrap();y.step_surface_mobile(1000,4000,&Jobs).unwrap();
        for j in 0..7 {for i in 0..12 {worst=worst.max((x.eta[j*12+i]-y.eta[i*7+j]).abs());}}
    }
    println!("S296 transposition x-y: {worst:e} m");
    assert!(worst<2e-6,"{worst}");
    x.set_free_surface(&[2.013;84],2.013).unwrap();
    x.set_velocity(&vec![0.;x.u.len()],&vec![0.;x.v.len()],&vec![0.;x.w.len()]).unwrap();
    for _ in 0..50 {
        let r=x.step_surface_mobile(1000,4000,&Jobs).unwrap();assert_eq!(r.iterations,0);
        assert!(x.eta.iter().all(|h|h.to_bits()==2.013f32.to_bits()));
        assert!(x.u.iter().chain(&x.v).chain(&x.w).chain(&x.p).all(|u|u.to_bits()==0));
    }
}

#[test]
fn mobile_refusals_restore_state_s296() {
    let (mut v,_) = volume(16,5,12,0.25,9.81);
    let bits=|v:&Volume3| [&v.u,&v.v,&v.w,&v.p,&v.eta,&v.eta_roundoff].iter()
        .map(|f|f.iter().map(|x|x.to_bits()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let eta:Vec<_>=(0..80).map(|c|2.+0.1*(c as f32*0.7).sin()).collect();
    v.set_free_surface(&eta,2.).unwrap();
    for _ in 0..5 {v.step_surface_mobile(1000,4000,&Jobs).unwrap();}
    let before=bits(&v);
    assert_eq!(v.step_surface_mobile(1000,0,&Jobs).err(),Some(Error::Convergence));assert_eq!(bits(&v),before);
    for height in [0.49,2.76] {
        v.set_free_surface(&[height;80],2.).unwrap();let before=bits(&v);
        assert_eq!(v.step_surface_mobile(1000,4000,&Jobs).err(),Some(Error::Domain));assert_eq!(bits(&v),before);
    }
    assert_eq!(v.step_surface_mobile(0,4000,&Jobs).err(),Some(Error::NotFinite));
    // Garde après transport : la crête atteint le sommet admis au demi-cycle.
    let (mut v,_) = volume(16,1,18,0.125,9.81);
    let eta:Vec<_>=(0..16).map(|i|2.+0.125*(std::f64::consts::PI*(i as f64+0.5)/16.).cos() as f32).collect();
    v.set_free_surface(&eta,2.).unwrap();
    let mut refused=false;
    for _ in 0..1604 {
        let before=bits(&v);
        match v.step_surface_mobile(1000,4000,&Jobs) {
            Ok(_)=>{}, Err(Error::Domain)=>{assert_eq!(bits(&v),before);refused=true;break;},
            Err(e)=>panic!("unexpected {e:?}"),
        }
    }
    assert!(refused,"la garde après transport doit être exercée");
}

#[test]
fn mobile_ghost_coefficients_and_jacobi_match_matrix_s296() {
    let (mut v,_) = volume(4,3,7,0.25,9.81);
    let eta:Vec<_>=(0..12).map(|c|1.1+0.2*(c as f32*1.9).sin()).collect();
    v.set_free_surface(&eta,1.1).unwrap();v.rhs_mobile3(0.);
    let n=v.p.len();let mut matrix=vec![vec![0.;n];n];let mut basis=vec![0.;n];
    for c in 0..n {basis[c]=1.;v.apply_mobile3(&basis,&mut matrix[c]);basis[c]=0.;}
    for c in 0..n {
        for d in 0..n {assert_eq!(matrix[c][d].to_bits(),matrix[d][c].to_bits(),"{c},{d}");}
        let inverse=if matrix[c][c]>0. {1./matrix[c][c]} else {0.};
        assert_eq!(v.prec[c].to_bits(),inverse.to_bits());
    }
}

#[test]
fn coupled_geometry_zero_and_oblique_ghosts_s297() {
    use crate::background::BackgroundSample;
    let (mut v,_) = volume(5,4,8,0.25,9.81);
    // S443 : le pas de S297, que cet essai mesure — le mode relatif est le défaut depuis la bascule.
    v.set_relative_background(0).unwrap();
    let u=vec![BackgroundSample::default();v.u.len()];let vv=vec![BackgroundSample::default();v.v.len()];
    let mut w=vec![BackgroundSample::default();v.w.len()];
    let eta:Vec<_>=(0..20).map(|c|1.2+0.2*(c as f32).sin()).collect();v.set_free_surface(&eta,1.2).unwrap();
    let p=noise(v.p.len(),9);let mut before=vec![0.;p.len()];let mut after=before.clone();
    v.apply_mobile3(&p,&mut before);
    v.prepare_background3(&BackgroundFaces3{domain:v.domain,time:crate::SimTime(0),density:1025.,gravity:9.81,u:&u,v:&vv,w:&w}).unwrap();
    v.apply_mobile3(&p,&mut after);assert_eq!(before,after);
    for k in 0..=8 {for j in 0..4 {for i in 0..5 {w[v.fw(i,j,k)].eta=0.1*((i+j) as f32).sin();}}}
    v.prepare_background3(&BackgroundFaces3{domain:v.domain,time:crate::SimTime(0),density:1025.,gravity:9.81,u:&u,v:&vv,w:&w}).unwrap();
    for j in 0..4 {for i in 0..5 {assert_eq!(v.height3(i,j),eta[j*5+i]+w[v.fw(i,j,0)].eta);}}
    assert!(v.ghost_bg_up.iter().any(|x|*x!=0.));
    v.surface_coupled=false;
}

#[test]
fn coupled_zero_background_reproduces_mobile_bits_s297() {
    use crate::{background::BackgroundSample,SimTime};
    for ny in [1,4] {
        let (mut a,_) = volume(12,ny,12,0.25,9.81);let (mut b,_) = volume(12,ny,12,0.25,9.81);
        let eta:Vec<_>=(0..12*ny).map(|c|2.+0.06*(c as f32*0.4).sin()).collect();
        a.set_free_surface(&eta,2.).unwrap();b.set_free_surface(&eta,2.).unwrap();
        let u=vec![BackgroundSample::default();a.u.len()];let v=vec![BackgroundSample::default();a.v.len()];let w=vec![BackgroundSample::default();a.w.len()];
        for n in 0..200 {
            let time=SimTime(n*1000);let bg=BackgroundFaces3{domain:a.domain,time,density:1025.,gravity:9.81,u:&u,v:&v,w:&w};
            a.step_surface_mobile(1000,4000,&Jobs).unwrap();b.step_perturbation_mobile(time,1000,4000,&bg,Sponge3::default(),&Jobs).unwrap();
            for (x,y) in [&a.u,&a.v,&a.w,&a.p,&a.eta,&a.eta_roundoff].into_iter().zip([&b.u,&b.v,&b.w,&b.p,&b.eta,&b.eta_roundoff]) {
                assert!(x.iter().zip(y).all(|(x,y)|x.to_bits()==y.to_bits()),"ny={ny} step={n}");
            }
        }
    }
}

#[test]
fn coupled_uniform_background_crosses_all_four_edges_s297() {
    use crate::{background::BackgroundSample,SimTime};
    let (mut v,_) = volume(8,6,12,0.25,9.81);
    v.set_free_surface(&[2.;48],2.).unwrap();
    let s=BackgroundSample{eta:0.07,u:[0.6,-0.4,0.],p_dyn:1025.*9.81*0.07,..BackgroundSample::default()};
    let u=vec![s;v.u.len()];let vv=vec![s;v.v.len()];let w=vec![s;v.w.len()];
    for n in 0..100 {
        let time=SimTime(n*1000);let bg=BackgroundFaces3{domain:v.domain,time,density:1025.,gravity:9.81,u:&u,v:&vv,w:&w};
        let r=v.step_perturbation_mobile(time,1000,4000,&bg,Sponge3{width_x:0.5,width_y:0.5,rate_per_s:1.},&Jobs).unwrap();
        assert_eq!(r.iterations,0);assert!(v.eta.iter().all(|x|x.to_bits()==2f32.to_bits()));
        assert!(v.u.iter().chain(&v.v).chain(&v.w).all(|x|*x==0.));
    }
}

#[test]
fn coupled_refusal_is_atomic_and_disarms_geometry_s297() {
    use crate::{background::BackgroundSample,SimTime};
    let (mut v,_) = volume(8,6,12,0.25,9.81);
    let eta:Vec<_>=(0..48).map(|c|2.+0.05*(c as f32).sin()).collect();v.set_free_surface(&eta,2.).unwrap();
    let u=vec![BackgroundSample::default();v.u.len()];let vv=vec![BackgroundSample::default();v.v.len()];let mut w=vec![BackgroundSample::default();v.w.len()];
    let before:Vec<_>=[&v.u,&v.v,&v.w,&v.p,&v.eta,&v.eta_roundoff].iter().map(|f|f.iter().map(|x|x.to_bits()).collect::<Vec<_>>()).collect();
    for mode in 0..4 {
        w.fill(BackgroundSample::default());
        if mode==1 {w[0].eta=0.1;} if mode==2 {w[0].u[1]=f32::NAN;}
        if mode==3 {for s in &mut w {s.eta=20.;}}
        let bg=BackgroundFaces3{domain:v.domain,time:SimTime(0),density:1025.,gravity:9.81,u:&u,v:&vv,w:&w};
        let e=v.step_perturbation_mobile(SimTime(0),1000,0,&bg,Sponge3::default(),&Jobs).unwrap_err();
        assert_eq!(e,[Error::Convergence,Error::BackgroundContext,Error::NotFinite,Error::Domain][mode]);
        let after:Vec<_>=[&v.u,&v.v,&v.w,&v.p,&v.eta,&v.eta_roundoff].iter().map(|f|f.iter().map(|x|x.to_bits()).collect::<Vec<_>>()).collect();
        assert_eq!(before,after);assert!(!v.surface_coupled && !v.homogeneous_ghost);
    }
    v.step_surface_mobile(1000,4000,&Jobs).unwrap();
}

use crate::background::BackgroundSample;
#[path = "../examples/support/standing_background.rs"]
#[allow(dead_code)]
mod standing297;
fn standing_samples297(v:&Volume3,time:f64,axis:usize)->(Vec<BackgroundSample>,Vec<BackgroundSample>,Vec<BackgroundSample>) {
    let d=v.domain;let wave=standing297::StandingWave{a:0.05,k:std::f64::consts::PI/2.,h:2.,g:9.81,rho:1025.};
    let mut fields=[vec![BackgroundSample::default();v.u.len()],vec![BackgroundSample::default();v.v.len()],vec![BackgroundSample::default();v.w.len()]];
    for component in 0..3 {
        let end=[d.nx+usize::from(component==0),d.ny+usize::from(component==1),d.nz+usize::from(component==2)];
        for k in 0..end[2] {for j in 0..end[1] {for i in 0..end[0] {
            let p=[i,j,k];let x=(p[axis] as f64+if component==axis {0.} else {0.5})*d.dx as f64;
            let z=(k as f64+if component==2 {0.} else {0.5})*d.dx as f64-2.;
            let mut s=wave.sample(x,z,time);
            if axis==1 {
                s.u.swap(0,1);s.du_dt.swap(0,1);s.grad_eta.swap(0,1);s.grad_p_dyn.swap(0,1);s.laplacian_u.swap(0,1);
                s.grad_u.swap(0,1);for row in &mut s.grad_u {row.swap(0,1);}
            }
            fields[component][(k*end[1]+j)*end[0]+i]=s;
        }}}
    }
    let [u,v,w]=fields;(u,v,w)
}
#[test]
fn coupled_transverse_invariance_and_rotation_s297() {
    use crate::SimTime;
    let (mut a,_) = volume(16,4,20,0.125,9.81);let (mut b,_) = volume(4,16,20,0.125,9.81);
    // S443 : le pas de S297, dont δ croît sous B seul — en mode relatif (le défaut), il resterait nul.
    a.set_relative_background(0).unwrap();b.set_relative_background(0).unwrap();
    a.set_free_surface(&[2.;64],2.).unwrap();b.set_free_surface(&[2.;64],2.).unwrap();
    let (mut transverse,mut rotated)=(0f32,0f32);
    for n in 0..400 {
        let time=SimTime(n*1000);
        let (u,v,w)=standing_samples297(&a,n as f64*0.001,0);
        let bg=BackgroundFaces3{domain:a.domain,time,density:1025.,gravity:9.81,u:&u,v:&v,w:&w};
        a.step_perturbation_mobile(time,1000,4000,&bg,Sponge3::default(),&Jobs).unwrap();
        let (u,v,w)=standing_samples297(&b,n as f64*0.001,1);
        let bg=BackgroundFaces3{domain:b.domain,time,density:1025.,gravity:9.81,u:&u,v:&v,w:&w};
        b.step_perturbation_mobile(time,1000,4000,&bg,Sponge3::default(),&Jobs).unwrap();
        for j in 0..4 {for i in 0..16 {
            transverse=transverse.max((a.eta[j*16+i]-a.eta[i]).abs());
            rotated=rotated.max((a.eta[j*16+i]-b.eta[i*4+j]).abs());
        }}
    }
    println!("S297 transverse={transverse:e} m rotation={rotated:e} m");
    assert!(transverse<2e-6 && rotated<2e-6);
    assert!(a.eta.iter().any(|h|(*h-2.).abs()>1e-4));
}

#[test]
fn real_background_grid_matches_mac_points_and_preserves_publication_s298() {
    use crate::{background::{Background,SeaState,DifferentialError},SimTime,WorldPos};
    let sample_bits=|s:BackgroundSample| core::iter::once(s.eta).chain(core::iter::once(s.p_dyn))
        .chain(s.grad_eta).chain(s.u).chain(s.du_dt).chain(s.grad_p_dyn).chain(s.laplacian_u)
        .chain(s.grad_u.into_iter().flatten()).map(f32::to_bits).collect::<Vec<_>>();
    let d=Domain3{nx:5,ny:3,nz:9,dx:0.25};
    let origin=[-1.,0.75,-1.5];
    let mut arena=Arena{stats:AllocStats::default(),sealed:false};
    let mut host=HostServices{alloc:&mut arena,jobs:&Jobs,sink:&Jobs};
    let sea=SeaState{hs:0.2,tp:2.,theta_turns:0.13,components:8,graine:298};
    let b=Background::configure(&mut host,sea,WorldPos::from_units(0,0,0)).unwrap();
    let bad=Background::configure(&mut host,SeaState{hs:f32::NAN,..sea},WorldPos::from_units(0,0,0)).unwrap();
    let before=host.alloc.stats().persistent_bytes;
    let mut grid=BackgroundGrid3::configure(&mut host,d,origin,1025.).unwrap();
    let faces=(d.nx+1)*d.ny*d.nz+d.nx*(d.ny+1)*d.nz+d.nx*d.ny*(d.nz+1);
    assert_eq!(host.alloc.stats().persistent_bytes-before,(2*faces+(d.nx+1)*(d.nz+1))*core::mem::size_of::<BackgroundSample>()+(d.nx+1)*core::mem::size_of::<[f32;2]>()+(2*(d.nx+d.ny+d.nz)+3)*core::mem::size_of::<f32>());
    assert!(grid.view().is_none());
    host.alloc.seal();
    for t in [0,987_654,9_876_543_210] {
        grid.sample(&b,SimTime(t)).unwrap();
        let bg=grid.view().unwrap();assert_eq!(bg.time,SimTime(t));assert_eq!(bg.gravity,b.gravity());
        for (axis,field) in [bg.u,bg.v,bg.w].into_iter().enumerate() {
            let dims=[d.nx+usize::from(axis==0),d.ny+usize::from(axis==1),d.nz+usize::from(axis==2)];
            for k in 0..dims[2] {for j in 0..dims[1] {for i in 0..dims[0] {
                let ijk=[i,j,k];let p=core::array::from_fn(|a|origin[a]+(ijk[a] as f32+if a==axis {0.} else {0.5})*d.dx);
                assert_eq!(sample_bits(field[(k*dims[1]+j)*dims[0]+i]),sample_bits(b.differential_local_extended(p,SimTime(t),1025.).unwrap()));
            }}}
        }
        let old=[bg.u.to_vec(),bg.v.to_vec(),bg.w.to_vec()];
        assert_eq!(grid.sample(&bad,SimTime(t+1)),Err(DifferentialError::Background));
        let bg=grid.view().unwrap();assert_eq!(bg.time,SimTime(t));
        assert_eq!([bg.u,bg.v,bg.w],[old[0].as_slice(),old[1].as_slice(),old[2].as_slice()]);
    }
    let calls=host.alloc.stats().persistent_calls;
    for (domain,point,rho) in [(Domain3{nx:0,..d},origin,1025.),(d,[4095.5,0.,0.],1025.),
        (d,[0.,0.,f32::NAN],1025.),(d,origin,0.),(Domain3{nx:usize::MAX,..d},origin,1025.)] {
        assert!(BackgroundGrid3::configure(&mut host,domain,point,rho).is_err());
    }
    assert_eq!(host.alloc.stats().persistent_calls,calls);
}

// ─────────────────────────────────────────────────────────────────────────────────────────────
// S310 — LE BILAN DE MASSE (lot 1 d'ADR-178 D7, angle mort A302).
//
// Le contrat est dans `delta3d_balance.rs` : les termes intérieurs du transport télescopent, donc
// la variation de volume vaut **exactement** la somme des faces de bord. Ces essais éprouvent les
// trois affirmations sur lesquelles repose cette exactitude, et une quatrième sur l'atomicité.

/// Montage commun : une bosse de surface, un fond uniforme qui traverse les quatre bords.
/// La surface **non uniforme** est ce qui rend `band_in` non nul — les deux faces opposées
/// n'intègrent pas la même hauteur.
fn bilan_montage_s310(bump: bool) -> (Volume3, Arena, Vec<BackgroundSample>) {
    let (mut v, arena) = volume(8, 6, 12, 0.25, 9.81);
    let eta: Vec<f32> = (0..48)
        .map(|c| if bump { 2. + 0.02 * ((c % 8) as f32 - 3.5) } else { 2. })
        .collect();
    v.set_free_surface(&eta, 2.).unwrap();
    let sample = BackgroundSample {
        eta: 0.07,
        u: [0.6, -0.4, 0.],
        p_dyn: 1025. * 9.81 * 0.07,
        ..BackgroundSample::default()
    };
    (v, arena, vec![sample; 1])
}

#[test]
fn bilan_de_masse_se_ferme_au_plancher_s310() {
    use crate::SimTime;
    let (mut v, _a, proto) = bilan_montage_s310(true);
    let (u, vv, w) = (vec![proto[0]; v.u.len()], vec![proto[0]; v.v.len()], vec![proto[0]; v.w.len()]);
    let (mut worst, mut scale) = (0f64, 0f64);
    for n in 0..200 {
        let time = SimTime(n * 1000);
        let bg = BackgroundFaces3 { domain: v.domain, time, density: 1025., gravity: 9.81,
            u: &u, v: &vv, w: &w };
        v.step_perturbation_mobile(time, 1000, 4000, &bg, Sponge3::default(), &Jobs).unwrap();
        let b = v.balance();
        // La perturbation ne traverse **pas** le bord : la garde `a > 0 && a < n` du transport.
        // Exactement zéro, pas « petit » — c'est une construction, pas une approximation.
        assert_eq!(b.perturbation_in, 0.);
        assert_eq!(b.sponge_out, 0.);
        worst = worst.max(b.residual.abs());
        scale = scale.max(b.band_in.abs()).max(b.delta.abs());
    }
    eprintln!("S310 bilan ferme : residu_max={worst:e} echelle={scale:e} rapport={:e}", worst / scale);
    assert!(scale > 1e-6, "le montage doit faire circuler de la masse, sinon il ne prouve rien");
    // Mesuré S310 : 9,85e-8, soit ≈ 1,6 ulp de f32 par pas. Seuil à dix fois la mesure.
    assert!(worst / scale < 1e-6, "residu {worst:e} hors plancher pour une echelle {scale:e}");
}

#[test]
fn cuve_fermee_garde_son_volume_s310() {
    use crate::SimTime;
    let (mut v, _a, _p) = bilan_montage_s310(true);
    let zero = BackgroundSample::default();
    let (u, vv, w) = (vec![zero; v.u.len()], vec![zero; v.v.len()], vec![zero; v.w.len()]);
    let depart = v.perturbation_volume();
    let mut worst = 0f64;
    for n in 0..200 {
        let time = SimTime(n * 1000);
        let bg = BackgroundFaces3 { domain: v.domain, time, density: 1025., gravity: 9.81,
            u: &u, v: &vv, w: &w };
        v.step_perturbation_mobile(time, 1000, 4000, &bg, Sponge3::default(), &Jobs).unwrap();
        let b = v.balance();
        // Aucun fond, aucune éponge : le domaine est fermé. Rien ne doit entrer ni sortir.
        assert_eq!(b.band_in, 0.);
        assert_eq!(b.perturbation_in, 0.);
        assert_eq!(b.sponge_out, 0.);
        worst = worst.max((b.volume - depart).abs());
    }
    let colonne = v.domain.dx as f64 * v.domain.dx as f64 * 48.;
    eprintln!("S310 cuve fermee : derive_volume_max={worst:e} m3, soit {:e} m de hauteur moyenne",
              worst / colonne);
    // Mesuré S310 : 1,13e-11 m de hauteur moyenne sur 200 pas. Seuil à cent fois la mesure.
    assert!(worst / colonne < 1e-9, "derive {worst:e} m3 au-dela du plancher");
}

#[test]
fn l_eponge_est_un_puits_et_il_se_compte_s310() {
    use crate::SimTime;
    let (mut v, _a, _p) = bilan_montage_s310(true);
    let zero = BackgroundSample::default();
    let (u, vv, w) = (vec![zero; v.u.len()], vec![zero; v.v.len()], vec![zero; v.w.len()]);
    let depart = v.perturbation_volume();
    let sponge = Sponge3 { width_x: 0.5, width_y: 0.5, rate_per_s: 1. };
    let (mut retire, mut worst) = (0f64, 0f64);
    for n in 0..200 {
        let time = SimTime(n * 1000);
        let bg = BackgroundFaces3 { domain: v.domain, time, density: 1025., gravity: 9.81,
            u: &u, v: &vv, w: &w };
        v.step_perturbation_mobile(time, 1000, 4000, &bg, sponge, &Jobs).unwrap();
        let b = v.balance();
        assert_eq!(b.band_in, 0.);
        retire += b.sponge_out;
        worst = worst.max(b.residual.abs());
    }
    let perdu = depart - v.perturbation_volume();
    eprintln!("S310 eponge : retire_cumule={retire:e} m3, volume_perdu={perdu:e} m3, \
               ecart={:e}, residu_max={worst:e}", (perdu - retire).abs());
    // Le domaine est fermé : **tout** ce qui manque est passé par l'éponge, et rien d'autre.
    assert!(retire.abs() > 1e-9, "l'eponge doit avoir retire quelque chose");
    // Mesuré S310 : 9,8e-7 relatif. Seuil à dix fois la mesure — le défaut que cet essai a
    // effectivement attrapé (compter `increment` au lieu de la hauteur compensée) valait
    // 8,6e-3, soit quatre ordres de grandeur au-dessus.
    assert!((perdu - retire).abs() / retire.abs() < 1e-5,
            "le volume perdu {perdu:e} ne s'explique pas par l'eponge {retire:e}");
}

#[test]
fn un_refus_ne_publie_pas_de_bilan_s310() {
    use crate::SimTime;
    let (mut v, _a, proto) = bilan_montage_s310(true);
    let (u, vv, mut w) = (vec![proto[0]; v.u.len()], vec![proto[0]; v.v.len()],
                          vec![proto[0]; v.w.len()]);
    let time = SimTime(0);
    {
        let bg = BackgroundFaces3 { domain: v.domain, time, density: 1025., gravity: 9.81,
            u: &u, v: &vv, w: &w };
        v.step_perturbation_mobile(time, 1000, 4000, &bg, Sponge3::default(), &Jobs).unwrap();
    }
    let bon = v.balance();
    assert_ne!(bon, Balance3::default());
    // Un fond hors du domaine fait échouer le pas **après** le transport : c'est exactement le
    // cas où un bilan publié trop tôt décrirait un pas qui n'a pas eu lieu.
    for s in &mut w { s.eta = 20.; }
    let bg = BackgroundFaces3 { domain: v.domain, time, density: 1025., gravity: 9.81,
        u: &u, v: &vv, w: &w };
    assert!(v.step_perturbation_mobile(time, 1000, 4000, &bg, Sponge3::default(), &Jobs).is_err());
    assert_eq!(v.balance(), bon, "le bilan doit survivre au refus, comme l'etat qu'il decrit");
}

#[test]
fn energie_et_quantite_de_mouvement_disent_ce_qu_elles_valent_s310() {
    let (mut v, _a) = volume(8, 6, 12, 0.25, 9.81);
    // Au repos, tout est nul : pas d'écart de surface, pas de vitesse.
    v.set_free_surface(&[2.; 48], 2.).unwrap();
    let e = v.perturbation_energy();
    assert_eq!((e.kinetic, e.potential, e.total), (0., 0., 0.));
    assert_eq!(v.perturbation_momentum(), [0.; 3]);

    // L'énergie potentielle est quadratique en l'écart : doubler l'amplitude la quadruple.
    let bosse: Vec<f32> = (0..48).map(|c| 2. + 0.01 * ((c % 8) as f32 - 3.5)).collect();
    v.set_free_surface(&bosse, 2.).unwrap();
    let simple = v.perturbation_energy().potential;
    let double: Vec<f32> = bosse.iter().map(|h| 2. + 2. * (h - 2.)).collect();
    v.set_free_surface(&double, 2.).unwrap();
    let quadruple = v.perturbation_energy().potential;
    assert!(simple > 0.);
    assert!((quadruple / simple - 4.).abs() < 1e-6, "rapport {}", quadruple / simple);

    // Une vitesse uniforme donne une quantité de mouvement `ρ·V·u` sur la part mouillée.
    v.set_free_surface(&[2.; 48], 2.).unwrap();
    let u = vec![0.5f32; v.u.len()];
    v.set_velocity(&u, &vec![0.; v.v.len()], &vec![0.; v.w.len()]).unwrap();
    let m = v.perturbation_momentum();
    // 48 colonnes de 2 m d'eau sur des mailles de 25 cm : 8 mailles pleines par colonne. Mais les
    // **murs** portent une vitesse nulle (`close_walls`), et la vitesse au centre d'une maille est
    // la moyenne de ses deux faces : les colonnes `i = 0` et `i = nx−1` n'en portent donc que la
    // moitié. La somme des vitesses centrées vaut `ny·(nx−1)·u`, pas `ny·nx·u` — 21 et non 24.
    let attendu = 1025. * (6. * 7. * 0.5) * 8. * 0.25f64.powi(3);
    assert!((m[0] / attendu - 1.).abs() < 1e-6, "{m:?} contre {attendu}");
    assert_eq!((m[1], m[2]), (0., 0.));
}

// ─────────────────────────────── S324 : le fond coupé (lot 3, critère 2) ───────────────────────────────

fn volume_bottom(nx: usize, ny: usize, nz: usize, dx: f32, fond: &[f32]) -> (Volume3, Arena) {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let v = Volume3::configure_with_bottom(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx, ny, nz, dx }, 1025., 9.81, fond).unwrap();
    (v, arena)
}

/// Les trois fonds de S232, en coordonnée physique.
fn fond_s232(forme: usize, x: f32) -> f32 {
    let d = (x - 3.0) / 1.2;
    match forme {
        0 => 0.5,
        1 => 0.4 + 0.6 * (-(d * d)).exp(),
        _ => 0.4 + 0.6 * (-(d * d)).exp() + 0.125 * (1.0 + (2.0 * (x - 6.0)).tanh()),
    }
}

/// Une bosse vraiment tridimensionnelle, centrée hors des axes du domaine.
fn bosse(x: f32, y: f32) -> f32 {
    let (a, b) = ((x - 3.0) / 1.2, (y - 1.3) / 0.9);
    0.4 + 0.6 * (-(a * a + b * b)).exp()
}

/// La découpe se compte : les tampons du fond plat, puis ceux de la découpe, en deux appels ; un fond
/// hors de `[0, z₀[` ou de mauvaise taille est refusé ; le fond plat ne publie aucune découpe.
#[test]
fn a_cut_bottom_counts_its_buffers_and_refuses_what_it_cannot_hold_s324() {
    let (plat, a0) = volume(6, 4, 5, 0.5, 9.81);
    assert!(plat.fluid_fraction().is_none() && plat.apertures().is_none());
    let fond = vec![0.3f32; 24];
    let (v, a1) = volume_bottom(6, 4, 5, 0.5, &fond);
    let (nx, ny, nz) = (6usize, 4usize, 5usize);
    // S328 : plus le plancher de chaque colonne, garde du pas mobile.
    let extra = ((nx + 1) * ny * nz + nx * (ny + 1) * nz + nx * ny * (nz + 1) + nx * ny * nz + nx * ny) * 4;
    assert_eq!(a1.stats.persistent_bytes, a0.stats.persistent_bytes + extra);
    assert_eq!(a1.stats.persistent_calls, 2);
    assert!(v.fluid_fraction().is_some());
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let d = Domain3 { nx, ny, nz, dx: 0.5 };
    assert_eq!(Volume3::configure_with_bottom(&mut host, d, 1025., 9.81, &fond[..23]).err(), Some(Error::Shape));
    assert_eq!(Volume3::configure_with_bottom(&mut host, d, 1025., 9.81, &vec![-0.1; 24]).err(), Some(Error::NotFinite));
    assert_eq!(Volume3::configure_with_bottom(&mut host, d, 1025., 9.81, &vec![2.5; 24]).err(), Some(Error::NotFinite));
}

/// **Critère 2a.** À `ny = 1`, sur les trois fonds de S232, la trajectoire linéaire de la 3D est celle
/// de la 2D **au bit** — surface, vitesses et nombre d'itérations, pas après pas.
#[test]
fn ny_1_on_the_s232_bottoms_reproduces_the_2d_linear_trajectory_s324() {
    use crate::delta_projection::{Domain, Volume};
    struct Still;
    impl crate::host::MonotonicClock for Still { fn now_ns(&self) -> u64 { 0 } }
    let (n, us) = (32usize, 2000u64);
    let dx = 8. / n as f32;
    for forme in 0..3 {
        let fond: Vec<f32> = (0..n).map(|i| fond_s232(forme, (i as f32 + 0.5) * dx)).collect();
        let (mut v3, _) = volume_bottom(n, 1, n / 2, dx, &fond);
        let mut arena = Arena { stats: AllocStats::default(), sealed: false };
        let mut v2 = Volume::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
            Domain { nx: n, nz: n / 2, dx }, 1025., 9.81, &fond).unwrap();
        let eta = s233_surface(n);
        v3.set_surface(&eta).unwrap();
        v2.set_surface(&eta).unwrap();
        for step in 0..200 {
            let r3 = v3.step_surface_linear(us, 4000, &Jobs).unwrap();
            let r2 = v2.step_surface_linear(us, 4000, 1_000_000, &Jobs, &Still).unwrap().report.unwrap();
            assert_eq!(r3.iterations, r2.iterations, "fond {forme}, pas {step}");
            for (a, b) in v3.surface().iter().zip(v2.surface()) {
                assert_eq!(a.to_bits(), b.to_bits(), "fond {forme}, pas {step} : surface");
            }
            for (a, b) in v3.velocity_u().iter().zip(v2.velocity_u()) {
                assert_eq!(a.to_bits(), b.to_bits(), "fond {forme}, pas {step} : u");
            }
            for (a, b) in v3.velocity_w().iter().zip(v2.velocity_w()) {
                assert_eq!(a.to_bits(), b.to_bits(), "fond {forme}, pas {step} : w");
            }
        }
    }
}

/// **Critère 2b.** Un lac au repos sur un fond coupé vraiment 3D reste **exactement** au repos : cent
/// pas, vitesses et surface au bit.
#[test]
fn a_lake_at_rest_on_a_cut_bottom_stays_exactly_at_rest_s324() {
    let (nx, ny, nz, dx) = (24usize, 12usize, 10usize, 0.25f32);
    let fond: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| bosse((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)))
        .collect();
    let (mut v, _) = volume_bottom(nx, ny, nz, dx, &fond);
    let repos = v.surface().to_vec();
    for _ in 0..100 {
        let r = v.step_surface_linear(2000, 2000, &Jobs).unwrap();
        assert!(!r.degraded);
    }
    assert!(v.velocity_u().iter().chain(v.velocity_v()).chain(v.velocity_w()).all(|x| x.to_bits() == 0));
    assert!(v.surface().iter().zip(&repos).all(|(a, b)| a.to_bits() == b.to_bits()));
}

/// **Critère 2c.** L'opérateur pondéré reste symétrique et défini positif sur les mailles fluides —
/// ce qui garde le gradient conjugué valide.
#[test]
fn the_cut_operator_is_symmetric_and_positive_s324() {
    let (nx, ny, nz, dx) = (10usize, 7usize, 6usize, 0.25f32);
    let fond: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| 0.2 + 0.5 * bosse((i as f32 + 0.5) * dx * 2.4, (j as f32 + 0.5) * dx * 2.4)))
        .collect();
    let (v, _) = volume_bottom(nx, ny, nz, dx, &fond);
    let frac = v.fluid_fraction().unwrap().to_vec();
    let n = v.domain.cells();
    let masque = |x: Vec<f32>| x.iter().zip(&frac).map(|(a, f)| if *f > 0. { *a } else { 0. }).collect::<Vec<f32>>();
    let (x, y) = (masque(noise(n, 17)), masque(noise(n, 23)));
    let (mut ax, mut ay) = (vec![0.; n], vec![0.; n]);
    v.apply(&x, &mut ax);
    v.apply(&y, &mut ay);
    let dot = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(p, q)| *p as f64 * *q as f64).sum::<f64>();
    let (xay, yax, xax) = (dot(&x, &ay), dot(&y, &ax), dot(&x, &ax));
    let scale = dot(&x, &x).sqrt() * dot(&ay, &ay).sqrt();
    assert!((xay - yax).abs() <= 1e-5 * scale, "{xay} contre {yax}");
    assert!(xax > 0.);
    assert!(frac.iter().any(|f| *f > 0. && *f < 1.), "le fond ne coupe aucune maille");
}

/// **Critère 2d.** Sur un fond vraiment 3D, un pas depuis une surface bosselée laisse un champ dont la
/// divergence **ouverte** tient la tolérance physique ; les faces fermées par le fond n'ont aucune
/// vitesse, et l'écoulement a bien une composante transverse.
#[test]
fn a_step_on_a_cut_bottom_leaves_an_open_divergence_free_field_s324() {
    let (nx, ny, nz, dx) = (24usize, 12usize, 10usize, 0.25f32);
    let fond: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| bosse((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)))
        .collect();
    let (mut v, _) = volume_bottom(nx, ny, nz, dx, &fond);
    let z0 = v.domain.z0();
    let eta: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| z0 + 0.01 * ((i as f32 + 0.5) * dx * 0.8).sin() * ((j as f32 + 0.5) * dx * 1.1).cos()))
        .collect();
    v.set_surface(&eta).unwrap();
    let r = v.step_surface_linear(2000, 4000, &Jobs).unwrap();
    assert!(!r.degraded && r.divergence <= PROJECTION_DIVERGENCE_TOLERANCE as f64, "divergence {}", r.divergence);
    let (ou, ov, ow) = v.apertures().unwrap();
    let fermees = |vit: &[f32], o: &[f32]| vit.iter().zip(o).all(|(x, a)| *a != 0. || x.to_bits() == 0);
    assert!(fermees(v.velocity_u(), ou) && fermees(v.velocity_v(), ov) && fermees(v.velocity_w(), ow));
    assert!(v.velocity_v().iter().any(|x| *x != 0.), "aucun écoulement transverse");
}

/// Le pas **couplé** ne porte pas encore la découpe : il la refuse au lieu de l'ignorer. *S324 le
/// vérifiait du pas mobile, qui la porte depuis S328.*
#[test]
fn the_coupled_step_refuses_a_cut_bottom_s324() {
    use crate::{background::BackgroundSample, SimTime};
    let (mut v, _) = volume_bottom(6, 4, 5, 0.5, &vec![0.3f32; 24]);
    let (u, vv, w) = (vec![BackgroundSample::default(); v.u.len()], vec![BackgroundSample::default(); v.v.len()],
        vec![BackgroundSample::default(); v.w.len()]);
    let bg = BackgroundFaces3 { domain: v.domain, time: SimTime(0), density: 1025., gravity: 9.81, u: &u, v: &vv, w: &w };
    assert_eq!(v.step_perturbation_mobile(SimTime(0), 2000, 100, &bg, Sponge3::default(), &Jobs).err(), Some(Error::Domain));
}

/// **S326, critère 2.** Jacobi sur le chemin coupé : la même solution à la tolérance près, en bien moins
/// d'itérations, sur la bosse de S324 à la maille moyenne de son banc ; à `ny = 1`, rien ne change —
/// les essais de S324 le gardent.
#[test]
fn jacobi_on_the_cut_path_keeps_the_solution_and_cuts_the_iterations_s326() {
    let (nx, ny, nz, dx) = (64usize, 32usize, 32usize, 0.125f32);
    let fond: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| bosse((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)))
        .collect();
    let mut sorties = Vec::new();
    for on in [false, true] {
        let (mut v, _) = volume_bottom(nx, ny, nz, dx, &fond);
        v.set_precondition_cut(on);
        let z0 = v.domain.z0();
        let eta: Vec<f32> = (0..ny)
            .flat_map(|_| (0..nx).map(move |i| z0 + 0.01 * (core::f32::consts::TAU * (i as f32 + 0.5) * dx / 8.).sin()))
            .collect();
        v.set_surface(&eta).unwrap();
        let r = v.step_surface_linear(2000, 20_000, &Jobs).unwrap();
        let (ou, _, _) = v.apertures().unwrap();
        let i = nx / 2;
        let mut q = 0f64;
        for k in 0..nz {
            for j in 0..ny {
                let f = (k * ny + j) * (nx + 1) + i;
                q += ou[f] as f64 * v.velocity_u()[f] as f64 * (dx * dx) as f64;
            }
        }
        sorties.push((r.iterations, q, r.divergence));
    }
    let ((it_sans, q_sans, _), (it_avec, q_avec, d_avec)) = (sorties[0], sorties[1]);
    println!("S326 : sans Jacobi {it_sans} itérations, avec {it_avec} ; débit {q_sans:e} contre {q_avec:e}");
    assert!(it_avec * 2 < it_sans, "{it_avec} contre {it_sans}");
    assert!(((q_avec - q_sans) / q_sans).abs() < 1e-5, "débit {q_avec} contre {q_sans}");
    assert!(d_avec <= PROJECTION_DIVERGENCE_TOLERANCE as f64);
}

// ─────────────────────────────── S328 : le mode mobile sur fond coupé (lot 3) ───────────────────────────────

/// Une surface mobile de S296 posée à `repos + a·cos(πx/L)` sur chaque rangée.
fn surface_mobile(nx: usize, ny: usize, repos: f32, a: f32) -> Vec<f32> {
    (0..ny)
        .flat_map(|_| (0..nx).map(move |i| (repos as f64 + a as f64 * (std::f64::consts::PI * (i as f64 + 0.5) / nx as f64).cos()) as f32))
        .collect()
}

/// **S328, critère 2.** À `ny = 1`, sur les trois fonds de S232, la trajectoire **mobile** de la 3D est
/// celle de la 2D (S237) **au bit** — surface, pression, vitesses et itérations, pas après pas.
#[test]
fn ny_1_on_the_s232_bottoms_reproduces_the_2d_mobile_trajectory_s328() {
    use crate::delta_projection::{Domain, Volume, MOBILE_MULTIGRID_OFF};
    struct Still;
    impl crate::host::MonotonicClock for Still { fn now_ns(&self) -> u64 { 0 } }
    struct Reset;
    impl Drop for Reset { fn drop(&mut self) { MOBILE_MULTIGRID_OFF.with(|c| c.set(false)); } }
    MOBILE_MULTIGRID_OFF.with(|c| c.set(true));
    let _reset = Reset;
    let (n, nz) = (32usize, 16usize);
    let dx = 8. / n as f32;
    for forme in 0..3 {
        let fond: Vec<f32> = (0..n).map(|i| fond_s232(forme, (i as f32 + 0.5) * dx)).collect();
        let (mut v3, _) = volume_bottom(n, 1, nz, dx, &fond);
        let mut arena = Arena { stats: AllocStats::default(), sealed: false };
        let mut v2 = Volume::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
            Domain { nx: n, nz, dx }, 1025., 9.81, &fond).unwrap();
        let eta = surface_mobile(n, 1, 2.5, 0.1);
        v3.set_free_surface(&eta, 2.5).unwrap();
        v2.set_free_surface(&eta, 2.5).unwrap();
        for step in 0..200 {
            let r3 = v3.step_surface_mobile(1000, 4000, &Jobs).unwrap();
            let r2 = v2.step_surface_mobile(1000, 4000, 1_000_000, &Jobs, &Still).unwrap().report.unwrap();
            assert_eq!(r3.iterations, r2.iterations, "fond {forme}, pas {step}");
            for (a, b) in v3.surface().iter().zip(v2.surface()).chain(v3.pressure().iter().zip(v2.pressure()))
                .chain(v3.velocity_u().iter().zip(v2.velocity_u())).chain(v3.velocity_w().iter().zip(v2.velocity_w())) {
                assert_eq!(a.to_bits(), b.to_bits(), "fond {forme}, pas {step}");
            }
        }
        assert!(v3.velocity_u().iter().any(|x| *x != 0.), "fond {forme} : rien n'a bougé");
    }
}

/// **S328, critère 3.** Un lac au repos sur la bosse 3D, en mode mobile, reste **exactement** au repos :
/// cent pas, vitesses et surface au bit.
#[test]
fn a_lake_at_rest_on_a_cut_bottom_stays_at_rest_in_mobile_mode_s328() {
    let (nx, ny, nz, dx) = (24usize, 12usize, 10usize, 0.25f32);
    let fond: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| bosse((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)))
        .collect();
    let (mut v, _) = volume_bottom(nx, ny, nz, dx, &fond);
    v.set_free_surface(&vec![2.; nx * ny], 2.).unwrap();
    for pas in 0..100 {
        v.step_surface_mobile(2000, 4000, &Jobs).unwrap();
        assert!(v.velocity_u().iter().chain(v.velocity_v()).chain(v.velocity_w()).all(|x| x.to_bits() == 0), "pas {pas}");
        assert!(v.surface().iter().all(|h| h.to_bits() == 2f32.to_bits()), "pas {pas}");
    }
}

/// **S328, critère 4.** Un fond sans `y`, à `ny = 4` : les quatre tranches sont identiques entre elles au
/// bit, et restent à 10⁻⁵ près en relatif de la tranche `ny = 1` après cent pas mobiles.
#[test]
fn a_bottom_without_y_keeps_the_mobile_slices_identical_s328() {
    let (n, nz) = (32usize, 16usize);
    let dx = 8. / n as f32;
    let (repos, a) = (2.5f32, 0.1f32);
    let mut surfaces = Vec::new();
    for ny in [1usize, 4] {
        let fond: Vec<f32> = (0..ny).flat_map(|_| (0..n).map(move |i| fond_s232(1, (i as f32 + 0.5) * dx))).collect();
        let (mut v, _) = volume_bottom(n, ny, nz, dx, &fond);
        v.set_free_surface(&surface_mobile(n, ny, repos, a), repos).unwrap();
        for _ in 0..100 {
            v.step_surface_mobile(1000, 4000, &Jobs).unwrap();
        }
        let s = v.surface().to_vec();
        for j in 1..ny {
            for i in 0..n {
                assert_eq!(s[j * n + i].to_bits(), s[i].to_bits(), "tranche {j}, colonne {i}");
            }
        }
        // L'écoulement transverse n'est nul qu'à l'arrondi près : la diagonale de Jacobi des rangées de
        // bord n'est pas celle des rangées intérieures, et le fond plat de S296 en fait autant
        // (5·10⁻⁹ m/s pour 0,037 m/s de `u`, S328 P3).
        let vmax = v.velocity_v().iter().fold(0f32, |m, x| m.max(x.abs()));
        let umax = v.velocity_u().iter().fold(0f32, |m, x| m.max(x.abs()));
        assert!(vmax <= 1e-6 * umax, "écoulement transverse {vmax} pour {umax}");
        surfaces.push(s);
    }
    let pire = (0..n).fold(0f32, |m, i| m.max(((surfaces[1][i] - repos) - (surfaces[0][i] - repos)).abs()));
    println!("S328 : écart ny = 4 contre ny = 1 après cent pas, {:e} de l'amplitude", pire / a);
    assert!(pire / a <= 1e-5, "{pire}");
}

/// **S328, critère 6.** Une surface à moins de deux mailles du plus haut coin du fond de sa colonne est
/// refusée (`Domain`), et l'état publié est restauré au bit.
#[test]
fn a_mobile_surface_too_close_to_the_cut_bottom_is_refused_s328() {
    let (nx, ny, nz, dx) = (24usize, 12usize, 10usize, 0.25f32);
    let fond: Vec<f32> = (0..ny)
        .flat_map(|j| (0..nx).map(move |i| bosse((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx)))
        .collect();
    let (mut v, _) = volume_bottom(nx, ny, nz, dx, &fond);
    // Au sommet de la bosse, le fond approche 1 m : 1,4 m de surface y laisse moins de deux mailles.
    v.set_free_surface(&vec![1.4; nx * ny], 1.4).unwrap();
    let avant: Vec<u32> = v.surface().iter().chain(v.velocity_u()).chain(v.velocity_v()).chain(v.velocity_w())
        .chain(v.pressure()).map(|x| x.to_bits()).collect();
    assert_eq!(v.step_surface_mobile(2000, 4000, &Jobs).err(), Some(Error::Domain));
    let apres: Vec<u32> = v.surface().iter().chain(v.velocity_u()).chain(v.velocity_v()).chain(v.velocity_w())
        .chain(v.pressure()).map(|x| x.to_bits()).collect();
    assert_eq!(avant, apres);
    // Loin du sommet, la même profondeur passe : la garde est bien celle de la colonne.
    let plat = vec![0.3f32; nx * ny];
    let (mut w, _) = volume_bottom(nx, ny, nz, dx, &plat);
    w.set_free_surface(&vec![1.4; nx * ny], 1.4).unwrap();
    assert!(w.step_surface_mobile(2000, 4000, &Jobs).is_ok());
}

// ─────────────────────────────── S329 : un solide quelconque dans les pas ───────────────────────────────

/// Sphère de rayon 0,3 m centrée en (0,6 ; 0,6 ; 0,55) dans un cube de 1,2 m, distance signée aux nœuds.
fn volume_sphere(n: usize) -> (Volume3, Arena) {
    let dx = 1.2 / n as f32;
    let mut noeuds = Vec::with_capacity((n + 1).pow(3));
    for k in 0..=n {
        for j in 0..=n {
            for i in 0..=n {
                let (x, y, z) = (i as f64 * dx as f64, j as f64 * dx as f64, k as f64 * dx as f64);
                noeuds.push((((x - 0.6).powi(2) + (y - 0.6).powi(2) + (z - 0.55).powi(2)).sqrt() - 0.3) as f32);
            }
        }
    }
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let v = Volume3::configure_with_solid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx: n, ny: n, nz: n, dx }, 1025., 9.81, &vec![0.; n * n], &noeuds).unwrap();
    (v, arena)
}

/// **S329, critère 4.** Autour d'une sphère immergée, un lac au repos reste **exactement** au repos,
/// en mode linéaire comme en mode mobile : cent pas, vitesses et surface au bit. Une onde qui passe
/// au-dessus garde sa divergence sous la tolérance, et aucune face fermée ne porte de vitesse.
#[test]
fn an_immersed_sphere_keeps_the_lake_at_rest_and_lets_a_wave_pass_s329() {
    let n = 24usize;
    let (mut lin, _) = volume_sphere(n);
    let z0 = lin.domain.z0();
    lin.set_surface(&vec![z0; n * n]).unwrap();
    for pas in 0..100 {
        lin.step_surface_linear(2000, 4000, &Jobs).unwrap();
        assert!(lin.velocity_u().iter().chain(lin.velocity_v()).chain(lin.velocity_w()).all(|x| x.to_bits() == 0), "linéaire, pas {pas}");
    }
    let (mut mob, _) = volume_sphere(n);
    mob.set_free_surface(&vec![1.05; n * n], 1.05).unwrap();
    for pas in 0..100 {
        mob.step_surface_mobile(2000, 4000, &Jobs).unwrap();
        assert!(mob.velocity_u().iter().chain(mob.velocity_v()).chain(mob.velocity_w()).all(|x| x.to_bits() == 0), "mobile, pas {pas}");
        assert!(mob.surface().iter().all(|h| h.to_bits() == 1.05f32.to_bits()), "mobile, pas {pas}");
    }
    let (mut onde, _) = volume_sphere(n);
    let eta: Vec<f32> = (0..n).flat_map(|_| (0..n).map(move |i| 1.05 + 0.01 * (core::f32::consts::TAU * (i as f32 + 0.5) / n as f32).cos())).collect();
    onde.set_free_surface(&eta, 1.05).unwrap();
    for _ in 0..20 {
        let r = onde.step_surface_mobile(2000, 4000, &Jobs).unwrap();
        assert!(r.divergence_plain <= PROJECTION_DIVERGENCE_TOLERANCE as f64, "{}", r.divergence_plain);
    }
    let (ou, ov, ow) = onde.apertures().unwrap();
    for (a, x) in ou.iter().zip(onde.velocity_u()).chain(ov.iter().zip(onde.velocity_v())).chain(ow.iter().zip(onde.velocity_w())) {
        if *a == 0. {
            assert_eq!(*x, 0., "vitesse sur une face fermée");
        }
    }
    assert!(onde.velocity_u().iter().any(|x| *x != 0.));
}

/// **S329, critère 4.** Le pas mobile garde la surface deux mailles au-dessus du solide : trop près du
/// sommet de la sphère, le pas est refusé et l'état restauré.
#[test]
fn a_mobile_surface_too_close_to_the_sphere_is_refused_s329() {
    let n = 24usize;
    let (mut v, _) = volume_sphere(n);
    v.set_free_surface(&vec![0.95; n * n], 0.95).unwrap();
    let avant: Vec<u32> = v.surface().iter().chain(v.velocity_u()).map(|x| x.to_bits()).collect();
    assert_eq!(v.step_surface_mobile(2000, 4000, &Jobs).err(), Some(Error::Domain));
    let apres: Vec<u32> = v.surface().iter().chain(v.velocity_u()).map(|x| x.to_bits()).collect();
    assert_eq!(avant, apres);
}

// ─────────────────────────────── S330 : la frontière mobile ───────────────────────────────

/// La sphère de S329, centrée en `x = cx`, distance signée aux nœuds d'un cube de 1,2 m.
fn noeuds_sphere(n: usize, cx: f64) -> Vec<f32> {
    // Le pas de `volume_sphere`, en f32 : deux sphères qui en diffèrent d'un arrondi ne se coupent pas pareil.
    let dx = (1.2 / n as f32) as f64;
    let mut noeuds = Vec::with_capacity((n + 1).pow(3));
    for k in 0..=n {
        for j in 0..=n {
            for i in 0..=n {
                let (x, y, z) = (i as f64 * dx, j as f64 * dx, k as f64 * dx);
                noeuds.push((((x - cx).powi(2) + (y - 0.6).powi(2) + (z - 0.55).powi(2)).sqrt() - 0.3) as f32);
            }
        }
    }
    noeuds
}

/// **S330, critère 1.** Reposer le même solide à vitesse nulle ne change rien : cinquante pas linéaires
/// sous une onde, au bit d'un volume qui ne l'a pas reposé.
#[test]
fn setting_the_same_solid_at_rest_changes_nothing_s330() {
    let n = 24usize;
    let (mut a, _) = volume_sphere(n);
    let (mut b, _) = volume_sphere(n);
    let z0 = a.domain.z0();
    let eta: Vec<f32> = (0..n).flat_map(|_| (0..n).map(move |i| z0 + 0.01 * (core::f32::consts::TAU * (i as f32 + 0.5) / n as f32).cos())).collect();
    a.set_surface(&eta).unwrap();
    b.set_surface(&eta).unwrap();
    let s = noeuds_sphere(n, 0.6);
    for pas in 0..50 {
        b.set_solid(&s, [0.; 3]).unwrap();
        let ra = a.step_surface_linear(2000, 4000, &Jobs).unwrap();
        let rb = b.step_surface_linear(2000, 4000, &Jobs).unwrap();
        assert_eq!(ra.iterations, rb.iterations, "pas {pas}");
        for (x, y) in a.surface().iter().zip(b.surface()).chain(a.velocity_u().iter().zip(b.velocity_u()))
            .chain(a.velocity_v().iter().zip(b.velocity_v())).chain(a.velocity_w().iter().zip(b.velocity_w())) {
            assert_eq!(x.to_bits(), y.to_bits(), "pas {pas}");
        }
    }
}

/// **S330, critère 3.** Une sphère qui traverse deux mailles à 1 m/s : des faces naissent et meurent,
/// la divergence reste sous la tolérance, aucune face fermée ne porte de vitesse, rien n'est non fini ;
/// un solide qui toucherait le couvercle est refusé sans rien écrire.
#[test]
fn a_moving_sphere_keeps_the_faces_sound_s330() {
    let n = 24usize;
    let (mut v, _) = volume_sphere(n);
    let z0 = v.domain.z0();
    v.set_surface(&vec![z0; n * n]).unwrap();
    let (u, dt) = (1.0f64, 0.002f64);
    let mut nees = 0usize;
    for pas in 1..=50 {
        let s = noeuds_sphere(n, 0.6 + u * dt * pas as f64);
        let avant: Vec<f32> = v.apertures().unwrap().0.to_vec();
        v.set_solid(&s, [u as f32, 0., 0.]).unwrap();
        nees += v.apertures().unwrap().0.iter().zip(&avant).filter(|(a, b)| **a > 0. && **b == 0.).count();
        let r = v.step_surface_linear(2000, 4000, &Jobs).unwrap();
        assert!(r.divergence <= PROJECTION_DIVERGENCE_TOLERANCE as f64, "pas {pas} : {}", r.divergence);
        let (ou, ov, ow) = v.apertures().unwrap();
        for (a, x) in ou.iter().zip(v.velocity_u()).chain(ov.iter().zip(v.velocity_v())).chain(ow.iter().zip(v.velocity_w())) {
            assert!(x.is_finite());
            if *a == 0. {
                assert_eq!(*x, 0., "pas {pas} : vitesse sur une face fermée");
            }
        }
    }
    assert!(nees > 0, "aucune face n'est née");
    let haut: Vec<f32> = noeuds_sphere(n, 0.6).iter().enumerate().map(|(q, x)| {
        let k = q / ((n + 1) * (n + 1));
        if k >= n - 1 { -1. } else { *x }
    }).collect();
    let bits: Vec<u32> = v.apertures().unwrap().0.iter().map(|x| x.to_bits()).collect();
    assert_eq!(v.set_solid(&haut, [0.; 3]).err(), Some(Error::Domain));
    assert_eq!(bits, v.apertures().unwrap().0.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
}

/// **S330, critère 2.** Le volume : une sphère immergée qui se déplace — `Σ(η − reste − z₀)·dx²` suit
/// la variation du volume discret du solide à 10⁻⁹ m³ près, pas après pas.
#[test]
fn a_moving_sphere_displaces_exactly_its_discrete_volume_s330() {
    let n = 24usize;
    let (mut v, _) = volume_sphere(n);
    let z0 = v.domain.z0();
    v.set_surface(&vec![z0; n * n]).unwrap();
    let dx = v.domain.dx as f64;
    let solide = |v: &Volume3| v.cut.as_ref().unwrap().base.as_ref().unwrap().solid_col.iter().map(|x| *x as f64).sum::<f64>();
    let v0 = solide(&v);
    let mut pire = 0f64;
    for pas in 1..=50 {
        let s = noeuds_sphere(n, 0.6 + 0.002 * pas as f64);
        v.set_solid(&s, [1., 0., 0.]).unwrap();
        v.step_surface_linear(2000, 4000, &Jobs).unwrap();
        let eau: f64 = v.eta.iter().zip(&v.eta_roundoff).map(|(h, r)| (*h as f64 - *r as f64 - z0 as f64) * dx * dx).sum();
        pire = pire.max((eau - (solide(&v) - v0)).abs());
    }
    println!("S330 : écart de volume au pire {pire:e} m³ ; volume discret {v0:e} → {:e}", solide(&v));
    assert!(pire <= 1e-9, "{pire}");
}

// ─────────────────────────────── S332 : le corps dans δ ───────────────────────────────

/// Distance signée d'un pavé droit, centre `c`, demi-côtés `h`, aux nœuds d'un cube de `n` mailles de
/// `dx` — le pas en f32, comme les volumes.
fn noeuds_boite(n: usize, dx: f32, c: [f64; 3], h: [f64; 3]) -> Vec<f32> {
    let dx = dx as f64;
    let mut out = Vec::with_capacity((n + 1).pow(3));
    for k in 0..=n {
        for j in 0..=n {
            for i in 0..=n {
                let p = [i as f64 * dx, j as f64 * dx, k as f64 * dx];
                let q: Vec<f64> = (0..3).map(|a| (p[a] - c[a]).abs() - h[a]).collect();
                let dehors = q.iter().map(|x| x.max(0.).powi(2)).sum::<f64>().sqrt();
                out.push((dehors + q[0].max(q[1]).max(q[2]).min(0.)) as f32);
            }
        }
    }
    out
}

/// **S332, critère 1.** Une sphère qui tourne sur elle-même ne pousse pas l'eau : après un pas, vitesse
/// maximale sous 1 % de `Ω·R`. Un pavé qui tourne en pousse. *Mesuré en S332 P2 : 1,08 % à 6 mailles par
/// rayon — manqué —, 0,73 % à 12 — tenu —, décroissance d'ordre 0,56 seulement : la vitesse de la paroi est
/// prise au centre des faces, non au centroïde de leur part couverte, et une petite maille coupée amplifie
/// l'écart. L'essai garde le critère à 12 mailles par rayon et publie les deux valeurs.*
#[test]
fn a_spinning_sphere_pushes_no_water_a_spinning_box_does_s332() {
    let omega = [0f32, 0., 2.];
    let residu = |n: usize| {
        let (mut v, _) = volume_sphere(n);
        let z0 = v.domain.z0();
        v.set_surface(&vec![z0; n * n]).unwrap();
        v.set_solid_rigid(&noeuds_sphere(n, 0.6), [0.; 3], omega, [0.6, 0.6, 0.55]).unwrap();
        v.step_surface_linear(2000, 4000, &Jobs).unwrap();
        v.velocity_u().iter().chain(v.velocity_v()).chain(v.velocity_w()).fold(0f32, |m, x| m.max(x.abs()))
    };
    let (grossier, sphere) = (residu(24), residu(48));
    let ordre = (grossier / sphere).log2();
    println!("S332 : sphère qui tourne, vitesse max {grossier:e} puis {sphere:e} m/s (Ω·R = 0,6), ordre {ordre:.2}");
    assert!(sphere <= 0.01 * 0.6, "{sphere}");
    assert!(sphere < grossier, "le résidu ne décroît pas");
    let n = 24usize;
    let z0 = 1.2f32;
    let dx = 1.2 / n as f32;
    let boite = noeuds_boite(n, dx, [0.6, 0.6, 0.55], [0.2, 0.2, 0.2]);
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut b = Volume3::configure_with_solid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx: n, ny: n, nz: n, dx }, 1025., 9.81, &vec![0.; n * n], &boite).unwrap();
    b.set_surface(&vec![z0; n * n]).unwrap();
    b.set_solid_rigid(&boite, [0.; 3], omega, [0.6, 0.6, 0.55]).unwrap();
    b.step_surface_linear(2000, 4000, &Jobs).unwrap();
    let pave = b.velocity_u().iter().chain(b.velocity_v()).chain(b.velocity_w()).fold(0f32, |m, x| m.max(x.abs()));
    println!("S332 : pavé qui tourne, {pave:e} m/s");
    assert!(sphere <= 0.01 * 0.6, "{sphere}");
    assert!(pave >= 0.1 * 2. * 0.2, "{pave}");
}

/// Le cube de C10 à son tirant de mer, centré en (0,6 ; 0,6), dans un cube de 1,2 m de 24 mailles : il
/// perce le couvercle, élevé de `dz`.
fn cube_flottant(dz: f64) -> (Vec<f32>, [f64; 3]) {
    let d = 500. / 1025. * 0.5;
    let c = [0.6, 0.6, 1.2 - d + 0.25 + dz];
    (noeuds_boite(24, 1.2 / 24., c, [0.25, 0.25, 0.25]), c)
}

/// **S332, critère 2.** Une coque qui perce le couvercle : le pas linéaire l'accepte, et un lac au repos
/// autour d'elle reste au repos au bit ; sans l'autorisation de percer, la configuration la refuse.
#[test]
fn a_floating_hull_pierces_the_lid_and_the_lake_stays_at_rest_s332() {
    let n = 24usize;
    let dx = 1.2 / n as f32;
    let (s, _) = cube_flottant(0.);
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let d = Domain3 { nx: n, ny: n, nz: n, dx };
    assert_eq!(Volume3::configure_with_solid(&mut host, d, 1025., 9.81, &vec![0.; n * n], &s).err(), Some(Error::Domain));
    let mut v = Volume3::configure_with_floating_solid(&mut host, d, 1025., 9.81, &vec![0.; n * n], &s).unwrap();
    let (_, _, ow) = v.apertures().unwrap();
    assert!(ow[n * n * n..].iter().any(|a| *a == 0.), "le couvercle n'est pas percé");
    v.set_surface(&vec![1.2; n * n]).unwrap();
    for pas in 0..100 {
        v.step_surface_linear(2000, 4000, &Jobs).unwrap();
        assert!(v.velocity_u().iter().chain(v.velocity_v()).chain(v.velocity_w()).all(|x| x.to_bits() == 0), "pas {pas}");
        assert!(v.surface().iter().all(|h| h.to_bits() == 1.2f32.to_bits()), "pas {pas}");
    }
}

/// **S332, critère 3.** Le cube de C10 pilonne en perçant le couvercle, 2 cm à 2 Hz : la surface suit le
/// volume de coque plongé dans le domaine à 10⁻⁹ m³ près, pas après pas ; divergence sous la tolérance.
#[test]
fn a_heaving_hull_through_the_lid_keeps_the_water_volume_s332() {
    let n = 24usize;
    let dx = 1.2 / n as f32;
    let (s0, _) = cube_flottant(0.);
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut v = Volume3::configure_with_floating_solid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx: n, ny: n, nz: n, dx }, 1025., 9.81, &vec![0.; n * n], &s0).unwrap();
    v.set_surface(&vec![1.2; n * n]).unwrap();
    let solide = |v: &Volume3| v.cut.as_ref().unwrap().base.as_ref().unwrap().solid_col.iter().map(|x| *x as f64).sum::<f64>();
    let v0 = solide(&v);
    let (a, w) = (0.02f64, 2. * core::f64::consts::PI * 2.);
    let mut pire = 0f64;
    for pas in 1..=100 {
        let t = pas as f64 * 0.002;
        let (s, _) = cube_flottant(a * (w * t).sin());
        v.set_solid(&s, [0., 0., (a * w * (w * t).cos()) as f32]).unwrap();
        let r = v.step_surface_linear(2000, 4000, &Jobs).unwrap();
        assert!(r.divergence <= PROJECTION_DIVERGENCE_TOLERANCE as f64, "pas {pas} : {}", r.divergence);
        let eau: f64 = v.eta.iter().zip(&v.eta_roundoff).map(|(h, r)| (*h as f64 - *r as f64 - 1.2f32 as f64) * (dx as f64).powi(2)).sum();
        pire = pire.max((eau - (solide(&v) - v0)).abs());
    }
    println!("S332 : coque qui pilonne, écart de volume au pire {pire:e} m³");
    assert!(pire <= 1e-9, "{pire}");
}

/// **S332, critère 5 (I-04).** Le cube de C10, lâché la base à la surface, pilonne sur B + W (eau calme) ;
/// à chaque pas, δ reçoit sa coque — pose et vitesse de corps rigide — et lui rend une force qui n'anime
/// que le décalage visuel. La trajectoire de jeu est **identique au bit** avec ou sans δ ; le décalage reste
/// sous 8 cm et n'est pas nul.
#[test]
fn the_game_body_drives_its_wall_in_delta_and_delta_never_drives_it_s332() {
    use crate::body::Milieu;
    use crate::rigid_body::{oriented_box_distance, CalmWater, RenderOffset, RigidBody};
    let n = 24usize;
    let dx = 1.2 / n as f32;
    let calme = CalmWater { level: 1.2 };
    let depart = RigidBody::cuboid([0.5; 3], 500., [0.6, 0.6, 1.2 + 0.25], [8, 8, 8]);
    let coque = |c: &RigidBody| -> Vec<f32> {
        let mut out = Vec::with_capacity((n + 1).pow(3));
        for k in 0..=n {
            for j in 0..=n {
                for i in 0..=n {
                    let p = [i as f64 * dx as f64, j as f64 * dx as f64, k as f64 * dx as f64];
                    out.push(oriented_box_distance(c.position, c.orientation, [0.25; 3], p) as f32);
                }
            }
        }
        out
    };
    let mut seul = depart.clone();
    let reference: Vec<[u64; 3]> = (0..300).map(|_| {
        seul.step(0.002, &calme, Milieu::MER);
        seul.position.map(f64::to_bits)
    }).collect();
    let mut corps = depart.clone();
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut v = Volume3::configure_with_floating_solid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain3 { nx: n, ny: n, nz: n, dx }, 1025., 9.81, &vec![0.; n * n], &coque(&corps)).unwrap();
    v.set_surface(&vec![1.2; n * n]).unwrap();
    let mut rendu = RenderOffset::new(2. * core::f64::consts::PI * 2., 0.7);
    let (mut decalage, mut plus_bas) = (0f64, f64::MAX);
    for pas in 0..300 {
        corps.step(0.002, &calme, Milieu::MER);
        assert_eq!(corps.position.map(f64::to_bits), reference[pas], "pas {pas} : δ a touché la trajectoire de jeu");
        let s = coque(&corps);
        v.set_solid_rigid(&s, corps.velocity.map(|x| x as f32), corps.angular_velocity.map(|x| x as f32),
            corps.position.map(|x| x as f32)).unwrap();
        let r = v.step_surface_linear(2000, 4000, &Jobs).unwrap();
        assert!(r.divergence <= PROJECTION_DIVERGENCE_TOLERANCE as f64, "pas {pas}");
        let f = v.solid_force(&s).unwrap();
        let o = rendu.step(0.002, f, corps.mass);
        decalage = decalage.max((o[0] * o[0] + o[1] * o[1] + o[2] * o[2]).sqrt());
        plus_bas = plus_bas.min(corps.position[2]);
    }
    println!("S332 : décalage visuel maximal {decalage:.4} m ; coque descendue à {:.3} m sous la surface", 1.2 + 0.25 - plus_bas);
    assert!(decalage > 0. && decalage <= 0.08 + 1e-12, "{decalage}");
}

/// **S333, critère 2 — au repos relatif, δ au repos.** Une coque 4 × 1,6 × 1 m qui suit exactement la houle
/// de B — 6 s, 25 cm —, portée par la vitesse de l'eau sous elle, à la hauteur de sa surface et inclinée comme
/// elle, entre dans δ à une pose **constante au bit**, sans vitesse de paroi : δ reste au repos au bit, 100 pas
/// de 10 ms. Contre-épreuve : la même coque tenue immobile dans le monde, que la houle traverse, met δ en
/// mouvement — c'est son mouvement relatif que δ voit. Mesures sur les faces entièrement ouvertes et les
/// colonnes au couvercle libre : une face couverte par la coque garde une vitesse que rien ne lit — jusqu'à
/// 3,7 m/s dans cette contre-épreuve —, et une lamelle ouverte à quelques pour cent y concentre son flux.
#[test]
fn a_hull_that_follows_the_swell_leaves_delta_at_rest_s333() {
    use crate::background::{Background, SeaState};
    use crate::rigid_body::{surface_tilt, BackgroundWater, HullInDelta, RigidBody, WaterQuery};
    use crate::types::{SimTime, WorldPos};
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let sea = SeaState { hs: (2. * 2f64.sqrt() * 0.25) as f32, tp: 12., theta_turns: 0., components: 1, graine: 333 };
    let b = Background::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, sea, WorldPos::default()).unwrap();
    let (n, dx) = ([40usize, 20, 8], 0.2f64);
    let z_r = 0.5 - 500. / 1025.;
    let eau = |pas: u64| BackgroundWater { background: &b, time: SimTime(pas * 10_000) };
    let lance = |suit: bool| -> (f32, f32) {
        let mut corps = RigidBody::cuboid([4., 1.6, 1.], 500., [0., 0., z_r + eau(0).surface(0., 0.)], [16, 8, 4]);
        if suit {
            corps.orientation = surface_tilt(eau(0).slope(0., 0.));
        }
        let mut coque = HullInDelta::new(&corps, &eau(0), [-4., -2.], n[2] as f64 * dx);
        let depart = (coque.center, coque.orientation);
        let mut noeuds = Vec::new();
        coque.box_nodes([2., 0.8, 0.5], n, dx, &mut noeuds);
        let mut arena = Arena { stats: AllocStats::default(), sealed: false };
        let mut v = Volume3::configure_with_floating_solid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
            Domain3 { nx: n[0], ny: n[1], nz: n[2], dx: dx as f32 }, 1025., 9.81, &vec![0.; n[0] * n[1]], &noeuds).unwrap();
        let z0 = v.domain.z0();
        v.set_surface(&vec![z0; n[0] * n[1]]).unwrap();
        let (mut vitesse, mut hauteur) = (0f32, 0f32);
        for pas in 0..100u64 {
            let w = eau(pas + 1);
            if suit {
                let [x, y, _] = corps.position;
                let u = w.velocity([x, y, 0.]);
                corps.position[0] += 0.01 * u[0];
                corps.position[1] += 0.01 * u[1];
                let [x, y, _] = corps.position;
                let u = w.velocity([x, y, 0.]);
                corps.velocity = [u[0], u[1], 0.];
                corps.position[2] = z_r + w.surface(x, y);
                corps.orientation = surface_tilt(w.slope(x, y));
            }
            let m = coque.advance(&corps, &w, 0.01);
            if suit {
                assert_eq!((m.center, m.orientation), depart, "pas {pas} : la pose relative a bougé");
                assert!(m.velocity.iter().chain(&m.angular).all(|x| *x == 0.), "pas {pas} : {:?} {:?}", m.velocity, m.angular);
            }
            coque.box_nodes([2., 0.8, 0.5], n, dx, &mut noeuds);
            v.set_solid_rigid(&noeuds, m.velocity, m.angular, m.center).unwrap();
            let r = v.step_surface_linear(10_000, 4000, &Jobs).unwrap();
            assert!(r.divergence <= PROJECTION_DIVERGENCE_TOLERANCE as f64, "pas {pas}");
            // Faces entièrement ouvertes et colonnes au couvercle libre seulement : sous la coque, une face
            // couverte garde une vitesse que rien ne lit, et une colonne une hauteur de pure comptabilité.
            let g = v.cut.as_ref().unwrap();
            let libre = |o: &[f32], x: &[f32]| o.iter().zip(x).filter(|(o, _)| **o == 1.).fold(0f32, |m, (_, x)| m.max(x.abs()));
            vitesse = vitesse.max(libre(&g.open_u, v.velocity_u())).max(libre(&g.open_v, v.velocity_v())).max(libre(&g.open_w, v.velocity_w()));
            let couvercle = &g.open_w[n[0] * n[1] * n[2]..];
            hauteur = v.surface().iter().zip(couvercle).filter(|(_, o)| **o == 1.).fold(hauteur, |m, (h, _)| m.max((h - z0).abs()));
            if suit {
                assert!(v.velocity_u().iter().chain(v.velocity_v()).chain(v.velocity_w()).all(|x| x.to_bits() == 0), "pas {pas}");
                assert!(v.surface().iter().all(|h| h.to_bits() == z0.to_bits()), "pas {pas}");
            }
        }
        (vitesse, hauteur)
    };
    let (vitesse, hauteur) = lance(true);
    let (tenue_v, tenue_h) = lance(false);
    println!("S333 : coque qui suit la houle — δ {vitesse:e} m/s, {hauteur:e} m ; tenue immobile — {tenue_v:.4} m/s, {tenue_h:.4} m");
    assert!(tenue_v > 0.01 && tenue_h > 0.001, "{tenue_v} {tenue_h}");
}

/// **S335, critère 3 — le couvercle partiel sans pointe.** La contre-épreuve du critère 2 de S333 — coque
/// tenue fixe sur la houle, qui glisse, pilonne et s'incline dans δ — au couvercle partiel : vitesse sur les
/// faces entièrement ouvertes sous 1 m/s. Sans plancher d'ouverture, 5,47 m/s : une lamelle ouverte à quelques
/// pour cent change en pointe le reste de découpe d'une coque qui tourne (S335 P3).
#[test]
fn a_hull_held_on_the_swell_stays_calm_under_the_partial_lid_s335() {
    use crate::background::{Background, SeaState};
    use crate::rigid_body::{BackgroundWater, HullInDelta, RigidBody, WaterQuery};
    use crate::types::{SimTime, WorldPos};
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let sea = SeaState { hs: (2. * 2f64.sqrt() * 0.25) as f32, tp: 12., theta_turns: 0., components: 1, graine: 333 };
    let b = Background::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, sea, WorldPos::default()).unwrap();
    let (n, dx) = ([40usize, 20, 8], 0.2f64);
    let z_r = 0.5 - 500. / 1025.;
    let eau = |pas: u64| BackgroundWater { background: &b, time: SimTime(pas * 10_000) };
    let mesure = |partiel: bool| -> f32 {
        let corps = RigidBody::cuboid([4., 1.6, 1.], 500., [0., 0., z_r + eau(0).surface(0., 0.)], [16, 8, 4]);
        let mut coque = HullInDelta::new(&corps, &eau(0), [-4., -2.], n[2] as f64 * dx);
        let mut noeuds = Vec::new();
        coque.box_nodes([2., 0.8, 0.5], n, dx, &mut noeuds);
        let mut arena = Arena { stats: AllocStats::default(), sealed: false };
        let mut v = Volume3::configure_with_floating_solid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
            Domain3 { nx: n[0], ny: n[1], nz: n[2], dx: dx as f32 }, 1025., 9.81, &vec![0.; n[0] * n[1]], &noeuds).unwrap();
        let z0 = v.domain.z0();
        v.set_surface(&vec![z0; n[0] * n[1]]).unwrap();
        v.set_partial_lid(partiel);
        let mut pire = 0f32;
        for pas in 0..100u64 {
            let w = eau(pas + 1);
            let m = coque.advance(&corps, &w, 0.01);
            coque.box_nodes([2., 0.8, 0.5], n, dx, &mut noeuds);
            v.set_solid_rigid(&noeuds, m.velocity, m.angular, m.center).unwrap();
            let r = v.step_surface_linear(10_000, 4000, &Jobs).unwrap();
            assert!(r.divergence <= PROJECTION_DIVERGENCE_TOLERANCE as f64, "pas {pas}");
            let g = v.cut.as_ref().unwrap();
            let libre = |o: &[f32], x: &[f32]| o.iter().zip(x).filter(|(o, _)| **o == 1.).fold(0f32, |m, (_, x)| m.max(x.abs()));
            pire = pire.max(libre(&g.open_u, v.velocity_u())).max(libre(&g.open_v, v.velocity_v())).max(libre(&g.open_w, v.velocity_w()));
        }
        pire
    };
    let (s332, partiel) = (mesure(false), mesure(true));
    println!("S335 : coque tenue sur la houle — couvercle de S332 {s332:.4} m/s, couvercle partiel {partiel:.4} m/s");
    assert!(partiel <= 1., "{partiel}");
}

/// **S337, critère 1 — l'éponge du mode linéaire.** Une crête de 10 cm lâchée au milieu d'une tranche de 32 m sur
/// 2 m, mailles de 25 cm, 20 s : avec des murs, les ondes restent ; avec l'éponge du pas couplé — 4 m, 2 /s —,
/// l'énergie de surface de l'intérieur (`Σ(η − z₀)²` hors des bandes d'éponge) tombe sous 20 % de celle des
/// murs, et le volume de δ plus celui que l'éponge retire reste celui de la crête, au plancher d'arrondi : sous
/// la borne cumulée du transport, `3·2⁻²⁴·Σ|Δη|·dx²` (S333) — avec des murs, l'écart est du même ordre.
#[test]
fn the_linear_sponge_lets_the_waves_out_s337() {
    let (nx, nz, dx) = (128usize, 8usize, 0.25f32);
    let lance = |eponge: bool| -> (f64, f64, f64) {
        let mut arena = Arena { stats: AllocStats::default(), sealed: false };
        let mut v = Volume3::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
            Domain3 { nx, ny: 2, nz, dx }, 1025., 9.81).unwrap();
        let z0 = v.domain.z0();
        let mut eta = vec![z0; nx * 2];
        for j in 0..2 {
            for i in 0..nx {
                let x = (i as f32 + 0.5) * dx - 16.;
                eta[j * nx + i] += 0.1 * (-x * x / (2. * 0.6 * 0.6)).exp();
            }
        }
        v.set_surface(&eta).unwrap();
        if eponge {
            v.set_linear_sponge(Some(Sponge3 { width_x: 4., width_y: 0., rate_per_s: 2. })).unwrap();
        }
        let v0 = v.perturbation_volume();
        let (mut pire, mut plancher, mut avant) = (0f64, 0f64, v.surface().to_vec());
        for _ in 0..2000 {
            v.step_surface_linear(10_000, 4000, &Jobs).unwrap();
            pire = pire.max((v.perturbation_volume() + v.linear_sponge_removed() - v0).abs());
            plancher += v.surface().iter().zip(&avant).map(|(h, a)| (h - a).abs() as f64).sum::<f64>() * (dx as f64).powi(2) * 3. / 16_777_216.;
            avant.copy_from_slice(v.surface());
        }
        let interieur: f64 = (0..nx)
            .filter(|i| { let x = (*i as f32 + 0.5) * dx; x > 4. && x < 28. })
            .map(|i| ((v.surface()[i] - z0) as f64).powi(2))
            .sum();
        (interieur, pire, plancher)
    };
    let (murs, volume_murs, _) = lance(false);
    let (eponge, volume, plancher) = lance(true);
    println!("S337 : énergie de surface intérieure à 20 s — murs {murs:.3e}, éponge {eponge:.3e} ({:.1} %) ; volume de δ plus retiré, écart {volume:.2e} m³ pour une borne d'arrondi {plancher:.2e} (murs : {volume_murs:.2e})", 100. * eponge / murs);
    assert!(eponge <= 0.2 * murs, "{eponge} contre {murs}");
    assert!(volume <= plancher, "{volume} contre {plancher}");
}

/// **S369, A289 — δ relatif à la dynamique de B.** Sous une houle B seule (une composante, 5 cm, λ = 4 m), δ nul au
/// départ : avec les trois termes propres à B retirés, δ nul est un **point fixe** — hauteur, vitesses et pression
/// nulles au bit, pas après pas ; avec le pas de S297, le même banc s'écarte (la source que S319 a mesurée). Un masque
/// hors des sept bits (trois retraits, quatre essais) est refusé.
#[test]
fn zero_delta_stays_zero_under_b_alone_when_relative_s369() {
    use crate::{background::{Background, SeaState}, SimTime, WorldPos};
    let (dx, h0) = (0.25f32, 2.0f32);
    let d = Domain3 { nx: 48, ny: 2, nz: (h0 / dx) as usize + 2, dx };
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let sea = SeaState { hs: 0.05 * 2. * 2f32.sqrt(), tp: 3.2, theta_turns: 0., components: 1, graine: 7 };
    let houle = Background::configure(&mut host, sea, WorldPos::from_units(0, 0, 0)).unwrap();
    let mut grid = BackgroundGrid3::configure(&mut host, d, [0., 0., -h0], 1025.).unwrap();
    let mut volumes = [0u8, Volume3::RELATIVE_ALL].map(|terms| {
        let mut v = Volume3::configure(&mut host, d, 1025., houle.gravity()).unwrap();
        v.set_free_surface(&vec![h0; d.columns()], h0).unwrap();
        v.set_relative_background(terms).unwrap();
        v
    });
    assert_eq!(volumes[0].set_relative_background(128), Err(Error::Domain));
    let sponge = Sponge3 { width_x: 2., width_y: 0., rate_per_s: 2. };
    for n in 0..150u64 {
        let time = SimTime(n * 10_000);
        grid.sample(&houle, time).unwrap();
        let bg = grid.view().unwrap();
        for v in &mut volumes {
            v.step_perturbation_mobile(time, 10_000, 60_000, &bg, sponge, &Jobs).unwrap();
        }
        let v = &volumes[1];
        assert!(v.eta.iter().all(|e| e.to_bits() == h0.to_bits()), "pas {n}");
        assert!(v.u.iter().chain(&v.v).chain(&v.w).chain(&v.p).all(|x| *x == 0.), "pas {n}");
    }
    let ecart = volumes[0].eta.iter().fold(0f32, |m, e| m.max((e - h0).abs()));
    println!("S369 : 1,5 s sous 5 cm de houle — pas de S297, δ jusqu'à {ecart:.3e} m ; relatif, nul au bit");
    assert!(ecart > 1e-5, "{ecart}");
}

/// **S434, C7d-3a** — avec les termes croisés sous la forme de Bernoulli, δ nul reste nul au bit sous B seul (relatif), comme en
/// S369 ; et la forme change bien le pas quand δ n'est pas nul (un germe : les deux pas s'écartent).
#[test]
fn zero_delta_stays_zero_with_the_bernoulli_cross_terms_s434() {
    use crate::{background::{Background, SeaState}, SimTime, WorldPos};
    let (dx, h0) = (0.25f32, 2.0f32);
    let d = Domain3 { nx: 48, ny: 2, nz: (h0 / dx) as usize + 2, dx };
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let sea = SeaState { hs: 0.05 * 2. * 2f32.sqrt(), tp: 3.2, theta_turns: 0., components: 1, graine: 7 };
    let houle = Background::configure(&mut host, sea, WorldPos::from_units(0, 0, 0)).unwrap();
    let mut grid = BackgroundGrid3::configure(&mut host, d, [0., 0., -h0], 1025.).unwrap();
    // 0 : nul, Bernoulli ; 1 : germe, sans ; 2 : germe, Bernoulli.
    let mut volumes = [(false, true), (true, false), (true, true)].map(|(seed, bernoulli)| {
        let mut v = Volume3::configure(&mut host, d, 1025., houle.gravity()).unwrap();
        let mut surface = vec![h0; d.columns()];
        if seed {
            for i in 20..28 {
                for j in 0..d.ny {
                    surface[j * d.nx + i] = h0 + 0.001 * ((i as f32 - 24.) * 0.4).cos();
                }
            }
        }
        v.set_free_surface(&surface, h0).unwrap();
        v.set_relative_background(Volume3::RELATIVE_ALL).unwrap();
        v.set_cross_bernoulli(bernoulli);
        v
    });
    let sponge = Sponge3 { width_x: 2., width_y: 0., rate_per_s: 2. };
    for n in 0..150u64 {
        let time = SimTime(n * 10_000);
        grid.sample(&houle, time).unwrap();
        let bg = grid.view().unwrap();
        for v in &mut volumes {
            v.step_perturbation_mobile(time, 10_000, 60_000, &bg, sponge, &Jobs).unwrap();
        }
        let v = &volumes[0];
        assert!(v.eta.iter().all(|e| e.to_bits() == h0.to_bits()), "pas {n}");
        assert!(v.u.iter().chain(&v.v).chain(&v.w).chain(&v.p).all(|x| *x == 0.), "pas {n}");
    }
    let ecart = volumes[1].eta.iter().zip(&volumes[2].eta).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
    println!("S434 : δ nul reste nul avec la forme de Bernoulli ; un germe de 1 mm, les deux formes s'écartent de {ecart:.3e} m en 1,5 s");
    assert!(ecart > 0., "la forme ne change rien");
}

/// **S436, A324** — sous une houle de plus d'une demi-maille (6,5 cm à 12,5 cm), la surface de B franchit des centres de mailles ;
/// le fantôme latéral d'avant rompait le point fixe du mode relatif (δ nul ne restait pas nul). Avec `set_lateral_own_ghost`, δ nul
/// reste nul au bit ; sans, il ne l'est plus — le défaut, gardé comme témoin ; et un germe de 1 mm ne dépasse pas 1,5 mm en 1 s.
#[test]
fn zero_delta_stays_zero_when_b_crosses_a_cell_centre_s436() {
    use crate::{background::{Background, SeaState}, SimTime, WorldPos};
    let (dx, h0) = (0.125f32, 2.5f32);
    let d = Domain3 { nx: 96, ny: 2, nz: (h0 / dx) as usize + 2, dx };
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let sea = SeaState { hs: 0.065 * 2. * 2f32.sqrt(), tp: 3.2, theta_turns: 0., components: 1, graine: 7 };
    let houle = Background::configure(&mut host, sea, WorldPos::from_units(0, 0, 0)).unwrap();
    let mut grid = BackgroundGrid3::configure(&mut host, d, [0., 0., -h0], 1025.).unwrap();
    // 0 : nul, remède ; 1 : nul, sans ; 2 : germe, remède.
    let mut volumes = [(false, true), (false, false), (true, true)].map(|(seed, own)| {
        let mut v = Volume3::configure(&mut host, d, 1025., houle.gravity()).unwrap();
        let mut surface = vec![h0; d.columns()];
        if seed {
            for i in 38..58 {
                for j in 0..d.ny {
                    surface[j * d.nx + i] = h0 + 0.001 * ((i as f32 - 48.) * 0.3).cos();
                }
            }
        }
        v.set_free_surface(&surface, h0).unwrap();
        v.set_relative_background(Volume3::RELATIVE_ALL).unwrap();
        v.set_lateral_own_ghost(own);
        v
    });
    let sponge = Sponge3 { width_x: 2., width_y: 0., rate_per_s: 2. };
    let mut germe_max = 0f32;
    for n in 0..100u64 {
        let time = SimTime(n * 10_000);
        grid.sample(&houle, time).unwrap();
        let bg = grid.view().unwrap();
        for v in &mut volumes {
            v.step_perturbation_mobile(time, 10_000, 200, &bg, sponge, &Jobs).unwrap();
        }
        let v = &volumes[0];
        assert!(v.eta.iter().all(|e| e.to_bits() == h0.to_bits()), "pas {n}");
        assert!(v.u.iter().chain(&v.v).chain(&v.w).chain(&v.p).all(|x| *x == 0.), "pas {n}");
        germe_max = germe_max.max(volumes[2].eta.iter().fold(0f32, |m, e| m.max((e - h0).abs())));
    }
    let sans = volumes[1].eta.iter().fold(0f32, |m, e| m.max((e - h0).abs()));
    println!("S436 : δ nul reste nul au bit avec le remède ; sans, {sans:.2e} m en 1 s ; germe de 1 mm : {germe_max:.2e} m au plus");
    assert!(sans > 1e-4, "le défaut d'avant ne se montre plus : le cas ne franchit plus de centre");
    assert!(germe_max <= 1.5e-3, "germe à {germe_max:.2e} m");
}

/// **S441, A322** — la bande relative sous Lax-Wendroff : δ nul reste nul au bit sous une houle qui franchit des centres de
/// maille ; avec un germe, la bande change bien le pas (les deux formes s'écartent).
#[test]
fn zero_delta_stays_zero_with_the_lax_wendroff_band_s441() {
    use crate::{background::{Background, SeaState}, SimTime, WorldPos};
    let (dx, h0) = (0.125f32, 2.5f32);
    let d = Domain3 { nx: 96, ny: 2, nz: (h0 / dx) as usize + 2, dx };
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
    let sea = SeaState { hs: 0.075 * 2. * 2f32.sqrt(), tp: 3.2, theta_turns: 0., components: 1, graine: 7 };
    let houle = Background::configure(&mut host, sea, WorldPos::from_units(0, 0, 0)).unwrap();
    let mut grid = BackgroundGrid3::configure(&mut host, d, [0., 0., -h0], 1025.).unwrap();
    // 0 : nul, Lax-Wendroff ; 1 : germe, sans ; 2 : germe, Lax-Wendroff.
    let mut volumes = [(false, true), (true, false), (true, true)].map(|(seed, lw)| {
        let mut v = Volume3::configure(&mut host, d, 1025., houle.gravity()).unwrap();
        let mut surface = vec![h0; d.columns()];
        if seed {
            for i in 38..58 {
                for j in 0..d.ny {
                    surface[j * d.nx + i] = h0 + 0.001 * ((i as f32 - 48.) * 0.3).cos();
                }
            }
        }
        v.set_free_surface(&surface, h0).unwrap();
        v.set_relative_background(Volume3::RELATIVE_ALL).unwrap();
        v.set_relative_band_lax_wendroff(lw);
        v
    });
    let sponge = Sponge3 { width_x: 2., width_y: 0., rate_per_s: 2. };
    for n in 0..60u64 {
        let time = SimTime(n * 10_000);
        grid.sample(&houle, time).unwrap();
        let bg = grid.view().unwrap();
        for v in &mut volumes {
            v.step_perturbation_mobile(time, 10_000, 200, &bg, sponge, &Jobs).unwrap();
        }
        let v = &volumes[0];
        assert!(v.eta.iter().all(|e| e.to_bits() == h0.to_bits()), "pas {n}");
        assert!(v.u.iter().chain(&v.v).chain(&v.w).chain(&v.p).all(|x| *x == 0.), "pas {n}");
    }
    let ecart = volumes[1].eta.iter().zip(&volumes[2].eta).fold(0f32, |m, (a, b)| m.max((a - b).abs()));
    println!("S441 : δ nul reste nul avec la bande sous Lax-Wendroff ; un germe de 1 mm, les deux formes s'écartent de {ecart:.3e} m en 0,6 s");
    assert!(ecart > 0., "la bande sous Lax-Wendroff ne change rien");
}

/// S375 (ADR-200 D2) — **l'entrée de volume par colonne**. Le volume de perturbation change de `Σ dh·dx²` (critère 1 :
/// ≤ 10⁻⁹ m³ par ajout, compensé) ; pression et vitesses intactes au bit ; sans ajout, un domaine au repos reste au repos
/// au bit ; refus atomiques (longueur, non fini, surface hors bornes).
#[test]
fn column_volume_is_added_exactly_and_atomically_s375() {
    let (mut v, _) = volume(12, 7, 12, 0.25, 9.81);
    v.set_free_surface(&[2.0; 84], 2.0).unwrap();
    for _ in 0..20 {
        v.step_surface_mobile(1000, 4000, &Jobs).unwrap();
    }
    assert!(v.surface().iter().all(|x| *x == 2.0), "le repos doit rester le repos");
    let mut pire = 0f64;
    let dh: Vec<f32> = (0..84).map(|c| 1e-4 * ((c * 37 % 11) as f32 - 5.0)).collect();
    for _ in 0..200 {
        let avant = v.perturbation_volume();
        let (p, u, w) = (v.pressure().to_vec(), v.velocity_u().to_vec(), v.velocity_w().to_vec());
        v.add_column_volume(&dh).unwrap();
        let attendu: f64 = dh.iter().map(|d| *d as f64).sum::<f64>() * 0.0625;
        pire = pire.max((v.perturbation_volume() - avant - attendu).abs());
        assert_eq!((p, u, w), (v.pressure().to_vec(), v.velocity_u().to_vec(), v.velocity_w().to_vec()));
    }
    println!("S375 ajout de volume : pire écart {pire:e} m³ par ajout");
    assert!(pire <= 1e-9, "{pire}");
    let mut w = v.surface().to_vec();
    let bits = |x: &Volume3| x.surface().iter().map(|e| e.to_bits()).collect::<Vec<_>>();
    let avant = bits(&v);
    assert_eq!(v.add_column_volume(&[0.; 83]).err(), Some(Error::Shape));
    w[3] = f32::NAN;
    assert_eq!(v.add_column_volume(&w).err(), Some(Error::NotFinite));
    let mut trop = vec![0.; 84];
    trop[5] = 1.0;
    assert_eq!(v.add_column_volume(&trop).err(), Some(Error::Domain));
    assert_eq!(bits(&v), avant, "un refus a écrit");
}


/// S375, ADR-201 — **une élévation uniforme est un état sans mouvement, et le pas mobile l'accepte.** Avant le plancher
/// de l'échelle de vitesse, le critère de divergence divisait deux arrondis (2 à 4) et refusait le pas — sur tout domaine,
/// dès 1 µm d'écart au repos. Maintenant : le pas passe, et rien ne bouge (vitesses sous 10⁻⁶ m/s, surface uniforme au
/// dixième de micromètre, sur 40 pas).
#[test]
fn a_uniform_rise_is_a_state_without_motion_s375() {
    for (nx, ny, nz, dx) in [(12usize, 7usize, 12usize, 0.25f32), (40, 20, 8, 0.2)] {
        let (mut v, _) = volume(nx, ny, nz, dx, 9.81);
        let h = 0.6 * nz as f32 * dx;
        v.set_free_surface(&vec![h; nx * ny], h).unwrap();
        v.add_column_volume(&vec![1e-4; nx * ny]).unwrap();
        for _ in 0..40 {
            v.step_surface_mobile(25_000, 4000, &Jobs).unwrap();
        }
        let vmax = v.velocity_u().iter().chain(v.velocity_v()).chain(v.velocity_w()).fold(0f32, |m, x| m.max(x.abs()));
        let (lo, hi) = v.surface().iter().fold((f32::MAX, f32::MIN), |(a, b), x| (a.min(*x), b.max(*x)));
        println!("S375 élévation uniforme {nx}x{ny}x{nz} : vitesse max {vmax:e} m/s, étendue de surface {:e} m", hi - lo);
        assert!(vmax < 1e-6 && hi - lo < 1e-7, "{vmax} {}", hi - lo);
    }
}

/// S375 — **déplacer le repos ne change pas la physique.** Deux domaines, la même onde stationnaire sur un niveau décalé
/// de 8 mm du repos ; l'un garde son repos, l'autre le déplace sur le niveau moyen (`shift_rest`) : après 400 pas, les
/// surfaces s'accordent **à deux ulps f32 près** (4,8·10⁻⁷ m à 2 m ; le premier seuil écrit, 10⁻⁷ m, était sous la
/// résolution de la surface — un ulp y vaut 2,4·10⁻⁷ ; mesuré : un ulp), et la pression du second est la perturbation
/// seule (sa moyenne tombe de `ρ·g·8 mm` à presque rien).
#[test]
fn shifting_rest_to_the_mean_level_changes_no_physics_s375() {
    let (mut a, _) = volume(12, 7, 12, 0.25, 9.81);
    let (mut b, _) = volume(12, 7, 12, 0.25, 9.81);
    let eta: Vec<f32> = (0..84).map(|c| 2.008 + 0.02 * (std::f32::consts::PI * ((c % 12) as f32 + 0.5) / 12.).cos()).collect();
    a.set_free_surface(&eta, 2.0).unwrap();
    b.set_free_surface(&eta, 2.0).unwrap();
    b.shift_rest(2.008).unwrap();
    let mut pire = 0f32;
    for _ in 0..400 {
        a.step_surface_mobile(10_000, 4000, &Jobs).unwrap();
        b.step_surface_mobile(10_000, 4000, &Jobs).unwrap();
        for (x, y) in a.surface().iter().zip(b.surface()) {
            pire = pire.max((x - y).abs());
        }
    }
    let moyenne = |v: &Volume3| {
        let n = v.pressure().iter().filter(|x| **x != 0.).count().max(1);
        v.pressure().iter().map(|x| *x as f64).sum::<f64>() / n as f64
    };
    println!("S375 repos déplacé : écart de surface {pire:e} m ; pression moyenne {:.3} Pa contre {:.3} Pa", moyenne(&b), moyenne(&a));
    assert!(pire <= 2.0 * f32::EPSILON * 2.0, "{pire}");
    assert!(moyenne(&b).abs() < 0.1 * moyenne(&a).abs());
}


// ---------------------------------------------------------------- S385 : la multigrille 3D (C1, ADR-207 D3)

fn volume_mg(nx: usize, ny: usize, nz: usize, dx: f32, bosse: bool) -> (Volume3, Arena) {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let domain = Domain3 { nx, ny, nz, dx };
    let v = {
        let mut host = HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs };
        if bosse {
            let b: Vec<f32> = (0..ny).flat_map(|j| (0..nx).map(move |i| {
                let (x, y) = ((i as f32 + 0.5) * dx, (j as f32 + 0.5) * dx);
                let (a, c) = ((x - 3.0) / 1.2, (y - 1.3) / 0.9);
                0.4 + 0.6 * (-(a * a + c * c)).exp()
            })).collect();
            Volume3::configure_with_bottom(&mut host, domain, 1025., 9.81, &b).unwrap()
        } else {
            Volume3::configure(&mut host, domain, 1025., 9.81).unwrap()
        }
    };
    (v, arena)
}

fn sine_surface(nx: usize, ny: usize, dx: f32) -> Vec<f32> {
    (0..ny).flat_map(|_| (0..nx).map(move |i| 4. + 0.01 * (core::f32::consts::TAU * (i as f32 + 0.5) * dx / 8.).sin()))
        .collect()
}

#[test]
fn multigrid_hierarchy_is_counted_exactly_s385() {
    assert_eq!(multigrid3::level_count3(32, 16, 24), 3);
    assert_eq!(multigrid3::level_count3(7, 16, 24), 0);
    assert_eq!(multigrid3::level_count3(120, 112, 28), 2);
    for (nx, ny, nz) in [(32, 16, 24), (16, 8, 12), (7, 5, 3)] {
        let mg = multigrid3::Multigrid3::new(nx, ny, nz, 0.25);
        let mut held = mg.z.len() + mg.t.len();
        for l in &mg.levels {
            held += l.open_u.len() + l.open_v.len() + l.open_w.len() + l.kind.len() + l.diag.len() + l.x.len()
                + l.r.len() + l.t.len();
        }
        assert_eq!(Some(held), multigrid3::hierarchy_floats3(nx, ny, nz));
        let (mut v, mut arena) = volume(nx, ny, nz, 0.25, 9.81);
        let before = arena.stats.persistent_bytes;
        v.enable_multigrid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
        assert_eq!(arena.stats.persistent_bytes - before, held * 4);
        assert_eq!(v.multigrid_levels(), Some(multigrid3::level_count3(nx, ny, nz)));
    }
    // Après `seal()`, la réserve est refusée et rien ne change.
    let (mut v, mut arena) = volume(16, 8, 12, 0.5, 9.81);
    arena.sealed = true;
    assert_eq!(v.enable_multigrid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }), Err(Error::Domain));
    assert_eq!(v.multigrid_levels(), None);
}

#[test]
fn multigrid_cycle_is_symmetric_and_positive_s385() {
    for bosse in [false, true] {
        let (nx, ny, nz, dx) = (32, 16, 24, 0.25);
        let (mut v, mut arena) = volume_mg(nx, ny, nz, dx, bosse);
        v.enable_multigrid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
        v.set_free_surface(&sine_surface(nx, ny, dx), 4.).unwrap();
        v.rhs.fill(0.);
        v.rhs_mobile3(1.);
        v.prepare_multigrid3();
        let mut vectors = [noise(v.p.len(), 11), noise(v.p.len(), 13)];
        for x in vectors.iter_mut() {
            for c in 0..x.len() {
                if v.prec[c] == 0. { x[c] = 0.; }
            }
        }
        let mut images = Vec::new();
        for x in &vectors {
            v.res.copy_from_slice(x);
            v.v_cycle3();
            images.push(v.mg.as_ref().unwrap().z.clone());
        }
        let dot = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(x, y)| *x as f64 * *y as f64).sum::<f64>();
        let (ab, ba) = (dot(&images[0], &vectors[1]), dot(&images[1], &vectors[0]));
        let scale = (dot(&images[0], &images[0]) * dot(&vectors[1], &vectors[1])).sqrt();
        assert!((ab - ba).abs() <= 1e-5 * scale, "bosse={bosse} : {ab} contre {ba}");
        for (z, x) in images.iter().zip(&vectors) {
            assert!(dot(z, x) > 0., "bosse={bosse}");
            for c in 0..z.len() {
                if v.prec[c] == 0. { assert_eq!(z[c], 0.); }
            }
        }
    }
}

#[test]
fn multigrid_reaches_the_same_surface_with_fewer_iterations_s385() {
    for bosse in [false, true] {
        let (nx, ny, nz, dx) = (32, 16, 24, 0.25);
        let (mut a, _) = volume_mg(nx, ny, nz, dx, bosse);
        let (mut b, mut arena) = volume_mg(nx, ny, nz, dx, bosse);
        b.enable_multigrid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }).unwrap();
        for v in [&mut a, &mut b] {
            v.set_free_surface(&sine_surface(nx, ny, dx), 4.).unwrap();
        }
        let (mut ia, mut ib) = (0u32, 0u32);
        for _ in 0..20 {
            let ra = a.step_surface_mobile(2000, 20_000, &Jobs).unwrap();
            let rb = b.step_surface_mobile(2000, 20_000, &Jobs).unwrap();
            assert!(!ra.degraded && !rb.degraded);
            ia += ra.iterations;
            ib += rb.iterations;
        }
        let worst = a.surface().iter().zip(b.surface()).fold(0f32, |m, (x, y)| m.max((x - y).abs()));
        assert!(worst <= 1e-6, "bosse={bosse} : écart {worst} m");
        assert!(2 * ib < ia, "bosse={bosse} : {ib} itérations contre {ia}");
    }
}

// ---------------------------------------------------------------- S386 : la colonne graduée (C2, ADR-208)

fn host(arena: &mut Arena) -> HostServices<'_> {
    HostServices { alloc: arena, jobs: &Jobs, sink: &Jobs }
}

#[test]
fn graded_column_is_counted_and_validated_s386() {
    let (mut v, mut arena) = volume(8, 4, 12, 0.5, 9.81);
    assert_eq!(v.enable_graded(&mut host(&mut arena), &[1, 5, 11]), Err(Error::Shape));
    assert_eq!(v.enable_graded(&mut host(&mut arena), &[0, 5, 10]), Err(Error::Shape));
    assert_eq!(v.enable_graded(&mut host(&mut arena), &[0, 5, 5, 11]), Err(Error::Shape));
    assert_eq!(v.pressure_unknowns_per_column(), 12);
    let before = arena.stats.persistent_bytes;
    let nodes = [0, 4, 7, 9, 10, 11];
    v.enable_graded(&mut host(&mut arena), &nodes).unwrap();
    assert_eq!(arena.stats.persistent_bytes - before, (7 * 32 * 6 + 6) * 4 + 6 * core::mem::size_of::<usize>());
    assert_eq!(v.pressure_unknowns_per_column(), 6);
    let (mut c, mut arena) = volume_mg(16, 8, 12, 0.5, true);
    assert_eq!(c.enable_graded(&mut host(&mut arena), &[0, 6, 11]), Err(Error::Domain));
}

#[test]
fn graded_operator_is_symmetric_and_positive_s386() {
    let (mut v, mut arena) = volume(8, 4, 16, 0.25, 9.81);
    v.enable_graded(&mut host(&mut arena), &[0, 3, 7, 10, 12, 13, 14, 15]).unwrap();
    let g = v.graded.take().unwrap();
    let n = g.x.len();
    let (x, y) = (noise(n, 21), noise(n, 23));
    let (mut ax, mut ay) = (vec![0f32; n], vec![0f32; n]);
    v.graded_apply(&g, &x, &mut ax, false);
    v.graded_apply(&g, &y, &mut ay, false);
    let dot = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(p, q)| *p as f64 * *q as f64).sum::<f64>();
    assert!((dot(&x, &ay) - dot(&y, &ax)).abs() < 1e-5 * (dot(&x, &x) * dot(&ay, &ay)).sqrt());
    assert!(dot(&x, &ax) > 0. && dot(&y, &ay) > 0.);
    // `Pᵀ` est la transposée exacte de `P`.
    let mut fine = vec![0f32; v.p.len()];
    let f = noise(v.p.len(), 29);
    let mut rf = vec![0f32; n];
    v.graded_expand(&g, &x, &mut fine);
    v.graded_restrict(&g, &f, &mut rf);
    assert!((dot(&fine, &f) - dot(&x, &rf)).abs() < 1e-5 * (dot(&fine, &fine) * dot(&f, &f)).sqrt());
}

#[test]
fn graded_rest_is_exact_and_volume_is_kept_s386() {
    let (mut v, mut arena) = volume(16, 8, 8, 0.5, 9.81);
    v.enable_graded(&mut host(&mut arena), &[0, 3, 5, 6, 7]).unwrap();
    let z0 = v.domain().z0();
    let r = v.step_surface_linear(2000, 4000, &Jobs).unwrap();
    assert_eq!(r.iterations, 0);
    assert!(v.eta.iter().all(|e| e.to_bits() == z0.to_bits()));
    let eta: Vec<f32> = (0..8).flat_map(|j| (0..16).map(move |i| {
        let (x, y) = ((i as f32 + 0.5) * 0.5 - 4., (j as f32 + 0.5) * 0.5 - 2.);
        z0 + 0.02 * (-(x * x + y * y)).exp()
    })).collect();
    v.set_surface(&eta).unwrap();
    let volume = |v: &Volume3| v.eta.iter().zip(&v.eta_roundoff).map(|(e, r)| (*e as f64 - z0 as f64) - *r as f64).sum::<f64>();
    let start = volume(&v);
    for _ in 0..50 {
        let r = v.step_surface_linear(2000, 4000, &Jobs).unwrap();
        assert!(!r.degraded, "{r:?}");
    }
    assert!((volume(&v) - start).abs() < 1e-9, "{} contre {}", volume(&v), start);
}

#[test]
fn graded_with_every_layer_is_the_fine_scheme_s386() {
    let (mut a, _) = volume(16, 8, 8, 0.5, 9.81);
    let (mut b, mut arena) = volume(16, 8, 8, 0.5, 9.81);
    b.enable_graded(&mut host(&mut arena), &(0..8).collect::<Vec<_>>()).unwrap();
    let z0 = a.domain().z0();
    let eta: Vec<f32> = (0..8).flat_map(|_| (0..16).map(move |i| z0 + 0.01 * ((i as f32 + 0.5) * 0.4).cos())).collect();
    a.set_surface(&eta).unwrap();
    b.set_surface(&eta).unwrap();
    for _ in 0..20 {
        a.step_surface_linear(2000, 4000, &Jobs).unwrap();
        b.step_surface_linear(2000, 4000, &Jobs).unwrap();
    }
    let worst = a.eta.iter().zip(&b.eta).fold(0f32, |m, (x, y)| m.max((x - y).abs()));
    assert!(worst < 1e-6, "{worst}");
}

/// S387 : le pas mobile porte la colonne graduée, mais refuse une surface qui descend sous son plus bas nœud cubique
/// (ADR-208 D4) — ici 3 m pour des couches cubiques à partir de 3,25 m.
#[test]
fn graded_is_refused_by_the_mobile_step_s386() {
    let (mut v, mut arena) = volume(8, 4, 16, 0.25, 9.81);
    v.enable_graded(&mut host(&mut arena), &[0, 4, 8, 11, 13, 14, 15]).unwrap();
    let eta: Vec<f32> = (0..32).map(|c| 3. + 0.01 * (c as f32).sin()).collect();
    v.set_free_surface(&eta, 3.).unwrap();
    let before: Vec<u32> = v.eta.iter().map(|x| x.to_bits()).collect();
    assert_eq!(v.step_surface_mobile(1000, 4000, &Jobs).err(), Some(Error::Domain));
    assert!(v.eta.iter().map(|x| x.to_bits()).eq(before));
}

/// La fréquence du schéma **gradué** (ADR-208), comme `scheme_frequency` pour le schéma fin : le problème vertical fin,
/// restreint par `P` (linéaire par morceaux entre `nodes`), résolu en f64 dense ; `s = Σ (P·φ̂)_k·dx`.
fn graded_scheme_frequency(kx: f64, ky: f64, dx: f64, nz: usize, nodes: &[usize], g: f64, dt: f64) -> (f64, f64) {
    let kappa2 = 4. / (dx * dx) * ((kx * dx / 2.).sin().powi(2) + (ky * dx / 2.).sin().powi(2));
    let mut a = vec![vec![0f64; nz]; nz];
    let mut d = vec![0f64; nz];
    for k in 0..nz {
        a[k][k] = kappa2 * dx * dx;
        if k > 0 { a[k][k] += 1.; a[k][k - 1] = -1.; }
        if k + 1 < nz { a[k][k] += 1.; a[k][k + 1] = -1.; } else { a[k][k] += 2.; d[k] = 2.; }
    }
    let r = nodes.len();
    let mut p = vec![vec![0f64; r]; nz];
    for s in 0..r - 1 {
        let (lo, hi) = (nodes[s], nodes[s + 1]);
        for k in lo..hi {
            let t = (k - lo) as f64 / (hi - lo) as f64;
            p[k][s] = 1. - t;
            p[k][s + 1] = t;
        }
    }
    p[nodes[r - 1]][r - 1] = 1.;
    // Aᵣ = Pᵀ A P, bᵣ = Pᵀ d, puis Gauss avec pivot partiel.
    let mut m = vec![vec![0f64; r + 1]; r];
    for i in 0..r {
        for j in 0..r {
            let mut acc = 0.;
            for k in 0..nz { for l in 0..nz { acc += p[k][i] * a[k][l] * p[l][j]; } }
            m[i][j] = acc;
        }
        m[i][r] = (0..nz).map(|k| p[k][i] * d[k]).sum();
    }
    for col in 0..r {
        let piv = (col..r).max_by(|x, y| m[*x][col].abs().total_cmp(&m[*y][col].abs())).unwrap();
        m.swap(col, piv);
        for row in col + 1..r {
            let f = m[row][col] / m[col][col];
            for c in col..=r { m[row][c] -= f * m[col][c]; }
        }
    }
    let mut x = vec![0f64; r];
    for row in (0..r).rev() {
        x[row] = (m[row][r] - (row + 1..r).map(|c| m[row][c] * x[c]).sum::<f64>()) / m[row][row];
    }
    let s: f64 = (0..nz).map(|k| (0..r).map(|j| p[k][j] * x[j]).sum::<f64>() * dx).sum();
    let ws2 = g * kappa2 * s;
    let big_omega = (1. - ws2 * dt * dt / 2.).acos() / dt;
    (big_omega, -ws2 * dt * dt / (2. * (big_omega * dt).sin()))
}

#[test]
fn graded_oblique_wave_follows_its_own_dispersion_s386() {
    // Le mode (1, 1) de S295 — cuve de 8 × 4 m, h = 4 m, A = 1 cm, 1 s —, sur une colonne graduée.
    let (lx, ly, h, a, g) = (8f64, 4f64, 4f64, 0.01f64, 9.81f64);
    let pi = std::f64::consts::PI;
    let (kx, ky) = (pi / lx, pi / ly);
    for (n, us, nodes) in [(16usize, 2000u64, vec![0usize, 3, 5, 6, 7]), (32, 1000, vec![0, 4, 8, 11, 13, 14, 15])] {
        let dx = lx / n as f64;
        let (nx, ny, nz) = (n, (ly / dx) as usize, (h / dx) as usize);
        // La fonction des essais fins redonne le schéma fin quand chaque couche est un nœud.
        let every: Vec<usize> = (0..nz).collect();
        let (fine, _) = scheme_frequency(kx, ky, dx, nz, g, us as f64 * 1e-6);
        let (again, _) = graded_scheme_frequency(kx, ky, dx, nz, &every, g, us as f64 * 1e-6);
        assert!((again / fine - 1.).abs() < 1e-12, "{again} contre {fine}");
        let (mut v, mut arena) = volume(nx, ny, nz, dx as f32, g as f32);
        v.enable_graded(&mut host(&mut arena), &nodes).unwrap();
        let mode = |i: usize, j: usize| (kx * (i as f64 + 0.5) * dx).cos() * (ky * (j as f64 + 0.5) * dx).cos();
        let eta: Vec<f32> = (0..ny).flat_map(|j| (0..nx).map(move |i| (h + a * mode(i, j)) as f32)).collect();
        v.set_surface(&eta).unwrap();
        let (big_omega, beta) = graded_scheme_frequency(kx, ky, dx, nz, &nodes, g, us as f64 * 1e-6);
        let mut e_scheme = 0f64;
        for step in 1..=1_000_000 / us {
            let r = v.step_surface_linear(us, 4000, &Jobs).unwrap();
            assert!(!r.degraded);
            let t = (step * us) as f64 * 1e-6;
            let phase = big_omega * t;
            for j in 0..ny {
                for i in 0..nx {
                    let h_num = v.eta[v.col(i, j)] as f64 - h;
                    e_scheme = e_scheme.max((h_num - a * mode(i, j) * (phase.cos() + beta * phase.sin())).abs() / a);
                }
            }
        }
        let omega = (g * (kx * kx + ky * ky).sqrt() * ((kx * kx + ky * ky).sqrt() * h).tanh()).sqrt();
        println!("S386 onde oblique (1,1) n={n} nœuds={} sur {nz} : erreur {:.4} % contre Ω du schéma gradué ; \
                  Ω_gradué/Ω_fin − 1 = {:.3e}, Ω_gradué/ω − 1 = {:.3e}", nodes.len(), 100. * e_scheme,
                 big_omega / fine - 1., big_omega / omega - 1.);
        assert!(e_scheme < 1e-3, "n={n} : {e_scheme}");
    }
}

// ---------------------------------------------------------------- S387 : la colonne graduée au pas mobile (C2b)

/// Nœuds du pas mobile de ces essais : gradués sous 3 m, cubiques de la couche 12 (3 m) au haut (6 m).
fn mobile_nodes() -> Vec<usize> {
    let mut n = vec![0, 5, 9];
    n.extend(12..24);
    n
}

#[test]
fn graded_mobile_with_every_layer_is_the_fine_mobile_step_s387() {
    let (nx, ny, nz, dx) = (32, 16, 24, 0.25);
    let (mut a, _) = volume(nx, ny, nz, dx, 9.81);
    let (mut b, mut arena) = volume(nx, ny, nz, dx, 9.81);
    b.enable_graded(&mut host(&mut arena), &(0..nz).collect::<Vec<_>>()).unwrap();
    for v in [&mut a, &mut b] {
        v.set_free_surface(&sine_surface(nx, ny, dx), 4.).unwrap();
    }
    for _ in 0..20 {
        assert!(!a.step_surface_mobile(2000, 20_000, &Jobs).unwrap().degraded);
        assert!(!b.step_surface_mobile(2000, 20_000, &Jobs).unwrap().degraded);
    }
    let worst = a.surface().iter().zip(b.surface()).fold(0f32, |m, (x, y)| m.max((x - y).abs()));
    assert!(worst <= 1e-6, "{worst}");
}

#[test]
fn graded_mobile_keeps_volume_and_refuses_a_surface_below_its_cubic_layers_s387() {
    let (nx, ny, nz, dx) = (32, 16, 24, 0.25);
    let (mut v, mut arena) = volume(nx, ny, nz, dx, 9.81);
    v.enable_graded(&mut host(&mut arena), &mobile_nodes()).unwrap();
    assert_eq!(v.pressure_unknowns_per_column(), 15);
    v.set_free_surface(&sine_surface(nx, ny, dx), 4.).unwrap();
    let start = v.perturbation_volume();
    for _ in 0..30 {
        let r = v.step_surface_mobile(2000, 20_000, &Jobs).unwrap();
        assert!(!r.degraded && r.divergence_plain <= PROJECTION_DIVERGENCE_TOLERANCE, "{r:?}");
    }
    assert!((v.perturbation_volume() - start).abs() < 1e-9, "{} contre {start}", v.perturbation_volume());
    // Une surface à 3,1 m : la couche 12 (centre à 3,125 m) est sèche — refus, état rendu au bit.
    let low: Vec<f32> = vec![3.1; nx * ny];
    v.set_free_surface(&low, 3.1).unwrap();
    let before: Vec<u32> = v.u.iter().chain(&v.eta).map(|x| x.to_bits()).collect();
    assert_eq!(v.step_surface_mobile(2000, 20_000, &Jobs).err(), Some(Error::Domain));
    assert!(v.u.iter().chain(&v.eta).map(|x| x.to_bits()).eq(before));
}

// ── S391 : l'advection au second ordre en temps (A321, ADR-209) ──────────────────────────────────────────────────────

/// Advection seule, sans projection, d'un courant de 2 m/s (Courant 0,264), avec ou sans un mode de quatre mailles de
/// 1 mm/s ; rend `u` après `pas` pas.
fn advection_seule_s391(corrected: bool, amplitude: f32, pas: usize) -> (Volume3, Vec<f32>) {
    let (mut v, _arena) = volume(128, 4, 6, 0.25, 9.81);
    if corrected {
        v.enable_advection_correction();
    }
    let d = v.domain;
    let mut u = vec![0f32; v.u.len()];
    for k in 0..d.nz {
        for j in 0..d.ny {
            for i in 1..d.nx {
                u[v.fu(i, j, k)] = 2. + amplitude * (core::f32::consts::FRAC_PI_2 * i as f32).sin();
            }
        }
    }
    let (vv, ww) = (vec![0f32; v.v.len()], vec![0f32; v.w.len()]);
    v.set_velocity(&u, &vv, &ww).expect("vitesses");
    for _ in 0..pas {
        v.advect_mobile3(0.033);
        v.correct_advection3(None, 0.033);
        v.u.copy_from_slice(&v.us);
        v.v.copy_from_slice(&v.vs);
        v.w.copy_from_slice(&v.ws);
    }
    let u = v.u.clone();
    (v, u)
}

/// Croissance du mode de quatre mailles sur `pas` pas : la **différence** de deux passages, avec et sans le mode, isole la
/// perturbation de ce que les murs font au courant ; amplitude lue au milieu du domaine, en `sin`/`cos(π·i/2)`.
fn croissance_s391(corrected: bool, pas: usize) -> f64 {
    let (v, avec) = advection_seule_s391(corrected, 1e-3, pas);
    let (_, sans) = advection_seule_s391(corrected, 0., pas);
    let Domain3 { nx, ny, .. } = v.domain;
    let (j, k) = (ny / 2, 2);
    let (mut s, mut c) = (0f64, 0f64);
    for i in 3 * nx / 8..5 * nx / 8 {
        let f = v.fu(i, j, k);
        let x = (avec[f] - sans[f]) as f64;
        let ph = core::f64::consts::FRAC_PI_2 * i as f64;
        s += x * ph.sin();
        c += x * ph.cos();
    }
    // Amplitude initiale : 1e-3 sur nx/4 faces, projection sur sin → 1e-3·(nx/4)/2.
    (s * s + c * c).sqrt() / (1e-3 * (nx / 4) as f64 / 2.)
}

/// **S391, A321** : le schéma centré à pas explicite fait croître un mode de quatre mailles (Courant 0,264 : `|G|` =
/// 1,034 par pas, ×7,4 en 60 pas) — vu échouer ; avec le terme de second ordre il ne croît plus (`|G|² = 1 − C² + C⁴`,
/// ×0,13 en 60 pas au mode de quatre mailles). Éteint par défaut : le pas d'avant.
#[test]
fn second_order_advection_stops_the_ftcs_growth_s391() {
    let ftcs = croissance_s391(false, 60);
    let corrected = croissance_s391(true, 60);
    eprintln!("S391 mode de quatre mailles, 60 pas : FTCS ×{ftcs:.3}, corrigé ×{corrected:.4}");
    assert!(ftcs > 5. && ftcs < 10., "FTCS : ×{ftcs}, ×7,4 attendus");
    assert!(corrected < 0.5, "le mode croît encore : ×{corrected}");
    assert!(!volume(8, 4, 6, 0.25, 9.81).0.advection_correction());
}

/// La distance signée d'une cloison `|x − xc| ≤ e`, toute la largeur, de `z0` à `z1`, aux nœuds d'une grille `nx × ny × nz`.
fn noeuds_cloison(nx: usize, ny: usize, nz: usize, dx: f64, xc: f64, e: f64, z0: f64, z1: f64) -> Vec<f32> {
    let mut out = Vec::with_capacity((nx + 1) * (ny + 1) * (nz + 1));
    for k in 0..=nz {
        for _j in 0..=ny {
            for i in 0..=nx {
                let (x, z) = (i as f64 * dx, k as f64 * dx);
                let q = [(x - xc).abs() - e, (z - 0.5 * (z0 + z1)).abs() - 0.5 * (z1 - z0)];
                let dehors = (q[0].max(0.).powi(2) + q[1].max(0.).powi(2)).sqrt();
                out.push((dehors + q[0].max(q[1]).min(0.)) as f32);
            }
        }
    }
    out
}

/// **S490 (liste 6.5) — un décor fixe qui perce la surface, éprouvé comme tel.** Une cuve de 2,4 m sur 0,8 m d'eau, une cloison fixe de
/// trois mailles au milieu (sa paroi en milieu de maille), du fond jusqu'au-dessus du couvercle. (1) Au repos, le repos au bit. (2) Une seiche dans la moitié gauche
/// (`η = a·cos(π·x/L)`) : la moitié droite reste au repos — aucune eau ne traverse le décor. (3) La période de la demi-cuve contre la
/// dispersion linéaire, `ω² = g·k·tanh(k·h)`, à 3 %.
#[test]
fn a_fixed_wall_through_the_surface_splits_the_tank_s490() {
    let (nx, ny, nz, dx) = (48usize, 4usize, 16usize, 0.05f32);
    // La cloison coupe les mailles (sa paroi en milieu de maille) ; alignée sur la grille, voir l'essai suivant (A327).
    let (h, xc, e) = (nz as f64 * dx as f64, 1.2f64, 0.075f64);
    // Diagnostic S490 : `CLOISON_Z0` (bas de la cloison), `CLOISON_E` (demi-épaisseur ; négative : aucune cloison).
    let z0: f64 = std::env::var("CLOISON_Z0").ok().and_then(|v| v.parse().ok()).unwrap_or(-1.0);
    let e: f64 = std::env::var("CLOISON_E").ok().and_then(|v| v.parse().ok()).unwrap_or(e);
    let solide = noeuds_cloison(nx, ny, nz, dx as f64, xc, e, z0, h + 1.0);
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let d = Domain3 { nx, ny, nz, dx };
    let config = Volume3::configure_with_floating_solid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, d,
        1000., 9.81, &vec![0.; nx * ny], &solide);
    let mut v = match config {
        Ok(v) => v,
        Err(err) => panic!("la cloison posée sur le fond est refusée : {err:?}"),
    };
    // (1) Le repos.
    v.set_surface(&vec![h as f32; nx * ny]).unwrap();
    for pas in 0..50 {
        v.step_surface_linear(2000, 4000, &Jobs).unwrap();
        assert!(v.surface().iter().all(|x| x.to_bits() == (h as f32).to_bits()), "repos, pas {pas}");
    }
    // (2) et (3) La seiche de la moitié gauche : les colonnes d'eau à gauche de la cloison, `x < xc − e`.
    let lg = xc - e;
    let a = 0.01f64;
    let k = core::f64::consts::PI / lg;
    let eta0: Vec<f32> = (0..nx * ny)
        .map(|c| {
            let x = ((c % nx) as f64 + 0.5) * dx as f64;
            if x < lg { (h + a * (k * x).cos()) as f32 } else { h as f32 }
        })
        .collect();
    v.set_surface(&eta0).unwrap();
    let periode = 2. * core::f64::consts::PI / (9.81 * k * (k * h).tanh()).sqrt();
    let (pas_us, n_pas) = (2000u64, (3.2 * periode / 0.002) as usize);
    let (mut droite_max, mut serie) = (0f64, Vec::with_capacity(n_pas));
    for p in 0..n_pas {
        let r = match v.step_surface_linear(pas_us, 100_000, &Jobs) {
            Ok(r) => r,
            Err(err) => {
                let s = v.surface();
                let (lo, hi) = s.iter().fold((f32::MAX, f32::MIN), |m, x| (m.0.min(*x), m.1.max(*x)));
                panic!("pas {p} ({:.3} s) : {err:?} ; surface de {lo} à {hi}", (p + 1) as f64 * 0.002);
            }
        };
        if p < 3 {
            println!("S490 : pas {p}, {} itérations", r.iterations);
        }
        let s = v.surface();
        for c in 0..nx * ny {
            let x = ((c % nx) as f64 + 0.5) * dx as f64;
            if x > xc + e {
                droite_max = droite_max.max((s[c] as f64 - h).abs());
            }
        }
        serie.push(((p + 1) as f64 * 0.002, s[0] as f64 - h));
    }
    // La période : les passages par zéro vers le bas de la colonne du bord gauche.
    let mut passages = Vec::new();
    for w in serie.windows(2) {
        if w[0].1 > 0. && w[1].1 <= 0. {
            passages.push(w[0].0 + (w[1].0 - w[0].0) * w[0].1 / (w[0].1 - w[1].1));
        }
    }
    let mesuree = (passages[passages.len() - 1] - passages[0]) / (passages.len() - 1) as f64;
    println!("S490 : periode {mesuree:.4} s contre {periode:.4} s (ecart {:.2e}), droite au pire {droite_max:e} m, {} passages",
        (mesuree / periode - 1.).abs(), passages.len());
    assert!(droite_max <= 1e-6, "de l'eau traverse la cloison : {droite_max}");
    assert!((mesuree / periode - 1.).abs() <= 0.03, "période {mesuree} contre {periode}");
}

/// **S490 — A327** : la même cloison, sa paroi **exactement sur un plan de la grille**. En S490 la projection se disait dégradée à la
/// demi-période de la seiche ; S492 l'a attribué à la tolérance de divergence, relative à la vitesse maximale, qui passe près de zéro au
/// point mort (1,1·10⁻⁴ m/s) — non à la géométrie. Avec le plancher de vitesse (ADR-225), l'essai passe.
#[test]
fn a_grid_aligned_wall_through_the_surface_a327_s490() {
    std::env::set_var("CLOISON_E", "0.05");
    a_fixed_wall_through_the_surface_splits_the_tank_s490();
}



/// **S505 — C23 sur le système** (CAS-CANONIQUES C23, ADR-035, ADR-229). Eau au repos, 2 m ; la coque de la porte D, qui perce la surface,
/// mise en mouvement à `u_p` de 0,5 à 20 m/s. (1) Sous la borne **gouvernante** (`courant_bound`), le Courant réalisé (`courant`, la même
/// vitesse) vaut `ν` = 0,45 au millième, et la paroi franchit moins de `ν` maille par pas ; (2) sous la borne **absolue** (`c` et la vitesse
/// absolue du fluide, nulle au repos), il vaut `ν·(u_p + c)/c` et dépasse 1 exactement au-delà de `u_p = c·(1/ν − 1)` ; (3) borne et compteur
/// dérivent de `governing_speed`, une seule fonction.
#[test]
fn the_courant_number_sees_the_moving_wall_c23_s505() {
    use crate::rigid_body::oriented_box_distance;
    let d = Domain3 { nx: 32, ny: 16, nz: 8, dx: 0.25 };
    let z_r = 0.5 - 500. / 1025.;
    let c = [4.1f64, 1.875, d.z0() as f64 + z_r];
    let mut noeuds = Vec::with_capacity((d.nx + 1) * (d.ny + 1) * (d.nz + 1));
    for k in 0..=d.nz {
        for j in 0..=d.ny {
            for i in 0..=d.nx {
                let p = [i as f64 * 0.25, j as f64 * 0.25, k as f64 * 0.25];
                noeuds.push(oriented_box_distance(c, [1., 0., 0., 0.], [2., 0.8, 0.5], p) as f32);
            }
        }
    }
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut v = Volume3::configure_with_floating_solid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, d, 1025., 9.81,
        &vec![0.; d.columns()], &noeuds).unwrap();
    v.set_surface(&vec![d.z0(); d.columns()]).unwrap();
    let nu = 0.45f64;
    let cel = v.celerity() as f64;
    let seuil = cel * (1. / nu - 1.);
    println!("S505 : c = {cel:.4} m/s, seuil analytique u_p = {seuil:.4} m/s");
    for u_p in [0.5f32, 1., 2., 5., 5.41, 5.42, 10., 20.] {
        v.set_solid_rigid(&noeuds, [u_p, 0., 0.], [0.; 3], c.map(|x| x as f32)).unwrap();
        let dt_g = v.courant_bound(nu, u_p);
        let realise_g = v.courant(dt_g, u_p);
        let franchi = u_p as f64 * dt_g / 0.25;
        // La borne absolue : la célérité et la vitesse absolue du fluide — la paroi n'y entre pas.
        // Le fluide est au repos : sa vitesse absolue, la seule que la borne absolue voie, est nulle.
        let dt_a = nu * 0.25 / cel;
        let realise_a = v.courant(dt_a, u_p);
        println!("S505 u_p = {u_p:5.2} : gouvernante dt {dt_g:.5} s, Courant {realise_g:.4}, mailles franchies {franchi:.3} ; absolue dt {dt_a:.5} s, Courant {realise_a:.4}");
        assert!((realise_g - nu).abs() <= 1e-3, "critère 1 : {realise_g}");
        assert!(franchi < nu, "critère 1 : {franchi}");
        assert!((realise_a - nu * (u_p as f64 + cel) / cel).abs() <= 1e-6, "critère 2 : {realise_a}");
        assert_eq!(realise_a > 1., (u_p as f64) > seuil, "critère 2, seuil : u_p {u_p}");
    }
}


/// **S508 — le recoupage limité à la boîte du solide rend les tableaux du recoupage entier, au bit.** Deux volumes, l'un recoupé sur toute
/// la grille (`set_full_recut`), l'autre dans la boîte du solide ; la coque de la porte D qui pilonne, roule et avance à 2 m/s, 60 pas de
/// 10 ms : après chaque recoupage, ouvertures, fractions, terme de paroi, faces, colonnes et poids du transfert ; après chaque pas, surface
/// et vitesses — identiques.
#[test]
fn the_boxed_recut_is_the_full_recut_s508() {
    use crate::rigid_body::oriented_box_distance;
    let d = Domain3 { nx: 40, ny: 16, nz: 8, dx: 0.25 };
    let z_r = 0.5 - 500. / 1025.;
    let pose = |t: f64| {
        let roulis = 0.05 * (2.5 * t).sin();
        ([3.1 + 2. * t, 1.875 + 0.1 * (1.3 * t).sin(), d.z0() as f64 + z_r + 0.05 * (3.5 * t).sin()], [(0.5 * roulis).cos(), (0.5 * roulis).sin(), 0., 0.])
    };
    let noeuds = |t: f64| -> Vec<f32> {
        let (c, q) = pose(t);
        let mut out = Vec::with_capacity((d.nx + 1) * (d.ny + 1) * (d.nz + 1));
        for k in 0..=d.nz {
            for j in 0..=d.ny {
                for i in 0..=d.nx {
                    out.push(oriented_box_distance(c, q, [2., 0.8, 0.5], [i as f64 * 0.25, j as f64 * 0.25, k as f64 * 0.25]) as f32);
                }
            }
        }
        out
    };
    let cree = || {
        let mut arena = Arena { stats: AllocStats::default(), sealed: false };
        let mut v = Volume3::configure_with_floating_solid(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, d, 1025., 9.81,
            &vec![0.; d.columns()], &noeuds(0.)).unwrap();
        v.set_surface(&vec![d.z0(); d.columns()]).unwrap();
        v
    };
    let (mut entier, mut boite) = (cree(), cree());
    entier.set_full_recut(true);
    let bits = |x: &[f32]| x.iter().map(|v| v.to_bits()).collect::<Vec<u32>>();
    let mut tampons = [vec![0f32; d.cells()], vec![0f32; d.cells()]];
    let nf = (d.nx + 1) * d.ny * d.nz + d.nx * (d.ny + 1) * d.nz + d.nx * d.ny * (d.nz + 1);
    let mut faces = [vec![0f32; nf], vec![0f32; nf]];
    let mut poids = [vec![0f32; 5 * d.columns()], vec![0f32; 5 * d.columns()]];
    // S509 : `changed_faces_in_place`, parti de la géométrie, rend chaque pas le tableau entier.
    let (ou, ov, ow) = boite.apertures().unwrap();
    let mut en_place: Vec<f32> = ou.iter().chain(ov).chain(ow).map(|a| if *a == 0. { 0. } else { f32::MAX }).collect();
    for n in 1..=60 {
        let t = n as f64 * 0.01;
        let (c, _) = pose(t);
        let (cp, _) = pose(t - 0.01);
        let vitesse = [((c[0] - cp[0]) / 0.01) as f32, ((c[1] - cp[1]) / 0.01) as f32, ((c[2] - cp[2]) / 0.01) as f32];
        let angulaire = [(0.05 * 2.5 * (2.5 * t).cos()) as f32, 0., 0.];
        let nds = noeuds(t);
        for (k, v) in [&mut entier, &mut boite].into_iter().enumerate() {
            v.set_solid_rigid(&nds, vitesse, angulaire, c.map(|x| x as f32)).unwrap();
            v.wall_divergence(&mut tampons[k]).unwrap();
            v.changed_faces(&mut faces[k]).unwrap();
            v.lid_transfer_weights(&mut poids[k]).unwrap();
        }
        let (ae, ab) = (entier.apertures().unwrap(), boite.apertures().unwrap());
        assert_eq!((bits(ae.0), bits(ae.1), bits(ae.2)), (bits(ab.0), bits(ab.1), bits(ab.2)), "ouvertures, pas {n}");
        assert_eq!(bits(entier.fluid_fraction().unwrap()), bits(boite.fluid_fraction().unwrap()), "fractions, pas {n}");
        assert_eq!(bits(&tampons[0]), bits(&tampons[1]), "terme de paroi, pas {n}");
        assert_eq!(bits(&faces[0]), bits(&faces[1]), "faces, pas {n}");
        boite.changed_faces_in_place(&mut en_place).unwrap();
        assert_eq!(bits(&en_place), bits(&faces[1]), "faces en place, pas {n}");
        assert_eq!(bits(&poids[0]), bits(&poids[1]), "transfert, pas {n}");
        assert_eq!(bits(entier.solid_column_volumes().unwrap()), bits(boite.solid_column_volumes().unwrap()), "colonnes, pas {n}");
        for v in [&mut entier, &mut boite] {
            v.step_surface_linear(10_000, 8000, &Jobs).unwrap();
        }
        assert_eq!(bits(entier.surface()), bits(boite.surface()), "surface, pas {n}");
        // S518 : les vitesses des faces aussi, le recoupage limité jusque dans les faces qui s'ouvrent.
        assert_eq!(
            (bits(entier.velocity_u()), bits(entier.velocity_v()), bits(entier.velocity_w())),
            (bits(boite.velocity_u()), bits(boite.velocity_v()), bits(boite.velocity_w())),
            "vitesses, pas {n}"
        );
    }
    println!("S508 : 60 pas, recoupage en boîte et entier identiques au bit");
}

/// **S542 — C16, le référentiel accéléré** (liste 4.17). Une cuve de 8 m et 1,5 m d'eau (32 × 1 × 6 mailles de 25 cm), pas de 10 ms.
/// (2) Sous 0,05 g latéral, partie plate, la pente moyenne de la surface sur quatre périodes à 2 % de `g_h/g` (perpendiculaire à `g_eff`).
/// (3) Sans pesanteur horizontale, le premier mode lâché en cosinus : sa période à 1 % de `2π/√(g·(π/L)·tanh(πh/L))` = 4,40 s.
#[test]
fn a_tank_in_an_accelerated_frame_tilts_and_sloshes_s542() {
    let (n, dx, h, g) = (32usize, 0.25f32, 1.5f64, 9.81f64);
    let pente = |eta: &[f32]| {
        let (mut sx, mut sy, mut sxx, mut sxy) = (0f64, 0f64, 0f64, 0f64);
        for (i, e) in eta.iter().enumerate() {
            let x = (i as f64 + 0.5) * dx as f64;
            sx += x;
            sy += *e as f64;
            sxx += x * x;
            sxy += x * *e as f64;
        }
        let m = eta.len() as f64;
        (m * sxy - sx * sy) / (m * sxx - sx * sx)
    };
    let k = std::f64::consts::PI / 8.;
    let periode = 2. * std::f64::consts::PI / (g * k * (k * h).tanh()).sqrt();
    // (2) La pente sous 0,05 g.
    let (mut v, _) = volume(n, 1, 6, dx, g as f32);
    v.set_surface(&vec![1.5; n]).unwrap();
    v.set_horizontal_gravity([(0.05 * g) as f32, 0.]).unwrap();
    let pas = (4. * periode / 0.01).round() as usize;
    let mut somme = 0f64;
    for _ in 0..pas {
        v.step_surface_linear(10_000, 2000, &Jobs).unwrap();
        somme += pente(v.surface());
    }
    let moyenne = somme / pas as f64;
    // (3) La période du premier mode.
    let (mut w, _) = volume(n, 1, 6, dx, g as f32);
    let eta0: Vec<f32> = (0..n).map(|i| 1.5 + 0.01 * (std::f64::consts::PI * (i as f64 + 0.5) / n as f64).cos() as f32).collect();
    w.set_surface(&eta0).unwrap();
    let (mut avant, mut passages) = (w.surface()[0] as f64 - 1.5, Vec::new());
    for s in 1..=3000 {
        w.step_surface_linear(10_000, 2000, &Jobs).unwrap();
        let e = w.surface()[0] as f64 - 1.5;
        if avant > 0. && e <= 0. || avant < 0. && e >= 0. {
            passages.push((s as f64 - 1. + avant / (avant - e)) * 0.01);
        }
        avant = e;
    }
    let mesure = 2. * (passages[passages.len() - 1] - passages[0]) / (passages.len() - 1) as f64;
    println!(
        "S542 C16 : pente moyenne {moyenne:.5} (g_h/g = 0,05, écart {:.2e}, {:.3}°) ; période {mesure:.4} s (formule {periode:.4} s, écart {:.2e})",
        moyenne / 0.05 - 1., (moyenne.atan() - 0.05f64.atan()).to_degrees(), mesure / periode - 1.
    );
    assert!((moyenne / 0.05 - 1.).abs() <= 0.02, "critère 2");
    assert!((mesure / periode - 1.).abs() <= 0.01, "critère 3");
    assert_eq!(v.set_horizontal_gravity([f32::NAN, 0.]), Err(Error::NotFinite));
}

/// **S543 — la rotation de C16** : une cuve de 20 m et 2 m d'eau (80 × 1 × 8 mailles) à 100 m de l'axe d'une station tournante (g = Ω²R) ;
/// la pesanteur horizontale `Ω²·(x − 10 m)`. La surface moyenne sur quatre périodes, ajustée par une parabole : sa courbure à 2 % de `1/R`.
#[test]
fn a_tank_on_a_rotating_station_takes_the_cylindrical_surface_s543() {
    let (n, dx, g, r) = (80usize, 0.25f32, 9.81f64, 100f64);
    let (mut v, _) = volume(n, 1, 8, dx, g as f32);
    v.set_surface(&vec![2.0; n]).unwrap();
    v.set_horizontal_gravity_field([0.; 2], (g / r) as f32, [10., 0.]).unwrap();
    let k = std::f64::consts::PI / 20.;
    let periode = 2. * std::f64::consts::PI / (g * k * (k * 2.0f64).tanh()).sqrt();
    let pas = (4. * periode / 0.01).round() as usize;
    let mut moyenne = vec![0f64; n];
    for _ in 0..pas {
        v.step_surface_linear(10_000, 2000, &Jobs).unwrap();
        for (m, e) in moyenne.iter_mut().zip(v.surface()) {
            *m += *e as f64 / pas as f64;
        }
    }
    // La parabole `a·x'² + c`, x' = x − 10 (symétrique) : moindres carrés sur x'².
    let (mut s1, mut s2, mut sy, mut s2y) = (0f64, 0f64, 0f64, 0f64);
    for (i, e) in moyenne.iter().enumerate() {
        let q = ((i as f64 + 0.5) * dx as f64 - 10.).powi(2);
        s1 += 1.;
        s2 += q;
        sy += e;
        s2y += q * e;
    }
    let s22: f64 = moyenne.iter().enumerate().map(|(i, _)| ((i as f64 + 0.5) * dx as f64 - 10.).powi(4)).sum();
    let a = (s1 * s2y - s2 * sy) / (s1 * s22 - s2 * s2);
    println!("S543 C16 rotation : courbure {:.6} m⁻¹ (1/R = {:.6}), écart {:.2e} ; flèche sur 20 m {:.4} m (0,500)", 2. * a, 1. / r, 2. * a * r - 1., a * 100.);
    assert!((2. * a * r - 1.).abs() <= 0.02, "critère 2");
}

/// Le compartiment de C17 (5 m² × 2 m, plafond à la flottaison, brèche d'1 dm² à son fond), ouvert, 20 s sous `g_eff` ; avec δ : un domaine
/// de 2,5 × 2 × 2 m (10 × 8 × 8 mailles) au-dessus du nœud, dont la surface monte à chaque pas de V de l'eau entrée (répartie sur ses
/// colonnes), puis qui avance de dix pas linéaires. Rend `volume_ml` du compartiment à chaque pas.
fn inondation_s544(g_eff: [f32; 3], avec_delta: bool) -> Vec<i64> {
    use crate::hydro_network::{step, Flow, HydroNode, Opening, Shapes, SHAPE_ENTRIES, SHARP_EDGE_DISCHARGE, STEP_US};
    let prism = |h: i64| -> [i64; SHAPE_ENTRIES] { core::array::from_fn(|i| h * i as i64 / (SHAPE_ENTRIES - 1) as i64) };
    let mut table = prism(20_000_000).to_vec();
    table.extend_from_slice(&prism(2_000_000));
    let shapes = Shapes::new(&table).unwrap();
    let mut nodes = [
        HydroNode { volume_ml: 500_000_000_000, capacity_ml: 1_000_000_000_000, origin_um: [0, 0, -10_000_000], shape: 0 },
        HydroNode { volume_ml: 0, capacity_ml: 10_000_000, origin_um: [0, 0, -2_000_000], shape: 1 },
    ];
    let mut edges = [Opening { from: 0, to: Some(1), flow: Flow::Orifice { area_mm2: 10_000 }, position_um: [0, 0, -2_000_000],
        discharge: SHARP_EDGE_DISCHARGE, ..Default::default() }];
    let mut scratch = [0i64; 1];
    let mut delta = avec_delta.then(|| {
        let (mut v, _) = volume(10, 8, 8, 0.25, -g_eff[2]);
        // Le pas mobile veut une surface sous le haut du domaine : l'eau à 50 cm sous lui, le repos au même niveau.
        let niveau = v.domain().z0() - 0.5;
        v.set_surface(&vec![niveau; 80]).unwrap();
        v.shift_rest(niveau).unwrap();
        (v, niveau)
    });
    let mut volumes = Vec::new();
    for _ in 0..200 {
        let avant = nodes[1].volume_ml;
        step(&mut nodes, &mut edges, &shapes, g_eff, crate::SimTime(STEP_US), &mut scratch).unwrap();
        if let Some((v, repos)) = delta.as_mut() {
            // L'eau entrée, répartie sur les colonnes (le domaine substitutif suit le niveau de V, comme la piscine de S375 ; une seule
            // colonne recevant 3,9 L par pas, la projection refuse) ; δ n'écrit rien dans V (ADR-025).
            let dh = ((nodes[1].volume_ml - avant) as f64 * 1e-6 / 5.0) as f32;
            v.add_column_volume(&vec![dh; 80]).unwrap();
            // S375 : le repos suit le niveau de V — la pression de δ reste la perturbation (un décalage de quelques millimètres sur le
            // repos d'origine consomme la précision f32 et fait refuser un pas calme).
            *repos += dh;
            v.shift_rest(*repos).unwrap();
            for _ in 0..10 {
                v.step_surface_mobile(10_000, 2000, &Jobs).unwrap();
            }
        }
        volumes.push(nodes[1].volume_ml);
    }
    volumes
}

/// **S544 — C21, la masse d'un compartiment avec et sans δ** (ADR-025 §4). Sans puis avec δ, en référentiel fixe : `volume_ml` identique à
/// l'entier près à chaque pas.
#[test]
fn a_compartment_holds_the_same_mass_with_and_without_delta_s544() {
    // Le référentiel accéléré n'est pas joué : sous un `g_eff` incliné, les tables de forme « +Z » de V refusent (`Orientation`) — il
    // faut des formes volumiques (tétraèdres), dont une mer de taille réaliste frôle le débordement des entiers en µm³.
    for g_eff in [[0.0f32, 0.0, -9.81]] {
        let (sans, avec) = (inondation_s544(g_eff, false), inondation_s544(g_eff, true));
        assert_eq!(sans, avec, "C21 sous g_eff {g_eff:?}");
        println!("S544 C21 sous g_eff {g_eff:?} : {} pas identiques à l'entier, {} ml entrés en 20 s", sans.len(), sans.last().unwrap());
        assert!(*sans.last().unwrap() > 100_000, "le compartiment s'inonde vraiment");
    }
}

