# ADR-238 — Treizième revue de méthode (S541–S545)

- **Statut : actée**, S546, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-237](ADR-237-douzieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un plan non committé avant le travail** (S544) — la règle la plus ancienne de la méthode, sautée dans l'élan d'une suite de sessions courtes ; rien ne l'arrêtait | un critère non redéclaré | **outil** (D1) : le rituel refuse sans commit « Snnn P1 » |
| **Une limite affirmée sans vérification, et fausse** : « les formes volumiques de V frôlent le débordement des entiers » (S544) — `Tetrahedron` calcule en i128 sur ± 4 096 m ; corrigée le lendemain (S545) — la famille d'ADR-232 D2 et du « blocage hérité qui se vérifie dans le code » (L176), dont la protection ne visait que l'hérité | une variante de C21 remise d'une session | **protection élargie** (D2) |
| Deux usages de δ refusés en route que S375 avait décrits (une colonne surchargée, une montée sur le repos d'origine) (S544) | deux essais | une fois ; la note de S375 était au bon endroit (`shift_rest`) ; **rien à ajouter** |
| C16 et sa formule recalculée (S542 : 4,40 s, pas 3,5), la rotation prédite et tenue (S543), C21 corrigé le lendemain (S545) | — | **rien à changer** |

## 2. Décisions

**D1 — Le rituel refuse une session sans commit « Snnn P1 »** (`outils/rituel.py` ; `--sans-plan "<raison>"` le dit, la raison va au
journal) : la règle passe d'une mémoire à un outil (L349).

**D2 — Un blocage, hérité ou supposé, se vérifie dans le code ou par un calcul avant d'être contourné, tranché — ou écrit** (élargit la
protection de L176).

## 3. La prochaine revue

S551.
