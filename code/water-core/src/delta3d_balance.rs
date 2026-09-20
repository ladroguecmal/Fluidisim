//! **Le bilan de masse d'un domaine δ** — S310, lot 1 d'[ADR-178] D7.
//!
//! [ADR-178]: ../../../docs/adr/ADR-178-strategie-en-trois-systemes-physiques.md
//!
//! # Pourquoi ce module existe
//!
//! Le couplage δ ↔ B/W existe depuis S250 et tourne en 3D depuis S302. **Rien n'a jamais compté ce
//! qui y entre et ce qui en sort.** La revue R11 a déclaré le raccord d'un domaine « invisible » ;
//! c'est un jugement de l'œil, et l'angle mort **A302** dit pourquoi cela ne suffit pas. La
//! stratégie de l'utilisateur le demande explicitement : « des échanges cohérents de masse, de
//! quantité de mouvement et d'énergie », et « une validation visuelle ne remplace pas une
//! validation numérique » (ADR-178 D3).
//!
//! # Ce que ce module mesure, et pourquoi c'est exact
//!
//! Le pas couplé ne déplace `η` qu'à **deux endroits** : `transport_coupled3`, qui applique la
//! divergence des flux de colonne, et `relax_coupled3`, l'éponge. Le transport s'écrit, pour la
//! colonne `(i, j)` :
//!
//! ```text
//! η[i,j] += −(dt/dx) · ( Fx[i+1,j] − Fx[i,j] + Bx[i+1,j] − Bx[i,j]
//!                      + Fy[i,j+1] − Fy[i,j] + By[i,j+1] − By[i,j] )
//! ```
//!
//! où `F` est le flux de la **perturbation** et `B` celui de la **bande** de fond B/W. Sommée sur
//! le domaine, chaque ligne **télescope** :
//!
//! ```text
//! Σᵢ ( F[i+1] − F[i] ) = F[nx] − F[0]
//! ```
//!
//! **Il ne reste donc que les faces de bord.** Ce n'est pas une approximation du bilan : c'est le
//! bilan, en arithmétique exacte. L'écart que la mesure trouve **est** le plancher numérique du
//! schéma, pas une erreur de l'instrument — c'est ce qui permet de le publier comme tel.
//!
//! # Trois précisions qui changent le chiffre
//!
//! 1. **La hauteur vraie n'est pas `η`.** Le pas somme `η` en compensé (S233) : `eta_roundoff`
//!    porte la part perdue, que l'incrément suivant retranche. La hauteur qui compte — celle que
//!    la pression lit déjà, `delta3d.rs` §`lid` — est `η − eta_roundoff`. Un volume calculé sur
//!    `η` seul manquerait exactement ce que la compensation existe pour retenir.
//! 2. **Le volume est sommé en `f64`.** Les champs sont en `f32` ; leur somme sur des dizaines de
//!    milliers de colonnes ne l'est pas. Le bilan mesure un plancher : il ne doit pas en fabriquer
//!    un.
//! 3. **Aux faces extérieures, le flux de perturbation est nul par construction.** La garde
//!    `a > 0 && a < n` de `transport_coupled3` l'y laisse à zéro : seule la bande transporte au
//!    bord. C'est « δ ne ressort pas vers W » écrit une seconde fois, dans une boucle — et c'est
//!    pourquoi ce module **publie quand même** `perturbation_out` : une valeur non nulle voudrait
//!    dire que cette lecture du code est fausse, et le dire tout de suite coûte une addition.
//!
//! # Ce que ce module ne mesure pas, et ne prétend pas mesurer
//!
//! - **La quantité de mouvement et l'énergie.** Leur bilan demande des termes que le pas ne
//!   produit pas : le travail de la pression aux faces de bord, et le flux advectif de quantité de
//!   mouvement. Ils ne se déduisent pas des flux de colonne. Ce module s'en tient à la masse, qui
//!   est exacte ; le reste est nommé dans la preuve de la session, non improvisé.
//! - **Ce que fait la carte.** Ce module vit dans la **référence** CPU. C'est sa place : ADR-175
//!   fait de la référence le juge de la production, pas l'inverse.
//! - **Une tolérance.** Le dépôt n'en a aucune pour la conservation. Elle se **propose** à
//!   l'utilisateur avec le premier bilan publié ; elle ne se décrète pas dans un module.

use super::Volume3;

/// Bilan de masse d'un pas, en mètres cubes. Tous les termes sont des **volumes de
/// perturbation** : le fond B/W n'est pas compté, seule la bande qu'il pousse à travers le bord.
///
/// Convention de signe : `band_in` et `perturbation_out` comptent **positivement ce qui entre** ;
/// `sponge_out` compte **positivement ce qui est retiré**.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Balance3 {
    /// Volume de perturbation à la fin du pas : `Σ (η − eta_roundoff − repos) · dx²`.
    pub volume: f64,
    /// Sa variation pendant le pas.
    pub delta: f64,
    /// Volume apporté par la **bande** B/W à travers les quatre faces extérieures.
    pub band_in: f64,
    /// Volume apporté par la **perturbation** à travers ces mêmes faces. **Nul par construction** ;
    /// publié pour que la construction soit vérifiée à chaque pas plutôt que crue.
    pub perturbation_in: f64,
    /// Volume **retiré par l'éponge**, cumulé sur les colonnes qu'elle touche.
    pub sponge_out: f64,
    /// `delta − band_in − perturbation_in + sponge_out`. Nul en arithmétique exacte : **c'est le
    /// plancher numérique du pas**, et il se publie, il ne s'absorbe pas dans une tolérance.
    pub residual: f64,
}

impl Volume3 {
    /// Volume de perturbation présent dans le domaine, en mètres cubes.
    ///
    /// `η − eta_roundoff` est la hauteur **compensée**, celle que la pression lit déjà ; `rest`
    /// est le repos, donc l'intégrale porte sur l'écart, pas sur la colonne d'eau entière. Somme
    /// en `f64` : voir la précision 2 de l'en-tête.
    pub fn perturbation_volume(&self) -> f64 {
        let area = self.domain.dx as f64 * self.domain.dx as f64;
        let rest = self.rest as f64;
        let mut total = 0f64;
        for (height, lost) in self.eta.iter().zip(&self.eta_roundoff) {
            total += (*height as f64 - *lost as f64) - rest;
        }
        total * area
    }

    /// Le bilan du dernier pas couplé. `Default` tant qu'aucun pas couplé n'a été exécuté.
    pub fn balance(&self) -> Balance3 {
        self.balance
    }
}
