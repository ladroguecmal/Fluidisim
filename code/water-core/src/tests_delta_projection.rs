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
