# S122 — Les bornes de construction d'un champ d'impact

2026-09-09. Suite de S121-1, sur A198.

## 1. Inventaire : treize bornes, six noms

`RadialImpact::new` refuse pour treize conditions distinctes, qui se partagent six noms
d'erreur. La colonne « paramètre à revoir » est ce qu'un appelant voudrait savoir, et que le
nom ne lui dit pas.

| # | condition | paramètre à revoir | nom aujourd'hui |
|---|---|---|---|
| 1 | `N ∈ [64, 256]` | nombre de modes | `Domain` |
| 2 | rayon fini, `> 0`, `< 4096` | rayon | `Domain` |
| 3 | `age_us ≠ 0` | horizon | `Domain` |
| 4 | `birth + age_us` représentable | naissance, horizon | `Domain` |
| 5 | milieu fini et `> 0` | milieu | `Medium` |
| 6 | `anisotropy == 0` | événement | `Anisotropy` |
| 7 | `hi` fini, `lo > 0` | longueur d'onde | `Domain` |
| 8 | `hi · radius ≤ 64` | **rayon et longueur d'onde ensemble** | `Domain` |
| 9 | `depth > π / lo` | **profondeur et longueur d'onde ensemble** | `Medium` |
| 10 | `dk · (radius + c_g · âge) ≤ π/2` | **quatre paramètres ensemble** | `Domain` |
| 11 | `scale` fini `> 0` | énergie | `Domain` |
| 12 | fréquence finie, `≥ 1`, `< u64::MAX` | longueur d'onde | `Domain` |
| 13 | pente finie, `≤ max_slope` | énergie, pente du milieu | `NotRepresentable` / `Steepness` |

**`Domain` recouvre sept conditions portant sur cinq paramètres différents.** `Medium` en
recouvre deux, dont une qui ne parle pas du milieu : la condition 9 refuse un milieu
parfaitement valide dès que la longueur d'onde dépasse la profondeur. C'est le régime d'eau
profonde, pas un défaut du milieu — et c'est le même défaut de nommage qu'ADR-081 vient de
corriger un cran plus loin.

## 2. La carte mesurée

`probe_degenerate` construit un champ pour chaque couple (longueur d'onde, rayon), à énergie
0,01 J et milieu sain — profondeur 20 m, pente maximale 0,1, horizon 4 s :

| λ \ rayon | 0,01 m | 1 m | 16 m | 1000 m |
|---|---|---|---|---|
| ≤ 0,1 m | `Domain` | `Domain` | `Domain` | `Domain` |
| 1 m | **construit** | **construit** | `Domain` | `Domain` |
| 10 m | **construit** | **construit** | **construit** | `Domain` |
| 100 m | `Medium` | `Medium` | `Medium` | `Domain` |
| ≥ 1000 m | `Medium` | `Medium` | `Medium` | `Medium` |

Trois enseignements, dont deux n'étaient écrits nulle part :

- **La zone acceptée est un couloir étroit** — longueurs d'onde de l'ordre du mètre à la
  dizaine de mètres, rayon d'autant plus petit que la longueur d'onde est courte. Rien ne le
  documentait ; il fallait construire pour l'apprendre.
- **Les refus qui la bordent ne sont pas les mêmes selon le côté.** En dessous, c'est la
  résolution (condition 10) qui mord : `dk · (radius + c_g · âge)` dépasse π/2. Au-dessus,
  c'est le régime d'eau profonde (condition 9). À droite, c'est la portée de la table de
  Bessel (condition 8). Trois causes, deux noms.
- **La carte est une coupe, pas une frontière fixe.** L'horizon entre dans la condition 10 :
  un âge plus court élargit le couloir vers les courtes longueurs d'onde. Aucune de ces
  dépendances n'apparaît dans le nom du refus.

## 3. Décision

Voir [ADR-082](../adr/ADR-082-nommer-la-borne-qui-refuse.md).

## 4. Construction et réception

*(à compléter)*
