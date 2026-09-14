//! WVST V1 : écarts à une base auteur immuable, ADR-140. Aucun stockage ni transport implicite.
use super::{geometry, Error, Flow, HydroNode, Opening, Shapes};
use crate::{hash::Hasher64, SimTime};

pub const HEADER: usize = 80;
const RECORD: usize = 12;
const TRAILER: usize = 8;
const VERSION: u16 = 1;
/// À incrémenter si le calcul de V change les bits de la continuation.
const CALCULATION_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotError {
    Length,
    Version,
    Reserved,
    Integrity,
    Configuration,
    Capacity,
    Record,
    State(Error),
}

/// Contexte au point de synchronisation. L'hôte reprend avec ce contexte et les mêmes entrées
/// futures ; le codec ne fait avancer ni l'horloge ni le graphe.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Context {
    pub time: SimTime,
    pub dt: SimTime,
    pub g_eff: [f32; 3],
}

/// La base est construite hors du pas puis empruntée sans mutation. Ses indices sont stables
/// dans cette révision ; graph_id identifie aussi le référentiel de l'hôte.
pub struct Baseline<'a> {
    nodes: &'a [HydroNode],
    edges: &'a [Opening],
    shapes: &'a Shapes<'a>,
    graph_id: u64,
    revision: u64,
    fingerprint: u64,
}

fn valid_edge(e: &Opening, nodes: usize) -> bool {
    let size = match e.flow { Flow::Orifice { area_mm2 } => area_mm2, Flow::Weir { width_mm } => width_mm };
    (e.from as usize) < nodes && e.to.map_or(true, |t| (t as usize) < nodes)
        && size >= 0 && e.discharge.is_finite() && e.discharge >= 0.0
        && (0..1_000_000).contains(&e.residue_nl)
}
fn same_node(a: &HydroNode, b: &HydroNode) -> bool {
    a.capacity_ml == b.capacity_ml && a.origin_um == b.origin_um && a.shape == b.shape
}
fn same_edge(a: &Opening, b: &Opening) -> bool {
    a.from == b.from && a.to == b.to && a.flow == b.flow && a.position_um == b.position_um
        && a.discharge.to_bits() == b.discharge.to_bits()
}
fn u64_at(b: &[u8], i: usize) -> u64 { u64::from_le_bytes(b[i..i+8].try_into().unwrap()) }
fn u32_at(b: &[u8], i: usize) -> u32 { u32::from_le_bytes(b[i..i+4].try_into().unwrap()) }
fn i64_at(b: &[u8], i: usize) -> i64 { i64::from_le_bytes(b[i..i+8].try_into().unwrap()) }
fn checksum(b: &[u8]) -> u64 {
    let mut h = Hasher64::new();
    for &x in b { h.write_u8(x); }
    h.finish()
}

impl<'a> Baseline<'a> {
    pub fn new(graph_id: u64, revision: u64, nodes: &'a [HydroNode], edges: &'a [Opening],
               shapes: &'a Shapes<'a>) -> Result<Self, SnapshotError> {
        // Indices du noyau u16 ; comptes du format u32, et calculs de longueur bornés.
        if nodes.len() > 65_536 || edges.len() > u32::MAX as usize {
            return Err(SnapshotError::Capacity);
        }
        for n in nodes {
            shapes.validate_node(n, [0., 0., 1.]).map_err(SnapshotError::State)?;
        }
        for e in edges {
            if !valid_edge(e, nodes.len()) || e.residue_nl != 0 { return Err(SnapshotError::Configuration); }
        }
        let mut h = Hasher64::new();
        h.write_u32(CALCULATION_VERSION);
        h.write_u64(graph_id);
        h.write_u64(revision);
        h.write_u64(nodes.len() as u64);
        for n in nodes {
            h.write_u64(n.volume_ml as u64);
            h.write_u64(n.capacity_ml as u64);
            for x in n.origin_um { h.write_u64(x as u64); }
            h.write_u32(n.shape as u32);
        }
        h.write_u64(edges.len() as u64);
        for e in edges {
            h.write_u32(e.from as u32);
            h.write_u32(e.to.map_or(u32::MAX, u32::from));
            match e.flow {
                Flow::Orifice { area_mm2 } => { h.write_u8(0); h.write_u64(area_mm2 as u64); }
                Flow::Weir { width_mm } => { h.write_u8(1); h.write_u64(width_mm as u64); }
            }
            for x in e.position_um { h.write_u64(x as u64); }
            h.write_u32(e.discharge.to_bits());
        }
        h.write_u8(u8::from(shapes.table.is_empty()));
        h.write_u64(shapes.count() as u64);
        if !shapes.table.is_empty() {
            for &x in shapes.table { h.write_u64(x as u64); }
        } else {
            for shape in shapes.volumes {
                h.write_u64(shape.capacity_ml() as u64);
                h.write_u64(shape.tetrahedra().len() as u64);
                for cell in shape.tetrahedra() {
                    for vertex in cell.vertices_um() {
                        for x in vertex { h.write_u64(x as u64); }
                    }
                }
            }
        }
        Ok(Self { nodes, edges, shapes, graph_id, revision, fingerprint: h.finish() })
    }

    /// Empreinte de compatibilité accidentelle ; ni signature ni garantie sans collision.
    pub fn fingerprint(&self) -> u64 { self.fingerprint }

    fn context(&self, c: Context) -> Result<(), SnapshotError> {
        if c.dt.0 == 0 { return Err(SnapshotError::State(Error::Domain)); }
        let (up, _) = geometry::vertical(c.g_eff).map_err(SnapshotError::State)?;
        if self.shapes.volumes.is_empty() && up != [0., 0., 1.] {
            return Err(SnapshotError::State(Error::Orientation));
        }
        Ok(())
    }

    fn counts(&self, nodes: &[HydroNode], edges: &[Opening]) -> Result<(usize, usize), SnapshotError> {
        if nodes.len() != self.nodes.len() || edges.len() != self.edges.len() {
            return Err(SnapshotError::Configuration);
        }
        let (mut nc, mut ec) = (0, 0);
        for (n, base) in nodes.iter().zip(self.nodes) {
            if !same_node(n, base) { return Err(SnapshotError::Configuration); }
            if !(0..=n.capacity_ml).contains(&n.volume_ml) { return Err(SnapshotError::State(Error::Capacity)); }
            nc += usize::from(n.volume_ml != base.volume_ml);
        }
        for (e, base) in edges.iter().zip(self.edges) {
            if !same_edge(e, base) { return Err(SnapshotError::Configuration); }
            if !(0..1_000_000).contains(&e.residue_nl) { return Err(SnapshotError::Record); }
            ec += usize::from(e.residue_nl != 0);
        }
        Ok((nc, ec))
    }

    pub fn snapshot_len(&self, nodes: &[HydroNode], edges: &[Opening]) -> Result<usize, SnapshotError> {
        let (nc, ec) = self.counts(nodes, edges)?;
        nc.checked_add(ec).and_then(|n| n.checked_mul(RECORD))
            .and_then(|n| n.checked_add(HEADER + TRAILER)).ok_or(SnapshotError::Length)
    }

    /// Aucun octet modifié au refus. Le suffixe au-delà de la longueur retournée reste intact.
    pub fn snapshot_into(&self, nodes: &[HydroNode], edges: &[Opening], context: Context,
                         output: &mut [u8]) -> Result<usize, SnapshotError> {
        self.context(context)?;
        let len = self.snapshot_len(nodes, edges)?;
        let (nc, ec) = self.counts(nodes, edges)?;
        if output.len() < len { return Err(SnapshotError::Capacity); }
        let b = &mut output[..len];
        b[..HEADER].fill(0);
        b[..4].copy_from_slice(b"WVST");
        b[4..6].copy_from_slice(&VERSION.to_le_bytes());
        b[8..16].copy_from_slice(&(len as u64).to_le_bytes());
        b[16..24].copy_from_slice(&self.fingerprint.to_le_bytes());
        b[24..32].copy_from_slice(&self.graph_id.to_le_bytes());
        b[32..40].copy_from_slice(&self.revision.to_le_bytes());
        b[40..48].copy_from_slice(&context.time.0.to_le_bytes());
        b[48..56].copy_from_slice(&context.dt.0.to_le_bytes());
        for (i, g) in context.g_eff.iter().enumerate() { b[56+4*i..60+4*i].copy_from_slice(&g.to_bits().to_le_bytes()); }
        b[68..72].copy_from_slice(&(nc as u32).to_le_bytes());
        b[72..76].copy_from_slice(&(ec as u32).to_le_bytes());
        let mut at = HEADER;
        let mut write = |index: usize, value: i64| {
            b[at..at+4].copy_from_slice(&(index as u32).to_le_bytes());
            b[at+4..at+RECORD].copy_from_slice(&value.to_le_bytes());
            at += RECORD;
        };
        for (i, (n, base)) in nodes.iter().zip(self.nodes).enumerate() {
            if n.volume_ml != base.volume_ml { write(i, n.volume_ml); }
        }
        for (i, e) in edges.iter().enumerate() {
            if e.residue_nl != 0 { write(i, e.residue_nl); }
        }
        let check = checksum(&b[..len-TRAILER]);
        b[len-TRAILER..].copy_from_slice(&check.to_le_bytes());
        Ok(len)
    }

    /// Validation complète puis reconstruction depuis la base ; destinations intactes au refus.
    /// Aucun calcul de plan ni pas caché. Les suffixes des pools restent intacts.
    pub fn restore_into(&self, b: &[u8], nodes: &mut [HydroNode], edges: &mut [Opening])
        -> Result<Context, SnapshotError> {
        if b.len() < HEADER + TRAILER { return Err(SnapshotError::Length); }
        if &b[..4] != b"WVST" || b[4..6] != VERSION.to_le_bytes() { return Err(SnapshotError::Version); }
        if b[6..8] != [0; 2] || b[76..80] != [0; 4] { return Err(SnapshotError::Reserved); }
        let (nc, ec) = (u32_at(b, 68) as usize, u32_at(b, 72) as usize);
        let expected = nc.checked_add(ec).and_then(|n| n.checked_mul(RECORD))
            .and_then(|n| n.checked_add(HEADER + TRAILER)).ok_or(SnapshotError::Length)?;
        if expected != b.len() || u64_at(b, 8) != b.len() as u64 { return Err(SnapshotError::Length); }
        if checksum(&b[..b.len()-TRAILER]) != u64_at(b, b.len()-TRAILER) { return Err(SnapshotError::Integrity); }
        if u64_at(b, 16) != self.fingerprint || u64_at(b, 24) != self.graph_id || u64_at(b, 32) != self.revision {
            return Err(SnapshotError::Configuration);
        }
        if nodes.len() < self.nodes.len() || edges.len() < self.edges.len() { return Err(SnapshotError::Capacity); }
        let context = Context { time: SimTime(u64_at(b, 40)), dt: SimTime(u64_at(b, 48)),
            g_eff: std::array::from_fn(|i| f32::from_bits(u32_at(b, 56+4*i))) };
        self.context(context)?;
        let mut at = HEADER;
        for (count, is_node) in [(nc, true), (ec, false)] {
            let mut previous = None;
            for _ in 0..count {
                let index = u32_at(b, at) as usize;
                let value = i64_at(b, at+4);
                if previous.is_some_and(|old| old >= index) { return Err(SnapshotError::Record); }
                if is_node {
                    let base = self.nodes.get(index).ok_or(SnapshotError::Record)?;
                    if !(0..=base.capacity_ml).contains(&value) || value == base.volume_ml { return Err(SnapshotError::Record); }
                } else if index >= self.edges.len() || !(1..1_000_000).contains(&value) {
                    return Err(SnapshotError::Record);
                }
                previous = Some(index);
                at += RECORD;
            }
        }
        // À partir d'ici, aucun refus : l'entrée et la base sont immuables et tous les indices reçus.
        nodes[..self.nodes.len()].copy_from_slice(self.nodes);
        edges[..self.edges.len()].copy_from_slice(self.edges);
        at = HEADER;
        for _ in 0..nc {
            nodes[u32_at(b, at) as usize].volume_ml = i64_at(b, at+4);
            at += RECORD;
        }
        for _ in 0..ec {
            edges[u32_at(b, at) as usize].residue_nl = i64_at(b, at+4);
            at += RECORD;
        }
        Ok(context)
    }
}

#[cfg(test)]
#[path = "tests_hydro_snapshot.rs"]
mod tests;
