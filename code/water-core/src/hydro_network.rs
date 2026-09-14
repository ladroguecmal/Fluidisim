//! Couche **V** — graphe hydraulique des volumes finis (ADR-010), premier module.
//!
//! Les volumes finis ne sont pas de petits océans : ce sont des **contenants** reliés par des
//! **ouvertures**, résolus à basse fréquence et en arithmétique entière, séparément de toute
//! surface libre. Ce module en construit le noyau : nœuds, orifices, pas fixe de 100 ms,
//! quantification en millilitres à report de reste, limiteur avec normalisation.
//!
//! **Ce que les invariants imposent ici, et qui a dicté la forme.**
//! - **I-03** : V est déterministe bit à bit. L'ordre de parcours des arêtes est celui du tableau
//!   fourni, jamais une adresse ni un ordre de conteneur ; les sommes sont faites dans cet ordre.
//! - **I-10** : le serveur exécute V, en arithmétique entière et à 10 Hz. L'état d'un nœud est un
//!   `i64` de millilitres ; aucun flottant ne le porte jamais.
//! - **I-06** : aucune allocation à l'exécution. Nœuds, arêtes et formes sont des tranches de
//!   l'appelant ; le pas n'en demande pas d'autres.
//! - **I-07** : `g_eff` est injectée **en vecteur**, direction comprise (S226). Le plan d'eau est
//!   perpendiculaire à `g_eff`, pas à `Z` : sans cela un vaisseau qui accélère ne verrait pas son
//!   réservoir fuir par le hublot latéral qui se retrouve « en bas » (ADR-010 §2). Le premier
//!   module (S224) n'en prenait que le **module**, ce qui revenait à écrire l'axe en dur — le
//!   défaut qu'I-07 qualifie de bloquant.
//!
//! Ce que ce module ne fait pas : ni réseau fermé sous pression (ADR-010 §4 le reporte
//! explicitement en v2), ni pompe, ni matériau poreux, ni pluie, ni absorption.
//!
//! **ADR-139, S228** : les formes géométriques fournissent un plan orienté conservant le volume.
//! Les anciennes tables n'acceptent que leur orientation cuite +Z. Géométrie, précision et coût
//! restent à recevoir dans le domaine de chaque nouvel usage ; le noyau n'est pas un solveur de
//! ballottement ni un budget temporel certifié.

use crate::SimTime;

#[path = "hydro_geometry.rs"]
pub mod geometry;

#[path = "hydro_snapshot.rs"]
pub mod snapshot;

/// Pas de la couche V — 100 ms, 10 Hz (ADR-010 §4, I-10). Aligné sur `T_sim`.
pub const STEP_US: u64 = 100_000;

/// Entrées des tables historiques volume → hauteur, limitées à +Z par ADR-139.
pub const SHAPE_ENTRIES: usize = 64;

/// Coefficient de débit d'un orifice à arête vive (ADR-010 §3). Sans provenance propre : il vient
/// de l'ADR, qui le tient de la littérature ; l'hôte peut le remplacer par arête.
pub const SHARP_EDGE_DISCHARGE: f32 = 0.62;

/// Coefficient de débit d'un déversoir rectangulaire (ADR-010 §3). Même statut que le précédent :
/// il vient de l'ADR, et l'hôte peut le remplacer par arête.
pub const WEIR_DISCHARGE: f32 = 0.60;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Indice de nœud ou de forme hors des tranches fournies.
    Capacity,
    /// Table de forme mal dimensionnée, non croissante, ou hauteurs non finies.
    Shape,
    /// Pas, gravité ou géométrie inutilisables — faute d'entrée de l'hôte, pas verdict physique.
    Domain,
    /// Un débit calculé n'est pas représentable.
    NonFinite,
    /// Une table horizontale ne définit pas la géométrie sous cette orientation (ADR-139).
    Orientation,
    /// L'inversion géométrique ne tient pas le demi-millilitre de résidu calculé.
    Resolution,
}

/// Contenant. L'état est **entier** : `volume_ml`, et rien d'autre.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HydroNode {
    pub volume_ml: i64,
    pub capacity_ml: i64,
    /// Origine FIXE de la géométrie du contenant, en micromètres dans le référentiel.
    /// Le plan local est `u·x = d`, puis translaté par cette origine (ADR-139).
    /// Pour une ancienne table +Z, l'origine est au niveau de son fond.
    pub origin_um: [i64; 3],
    /// Indice de la forme dans la table fournie.
    pub shape: u16,
}

/// Ouverture entre deux contenants, ou vers l'extérieur.
///
/// `residue_nl` porte le **report de reste** d'ADR-010 §4 : le débit est calculé en flottant puis
/// quantifié en millilitres, et la fraction perdue est conservée d'un pas au suivant. Sans lui, la
/// troncature ne perd aucune masse — un transfert entier reste entier des deux côtés — mais elle
/// **biaise le débit** à chaque pas, toujours dans le même sens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flow {
    /// Orifice noyé ou dénoyé, loi de Torricelli (ADR-010 §3). Section en millimètres carrés.
    Orifice { area_mm2: i64 },
    /// Déversoir rectangulaire, `Q = (2/3)·C_d·b·√(2g)·H^{3/2}` (ADR-010 §3). Largeur en
    /// millimètres. La charge `H` est comptée **au-dessus du seuil**, et c'est ce qui distingue
    /// cette loi de la précédente : un orifice garde sa section quand la charge monte, un
    /// déversoir élargit sa lame.
    Weir { width_mm: i64 },
}
impl Default for Flow {
    fn default() -> Self {
        Flow::Orifice { area_mm2: 0 }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Opening {
    pub from: u16,
    /// `None` : rejet hors réseau. Le volume sort du bilan, et c'est voulu.
    pub to: Option<u16>,
    pub flow: Flow,
    /// Position de l'ouverture, en micromètres, dans le repère du référentiel. **Une ouverture est
    /// quelque part**, pas à une hauteur : c'est ce qui permet à un hublot latéral de se retrouver
    /// « en bas » quand `g_eff` s'incline.
    pub position_um: [i64; 3],
    pub discharge: f32,
    /// Reste fractionnaire, en nanolitres (10⁻⁶ ml). Entretenu par `step`.
    pub residue_nl: i64,
}

/// Formes empruntées à l'hôte : géométrie orientable (ADR-139) ou anciennes tables +Z.
/// Aucun mélange implicite : l'hôte choisit la représentation à la construction.
pub struct Shapes<'a> {
    table: &'a [i64],
    volumes: &'a [geometry::VolumeShape<'a>],
}
impl<'a> Shapes<'a> {
    /// Anciennes tables : hauteur verticale en µm, au volume `i/(SHAPE_ENTRIES-1)` de la capacité.
    /// Croissance et dimension contrôlées une fois ; autre orientation refusée par le pas.
    pub fn new(table: &'a [i64]) -> Result<Self, Error> {
        if table.is_empty() || table.len() % SHAPE_ENTRIES != 0 {
            return Err(Error::Shape);
        }
        for chunk in table.chunks_exact(SHAPE_ENTRIES) {
            if chunk[0] != 0 {
                return Err(Error::Shape);
            }
            for w in chunk.windows(2) {
                if w[1] < w[0] {
                    return Err(Error::Shape);
                }
            }
        }
        Ok(Self { table, volumes: &[] })
    }
    /// Géométries validées avant le pas ; leurs capacités proviennent de leur volume intérieur.
    pub fn from_volumes(volumes: &'a [geometry::VolumeShape<'a>]) -> Result<Self, Error> {
        if volumes.is_empty() { return Err(Error::Shape); }
        Ok(Self { table: &[], volumes })
    }
    pub fn count(&self) -> usize {
        if self.table.is_empty() { self.volumes.len() } else { self.table.len() / SHAPE_ENTRIES }
    }
    fn validate_node(&self, node: &HydroNode, up: [f64; 3]) -> Result<(), Error> {
        if node.shape as usize >= self.count() || node.capacity_ml <= 0
            || !(0..=node.capacity_ml).contains(&node.volume_ml) {
            return Err(Error::Capacity);
        }
        if self.volumes.is_empty() {
            if up != [0.0, 0.0, 1.0] { return Err(Error::Orientation); }
        } else if node.capacity_ml != self.volumes[node.shape as usize].capacity_ml() {
            return Err(Error::Capacity);
        }
        Ok(())
    }
    /// Plan exposé par V, dans les coordonnées locales fixes de la forme. Le pas utilise
    /// exactement le même calcul : aucune reconstruction indépendante par son consommateur.
    pub fn surface_plane(&self, node: &HydroNode, g_eff: [f32; 3]) -> Result<geometry::SurfacePlane, Error> {
        let (up, _) = geometry::vertical(g_eff)?;
        self.validate_node(node, up)?;
        self.surface_up(node, up)
    }
    fn surface_up(&self, node: &HydroNode, up: [f64; 3]) -> Result<geometry::SurfacePlane, Error> {
        if self.volumes.is_empty() {
            Ok(geometry::SurfacePlane {
                up,
                offset_um: self.height_um(node.shape, node.volume_ml, node.capacity_ml)? as f64,
            })
        } else {
            self.volumes[node.shape as usize].plane_up(node.volume_ml, up)
        }
    }
    /// Hauteur de la surface libre au-dessus du fond, en micromètres, par interpolation linéaire
    /// en volume, avec les arrondis en µm des échantillons et du résultat.
    fn height_um(&self, shape: u16, volume_ml: i64, capacity_ml: i64) -> Result<i64, Error> {
        let s = shape as usize;
        if s >= self.count() || capacity_ml <= 0 {
            return Err(Error::Capacity);
        }
        let row = &self.table[s * SHAPE_ENTRIES..(s + 1) * SHAPE_ENTRIES];
        if volume_ml <= 0 {
            return Ok(row[0]);
        }
        if volume_ml >= capacity_ml {
            return Ok(row[SHAPE_ENTRIES - 1]);
        }
        // Position dans la table, en unités de (SHAPE_ENTRIES - 1) fractions de capacité.
        let steps = (SHAPE_ENTRIES - 1) as i128;
        let pos = volume_ml as i128 * steps;
        let i = (pos / capacity_ml as i128) as usize;
        let i = i.min(SHAPE_ENTRIES - 2);
        let frac = pos - i as i128 * capacity_ml as i128;
        let lo = row[i] as i128;
        let hi = row[i + 1] as i128;
        // Arrondi **au plus proche**, pas troncature. Avec la troncature, une tranche de fluide
        // qui vaut un micromètre se lit zéro — la charge s'annule et le contenant cesse de se
        // vider, à un millilitre près sur un million. Le plancher de vidange existe quand même
        // (la hauteur est entière), mais il vaut alors une unité de représentation, pas deux.
        let cap = capacity_ml as i128;
        Ok((lo + ((hi - lo) * frac + cap / 2) / cap) as i64)
    }
}

/// Projection d'un déplacement entier sur la verticale locale, en micromètres.
///
/// Calculée en `f64` depuis des différences **entières** : sous `u = (0, 0, 1)` elle rend
/// exactement `q_z − c_z`, donc la généralisation se réduit au bit au cas vertical. IEEE strict la
/// rend reproductible (I-03) ; le module emploie déjà `f64` pour le débit.
fn sub(a: [i64; 3], b: [i64; 3]) -> [i128; 3] {
    // Élargir AVANT la soustraction : deux coordonnées valides peuvent avoir une différence
    // hors i64. Convertir chaque coordonnée en flottant avant soustraction perdrait les petits
    // déplacements à grande origine ; i128 conserve les deux propriétés.
    [a[0] as i128 - b[0] as i128, a[1] as i128 - b[1] as i128, a[2] as i128 - b[2] as i128]
}

fn along(delta: [i128; 3], u: [f64; 3]) -> f64 {
    delta[0] as f64 * u[0] + delta[1] as f64 * u[1] + delta[2] as f64 * u[2]
}

/// Un pas de la couche V.
///
/// **Déroulé, et chaque étape répond à une ligne d'ADR-010 §4.**
/// 1. Contrôles d'entrée, avant toute écriture — le refus est **atomique**.
/// 2. Pour chaque arête, dans l'ordre du tableau : charge `Δh`, débit de Torricelli, volume du pas,
///    ajout du reste, quantification en millilitres, reste conservé.
/// 3. **Normalisation** : quand plusieurs arêtes vident le même nœud au-delà de ce qu'il contient,
///    toutes sont réduites dans la même proportion. Sans cela un nœud presque vide alimente trois
///    fuites et devient négatif.
/// 4. Limiteur d'arrivée : **somme des transferts entrants ≤ capacité libre aval**. Un plafond
///    par arête seul n'empêche pas plusieurs arrivées de remplir la même place (S227).
/// 5. Application.
///
/// `scratch` reçoit un transfert par arête ; il appartient à l'appelant (I-06).
pub fn step(
    nodes: &mut [HydroNode],
    edges: &mut [Opening],
    shapes: &Shapes<'_>,
    g_eff: [f32; 3],
    dt: SimTime,
    scratch: &mut [i64],
) -> Result<(), Error> {
    if scratch.len() < edges.len() {
        return Err(Error::Capacity);
    }
    if dt.0 == 0 {
        return Err(Error::Domain);
    }
    let (up, magnitude) = geometry::vertical(g_eff)?;
    for e in edges.iter() {
        let size = match e.flow {
            Flow::Orifice { area_mm2 } => area_mm2,
            Flow::Weir { width_mm } => width_mm,
        };
        if e.from as usize >= nodes.len()
            || e.to.is_some_and(|t| t as usize >= nodes.len())
            || size < 0
            || !(e.discharge >= 0.0)
            || !e.discharge.is_finite()
        {
            return Err(Error::Capacity);
        }
    }
    for n in nodes.iter() {
        shapes.validate_node(n, up)?;
    }
    let dt_s = dt.0 as f64 * 1e-6;

    // --- 2. Débit par arête, dans l'ordre du tableau (I-03).
    for (e, out) in edges.iter().zip(scratch.iter_mut()) {
        *out = 0;
        let src = nodes[e.from as usize];
        let h_up = shapes.surface_up(&src, up)?.offset_um;
        // Cote de l'ouverture au-dessus du point de référence amont, **le long de la verticale
        // locale** : c'est la « distance signée au plan de surface » d'ADR-010 §2.
        let sill = along(sub(e.position_um, src.origin_um), up);
        // Charge en aval : la surface du receveur ramenée au même repère, ou le seuil si l'arête
        // rejette hors réseau. Le maximum interdit une charge négative — une ouverture au-dessus
        // de la surface aval ne débite pas plus qu'à l'air libre.
        let downstream = match e.to {
            Some(t) => {
                let dn = nodes[t as usize];
                let h_dn = shapes.surface_up(&dn, up)?.offset_um;
                let dn_surface = along(sub(dn.origin_um, src.origin_um), up) + h_dn;
                dn_surface.max(sill)
            }
            None => sill,
        };
        if h_up <= downstream {
            continue;
        }
        let head_m = (h_up - downstream) * 1e-6;
        let g = 2.0 * magnitude;
        let q_m3s = match e.flow {
            // Torricelli, ADR-010 §3. Section en mm² → m² : 1e-6.
            Flow::Orifice { area_mm2 } => {
                e.discharge as f64 * (area_mm2 as f64 * 1e-6) * (g * head_m).sqrt()
            }
            // Déversoir rectangulaire, ADR-010 §3. Largeur en mm → m : 1e-3. La charge est comptée
            // au-dessus du **seuil** ; `head_m` la porte déjà, le seuil entrant dans `downstream`.
            Flow::Weir { width_mm } => {
                (2.0 / 3.0) * e.discharge as f64 * (width_mm as f64 * 1e-3)
                    * g.sqrt()
                    * head_m.powf(1.5)
            }
        };
        if !q_m3s.is_finite() {
            return Err(Error::NonFinite);
        }
        // Volume du pas, en **nanolitres** : m³ → ml est 1e6, ml → nl est 1e6.
        let nl = q_m3s * dt_s * 1e12;
        if !nl.is_finite() || nl.abs() >= i64::MAX as f64 {
            return Err(Error::NonFinite);
        }
        *out = nl as i64;
    }

    // --- 3. Quantification avec report de reste, puis limiteur d'arrivée par arête.
    for (e, want) in edges.iter_mut().zip(scratch.iter_mut()) {
        let total_nl = (*want as i128) + (e.residue_nl as i128);
        if total_nl <= 0 {
            e.residue_nl = 0;
            *want = 0;
            continue;
        }
        let mut ml = (total_nl / 1_000_000) as i64;
        if let Some(t) = e.to {
            let free = nodes[t as usize].capacity_ml - nodes[t as usize].volume_ml;
            if ml > free {
                ml = free.max(0);
            }
        }
        // Le reste est ce qui n'a pas franchi le millilitre, **borné à un millilitre** : une arête
        // affamée ne doit pas accumuler une dette qu'elle déverserait d'un coup au premier
        // millilitre disponible. Ce qu'une vidange n'a pas eu lieu de faire n'a pas eu lieu.
        e.residue_nl = (total_nl - ml as i128 * 1_000_000).clamp(0, 999_999) as i64;
        *want = ml;
    }

    // --- 4. Normalisation par nœud amont, **en millilitres**.
    //
    // ADR-010 §4 : quand plusieurs arêtes vident le même nœud au-delà de ce qu'il contient, toutes
    // sont réduites dans la même proportion. La normalisation se fait ici **après** quantification
    // et non avant : à l'échelle du nanolitre, répartir proportionnellement un contenant presque
    // vide donne à chaque arête une part inférieure au millilitre, qui s'arrondit à zéro — et le
    // contenant cesse de se vider tout en gardant de la charge.
    //
    // La répartition se fait par **arrondi cumulatif** : la part de l'arête `k` est la différence
    // des sommes proportionnelles arrondies jusqu'à `k` et jusqu'à `k-1`. Les parts somment alors
    // **exactement** au volume disponible, chacune est à moins d'un millilitre de sa valeur
    // proportionnelle, et l'ordre du tableau suffit à la reproduire (I-03) — aucun reste à stocker.
    for i in 0..nodes.len() {
        let mut asked: i128 = 0;
        for (e, ml) in edges.iter().zip(scratch.iter()) {
            if e.from as usize == i {
                asked += *ml as i128;
            }
        }
        let available = nodes[i].volume_ml as i128;
        if asked <= available || asked <= 0 {
            continue;
        }
        let (mut cum, mut cum_given) = (0i128, 0i128);
        for (e, ml) in edges.iter().zip(scratch.iter_mut()) {
            if e.from as usize != i {
                continue;
            }
            cum += *ml as i128;
            let target = cum * available / asked;
            *ml = (target - cum_given) as i64;
            cum_given = target;
        }
    }

    // --- 5. Capacité collective des receveurs (S227).
    // Chaque arête a déjà été bornée et chaque source normalisée. Réduire encore ses transferts
    // ne peut donc rendre une source négative. Le receveur partage sa place libre initiale par
    // le même arrondi cumulatif ; la place libérée pendant ce pas ne sera disponible qu'au pas
    // suivant. Pas de redistribution itérative ni de dette accumulée pour le transfert refusé.
    for (i, node) in nodes.iter().enumerate() {
        let asked: i128 = edges.iter().zip(scratch.iter())
            .filter(|(e, _)| e.to.is_some_and(|t| t as usize == i))
            .map(|(_, ml)| *ml as i128).sum();
        let free = (node.capacity_ml - node.volume_ml) as i128;
        if asked <= free {
            continue;
        }
        let (mut cumulative, mut given) = (0i128, 0i128);
        for (e, ml) in edges.iter().zip(scratch.iter_mut()) {
            if !e.to.is_some_and(|t| t as usize == i) {
                continue;
            }
            cumulative += *ml as i128;
            let target = cumulative * free / asked;
            *ml = (target - given) as i64;
            given = target;
        }
    }

    // --- 6. Application, dans l'ordre du tableau. Le retrait précède l'ajout ; la somme des
    // volumes est conservée par construction, et rien ne dépasse ce qui a été normalisé.
    for (e, ml) in edges.iter().zip(scratch.iter()) {
        if *ml <= 0 {
            continue;
        }
        nodes[e.from as usize].volume_ml -= *ml;
        if let Some(t) = e.to {
            nodes[t as usize].volume_ml += *ml;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests_hydro_network.rs"]
mod tests;
