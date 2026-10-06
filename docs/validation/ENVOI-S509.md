# La coque qui bouge sous 1 ms de CPU par pas — S509 (liste 6.4, validée)

*S509, 2026-10-06, en autonomie.* La suite de [RECOUPAGE-S508](RECOUPAGE-S508.md) : le recoupage dans la boîte du solide avait ramené le
coût CPU d'une coque qui bouge sur la carte de 8 à 1,4–1,55 ms par pas ; le critère était 1 ms.

## Reproduire

- `MODE=pilonnement|roulis water-viewer --lineaire-coque` — lignes `COQUE_S504 bilan` et `etages_ms` (`valeurs_par_pas`).
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s508` — recoupage en boîte et faces en place contre
  l'entier, au bit.

## 1. La construction

- **N'envoyer que ce qui change** (`Linear3::set_motion`) : la carte garde une ombre de ce qu'elle a reçu ; les valeurs qui en diffèrent
  (bit à bit) partent en paires (indice, bits) ; un noyau de dispersion, dans un module à part (la géométrie y est en écriture, les
  noyaux du pas la gardent en lecture seule : leur code ne change pas), les écrit au début du pas. La validation porte sur les seules
  valeurs changées ; l'ombre n'est mise à jour qu'une fois toutes validées.
- **Le test bon marché d'abord** dans la vérification du solide (la maille coupée par le fond ou dans la couche du couvercle, puis
  seulement sa part solide) : même refus, dans le même ordre.
- **`changed_faces_in_place`** : les faces de la seule boîte du dernier recoupage (hors d'elle, rien n'a changé).
- **Le banc** : les nœuds dans la boîte englobante de la coque orientée, plutôt que dans sa sphère.

## 2. Mesuré (coque de la porte D, 48 × 32 × 8 mailles de 25 cm)

| étage (ms par pas) | S504 | S508 | **S509** |
|---|---|---|---|
| nœuds (l'hôte) | 0,69 | 0,25 | **0,07** |
| `set_solid_rigid` (le cœur) | 6,18 | 0,49–0,53 | **0,41–0,42** |
| terme de paroi, faces, transfert, colonnes, géométrie | 0,33 | 0,12 | **0,05** |
| envoi | 0,66 | 0,40–0,65 | **0,26–0,28** (644 à 882 valeurs par pas, ≈ 6 Ko) |
| **total** | **7,9** | **1,4–1,55** | **0,81–0,82** |

Les bancs donnent les mêmes chiffres qu'avant (S358 identique au binaire d'avant ; S493 5,96·10⁻⁸ m ; S503 2,205·10⁻⁶ m ; S504 1,669·10⁻⁶
et 2,384·10⁻⁷ m) ; l'essai au bit (60 pas, coque qui pilonne, roule et avance) tient avec les faces en place. Suite du cœur : 665 essais.
L'envoi garde un coût fixe d'environ 0,13 ms par écriture, quelle que soit sa taille.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) les mêmes bits qu'avec l'envoi entier ; S358 identique | mêmes chiffres partout ; identique | tenu |
| (2) ≤ 1 ms par pas sur la coque de la porte D | 0,81–0,82 ms | **tenu** |

## 4. 6.4

Ses manques nommés sont faits : la coque qui bouge sur la carte (S503, S504), C23 sur le système (S505), le coût du recoupage (S508, S509).
**6.4 validée** — parois et corps mobiles dans δ, sous ses limites écrites : l'ordre 1 en temps du δ linéaire, cinq fois plus coûteux près
d'une coque qui bouge (A328, S507 : 10 % du champ proche à ≈ 6 ms) ; le recoupage sur le CPU, la carte ne recevant que ce qui change. La
liste : **8 points validés sur 120**.
