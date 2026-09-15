use super::*;
use crate::host::{AllocError, AllocStats, Allocator, HostServices, JobSystem, Sink};

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
        // Découpe réelle, fusion **dans l'ordre des indices** : c'est la propriété que
        // SPEC-004 §8.2 exige, et la tester ici vaut mieux que la supposer.
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

/// Fond non plat : une bosse continue et une marche, pour que le découpage ait du travail.
fn bottom(nx: usize, dx: f32) -> Vec<f32> {
    (0..nx)
        .map(|i| {
            let x = i as f32 * dx;
            let bump = 0.6 * (-(((x - 3.0) / 1.2) * ((x - 3.0) / 1.2))).exp();
            let step = if i > 3 * nx / 4 { 0.35 } else { 0. };
            0.4 + bump + step
        })
        .collect()
}

fn build(nx: usize, nz: usize, dx: f32, g: f32) -> (Volume, Arena) {
    let mut arena = Arena {
        stats: AllocStats::default(),
        sealed: false,
    };
    let jobs = Jobs;
    let b = bottom(nx, dx);
    let v = Volume::configure(
        &mut HostServices {
            alloc: &mut arena,
            jobs: &jobs,
            sink: &jobs,
        },
        Domain { nx, nz, dx },
        1025.0,
        g,
        &b,
    )
    .unwrap();
    (v, arena)
}

/// **Filtre 1 d'ADR-038 §4, la réception qui décide.** Le lac au repos sur un fond non plat
/// ne bouge pas — pas « peu » : **pas du tout, en bits**, et pour mille pas. C'est ce que
/// « par construction » veut dire : la gravité n'entre jamais dans la mise à jour de la
/// quantité de mouvement, donc aucune annulation n'est demandée à deux termes discrets.
#[test]
fn lake_at_rest_over_a_cut_bottom_does_not_move_at_all() {
    let jobs = Jobs;
    let (mut v, _) = build(48, 20, 0.25, 9.81);
    for _ in 0..1000 {
        let r = v.step(0.01, 200, &jobs).unwrap();
        assert_eq!(r.iterations, 0, "au repos, aucune itération n'est due");
        assert!(!r.degraded);
    }
    for x in v.velocity_u().iter().chain(v.velocity_w()) {
        assert_eq!(x.to_bits(), 0f32.to_bits(), "le lac a bougé de {x}");
    }
    assert!(v.pressure().iter().all(|p| *p == 0.));
}

/// Le repos est exact **sous une autre gravité** : `g_eff` est fournie, et rien n'est codé
/// en dur. ADR-007 §2 disqualifie d'emblée un `−9,81·Z` écrit dans le schéma.
#[test]
fn rest_is_exact_under_lunar_gravity_too() {
    let jobs = Jobs;
    for g in [1.62f32, 9.81, 24.79] {
        let (mut v, _) = build(32, 16, 0.25, g);
        for _ in 0..50 {
            v.step(0.01, 200, &jobs).unwrap();
        }
        assert!(v
            .velocity_u()
            .iter()
            .chain(v.velocity_w())
            .all(|x| x.to_bits() == 0f32.to_bits()));
    }
}

/// **I-06.** Après `seal()`, plus une seule allocation d'hôte n'est due — ni par un pas
/// complet, ni par un pas dégradé. Le scellement rend l'invariant mécanique (`host.rs`).
#[test]
fn no_host_allocation_after_seal() {
    let jobs = Jobs;
    let (mut v, mut arena) = build(24, 12, 0.25, 9.81);
    let before = arena.stats();
    arena.seal();
    let eta: Vec<f32> = (0..24).map(|i| v.domain().z0() + 0.02 * (i as f32).sin()).collect();
    v.set_surface(&eta).unwrap();
    v.step(0.005, 300, &jobs).unwrap();
    v.step(0.005, 1, &jobs).unwrap();
    let after = arena.stats();
    assert_eq!(after.persistent_calls, before.persistent_calls);
    assert_eq!(after.persistent_bytes, before.persistent_bytes);
    assert_eq!(after.refused_after_seal, 0);
}

/// Une surface non plate met le fluide en mouvement, et **proportionnellement à `g_eff`** :
/// c'est par là que la gravité agit, et cela rend le contrôle précédent non vide.
#[test]
fn a_tilted_surface_drives_flow_in_proportion_to_gravity() {
    let jobs = Jobs;
    let mut peak = Vec::new();
    for g in [4.905f32, 9.81] {
        let (mut v, _) = build(32, 16, 0.25, g);
        let z0 = v.domain().z0();
        let eta: Vec<f32> = (0..32).map(|i| z0 + 0.01 * (i as f32 / 31. - 0.5)).collect();
        v.set_surface(&eta).unwrap();
        let r = v.step(0.002, 400, &jobs).unwrap();
        assert!(!r.degraded, "résidu {}", r.residual);
        let m = v
            .velocity_u()
            .iter()
            .chain(v.velocity_w())
            .fold(0f32, |a, b| a.max(b.abs()));
        assert!(m > 0., "une surface inclinée doit mettre en mouvement");
        peak.push(m as f64);
    }
    // Sur un premier pas depuis le repos, la réponse est linéaire en `g_eff`.
    let ratio = peak[1] / peak[0];
    assert!((ratio - 2.).abs() < 0.02, "rapport {ratio} attendu 2");
}

/// Le champ projeté est à divergence nulle à la tolérance du solveur, et le solveur le dit.
#[test]
fn projection_removes_divergence_and_reports_it() {
    let jobs = Jobs;
    let (mut v, _) = build(32, 16, 0.25, 9.81);
    let z0 = v.domain().z0();
    let eta: Vec<f32> = (0..32).map(|i| z0 + 0.02 * (i as f32 * 0.3).sin()).collect();
    v.set_surface(&eta).unwrap();
    let r = v.step(0.002, 500, &jobs).unwrap();
    assert!(!r.degraded, "résidu {}", r.residual);
    assert!(r.divergence < 1e-5, "divergence {}", r.divergence);
    assert!(r.iterations > 0);
}

/// Plafond d'itérations, pas réception du budget temporel ADR-007 §2. À une seule
/// itération, le solveur rend la main sans converger **et l'annonce** ; à budget large, il
/// converge. S200 conserve cette réception restreinte ; aucun temps mural n'est mesuré.
#[test]
fn a_tight_budget_degrades_and_says_so() {
    let jobs = Jobs;
    let (mut v, _) = build(32, 16, 0.25, 9.81);
    let z0 = v.domain().z0();
    let eta: Vec<f32> = (0..32).map(|i| z0 + 0.02 * (i as f32 * 0.3).sin()).collect();
    v.set_surface(&eta).unwrap();
    let tight = v.step(0.002, 1, &jobs).unwrap();
    assert!(tight.degraded && tight.iterations == 1);
    let (mut w, _) = build(32, 16, 0.25, 9.81);
    w.set_surface(&eta).unwrap();
    let wide = w.step(0.002, 500, &jobs).unwrap();
    assert!(!wide.degraded && wide.residual < tight.residual);
}

/// Refus atomiques : un domaine vide, un fond de mauvaise longueur ou un pas non fini sont
/// refusés, et rien n'est construit à moitié.
#[test]
fn refusals_are_declared() {
    let jobs = Jobs;
    let mut arena = Arena {
        stats: AllocStats::default(),
        sealed: false,
    };
    let mut host = HostServices {
        alloc: &mut arena,
        jobs: &jobs,
        sink: &jobs,
    };
    let d = Domain { nx: 8, nz: 4, dx: 0.25 };
    assert_eq!(
        Volume::configure(&mut host, Domain { nx: 0, ..d }, 1025., 9.81, &[]).err(),
        Some(Error::Domain)
    );
    assert_eq!(
        Volume::configure(&mut host, d, 1025., 9.81, &[0.1; 3]).err(),
        Some(Error::Shape)
    );
    assert_eq!(
        Volume::configure(&mut host, d, -1., 9.81, &[0.1; 8]).err(),
        Some(Error::NotFinite)
    );
    let (mut v, _) = build(8, 4, 0.25, 9.81);
    assert_eq!(v.step(0., 10, &jobs).err(), Some(Error::NotFinite));
    assert_eq!(v.set_surface(&[1.0; 3]).err(), Some(Error::Shape));
}

/// Le noyau déclare ce qu'il ne fait pas. Les trois `false` sont le résultat de la session,
/// pas un oubli : B3 attend quatre scénarios, et aucun n'est traité.
#[test]
fn caps_declare_what_is_not_supported() {
    let (v, _) = build(8, 4, 0.25, 9.81);
    let c = v.caps();
    assert!(!c.supports_substitutive && !c.supports_air_phase && !c.supports_moving_solid);
    assert!(!c.supports_frame_accel);
    assert_eq!(c.min_dx,None);
    assert_eq!(c.max_dx,None);
    assert_eq!(c.stability_cfl_max,None);
    assert_eq!(c.latency_frames, 0);
}

#[test]
fn true_pressure_residual_is_received_against_independent_f64_rows_s231() {
    for n in [16, 32, 64, 128] {
        let (mut v, _) = build(n, n/2, 8./n as f32, 9.81);
        let eta: Vec<_> = (0..n).map(|i| 4.+0.01*(std::f32::consts::TAU*(i as f32+0.5)/n as f32).sin()).collect();
        v.set_surface(&eta).unwrap();
        let r = v.step(0.002, 20000, &Jobs).unwrap();
        assert!(!r.degraded, "n={n}, {r:?}");
        // Oracle du système linéaire : lignes recomposées en f64 à partir de la géométrie et
        // de l'entrée physique, sans appel à apply/dot/lid ni lecture du résidu CG.
        let (mut residual2, mut rhs2) = (0f64, 0f64);
        let dx = v.domain.dx as f64;
        for k in 0..n/2 { for i in 0..n {
            let c = k*n+i;
            if v.frac[c] == 0. { continue; }
            let p = v.p[c] as f64;
            let mut ap = 0.;
            for (neighbor, opening) in [
                (i.checked_sub(1).map(|j| k*n+j), v.open_u[k*(n+1)+i]),
                ((i+1<n).then_some(k*n+i+1), v.open_u[k*(n+1)+i+1]),
                (k.checked_sub(1).map(|j| j*n+i), v.open_w[k*n+i]),
                ((k+1<n/2).then_some((k+1)*n+i), v.open_w[(k+1)*n+i]),
            ] {
                if let Some(j) = neighbor { if v.frac[j] > 0. { ap += opening as f64*(p-v.p[j] as f64); } }
            }
            let mut rhs = 0.;
            if k+1 == n/2 {
                let a = v.open_w[(k+1)*n+i] as f64;
                ap += 2.*a*p;
                rhs = 2.*a*1025.*(9.81f32 as f64)*(eta[i]-4.) as f64;
            }
            ap /= dx*dx; rhs /= dx*dx;
            residual2 += (rhs-ap)*(rhs-ap); rhs2 += rhs*rhs;
        }}
        let actual = (residual2/rhs2).sqrt();
        assert!(actual <= 1e-6, "n={n} oracle={actual:e} report={:e}",r.residual);
        println!("S231 n={n} vrai_residu_f64={actual:e} rapport_f32={:e}", r.residual);
    }
}

#[test]
fn underflowing_nonzero_rhs_is_not_reported_as_rest_s231() {
    let (mut v, _) = build(8,4,1.,9.81);
    v.rho = 1e-30;
    v.set_surface(&[4.01;8]).unwrap();
    assert_eq!(v.step(0.002,100,&Jobs),Err(Error::NotFinite));
    assert!(v.pressure().iter().all(|p| p.to_bits()==0));
    assert!(v.velocity_w().iter().all(|p| p.to_bits()==0));
}

#[test]
fn thin_cut_wedges_remain_fluid_and_project_their_open_flux_s232() {
    for b in [[1.4, 1., 0.98], [0.98, 1., 1.4]] {
        let mut arena = Arena { stats: AllocStats::default(), sealed: false };
        let mut v = Volume::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
            Domain { nx: 3, nz: 3, dx: 1. }, 1025., 9.81, &b).unwrap();
        // Triangle indépendant : aire = base * hauteur / 2, depuis les arêtes f32 fournies.
        let a = (0.5f32 * (b[0]+b[1])) as f64;
        let z = (0.5f32 * (b[1]+b[2])) as f64;
        let height = 1. - a.min(z);
        let expected = 0.5 * height * height / (a-z).abs();
        let fraction = v.fluid_fraction()[1] as f64;
        assert!(fraction > 0., "triangle perdu : aire={expected:e}");
        assert!((fraction-expected).abs() <= 64.*f32::EPSILON as f64*expected);
        let u: Vec<_> = v.open_u.iter().map(|a| if *a>0. {0.001} else {0.}).collect();
        let w = vec![0.; v.w.len()];
        v.set_velocity(&u,&w).unwrap();
        let r = v.step(0.002,2000,&Jobs).unwrap();
        assert!(!r.degraded, "{r:?}");
        let peak = v.u.iter().chain(&v.w).fold(0f64,|m,x|m.max(x.abs() as f64));
        // Somme indépendante des quatre débits orientés, toutes cellules fluides incluses.
        for k in 0..3 { for i in 0..3 {
            if v.frac[k*3+i] == 0. {continue;}
            let flux = v.open_u[k*4+i+1] as f64*v.u[k*4+i+1] as f64
                -v.open_u[k*4+i] as f64*v.u[k*4+i] as f64
                +v.open_w[(k+1)*3+i] as f64*v.w[(k+1)*3+i] as f64
                -v.open_w[k*3+i] as f64*v.w[k*3+i] as f64;
            assert!(flux.abs()/peak < 1e-5, "i={i} k={k} flux={flux:e}");
        }}
    }
}

#[test]
fn cut_fraction_integrates_clipped_linear_profiles_s232() {
    // Aires géométriques sur [0,1]x[0,1] : plein, vide, rectangle, trapèze,
    // triangle, puis segment traversant les deux hauteurs de la cellule.
    for (a,b,area) in [(0.,0.,1.),(1.,2.,0.),(0.25,0.25,0.75),
        (0.25,0.75,0.5),(0.5,1.5,0.125),(0.,2.,0.25)] {
        for (l,r) in [(a,b),(b,a)] {
            assert_eq!(Volume::cut_fraction(l,r,1.,1.),area);
        }
    }
    // Même aire coupée en deux colonnes : partition additive du trapèze.
    for (a,b) in [(0.2,1.7),(0.7,0.3),(1.4,0.999)] {
        let whole = Volume::cut_fraction(a,b,1.,1.);
        let middle = 0.5*(a+b);
        let split = 0.5*(Volume::cut_fraction(a,middle,1.,1.)+Volume::cut_fraction(middle,b,1.,1.));
        assert!((whole-split).abs() <= 16.*f32::EPSILON*whole);
    }
}

struct StillClock;
impl crate::host::MonotonicClock for StillClock { fn now_ns(&self) -> u64 {0} }

#[test]
fn linear_surface_matches_standing_wave_and_preserves_rest_s233() {
    for g in [9.81f32,1.62] {
        let mut errors = Vec::new();
        for (n,us) in [(16,2000u64),(32,2000),(64,2000),(64,1000)] {
            let mut arena = Arena { stats: AllocStats::default(), sealed:false };
            let mut v = Volume::configure(&mut HostServices {alloc:&mut arena,jobs:&Jobs,sink:&Jobs},
                Domain{nx:n,nz:n/2,dx:8./n as f32},1025.,g,&vec![0.;n]).unwrap();
            let eta:Vec<_>=(0..n).map(|i|4.+0.01*(std::f64::consts::PI*(i as f64+0.5)/n as f64).cos() as f32).collect();
            v.set_surface(&eta).unwrap();
            let initial_sum:f64=eta.iter().map(|v|*v as f64).sum();
            let omega=(g as f64*std::f64::consts::PI/8.*(std::f64::consts::PI*0.5).tanh()).sqrt();
            let mut error=0f64;
            for step in 1..=1_000_000/us {
                let r=v.step_surface_linear(us,2000,1_000_000,&Jobs,&StillClock).unwrap_or_else(|e|panic!("n={n} us={us} step={step} g={g} {e:?}"));
                assert_eq!(r.advanced_us,us);
                let time=(step*us) as f64*1e-6;
                for (i,h) in v.surface().iter().enumerate() {
                    let oracle=0.01*(std::f64::consts::PI*(i as f64+0.5)/n as f64).cos()*(omega*time).cos();
                    error=error.max(((*h as f64-4.)-oracle).abs()/0.01);
                }
            }
            let drift=(v.surface().iter().map(|v|*v as f64).sum::<f64>()-initial_sum).abs()/n as f64;
            println!("S233 g={g} n={n} dt_us={us} error={error:e} mean_drift={drift:e}");
            assert!(drift < 64.*f32::EPSILON as f64*4.);
            errors.push(error);
            if g>9. && n==64 { assert!(v.surface()[0]<4., "retour de signe absent"); }
        }
        assert!(errors[2]<errors[0],"{errors:?}");
        assert!(errors[3]<errors[2],"le raffinement temporel doit reduire l'erreur : {errors:?}");
        assert!(errors[3]<0.01,"{errors:?}");
    }
    let (mut rest,_) = build(8,4,1.,9.81);
    for _ in 0..100 {
        rest.step_surface_linear(2000,100,1_000_000,&Jobs,&StillClock).unwrap();
        assert!(rest.surface().iter().all(|v|*v==4.));
        assert!(rest.velocity_u().iter().chain(rest.velocity_w()).all(|v|v.to_bits()==0));
    }
    rest.g_eff = -1.;
    assert_eq!(rest.step_surface_linear(2000,100,1_000_000,&Jobs,&StillClock),Err(Error::Domain));
    rest.g_eff = 9.81;
    // Une partie sèche du couvercle sort des hypothèses de la surface linéarisée.
    let top=rest.fw(0,rest.domain.nz);
    rest.open_w[top]=0.5;
    assert_eq!(rest.step_surface_linear(2000,100,1_000_000,&Jobs,&StillClock),Err(Error::Domain));
    assert!(rest.surface().iter().all(|v|*v==4.));
}

// ---- S237 : surface géométriquement mobile ---------------------------------------------------

fn mobile_volume(nx: usize, nz: usize, dx: f32, bottom: &[f32]) -> Volume {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    Volume::configure(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs },
        Domain { nx, nz, dx }, 1025., 9.81, bottom).unwrap()
}

#[test]
fn mobile_operator_is_symmetric_with_side_and_top_ghosts_s237() {
    // Bassin 2 × 3 m, surface ondulée qui traverse plusieurs mailles d'une colonne à l'autre.
    let (nx, nz, dx) = (8usize, 12usize, 0.25f32);
    let mut v = mobile_volume(nx, nz, dx, &vec![0.; nx]);
    let eta: Vec<f32> = (0..nx).map(|i| {
        let x = (i as f32 + 0.5) * dx;
        2.0 + 0.5 * (std::f32::consts::PI * x / 2.).cos() + 0.1 * (3. * x).sin()
    }).collect();
    v.set_free_surface(&eta, 2.0).unwrap();
    v.mobile = true;
    let mut ctl = Control::unlimited();
    v.rhs.fill(0.);
    v.rhs_mobile(-1025. / 0.002, &mut ctl).unwrap();
    let cells = nx * nz;
    let wet: Vec<usize> = (0..cells).filter(|c| v.wet_cell(*c)).collect();
    let side_ghosts = wet.iter().filter(|&&c| {
        let (i, k) = (c % nx, c / nx);
        (i > 0 && !v.wet(i - 1, k)) || (i + 1 < nx && !v.wet(i + 1, k))
    }).count();
    assert!(side_ghosts >= 4, "l'essai doit exercer des faces fantômes horizontales : {side_ghosts}");
    let mut columns = vec![vec![0f32; cells]; cells];
    for &j in &wet {
        let mut e = vec![0f32; cells];
        e[j] = 1.;
        v.apply(&e, &mut columns[j], &mut ctl).unwrap();
    }
    for &i in &wet {
        for &j in &wet {
            assert_eq!(columns[j][i].to_bits(), columns[i][j].to_bits(), "A[{i}][{j}] ≠ A[{j}][{i}]");
        }
        assert_eq!(v.prec[i].to_bits(), (1. / columns[i][i]).to_bits(), "diagonale du préconditionneur, maille {i}");
        assert!(columns[i][i] > 0.);
    }
    for c in (0..cells).filter(|c| !v.wet_cell(*c)) {
        assert_eq!((v.rhs[c], v.prec[c]), (0., 0.));
    }
    v.mobile = false;
    // Projection d'un champ quelconque sur cette surface : converge, divergence nulle au fluide.
    let u: Vec<f32> = (0..v.u.len()).map(|f| 0.1 * ((f * 7 % 13) as f32 / 13. - 0.5)).collect();
    let w: Vec<f32> = (0..v.w.len()).map(|f| 0.1 * ((f * 5 % 11) as f32 / 11. - 0.5)).collect();
    v.set_velocity(&u, &w).unwrap();
    let r = v.project_mobile_for_test(0.002, 500, &Jobs).unwrap();
    assert!(!r.degraded && r.divergence < 1e-4, "{r:?}");
}

#[test]
fn mobile_step_keeps_rest_conserves_volume_and_changes_topology_s237() {
    // Repos exact sur plusieurs pas, fond bosselé, niveau non aligné sur les faces.
    let (nx, nz, dx) = (16usize, 20usize, 0.125f32);
    let floor = bottom(nx, dx);
    let mut rest = mobile_volume(nx, nz, dx, &floor);
    rest.set_free_surface(&vec![1.9; nx], 1.9).unwrap();
    for _ in 0..50 {
        rest.step_surface_mobile(1000, 200, 1_000_000, &Jobs, &StillClock).unwrap();
        assert!(rest.surface().iter().all(|h| h.to_bits() == 1.9f32.to_bits()));
        assert!(rest.velocity_u().iter().chain(rest.velocity_w()).all(|x| x.to_bits() == 0));
    }
    // Onde d'amplitude finie, fond plat : volume à l'arrondi, mailles qui entrent et sortent.
    let mut v = mobile_volume(nx, nz, dx, &vec![0.; nx]);
    let wave: Vec<f32> = (0..nx).map(|i| 2.0 + 0.1 * (std::f32::consts::PI * (i as f32 + 0.5) / nx as f32).cos()).collect();
    v.set_free_surface(&wave, 2.0).unwrap();
    let volume0: f64 = v.surface().iter().map(|h| *h as f64).sum();
    let (mut wet_min, mut wet_max) = (usize::MAX, 0);
    for step in 0..400 {
        let r = v.step_surface_mobile(1000, 2000, 1_000_000, &Jobs, &StillClock)
            .unwrap_or_else(|e| panic!("pas {step} : {e:?}"));
        assert_eq!(r.advanced_us, 1000);
        assert!(!v.mobile, "le mode mobile ne doit pas rester armé après un pas");
        let wet = v.wet_cells();
        (wet_min, wet_max) = (wet_min.min(wet), wet_max.max(wet));
    }
    let drift = (v.surface().iter().map(|h| *h as f64).sum::<f64>() - volume0).abs() / nx as f64;
    assert!(drift < 64. * f32::EPSILON as f64 * 2., "dérive de volume {drift:e}");
    assert!(wet_max > wet_min, "aucun changement de topologie : {wet_min}..{wet_max}");
    assert!(v.surface()[0] < 2.05, "la crête initiale doit être redescendue en un quart de période");
}

#[test]
fn mobile_small_amplitude_follows_the_linear_mode_s237() {
    // Même bassin 2×2 m, amplitude 1 mm : l'écart non linéaire vaut ≈0,15 % de a (P3).
    let (nx, dx) = (16usize, 0.125f32);
    let a = 0.001f32;
    let wave: Vec<f32> = (0..nx).map(|i| 2.0 + a * (std::f32::consts::PI * (i as f32 + 0.5) / nx as f32).cos()).collect();
    let mut mobile = mobile_volume(nx, 18, dx, &vec![0.; nx]);
    mobile.set_free_surface(&wave, 2.0).unwrap();
    let mut linear = mobile_volume(nx, 16, dx, &vec![0.; nx]);
    linear.set_surface(&wave).unwrap();
    let mut worst = 0f32;
    for _ in 0..800 {
        mobile.step_surface_mobile(1000, 2000, 1_000_000, &Jobs, &StillClock).unwrap();
        linear.step_surface_linear(1000, 2000, 1_000_000, &Jobs, &StillClock).unwrap();
        for (m, l) in mobile.surface().iter().zip(linear.surface()) {
            worst = worst.max((m - l).abs() / a);
        }
    }
    assert!(worst < 0.01, "écart mobile − linéaire {worst} de a");
}

#[test]
fn mobile_step_refuses_geometry_out_of_bounds_atomically_s237() {
    let (nx, nz, dx) = (8usize, 12usize, 0.25f32);
    let mut v = mobile_volume(nx, nz, dx, &vec![0.; nx]);
    let snapshot = |v: &Volume| -> Vec<u32> {
        v.u.iter().chain(&v.w).chain(&v.p).chain(&v.eta).chain(&v.eta_roundoff).map(|x| x.to_bits()).collect()
    };
    // Trop près du sommet (au-dessus de (nz−1)·dx) puis du fond (moins de deux mailles).
    for level in [2.76f32, 0.49] {
        let mut eta = vec![1.5; nx];
        eta[3] = level;
        v.set_free_surface(&eta, 1.5).unwrap();
        let before = snapshot(&v);
        assert_eq!(v.step_surface_mobile(1000, 200, 1_000_000, &Jobs, &StillClock), Err(Error::Domain));
        assert_eq!(snapshot(&v), before);
        assert!(!v.mobile);
    }
    // Après le pas, sur une dynamique réelle : onde stationnaire dont la crête initiale touche presque
    // le sommet admis ; au demi-cycle, la crête opposée le dépasse (l'harmonique d'ordre deux est
    // positive aux deux murs). La garde d'après pas doit refuser, état intact. Un champ à divergence
    // nulle construit à la main ne convenait pas : son second membre n'est que de l'arrondi, et la
    // pression f32 y plafonnait à un résidu relatif 3,3e-6 (notes S237 P4b).
    let (wx, wz, wdx) = (16usize, 24usize, 0.125f32);
    let mut wave_volume = mobile_volume(wx, wz, wdx, &vec![0.; wx]);
    let top = (wz - 1) as f32 * wdx;
    let crest: Vec<f32> = (0..wx).map(|i| top - 0.05 + 0.05 * (std::f32::consts::PI * (i as f32 + 0.5) / wx as f32).cos()).collect();
    wave_volume.set_free_surface(&crest, top - 0.05).unwrap();
    let mut refused = None;
    for step in 0..1600 {
        let before = snapshot(&wave_volume);
        match wave_volume.step_surface_mobile(1000, 2000, 1_000_000, &Jobs, &StillClock) {
            Ok(_) => {}
            Err(e) => { refused = Some((step, e)); assert_eq!(snapshot(&wave_volume), before); break; }
        }
    }
    let (step, error) = refused.expect("la crête devait dépasser le sommet admis");
    assert_eq!(error, Error::Domain, "pas {step}");
    assert!(wave_volume.surface().iter().all(|h| *h <= top), "l'état publié reste dans les bornes");
    assert!(step > 400, "le dépassement vient de la dynamique, pas de l'état initial : pas {step}");
    // Pression non convergée et gravité nulle : refus, état intact, mode désarmé.
    let wave: Vec<f32> = (0..nx).map(|i| 1.5 + 0.2 * (i as f32).sin()).collect();
    v.set_free_surface(&wave, 1.5).unwrap();
    v.set_velocity(&vec![0.; v.u.len()], &vec![0.; v.w.len()]).unwrap();
    let before = snapshot(&v);
    assert_eq!(v.step_surface_mobile(1000, 1, 1_000_000, &Jobs, &StillClock), Err(Error::Convergence));
    assert_eq!(snapshot(&v), before);
    v.g_eff = 0.;
    assert_eq!(v.step_surface_mobile(1000, 200, 1_000_000, &Jobs, &StillClock), Err(Error::Domain));
    assert_eq!(snapshot(&v), before);
    assert!(!v.mobile);
}

/// S238 P3 — loi du plancher, famille S231 : 8×4 m, surface `4 + 0,01·sin`, un pas depuis le repos,
/// fond plat 0,5 m, plafond large. Rend les traces de vrai résidu relatif et d'erreur inverse.
#[test]
#[ignore = "mesure S238, lancée explicitement"]
fn pressure_floor_law_s231_family_s238() {
    for nx in [16usize, 32, 64, 128, 256] {
        let nz = nx / 2;
        let dx = 8. / nx as f32;
        let mut v = mobile_volume(nx, nz, dx, &vec![0.5; nx]);
        let eta: Vec<f32> = (0..nx).map(|i| 4. + 0.01 * (std::f32::consts::TAU * (i as f32 + 0.5) / nx as f32).sin()).collect();
        v.set_surface(&eta).unwrap();
        PRESSURE_TRACE.with(|t| t.borrow_mut().clear());
        let r = v.step(0.002, 20_000, &Jobs).unwrap();
        let trace = PRESSURE_TRACE.with(|t| t.borrow().clone());
        let (min_rel, at) = trace.iter().fold((f64::INFINITY, 0usize), |(m, a), (_, rel, _)| if *rel < m { (*rel, a + 1) } else { (m, a + 1) });
        let first_under = trace.iter().position(|(_, rel, _)| *rel <= 1e-6);
        let last = trace.last().copied().unwrap();
        println!(
            "PLANCHER_S231 nx={nx} mailles={} relances={} residu_min={min_rel:.4e} (relance {at}) premier_sous_1e-6={first_under:?} dernier: it={} rel={:.4e} omega={:.3e} | rapport it={} degrade={} residu={:.4e} divergence={:.3e}",
            nx * nz, trace.len(), last.0, last.1, last.2, r.iterations, r.degraded, r.residual, r.divergence
        );
        let head: Vec<String> = trace.iter().take(6).map(|(it, rel, om)| format!("{it}:{rel:.3e}/{om:.2e}")).collect();
        println!("  TRACE nx={nx} {}", head.join(" "));
    }
}

/// S238 P3 — le cas refusé de S237, et ses voisins : onde stationnaire 5 cm, bassin 2×2 m, quart de
/// période, trois résolutions.
#[test]
#[ignore = "mesure S238, lancée explicitement (plusieurs minutes)"]
fn pressure_floor_law_mobile_quarter_period_s238() {
    for nx in [32usize, 64, 128] {
        let dx = 2. / nx as f32;
        let nz = (2.25 / dx).round() as usize;
        let mut v = mobile_volume(nx, nz, dx, &vec![0.; nx]);
        let eta: Vec<f32> = (0..nx).map(|i| 2. + 0.05 * (std::f32::consts::PI * (i as f32 + 0.5) / nx as f32).cos()).collect();
        v.set_free_surface(&eta, 2.).unwrap();
        let (mut worst_rel, mut worst_omega, mut worst_step) = (0f64, 0f64, 0usize);
        for step in 1..=400usize {
            PRESSURE_TRACE.with(|t| t.borrow_mut().clear());
            let result = v.step_surface_mobile(1000, 4000, 60_000_000, &Jobs, &StillClock);
            let trace = PRESSURE_TRACE.with(|t| t.borrow().clone());
            let last = trace.last().copied().unwrap_or((0, 0., 0.));
            if last.1 > worst_rel { (worst_rel, worst_omega, worst_step) = (last.1, last.2, step); }
            if result.is_err() {
                let tail: Vec<String> = trace.iter().rev().take(6).rev().map(|(it, rel, om)| format!("{it}:{rel:.4e}/{om:.2e}")).collect();
                let min = trace.iter().map(|x| x.1).fold(f64::INFINITY, f64::min);
                println!("PLANCHER_MOBILE nx={nx} pas={step} REFUS {result:?} relances={} residu_min={min:.4e} fin: {}", trace.len(), tail.join(" "));
                break;
            }
        }
        println!("PLANCHER_MOBILE nx={nx} pire_residu_final={worst_rel:.4e} omega={worst_omega:.3e} au_pas={worst_step} mailles={}", nx * nz);
    }
}

#[test]
fn mobile_projection_keeps_rest_exact_at_any_level_s237() {
    let (nx, nz, dx) = (16usize, 16usize, 0.15625f32);
    let floor = bottom(nx, dx);
    for rest in [2.0f32, 2.013, 1.9] {
        let mut v = mobile_volume(nx, nz, dx, &floor);
        v.set_free_surface(&vec![rest; nx], rest).unwrap();
        let r = v.project_mobile_for_test(0.002, 200, &Jobs).unwrap();
        assert_eq!(r.iterations, 0, "niveau {rest}");
        assert!(v.pressure().iter().chain(v.velocity_u()).chain(v.velocity_w()).all(|x| x.to_bits() == 0), "niveau {rest}");
    }
}
