# Cote2D à huit composantes : la composition, la mémoire — S667 (liste 2.7)

*S667, 2026-10-07, en autonomie, vers la v2.* `Cote2D` (S664–S665) n'avait été jugée que sur une composante.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s667 -- --nocapture` (≈ 2 s).

## Mesuré

**Le montage** : une mer de huit composantes (périodes de 7 à 12 s, directions de −20° à +20°, `Hs` = 2,19 m) sur la plage de S364. Les
deux côtes, 1D et 2D, sont évaluées en 60 points × 3 instants.

**La borne** (ADR-268 D1) : en chaque point, `Σ a_c·f_c·(|Δφ_c| + |Δf_c|/f_c)`, calculée depuis les écarts de chaque composante en ce
point. Ce qui départage : une composition juste reste dessous, une diaphonie entre composantes la dépasse. Au plan, son ordre était de
19 cm (4° de phase par composante) ; en ces points, elle est bien plus serrée.

| | critère | mesuré |
|---|---|---|
| (1) `|η₂D − η₁D|` sous la borne, en chaque point | 180/180 | **180/180** ; rapport au plus 0,78 ; `|Δη|` au plus **0,5 cm** |
| (2) la cuisson | rapportée | 1,5 s pour 8 composantes (1 951 × 101 nœuds, 3,9 km × 200 m) |
| (2) la mémoire | rapportée | 3,9 Mo par composante ; **1 km² au pas de 2 m, 32 composantes : 160 Mo** |

## Ce que cela dit

La côte 2D compose une mer entière sans diaphonie. Une mer de 2 m y est rendue à un demi-centimètre de la référence exacte.

La mémoire, en revanche, ne tient pas une planète : 160 Mo par km² (ADR-196 l'estimait à 128). Les voies qu'ADR-196 §3 nomme sont à
mesurer :

- **un pas adapté au gradient de `k`** : grossier au large, où rien ne change, fin seulement près du rivage et des hauts-fonds ;
- **des transformations partagées** entre composantes voisines en période et en direction.

Manquent : la mémoire réduite, une côte non uniforme le long de ses bords, la côte 2D dans les autres chemins de B et dans Godot.
