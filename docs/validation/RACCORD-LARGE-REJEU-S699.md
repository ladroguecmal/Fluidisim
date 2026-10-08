# Le raccord du large entre deux 3D : le rejeu — S699 (liste 4.14)

*S699, 2026-10-08, en autonomie.* S698 : le raccord du large par particules, nourri par SGN, laisse −0,047 s au retournement. Trois
causes y restent mêlées : le porteur, le profil, la pose. Ici, le raccord est jugé entre deux copies du même solveur (ADR-273 D1) : on
le nourrit **exactement** de ce que la 3D fait passer au même endroit.

## Ce qui est fait

- **L'enregistrement.** Le tout-3D (le montage sans raccord de S697, par la même fonction) enregistre au plan x = 5,0 m, à chaque pas :
  - la vitesse normale de chaque face (4 rangées × 40 couches) ;
  - chaque particule qui franchit le plan vers la droite : l'instant, la position, la vitesse, la matrice affine.

  Une paire avant/après qui saute de plus de 0,1 m est écartée : c'est un échange d'indices quand le rivage retire une particule.
- **Le rejeu** (`pose_left`). Le montage raccordé à 5,0 m impose au bord les vitesses des faces, interpolées dans le temps. Il pose les
  particules enregistrées telles quelles, au milieu du pas qui suit leur passage. Ni SGN, ni profil.

## Reproduire

- `python outils/essai.py the_offshore_relay_between_two_3d_copies_s699 --ignore` (≈ 20 min : 13 min pour l'enregistrement, 7 min pour le
  rejeu).

## Mesuré

| montage | premier retournement | air enfermé | masse |
|---|---|---|---|
| tout-3D, enregistré (le témoin) | 2,637 s, 9,988 m | 2,790 s, 10,325 m | 1,2·10⁻¹⁶ |
| **rejeu au raccord à 5,0 m** | **2,626 s, 9,963 m** (−0,011 s) | 2,776 s, 10,296 m | 2,6·10⁻¹⁴ |
| S698 : SGN au raccord à 5,0 m | 2,590 s, 9,888 m (−0,047 s) | 2,792 s, 10,346 m | 2,5·10⁻¹⁵ |

752 pas enregistrés, 15 824 particules passées. Cela fait 95 % du volume de l'onde, `4·√(a·d³/3)` par mètre de large : à 2 particules
par axe, 16 200. Le reste de l'onde était déjà à droite du plan au départ. La dette sous un quantum.

**Critère 1 (0,02 s, 0,15 m) : tenu**, −0,011 s et −0,025 m. **Critère 2 : tenu.**

## Ce que cela dit

- **Nourri exactement, le bord par particules est presque transparent** : −0,011 s, un peu moins que le bord non traversé de S698
  (−0,017 s).
- **Des −0,047 s de S698, environ −0,036 s viennent donc de l'alimentation par SGN.** Trois causes restent : le porteur (ū et h au
  raccord), la forme du profil (`h²/6 − z²/2` sur fond plat), la pose (la matrice affine nulle). La suite (S700) les départage. Chacune se
  compare à l'enregistrement, qui donne au même plan la vérité de la 3D.

## Un piège rencontré

Le premier rejeu passait `particules = false`, et le montage allumait alors, en plus du bord à particules, la zone de colonnes de S693.
La 3D perdait des particules dès 0,5 s, et Saint-Venant a refusé un pas à 3 s : 34 min perdues. Corrigé : la zone de colonnes ne
s'allume plus avec le rejeu. Une combinaison de drapeaux qui rallume en silence un mécanisme ancien est une seconde source cachée
(ADR-276 D1) : c'est à relire à la revue de S701.
