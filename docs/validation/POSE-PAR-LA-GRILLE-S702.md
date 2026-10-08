# La pose par la grille, jugée contre l'enregistrement de la 3D — S702 (liste 4.14)

*S702, 2026-10-08, en autonomie.* S700 : la pose par faces de S698, même nourrie des vitesses de la 3D, retarde le retournement de
0,085 s. Ici, une pose qui ne dépend pas du porteur, jugée d'abord contre la 3D elle-même (ADR-273 D1).

## Ce qui est fait

- **`feed_left_grid`, la pose par la grille.**
  - Le porteur ne donne que deux choses : la vitesse normale de chaque face du bord, et le volume qui la franchit.
  - Chaque quantum naît dans la tranche que le flux a balayée pendant le pas, `x ∈ [0, u·dt)`, à la sous-maille (y, z) la moins occupée de
    la face, décalée par la suite à faible discrépance R₂.
  - Sa vitesse et sa matrice affine sont celles que la grille lui donne là (le G2P d'APIC). `w` et le gradient viennent de la 3D, non
    d'un profil.
  - Une seule passe sur les particules compte l'occupation.
- **Le montage en mode nommé** (ADR-277 D2) : `Large::{Aucun, Colonnes, ProfilSgn, Rejeu(Exact | SansAffine | ParFaces | ParGrille)}`.
  Les combinaisons sans sens sont refusées par des assertions. Le passage qui enregistre redonne 2,637 349 s **au bit** de S699 : le
  changement de forme n'a rien changé.
- **R4** rejoue l'enregistrement du tout-3D par cette pose, à partir de deux données au plan : la vitesse des faces, et `h` de la 3D. Ce
  sont celles que SGN donnerait.

## Reproduire

- `python outils/essai.py the_grid_pose_judged_against_the_3d_record_s702 --ignore` (≈ 20 min : 13 min pour l'enregistrement, 7 min pour
  le rejeu).

## Mesuré

| montage au raccord de 5,0 m | retournement | écart au tout-3D | air enfermé |
|---|---|---|---|
| le tout-3D (le témoin) | 2,637 s, 9,988 m | — | 2,790 s, 10,325 m |
| rejeu exact (S699) | 2,626 s | −0,011 s | 2,776 s |
| R3, pose par faces (S700) | 2,711 s | +0,074 s | 2,849 s |
| **R4, pose par la grille** | **2,658 s, 10,038 m** | **+0,021 s** | 2,815 s, 10,408 m |

La masse : 1,2·10⁻¹⁵. La dette sous un quantum.

**Critère 1 (0,02 s, 0,15 m) : échoue d'une milliseconde** (+0,021 s ; +0,05 m). **Critère 2 : tenu** (le témoin au bit, la masse, la dette).

## Ce que cela dit

- **La pose par la grille ôte les trois quarts du défaut de la pose par faces** : +0,074 s → +0,021 s, à partir des mêmes données (la
  vitesse des faces, `h`). Elle est la meilleure des poses nourries par une vitesse ; elle ne passe pas le critère.
- **Le reste (+0,021 s, un retard)** : une cause encore à nommer. Les candidats, chacun à juger seul :
  - la couche de surface, comptée au prorata d'un `h` lu au quantum de 6 mm. Un `h` trop bas fait entrer moins d'eau, et une onde plus
    petite déferle plus tard ;
  - la grille du pas précédent, qui donne aux particules posées une vitesse en retard d'un pas.
- **La suite (S703)** :
  - nourrir la pose par la grille par SGN (ADR-273 D1 : la pose est maintenant jugée contre la 3D) ;
  - mesurer en même temps, au plan, le volume entré par R4 contre celui des particules enregistrées. Il dira si la couche de surface
    est en cause.
