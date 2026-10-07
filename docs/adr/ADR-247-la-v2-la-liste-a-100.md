# ADR-247 — La v2 : la liste à 100 %, la physique d'abord

- **Statut : actée**, S585, 2026-10-07, **décision de l'utilisateur** : « L'objectif est de réaliser une v2 », puis, à la question de sa
  portée : « La liste à 100 % » ; à celle de l'ordre : « Non, la physique d'abord ».
- Prolonge [ADR-190](ADR-190-apres-la-v1-la-liste-entiere.md) (après la v1, la liste entière) et [ADR-218](ADR-218-le-systeme-de-l-eau-complet.md)
  (le système de l'eau est fini quand la liste est validée à 100 %).

## 1. Décisions

**D1 — La v2 est la liste du projet fini validée à 100 %** : chacun de ses points sur son périmètre final, reçu par une preuve publiée
(et, pour ce qui se juge à l'œil, par l'utilisateur). La v1 (ADR-174 D4) était quatre portes reçues séparément ; la v2 est la fin du
système de l'eau. La liste peut grandir en chemin (ADR-218), jamais rétrécir sans l'utilisateur (ADR-127).

**D2 — La physique d'abord.** Les sessions remplissent d'abord le cœur — les points absents, puis les partiels jusqu'à leur périmètre
final ; l'intégration dans la scène (Godot, le banc visuel) et les revues visuelles viennent ensuite. L'alternance rendu / physique
d'ADR-191 est **suspendue** jusqu'à ce que la physique de la liste soit construite ; elle reprend pour l'intégration.

**D3 — Le compte.** Le tableau de bord (`outils/tableau_de_bord.py`) mesure l'avancement vers la v2 ; au 2026-10-07 : 10 validés,
81 partiels, 30 absents sur 121. Un point ne passe « validé » que sur son périmètre final ; un manque hors de la portée d'une décision
technique reste écrit comme manque (S563).

## 2. Conséquences

- Le choix de chaque session suit : les points **absents** d'abord (rien n'existe), par campagne du plan de complétion ; puis les
  partiels dont il manque le moins pour être validés.
- Les points qui ne peuvent se valider qu'à l'œil (le rendu, 8.x, 8.10) ou dans le jeu (13.4) attendent la phase d'intégration.
