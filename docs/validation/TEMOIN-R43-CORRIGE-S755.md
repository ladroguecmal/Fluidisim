# Le témoin de R43 refait avec la 3D corrigée — S755 (liste 4.14 ; ADR-289 D3.3, ADR-292)

*S755, 2026-10-10.* **La question** : avec une 3D qui garde l'onde (ADR-292), où et quand la vague de R43 plonge-t-elle, et de combien les
mesures du témoin changent-elles ?

## Ce qui est fait

`Large::AucunCorrigee` : le tout-3D de S730 E2 (le raccord du rivage à 12,0 m), 5 s, avec la projection `Complete` consciente du fond ; le
film dans `calculs/s755_tout3d_corrigee.bin`.

## Reproduire

`python outils/essai.py the_r43_witness_with_the_corrected_3d_s755 --ignore` (21 min).

## Mesuré

| | S730 E2, sans correction | **S755, la 3D corrigée** |
|---|---|---|
| le retournement | 2,620 s ; 9,938 m | **3,092 s ; 11,088 m** |
| l'air enfermé | 2,804 s ; 10,375 m | **3,257 s ; 11,425 m** |
| la remontée | 0,3547 m à 3,942 s | **0,3005 m à 3,920 s** |
| le mur au raccord (12,0 m), de 2,4 à 3,2 s | 0 mm | 0 mm |
| la masse | 1,2·10⁻¹⁶ | 1,2·10⁻¹⁶ |
| le coût | 1 152 s | 1 235 s (+7 %) |

**Critères** : (1) la masse, (2) le mur, (3) un retournement — **tenus**.

Le prédicteur SGN (S733), rapporté : Kennedy 0,65 à 2,73 s et 10,46 m ; Froude 0,8 à 2,97 s et 10,79 m. La 3D corrigée déferle plus tard
encore, et dans le même sens.

## Ce que cela dit

- **La vague de R43 déferle 0,47 s plus tard et 1,15 m plus près du rivage**, dans environ 5 cm d'eau au repos (le rivage est à 11,70 m).
  L'ancienne 3D, qui raidissait l'onde, la faisait plonger trop tôt. Sa remontée baisse de 15 %.
- **Le raccord du rivage à 12,0 m devient trop près** pour cette vague. Le jet retombe vers 11,4 m, et la règle d'ADR-284 D2
  (`x_b + 3·L_jet`) le placerait vers 12,2 m. La fenêtre du mur s'arrête à 3,2 s, juste après le déferlement : elle ne peut pas le voir. À
  reprendre avec les raccords (ADR-289 D3.5).
- **Les mesures qui dépendaient de l'ancienne 3D sont à relire**, avec ce témoin : le juge du déferlement, les témoins du sélecteur, la
  vague de bout en bout.
