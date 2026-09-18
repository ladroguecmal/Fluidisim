# ADR-152 — Surface mobile couplée : géométrie totale, hauteur perturbative, bande du fond

Actée S253, 2026-09-16, autonomie S71. Étend ADR-149 (source volumique B/W→δ) à la surface
géométriquement mobile de S237. Conserve ADR-141 (coefficients temporels) et ADR-143/144
(acceptation). L'affinage d'ADR-150/151 ne s'applique pas au mode mobile.

## Équations

La perturbation s'écrit `v = u − U`, `p' = p_dyn − P`, `η' = ζ − ζ_fond`, avec `ζ` la surface
totale et `(U, P, ζ_fond)` le fond B/W. Le fond est supposé **linéaire, à surface linéarisée au
plan moyen** : `ζ_fond,t = W(x, 0)` et `P(x, 0) = ρg·ζ_fond`. Il est prolongé au-dessus du plan
moyen de façon **incompressible**, et `S` se calcule sur les champs de ce prolongement (ADR-149).

- **Volume** : ADR-149, inchangé.
- **Surface dynamique** (`p = 0` en `z = ζ`) : `p'(Γ) = ρg(z_Γ − repos) − P(Γ)`.
- **Surface cinématique.** Le total conserve sa masse et le fond du domaine est imperméable, pour
  le total comme pour le fond (`W(b) = 0`). On a `ζ_t = −∂x∫_b^ζ (U + v) dz` et
  `ζ_fond,t = W(0) = −∂x∫_b^0 U dz`. Par différence :

  ```
  η'_t = −∂x [ ∫_b^ζ v dz + ∫_0^ζ U dz ]
  ```

  Le flux du fond sous le plan moyen ne se discrétise jamais, et un fond exact n'est pas altéré.
  Le terme `∫_0^ζ U dz` contient à lui seul le résidu cinématique du fond linéaire :
  `ζ_t + U(ζ)ζ_x − W(ζ) = ∂x∫_0^ζ U dz`, par incompressibilité du prolongement.

## Discrétisation

- **État** : `eta = repos + η'`, somme compensée de S233 ; `repos` = plan moyen du fond.
  Échantillons aux faces MAC, `z` compté depuis le repos. Un domaine naît à `η' = 0` (I-12).
- **Géométrie** : `ζ_i = eta_i + ζ_fond,i`, où `ζ_fond,i` est le champ `eta` des échantillons w
  de la colonne, qui doivent être identiques au bit (refus `BackgroundContext` sinon).
  Le mouillage, les `θ` et les gardes de S237 portent sur `ζ`.
- **Fantôme vertical** (maille haute de la colonne) : valeur S237, plus `ρg·ζ_fond,i`, moins
  `P_f + (ζ_i − z_f)·∂zP_f`, pris à la face w située au-dessus de la maille.
- **Fantôme latéral** : valeur S237, moins `P_f + (x_Γ − x_f)·∂xP_f`, pris à la face u entre les
  deux colonnes, avec `x_Γ` au `θ` borné de S237.
- **Transport** : débit S237 inchangé au bit, `Q_v = Σ ouverture·v·dx·mouillé_k(ζ_f)`, plus la
  **bande** `Σ ouverture·U_f·dx·(mouillé_k(ζ_f) − mouillé_k(repos))`, avec `ζ_f` la demi-somme
  des deux colonnes. Tous les débits lisent `ζ^n`.
- **Ordre du pas** : gardes, sauvegarde, géométrie et valeurs du fond, prédiction couplée
  (advection S237 plus termes d'ADR-149), projection mobile, extrapolation, transport,
  validation, garde sur `η'^{n+1} + ζ_fond(t_n)`, publication. Tout refus est atomique, `η'` et
  restes compris. L'éponge agit sur la vitesse seule.

## Domaine et limites

2D x-z, surface graphe, fond linéaire, pas de flux du fond au fond du domaine. Hors de ce
domaine :

- la **frontière du total** (`W(b) ≠ 0`, B profond sur un fond fini, coques) ;
- le **prolongement de B en production** : ADR-113 refuse `z > 0`, et le prolongement de Taylor
  d'ordre un n'est pas incompressible ;
- un **fond non linéaire**, qui exigerait `ζ_fond,t` ;
- la **relaxation de `η'`** vers zéro, la multigrille et l'affinage du mode mobile.

[Protocole et réception](../validation/SURFACE-COUPLEE-S253.md).

**Note S253, 2026-09-16.** L'exclusion de l'affinage en mode mobile (en-tête et dernière liste) est
levée pour le pas couplé par [ADR-153](ADR-153-affinage-en-mode-mobile-couple.md), après un refus
au premier pas à 128 colonnes. Le pas S237 total reste sans affinage.

**Note S254, 2026-09-16.** Le « prolongement de B en production » de la liste des limites est levé
pour B par [ADR-154](ADR-154-prolongement-borne-du-fond.md) : règle bornée, reçue contre l'oracle
S253. Les couches W au-dessus du plan moyen restent hors domaine.

**Note S273, 2026-09-18.** La formule de la bande (§ Discrétisation, « Transport ») est remplacée
par la quadrature linéaire d'[ADR-166](ADR-166-quadrature-lineaire-de-la-bande.md) : la règle
`U_f·dx·(mouillé_k(ζ_f) − mouillé_k(repos))` est d'ordre un sur une couche partielle (S272).
