# ADR-258 — Trente-et-unième revue de méthode (S631–S635)

- **Statut : actée**, S636, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-257](ADR-257-trentieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | suite |
|---|---|---|
| S631 | la revue (ADR-257 : les bornes d'un montage, assertées) | — |
| S632 | le déferlement le long des rayons : bornes assertées (le départ en eau profonde, la coupure sous la profondeur de déferlement) — première application d'ADR-257 D1 ; tenu du premier essai | **rien à changer** |
| S633 | les sommets le long d'un faisceau : le montage de S632 réemployé, ses bornes rappelées ; tenu du premier essai | rien à changer |
| S634 | le courant de marée avec frottement et Coriolis : jugé contre une intégration indépendante (ADR-239) et deux résultats analytiques ; bornes assertées (le transitoire éteint, `f < ω`) ; tenu du premier essai | rien à changer |
| S635 | le dégel par le bilan d'énergie : l'état exact est le temps cumulé (ADR-245), la borne de la fenêtre assertée ; tenu du premier essai | rien à changer |

Aucune friction : aucune relance, aucun critère manqué, aucune exploration abandonnée. Les protections des cinq dernières revues
(ADR-253 à ADR-257) ont toutes servi au moins une fois sans coût.

## 2. Décision

**Aucune règle nouvelle.** La méthode est laissée en l'état ; la revue suivante dira si une friction revient.

## 3. La prochaine revue

S641.
