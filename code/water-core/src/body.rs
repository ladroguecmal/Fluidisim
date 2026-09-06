//! Flottaison — la première **force** du cœur. Cas canonique C10, ADR-008 §3.
//!
//! # Portée, et ce que ce module n'est pas
//!
//! Ce n'est pas le système de corps rigides du projet : il n'y a ici ni intégrateur, ni rotation, ni
//! masse ajoutée, ni amortissement. C'est le **modèle statique minimal** dont C10 a besoin pour que
//! trois références fermées deviennent mesurables — tirant d'eau, raideur hydrostatique, et la
//! période de pilonnement que cette raideur implique.
//!
//! Il vit dans le cœur, et non dans le harnais, pour une raison qui n'est pas de commodité : une
//! référence calculée par le test lui-même ne vérifie que l'arithmétique du test. La force doit
//! être produite par le code qu'on juge.
//!
//! # Les deux approximations, et leur domaine
//!
//! - **Ligne de flottaison plane.** Le volume immergé est `A·d`, avec `d` mesuré contre la surface
//!   libre au seul point du centre. Valable tant que le corps est petit devant la longueur d'onde —
//!   `H ≪ λ`. Pour le cube de C10 (0,5 m) contre les 25 m de C02, le rapport est de 1 à 50.
//! - **Pression hydrostatique.** Aucune correction de Froude-Krylov, aucun terme de Wagner. Le
//!   slamming a son propre traitement (ADR-023 §2) et son propre cas (C20).
//!
//! Ces deux approximations sont exactes dans le montage de C10 — eau calme — et c'est exactement le
//! domaine où les références de C10 sont, elles aussi, exactes.
//!
//! # Une convention qui manquait au corpus
//!
//! Aucun document du projet ne fixait la masse volumique de l'eau. La référence de C10
//! (`T = 1,00 s`) n'est retrouvée qu'avec `ρ = 1000 kg/m³` : l'eau douce. La constante est donc
//! posée ici, explicitement, et l'eau de mer — `≈1025` — est signalée comme un choix à faire, pas
//! comme un détail. Voir angle mort A103.

/// Masse volumique de l'eau douce, en kg/m³.
///
/// **Convention, pas mesure.** Elle vaut `1000` parce que c'est la valeur avec laquelle la référence
/// de C10 se referme. L'eau de mer vaut ≈1025 kg/m³ : un cube de densité 500 y flotte avec 2,5 % de
/// tirant en moins. Le jour où le projet distingue les deux, ce choix devient un paramètre de
/// `HydroSample` et cette constante devient sa valeur par défaut.
pub const RHO_EAU: f64 = 1000.0;

/// Accélération de la pesanteur, en m/s². Même valeur que `background.rs`.
pub const G: f64 = 9.81;

/// Un pavé droit homogène, flottant à ligne d'eau plane.
#[derive(Clone, Copy, Debug)]
pub struct FloatingBox {
    /// Côté, en mètres. Le pavé est un cube.
    pub cote_m: f64,
    /// Masse volumique du corps, en kg/m³.
    pub rho: f64,
}

impl FloatingBox {
    pub fn masse(&self) -> f64 {
        self.rho * self.cote_m * self.cote_m * self.cote_m
    }

    /// Aire de flottaison, en m².
    pub fn aire(&self) -> f64 {
        self.cote_m * self.cote_m
    }

    /// Volume immergé, pour un centre à l'altitude `z_c` et une surface libre à `eta`.
    ///
    /// Saturé aux deux bouts : un corps entièrement hors de l'eau déplace un volume nul, un corps
    /// entièrement immergé déplace son volume propre et **pas davantage**. Sans cette saturation, la
    /// force croîtrait indéfiniment avec la profondeur, ce qui est le défaut le plus courant d'une
    /// flottabilité écrite trop vite.
    pub fn volume_immerge(&self, z_c: f64, eta: f64) -> f64 {
        let bas = z_c - self.cote_m * 0.5;
        let d = (eta - bas).clamp(0.0, self.cote_m);
        d * self.aire()
    }

    /// Force verticale nette, en newtons : poussée d'Archimède moins poids. Vers le haut si positive.
    pub fn force_verticale(&self, z_c: f64, eta: f64) -> f64 {
        RHO_EAU * G * self.volume_immerge(z_c, eta) - self.masse() * G
    }

    /// Altitude d'équilibre du centre, par bissection sur la force.
    ///
    /// La force est **décroissante** en `z_c` — monter, c'est s'immerger moins — donc la bissection
    /// converge sans hypothèse supplémentaire. Renvoie `None` si le corps ne peut pas flotter,
    /// c'est-à-dire si sa masse volumique dépasse celle de l'eau : le cas est réel, il ne doit pas
    /// se traduire par une valeur silencieuse.
    pub fn equilibre(&self, eta: f64) -> Option<f64> {
        if self.rho >= RHO_EAU {
            return None;
        }
        // Encadrement : entièrement immergé (force maximale, positive) et entièrement émergé
        // (force = −poids, négative).
        let (mut bas, mut haut) = (eta - self.cote_m, eta + self.cote_m);
        for _ in 0..80 {
            let m = 0.5 * (bas + haut);
            if self.force_verticale(m, eta) > 0.0 {
                bas = m;
            } else {
                haut = m;
            }
        }
        Some(0.5 * (bas + haut))
    }

    /// Tirant d'eau à l'équilibre, en mètres — la référence fermée de C10 vaut
    /// `d = (ρ_corps/ρ_eau)·H`.
    pub fn tirant(&self, eta: f64) -> Option<f64> {
        let z_c = self.equilibre(eta)?;
        Some(eta - (z_c - self.cote_m * 0.5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cube() -> FloatingBox {
        FloatingBox {
            cote_m: 0.5,
            rho: 500.0,
        }
    }

    #[test]
    fn tirant_du_cube_de_c10() {
        // d = (ρ_corps/ρ_eau)·H = 0,25 m — CAS-CANONIQUES, C10.
        let d = cube().tirant(0.0).expect("le cube flotte");
        assert!((d - 0.25).abs() < 1e-6, "tirant {d}");
    }

    #[test]
    fn le_tirant_ne_depend_pas_du_niveau_de_la_surface() {
        // La référence est relative à la surface libre, pas à z = 0. Un test qui n'interroge que
        // eta = 0 ne peut pas révéler une confusion entre les deux — angle mort A100.
        for eta in [-3.0, -0.4, 0.0, 0.7, 12.5] {
            let d = cube().tirant(eta).expect("le cube flotte");
            assert!((d - 0.25).abs() < 1e-6, "eta {eta}, tirant {d}");
        }
    }

    #[test]
    fn un_corps_plus_dense_que_l_eau_ne_flotte_pas() {
        let plomb = FloatingBox {
            cote_m: 0.5,
            rho: 11340.0,
        };
        assert!(plomb.equilibre(0.0).is_none());
    }

    #[test]
    fn le_volume_immerge_sature_a_la_pleine_immersion() {
        let c = cube();
        let plein = c.cote_m * c.aire();
        assert!((c.volume_immerge(-50.0, 0.0) - plein).abs() < 1e-9);
        assert_eq!(c.volume_immerge(50.0, 0.0), 0.0);
    }
}
