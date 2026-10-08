# ADR-274 — Quarante-deuxième revue de méthode (S686–S690)

- **Statut : actée**, S691, 2026-10-08 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-273](ADR-273-quarante-et-unieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S686 | la revue (ADR-273) | — | — |
| S687 | **la borne (6 %) sous le quantum de la lecture** : la remontée lue par le fond de la maille monte par marches de `dx/3`, 7,6 % à 5 cm. L'écart mesuré en était exactement une | un critère manqué par sa borne | **D3** |
| S688 | **une borne non calculée** (1 % pour le raccord seul, sous la maille) ; l'écart réel converge de 3,5 à 0,6 % | un critère manqué par sa borne | **D3** — la deuxième fois en deux sessions |
| S689 | une ligne étonnante (25 L entrés, 5 L sortis) relue avant d'écrire, comptée et expliquée (le remboursement) | une relance de 5 min | la pratique d'ADR-244 |
| S690 | **un calcul de 8 h attendu à l'aveugle.** Estimé à 35 min sans mesurer le coût d'un pas. Lancé sur un seul cœur, alors que la machine en a 16 et qu'APIC sait s'en servir (S483) ; le pas de Saint-Venant figé sur une borne quatre fois trop courte ; aucune progression affichée. L'utilisateur : *« Trop long il n'y a pas des solutions »*, puis *« tous les temps de calculs peuvent être utilisés à des fins »* | 8 h de calcul, une demi-journée | **D1**, **D2** |
| S690 | **un plancher numérique au lieu d'une borne physique.** La hauteur du raccord, bornée à 1 mm, a laissé la vitesse du bord monter à 1 644 m/s quand une colonne s'est vidée : le pas est tombé à 0,1 ms. Localisé par la progression affichée et un diagnostic | les 8 h, en partie | **D4** |

## 2. Décisions

**D1 — Un calcul long se mesure avant de s'estimer, et se montre pendant qu'il tourne.**
- Une durée annoncée est le coût mesuré de quelques pas, multiplié par le nombre de pas calculé.
- Tout essai de plus de 5 min affiche sa progression (l'instant simulé, le pas, l'horloge) au moins chaque minute.
- Il se lance par `outils/essai.py`, qui n'affiche que les lignes marquées et garde le journal dans `calculs/`.
- Un calcul qui dépasse le double de son estimation est arrêté et diagnostiqué.

**D2 — Un essai long emploie la machine, et son attente sert.**
- Les fils (`set_jobs`, 16 cœurs, au bit du séquentiel, S483), le pas stable réel de chaque solveur (non une borne fixe).
- Pendant qu'il tourne, une tâche utile qui ne dépend pas de lui : le diagnostic suivant, le remède candidat, la preuve, la revue.

**D3 — Une borne d'essai sur une lecture quantifiée porte son quantum.**
- Le plan écrit le quantum de la lecture (la marche, la maille, la couche) à côté de la borne.
- Le script refuse une borne sous deux quanta.
- C'est l'élargissement d'ADR-268 D1 aux lectures, manqué deux fois en deux sessions (S687, S688).

**D4 — Une division par une hauteur se borne par la physique, non par un epsilon.**
- Une vitesse tirée d'un flux et d'une hauteur est bornée par la célérité du lieu (`|u| + 2·√(g·h)`).
- Le plancher de la hauteur est une fraction de maille, non un millimètre.
- Ce qu'un epsilon cache devient une vitesse sans borne quand la hauteur s'annule.

## 3. Le contexte de la discussion (demandé par l'utilisateur)

Les pratiques sont écrites dans [ANALYSE-PHASES-S690](../registres/ANALYSE-PHASES-S690.md) §4. Rien d'important ne vit seulement dans la
discussion, et les sorties sont filtrées (`outils/essai.py`). Un fichier est lu par morceaux, jamais un registre entier d'une ligne
(S690 : le journal des décisions affiché en entier par mégarde).

## 4. La prochaine revue

S696.
