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
