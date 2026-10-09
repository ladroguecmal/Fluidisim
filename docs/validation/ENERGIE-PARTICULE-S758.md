# Une projection neutre en énergie, particule par particule : réfutée — S758 (liste 4.1 ; ADR-289 D3.2)

*S758, 2026-10-10.* BALLOTTEMENT-S757 : la 3D corrigée a la période juste, mais l'oscillation grandit de 2,8 % par période ; la projection
ajoute de l'énergie potentielle (S752). **L'hypothèse nommée** (ADR-290 D1) : la projection élève des particules sans rien leur prendre ;
rendre à chaque particule l'énergie de son déplacement vertical, `|v|² ← max(|v|² − 2g·Δz, 0)`, arrête l'injection.

## Ce qui est fait

`set_density_energy_neutral` (éteint par défaut, au bit) : la règle ci-dessus, la vitesse mise à l'échelle dans sa direction.

## Reproduire

`python outils/essai.py energy_neutral_projection_s758 --ignore` (18,5 min).

## Mesuré

| essai | la 3D corrigée (S744–S757) | **avec la règle** | critère |
|---|---|---|---|
| le repos, l'escalier 1:30 | 6,8 mm/s ; 0,02 mm | **0,16 m/s ; 7,8 mm** | 1 cm/s ; 3 mm |
| le repos, l'escalier 1:12 | 6,8 mm/s ; 0,07 mm | **0,11 m/s ; 8,4 mm** | 1 cm/s ; 3 mm |
| le ballottement, la période | +0,33 % | **−36,7 %** | 1 % |
| le ballottement, l'amortissement | −2,8 % par période | **68 % par période** | −0,5 à +1 % |
| le bilan propre, cinétique / potentielle | 0 / +2,1 J | **+5,9 J** / +2,2 J | — |
| l'onde solitaire : la largeur, le creux, la célérité | 92 % ; 9 mm ; +0,4 % | 59 % ; 10,9 mm ; −4,3 % | 80 % ; 10 mm ; 1 % |

**Tous les critères tombent. L'hypothèse est réfutée.**

## Ce que cela dit

- Les déplacements de la projection **ne sont pas des mouvements dans la pesanteur** : ils vont autant vers le bas que vers le haut. La
  règle accélère toutes les particules déplacées vers le bas (+5,9 J d'énergie cinétique) et détruit le repos.
- Le gain d'énergie potentielle vient du gonflement d'ensemble de l'eau, que la projection restaure, et non de chaque particule.

## La suite (S759)

**Une correction globale** : à chaque pas, l'énergie potentielle que la projection ajoute en tout est retirée à l'énergie cinétique, par une
mise à l'échelle uniforme des vitesses. L'énergie totale est conservée sans toucher les particules une à une. Elle est jugée sur le repos, le
ballottement et l'onde solitaire.
