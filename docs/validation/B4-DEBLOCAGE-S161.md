# S161 — B4 : ce qui bloque, et ce qui ne bloque pas

2026-09-10. Instruit le déblocage de **B4**, désigné par ADR-109 comme seule voie ouverte et
attendu seul par **A214**. Protocole : [PLAN-BENCHMARK](PLAN-BENCHMARK.md) §B4.

## 1. Ce que B4 demande, volet par volet

> Même scène simulée deux fois : (a) perturbative B+W+δ, (b) **substitutive intégrale de
> référence, à résolution élevée**. Comparaison de la surface, des forces sur la coque, et de la
> perception en double aveugle.

| volet | état |
|---|---|
| comparaison de **surface** | demande la référence intégrale, rien d'autre |
| comparaison des **forces sur la coque** | demande la référence **et** un intégrateur de corps rigide, que le corpus range en construction (BILAN-S69 §5.3) |
| **perception en double aveugle** | demande des personnes — hors de portée d'une session (`REPRISE.md` §5) |
| contrôle du **terme source** (ajout S04, A50) | demande la référence, et un `S` que l'on sait tronquer |

**Un seul obstacle est commun aux quatre : la référence.** C'est bien elle qu'il faut instruire, et
c'est ce que le corpus dit depuis S158 sans l'avoir ouverte.

## 2. Ce que la référence doit être — et ce qu'elle ne peut pas être

Elle doit calculer **le champ total** dans son emprise, sans décomposition, pour qu'on puisse
comparer `simuler(A) + simuler(B)` à `simuler(A et B ensemble)`.

**Un solveur linéaire ne peut pas servir.** `dispersif.rs` — le milieu à dispersion exacte de S39,
instrument qui a rétracté ADR-042 — est **linéaire** : la superposition y est vraie par
construction, et l'écart mesuré serait nul quel que soit le rapport d'amplitude. Il mesurerait la
justesse de son intégration, pas la validité de l'additivité. C'est le piège le plus proche, parce
que ce fichier est le plus « référence » d'apparence du dépôt.

**Le candidat est donc un solveur non linéaire**, et le dépôt en a un : `shallow.rs`, Saint-Venant
1D en `f64`, bien équilibré, flux HLL, second ordre et RK2 optionnels, 1 165 lignes, reçu par le
mode `physics` du harnais sur six montages canoniques et par l'oracle croisé de S37. Sa
non-linéarité est celle qui compte ici : advection et terme `h·u`.

## 3. Ce que le dépôt possède, mis à plat

| pièce | nature | utilisable comme référence B4 ? |
|---|---|---|
| `shallow.rs` | Saint-Venant 1D, **non linéaire**, f64 | **oui**, en 1D |
| `delta.rs` | même modèle, f32, véhicule d'essai | comme témoin croisé, pas comme référence |
| `dispersif.rs` | milieu **linéaire** à dispersion exacte | **non** — additif par construction |
| `background.rs` | houle analytique de Gerstner | c'est le `B` à tester, pas la référence |
| oracle croisé (S37) | confronte `delta` et `shallow` | méthode déjà écrite, réutilisable |

**Le blocage n'est donc pas « il n'y a pas de référence ».** Il est que la référence disponible est
**1D et non dispersive**, quand `B` est une houle dispersive 2D. Ce que cela autorise et interdit
est la question de la section suivante — et c'est une question de conception, pas d'outillage.

## 4. Ce qu'un B4 en une dimension peut rendre, et ce qu'il ne peut pas

**Ce qu'il peut rendre : une infirmation.** L'additivité est une propriété mathématique ; il suffit
d'**un** contre-exemple pour la borner. Si `simuler(A) + simuler(B)` s'écarte de
`simuler(A et B)` au-delà d'un rapport d'amplitude donné, dans un modèle non linéaire d'eau, alors
la décomposition additive a un domaine de validité et ce rapport en donne une borne — dans ce
régime.

**Ce qu'il ne peut pas rendre : une validation.** L'absence de rupture en 1D ne dit rien du 2D
dispersif, ni des forces sur une coque, ni de la visibilité. Un banc qui ne peut qu'infirmer garde
sa valeur — c'est même la forme la plus honnête d'un juge — mais il ne remplace pas B4, qui doit
pouvoir infirmer **l'architecture**, pas seulement l'additivité d'un modèle.

**Le régime testé n'est pas quelconque.** Saint-Venant est non dispersif et vaut en eau peu
profonde ; c'est **exactement** le régime qu'ADR-001 §3.3 nomme pour justifier la bascule
substitutive : « rouleau de déferlement, piscine, coque qui émerge entièrement, cavité
traversante ». Le contre-exemple, s'il existe, tombera donc dans le domaine que la décision vise —
pas à côté.

**Ce qui reste hors d'atteinte, et qu'il faudra dire à chaque fois** : la houle de `B` est
dispersive en eau profonde ; un écart mesuré en eau peu profonde ne se transporte pas tel quel.

### La question posée, et la forme de la réponse

> À partir de quel rapport `A_δ / A_B` l'écart entre l'addition et la simulation conjointe dépasse
> un seuil donné, dans un modèle d'eau non linéaire ?

ADR-001 propose `max|δ| > 0,35·Hs` **comme valeur de départ, à calibrer**. La mesure la confrontera
directement. Et parce que « visiblement fausse » est perceptuel et que nous ne jugeons pas de
visibilité, la réponse sera rendue **à plusieurs seuils** — 1 %, 5 %, 10 %, 25 % — plutôt qu'en un
nombre unique qui cacherait le seuil choisi (**L242**).

*Une réserve de plan d'expérience, posée avant la mesure* : dans Saint-Venant, la non-linéarité
dépend aussi de l'amplitude absolue rapportée à la profondeur, `a/h0`. Le rapport `A_δ/A_B` seul ne
peut donc pas gouverner l'écart ; les deux doivent varier séparément, sans quoi le plan est
dégénéré comme celui de S157 (**L235**).
