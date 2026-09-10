# S133 — Étendre sans interrompre : la fenêtre de S131 n'existait pas

2026-09-10. [ADR-089](../adr/ADR-089-extension-sans-interruption.md) actée. S132-1 réalisée.

## 1. Ce que S131 avait conclu, et pourquoi c'était faux

S131 a mesuré la sortie de saturation et conclu que la fenêtre sans champ — 12,21 ms à
224×128 — était **incompressible**, « il n'existe pas de chemin qui republie sans recalculer ».

La conclusion tenait à la forme supposée de la sortie, pas à une propriété du calcul. Deux
choses l'ont défaite : les coefficients publiés restent valides pour toutes les sources sauf
celle qui était en attente, et ADR-088 savait déjà ajouter une source à un champ existant.

Mais le point qui décide n'est ni l'un ni l'autre : c'est **qui tient le champ pendant
l'opération**. Une méthode qui consommerait le contrôleur raccourcirait la fenêtre sans la
supprimer — et en cas de refus, l'hôte aurait perdu son champ pour rien, au moment précis où il
en a besoin. Une méthode qui **lit** le contrôleur et en construit un second n'a aucun de ces
défauts.

## 2. Décision

[ADR-089](../adr/ADR-089-extension-sans-interruption.md) : `Controller::extend_into(&self,
journal, active, spare)` construit un second contrôleur sur un journal élargi, en repartant des
coefficients publiés, **sans toucher au premier**. L'ancien sert pendant et après ; le nouveau
naît prêt ; le résultat est celui d'une préparation complète, au bit près.

La condition d'ordre est vérifiée et non supposée : le raccourci ne s'applique que si le journal
élargi contient exactement les mêmes sources, dans le même ordre, plus une **en dernier**. Sinon
la préparation complète prend le relais, comme dans `admit` depuis ADR-088.

## 3. Réception

Le test exerce **les deux configurations** — reprise en dernier, où le raccourci s'applique, et
reprise au milieu, où il ne doit surtout pas s'appliquer — en partant chaque fois d'une vraie
saturation, d'un `copy_into` et d'un `retry`.

- Le champ du nouveau contrôleur est comparé en bits à une préparation directe du journal
  élargi : identique dans les deux cas.
- **L'ancien est réinterrogé après l'opération** : il sert toujours son champ, inchangé, et son
  journal a toujours ses sources. C'est cela qui supprime la fenêtre, et c'est vérifié plutôt
  qu'affirmé.

**Le test a été vérifié comme témoin** : condition d'ordre forcée à vrai, il échoue sur la
reprise du milieu — « reprise 2 : le champ étendu doit être celui de la voie directe ».

170 core + 93 harnais = **263 tests réussis, cinq ignorés** ; ciblé aussi en release. Hachages de
la campagne `cycle_mixed` identiques à ceux de S118.

## 4. Ce que cela change, chiffré

Médianes sur 21 mesures, après mise en régime (A195) :

| | 224×128 | 256×128 |
|---|---|---|
| élargissement + reprise, service maintenu (S131) | 0,1 µs | 0,1 µs |
| **reconstruction complète, sans champ (S131)** | **12,91 ms** | **15,91 ms** |
| extension quand le raccourci ne s'applique pas | 12,73 ms | 14,56 ms |
| **extension prolongée, service maintenu** | **6,21 ms** | **6,91 ms** |
| construction complète du même journal à trois sources | 19,11 ms | 21,69 ms |

Deux lectures, et la seconde est la vraie.

- **Le coût tombe d'un facteur 3,1** : 6,21 ms contre 19,11 ms pour obtenir le même champ à
  trois sources.
- **Mais le chiffre qui comptait n'était pas le coût.** La fenêtre de S131 n'était pas une
  dépense, c'était une **absence de service**. Elle ne raccourcit pas : elle disparaît. L'hôte
  est servi pendant les 6,21 ms, comme il l'était pendant l'élargissement.

Quand le raccourci ne s'applique pas — reprise au milieu — l'extension coûte comme la
reconstruction, aux fluctuations près. Elle garde alors son seul avantage : le service continue.

## 5. Ce qui n'est pas revendiqué

Le coût n'est pas supprimé, il est déplacé hors du chemin critique. Le prix est un **second jeu
de pools** pendant la transition : l'hôte échange de la mémoire contre la continuité, et rien ne
l'y oblige — `Controller::new` reste le chemin quand la mémoire prime.

Aucune source n'est admise ni modifiée par l'extension : le journal élargi vient de `copy_into`,
et son contenu est celui que l'hôte a décidé. Plusieurs sources ajoutées à la fois ne sont pas
traitées ; le retrait n'existe toujours pas.

Aucun résultat ne bouge : les hachages de campagne le confirment.

## 6. Suite

**S133-1, S134 :** trois sessions de suite ont buté sur la même limite implicite — une source
ajoutée n'est exacte en incrémental que si elle vient en dernier. En pratique, les identifiants
sont attribués par l'hôte, et rien ne garantit qu'ils croissent. **Mesurer d'abord** ce que
coûterait de rendre l'ordre canonique indépendant de l'ordre d'accumulation — par exemple une
sommation par nœud indépendante de l'ordre, ou une accumulation en précision étendue — avant de
décider si la condition doit rester, ou disparaître.

Restent ouverts : la transaction mixte, l'extension de fenêtre, la profondeur finie de pression
(S116-2), le bilan mixte, la durabilité disque, le générateur physique d'ADR-055.

89 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas.
