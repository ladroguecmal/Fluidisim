# La polyligne de déferlement — S588 (liste 3.5 ; SPEC-006 §6)

*S588, 2026-10-07, en autonomie (ADR-247 : la physique d'abord).* 3.5 était absent. SPEC-006 §6 ferme le format : une polyligne cuite,
dérivée de la bathymétrie et de l'état de mer, dont chaque sommet porte le flux dissipé (kW/m) et la direction de crête.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s588 -- --nocapture` ; suite du cœur : 763.

## 1. Ce qui est construit

`deferlement::polyligne(…)` : sur une grille de profondeurs (la côte le long de `y`, le large vers `+x`), la hauteur de la houle en chaque
nœud par la référence de B (`bathymetrie::transformer` : levée et réfraction), l'écart `H − 0,78·h` (McCowan), et ligne par ligne depuis le
large, le premier passage par zéro interpolé linéairement — un sommet, avec `ρ·g·H²/8·c_g` en kW/m et la direction de crête. Une ligne
où la houle atteint la terre sans déferler dans la grille n'a pas de sommet.

## 2. Mesuré (références écrites au plan par son script, avec ses propres formules de dispersion, de levée et de réfraction)

Une plage `h = 0,02·x`, une houle de 2 m, 8 s, à 20°, une grille de 5 m (60 × 8 nœuds).

| | référence | mesuré |
|---|---|---|
| le sommet de chaque ligne | l'algorithme (interpolation linéaire) : 142,1033 m ; la racine : 142,0982 m (`h_b` = 2,8420 m) | **142,1033 m**, les 8 lignes |
| le flux dissipé | 29,799753 kW/m | **29,799763 kW/m** (3·10⁻⁷) |
| la direction de la crête | 8,0632° de la normale | **8,0634°** |
| une houle de 1 cm (elle déferle sous le premier nœud mouillé) | aucun sommet | **0** |
| une grille d'une colonne, un tampon trop court | refus | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La ligne où la houle se brise est publiée avec ce qu'elle dissipe : l'audio, l'ordonnanceur (qui active une plage) et le rendu ont leur
donnée. Manquent pour 3.5 : une côte quelconque (les marching squares, le chaînage des segments, les îles), la largeur de la zone de
déferlement, une polyligne par phase de marée (la marée de B existe depuis S577), la publication par le chemin poussé.
