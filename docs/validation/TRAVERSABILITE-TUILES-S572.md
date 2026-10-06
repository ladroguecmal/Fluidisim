# Les tuiles de traversabilité — S572 (liste 7.7 ; SPEC-006 §5.2)

*S572, 2026-10-06, en autonomie.* L'unité de publication de la traversabilité, après l'échantillon de [S570](TRAVERSABILITE-S570.md).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s572 -- --nocapture` ; suite du cœur : 745.

## 1. Ce qui est construit

`TileDesc` (référentiel, code de Morton, classe de cadence `Maree | Debit | NoeudV | Immediat`, subdivision ≤ 4, séquence propre à la
tuile) ; `publier(…)` — les `(16·2^s)²` échantillons depuis un échantillonneur de l'appelant, la prévision par cellule (`t_next_cross` et
sa cause) quand une profondeur prévisible est fournie, la séquence qui avance, et **les événements de franchissement** : une cellule dont
la classe de profondeur ou de danger a changé depuis la publication précédente, dans un tampon de l'appelant. Tout se calcule avant
d'écrire : un refus ne laisse rien.

## 2. Mesuré (références écrites au plan par son script)

Une tuile de 16 × 16 cellules de 64 m sur une plage (fond de −2 à +2 m), une marée de 0,4 m (M2), courant nul ; publiée à `t₀` = 0 puis à
`t₁ = T/12`.

| | référence | mesuré |
|---|---|---|
| événements à la première publication | 0 | 0 |
| événements entre `t₀` et `t₁` | **80**, colonnes 2, 3, 4, 6, 7 | **80**, colonnes 2, 3, 4, 6, 7 ; avant ≠ après |
| prévision de la colonne 6 à `t₁` (0,575 m d'eau) | 0,50 m repassé en descendant dans 16 368,323 s, cause `Maree` | **16 368,323 s**, `Maree` |
| Morton de la tuile (3, 5) ; séquences | 39 ; 1 puis 2 | 39 ; 2 |
| subdivision 1 | 32 × 32 = 1 024 échantillons | 1 024 |

Critères (écrits avant) : (1) — **tenu** ; (2) à 10 ms — **tenu** ; (3) — **tenu** ; (4) les refus (tampon court, subdivision 5,
profondeur négative), la séquence intacte — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La traversabilité se publie comme SPEC-006 la décrit : par tuile, avec sa séquence, ses franchissements et ses prévisions. Manquent pour
7.7 : l'invalidation des prévisions par une commande de V (SPEC-006 §5.4), la glace porteuse et sa charge, la température, le gué d'un
véhicule et le tirant d'eau d'un bateau, la marée de B (2.2), et la source réelle des échantillons (les couches répliquées : B, W, V —
le canal est autoritaire, I-15).
