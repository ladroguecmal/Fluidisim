# La solution d'un réseau en charge — S565 (liste 5.8)

*S565, 2026-10-06, en autonomie.* 5.8, le réseau fermé sous pression, était absent — « reporté en v2 » par ADR-010 §4 ; la v1 est
atteinte et la liste entière est l'objectif (ADR-190). Première pièce : résoudre un réseau de conduites en charge.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s565 -- --nocapture` ; suite du cœur : 733.

## 1. Ce qui est construit

`hydro_charge.rs`, sous-module de V : des conduites `h_a − h_b = R·Q·|Q|` entre des sommets fixes (la surface d'un nœud de V, un réservoir)
et des jonctions (charges inconnues, une demande chacune). Newton sur les charges : la jacobienne, un laplacien pondéré par `dQ/dΔh`,
résolue par Gauss à pivot partiel dans un tampon de l'appelant ; un pas amorti s'il n'abaisse pas le résidu ; sous 1 µm de perte, la
conduite linéarisée (raccord continu). Une jonction sans chemin vers une charge fixe est refusée avant tout calcul.

## 2. Mesuré, contre deux méthodes indépendantes (écrites au plan)

| cas | référence | mesuré | itérations |
|---|---|---|---|
| trois réservoirs (100, 80, 50 m) — **bissection** | `h_j` = 77,455794994 m ; 0,106170158 / 0,029121613 / −0,135291771 m³/s | 77,455794994 m ; 0,1061701583 / 0,0291216129 / −0,1352917711 | 4 |
| une maille, un réservoir à 60 m — **Hardy Cross** | J0 43,8 ; J1 34,964659639 ; J2 33,231017516 ; J3 34,924075772 m | 43,8 ; 34,9646596388 ; 33,2310175162 ; 34,9240757725 | 8 |

La continuité aux jonctions à 5·10⁻¹⁷ m³/s. Critères (écrits avant) : (1), (2) les charges à 10⁻⁸ m, les débits à 10⁻⁹ m³/s, la
continuité sous 10⁻¹², moins de 30 itérations — **tenus** ; (3) les refus (une île sans charge fixe, une résistance nulle, un tampon court)
— **tenus**.

## 3. Ce que cela dit — et ne dit pas

V sait résoudre un réseau en charge : la plomberie d'un navire, une conduite forcée, un réseau de ville, entre des réservoirs de V. Ne dit
rien encore : **le couplage au pas de V** (les réservoirs qui se vident et se remplissent par le réseau, en millilitres entiers), les
pompes et les clapets dans le réseau, le coup de bélier (un réseau incompressible n'en a pas : un modèle à part), le coût sur un grand
réseau (la matrice dense, `n²`).
