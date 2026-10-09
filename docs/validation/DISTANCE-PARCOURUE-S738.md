# La 3D trop haute sur S4 : la distance parcourue — S738 (liste 4.14 ; SELECTEUR-DOMAINES-S732, P2)

*S738, 2026-10-09.* LECTURE-PARTICULES-S737 : S4 (`H/d` = 0,2, 1:3, `d` = 0,5 m) remontait à 0,52 m au moins. S645, le même cas à `d` =
0,35 m, remontait à 98 % de la loi. **La question** : quel écart de montage fait la différence ? Une cause à la fois (ADR-276 D2) ; aucun
remède.

## Reproduire

- E1 : `python outils/essai.py ballistic_air_lets_the_swash_run_up_fine_s645 --ignore` (6 min).
- E2 : `python outils/essai.py s4_toward_s645_ceiling_s738 --ignore` (41 min).
- E3 : `python outils/essai.py s4_toward_s645_approach_s738 --ignore` (32 min).

## Mesuré

| essai | ce qui change | la particule la plus haute (S645) | le front par les particules | contre la loi |
|---|---|---|---|---|
| **E1** — S645 tel quel, `d` = 0,35 m | — | **0,2307 m**, exactement comme en S645 | — | 100,5 % |
| S737 — S4, fond lisse, le plafond à 0,60 m | — | ≥ 0,594 m (plafonnée) | 0,517 m | ≥ +58 % |
| **E2** — S4, escalier, le plafond à 1,0 m | le plafond | **0,4808 m** | 0,4335 m | **+47 %** |
| **E3** — E2, l'onde à la distance canonique du pied | **l'approche de 3 m retirée** | **0,3046 m** | 0,2751 m | **−7 %** |

La loi de Synolakis vaut 0,3279 m. Le critère du plan (moins de 15 % de la loi) est atteint en E3 : **l'écart désigné est la distance
parcourue sur le fond plat avant la pente.** E4 et E5 ne tournent pas, comme le plan le prévoyait.

- **E1 : aucune régression.** Le code d'aujourd'hui redonne S645 au millimètre.
- **E2 : le plafond n'était pas la cause** (sur l'escalier). Le figement du front lu par φ à 0,4835 m y est le même qu'en S734.

**La crête sur le fond plat**, lue par la surface, oscille dans les deux cas : 99,8 ; 93,9 ; 90 ; 99,6 ; 94 à 108 mm. Elle arrive au pied à
peu près à la même hauteur : ≈ 104–108 mm pour E2 après 3 m de plus, 107 mm pour E3. **La différence n'est donc pas la hauteur de la
crête.** Elle tient à ce que l'onde est devenue pendant ces 3 m : sa largeur, son champ de vitesse, une traîne.

## Ce que cela dit

- **La 3D ne garde pas une onde solitaire sur le fond plat.** Posée avec une vitesse uniforme sur la verticale et sans vitesse verticale
  (`c·η/(d + η)`), l'onde se transforme en se propageant. Elle arrive alors sur la pente avec de quoi remonter moitié plus haut. Posée près
  du pied, elle n'a pas le temps de changer, et S645 tombait juste.
- C'est le suspect que la littérature décrit (une onde de départ inexacte se scinde et évolue), et il touche aussi :
  - Synolakis (S713 : une onde trop haute et trop étroite à t = 15) ;
  - S1 (une crête de 228 mm contre 168 mm pour SGN) ;
  - les juges de S690 à S730, partis de la même onde.
- **Les témoins de la batterie** (S2, S3, S1) partent de cette même onde : leurs nombres restent ceux de cette onde de départ, à relire
  quand elle sera corrigée.

## La suite (S739)

**L'onde dans la 3D sur un long canal plat**, mesurée avant tout remède (ADR-226 D1) : sa crête, sa largeur, son volume, sa vitesse
(`u(z)`, `w`), sa traîne, en fonction de la distance parcourue. Puis le remède : une onde de départ exacte pour la 3D (la vitesse verticale,
`u(z)` ; une solution de Fenton, ou SGN avec son profil vertical, S698), jugée sur ce canal et sur S4 avec l'approche de 3 m.
