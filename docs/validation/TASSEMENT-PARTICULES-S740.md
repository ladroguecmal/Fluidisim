# Pourquoi la 3D raidit l'onde solitaire : le tassement des particules — S740 (liste 4.1, 4.14)

*S740, 2026-10-09.* ONDE-SOLITAIRE-3D-S739 : sur un fond plat, la 3D raidit l'onde solitaire (la crête +50 %, la largeur ÷ 5, un creux
derrière). **La question** : quelle partie du pas en est la cause ? Les suspects un à un, sur le canal de S739 (A, 5 cm, 4,25 s). « Guérit »
veut dire : la largeur à mi-hauteur reste au-dessus de 80 % de celle de 0,25 s, et le creux sous 10 % de `H`.

## Ce qui est fait

- Deux réglages du cœur, à leurs défauts au bit : `Apic3::set_pressure_max_iterations` (4 000) et `set_separation_passes` (2).
- Le canal (`canal_regle_s740`) prend ces réglages, le plafond du pas et la projection de densité (S709, S710). Il relève à chaque pas les
  itérations et le résidu de la pression, et, sur demande (`PROFILS=1`), écrit le profil de la surface toutes les 0,5 s.
- Le résumé prend sa référence à 0,25 s.

## Reproduire

- `python outils/essai.py channel_pressure_as_is_s740 --ignore` (E1) ; `channel_converged_pressure_s740` (E2) ;
  `channel_without_separation_s740` (E3) ; `channel_short_step_s740` (E4) ; `channel_density_projection_s740` (E5) ;
  `channel_weak_density_projection_s740` (E6) ; `channel_density_projection_fine_s740` (E7, 2,5 cm, 9 min). Chacun avec `--ignore`.
- Les profils : `PROFILS=1` devant la commande ; `captures/s740_profils.png` (E1) et `captures/s740_profils_densite.png` (E7).

## Mesuré

| essai | maille | la largeur au plus bas | le creux | la crête finale | la pression | verdict |
|---|---|---|---|---|---|---|
| E1, tel quel | 5 cm | 20 % | 54,3 mm | 99,5 mm | 138 itérations au plus, jamais au plafond, résidu 10⁻⁶ | ne guérit pas |
| E2, le plafond à 100 000 | 5 cm | identique au bit | | | | ne guérit pas |
| E3, sans séparation | 5 cm | 22 % | 46,1 mm | 82,0 mm | | ne guérit pas |
| E4, le pas à 2,5 ms | 5 cm | 23 % | 25,0 mm | 62,2 mm | | ne guérit pas |
| **E5, la projection de densité** | 5 cm | **84 %** | 15,3 mm | 112,2 mm | | presque (le creux) |
| E6, la projection faible (κ = 0,05) | 5 cm | 39 % | 34,1 mm | 207,1 mm | | ne guérit pas |
| **E7, la projection de densité** | **2,5 cm** | **92 %** | **9,0 mm** | **98,8 mm** | 266 itérations au plus | **guérit** |

**Le profil d'E1** : l'onde se scinde. Un long plateau bas, d'environ 20 mm, file devant ; un pic serré, plus lent, reste derrière.
**Celui d'E7** : une onde solitaire propre, qui avance sans se déformer sur 10 m.

E5, E6 et E7 ont été ajoutés au plan après E4, quand le profil a montré l'onde qui se scinde ; leurs critères étaient les mêmes.

## Ce que cela dit

- **La cause est trouvée : le tassement des particules sous la crête.** Les particules se concentrent là où l'eau converge, et la surface
  reconstruite se soulève avec elles. Ni la pression (elle converge), ni la séparation, ni le pas n'en sont la cause. La projection de densité
  (S709), qui ramène chaque maille à sa densité, garde l'onde : à 2,5 cm, la largeur à 92 %, le creux à 9 % de `H`, la crête au millimètre.
- **Ce que cela change** :
  - le juge du déferlement de S647 à S730 (sans projection) partait d'une onde artificiellement raidie. Son plongeon (2,637 s, 9,988 m)
    est donc en partie un artefact ;
  - S709 avait écarté la projection complète parce que « la vague ne plongeait plus », en prenant ce juge pour la vérité. Avec la
    projection sans la surface, la vague plongeait plus tard (2,932 s, 10,763 m), comme le prévoit SGN (S733) ;
  - **qui a raison se juge contre une référence extérieure** (ADR-280 D2) : les mesures de Synolakis (S712), ou les formules de Grilli et
    al. (1997), dont le texte n'a pas pu être lu ici.

## La suite

1. **S741 : la revue de méthode** (due).
2. Puis la 3D avec la projection de densité, jugée contre les mesures de Synolakis (S712 : la crête à t = 15, 20, 25) et sur la vague de
   R43. Plonge-t-elle, où, et quand ? Les variantes de S709 (`Complete`, `WithoutSurface`) départagées par la mesure.
