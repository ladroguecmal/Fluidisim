# Le coût du recoupage d'une coque qui bouge — S508 (liste 6.4)

*S508, 2026-10-06, en autonomie.* Une coque qui bouge sur la carte coûtait 8 ms de CPU par pas (S503–S504) contre 0,7 à 0,9 ms pour le
pas de la carte : le `set_solid_rigid` entier du cœur, les extractions, l'envoi de toute la géométrie.

## Reproduire

- `MODE=pilonnement|roulis water-viewer --lineaire-coque` — ligne `COQUE_S504 etages_ms` : la part de chaque étage.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s508` — le recoupage en boîte contre l'entier, au bit.

## 1. Où passaient les 8 ms

| étage (ms par pas, coque en roulis) | avant | après |
|---|---|---|
| nœuds (la distance signée, l'hôte) | 0,69 | **0,25** (dans la boîte de la coque) |
| `set_solid_rigid` (le cœur) | **6,18** | **0,49 à 0,53** |
| terme de paroi | 0,22 | 0,01 |
| faces, transfert, colonnes, géométrie | 0,11 | 0,11 |
| envoi à la carte | 0,66 | 0,40 à 0,65 |
| **total** | **7,86** | **1,4 à 1,55** |

## 2. La construction

- **Le recoupage du cœur dans la boîte du solide** (`delta3d_cut::solid_box`, `check_solid_in`, `add_solid_in`) : les mailles qui touchent
  un nœud négatif, deux mailles de marge — hors d'elles, le solide ne coupe rien. La découpe du fond restaurée dans la boîte du pas
  précédent unie à la nouvelle ; la vérification faite une fois (elle passait deux fois) ; la diagonale de Jacobi revue dans cette boîte
  plus une maille ; le terme de paroi nul hors de la boîte. Le dépôt garde sa boucle entière : il renormalise la somme compensée de chaque
  colonne, et la limiter changerait des bits hors de la boîte.
- `Volume3::set_full_recut` (essais) garde la grille entière.
- **L'envoi** : deux écritures par pas au lieu de cinq, dans un tampon de conversion gardé.

## 3. Mesuré

| | |
|---|---|
| recoupage en boîte contre entier, 60 pas (coque qui pilonne, roule et avance à 2 m/s) : géométrie, terme de paroi, faces, colonnes, transfert, surface | **identiques au bit** |
| bancs de S358, S503, S504 | inchangés (S358 identique au binaire d'avant ; 2,205·10⁻⁶ m ; 1,669·10⁻⁶ et 2,384·10⁻⁷ m) |
| suite du cœur | 665 essais |

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) la part de chaque étage, publiée | §1 | tenu |
| (2) le recoupage limité identique au bit ; bancs inchangés ; suite | au bit ; inchangés ; 665 | tenu |
| (3) ≤ 1 ms par pas sur la coque de la porte D | 1,4 à 1,55 ms | **manqué** — cinq fois moins qu'avant |

**Ce qui reste** : l'envoi (0,4 à 0,65 ms : 450 Ko par pas, dont un tableau de faces presque tout « à garder » — n'envoyer que la boîte),
le cœur (0,5 ms : la coque couvre 17 % de ce petit domaine ; la part baisserait dans un domaine de jeu), les nœuds (0,25 ms, l'hôte).
**6.4 reste partielle** sur ce seul point.
