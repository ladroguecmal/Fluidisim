# `Complete` consciente contre Synolakis — S754 (liste 4.1, 4.14 ; ADR-292)

*S754, 2026-10-09.* SYNOLAKIS-CORRIGEE-S753 : R1 rend la hauteur, mais ralentit la vague. **La question** : `Complete` consciente du fond,
sans R1, suit-elle mieux les mesures de Synolakis ?

## Reproduire

`python outils/essai.py complete_density_against_synolakis_s754 --ignore --marque "S712 photo"` (42 min).

## Mesuré

| t·√(g/d) | la mesure | **`Complete` consciente** | R1 (S753) | l'écart quadratique, R1 → `Complete` |
|---|---|---|---|---|
| 15 | 0,3135 d en 8,38 d | **0,3644 d en 7,77 d** | 0,3258 d en 9,42 d | 0,030 → 0,035 d |
| 20 | 0,3175 d en 3,66 d | **0,3228 d en 2,97 d** | 0,3194 d en 5,82 d | 0,075 → **0,047 d** |
| 25 | 0,1897 d en 0,30 d | **0,1967 d en −1,53 d** | 0,4497 d en 0,72 d | 0,059 → **0,025 d** |

Le volume par la surface reste à 0,998 du compte.

**Critères** :
- (1) la crête à t = 15 à 0,05 d : **manqué de 0,0009 d** (0,0509 d ; 0,45 mm, vingt fois sous le quantum de lecture) ;
- (2) la place plus proche que R1 à t = 15 et t = 20 : **tenu** (0,61 et 0,69 d d'écart, contre 1,04 et 2,16) ;
- (3) l'écart plus petit que R1 à deux instants : **tenu** (t = 20 et 25).

Contre la 3D sans correction (S713 E2 : 0,044, 0,066, 0,035 d), `Complete` fait mieux aux trois instants.

## Ce que cela dit, et la décision

Le laboratoire préfère `Complete` : la célérité juste, et la lame sur la plage (t = 25) au plus près de la mesure. **ADR-292** la retient
comme la 3D corrigée, et lève par écrit le critère (1), manqué sous le quantum. Ce qui reste ouvert : la remontée de S645 contre la loi
théorique (1:3, −11 %), et la crête un peu en avance.
