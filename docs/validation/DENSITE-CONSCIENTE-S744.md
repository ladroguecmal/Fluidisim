# La projection de densité consciente du fond — S744 (liste 4.1 ; ADR-287 D1)

*S744, 2026-10-09.* REPOS-PENTE-S743 : la projection de densité mettait le lac en mouvement contre un fond. **La question** : une densité
rapportée à la nominale de chaque maille, qui compte le fond, rend-elle le repos et la remontée, sans perdre l'onde solitaire ?

## Ce qui est fait

- `Apic3::set_density_bed_aware` (éteint par défaut, au bit) : chaque maille a sa densité nominale, `Σ w/8` d'un réseau régulier de
  `2 × 2 × 2` points par maille posé partout hors du fond (l'escalier ou le fond lisse). Elle est calculée au premier pas, dans un tableau
  réservé à la configuration. La densité vaut `Σ w / nominale` (sous une nominale de 0,05, elle n'est pas corrigée).
- Les essais : le repos (S743), le canal (S740), la remontée de S645 (S742), avec la densité consciente.

## Reproduire

- `python outils/essai.py the_lake_at_rest_with_bed_aware_density_s744 --ignore` (4 min).
- `python outils/essai.py the_channel_is_unchanged_by_bed_awareness_s744 --ignore` (5 min).
- `python outils/essai.py the_runup_with_bed_aware_density_s744 --ignore` (6 min).

## Mesuré

| essai | sans projection | la projection d'avant | **la projection consciente du fond** | critère | verdict |
|---|---|---|---|---|---|
| (1) le repos, l'escalier, 1:30 | 6,8 mm/s ; 0,01 mm | 6,3 cm/s ; 4,63 mm | **6,8 mm/s ; 0,02 mm** | 1 cm/s ; 3 mm | **tenu** |
| (1) le repos, l'escalier, 1:12 | 6,8 mm/s ; 0,07 mm | 6,3 cm/s ; 4,71 mm | **6,8 mm/s ; 0,07 mm** | 1 cm/s ; 3 mm | **tenu** |
| le repos, le fond lisse, 1:30 | 0,68 m/s ; 0,82 mm | 1,96 m/s ; 52,8 mm | 0,80 m/s ; 7,15 mm | rapporté | non tenu (son défaut propre) |
| le repos, le fond lisse, 1:12 | 0,27 m/s ; 1,36 mm | 1,87 m/s ; 36,3 mm | 0,26 m/s ; 10,9 mm | rapporté | non tenu |
| (2) le canal, la crête finale | — | 112,24 mm | 114,02 mm | au bit | **manqué** : la prémisse était fausse |
| (3) la remontée de S645 | 0,2307 m (+0,5 %) | 0,2020 m (−12 %) | **0,2042 m (−11,0 %)** | 10 % | **manqué** |

**(2), la prémisse fausse** : aux murs du domaine, les poids des particules sont rabattus sur la maille du bord, et sa nominale dépasse 1
(≈ 1,125). La densité consciente corrige donc aussi ce biais des murs, que l'ancienne gardait. Ce n'est pas une régression ; le critère « au
bit » ne pouvait pas tenir.

## Ce que cela dit

- **La projection consciente du fond rend le repos sur l'escalier**, aussi bien que sans projection. Le déficit de densité au contact du fond
  était la cause du mouvement de S743.
- **Elle ne rend pas la remontée** (−11 %) : le fond n'était pas la cause du freinage de la lame. Le suspect suivant est le traitement de la
  surface. Dans une lame mince, toutes les mailles sont des mailles de surface, où la projection corrige l'excès et déplace les particules.
  La variante `WithoutSurface` (S709) n'y touche pas.
- Le fond lisse garde son défaut propre (des vitesses parasites), à traiter à part.

## La suite

1. S745 : la projection consciente **sans correction de surface** : le repos, la remontée de S645, le canal.
2. S746 : la revue de méthode.
