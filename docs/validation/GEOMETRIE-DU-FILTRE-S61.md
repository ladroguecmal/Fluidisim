# Le filtre de contamination est une condition géométrique — S61, 2026-09-08

Instruction de l'action **S60-1**, qui demandait de mesurer une grille dont l'erreur passe **sous**
l'écart des deux oracles. La session a commencé par chiffrer l'expérience avant de la lancer
(**L177**). Elle ne l'a pas lancée : **le régime visé n'existe pas dans ce dispositif.**

Aucun calcul nouveau n'a été nécessaire. Tout ce qui suit se lit sur les cinq campagnes déjà
consignées — S48/S49, S56, S57, S59 et l'essai de refus de S60.

## 1. Le rapport erreur/écart ne dépend presque que du rapport oracle/grille

Treize couples (grille, oracle) mesurés, sur des oracles allant de **3200 à 89600** — un facteur
28 — et des rapports `k = oracle/grille` de 2 à 32 :

| `k` | ratio `e / écart` | oracle | verdict du filtre |
|---:|---:|---:|:--|
| 2 | 4,75 | 3200 | refusée |
| 4 | 14,81 | 51200 | refusée |
| 4 | 19,43 | 3200 | refusée |
| 6 | 28,67 | 76800 | **refusée — à 4,4 % du seuil** |
| **7** | **36,72** | **89600** | **admise** |
| 8 | 60,06 | 51200 | admise |
| 8 | 72,34 | 3200 | admise |
| 12 | 115,66 | 76800 | admise |
| 14 | 147,98 | 89600 | admise |
| 16 | 236,29 · 240,79 | 3200 · 51200 | admises |
| 32 | 746,32 · 969,55 | 3200 · 51200 | admises |

Ajustement sur les treize points :

```text
ratio ≈ 2,011 · k^1,902 · o^-0,058        écart maximal du modèle : 23,5 %
```

**La taille de l'oracle ne compte presque pas** — exposant −0,058 — et le rapport des grilles
compte au carré. Ce n'est pas une coïncidence numérique, c'est une identité : pour un schéma
d'ordre `p`, l'erreur d'une grille vaut `e(n) ≈ C·n^-p` et l'écart des deux oracles
`≈ C·o^-p·(1 − 2^-p)`, d'où

```text
ratio = k^p / (1 − 2^-p)
```

soit `1,33·k²` à l'ordre deux — le même exposant que l'ajustement, à un facteur 1,5 près qui
tient à ce que l'écart mesuré est une norme L1 et non une différence signée.

## 2. Ce que le filtre ×30 exige réellement

| Oracle | `k` requis pour franchir 30 | Grille la plus fine admissible |
|---:|---:|---:|
| 25600 | 5,63 | 4 547 |
| 51200 | 5,75 | 8 906 |
| 89600 | 5,85 | 15 323 |
| 179200 | 5,97 | 30 009 |

**Le filtre ×30 équivaut à « l'oracle doit être environ six fois plus fin que la grille la plus
fine ».** La condition est **géométrique** : elle se lit sur un rapport d'entiers, avant tout
calcul. L'historique de C22 s'y range sans exception :

| Campagne | oracle | `k` pour la grille 12800 | résultat observé |
|---|---:|---:|---|
| S48 | 25600 | 2 | refusée |
| S49, S56 | 51200 | 4 | refusée |
| S57 | 76800 | 6 | refusée, à 4,4 % |
| **S59** | **89600** | **7** | **admise, marge 22,4 %** |

**Quatre campagnes et près d'une heure de calcul ont mesuré ce que la suite 2, 4, 6, 7 donnait.**
Le point de bascule était calculable dès S48.

## 3. Pourquoi S60-1 est sans objet

Le régime que l'action prescrivait — `ratio < 1` — demande, d'après le modèle, `k ≈ 1,0`. Or
**l'emboîtement exige `k ≥ 2`** : la grille la plus fine possible est la moitié de l'oracle.

À `k = 2`, le ratio vaut `2,011 · 2^1,902 ≈ 7,5` en modèle, **4,75 mesuré**. La dérive en taille
d'oracle est trop faible pour l'effacer : ramener 4,75 sous 1 par ce seul levier demanderait un
oracle de l'ordre de `10^24` cellules.

> **Le régime « erreur noyée sous l'écart des oracles » n'est pas coûteux : il n'existe pas.** Les
> deux quantités ont la même origine — l'erreur du schéma — et leur rapport est borné en dessous
> par la géométrie de l'emboîtement. **S60-1 est dissoute**, au sens de `REPRISE.md` §8 : la
> question ne reçoit pas de réponse, elle cesse de se poser.

*Correction d'une estimation de cette session même* : le premier chiffrage, écrit dans le plan de
S61, annonçait 1 355 000 cellules et **72,3 h**. Il utilisait l'exposant **local** 1,596, mesuré
entre 76800 et 89600, là où le calage sur toute la plage donne **1,9**. Le premier chiffre était
faux, et la conclusion qu'il annonçait — « hors budget » — était trop douce : c'est *impossible*,
pas *cher*. **Un exposant mesuré sur deux points voisins ne s'extrapole pas sur cinq décades**,
et c'est L175 une seconde fois.

## 4. Ce que cela apprend sur le filtre lui-même

Le filtre est décrit depuis S48 comme *« un indicateur empirique, sans borne prouvée de l'erreur
commune aux oracles »*. C'est vrai de ce qu'il **borne**. Ce ne l'est pas de ce qu'il **exige** :

```text
la grille n est admise  ⟺  (o/n)^p ≥ 30·(1 − 2^-p)
```

**Le seuil d'admission d'une mesure d'ordre est lui-même une fonction de l'ordre.** À l'ordre 2 il
demande `k ≥ 4,7` ; à l'ordre 1, `k ≥ 15` ; à l'ordre 3, `k ≥ 3,2`. Dimensionner la campagne
suppose donc de connaître la réponse qu'elle cherche — et se tromper d'hypothèse ne produit pas
une erreur visible, mais un **« sans verdict »** : exactement l'histoire de C22 de S48 à S57.

C'est **A183**, et ce n'est pas une faute de conception du filtre : c'est une propriété de tout
critère d'admission bâti sur une comparaison d'erreurs. Elle doit être **écrite**, parce qu'elle
change la manière de dimensionner une campagne : on ne choisit pas un oracle, on choisit un `k`,
et on l'assume comme une hypothèse d'ordre.

## 5. Ce qui reste mesurable, et ce qui remplace l'action

L'hypothèse que S60-1 voulait éprouver — *la contamination reste-t-elle additive et uniforme ?* —
**se teste directement**, sans atteindre aucun régime extrême : la colonne « variation » du
rapport, `|e(n vs o1) − e(n vs o2)|`, mesure la contamination grille par grille. Si elle est
constante en `n`, le biais est uniforme.

Elle l'est, et sur toutes les campagnes. En S59, de `nx = 800` à `nx = 12800` — un facteur 16 en
grille, 256 en erreur :

```text
9,842e-11   9,999e-11   9,770e-11   9,792e-11   9,789e-11
```

Plate à 2 % près. Le même plateau apparaît en S56, S57 et jusque dans l'essai à oracle 3200. **La
propriété est donc établie sur le domaine accessible**, et elle n'a jamais eu besoin du régime
inatteignable pour l'être.

Ce qui reste ouvert n'est plus l'uniformité : c'est de savoir **quel `k` on assume**. L'action
**S61-1** porte la conséquence pratique — annoncer, avant de calculer, quelles grilles une
campagne pourra admettre.

## 6. Coût des campagnes futures, calculable d'avance

`o ≈ 6·n`, coût en `o²`, calé sur les 1137,3 s d'oracles de S59 :

| Grille la plus fine visée | Oracle requis | Coût projeté |
|---:|---:|---:|
| 12 800 | 83 200 | 16 min *(S59 en a payé 19 avec 89600)* |
| 25 600 | 166 400 | 65 min |
| 51 200 | 332 800 | 4 h 21 |

## Vérification

Aucun calcul de solveur n'a été lancé pour ce rapport : les treize points proviennent des
campagnes S48/S49, S56, S57, S59 et de l'essai de refus de S60, toutes consignées. Le modèle est
un ajustement par moindres carrés en logarithmes, dont l'écart maximal aux données est donné.
128 tests réussis, deux ignorés ; hashs `check` inchangés ; aucun verdict déplacé.
