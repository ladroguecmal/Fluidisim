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
//! # La masse volumique n'est pas une constante d'ici — ADR-048
//!
//! Aucun document du projet ne fixait la masse volumique de l'eau, et ce fichier l'a longtemps
//! posée à `1000` en affirmant que *c'était la valeur avec laquelle la référence de C10 se
//! referme*. **C'était faux, et mesurément faux** : les trois références de C10 sont construites
//! *avec* la constante, donc leur écart est nul pour toute valeur. Le balayage est dans
//! `docs/validation/RHO-EAU-S58.md` ; la faute est une instance de l'angle mort **A104**, énoncé
//! trois modules plus loin dans le harnais.
//!
//! ADR-048 tranche A103 : la valeur du projet est celle de l'**eau de mer**, et elle devient une
//! propriété du **milieu**, parce que le monde contient aussi des eaux intérieures — et que
//! l'estuaire est l'endroit où un même corps change de tirant en avançant.

/// Le milieu dans lequel un corps flotte — ADR-048 §3 D2.
///
/// Un paramètre, **pas un champ** : aucun mélange, aucune stratification, aucun transport de
/// salinité n'est introduit ici. Le jour où la salinité devient un champ transporté, c'est ici
/// qu'elle arrive, et ce sera un autre ADR.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Milieu {
    /// Masse volumique, en kg/m³.
    pub rho: f64,
}

impl Milieu {
    /// Eau de mer — **la valeur du projet** (ADR-048 D1). Convention, pas mesure : aucune mesure
    /// du corpus ne départage 1025 de 1000, et il n'en existe pas de candidate. Le choix se fait
    /// sur le domaine — mer ouverte, ADR-001 — et ADR-048 §4 dit ce qui l'inverserait.
    pub const MER: Milieu = Milieu { rho: 1025.0 };

    /// Eau douce — rivières, lacs, et la valeur que le projet a portée par défaut de S21 à S58.
    pub const EAU_DOUCE: Milieu = Milieu { rho: 1000.0 };
}

impl Default for Milieu {
    fn default() -> Self {
        Milieu::MER
    }
}

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
    pub fn force_verticale(&self, milieu: Milieu, z_c: f64, eta: f64) -> f64 {
        milieu.rho * G * self.volume_immerge(z_c, eta) - self.masse() * G
    }

    /// Altitude d'équilibre du centre, par bissection sur la force.
    ///
    /// La force est **décroissante** en `z_c` — monter, c'est s'immerger moins — donc la bissection
    /// converge sans hypothèse supplémentaire. Renvoie `None` si le corps ne peut pas flotter,
    /// c'est-à-dire si sa masse volumique dépasse celle de l'eau : le cas est réel, il ne doit pas
    /// se traduire par une valeur silencieuse.
    pub fn equilibre(&self, milieu: Milieu, eta: f64) -> Option<f64> {
        if self.rho >= milieu.rho {
            return None;
        }
        // Encadrement : entièrement immergé (force maximale, positive) et entièrement émergé
        // (force = −poids, négative).
        let (mut bas, mut haut) = (eta - self.cote_m, eta + self.cote_m);
        for _ in 0..80 {
            let m = 0.5 * (bas + haut);
            if self.force_verticale(milieu, m, eta) > 0.0 {
                bas = m;
            } else {
                haut = m;
            }
        }
        Some(0.5 * (bas + haut))
    }

    /// Tirant d'eau à l'équilibre, en mètres — la référence fermée de C10 vaut
    /// `d = (ρ_corps/ρ_eau)·H`.
    pub fn tirant(&self, milieu: Milieu, eta: f64) -> Option<f64> {
        let z_c = self.equilibre(milieu, eta)?;
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
        // d = (ρ_corps/ρ_eau)·H. **Ce littéral est une conséquence d'ADR-048 D1, pas une mesure** :
        // 500/1025 × 0,5 = 0,243902…  En eau douce il vaudrait 0,25, le chiffre que le corpus a
        // porté de S21 à S58. Ces deux assertions sont le SEUL contrôle du projet sur la masse
        // volumique — le cas canonique C10, lui, est aveugle (A180, RHO-EAU-S58).
        let d = cube().tirant(Milieu::MER, 0.0).expect("le cube flotte");
        assert!((d - 0.243_902_439_024_39).abs() < 1e-9, "tirant {d}");

        let douce = cube().tirant(Milieu::EAU_DOUCE, 0.0).expect("le cube flotte");
        assert!((douce - 0.25).abs() < 1e-9, "tirant en eau douce {douce}");
    }

    #[test]
    fn le_tirant_ne_depend_pas_du_niveau_de_la_surface() {
        // La référence est relative à la surface libre, pas à z = 0. Un test qui n'interroge que
        // eta = 0 ne peut pas révéler une confusion entre les deux — angle mort A100.
        for eta in [-3.0, -0.4, 0.0, 0.7, 12.5] {
            let d = cube().tirant(Milieu::MER, eta).expect("le cube flotte");
            assert!((d - 0.243_902_439_024_39).abs() < 1e-9, "eta {eta}, tirant {d}");
        }
    }

    /// Le milieu commande, et il commande la bonne quantité — ADR-048 §2.
    ///
    /// Essai de sensibilité : la grandeur doit varier, et varier **de la quantité prévue par la
    /// formule fermée**. Un paramètre qu'on introduit sans vérifier qu'il déplace quelque chose
    /// est un paramètre qu'on croira réglé alors qu'il sera mort — c'est la faute inverse de celle
    /// qu'A180 décrit, et elle est aussi facile à commettre.
    #[test]
    fn le_tirant_suit_la_masse_volumique_du_milieu() {
        let c = cube();
        for rho_eau in [1000.0, 1010.0, 1025.0, 1100.0] {
            let m = Milieu { rho: rho_eau };
            let d = c.tirant(m, 0.0).expect("le cube flotte");
            let attendu = (c.rho / rho_eau) * c.cote_m;
            assert!((d - attendu).abs() < 1e-9, "ρ_eau {rho_eau}, tirant {d}");
        }
        // Et l'écart mer/eau douce est bien celui que le corpus annonçait depuis S21 : 2,44 %.
        let mer = c.tirant(Milieu::MER, 0.0).unwrap();
        let douce = c.tirant(Milieu::EAU_DOUCE, 0.0).unwrap();
        assert!(((douce - mer) / douce - 0.024_390_243_9).abs() < 1e-9);
    }

    #[test]
    fn un_corps_plus_dense_que_l_eau_ne_flotte_pas() {
        let plomb = FloatingBox {
            cote_m: 0.5,
            rho: 11340.0,
        };
        assert!(plomb.equilibre(Milieu::MER, 0.0).is_none());

        // Le seuil est celui du milieu, pas une constante : un corps à 1010 kg/m³ coule en eau
        // douce et flotte en mer. C'est le cas que D2 rend exprimable.
        let saumatre = FloatingBox {
            cote_m: 0.5,
            rho: 1010.0,
        };
        assert!(saumatre.equilibre(Milieu::EAU_DOUCE, 0.0).is_none());
        assert!(saumatre.equilibre(Milieu::MER, 0.0).is_some());
    }

    #[test]
    fn le_defaut_du_projet_est_la_mer() {
        assert_eq!(Milieu::default(), Milieu::MER);
        assert_eq!(Milieu::MER.rho, 1025.0);
        assert_eq!(Milieu::EAU_DOUCE.rho, 1000.0);
    }

    #[test]
    fn le_volume_immerge_sature_a_la_pleine_immersion() {
        let c = cube();
        let plein = c.cote_m * c.aire();
        assert!((c.volume_immerge(-50.0, 0.0) - plein).abs() < 1e-9);
        assert_eq!(c.volume_immerge(50.0, 0.0), 0.0);
    }
}
