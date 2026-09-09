# ADR-089 — Étendre sans interrompre

- **Statut : actée**, S133, 2026-09-10, autonomie technique S71.
- **Prolonge :** admission dynamique ADR-086, sortie de saturation ADR-087, admission
  incrémentale ADR-088.
- **Résout :** S132-1.

## Problème

Sortir d'une saturation demande un journal élargi, donc un nouveau contrôleur. S131 a mesuré ce
que cela coûte : l'élargissement lui-même est gratuit et n'interrompt rien, mais la
reconstruction prive l'hôte de champ pendant **12,21 ms** à 224×128 — une préparation complète —
et S131 a conclu que cette fenêtre était incompressible.

**Elle ne l'est pas, et la conclusion tenait à la forme supposée de la sortie**, pas à une
propriété du calcul. Deux choses le montrent : les coefficients publiés restent valides pour
toutes les sources sauf celle qui était en attente, et ADR-088 sait déjà ajouter une source à
un champ existant, exactement, quand elle vient en dernier.

## Ce qui décide de la forme

La question n'est pas comment recycler les coefficients — `add_segments` le fait — mais **qui
tient le champ pendant l'opération**.

Une méthode qui consommerait le contrôleur raccourcirait la fenêtre sans la supprimer, et
surtout : en cas de refus, l'hôte aurait perdu son champ pour rien, alors que le refus est
précisément le moment où il en a besoin.

Une méthode qui **lit** le contrôleur existant et en construit un second sur des pools fournis
par l'hôte n'a aucun de ces défauts. L'ancien sert jusqu'au basculement, le nouveau naît prêt,
et un refus ne coûte que le temps passé.

## Décision

**`Controller::extend_into(&self, journal, active, spare)`** construit un second contrôleur sur
un journal élargi, en repartant des coefficients déjà publiés, sans toucher au premier.

- L'ancien contrôleur reste **intact et servable** pendant toute l'opération, et après.
- Le nouveau publie **au même instant** que l'ancien : c'est la condition pour que les
  coefficients repris soient ceux de cet instant.
- Le résultat est celui d'une préparation complète du journal élargi, **au bit près**.

**La condition d'ordre est vérifiée, jamais supposée.** Le raccourci ne s'applique que si le
journal élargi contient exactement les sources de l'ancien, dans le même ordre, **plus une en
dernière position**. Sinon la préparation complète prend le relais — comme `admit` le fait
depuis ADR-088, et pour la même raison : le résultat doit être celui de la voie directe, sinon
le déterminisme d'I-03 tombe.

**Le prix est un second jeu de pools**, le temps de la transition. C'est un choix de l'hôte, qui
les fournit ; il échange de la mémoire contre la continuité du service. Rien ne l'oblige à le
faire — `Controller::new` reste disponible et reste le chemin quand la mémoire prime.

## Ce que cette décision ne fait pas

Elle ne supprime pas le coût, elle le déplace hors du chemin critique : le nouveau contrôleur
coûte toujours au moins une source à préparer, mais l'hôte continue d'être servi pendant ce
temps.

Elle ne transporte que des coefficients d'un pool à l'autre, jamais des sources : le journal
élargi vient de `copy_into`, et rien ici ne l'admet ni ne le modifie.

Elle ne traite pas le cas de plusieurs sources ajoutées à la fois, ni le retrait — qui n'existe
toujours pas. Elle ne change aucun résultat : les hachages de campagne doivent rester
identiques, et c'est la vérification qui accompagne la construction.

## Réception

[EXTENSION-S133](../validation/EXTENSION-S133.md). Le champ du nouveau contrôleur est comparé en
bits à une préparation directe du journal élargi ; l'ancien est réinterrogé **après** l'opération
pour vérifier qu'il sert toujours le sien ; et le cas d'insertion au milieu est exercé pour que
la préparation complète soit réellement empruntée, avec le même résultat.
