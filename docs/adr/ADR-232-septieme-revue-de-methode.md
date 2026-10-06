# ADR-232 — Septième revue de méthode (S512–S515)

- **Statut : actée**, S516, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-231](ADR-231-sixieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un corps d'essai porteur d'un degré de liberté que l'analytique ignore** : la bouée flottante qui tangue sous sa propre traînée (S513, 17 % d'écart à une dérive de translation pure) ; la coque sans amortissement qui rebondit hors de l'eau et fait un second impact (S512, l'essai du seuil faussé) — malgré ADR-228 D1, écrit pour la marge de stabilité, pas pour les couplages | deux reprises | **protection changée** (D1) — une erreur répétée sous une protection trop étroite |
| **Des ordres de grandeur écrits de tête, puis faux** : la prévision du témoin d'un retrait sec (S510 : « ≈ 100 % » pour 8,6 %), l'écart d'interpolation et le débit d'une pompe avec pertes (S515 : 1,9 % pour 1,6 %, 6,32 l/s pour 6,83) | des plans à corriger | **protection nouvelle** (D2) |
| Un hôte d'essai recopié avec une signature fausse (S514) | une compilation | **rien à ajouter** |
| Un premier manqué de C20 : les pas symplectiques en l'air faisaient manquer l'instant du passage (S512) | une reprise | localisé avant remède (ADR-226 D1) ; **rien à changer** |
| Les enchaînements arrêtés au premier échec (ADR-231 D1), le rappel du lot en dernière ligne (D2) | — | appliqués sur S512–S515 ; **rien à changer** |

## 2. Décisions

**D1 — Un corps d'essai n'a que les degrés de liberté que la référence décrit** (élargit ADR-228 D1) : avant de comparer à une analytique,
nommer ses hypothèses (translation pure, une seule entrée dans l'eau, pas de rotation…) et choisir un corps qui les tient — un corps neutre
immergé, symétrique, amorti — ou borner l'essai à la fenêtre où elles tiennent.

**D2 — Un ordre de grandeur s'écrit après l'avoir calculé** (une ligne de script), jamais de tête : le plan porte le nombre calculé.

## 3. La prochaine revue

S521.
