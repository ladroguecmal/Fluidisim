# La vitesse des pompes et le raccord qui se dénoie — S569 (liste 5.8)

*S569, 2026-10-06, en autonomie.* Deux manques de [S568](RESEAU-ORGANES-S568.md) : la vitesse commandée d'une pompe, et un raccord hors de
l'eau — que S567 refusait (`Domain`), laissant l'hôte sans pas possible.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s569 -- --nocapture` ; suite du cœur : 741.

## 1. Ce qui est construit

- `Organe::Pompe { h0_m, qmax_m3s, vitesse }` : lois de similitude (`H ∝ n²`, `Q ∝ n`), `H(Q) = H₀·n² − H₀·Q²/Q_max²`.
- **Un raccord hors de l'eau est un exutoire à l'air libre** : sa charge est sa cote, et il ne fait que recevoir — le débit qui sortirait
  de son nœud (le réseau aspirerait de l'air) est coupé comme par un clapet. Plus de refus.

## 2. Mesuré (références écrites au plan par son script)

| cas | référence | mesuré |
|---|---|---|
| (1) la pompe de S568 à `n` = 0,9 — bissection (à 0,8 elle ne monterait plus à 20 m : le script l'a dit avant d'écrire le plan) | `h_j` 20,758823529 m, `Q` 0,015904125 m³/s | **20,758823529 m, 0,015904125 m³/s** |
| (2) exutoire : A (1,5 m) se vide par le fond vers B dont l'arrivée est à 1,0 m, au-dessus de son eau | `h_A` 1,208947 m à 100 s, 1,042893 m à 200 s ; arrêt à 1,0 m (283 s), B à 0,7 m | **1,208897 m, 1,042830 m** (l'Euler du pas) ; **A 1 000 000 ml, B 700 001 ml** ; la masse à l'entier |
| (3) la sortie de A en paroi à 1,0 m, B rempli par le fond | A s'arrête sous sa sortie, dépassement ≤ 0,274 mm | **A 999 874 ml** (0,126 mm sous la sortie), immobile dès le pas 1 184 |

Critères (écrits avant) : (1) à 10⁻⁸ — **tenu** ; (2) 10⁻³ m de la loi fermée, l'état final à 2·10⁻⁴ m, la masse — **tenus** ; (3) dans
l'intervalle, puis immobile, aucun refus — **tenu** ; (4) S565–S568 inchangés (l'essai de S567 qui attendait le refus d'un raccord sec le
voit désormais passer, sans débit) — **tenu**.

## 3. Ce que cela dit

Le réseau en charge tient les situations d'un jeu : une citerne qui se vide par une sortie en paroi s'arrête quand l'eau passe dessous,
une conduite qui déverse au-dessus de l'eau d'une cuve la remplit comme une fontaine, et une pompe se commande en vitesse. Manquent pour
5.8 : l'air des poches aux raccords, le coup de bélier, le coût d'un grand réseau.
