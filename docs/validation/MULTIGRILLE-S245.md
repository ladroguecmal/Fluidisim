# Une multigrille pour préconditionner la pression de δ — S245, 2026-09-16

> **Note datée S252, 2026-09-16.** Le gradient conjugué multigrille mesuré ici employait un β
> fautif (A285). Les comptes et coûts « avec » de ce document, la conclusion « la multigrille ne
> gagne pas de vitesse » et l'explication d'A275 par « moins d'itérations » sont invalides. La
> symétrie du cycle, l'opérateur grossier et la comptabilité mémoire restent reçus.
> [Re-mesure](MULTIGRILLE-BETA-S252.md), [ADR-151](../adr/ADR-151-affinage-au-pas-fixe-et-travail-compte.md).

Suite directe de [COUT-DELTA-S244](COUT-DELTA-S244.md), qui a désigné le levier : le coût d'une
itération est **stable en structure** à toutes les tailles — écritures disjointes 67 à 73 % du pas,
réductions 12 à 13 % —, et c'est le **nombre d'itérations** qui double à chaque raffinement :
**30, 61, 114, 220, 425** de 128 à 32 768 mailles. La multigrille est le seul levier dont le gain
**croît avec la taille**, puisqu'elle vise ce nombre et non le coût de chacune.

## 1. Protocole, écrit avant construction

### 1.1 Note corrective sur S244, datée du 2026-09-16

L'en-tête ADR-131 de COUT-DELTA-S244 §6 annonce, parmi les techniques **présentes**, « Jacobi
diagonal ». **C'est faux du chemin mesuré.** `project` pose `jacobi = self.mobile` : le
préconditionnement diagonal n'existe que pour le mode à **surface mobile**. Le chemin à **couvercle
fixe** — celui de S230, de `delta_precision`, de `delta_filters` et de toute la décomposition de
S244 — fait `dir = res` : c'est un gradient conjugué **nu**.

Rien d'autre ne change dans S244 : les mesures, la décomposition et la fermeture du parallélisme
portaient sur ce chemin et restent exactes. Et Jacobi n'y aurait rien apporté — sur une grille
uniforme la diagonale est constante, donc un préconditionnement **scalaire**, sans effet sur le
gradient conjugué, qui est invariant à l'échelle de son préconditionneur.

### 1.2 Ce qui rend ce lot sûr, et ce qu'il exige en retour

**Un préconditionneur ne peut pas rendre la réponse fausse.** Il change les directions de recherche,
jamais le test d'acceptation : le résidu premier de 10⁻⁶ et la tolérance physique
d'[ADR-144](../adr/ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md) restent les
mêmes portes, appliquées au même vrai résidu recalculé. **Le pire cas de ce lot est « ça ne gagne
rien, et c'est mesuré »**, jamais « ça donne un résultat faux ».

En retour, la méthode exige une chose qu'on ne peut pas négliger : **le gradient conjugué n'est
valide qu'avec un préconditionneur symétrique défini positif.** Un cycle multigrille asymétrique
casserait la récurrence, et le symptôme — une convergence erratique — serait facile à confondre avec
un mauvais réglage. La recette retenue assure la symétrie **par construction** (§1.3), et elle sera
**testée** numériquement, jamais supposée.

### 1.3 Décisions de construction, déclarées avant d'écrire

| décision | valeur retenue | provenance |
|---|---|---|
| hiérarchie | division par deux tant que les deux dimensions sont **paires et au moins 4** | aucune interpolation à inventer sur une dimension impaire |
| opérateur grossier | **re-discrétisé** : même stencil à cinq points, `dx` doublé, ouvertures et fractions **moyennées** sur les quatre mailles filles | approximation assumée ; licite puisqu'il ne s'agit que d'un préconditionneur |
| lissage | **Jacobi amorti**, facteur `2/3` | se **dérive** : `2/3` minimise le facteur de lissage des modes hautes fréquences du stencil à cinq points (facteur `1/3`). Ce n'est pas un réglage |
| prolongation `P` | bilinéaire | — |
| restriction `R` | **transposée de `P`** (pondération complète) | c'est elle qui rend le cycle symétrique |
| pré/post-lissage | **autant avant qu'après** | idem : la symétrie du cycle l'exige |
| mode mobile | **inchangé**, garde son Jacobi diagonal | un seul chemin à la fois |
| mémoire | toute la hiérarchie allouée dans `configure`, avant `seal` | I-06 |

### 1.4 Critères de réception, déclarés avant construction

1. **Symétrie et positivité testées** : le produit scalaire de `M⁻¹x` avec `y` égale celui de `x`
   avec `M⁻¹y` à l'arrondi près, et celui de `x` avec `M⁻¹x` est strictement positif, sur des
   vecteurs quelconques et plusieurs tailles.
2. **Compte d'itérations aux cinq tailles de S244**, et la **loi de croissance** qui en sort. C'est
   la revendication : un gain qui ne croîtrait pas avec la taille raterait la cible de S244.
3. **Coût par pas mesuré.** Une itération préconditionnée coûte plus cher ; c'est le **produit** qui
   décide, et lui seul.
4. **Acceptation d'ADR-144 inchangée** : les dix cas de `delta_precision` non dégradés, les ordres de
   `delta_filters` au moins 1,8 (ADR-038 §4), la réception mobile de S237/S238 intacte.
5. **Les bits du chemin fixe changeront**, et c'est attendu : nouvelle empreinte `delta_filters`
   publiée, ancienne conservée, écart expliqué champ par champ (MÉTHODE).
6. **Aucune allocation après `seal`**, vérifiée par l'allocateur compteur du cœur.

### 1.5 Arrêt

Multigrille construite, symétrie prouvée, itérations et coût mesurés, acceptation conservée. **Ou**,
si le lot dépasse la session, la hiérarchie et le cycle validés seuls, et le branchement déclaré en
file avec son déclencheur.

### 1.6 Référence, avant toute modification

Ce sont les chiffres de S244, chemin à couvercle fixe, gradient conjugué **nu**, pas de temps
`1/60 s`, plafond 512, release, un fil :

| mailles | 128 | 512 | 2 048 | 8 192 | 32 768 |
|---|---:|---:|---:|---:|---:|
| itérations | 30 | 61 | 114 | 220 | 425 |
| pas | 0,089 ms | 0,699 ms | 5,256 ms | 37 à 42 ms | **286,2 ms** |

## 2. Ce qui a été construit

| pièce | essai qui la garde |
|---|---|
| hiérarchie : division par deux, géométrie moyennée, diagonale par niveau | dimensions, arrêt de la règle, diagonale positive sur mouillé et nulle sur sec |
| `apply_level`, l'opérateur d'un niveau | rend **les mêmes bits** que `Volume::apply` sur la grille fine, fond plat et fond coupé — l'opérateur est écrit deux fois, et cet essai interdit aux deux écritures de diverger (L137) |
| restriction et prolongation | **adjointes à un facteur quatre près**, constant |
| cycle en V, lissage de Jacobi amorti | **symétrie et positivité vérifiées** ; un cycle réduit le résidu de plus de 10 % |
| branchement | repli : le pas est rejoué avec la multigrille **seulement s'il a été refusé** |

Le cycle rend sa correction dans `prec` et se sert de `tmp` comme résidu fin : les deux sont libres
à cet instant — `prec` ne sert qu'au mode mobile, `tmp` a déjà été consommé par le produit qui
précède. **Aucune allocation** n'est faite hors de `configure`, et le comptage d'I-06 l'inclut.

## 3. Ce que la mesure a donné — et ce n'est pas ce qui était cherché

Même pas que S244, pas de temps `1/60 s`, release, un fil. Colonne « avec » : le cycle **forcé** à
toutes les tailles, pour voir ce qu'il fait.

| mailles | niveaux | sans : itérations / pas / divergence | avec : itérations / pas / divergence |
|---:|---:|---|---|
| 128 | 1 | 30 / 0,088 ms / 6,08e-6 | 99 / 1,94 ms / 1,62e-6 |
| 512 | 2 | 61 / 0,630 ms / 2,60e-6 | 252 / 18,07 ms / 6,90e-6 |
| 2 048 | 3 | 114 / 4,55 ms / 5,14e-6 | 220 / 65,40 ms / 6,54e-6 |
| 8 192 | 4 | 220 / 32,7 ms / 8,10e-6 | 167 / 203,5 ms / 8,08e-6 |
| 32 768 | 5 | **425 / 319 ms / 1,339e-5 — REFUSÉ** | **134 / 729 ms / 8,512e-6 — REÇU** |

### 3.1 Le levier de vitesse ne paie pas

Le cycle coûte cinq produits fins par itération là où le gradient conjugué nu en fait un. Il divise
les itérations par 3,2 **à la plus grande taille seulement** — et les multiplie par trois aux
petites, où la hiérarchie n'a qu'un ou deux niveaux et ne corrige donc presque rien. Le produit est
perdant partout : **la multigrille, telle que construite, ne rend pas δ plus rapide.**

Une contre-épreuve a été faite avant de conclure : hiérarchie plus profonde et cycle allégé (un
lissage au lieu de deux, seize au niveau le plus grossier). Résultat **pire** — 282 / 260 / 292 /
284 / 300 itérations, plates mais hautes. Un compte plat est la signature d'une multigrille qui
fonctionne ; un compte plat **et haut** dit que le taux par cycle est mauvais. Ce n'est donc ni le
niveau grossier ni le lissage qui manquent : c'est le **transfert**, la prolongation étant constante
par morceaux. La suite est nommée au §5.

### 3.2 Le levier de précision, lui, paie — et c'est A275

**À 32 768 mailles, le pas passe de refusé à reçu.** A275 disait, mesure de S239 à l'appui, que f32
ne tenait pas la tolérance d'ADR-144 à cette taille : le certificat d'arrondi arrêtait le solveur à
une divergence de 1,09 à 1,34e-5 pour 1e-5 exigé. La cause n'était pas la taille en soi — c'était
**l'arrondi accumulé sur 425 itérations**. En en demandant **134**, la multigrille laisse la
divergence tomber à **8,512e-6**, et ADR-144 accepte.

Ce n'est pas une tolérance relâchée : c'est le **même** test, appliqué au **même** vrai résidu
recalculé, et il passe.

### 3.3 D'où le branchement retenu : un repli, pas un remplacement

Le pas ordinaire s'exécute d'abord. **S'il est refusé** — et seulement alors — le même pas est
rejoué avec la multigrille. Conséquences mesurées :

- aux tailles qui passaient déjà, **rien ne change, au bit** : 30 / 61 / 114 / 220 itérations, mêmes
  résidus, mêmes divergences, et **empreinte `delta_filters` inchangée : `0xfb12b2092df4ee6d`** ;
- à 32 768 mailles, le pas est **reçu** au lieu d'être refusé, pour 1 006 ms au lieu de 319 — le
  prix du premier essai perdu plus celui du cycle. **On ne le paie que là où l'autre échoue.**

## 4. Réception

| contrôle | résultat |
|---|---|
| suite `code/` complète | **437 réussis, 0 échec, 15 ignorés** |
| empreinte `delta_filters` | **`0xfb12b2092df4ee6d`** — inchangée ; ordres 1,947 / 1,957 / 1,959 |
| `delta_precision`, dix cas | itérations, résidus et divergences **identiques** à S239 |
| symétrie du cycle | vérifiée, deux tailles, fond plat et fond coupé |
| opérateur grossier contre l'opérateur fin | **mêmes bits** |
| allocation | comptée avant d'allouer ; l'essai de comptabilité la **recalcule indépendamment** |
| mode mobile | intouché : le repli est gardé par `!self.mobile`, et le chemin mobile passe `false` |

### 4.1 Coût (ADR-131)

- **Techniques présentes** : préconditionneur multigrille en repli, f32, un fil.
- **Techniques absentes** : prolongation d'ordre deux, opérateur grossier de Galerkin, lisseur autre
  que Jacobi amorti, multigrille **en régime** (elle n'est employée qu'au refus), GPU, parallélisme
  (fermé pour cette boucle par S244), itérations fixes.
- **Domaine** : 8 par 4 m, fond plat, pas de temps `1/60 s`, release, un fil, une machine ; 128 à
  32 768 mailles.
- **Mémoire** : la hiérarchie ajoute 1 156 flottants à une grille 32 par 16, comptés avant
  allocation.

## 5. Ce qui n'est pas reçu, et la suite qu'il désigne

1. **La multigrille ne rend pas δ plus rapide.** Le diagnostic est mesuré, pas supposé : approfondir
   la hiérarchie et alléger le lissage **aggravent**, donc le taux par cycle tient au **transfert**.
   La suite est la **prolongation bilinéaire** avec sa restriction transposée — l'essai d'adjonction
   et l'essai de symétrie sont déjà écrits pour l'accueillir.
2. **Le repli coûte un pas perdu.** Un critère qui prédirait le refus avant de résoudre l'éviterait ;
   aucun n'est mesuré ici.
3. **Le mode mobile ne l'a pas.** Son opérateur a des lignes à fantôme dont le grossissement n'est
   pas écrit.
4. **Une seule machine, un pas de temps, un fond plat au banc de coût** ; les mailles coupées ne
   sont éprouvées que par `delta_filters` et `delta_precision`.
