# ADR-190 — Après la v1, la liste du projet fini entière

- **Statut : actée**, S351, 2026-09-24, **décision de l'utilisateur** : *« Continue, après la V1 ton objectif
  seras de completer entièrement la to do liste »*, après le compte rendu de S350 — la liste actualisée,
  3 validés, 56 partiels, 61 absents sur 120.
- **Précise** [ADR-189](ADR-189-la-v1-d-abord.md) D2 : le terme de la suspension du lot 5 (D4 ci-dessous).
- **Laisse entiers** [ADR-127](ADR-127-ambition-complete-construction-progressive.md) — le périmètre, dont elle
  fait l'objectif de travail —, [ADR-174](ADR-174-arbitrages-du-2026-09-19.md) D4 (la v1 = portes A, B, C et
  D), [ADR-178](ADR-178-strategie-en-trois-systemes-physiques.md) (trois systèmes, ordre des lots) et
  [ADR-187](ADR-187-methode-refondue-s321.md) (la méthode).

## 1. Ce que l'utilisateur a décidé

« La to do liste » est la [liste du projet fini](../LISTE-PROJET-FINI.md) : c'est ainsi que l'utilisateur l'a
nommée en S271 et en S350, sans objection aux deux interprétations. La phrase dit deux choses : **continuer
jusqu'à la v1** — la porte A, dont reste la dégradation de rang 1 — ; **ensuite, compléter entièrement la
liste**.

## 2. Décisions

**D1 — Jusqu'à la v1, rien ne change.** La porte en cours de la feuille de route §3 bis désigne la session
suivante ([ADR-189](ADR-189-la-v1-d-abord.md) D1).

**D2 — Après la v1, l'objectif des sessions est la liste entière : ses 120 points validés.** « Compléter » se lit
au sens de la liste : chaque point **construit et reçu par une preuve publiée, sur son périmètre final** — pas un
décompte de partiels, pas un point réduit pour être coché. La liste dit **quoi** ; la
[feuille de route](../FEUILLE-DE-ROUTE.md) — jalons, portes E et F, dépendances — reste seule à dire **dans quel
ordre**.

**D3 — La première session après la v1 range les points restants par dépendance**, dans la feuille de route : ce
que chacun attend, ce qu'il débloque, à quel système il appartient ([TROIS-SYSTEMES-S308](../registres/TROIS-SYSTEMES-S308.md)
§8). Ensuite, chaque session prend un point dont les dépendances sont levées. Le compteur de maillons ne change
pas : un point de la liste qui change d'état est une capacité reçue (REPRISE §6, depuis S294).

**D4 — Le lot 5 reprend après la v1 entière**, par A316, là où S327 l'a laissé. C'est la lecture de « après la
V1 » ; elle répond à la question qu'ADR-189 D2 et sa note de S338 laissaient à l'utilisateur. Un mot de sa part la
renverse.

**D5 — Les points qu'une session ne peut pas valider seule restent dans l'objectif.** Certains demandent ce qu'aucun
code ne produit (AGENTS.md, « Ce que tu ne décides pas ») : un **verdict humain** (8.10) ; un **fait d'intégration**
non constaté — moteur du jeu, serveur réel, format réseau, terrain (8.1, 10.1–10.3, 10.5) ; un **second matériel**
(10.3, 11.5, A98) ; une **donnée externe** — référence expérimentale, météo, assets (2.8, 5.2). La session construit
tout ce qui se construit sans eux, puis **demande** à l'utilisateur, au moment où le point bloque, ce qui lui
manque. Jamais un retrait silencieux ; une réduction reste la décision de l'utilisateur (ADR-127).

## 3. Ce que cette décision ne tranche pas

- **La voie d'A289**, qui bloque l'ordre E et donc 4.8 et 4.21, et la **sauvegarde** par `git bundle` : toujours
  en attente.
- **L'ordre** entre les systèmes après la v1 : D3 le fait écrire, sur les dépendances, par la première session qui
  suit la v1.
