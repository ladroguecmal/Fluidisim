# ADR-157 — Queue d'équilibre en f⁻⁴ et vagues pointues de Lagrange

Actée S260, 2026-09-17, autonomie S71. Répond au verdict R3
([REVUE-VISUELLE](../validation/REVUE-VISUELLE.md) §10) et à A287. Complète ADR-155 (queue en
pentes par pixel) et ADR-156 (mer multimodale). Ne modifie ni la scène par défaut, ni `--houle`.

## Constat

Verdict R3 : trop lisse, trop de petites bosses, pas assez de mini pics, rien d'uniforme. Mesuré
sur la même réalisation, le rendu a des **pentes gaussiennes** (`c40` −0,026 contre 0,40 selon
Cox–Munk) et une rugosité de 0,020 contre 0,044. Parmi les candidats calculés, une seule combinaison
atteint **sans ajustement** la rugosité et la pointe observées : une queue d'équilibre en `f⁻⁴`
avec CWM. Elle donne `mss` 0,0495, et `c40`, `c22`, `c04` à 0,21, 0,07 et 0,21, dans les
incertitudes. CWM seul, ou le second ordre par composante, ne suffisent pas.

## Décision

1. **Queue d'équilibre.** `bake_tail_equilibrium(recette, s_max, b_Q, N)` continue la densité
   absolue de la bande en `f⁻⁴` depuis `b·fp`, jusqu'à `b_Q = 32` (`λ` ≥ 5,5 cm en `Tp` 6 s). Les
   poids sont analytiques par cellule (SPEC-001 §1 octies), et les directions suivent ADR-156
   (`s_max` 10, gelé au-delà de la bande).
2. **CWM, λ = 1, sans coefficient libre.** Le rendu déplace chaque sommet de la grille de
   `D_B(q)`, le déplacement de la bande, pondéré par le filtre d'ADR-148. Le fragment calcule
   `∂D_T` et `∇η_T` de la queue au point de Lagrange interpolé, pondérés par le filtre d'ADR-155.
   Il reçoit `∂D_B` et `∇η_B` du sommet, puis prend la normale de `(I + ∂D_B + ∂D_T)⁻ᵀ·(∇η_B + ∇η_T)`.
   Si `det J < 0,1`, il retombe sur la pente de Lagrange : le repli est compté par la vérification,
   jamais masqué dans le banc.
3. **Couches W non déplacées** : impacts et sillages sont évalués au point de Lagrange `q`, et
   l'écart `D·∇W` est publié.
4. **Requêtes de jeu inchangées.** Elles restent eulériennes et linéaires. La surface rendue s'en
   écarte d'un ordre `k·a²`, mesuré et publié. Une requête eulérienne CWM, qui demande l'inversion
   de `x = α + D(α)`, reste à construire (A288).
5. **Variante déclarée** `--vagues`, sur la recette de `--houle`. Défaut et `--houle` au bit.
6. **Réception** : [VAGUES-POINTUES-S260](../validation/VAGUES-POINTUES-S260.md), écrit avant le code.

## Ce qui reste hors de portée de cette décision

- L'**asymétrie des pentes** due au vent (`c03` −0,22) et l'**asymétrie de l'élévation**
  (interactions du second ordre entre composantes) : aucun candidat ne les donne.
- L'**alignement des ondes courtes** sur le vent (`σu²/σc²` 1,37 observé, 0,96 ici) : l'étalement
  gelé de la queue est trop large.
- Écume, micro-déferlement, capillaires, et modulation par les rafales.
