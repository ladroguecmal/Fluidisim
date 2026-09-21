//! **Le registre du transfert δ → W** — S312, [ADR-180] D2.
//!
//! [ADR-180]: ../../../docs/adr/ADR-180-retour-delta-w-et-conservation-du-volume.md
//!
//! # Pourquoi ce module existe, et pourquoi il est si petit
//!
//! L'utilisateur autorise un premier transfert qui ne porte qu'une partie de ce qui sort, et il
//! pose une condition qui n'est pas négociable : *« tout volume non restitué doit être
//! comptabilisé et publié séparément […] Il ne doit toutefois pas être présenté comme une
//! restitution physique accomplie. »*
//!
//! Le danger n'est pas de mal calculer. Il est d'**écrire un compteur qui se laisse satisfaire** :
//! un champ `deficit` qu'une ligne de code remet à zéro, un total qui additionne le transmis et
//! l'attendu, une méthode `restituer()` qui n'a rien restitué. ADR-179 §4 nommait déjà ce risque
//! pour l'éponge. Ce module y répond par sa **forme**, pas par sa documentation :
//!
//! - **`pending` n'est pas un champ, c'est une différence.** Rien ne peut l'écrire, donc rien ne
//!   peut l'effacer. La seule façon de le faire baisser est d'augmenter `transferred`,
//!   c'est-à-dire de transférer pour de bon.
//! - **Il est signé.** Négatif, il ne veut pas dire « tout va bien » : il veut dire qu'on a remis
//!   à W **plus** qu'il n'est sorti, donc qu'on a **créé de l'eau**. `created()` l'isole, parce
//!   que c'est exactement ce que l'utilisateur interdit — « sans créer ni détruire artificiellement
//!   de l'eau ».
//! - **Aucune méthode ne s'appelle « restituer ».** Le vocabulaire du module est *sorti*,
//!   *transféré*, *en attente*, *résidu*. ADR-180 D1 interdit le mot « conforme » tant qu'un
//!   receveur n'existe pas ; `global_conservation_claimable()` le **calcule** au lieu de laisser
//!   une prose l'affirmer.
//!
//! # L'unité n'est pas dans le type, et c'est voulu
//!
//! Le même registre sert au **volume** (m³) et à la grandeur **propagative** que le transfert
//! transporte (une énergie de jauge, ADR-179 D7 : un état, jamais un bilan fermé). Les deux
//! obéissent à la même arithmétique et aux mêmes interdits ; leur donner deux types jumeaux
//! n'apporterait qu'une occasion de diverger. L'appelant tient un registre par grandeur et le
//! nomme lui-même — c'est ce que fait le banc.
//!
//! # Ce que ce module ne fait pas
//!
//! Il ne mesure rien et ne décide rien : il **compte**, dans l'ordre où on l'appelle, en `f64`,
//! sans allocation (I-06) et sans dépendre d'une horloge. Ce qui sort d'un domaine se lit avec
//! [`Volume3::control_flux_x`](super::Volume3::control_flux_x) ; ce que W porte se **mesure** sur
//! les champs de W (S312 §1.2), et le résultat d'aujourd'hui est zéro pour le volume. Ce module
//! ne le sait pas et n'a pas à le savoir : lui faire écrire ce zéro en dur le rendrait faux le
//! jour où un receveur existera.

/// Refus du registre. Aucun n'est un verdict physique : ce sont des entrées inutilisables.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LedgerError {
    /// Une des trois valeurs n'est pas finie.
    NonFinite,
}

/// Registre d'une grandeur suivie à travers le raccord δ → W, cumulée sur les pas.
///
/// Convention de signe : `outgoing` et `transferred` comptent **positivement ce qui quitte le
/// domaine**. `numerical` compte positivement un résidu, quel que soit son signe d'origine.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ledger3 {
    outgoing: f64,
    transferred: f64,
    /// Volume **reçu par une région** — S317, ADR-185. Ne croît que par un reçu.
    restituted: f64,
    numerical: f64,
    worst_numerical: f64,
    steps: u64,
}

impl Ledger3 {
    /// Enregistre un pas. `numerical` est pris en valeur absolue : un résidu n'a pas de sens
    /// signé, et le laisser s'annuler d'un pas à l'autre masquerait exactement ce qu'il mesure.
    pub fn account(
        &mut self,
        outgoing: f64,
        transferred: f64,
        numerical: f64,
    ) -> Result<(), LedgerError> {
        if !outgoing.is_finite() || !transferred.is_finite() || !numerical.is_finite() {
            return Err(LedgerError::NonFinite);
        }
        self.outgoing += outgoing;
        self.transferred += transferred;
        self.numerical += numerical.abs();
        self.worst_numerical = self.worst_numerical.max(numerical.abs());
        self.steps += 1;
        Ok(())
    }

    /// Ce qui a quitté le domaine à travers la surface de contrôle.
    pub fn outgoing(&self) -> f64 {
        self.outgoing
    }

    /// Ce qu'une primitive de W porte **effectivement**. À fournir mesuré, jamais supposé.
    pub fn transferred(&self) -> f64 {
        self.transferred
    }

    /// **Présente le reçu d'une région** — S317, [ADR-185] D7. C'est la **seule** autre façon,
    /// avec le transfert à W, de faire baisser l'attente : le reçu prouve qu'une région a pris le
    /// volume, et il est consommé ici, une fois.
    ///
    /// [ADR-185]: ../../../docs/adr/ADR-185-ordre-d-receveur-sous-i15.md
    pub fn account_restitution(
        &mut self,
        receipt: crate::regional_level::Receipt,
    ) -> Result<(), LedgerError> {
        let v = receipt.volume();
        if !v.is_finite() {
            return Err(LedgerError::NonFinite);
        }
        self.restituted += v;
        Ok(())
    }

    /// Ce qu'une région **a reçu** — par reçus, jamais par déclaration.
    pub fn restituted(&self) -> f64 {
        self.restituted
    }

    /// **En attente de restitution** — sorti, ni transféré à W, ni reçu par une région. Dérivé,
    /// jamais écrit : aucun appel ne peut le réduire autrement qu'en transférant ou en présentant
    /// un reçu.
    ///
    /// **Négatif, il dit qu'on a créé de l'eau**, pas que le compte est bon. Voir [`created`].
    ///
    /// [`created`]: Self::created
    pub fn pending(&self) -> f64 {
        self.outgoing - self.transferred - self.restituted
    }

    /// La part du `pending` qui est **de l'eau créée** : `max(0, −pending)`. Toute valeur non
    /// nulle est un défaut, pas une tolérance (ADR-180 D8).
    pub fn created(&self) -> f64 {
        (-self.pending()).max(0.0)
    }

    /// Résidu numérique cumulé, en valeur absolue, et son pire pas.
    pub fn numerical(&self) -> f64 {
        self.numerical
    }
    pub fn worst_numerical(&self) -> f64 {
        self.worst_numerical
    }
    pub fn steps(&self) -> u64 {
        self.steps
    }

    /// Part du sortant restée en attente, rapportée au sortant. `NaN` si rien n'est sorti — et
    /// c'est la bonne réponse : une fraction de rien n'existe pas.
    pub fn pending_ratio(&self) -> f64 {
        self.pending() / self.outgoing
    }

    /// **La conservation globale est-elle revendicable ?** ADR-180 D1 interdit de l'affirmer en
    /// prose ; cette méthode la calcule. Elle est fausse tant qu'un volume reste en attente, et
    /// fausse aussi si de l'eau a été créée.
    ///
    /// **Et fausse dès qu'une région locale a reçu quoi que ce soit** (S317, ADR-185 §3) : un
    /// receveur local ferme le bilan **de la représentation**, pas celui du monde, que seules les
    /// couches autoritaires portent. Voir [`Self::representation_closed`].
    pub fn global_conservation_claimable(&self) -> bool {
        self.steps > 0 && self.pending() == 0.0 && self.restituted == 0.0
    }

    /// **Le bilan de la représentation est-il fermé ?** Rien en attente, rien de créé : tout ce
    /// qui est sorti est parti dans W ou dans une région locale, par reçus. C'est ce que l'ordre D
    /// peut revendiquer — et rien de plus.
    pub fn representation_closed(&self) -> bool {
        self.steps > 0 && self.pending() == 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sans transfert, l'attente **est** le sortant, exactement — et la conservation n'est pas
    /// revendicable. C'est l'état du prototype d'aujourd'hui pour le volume.
    #[test]
    fn pending_is_the_outgoing_when_nothing_is_transferred() {
        let mut l = Ledger3::default();
        for i in 0..1000 {
            l.account(1e-3 * (i as f64 + 1.0), 0.0, 1e-15).unwrap();
        }
        assert_eq!(l.pending(), l.outgoing());
        assert_eq!(l.transferred(), 0.0);
        assert_eq!(l.pending_ratio(), 1.0);
        assert_eq!(l.created(), 0.0);
        assert!(!l.global_conservation_claimable());
        assert_eq!(l.steps(), 1000);
        assert!(l.worst_numerical() <= 1e-15);
    }

    /// Transférer plus qu'il n'est sorti ne « comble » rien : cela **crée** de l'eau, et le
    /// registre le dit au lieu de l'absorber.
    #[test]
    fn transferring_more_than_left_is_created_water() {
        let mut l = Ledger3::default();
        l.account(1.0, 1.5, 0.0).unwrap();
        assert_eq!(l.pending(), -0.5);
        assert_eq!(l.created(), 0.5);
        assert!(!l.global_conservation_claimable());
    }

    /// Un transfert complet, et seulement lui, rend la conservation revendicable.
    #[test]
    fn a_complete_transfer_is_the_only_way_to_close_the_ledger() {
        let mut l = Ledger3::default();
        l.account(0.25, 0.25, 0.0).unwrap();
        l.account(0.75, 0.75, 0.0).unwrap();
        assert_eq!(l.pending(), 0.0);
        assert_eq!(l.created(), 0.0);
        assert!(l.global_conservation_claimable());
    }

    /// Un registre vierge ne revendique rien : zéro pas n'est pas zéro déficit.
    #[test]
    fn an_empty_ledger_claims_nothing() {
        let l = Ledger3::default();
        assert!(!l.global_conservation_claimable());
        assert!(l.pending_ratio().is_nan());
    }

    /// Le résidu ne s'annule pas d'un pas à l'autre : il se cumule en valeur absolue.
    #[test]
    fn residuals_do_not_cancel_each_other() {
        let mut l = Ledger3::default();
        l.account(0.0, 0.0, 1e-12).unwrap();
        l.account(0.0, 0.0, -1e-12).unwrap();
        assert_eq!(l.numerical(), 2e-12);
        assert_eq!(l.worst_numerical(), 1e-12);
    }

    #[test]
    fn non_finite_entries_are_refused() {
        let mut l = Ledger3::default();
        assert_eq!(l.account(f64::NAN, 0.0, 0.0), Err(LedgerError::NonFinite));
        assert_eq!(l.account(0.0, f64::INFINITY, 0.0), Err(LedgerError::NonFinite));
        assert_eq!(l.account(0.0, 0.0, f64::NAN), Err(LedgerError::NonFinite));
        assert_eq!(l.steps(), 0);
    }
}
