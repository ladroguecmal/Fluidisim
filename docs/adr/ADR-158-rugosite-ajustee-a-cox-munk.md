# ADR-158 — Rugosité ajustée à Cox–Munk : coupure de la queue et modulation par la bande

Actée S261, 2026-09-17, autonomie S71. Répond au verdict R4
([REVUE-VISUELLE](../validation/REVUE-VISUELLE.md) §11). Complète ADR-157, sans le modifier :
`--vagues` reste reproductible au bit.

## Constat

Le verdict R4 dit : trop rugueuse, lisse entre les pics, beaucoup de vaguelettes. Mesuré : `mss` 13 %
au-dessus de Cox–Munk, et une pointe `c40` à la moitié de l'observé. Parmi 18 candidats, le critère
écrit avant mesure retient une coupure de la queue à `28 fp` et une modulation de l'énergie des
ondes courtes par la bande, d'intensité `M` = 2. Il donne `mss` 0,0435, et `c40`, `c22`, `c04` à 0,353,
0,129 et 0,351. Cox–Munk borne la modulation : une surface franchement lisse entre les pics
dépasserait la pointe observée.

## Décision

1. **Coupure** : la queue d'équilibre ne rend que ses composantes jusqu'à `28 fp` (`λ` ≥ 7,2 cm à
   `Tp` 6 s). La cuisson d'ADR-157 reste identique ; l'hôte limite le nombre de lignes lues.
2. **Modulation** : chaque composante de la queue reçoit le facteur d'amplitude
   `√max(0, 1 + M·ε)`, avec `ε = Σ_bande a·k·sin ψ` au point de Lagrange. La compression orbitale est
   maximale aux crêtes. Même mécanisme que la modulation hydrodynamique (Longuet-Higgins et Stewart,
   1960 ; Keller et Wright, 1975). Son intensité n'est pas prédite par ces théories : elle est
   **ajustée**, `M` = 2, contre la pointe de Cox–Munk.
3. **Ajustements déclarés.** `b_Q` = 28 et `M` = 2 viennent d'un critère contre Cox–Munk, pour une mer
   à 8 m/s. Ce ne sont pas des lois, et ils se recalent si le vent de la scène change.
4. **Variante** `--modulation`, sur `--vagues`. Défaut, `--houle` et `--vagues` au bit.
5. **Réception** : [RUGOSITE-S261](../validation/RUGOSITE-S261.md).

## Hors de cette décision

Le vent de la scène comme paramètre, qui relierait `Hs`, `Tp`, `b_Q` et `M` à `W`. La transition des
ondes non résolues vers la BRDF (Bruneton et al., 2010). L'anisotropie des ondes courtes,
l'asymétrie des pentes et A288, qui restent ouverts.
