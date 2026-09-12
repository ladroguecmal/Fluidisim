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

*À recevoir en P3b. Aucun chiffre n'est écrit avant l'exécution.*

## 9. Verdict sur A241

*À recevoir en P4.*
