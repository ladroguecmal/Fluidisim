# ADR-277 — Quarante-quatrième revue de méthode (S696–S700)

- **Statut : actée**, S701, 2026-10-08 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-276](ADR-276-quarante-troisieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S696 | la revue (ADR-276) | — | — |
| S697 | le raccord du large jugé seul : trois écarts, une cause chacun (ADR-276 D2 appliqué) | — | — |
| S698 | **un ordre prescrit sauté.** ADR-273 D1 veut un raccord jugé d'abord entre deux copies du même solveur. Le plan de S698 nommait ADR-273, mais nourrissait le bord neuf directement par un autre modèle (SGN). Le résultat (−0,047 s) mêlait deux causes opposées, que S700 a départagées (+0,085 s et −0,121 s) | une session dont le chiffre ne prouvait rien ; deux sessions pour le défaire | **D1** |
| S698 | une première pose par rangée laissait un trou d'air dans la colonne d'entrée, lu comme un retournement à 0,14 s | un essai de 7 min | aucune : le lecteur a fait son office, la faute s'est vue à la première lecture |
| S699 | **une combinaison de drapeaux qui rallume un mécanisme ancien.** Le montage (`deux_raccords_porteur`) prenait trois booléens ; `particules = false` avec le rejeu rallumait en silence la zone de colonnes de S693 | un essai de 34 min | **D2** |
| S700 | `outils/essai.py` imprimait « ū » sur la console Windows (charmap) ; sa mort a tué l'essai | un essai de 13 min | corrigé dans l'outil (S700) ; le piège d'encodage est déjà en mémoire |

## 2. Décisions

**D1 — Un ADR nommé dans la ligne « ADR » du plan y dit comment il est tenu.** Pour chaque ADR qui prescrit un ordre ou une condition
(ADR-273 D1 : un raccord jugé d'abord entre deux copies du même solveur), la ligne dit en quelle session et par quel essai il est tenu,
ou pourquoi il ne s'applique pas. Nommer un ADR ne suffit pas : S698 nommait ADR-273 et sautait son premier pas.

**D2 — Un montage d'essai à plusieurs variantes prend un mode nommé, non des booléens qui se combinent.**
- Chaque mode énumère, en commentaire, les mécanismes qu'il allume.
- Une combinaison qui n'a pas de sens est refusée par une assertion, non traduite en silence.
- Le montage partagé de S693–S700 passera à un mode nommé à sa prochaine modification.

C'est la suite d'ADR-276 D1 : une seconde source peut se cacher dans une aide reprise, mais aussi dans une combinaison de drapeaux.

## 3. La prochaine revue

S706.
