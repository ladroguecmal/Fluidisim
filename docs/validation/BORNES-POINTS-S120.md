# S120 — Ce qui est annonçable des points

2026-09-09. Suite de S119-1, sur A196.

## 1. Inventaire : les conditions que la requête évalue point par point

Établi en lisant les trois couches avant de décider quoi que ce soit. `sample_world_batch`
refuse un lot entier dès qu'un seul point échoue — l'atomicité d'ADR-077 — donc chacune de ces
conditions coûte le lot complet, pas seulement le point fautif.

| # | condition | où | nature |
|---|---|---|---|
| 1 | `\|Δ monde\| < 4096 m` sur x, y, z, puis `\|local\| < 4096 m` | `WorldPos::to_local`, `Background::eval_local` | géométrique, exacte |
| 2 | `\|point\| < 4096`, `\|point − position\| < 4096`, `r ≤ radius`, par champ | `RadialImpact::sample` | géométrique, exacte |
| 3 | `min ≤ point ≤ max`, rectangle fermé | `spectral_pressure::Field::sample` | géométrique, exacte |
| 4 | `steepness_B(point)·π + Σ slope_bound + slope_envelope ≤ max_slope` | `mixed` | une seule part dépend du point |
| 5 | `points.len() ≤ scratch.len()` et `≤ output.len()` | `mixed` | ne dépend pas des points, mais de leur nombre |
| 6 | finitude des sorties de chaque couche | partout | numérique, pas géométrique |

**Ce que l'inventaire change par rapport à ce qui était supposé.** Trois choses, dont deux
n'étaient pas anticipées en ouvrant la session :

- Les conditions 1 à 3 sont **exactes et peu coûteuses** — des comparaisons, pas des sommes
  sur 14 336 modes. Elles sont donc décidables par point, avant de payer l'évaluation. Ce
  n'est pas une « borne » : c'est le prédicat lui-même, transposé.
- La condition 4 a **un seul terme qui dépend du point**, et il est positif ou nul. La somme
  des autres est donc un **plancher** : si `max_slope` lui est inférieur, aucun point ne peut
  passer, quel que soit le lot. Cela s'annonce sans voir un seul point.
- `RadialImpact::sample` rend `Domain` **aussi** pour une sortie non finie (condition 6). Un
  prédicat géométrique ne peut donc pas promettre l'absence de `Domain` ; il ne peut promettre
  que l'absence de refus **géométrique**. C'est la limite exacte de ce qui est annonçable, et
  elle doit être dite plutôt que contournée.

La condition 5 ne demande aucune fonction : l'hôte connaît la taille de ses tampons et celle
de son lot. L'annoncer serait du code sans usage.

## 2. Décision

Voir [ADR-080](../adr/ADR-080-annonce-des-points-du-montage-mixte.md).

## 3. Construction et réception

*(à compléter)*
