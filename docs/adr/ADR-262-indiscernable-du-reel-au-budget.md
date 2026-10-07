# ADR-262 — Indiscernable du réel, au budget

- **Statut : actée**, S643, 2026-10-07. Elle découle des décisions de l'utilisateur d'[ADR-261](ADR-261-reponses-du-2026-10-07.md) D3
  (*« l'objectif reste le même d'une performance et réalisme insane »*) et D4 (*« pas de limites sur le réalisme »*), et du registre
  [REEVALUATION-INTENTIONS-S643](../registres/REEVALUATION-INTENTIONS-S643.md). Elle précise le critère de la source (§1) et d'ADR-127,
  sans rien retirer à l'ambition.

## Contexte

La source fait du rendu *perçu* le critère principal : la précision scientifique y est recherchée quand elle améliore la cohérence, mais
« n'est pas une fin en soi ». L'eau y reste « aussi simplifiée que possible tant que cela n'altère pas sa crédibilité ». L'utilisateur lève
la limite de réalisme tout en gardant la performance au premier rang. *Crédible* ne suffit plus.

## Décisions

**D1 — Le critère : indiscernable.** Une représentation simplifiée (analytique, précalculée, artificielle, de résolution réduite) est
légitime si elle est **indiscernable** de la physique qu'elle remplace, à l'endroit et à l'échelle où elle est vue ou ressentie. Ce n'est
plus « crédible ». L'indiscernabilité se juge de deux façons :

- d'abord contre une **référence physique mesurée**, traduite en grandeurs visibles (hauteur, pente, phase, vitesse : la précision rapportée
  à l'usage) ;
- puis par le **regard de l'utilisateur** (le banc visuel, ADR-216).

La simplification reste le moyen de la performance. Elle n'en est plus l'excuse.

**D2 — Le budget est un plafond.** Les 2 ms et 60 images/s d'ADR-125 bornent le coût de l'eau ; ils ne sont pas une cible à remplir. À
réalisme égal, le moins cher gagne.

**D3 — L'activation se juge sur l'erreur visible.** Activer la physique, la raffiner, la dégrader ou l'éteindre se décide sur l'**erreur
que sa simplification produirait à l'écran**, ou dans les forces de jeu. Ce n'est plus une distance, ni un seuil posé. Les facteurs de la
source (§7) en sont les entrées (9.1, 9.9).

**D4 — Aucun plafond arbitraire.** Les intentions qui plafonnaient le réalisme pour le coût sont rejugées une à une : elles disparaissent
si la physique peut se faire au budget. C'est le cas de la profondeur de 200 m, de la glace bornée aux lacs, et des débordements « s'ils
apportent une valeur ». Il n'y a pas de limite de réalisme, il y a un budget.

## Conséquences

- 9.1 et 9.9 : le critère d'activation et de dégradation devient l'erreur visible.
- 9.11 : le plafond, non la cible.
- 4.4 : aucun plafond de profondeur.
- Les simplifications existantes sont jugées contre leur référence quand leur point est repris. Aucune n'est défaite d'avance.
