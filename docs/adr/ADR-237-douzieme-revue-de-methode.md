# ADR-237 — Douzième revue de méthode (S536–S540)

- **Statut : actée**, S541, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-236](ADR-236-onzieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un nombre du plan ajusté de tête après un changement de paramètre** : le temps de Torricelli calculé avec `C_d` = 0,61, puis « corrigé » à vue pour 0,62 (S538 : 467 s pour 463,5) — la troisième fois sous ADR-232 D2 (S510, S515 avant elle) | une valeur fausse au plan, corrigée par l'essai | **protection changée** (D1) |
| Une ligne de la liste en retard de vingt-six sessions : 13.2 rangeait encore C20 parmi les cas non exécutés (passé en S512) ; rattrapée en S538 | une ligne fausse | une fois relevée ; **rien à ajouter** — la règle « une liste à tenir se confie à un outil » (L349) le couvre, et la revue suivante dira si un contrôle est dû |
| Le décompte affiché de la liste qui n'avait pas suivi un changement d'état (S538) | — | l'outil l'a arrêté (`etat_projet --check`) ; **rien à changer** |
| La loi calculée au plan avant le code (S540 : la traînée choisie sur ses vitesses) ; le point de non-retour prédit puis tenu (S539) | — | ADR-232 D2 a payé ; **rien à changer** |

## 2. Décision

**D1 — Un nombre du plan se recalcule à chaque changement d'un de ses paramètres** (élargit ADR-232 D2) : la ligne de script relancée,
sa sortie recopiée ; jamais un ajustement à vue d'un nombre déjà calculé.

## 3. La prochaine revue

S546.
