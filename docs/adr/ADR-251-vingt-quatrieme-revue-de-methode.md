# ADR-251 — Vingt-quatrième revue de méthode (S596–S600)

- **Statut : actée**, S601, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-250](ADR-250-vingt-troisieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un nombre promis au plan sans être calculé** (S598 : « le nombre d'agrandissements, compté par le script » — le script ne l'avait pas compté) ; **une attente d'essai que le plan n'avait pas** (S599 : « une ligne par rangée pour chaque état » — la référence du plan plaçait elle-même une ligne hors de la grille). Deux fois rattrapés par les notes (ADR-244 D1), avant de juger | deux notes, aucune fausse mesure | **protection élargie** (D1) |
| Les tailles d'ensemble calculées (S597, ADR-250 D1) ; les comptes exacts par le script (S600) ; les formules indépendantes (S598, S599) | — | **rien à changer** |

## 2. Décision

**D1 — Avant d'écrire l'essai, le plan se relit contre ses critères** : chaque nombre que les critères exigent est-il calculé par le script
et écrit ? chaque assertion que l'essai portera est-elle un critère du plan ? Ce qui manque passe d'abord aux notes (ADR-244 D1), avec sa
valeur calculée (élargit ADR-243 D1 et ADR-244 D1).

## 3. La prochaine revue

S606.
