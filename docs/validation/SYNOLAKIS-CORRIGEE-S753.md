# La 3D corrigée contre Synolakis — S753 (liste 4.1, 4.14 ; ADR-289 D3.3 ; ADR-291 D4)

*S753, 2026-10-09.* **La question** : la 3D corrigée d'ADR-291 (`Complete`, consciente du fond, R1) suit-elle les mesures de laboratoire de
Synolakis (l'onde solitaire `H/d` = 0,3 qui déferle sur 1:19,85) mieux que la 3D sans correction ?

## Ce qui est fait

`plage_synolakis_conf_s753` : le montage de S712, la 3D corrigée en option. L'essai : 2,5 cm, le pas plafonné à 2,5 ms (comme S713 E2), les
profils à t·√(g/d) = 15, 20, 25, comparés aux mesures.

## Reproduire

`python outils/essai.py the_corrected_3d_against_synolakis_s753 --ignore --marque "S712 photo"` (44 min).

## Mesuré

| t·√(g/d) | la crête mesurée | S713 E2, sans correction | **la 3D corrigée** | l'écart quadratique, sans → corrigée |
|---|---|---|---|---|
| 15 | 0,314 d en 8,38 d | 0,424 d en 8,07 d | **0,326 d en 9,42 d** | 0,044 → **0,030 d** |
| 20 | 0,318 d en 3,66 d | 0,316 d en 3,77 d | 0,319 d en **5,82 d** | 0,066 → 0,075 d |
| 25 | 0,190 d en 0,30 d | 0,218 d en −3,03 d | 0,450 d en 0,72 d | 0,035 → 0,059 d |

Le volume par la surface reste à 0,998 du compte.

**Critères** :
- (1) la crête à t = 15 à 0,05 d : **tenu** (+0,012 d ; sans correction, +0,11 d) ;
- (2) l'écart plus petit à deux des trois instants : **manqué** (à t = 15 seulement).

**La vague corrigée a la bonne hauteur, mais elle est en retard, et le retard grandit** : 1,0 d à t = 15, 2,2 d à t = 20 ; à t = 25, elle
n'a pas encore déferlé.

**La célérité, relue sur le canal de S752** (la crête de 1 à 4,25 s, contre 2,426 m/s exacts) :

| version | célérité |
|---|---|
| `Complete` consciente | 2,436 m/s (+0,4 %) |
| **R1 (ADR-291)** | **2,290 m/s (−5,6 %)** |
| R1′ | 2,395 m/s (−1,3 %) |

## Ce que cela dit

- **R1 ralentit la vague de 5,6 %** : c'est le retard qui grandit contre Synolakis. La reprise de la vitesse depuis la grille, qui rendait la
  remontée de S645, freine la propagation. `Complete` garde la célérité juste.
- **Le choix d'ADR-291 était prématuré.** Il reposait sur la remontée de S645 jugée contre une loi théorique ; la référence extérieure (le
  laboratoire) le contredit. Une note datée est ajoutée à ADR-291 (on ne le réécrit pas).
- La remontée de S645 et la célérité tirent en sens opposés : c'est au laboratoire de trancher (ADR-280 D2).

## La suite (S754)

`Complete` consciente (sans R1) contre Synolakis : si elle suit mieux les mesures, y compris sur la plage à t = 25, c'est elle que l'on
garde, et la remontée de S645 contre la loi passe au second plan.
