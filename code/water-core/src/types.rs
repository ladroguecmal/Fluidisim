//! Types fondamentaux — SPEC-004 §1.1 et §2, ADR-028 §3.
//!
//! Deux règles gouvernent ce module et ne se négocient pas :
//!
//! - **aucun type n'exprime une coordonnée monde en flottant** (SPEC-004 §1.1, I-08) ;
//! - **le temps ne transite jamais en `f32`** (I-08, ADR-003 §2.2) : `SimTime` est un entier de
//!   microsecondes, et seules des *phases repliées* atteignent l'arithmétique flottante.

/// Instant de simulation, en microsecondes, dans `T_sim`. SPEC-004 §1.1.
///
/// L'ulp d'un `f32` de temps à `t = 10⁶ s` vaut 62,5 ms (SPEC-001 §7) : le temps ne peut donc pas
/// être un flottant, et ce type n'offre aucune conversion vers `f32`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct SimTime(pub u64);

impl SimTime {
    pub const fn from_micros(us: u64) -> Self {
        SimTime(us)
    }
    pub const fn micros(self) -> u64 {
        self.0
    }
}

/// Identifiant de référentiel. Tout domaine appartient à un référentiel — I-07.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct FrameId(pub u32);

/// Masque de couches — SPEC-004 §2, invariant I-01.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LayerMask(pub u8);

impl LayerMask {
    pub const B: LayerMask = LayerMask(1);
    pub const W_REP: LayerMask = LayerMask(2);
    pub const W_LOCAL: LayerMask = LayerMask(4);
    pub const DELTA: LayerMask = LayerMask(8);

    pub const fn contains(self, other: LayerMask) -> bool {
        (self.0 & other.0) != 0
    }
}

/// Nombre d'unités de position monde par mètre — ADR-028 §3.
///
/// `2048 = 2¹¹`, et ce n'est pas un choix rond : c'est **exactement l'ulp d'un `f32` au rayon de
/// référentiel**. I-08 borne `|x_local| < 4096 m = 2¹² m` ; l'ulp d'un `f32` y vaut
/// `2¹² · 2⁻²³ = 2⁻¹¹ m`. Une résolution plus fine transporterait une précision que la conversion
/// détruit ; une plus grossière perdrait de l'information avant elle.
pub const WORLD_UNITS_PER_METRE: i64 = 2048;

/// Position monde — entier 64 bits en virgule fixe. ADR-028 §3.
///
/// Portée : `2⁶³ / 2048 ≈ 4,5·10¹⁵ m`, environ une demi-année-lumière.
///
/// Le déterminisme d'I-03 est ici **structurel** : une addition d'entiers donne le même résultat
/// sur toute plateforme, sans dépendre d'un drapeau de compilation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct WorldPos {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl WorldPos {
    pub const fn from_units(x: i64, y: i64, z: i64) -> Self {
        WorldPos { x, y, z }
    }

    /// Construit depuis des mètres. Réservé à l'outillage et aux scénarios : le cœur ne convertit
    /// jamais de mètres flottants en position monde à l'exécution.
    pub fn from_metres(x: f64, y: f64, z: f64) -> Self {
        let s = WORLD_UNITS_PER_METRE as f64;
        WorldPos {
            x: (x * s).round() as i64,
            y: (y * s).round() as i64,
            z: (z * s).round() as i64,
        }
    }

    /// Conversion vers une coordonnée locale à un référentiel — ADR-028 §3.3.
    ///
    /// La soustraction d'entiers est exacte ; le quotient tient dans `|x| < 4096 m` par
    /// construction (I-08), et la conversion vers `f32` y est exacte à l'ulp près — qui est
    /// précisément la résolution d'origine. **Aucune information n'est perdue à la frontière.**
    ///
    /// Renvoie `None` si le point sort du rayon de référentiel : c'est une violation de contrat
    /// (SPEC-004 §1.3), détectée et non subie.
    pub fn to_local(self, anchor: WorldPos) -> Option<[f32; 3]> {
        const LIMIT: i64 = 4096 * WORLD_UNITS_PER_METRE;
        let d = [self.x.checked_sub(anchor.x)?, self.y.checked_sub(anchor.y)?, self.z.checked_sub(anchor.z)?];
        for c in d {
            if c >= LIMIT || c <= -LIMIT {
                return None;
            }
        }
        let s = WORLD_UNITS_PER_METRE as f32;
        Some([d[0] as f32 / s, d[1] as f32 / s, d[2] as f32 / s])
    }
}

/// Échantillon d'eau rendu aux consommateurs — SPEC-004 §2.
///
/// `u_total` porte son nom depuis la correction S09 (écart E05) : c'est **orbitale + courant**, et
/// tout seuil de danger calculé dessus oscillerait à la période de la houle. La grandeur juste, le
/// courant seul, appartient au chemin poussé (SPEC-006 §5.5).
#[derive(Clone, Copy, Debug, Default)]
pub struct WaterSample {
    /// Élévation le long du radial local, en mètres.
    pub eta: f32,
    /// Vitesse de surface : **orbitale + courant**.
    pub u_total: [f32; 3],
    pub normal: [f32; 3],
    pub deta_dt: f32,
    /// Cambrure locale — déclencheur d'écume, SPEC-001 §3.
    pub steepness: f32,
    /// Fraction volumique d'air — ADR-014 §5.2.
    pub aeration: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution_monde_egale_ulp_f32_au_rayon_de_referentiel() {
        // ADR-028 §3.1 : la résolution se dérive, elle ne se choisit pas.
        let ulp_a_4096 = 4096.0f32 * 2f32.powi(-23);
        let resolution = 1.0f32 / WORLD_UNITS_PER_METRE as f32;
        assert_eq!(ulp_a_4096, resolution);
    }

    #[test]
    fn conversion_locale_exacte_et_bornee() {
        let anchor = WorldPos::from_metres(1_000_000.0, -500_000.0, 0.0);
        // Un point à exactement 1234,5 m de l'ancre.
        let p = WorldPos::from_units(
            anchor.x + 1234 * WORLD_UNITS_PER_METRE + WORLD_UNITS_PER_METRE / 2,
            anchor.y,
            anchor.z,
        );
        let l = p.to_local(anchor).expect("dans le rayon");
        assert_eq!(l[0], 1234.5);
        assert_eq!(l[1], 0.0);

        // Hors rayon : violation de contrat détectée, pas subie.
        let loin = WorldPos::from_units(anchor.x + 4096 * WORLD_UNITS_PER_METRE, anchor.y, anchor.z);
        assert!(loin.to_local(anchor).is_none());
    }
}

/// **Ce que les saturations d'un solveur ont fait** — S38, action S34-1, angle mort A146.
///
/// Ce type vit ici, et non dans l'un des deux solveurs, parce qu'il est un **type de rapport** :
/// `delta.rs` et `shallow.rs` le remplissent tous les deux et ne se connaissent pas. Les coupler
/// pour partager ce compteur détruirait l'indépendance sur laquelle repose l'oracle croisé
/// (`ADR-043` §3).
#[derive(Clone, Copy, Debug, Default)]
pub struct Saturations {
    /// **S7** — nombre de cellules remises à `h = 0` parce que le pas les avait rendues négatives.
    pub etat: u64,
    /// **S7** — masse **créée** par ces remises à zéro, cumulée, en m² (aire d'une tranche de
    /// canal d'épaisseur unité). C'est la grandeur qui manquait : *combien de fois* ne dit pas
    /// *combien*.
    pub masse_creee: f64,
    /// **S7** — quantité de mouvement **détruite** par la remise à zéro de `hu`, en valeur absolue
    /// cumulée. Elle n'est transférée nulle part.
    pub qdm_detruite: f64,
    /// **Étage intermédiaire de RK2** — saturations subies par l'état provisoire `U₁`, qui n'est
    /// pas un état publié. Compté à part : une saturation à l'étage dit que le **demi-pas** a
    /// produit un état impossible, ce qui n'implique pas que le pas complet en produise un. Reste
    /// à zéro pour un solveur sans RK2.
    ///
    /// *Ce point de saturation avait échappé au recensement écrit en P2 : il n'a été vu que parce
    /// que le compilateur a refusé l'appel restant. Un recensement à la lecture en manque.*
    pub etage_rk2: u64,
    /// **S5** — cellules fantômes ramenées à `h = 0` par la condition de bord sur fond montant.
    pub bord: u64,
    /// **Témoin de S6** — cellules trouvées à `h < 0` **à l'entrée** d'un pas, là où la protection
    /// de racine `(g·h)⁺` mordrait. Doit rester à zéro : S7 a agi à la fin du pas précédent. Un
    /// compteur non nul dirait que la protection de racine n'est **pas** morte, et qu'elle masque.
    pub h_negatif_en_entree: u64,
}
