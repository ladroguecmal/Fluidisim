# La prolongation de la multigrille de δ — S246, 2026-09-16

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
