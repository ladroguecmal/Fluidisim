# APIC à quatre bords — S724 (liste 4.14 ; LOD, étape 3, B2)

*S724, 2026-10-09, en autonomie ; l'utilisateur dort.* LOD-ETAPE-3-S722, B2. La boîte de 3D autour d'un corps doit être ouverte sur ses
quatre côtés. En x, les bords à particules existent (S682, S698). Ici, ceux en y.

## Ce qui est fait

**`apic3d_bords_y.rs`**, les bords du devant (`y = 0`) et du derrière (`y = ly`) par particules :
- `set_y_boundaries`, la vitesse normale de chaque face ;
- `drain_y`, les particules qui sortent, retirées et comptées ;
- `feed_y_grid`, l'entrée posée par quanta, avec la vitesse et l'affine du G2P (la pose par la grille, S702).

Ils sont branchés dans `walls`, dans l'advection (le devant et le derrière ne retiennent plus) et dans le pas (le retrait en y après celui
en x). Les gouttes sont refusées avec eux. Sans eux, le banc est au bit.

## Reproduire

- E1 : `python outils/essai.py the_four_sided_box_at_rest_s724 --ignore` (≈ 1 min) ;
- E2 : `python outils/essai.py the_four_sided_box_with_a_slanted_current_s724 --ignore` (≈ 6 min) ;
- E2b : `python outils/essai.py the_four_sided_box_with_a_straight_current_s724 --ignore` (≈ 6 min).

## Mesuré

Une boîte de 1 m × 1 m, 0,4 m d'eau, `dx` = 2,5 cm (≈ 205 000 particules), ouverte sur ses quatre côtés.

| | `V_φ` | la vitesse moyenne de l'intérieur | l'écart maximal | l'étendue de η | la plus rapide |
|---|---|---|---|---|---|
| **E1, le repos, 1 s** | −1,3·10⁻⁷ | — | — | 0,9 µm | 4,9·10⁻⁶ m/s |
| **E2, le courant (0,2 ; 0,1) m/s, 5 s** | **−0,39 %** | **(0,1991 ; 0,0996)** | 0,100 m/s | **7,2 mm** | 1,80 m/s |
| E2b, le témoin, le courant (0,2 ; 0) m/s | −0,25 % | (0,1992 ; 0,0000) | 0,103 m/s | 3,7 mm | 0,82 m/s |

**Critères** :
- E1 : **tenu** ;
- E2 : le volume (0,5 %) et la vitesse moyenne (2 %) **tenus** ; l'écart maximal (5 cm/s) et la surface (3 mm) **échoués**.

**Le diagnostic** : les écarts sont à la surface. La particule la plus éloignée du courant est à z = 0,400 m, la surface, à demi-vitesse ;
la plus rapide tombe en vol libre à la surface ; un creux de 5,4 mm se forme à l'intérieur.

## Ce que cela dit

- **La boîte à quatre bords tient la masse et l'écoulement** : au repos au micron, un courant en biais à 0,5 % de sa vitesse, le volume à
  0,4 %.
- **L'écart de vitesse maximal n'est pas celui des bords en y.** Le témoin en x seul a le même (0,103 m/s) : c'est la surface libre d'APIC
  dans un écoulement ouvert, quelques particules de surface ralenties ou détachées.
- **L'entrée par le devant ajoute ≈ 3,5 mm d'irrégularité de surface** (7,2 contre 3,7 mm). La piste : la pose se fait jusqu'au niveau
  nominal, alors que la vraie surface est un peu plus bas (le volume a baissé de 0,4 %). Des particules posées au-dessus de la surface se
  détachent.
- Ramené à l'usage : quelques millimètres de rides sur 0,4 m d'eau, sous le visible à l'échelle d'un joueur. On les juge à nouveau dans B3,
  où le niveau viendra de Saint-Venant.

## La suite

**B3** : le raccord de la boîte, Saint-Venant troué (B1) avec APIC à quatre bords (B2). Au repos, puis une onde longue qui traverse la
boîte, contre Saint-Venant seul.
