# ADR-109 — Le repliement est une infidélité, pas une faute

Statut : acté, S158, 2026-09-10, délégation technique. Complète ADR-107 et ADR-108, et requalifie
A214 sans la clore ; aucune décision antérieure n'est modifiée, aucun ADR réécrit.

## Constat

A214 réclame depuis S156 un garde-fou contre le repliement d'un champ de sillage. S157 a montré
qu'aucune loi ne permet de le construire, et qu'il manquait une **tolérance**. S158 devait la
dériver. L'inventaire des consommateurs donne autre chose, et de plus utile : **la question de la
tolérance n'est pas la première à poser.**

### Les consommateurs qui lisent une borne sont immunisés

La récurrence rephase les modes ; elle ne change pas l'amplitude des coefficients. Toute grandeur
calculée depuis les coefficients est donc insensible, et cela se mesure.

| grandeur | radial 128 contre 512, 8 s | à 60 s |
|---|---:|---:|
| enveloppe de pente `slope_envelope` | 7,5e-5 | **6,2e-3** |
| énergie `energy_j` | 2,5e-4 | **4,2e-4** |
| champ **échantillonné** en champ proche (S156) | 2,4e-2 | **facteur 48** |

**Environ huit mille fois plus sensible d'un côté que de l'autre.** Conséquence directe : le
déclencheur d'écume, qui passe par `slope_envelope` et non par un échantillon
(`bound_pressure`, composition de `base.steepness·π + slope_envelope`), **ne voit pas le
repliement**. Le budget de pente d'ADR-080 non plus. L'admissibilité non plus.

C'est une propriété de conception, pas une chance : **un consommateur qui lit une borne est immunisé
contre une erreur de phase ; un consommateur qui lit un échantillon ne l'est pas.** La règle vaut
au-delà de ce cas.

### L'erreur est identique chez tous, donc elle ne peut pas diviser

Le repliement est **déterministe** : I-03 garantit le bit à bit entre plateformes, et c'est le
même calcul partout. Il ne produit donc ni désynchronisation, ni divergence de réplique, ni
inégalité entre joueurs. I-15 — une grandeur dérivée est autoritaire si tous la calculent à
l'identique — reste satisfait **avec l'erreur dedans**. L'autorité est préservée ; seule la
fidélité souffre.

Ce n'est pas un détail de vocabulaire. Un garde-fou contre une faute protège d'un dommage ; un
garde-fou contre une infidélité arbitre une apparence. Les deux n'ont ni la même urgence, ni le
même juge, ni le même coût d'erreur.

### Le juge de la fidélité existe dans le corpus, et n'a pas siégé

**B4** — « à partir de quel rapport la décomposition additive devient-elle *visiblement* fausse ? »,
protocole incluant une « perception en double aveugle », valeur de départ 0,35·Hs explicitement
provisoire. B4 est conçu pour produire exactement le type de nombre qu'A214 réclame. Il est bloqué
parce qu'il exige la référence substitutive intégrale, qui n'existe pas.

Aucun autre consommateur n'a de seuil déclaré que le repliement pourrait franchir. Le seul seuil
chiffré du voisinage — la cambrure limite de Stokes 1/7, SPEC-001 §3 — appartient au déclencheur
d'écume, donc à une borne, donc à ce qui est immunisé ; et les pentes du sillage mesuré, 5,6e-3,
en sont à deux ordres de grandeur.

## Décision

**1. Requalifier le mode de défaillance.** Le repliement d'un champ de sillage est une
**infidélité**, non une faute : il n'attente ni au déterminisme, ni à l'autorité, ni à l'égalité
entre participants. Toute décision future à son sujet part de là.

**2. Ne rien ajouter en production.** Aucun consommateur n'a demandé d'estimateur, et ajouter une
API publique pour instrumenter est précisément ce que le dépôt s'interdit depuis S132. Le coût
serait par ailleurs réel : deux préparations au lieu d'une.

**3. Publier la méthode d'estimation avec sa fidélité mesurée**, pour l'intégrateur qui voudra
mesurer son erreur sans référence plus fine — donc sans buter sur le plafond de 512 qui rend la
référence indisponible.

> Comparer `radial` et `radial + 1`. Les deux ont presque la même erreur de quadrature et des
> périodes spatiales différentes ; leur écart isole le repliement. Aucune extension de la
> grammaire de recette n'est nécessaire.

Fidélité mesurée : l'estimateur suit l'erreur réelle **à un facteur 2,5 près dans le régime où
elle est significative**, et il la **sous-estime** — le mauvais sens pour un garde-fou, et c'est
écrit avec lui. Là où il s'effondre (facteur 43 à radial 256 et 8 s), l'erreur réelle vaut 0,2 % :
il est aveugle là où il n'y a rien à voir.

**4. A214 change de dépendance.** Elle ne réclame plus une mesure (S157), ni une spécification que
nous pourrions écrire (S158) : elle attend **B4**, seul juge de fidélité du corpus. En attendant,
la conduite à tenir est celle d'ADR-107 — rester dans le domaine déduit de la recette — et le
recours est l'estimateur ci-dessus.

## Ce que cette décision ne dit pas

- **Elle ne dit pas que le repliement est sans conséquence.** Un facteur 48 sur le champ visible
  est un défaut grossier. Elle dit qu'il se juge à l'œil et non au théorème, et que le dispositif
  pour le juger est nommé.
- **Elle ne débloque pas B4**, qui attend une référence substitutive intégrale — un chantier sans
  rapport avec le sillage.
- **Elle ne généralise pas l'immunité** : elle vaut pour les erreurs de **phase**, qui laissent les
  amplitudes intactes. Une erreur d'amplitude traverserait les bornes comme les échantillons.
- **Elle ne touche ni le code, ni les invariants** : rien n'est modifié en production.
