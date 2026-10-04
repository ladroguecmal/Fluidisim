# ADR-218 — L'objectif : le système de l'eau complet, la liste validée à 100 %

- **Statut : actée**, S475, 2026-10-04 ; **décision de l'utilisateur**, telle qu'écrite :

  > Je suis d'accord avec ton plan qui suit mais l'objectif est de finir le système complet de l'eau, à ce moment précis la feuille
  > to do list devra être validée à 100% pas moins mais plus possible ou changement durant le processus.

- **Remplace** le critère d'arrêt d'[ADR-215](ADR-215-autonomie-jusqu-a-une-v1-solide.md) D3 (la v1 solide, atteinte en S458) ;
  **garde** ADR-215 D1 (les sessions s'enchaînent) et D2 (les arbitrages techniques se tranchent ici, par écrit).
- **Précise** [ADR-190](ADR-190-apres-la-v1-la-liste-entiere.md) (la liste entière, objectif depuis S351) : ce n'est plus une
  direction, c'est la condition de fin.

## 1. Décisions

**D1 — Le système de l'eau est fini quand la [liste du projet fini](../LISTE-PROJET-FINI.md) est validée à 100 %.** Chaque point coché
`[x]`, sur son **périmètre final**, par une preuve publiée (la règle de la liste : jamais sur un banc isolé quand l'énoncé vise le
système). Pas moins. Le périmètre compte aujourd'hui **119 points** : les 120 de la liste moins 5.11, les eaux souterraines, retiré
par l'utilisateur ([ADR-197](ADR-197-reponses-du-2026-09-26.md) D4).

**D2 — La liste peut grandir ou changer en chemin.** Un point ajouté, scindé, reformulé ou retiré l'est par une note datée dans la
liste, avec sa raison et sa source (une décision de l'utilisateur, un ADR) ; le décompte et le registre des dépendances suivent
(`etat_projet.py --check`). Retirer ou alléger un point demande l'utilisateur (ADR-127 : l'ambition ne se réduit pas en silence) ;
en ajouter, non.

**D3 — Le plan de complétion** ([PLAN-COMPLETION-S475](../registres/PLAN-COMPLETION-S475.md)) range les points ouverts en campagnes,
dans l'ordre des dépendances (DEPENDANCES-LISTE, fronts 0 à 5), et nomme **les faits que seul l'utilisateur peut fournir**, avec une
recommandation pour chacun. Il se tient comme la feuille de route : retouché quand l'ordre change, jamais recopié ailleurs (L137).

**D4 — Ce qui vient « à la fin » reste dans les 100 %.** La météo (2.8) et l'audio de l'eau (7.8), placés à la fin par ADR-197 D5,
sont dans la liste : la part de l'eau — ce qu'elle consomme du temps qu'il fait, le son qu'elle produit — se fait avant de déclarer le
système fini ; le système d'atmosphère et de climat qui produit le temps qu'il fait vient après lui (ADR-217 D3).

**D5 — La liste se recompte en entier toutes les vingt-cinq sessions** (une actualisation complète, comme S309 et S350) et à chaque
changement de catégorie d'un point ; entre-temps, les lots de registres (ADR-213 D3) continuent.

## 2. Conséquences

- REPRISE, `Session suivante`, porte toujours la prochaine étape du plan de complétion ; une reprise continue sans question.
- Les verdicts visuels (8.7, 8.10, 4.5, 9.9) se demandent aux jalons du plan, avec le banc visuel (ADR-216) pour ce qui se mesure.
- Les faits extérieurs se demandent quand leur point devient le prochain du plan ; d'ici là, le travail avance sur le reste.
