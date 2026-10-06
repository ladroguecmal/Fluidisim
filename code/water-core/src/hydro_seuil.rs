//! **Le seuil adaptatif à l'échelle du contenant** (S564, liste 5.6 ; intentions d'origine §2.2 : « une quantité significative dans un
//! bidon peut être négligeable dans une piscine »).
//!
//! Le seuil se mesure **en hauteur de surface**, le long de la verticale locale : c'est ce que voient le rendu et le joueur, et c'est ce
//! qui le rend propre à chaque contenant sans réglage — un litre lève de 5 cm un bidon de 2 dm², de 20 µm une piscine de 50 m². La
//! comparaison se fait contre le **dernier état publié** : une fuite lente s'accumule jusqu'à être publiée. Rien n'est retiré de la masse
//! (I-10) : le seuil décide de ce qu'on montre et transmet, pas de ce qui existe.

use super::{geometry, Error, HydroNode, Shapes};

/// **L'écart de hauteur de surface** entre l'état présent du nœud et un volume publié, µm, le long de la verticale locale (positif si
/// la surface a monté) : la différence des plans de la géométrie pour les deux volumes. Refus : un volume publié hors de `0..=capacité`
/// (`Capacity`), et ceux de la géométrie.
pub fn ecart_hauteur_um(node: &HydroNode, publie_ml: i64, shapes: &Shapes<'_>, g_eff: [f32; 3]) -> Result<f64, Error> {
    let (up, _) = geometry::vertical(g_eff)?;
    shapes.validate_node(node, up)?;
    if !(0..=node.capacity_ml).contains(&publie_ml) {
        return Err(Error::Capacity);
    }
    let present = shapes.surface_up(node, up)?.offset_um;
    let publie = shapes.surface_up(&HydroNode { volume_ml: publie_ml, ..*node }, up)?.offset_um;
    Ok(present - publie)
}

/// **Le changement est-il significatif ?** — l'écart de hauteur au moins égal au seuil (µm, fini et positif ; `Domain` sinon).
pub fn changement_significatif(node: &HydroNode, publie_ml: i64, shapes: &Shapes<'_>, g_eff: [f32; 3], seuil_um: f64)
    -> Result<bool, Error> {
    if !(seuil_um >= 0.0) || !seuil_um.is_finite() {
        return Err(Error::Domain);
    }
    Ok(ecart_hauteur_um(node, publie_ml, shapes, g_eff)?.abs() >= seuil_um)
}
