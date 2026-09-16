# La prolongation de la multigrille de δ — S246, 2026-09-16

> **Note datée S252, 2026-09-16.** Les taux par cycle de ce document emploient le cycle comme
> itération stationnaire : le défaut de β du gradient conjugué (A285) ne les touche pas.
> En revanche, les « itérations forcées » (94/177/158/120/106), le « pas forcé » (579 ms) et le
> « pas réel » (828 ms) ont été mesurés avec ce gradient fautif, et sont invalides.
> [Re-mesure](MULTIGRILLE-BETA-S252.md).

Traite **A280**, ouverte par [MULTIGRILLE-S245](MULTIGRILLE-S245.md) §5. La multigrille y a fermé
A275 en précision, mais **ne gagne pas de vitesse** : un cycle coûte cinq produits fins par
itération, et le taux de réduction par cycle est trop faible pour l'amortir.

## 1. Protocole, écrit avant mesure et construction

### 1.1 Ce que la contre-épreuve de S245 a déjà écarté

Deux suspects ont été éliminés par la mesure, avant celui-ci :

- **le niveau le plus grossier** — approfondir la hiérarchie n'a pas aidé ;
- **le lisseur** — alléger le lissage n'a pas aidé non plus.

Les deux ensemble donnaient **282 à 300 itérations, plates mais hautes**. Un compte plat est la
signature d'une multigrille qui fonctionne ; plat **et haut** dit que son taux par cycle est mauvais.
Reste le **transfert**, et la prolongation y est aujourd'hui **constante par morceaux** : chaque
maille fine reçoit brutalement la valeur de sa mère.

### 1.2 Ce que la géométrie impose

La grille est **centrée sur les mailles**. Le centre d'une maille fine est donc à **un quart** de
l'espacement grossier du centre de sa mère, et non dessus. La prolongation bilinéaire y a les poids
**(3/4, 1/4)** par direction — soit 9/16, 3/16, 3/16, 1/16 en deux dimensions.

Aux bords, le voisin grossier manquant **reporte son poids sur la mère**. L'opérateur reste linéaire,
et c'est sa **transposée exacte**, report compris, qui doit servir de restriction : c'est `R = Pᵀ`
qui rend le cycle symétrique, et une prolongation non triviale est la manière la plus facile de
casser cette propriété sans s'en apercevoir.

### 1.3 Une question ouverte, déclarée comme telle

L'opérateur grossier est **re-discrétisé**, pas construit par Galerkin. Le facteur constant qui
accorde les deux valait `1/4` pour l'injection ; avec une prolongation bilinéaire, **il n'est pas
évident**, et rien ici ne prétend le deviner. Il sera **mesuré**.

Ce n'est pas un risque de correction : un facteur faux **ne casse pas la symétrie** — elle n'en
dépend pas — mais il casse la **réduction du résidu**, et l'essai de S245 le voit. Et de toute
façon, un préconditionneur ne peut pas rendre la réponse fausse : les portes d'ADR-144 sont les
mêmes, appliquées au même vrai résidu recalculé.

### 1.4 L'instrument : le taux par cycle, pas le compte d'itérations

Le compte d'itérations mélange le cycle et le gradient conjugué. Ce qu'on veut savoir tient en un
nombre : **de combien un cycle réduit-il le résidu ?** On l'obtient en employant la multigrille
**seule**, comme solveur : `x ← x + M⁻¹(b − A x)`, et en relevant le rapport de deux résidus
successifs. Un bon cycle pour ce stencil donne 0,1 à 0,3 ; un mauvais s'approche de 1.

C'est ce nombre qui décide, avant et après, et il se mesure sans toucher au gradient conjugué.

### 1.5 Critères de réception, déclarés avant construction

1. **Adjonction exacte** de `R` et `P`, report de bord compris.
2. **Symétrie et positivité** du cycle, conservées.
3. **Taux par cycle** mesuré avant et après — c'est la revendication du lot.
4. **Compte d'itérations et coût par pas** aux cinq tailles, sans et avec.
5. **Acceptation d'ADR-144 inchangée**, et **32 768 mailles toujours reçu** : A275 ne se rouvre pas.
6. Aux tailles qui passent sans repli, **rien ne change au bit** : empreinte `delta_filters`
   `0xfb12b2092df4ee6d`, dix cas de `delta_precision`.

### 1.6 Arrêt

Prolongation bilinéaire reçue et le gain chiffré. **Ou** constat mesuré qu'elle ne suffit pas — et
alors ce document dit ce que le taux vaut désormais et ce qui le plafonne encore, **sans garder un
changement que la mesure ne soutient pas** (L323).

## 2. Le taux par cycle, mesuré avant toute modification

`the_cycle_reduction_rate_s246`, release, fond plat, multigrille employée **seule** comme solveur.

| mailles | niveaux | quatre premiers cycles | taux asymptotique |
|---:|---:|---|---:|
| 2 048 | 3 | 0,0589 · 0,1918 · 0,4512 · 0,3740 | **0,620** |
| 8 192 | 4 | 0,0706 · 0,1966 · 0,4746 · 0,4577 | **0,630** |
| 32 768 | 5 | 0,0605 · 0,1871 · 0,3009 · 0,4436 | **0,725** |

Le diagnostic de S245 est confirmé et chiffré : **0,62 à 0,73 par cycle**, là où ce stencil devrait
donner 0,1 à 0,3. Il faut cinq à sept cycles pour gagner un ordre de grandeur, quand deux devraient
suffire.

**Et le profil dit davantage que le nombre.** Les premiers cycles sont excellents — 0,06, puis
0,19 — et le taux se dégrade ensuite jusqu'à son asymptote. C'est la signature d'une **composante
de l'erreur que le cycle ne réduit pas** : les modes que le lisseur et la correction grossière
traitent bien disparaissent d'abord, et ce qui reste résiste. La prolongation constante par morceaux
est le suspect désigné par la contre-épreuve de S245 ; le traitement des bords en est un second, que
cette mesure ne sépare pas encore du premier.

## 3. La prolongation bilinéaire, construite puis annulée

Elle a été écrite comme le protocole le demandait : poids (3/4, 1/4) par direction, report du voisin
manquant sur la mère au bord, et une **source unique** — un seul gabarit lu par la prolongation pour
rassembler et par la restriction pour disperser, afin qu'elles restent transposées l'une de l'autre,
report compris. L'adjonction et la symétrie du cycle sont restées vérifiées.

**Elle ne gagne rien.** Mesurée deux fois, à l'ancien amortissement et au nouveau :

| mailles | injection | bilinéaire |
|---:|---:|---:|
| 2 048 | 0,637 | 0,648 |
| 8 192 | 0,635 | 0,642 |
| 32 768 | 0,674 | 0,652 |

L'écart est dans le bruit, et il change de signe. **Annulée** (L323) : ce projet ne garde pas un
changement que la mesure ne soutient pas, et un opérateur plus coûteux gardé « parce qu'il est
meilleur en théorie » serait une dette que la session suivante lirait comme une amélioration reçue.

## 4. Ce que la recherche a trouvé à la place : une provenance fausse

En cherchant ce qui plafonne le taux, l'amortissement du lisseur a été revérifié — et il était
**faux, avec une provenance inventée**.

`delta_multigrid.rs` portait `2/3` en écrivant qu'il « se dérive » et « minimise le facteur de
lissage du stencil à cinq points ». **C'est l'optimum à une dimension.** En deux dimensions, un
balayage de Jacobi amorti multiplie le mode `(θx, θz)` par `1 − (ω/2)(2 − cos θx − cos θz)` ; sur les
modes de haute fréquence, les deux extrêmes sont `(π/2, 0)` qui donne `1 − ω/2` et `(π, π)` qui donne
`1 − 2ω`. Les égaler donne **ω = 4/5**, pour un facteur de lissage de **3/5**. Avec `2/3`, le facteur
vaut `2/3` — pire.

C'est exactement ce contre quoi I-14 existe : le nombre n'était pas mauvais, sa **provenance** était
fausse, et une provenance fausse ne se voit que quand on la relit.

**Ce que la correction rend, mesuré.**

| | avec `2/3` | avec `4/5` |
|---|---|---|
| deuxième cycle, 32 768 mailles | 0,187 | **0,113** |
| taux asymptotique, 32 768 mailles | 0,798 | **0,674** |
| itérations forcées, cinq tailles | 99 / 252 / 220 / 167 / 134 | **94 / 177 / 158 / 120 / 106** |
| pas forcé, 32 768 mailles | 729 ms | **579 ms** |
| **pas réel à 32 768 mailles, repli compris** | 1 006 ms | **828 ms** |
| **divergence à 32 768 mailles** | 8,512e-6 | **7,265e-6** |

**A275 reste fermée, avec plus de marge et pour moins cher.**

## 5. Ce que ce lot élimine, et ce qui reste

Le taux asymptotique reste à **0,63–0,67**, là où ce stencil devrait donner 0,1 à 0,3. Quatre
suspects ont été éliminés par la mesure, et c'est le vrai apport de ces deux sessions :

| suspect | verdict | comment |
|---|---|---|
| niveau le plus grossier trop fin | **écarté** | approfondir la hiérarchie aggrave (S245) |
| lissage insuffisant | **écarté** | l'alléger aggrave, l'alourdir ne rend pas le taux (S245) |
| prolongation d'ordre un | **écarté** | la bilinéaire ne change rien, mesuré deux fois (§3) |
| géométrie des mailles coupées | **écarté** | sans aucune coupe, le taux est le même (0,71 / 0,56 / 0,63) |
| amortissement du lisseur | **fautif, corrigé** | provenance 1D employée en 2D ; 0,798 → 0,674 (§4) |

Ce qui reste et que ce lot **ne nomme pas** : le profil — premiers cycles excellents, puis stagnation
— dit qu'une composante de l'erreur échappe au cycle. Les bords en sont le dernier suspect debout :
murs de Neumann sur trois côtés, couvercle de Dirichlet sur le quatrième, et un opérateur grossier
**re-discrétisé** plutôt que construit par Galerkin. La différence entre les deux se voit surtout
là où les coefficients varient — c'est-à-dire aux bords.

## 6. Réception

| contrôle | résultat |
|---|---|
| suite `code/` complète | **437 réussis, 0 échec, 16 ignorés** |
| empreinte `delta_filters` | **`0xfb12b2092df4ee6d`** — inchangée |
| `delta_precision`, dix cas | **identiques** |
| symétrie et positivité du cycle | conservées |
| opérateur grossier contre opérateur fin | **mêmes bits** |
| A275 | **reste fermée**, divergence 7,265e-6 à 32 768 mailles |

Aux tailles qui passent sans repli, **rien ne change** : le repli ne s'y déclenche pas, et
l'amortissement du lisseur n'entre donc dans aucun bit publié.

### 6.1 Coût (ADR-131)

- **Techniques présentes** : multigrille en repli avec lissage de Jacobi amorti à `4/5`, f32, un fil.
- **Techniques absentes** : opérateur grossier de Galerkin, lisseur autre que Jacobi, prolongation
  d'ordre deux (**mesurée sans effet**), GPU, parallélisme, itérations fixes.
- **Domaine** : 8 par 4 m, fond plat et fond coupé, pas de temps `1/60 s`, release, un fil, une
  machine ; 2 048 à 32 768 mailles pour le taux, 128 à 32 768 pour le solveur.
