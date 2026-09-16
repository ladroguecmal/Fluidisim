# Une multigrille pour préconditionner la pression de δ — S245, 2026-09-16

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
