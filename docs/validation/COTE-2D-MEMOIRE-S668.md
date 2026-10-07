# La mémoire de Cote2D réduite : la marche fine, les tables décimées — S668 (liste 2.7)

*S668, 2026-10-07, en autonomie, vers la v2.* `Cote2D` coûtait 160 Mo/km² pour 32 composantes au pas de 2 m (S667).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s668 -- --nocapture` (≈ 3 s).

## Le principe

`Cote2D::cuire_decime(…, m)` sépare deux pas :

- **la marche** du modèle parabolique reste au pas fin (2 m), ce qui garde sa justesse ;
- **les tables** (phase, facteur, `kv`, `coth`) ne gardent qu'un nœud sur `m` dans chaque direction. Leur pas devient `m·pas`, et la
  mémoire est divisée par `m²`.

La phase se dérive sur la marche fine, puis se lit aux nœuds gardés. `cuire` revient à `cuire_decime` avec `m` = 1, au bit près.

## Mesuré

**L'instrument** : la côte 2D pleine (`m` = 1), la mer de huit composantes de S667 (`Hs` 2,2 m), 60 points × 3 instants.

**Ce qui départage** :

- une décimation juste donne un écart qui croît avec le pas (la loi `Δ²` d'ADR-196 D2) ;
- une faute d'indice donne un écart dès `m` = 1, ou un écart qui ne croît pas avec `m`.

| `m` | pas des tables | `|Δη|` au plus | 1 km², 32 composantes |
|---|---|---|---|
| 1 | 2 m | **0, au bit** (critère 1) | 160 Mo |
| 2 | 4 m | 0,21 mm | 40 Mo |
| **4** | **8 m** | **0,70 mm** | **10 Mo** |
| 8 | 16 m | 4,3 mm | 2,5 Mo |

**Le plus grand `m` sous 3 mm** (la tolérance d'image, S201) est **`m` = 4** : les tables à 8 m, **10 Mo par km²** pour 32 composantes,
soit seize fois moins. L'écart croît avec `m` (critère 3, asserté) : ×3,3 de 2 à 4, puis ×6,2 de 4 à 8, ce qui rejoint `Δ²` aux
grands pas.

Le plan avait borné l'écart par la loi 1D (1,9 mm à 4 m, 7,7 mm à 8 m, 31 mm à 16 m pour `Σa` = 2,1 m). Les mesures restent dessous :
cette borne additionne les amplitudes au pire, alors que les composantes réelles ne s'alignent pas en phase.

## Ce que cela dit

Une côte d'un km² sur 32 composantes tient en 10 Mo. C'est assez pour un rivage de jeu chargé autour du joueur, mais pas encore pour
toutes les côtes d'une planète à la fois. Restent à mesurer :

- **un pas adapté au gradient de `k`** : grossier au large, fin seulement près du rivage, où le gain serait encore d'un ordre ;
- **des transformations partagées** entre composantes voisines ;
- une côte non uniforme le long de ses bords ;
- la côte 2D dans les autres chemins de B et dans Godot.
