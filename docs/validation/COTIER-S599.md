# La bibliothèque côtière — S599 (listes 12.3, 2.8 ; SPEC-005 §6)

*S599, 2026-10-07, en autonomie (ADR-247).* 12.3 (le précalcul côtier stocké) et 2.8 (le précalcul côtier et la météo — la météo à la fin)
étaient absents. Une zone de déferlement met des dizaines de secondes à s'établir (ADR-013 §4) : on la cuit.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s599 -- --nocapture` ; suite du cœur : 775.

## 1. Ce qui est construit

`cotier.rs` : `cuire` — pour chaque état de mer et chaque phase de marée, un `CoastalState` : un champ 2D au pas de la grille (hauteur
de houle, `u`, `v`, rouleau) en **`f16`** (une conversion binary16 exacte et déterministe, écrite ici) et la polyligne de déferlement (S588) ;
la hauteur levée par la référence de B hors de la zone de déferlement, saturée à `0,78·h` dedans ; le rouleau, la dissipation `d(E·c_g)/dx` ;
`u`, `v` **nuls** tant que le courant de dérive littorale n'est pas calculé. `recherche(hs, phase)` — l'état le plus proche **par ses
paramètres** (I-09), la phase repliée ; `a_jour` — l'empreinte FNV-1a des entrées.

## 2. Mesuré (références écrites au plan par son script)

Une plage de pente 0,04 (120 × 20 m, 240 × 40 texels comme SPEC-005), une houle de 8 s, Hs ∈ {0,5 ; 1 ; 1,5 ; 2} m, une marée de 1 m (4
phases).

| | référence | mesuré |
|---|---|---|
| la taille | 76 800 octets par état, 1 228 800 pour seize (SPEC-005 : « 77 Ko », « 1,2 Mo ») | **1 228 800** |
| les lignes de déferlement | `x_b = (h_b − η)/0,04` | **0,22 mm** au pire ; l'état (0,5 m ; marée haute) sans ligne — sa référence la place hors de la grille (−1,64 m) |
| le déplacement avec la marée, Hs = 2 m | 50 m | **50,0000 m** (47,6081 → 97,6081 m) |
| le champ : la hauteur dans la zone de déferlement ; le rouleau | `0,78·h` à 2⁻¹⁰ ; nul au large, positif dedans | tenu |
| la recherche | (1,1 m ; 0,97) → (1 m ; 0) ; (1,8 m ; 0,6) → (2 m ; ½) | tenu |
| l'empreinte | deux cuissons : la même ; un centimètre de fond changé : obsolète | tenu |

Critères (écrits avant) : (1)–(5) — **tenus**. **En route** : l'essai exigeait une ligne pour chaque état, attente hors du plan ; la
référence du plan elle-même plaçait une ligne hors de la grille — écrit aux notes, puis l'essai corrigé (aucun sommet attendu là).

## 3. Ce que cela dit — et ne dit pas

Une plage se cuit en seize états de 75 Ko, se retrouve par ses paramètres et sait quand elle est périmée. Manquent : le courant de dérive
littorale (`u`, `v`), le rouleau d'un champ 3D ré-établi dans δ (la suite de 4.14), le stockage sur disque et la compression, les plages
réelles (la bathymétrie du jeu), et la météo — à la fin (ADR-197 D5).
