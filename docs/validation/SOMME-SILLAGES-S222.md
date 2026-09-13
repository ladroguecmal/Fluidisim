# La part somme d'A254 sur plusieurs sillages, et ce qu'une borne locale conjointe rendrait — S222, 2026-09-13

Traite la **part somme** d'**A254** du côté des sillages. **Aucun ADR, aucune migration
d'admission** : la mesure conclut à ne rien changer, et dit pourquoi. A254 n'est pas close — elle
change de terme.

## En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : enveloppe directionnelle ADR-134 (terme actuel du budget) ; borne
  locale à coupure spectrale ADR-137 et sa partition adaptative S219 ; majorant d'impact à
  dispersion ADR-133. Échantillonnage du maximum réel : balayage 0,5 m sur l'emprise puis
  raffinement local à 2 cm.
- **Techniques absentes** : aucune technique de rendu — GPU, LOD, visibilité, mutualisation ;
  un seul fil ; aucun SIMD explicite. Ce document mesure un **contrat d'admission**, pas une image.
- **Domaine de validité** : recette S219–S221 (σ 2, cutoff 3, radial 64, angular 128), emprise
  128 × 96 m, instant de la fixture « base » (8 s de forçage puis τ = 4, soit ≈ 9,81 s) ; une à
  trois sources de sillage **dans un même journal**, trajectoires parallèles écartées de 4 m
  (« proches ») ou 30 m (« éloignées ») ; impact de la scène J1 (λ 3,35 m, E 164 J).
  **Ne dit rien** de sources à recettes ou emprises différentes — le cœur ne sait pas les composer.
- **Rang de passage** : troisième passage pour les relevés d'admission ; les coûts de partition
  sont ceux des passages deux et trois, écart ≤ 4 % (L289).

## 1. Ce que la relecture établit, et que la mesure confirme

`mixed_water::slope_floor(impacts, pressure, time)` somme **un majorant par impact** (ADR-133) plus
**un seul** terme de pression — sa signature ne prend qu'un `Option<&bound_pressure::Prepared>`.
Plusieurs sillages n'entrent donc au budget qu'en **partageant un journal**, une recette et une
emprise, et c'est la seule configuration que le cœur sache composer.

**Témoin** : `modes = 4096` pour une, deux et trois sources. Les emplacements du demi-spectre sont
communs, les amplitudes modales s'additionnent **en complexe**. Aucun refus — ni à la construction,
ni au journal, ni à la préparation. La préparation, elle, est **linéaire** : 6,3 / 13,4 / 18,9 ms.

## 2. L'enveloppe est sous-additive, mais elle pénalise la séparation

| config | sources | enveloppe globale | × une source | maximum réel | pessimisme |
|---|---:|---:|---:|---:|---:|
| proches (4 m) | 1 | 0,115171 | 1,0000 | 0,070316 | 1,6379 |
| proches | 2 | 0,165659 | 1,4384 | 0,111800 | 1,4818 |
| proches | 3 | 0,192466 | **1,6711** | 0,130667 | 1,4730 |
| éloignées (30 m) | 2 | 0,146879 | 1,2753 | 0,070390 | 2,0867 |
| éloignées | 3 | 0,165357 | **1,4358** | 0,070463 | **2,3467** |

**Trois sillages coûtent 1,67 fois un seul quand ils sont proches, 1,44 quand ils sont éloignés** —
très loin du facteur 3 que subissent les impacts. La composition modale complexe absorbe
l'essentiel de la somme.

**Le fait neuf est que le pessimisme empire avec la séparation.** Éloignées, le maximum réel ne
bouge pas — 0,070316 / 0,070390 / 0,070463 : chaque sillage a sa région, et le maximum reste celui
d'un seul. L'enveloppe croît de 44 %. Le pessimisme passe de 1,64 à **2,35**. C'est **A261** —
aucune enveloppe de modules ne voit la localisation spatiale — mesurée pour la première fois sur une
scène et non sur une maille.

## 3. La borne locale rend tout, et son prix est une loi d'échelle

**Partition sur l'emprise**, budget d'évaluations égal d'une configuration à l'autre :

| config | sources | 2 047 éval. | 8 191 éval. | 32 767 éval. (borne/max) | gain sur l'enveloppe | coût |
|---|---:|---:|---:|---:|---:|---:|
| proches | 3 | 1,0097 | 1,2206 | **1,0053** | 1,4652 | 25,0 s |
| éloignées | 2 | 1,0031 | 1,1310 | **1,0088** | **2,0683** | 24,2 s |
| éloignées | 3 | 1,0023 | 1,1319 | **1,0099** | **2,3237** | 25,1 s |

À 32 767 évaluations la borne colle au maximum à **0,5–1,0 %** dans toutes les configurations, et le
gain suit exactement le pessimisme du §2. **La borne locale voit précisément ce que l'enveloppe ne
voit pas.**

**Une requête locale n'est pas un raccourci** — c'était l'hypothèse à écarter, et elle l'est sur les
deux plans :

| demi-côté | borne / enveloppe globale | borne / maximum local | µs par appel |
|---:|---:|---:|---:|
| 1 m (au pire point) | 1,03 à **1,09** | 1,41 à 2,15 | ~670 |
| 4 m | **0,9977** | 1,48 à 2,09 | ~650 |
| 16 m | **0,9977** | 1,48 à 2,09 | ~640 |

Au-delà d'environ un mètre de demi-côté, **la borne locale *est* l'enveloppe globale** — 0,9977,
soit l'enveloppe plus sa réserve numérique. ADR-137 dit pourquoi : passé une demi-longueur d'onde,
tous les modes tombent dans la classe non résolue `D ≥ 2`, la coupure les renvoie à `G(U)`, et il ne
reste rien à gagner. La plus courte longueur d'onde représentée vaut `2π/cutoff = 2,09 m` : **la
maille doit être sous-ondulatoire, ou elle ne sert à rien.** Et un appel coûte **640 à 700 µs**, pas
quelques microsecondes : il est `O(N)` à 4 096 modes.

**Le prix est donc structurel.** Les mailles utiles font ≈ 2 m ; couvrir 128 × 96 m en demande
~3 000, atteindre 1,006 en demande 16 384. À 640 µs l'unité : 2 s et 25 s — exactement ce que la
partition mesure. Aucune optimisation de constante ne franchit les quatre ordres de grandeur qui
séparent cela du budget d'image de 2 ms.

**A261 se chiffre au passage** : loin de la source, à 2 × 2 m, la meilleure borne disponible vaut
**213 à 757 fois** le maximum local. `G(U)` ne dépend ni du point ni des phases ; il est
spatialement aveugle par construction, et c'est lui qui plafonne tout.

## 4. La réponse à A254, dans la monnaie qui décide

`π/7 = 0,448799` ; terme d'impact unitaire à l'instant mesuré **0,030069** ; impacts toujours
sommés.

| config | sillages | enveloppe | part | impacts admis | borne partitionnée | part | impacts | maximum réel | impacts |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| proches | 1 | 0,115171 | 25,7 % | **11** | 0,070740 | 15,8 % | 12 | 0,070316 | 12 |
| proches | 3 | 0,192466 | **42,9 %** | **8** | 0,131362 | 29,3 % | 10 | 0,130667 | 10 |
| éloignées | 3 | 0,165357 | 36,8 % | **9** | 0,071160 | **15,9 %** | **12** | 0,070463 | 12 |

En S214, *une* source de chaque type consommait 84 % de π/7 et la deuxième refusait l'image. Ici,
**trois sillages et huit impacts passent**, neuf s'ils sont éloignés. La borne partitionnée en
rendrait **deux à trois de plus**, et elle est **à un impact près du maximum réel** partout : il n'y
a pas de troisième chemin à chercher — l'instrument est aussi bon qu'il peut l'être, c'est son prix
qui le disqualifie.

**Mais la conclusion a un domaine, et il la corrige.** Le terme d'impact dépend fortement de l'âge :

| âge de l'impact (s) | terme unitaire | part de π/7 | impacts admis à côté de trois sillages |
|---:|---:|---:|---:|
| 0 à 0,5 | 0,212607 | **47,4 %** | **1** |
| 2 | 0,142269 | 31,7 % | 1 |
| 4 | 0,054900 | 12,2 % | 5 |
| 16 | 0,021545 | 4,8 % | 13 |
| 56 | 0,007390 | 1,7 % | 38 |

Sous deux secondes, **un seul** impact passe à côté de trois sillages : une scène à deux
éclaboussures simultanées reste refusée. **Et la borne partitionnée n'y change rien** — pression à
0,071160 au lieu de 0,165357, un impact neuf laisse encore la place d'un seul. Le terme qui sature
est le **majorant de naissance de l'impact**, et ADR-133 ne peut rien pour lui : à τ = 0 il est
**exactement atteint** (S215, 0,999998). Ce n'est pas du pessimisme, c'est la physique.

## 5. Décision : ne rien migrer, et dire où est le goulot

**Aucun ADR, aucune migration d'admission.**

1. **Le prix disqualifie la borne partitionnée** comme terme d'admission : +2 à +3 impacts pour
   25 s contre un budget d'image de 2 ms, défendu par la loi d'échelle du §3.
2. **Le terme de pression n'est plus le goulot** : 16 à 43 % de π/7, que la borne porterait à 16 %.
   Migrer coûterait des secondes pour desserrer ce qui ne serre plus.
3. **Le goulot a changé de côté.** La part somme d'A254 est **absorbée** chez les sillages — ce qui
   y reste est spatial (A261) — et **littérale** chez les impacts : `slope_floor` somme
   `slope_max_at` par champ **sans aucune conscience de la distance**. Deux impacts frais à cent
   mètres l'un de l'autre s'ajoutent exactement comme s'ils étaient au même point. Rien dans
   S215–S222 ne l'a touchée : ADR-133 a resserré chaque majorant **dans le temps**, jamais leur
   **somme dans l'espace**.

Ce que la décision ne fait pas : elle ne retire rien à ADR-135/136/137, publiées et mesurées ; elle
ne dit pas la borne locale inutile — elle dit qu'elle n'est pas un terme d'admission **par image**.
Un usage hors image — validation, outillage auteur, banc — reste ouvert.

## Suite

**La somme spatiale sur les impacts.** C'est le goulot mesuré, et la géométrie y est bien plus
favorable qu'au sillage : un `RadialImpact` a un **support compact et connu** — un disque de rayon
déclaré — et sa pente décroît avec la distance à son centre, ce qu'aucune somme de modules ne sait
exprimer. Deux impacts disjoints ne devraient pas s'additionner ; le dire demande une annonce qui
tienne compte de la position relative des champs, pas une table.

Restent, inchangés : A261 (l'aveuglement spatial des enveloppes de modules, désormais chiffré sur
une scène), A258 (certification f32), le coût de passe, la cadence complète de l'hôte, la loi GPU,
J2/δ général et V-noyau (ADR-127).
