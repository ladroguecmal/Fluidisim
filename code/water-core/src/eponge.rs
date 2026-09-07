//! Le profil d'éponge d'ADR-005 §2, **écrit une fois**.
//!
//! # Pourquoi ce module existe séparément
//!
//! B-S26 a mesuré l'éponge sur le solveur de Saint-Venant. B-S27 la mesure sur un milieu **dispersif**,
//! qui est un autre porteur. Si chacun portait sa propre copie du profil, la seconde session
//! mesurerait une réimplémentation et non l'objet spécifié — et les deux mesures ne seraient pas
//! comparables.
//!
//! Ce qu'ADR-005 §2 spécifie tient en deux lignes, et les voici, à un seul endroit :
//!
//! ```text
//! σ(s) = σ_max · s²        s ∈ [0,1], 0 à l'entrée de la bande, 1 au bord
//! q ← q · (1 − σ(s)·dt)
//! ```
//!
//! Le profil quadratique est celui du document ; il y est préféré au linéaire pour éviter la
//! réflexion créée par la discontinuité de dérivée à l'entrée. **Cet argument n'a jamais été
//! mesuré** — ADR-042 §8 le note comme ouvert.

/// Amortissement `σ` à l'abscisse `x`, pour une bande de largeur `largeur` en bord **droit** d'un
/// domaine de longueur `longueur`. Nul hors de la bande.
///
/// `sigma_max = 0` laisse la bande en place sans amortir : c'est l'essai témoin (L136).
pub fn sigma(x: f64, longueur: f64, largeur: f64, sigma_max: f64) -> f64 {
    sigma_bande(x, longueur - largeur, largeur, sigma_max)
}

/// La même bande, **placée explicitement** : `σ` croît de 0 en `debut` à `σ_max` en
/// `debut + largeur`, et **reste nulle au-delà**.
///
/// Cette forme existe parce que B-S27 doit faire varier la largeur **sans déplacer l'entrée** de la
/// bande : l'instant d'arrivée du train réfléchi dépend de la position de l'entrée, et le laisser
/// bouger avec la largeur mélangerait deux effets dans une même colonne de chiffres.
///
/// Au-delà de la bande, `σ = 0` et non `σ_max` : la bande **est** l'éponge, ce qui la suit est du
/// milieu libre. La distinction ne change rien quand la bande touche le bord ; elle compte dès
/// qu'elle ne le touche pas.
pub fn sigma_bande(x: f64, debut: f64, largeur: f64, sigma_max: f64) -> f64 {
    if largeur <= 0.0 || x <= debut || x > debut + largeur {
        return 0.0;
    }
    let s = (x - debut) / largeur;
    sigma_max * s * s
}

/// Facteur multiplicatif d'un pas d'amortissement, saturé à zéro.
///
/// **La saturation change la nature de l'opérateur** — angle mort A160. Au-delà de `σ·dt = 1`, la
/// maille n'est plus amortie : elle est remise à l'état de repos à chaque pas. Le facteur est
/// saturé plutôt que laissé négatif, parce qu'un facteur négatif inverserait le signe du champ ;
/// mais **c'est le régime qu'il faut surveiller, pas la saturation**, et c'est pourquoi
/// `sigma_dt_max` existe.
pub fn facteur(sigma: f64, dt: f64) -> f64 {
    (1.0 - sigma * dt).max(0.0)
}

/// `σ_max·dt` — la grandeur à rapporter à chaque mesure. **Au-delà de 1, le chiffre mesuré ne
/// décrit plus une éponge** (A160, ADR-042 §4.1).
pub fn sigma_dt(sigma_max: f64, dt: f64) -> f64 {
    sigma_max * dt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_profil_est_nul_hors_de_la_bande_et_maximal_au_bord() {
        let (l, ls, s) = (100.0, 10.0, 4.0);
        assert_eq!(sigma(0.0, l, ls, s), 0.0);
        assert_eq!(sigma(90.0, l, ls, s), 0.0);
        assert!((sigma(100.0, l, ls, s) - s).abs() < 1e-12);
        // Quadratique : à mi-bande, le quart.
        assert!((sigma(95.0, l, ls, s) - s * 0.25).abs() < 1e-12);
    }

    #[test]
    fn la_bande_explicite_est_nulle_des_deux_cotes() {
        let (d, l, sm) = (100.0, 20.0, 4.0);
        assert_eq!(sigma_bande(100.0, d, l, sm), 0.0);
        assert_eq!(sigma_bande(121.0, d, l, sm), 0.0);
        assert!((sigma_bande(120.0, d, l, sm) - sm).abs() < 1e-12);
        assert!((sigma_bande(110.0, d, l, sm) - sm * 0.25).abs() < 1e-12);
        // La forme au bord droit doit coincider avec l'ancienne.
        assert!((sigma_bande(95.0, 90.0, 10.0, sm) - sigma(95.0, 100.0, 10.0, sm)).abs() < 1e-12);
    }

    #[test]
    fn le_facteur_sature_et_le_regime_est_visible() {
        // À σ·dt = 2, le facteur vaut 0 : la maille est écrasée, pas amortie. Le test verrouille
        // le comportement **et** le fait que `sigma_dt` le rende visible.
        assert_eq!(facteur(4.0, 0.5), 0.0);
        assert!((facteur(1.0, 0.5) - 0.5).abs() < 1e-12);
        assert!(sigma_dt(4.0, 0.5) > 1.0);
    }
}
