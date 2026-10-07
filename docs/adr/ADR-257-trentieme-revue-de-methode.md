# ADR-257 — Trentième revue de méthode (S626–S630)

- **Statut : actée**, S631, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-256](ADR-256-vingt-neuvieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un montage employé hors de ses bornes** — trois fois en dix sessions : S622, une onde solitaire plus large que le bassin ; S624, une durée plus courte que le trajet de l'onde ; S629, la plage de S625 (sommet à +1 m) réemployée pour des remontées de plusieurs mètres — l'eau butait au mur et la mesure lisait sa hauteur. Chaque fois vu aux valeurs, avant la mesure du code ; en S629, une exploration abandonnée sans commit | trois relances, une exploration perdue | **protection élargie** (D1) |
| La tolérance posée sur la sensibilité mesurée (ADR-256 D1), appliquée en S627, S628, S629 : tenue chaque fois, à un ou deux ordres de grandeur sous la tolérance | — | **rien à changer** |
| L'attribution par deux variations (ADR-256 D2) : en S629, la part non linéaire séparée de la part numérique (l'écart proportionnel à l'amplitude, la part numérique convergente) | — | **rien à changer** |
| S627, S628, S630 tenus du premier essai | — | rien à changer |

## 2. Décision

**D1 — Un montage porte ses bornes, et le script du plan les asserte.** La plage plus haute que la remontée attendue, le domaine plus long
que l'onde, la durée plus longue que le trajet, la grille plus large que le contour : chaque borne s'écrit avec la valeur qui la justifie,
calculée, et le script refuse d'écrire un plan qui la franchit — d'abord quand un montage est réemployé pour un autre cas (élargit ADR-254
D1, du garde-fou du code à celui du montage).

## 3. La prochaine revue

S636.
