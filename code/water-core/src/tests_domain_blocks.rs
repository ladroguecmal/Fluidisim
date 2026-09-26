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
