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

## 8. Relevés

```
cargo run --release --manifest-path code/Cargo.toml -p water-core --example composed_error
```

Bibliothèque inchangée ; workspace **331 réussis / cinq ignorés** en debug et en release.
Bloc 16³, 2744 mailles intérieures, `dt = 10 ms`, 100 pas, 129 instantanés **par réseau** —
2744 nœuds à `r = 1`, 512 à `r = 2`, 125 à `r = 4`, 27 à `r = 8`. Mailles intérieures à
`z ∈ [−4,05 ; −0,80] m`. `max |S| = 1,540547e-4 m/s²`, `max |u'(T)| = 7,993168e-5 m/s`.

Ce véhicule ne mesure **aucune durée** : sa sortie entière est un résultat, et un `diff`
strict entre deux exécutions est vide. C'est une propriété plus forte que celle de S185, qui
publiait trois lignes de durée.

### 8.1 Réceptions — les six passent

1. **Reproductibilité.** Empreinte `0x0e743846d4656870`, et un `diff` strict entre deux
   exécutions est **vide**.
2. **`scatter` à `r = 1` est le chargement direct**, bit pour bit. Le poids d'interpolation
   vaut zéro et `a + 0·x == a` : il n'y a pas de cas particulier à écrire, et il n'y en a pas.
3. **Contrôle croisé avec S185.** `max |S| = 1,540547e-4 m/s²` et
   `max |u'(T)| = 7,993168e-5 m/s` — les valeurs de S185 §6. Et la ligne `r = 1` redonne
   **exactement** les couples `eS/eU` de S185 §6.2 : maintien `c=2` 1,6894 / 0,7700 ;
   maintien `c=64` 48,4130 / 33,2115 ; extrapolation `c=8` 4,7449 / 0,7754 ; interpolation
   `c=64` 12,3586 / 6,7740. Deux véhicules écrits à une session d'intervalle, la même
   référence, les mêmes chiffres : **c'est ce qui autorise à composer les deux mesures.**
4. **À `c = 1` les trois modes rendent le même champ**, en bits, pour chacun des quatre `r`.
   La colonne `c = 1` est donc purement spatiale, comme annoncé.
5. **Plancher.** La référence à `dt/2` sur 200 pas s'écarte de **0,386 %** de `max |u'(T)|` —
   à la décimale la valeur de S185, ce qui est attendu puisque c'est la même référence.
6. **Tout est fini**, source et champ, sur les 84 cases.

**Un déplacement, et sa vérification.** `Mode` et `build_source` vivaient dans
`cadence_error.rs` ; ils sont désormais dans `examples/support/reuse_mode.rs`, parce que S186
réemploie les mêmes trois modes et qu'une seconde copie aurait divergé (L137). `cadence_error`
a été rejoué : **empreinte `0x39567a1d4bc2ba4c` inchangée**, celle que S185 a publiée. Le code
est déplacé, pas réécrit, et ce n'est pas une affirmation mais un relevé.

### 8.2 Le contenu réellement présent : la profondeur filtre

`k_eff` par tranche à l'instant `T0`, sur le réseau plein. `λ_eff = 2π/k_eff`.

| k | z (m) | max abs S (m/s²) | k_eff x | k_eff y | k_eff z | λ_eff z (m) |
|---:|---:|---:|---:|---:|---:|---:|
| 1 | −4,05 | 1,4495e-5 | 0,367 | 0,256 | — | — |
| 2 | −3,80 | 1,5549e-5 | 0,362 | 0,261 | 0,496 | 12,66 |
| 4 | −3,30 | 1,7884e-5 | 0,372 | 0,280 | 0,528 | 11,89 |
| 6 | −2,80 | 2,0589e-5 | 0,382 | 0,305 | 0,573 | 10,97 |
| 8 | −2,30 | 2,3781e-5 | 0,402 | 0,349 | 0,638 | 9,85 |
| 10 | −1,80 | 2,7677e-5 | 0,471 | 0,434 | 0,743 | 8,46 |
| 12 | −1,30 | 3,2630e-5 | 0,587 | 0,553 | 0,963 | 6,52 |
| 13 | −1,05 | 3,5607e-5 | 0,674 | 0,643 | 1,166 | 5,39 |
| 14 | −0,80 | 4,1412e-5 | 0,791 | 0,753 | — | — |

*(table complète — quatorze tranches — dans la sortie du programme ; une ligne sur deux ici.)*

**La branche verticale du §3 est réfutée, et l'autre est confirmée largement.** Le contenu
présent au bloc varie sur `0,37` à `0,79 rad/m` horizontalement et `0,50` à `1,17` verticalement
— soit `λ_eff` de **8 à 17 m** et de **5,4 à 12,7 m** — quand la recette annonce
`λ_min = 1,081 m` (`k_max = 5,81 rad/m`) et que la décroissance verticale `1/k_max` vaudrait
`0,172 m`. **Le contenu réellement présent est 5 à 16 fois plus lisse que la coupure de la
recette.** L'amplitude confirme : le maximum de la source par tranche ne croît que d'un facteur
2,86 de `z = −4,05` à `z = −0,80`, soit une longueur d'atténuation de **3,1 m**, et non 0,172 m.

C'est le filtrage `exp(k z)` qui l'explique : à `z = −0,80 m`, le mode le plus court est déjà
divisé par `exp(5,81 · 0,80)`, environ cent. Ce qui reste, ce sont les modes longs — et `B`,
dont la composante la plus courte fait 14 m.

**Conséquence, et c'est une correction de méthode :** les 2,16 points par longueur d'onde à
`r = 2` annoncés en S184 §5 mesurent la recette, pas la source telle qu'un consommateur en
profondeur la reçoit. Le critère doit porter sur **l'échelle qui survit à la profondeur du
consommateur**. Ici il est **cinq à seize fois trop pessimiste** ; pour un consommateur de
surface il serait juste, et pour un consommateur plus profond encore trop sévère. Un critère
qui donne la bonne réponse par le mauvais chemin se trompera ailleurs (**A230**).

### 8.3 L'erreur spatiale seule, et l'endroit où elle vit

| r | h (m) | h/λ_min | eS % | eU % | eU/(h/λ_min)² |
|---:|---:|---:|---:|---:|---:|
| 1 | 0,25 | 0,2313 | 0 | 0 | — |
| 2 | 0,50 | 0,4625 | 3,5922 | **2,5401** | 0,1187 |
| 4 | 1,00 | 0,9251 | 17,8866 | **13,6043** | 0,1590 |
| 8 | 2,00 | 1,8502 | 27,5989 | **32,9593** | *saturé* |

À `r = 8` l'erreur de champ (32,96 %) **dépasse** l'erreur de source (27,60 %) : l'évolution
n'atténue pas le défaut, elle l'accumule. Et `h/λ_min = 1,85` est au-delà de la limite où une
loi d'ordre deux a cours ; la constante n'y est pas publiée.

**Où l'erreur vit.** `eU` par tranche, en pourcentage du maximum **global** de `u'(T)`, à
`c = 1` :

| k | z (m) | max local de u' , % | r = 2 | r = 4 | r = 8 |
|---:|---:|---:|---:|---:|---:|
| 1 | −4,05 | 34,60 | 0,1124 | 0,4299 | 1,5372 |
| 5 | −3,05 | 46,69 | 0,1791 | 0,6805 | 6,3102 |
| 9 | −2,05 | 64,16 | 0,2981 | 1,1392 | 4,1665 |
| 12 | −1,30 | 83,26 | 1,1622 | 2,8568 | 22,3447 |
| 13 | −1,05 | 91,22 | 1,0506 | 3,6347 | 27,9292 |
| 14 | −0,80 | 100,00 | **2,5401** | **13,6043** | **32,9593** |

**L'erreur globale est exactement celle de la tranche la plus haute**, aux trois `r`. La tranche
du fond ne vaut que 0,11 / 0,43 / 1,54 % — **vingt-trois fois moins à `r = 2`**. Un réseau
isotrope dépense donc la même densité de nœuds là où le contenu est lisse et là où il ne l'est
pas, et c'est la seconde qui fixe le résultat. Un réseau **gradué en profondeur** est la suite
évidente, et elle n'est pas mesurée ici (§7).

### 8.4 H1 : confirmée, au nombre d'axes près

Les deux erreurs sont du second ordre dans un pas sans dimension. Leur **rapport de
constantes** ne dépend donc pas du `λ` choisi pour normaliser : si le contenu effectif est plus
lisse que `λ_min` — et §8.2 montre qu'il l'est d'un facteur 5 à 16 — les deux constantes sont
multipliées par le **même** facteur. C'est ce rapport, et lui seul, qui juge H1.

| r | h/λ_min | A_espace | A_espace / A_temps |
|---:|---:|---:|---:|
| 2 | 0,4625 | 0,1187 | **2,27** |
| 4 | 0,9251 | 0,1590 | **3,05** |

avec `A_temps = 0,0522`, relevé sur les cadences jugées du mode interpolation — le même
opérateur que `scatter` : linéaire entre deux échantillons. S185 mesurait 0,052, stable de
`c = 8` à `c = 32`.

**2,27 et 3,05 : c'est le nombre d'axes interpolés.** `scatter` interpole linéairement sur
trois axes et leurs erreurs s'ajoutent ; le temps n'en a qu'un. Interpoler en espace coûte donc
ce qu'interpoler en temps coûte, **par axe** — et H1 est vraie à ce facteur près. La branche
concurrente du §3, qui prédisait un facteur approchant 39 par la décroissance verticale, est
écartée par un facteur quinze.

L'interprétation par le nombre d'axes repose sur deux points de mesure ; elle est cohérente
avec eux et avec la structure de `scatter`, elle n'est pas démontrée. Ce qui est mesuré, c'est
le rapport.

### 8.5 La composition : le verdict déclaré, puis ce qu'il cachait

**Le critère de §5, appliqué tel qu'il a été déclaré, rejette les trois lois** sur l'ensemble
des cases jugées :

| loi | rapport mesuré/prédit | verdict |
|---|---|---|
| additive | 0,529 – 0,988 | **rejetée** |
| quadratique | 0,749 – 1,209 | **rejetée** |
| maximum | 0,826 – 1,489 | **rejetée** |

Deux choses se lisent déjà là. L'additive n'est **jamais dépassée** — 0,988 au plus fort, sur
les 84 cases : c'est une **enveloppe sûre**, avec jusqu'à 1,9 fois de mou. Et le maximum, lui,
est dépassé jusqu'à 1,489 : il n'en est pas une.

**Séparé par mode de réemploi, le même relevé devient net :**

| mode | additive | quadratique | maximum | loi retenue |
|---|---|---|---|---|
| maintien | 0,529 – 0,976 | 0,749 – 0,999 | 0,826 – 1,155 | **maximum** |
| extrapolation | 0,540 – 0,976 | 0,760 – 0,999 | 0,860 – 1,034 | **maximum** |
| interpolation | 0,803 – 0,988 | 0,991 – 1,209 | 0,998 – 1,489 | **additive, quadratique** |

**La loi de composition dépend du mode de réemploi.** Ce n'est pas une nuance : c'est pourquoi
le verdict global rejetait tout. Et le partage est favorable, parce qu'il l'est du bon côté :

- **Pour les deux modes causaux — les seuls dont un runtime dispose (S185 §1) — la loi est le
  maximum.** Les deux erreurs ne s'ajoutent pas ; la plus grande gagne. **L'axe bon marché est
  donc gratuit jusqu'à la parité avec l'axe dominant**, et le raffiner au-delà n'achète rien.
- Pour l'interpolation, elles se composent quadratiquement (0,991–1,209), c'est-à-dire comme
  deux erreurs indépendantes.

C'est la réponse à la question de §1 : **un budget conjoint `r × c` est licite**, au sens du
maximum, pour un consommateur causal. La règle de dimensionnement qui en découle est
d'**égaliser** les deux erreurs seules, puis de s'arrêter.

Exemple lu dans la grille, maintien, `r = 2` (erreur spatiale seule 2,54 %) : la cadence est
invisible jusqu'à `c = 8` (4,54 % contre 5,05 % pour la cadence seule) et ne devient dominante
qu'à `c = 32`. Le gain de coût entre `c = 1` et `c = 8` est exactement 8 (S184) ; il est obtenu
pour un facteur 1,8 sur l'erreur, et **aucun** si l'on s'arrête à `c = 4`.

### 8.6 Dégrader la cadence peut réduire l'erreur totale

| mode | r | eU(c=1) % | minimum sur c % | c du minimum | gain |
|---|---:|---:|---:|---:|---:|
| maintien | 2 | 2,5401 | 2,2048 | 2 | **−13,20 %** |
| maintien | 4 | 13,6043 | 11,2323 | 8 | **−17,44 %** |
| extrapolation | 2 | 2,5401 | 2,1832 | 8 | **−14,05 %** |
| extrapolation | 4 | 13,6043 | 11,9516 | 16 | **−12,15 %** |
| interpolation | 2 | 2,5401 | 2,5401 | 1 | 0,00 % |
| interpolation | 4 | 13,6043 | 13,5988 | 8 | −0,04 % |

Sur un réseau décimé, **reconstruire moins souvent donne un champ plus juste**, jusqu'à 17 %
de mieux. Les deux erreurs se compensent partiellement, et la compensation appartient aux
**modes causaux** : elle disparaît avec l'interpolation, dont l'erreur temporelle est sept fois
plus petite (S185). Elle est visible dans `eS` seule — interpolation `r = 4` passe de 17,89 %
à 15,16 % à `c = 32` — donc ce n'est pas un artefact de l'évolution du champ, mais bien une
propriété de la source appliquée.

C'est un **piège de réglage**, et c'est l'angle mort de cette session (**A229**) : une
procédure qui balaie un axe en tenant l'autre fixe trouve un optimum, croit avoir réglé, et a
seulement trouvé l'endroit où deux erreurs s'annulent le mieux. La compensation dépend du
contenu, du mode et de la métrique ; elle n'est pas un acquis de conception et ne doit pas être
dépensée.

## 9. Ce que la session conclut, et ce qu'elle laisse ouvert

**Conclu.**

1. **Un budget conjoint est licite pour un consommateur causal**, au sens du maximum. La règle
   est d'égaliser les erreurs des deux axes pris seuls.
2. **L'additive est une enveloppe sûre** dans tous les cas, avec jusqu'à 1,9 fois de mou.
3. **La loi dépend du mode de réemploi** — maximum pour maintien et extrapolation, quadratique
   pour l'interpolation. Un seul chiffre de composition n'existe pas.
4. **Espace et temps sont le même opérateur, par axe** (H1), avec un facteur 2,3 à 3,1 pour les
   trois axes de `scatter`.
5. **Le contenu de la source, vu en profondeur, est 5 à 16 fois plus lisse que la coupure de sa
   recette.** Compter les points par longueur d'onde de la recette est un critère faux — trop
   pessimiste ici, trop optimiste près de la surface.
6. **L'erreur spatiale est intégralement celle de la tranche la plus haute** du bloc.
7. **Dégrader la cadence peut réduire l'erreur** de 12 à 17 % sur les modes causaux.

**Non conclu, et pas contourné.**

- **Aucun seuil de justesse.** A50 n'attend plus un chiffre mais une décision, et elle reste
  entière : rien ici ne dit si 2,5 % est acceptable.
- **Un seul montage, un seul couple d'échelles.** Les formes sans dimension voyagent, les
  valeurs non.
- **Réseau isotrope seulement.** Le réseau gradué en profondeur que §8.3 appelle n'est pas
  mesuré, et `nodes_per_axis` / `scatter` ne savent pas le faire.
- **Aucun coût conjoint annoncé.** Le gain reste le produit des nœuds par la cadence (S184) ;
  la présente session dit seulement qu'on a le droit de le dépenser.
- **Le véhicule ne projette pas**, et l'advection y reste d'ordre supérieur.
- **`A_temps` est relevé sur deux cadences** seulement dans cette grille, faute de cases au-delà
  du plancher et en deçà de la saturation. S185 en avait cinq, concordantes.

**Suite recommandée — S187 : S186-1.** Le réseau **gradué en profondeur**. §8.3 montre que
l'erreur vient d'une seule tranche sur quatorze et que les treize autres sont surrésolues :
un réseau dont le pas suit `1/k_eff(z)` devrait rendre la même erreur pour une fraction des
nœuds. C'est le premier lot où la mesure recommande une **construction**, et non un chiffre de
plus. Il touche `nodes_per_axis` et `scatter`, donc il exige de rejouer S184 et S186 et de
vérifier leurs empreintes, comme S186 l'a fait pour S185.

---

**Suivi S187 — 2026-09-12 : les erreurs spatiales de ce document valent pour un réseau
inutilement mauvais.** Le réseau employé ici pose son dernier nœud **hors** du bloc —
`nodes_per_axis` déborde, et à `r = 8` le dernier nœud vertical tombe à l'indice 17 quand
les mailles intérieures s'arrêtent à 14. La tranche du haut, dont §8.3 montre qu'elle porte
**intégralement** le maximum, était donc interpolée sur 2 m au lieu d'être échantillonnée.
[RESEAU-GRADUE-S187](RESEAU-GRADUE-S187.md) §8.4 mesure le coût de cette convention : à
nombre de nœuds verticaux égal, **41,2 % contre 6,8 %** à trois nœuds, **13,6 % contre
2,5 %** à cinq, **2,54 % contre 1,69 %** à huit. Voir **A231** et
[ADR-118](../adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md).

Rien n'est réécrit : les chiffres de §8.3 et §8.5 mesurent correctement **ce** réseau, et la
réception croisée de S187 les redonne à la décimale. Ce qui change est leur portée :

- la **magnitude** de l'erreur spatiale est jusqu'à six fois plus faible sur un réseau
  ancré, donc la parité entre axe spatial et axe temporel — qui est la règle de
  dimensionnement de §8.5 — se déplace entièrement. L'exemple du maintien à `r = 2`, où la
  cadence devient dominante à `c = 32`, ne tient pas sur un réseau ancré ;
- la **loi** de composition — le maximum pour les modes causaux — a été établie sur une
  erreur **concentrée** sur une tranche. La graduation la **répartit** (S187 §8.5 : erreur
  de tranche haute nulle, maximum déplacé vers le milieu du bloc). Rien ne dit que la loi
  survit à cette redistribution, et **elle n'est pas rejouée**. C'est la suite recommandée.

L'avertissement de §8.2 sur le décompte « points par longueur d'onde » (**A230**) reste
entier et se double : une densité ne dit rien du **placement**, et S187 mesure que le
placement pèse plus.

**Suivi S187 — la compensation entre axes vaut aussi dans l'espace.** §8.6 et A229 ont
trouvé que dégrader la cadence peut réduire l'erreur totale. S187 §8.2 trouve le même
phénomène entre les axes d'espace : l'erreur isotrope est **sous** celle de l'axe vertical
seul aux trois ratios (2,54 contre 2,66 ; 13,60 contre 15,41 ; 32,96 contre 45,51 %).
Le piège de réglage de A229 est donc aussi interne à une seule grandeur.
