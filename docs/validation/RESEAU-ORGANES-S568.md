# Les pompes et les clapets du réseau en charge — S568 (liste 5.8)

*S568, 2026-10-06, en autonomie.* La suite de [S565](RESEAU-CHARGE-S565.md) et [S567](RESEAU-COUPLE-S567.md).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s568 -- --nocapture` ; suite du cœur : 738.

## 1. Ce qui est construit

Une conduite porte un **organe** (`Organe`) : aucun ; un **clapet** (le débit de `a` vers `b` seulement ; fermé, une fuite linéaire de
10⁻¹² m²/s garde la jacobienne inversible) ; une **pompe** centrifuge avec son clapet, la loi de V (ADR-199 D3), `h_a − h_b + H₀ =
(R + H₀/Q_max²)·Q²`, `Q ≥ 0`. **En route**, un arrêt au plancher flottant : une conduite presque sans résistance amplifie l'ulp de la
charge (×10⁶ ici) au-delà de la tolérance de continuité, que Newton ne pouvait plus atteindre (`NonFinite`) ; désormais un pas de Newton
sous la résolution des charges arrête le calcul, le résidu publié tel quel.

## 2. Mesuré (références écrites au plan par son script)

| cas | référence | mesuré |
|---|---|---|
| (1) une pompe (`H₀` 30 m, `Q_max` 0,05 m³/s) de 0 m vers 20 m par une jonction — bissection | `h_j` 21,764705882 m, `Q` 0,024253563 m³/s | **21,764705882 m, 0,024253563 m³/s** (9 itérations) |
| (2) les trois réservoirs de S565, un clapet sur la branche de 80 m — forme fermée | `h_j` 71,428571429 m, la branche fermée | **71,428571430 m** ; la branche à −8,6·10⁻¹² m³/s (la fuite) |
| (3) couplé : une pompe (`H₀` 1 m) remplit la cuve haute (1,5 / 0,2 m), 430 s | 0,35 et 1,35 m au refoulement nul, vers 213 s | **0,350000 et 1,350001 m** ; rien ne revient ensuite ; la masse à l'entier |

Critères (écrits avant) : (1), (2) à 10⁻⁸ — **tenus** ; (3) à 2·10⁻⁴ m, la masse, aucun retour — **tenus** ; (4) S565 et S567 inchangés —
**tenu**. Le plan avait écrit le déplacement par la fuite (« ~4·10⁻⁹ m ») d'un calcul à la main, contre ADR-243 D1 : un majorant, sans
effet sur le critère.

## 3. Ce que cela dit — et ne dit pas

Le réseau en charge pompe et retient : une pompe de cale qui refoule vers un réservoir haut s'arrête d'elle-même à sa hauteur de barrage,
et son clapet tient l'eau montée. Manquent pour 5.8 : la vitesse de la pompe commandée (la commande d'ADR-199 D1), l'air des poches aux
raccords, un raccord qui se dénoie, le coup de bélier, le coût d'un grand réseau.
