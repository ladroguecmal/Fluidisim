# ADR-288 — Cinquante-troisième revue de méthode (S741–S745)

- **Statut : actée**, S746, 2026-10-09 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-287](ADR-287-cinquante-deuxieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S741 | la revue (ADR-287 : le banc canonique) | — | — |
| S742 | **un essai de propagation avant l'essai de repos** : la levée tournait sur un fond lisse qui ne tient pas le repos (S743) | un essai de 18 min sans conclusion | **D1** |
| S742 | **une amplitude sous le quantum de pose** (15 mm ; une couche vaut 12,5 mm), contre ADR-236 ; **une crête suivie par le maximum global**, détournée par toute bosse parasite | le même essai | **D2** |
| S743 | **le lot passé sans ses lignes** : `fermer.py --lot` l'a accepté ; les lignes ont été écrites en S744 | une session de retard sur la feuille de route | **D3**, fait |
| S744 | **une prémisse « au bit » non vérifiée** : la nominale aux murs du domaine dépasse 1, la densité consciente change donc le canal | un critère faux | **D4** |
| S740, S742 | le piège d'ADR-223 D4 encore (un `\n` dans un *heredoc*), corrigé par l'outil d'édition, sans dommage | quelques minutes | aucune règle neuve |
| S743–S745 | chaque essai a désigné une cause : le repos perdu par la projection, le fond compté, la surface dans la lame mince | — | la méthode a tenu |

## 2. Décisions

**D1 — Le banc canonique a un ordre : le repos d'abord.** Chaque configuration nouvelle (un fond, une option, une variante) passe le repos
avant toute propagation, levée ou remontée. Un essai de propagation sur une configuration qui n'a pas passé le repos ne conclut rien.

**D2 — Une amplitude se pose au-dessus de trois quanta, et une crête se suit dans une fenêtre.** Le plan écrit, à côté de chaque amplitude,
le quantum de la pose ou de la lecture : `dx/2` pour la pose d'APIC (ADR-236). Une crête se lit dans une fenêtre qui suit sa position
attendue (celle d'un témoin, ou de la célérité), jamais par le maximum global d'un champ bruité.

**D3 — `rituel.py fin --lot` refuse sans la ligne de lot** de la feuille de route qui finit par la session (« **S…–Snnn** »). C'est fait en
S746, et éprouvé par un faux appel.

**D4 — Une prémisse « au bit » se vérifie d'abord sur un petit cas.** Un critère qui affirme qu'un changement ne touche pas un montage
(« identique au bit ») se vérifie par le raisonnement sur le chemin du code, puis par un cas de quelques secondes, avant l'essai long.

## 3. La prochaine revue

S751.
