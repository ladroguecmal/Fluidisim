# Le juge éprouvé : la crête de l'onde sur fond plat — S704 (liste 4.14)

*S704, 2026-10-08, en autonomie.* S703 : à pose égale, les données de SGN avancent le retournement de 0,089 s. Au plan de 5 m (S700), la
crête de la 3D était lue à 0,139 m au-dessus du niveau, celle de SGN à 0,150 m. Question : le juge, le tout-3D à 2,5 cm, amortit-il l'onde ?

## Ce qui est fait

L'onde de départ (a = 0,15 m, d = 0,5 m, x₁ = 3,4 m), seule, sur un fond plat de 8 m, pendant 0,8 s. On lit la plus haute hauteur d'eau à
cinq plans, dans trois calculs :
- APIC 3D à 2,5 cm sur quatre rangées, le juge ;
- APIC 3D à 1,25 cm sur deux rangées ;
- SGN.

La hauteur de la 3D est lue **par le volume** des particules d'une tranche de 10 cm. La résolution est de 0,2 mm ; le lissage de la crête,
`(k·w)²/3`, de 0,1 mm.

**L'instrument a été corrigé avant toute attribution.** Une première lecture sur une tranche d'une maille (`|x − plan| < dx/2`) a donné
0,29 m au plan de 5 m. C'était le regroupement passager des particules, que le maximum dans le temps retenait ; le pas, lui, restait à
10 ms.

## Reproduire

- `python outils/essai.py the_judge_on_a_flat_bed_s704 --ignore` (≈ 5 min).

## Mesuré

La crête au-dessus du niveau, en mètres :

| plan | 3,4 m | 4,0 m | 4,5 m | 5,0 m | 5,4 m |
|---|---|---|---|---|---|
| 3D à 2,5 cm (le juge) | 0,1500 | 0,1516 | 0,1490 | 0,1469 | 0,1502 |
| 3D à 1,25 cm | 0,1500 | 0,1434 | 0,1406 | 0,1449 | 0,1430 |
| SGN | 0,1499 | 0,1513 | 0,1510 | 0,1501 | 0,1495 |

Les particules sont tenues : 220 752 et 441 768, sans perte. 58 s et 235 s.

## Ce que cela dit

- **Le juge n'amortit pas l'onde : l'hypothèse est réfutée.** Plus fine, la 3D donne une crête *plus basse*, ≈ 0,143 m sur les quatre plans
  aval, contre ≈ 0,149 m à 2,5 cm. L'onde de départ (un profil de Boussinesq, une vitesse uniforme sur la verticale) n'est pas une onde
  solitaire exacte de la 3D : celle-ci la réajuste un peu plus bas.
- **SGN garde 0,150 m**, au-dessus de la 3D convergente d'environ 7 mm, soit 5 %. C'est le sens de S703 : une onde un peu plus haute
  déferle un peu plus tôt.
- **Le juge à 2,5 cm n'est pas convergé lui non plus** (≈ 6 mm de crête en trop). Son retournement n'est pas une vérité au centième de
  seconde.
- **Le 0,139 m de S700** venait de la lecture par la plus haute particule (+ dx/4), non de l'onde.

## Rapporté à l'usage

L'écart restant du raccord par la grille nourri par SGN (S703) est de −0,068 s et −0,13 m. À la célérité de l'onde (≈ 2,5 m/s), c'est
une douzaine de centimètres sur une plage : sous ce qu'un joueur peut voir. Et il est du même ordre que l'écart du juge lui-même à sa
propre convergence. Pousser le raccord plus près d'un juge non convergé ne rendrait rien de visible (la règle de la précision rapportée à
l'usage).

**La suite proposée (S705)** :
- retenir le raccord du large de S703 : le bord à particules, la pose par la grille, les données de SGN ;
- inscrire ses écarts dans la liste ;
- reprendre le LOD de simulation (ADR-275) : la bande 3D qui naît et meurt avec la vague.

La crête de SGN 5 % trop haute reste une question ouverte, celle de l'onde de départ.

*Note datée du 2026-10-08 (S707)* : les hauteurs de cette preuve sont lues par le compte des particules. S707 a montré qu'APIC tasse ses
particules sous la crête (+3,8 %), si bien que le compte n'en donne pas la surface. L'écart entre 2,5 et 1,25 cm peut donc venir du
tassement et non de l'onde. Il est à relire par la surface (S708).
