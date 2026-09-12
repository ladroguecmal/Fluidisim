# S186 — Composer l'erreur spatiale et l'erreur temporelle

2026-09-12. S185-1 / A50. **Mesure locale sur un montage, aucun seuil de justesse adopté.**

[CONSOMMATION-S184](CONSOMMATION-S184.md) a chiffré ce que coûte la source, et montré que la
décimation spatiale achète exactement le rapport des nœuds et la cadence exactement `c`.
[CADENCE-3D-S185](CADENCE-3D-S185.md) a chiffré ce que la **cadence** coûte en justesse.
Il manque la moitié qui décide : les deux réductions se cumulent-elles, et **comment** ?

Comme en S183, S184 et S185, les conditions sont publiées **avant** la première exécution
(§1–§7) ; les relevés viennent après (§8).

## 1. L'écart précis que cette session ferme

Les deux axes de réduction ont été mesurés séparément, et **sur deux véhicules différents** :

| axe | où | véhicule | ce qui manque |
|---|---|---|---|
| **espace** — réseau de pas `h` | [SOURCE-DECIMEE-S170](SOURCE-DECIMEE-S170.md) | Saint-Venant **1D**, source **figée** | la 3D, un contenu qui bouge |
| **temps** — cadence `τ` | [CADENCE-3D-S185](CADENCE-3D-S185.md) | bloc **3D**, réseau **plein** | la décimation |

Personne n'a jamais mesuré les deux **ensemble**, et S170 a laissé un avertissement qui
interdit précisément de les additionner de tête :

> *« Un ratio de décimation ne décrit donc pas à lui seul la précision : la taille physique
> du réseau devant la variation de `S` compte. »*
> — [SOURCE-DECIMEE-S170](SOURCE-DECIMEE-S170.md) §2.2

Un budget conjoint `r × c` — qui est **le** produit que S184 rend tentant, puisque le gain de
coût y est le produit `r³ · c` — n'a de sens que si l'on sait composer les erreurs. Trois
réponses sont possibles, et **elles n'ont pas les mêmes conséquences** : si les erreurs
s'ajoutent, le budget se partage ; si elles se composent quadratiquement, l'axe dominant
emporte tout et l'autre est gratuit jusqu'à parité ; si elles interagissent, aucun budget
séparé n'existe.

## 2. Le véhicule, et l'unique référence

Le bloc de S184 et S185, **sans un octet de différence** : `examples/support/`, côté 16, 2744
mailles intérieures, `dx = 0,25 m`, `dt = 10 ms`, 100 pas, `T0 = 1,5 s`, même montage d'eau,
même pas (advection centrée, laplacien à sept points, `− S` soustraite, SPEC-004 §6.1). Le
solveur du projet reste à B3 (ADR-007 §5) : ceci est un véhicule d'essai.

Le réseau et son interpolation ne sont pas écrits non plus : `lattice_points`,
`nodes_per_axis` et `scatter` (trilinéaire, coin supérieur borné) sont ceux dont S184 a mesuré
le **coût**. **S186 les mesure en erreur, sans en écrire un second** (L137).

**Une seule référence pour tout le tableau** : `r = 1`, `c = 1`, source reconstruite à chaque
pas sur le réseau plein et chargée sans interpolation. C'est exactement la référence de S185.
Deux références rendraient la composition dénuée de sens — chaque case se comparerait à autre
chose qu'une autre.

**L'ordre des deux réductions.** Le runtime conserve des instantanés **de nœuds** : il réemploie
en temps ce qu'il a échantillonné en espace. Le véhicule fait donc, dans cet ordre, réemploi
temporel **sur les valeurs de nœuds**, puis `scatter` vers les mailles. Les deux opérateurs sont
linéaires et commutent en exact ; ils ne commutent pas en flottant, et c'est l'ordre du runtime
qui est retenu, pas celui qui arrangerait la mesure.

La grille : `r ∈ {1, 2, 4, 8}` (pas de réseau 0,25 / 0,5 / 1 / 2 m) × `c ∈ {1, 2, 4, 8, 16,
32, 64}` (maintien de 10 ms à 640 ms) × les trois modes de réemploi de S185 — maintien,
extrapolation causale, interpolation à une période de latence. Soit 84 évolutions.

## 3. Les échelles du contenu, et l'hypothèse qu'elles imposent

Les échelles sont **calculées par le programme** depuis la recette, jamais posées à la main
(I-14) :

| contenu | échelle spatiale | échelle temporelle |
|---|---|---|
| composantes de `B`, périodes `Tp/2` à `2·Tp` | `λ` de 14,05 à 224,8 m | 3 à 12 s |
| onde d'impact, `λ = 4 m` | 4 m | ≈ 1,6 s |
| mode de pression le plus court | `λ_min = 1,081 m` | **`T = 0,5405 s`** (advecté à 2 m/s) |

**C'est la même couche — la pression — qui fixe les deux échelles.** D'où l'hypothèse que
cette session met à l'épreuve, et qui est **falsifiable** :

> **H1 — les deux erreurs sont le même opérateur.** Pour un contenu advecté à `v`, interpoler
> linéairement entre deux nœuds distants de `h` et interpoler linéairement entre deux instants
> distants de `τ` agissent sur la même structure : `h/λ_min` et `v·τ/λ_min = τ/T` sont le
> **même** nombre sans dimension. Si H1 est vraie, l'erreur spatiale doit être d'**ordre deux**
> en `h/λ_min`, avec une constante proche du `0,05` mesuré en S185 pour l'interpolation
> temporelle — et alors un seul nombre gouverne les deux axes.

H1 a une raison identifiée d'être fausse, et elle est écrite **avant** la mesure : un mode de
pression profond décroît en `exp(k z)`, donc sa longueur de variation **verticale** est
`1/k = λ_min/2π = 0,172 m`, soit `2π` fois plus courte que sa longueur d'onde horizontale. Un
réseau isotrope de pas `h` ne voit donc pas la même chose sur les trois axes, alors que le
temps ne voit que l'advection horizontale. Deux conséquences opposées, toutes deux plausibles :

- l'axe vertical est le plus sévère, et l'erreur spatiale **dépasse** H1 d'un facteur qui se
  rapproche de `(2π)² ≈ 39` ;
- ou bien la profondeur **filtre** le contenu court avant qu'il n'atteigne le bloc — les
  mailles intérieures sont à `z ∈ [−4,05 ; −0,80] m`, et `exp(k_max·z)` y vaut au plus `1e-2`
  — si bien que le contenu réellement présent est **beaucoup plus lisse** que `λ_min`, et que
  l'erreur spatiale est **inférieure** à ce que « 2,16 points par longueur d'onde »
  (S184 §5) laisse craindre.

La seconde possibilité disqualifierait un critère qui circule depuis S184 : **« points par
longueur d'onde » n'est pas un critère valide pour une source 3D échantillonnée en
profondeur**, parce que la longueur d'onde qui compte n'est pas celle de la recette mais celle
qui survit à la profondeur du consommateur. C'est mesurable, donc c'est mesuré.

**Le contenu effectif est donc relevé, pas déduit** : pour chaque axe et chaque tranche de
profondeur, le programme calcule un nombre d'onde effectif
`k_eff = √( max|∂²S/∂x²| / max|S| )` par différences finies sur le réseau plein. C'est un
estimateur grossier, et il est annoncé comme tel ; sa seule fonction est de dire **quelle
échelle est réellement présente là où le bloc se trouve**.

## 4. Métriques

État initial `u' = 0`, comme en S185, pour que `u'(T)` soit entièrement ce que la source a
produit. Deux grandeurs, identiques à celles de S185 :

- **erreur de source** `eS = max |S_utilisée − S_réf|` sur toutes les mailles et tous les pas,
  rapportée à `max |S_réf|` ;
- **erreur de champ** `eU = max |u'(T) − u'_réf(T)|` sur les mailles intérieures, rapportée à
  `max |u'_réf(T)|`.

Plus, spécifique à cette session, **l'erreur par tranche de profondeur** : `eU` restreinte à
chaque plan `k` du bloc, pour l'axe spatial seul. C'est elle qui tranche entre les deux
branches du §3.

## 5. Les trois lois de composition, et comment elles sont jugées

Pour chaque case `(r, c, mode)`, l'erreur mesurée `eU(r, c)` est comparée aux erreurs des deux
axes pris seuls — `eU(r, 1)`, purement spatiale, et `eU(1, c)`, purement temporelle, qui doit
redonner S185 :

| loi | prédiction | ce qu'elle impliquerait |
|---|---|---|
| **additive** | `eU(r,1) + eU(1,c)` | les erreurs se partagent un budget ; réduire l'un des deux axes paie toujours |
| **quadratique** | `√(eU(r,1)² + eU(1,c)²)` | erreurs indépendantes ; l'axe dominant emporte tout, l'autre est **gratuit** jusqu'à parité |
| **maximum** | `max(eU(r,1), eU(1,c))` | saturation : un seul axe compte à la fois |

Le rapport `mesuré / prédit` est publié pour les trois, sur toute la grille. Le critère de
jugement est déclaré ici, avant d'avoir vu les chiffres : **une loi est retenue si son rapport
reste dans `[0,8 ; 1,25]` sur toutes les cases situées au-dessus du plancher de référence** ;
sinon elle est rejetée, et si les trois sont rejetées la conclusion est qu'**aucun budget
séparé n'existe sur ce montage** — ce qui est un résultat, pas un échec.

Les cases sous le plancher de la référence (0,386 % de `max |u'(T)|`, S185 §6.1) ne jugent
rien : deux bruits ne composent pas. Elles sont marquées et exclues du verdict.

## 6. Réceptions exigées avant tout chiffre

1. **Reproductibilité en bits.** L'erreur mesurée est déterministe. Deux exécutions doivent
   rendre les mêmes bits ; une empreinte est publiée.
2. **`(r = 1, c = 1)` est la référence, littéralement.** `scatter` à `r = 1` doit rendre les
   valeurs de nœuds **telles quelles** — le poids d'interpolation vaut zéro et `a + 0·x == a` —
   donc le champ doit être identique **bit pour bit** à celui chargé sans interpolation.
3. **La ligne `r = 1` doit redonner S185.** `max |S|`, `max |u'(T)|` et les `eU` de chaque mode
   doivent retomber sur les valeurs publiées en [CADENCE-3D-S185](CADENCE-3D-S185.md) §6.2.
   C'est un contrôle **croisé entre sessions** : il éprouve à la fois ce véhicule et le
   précédent, et c'est la seule raison pour laquelle les deux mesures peuvent se composer.
4. **La colonne `c = 1` est purement spatiale**, pour les trois modes indifféremment : à
   `c = 1` le réemploi n'existe pas, donc les trois modes doivent rendre le **même** champ, en
   bits, pour un `r` donné.
5. **La référence est assez convergée.** Le même calcul à `dt/2` sur 200 pas ; l'écart est
   publié et sert de plancher.
6. **Tout reste fini**, source et champ, sur les 84 cases.

## 7. Ce que la mesure ne prouvera pas

- **Aucun seuil de justesse.** Rien ici ne dit si 1 % est acceptable. C'est un critère
  perceptuel ou B4, et aucun n'est adopté. A50 attend une **décision**, pas un chiffre de plus.
- **Un seul montage**, donc un seul couple d'échelles. Les lois sont données sous forme sans
  dimension pour cette raison ; elles n'ont été vérifiées que sur ce contenu.
- **Un seul réseau, isotrope.** Un réseau anisotrope — fin en profondeur, grossier à
  l'horizontale — est la suite évidente si le §3 penche du côté vertical, et il n'est **pas**
  mesuré ici.
- **Aucun coût.** Le gain de coût est celui de S184 ; cette session ne le remesure pas et
  n'annonce aucun budget conjoint. Elle dit seulement si un tel budget est licite.
- **Le véhicule ne projette pas** et ne modélise ni surface libre ni bord (S184 §3).
- **L'interpolation temporelle n'est pas proposée** : elle exige une période de latence, et
  reste mesurée comme plafond (S185 §1).
