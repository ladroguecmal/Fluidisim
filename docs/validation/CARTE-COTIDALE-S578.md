# La carte cotidale — S578 (liste 2.2)

*S578, 2026-10-07, en autonomie.* La marée de [S577](MAREE-S577.md) vaut en un lieu ; amplitude et phase varient dans l'espace — la
marée se propage.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s578 -- --nocapture` ; suite du cœur : 753.

## 1. Ce qui est construit

`CarteCotidale` : pour chaque composante, l'amplitude complexe `H = A·e^(−ig)` aux nœuds d'une grille régulière, **interpolée sous forme
complexe** (bilinéaire sur `Re H` et `Im H`) : `η = Z₀ + Σ (Re Hₖ·cos ωₖt − Im Hₖ·sin ωₖt)` = `Z₀ + Σ Aₖ·cos(ωₖt − gₖ)`. Que des opérations
IEEE de base et les polynômes de `PhaseQ32` : déterministe, sans `atan2`, sans saut de phase à 2π. Hors de la grille, refus.

## 2. Mesuré (références écrites au plan par son script)

Une onde M2 progressive dans un chenal de 20 m (`c` = 14,007 m/s, longueur d'onde 626,3 km), une carte de 10 km de pas sur 100 km, 25 h au
pas d'une minute.

| | référence | mesuré |
|---|---|---|
| aux nœuds, contre `cos(ωt − kx)` | à la dérive de S577 près (2·10⁻⁵ m sur 25 h) | **1,98·10⁻⁵ m** au pire |
| au milieu d'une maille, l'amplitude | `1 − cos(k·Δx/2)` = 1 − 1,258·10⁻³ = 0,998742 | **0,998739** |
| le retard de la pleine mer de 0 à 50 km | 3 569,6 s | **3 540 s** (le pas d'échantillonnage : 60 s) |
| hors de la grille ; deux évaluations au même `(x, t)` | refus ; au bit | tenus |

Critères (écrits avant) : (1) à 10⁻⁴ m ; (2) à 10⁻⁴ ; (3) à 60 s ; (4) — **tenus**. Le plan avait écrit `+ Im H·sin ωt` avec
`H = A·e^(−ig)` : avec cette convention c'est `−` (la forme voulue, `A·cos(ωt − g)`, était juste) — implémenté selon la forme voulue.

## 3. Ce que cela dit — et ne dit pas

Une région décrit sa marée par une carte : la pleine mer remonte un chenal, plus tard d'une heure à 50 km. Manquent pour 2.2 : l'entrée
de la marée dans la surface de B (le niveau moyen variable que B échantillonne), les corrections nodales (18,6 ans), le niveau moyen
variable par la météo, des houles issues d'une météo, l'adoption par défaut ; et la carte d'une vraie côte (un précalcul, 12.3).
