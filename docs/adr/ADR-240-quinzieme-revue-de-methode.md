# ADR-240 — Quinzième revue de méthode (S551–S555)

- **Statut : actée**, S556, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-239](ADR-239-quatorzieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un script de clôture écrit en ligne (heredoc) et cassé par une apostrophe française** (S520, S555 — deux fois) : la chaîne se ferme au premier « l'… », rien ne part, et toute la clôture se réécrit | une clôture entière à refaire, et un résumé de contexte coupé au milieu (S555) | **protection élargie** (D1) |
| **Une durée d'essai trop courte vers un équilibre** (S553) : l'essai s'arrêtait à 900 s en plein envahissement ; la trace, lue avant de conclure, a montré l'équilibre vers 1 800 s | une mesure relancée | **protection élargie** (D2) ; S554 l'a déjà appliquée |
| Un écart de définition entre la référence et la mesure (S552 : le tirant le long de l'axe contre la verticale, 0,997 % au lieu de 0,01 %) | — | relevé et écrit tel quel, la protection « localiser un écart » (ADR-226 D1) a tenu ; **rien à changer** |
| Une assertion canonique inapplicable telle qu'écrite (S555 : `dE/dt ≤ 0` sur l'énergie naturelle d'un schéma décalé) | — | la mise en garde écrite au plan avant la mesure ; critères gardés, A332 ouverte ; **rien à changer** |
| Les références indépendantes avant la mesure (S552–S554, ADR-239 D1) | — | appliquées trois fois, chaque fois juste ; **rien à changer** |

## 2. Décisions

**D1 — Un texte du dépôt (édition, note, message de commit long) passe par un fichier écrit par l'outil d'écriture, puis exécuté** ; jamais
par un heredoc ou une chaîne en ligne de commande, où l'apostrophe du français ferme la chaîne (élargit L380, « une édition par script »).

**D2 — Un essai qui attend un équilibre en calcule la constante de temps au plan, ou lit sa trace avant de conclure** ; la durée se choisit
à plusieurs constantes de temps (élargit L369, « un comportement s'éprouve sur sa durée d'usage »).

## 3. La prochaine revue

S561.
