# Le banc canonique de la 3D, première passe — S742 (liste 4.1 ; ADR-287 D1)

*S742, 2026-10-09.* ADR-287 D1 : un solveur ne sert de référence qu'après ses essais canoniques. **La question** : la 3D avec la
projection de densité (S740) passe-t-elle les essais de base, sans perdre ce qui marchait ?

## Ce qui est fait

- `levee_s742` (B2) : une bosse gaussienne sur 0,30 m d'eau, une pente de 1:30 jusqu'à 0,12 m, un plateau de 7 m. La 3D est sur fond lisse,
  avec et sans projection ; Saint-Venant et SGN servent de témoins sur la même bosse. Les profils s'écrivent avec `PROFILS=1`.
- `onde_sur_pente_densite_s742` (B4) : le montage de S645, avec la projection en option (sans elle, au bit).

## Reproduire

- `python outils/essai.py the_canonical_shoaling_bench_s742 --ignore` (B2, 18 min) ; `PROFILS=1` devant
  `python outils/essai.py the_shoaling_profiles_s742 --ignore`, pour les profils (`captures/s742_levee_profils.png`).
- `python outils/essai.py the_canonical_runup_bench_with_density_s742 --ignore` (B4, 6 min).

## Mesuré

| essai | mesuré | critère | verdict |
|---|---|---|---|
| **B1** — l'onde solitaire sur un canal plat, avec projection (S740, E7) | la largeur à 92 %, le creux à 9 % de `H` | 80 % ; 10 % | **tenu** |
| **B2** — la levée | Saint-Venant 1,152, SGN 1,271 (Green 1,257) ; la 3D avec projection 3,33, sans projection aucune crête lue | 10 % de Saint-Venant | **sans conclusion** |
| **B4** — la remontée de S645 avec la projection | **0,2020 m** (−12 % ; sans projection, 0,2307 m, +0,5 %) | 10 % de la loi | **manqué** |

**Pourquoi B2 ne conclut pas** (le profil, regardé avant toute autre hypothèse, ADR-287 D4) :
- **la bosse est sous le quantum de pose.** Les particules sont posées par couches de 12,5 mm (deux par maille et par direction) ; une
  bosse de 15 mm n'en fait qu'une couche, posée en plateau carré. Le plan n'avait pas écrit le quantum à côté de son amplitude (ADR-236) ;
- **sur le fond lisse en pente, la lecture porte des dents de scie de ±10 mm dès le départ.** Leur période, ≈ 0,75 m, est celle du fond qui
  monte d'une maille toutes les trente colonnes ;
- **puis tout le bassin oscille de ±20 mm**, bien au-delà de la bosse. La 3D sur un fond lisse en pente douce ne tient pas le repos, ou sa
  lecture ne le tient pas. S678 l'avait vu (« le repos pas tenu sur toutes les plages »).

**B4** : la projection complète, qui garde l'onde solitaire au large, freine la lame sur la plage de 12 %. S709 l'avait vue lisser le
plongeon. Elle n'est donc pas gratuite.

## Ce que cela dit

- **La projection de densité garde l'onde au large, mais freine le jet de rive.** Le réglage à trouver doit tenir les deux. Les variantes
  de S709 (`WithoutSurface`, `SolidExcessOnly`) ne projettent pas près de la surface : ce sont les premières candidates.
- **Le banc a d'abord besoin de son essai le plus élémentaire : le lac au repos sur une pente**, en escalier et lisse, avec et sans
  projection. Sans lui, ni la levée ni la remontée ne se lisent.

## La suite (S743)

1. Le lac au repos sur une pente (1:30 et 1:12 ; l'escalier et le fond lisse ; avec et sans projection) : la vitesse et la surface, lues
   par φ et par les particules.
2. Les variantes de la projection sur le canal (B1) et sur la remontée (B4) : laquelle tient les deux ?
3. B2 refait, l'amplitude à au moins trois quanta (≥ 40 mm), sur le fond qui tient le repos.
