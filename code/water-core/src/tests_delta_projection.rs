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
