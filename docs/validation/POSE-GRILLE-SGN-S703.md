# La pose par la grille nourrie par SGN — S703 (liste 4.14)

*S703, 2026-10-08, en autonomie.* S702 : la pose par la grille, nourrie par la 3D, est à +0,021 s. Ici, la même pose est nourrie par SGN :
la vitesse du bord et le volume de chaque face viennent du profil vertical de SGN (les données de S698).

## Reproduire

- `python outils/essai.py the_grid_pose_fed_by_serre_s703 --ignore` (≈ 21 min : 13 min pour l'enregistrement du tout-3D, 7 min pour le
  montage).

## Mesuré

| montage au raccord de 5,0 m | les données | la pose | retournement | écart au tout-3D |
|---|---|---|---|---|
| le tout-3D (le témoin) | — | — | 2,637 s, 9,988 m | — |
| R4 (S702) | la 3D | par la grille | 2,658 s | +0,021 s |
| **`GrilleSgn`** | **SGN** | **par la grille** | **2,569 s, 9,863 m** | **−0,068 s** |
| S698 | SGN | par faces | 2,590 s | −0,047 s |

- L'air enfermé : 2,747 s, 10,294 m. La masse : 2,9·10⁻¹⁵. La dette sous un quantum.
- **Le volume au plan** pendant l'enregistrement :
  - les données de R4 (la vitesse des faces, la couche de surface au prorata de `h`) donnent 0,03047 m³ ;
  - les particules passées en font 0,03091 m³ ;
  - le rapport est de **0,986**.

**Critère 1 (0,02 s) : échoue**, −0,068 s. **Critère 2 : tenu.**

## Ce que cela dit

| écart | la seule cause qui change | l'effet |
|---|---|---|
| `GrilleSgn` − R4 | les données : SGN au lieu de la 3D, à pose égale | **−0,089 s** |
| `GrilleSgn` − S698 | la pose : par la grille au lieu de par faces, à données égales | −0,021 s |

- **Les données de SGN sont la cause principale** : à pose égale, elles avancent le retournement de 0,089 s. C'est le même sens et le même
  ordre que S700 (−0,121 s avec la pose par faces).
- **Le retard de R4 tient en partie au volume** : ses données font entrer 1,4 % d'eau de moins que la 3D. Une onde plus petite déferle
  plus tard.
- **Une question sur le juge lui-même.** Au plan de 5 m (S700) :
  - la crête de la 3D est à 0,139 m au-dessus du niveau, lue au quantum de 6 mm ;
  - celle de SGN est à 0,150 m, l'amplitude de l'onde de départ.

  Si le tout-3D à 2,5 cm amortit l'onde sur les 1,6 m qu'elle parcourt avant le plan, le témoin n'est pas la vérité. SGN, qui garde
  l'amplitude, ferait alors déferler plus tôt *et plus juste*. **S704** le mesure :
  - l'amplitude de la crête dans le tout-3D à plusieurs plans, sur le fond plat ;
  - la même pour SGN, contre l'onde solitaire.
