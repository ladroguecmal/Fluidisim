# ADR-044 — Ce que l'oracle croisé peut dire, ce qu'il ne peut pas, et les deux choses qu'il a trouvées

- **Statut** : proposée
- **Session** : S37
- **Tranche** : [`ADR-043`](ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) §3 — l'oracle croisé ;
  clôt l'action **S35-3**
- **Corrige** : rien n'est réécrit. `ADR-043` §7.2 reçoit une note corrective datée — l'oracle est
  désormais exercé, et son §3 était trop optimiste sur ce qu'il pouvait dire.
- **Produit** : `code/water-harness/src/oracle.rs` — le comparateur, le **plancher mesuré**, et six
  tests ; la découverte de **deux seuils de sec incompatibles**
- **Clôt** : action **S35-3**
- **Ouvre** : **A163**, **A164**, et une question de conception qui n'appartient pas à cette session

---

## 1. Ce qui a été fait

`ADR-043` §3 promettait un oracle : deux implémentations indépendantes du même modèle, dont le
désaccord sur un cas sans solution analytique désigne une faute d'implémentation. S35 a importé le
second solveur, S36 a monté ses cas — mais **aucun code ne comparait les deux sorties**. C'est fait.

`oracle.rs` fait tourner `delta.rs` et `shallow.rs` sur le **même montage**, au même temps final, et
compare leurs champs cellule à cellule en `L∞` et en `L¹`.

**Les montages n'ont rien demandé.** C01 coïncide rigoureusement des deux côtés — même grille
(centre de cellule à `(i+½)·dx`), même fond `−3 + 0,05·x`, même `η₀`, mêmes 160 cellules, même CFL.
C04 aussi, index à index, malgré des origines qui diffèrent de 20 m. Deux lignées qui s'ignoraient
ont écrit le même montage parce qu'elles lisaient le même `CAS-CANONIQUES`.

## 2. Le plancher, et pourquoi il fallait le mesurer d'abord

**`delta.rs` calcule en `f32`, `shallow.rs` en `f64`.** Aucun document du corpus ne le disait avant
cette session. C'est pourtant ce qui décide de tout ce que l'oracle peut lire : sous un certain
écart, une comparaison croisée mesure l'arithmétique et non le schéma.

Le plancher se **mesure** au lieu de se supposer. C01 au repos a une solution exacte triviale —
`u ≡ 0`, `η ≡ η₀` — donc chaque solveur y a une erreur mesurable contre la vérité, sans oracle
intermédiaire :

| durée | `delta.rs` (`f32`), `max\|u\|` | `shallow.rs` (`f64`), `max\|u\|` | rapport |
|---|---|---|---|
| 1 s | 1,88·10⁻⁶ | 9,48·10⁻¹⁶ | 2·10⁹ |
| 10 s | 3,27·10⁻⁶ | 1,58·10⁻¹⁵ | 2·10⁹ |
| 60 s | **4,40·10⁻⁶** | **2,72·10⁻¹⁵** | 1,6·10⁹ |

**Neuf ordres de grandeur.** Et l'erreur `f32` **croît** avec le temps simulé, comme une marche
d'arrondi, là où l'erreur `f64` reste au niveau de l'ulp.

> **Conséquence pour la conception, et elle n'est pas mince.** « Bien équilibré » n'est pas une
> propriété binaire : en `f32`, elle vaut **4,4 µm/s après une minute**, pas l'arrondi machine. Le
> seuil de C01 est 1 mm/s : `delta.rs` passe avec une marge de **×227**, ce qui est confortable et
> n'était pas connu. Pour un jeu de très grande échelle, `f32` sera vraisemblablement imposé par la
> mémoire et la bande passante — **c'est donc ce chiffre-là qui est le plancher réel de la couche
> `δ`**, et non celui que `shallow.rs` affiche.

## 3. Sur C01, l'oracle croisé n'apprend rien — et il fallait le constater

L'écart croisé mesuré vaut **exactement** le plancher, à chaque durée : 1,8847·10⁻⁶ à 1 s,
3,2695·10⁻⁶ à 10 s, 4,4019·10⁻⁶ à 60 s. Ce n'est pas une coïncidence, c'est mécanique :
`shallow.rs` est si exact que la différence `|u_delta − u_shallow|` se confond avec `|u_delta − 0|`,
c'est-à-dire avec l'erreur de `delta.rs` contre la solution exacte — que C01 mesurait déjà seul.

> **Quand les deux précisions diffèrent de neuf ordres de grandeur, la comparaison croisée dégénère
> en mesure d'erreur du moins précis.** L'oracle n'est symétrique que si les précisions le sont.

Ce n'est pas un échec : `ADR-043` §3 disait que la valeur de l'oracle est sur les cas **sans**
référence analytique. La confrontation le rend concret — sur les cas *avec* référence, il est
redondant.

Un fait accessoire, contraire à ce que le plan de S37 supposait : **les deux véhicules font
exactement le même nombre de pas** sur C01 — 49, 482, 2891. Les `dt_cfl` calculés dans deux
précisions ne divergent jamais assez pour franchir une frontière de pas. Sur C04, ils divergent :
475 contre 454.

## 4. Sur C04, l'oracle sert — et il a fallu aligner les flux pour cela

C04 n'a pas de solution analytique **du schéma** : Ritter est la solution exacte de l'équation, pas
de sa discrétisation. C'est le terrain de l'oracle.

**Un réglage a dû être changé, et c'est le point délicat.** La lignée B monte C04 avec le flux
**HLL**, choisi en B-S22 pour les états secs ; `delta.rs` n'a que **Rusanov**. Comparer les deux
tels quels mesurerait la différence entre deux *flux*, pas entre deux implémentations du même
schéma. `shallow.rs` a donc été remis sur Rusanov, à l'ordre un.

| comparaison | écart `L∞` sur la hauteur | rapporté à `h₀ = 1 m` |
|---|---|---|
| **même flux** (Rusanov des deux côtés) | 6,48·10⁻⁴ m | **0,065 %** |
| flux différents (HLL contre Rusanov) | 1,37·10⁻² m | 1,4 % — **×21** |

**Deux implémentations écrites sans se voir concordent à six pour dix mille sur un front de rupture
de barrage.** C'est le résultat le plus fort qu'ait reçu ce code, et il ne pouvait venir que d'ici.

## 5. Ce que l'oracle a trouvé : deux définitions du mot « sec »

Sur le même montage, à flux égal, **la vitesse diverge de 6,16 m/s sur une cellule** — soit **98 %
de `2c₀`, la vitesse du front de Ritter**, la plus grande que ce montage puisse produire. Un écart
de cette taille n'est pas une divergence de schéma. Son écart *moyen* reste à 7·10⁻² : le désaccord
est concentré sur **trois cellules**.

La cause est un seuil, et il n'est le même dans aucun des deux fichiers :

| | seuil de sec | où il vit | justification |
|---|---|---|---|
| `delta.rs` | `H_SEC = 10⁻⁶ m` | constante publique | `ADR-031` §5 en donne la provenance |
| `shallow.rs` | `10⁻¹⁰ m` | **en dur**, dans `vitesse()` et dans le flux HLL | aucune |

**Quatre ordres de grandeur.** Une cellule dont la hauteur tombe entre les deux est sèche pour l'un
et mouillée pour l'autre — et `hu/h` sur un film pareil rend n'importe quoi.

En écartant les cellules litigieuses, l'écart de vitesse **retombe de 98 % à 1,8 % de `2c₀`**. La
démonstration est un test, pas une affirmation.

### 5.1 Et un second désaccord, sous le premier

À la cellule fautive, `delta.rs` porte **zéro exactement** et `shallow.rs` un film de
**1,05·10⁻¹⁰ m** — un dixième de nanomètre, moins qu'un atome. Ce n'est pas le même défaut que le
précédent : le premier porte sur la **lecture** de la vitesse, celui-ci sur ce que le schéma
**laisse derrière le front**. `shallow.rs` n'écrase jamais ce résidu ; `delta.rs` le remet à zéro.

Les deux lignées avaient pourtant identifié la question. `ADR-031` s'intitule *« une position de
front n'existe pas sans seuil »* ; **A150**, importée de la lignée B, dit *« la position d'un front
numérique dépend du seuil qui la définit »*. **Elles ont trouvé le même problème et choisi deux
réponses différentes, sans le savoir.**

## 6. Les décisions

**D1 — l'oracle croisé s'emploie sur les cas sans référence analytique, et nulle part ailleurs.**
Sur un cas à solution exacte connue, il est redondant avec la référence ; le §3 le montre sur C01.
Son emploi utile est C04, C06, C08 — et tout cas futur dont la solution du schéma est inconnue.

**D2 — toute confrontation aligne d'abord ce qui doit l'être, et le dit.** Flux, ordre du schéma,
nombre de Courant, montage. Un désaccord mesuré sur des réglages différents ne dit rien de
l'implémentation : le §4 chiffre à **×21** ce que le seul changement de flux déplace.

**D3 — le plancher se mesure avant chaque emploi, et il se rapporte avec le résultat.** Un écart
croisé sans son plancher n'est pas interprétable. `oracle.rs` fournit `plancher_c01`, et le principe
vaut pour tout montage : *un instrument non calibré ne mesure rien* (**L131**, **A157**).

**D4 — la précision arithmétique de chaque véhicule entre au corpus.** `f32` pour `delta.rs`, `f64`
pour `shallow.rs`, avec ce que cela implique — §2. C'est une caractéristique de conception, pas un
détail d'implémentation, et elle était absente de tous les documents. Angle mort **A164**.

## 7. Ce que cette session ne décide pas

**Le seuil de sec du projet n'est pas tranché ici, et ne doit pas l'être par un effet de bord.**
Deux valeurs coexistent, `10⁻⁶` et `10⁻¹⁰`, dont une seule a une provenance écrite. Les aligner est
un **choix de conception** : il déplace la position du front, donc le verdict de C04, donc le
critère d'entrée au banc B3 d'`ADR-031`. Une session qui les alignerait au passage changerait une
décision par une retouche de constante.

> **Note S40 — la prudence était bonne, sa raison était fausse.** Ce §7 annonçait qu'aligner les
> deux seuils *déplacerait la position du front, donc le verdict de C04, donc le critère d'entrée au
> banc B3*. Mesuré sur sept décades et deux véhicules : **le front bouge de 0,148 %** au pire, pour
> une tolérance de 3 %. Le verdict de C04 ne bouge pas.
>
> Ce qui justifiait vraiment de ne pas trancher à la légère était qu'une des deux valeurs n'avait
> **aucune provenance** — un défaut d'ignorance, pas d'écart. Le coût annoncé, trop lourd, a fait
> **reporter l'action quatre fois** (**A169**). Voir
> [`ADR-047`](ADR-047-le-seuil-de-sec-ne-decide-de-rien-de-publiable.md).

Ce qui est acquis, et suffit pour aujourd'hui : **les deux valeurs sont incompatibles, l'écart est
mesuré, et sa conséquence est chiffrée.** La question va aux points ouverts (**A163**).

## 8. Ce qui reste ouvert

1. **Le seuil de sec** — §7, et c'est le plus urgent des quatre.
2. **C06 et C08 n'ont pas été confrontés.** Ce sont les deux autres cas sans référence analytique du
   schéma, et l'oracle y est utile par construction.
3. **Le résidu de `10⁻¹⁰ m` derrière le front de `shallow.rs`** n'a pas été expliqué, seulement
   constaté. Est-ce une saturation qui ne sature pas, ou un flux qui laisse passer ? C'est
   exactement ce que S34 appelait une **saturation de modèle** non comptée (**A146**, action
   **S34-1**, reportée trois fois).
4. **L'écart résiduel de 1,8 % de `2c₀`** hors zone litigieuse n'a pas été attribué. Il est de
   l'ordre de la troncature de deux schémas d'ordre un près d'un front, mais rien ne le démontre.
