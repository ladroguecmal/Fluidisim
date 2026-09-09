# ADR-080 — Annonce des points du montage mixte

- **Statut : actée**, S120, 2026-09-09, autonomie technique S71.
- **Prolonge :** requête mixte ADR-077, annonce du montage ADR-079.
- **Résout :** A196 en partie — ce qui est décidable sans évaluer.

## Problème

ADR-079 rend prévisible ce qui dépend du montage et de l'instant. Le reste — domaine de
chaque position, pente totale, capacité — est évalué par la requête, point par point. Et la
requête est **atomique** : un seul point hors domaine coûte le lot entier. L'hôte découvre
donc après avoir payé la préparation, et il perd tout, pas seulement le point fautif.

## Ce que l'inventaire a montré, et qui change la forme de la réponse

L'inventaire (BORNES-POINTS-S120 §1) contredit la formulation de la suite S119-1, qui
attendait des *bornes*. Deux des trois conditions par point ne demandent pas de borne :

**Les conditions géométriques sont exactes et bon marché.** Domaine de B, portée des impacts,
emprise de la pression : ce sont des comparaisons, pas des sommes sur les modes. Le prédicat
se **transpose** — il peut être posé avant l'évaluation, au lieu d'être approché par une boîte.
Une AABB serait strictement moins informative qu'un prédicat exact qui coûte le même ordre.

**La pente, elle, demande bien une borne**, mais une borne d'un genre précis : dans
`steepness_B(point)·π + Σ slope_bound + slope_envelope`, un seul terme dépend du point, et il
est positif ou nul. La somme des autres est donc un **plancher** que l'enveloppe ne peut pas
passer sous.

## Décision

Deux fonctions dans `prepared_water::mixed`.

**`admits(bound, impacts, pressure, point) -> bool`** — vrai si le point satisfait les trois
conditions géométriques. Garanties, dans les deux sens et pas seulement le sens prudent :

- `admits(p) == false` ⟹ la requête refusera ce point, avec `Domain`. L'hôte peut filtrer.
- `admits(p) == true` ⟹ **aucun refus géométrique** sur ce point. Un refus reste possible si
  une couche produit une sortie non finie — `RadialImpact::sample` rend d'ailleurs `Domain`
  dans ce cas aussi, ce qui interdit de promettre plus.

**`slope_floor(impacts, pressure) -> f32`** — la part de l'enveloppe de pente indépendante des
points. Garantie : si `max_slope < slope_floor(...)`, la requête refusera tout lot non vide,
quels que soient les points. La réciproque est fausse et n'est pas revendiquée : au-dessus du
plancher, c'est la raideur de B au point qui décide.

**Prédicats descendus dans les couches, pas recopiés dans l'annonce.** `admits` ne réécrit
aucune condition : chaque couche expose le prédicat de domaine qu'elle applique déjà et
l'utilise elle-même — `RadialImpact::admits`, `Field::admits`, `Background::admits`. `admits`
les compose. C'est la même exigence qu'ADR-079 : l'équivalence entre l'annonce et le
comportement doit tenir *par structure*, parce qu'une annonce vérifiée seulement par vigilance
finit par mentir (**L137**). La différence avec ADR-079 est que la factorisation ne pouvait pas
se faire par extraction dans `mixed` : les conditions vivent dans trois couches distinctes, et
c'est là qu'il fallait aller les poser.

## Ce que cette décision ne fait pas

Elle ne rend pas la requête partielle : l'atomicité d'ADR-077 n'est pas rouverte, et un lot
contenant un point inadmis échoue toujours en entier. `admits` sert à composer un lot valide,
pas à en sauver un.

**Aucune boîte englobante n'est construite.** Une AABB serait utile pour engendrer des points
plutôt que les tester, mais elle demanderait d'exposer l'ancre de `Background` — une décision
sur B, pas sur le montage mixte — et aucun consommateur ne la demande aujourd'hui. Point
ouvert daté plutôt que code sans usage.

Elle ne dit rien de la précision, ni de la finitude des calculs, ni d'un budget. `slope_floor`
n'est pas une borne de la pente réelle : c'est un plancher de l'enveloppe déclarée.

## Réception

[BORNES-POINTS-S120](../validation/BORNES-POINTS-S120.md). La propriété reçue est
l'équivalence, sur un balayage de points couvrant chaque frontière et ses deux côtés, entre
`admits(p)` et l'existence d'un refus géométrique de la requête sur `[p]`. Un montage
numériquement dégénéré vérifie la limite annoncée : `admits` reste vrai et la requête refuse
pour non-finitude — l'annonce ne promet pas ce qu'elle ne peut pas tenir.

## Note corrective du 2026-09-09, à la construction (S120, P5)

La décision annonce, pour `admits(p) == false`, un refus « avec `Domain` ». **C'est trop
précis, et faux dans un cas** : lorsque la conversion monde/local réussit mais que la borne
locale de 4096 m ne passe pas, `eval_local` rend `None` et la requête nomme ce refus
`InvalidBackground`, pas `Domain`. Les deux frontières du fond ne sont pas exactement la même,
et elles ne portent pas le même nom d'erreur.

La garantie exacte est donc : `admits(p) == false` ⟹ **la requête refuse ce point**, par
`Domain` ou par `InvalidBackground` selon la couche qui borne. Le sens de la garantie est
inchangé ; seul le nom de l'erreur était mal annoncé. Le test de réception vérifie l'appartenance
à ces deux causes.

**Second point, sur la Réception ci-dessus.** Elle annonce un montage numériquement dégénéré
où `admits` resterait vrai pendant que la requête refuse pour non-finitude. **Ce montage n'a pas
été construit.** La limite est bien vérifiée, mais par la **pente totale** : un point admis que
la requête refuse sur son enveloppe. La démonstration porte donc sur « `admits` ne promet pas
l'acceptation », pas spécifiquement sur la non-finitude. Écrire l'inverse aurait annoncé une
réception qui n'a pas eu lieu.

## Point ouvert daté — 2026-09-09

Aucune boîte englobante n'est fournie. Un hôte qui voudrait **engendrer** des points dans la
zone servable, plutôt que les tester, n'a pas de quoi la situer : `Background` n'expose pas son
ancre, et sans elle une boîte en coordonnées locales n'est pas convertible en positions monde.
Exposer l'ancre est une décision sur B, à prendre le jour où un consommateur la demande.
