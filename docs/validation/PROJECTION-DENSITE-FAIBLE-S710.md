# La projection de densité faible — S710 (liste 4.14, 1.6)

*S710, 2026-10-08, en autonomie ; session longue.* S709 : la projection de densité, corrigeant 100 % de l'écart à chaque pas, tient le
volume mais empêche le plongeon. Ici, une fraction κ seulement de l'écart est corrigée à chaque pas (`set_density_relaxation`).

## Le calcul de κ

La perte à combattre est de 0,013 % par pas de 10 ms. À l'équilibre, `κ·(ρ − 1)` la compense, d'où un biais de densité de
`0,013 % / κ`. Pour κ = 0,05, cela fait 0,26 %.

## Reproduire

- E1 : `python outils/essai.py the_flat_wave_with_weak_density_projection_s710 --ignore` (≈ 3,5 min) ;
- E2 : `python outils/essai.py the_full_3d_with_weak_density_projection_s710 --ignore` (≈ 19 min).

## Mesuré

**E1 — l'onde plate, κ = 0,05 : tenu.** `V_φ/V_φ(0)` vaut 0,9975 à 0,4 s, 0,9954 à 1,0 s et 0,9955 à 1,6 s. C'est un équilibre à −0,45 %,
comme le calcul l'attendait (sans projection : −1,87 %).

**E2 — le tout-3D de S690, κ = 0,05 : échoue.**

| | `V_φ/V_n` à 0,25 s | à 2,5 s | retournement | air | coût |
|---|---|---|---|---|---|
| sans projection (S708) | 0,9966 | 0,9667 | 2,637 s, 9,988 m | 2,790 s | 777 s |
| κ = 1, `Complete` (S709) | 0,9924 | 0,9916 | aucun | aucun | 1 064 s |
| κ = 1, `WithoutSurface` (S709) | 0,9914 | 0,9880 | 2,932 s, 10,763 m | 2,932 s | 1 081 s |
| **κ = 0,05, `Complete`** | 0,9937 | **0,9899** | **2,861 s, 10,638 m** | 2,907 s | 1 136 s |

Le volume par rapport au départ est à −0,9 % au déferlement : le saut du premier quart de seconde (−0,5 %), puis l'équilibre. Le
plongeon est en retard de 0,22 s et 0,65 m.

## Ce que cela dit

- **Le lissage n'est pas la cause principale du retard.** Une correction vingt fois plus faible le retarde presque autant : 0,22 s contre
  0,30 s.
- **C'est le volume gardé qui déplace le déferlement.** Sans projection, la 3D perd ≈ 3 % de son eau effective avant de déferler. Son
  niveau moyen baisse, la vague voit une eau moins profonde sur la pente, et elle déferle plus tôt et plus au large.
- **Le point de déferlement du tout-3D a donc une incertitude du même ordre (0,2 à 0,3 s)**, selon la façon dont il tient son volume.
  C'est plus que l'écart des raccords (S703 : 0,07 s). Les raccords restent jugés contre le même juge, sans projection, ce qui garde la
  comparaison cohérente (ADR-278).
- **Qui a raison ne se tranche qu'avec une référence extérieure** : des mesures de laboratoire d'une onde solitaire qui déferle sur une
  pente, le point et la profondeur de déferlement. S647 n'avait que la classification qualitative de Grilli.
- **La projection reste éteinte par défaut.** Le saut du départ, près de l'escalier, reste à traiter (S709 E3b).

## La suite

- **S711** : la quarante-sixième revue (S706–S710).
- **Puis la référence extérieure**, cherchée et lue : une onde solitaire sur une pente proche de 1:12, le point de déferlement mesuré. Le
  juge se compare à elle, avec et sans la projection faible.
