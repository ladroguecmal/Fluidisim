# ADR-291 — La 3D corrigée, version 1

- **Statut : actée**, S752, 2026-10-09. Un arbitrage technique tranché ici, par écrit (ADR-215 D2, ADR-222) ; ADR-289 D3.1.

## Contexte

La 3D (APIC) ne gardait pas une onde solitaire : ses particules se tassent sous la crête (S739, S740). Huit remèdes ont été éprouvés de S740
à S752, sur trois essais : le repos sur une pente, l'onde solitaire sur un canal plat, la remontée de S645 contre la loi de Synolakis. Aucun
ne les tient tous parfaitement.

## Décision

**D1 — La 3D corrigée, version 1, est** :
- la projection de densité `Complete` (S709) ;
- consciente du fond (`set_density_bed_aware`, S744) ;
- chaque particule déplacée reprenant la vitesse et la matrice affine de la grille à sa nouvelle place (`set_density_shift_resample`,
  S750, « R1 »).

**D2 — Ses écarts mesurés**, à 2,5 cm :

| essai | sans correction | **3D corrigée, version 1** |
|---|---|---|
| le repos sur une pente (l'escalier) | tenu | **tenu** (6,8 mm/s ; 0,02–0,07 mm) |
| l'onde solitaire, la largeur à mi-hauteur sur 10 m | 19 % | **92 %** |
| l'onde solitaire, la crête finale | +50 % | **−7,5 %** |
| l'onde solitaire, le creux derrière | 42 mm | **10,8 mm** |
| la remontée de S645 (la loi de Synolakis) | +0,5 % (onde posée près du pied) ; +47 % avec 3 m d'approche | **+7,9 %** |

Ces écarts sont du même ordre que ceux des solveurs 2D sur les mêmes cas (Saint-Venant : +4,5 % sur la remontée).

**D3 — Les huit tentatives, pour mémoire** (S740–S752) :

| variante | ce qui manque |
|---|---|
| `Complete` | freine la lame (−11 %) |
| la projection faible (κ = 0,05) | ne garde pas l'onde |
| `WithoutSurface` | laisse un creux de 40 mm |
| `Hybrid` | freine la lame |
| `SurfaceTarget` | perd le repos |
| R2, la surface relâchée | garde mal l'onde |
| R1′, `v + C·Δx` | injecte de l'énergie, la crête +15 % |
| **R1, retenue** | les écarts de D2 |

**D4 — La suite (ADR-289 D3.2, D3.3)** : la 3D corrigée est jugée contre des références extérieures, en premier les mesures de Synolakis (le
déferlement, H/d = 0,3). Ses écarts restants se liront contre une vraie mesure, plutôt que par un réglage de plus. Les témoins du
sélecteur, R43 et S1 sont refaits avec elle.

## Conséquences

- La 3D corrigée reste une option du cœur (éteinte par défaut, au bit pour les essais anciens). Les montages nouveaux l'activent.
- Le bilan propre de la projection (`density_projection_budget`, S752) le montre : toutes les variantes soulèvent un peu l'eau, de +2 à +3 J
  d'énergie potentielle en 4 s sur le canal. C'est un écart connu, à suivre.

*Note datée du 2026-10-09 (S753)* : **le choix de D1 était prématuré.** Contre les mesures de Synolakis, la 3D corrigée a la bonne hauteur
(+0,012 d à t = 15), mais elle est en retard, et le retard grandit (1,0 d, puis 2,2 d). R1 ralentit la vague de 5,6 % (2,290 m/s contre
2,426 ; `Complete` : +0,4 %). Le laboratoire tranchera entre `Complete` et R1 (S754)
([preuve](../validation/SYNOLAKIS-CORRIGEE-S753.md)).
