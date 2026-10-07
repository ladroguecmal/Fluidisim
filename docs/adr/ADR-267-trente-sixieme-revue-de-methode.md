# ADR-267 — Trente-sixième revue de méthode (S656–S660)

- **Statut : actée**, S661, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-266](ADR-266-trente-cinquieme-revue-de-methode.md). La première revue depuis les contrôles du plan.

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S656 | la revue des erreurs ; le bloc « Contrôles du plan » exigé par le rituel | — | — |
| S657 | le premier plan sous contrôle : le comparant (l'eau autour du corps) éprouvé sur un cas connu avant de juger ; le témoin a tranché (l'excès venait du comparant) | — | **la protection a servi** |
| S658 | la séance visuelle : le premier rendu, **regardé avant l'envoi**, laissait vide la zone des colonnes ; l'enregistrement refait | 8 min | rien à changer : regarder avant d'envoyer est déjà la pratique (le contrôle « instrument ») |
| S659 | le contrôle « pièges » nommait **l'axe inversé des mesures** de Berkhoff : il a été appliqué juste, puis vérifié par un témoin ; le critère manqué, localisé | — | **la protection a servi** |
| S660 | **la ligne « instrument » du plan était remplie, mais fausse** : juger l'onde oblique sur `|A|` = 1, qu'une onde plane garde dans les deux modèles. Vu en codant, avant la mesure, rapporté | aucun, mais par chance | **D1** |
| S659–S660 | **deux longues commandes à heredoc rejetées par le shell**, malgré ADR-245 D3 (les scripts sont des fichiers) | deux allers-retours | **D2** |

## 2. Décisions

**D1 — La ligne « instrument » dit ce qui départagerait.** Elle ne se contente plus de nommer le lecteur et son cas connu. Elle écrit ce
que le lecteur rendrait **sous chacune des hypothèses en jeu**. Un lecteur qui rend la même chose sous toutes ne juge rien et se change
avant la mesure. S660 l'aurait vu au plan : `|A|` = 1 sous les deux modèles, la phase différente.

**D2 — Un script de plus de vingt lignes s'écrit en fichier**, par l'outil d'écriture, puis se lance. La règle d'ADR-245 D3 avait des
exceptions tolérées dans la pratique (les « heredocs quotés »), et ce sont elles qui ont échoué. Elle se resserre : plus d'heredoc pour un
script long. La règle est aussi dans la mémoire de l'assistant.

## 3. La prochaine revue

S666.
