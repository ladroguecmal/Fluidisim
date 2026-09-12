# S184 — Ce que coûte de consommer la source

2026-09-12. S183-1 / A50. **Mesure locale sur une machine, aucun budget cible certifié.**

[COUT-DIFFERENTIEL-S183](COUT-DIFFERENTIEL-S183.md) a mesuré ce que coûte de **produire** la
source. Ce document mesure ce que coûte de **s'en servir**, ce qui n'est pas la même chose et
ne s'en déduit pas : un coût de production ne devient une contrainte qu'une fois rapporté au
travail qu'il accompagne.

Comme en S183, les conditions sont publiées **avant** la première exécution (§1–§5), les
relevés viennent après (§6). L'ordre n'est pas décoratif : c'est lui qui a rendu lisible, en
S183, l'écart entre ce qu'un axe devait mesurer et ce qu'il mesurait (L263).

## 1. La question, et pourquoi elle n'était pas encore posée

S170 a mesuré l'**erreur** de la décimation spatiale de la source sur un véhicule 1D, et
[l'a écrit explicitement](SOURCE-DECIMEE-S170.md) §2.2 : *« Le nombre de nœuds est un coût
géométrique, pas un gain de temps runtime mesuré […] ils ne prouvent ni un gain de temps, ni
le facteur 64 en 3D. »* S174 a fait de même pour la cadence temporelle. Les deux ont
délibérément laissé le temps de côté, faute d'un fournisseur réel à chronométrer.

Ce fournisseur existe depuis S177–S182, et son coût est chiffré depuis S183. La question
restée ouverte est donc exactement celle-ci, et elle est neuve :

> **Rapporté au pas de solveur qu'elle alimente, la source coûte-t-elle peu, beaucoup, ou
> trop ?** Et si c'est trop, qu'achète la décimation — en temps, pas en nœuds ?

**Ce document ne remesure pas l'erreur de décimation.** S170 et S174 l'ont fait en 1D, et leur
conclusion tient et s'applique : *un ratio de décimation ne décrit pas à lui seul la précision*
(S170 §2.2). Aucun `H` recommandé ne sortira d'ici — seulement ce que chaque `H` coûte.

## 2. Le véhicule

Un **pas explicite de quantité de mouvement perturbative** sur un bloc 3D de mailles, écrit
comme exemple et non comme bibliothèque : c'est un véhicule d'essai, et le choix du solveur du
projet appartient au banc B3 (ADR-007 §5). Précédent de forme : `source_decimee.rs` (S170).

Par maille intérieure, une couche de mailles fantômes autour :

```
adv_i   = Σ_j u'_j · ∂_j u'_i           différences centrées
lap_i   = Δ u'_i                         laplacien à sept points
u'_i   ← u'_i + dt · ( −adv_i + ν·lap_i − S_i )
```

`S` est `DifferentialSample::momentum_residual`, en m/s², **soustraite** — SPEC-004 §6.1 et
ADR-114. Les deux variantes comparées sont le **même pas**, au terme `− S_i` près.

Trois façons d'obtenir `S`, mesurées séparément :

| mode | ce qui change |
|---|---|
| **par maille, chaque pas** | un point de requête par maille intérieure, à chaque pas |
| **décimé en espace**, `H = r·dx`, `r ∈ {2,4,8}` | requête aux nœuds d'un réseau de pas `H`, puis interpolation trilinéaire aux centres de mailles |
| **cadence `c ∈ {1,2,4,8,16}`** | la source n'est reconstruite qu'un pas sur `c`, et réemployée entre-temps |

L'interpolation est **trilinéaire au centre de maille**, pas une moyenne de cellule intégrée
exactement comme en S170 §1. Ce choix ajoute une erreur d'interpolation que ce document ne
mesure pas et ne prétend pas majorer ; il ne change pas le **coût**, qui est ce qui est mesuré.

## 3. Ce que le véhicule ne fait pas, et ce que cela fausse

**Il ne projette pas.** Aucune résolution de pression, donc aucune incompressibilité imposée :
choisir une projection est une décision de solveur, et elle appartient à B3. Ce n'est pas un
détail neutre, et il tire dans un sens précis :

> omettre la projection **sous-estime** le pas de solveur, donc **sur-estime** la part de la
> source dans le total.

C'est la faiblesse principale de ce montage, et elle est déclarée avant les chiffres pour
qu'elle ne se découvre pas après. Elle sera **bornée** en §7 plutôt qu'ignorée : les relevés
permettent de dire de combien la projection devrait coûter pour que la conclusion bascule, et
ce nombre-là ne dépend d'aucun choix de solveur.

Il ne modélise pas non plus la surface libre, ni les conditions de bord réelles, ni la
concurrence, ni la pression de cache d'un jeu. C'est un pas, pas un solveur.

## 4. Conditions matérielles et grille

AMD Ryzen AI 7 350, Windows 11 x86_64 MSVC, rustc 1.97.0 / LLVM 22.1.6, profil `release` du
dépôt (`overflow-checks = true`). Même machine et même chaîne que S125 et S183 ; les trois
séries sont comparables entre elles, et avec rien d'autre.

Montage d'eau : celui de S183 §3, inchangé — `SeaState{hs 0,1 ; tp 6 ; θ 0,125}` à 16
composantes, un impact `N=64` de 0,01 J, deux sources de pression, recette 16×24 soit 192
créneaux, ancre monde à `1e9 m`. Bloc de mailles `dx = 0,25 m`, centré sur le local `(1 ; 1)`,
`z` de −3,5 m à −0,05 m, entièrement dans le domaine du fond, dans le rayon d'impact et dans
la boîte de pression.

> **Correction, P3.** Cette géométrie fixe ne tient pas : le nœud supérieur d'un réseau
> grossier **déborde du bloc**, et à `r = 8` il sortait du domaine (`z > 0`, refusé par
> `differential_local`). Le bloc est donc placé en fonction de la portée réelle des réseaux —
> `z` va de −2,30 / −4,30 / −6,30 m à −0,05 m pour les côtés 10 / 16 / 20, et `x, y` restent
> centrés sur `(1 ; 1)`. Conséquence à dire : **les trois blocs n'échantillonnent pas la même
> profondeur d'eau**. Cela ne change pas l'arithmétique mesurée, et les trois donnent bien le
> même coût par maille à 4 % près (§6.2).

| axe | valeurs | ce qu'il déplace |
|---|---|---|
| **côté du bloc** | 10, **16**, 20 mailles → 512, **2744**, 5832 mailles intérieures | prolonge l'axe « lot » de S183 bien au-delà de 256 points |
| **décimation** `r = H/dx` | **1**, 2, 4, 8 | nombre de requêtes par pas |
| **cadence** `c` | **1**, 2, 4, 8, 16 | nombre de pas entre deux reconstructions |

**Sept blocs**, ordre renversé un sur deux, une seconde de mise en régime, `black_box` sur
entrées et résultats, min/médiane/max des moyennes de bloc, **deux exécutions publiées**.
Sept et non quinze comme en S183 : une reconstruction complète de la source sur 5832 mailles
coûte à elle seule de l'ordre de la fraction de seconde, et la quantité mesurée est grande et
stable — quinze blocs coûteraient huit fois plus pour une précision dont on n'a pas l'usage.

## 5. Réceptions exigées avant tout chronométrage

Comparer deux durées n'a de sens que si les deux chemins font ce qu'ils annoncent. Quatre
contrôles, tous fermés, tous à l'arrondi ou au bit :

1. **La source atteint l'état.** À `ν = 0` et `u' = 0` partout, un pas avec source doit laisser
   exactement `u' = −dt·S`, comparé **en bits** à `−dt·S` calculé directement. Rien d'autre ne
   contribue, et le signe est celui que SPEC-004 §6.1 impose.
2. **Les deux chemins ne diffèrent que par la source.** Au même état de départ, la différence
   entre le pas avec source et le pas sans doit valoir exactement `−dt·S`, en bits.
3. **La décimation à `r = 1` n'est pas un chemin séparé.** Réseau aligné sur les mailles, poids
   d'interpolation 0 ou 1 : elle doit reproduire la source par maille **en bits**. Sans quoi le
   chemin décimé mesurerait autre chose que le chemin direct.
4. **Tout reste fini** sur l'ensemble des pas de tous les montages, source et état.

## 6. Relevés

```
cargo run --release --manifest-path code/Cargo.toml -p water-core --example perturbative_step
cargo run --release --manifest-path code/Cargo.toml -p water-core --example lattice_phase
```

Deux exécutions de chaque binaire, notées **E1 / E2**. Médianes des sept blocs sauf mention.
Bibliothèque inchangée ; workspace **331 réussis / cinq ignorés** en debug et en release.

### 6.1 Réceptions, et une correction de protocole

Les quatre contrôles de §5 passent aux trois tailles de bloc, **sans aucune composante
exemptée** par la tolérance ±0 prévue : les égalités sont exactes au bit, pas à la valeur.

> **Correction, P3.** La réception 2 telle qu'elle était formulée en §5 était **fausse**, et
> l'écrire avant de mesurer est ce qui l'a rendue visible. Elle demandait que la différence
> entre le pas avec source et le pas sans vaille exactement `−dt·S`. C'est vrai en algèbre et
> faux en flottant : `dt·(X − s)` n'est pas `dt·X − dt·s`. Le contrôle exact qui la remplace
> est plus fort et non plus faible — **source forcée à zéro, le chemin « avec source » doit
> rejoindre le chemin « sans source » bit pour bit**, ce qui vaut pour tout état de départ et
> pas seulement pour `u' = 0`. Il passe. L'énoncé initial reste ci-dessus, non réécrit.

### 6.2 Le pas, et la source qui l'alimente

Le pas, en **nanosecondes** par maille intérieure (min/médiane/max, E1 puis E2) :

| côté | mailles | sans source | avec source |
|---|---:|---|---|
| 10 | 512 | 14,5/15,1/23,6 · 14,5/14,8/18,0 | 16,2/16,2/19,6 · 16,2/17,1/20,5 |
| 16 | 2744 | 14,2/14,5/17,8 · 14,1/14,6/24,1 | 15,9/16,4/19,9 · 15,8/16,9/21,0 |
| 20 | 5832 | 14,1/15,5/17,6 · 14,2/16,5/26,0 | 15,6/16,7/17,8 · 15,7/16,5/19,0 |

La source, en **microsecondes** par maille intérieure (médianes E1 / E2) :

| côté | par maille | réseau r=2 | réseau r=4 | réseau r=8 | part du pas dans le total |
|---|---:|---:|---:|---:|---:|
| 10 | 34,29 / 35,24 | 8,66 / 8,51 | 1,82 / 1,85 | 0,565 / 0,544 | 0,047 / 0,049 % |
| 16 | 35,16 / 34,48 | 6,54 / 6,59 | 1,63 / 1,61 | 0,361 / 0,360 | 0,047 / 0,049 % |
| 20 | 34,45 / 34,13 | 6,00 / 5,90 | 1,33 / 1,31 | 0,400 / 0,407 | 0,049 / 0,048 % |

Le rapport est le résultat de la session :

> **La source coûte environ 2 100 fois le pas qu'elle alimente.** 35 µs contre 16 ns par
> maille. Le pas explicite complet — advection, laplacien, mise à jour, trois composantes —
> représente **0,047 à 0,049 %** du travail total.

Ce n'est pas un effet de taille : les trois blocs, de 512 à 5832 mailles, donnent le même
rapport à 4 % près. C'est aussi le prolongement de l'axe « lot » de S183 bien au-delà de 256
points, et il confirme la saturation : 34–35 µs par point à 2744 et 5832 points, contre 39 µs
à 256 points en S183 — le coût par point cesse de monter et redescend légèrement.

### 6.3 Ce que la décimation achète, et ce qu'elle a le droit d'ignorer

Le gain de la décimation spatiale est **exactement le rapport des nombres de nœuds**, pas
davantage. Pour le bloc 16, les réseaux comptent 2744 / 512 / 125 / 27 nœuds :

| r | nœuds | coût prédit par le seul comptage | mesuré | écart = interpolation |
|---:|---:|---:|---:|---:|
| 1 | 2744 | 35,0 µs | 35,08 | ~0,01 µs |
| 2 | 512 | 6,53 | 6,54 | ~0,01 |
| 4 | 125 | 1,59 | 1,63 | ~0,04 |
| 8 | 27 | 0,344 | 0,361 | ~0,02 |

L'interpolation trilinéaire coûte donc **10 à 40 ns par maille** — le même ordre que le pas
lui-même. Elle ne se voit pas tant que la source est chère ; elle se verrait si la source
devenait bon marché.

La cadence divise **exactement** par `c`, sur tous les blocs et aux deux exécutions :

| côté | c=1 | c=2 | c=4 | c=8 | c=16 |
|---|---:|---:|---:|---:|---:|
| 16 | 34,50 / 34,92 | 17,37 / 17,36 | 8,59 / 8,49 | 4,28 / 4,19 | 2,19 / 2,16 |

La loi `1/c` est vérifiée à 1 % près sur toute la plage mesurée.

**Mais les deux axes ne sont pas également disponibles, et c'est le contenu de la source qui
en décide.** La recette de pression a une coupure `k_max = 5,8125 rad/m` — calculée, non
posée : `(15+0,5)·6,0/16` dans `gaussian_spectrum::bake`. Soit `λ_min = 1,081 m`. À
`dx = 0,25 m` :

| r | H | points par `λ_min` de pression | points par `λ = 4 m` de l'impact |
|---:|---:|---:|---:|
| 1 | 0,25 m | 4,32 | 16,0 |
| 2 | 0,50 m | **2,16** | 8,0 |
| 4 | 1,00 m | 1,08 | 4,0 |
| 8 | 2,00 m | 0,54 | 2,0 |

`r = 2` est déjà **à la limite de Nyquist** pour la plus courte longueur d'onde de pression ;
`r = 4` et `r = 8` sont sous cette limite et replient le contenu. La décimation spatiale est
donc plafonnée par la physique à **environ 5,4×**, quoi qu'en dise le coût.

Le contenu temporel, lui, est lent : périodes de 3 à 12 s pour `B`, segments de pression de
2 s. À `dt = 1 ms`, `c = 16` ne couvre que 16 ms. **L'axe cher est l'espace, l'axe bon marché
est le temps** — l'inverse de ce que suggère l'intuition d'un réseau 3D, où l'on économise
`r³` en espace et seulement `c` en temps.

### 6.4 Ce qu'une évaluation par réseau retirerait — et c'est peu

Puisque le coût est dans les sommes modales, l'optimisation évidente est de ne plus payer la
phase par point : sur un réseau régulier, la phase de chaque mode avance d'un incrément
constant, et une récurrence la remplace par une rotation. Mesuré sur le même réseau 14³ que
le bloc 16, spectre JONSWAP `from_spectrum` (médianes E1 / E2, ns par nœud) :

| composantes | B différentiel | phase+trigo par point | par récurrence | part de la trigo | plafond du gain |
|---:|---:|---:|---:|---:|---:|
| 32 | 4483 / 4079 | 668 / 630 | 120 / 110 | 14,9 / 15,4 % | **12,2 / 12,7 %** |
| 64 | 8172 / 8124 | 1337 / 1330 | 216 / 221 | 16,4 / 16,4 % | **13,7 / 13,7 %** |
| 128 | 17288 / 16227 | 3129 / 2993 | 532 / 500 | 18,1 / 18,4 % | **15,0 / 15,4 %** |

**Le résultat est négatif, et c'est ce qu'il apporte.** La trigonométrie ne pèse que 15 à
18 % du différentiel de `B`, et la récurrence n'en retire que **12 à 15 %**. À 32 composantes,
un nœud coûte 140 ns par composante, dont 21 de phase : les 119 restants sont l'arithmétique
ordinaire qui produit les 26 scalaires de `BackgroundSample`, et changer la traversée n'en
retire rien.

*Ce montage utilise le spectre JONSWAP, dont le profil V1 exige au moins 32 composantes ; les
chiffres absolus ne se comparent donc pas directement au `configure` historique à 16
composantes de S183. Les **parts**, elles, sont internes au montage et valides.*

**Et ce n'est pas une proposition.** Une récurrence ne rend pas les mêmes bits qu'une
évaluation directe, et elle dérive le long d'une ligne : I-03 l'interdirait telle quelle. On
chiffre une occasion pour savoir qu'elle ne vaut pas la peine, pas pour l'adopter.

## 7. Ce qui est reçu, et ce qui ne l'est pas

**Reçu.**

1. **Le rapport : ~2100.** La source coûte 34–35 µs par maille, le pas qu'elle alimente
   14–17 ns. Le pas est 0,047–0,049 % du total, aux trois tailles de bloc.
2. **La décimation spatiale achète exactement le rapport des nœuds**, ni plus ni moins ;
   l'interpolation trilinéaire coûte 10–40 ns par maille, l'ordre du pas.
3. **La cadence divise exactement par `c`** sur toute la plage mesurée.
4. **Le contenu plafonne l'espace et libère le temps.** `λ_min = 1,081 m` interdit d'aller
   au-delà de `r = 2` à `dx = 0,25 m` ; les échelles temporelles de 2 à 12 s rendent la
   cadence bien plus disponible que la décimation.
5. **Optimiser la traversée ne sauverait pas la situation** : 12–15 % au mieux. Le coût est le
   volume de sortie par composante, pas la trigonométrie.
6. Les quatre réceptions passent au bit, sans exemption, et la réception 2 a été **corrigée**
   par une formulation strictement plus forte (§6.1).

**Non reçu.**

- **Aucune erreur de décimation n'est mesurée ici.** S170 et S174 l'ont fait en 1D sur des
  fonds analytiques ; leur avertissement tient — *un ratio ne décrit pas à lui seul la
  précision*. Rien ici ne recommande un `H` ni un `c`.
- **Le véhicule ne projette pas.** Comme annoncé en §3, cela sous-estime le pas et sur-estime
  la part de la source. Voici la borne promise, et elle est indépendante de tout choix de
  solveur : pour que la source tombe sous la moitié du total **à `r = 1`**, le pas devrait
  coûter 35 µs par maille, soit **2 100 fois** ce qui est mesuré. Aucune projection ne coûte
  cela. En revanche, à `r = 2` et `c = 16` la source revient à 0,41 µs par maille et par pas,
  et il suffirait alors d'un pas 25 fois plus cher que celui-ci — ce qu'une projection
  multigrille peut plausiblement valoir — pour que les deux soient du même ordre. **La
  conclusion n'est donc pas « impossible » : elle est « pas à cadence d'image, et pas sans la
  cadence temporelle ».**
- **Une seule machine**, une seule chaîne, pas de vectorisation, pas de concurrence, pas de
  pression de cache d'un jeu réel. I-03 porte sur les valeurs, jamais sur les durées.
- **Le levier non testé est la vectorisation**, pas le parcours : 85 % du coût est de
  l'arithmétique scalaire régulière sur 26 scalaires par composante. Personne ne l'a mesurée.

## 8. Suite

**A50 reste partielle**, mais elle a changé de nature. Jusqu'ici il lui manquait un chiffre ;
désormais il lui manque une **décision** : à quelle cadence, et sur quel réseau, un solveur
perturbatif consomme cette source. Les deux axes sont mesurés en temps ; leur erreur ne l'est
qu'en 1D.

**S184-1 — l'erreur de cadence en 3D, avec le fournisseur réel.** C'est le seul des deux axes
qui soit à la fois bon marché et non mesuré ici. S174 l'a fait en 1D sur des instantanés
connus ; il s'agit de le refaire sur `B+W+pression`, à `c` croissant, en comparant le champ
obtenu à celui d'une reconstruction à chaque pas. La décimation spatiale suit, bornée à
`r = 2` par §6.3.

Voir **A227** pour ce que ces mesures disent de la forme du fournisseur, et **L264** pour la
méthode qui a évité d'optimiser le mauvais tiers.

Aucun ADR : rien n'a changé de contrat, et le choix du solveur reste à B3 (ADR-007 §5).
Aucun arbitrage humain nouveau.
