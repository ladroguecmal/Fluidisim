# Le décor qui perce la surface, sur la carte — S493 (liste 6.5, validée)

*S493, 2026-10-06, en autonomie.* La suite de [DECOR-S490](DECOR-S490.md) (la référence) et [A327-S492](A327-S492.md) (le point mort).

## Reproduire

- `CLOISON_E=<0.05|0.075> [CYCLES=60] water-viewer --lineaire-cloison` — la cloison de S490, alignée sur la grille (0,05) ou en milieu de
  maille (0,075) ; la référence (`Volume3`) et la carte (`Linear3`) depuis le même état, l'écart relevé tous les 50 pas.
- `water-viewer --lineaire-carte --solide --cycles=32` — le banc de S358 (couvercle plein), au bit avant et après.

## 1. La construction

`Linear3` (`viewer/src/delta3d_linear.{rs,wgsl}`) refusait un couvercle qui n'était pas entièrement ouvert. **Le couvercle partiel d'un
décor fixe** y est porté : sur la part libre `a` d'une colonne en partie couverte, la pression `ρg·η/max(a, plancher)` de S334, le plancher
`dt²·g/dx` borné à [0,1 ; 1] (celui du cœur), dans le second membre et dans la correction. Ni dépôt ni transfert (une coque qui bouge : non
portée). **Le compilateur dans la boucle (L345)** : une première écriture, une branche dans `lid`, changeait les bits du couvercle plein
(résidu 8,874 → 8,941·10⁻⁵) ; la branche placée aux deux endroits d'usage garde l'expression d'avant pour le couvercle plein — **au bit**.

## 2. Mesuré

| cloison | colonnes à couvercle partiel / fermé | carte contre référence | moitié droite (carte) | période (carte) | théorie |
|---|---|---|---|---|---|
| alignée sur la grille | 8 / 4 | **5,96·10⁻⁸ m** | **0** | 1,2312 s | 1,2292 s (0,16 %) |
| milieu de maille | 8 / 8 | **5,96·10⁻⁸ m** | **0** | 1,2111 s | 1,2143 s (0,27 %) |

Le banc de S358 (sphère, 32 cycles) : **identique au bit** à avant S493 (deux exécutions de chaque version, déterministes).

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) les bancs de S358 inchangés, au bit | identique au bit | tenu |
| (2) la carte suit la référence à 10⁻⁴ m, alignée et en milieu de maille | 5,96·10⁻⁸ m | tenu |
| (3) la moitié droite sous 10⁻⁶ m ; la période à 3 % | 0 ; 0,16 et 0,27 % | tenu |
| 6.5 validée | référence (S490, S492) et production (S493) | **validée** |

La liste : **5 points validés sur 120**.
