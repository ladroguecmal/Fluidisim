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
//! explicitement en v2), ni matériau poreux, ni pluie, ni absorption.
//!
//! **ADR-199, S372** : chaque arête porte une **commande** (`control_pm`, 0 à 1 000), état répliqué posé par l'hôte
//! entre deux pas : un orifice ou un déversoir commandé est une **vanne** ; la **pompe** est une loi à part, en réseau
//! ouvert.
//!
//! **ADR-204, S378** : la **pluie** est une arête du ciel vers un nœud (surface d'ouverture × exposition × intensité) ;
//! l'exposition est sa commande (bâche, demi-bâche), l'intensité une entrée du pas (`step_meteo`, `Meteo`).
//!
//! **ADR-139, S228** : les formes géométriques fournissent un plan orienté conservant le volume.
//! Les anciennes tables n'acceptent que leur orientation cuite +Z. Géométrie, précision et coût
//! restent à recevoir dans le domaine de chaque nouvel usage ; le noyau n'est pas un solveur de
//! ballottement ni un budget temporel certifié.

use crate::SimTime;

#[path = "hydro_geometry.rs"]
pub mod geometry;

/// S559 — les liquides de V (ADR-241).
#[path = "hydro_liquids.rs"]
pub mod liquids;

/// S564 — le seuil adaptatif à l'échelle du contenant (liste 5.6).
#[path = "hydro_seuil.rs"]
pub mod seuil;

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

/// Commande pleine d'une arête (ADR-199 D1) : vanne ouverte, pompe à sa vitesse nominale. À cette valeur, les lois sont
/// celles d'avant la commande, au bit.
pub const CONTROL_FULL: i64 = 1_000;

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
    /// **Pompe centrifuge en réseau ouvert** (ADR-199 D3). La prise est la position de l'arête, dans le nœud amont ; le
    /// refoulement, `outlet_um`. Courbe parabolique `H(Q) = H0·(1 − (Q/Qmax)²)`, point de fonctionnement contre la
    /// hauteur statique `Δh` = cote de refoulement (la sortie, ou la surface du receveur si elle la noie) − surface amont :
    /// `Q = Qmax·√(n² − Δh/H0)`, zéro si le radical ne l'est pas (clapet : jamais de retour) et zéro si la surface amont
    /// est sous la prise (à sec). Vitesse `n = commande/1 000`, lois de similitude. Débit maximal en millilitres par
    /// seconde, hauteur de barrage `H0` en micromètres ; données de l'auteur de l'arête.
    Pump { max_flow_mlps: i64, shutoff_head_um: i64, outlet_um: [i64; 3] },
    /// **S515 — une vanne et sa courbe** (ADR-199 §3 : « remplacer la section par une table ») : Torricelli, la section multipliée par
    /// la courbe d'ouverture du constructeur — la fraction de débit, en pour mille, aux ouvertures 0, 100, …, 1 000 ‰ de la commande,
    /// interpolée linéairement (linéaire, à pourcentage égal, à ouverture rapide…). La courbe ne décroît pas et reste sous 1 000.
    Valve { area_mm2: i64, curve_pm: [u16; 11] },
    /// **S515 — une pompe sur sa conduite** : la loi de `Pump`, plus la perte de charge de la conduite `K·Q²` (`loss_um_per_l2s2`, en
    /// micromètres par (litre/seconde)²) et le rendement (`efficiency_pm`, pour mille, 1 à 1 000). Point de fonctionnement
    /// `H₀·n² − H₀·(Q/Qmax)² = Δh + K·Q²`, soit `Q = √((n²·H₀ − Δh)/(H₀/Qmax² + K))` ; zéro si le radical ne l'est pas, à sec, ou à
    /// commande nulle. La puissance (`pump_operating_point`) : hydraulique `ρ·g·Q·(Δh + K·Q²)`, à l'arbre celle-ci divisée par le
    /// rendement.
    PumpLine { max_flow_mlps: i64, shutoff_head_um: i64, outlet_um: [i64; 3], loss_um_per_l2s2: i64, efficiency_pm: i64 },
    /// **Pluie** (ADR-204) : du ciel vers le nœud que `from` et `to` désignent tous deux. Débit `intensité × surface
    /// d'ouverture × exposition` — l'intensité vient du pas (`Meteo`), la surface d'ouverture est une donnée d'auteur (mm²,
    /// horizontale : la pluie qui tombe dans l'ouverture d'un contenant finit dans son eau), l'exposition est la commande
    /// de l'arête (bâche entière : 0 ; demi-bâche : 500). Ne vide aucun nœud ; bornée par la place libre du receveur.
    Rain { catchment_mm2: i64 },
    /// **Débordement** (S489, liste 5.3) : le trop-plein du nœud `from` **vers l'extérieur** (`to` vaut `None` ; vers un autre nœud,
    /// le pas est refusé, `Error::Domain` — une chaîne de capacités n'est pas dans cette version). Quand le nœud reçoit, dans un pas,
    /// plus que sa place libre (arêtes, pluie), l'excédent n'est plus refusé : il passe par-dessus le bord, par cette arête ; le nœud
    /// finit plein. L'hôte lit le volume déversé dans `scratch` à l'indice de l'arête, et la position de l'arête lui dit où le
    /// déposer (le sol, δ, la mer). La première arête de débordement d'un nœud, dans l'ordre du tableau, le porte (I-03). Sans
    /// arête de débordement, le transfert est refusé comme avant (S227), au bit.
    Spill,
    /// **S530 — l'infiltration dans le sol, Green–Ampt** (liste 5.5) : de la flaque `from` (la surface du sol à la cote de l'arête, `h₀`
    /// la lame au-dessus) vers le sol `to` (obligatoire), dont le remplissage rapporté à l'aire `area_mm2` est la lame infiltrée cumulée
    /// `F`. Capacité d'infiltration `f = K·(1 + (ψ + h₀)·Δθ/F)` — conductivité à saturation `K` (`conductivity_nm_s`, nm/s), succion au
    /// front `ψ` (`suction_um`), déficit d'humidité `Δθ` (`deficit_pm`, ‰). Intégrée **exactement** sur le pas : `t(F) = (F − M ln(1 +
    /// F/M))/K`, `M = (ψ + h₀)Δθ`, inversée par bissection. Le sol plein, le limiteur d'arrivée l'arrête ; la flaque à sec, rien ne passe.
    Infiltration { area_mm2: i64, conductivity_nm_s: i64, suction_um: i64, deficit_pm: i64 },
    /// **S535 — le drainage gravitaire d'un sol** (liste 5.5) : du sol `from` vers le dessous (`to` : une nappe, ou dehors),
    /// `q = K·Sᶜ` par unité d'aire (Brooks–Corey, gradient unitaire), `S` le remplissage du nœud rapporté à sa capacité, `c` =
    /// `exponent_pm`/1 000 (≥ 1) ; la lame de stockage est la capacité rapportée à l'aire `area_mm2`. Intégré exactement sur le pas :
    /// `dS/dt = −a·Sᶜ`, `a = K/lame` — `S₁ = (S₀^{1−c} + (c − 1)·a·dt)^{1/(1−c)}`, `S₀·e^{−a·dt}` pour `c` = 1.
    Drainage { area_mm2: i64, conductivity_nm_s: i64, exponent_pm: i64 },
    /// **S535 — l'évaporation** : du nœud `from` vers dehors (`to` vaut `None`), au taux potentiel d'auteur `rate_nm_s` sur l'aire
    /// `area_mm2`, fois la commande (l'exposition ; la météo viendra à la fin, ADR-197 D5) ; bornée par ce que le nœud contient.
    Evaporation { area_mm2: i64, rate_nm_s: i64 },
    /// **S547 — l'évent d'un compartiment** (ADR-015 §2, liste 5.9) : du nœud `from` vers l'air libre (`to` vaut `None`) ; aucune eau n'y
    /// passe. Sous `step_air`, l'air d'une poche scellée en sort à `Q_a = C_d·A·√(2Δp/ρ_a)` (`C_d` : `discharge` ; `ρ_a = 1,2·p/p_atm`,
    /// isotherme), et le produit `p·V_air` de la poche baisse de `p·Q_a·dt`. Ailleurs (`step`, nœud ouvert), sans effet.
    Vent { area_mm2: i64 },
}

/// **La météo du pas** (ADR-204 D4) : une entrée, fournie à l'identique à tous les participants (I-03) ; qui la calcule
/// relève de la météo (ADR-203 D1). Une valeur pour tout le réseau.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Meteo {
    /// Intensité de la pluie, mm/h ; finie et positive.
    pub pluie_mm_h: f32,
}
impl Meteo {
    /// Temps sec : le pas de V d'avant la pluie, au bit.
    pub const SEC: Meteo = Meteo { pluie_mm_h: 0.0 };
}
impl Default for Flow {
    fn default() -> Self {
        Flow::Orifice { area_mm2: 0 }
    }
}

#[derive(Clone, Copy, Debug)]
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
    /// **Commande**, en pour mille (ADR-199 D1) : ouverture d'une vanne, vitesse d'une pompe. Un **état**, comme le
    /// volume d'un nœud : posé par l'hôte entre deux pas, depuis un événement de jeu répliqué ; V ne la décide jamais.
    /// `CONTROL_FULL` par défaut ; hors de 0..=1 000, le pas est refusé.
    pub control_pm: i64,
}
impl Default for Opening {
    fn default() -> Self {
        Self {
            from: 0,
            to: None,
            flow: Flow::default(),
            position_um: [0; 3],
            discharge: 0.0,
            residue_nl: 0,
            control_pm: CONTROL_FULL,
        }
    }
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
    step_meteo(nodes, edges, shapes, g_eff, Meteo::SEC, dt, scratch)
}

/// Un pas de la couche V **sous une météo** (ADR-204) : `step` avec l'intensité de la pluie. Refus `Domain` si l'intensité
/// n'est pas finie et positive ; une arête de pluie dont `from` et `to` ne désignent pas le même nœud est refusée
/// (`Capacity`), comme toute arête mal formée.
pub fn step_meteo(
    nodes: &mut [HydroNode],
    edges: &mut [Opening],
    shapes: &Shapes<'_>,
    g_eff: [f32; 3],
    meteo: Meteo,
    dt: SimTime,
    scratch: &mut [i64],
) -> Result<(), Error> {
    step_inner(nodes, edges, shapes, g_eff, meteo, dt, scratch, None, None)
}

/// **S538 — l'état de l'air d'un nœud** (ADR-015 T2) : ouvert (l'air à la pression atmosphérique, T0) ou **scellé** — une poche
/// isotherme dont le produit `p·V_air` (Pa·ml) est constant ; sa pression `p·V/(capacité − volume)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Air {
    Open,
    Sealed { pv_pa_ml: f64 },
}

/// S538 : la pression atmosphérique de référence, Pa.
pub const P_ATM_PA: f64 = 101_325.0;

/// **S538 — un pas de V avec l'air des compartiments** (liste 5.9, C17, ADR-015 T2) : `step_meteo`, la pression de jauge de chaque poche
/// scellée ajoutée aux charges des arêtes, `(p − p_atm)/(ρ·g)` — l'eau qui entre comprime l'air, qui la retient. `air` : un état par
/// nœud ; `heads_um` : un tampon de l'appelant, un par nœud (I-06). Tous ouverts : `step_meteo` au bit. Une poche dont l'air disparaîtrait
/// (volume ≥ capacité) est refusée (`Domain`). Les pompes ne voient pas la pression des poches.
#[allow(clippy::too_many_arguments)]
pub fn step_air(
    nodes: &mut [HydroNode],
    edges: &mut [Opening],
    shapes: &Shapes<'_>,
    g_eff: [f32; 3],
    meteo: Meteo,
    dt: SimTime,
    scratch: &mut [i64],
    air: &mut [Air],
    density: f64,
    heads_um: &mut [f64],
) -> Result<(), Error> {
    if air.len() != nodes.len() || heads_um.len() != nodes.len() || !(density > 0.0) || !density.is_finite() {
        return Err(Error::Capacity);
    }
    if air.iter().all(|a| *a == Air::Open) {
        return step_inner(nodes, edges, shapes, g_eff, meteo, dt, scratch, None, None);
    }
    let (_, g) = geometry::vertical(g_eff)?;
    for ((n, a), h) in nodes.iter().zip(air.iter()).zip(heads_um.iter_mut()) {
        *h = match *a {
            Air::Open => 0.0,
            Air::Sealed { pv_pa_ml } => {
                let v_air = (n.capacity_ml - n.volume_ml) as f64;
                if !(v_air > 0.0) || !(pv_pa_ml > 0.0) || !pv_pa_ml.is_finite() {
                    return Err(Error::Domain);
                }
                (pv_pa_ml / v_air - P_ATM_PA) / (density * g) * 1e6
            }
        };
    }
    step_inner(nodes, edges, shapes, g_eff, meteo, dt, scratch, Some(heads_um), None)?;
    vent_air(nodes, edges, air, dt, |i| heads_um[i] * 1e-6 * density * g);
    Ok(())
}

/// S547 : l'air qui sort par les évents, à la pression du début du pas (celle des charges, isotherme) ; jamais sous la pression
/// atmosphérique. `jauge` : la pression de jauge de chaque poche au début du pas, Pa. S563 : partagé avec `liquids::step_liquids_air`.
fn vent_air(nodes: &[HydroNode], edges: &[Opening], air: &mut [Air], dt: SimTime, jauge: impl Fn(usize) -> f64) {
    let dt_s = dt.0 as f64 * 1e-6;
    for e in edges.iter() {
        let Flow::Vent { area_mm2 } = e.flow else { continue };
        let i = e.from as usize;
        let Air::Sealed { pv_pa_ml } = air[i] else { continue };
        let n = nodes[i];
        let v_air = (n.capacity_ml - n.volume_ml) as f64;
        let p = jauge(i) + P_ATM_PA;
        let surpression = p - P_ATM_PA;
        if !(surpression > 0.0) || e.control_pm == 0 {
            continue;
        }
        let rho_air = 1.2 * p / P_ATM_PA;
        let q_m3s = e.discharge as f64 * (area_mm2 as f64 * 1e-6) * (2.0 * surpression / rho_air).sqrt()
            * (e.control_pm as f64 / CONTROL_FULL as f64);
        let sortie = p * q_m3s * dt_s * 1e6;
        air[i] = Air::Sealed { pv_pa_ml: (pv_pa_ml - sortie).max(P_ATM_PA * v_air) };
    }
}

#[allow(clippy::too_many_arguments)]
fn step_inner(
    nodes: &mut [HydroNode],
    edges: &mut [Opening],
    shapes: &Shapes<'_>,
    g_eff: [f32; 3],
    meteo: Meteo,
    dt: SimTime,
    scratch: &mut [i64],
    pression_um: Option<&[f64]>,
    liquides: Option<(&[i64], &[liquids::Liquid], Option<&[f64]>)>,
) -> Result<(), Error> {
    // S538 : la charge de pression d'une poche, en µm ; zéro sans air scellé (le pas d'avant au bit : `x + 0.0` est `x`).
    let charge = |i: usize| pression_um.map_or(0.0, |p| p[i]);
    if !(meteo.pluie_mm_h >= 0.0) || !meteo.pluie_mm_h.is_finite() {
        return Err(Error::Domain);
    }
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
            // Une pompe sans hauteur de barrage ne se représente pas (division par H0) : refusée comme une taille.
            Flow::Pump { max_flow_mlps, shutoff_head_um, .. } => {
                if shutoff_head_um <= 0 { -1 } else { max_flow_mlps }
            }
            Flow::Valve { .. } | Flow::PumpLine { .. } => law_size(&e.flow),
            // S530 : l'infiltration va vers un sol.
            Flow::Infiltration { .. } => if e.to.is_none() { -1 } else { law_size(&e.flow) },
            Flow::Drainage { .. } => law_size(&e.flow),
            Flow::Evaporation { .. } | Flow::Vent { .. } => if e.to.is_some() { -1 } else { law_size(&e.flow) },
            // La pluie tombe sur le nœud que `from` et `to` désignent tous deux.
            Flow::Rain { catchment_mm2 } => {
                if e.to != Some(e.from) { -1 } else { catchment_mm2 }
            }
            // S489 : le débordement va dehors ; vers un nœud, refusé plus bas (`Error::Domain`).
            Flow::Spill => 0,
        };
        if matches!(e.flow, Flow::Spill) && e.to.is_some() {
            return Err(Error::Domain);
        }
        if e.from as usize >= nodes.len()
            || e.to.is_some_and(|t| t as usize >= nodes.len())
            || size < 0
            || !(0..=CONTROL_FULL).contains(&e.control_pm)
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
        // S489 : le débordement ne débite que l'excédent des arrivées (étape 5). S547 : l'évent ne débite que de l'air (`step_air`).
        if matches!(e.flow, Flow::Spill | Flow::Vent { .. }) {
            continue;
        }
        // ADR-204 : la pluie, sans lecture de surface — intensité (mm/h → m/s) × ouverture (mm² → m²) × exposition.
        if let Flow::Rain { catchment_mm2 } = e.flow {
            if meteo.pluie_mm_h == 0.0 || e.control_pm == 0 {
                continue;
            }
            let q_m3s = meteo.pluie_mm_h as f64 * (1e-3 / 3600.0) * (catchment_mm2 as f64 * 1e-6)
                * (e.control_pm as f64 / CONTROL_FULL as f64);
            let nl = q_m3s * dt_s * 1e12;
            if !nl.is_finite() || nl.abs() >= i64::MAX as f64 {
                return Err(Error::NonFinite);
            }
            *out = nl as i64;
            continue;
        }
        // S535 : le drainage d'un sol, intégré exactement ; l'évaporation au taux d'auteur. Ni l'un ni l'autre ne lit de surface.
        if let Flow::Drainage { area_mm2, conductivity_nm_s, exponent_pm } = e.flow {
            let sol = nodes[e.from as usize];
            if sol.volume_ml <= 0 || sol.capacity_ml <= 0 || e.control_pm == 0 {
                continue;
            }
            let s0 = sol.volume_ml as f64 / sol.capacity_ml as f64;
            let lame = sol.capacity_ml as f64 * 1e-6 / (area_mm2 as f64 * 1e-6);
            let a_dt = conductivity_nm_s as f64 * 1e-9 / lame * dt_s * (e.control_pm as f64 / CONTROL_FULL as f64);
            let c = exponent_pm as f64 / 1000.;
            let s1 = if exponent_pm == 1000 { s0 * (-a_dt).exp() } else { (s0.powf(1. - c) + (c - 1.) * a_dt).powf(1. / (1. - c)) };
            let nl = (s0 - s1) * sol.capacity_ml as f64 * 1e6;
            if !nl.is_finite() || nl.abs() >= i64::MAX as f64 {
                return Err(Error::NonFinite);
            }
            *out = nl as i64;
            continue;
        }
        if let Flow::Evaporation { area_mm2, rate_nm_s } = e.flow {
            let q_m3s = rate_nm_s as f64 * 1e-9 * (area_mm2 as f64 * 1e-6) * (e.control_pm as f64 / CONTROL_FULL as f64);
            let nl = q_m3s * dt_s * 1e12;
            if !nl.is_finite() || nl.abs() >= i64::MAX as f64 {
                return Err(Error::NonFinite);
            }
            *out = nl as i64;
            continue;
        }
        let src = nodes[e.from as usize];
        let h_up = shapes.surface_up(&src, up)?.offset_um + charge(e.from as usize);
        // Cote de l'ouverture au-dessus du point de référence amont, **le long de la verticale
        // locale** : c'est la « distance signée au plan de surface » d'ADR-010 §2.
        let sill = along(sub(e.position_um, src.origin_um), up);
        if let Flow::Infiltration { area_mm2, conductivity_nm_s, suction_um, deficit_pm } = e.flow {
            // S530 : à sec (la surface de la flaque sous celle du sol) ou fermée, rien ne passe.
            if h_up <= sill || e.control_pm == 0 {
                continue;
            }
            let aire = area_mm2 as f64 * 1e-6;
            let sol = nodes[e.to.expect("vérifié") as usize];
            let f0 = sol.volume_ml as f64 * 1e-6 / aire;
            let m = (suction_um as f64 * 1e-6 + (h_up - sill) * 1e-6) * (deficit_pm as f64 / 1000.);
            let f1 = green_ampt_step(f0, m, conductivity_nm_s as f64 * 1e-9 * dt_s);
            let nl = (f1 - f0) * aire * 1e12;
            if !nl.is_finite() || nl.abs() >= i64::MAX as f64 {
                return Err(Error::NonFinite);
            }
            *out = nl as i64;
            continue;
        }
        if let Flow::PumpLine { .. } = e.flow {
            // S515 : la pompe sur sa conduite — le point de fonctionnement, partagé avec `pump_operating_point`.
            let Some(point) = pump_line_point(nodes, e, shapes, up, 1000.)? else {
                continue;
            };
            let nl = point.flow_m3s * dt_s * 1e12;
            if !nl.is_finite() || nl.abs() >= i64::MAX as f64 {
                return Err(Error::NonFinite);
            }
            *out = nl as i64;
            continue;
        }
        if let Flow::Pump { max_flow_mlps, shutoff_head_um, outlet_um } = e.flow {
            // ADR-199 D3. À sec si la surface amont n'atteint pas la prise ; arrêtée à commande nulle.
            if h_up <= sill || e.control_pm == 0 {
                continue;
            }
            let delivery = pump_delivery(nodes, e, shapes, up, outlet_um)?;
            let n = e.control_pm as f64 / CONTROL_FULL as f64;
            let radical = n * n - (delivery - h_up) / shutoff_head_um as f64;
            if !(radical > 0.0) {
                continue;
            }
            let q_m3s = max_flow_mlps as f64 * 1e-6 * radical.sqrt();
            let nl = q_m3s * dt_s * 1e12;
            if !nl.is_finite() || nl.abs() >= i64::MAX as f64 {
                return Err(Error::NonFinite);
            }
            *out = nl as i64;
            continue;
        }
        // S560 (ADR-241 D4) : sous plusieurs liquides, un orifice ou une vanne débite sur la différence de **pression** au seuil, ramenée
        // en hauteur du liquide qui sort ; la comparaison des surfaces ne vaut plus (un côté chargé d'huile a sa surface plus haute à
        // l'équilibre).
        let par_couches = match (liquides, e.flow) {
            (Some((composition, table, poches)), Flow::Orifice { .. } | Flow::Valve { .. }) => {
                Some(liquids::orifice_head_m(nodes, e, shapes, g_eff, composition, table, poches)?)
            }
            _ => None,
        };
        let head_m = match par_couches {
            Some(None) => continue,
            Some(Some(_)) if e.control_pm == 0 => continue,
            Some(Some(h)) => h,
            None => {
                // Charge en aval : la surface du receveur ramenée au même repère, ou le seuil si l'arête
                // rejette hors réseau. Le maximum interdit une charge négative — une ouverture au-dessus
                // de la surface aval ne débite pas plus qu'à l'air libre.
                let downstream = match e.to {
                    Some(t) => {
                        let dn = nodes[t as usize];
                        let h_dn = shapes.surface_up(&dn, up)?.offset_um + charge(t as usize);
                        let dn_surface = along(sub(dn.origin_um, src.origin_um), up) + h_dn;
                        dn_surface.max(sill)
                    }
                    None => sill,
                };
                if h_up <= downstream || e.control_pm == 0 {
                    continue;
                }
                (h_up - downstream) * 1e-6
            }
        };
        // La vanne (ADR-199 D2) : la section ou la largeur réduite dans le rapport de la commande. À commande pleine le
        // facteur vaut exactement 1, et le débit est celui d'avant, au bit.
        let ouverture = e.control_pm as f64 / CONTROL_FULL as f64;
        let g = 2.0 * magnitude;
        let q_m3s = match e.flow {
            // Torricelli, ADR-010 §3. Section en mm² → m² : 1e-6.
            Flow::Orifice { area_mm2 } => {
                e.discharge as f64 * (area_mm2 as f64 * 1e-6) * (g * head_m).sqrt() * ouverture
            }
            // Déversoir rectangulaire, ADR-010 §3. Largeur en mm → m : 1e-3. La charge est comptée
            // au-dessus du **seuil** ; `head_m` la porte déjà, le seuil entrant dans `downstream`.
            Flow::Weir { width_mm } => {
                (2.0 / 3.0) * e.discharge as f64 * (width_mm as f64 * 1e-3)
                    * g.sqrt()
                    * head_m.powf(1.5)
                    * ouverture
            }
            // S515 : la vanne et sa courbe — la section multipliée par la fraction de débit tabulée à cette ouverture.
            Flow::Valve { area_mm2, curve_pm } => {
                e.discharge as f64 * (area_mm2 as f64 * 1e-6) * (g * head_m).sqrt() * valve_fraction(&curve_pm, e.control_pm)
            }
            Flow::Pump { .. } | Flow::PumpLine { .. } | Flow::Rain { .. } | Flow::Spill | Flow::Infiltration { .. } | Flow::Drainage { .. }
            | Flow::Evaporation { .. } | Flow::Vent { .. } => {
                unreachable!("pompes, pluie, débordement et infiltration ont leur propre calcul")
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
    for k in 0..edges.len() {
        // S489 : le receveur déborde-t-il ? (une lecture du tableau, avant d'emprunter l'arête en écriture ; aucune allocation)
        let deborde = edges[k].to.is_some_and(|t| edges.iter().any(|x| x.from == t && matches!(x.flow, Flow::Spill)));
        let (e, want) = (&mut edges[k], &mut scratch[k]);
        let total_nl = (*want as i128) + (e.residue_nl as i128);
        if total_nl <= 0 {
            e.residue_nl = 0;
            *want = 0;
            continue;
        }
        let mut ml = (total_nl / 1_000_000) as i64;
        if let Some(t) = e.to {
            let free = nodes[t as usize].capacity_ml - nodes[t as usize].volume_ml;
            // S489 : un receveur qui déborde ne borne pas ses arrivées — l'excédent sortira par son débordement (étape 5).
            if ml > free && !deborde {
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
            // ADR-204 : une arête de pluie ne vide pas son nœud.
            if e.from as usize == i && !matches!(e.flow, Flow::Rain { .. }) {
                asked += *ml as i128;
            }
        }
        let available = nodes[i].volume_ml as i128;
        if asked <= available || asked <= 0 {
            continue;
        }
        let (mut cum, mut cum_given) = (0i128, 0i128);
        for (e, ml) in edges.iter().zip(scratch.iter_mut()) {
            if e.from as usize != i || matches!(e.flow, Flow::Rain { .. }) {
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
        // S489 : un nœud qui déborde accepte tout ; l'excédent sort par sa première arête de débordement.
        if let Some(k) = edges.iter().position(|e| e.from as usize == i && matches!(e.flow, Flow::Spill)) {
            scratch[k] += (asked - free) as i64;
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
        // ADR-204 : la pluie vient du ciel ; elle n'est retirée de rien.
        if !matches!(e.flow, Flow::Rain { .. }) {
            nodes[e.from as usize].volume_ml -= *ml;
        }
        if let Some(t) = e.to {
            nodes[t as usize].volume_ml += *ml;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests_hydro_network.rs"]
mod tests;

#[cfg(test)]
#[path = "tests_hydro_controles.rs"]
mod tests_controles;

#[cfg(test)]
#[path = "tests_infiltration.rs"]
mod tests_infiltration;

#[cfg(test)]
#[path = "tests_air.rs"]
mod tests_air;

#[cfg(test)]
#[path = "tests_liquids.rs"]
mod tests_liquids;

#[cfg(test)]
#[path = "tests_seuil.rs"]
mod tests_seuil;


/// **S515 — la taille d'une loi de S515** pour la validation (négative : refusée) — partagée par le pas et l'instantané (L137).
pub(crate) fn law_size(flow: &Flow) -> i64 {
    match *flow {
        Flow::Valve { area_mm2, curve_pm } => {
            if curve_pm.iter().all(|c| *c <= 1000) && curve_pm.windows(2).all(|w| w[0] <= w[1]) { area_mm2 } else { -1 }
        }
        Flow::PumpLine { max_flow_mlps, shutoff_head_um, loss_um_per_l2s2, efficiency_pm, .. } => {
            if shutoff_head_um <= 0 || loss_um_per_l2s2 < 0 || !(1..=1000).contains(&efficiency_pm) { -1 } else { max_flow_mlps }
        }
        // S530 : une aire positive, des paramètres de sol positifs, un déficit en ‰. (Le sol receveur, `to`, est vérifié par l'appelant.)
        Flow::Infiltration { area_mm2, conductivity_nm_s, suction_um, deficit_pm } => {
            if area_mm2 <= 0 || conductivity_nm_s < 0 || suction_um < 0 || !(0..=1000).contains(&deficit_pm) { -1 } else { area_mm2 }
        }
        // S535 : le drainage (c ≥ 1), l'évaporation (vers dehors, vérifié par l'appelant).
        Flow::Drainage { area_mm2, conductivity_nm_s, exponent_pm } => {
            if area_mm2 <= 0 || conductivity_nm_s < 0 || exponent_pm < 1000 { -1 } else { area_mm2 }
        }
        Flow::Evaporation { area_mm2, rate_nm_s } => if area_mm2 <= 0 || rate_nm_s < 0 { -1 } else { area_mm2 },
        // S547 : l'évent (vers dehors, vérifié par l'appelant).
        Flow::Vent { area_mm2 } => if area_mm2 <= 0 { -1 } else { area_mm2 },
        _ => 0,
    }
}

/// **S530 — un pas de Green–Ampt, exact** : la lame `F₁` telle que `t(F₁) − t(F₀) = K·dt`, `t(F) = (F − M ln(1 + F/M))/K` —
/// `F₁ − F₀ − M ln((M + F₁)/(M + F₀)) = K·dt` —, par bissection en f64 (60 itérations, déterministe). `M` = 0 : `F₁ = F₀ + K·dt`.
pub(crate) fn green_ampt_step(f0: f64, m: f64, k_dt: f64) -> f64 {
    if !(k_dt > 0.0) {
        return f0;
    }
    if !(m > 0.0) {
        return f0 + k_dt;
    }
    let g = |f: f64| f - f0 - m * ((m + f) / (m + f0)).ln() - k_dt;
    let (mut lo, mut hi) = (f0, f0 + k_dt + 2.0 * (2.0 * (m + f0) * k_dt).sqrt() + k_dt);
    for _ in 0..64 {
        if g(hi) >= 0.0 {
            break;
        }
        hi = f0 + 2.0 * (hi - f0);
    }
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if g(mid) < 0.0 { lo = mid } else { hi = mid }
    }
    0.5 * (lo + hi)
}

/// S515 : la fraction de débit d'une vanne à la commande `c` (‰), interpolée linéairement sur sa courbe de onze points.
fn valve_fraction(curve: &[u16; 11], c: i64) -> f64 {
    let x = c as f64 / 100.;
    let i = (x.floor() as usize).min(9);
    let f = x - i as f64;
    (curve[i] as f64 + (curve[i + 1] as f64 - curve[i] as f64) * f) / 1000.
}

/// La cote de refoulement d'une pompe au-dessus du point de référence amont (µm) : la sortie, ou la surface du receveur si elle la
/// noie (ADR-199 D3) — partagée par `Pump` et `PumpLine`.
fn pump_delivery(nodes: &[HydroNode], e: &Opening, shapes: &Shapes<'_>, up: [f64; 3], outlet_um: [i64; 3]) -> Result<f64, Error> {
    let src = nodes[e.from as usize];
    let outlet = along(sub(outlet_um, src.origin_um), up);
    Ok(match e.to {
        Some(t) => {
            let dn = nodes[t as usize];
            let h_dn = shapes.surface_up(&dn, up)?.offset_um;
            (along(sub(dn.origin_um, src.origin_um), up) + h_dn).max(outlet)
        }
        None => outlet,
    })
}

/// **S515 — le point de fonctionnement d'une pompe sur sa conduite** : débit (m³/s), hauteur de la pompe (m : la hauteur statique plus
/// la perte de charge), puissance hydraulique et à l'arbre (W) pour un liquide de masse volumique `rho`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PumpPoint {
    pub flow_m3s: f64,
    pub head_m: f64,
    pub hydraulic_w: f64,
    pub shaft_w: f64,
}

fn pump_line_point(nodes: &[HydroNode], e: &Opening, shapes: &Shapes<'_>, up: [f64; 3], rho: f64) -> Result<Option<PumpPoint>, Error> {
    let Flow::PumpLine { max_flow_mlps, shutoff_head_um, outlet_um, loss_um_per_l2s2, efficiency_pm } = e.flow else {
        return Ok(None);
    };
    let src = nodes[e.from as usize];
    let h_up = shapes.surface_up(&src, up)?.offset_um;
    let sill = along(sub(e.position_um, src.origin_um), up);
    if h_up <= sill || e.control_pm == 0 {
        return Ok(None);
    }
    let dh_m = (pump_delivery(nodes, e, shapes, up, outlet_um)? - h_up) * 1e-6;
    let n = e.control_pm as f64 / CONTROL_FULL as f64;
    let (h0_m, qmax_lps, k) = (shutoff_head_um as f64 * 1e-6, max_flow_mlps as f64 * 1e-3, loss_um_per_l2s2 as f64 * 1e-6);
    let numerateur = n * n * h0_m - dh_m;
    if !(numerateur > 0.0) || !(qmax_lps > 0.0) {
        return Ok(None);
    }
    let q_lps = (numerateur / (h0_m / (qmax_lps * qmax_lps) + k)).sqrt();
    let head_m = dh_m + k * q_lps * q_lps;
    let flow_m3s = q_lps * 1e-3;
    let hydraulic_w = rho * crate::body::G * flow_m3s * head_m;
    Ok(Some(PumpPoint { flow_m3s, head_m, hydraulic_w, shaft_w: hydraulic_w / (efficiency_pm as f64 / 1000.) }))
}

/// **S515 — le point de fonctionnement de l'arête `index`** si c'est une pompe sur sa conduite qui débite dans l'état présent du réseau ;
/// l'hôte cumule l'énergie (`shaft_w · dt`). Même calcul que le pas.
pub fn pump_operating_point(nodes: &[HydroNode], edges: &[Opening], shapes: &Shapes<'_>, g_eff: [f32; 3], index: usize, rho: f64)
    -> Result<Option<PumpPoint>, Error> {
    let (up, _) = geometry::vertical(g_eff)?;
    let e = edges.get(index).ok_or(Error::Capacity)?;
    pump_line_point(nodes, e, shapes, up, rho)
}
