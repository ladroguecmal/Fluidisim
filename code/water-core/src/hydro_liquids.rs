//! **Les liquides de V** (S559, [ADR-241](../../../docs/adr/ADR-241-les-liquides-de-v.md), liste 5.7, A17).
//!
//! Plusieurs liquides par nœud, **non miscibles, en couches** planes perpendiculaires à `g_eff`, la plus dense au fond (à densité égale,
//! l'ordre de la table : I-03). Chaque interface est le plan de la géométrie du nœud pour le volume cumulé des couches qu'elle couvre —
//! le calcul de la surface libre (ADR-139), donc juste sous une gravité inclinée et dans une forme quelconque.
//!
//! L'état reste entier (I-10) : `volume_ml` du nœud est le total ; la composition est une ligne de millilitres de l'appelant, une
//! entrée par liquide de la table, qui somme à `volume_ml`. Aucune allocation (I-06) : au plus [`MAX_LIQUIDS`] liquides.
//!
//! Cette première pièce donne **la pression en un point d'un nœud** (ADR-241 D3) ; le débit par couches (D4) vient ensuite.

use super::{along, geometry, sub, Error, HydroNode, Shapes};

/// Le nombre de liquides d'une table, au plus — une borne sans allocation (ADR-241 D2).
pub const MAX_LIQUIDS: usize = 8;

/// Un liquide de la table : une donnée d'auteur.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Liquid {
    /// Masse volumique, kg/m³ ; finie et positive.
    pub density_kg_m3: f32,
}

/// Les indices des liquides présents dans `composition`, du fond vers la surface (densité décroissante ; à densité égale, l'indice
/// croissant), et leur nombre. Refus : longueurs, densités, volumes négatifs, une somme qui n'est pas `volume_ml`.
fn layers(node: &HydroNode, composition: &[i64], liquids: &[Liquid]) -> Result<([usize; MAX_LIQUIDS], usize), Error> {
    if liquids.is_empty() || liquids.len() > MAX_LIQUIDS || composition.len() != liquids.len() {
        return Err(Error::Capacity);
    }
    if liquids.iter().any(|l| !(l.density_kg_m3 > 0.0) || !l.density_kg_m3.is_finite()) {
        return Err(Error::Domain);
    }
    if composition.iter().any(|v| *v < 0) || composition.iter().map(|v| *v as i128).sum::<i128>() != node.volume_ml as i128 {
        return Err(Error::Capacity);
    }
    let mut order = [0usize; MAX_LIQUIDS];
    let mut n = 0;
    for (i, v) in composition.iter().enumerate() {
        if *v == 0 {
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
    Ok((order, n))
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
