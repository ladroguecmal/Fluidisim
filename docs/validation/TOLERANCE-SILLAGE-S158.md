# D'où viendrait la tolérance d'un champ de sillage — S158, 2026-09-10

Sondes, depuis `code/` : `cargo run -p water-core --release --example wake_long` (bornes),
`cargo run -p water-core --release --example wake_estimator` (estimateur).
Test de réception : `cargo test -p water-core bornes_insensibles`, debug et release.
Décision : [ADR-109](../adr/ADR-109-le-repliement-est-une-infidelite-pas-une-faute.md).
Suite de [LOI-DUREE-S157](LOI-DUREE-S157.md) et d'A214.

S157 laissait deux voies : sortir la mesure de sa dégénérescence, ou obtenir la tolérance. La
seconde a été prise, parce que la première mesurerait plus finement une quantité dont S157 a montré
que la définition dépend d'une convention non écrite. ADR-028 : il n'y a personne à qui demander,
donc « obtenir » voulait dire **dériver**, avec provenance.

**La dérivation n'a pas eu lieu, et l'inventaire qui devait la préparer a donné mieux.**

## 1. Les consommateurs qui lisent une borne sont immunisés

La récurrence rephase les modes ; elle ne touche pas à l'amplitude des coefficients. Toute grandeur
calculée depuis les coefficients y est donc insensible — argument vérifiable, donc vérifié.

Radial 128 contre 512, angulaire 512, même sillage :

| grandeur | 8 s | 60 s |
|---|---:|---:|
| enveloppe de pente `slope_envelope` | 7,5e-5 | **6,2e-3** |
| énergie `energy_j` | 2,5e-4 | **4,2e-4** |
| champ échantillonné, champ proche (S156) | 2,4e-2 | **facteur 48** |

**Environ huit mille fois plus sensible d'un côté que de l'autre.** Le déclencheur d'écume passe
par `slope_envelope` — composition de `base.steepness·π + slope_envelope` — donc il ne voit pas le
repliement. Le budget de pente d'ADR-080 non plus, l'admissibilité non plus.

Ce n'est pas une chance, c'est une propriété : **un consommateur qui lit une borne est immunisé
contre une erreur de phase ; un consommateur qui lit un échantillon ne l'est pas.**

## 2. L'erreur ne peut pas diviser

Le repliement est déterministe : I-03 garantit le bit à bit, et c'est le même calcul chez tous les
participants. Ni désynchronisation, ni divergence de réplique, ni inégalité entre joueurs. I-15 —
une grandeur dérivée est autoritaire si tous la calculent à l'identique — reste satisfait **avec
l'erreur dedans**.

D'où la requalification d'ADR-109 : le repliement est une **infidélité**, pas une faute. Un
garde-fou contre une faute protège d'un dommage ; un garde-fou contre une infidélité arbitre une
apparence. Ni la même urgence, ni le même juge, ni le même coût d'erreur — et **poser la question
de la tolérance avant celle du mode de défaillance était l'ordre inverse du bon**.

## 3. Le juge existe, et n'a pas siégé

**B4** demande « à partir de quel rapport la décomposition additive devient-elle *visiblement*
fausse ? », prévoit une « perception en double aveugle », et annonce sa valeur de départ 0,35·Hs
comme provisoire. C'est exactement le type de nombre qu'A214 réclame. B4 est bloqué : il exige la
référence substitutive intégrale, qui n'existe pas.

Aucun autre consommateur n'a de seuil déclaré que le repliement pourrait franchir. Le seul seuil
chiffré du voisinage — cambrure limite de Stokes 1/7, SPEC-001 §3 — appartient au déclencheur
d'écume, donc à une borne, donc à l'immunisé ; et les pentes du sillage mesuré, 5,6e-3, en sont à
deux ordres de grandeur.

## 4. Ce qu'un intégrateur peut faire en attendant

Mesurer son erreur sans référence plus fine, donc sans buter sur le plafond de 512 :

> comparer `radial` et `radial + 1`. Presque la même erreur de quadrature, des périodes spatiales
> différentes — `2π·radial/cutoff` contre `2π·(radial+1)/cutoff`. Leur écart isole le repliement.

Aucune extension de grammaire nécessaire. Fidélité mesurée, contre une référence quatre fois plus
fine, rapportées à l'amplitude du champ fin :

| radial | 8 s | 24 s | 40 s | 60 s |
|---:|---:|---:|---:|---:|
| 64 | 0,89 | 0,71 | 0,54 | 0,40 |
| 128 | 1,17 | 0,97 | 1,08 | 0,59 |
| 256 | **0,02** | **0,08** | 0,70 | 1,37 |

Dans le régime où l'erreur est significative, l'estimateur la suit **à un facteur 2,5 près, et il
la sous-estime** — le mauvais sens pour un garde-fou, et cela s'écrit avec lui. Là où il s'effondre
— radial 256 à 8 et 24 s — l'erreur réelle vaut 0,2 % : il est aveugle exactement là où il n'y a
rien à voir, parce que l'erreur y est de **quadrature** et non de repliement, et que deux voisins
la partagent.

Coût : deux préparations au lieu d'une. Rien n'est ajouté en production, aucun consommateur ne
l'ayant demandé — S132.

## 5. Ce qui est reçu

Un test, `bornes_insensibles_au_repliement_s158` : à 60 s, enveloppe et énergie s'écartent de
5,27e-3 et 4,24e-4 quand le champ échantillonné est 48,4 fois plus grand. Trois assertions liées,
dont la dernière interdit au test de passer sur un champ où il ne se passe rien. Témoin vérifié.

C'est un témoin de **conception** : si le déclencheur d'écume lisait un jour un échantillon au lieu
de l'enveloppe, il tomberait. C'est la raison de l'écrire, plus que la vérification d'aujourd'hui.

## 6. Portée et suite

Une seule fixture de sillage, un seul sigma. L'immunité démontrée vaut pour les erreurs de
**phase** ; une erreur d'amplitude traverserait les bornes comme les échantillons, et rien ici ne
dit le contraire.

**A214 change de dépendance** : elle ne réclame plus une mesure (S157) ni une spécification que
nous pourrions écrire (S158) ; elle attend **B4**. La conduite à tenir reste celle d'ADR-107.

**S158-1, prochaine S159 :** le facteur 2,5 revient partout dans ce problème — étendue du
groupement de S157, dispersion de l'estimateur ici. Vérifier si c'est le plafond de précision de
tout ce qui touche au repliement, ou une coïncidence entre deux mesures indépendantes. Peu cher,
les deux jeux de données existent. À défaut, la voie restée ouverte est celle qu'ADR-109 nomme :
débloquer B4, chantier sans rapport avec le sillage.

299 tests réussis, cinq ignorés. 109 ADR, 214 angles, 239 leçons, 18 invariants, 6 SPEC, 23 cas.

## Note du 2026-09-10 (S160) — deux corrections à ce rapport

**Le « facteur 2,5 » de la §4 est une *déviation* au rapport idéal 1, pas une étendue.** L'étendue
du même tableau vaut 3,42. La distinction n'est pas académique : c'est elle qui a fabriqué la
coïncidence avec le 2,5 de S157 — qui est, lui, une étendue — et qui a coûté la session S160.
Publier un « facteur » sans dire quelle statistique il désigne rend deux nombres comparables qui
ne le sont pas.

**Le régime écarte deux cases pour un motif inexact.** Le texte dit « l'erreur réelle vaut 0,2 % »
pour radial 256 à 8 s **et** 24 s. C'est vrai à 8 s (2,1e-3) et faux à 24 s : l'erreur y vaut
8,7e-2, quarante fois plus. Avec un seuil uniforme à 1 %, la case revient et la déviation passe de
2,47 à **12,30**.

**Et la fidélité dépend de la bande conservée.** Elle vaut 2,17 à 2,47 pour un cutoff de 6, sur un
facteur 4 en sigma — donc le nombre est robuste dans sa famille — mais **24,08** à cutoff 1,5. Un
appelant qui lirait « facteur 2,5 » hors de cette famille se tromperait d'un ordre de grandeur.

Ce qui tient sans réserve : l'estimateur **sous-estime**, et il est aveugle là où l'erreur est de
quadrature. Voir [FACTEUR-25-S160](FACTEUR-25-S160.md).
