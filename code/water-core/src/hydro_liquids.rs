//! **Les liquides de V** (S559, [ADR-241](../../../docs/adr/ADR-241-les-liquides-de-v.md), liste 5.7, A17).
//!
//! Plusieurs liquides par nœud, **non miscibles, en couches** planes perpendiculaires à `g_eff`, la plus dense au fond (à densité égale,
//! l'ordre de la table : I-03). Chaque interface est le plan de la géométrie du nœud pour le volume cumulé des couches qu'elle couvre —
//! le calcul de la surface libre (ADR-139), donc juste sous une gravité inclinée et dans une forme quelconque.
//!
//! L'état reste entier (I-10) : `volume_ml` du nœud est le total ; la composition est une ligne de millilitres de l'appelant, une
//! entrée par liquide de la table, qui somme à `volume_ml`. Aucune allocation (I-06) : au plus [`MAX_LIQUIDS`] liquides.
//!
//! S559 : **la pression en un point d'un nœud** (ADR-241 D3). S560 : **le débit par couches** (D4) — [`step_liquids`]. S562 :
//! **l'instantané de la composition**, un bloc `WVLQ` à côté de WVST (ADR-140).

use super::snapshot::SnapshotError;
use super::{along, geometry, step_inner, sub, Error, Flow, HydroNode, Meteo, Opening, Shapes};
use crate::SimTime;

/// Le nombre de liquides d'une table, au plus — une borne sans allocation (ADR-241 D2).
pub const MAX_LIQUIDS: usize = 8;

/// Un liquide de la table : une donnée d'auteur.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Liquid {
    /// Masse volumique, kg/m³ ; finie et positive.
    pub density_kg_m3: f32,
}

/// Les indices des liquides présents dans `row`, du fond vers la surface (densité décroissante ; à densité égale, l'indice croissant),
/// et leur nombre. Ne valide rien.
fn order_of(row: &[i64], liquids: &[Liquid]) -> ([usize; MAX_LIQUIDS], usize) {
    let mut order = [0usize; MAX_LIQUIDS];
    let mut n = 0;
    for (i, v) in row.iter().enumerate() {
        if *v <= 0 {
            continue;
        }
        // Insertion stable : avant le premier strictement moins dense.
        let mut at = n;
        while at > 0 && liquids[order[at - 1]].density_kg_m3 < liquids[i].density_kg_m3 {
            at -= 1;
        }
        order.copy_within(at..n, at + 1);
        order[at] = i;
        n += 1;
    }
    (order, n)
}

/// La table : non vide, au plus [`MAX_LIQUIDS`], des densités finies et positives.
fn check_table(liquids: &[Liquid]) -> Result<(), Error> {
    if liquids.is_empty() || liquids.len() > MAX_LIQUIDS {
        return Err(Error::Capacity);
    }
    if liquids.iter().any(|l| !(l.density_kg_m3 > 0.0) || !l.density_kg_m3.is_finite()) {
        return Err(Error::Domain);
    }
    Ok(())
}

/// [`order_of`], après les refus : longueurs, densités, volumes négatifs, une somme qui n'est pas `volume_ml`.
fn layers(node: &HydroNode, composition: &[i64], liquids: &[Liquid]) -> Result<([usize; MAX_LIQUIDS], usize), Error> {
    check_table(liquids)?;
    if composition.len() != liquids.len() {
        return Err(Error::Capacity);
    }
    if composition.iter().any(|v| *v < 0) || composition.iter().map(|v| *v as i128).sum::<i128>() != node.volume_ml as i128 {
        return Err(Error::Capacity);
    }
    Ok(order_of(composition, liquids))
}

/// **La pression relative en un point d'un nœud stratifié**, Pa (ADR-241 D3) : la somme, sur les couches au-dessus du point, de
/// `ρᵢ·|g|·épaisseurᵢ`, les épaisseurs le long de la verticale locale. Zéro au-dessus de la surface. Ni l'atmosphère ni la charge d'une
/// poche d'air (`step_air`) n'y sont : l'appelant les ajoute.
pub fn pressure_at(node: &HydroNode, composition: &[i64], liquids: &[Liquid], shapes: &Shapes<'_>, g_eff: [f32; 3], point_um: [i64; 3])
    -> Result<f64, Error> {
    let (up, magnitude) = geometry::vertical(g_eff)?;
    shapes.validate_node(node, up)?;
    let (order, n) = layers(node, composition, liquids)?;
    let z = along(sub(point_um, node.origin_um), up);
    let (mut cumul, mut bas, mut pression) = (0i64, f64::NEG_INFINITY, 0f64);
    for &i in &order[..n] {
        cumul += composition[i];
        let haut = shapes.surface_up(&HydroNode { volume_ml: cumul, ..*node }, up)?.offset_um;
        if haut > z {
            pression += liquids[i].density_kg_m3 as f64 * magnitude * (haut - z.max(bas)).max(0.0) * 1e-6;
        }
        bas = haut;
    }
    if !pression.is_finite() {
        return Err(Error::NonFinite);
    }
    Ok(pression)
}

/// Le volume du nœud sous le plan du point (perpendiculaire à `up`), ml — formes volumiques seulement.
fn volume_below_point(node: &HydroNode, shapes: &Shapes<'_>, up: [f64; 3], point_um: [i64; 3]) -> Result<f64, Error> {
    let forme = shapes.volumes.get(node.shape as usize).ok_or(Error::Capacity)?;
    forme.volume_below_ml(geometry::SurfacePlane { up, offset_um: along(sub(point_um, node.origin_um), up) })
}

/// **Le liquide à un point** : la couche dont le haut (volume cumulé) dépasse le volume sous le point ; au-dessus de toutes, celle du
/// dessus. `None` dans un nœud vide.
fn liquid_at(row: &[i64], liquids: &[Liquid], below_ml: f64) -> Option<usize> {
    let (order, n) = order_of(row, liquids);
    let mut cumul = 0i64;
    for &i in &order[..n] {
        cumul += row[i];
        if cumul as f64 > below_ml {
            return Some(i);
        }
    }
    order[..n].last().copied()
}

/// **S560 — la charge d'un orifice sous plusieurs liquides**, m du liquide qui sort (ADR-241 D4) : `Δp/(ρ·|g|)`, `Δp` la différence des
/// pressions au seuil des deux côtés (zéro dehors), `ρ` le liquide amont au seuil. `None` si rien ne pousse vers l'aval.
pub(super) fn orifice_head_m(nodes: &[HydroNode], e: &Opening, shapes: &Shapes<'_>, g_eff: [f32; 3], composition: &[i64],
    liquids: &[Liquid]) -> Result<Option<f64>, Error> {
    let (up, magnitude) = geometry::vertical(g_eff)?;
    let n = liquids.len();
    let ligne = |i: usize| &composition[i * n..(i + 1) * n];
    let f = e.from as usize;
    let amont = pressure_at(&nodes[f], ligne(f), liquids, shapes, g_eff, e.position_um)?;
    let aval = match e.to {
        Some(t) => pressure_at(&nodes[t as usize], ligne(t as usize), liquids, shapes, g_eff, e.position_um)?,
        None => 0.0,
    };
    let dp = amont - aval;
    if !(dp > 0.0) {
        return Ok(None);
    }
    let Some(sortant) = liquid_at(ligne(f), liquids, volume_below_point(&nodes[f], shapes, up, e.position_um)?) else {
        return Ok(None);
    };
    Ok(Some(dp / (liquids[sortant].density_kg_m3 as f64 * magnitude)))
}

/// Retire `ml` de `row` : la couche `depart`, puis celles du dessus, puis celles du dessous ; ce qui est pris s'ajoute à `pris`.
fn take(row: &mut [i64], liquids: &[Liquid], depart: usize, mut ml: i64, pris: &mut [i64; MAX_LIQUIDS]) {
    let (order, n) = order_of(row, liquids);
    let Some(pos) = order[..n].iter().position(|&i| i == depart) else {
        return;
    };
    for &i in order[pos..n].iter().chain(order[..pos].iter().rev()) {
        let part = ml.min(row[i]);
        row[i] -= part;
        pris[i] += part;
        ml -= part;
        if ml == 0 {
            break;
        }
    }
}

/// **S560 — un pas de V sous plusieurs liquides** (ADR-241 D4). `composition` : `nœuds × liquides` millilitres, chaque ligne sommant au
/// volume du nœud ; `liquids` : la table ; `pluie` : le liquide qu'apporte la pluie.
///
/// Le pas de [`super::step_meteo`], sauf que : un orifice ou une vanne débite sur la différence de **pression** au seuil,
/// `Q = C_d·A·√(2Δp/ρ)`, `ρ` le liquide amont au seuil ; puis la composition suit les transferts entiers — chaque arête prend dans la
/// couche à son seuil (une évaporation et un débordement, dans celle du dessus), puis au-dessus, puis au-dessous ; les débordements en
/// dernier ; la pluie apporte `pluie`. Formes volumiques seulement (`Shape` sinon : une table +Z n'a pas de volume sous un plan) ; sans
/// air scellé dans cette version. Refus atomique : rien n'est écrit si le pas refuse.
#[allow(clippy::too_many_arguments)]
pub fn step_liquids(nodes: &mut [HydroNode], edges: &mut [Opening], shapes: &Shapes<'_>, g_eff: [f32; 3], meteo: Meteo, dt: SimTime,
    scratch: &mut [i64], composition: &mut [i64], liquids: &[Liquid], pluie: usize) -> Result<(), Error> {
    if shapes.volumes.is_empty() {
        return Err(Error::Shape);
    }
    check_table(liquids)?;
    let n = liquids.len();
    if composition.len() != nodes.len() * n || pluie >= n {
        return Err(Error::Capacity);
    }
    for (i, node) in nodes.iter().enumerate() {
        layers(node, &composition[i * n..(i + 1) * n], liquids)?;
    }
    let (up, _) = geometry::vertical(g_eff)?;
    // Le volume sous chaque seuil, essayé avant le pas : la mise à jour de la composition, après, ne peut plus refuser.
    for e in edges.iter() {
        let node = nodes.get(e.from as usize).ok_or(Error::Capacity)?;
        volume_below_point(node, shapes, up, e.position_um)?;
    }
    step_inner(nodes, edges, shapes, g_eff, meteo, dt, scratch, None, Some((composition, liquids)))?;
    for debordements in [false, true] {
        for (e, ml) in edges.iter().zip(scratch.iter()) {
            if *ml <= 0 || matches!(e.flow, Flow::Spill) != debordements || matches!(e.flow, Flow::Vent { .. }) {
                continue;
            }
            let mut pris = [0i64; MAX_LIQUIDS];
            let f = e.from as usize;
            if matches!(e.flow, Flow::Rain { .. }) {
                pris[pluie] = *ml;
            } else {
                let ligne = &mut composition[f * n..(f + 1) * n];
                let depart = if matches!(e.flow, Flow::Spill | Flow::Evaporation { .. }) {
                    let (order, k) = order_of(ligne, liquids);
                    order[..k].last().copied()
                } else {
                    let dessous = volume_below_point(&nodes[f], shapes, up, e.position_um)?;
                    liquid_at(ligne, liquids, dessous)
                };
                if let Some(d) = depart {
                    take(ligne, liquids, d, *ml, &mut pris);
                }
            }
            if let Some(t) = e.to {
                for (c, p) in composition[t as usize * n..(t as usize + 1) * n].iter_mut().zip(&pris) {
                    *c += *p;
                }
            }
        }
    }
    debug_assert!(nodes.iter().enumerate().all(|(i, node)| composition[i * n..(i + 1) * n].iter().sum::<i64>() == node.volume_ml));
    Ok(())
}

// --- S562 — l'instantané de la composition (I-17).

/// En-tête du bloc `WVLQ` : magie, version, réservé, nœuds, liquides, empreinte de la table.
pub const COMPOSITION_HEADER: usize = 4 + 2 + 2 + 4 + 4 + 8;
const COMPOSITION_TRAILER: usize = 8;
const COMPOSITION_MAGIC: &[u8; 4] = b"WVLQ";
const COMPOSITION_VERSION: u16 = 1;

/// L'empreinte de la table : FNV-1a des densités, par leurs bits.
fn table_fingerprint(liquids: &[Liquid]) -> u64 {
    let mut h = crate::hash::Hasher64::new();
    for l in liquids {
        h.write_u32(l.density_kg_m3.to_bits());
    }
    h.finish()
}

fn fnv(b: &[u8]) -> u64 {
    let mut h = crate::hash::Hasher64::new();
    for &x in b {
        h.write_u8(x);
    }
    h.finish()
}

/// La longueur du bloc `WVLQ` d'un réseau de `nodes` nœuds et `liquids` liquides.
pub fn composition_snapshot_len(nodes: usize, liquids: usize) -> usize {
    COMPOSITION_HEADER + nodes * liquids * 8 + COMPOSITION_TRAILER
}

/// **S562 — le bloc `WVLQ`** : la composition entière, à côté de l'instantané WVST de V (ADR-140), qui sauve les nœuds. Rend la
/// longueur écrite. Refus : une composition invalide (`State`), un tampon trop court (`Length`).
pub fn snapshot_composition_into(nodes: &[HydroNode], composition: &[i64], liquids: &[Liquid], out: &mut [u8])
    -> Result<usize, SnapshotError> {
    let n = liquids.len();
    check_table(liquids).map_err(SnapshotError::State)?;
    if composition.len() != nodes.len() * n {
        return Err(SnapshotError::State(Error::Capacity));
    }
    for (i, node) in nodes.iter().enumerate() {
        layers(node, &composition[i * n..(i + 1) * n], liquids).map_err(SnapshotError::State)?;
    }
    let len = composition_snapshot_len(nodes.len(), n);
    if out.len() < len {
        return Err(SnapshotError::Length);
    }
    let b = &mut out[..len];
    b[..4].copy_from_slice(COMPOSITION_MAGIC);
    b[4..6].copy_from_slice(&COMPOSITION_VERSION.to_le_bytes());
    b[6..8].copy_from_slice(&0u16.to_le_bytes());
    b[8..12].copy_from_slice(&(nodes.len() as u32).to_le_bytes());
    b[12..16].copy_from_slice(&(n as u32).to_le_bytes());
    b[16..24].copy_from_slice(&table_fingerprint(liquids).to_le_bytes());
    for (k, v) in composition.iter().enumerate() {
        let at = COMPOSITION_HEADER + 8 * k;
        b[at..at + 8].copy_from_slice(&v.to_le_bytes());
    }
    let somme = fnv(&b[..len - COMPOSITION_TRAILER]);
    b[len - COMPOSITION_TRAILER..].copy_from_slice(&somme.to_le_bytes());
    Ok(len)
}

/// **S562 — restaurer la composition** depuis un bloc `WVLQ`, **après** les nœuds (chaque ligne doit sommer au volume restauré).
/// Refus, sans rien écrire : `Length`, `Version` (magie ou version), `Reserved`, `Configuration` (nombre de nœuds, de liquides, autre
/// table), `Integrity`, `Record` (une ligne négative ou qui ne somme pas).
pub fn restore_composition_into(b: &[u8], nodes: &[HydroNode], liquids: &[Liquid], composition: &mut [i64]) -> Result<(), SnapshotError> {
    let n = liquids.len();
    let len = composition_snapshot_len(nodes.len(), n);
    if b.len() != len || composition.len() != nodes.len() * n {
        return Err(SnapshotError::Length);
    }
    if &b[..4] != COMPOSITION_MAGIC || u16::from_le_bytes([b[4], b[5]]) != COMPOSITION_VERSION {
        return Err(SnapshotError::Version);
    }
    if b[6..8] != [0, 0] {
        return Err(SnapshotError::Reserved);
    }
    if fnv(&b[..len - COMPOSITION_TRAILER]) != u64::from_le_bytes(b[len - COMPOSITION_TRAILER..].try_into().unwrap()) {
        return Err(SnapshotError::Integrity);
    }
    let u32_at = |i: usize| u32::from_le_bytes(b[i..i + 4].try_into().unwrap()) as usize;
    if u32_at(8) != nodes.len() || u32_at(12) != n
        || u64::from_le_bytes(b[16..24].try_into().unwrap()) != table_fingerprint(liquids) {
        return Err(SnapshotError::Configuration);
    }
    let valeur = |k: usize| i64::from_le_bytes(b[COMPOSITION_HEADER + 8 * k..COMPOSITION_HEADER + 8 * k + 8].try_into().unwrap());
    for (i, node) in nodes.iter().enumerate() {
        let mut somme = 0i128;
        for j in 0..n {
            let v = valeur(i * n + j);
            if v < 0 {
                return Err(SnapshotError::Record);
            }
            somme += v as i128;
        }
        if somme != node.volume_ml as i128 {
            return Err(SnapshotError::Record);
        }
    }
    for (k, c) in composition.iter_mut().enumerate() {
        *c = valeur(k);
    }
    Ok(())
}
