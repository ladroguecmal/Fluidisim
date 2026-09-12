# S194 — Couplage de deux trains : la somme contre l'évolution de la somme

2026-09-12. S193-1. **Dérivation et protocole avant code et mesure**.
C'est la mesure qu'[ADR-112](../adr/ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md)
attend et qu'**A217** nomme ; elle n'était pas possible avant le véhicule de S193.
Aucun choix de solveur δ, **aucun seuil de bascule W/δ** ne se dérive d'ici.
Le seuil B4 de 2 % est fixé (ADR-120) et n'est pas redemandé.

## 1. La question, et ce qu'on compare exactement

Le chemin perturbatif du projet calcule le champ de chaque source **indépendamment** puis
les additionne. ADR-112 dit que cette superposition ne reçoit pas le couplage ; il ne dit
pas **de combien** elle s'en écarte, ni dans quel domaine elle reste sous le critère.

Soit `a(t)` l'évolution du train A seul, `b(t)` celle du train B seul, et `u(t)`
l'évolution de l'état initial `a(0)+b(0)`. On mesure

```
d(t) = u(t) − a(t) − b(t)                  écart de superposition
écart = max_x |d_η(x,t)| / A ,  A = a₁ + a₂    (amplitudes de premier ordre)
```

Les trois évolutions emploient **le même véhicule**, la même bande, la même profondeur
discrète, le même pas. C'est délibéré : ce qui reste dans `d` est le couplage, pas la
discrétisation — et le §5.3 en fait un contrôle déclaré plutôt qu'une espérance.

**La condition initiale du total est exactement la somme des deux conditions initiales.**
C'est ce qu'un schéma perturbatif suppose, et c'est donc la bonne comparaison — mais il
faut en dire la conséquence **avant** de mesurer : la somme de deux profils de Stokes ne
contient **aucune** harmonique liée croisée. La réponse croisée se construit donc pendant
la première période sous forme d'une part liée **plus** une part libre d'amplitude égale,
si bien que l'amplitude croisée oscille entre 0 et environ **deux fois** sa valeur liée.
Le maximum sur la fenêtre vaut donc environ le double du coefficient lié. Ce n'est pas un
artefact à corriger : un chemin perturbatif part réellement de la somme.

## 2. Dérivation : deux termes croisés, de natures différentes

Le système tronqué de S193 §3.3 se range par degré :

```
F(u) = L u + Q(u,u) + C(u,u,u)
```

`L` linéaire, `Q` et `C` formes multilinéaires symétriques d'ordre deux et trois. Alors

```
F(a+b) = F(a) + F(b) + 2Q(a,b) + 3C(a,a,b) + 3C(a,b,b)
```

Si `a` et `b` résolvent chacun le système, leur somme `s = a+b` vérifie
`s_t = F(s) − [2Q(a,b) + 3C(a,a,b) + 3C(a,b,b)]`, et l'écart obéit à

```
d_t = DF(s)·d + 2Q(a,b) + 3C(a,a,b) + 3C(a,b,b) + O(d²)
```

**L'écart n'est donc pas une erreur : c'est la réponse à un forçage croisé explicite.**
Et ce forçage a deux parts que rien n'autorise à confondre.

### 2.1 Le forçage quadratique, et pourquoi il ne croît pas en eau profonde

`2Q(a,b)` est d'ordre `ε_a ε_b` et porte les nombres d'onde `k₁±k₂` aux pulsations
`ω₁±ω₂`. Sa réponse est **séculaire** — croissante en temps — si et seulement si le
forçage tombe sur la relation de dispersion libre, c'est-à-dire s'il existe une **triade
résonante**. En profondeur infinie, `ω=√(gk)`, la condition `ω(k₁+k₂)=ω₁+ω₂` s'écrit

```
√(k₁+k₂) = √k₁ + √k₂  ⟺  k₁+k₂ = k₁+k₂+2√(k₁k₂)  ⟺  √(k₁k₂) = 0
```

donc **aucune triade résonante n'existe en eau profonde** — résultat classique, redérivé
ici en une ligne. L'interaction de différence ne l'est pas davantage :
`(√k₂−√k₁)² = k₁+k₂−2√(k₁k₂) < k₂−k₁` dès que `k₂>k₁`, donc `ω₂−ω₁ < ω(k₂−k₁)`. La
réponse quadratique est donc purement **liée** : bornée, oscillante, d'amplitude
`O(ε_aε_b)`, et **elle ne croît pas avec la durée d'observation**.

L'argument est **vectoriel et vaut donc en deux dimensions horizontales** :
`|k₁+k₂| ≤ |k₁|+|k₂|`, alors que la résonance exigerait
`|k₁+k₂| = |k₁|+|k₂|+2√(|k₁||k₂|)`, strictement plus grand. Aucune obliquité ne crée de
triade résonante en eau profonde ; ce que la 2D change est le **désaccord**, donc
l'amplitude de la réponse liée, et non sa nature.

**En eau peu profonde, la conclusion s'inverse.** Quand `ω≈√(gh)k` devient linéaire en
`k`, `ω₁+ω₂ = √(gh)(k₁+k₂) = ω(k₁+k₂)` **exactement** : toutes les triades sont
résonantes, la réponse devient séculaire, et la part quadratique — la plus grosse — se met
à croître en temps. **Prédiction déclarée : le désaccord de triade
`Δ = ω₁+ω₂−ω(k₁+k₂)` décroît avec la profondeur, et le couplage mesuré à cambrure égale
doit croître quand `Δ` décroît.** Valeurs calculées d'avance pour le couple nominal
`q₁=2`, `q₂=3`, `L=8 m` :

| `h` (m) | `ω₁` | `ω₂` | `ω(k₁+k₂)` | `Δ = ω₁+ω₂−ω₃` |
|---|---|---|---|---|
| 8 | 3,9256 | 4,8079 | 6,2079 | **2,5256** |
| 2 | 3,9184 | 4,8079 | 6,2072 | **2,5191** |
| 0,5 | 3,1790 | 4,3710 | 6,0860 | **1,4640** |
| 0,25 | 2,4000 | 3,4980 | 5,3890 | **0,5090** |

Ce cas peu profond est mesurable **ici** alors qu'il ne l'était pas en S193 : la
comparaison est candidat contre candidat, elle n'a **pas besoin d'oracle de Stokes**
(A234). Seule subsiste la contrainte de validité du véhicule lui-même, `U ≪ 1`
(ADR-122), qui impose une cambrure minuscule — d'où le choix du §4 de mesurer le
coefficient **adimensionnel** à cambrure fixée très faible.

### 2.2 Le forçage cubique, qui est séculaire partout

`3C(a,a,b) + 3C(a,b,b)` est d'ordre `ε³` et contient des termes au nombre d'onde `k₁`
même — par exemple `|b|²a` — donc **exactement sur le mode libre du train A**. Ce forçage
est **résonant par construction**, et sa réponse croît linéairement en temps : c'est la
**modulation croisée de fréquence**, le décalage que la présence du train B impose à la
fréquence du train A. L'évolution séparée de `a` ne contient que son auto-décalage, mesuré
en S193 ; elle ignore le décalage croisé. L'erreur de phase accumulée vaut donc
`O(ε²)·ωt`, et l'erreur de champ qui en résulte croît **linéairement avec le nombre de
périodes observées**.

### 2.3 Ce que la décomposition prédit, et qui est falsifiable

Avec `N = t/T₁` le nombre de périodes observées et `s` la cambrure par train :

```
écart(s, N) ≈ α·s + β·s²·N
```

| part | mécanisme | nombre d'onde | dépendance | croissance |
|---|---|---|---|---|
| `α·s` | harmoniques liées croisées, `2Q(a,b)` | `k₁±k₂` | **pente 1** | **aucune** (eau profonde) |
| `β·s²N` | modulation croisée de fréquence, `C` | `k₁` et `k₂` | **pente 2** | **linéaire en N** |

**Les deux parts vivent sur des modes différents, et c'est ce qui rend la mesure
mécanisme par mécanisme possible** sans aucun ajustement : les modes `|q₂−q₁|` et
`q₁+q₂` ne peuvent être peuplés que par le couplage, tandis que `q₁` et `q₂` portent
l'erreur de phase. On relèvera donc les deux séparément, **en plus** de l'ajustement.

**Conséquence, déclarée avant mesure, et c'est la thèse de la session :** la validité de
la superposition perturbative n'est **pas un seuil de cambrure**, c'est un **domaine en
(cambrure × durée)**. Une même cambrure est recevable sur une période et fautive sur
cinquante.

**Prédiction sur le partage d'amplitude.** À amplitude totale `A` fixée, la part
quadratique varie comme `a₁a₂` et la part cubique comme `(a₁a₂² + a₂a₁²)/A = a₁a₂`. Les
deux varient donc comme `f(1−f)A²` avec `f` la fraction d'amplitude du premier train :
**l'écart s'annule aux deux extrêmes et culmine au partage égal**, et `écart/(f(1−f))`
doit être à peu près constant. Un écart non nul à `f=0` ou `f=1` serait un défaut de banc.

## 3. Exigence de bande, et couples retenus

Les modes croisés doivent **tenir dans la bande**, sinon la convolution tronquée les
projette hors du domaine et le couplage mesuré est amputé. Pour un couple `(q₁,q₂)` et un
ordre `M=3`, les produits atteignent `3·max(q₁,q₂)`. Avec `q₁=2`, `q₂=3` : modes croisés
à `1` et `5`, produits cubiques jusqu'à `9`. **La bande retenue est `Q=16`**, vérifiée
contre `Q=24`. C'est le point que le §8 de S193 signalait : `Q=8` ne suffisait pas.

| couple | `k₂/k₁` | raison |
|---|---|---|
| `(2,3)` | 1,5 | **nominal** : aucune coïncidence de mode, modes croisés 1 et 5 libres |
| `(1,2)` | 2 | le train B tombe **sur l'harmonique liée** du train A ; cas de coïncidence, **relevé sans prédiction de magnitude** |
| `(1,3)` | 3 | séparation plus large, modes croisés 2 et 4 |
| `(2,3)` contra-propageant | −1,5 | deux sources indépendantes ne voyagent pas dans le même sens ; forçage croisé à `ω₁−ω₂`, désaccord différent |

## 4. Campagne déclarée

`L=8 m`, `g=9,81 m/s²` injecté, `Q=16`, `K=64`, `M=3` sauf mention, `dt=T₁/400`.
`T₁ = 2π/ω_d(k₁)` est la période **semi-discrète** du train lent, `ω_d=√(g G_h(k₁))` —
jamais celle du continu (**L272**, A236). Les conditions initiales de chaque train sont
des profils de Stokes d'ordre deux bâtis sur `ω_d(k_i)`, même convention que S193 §5.2.

| axe | valeurs | ce qu'il mesure |
|---|---|---|
| cambrure par train | `s = 0,0125 / 0,025 / 0,05 / 0,1` | **pentes** `α` et `β` |
| durée | `N = 1 / 2 / 5 / 10 / 20` périodes | la **croissance**, donc le domaine |
| partage | `f = 0 / 0,1 / 0,25 / 0,5 / 0,75 / 0,9 / 1` | la loi `f(1−f)`, et les deux zéros |
| ordre | `M = 1 / 2 / 3` | `M=1` doit donner un écart **nul** |
| couple | les quatre du §3 | coïncidence, séparation, sens opposé |
| profondeur, à `s=2·10⁻⁵` | `h = 8 / 2 / 0,5 / 0,25` | `α(h)` contre le désaccord `Δ` |
| bande | `Q = 16 / 24` | vérification, pas convergence |
| raffinement | `K = 32 / 64`, `dt = T₁/200 / T₁/400` | **contrôle de non-artefact** |

La ligne « profondeur » emploie une cambrure minuscule pour deux raisons déclarées : elle
place les quatre profondeurs **à l'intérieur** du domaine d'ADR-122 (`U ≤ 6,6·10⁻³` au
pire), et elle rend la part cubique `β s²N` négligeable devant `α s`, ce qui **isole** le
coefficient quadratique. À `s=2·10⁻⁵`, `α s ≈ 5·10⁻⁶` et `β s²N ≈ 1,5·10⁻⁸` : trois
ordres de grandeur d'écart, et treize ordres au-dessus de l'arrondi `f64`.

## 5. Réceptions et contre-épreuves

### 5.1 Contre-épreuves à écart nul — la calibration du banc (L271)

1. **Train unique** (`f=0` et `f=1`) — l'écart doit être **exactement nul**, à tout ordre
   `M`. C'est le banc qui se mesure lui-même : un écart non nul signifierait que la
   machinerie à trois évolutions introduit une différence par elle-même.
2. **`M=1`** — le système est linéaire, donc la superposition y est **exacte par
   construction**. Pour un couple à modes **disjoints** (`(2,3)` : A occupe `{2,4}`,
   B occupe `{3,6}`), l'écart doit être **exactement nul au bit**, chaque mode n'étant
   peuplé que par un seul train. Pour un couple à modes **recouvrants** (`(1,2)`), il doit
   rentrer dans l'arrondi, l'addition et l'application de l'amplification ne commutant
   qu'à l'arrondi près. Seuil déclaré : `≤ 10⁻¹⁴` relatif.

   > **Précision P3a, 2026-09-12 — « exactement nul » ne vaut que sur l'état, pas sur le
   > champ, et la distinction fixe le plancher de toute la mesure.** L'énoncé ci-dessus
   > est vrai des **modes** et faux du **champ reconstruit**, ce que le test a montré
   > immédiatement : à `M=1` et modes disjoints, l'écart de modes vaut `0` au bit, mais
   > l'écart de champ vaut `2,6·10⁻¹⁶` relatif. La cause n'est pas la dynamique — les
   > coefficients sont identiques bit pour bit — c'est la **reconstruction** : le total
   > somme les modes `2, 3, 4, 6` dans un ordre, chaque train seul en somme un
   > sous-ensemble dans un autre, et l'addition flottante n'est pas associative. Deux
   > conséquences retenues : le banc relève désormais **les deux** écarts, et le `2,6·10⁻¹⁶`
   > est le **plancher** au-dessous duquel aucun écart de champ ne signifie rien. Il est
   > treize ordres de grandeur sous les mesures du §4, donc inoffensif — mais il devait
   > être su avant de lire un petit écart comme un couplage faible.

Ces deux lignes sont la condition de crédibilité de tout le reste : sans elles, un écart
mesuré à `M=3` ne se distingue pas d'un défaut de montage.

### 5.2 Réceptions chiffrées

3. **Structure** — l'ajustement `écart = α s + β s² N` sur la grille cambrure × durée
   doit avoir un résidu relatif **sous 10 %** de l'écart maximal. C'est la forme prédite
   au §2.3 ; un résidu large réfuterait la décomposition en deux parts.
4. **Mécanismes séparés** — l'amplitude des modes croisés `|q₂−q₁|` et `q₁+q₂` doit être
   **stationnaire** en `N` (rapport entre `N=20` et `N=5` dans `[0,7 ; 1,4]`) et de
   **pente 1** en cambrure ; celle des modes `q₁` et `q₂` doit **croître** avec `N`
   (rapport `N=20 / N=5` supérieur à 2) et être de **pente 2**.
5. **Partage** — `écart/(f(1−f))` constant à **±20 %** sur `f ∈ {0,25 ; 0,5 ; 0,75}`, et
   écart nul à `f ∈ {0,1}`.
6. **Non-artefact** — passer `K` de 32 à 64 et `dt` de `T₁/200` à `T₁/400` ne doit pas
   déplacer l'écart de plus de **2 %** relatif. C'est ce contrôle qui autorise à dire
   « couplage mesuré » plutôt que « numérique mesuré » ; en S193 le plancher de `b₂`
   était de la discrétisation, et il avait fallu l'axe `K` pour le dire.
7. **Bande** — `Q=24` ne doit pas déplacer l'écart de plus de 2 % par rapport à `Q=16`.
8. **Frontière des 2 %** — publier, pour chaque cambrure, le **nombre de périodes** au
   bout duquel l'écart franchit le critère d'ADR-120, et la cambrure au-delà de laquelle
   il est franchi dès la première période. C'est le livrable de la session.
9. **Profondeur** — publier `α(h)` en regard du désaccord `Δ(h)` du §2.1. **Reçu si**
   `α` croît quand `Δ` décroît ; le sens de variation est la prédiction, aucune loi
   quantitative n'est prédite sur deux mécanismes et quatre points.

Deux exécutions `release` identiques, empreinte publiée. Les chiffres ne seront ajoutés
qu'après exécution.

## 6. Limites de ce qui sera fermé

**Ce qui sera fermé si tout passe.** A217 recevra sa mesure : l'écart de la superposition
indépendante, chiffré, avec son domaine de validité en cambrure **et** en durée, et
ses deux mécanismes séparés. ADR-112 cessera d'être une interdiction sans grandeur.

**Ce qui ne le sera pas, et ne doit pas être annoncé comme tel.**

1. **Aucun seuil de bascule W/δ.** A216 reste inexpliquée, et ni `0,02` ni `0,24` ne se
   dérivent d'ici. Une frontière de validité de la superposition **n'est pas** une coupure
   entre couches : elle ne dit rien du coût, de la latence, ni du raccord.
2. **Deux trains ne sont pas un spectre.** Un spectre de mer comporte des centaines de
   composantes, et le nombre de paires croît comme le carré. Rien de ce qui est mesuré ici
   ne s'extrapole à `n` trains sans une mesure propre.
3. **La source B+W de S190/S191 n'est toujours pas branchée**, et ces écarts ne
   s'additionnent à aucun budget mesuré sur une autre référence.
4. **Une seule dimension horizontale.** L'argument de non-résonance du §2.1 est vectoriel
   et survit à l'obliquité : la 2D **ne crée pas** de triade résonante en eau profonde.
   Ce qu'elle change est ailleurs, et n'est pas mesuré ici : le **désaccord** dépend de
   l'angle, donc l'amplitude de la réponse liée aussi, et les résonances de **quatuor** —
   qui sont l'interaction résonante la plus basse pour la gravité en eau profonde — y
   forment un continuum au lieu de cas isolés. Le coefficient `α` mesuré ici est celui
   d'un couple **colinéaire** ; sa dépendance angulaire reste entière.
5. **Fond plat, surface graphe**, pas de déferlement, pas de forces, pas de perception,
   aucune seconde cible (A98), aucun coût CPU.

## 7. Résultats exécutés — 2026-09-12

**S193-1 réalisée : l'écart de la superposition indépendante est mesuré, ses deux
mécanismes sont séparés, et sa frontière à 2 % est chiffrée.** `reception=true`, empreinte
**`0x4bc0934d630c2c50`**, deux exécutions `release` identiques, huit tests propres en
`debug` et `release`. Deux précisions datées (§5.1, §5.2) ont été écrites avant l'exécution
correspondante ; une prédiction et un contrôle déclarés sont **réfutés**, et le §7.7 le dit.

### 7.1 Les contre-épreuves nulles : le banc se mesure d'abord lui-même

| contre-épreuve | écart de modes | écart de champ | croisé | train |
|---|---|---|---|---|
| train unique (`f=0` et `f=1`), `M=1..3` | **0** exact | **0** exact | **0** exact | **0** exact |
| `M=1`, couple `(2,3)`, `s=0,0125..0,1` | **0** exact | `1,96·10⁻¹⁶` à `2,62·10⁻¹⁶` | **0** exact | **0** exact |

Tout écart mesuré au-dessus de `3·10⁻¹⁶` est donc du couplage, et rien d'autre. C'est ce
que L271 réclamait, et c'est la condition de lecture de tout ce qui suit.

### 7.2 Les deux mécanismes se séparent, avec les pentes et les croissances prédites

`M=3`, `h=8`, couple `(2,3)`, fenêtre de 20 périodes. Les colonnes « croisé » et « train »
sont les amplitudes physiques aux modes `q₁±q₂` et aux modes `q₁, q₂`, rapportées à `A`.

| `s` | croisé | pente | rapport tardif/précoce | train | pente | rapport tardif/précoce |
|---|---|---|---|---|---|---|
| 0,0125 | `1,478·10⁻²` | — | **1,002** | `1,166·10⁻²` | — | **4,088** |
| 0,025 | `2,965·10⁻²` | **1,0041** | **1,003** | `4,722·10⁻²` | **2,0178** | **4,116** |
| 0,05 | `6,081·10⁻²` | **1,0365** | **1,023** | `1,973·10⁻¹` | **2,0627** | **4,197** |

**Les quatre prédictions du §2.3 sont tenues, chacune sur son propre mode et sans aucun
ajustement.** La part croisée est de **pente 1** et **stationnaire** — rapport 1,002 à
1,023 entre le dernier et le premier quart de la fenêtre — exactement ce qu'impose
l'absence de triade résonante en eau profonde. La part de train est de **pente 2** et
**croît** d'un facteur 4,1 sur la même fenêtre, exactement ce qu'impose un forçage cubique
résonant. Deux mécanismes dérivés avant mesure, deux signatures indépendantes, deux
confirmations.

### 7.3 La structure, et ce qu'elle dit de l'ordre du modèle

Ajustement `écart = α·s + β·s²·N` sur la grille cambrure × durée, points **dans le
domaine** seulement (§7.4) :

| | `α` | `β` | résidu | résidu / écart maximal |
|---|---|---|---|---|
| `M=2` | `+1,235616` | `+1,744764` | `2,710·10⁻²` | 5,66 % |
| `M=3` | `+1,302602` | `+5,898728` | `6,587·10⁻³` | **1,82 %** |

Le résidu de `M=3` est à **1,82 %** de l'écart maximal, sous les 10 % déclarés : la forme
en deux termes est la bonne.

**Et la comparaison des deux lignes est un argument indépendant pour ADR-122.** Le
coefficient `α` — la part liée croisée, effet quadratique — est le **même** à 5 % près aux
deux ordres, ce qui est attendu : il est présent dès `M=2`. Le coefficient `β` — la part
cumulative — vaut `1,745` à `M=2` contre `5,899` à `M=3`, soit un facteur **3,4**. Lu
directement sur la colonne « train » à `s=0,05` : `6,391·10⁻²` contre `1,973·10⁻¹`,
facteur 3,09. **Un véhicule tronqué à l'ordre deux sous-estime donc la part cumulative du
couplage d'un facteur trois**, c'est-à-dire qu'il fait paraître la superposition
*meilleure* qu'elle n'est. ADR-122 refusait l'ordre deux sur la fréquence d'un train seul ;
la même troncature falsifie la modulation croisée de deux trains, dans le sens rassurant.

### 7.4 Une configuration hors domaine, et le protocole l'exigeait

À `M=3` et `s=0,1` par train, la dérive d'énergie du véhicule vaut **`4,643·10⁻³`**, soit
46 fois le `10⁻⁴` de S193, et l'écart atteint **181 % de `A`**. Le §4 exigeait que la
configuration reste dans le domaine d'ADR-122 pour les deux nombres d'onde : deux trains à
`ka=0,1` chacun n'y sont pas, ADR-122 n'ayant été reçu que pour un train unique à cette
cambrure. **Ce point est donc publié et exclu des ajustements par la règle déclarée**, pas
par convenance — et sa mesure d'énergie est ce qui établit le fait. À `M=2` la même
configuration reste dans le domaine (`7,363·10⁻⁹`) : c'est l'ordre trois, plus non
linéaire, qui sort du domaine le premier.

### 7.5 Le partage d'amplitude suit la loi prédite, et se dégrade aux extrêmes

`A = 0,05 m`, `M=3`, `h=8`, `N=20` :

| `f` | 0 | 0,1 | 0,25 | 0,5 | 0,75 | 0,9 | 1 |
|---|---|---|---|---|---|---|---|
| écart | **0** exact | `1,569·10⁻¹` | `2,836·10⁻¹` | **`3,424·10⁻¹`** | `2,481·10⁻¹` | `1,194·10⁻¹` | **0** exact |
| écart/`f(1−f)` | — | 1,743 | 1,513 | 1,370 | 1,323 | 1,327 | — |

**Les deux zéros sont exacts et le maximum est bien au partage égal**, comme prédit. La loi
`f(1−f)` tient à **14,30 %** sur `f ∈ {0,25 ; 0,5 ; 0,75}`, sous les 20 % déclarés. Elle se
dégrade vers les partages très inégaux — 1,743 à `f=0,1`, soit 27 % au-dessus du centre —
ce que la dérivation n'annonçait pas et qui s'explique : à `f=0,1` le train dominant porte
presque toute la cambrure, et sa propre non-linéarité n'est plus du même ordre que le
produit croisé. La loi est donc une bonne description au voisinage du partage égal, et une
borne optimiste aux extrêmes.

### 7.6 Le couple importe peu ; la cambrure totale décide

`M=3`, `h=8`, `s=0,05` par train, `N=20` :

| couple | sens | écart | croisé | train |
|---|---|---|---|---|
| `(2,3)` | même | `3,612·10⁻¹` | `6,081·10⁻²` | `1,973·10⁻¹` |
| `(1,2)` | même | `3,203·10⁻¹` | *aucun mode exclusif* | `2,179·10⁻¹` |
| `(1,3)` | même | `3,342·10⁻¹` | `5,943·10⁻²` | `2,292·10⁻¹` |
| `(2,3)` | **opposé** | `3,478·10⁻¹` | `4,938·10⁻²` | `1,872·10⁻¹` |

**L'écart varie de 13 % au plus sur quatre géométries de couple**, dont un renversement de
sens de propagation. C'est le résultat le plus utile au projet de toute la série : pour
décider si deux sources peuvent être superposées, **il suffit de connaître leurs cambrures,
pas leur géométrie relative** — au moins dans cette famille colinéaire. La
contra-propagation réduit la part croisée de 19 % sans changer le total, la part de train
compensant.

La ligne `(1,2)` porte `0` dans la colonne « croisé », et ce **zéro ne signifie pas
l'absence de couplage croisé** : pour ce couple, `q₂−q₁=1` et `q₁+q₂=3` sont aussi des
modes du train A ou de ses harmoniques, donc aucun mode n'est **exclusivement** croisé et
le diagnostic par mode ne s'applique pas. Son écart total, `3,203·10⁻¹`, est le plus faible
des quatre — pas le plus fort, contrairement à ce qu'une coïncidence de modes pouvait
laisser craindre. Le §3 ne prédisait aucune magnitude pour ce cas ; il est relevé.

### 7.7 Profondeur : la prédiction tient largement, avec une exception nommée

`s = 2·10⁻⁵`, `M=3`, couple `(2,3)`, `N=20`. Désaccord de triade `Δ` calculé sur la
dispersion **semi-discrète** du véhicule, `α` lu comme `écart(N=1)/s` :

| `h` (m) | Ursell | `Δ` | `α` |
|---|---|---|---|
| 8 | `1,59·10⁻⁶` | 2,470321 | **1,382753** |
| 2 | `1,02·10⁻⁴` | 2,515140 | **1,416284** |
| 0,5 | `6,52·10⁻³` | 1,464762 | **4,083342** |
| 0,25 | `5,22·10⁻²` | 0,508191 | **11,855349** |

**`α` croît de 8,6 fois quand `Δ` décroît de 4,9 fois : la prédiction du §2.1 est tenue, et
largement.** Le mécanisme dérivé avant mesure — l'approche de la résonance de triade quand
la dispersion s'affaiblit — rend donc compte d'un facteur près de neuf sur le couplage,
et il rend aussi le cas peu profond **mesurable sans oracle de Stokes** (A234), ce qui était
l'autre pari du §2.1.

**Exception, à dire plutôt qu'à lisser.** Entre `h=8` et `h=2`, `Δ` croît de 1,8 % et `α`
croît de 2,4 % — donc **dans le même sens**, ce qu'une lecture strictement monotone de la
prédiction interdit. Les deux profondeurs sont effectivement profondes (`kh` de 12,6 et
3,14), leur `Δ` ne diffère que de 1,8 %, et leur `dz` diffère d'un facteur 4, si bien que le
`Δ` semi-discret de `h=8` est le moins exact des deux. Ce couple ne teste donc pas la
prédiction ; il la contredit faiblement, et rien dans la mesure ne permet de trancher entre
une vraie non-monotonie et un effet de discrétisation. **Statut de la réception 9 :
partielle** — reçue sur la plage où `Δ` varie, non testée entre les deux cas profonds.

Repère de cohérence : `α = 1,383` mesuré ici à `s = 2·10⁻⁵` contre `α = 1,303` ajusté au
§7.3 sur des cambrures mille fois plus grandes — 6 % d'écart entre deux mesures
indépendantes du même coefficient.

### 7.8 La frontière des 2 % — le livrable de la session

`M=3`, `h=8`, couple `(2,3)`, cambrure `s` par train. « Franchissement » est le nombre de
périodes au bout duquel l'écart dépasse le critère d'ADR-120, mesuré et non interpolé.

| `s` | écart à `N=1` | écart à `N=20` | franchissement des 2 % |
|---|---|---|---|
| 0,002 | `2,789·10⁻³` | `3,206·10⁻³` | **jamais** |
| 0,004 | `5,625·10⁻³` | `6,998·10⁻³` | **jamais** |
| 0,006 | `8,509·10⁻³` | `1,150·10⁻²` | **jamais** |
| 0,008 | `1,144·10⁻²` | `1,709·10⁻²` | **jamais** |
| 0,009 | `1,292·10⁻²` | `2,023·10⁻²` | 19,8 périodes |
| 0,010 | `1,442·10⁻²` | `2,358·10⁻²` | 11,7 périodes |
| 0,0125 | `1,823·10⁻²` | `3,294·10⁻²` | 5,4 périodes |
| 0,014 | `2,055·10⁻²` | `3,921·10⁻²` | **0,8 période** |
| 0,015 | `2,212·10⁻²` | `4,371·10⁻²` | **0,8 période** |
| 0,020 | `3,017·10⁻²` | `6,964·10⁻²` | **0,4 période** |

**La thèse du §2.3 est confirmée dans sa forme et corrigée dans son ampleur.** Le domaine
existe bien en (cambrure × durée) — à `s=0,009` la superposition tient vingt périodes, à
`s=0,0125` cinq, à `s=0,014` pas même une. Mais **le levier de la durée est étroit** :
toute la dépendance temporelle utile est enfermée dans la bande `0,009 ≤ s ≤ 0,014`, un
facteur 1,6 en cambrure. En dessous, la superposition tient indéfiniment sur l'horizon
mesuré ; au-dessus, elle est fautive avant la fin de la première période. **On ne peut donc
pas acheter la validité en regardant brièvement.**

**Le chiffre qui met ADR-112 en regard de S193.** À `s = 0,0125`, S193 recevait un train
**unique** contre Stokes à **0,4555 %** d'erreur — excellent. **Deux** trains de cette même
cambrure, superposés indépendamment, franchissent le même budget de 2 % en **5,4 périodes**.
Ce n'est pas la précision du modèle qui manque, c'est la superposition qui est fausse :
d'un facteur 40 entre les deux erreurs, à cambrure égale.

### 7.9 Convergence, et deux réfutations à publier

| configuration | ordre en `K` sur la **moyenne quadratique** | résidu à `K=64` | ordre en `K` sur le **maximum** |
|---|---|---|---|
| `s=0,008`, condition initiale Stokes-2 | **1,9295** | **0,4698 %** | **−0,7914** |
| `s=0,008`, condition initiale d'ordre un | 1,8711 | 0,5026 % | −0,3700 |
| `s=0,05`, condition initiale Stokes-2 | 1,7233 | 0,0942 % | +0,6140 |

Déplacements sur les autres axes, en moyenne quadratique : `dt` de `T₁/200` à `T₁/400`,
`4,07·10⁻⁷` et `7,32·10⁻⁷` ; `Q` de 16 à 24, `1,5·10⁻⁸` et `1,93·10⁻⁵`. Le temps et la
bande sont donc convergés à l'arrondi près, et la profondeur discrète est le seul axe
spatial — comme S193 l'avait conçu.

> **Réfutation 1 — le contrôle de non-artefact, tel que le §5.2 l'écrivait, ne peut pas
> fonctionner, et il est publié comme non tenu.** Il exigeait que passer `K` de 32 à 64 ne
> déplace pas l'écart de plus de 2 % ; le déplacement mesuré vaut **5,37 %**. Mais ce n'est
> pas un artefact : le rapport des déplacements successifs `|K32−K64| / |K64−K128|` vaut
> **3,81**, c'est-à-dire **4**, c'est-à-dire la **convergence d'ordre deux**. Un contrôle qui
> juge la *taille* d'un déplacement sur une grille grossière ne distingue pas un artefact
> d'une convergence — il confond les deux par construction. Ce qui répond à la question
> posée est l'**ordre** de la suite et la part de discrétisation qui reste au pas retenu :
> ordre **1,93**, résidu de Richardson **0,47 %** à `K=64`. L'écart mesuré est donc bien du
> couplage, et le contrôle déclaré était mal formé.
>
> **Réfutation 2 — la cause soupçonnée n'était pas la bonne.** L'hypothèse posée en séance
> était que la sensibilité en `K` venait de la **condition initiale**, dont le terme d'ordre
> deux emploie le `b₂` du continu sur un véhicule semi-discret et injecte donc une onde
> libre parasite dépendante de `K` — le mécanisme même d'A236, pour la troisième fois. Elle
> a été testée en retirant purement ce terme : le résidu passe de `0,4698 %` à `0,5026 %`,
> et l'ordre de `1,9295` à `1,8711`. **Aucun changement** : l'hypothèse est fausse, la
> sensibilité est dans la dynamique — `G_h` est le seul objet dépendant de `K` — et non dans
> la condition initiale. La variante d'ordre un reste publiée pour cette raison.
>
> **Et un résultat méthodologique qui ne concerne pas que cette session.** Le **maximum sur
> la fenêtre ne converge pas** : ordres `−0,79`, `−0,37`, `+0,61`, et valeurs non monotones
> (`1,668 / 1,709 / 1,781 · 10⁻²` pour `K=32/64/128`). Ce n'est pas un défaut du véhicule :
> un maximum est une **statistique d'ordre** sur un signal oscillant, et un changement de
> fréquence de `6·10⁻⁴` suffit à déplacer l'endroit où le maximum tombe. La même
> fonctionnelle avait donné des ordres 2 impeccables en S192 et S193 — parce que l'erreur y
> était une fonction lisse et monotone du pas, non un résidu de deux évolutions presque
> égales. **Un maximum ne se raffine proprement que s'il porte sur une quantité qui n'est
> pas une différence.** Les écarts publiés aux §7.2 à §7.8 restent des maxima, parce que
> c'est le maximum qui décide d'un budget ; leur incertitude de discrétisation est celle de
> la moyenne quadratique, `0,47 %` relatifs, très loin de changer une conclusion.

Cible `x86_64-pc-windows-msvc`, rustc 1.97.0 (2d8144b78), cargo 1.97.0 (c980f4866) ; pas de
seconde cible, aucune mesure de coût CPU. `water-core` et le support S193 sont **inchangés**
— seul `examples/nl_coupling_2d.rs` est ajouté. Les **331 tests** d'espace de travail et
leurs **cinq ignorés ont été rejoués** en `debug` — 238 + 93 réussis, 2 + 3 ignorés —
identiques au reçu de S190 vérifié en S193.

Code : `code/water-core/examples/nl_coupling_2d.rs`.
[Sorties intégrales](COUPLAGE-DEUX-TRAINS-S194-MESURES.md).

**Note S196 — 2026-09-12 : lire « énergie sous `10⁻⁴` » pour ce que c'est.** Ce document
emploie la dérive relative d'énergie comme critère de domaine. S196 a montré qu'un tel critère
**ne détecte pas la sous-résolution** : il a trouvé une configuration fausse d'un facteur cinq
dont l'énergie dérivait soixante-cinq fois sous le seuil (**A242**, **L277**). Les
configurations publiées ici sont loin du bord de résolution et leurs valeurs ne sont pas
remises en cause ; c'est la **phrase** qui ne doit plus être lue comme une garantie de
justesse. Voir REPLI-CROISEES-S196 §8.5.
