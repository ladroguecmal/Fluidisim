# La pression d'un nœud stratifié — S559 (ADR-241 ; liste 5.7 ; A17)

*S559, 2026-10-06, en autonomie.* [ADR-241](../adr/ADR-241-les-liquides-de-v.md) : plusieurs liquides par nœud de V, non miscibles, en
couches. Sa première pièce : la pression en un point d'un nœud stratifié (D3), dont dépendra le débit par couches (D4).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s559 -- --nocapture` ; suite du cœur : 721.

## 1. Ce qui est construit

`hydro_liquids.rs`, sous-module de V : `Liquid { density_kg_m3 }`, `MAX_LIQUIDS` = 8, `pressure_at(nœud, composition, liquides, formes,
g_eff, point)`. Les couches se rangent par densité décroissante (à densité égale, l'ordre de la table) ; chaque interface est le plan de
la géométrie du nœud pour le volume cumulé — le calcul même de la surface libre ; la pression est la somme des `ρᵢ·|g|·épaisseurᵢ`
au-dessus du point. L'état reste entier : la composition est une ligne de millilitres qui somme à `volume_ml`, sinon refus.

## 2. Mesuré, contre des formes fermées écrites au plan

| cas | référence | mesuré | écart |
|---|---|---|---|
| (1) cuve droite 4 × 1 × 2 m, eau 1 m sous huile 0,5 m, au fond | 13 979,25 Pa | 13 979,2506 Pa | 4·10⁻⁸ |
| (1) la même, à 1,2 m (dans l'huile) | 2 501,55 Pa | 2 501,5501 Pa | 4·10⁻⁸ |
| (2) la même sous `g_eff = (1 ; 0 ; −9,759)`, au coin bas | 11 906,5750 Pa | 11 906,5747 Pa | −2,1·10⁻⁸ |
| (3) carène en V (`V = h²`), eau 1 m³ sous huile 0,69 m³, à la quille | 12 311,55 Pa | 12 311,5505 Pa | 4·10⁻⁸ |
| (3) la même, à 1,2 m | 833,85 Pa | 833,8500 Pa | < 10⁻⁷ |
| (4) un seul liquide, 1,44 m³ dans la carène, à 0,3 m | `ρ·g·(surface − z)` = 8 829 Pa | 8 829,0004 Pa | 5·10⁻⁸ |

Critères (écrits avant) : (1)–(4) à 10⁻⁵ relatif — **tenus** (le quantum calculé, 10⁻⁷) ; (5) les refus (somme, longueurs, volume
négatif, densité nulle, neuf liquides) — **tenus** ; (6) la table inversée (l'huile déclarée avant l'eau) donne la même pression —
**tenu**.

## 3. Ce que cela dit — et ne dit pas

Un nœud de V sait désormais **où sont ses liquides et ce qu'ils pèsent**, dans toute forme et sous toute gravité. Rien ne coule encore
par couches : le pas de V ignore la composition (un réseau à un liquide, au bit). La suite : le débit d'une ouverture sur la différence
de pression au seuil, le liquide qui sort celui de la couche au seuil (D4) — le manomètre en U, la vidange d'une cuve stratifiée.
