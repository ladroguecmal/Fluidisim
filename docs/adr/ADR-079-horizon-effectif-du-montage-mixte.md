# ADR-079 — Horizon effectif du montage mixte

- **Statut : actée**, S119, 2026-09-09, autonomie technique S71.
- **Prolonge :** requête mixte ADR-077, contrôleur de publication ADR-078.
- **Résout :** A194, constaté en S118 — S118-1.

## Problème

Le contrôleur de pression valide **sa** fenêtre. Les champs d'impact ont leur propre
validité. Les deux ne coïncident pas, et rien ne les rapprochait : en S118, `update(6 s)`
réussit sur une fenêtre de pression qui va jusqu'à 8 s, alors que les impacts du montage
expirent à 4 s. La vue publiée est finie et parfaitement utilisable pour la pression seule ;
c'est la requête mixte qui refuse ensuite, sur `renewal_deadline`.

Un hôte qui lit « publication réussie » et en conclut « je peux échantillonner » se trompe.
Il ne peut le découvrir qu'après avoir payé une préparation complète — 12,6 ms à 224×128 —
et rien dans le type ne l'en avertit.

Le fond du problème n'est pas le refus : il est **tardif**. Toutes les données nécessaires
pour le prononcer plus tôt sont disponibles avant la publication — le fond lié, les impacts
préparés, la fenêtre du contrôleur. Ce qui manquait, c'est de les avoir mises ensemble.

## Décision

Deux fonctions dans `prepared_water::mixed`, à côté de la requête qu'elles annoncent.

**`horizon(impacts, pressure) -> Option<(SimTime, SimTime)>`** rend la fenêtre des dates que
les contrôles de montage acceptent : l'intersection de la fenêtre du contrôleur et de la
validité du plus court des champs d'impact. `None` signifie qu'aucune date ne convient —
l'intersection est vide, et c'est un état légitime, pas une erreur d'appel. Sans contrôleur
de pression, la borne haute est celle des impacts seuls.

**`state(bound, impacts, pressure, requested) -> State`** répond, avant toute publication,
à « que se passera-t-il si j'échantillonne à cet instant ». Six réponses, une par cause de
refus **indépendante des points** :

| `State` | ce que l'hôte apprend |
|---|---|
| `Ready` | la requête passera ses contrôles de montage |
| `NeedsUpdate { published }` | date servable, mais la pression est publiée ailleurs |
| `OutsideWindow { start, end }` | hors de la fenêtre du contrôleur |
| `ImpactsExpired { id, until }` | au-delà de la validité d'un champ d'impact |
| `LossKnown` | le journal d'impacts a une perte connue : aucune date ne convient |
| `Context` | montage incohérent : aucune date ne convient |

`Ready` est une promesse sur les contrôles de montage, **pas** sur les points : pente totale,
capacité des tampons et domaine de chaque position restent évalués par la requête, parce
qu'ils dépendent de ses arguments et non du montage.

**Une seule implémentation, deux appelants.** Les contrôles ne sont pas recopiés dans
l'annonce : ils sont extraits de `sample_world_batch` dans une fonction interne unique, que
la requête consulte et traduit en ses erreurs, et que `state` rend telle quelle. C'est la
seule construction qui rend l'équivalence vraie *par structure* plutôt que *par vigilance* —
deux implémentations du même contrôle divergent, et le dépôt connaît le prix de cette
divergence ailleurs (**L137**).

L'ordre d'évaluation est celui de la requête avant ce changement, à la variante près :
contexte des impacts, perte connue, validité des impacts, contexte de la pression, puis
instant. Ce qui était un unique `Time` sur l'instant se scinde en `OutsideWindow` et
`NeedsUpdate`, que la requête retraduit toutes deux en `Time`. **Aucun refus de la requête
ne change de nature.**

`Controller::context()` est ajouté : sans lui, l'annonce ne peut pas lire la fenêtre qu'elle
doit intersecter.

## Ce que cette décision ne fait pas

**Elle n'empêche pas la publication tardive, et c'est délibéré.** `update(6 s)` continue de
réussir : la vue produite est correcte pour la pression seule, et le contrôleur n'a pas à
connaître les impacts. Faire dépendre le contrôleur du montage l'obligerait à emprunter les
impacts pour toute sa vie, donc à figer leur renouvellement — un couplage plus coûteux que
le problème qu'il résout. L'horizon est calculé **à la demande**, par qui connaît les deux.

Elle ne renouvelle rien, ne prolonge aucune fenêtre et ne choisit aucune date à la place de
l'hôte. Elle ne dit rien de la précision, ni d'un budget. Sans pression, la borne basse rendue
est `SimTime(0)` : la naissance des champs d'impact produit un refus **par point**, hors du
champ de l'annonce.

## Réception

[HORIZON-MIXTE-S119](../validation/HORIZON-MIXTE-S119.md). La propriété qui compte n'est pas
que l'annonce existe, mais qu'elle **coïncide avec le comportement réel** : un balayage
d'instants compare `state(t)` à ce que fait vraiment la séquence — publier si nécessaire, puis
requêter — et vérifie l'équivalence dans les deux sens, y compris pour les montages
inutilisables. Une annonce qui se contenterait d'être prudente passerait un test de sûreté
et échouerait celui-ci.
