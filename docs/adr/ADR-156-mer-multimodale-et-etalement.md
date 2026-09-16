# ADR-156 — Une mer à plusieurs systèmes, chacun avec sa loi d'étalement directionnel

Actée S259, 2026-09-16, autonomie S71. Répond au verdict R2
([REVUE-VISUELLE](../validation/REVUE-VISUELLE.md) §8) et à A287. Complète ADR-100/101 (bande
explicite), et ADR-155 (queue). Ne modifie ni `bake`, ni son empreinte, ni la scène par défaut.

## Constat

B est une seule mer de vent JONSWAP : il n'y a pas de houle longue (verdict R2). Sa cuisson donne
à chaque composante la direction `θ + étalement·((i + ½)/N − ½)` : la direction suit le rang, donc
la fréquence. C'est une fixture (ADR-100, commentaire de `Recipe`), et elle produit les stries de R2.

## Décision

1. **Un système = une recette spectrale plus une loi d'étalement.** Chaque système garde les
   cellules, amplitudes, nombres d'onde et fréquences de `bake` **au bit**. Seules les directions
   changent : `θ_i = θm + F_s⁻¹(u_i)`, avec `D ∝ cos^{2s}` et `s(f/fp)` de Mitsuyasu, `s_max` déclaré
   (SPEC-001 §1 septies).
2. **Directions indépendantes du rang.** `u_i = frac(½ + i·φ⁻¹)`, `φ⁻¹ = 0,618 033 988 7` : une
   suite de Weyl déterministe, équirépartie, non monotone en `i`. L'inverse `F_s⁻¹` se calcule
   dans la cuisson, en f32 sans libm : table de la fonction de répartition par Simpson, puis
   recherche et interpolation linéaire.
3. **Assemblage.** Une mer est la concaténation de systèmes de même gravité, au plus 256
   composantes. Les indices de phase de chaque système sont décalés de `2⁴⁰·rang du système`. Le
   `Hs` total vaut `√Σ Hs²`, puisque les systèmes ont des phases indépendantes.
4. **Queue** (ADR-155) : celle du système de vent, avec les amplitudes de `bake_tail` au bit et
   des directions selon la même loi. Au-delà de la bande représentée, `s` est **gelé** à sa valeur
   au bord de bande (`s_max·b^-2,5`). Cette convention n'est pas reçue par une observation.
5. **Pas de migration silencieuse.** `bake`, `bake_tail`, l'empreinte figée et la scène S201
   restent au bit. La mer multimodale est une **variante déclarée** de l'afficheur (`--houle`). Son
   adoption comme scène par défaut serait une décision distincte, prise après revue.
6. **Scène `--houle`, fixture déclarée et non calibrée.** Mer de vent S201 (`Hs` 1,5 m, `Tp` 6 s,
   γ 3,3, `θm` 0,12 tour, `s_max` 10) et houle longue (`Hs` 2 m, `Tp` 12 s, γ 7, bande
   `[0,7 ; 1,6] fp`, `θm` 0 tour, `s_max` 75), soit 32 composantes chacune. `Hs` total 2,5 m.
   Houle de 225 m, à 43° du vent.
7. **Réception** : [MER-MULTIMODALE-S259](../validation/MER-MULTIMODALE-S259.md), écrit avant le code.

## Hors de cette décision

Levée et réfraction sur la bathymétrie (J5), crêtes non linéaires, écume. Étalement bimodal des
ondes courtes et spectre court de Cox–Munk : A287 reste ouvert sur la rugosité. Houles multiples
issues d'une météo, et variation régionale de la recette (I-09).
