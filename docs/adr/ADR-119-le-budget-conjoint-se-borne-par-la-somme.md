# ADR-119 — Le budget d'erreur conjoint se borne par la somme ; la loi du maximum n'est pas portable

- Statut : actée, 2026-09-12, S189 ; autonomie technique S71.
- Applique SPEC-004 §6.2, ADR-118 ; mesures S186, S188, S189.
- **Remplace la règle de dimensionnement** publiée par
  [COMPOSITION-ERREURS-S186](../validation/COMPOSITION-ERREURS-S186.md) §8.5 et reprise dans
  le suivi daté d'ADR-118. Ne remplace aucun ADR ; ADR-118 reste entier.
- Ne choisit ni solveur ni seuil de justesse.

## Contrat

Un consommateur qui réduit **deux** axes à la fois — décimation spatiale `r` et cadence
temporelle `c` — dimensionne son budget d'erreur comme suit.

**1. La borne est la somme.** `eU(r, c) ≤ eU(r, 1) + eU(1, c)`, où les deux termes sont les
erreurs de chaque axe pris seul, mesurées séparément. C'est la **seule** forme portable : elle
n'a été dépassée sur aucune case jugée, sur trois géométries de réseau et trois sessions
(rapports maximaux 0,988 · 0,984 · 0,981).

**2. Le maximum n'est pas une estimation.** `eU ≈ max(eU(r,1), eU(1,c))` dépend de la
**géométrie** des deux champs d'erreur et non de la composition : il est exact quand leurs
pics sont disjoints, dépassé jusqu'à 1,71 quand ils se renforcent, et surestimateur de 27 %
quand ils s'annulent. Il ne se transporte donc pas d'un réseau à un autre, et **surtout pas
vers le réseau gradué que recommande ADR-118**, où il est rejeté.

**3. La règle « égaliser les deux axes puis s'arrêter » est abandonnée.** Elle supposait que
le maximum gouverne, donc que l'axe le moins mauvais est gratuit jusqu'à la parité. Sur le
réseau gradué ce n'est pas le cas : l'erreur composée peut valoir 0,73 fois le maximum des
deux, donc la parité n'est pas l'optimum, et rien ne dit où il est. **Dimensionner par la
borne 1, et mesurer le couple retenu** plutôt que l'inférer.

**4. Ce qui reste vrai de S186.** Les trois erreurs — spatiale seule, temporelle seule,
composée — sont mesurables séparément et leur somme borne le total : un budget conjoint reste
**licite**, et c'est ce que S186 cherchait à établir. Ce qui tombe est la façon de le
répartir, pas son existence.

**Ce que le contrat ne dit pas.** Aucun seuil de justesse, aucune valeur de `r` ni de `c`,
aucune erreur acceptable. `N` de SPEC-004 §6.2 reste à fixer, et c'est une décision, pas une
mesure.

## Pourquoi, et sur quels chiffres

[COMPOSITION-GRADUEE-S189](../validation/COMPOSITION-GRADUEE-S189.md), même bloc, même
référence et même critère de jugement `[0,80 ; 1,25]` que S186 et S188.

**Le mécanisme est mesuré, et il rend les trois « lois » caduques comme lois.** L'écart de
champ est **additif maille par maille** :
`Δu(r,c)(x) ≈ Δu(r,1)(x) + Δu(1,c)(x)`, avec un résidu d'au plus **1,93 %** de `max |u'|` —
soit **10 %** de l'erreur de la case — et **exactement nul** quand l'un des deux termes est
nul. Le comportement en norme maximum n'est donc pas une loi de composition : c'est une
conséquence de cette additivité et de la **position relative des deux pics**.

| pics | max de la somme | ce qu'on croyait lire |
|---|---|---|
| disjoints | le plus grand des deux | « loi du maximum » |
| recouvrants, mêmes signes | leur somme locale | « loi additive » |
| recouvrants, signes opposés | **moins** que le plus grand | rien — cas non prévu |

Et les deux pics ne tombent **jamais** sur la même maille : 0 cas sur 78 jugés. La
« coïncidence des maxima » qu'avançait S188 était un effet de granularité — il localisait à la
tranche, un plan de 196 mailles.

**L'attribution à la géométrie est directe**, parce que deux familles de réseau ont été
mesurées côte à côte, mêmes cadences et même critère :

| famille | maintien | extrapolation | interpolation |
|---|---|---|---|
| **graduée** — pics séparés de 6 à 10 mailles | maximum **rejeté** (0,730–1,000) | maximum (0,811–1,000) | quadratique (0,892–1,102) |
| **ancrée** — pics au même étage | maximum (0,936–1,060) | maximum (0,869–1,000) | additive, quadratique |

Le réseau gradué sépare les pics parce que la graduation pose un nœud **sur** la tranche haute
et y annule l'erreur spatiale, tandis que l'erreur temporelle reste accrochée au maximum du
**champ**, qui est en haut quoi qu'on fasse du réseau.

## Ce qu'il faudrait pour l'inverser

- **Un solveur qui projette.** La projection de pression couple toutes les mailles à chaque
  pas ; l'additivité locale — d'où tout le reste découle — y serait à remesurer. Le véhicule
  de S186–S189 est explicite, sans projection ni surface libre, et c'est précisément ce qui
  rend l'additivité plausible. **C'est la limite la plus sérieuse de cet ADR.**
- **Une métrique qui ne serait pas un maximum.** En norme quadratique, deux champs additifs
  composent quadratiquement dès qu'ils sont décorrélés, et la question de la position des pics
  disparaît. Le dépôt mesure en maximum depuis S170 ; en changer est une décision qui n'est
  pas prise ici.
- **Une borne plus serrée qui serait portable.** La somme est lâche : son rapport descend à
  0,437, donc dimensionner par elle peut coûter jusqu'à **2,3 fois** la résolution nécessaire
  (**A233**). Une estimation qui tiendrait sur toutes les géométries remplacerait la règle 1 ;
  aucune n'est connue.
- **Un contenu dont le champ ne culmine pas sur une frontière.** Toute la géométrie décrite
  ici tient à ce que `|u'|` soit maximal en haut du bloc.

## Réception et limites

Protocole : [COMPOSITION-GRADUEE-S189](../validation/COMPOSITION-GRADUEE-S189.md). Six
réceptions, dont le résidu d'additivité **exactement nul** là où il doit l'être, le témoin
ancré qui redonne S188 (1,7160 % et 3,6805 %) et la famille graduée qui redonne S187 §8.5
(6,5503 / 3,4373 / 1,7164 / 0,6393 %). Empreinte `0x30b0b9eee43f6255`, `diff` identique sur
deux exécutions. Les quatre empreintes publiées du support tiennent après le déplacement du
profil et des indices gradués dans `support/`. Workspace 331 réussis / cinq ignorés en debug
et release.

Aucun code de bibliothèque n'est modifié. Un seul montage, une seule profondeur de bloc, un
seul instant de profil, et un véhicule qui ne projette pas. A50 et B4 restent partiels ;
aucun solveur choisi.
