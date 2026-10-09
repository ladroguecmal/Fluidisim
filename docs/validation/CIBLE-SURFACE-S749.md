# La surface corrigée vers sa densité attendue — S749 (liste 4.1 ; ADR-289 D3.1)

*S749, 2026-10-09.* SURFACE-DILATEE-S748 : la projection dilatait la couche de surface, en n'y corrigeant que l'excès. **La question** :
corrigée dans les deux sens vers sa densité attendue, la surface reste-t-elle à sa place, et la 3D tient-elle les trois essais ?

## Ce qui est fait

- `DensityVariant::SurfaceTarget` : l'intérieur vers 1, la surface dans les deux sens vers `densite_attendue_s749(−φ/dx)`. C'est le noyau
  (le chapeau trilinéaire) appliqué à une eau uniforme sous une surface plane : `0,5 + a − a²/2`, qui vaut 0,875 pour la surface sur la face
  haute de la maille.
- Le canal relève le niveau moyen loin derrière l'onde (0,5 à 2,5 m), par les particules.

## Reproduire

- `python outils/essai.py runup_with_surface_target_s749 --ignore` (6 min).
- `python outils/essai.py rest_channel_and_level_with_surface_target_s749 --ignore` (20 min).

## Mesuré

| essai | critère | `Complete` consciente | **`SurfaceTarget` consciente** |
|---|---|---|---|
| (1) le repos, l'escalier 1:30 et 1:12 | 1 cm/s ; 3 mm | tenu (S744 : 6,8 mm/s ; 0,02 mm) | **non tenu** : 9,8 mm/s ; **4,38 mm** |
| (2) l'onde solitaire, canal à 2,5 cm | largeur 80 % ; creux 10 % de `H` | tenu : 92 % ; 9,0 mm | **tenu** : 96 % ; 8,3 mm ; la crête finale 101,4 mm |
| (3) la remontée de S645 | 10 % | −11,0 % | **−12,3 %, non tenu** |
| (4) le niveau derrière l'onde, de 0,25 à 4,25 s | 1 mm (rapporté) | +2,73 → −0,48 mm | +6,66 → +4,40 mm |

## Ce que cela dit

- **La densité attendue ne tient pas le repos.** La surface reconstruite ne tombe pas où la formule la suppose (`a` = ½ au repos), et la
  correction soulève le lac de 4,4 mm. La formule d'une surface plane devant le noyau n'est pas la lecture réelle.
- **La remontée reste freinée de 12 %, même sans dilatation de la surface.** La dilatation de S748 n'était donc qu'une partie de l'histoire.
- **Bilan des six variantes** (S740, S744, S745, S747, S749) :
  - aucune ne tient à la fois le repos, l'onde solitaire et la remontée ;
  - les variantes qui corrigent la surface freinent la lame ; celle qui ne la corrige pas laisse une traîne derrière l'onde.
- **Le mécanisme suspect suivant** : la projection déplace les particules sans mettre à jour leur vitesse. Dans un champ de vitesse
  cisaillé (sous la crête, dans le ressaut), chaque déplacement place une vitesse au mauvais endroit, et le transfert vers la grille l'y
  lisse : une dissipation de plus, à chaque pas.

## La suite (S750)

Deux remèdes candidats, éprouvés d'abord sur la remontée (le critère manqué, 6 min chacun), puis sur le repos et l'onde :
- **le déplacement avec sa vitesse** : chaque particule déplacée reprend la vitesse (et la matrice affine) de la grille à sa nouvelle place ;
- **la surface relâchée** : la correction de surface faite à κ = 0,1 par pas, l'intérieur à 1.
