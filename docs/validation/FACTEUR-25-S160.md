# S160 — Le facteur 2,5 : coïncidence, et le nombre lui-même est fragile

2026-09-10. Traite **S158-1**. Sonde : `code/water-core/examples/wake_plafond.rs`.

## 1. La question, et pourquoi elle méritait une heure

Deux sessions consécutives ont publié « 2,5 » à propos du même problème :

- **S157** — le groupement `t·½√(g·sigma)·dk` va de 1,17 à 2,94 sur sept configurations, « facteur
  2,5, mieux, pas constant » ;
- **S158** — l'estimateur d'erreur suit l'erreur vraie « à un facteur 2,5 près, et il la
  sous-estime ».

S158 demandait si c'était **le plafond de précision de tout ce qui touche au repliement**, ou une
coïncidence. La réponse est : **coïncidence**, pour deux raisons indépendantes, dont la première
ne demandait aucune mesure.

## 2. Ce ne sont pas les mêmes statistiques

Les deux livrables écrivent « facteur ». Ils ne mesurent pas la même chose :

| | S157 | S158 |
|---|---|---|
| grandeur | groupement adimensionnel sur sept configurations | rapport estimé/vrai sur douze cases |
| statistique publiée | **étendue** `max/min` | **déviation** au rapport idéal 1 |
| valeur publiée | 2,51 | 2,50 |

Mises à la même toise, elles cessent de coïncider :

| toise | S157 | S158 | écart |
|---|---:|---:|---:|
| étendue `max/min` | **2,51** | **3,42** | 36 % |
| déviation au centre géométrique | 1,59 | 1,98 | 25 % |

*Vérification des chiffres bruts, refaits à la source et non recopiés. `wake_law` rend `t·dk` de
0,75 à 2,44 ; multipliés par `½√(g·sigma)` — 0,783 à sigma 0,25, 1,566 à 1, 3,132 à 4 — les sept
points donnent 1,175 / 1,911 / 1,543 / 2,944 / 2,646 / 2,051 / 2,349, soit une étendue de 2,506.
`wake_estimator` est reproduit à l'identique par la sonde de cette session.*

**L'égalité apparente vient d'avoir comparé une étendue à une déviation.** Sur le seul jeu S158,
les deux statistiques valent 3,42 et 2,50 — elles ne sont pas interchangeables, et rien dans les
deux livrables ne dit laquelle est publiée.

## 3. Le 2,5 de S158 n'est pas un plafond : il dépend du montage

Un plafond de précision ne dépend pas de la source. Celui-ci, si. Même estimateur, même protocole,
quatre couples `sigma / cutoff` — `validate_recipe` impose `sigma·cutoff ∈ [1 ; 8]`, donc à
cutoff 6 le sigma ne peut pas dépasser 1,33 ; le quatrième couple sort de la plage en baissant
cutoff, à produit réduit 6.

| sigma / cutoff | cases retenues (erreur > 10 %) | étendue | **déviation à 1** |
|---|---:|---:|---:|
| 0,25 / 6 | 9 | 4,48 | **2,37** |
| 0,5 / 6 | 9 | 4,17 | **2,17** |
| 1 / 6 | 9 | 3,38 | **2,47** |
| 4 / 1,5 | 5 | 16,84 | **24,08** |

**À cutoff 6, la déviation est remarquablement stable — 2,17 à 2,47 sur un facteur 4 en sigma.**
Ce n'est pas rien : le 2,5 de S158 est robuste *dans sa famille de montages*. Mais il est
**dix fois plus grand** dès que le cutoff descend à 1,5, c'est-à-dire dès que le spectre est
tronqué près du pic.

Ce qui gouverne n'est donc pas sigma, et n'est pas le repliement en général : c'est **la largeur
de bande conservée**. Le nombre publié par S158 décrit son montage, pas une propriété du problème.

*Le point `4 / 1,5` est aussi celui où S157 butait — deux de ses trois cases y tombent « au-delà
de 64 s ». Les deux jeux se dégradent au même endroit, et pour la même raison : un spectre coupé
près du pic n'a plus assez de modes pour que quoi que ce soit se moyenne.*

## 4. Le régime de S158 était mal justifié

S158 écarte deux cases — radial 256 à 8 s et à 24 s — en écrivant que « l'erreur réelle vaut
0,2 % ». C'est vrai de la première (2,1e-3) et **faux de la seconde (8,7e-2, quarante fois plus)**.

La conséquence n'est pas cosmétique. Au seuil uniforme de 1 %, la case revient dans le jeu et la
déviation à sigma 1 passe de **2,47 à 12,30** :

| sigma / cutoff | seuil 1 % | seuil 10 % |
|---|---:|---:|
| 0,25 / 6 | 2,37 | 2,37 |
| 0,5 / 6 | 2,17 | 2,17 |
| 1 / 6 | **12,30** | **2,47** |
| 4 / 1,5 | 53,81 | 24,08 |

**Le nombre publié dépend donc d'un seuil de régime que le livrable n'énonce pas** et qu'il
justifie par une valeur inexacte. Les deux autres sigma sont insensibles au seuil, ce qui explique
que personne ne l'ait vu : le défaut ne se manifeste qu'à sigma 1, c'est-à-dire exactement sur la
seule ligne mesurée par S158.

## 5. Ce que S158-1 devient

**Fermée.** Il n'y a pas de plafond de précision du repliement, et la question est née de deux
nombres qui n'étaient pas comparables. Ce qui reste, et qui vaut mieux qu'une constante :

- l'estimateur de S158 **reste utile** — il suit l'erreur à un facteur ~2,3 près à cutoff 6, sur
  un facteur 4 en sigma, en la sous-estimant ;
- **sa fidélité doit être annoncée avec sa bande** : à cutoff 1,5 elle vaut 24, et un appelant qui
  lirait « facteur 2,5 » se tromperait d'un ordre de grandeur ;
- le seuil de régime fait partie de l'énoncé, pas du commentaire.

Une note datée est portée à [TOLERANCE-SILLAGE-S158](TOLERANCE-SILLAGE-S158.md).

**Ce que cette session ne dit pas.** Elle n'explique pas *pourquoi* la fidélité est stable à
cutoff 6 — trois sigma ne font pas une loi, et S157 a montré ce que coûte un ajustement sur un
plan d'expérience contraint (L235). Elle constate une stabilité et une rupture, sans les modéliser.
