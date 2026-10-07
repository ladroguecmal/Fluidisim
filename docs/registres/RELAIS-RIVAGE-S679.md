# Le relais au rivage : le film confié à Saint-Venant 2D — conception (S679, liste 4.14)

*S679, 2026-10-07.* [ADR-271](../adr/ADR-271-le-film-du-rivage-a-saint-venant.md) décide ; ce document conçoit la campagne.

## 1. Pourquoi

Au rivage, APIC 3D ne tient pas l'eau au repos.

- **S640** : 0,26 m/s.
- **S678** : deux causes, la surface du film et les faces à peine ouvertes. Leur remède tient sur quatre plages sur six, pas sur toutes.

La ligne de contact d'un film sur un fond coupé, dans une méthode à particules, reste un problème ouvert. Saint-Venant 2D le résout par
construction :

- le lac au repos à bords secs reste immobile (S613) ;
- le rivage mobile suit Thacker à l'ordre deux (S620) ;
- la remontée d'une houle est à 0,05 % de Keller et Keller (S625) ;
- les ressauts mobiles sont captés (S627).

Le ressaut qui suit le déferlement et le jet de rive sont justement ce que Saint-Venant porte bien. Le retournement et le rouleau, non :
ils restent à la 3D.

## 2. Les rôles

| bande | solveur | ce qu'il porte |
|---|---|---|
| le large | B, puis Saint-Venant 2D (S650) | la houle qui approche |
| la bande de déferlement | APIC 3D | le retournement, le rouleau, l'air enfermé (S647–S648), les corps (S652–S657) |
| le film, du ressaut au jet de rive | **Saint-Venant 2D** | le ressaut, la remontée, le reflux, le mouillage et le séchage |

Le raccord du rivage se place **au-delà du point de plongée**, là où le ressaut est formé, et jamais à moins de **trois mailles** de
profondeur au repos. Au-delà de trois mailles, le noyau lit juste : la pente immergée de S640 tient à 8·10⁻⁶ m/s.

L'onde de S647 aurait `H/h` = 0,7 à trois mailles de profondeur (5 cm) : c'est le déferlement lui-même. Le raccord y sera donc plus
profond que la règle des trois mailles, ou à la maille de 2,5 cm.

## 3. L'interface

- **Un plan vertical fixe** `x = x_f`, à travers la largeur. APIC 3D a son bord droit ouvert (`set_open_boundaries`). Saint-Venant 2D a
  son bord gauche donné (`pas_avec_bords`).
- **Une zone de colonnes** borde le raccord du côté 3D, comme au large (S650). Elle compte la masse qui sort et pose des particules
  pour celle qui entre. **Obstacle** : la zone des colonnes refuse le fond lisse (S640). Deux voies :
  - le fond en escalier sous la zone, comme S650 ;
  - les colonnes rendues compatibles avec le fond lisse (la profondeur `η − z_b` par colonne).

  À trancher à l'étape 1, par la mesure.
- **Un seul flux d'interface** par pas et par rangée `j`. C'est un problème de Riemann (HLL) entre l'état de la dernière colonne 3D
  (`h`, `u` moyen) et celui de la première maille de Saint-Venant. Le même flux sert aux deux côtés : la masse est échangée au bit.
- **Au repos**, les deux niveaux sont égaux et le flux est nul : rien ne bouge. C'est le premier essai.
- **Le pas de temps** est commun, le plus petit des deux (la CFL d'APIC, celle de Saint-Venant).

## 4. Les étapes et leurs essais

Ce que chaque essai rendrait est écrit avant (ADR-267 D1).

| étape | ce qu'elle fait | l'essai | si juste | si faux |
|---|---|---|---|---|
| **1** | le raccord au repos | les six plages de S678, l'eau au repos, 2 s, à 5 et 2,5 cm | **≤ 1 cm/s partout**, la masse au bit | la vitesse de S678 (0,1 à 0,6 m/s) si le film reste à APIC ; un flux au raccord si les niveaux ne s'accordent pas |
| **2** | l'onde solitaire non déferlante de S644 (1:3) à travers le raccord | la remontée, contre Synolakis et le tout-Saint-Venant ; la masse | la remontée à 5 % du tout-Saint-Venant, la masse au bit | une remontée tronquée (le flux mal transmis), une onde réfléchie au raccord |
| **3** | le reflux : l'eau qui redescend repasse en 3D | la même onde, jusqu'au retour au large | la masse au bit ; aucune vitesse parasite au raccord au retour au repos | un ressaut piégé au raccord |
| **4** | la vague qui plonge (S647, 1:12) avec les deux raccords | le retournement, l'air enfermé, la remontée | le retournement et l'air comme le tout-3D (S647–S650) ; la remontée comme le tout-Saint-Venant | le rouleau coupé par le raccord |
| **5** | le coût | le relais complet contre le tout-3D | la 3D réduite à la bande de déferlement | — |

## 5. Ce qui reste hors de la campagne

- La bande 3D qui naît et meurt avec la vague (S637–S638), ensuite.
- La zone sèche du jeu (le sable mouillé, la trace de l'écume) : l'écume est suspendue.

## Note du 2026-10-08 (S680) — l'interface précisée

- **Le flux unique** n'est pas un HLL calculé à part. Saint-Venant 2D garde son bord caractéristique (S622, S628), nourri par l'état de
  la dernière colonne 3D, et rend le flux de masse qu'il a fait passer (`flux_des_bords`, la demi-somme des deux étages de Heun).
- **APIC** retire ou pose des particules pour ce volume, et un réservoir garde le reste d'un quantum. La masse se compte au bit :
  volume de Saint-Venant + particules × quantum + réservoir.
- **L'ordre des briques** :
  1. le flux des bords de Saint-Venant ([S680](../validation/RELAIS-RIVAGE-FLUX-S680.md)) ;
  2. le bord droit d'APIC qui retire et pose des particules ;
  3. le raccord au repos (l'étape 1 de la conception).
