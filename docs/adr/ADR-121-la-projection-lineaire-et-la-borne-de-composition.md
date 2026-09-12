# ADR-121 — Projection linéaire et borne de composition avec résidu

- **Statut : actée**, S191, 2026-09-12 ; autonomie technique S71.
- **Précise et restreint la règle 1 d'ADR-119** : la somme des erreurs des axes
  est une enveloppe mesurée, pas une borne universelle sans hypothèse.
- Conserve le seuil utilisateur de **2 %** d'ADR-120 et les mesures S186–S190.

## Décision et dérivation

Pour des opérateurs spatiaux et des bords fixes, une projection de pression exacte
est une application linéaire : `P(a+b)=Pa+Pb`. Son caractère global ne détruit pas
l'additivité. Il peut déplacer les pics, et la contraction L2 ne garantit aucune
contraction du maximum composante par composante employé dans B4.

Définir les champs d'erreur `Δ_rc`, `Δ_r1`, `Δ_1c` contre une référence commune,
et le résidu `R = Δ_rc − Δ_r1 − Δ_1c`. Alors, par inégalité triangulaire :

```
e_rc ≤ e_r1 + e_1c + ||R||∞ / M
```

**La borne sans résidu ne peut être transportée sans vérification.** Elle était
respectée dans les campagnes d'ADR-119, et peut continuer de l'être sans que R soit
nul. Une campagne mesure le champ composé et R ; elle ne remplace pas cette mesure
par l'affirmation « la somme ne suppose rien ». Le résidu vient notamment des termes
non linéaires de l'évolution et de ses approximations, même si chaque P est linéaire.

Pour B4, conserver le dimensionnement par somme et le contrôle direct du composé
d'ADR-120 ; si l'on revendique une **borne algébrique** de composition, ajouter R.
La réserve de référence est ajoutée dans la même normalisation. Aucun des deux
contrôles ne constitue un oracle physique du solveur.

## Conséquences pour la construction

Le projecteur S191 applique `DDᵀ` avec les opérateurs réellement utilisés, et est
reçu contre une matrice indépendante. C'est un **instrument collocatif de banc**,
avec extension nulle et dimensions paires ; il ne choisit pas la discrétisation
δ, les bords ouverts ou la surface libre. Les allocations f64 du banc ne sont pas
proposées comme runtime I-06/I-08.

Le choix d'un solveur à surface libre reste B3. La prochaine réception doit avoir
des bords physiques explicites ; aucun « profil universel » ne découle du succès
de ce projecteur algébrique. Les 2 % ne sont pas remis en arbitrage.

## Réception et ce qui reste ouvert

Voir [PROJECTION-B4-S191](../validation/PROJECTION-B4-S191.md) : tests manufacturés,
référence matricielle indépendante et campagne du profil S190. La linéarité n'est
pas une conclusion statistique de la campagne ; elle découle de l'opérateur fixé.
La proximité de l'évolution non linéaire à cette propriété est, elle, mesurée.

Changer de bords, de domaine actif ou d'opérateur selon le champ peut rendre la
projection globale non linéaire ; la preuve ci-dessus ne couvre pas ces changements.
Surface libre/mobile, comparaison au substitutif, forces/perception et B3 restent
ouverts. Un nouveau budget doit publier ses opérateurs et ses erreurs de résolution.
