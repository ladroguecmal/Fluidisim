# ADR-250 — Vingt-troisième revue de méthode (S591–S595)

- **Statut : actée**, S596, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-249](ADR-249-vingt-deuxieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Une propriété d'ensemble jugée sur un seul tirage** (S594 : « deux réalisations indépendantes mélangées perdent 21 % » — vrai en moyenne ; un couple de graines, corrélé à ρ = −0,43, en perdait 32 %). La formule relue d'abord était juste ; l'ensemble de 800 couples, ajouté par les notes, a tenu | un critère manqué, publié | **protection élargie** (D1) |
| Un essai de 65 s (S593 : 216 000 pas de 20 biefs) — le pas imposé par la stabilité, calculé au plan | la suite passe de ~90 à ~160 s | **rien à changer** : sous le plafond du rituel ; à revoir si la suite dépasse 5 min |
| Le script du plan qui vérifie ses rapports seuil/quantum (ADR-249 D1), S592–S595 ; trois références pour la rivière (S593) | — | **rien à changer** |

## 2. Décision

**D1 — Une propriété d'ensemble (une moyenne sur des tirages : des graines, des phases, des conditions initiales) se juge sur un ensemble**
dont la taille est calculée au plan, l'écart-type de l'estimateur au moins dix fois sous le seuil (ADR-236) — jamais sur un seul tirage
(élargit L376 : « un écoulement instable se compare en statistiques »).

## 3. La prochaine revue

S601.
