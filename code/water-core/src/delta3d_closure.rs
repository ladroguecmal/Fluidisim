//! **Le critère de fermeture d'un bilan** — S313, [ADR-181] D5.
//!
//! [ADR-181]: ../../../docs/adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md
//!
//! # Pourquoi ce module existe
//!
//! T1 demandait « résidu ≤ 10⁻⁶ de l'échelle du pas ». S312 a montré que ce dénominateur rétrécit
//! avec `dt` et s'annule sur un champ de moyenne nulle (A304) : le critère produisait des échecs
//! qui ne disaient rien. L'utilisateur a retiré le chiffre et demandé **quatre grandeurs
//! distinctes**, chacune avec son unité :
//!
//! | | ce qu'elle dit | ici |
//! |---|---|---|
//! | **résidu absolu** | ce que le pas ne referme pas, en m³ | [`absolute_worst`], [`absolute_mean`] |
//! | **résidu relatif** | rapporté à une **échelle pertinente**, à justifier | [`relative_worst`] |
//! | **plancher attendu** | ce que la représentation impose, **loi observée** | [`expected_floor`], [`over_floor`] |
//! | **cumulé** | dérive ou bruit, sur une durée | [`cumulative_signed`], [`random_walk_ratio`] |
//!
//! [`absolute_worst`]: Closure3::absolute_worst
//! [`absolute_mean`]: Closure3::absolute_mean
//! [`relative_worst`]: Closure3::relative_worst
//! [`expected_floor`]: Closure3::expected_floor
//! [`over_floor`]: Closure3::over_floor
//! [`cumulative_signed`]: Closure3::cumulative_signed
//! [`random_walk_ratio`]: Closure3::random_walk_ratio
//!
//! # L'échelle pertinente, et pourquoi c'est celle-là
//!
//! **Le volume absolu de perturbation**, `Σ|η − repos|·dx²`. Ce n'est pas un choix de confort :
//! S313 a mesuré le rapport `résidu / volume absolu` sur **quatre décades d'amplitude** et l'a
//! trouvé **constant** (6,2 ; 5,8 ; 5,2 ; 5,4·10⁻¹¹). Les deux autres candidats échouent :
//! l'**incrément du pas** rétrécit avec `dt` alors que le résidu aussi, mais pas au même rythme ;
//! le **volume signé** est nul par construction sur un paquet, et diviser par lui donne un
//! nombre sans rapport avec quoi que ce soit.
//!
//! # Le plancher attendu — une **loi observée**, pas une borne démontrée
//!
//! [ADR-182](../../../docs/adr/ADR-182-criteres-de-conservation-actes-et-ordre-b.md) D1 exige que
//! la distinction soit portée partout où le plancher est invoqué : ce qui suit est **raisonné puis
//! vérifié par la mesure**, ce n'est pas une majoration prouvée. Aucun document ne doit écrire
//! « borne » à la place.
//!
//! Le transport télescope : sommée sur le domaine, la divergence des flux de colonne ne laisse
//! que les faces de bord. Mais chaque différence `F[i+1] − F[i]` est **arrondie en `f32`** avant
//! d'être appliquée, une fois par colonne, avec un signe indépendant d'une colonne à l'autre. Le
//! domaine accumule donc `√N` de ces arrondis, chacun valant au plus `u₃₂` fois ce que la colonne
//! a bougé :
//!
//! ```text
//! plancher = u₃₂ · activité / √N        u₃₂ = 2⁻²⁴,  activité = Σ|Δ(η−repos)|·dx²
//! ```
//!
//! **Le rapport `résidu / plancher` reste entre 3,0 et 5,7 sur tout le balayage de S313** —
//! amplitude sur quatre décades, `dt` sur un facteur 16, résolution sur un facteur 16 en `N`, et
//! trois montages. C'est ce **domaine de validité** qui accompagne la loi ; hors de lui, elle est
//! à revérifier (ADR-182 D1 : schéma, précision, méthode de sommation, régime). Les estimations
//! naïves en `ulp(h₀)` sont
//! fausses de **cinq ordres** : la somme compensée de S233 retire la hauteur du problème, et il
//! ne reste que ce que le pas déplace.
//!
//! # Ce que ce module ne fait pas, et c'est une décision
//!
//! **Il ne rend aucun verdict et ne porte aucun seuil.** ADR-181 D5 interdit de fixer un chiffre
//! avant la démonstration complète, qui comprend la sensibilité sur une erreur volontaire. Ce
//! module **publie** ; l'appelant compare. Le jour où une tolérance sera actée, elle se posera
//! sur `over_floor` et sur `random_walk_ratio`, pas ici.

/// Refus du critère. Aucun n'est un verdict physique.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosureError {
    /// Une des valeurs n'est pas finie.
    NonFinite,
    /// Activité ou volume absolu négatif : ce sont des grandeurs absolues.
    Negative,
    /// Domaine sans colonne.
    Empty,
}

/// `u₃₂ = 2⁻²⁴` — la demi-résolution relative d'un `f32`, et la constante de la loi observée.
pub const U32: f64 = 5.960_464_477_539_063e-8;

/// Le critère de fermeture d'un bilan de masse, cumulé sur les pas.
///
/// Toutes les grandeurs sont en mètres cubes, sauf les rapports, sans dimension.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Closure3 {
    columns: f64,
    absolute_worst: f64,
    absolute_sum: f64,
    relative_worst: f64,
    floor_worst: f64,
    over_floor_worst: f64,
    cumulative_signed: f64,
    cumulative_absolute: f64,
    steps: u64,
}

impl Closure3 {
    /// `columns` est le nombre de colonnes du domaine — le `N` de la loi.
    pub fn new(columns: usize) -> Result<Self, ClosureError> {
        if columns == 0 {
            return Err(ClosureError::Empty);
        }
        Ok(Self {
            columns: columns as f64,
            absolute_worst: 0.,
            absolute_sum: 0.,
            relative_worst: 0.,
            floor_worst: 0.,
            over_floor_worst: 0.,
            cumulative_signed: 0.,
            cumulative_absolute: 0.,
            steps: 0,
        })
    }

    /// Enregistre un pas.
    ///
    /// - `residual` : `delta − band_in − perturbation_in + sponge_out`, **signé**.
    /// - `activity` : `Σ|Δ(η − repos)|·dx²` du pas — ce que les colonnes ont bougé **en valeur
    ///   absolue**, et non leur somme, qui est `delta`. C'est l'entrée de la loi.
    /// - `absolute_volume` : `Σ|η − repos|·dx²` à la fin du pas — l'échelle pertinente.
    ///
    /// Un pas dont l'activité est nulle ne contribue pas au rapport au plancher : son plancher
    /// vaut zéro, et `résidu / 0` ne mesurerait que la division. Il compte en revanche dans les
    /// grandeurs absolues et dans le cumulé — c'est justement là qu'une fuite se verrait.
    pub fn account(
        &mut self,
        residual: f64,
        activity: f64,
        absolute_volume: f64,
    ) -> Result<(), ClosureError> {
        if !residual.is_finite() || !activity.is_finite() || !absolute_volume.is_finite() {
            return Err(ClosureError::NonFinite);
        }
        if activity < 0. || absolute_volume < 0. {
            return Err(ClosureError::Negative);
        }
        let r = residual.abs();
        self.absolute_worst = self.absolute_worst.max(r);
        self.absolute_sum += r;
        self.cumulative_signed += residual;
        self.cumulative_absolute += r;
        if absolute_volume > 0. {
            self.relative_worst = self.relative_worst.max(r / absolute_volume);
        }
        let floor = U32 * activity / self.columns.sqrt();
        self.floor_worst = self.floor_worst.max(floor);
        if floor > 0. {
            self.over_floor_worst = self.over_floor_worst.max(r / floor);
        }
        self.steps += 1;
        Ok(())
    }

    /// **Résidu absolu**, en m³ : le pire pas, et la moyenne.
    pub fn absolute_worst(&self) -> f64 {
        self.absolute_worst
    }
    pub fn absolute_mean(&self) -> f64 {
        if self.steps == 0 {
            return 0.;
        }
        self.absolute_sum / self.steps as f64
    }

    /// **Résidu relatif** au volume absolu de perturbation. Sans dimension.
    pub fn relative_worst(&self) -> f64 {
        self.relative_worst
    }

    /// **Plancher attendu** du pire pas, en m³ — `u₃₂ · activité / √N`.
    pub fn expected_floor(&self) -> f64 {
        self.floor_worst
    }

    /// Le pire rapport `résidu / plancher attendu`. **C'est la grandeur à lire** : au-dessus de
    /// quelques unités, ce que le bilan ne referme pas n'est plus de l'arrondi.
    pub fn over_floor(&self) -> f64 {
        self.over_floor_worst
    }

    /// **Cumulé signé** : il dit si les résidus se compensent ou s'ajoutent.
    pub fn cumulative_signed(&self) -> f64 {
        self.cumulative_signed
    }
    /// **Cumulé absolu** : il dit combien d'arrondi a été produit, compensé ou non.
    pub fn cumulative_absolute(&self) -> f64 {
        self.cumulative_absolute
    }

    /// **La forme du cumulé**, sans dimension : `|cumulé signé| / (moyen · √pas)`.
    ///
    /// Des résidus indépendants et de signe aléatoire donnent ≈ 1 — une marche aléatoire. Des
    /// résidus d'un **même signe** donnent `√pas`, parce que le cumulé croît alors linéairement.
    /// C'est le seul des quatre indicateurs qui distingue un **bruit** d'une **fuite lente**, et
    /// c'est pour cela que l'utilisateur demande le comportement « sur une durée donnée ».
    pub fn random_walk_ratio(&self) -> f64 {
        let mean = self.absolute_mean();
        if mean == 0. || self.steps == 0 {
            return f64::NAN;
        }
        self.cumulative_signed.abs() / (mean * (self.steps as f64).sqrt())
    }

    pub fn steps(&self) -> u64 {
        self.steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Des résidus de signe alterné se compensent : le cumulé signé reste au niveau d'un pas,
    /// et le rapport de forme reste d'ordre 1.
    #[test]
    fn alternating_residuals_are_a_random_walk() {
        let mut c = Closure3::new(100).unwrap();
        for n in 0..400 {
            let signe = if n % 2 == 0 { 1. } else { -1. };
            c.account(signe * 1e-12, 1e-4, 1e-2).unwrap();
        }
        assert_eq!(c.absolute_worst(), 1e-12);
        assert!(c.cumulative_signed().abs() <= 1e-12);
        assert!(c.random_walk_ratio() < 0.1, "{}", c.random_walk_ratio());
        // 400 additions de `1e-12` en `f64` ne rendent pas `4e-10` au bit, et c'est normal :
        // le cumulé est une somme, pas un compteur.
        assert!((c.cumulative_absolute() / 400e-12 - 1.).abs() < 1e-12);
    }

    /// Des résidus d'un seul signe **ne** se compensent pas : le rapport de forme vaut `√pas`,
    /// et c'est la signature d'une fuite lente que le résidu par pas ne montre pas.
    #[test]
    fn one_signed_residuals_show_up_as_a_drift() {
        let mut c = Closure3::new(100).unwrap();
        for _ in 0..400 {
            c.account(1e-12, 1e-4, 1e-2).unwrap();
        }
        let attendu = 400f64.sqrt();
        assert!(
            (c.random_walk_ratio() - attendu).abs() < 1e-9,
            "{} contre {attendu}",
            c.random_walk_ratio()
        );
    }

    /// La loi est celle que S313 a observée, et elle se calcule sans rien d'autre que l'activité
    /// et le nombre de colonnes.
    #[test]
    fn the_floor_is_the_observed_law() {
        let mut c = Closure3::new(64).unwrap();
        c.account(0., 1.6e-4, 1e-2).unwrap();
        // u₃₂ · 1,6e-4 / 8
        assert!((c.expected_floor() - U32 * 1.6e-4 / 8.).abs() < 1e-24);
        assert_eq!(c.over_floor(), 0.);
    }

    /// Un pas **sans activité** ne fabrique pas un rapport au plancher : son plancher est nul.
    /// Il compte en revanche dans le cumulé — c'est là qu'une fuite sur domaine calme se voit.
    #[test]
    fn a_still_step_does_not_manufacture_a_ratio() {
        let mut c = Closure3::new(64).unwrap();
        c.account(3e-12, 0., 1e-2).unwrap();
        assert_eq!(c.over_floor(), 0.);
        assert_eq!(c.expected_floor(), 0.);
        assert_eq!(c.absolute_worst(), 3e-12);
        assert_eq!(c.cumulative_signed(), 3e-12);
        assert_eq!(c.relative_worst(), 3e-10);
    }

    #[test]
    fn refusals() {
        assert_eq!(Closure3::new(0).err(), Some(ClosureError::Empty));
        let mut c = Closure3::new(4).unwrap();
        assert_eq!(c.account(f64::NAN, 1., 1.), Err(ClosureError::NonFinite));
        assert_eq!(c.account(0., -1., 1.), Err(ClosureError::Negative));
        assert_eq!(c.account(0., 1., -1.), Err(ClosureError::Negative));
        assert_eq!(c.steps(), 0);
        assert!(c.random_walk_ratio().is_nan());
    }
}
