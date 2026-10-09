# L'onde solitaire se déforme dans la 3D — S739 (liste 4.1, 4.14 ; SELECTEUR-DOMAINES-S732, P2)

*S739, 2026-10-09.* DISTANCE-PARCOURUE-S738 : la remontée de S4 était trop haute à cause de la distance parcourue sur le fond plat.
**La question** : comment l'onde solitaire se transforme-t-elle dans la 3D sur un fond plat ? Une onde de départ qui porte son profil
vertical se garde-t-elle mieux ?

## Ce qui est fait

- `canal_s739` : un canal plat de 24 m, `d` = 0,5 m, `H` = 0,1 m (`H/d` = 0,2), deux rangées, des murs ; l'onde centrée à 4 m, 5 s. Toutes
  les 0,25 s : la crête (par la surface lissée sur 10 cm, et par les particules), la largeur à mi-hauteur, le creux derrière (de 1,5 à 4 m
  en arrière).
- Deux ondes de départ :
  - **A**, celle d'aujourd'hui : Boussinesq, `u = c·η/(d + η)` uniforme, `w = 0` ;
  - **B**, Rayleigh avec le profil vertical de SGN : `u(z) = ū + (h²/6 − z²/2)·ū_xx`, `w = −z·ū_x`.

## Reproduire

- `python outils/essai.py the_solitary_wave_in_a_flat_channel_s739 --ignore` (A et B à 5 cm, 6 min).
- `python outils/essai.py the_solitary_wave_in_a_flat_channel_fine_s739 --ignore` (A à 2,5 cm, 15 min).

## Mesuré

Une onde solitaire doit garder sa forme : la largeur à mi-hauteur vaut 2,27 m, la célérité 2,43 m/s.

**A à 2,5 cm** (la maille des témoins) :

| t | crête (surface) | crête (particules) | largeur à mi-hauteur | creux derrière |
|---|---|---|---|---|
| 0,25 s | 96,4 mm | 98,6 | **2,35 m** | −4,8 mm |
| 1,00 s | 101,4 | 110,1 | 1,86 | −8,1 |
| 2,00 s | 102,2 | 107,3 | 1,59 | −16,6 |
| 2,50 s | 103,1 | 110,5 | 1,08 | −17,9 |
| 3,00 s | 125,2 | 135,5 | 0,75 | −22,5 |
| 4,00 s | **149,7** | 164,6 | 0,52 | −41,6 |
| 5,00 s | 148,7 | 164,9 | **0,45 m** | **−41,8 mm** |

La crête avance de 4,49 m à 14,89 m en 4,75 s : **2,19 m/s**, contre 2,43 m/s attendus (−10 %).

**À 5 cm**, A et B font de même, et plus vite :
- A : la largeur passe de 2,33 m à 0,49 m en 4,25 s, le creux atteint −54 mm ;
- B : la largeur passe de 2,43 m à 0,57 m, le creux atteint −47 mm.

**Critères** :
- E2, B retenue : **non**. B ne fait pas mieux que A ; l'onde de départ n'est pas la cause.
- E3, A à 2,5 cm, la largeur au-dessus de 80 % et le creux sous 10 % de `H` : **manqué** (19 % ; 42 %).

Le résumé imprimé par l'essai (±inf) est faux : sa mesure à t = 0 est prise avant toute reconstruction de la surface. Les séries ci-dessus
font foi.

## Ce que cela dit

- **La 3D ne garde pas une onde solitaire.** Sur un fond plat, elle la tient environ 2 s, puis la raidit : la crête grandit de 50 %, la
  largeur est divisée par 5, un creux se forme derrière, la célérité baisse de 10 %. C'est le comportement d'un modèle qui perd sa
  dispersion. La maille n'en est pas la cause (le défaut est le même à 5 et à 2,5 cm, plus lent à 2,5 cm), ni l'onde de départ.
- **Ce défaut explique tout ce qui était ouvert** :
  - la remontée de S4 trop haute de 47 % avec 3 m d'approche (S738) ;
  - la crête de R43 et de S1, 228 mm contre 168 mm pour SGN (S733) ;
  - l'onde « trop haute et trop étroite » contre Synolakis (S713).

  Il touche aussi tous les juges du déferlement depuis S690, qui partent de la même onde et de la même 3D.
- **Le suspect suivant : la pression.** C'est elle qui porte la dispersion dans un modèle 3D. Elle est résolue par un gradient conjugué
  préconditionné par la diagonale, plafonné à 4 000 itérations (`PRESSURE_MAX_ITERATIONS`). Sur un domaine long et mince (960 × 20 mailles),
  ce solveur converge lentement, et un arrêt avant la convergence laisse justement les grandes longueurs d'onde fausses. Autres suspects, à
  ordonner par la mesure : la séparation des particules, la perte de volume en mouvement (S708), le pas de temps (une maille par pas).

## La suite (S740)

1. Le nombre d'itérations de la pression, et son résidu, à chaque pas du canal : le plafond est-il atteint ?
2. Le canal, le plafond levé (ou une tolérance plus fine) : l'onde se garde-t-elle ?
3. Sinon, les autres suspects, un à la fois.
