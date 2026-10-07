# ADR-255 — Vingt-huitième revue de méthode (S616–S620)

- **Statut : actée**, S621, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-254](ADR-254-vingt-septieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un paramètre de régularisation hérité hors de son échelle** (S620) : l'`ε` de la vitesse désingularisée, choisi en S613 à l'échelle du millimètre pour tenir Courant au front, figeait l'écart à Thacker de l'ordre deux à 0,030 dès 100² — les couches utiles du rivage y sont plus minces que 1 mm. Trouvé par la mesure de convergence du plan | une exploration au plan | **protection élargie** (D1) |
| Des gains et des seuils choisis par exploration au plan (S617 : les gains du régulateur, le nombre d'inversions) — dits comme tels, avec leur origine, avant la mesure du code | — | rien à changer (ADR-253 : un seuil fixé sur sa référence s'écrit avec son origine) |
| Des lignes de code fautives écrites puis retirées avant l'exécution (S618 : une correction sans effet, des champs mal nommés) — arrêtées par la relecture et la compilation | — | rien à changer |
| Le défaut I-06 relevé par un banc (S618) et levé à la session suivante (S619) ; les murs éprouvés par un cas mouillé (S620, ADR-254 D2) | — | **rien à changer** : tenues |

## 2. Décision

**D1 — Un paramètre de régularisation porte son échelle, et le plan la confronte à la plus petite échelle utile du cas.** Un `ε`, une
hauteur sèche, un seuil de mouillage : l'échelle qu'il fixe (1 mm, 1 µm) s'écrit à côté de lui, et un plan qui le réemploie sur un autre cas
vérifie — par une mesure de convergence ou par l'échelle du cas — qu'il ne la tronque pas (élargit ADR-237 D1).

## 3. La prochaine revue

S626.
