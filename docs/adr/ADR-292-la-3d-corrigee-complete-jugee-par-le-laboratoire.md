# ADR-292 — La 3D corrigée : `Complete` consciente du fond, jugée par le laboratoire

- **Statut : actée**, S754, 2026-10-09. Un arbitrage technique tranché ici, par écrit (ADR-222) ; remplace ADR-291 D1 (sans le réécrire).

## Contexte

ADR-291 avait retenu R1 (`Complete`, consciente du fond, la vitesse reprise de la grille), sur la remontée de S645 jugée contre une loi
théorique. Contre les mesures de Synolakis (S753), R1 a la bonne hauteur, mais ralentit la vague de 5,6 %, et son retard grandit. S754 a
lancé `Complete` consciente du fond, sans R1, sur le même montage.

## Décision

**D1 — La 3D corrigée est `Complete` consciente du fond** (`enable_density_projection` + `set_density_bed_aware(true)`), **sans R1**. La
référence extérieure tranche (ADR-280 D2) :

| t·√(g/d) | la mesure | sans correction | R1 | **`Complete` consciente** |
|---|---|---|---|---|
| 15 | 0,314 d en 8,38 d | 0,424 d ; écart 0,044 d | 0,326 d en 9,42 d ; 0,030 d | 0,364 d en 7,77 d ; 0,035 d |
| 20 | 0,318 d en 3,66 d | écart 0,066 d | 0,319 d en 5,82 d ; 0,075 d | 0,323 d en 2,97 d ; **0,047 d** |
| 25 | 0,190 d en 0,30 d | écart 0,035 d | 0,450 d en 0,72 d ; 0,059 d | 0,197 d en −1,53 d ; **0,025 d** |

`Complete` fait mieux que la 3D sans correction aux trois instants, et mieux que R1 à t = 20 et à t = 25, dont la remontée sur la plage. Sa
célérité est juste (+0,4 % sur le canal, S752) ; elle garde le repos (S744) et l'onde solitaire (92 % ; 9 mm, S744).

**D2 — Le critère (1) de S754 est manqué de 0,0009 d** : la crête à t = 15, 0,0509 d au-dessus de la mesure pour un seuil de 0,05 d. C'est
0,45 mm, vingt fois sous le quantum de la lecture (`dx/4` ≈ 6 mm, ADR-288 D2). La règle du plan, prise à la lettre, gardait R1 ; elle
est levée ici, par écrit, pour cette raison, et non effacée.

**D3 — Ce qui reste ouvert** :
- la remontée de S645 contre la loi théorique, sur une pente de 1:3 : −11 %. Le laboratoire (1:19,85) ne la contredit pas, mais ne la juge
  pas non plus ;
- la crête un peu en avance à t = 15 et t = 20 (0,6 à 0,7 d) ;
- l'eau un peu soulevée par la projection (+2 J en 4 s, S752).

## Conséquences

- Les montages nouveaux activent `Complete` consciente du fond. R1 reste une option du cœur, éteinte.
- ADR-289 D3.3 continue : refaire R43, S1 et les témoins du sélecteur avec la 3D corrigée.
