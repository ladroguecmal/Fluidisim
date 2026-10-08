# ADR-279 — Quarante-cinquième revue de méthode (S701–S705)

- **Statut : actée**, S706, 2026-10-08 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-277](ADR-277-quarante-quatrieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S701 | la revue (ADR-277) ; `rituel.py --sans-banc` attend une raison, donnée au second essai | aucun | — |
| S702 | un remplacement ambigu (le même appel deux fois) : l'assertion du script l'a arrêté avant toute écriture du fichier d'essai | aucun | l'assertion de compte a fait son office |
| S702 | **un échec masqué par un tube** : `rituel.py … \| tail` rendait le code de `tail`, et la chaîne `&&` a commis un état partiel sous le message de la session | un commit amendé | **D2** |
| S704 | la lecture d'une crête par une tranche d'une maille, bruitée par le regroupement des particules et retenue par le maximum dans le temps : 0,29 m lu au lieu de 0,15 m | un essai d'une minute, un autre arrêté | aucune : l'instrument a été corrigé avant toute attribution, à sa première lecture |
| S693–S704 | **un juge non convergé, pris pour la vérité.** Depuis S693, les critères du raccord demandaient 0,02 s contre le tout-3D à 2,5 cm. S704 a montré que ce juge ne converge pas à ce niveau : à 1,25 cm, sa crête est 4 % plus basse. Une partie de la chasse au centième de seconde visait sous la précision du juge | une part de dix sessions (S693–S703) ; elles ont tout de même livré le bord à particules et la pose par la grille | **D1** |

## 2. Décisions

**D1 — Avant de fixer une tolérance contre une simulation de référence, on mesure la convergence de cette référence.**
- La tolérance d'un critère n'est pas plus fine que l'écart du juge à lui-même entre deux résolutions, mesuré sur la grandeur jugée ou
  sur celle qui la commande (ici, la crête).
- Faute de cette mesure, le plan la fait, sur le cas le moins cher qui la donne (S704 : 5 min, fond plat), avant la première session de
  la campagne.
- La règle de la précision rapportée à l'usage reste la borne haute : une tolérance plus fine que le visible ne se justifie que par
  une raison écrite.

**D2 — Une commande dont l'échec doit arrêter une chaîne n'est pas suivie d'un tube** (`… \| tail`), ou la chaîne commence par
`set -o pipefail`. Le rituel se lance seul, son résultat se lit, puis le commit suit.

**D3 — Des sessions plus longues : une session couvre une question entière, non un seul essai** (demande de l'utilisateur, 2026-10-08 :
« pourquoi ne pas faire des sessions plus longues […] cela prend moins de temps à fermer, ouvrir »).
- Le coût fixe d'une session (le jeton, le plan, la preuve, le journal, le rituel et son banc) se paie une fois pour plusieurs essais.
  S697–S704 en ont payé huit pour une seule question, le raccord du large.
- Une session porte une pièce entière d'un registre de conception (LOD-ETAPE-2-S705 : N1, N2…), ou une question et ses témoins. Le
  plan liste d'avance ses essais successifs, et dit lequel suit selon le résultat du précédent.
- **Les règles de preuve ne changent pas** : un témoin ne fait varier qu'une cause (ADR-276 D2), chaque essai a ses critères écrits avant
  lui, chaque résultat s'inscrit dans la preuve de la session. Un commit marque chaque essai (`Snnn Pk`), la session se ferme une fois.
- Une session s'arrête quand sa question est tranchée, ou quand un résultat change la question. La suite devient alors une session
  nouvelle.
- Les lots et les revues se comptent toujours en sessions : ils reviennent moins souvent en temps, et c'est voulu.

## 3. La prochaine revue

S711.
