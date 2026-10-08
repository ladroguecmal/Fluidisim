# L'onde trop haute de la 3D : ni la maille, ni le pas — S713 (liste 4.14, 4.16)

*S713, 2026-10-09, en autonomie ; session longue ; l'utilisateur dort et a dit de ne pas s'arrêter.* S712 : contre les mesures de
Synolakis, la 3D fait l'onde trop étroite et trop haute avant le déferlement (0,43 d contre 0,31 d à t = 15). Deux causes candidates,
une à la fois (ADR-276 D2), par la même fonction (`plage_synolakis`).

## Reproduire

- E1 : `python outils/essai.py the_judge_against_synolakis_at_half_the_cell_s713 --ignore` (≈ 1 h 05 jusqu'à t = 15 ; ensuite, le pas
  tombe sous 1 ms) ;
- E2 : `python outils/essai.py the_judge_against_synolakis_small_step_s714 --ignore --marque "S712 photo"` (≈ 33 min) ;
- le graphique : `python outils/graphique_synolakis.py` (`captures/s712_synolakis.svg`, rendu dans `captures/s713_synolakis.png`).

## Mesuré

| | t = 15 : crête | écart | t = 20 : crête | écart | t = 25 : crête | écart |
|---|---|---|---|---|---|---|
| **la mesure** | **0,314 d en 8,38 d** | — | **0,318 d en 3,66 d** | — | **0,190 d en 0,30 d** | — |
| S712 : 2,5 cm, pas ≤ 10 ms (le juge) | 0,433 d en 8,12 d | 0,048 d | 0,277 d en 3,92 d | 0,066 d | 0,234 d en −2,98 d | 0,038 d |
| **E1 : 1,25 cm**, pas ≈ 6 ms | **0,421 d** en 7,95 d | 0,038 d | — | — | — | — |
| **E2 : 2,5 cm, pas ≤ 2,5 ms** | **0,424 d** en 8,07 d | 0,044 d | **0,316 d en 3,77 d** | 0,066 d | 0,218 d en −3,03 d | 0,035 d |

Dans les deux calculs de S712, l'onde parcourait ≈ une maille par pas (`c·dt/dx` ≈ 1). La règle d'APIC, `0,5·dx/(max|v| + √(g·dx))`,
borne l'onde de la taille d'une maille, non l'onde longue (√(g·h) ≈ 2,2 m/s).

## Ce que cela dit

- **La maille n'est pas la cause de l'onde trop haute** (E1 : −0,012 d), **ni le pas** (E2 : −0,009 d). Les critères attendaient plus de
  0,05 d.
- **Le pas compte au déferlement.** Avec 2,5 ms, la crête de t = 20 rejoint la mesure à 0,002 d et 0,11 d près. Avec 10 ms, elle était
  0,04 d trop basse et 0,26 d en arrière. **Le juge de S690–S703, à 10 ms, a donc un instant de déferlement qui dépend du pas.** C'est
  une option numérique de plus au sens d'ADR-280 D2.
- **Ce qui reste ouvert** : l'onde trop haute à t = 15, et la remontée trop longue à t = 25 (≈ 3 d), sous les trois variantes. Les causes
  qui restent sont l'onde réelle du bassin (générée par un batteur, amortie par le frottement des parois sur 15 m) et une limite propre à
  APIC (la surface libre explicite). Aucune ne se tranche sans une mesure de plus.
- **Le coût** : à 1,25 cm, ≈ 1 h 05 pour t = 15, puis des heures, car le pas tombe sous 1 ms au déferlement. À 2,5 cm et 2,5 ms,
  33 min pour t = 25.

## La suite

- **S714** : le juge de S690 avec le pas plafonné à 2,5 ms. Il mesure le déplacement de son déferlement (2,637 s) et dira s'il faut
  abaisser le plafond par défaut des montages de déferlement.
- L'écart au laboratoire à t = 15 et à t = 25 est inscrit comme question ouverte (A-question) ; il n'arrête pas le LOD.
