# ADR-293 — Cinquante-cinquième revue de méthode (S751–S755)

- **Statut : actée**, S756, 2026-10-10 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-290](ADR-290-cinquante-quatrieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S751–S752 | la méthode du bilan propre (ADR-290 D1) a réfuté R1′ en une session | — | elle a tenu |
| S752 | **une configuration décidée avant sa référence extérieure** : ADR-291 retenait R1 sur la remontée de S645, jugée contre une loi théorique ; le laboratoire l'a renversée deux sessions plus tard (ADR-292) | deux sessions, un ADR remplacé | **D1** |
| S753 | **la grandeur qui départageait n'était pas mesurée par le banc** : la célérité de l'onde. Les données du canal la portaient ; elle n'a été lue qu'après coup | une session | **D2** |
| S754 | **un critère manqué sous le quantum** (0,45 mm, vingt fois sous la lecture), levé par écrit : le critère ne portait pas sa bande de quantum | aucun résultat faussé ; une dérogation écrite | **D3** |
| S755 | **une fenêtre de mesure figée**, tirée de l'ancien déferlement (le mur sur [2,4 ; 3,2 s]) : quand l'événement s'est déplacé, la fenêtre ne le voyait plus | une mesure muette | **D4** |

## 2. Décisions

**D1 — Une configuration se décide après sa référence extérieure.** Quand des variantes se départagent, la décision (un ADR de
configuration) attend l'essai contre la référence extérieure disponible : des mesures, une solution exacte de la même physique. Une loi
théorique d'un régime voisin (S645 : 1:3, une onde posée près du pied) éclaire, elle ne tranche pas (ADR-280 D2).

**D2 — L'essai canonique d'une onde mesure aussi sa célérité**, en plus de sa forme : la place de la crête dans le temps, contre la célérité
exacte. Une onde juste en forme et fausse en vitesse arrive au mauvais endroit.

**D3 — Un critère porte sa bande de quantum.** Il s'écrit « à X près, plus le quantum de la lecture q » ; un écart entre X et X + q est
rapporté « à la limite », et ne fait ni tenir ni tomber seul. Ainsi aucune dérogation après coup.

**D4 — Une fenêtre de mesure suit l'événement qu'elle mesure** : ses bornes viennent de l'instant mesuré dans le même calcul (le
retournement, l'arrivée d'un front), jamais d'une constante tirée d'un montage antérieur.

## 3. La prochaine revue

S761.
