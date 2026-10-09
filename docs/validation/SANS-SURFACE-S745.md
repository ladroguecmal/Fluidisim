# La projection consciente du fond, sans correction de surface — S745 (liste 4.1 ; ADR-287 D1)

*S745, 2026-10-09.* DENSITE-CONSCIENTE-S744 : la projection consciente du fond rendait le repos, mais freinait la remontée (−11 %). **La
question** : sans correction aux mailles de surface (`WithoutSurface`), la 3D tient-elle à la fois le repos, la remontée et l'onde
solitaire ? Une seule différence avec S744 : la variante.

## Reproduire

- `python outils/essai.py runup_without_surface_correction_s745 --ignore` (8 min).
- `python outils/essai.py rest_and_channel_without_surface_correction_s745 --ignore` (13 min).

## Mesuré

| essai | critère | sans projection | `Complete`, consciente (S744) | **`WithoutSurface`, consciente** |
|---|---|---|---|---|
| (1) le repos, l'escalier 1:30 | 1 cm/s ; 3 mm | 6,8 mm/s ; 0,01 mm | 6,8 mm/s ; 0,02 mm | **6,8 mm/s ; 0,02 mm — tenu** |
| (1) le repos, l'escalier 1:12 | 1 cm/s ; 3 mm | 6,8 mm/s ; 0,07 mm | 6,8 mm/s ; 0,07 mm | **6,8 mm/s ; 0,07 mm — tenu** |
| (2) la remontée de S645 | 10 % de 0,2295 m | 0,2307 m (+0,5 %) | 0,2042 m (−11,0 %) | **0,2169 m (−5,5 %) — tenu** |
| (3) l'onde solitaire, canal à 2,5 cm | largeur 80 % ; creux 10 % de `H` | 19 % ; 42 mm (S739) | 92 % ; 9 mm (S740, `Complete` sans le fond) | **87 % ; 40,5 mm — non tenu** |

## Ce que cela dit

- **Chaque variante tient ce que l'autre perd.** Corriger la surface (`Complete`) garde l'onde au large mais freine la lame sur la plage.
  Ne pas la corriger (`WithoutSurface`) rend la lame à 5,5 % de la loi, mais laisse derrière l'onde un creux de 40 mm.
- **La voie suivante est hybride** : corriger la surface en eau profonde, ne plus la corriger dans la lame mince (moins de trois mailles
  d'eau dans la colonne). C'est la règle de S678 pour le film du rivage. Le repos tient dans les trois cas : le fond compté suffit pour lui.

## La suite

- S746 : la revue de méthode.
- S747 : la variante hybride, jugée sur les trois essais.
