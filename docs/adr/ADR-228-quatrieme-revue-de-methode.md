# ADR-228 — Quatrième revue de méthode (S497–S500)

- **Statut : actée**, S501, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-226](ADR-226-troisieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un corps d'essai à la limite de sa stabilité** : une bouée haute et étroite (GM vrai de 2 mm pour 25 cm de côté) a chaviré (S494), roulé (S495), puis rendu un essai faux quand le proxy est devenu exact (S500) ; une part de la « dérive du second ordre » publiée en S494 était son roulis (102 % → 35 %) | trois sessions touchées, une conclusion à corriger | **protection nouvelle** (D1) — une erreur répétée |
| Les battements écrits d'avance, de tête : 03:50 pour une horloge à 03:39 (S497), 03:58 pour 03:52 (S498), 04:05 pour 04:03 (S499) | un journal qui ment de dix minutes | **fait** (D2) : le battement lu par le script qui écrit le jeton |
| Un estimateur de `\|G\|` (maxima paraboliques) bruité à 6·10⁻⁴ jugé contre un seuil de 10⁻⁹ (S498) | une reprise | l'instrument refait sans toucher au seuil — la protection « critère avant, seuil jamais relevé » a joué ; **rien à ajouter** |
| Un témoin trop fort (1 mm) sorti du régime qu'il devait montrer dès le premier pas (S498) | une reprise | un instrument s'éprouve sur un cas connu (L360) ; **rien à ajouter** |
| Un proxy à une couche accepté par compensation de deux erreurs (S499) | évité | la protection L278 a joué : la recherche « sans compensation » ajoutée ; **rien à ajouter** |
| Deux contraintes d'API découvertes à l'essai — le journal emprunte les émissions, un contrôleur refuse un journal vide (S497) | deux compilations | **rien à ajouter** |

## 2. Décisions

**D1 — Un corps d'essai se choisit loin de ses limites** : avant de mesurer avec lui, calculer sa marge (stabilité de forme `GM`, régime
`ω·dt`, rampe d'immersion) avec le proxy qu'il porte ; une marge de l'ordre de l'erreur du proxy (`BM/n²`) disqualifie le corps.

**D2 — Le battement se lit dans le script qui l'écrit** (`datetime.now()`), jamais tapé : la règle d'L237 (« l'horloge se lit dans un appel
séparé ») tenue par l'outil plutôt que par l'attention.

## 3. La prochaine revue

S506.
