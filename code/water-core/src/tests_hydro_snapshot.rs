use super::*;
use crate::hydro_network::{step, SHAPE_ENTRIES, STEP_US};

fn table() -> [i64; SHAPE_ENTRIES] { std::array::from_fn(|i| i as i64 * 1_000_000 / 63) }
fn node(volume_ml: i64) -> HydroNode {
    HydroNode { volume_ml, capacity_ml: 1_000_000, origin_um: [0; 3], shape: 0 }
}
fn edge() -> Opening {
    Opening { from: 0, to: Some(1), flow: Flow::Orifice { area_mm2: 1 },
        position_um: [0; 3], discharge: 0.62, residue_nl: 0, control_pm: crate::hydro_network::CONTROL_FULL }
}
fn context() -> Context { Context { time: SimTime(0), dt: SimTime(STEP_US), g_eff: [-0., 0., -9.81] } }
fn refresh(b: &mut [u8]) {
    let end = b.len() - TRAILER;
    let hash = checksum(&b[..end]);
    b[end..].copy_from_slice(&hash.to_le_bytes());
}
fn edge_bits(edges: &[Opening]) -> Vec<(u16, Option<u16>, Flow, [i64; 3], u32, i64)> {
    edges.iter().map(|e| (e.from, e.to, e.flow, e.position_um, e.discharge.to_bits(), e.residue_nl)).collect()
}

#[test]
fn deltas_restore_from_author_base_and_preserve_context_bits() {
    let t = table(); let shapes = Shapes::new(&t).unwrap();
    let original = [node(750_000), node(0)]; let original_edges = [edge()];
    let base = Baseline::new(17, 3, &original, &original_edges, &shapes).unwrap();
    let mut n = original; let mut e = original_edges;
    n[0].volume_ml -= 100; e[0].residue_nl = 999_999;
    let c = Context { time: SimTime(u64::MAX), dt: SimTime(7), ..context() };
    let mut b = vec![0xaa; base.snapshot_len(&n, &e).unwrap() + 3];
    let len = base.snapshot_into(&n, &e, c, &mut b).unwrap();
    assert_eq!(len, 112); assert_eq!(&b[len..], &[0xaa; 3]);
    // S372 (ADR-199 D5) : version 2 — la commande des arêtes est un état ; la version 1 est refusée, sans migration.
    assert_eq!(&b[..8], b"WVST\x02\0\0\0");
    assert_eq!(&b[40..48], &u64::MAX.to_le_bytes());
    assert_eq!(&b[80..84], &0u32.to_le_bytes());
    assert_eq!(&b[84..92], &749_900i64.to_le_bytes());
    let mut dest = [node(-1); 3]; let mut out = [Opening { discharge: f32::NAN, ..edge() }; 2];
    let sentinel = edge_bits(&out[1..]);
    let restored = base.restore_into(&b[..len], &mut dest, &mut out).unwrap();
    assert_eq!(restored.time, c.time); assert_eq!(restored.dt, c.dt);
    assert_eq!(restored.g_eff.map(f32::to_bits), c.g_eff.map(f32::to_bits));
    assert_eq!(&dest[..2], &n); assert_eq!(dest[2], node(-1));
    assert_eq!(edge_bits(&out[..1]), edge_bits(&e)); assert_eq!(edge_bits(&out[1..]), sentinel);
    let mut again = vec![0; len]; base.snapshot_into(&dest[..2], &out[..1], restored, &mut again).unwrap();
    assert_eq!(again, b[..len]);
    // L'absence d'écart remet aussi une destination sale à la valeur auteur et au reste zéro.
    let mut empty = [0; 88]; base.snapshot_into(&original, &original_edges, context(), &mut empty).unwrap();
    base.restore_into(&empty, &mut dest, &mut out).unwrap();
    assert_eq!(&dest[..2], &original); assert_eq!(out[0].residue_nl, 0);
}

#[test]
fn c19_v_tabulated_continuation_detects_omitted_residues() {
    let t = table(); let shapes = Shapes::new(&t).unwrap();
    let original = [node(750_000), node(0)]; let original_edges = [edge()];
    let base = Baseline::new(17, 3, &original, &original_edges, &shapes).unwrap();
    let (mut n, mut e, mut c) = (original, original_edges, context());
    for _ in 0..3 { step(&mut n, &mut e, &shapes, c.g_eff, c.dt, &mut [0]).unwrap(); c.time.0 += c.dt.0; }
    assert!(e[0].residue_nl > 0);
    let mut b = vec![0; base.snapshot_len(&n, &e).unwrap()]; base.snapshot_into(&n, &e, c, &mut b).unwrap();
    let (mut restored_n, mut restored_e) = (original, original_edges);
    let mut restored_c = base.restore_into(&b, &mut restored_n, &mut restored_e).unwrap();
    let (mut wrong_n, mut wrong_e) = (n, original_edges);
    let mut divergent = false;
    for _ in 0..1000 {
        step(&mut n, &mut e, &shapes, c.g_eff, c.dt, &mut [0]).unwrap(); c.time.0 += c.dt.0;
        step(&mut restored_n, &mut restored_e, &shapes, restored_c.g_eff, restored_c.dt, &mut [0]).unwrap();
        restored_c.time.0 += restored_c.dt.0;
        step(&mut wrong_n, &mut wrong_e, &shapes, c.g_eff, c.dt, &mut [0]).unwrap();
        assert_eq!(n, restored_n); assert_eq!(e[0].residue_nl, restored_e[0].residue_nl);
        assert_eq!(c.time, restored_c.time);
        assert_eq!(n.iter().map(|n| n.volume_ml).sum::<i64>(), 750_000);
        divergent |= n != wrong_n;
    }
    assert!(divergent, "le temoin doit distinguer l'oubli du reste sur les volumes eux-memes");
}

#[test]
fn malformed_snapshots_and_small_pools_are_atomic() {
    let t = table(); let shapes = Shapes::new(&t).unwrap();
    let original = [node(750_000), node(0)]; let original_edges = [edge(), edge()];
    let base = Baseline::new(17, 3, &original, &original_edges, &shapes).unwrap();
    let n = [node(749_998), node(2)]; let e = [Opening { residue_nl: 5, ..edge() }; 2];
    let mut good = vec![0; base.snapshot_len(&n, &e).unwrap()]; base.snapshot_into(&n, &e, context(), &mut good).unwrap();
    let reject = |b: &[u8]| {
        let mut dest = [node(123); 2]; let mut out = [Opening { residue_nl: 17, ..edge() }; 2];
        let before = dest; let before_e = edge_bits(&out);
        assert!(base.restore_into(b, &mut dest, &mut out).is_err());
        assert_eq!(dest, before); assert_eq!(edge_bits(&out), before_e);
    };
    for end in 0..good.len() { reject(&good[..end]); }
    let mut extra = good.clone(); extra.push(0); reject(&extra);
    for i in 0..good.len() { let mut bad = good.clone(); bad[i] ^= 1; reject(&bad); }
    // Recalculer le contrôle d'intégrité pour réellement exercer les validations sémantiques.
    for (offset, bytes) in [
        (68, u32::MAX.to_le_bytes().to_vec()),
        (48, 0u64.to_le_bytes().to_vec()),
        (56, f32::NAN.to_bits().to_le_bytes().to_vec()),
        (64, 9.81f32.to_bits().to_le_bytes().to_vec()),
        (80, 2u32.to_le_bytes().to_vec()),
        (92, 0u32.to_le_bytes().to_vec()), // doublon nœud
        (84, (-1i64).to_le_bytes().to_vec()),
        (84, 1_000_001i64.to_le_bytes().to_vec()),
        (84, 750_000i64.to_le_bytes().to_vec()), // entrée redondante
        (116, 0u32.to_le_bytes().to_vec()), // doublon arête
        (108, 1_000_000i64.to_le_bytes().to_vec()),
        (108, 0i64.to_le_bytes().to_vec()),
    ] {
        let mut bad = good.clone(); bad[offset..offset+bytes.len()].copy_from_slice(&bytes); refresh(&mut bad); reject(&bad);
    }
    let mut dest = original; let mut out = original_edges;
    assert_eq!(base.restore_into(&good, &mut dest[..1], &mut out), Err(SnapshotError::Capacity));
    assert_eq!(base.restore_into(&good, &mut dest, &mut out[..1]), Err(SnapshotError::Capacity));
    assert_eq!(dest, original); assert_eq!(edge_bits(&out), edge_bits(&original_edges));
}

#[test]
fn capture_refusals_do_not_touch_output() {
    let t = table(); let shapes = Shapes::new(&t).unwrap();
    let original = [node(750_000), node(0)]; let original_edges = [edge()];
    let base = Baseline::new(17, 3, &original, &original_edges, &shapes).unwrap();
    let mut out = [0xaa; 100];
    let mut bad = original; bad[1].volume_ml = -1;
    assert!(base.snapshot_into(&bad, &original_edges, context(), &mut out).is_err());
    let mut bad_e = original_edges; bad_e[0].residue_nl = 1_000_000;
    assert!(base.snapshot_into(&original, &bad_e, context(), &mut out).is_err());
    bad_e[0] = Opening { discharge: 0.5, ..edge() };
    assert_eq!(base.snapshot_into(&original, &bad_e, context(), &mut out), Err(SnapshotError::Configuration));
    assert!(base.snapshot_into(&original, &original_edges, Context { dt: SimTime(0), ..context() }, &mut out).is_err());
    assert!(base.snapshot_into(&original, &original_edges, context(), &mut out[..87]).is_err());
    assert_eq!(out, [0xaa; 100]);
}

#[test]
fn author_configuration_and_geometry_changes_refuse_old_state() {
    let t = table(); let shapes = Shapes::new(&t).unwrap();
    let n = [node(750_000), node(0)]; let e = [edge()];
    let base = Baseline::new(17, 3, &n, &e, &shapes).unwrap();
    let mut b = [0; 88]; base.snapshot_into(&n, &e, context(), &mut b).unwrap();
    let mut dest = n; let mut out = e;
    let mut check = |other: &Baseline<'_>| {
        assert_ne!(base.fingerprint(), other.fingerprint());
        assert_eq!(other.restore_into(&b, &mut dest, &mut out), Err(SnapshotError::Configuration));
        assert_eq!(dest, n); assert_eq!(edge_bits(&out), edge_bits(&e));
    };
    check(&Baseline::new(18, 3, &n, &e, &shapes).unwrap());
    check(&Baseline::new(17, 4, &n, &e, &shapes).unwrap());
    for field in 0..3 {
        let mut changed = n;
        match field { 0 => changed[0].volume_ml += 1, 1 => changed[0].capacity_ml += 1, _ => changed[0].origin_um[0] += 1 }
        check(&Baseline::new(17, 3, &changed, &e, &shapes).unwrap());
    }
    for changed in [Opening { to: None, ..edge() }, Opening { discharge: 0.5, ..edge() },
        Opening { flow: Flow::Weir { width_mm: 1 }, ..edge() }, Opening { position_um: [1, 0, 0], ..edge() }] {
        check(&Baseline::new(17, 3, &n, &[changed], &shapes).unwrap());
    }
    let mut different = t; different[1] += 1;
    let different_shapes = Shapes::new(&different).unwrap();
    check(&Baseline::new(17, 3, &n, &e, &different_shapes).unwrap());
    // Même volume exact, coordonnées différentes : l'empreinte ne se réduit pas à la capacité.
    let vertices = [[0,0,0], [1_000_000,0,0], [0,1_000_000,0], [0,0,1_000_000]];
    let cells = [geometry::Tetrahedron::new(vertices).unwrap()];
    let translated = [geometry::Tetrahedron::new(vertices.map(|v| [v[0]+1,v[1],v[2]])).unwrap()];
    let volumes = [geometry::VolumeShape::new(&cells).unwrap()];
    let other_volumes = [geometry::VolumeShape::new(&translated).unwrap()];
    let s1 = Shapes::from_volumes(&volumes).unwrap(); let s2 = Shapes::from_volumes(&other_volumes).unwrap();
    let n = [HydroNode { capacity_ml: volumes[0].capacity_ml(), volume_ml: 0, ..node(0) }];
    let a = Baseline::new(17, 3, &n, &[], &s1).unwrap(); let other = Baseline::new(17, 3, &n, &[], &s2).unwrap();
    assert_ne!(a.fingerprint(), other.fingerprint());
    a.snapshot_into(&n, &[], context(), &mut b).unwrap();
    assert_eq!(other.restore_into(&b, &mut dest, &mut out), Err(SnapshotError::Configuration));
}
