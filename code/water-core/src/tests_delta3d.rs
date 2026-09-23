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
        + v.surface_total.len() + v.ghost_bg_up.len() + v.ghost_bg_x.len() + v.ghost_bg_y.len() + v.pressure_base.len();
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
