//! Essais des ensembles de blocs (S396, C8a).
use super::*;
use crate::apic3d::tests::{Arena, Jobs};
use crate::host::{AllocStats, HostServices};

fn set(capacity: usize, blocks: &[(i32, i32)]) -> BlockSet {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut s = BlockSet::with_capacity(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, capacity).unwrap();
    assert_eq!(arena.stats.persistent_bytes, BlockSet::reserved_bytes(capacity).unwrap());
    for (i, j) in blocks {
        s.insert(Block { i: *i, j: *j }).unwrap();
    }
    s
}

#[test]
fn a_set_is_sorted_bounded_and_refuses_beyond_capacity_s396() {
    let mut s = set(3, &[(2, 0), (0, 1), (2, 0)]);
    assert_eq!(s.len(), 2);
    assert_eq!(s.blocks(), &[Block { i: 0, j: 1 }, Block { i: 2, j: 0 }]);
    assert_eq!(s.bounds(), Some((Block { i: 0, j: 0 }, Block { i: 2, j: 1 })));
    s.insert(Block { i: 5, j: 5 }).unwrap();
    assert_eq!(s.insert(Block { i: 6, j: 5 }), Err(BlockError::Capacity));
    assert_eq!(s.len(), 3);
    let mut sealed = Arena { stats: AllocStats::default(), sealed: true };
    assert!(matches!(
        BlockSet::with_capacity(&mut HostServices { alloc: &mut sealed, jobs: &Jobs, sink: &Jobs }, 4),
        Err(BlockError::Host)
    ));
}

#[test]
fn fusion_is_the_union_and_uses_the_same_relation_as_separation_s396() {
    // r = 2 : deux blocs sont liés jusqu'à une distance de Chebyshev 5 (dilatations de deux blocs qui se touchent).
    assert!(linked(Block { i: 0, j: 0 }, Block { i: 5, j: 3 }, 2));
    assert!(!linked(Block { i: 0, j: 0 }, Block { i: 6, j: 0 }, 2));
    for gap in 4..8 {
        let mut a = set(16, &[(0, 0), (1, 0)]);
        let b = set(16, &[(1 + gap, 0), (2 + gap, 1)]);
        let touches = a.touches(&b, 2);
        assert_eq!(touches, gap <= 5, "écart {gap}");
        a.union_with(&b).unwrap();
        assert_eq!(a.len(), 4);
        let mut labels = [0u32; 16];
        // La même relation : fusionner, c'est n'avoir plus qu'une composante ; sinon deux.
        let n = a.components(2, &mut labels);
        assert_eq!(n, if touches { 1 } else { 2 }, "écart {gap}");
    }
    let mut full = set(2, &[(0, 0), (1, 0)]);
    assert_eq!(full.union_with(&set(2, &[(9, 9)])), Err(BlockError::Capacity));
    assert_eq!(full.len(), 2);
}

#[test]
fn components_are_numbered_in_order_of_appearance_s396() {
    // Trois amas au rayon 1 (liés jusqu'à 3) : {(0,0),(3,0)}, {(10,0)}, {(0,10),(2,12)}.
    let mut s = set(8, &[(0, 0), (3, 0), (10, 0), (0, 10), (2, 12)]);
    let mut labels = [9u32; 8];
    assert_eq!(s.components(1, &mut labels), 3);
    // Ordre trié : (0,0) (0,10) (2,12) (3,0) (10,0).
    assert_eq!(&labels[..5], &[0, 1, 1, 0, 2]);
    // Au rayon 0, seuls les voisins directs sont liés : cinq composantes.
    assert_eq!(s.components(0, &mut labels), 5);
}

#[test]
fn separation_waits_one_continuous_second_and_never_follows_a_flicker_s396() {
    let mut c = SplitClock::default();
    // Deux composantes sans interruption : due à 1,0 s exactement, pas avant.
    for step in 1..=9 {
        assert!(!c.advance(100_000, 2), "pas {step}");
    }
    assert!(c.advance(100_000, 2));
    // Une composante qui clignote — 0,9 s à deux, 0,1 s à une — pendant dix secondes : jamais.
    let mut f = SplitClock::default();
    for _ in 0..10 {
        for _ in 0..9 {
            assert!(!f.advance(100_000, 2));
        }
        assert!(!f.advance(100_000, 1));
        assert_eq!(f.split_for_us(), 0);
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// S401 — le domaine qui suit la perturbation, et sa prévision (C8b).

fn follow(nx: usize, ny: usize) -> Follow {
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let f = Follow::with_capacity(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, nx, ny, 0.25, 2, 1e-3,
        250_000)
    .unwrap();
    let (nbx, nby) = f.blocks();
    assert_eq!(arena.stats.persistent_bytes, Follow::reserved_bytes(nbx, nby).unwrap());
    f
}

#[test]
fn the_useful_horizon_is_that_of_adr_013_s401() {
    // ADR-013 §2 : avion de chasse (20 m/s², 20 m) 1,4 s ; en perte de contrôle (3 m/s²) 3,7 s ; vaisseau lourd (5 m/s², 60 m)
    // 4,9 s ; balistique : la borne donnée.
    assert!((useful_horizon(20., 20., 10.) - std::f32::consts::SQRT_2).abs() < 1e-6);
    assert!((useful_horizon(20., 3., 10.) - 3.6515).abs() < 1e-3);
    assert!((useful_horizon(60., 5., 10.) - 4.8990).abs() < 1e-3);
    assert_eq!(useful_horizon(20., 0., 2.5), 2.5);
    assert_eq!(useful_horizon(20., 1., 2.5), 2.5);
}

#[test]
fn activity_is_dilated_by_the_coupling_radius_then_released_after_a_quarter_second_s401() {
    let (nx, ny) = (64, 64);
    let mut f = follow(nx, ny);
    let mut eta = vec![2f32; nx * ny];
    eta[20 * nx + 20] = 2.0011; // bloc (2, 2), au-dessus du seuil de 1 mm
    eta[50 * nx + 50] = 2.0009; // sous le seuil
    let set = f.update(0, &eta, 2., nx, &[]).to_vec();
    let (nbx, _) = f.blocks();
    for bj in 0..8 {
        for bi in 0..8 {
            assert_eq!(set[bj * nbx + bi] != 0, bi <= 4 && bj <= 4, "bloc ({bi}, {bj})");
        }
    }
    // Plus rien ne le requiert : il reste 0,25 s, pas davantage — la durée de vie minimale d'un bloc (ADR-006 §4).
    let rest = vec![2f32; nx * ny];
    assert_eq!(f.update(249_999, &rest, 2., nx, &[]).iter().filter(|b| **b != 0).count(), 25);
    assert_eq!(f.update(250_000, &rest, 2., nx, &[]).iter().filter(|b| **b != 0).count(), 0);
    // Les colonnes suivent leurs blocs.
    f.update(300_000, &eta, 2., nx, &[]);
    let mut cols = vec![0u8; nx * ny];
    f.columns(nx, ny, &mut cols);
    assert_eq!(cols.iter().filter(|c| **c != 0).count(), 40 * 40);
    assert_eq!((cols[39 * nx + 39], cols[40 * nx + 39]), (1, 0));
}

/// Un générateur congruentiel, pour des tirages reproductibles.
struct Lcg(u64);
impl Lcg {
    fn unit(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[test]
fn every_bounded_maneuver_stays_in_the_predicted_set_s401() {
    // Critère 5 : pour des manœuvres tirées — accélération constante par dixième de seconde, `|a| ≤ a_max` —, la position à
    // tout instant jusqu'à l'horizon est dans un bloc du domaine prévu, et ses voisins à `r_c` aussi : 100 %.
    let (nx, ny) = (256, 256); // 64 m, 32 × 32 blocs
    let side = 2.0f64;
    let mut rng = Lcg(401);
    let (mut checked, mut blocks_used) = (0usize, 0usize);
    for trial in 0..300 {
        let mut f = follow(nx, ny);
        let a_max = [0.5f32, 2., 5.][trial % 3];
        let speed = 10. * rng.unit();
        let heading = std::f64::consts::TAU * rng.unit();
        let p = [16. + 32. * rng.unit(), 16. + 32. * rng.unit()];
        let v = [speed * heading.cos(), speed * heading.sin()];
        let horizon = useful_horizon(4., a_max, 5.);
        let t = Tracked { position: [p[0] as f32, p[1] as f32], velocity: [v[0] as f32, v[1] as f32], a_max, horizon,
            radius: 0.5 };
        let rest = vec![2f32; nx * ny];
        let set = f.update(0, &rest, 2., nx, &[t]).to_vec();
        blocks_used += set.iter().filter(|b| **b != 0).count();
        let (nbx, nby) = f.blocks();
        let (mut x, mut u, mut a) = (p, v, [0f64; 2]);
        let dt = 0.01;
        let mut time = 0.;
        while time < horizon as f64 {
            if (time / 0.1).fract() < 1e-9 || (time / 0.1).fract() > 1. - 1e-9 {
                let (m, th) = (a_max as f64 * rng.unit().sqrt(), std::f64::consts::TAU * rng.unit());
                a = [m * th.cos(), m * th.sin()];
            }
            for d in 0..2 {
                x[d] += u[d] * dt + 0.5 * a[d] * dt * dt;
                u[d] += a[d] * dt;
            }
            time += dt;
            let (bi, bj) = ((x[0] / side).floor() as isize, (x[1] / side).floor() as isize);
            for (di, dj) in (-2..=2).flat_map(|di| (-2..=2).map(move |dj| (di, dj))) {
                let (qi, qj) = (bi + di, bj + dj);
                if qi >= 0 && qj >= 0 && (qi as usize) < nbx && (qj as usize) < nby {
                    assert_eq!(set[qj as usize * nbx + qi as usize], 1, "essai {trial}, t = {time:.2} s, bloc ({qi}, {qj})");
                    checked += 1;
                }
            }
        }
    }
    println!("S401 enveloppe : {checked} blocs vérifiés, 100 % dans le domaine prévu ; {:.1} blocs par objet en moyenne",
        blocks_used as f64 / 300.);
}

#[test]
fn a_level_change_requires_the_blocks_that_cover_the_set_s404() {
    // S404 : un domaine qui passe de 25 à 50 cm hérite de son ensemble. Fenêtre de 8 × 6 m : 32 × 24 colonnes à 25 cm, 16 × 12 à
    // 50 cm, soit 2 × 2 blocs de 4 m. L'ensemble de départ, les colonnes [8, 24) × [8, 16) — x de 2 à 6 m, y de 2 à 4 m —, est
    // couvert par les blocs (0, 0) et (1, 0) ; une colonne qui touche le bord d'un bloc (x = 2 m) n'y met pas le voisin.
    let mut arena = Arena { stats: AllocStats::default(), sealed: false };
    let mut f = Follow::with_capacity(&mut HostServices { alloc: &mut arena, jobs: &Jobs, sink: &Jobs }, 16, 12, 0.5, 1, 1e-3,
        250_000)
    .unwrap();
    let mask: Vec<u8> = (0..32 * 24).map(|c| u8::from((8..24).contains(&(c % 32)) && (8..16).contains(&(c / 32)))).collect();
    f.require_cover(1_000_000, &mask, 32, 24, 0.25);
    assert_eq!(f.set(), &[1, 1, 0, 0]);
    let mut cols = vec![0u8; 16 * 12];
    f.columns(16, 12, &mut cols);
    assert_eq!(cols.iter().filter(|c| **c != 0).count(), 16 * 8);
    // Puis il suit sa perturbation : rien ne le requiert plus, il sort après le délai, pas avant.
    let rest = vec![2f32; 16 * 12];
    assert_eq!(f.update(1_249_999, &rest, 2., 16, &[]), &[1, 1, 0, 0]);
    assert_eq!(f.update(1_250_000, &rest, 2., 16, &[]), &[0, 0, 0, 0]);
}
