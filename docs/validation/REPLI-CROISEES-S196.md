# S196 — Le repli des harmoniques croisées : cause ou coïncidence ?

2026-09-12. **A241**, ouverte par S195. **Protocole et prédictions avant tout code et toute
mesure.** Aucun choix de solveur δ, aucun seuil de bascule W/δ, aucun ADR attendu : le seuil
B4 de 2 % est fixé (ADR-120) et n'est pas redemandé.

## 1. Ce que S195 a laissé, et pourquoi ce n'est pas suffisant

[S195](SOURCES-MULTIPLES-S195.md) a mesuré `n` sources et clos A240. Mais il a fermé une
question en en ouvrant une autre, et celle-ci est plus gênante : **la loi mesurée ne coïncide
avec aucune des deux lois dérivées.** À cambrure totale fixée, l'exposant mesuré vaut `−0,44`
là où l'addition dispersée prédit `−0,77` sur la même plage et l'addition cohérente `+0,46`.

S195 a proposé un mécanisme, et l'a chiffré sans le démontrer : les harmoniques croisées
`kᵢ±kⱼ` **retombent en partie sur des modes de train**. La part qui y retombe décroît 4,85
fois moins vite que celle qui vit sur des modes propres au couplage, et domine dès `n = 4`.

C'est une **explication plausible appuyée sur une corrélation**, pas une cause établie. Les
deux quantités — le repli et l'exposant — croissent ensemble avec `n`, et rien dans S195 ne
les sépare. Cette session les sépare.

> **La question, en une phrase.** Si l'on supprime le repli **sans rien changer d'autre**,
> la loi rejoint-elle la prédiction dispersée ?

## 2. Le montage : la parité sépare le repli du reste

Le repli n'est pas un phénomène physique : c'est une propriété **arithmétique** du jeu de
modes. Une somme ou une différence de deux modes de train retombe sur un mode de train ou
non, selon les nombres choisis. On peut donc l'éteindre sans toucher à la physique.

| famille | modes | somme et différence de deux trains | repli des paires |
|---|---|---|---|
| **dense** | `2, 3, …, n+1` | de toute parité | partiel, croissant |
| **impaire** | `3, 5, …, 2n+1` | **paires** — jamais un mode de train | **nul** |
| **paire** | `4, 6, …, 2n+2` | **paires** — toujours candidates | partiel, croissant |

Impair ± impair = pair, et les trains sont impairs : **aucun terme croisé de paire ne peut
retomber sur un mode de train.** C'est exact, pas approché, et c'est vérifié par construction
en P3a plutôt que supposé.

Fractions de repli calculées d'avance, par comptage des paires sur la bande retenue :

| n | dense | impaire | paire |
|---:|---:|---:|---:|
| 2 | 0,000 | 0,000 | 0,000 |
| 3 | 0,167 | **0,000** | 0,167 |
| 4 | 0,333 | **0,000** | 0,333 |
| 6 | 0,467 | **0,000** | 0,467 |
| 8 | 0,536 | **0,000** | 0,536 |
| 12 | 0,606 | — | — |
| 16 | 0,642 | — | — |

**La famille paire est la famille dense aux modes doublés.** Elle a donc, à `n` égal, la même
fraction de repli *et* la même largeur de bande relative `max/min` ; seule l'échelle absolue
des nombres d'onde change. Cela donne deux comparaisons distinctes :

- **dense contre paire** — même repli, même bande relative, échelle doublée : éprouve si
  l'échelle absolue compte ;
- **paire contre impaire** — même `n`, mêmes cambrures, modes décalés de **1** seulement, et
  repli **opposé** : c'est la comparaison qui isole le repli.

## 3. Ce que ce montage ne contrôle pas

Trois réserves, écrites avant les chiffres.

1. **La bande relative n'est pas identique entre pair et impair.** `max/min` vaut `(n+1)/2`
   pour la famille paire et `(2n+1)/3` pour l'impaire — 3,50 contre 4,33 à `n=6`, 4,50 contre
   5,67 à `n=8`, soit **24 % d'écart**. C'est le confondant résiduel du montage, et il n'est
   pas supprimable : deux classes de parité ne peuvent pas porter les mêmes nombres d'onde.
   Il est **borné par la mesure elle-même** : la famille dense parcourt des bandes relatives
   de 1,5 à 8,5 sur sa plage de `n`, donc si la bande gouvernait l'exposant, la famille dense
   le montrerait seule.
2. **La parité n'éteint le repli que pour les termes de paire.** Les termes **triples**
   `kᵢ±kⱼ±kₖ`, qui apparaissent à `n ≥ 3`, sont impairs pour trois impairs et **retombent**
   donc sur des modes de train. La famille impaire n'a pas « zéro repli » : elle a zéro repli
   sur la famille de termes **dominante**, celle qui gouverne l'écart au premier ordre non
   trivial. Si les triples comptaient autant que les paires, le montage ne conclurait pas — et
   ce serait alors, en soi, le résultat.
3. **Six à seize trains ne sont pas un spectre.** La loi reste une tendance sur `n` fini.

## 4. Campagne déclarée

Série à **cambrure totale fixée** `S = 0,024`, `s = S/n` par train — le cas qui décide si
répartir une même mer sur plus de composantes aide. `L = 8 m`, `g = 9,81` injecté, `M = 3`,
`h = 8 m`, `K = 64`, `dt = T₁/400`, fenêtre de 10 périodes, observations 40 par période,
écart relevé à `N = 1, 2, 5, 10`. **Un seul jeu de phases**, dispersé : S195 a établi que le
jeu de phases ne tranche pas (facteur 1,05 à 1,39), et un point de contrôle à grand `n` le
revérifie ici plutôt que de le supposer acquis.

Bande **serrée** à `Q = 2·max(q) + 4`. S195 a mesuré que la bande ne déplace rien
(`Q = 32` contre `24` : 0,0000 %) ; c'est ce qui rend `n` grand abordable, le coût allant
comme `Q²·(n+1)`. La bande est vérifiée par famille, pas supposée.

| axe | valeurs | ce qu'il mesure |
|---|---|---|
| **limite** | dense, `n ∈ {2,3,4,6,8,12,16}` | l'exposant sature-t-il ? |
| **mécanisme** | impaire et paire, `n ∈ {2,3,4,6,8}` | le repli est-il la cause ? |
| bande | `Q` et `Q+8`, une fois par famille | vérification, pas convergence |
| profondeur discrète | `K = 32 / 64 / 128` | convergence, **trois** niveaux (L274) |
| phases | alignées contre dispersées, à `n=8` dense | le constat de S195 tient-il à grand `n` ? |
| continuité | dense `n=6` à `Q=24` | doit reproduire S195 |

## 5. Prédictions, déclarées avant la mesure

L'exposant est ajusté en moindres carrés log-log sur la **moyenne quadratique** de l'écart
(A238), rapportée à `A`. Lois dérivées en S195, exposants effectifs recalculés sur chaque
plage de `n` :

| plage | addition cohérente | addition dispersée | mesuré S195, dense |
|---|---:|---:|---:|
| `n = 2..6` | `+0,461` | `−0,769` | **`−0,437`** |
| `n = 2..8` | `+0,391` | `−0,805` | — |
| `n = 2..16` | `+0,277` | `−0,861` | — |

**Prédiction 1 — si le repli est la cause.** La famille **impaire** doit s'approcher
nettement de la loi dispersée, disons un exposant sous `−0,65`, et la famille **paire** doit
rester près du `−0,44` de S195. **L'écart entre les deux exposants doit dépasser `0,20`.**

**Prédiction 2 — si le repli n'est pas la cause.** Les deux familles s'accordent à mieux que
`0,10`, et l'explication de S195 tombe. Ce serait un résultat, pas un échec : A241 serait
**réfutée** et la cause resterait à chercher.

**Prédiction 3 — sur la limite.** La fraction de repli de la famille dense **sature** : 0,47
à `n=6`, 0,54 à `n=8`, 0,61 à `n=12`, 0,64 à `n=16`, et elle tend vers **≈ 0,74** quand `n`
croît — c'est de l'arithmétique, calculée d'avance, sans mesure. **Donc, si le repli gouverne
l'exposant, l'exposant doit saturer lui aussi.** Un exposant qui continuerait de dériver
entre `n=8` et `n=16` alors que la fraction de repli n'y bouge plus que de 0,54 à 0,64
affaiblirait le lien de cause.

**Prédiction 4 — dense contre paire.** À `n` égal, mêmes exposants à mieux que `0,10` : le
repli et la bande relative y sont identiques, seule l'échelle absolue change, et l'eau est
profonde pour les deux.

## 6. Réceptions

1. **Décompte de repli par construction** — les fractions du §2 doivent être reproduites
   exactement par le banc, pour les trois familles et tous les `n`. Sans quoi le montage ne
   fait pas ce qu'il annonce.
2. **`n=1`** — écart exactement nul, à tout ordre et dans les trois familles (L271).
3. **`M=1`** — superposition exacte, écart de modes `≤ 10⁻¹⁴`, dans les trois familles.
4. **Continuité avec S195** — dense `n=6`, `Q=24`, `K=64`, `S=0,024`, phases dispersées,
   10 périodes : doit reproduire le `4,507029e-3` de L2 publié par S195, à `10⁻⁶` relatif.
5. **Bande** — `Q+8` ne doit pas déplacer la L2 de plus de 2 %, dans chaque famille.
6. **Conservation** — dérive relative d'énergie sous `10⁻⁴` ; toute configuration qui dépasse
   est hors domaine, publiée et exclue des ajustements.
7. **Convergence** — trois niveaux de `K`, ordre entre 1,5 et 2,5, résidu de Richardson sous
   2 % à `K=64`, sur la moyenne quadratique (L274, A238).
8. **Phases** — à `n=8` dense, alignées et dispersées doivent rester dans un facteur 2, comme
   S195 l'a mesuré à `n=6`.
9. **Reproductibilité** — deux exécutions `release` identiques, empreinte publiée.

## 7. Ce qui ne sera pas fermé

1. **Aucune dérivation.** Même si le repli est confirmé comme cause, cette session ne produit
   pas la loi qui en découle : elle montre que la variable compte, pas ce qu'elle donne.
2. **Le confondant de bande relative reste** (§3.1), borné et non supprimé.
3. **Les termes triples ne sont pas isolés** (§3.2).
4. **Une dimension horizontale, trains colinéaires, fond plat, eau profonde.** L'obliquité
   reste entière et S194 a montré que le couplage est 8,6 fois plus fort vers le rivage.
5. **Aucun contrat runtime** : `f64`, allocations de banc, `water-core` et le support S193
   inchangés, aucune mesure de coût CPU, aucune seconde cible (A98).


## 8. Relevés

`cargo run --release --manifest-path code/Cargo.toml -p water-core --example nl_fallback_2d`

Empreinte **`0xbcf2911362458c13`**, deux exécutions `release` identiques ligne pour ligne.
`water-core` et `support/nl_surface.rs` inchangés ; workspace **331 réussis / cinq ignorés**,
et les **dix** réceptions propres au banc passent.

> **Refactorisation contrôlée.** La flottille et la mesure de S195 ont été sorties dans
> `support/nl_fleet.rs` pour que les deux bancs portent **un seul** véhicule (L137).
> L'empreinte de S195 se reproduit à l'identique, `0x5eb378f6ffe26c9f` : aucune arithmétique
> n'a bougé, et c'était le contrôle de la manœuvre.

### 8.1 Les trois familles

`S = 0,024` répartie sur `n` trains, `K = 64`, `M = 3`, `dt = T₁/400`, 10 périodes, phases
dispersées, bande serrée. Écarts rapportés à `A`.

| famille | n | modes | repli | max/A | **L2/A** | croisé | train |
|---|---:|---|---:|---:|---:|---:|---:|
| dense | 2 | 2..3 | 0,000 | 2,2985e-2 | **7,3390e-3** | 1,417e-2 | 5,327e-3 |
| dense | 4 | 2..5 | 0,333 | 2,4230e-2 | **5,4282e-3** | 6,813e-3 | 6,229e-3 |
| dense | 8 | 2..9 | 0,536 | 2,4600e-2 | **3,8835e-3** | 2,972e-3 | 3,022e-3 |
| dense | 12 | 2..13 | 0,606 | 2,1456e-2 | **3,0665e-3** | 9,980e-4 | 2,258e-3 |
| dense | 16 | 2..17 | 0,642 | 1,7695e-2 | **2,7174e-3** | 1,020e-3 | 1,676e-3 |
| **impaire** | 2 | 3..5 | **0,000** | 2,5727e-2 | **7,1227e-3** | 1,367e-2 | 5,463e-3 |
| **impaire** | 4 | 3..9 | **0,000** | 2,3878e-2 | **4,8078e-3** | 5,859e-3 | 2,488e-3 |
| **impaire** | 8 | 3..17 | **0,000** | 2,3457e-2 | **3,4844e-3** | 3,106e-3 | 2,236e-3 |
| paire | 2 | 4..6 | 0,000 | 2,2349e-2 | **6,9643e-3** | 1,328e-2 | 5,264e-3 |
| paire | 4 | 4..10 | 0,333 | 2,1594e-2 | **5,0301e-3** | 5,697e-3 | 6,304e-3 |
| paire | 8 | 4..18 | 0,536 | 2,3924e-2 | **4,1147e-3** | 2,342e-3 | 3,513e-3 |

*Les `n = 3` et `n = 6` sont dans la sortie du banc ; seuls les points repères sont repris ici.*
**Aucune configuration hors domaine** dans la campagne : dérive d'énergie de `1,4e-9` à
`4,9e-8`, trois à cinq ordres sous le `10⁻⁴` exigé. Les décomptes de repli sont ceux du §2,
reproduits **par construction** et vérifiés par test, famille par famille.

### 8.2 Le verdict : ni la prédiction 1, ni la prédiction 2

Exposants ajustés en log-log sur la moyenne quadratique, plage commune `n = 2..8` :

| famille | repli | exposant | écart à la loi dispersée (`−0,805`) |
|---|---:|---:|---:|
| paire | 0 → 0,536 | **−0,394** | 0,411 |
| dense | 0 → 0,536 | **−0,451** | 0,354 |
| **impaire** | **0 partout** | **−0,525** | **0,280** |

Écart pair/impair : **0,131**. Le protocole demandait **plus de 0,20** pour confirmer que le
repli est la cause, **moins de 0,10** pour la réfuter. **Ni l'un ni l'autre.** Et c'est
précisément la valeur d'une prédiction déclarée avant la mesure : aucune des deux conclusions
préparées ne peut être revendiquée.

Ce que la mesure dit, en propre :

> **Le repli agit, dans le sens prédit, et il n'explique qu'un tiers.** Éteindre entièrement
> le repli des termes de paire déplace l'exposant de `−0,394` à `−0,525`, soit **32 %** du
> chemin qui reste à parcourir jusqu'à la loi dispersée. **Les deux autres tiers ont une
> cause que cette session n'identifie pas.**

La direction, elle, est sans ambiguïté : **moins de repli, décroissance plus rapide**. Le
mécanisme proposé par S195 est réel ; ce qui était faux est son poids, que S195 avait déduit
d'une corrélation — les deux quantités croissant ensemble avec `n`.

**Prédiction 4 confirmée, et elle valide le montage.** Dense contre paire — même fraction de
repli, même bande relative, échelle des nombres d'onde **doublée** — donnent `−0,451` et
`−0,394`, soit **0,057** d'écart, sous le seuil de 0,10. L'échelle absolue ne compte pas en
eau profonde, comme attendu, ce qui confirme que la comparaison pair/impair mesure bien le
repli et non un artefact d'échelle.

Reste le confondant déclaré au §3.1 : la bande relative diffère de 24 % entre pair et impair.
La famille dense le borne par sa propre plage, où la bande relative va de 1,5 à 8,5 — soit un
facteur **5,7** — pour un exposant qui ne varie que de `−0,438` à `−0,520`. Un confondant de
24 % ne peut donc pas porter les 0,131 observés ; il peut en porter une petite part, et cette
part n'est pas séparée.

### 8.3 La limite existe, et `n ≤ 6` la sous-estimait

Fenêtres glissantes sur la famille dense :

| fenêtre | exposant | repli, début → fin |
|---|---:|---|
| `n = 2,3,4,6` | **−0,438** | 0,000 → 0,467 |
| `n = 4,6,8,12` | **−0,520** | 0,333 → 0,606 |
| `n = 8,12,16` | **−0,520** | 0,536 → 0,642 |

**L'exposant sature à `−0,52` dès `n ≈ 4`.** La réponse à la moitié « limite » d'A241 est donc
oui : la loi tend vers quelque chose, et `n ≤ 6` — la plage de S195 — la sous-estimait un peu,
`−0,44` contre `−0,52`.

Mais la lecture qui compte est ailleurs. Entre les deux dernières fenêtres, la fraction de
repli monte encore de `0,536` à `0,642` — elle n'a pas fini de saturer, sa limite arithmétique
étant `≈ 0,74` — **pendant que l'exposant ne bouge plus du tout**, à trois décimales près.
Si le repli gouvernait, il resterait du mouvement. **Cette immobilité affaiblit le lien de
cause davantage encore que le 0,131 du §8.2**, et dans une direction que la prédiction 3 avait
anticipée sans en fixer le sens.

### 8.4 Réceptions : huit sur neuf, et l'échec est instructif

| # | réception | résultat |
|---:|---|---|
| 1 | décompte de repli par construction | **passe** (test, trois familles) |
| 2 | `n=1` écart exactement nul | **passe** (test, trois familles) |
| 3 | `M=1` exact à `10⁻¹⁴` | **passe** (test, trois familles) |
| 4 | continuité S195 | **passe** — `4,507029e-3` contre `4,507029e-3`, écart **5,03e-8** |
| 5 | bande `Q+8` sous 2 % | **passe** — 0,001 %, 0,008 %, 0,032 % |
| 6 | énergie sous `10⁻⁴` | **passe** — max `4,9e-8` |
| 7 | convergence en `K` | **ÉCHOUE** |
| 8 | phases dans un facteur 2 | **passe** — 0,747 (max) et 0,885 (L2) à `n=8` |
| 9 | deux exécutions identiques | **passe** — empreinte reproduite |

**La réception 7 échoue, et il faut le dire clairement.** À `n = 6` : ordre **1,268**, sous le
`1,5` exigé, et résidu de Richardson **2,27 %**, au-dessus des 2 % exigés. S195 passait cette
même réception (ordre 1,756, résidu 0,892 %) sur une autre configuration ; l'ordre dépend donc
du montage, et celui-ci est moins bien résolu verticalement.

**Ce qui sauve la conclusion n'est pas une indulgence, c'est une borne mesurée.** La question
n'est pas « `K=64` est-il convergé » mais « le défaut de résolution biaise-t-il l'exposant ».
La pente entre les deux bouts de la plage a donc été refaite aux deux résolutions :

| pente `n = 6 → 16` | valeur |
|---|---:|
| à `K = 64` | `−0,516` |
| à `K = 128` | `−0,506` |
| **biais** | **`+0,010`** |

Un centième, contre les `0,131` qui portent le résultat. **La conclusion tient**, et elle tient
parce qu'on a mesuré le biais au lieu de l'espérer petit.

### 8.5 Ce que le critère d'énergie n'a pas vu

À `n = 16`, le niveau `K = 32` rend une L2 de `1,3765e-2` là où `K = 64` donne `2,7174e-3` et
`K = 128` `2,8352e-3` : **faux d'un facteur cinq**, et du mauvais côté. Ce n'est pas une
divergence — aucun pas n'a été refusé, rien n'est infini, et la **dérive relative d'énergie
vaut `1,54e-6`**, soit soixante-cinq fois sous le seuil de domaine de `10⁻⁴` que S194, S195 et
cette session emploient.

**Le critère de conservation a donc déclaré sain un résultat faux d'un facteur cinq.** Il
détecte l'instabilité ; il est aveugle à la sous-résolution, qui produit une réponse lisse,
conservative, et fausse.

Le triplet de Richardson en devient inutilisable : ses incréments valent `−1,105e-2` puis
`+1,178e-4` — signes opposés, donc pas de convergence monotone, donc **aucun ordre n'en sort**.
Le banc le **dit** au lieu d'imprimer le `6,552` que la formule aurait rendu sans broncher.
Voir **A242** et **L277**.

## 9. Verdict sur A241

**A241 n'est pas close ; elle est requalifiée, et ses deux moitiés n'ont pas le même sort.**

**La moitié « limite » est close.** L'exposant sature à `−0,52` dès `n ≈ 4`, et reste immobile
jusqu'à `n = 16`. La loi tend vers quelque chose. `n ≤ 6` la sous-estimait de `0,08`, ce qui
est réel mais modeste : S195 ne trompait pas gravement.

**La moitié « cause » reçoit une réponse partielle, et c'est le résultat de la session.** Le
repli existe, agit dans le sens prédit, et pèse **un tiers**. La thèse de S195 — le repli
gouverne la loi à grand `n` — est donc **trop forte** : il la déplace, il ne la gouverne pas.
Deux tiers de l'écart entre la mesure et l'addition dispersée restent sans explication, et
c'est désormais une question nette plutôt qu'un soupçon.

Ce que la session ne fait pas, et qu'il faut redire : elle n'identifie pas les deux autres
tiers, elle ne dérive aucune loi, et elle ne sépare pas la part du confondant de bande
relative (§3.1) ni celle des termes triples (§3.2) — les deux premiers suspects pour la suite.

**Aucun ADR.** Rien ne change de contrat ; ADR-123 n'est pas touché, aucun solveur δ n'est
choisi, aucun seuil de bascule W/δ n'est dérivé.
