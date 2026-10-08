# La projection de densité — S709 (liste 4.14, 1.6)

*S709, 2026-10-08, en autonomie ; session longue.* S708 : à compte exact, APIC perd ≈ 1,3 % par seconde de volume géométrique en
mouvement. Le remède essayé : la projection de densité (Kugelstadt et al. 2019), en option.

## Ce qui est fait

- **`apic3d_densite.rs`**. À la fin de chaque pas, après la séparation :
  - la densité aux centres des mailles, par les poids trilinéaires ;
  - un Poisson sur son écart, avec la surface à `q = 0` ;
  - chaque particule déplacée de `∇q`, borné à un quart de maille ; les vitesses ne changent pas.
- **`pcg`**, le gradient conjugué, sorti de `project` par un simple déplacement de code (le banc au bit).
- **Les variantes** (`DensityVariant`) : `Complete`, `WithoutSurface`, `SolidExcessOnly`.
- **Le montage** : `Large::AucunDensite(variante)`, un mode nommé.

## Reproduire

- E1 : `python outils/essai.py the_density_projection_at_rest_s709 --ignore` (≈ 2 min) ;
- E2 : `python outils/essai.py the_flat_wave_with_density_projection_s709 --ignore` (≈ 3 min) ;
- E3 : `python outils/essai.py the_full_3d_with_density_projection_s709 --ignore` (≈ 18 min) ;
- E3a : `python outils/essai.py the_full_3d_with_density_projection_without_surface_s709 --ignore` (≈ 18 min).

## Mesuré

**E0 — le défaut, au bit : tenu** (le banc de non-régression, les empreintes inchangées).

**E1 — le repos : tenu.** Avec la projection, 8,5·10⁻⁶ m/s (sans : 7,0·10⁻⁶) ; `V_φ/V_n` 0,99967, constant ; un déplacement maximal de
0,5 µm.

**E2 — l'onde plate (10 m, 1,6 s).**

| | `V_φ/V_φ(0)` à 1,6 s | la crête par la surface | par le compte |
|---|---|---|---|
| sans projection (S708) | 0,9813 | 0,137 m | 0,178 m |
| la projection d'un seul côté (l'excès), premier passage | **1,0736** | 0,185 m | 0,119 m |
| **la projection dans les deux sens à l'intérieur** | **0,9990** | 0,140 m | 0,142 m |

D'un seul côté, la correction d'une densité bruitée dilatait l'eau à chaque pas. Dans les deux sens, **tenu** (−0,1 %). Le coût est de
196 s contre 131 s.

**E3 — le tout-3D de S690 : échoue.**

| | `V_φ/V_n` à 0,25 s | à 2,5 s | à 4 s | retournement | air | coût |
|---|---|---|---|---|---|---|
| sans projection (S708) | 0,9966 | 0,9667 | 0,9409 | 2,637 s, 9,988 m | 2,790 s | 777 s |
| `Complete` | 0,9924 | 0,9916 | 0,9905 | **aucun** | **aucun** | 1 064 s |
| `WithoutSurface` (E3a) | 0,9914 | 0,9880 | 0,9850 | **2,932 s, 10,763 m** | 2,932 s | 1 081 s |

- **Le volume est tenu** : après un saut de 0,6 % au premier quart de seconde, 0,991 jusqu'à 4 s, contre 0,941 sans projection.
- **Mais la vague ne plonge plus.** La classification de Grilli (S647) l'exige, avec S₀ ≈ 0,23. La correction aux mailles de surface
  comble la cavité sous la lèvre (E3a). Sans elle, le plongeon revient, mais 0,30 s et 0,77 m plus loin.
- Le saut du départ vient sans doute des mailles voisines de l'escalier, où les poids se perdent dans le solide (E3b, non lancé).

## Ce que cela dit

- **Corriger 100 % de l'écart de densité à chaque pas lisse la dynamique rapide** : le bruit de la densité, et la raideur du front qui fait
  plonger. Or la perte à combattre est lente, ≈ 0,013 % par pas de 10 ms.
- **La projection reste éteinte par défaut.** Telle quelle, elle casse le critère qualitatif validé en S647 : la vague doit plonger.
- **La suite (S710) : une projection faible.** On ne corrige qu'une petite fraction de l'écart par pas, assez pour tenir une dérive de
  1,3 %/s sans lisser le front. Elle sera jugée sur les mêmes essais :
  - `V_φ` tenu à 0,5 % sur le tout-3D ;
  - le plongeon gardé, à la tolérance d'ADR-278 D2 (0,1 s, 0,15 m) du juge sans projection.
