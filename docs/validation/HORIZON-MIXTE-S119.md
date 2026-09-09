# S119 — Horizon effectif et annonce du montage mixte

2026-09-09. [ADR-079](../adr/ADR-079-horizon-effectif-du-montage-mixte.md) actée. A194 résolue.

## Ce qui est construit

`prepared_water::mixed` expose deux fonctions, utilisables **avant** toute publication :

- `horizon(impacts, pressure)` — la fenêtre des dates que les contrôles de montage acceptent,
  `None` si l'intersection est vide ;
- `state(bound, impacts, pressure, requested)` — six réponses, une par cause de refus
  indépendante des points.

Elles ne dupliquent rien : les contrôles ont été extraits de `sample_world_batch` dans une
fonction interne unique, que la requête consulte et traduit en ses erreurs. L'ordre
d'évaluation est celui d'avant, à ceci près que l'unique `Time` sur l'instant se scinde en
`OutsideWindow` et `NeedsUpdate` — que la requête retraduit toutes deux en `Time`.
**Les 154 tests antérieurs passent inchangés, et les hachages de la campagne S118 sont
identiques** : aucun refus n'a changé de nature, aucun résultat numérique n'a bougé.

`Controller::context()` est ajouté ; sans lui, l'annonce ne peut pas lire la fenêtre.

## La propriété reçue

Ce qui compte n'est pas que l'annonce existe, mais qu'elle **coïncide avec le comportement
réel**. Une annonce seulement prudente — « dans le doute, pas prêt » — passerait un test de
sûreté et serait inutile. Le test balaie douze instants et confronte chaque annonce à ce que
la séquence fait vraiment : publier si nécessaire, puis requêter à lot vide.

- `Ready` ou `NeedsUpdate` ⟹ la publication réussit, l'annonce devient `Ready`, la requête
  passe. Aucun `Ready` ne ment.
- `ImpactsExpired` ⟹ la publication peut réussir — c'est exactement A194 — mais la requête
  refuse toujours, et l'annonce ne repasse jamais à `Ready`.
- `OutsideWindow` ⟹ la publication elle-même refuse.
- Hors horizon ⟺ `ImpactsExpired` ou `OutsideWindow`, dans les deux sens.

Deux montages supplémentaires couvrent ce que la fixture de référence n'atteint pas :
impacts valides 10 s contre fenêtre de 8 s (c'est alors la fenêtre qui borne, et
`OutsideWindow` est enfin exercé), puis impacts éteints à 1 s contre fenêtre ouverte à 2 s —
**horizon vide**.

## Ce que la construction a appris, et qui n'était pas dans la décision

**L'annonce ponctuelle ne peut pas dire qu'aucune date ne convient.** Sur le montage à horizon
vide, la cause change de côté selon la date : avant l'ouverture de la fenêtre, les impacts
vivent encore et l'annonce dit `OutsideWindow` ; après, ils sont éteints et elle dit
`ImpactsExpired`. Chaque réponse est exacte, et aucune ne révèle qu'il n'existe **aucune**
date servable. Seul `horizon` le dit, en rendant `None`.

C'est la justification des deux fonctions, et elle n'apparaît qu'en les écrivant : une
annonce par date ne peut pas répondre à une question sur toutes les dates. Portée en note
datée dans ADR-079.

**L'annonce donne la première cause, pas l'ensemble des causes.** Avec des impacts qui
expirent avant la fin de la fenêtre, `OutsideWindow` n'est jamais rendu : `ImpactsExpired`
arrive d'abord dans l'ordre. La cause annoncée est vraie, elle n'est pas unique — un hôte qui
lèverait la première trouverait la seconde.

## Coûts

| | 224×128 | 256×128 |
|---|---|---|
| annonce `state`, 1000 appels | 23,0 µs | 22,8 µs |
| `update` vers un instant qui change | 12,78 ms | 14,42 ms |
| le même, mesuré en dernier | 13,21 ms | 14,38 ms |
| préparation directe, témoin | 12,64 ms | 14,50 ms |

**23 nanosecondes par annonce**, contre 12,6 ms pour la préparation qu'elle évite : un rapport
de l'ordre de 500 000. Le coût de savoir est sans commune mesure avec celui de découvrir.

**A195 est corrigé, et la correction est vérifiée.** Un bloc de mise en régime précède
désormais la première mesure. Les trois valeurs ci-dessus coïncident maintenant, alors que
S118 lisait un écart de 15 à 28 % entre la première et les suivantes ; vérifié sur trois
exécutions. La mise en régime était donc bien la bonne correction, et pas seulement une
hypothèse plausible de plus.

## Ce qui n'est pas revendiqué

`Ready` ne promet rien sur les points : domaine, pente totale et capacité des tampons restent
évalués par la requête, parce qu'ils dépendent de ses arguments. La publication tardive reste
possible et c'est délibéré — la vue produite est correcte pour la pression seule. Aucun
renouvellement, aucune prolongation de fenêtre, aucun choix de date à la place de l'hôte.
Aucune précision spatiale nouvelle ; les mesures viennent d'une machine unique, non isolée.

## Vérification

156 core + 93 harnais = **249 tests réussis, cinq ignorés** ; les six tests mixtes passent
aussi en release. Quatre avertissements préexistants, aucun nouveau. Campagne `cycle_mixed`
reçue aux deux recettes, hachages inchangés depuis S118.

## Suite

**S119-1, S120 :** l'annonce couvre le montage, pas les points. Domaine, pente totale et
capacité restent découverts au moment de la requête, donc tardivement au sens exact où A194
l'était. Ce qui en est annonçable est une **borne** — pente maximale atteignable sur un lot,
emprise du domaine — et non un verdict par point. Fréquence relative de ces refus : non mesurée. Restent ouverts :
admission dynamique dans le contrôleur, renouvellement de fenêtre, profondeur finie de pression
(S116-2), bilan mixte, durabilité disque. A196 enregistre ce que l’annonce ne couvre pas.

79 ADR, 196 angles, 17 invariants, 6 spécifications, 23 cas.
