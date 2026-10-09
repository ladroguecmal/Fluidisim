# Deux remèdes au freinage de la lame — S750 (liste 4.1 ; ADR-289 D3.1)

*S750, 2026-10-09.* CIBLE-SURFACE-S749 : aucune des six variantes de la projection ne tenait le repos, l'onde solitaire et la remontée.
Celles qui corrigent la surface freinaient la lame (−11 à −12 %). **La question** : l'un de deux remèdes rend-il la remontée à `Complete`
(consciente du fond), qui tient déjà le repos et l'onde ?

## Ce qui est fait

- **R1, `set_density_shift_resample`** : chaque particule déplacée par la projection reprend la vitesse et la matrice affine de la grille à
  sa nouvelle place (`grid_affine`, la règle du G2P d'APIC).
- **R2, `set_density_surface_relaxation`** : aux mailles de surface, la correction est faite à κ par pas.
- Les deux sont éteints par défaut, au bit. Les montages du repos, du canal et de S645 prennent ces réglages.

## Reproduire

- `python outils/essai.py runup_with_two_remedies_s750 --ignore` (13 min).
- `python outils/essai.py rest_and_channel_with_two_remedies_s750 --ignore` (23 min).

## Mesuré

| essai | critère | `Complete` consciente (S744) | **R1, le déplacement avec sa vitesse** | R2, la surface relâchée (κ = 0,1) |
|---|---|---|---|---|
| le repos, l'escalier 1:30 et 1:12 | 1 cm/s ; 3 mm | tenu | **tenu** (6,8 mm/s ; 0,02–0,07 mm) | tenu |
| la remontée de S645 | 10 % de 0,2295 m | −11,0 % | **+7,9 %, tenu** (0,2475 m) | −9,8 %, tenu (0,2071 m) |
| l'onde solitaire, la largeur | 80 % | 92 % | **92 %, tenu** | 84 %, tenu |
| l'onde solitaire, le creux | 10 mm (10 % de `H`) | 9,0 mm | **10,8 mm, manqué** | 13,4 mm, manqué |
| la crête finale de l'onde (100 mm au départ) | rapportée | 100,2 mm | 91,3 mm | 112,6 mm |
| le niveau derrière l'onde | rapporté | +2,73 → −0,48 mm | +2,57 → −0,41 mm | +2,51 → −1,94 mm |

## Ce que cela dit

- **Le mécanisme du freinage est confirmé** : la projection déplaçait les particules sans leur vitesse. Les mettre à jour (R1) fait passer
  la remontée de −11 % à +7,9 %.
- **R1 est le premier remède qui tient presque tout** : le repos, la remontée, la largeur de l'onde. Le creux derrière l'onde dépasse son
  seuil de 0,8 mm, et l'onde s'atténue de 9 % sur 10 m : la reprise des vitesses depuis la grille ajoute un peu de dissipation.
- R2 tient la remontée de justesse, mais garde moins bien l'onde.

## La suite

- S751 : la revue de méthode.
- Puis R1 affiné :
  - les 0,8 mm du creux, au regard du plancher de bruit de la lecture (deux lectures, ADR-286 D2) ;
  - l'atténuation de l'onde, et d'où elle vient ;
  - R1 avec la relaxation de surface, ou R1 réservé aux mailles proches de la surface.
