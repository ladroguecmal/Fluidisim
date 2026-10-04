# ADR-221 — La structure du projet : la boussole, les registres générés, les calculs longs, le rituel outillé

- **Statut : actée**, S480, 2026-10-04 ; **décision de l'utilisateur** (*« Ok »*) sur la proposition faite à ses questions : reprendre
  depuis une autre conversation, les défauts de méthode, une structure de gestion pour *« avoir en contexte les choses essentielles
  nouvelles mais aussi les intentions initiales […] pour ne pas travailler dans le flou »*.
- **Précise** [ADR-187](ADR-187-methode-refondue-s321.md) (la méthode refondue : la lecture bornée, le rituel en deux parties) et
  [ADR-213](ADR-213-accelerer-tolerance-plafond-rituel-bancs.md) D3 (le lot des registres) ; ne touche ni au périmètre
  ([ADR-127](ADR-127-ambition-complete-construction-progressive.md), [ADR-218](ADR-218-le-systeme-de-l-eau-complet.md)) ni à l'ordre des
  campagnes. La preuve : [STRUCTURE-S480](../validation/STRUCTURE-S480.md).

## 1. Décisions

**D1 — [`BOUSSOLE.md`](../../BOUSSOLE.md), deux pages, se lit en premier** (AGENTS.md, étape 1) : pourquoi le projet existe (les
intentions d'origine, le jeu DyingStar et la surprise), vers quoi il va (la liste à 100 %), l'architecture en quatre lignes, les
décisions en vigueur en une ligne chacune, ce que l'utilisateur juge et ce qui attend de lui, la méthode, les pièges. Une décision
nouvelle de l'utilisateur s'y ajoute en une ligne le jour où elle est prise ; une question qui l'attend s'y inscrit et s'en retire.

**D2 — L'état courant ne s'écrit plus à la main : trois registres générés**, chacun par son outil, tenus par
`etat_projet.py --check` — le [tableau de bord](../registres/TABLEAU-DE-BORD.md) vers 100 % (`tableau_de_bord.py` : la liste croisée
avec le plan de complétion, l'historique du décompte), les [décisions en vigueur](../registres/DECISIONS-EN-VIGUEUR.md)
(`decisions.py` : chaque ADR, son statut lu, qui le nomme), les [anomalies ouvertes](../registres/ANOMALIES-OUVERTES.md)
(`anomalies.py` : lu dans ANGLES-MORTS, où seul on écrit). REPRISE §4 renvoie au tableau de bord.

**D3 — Le journal vivant ne garde que les dix dernières sessions environ** ; le reste va par centaine dans `notes/journal/`, texte
inchangé. Une archive ne se modifie plus.

**D4 — Un calcul de plus de dix minutes se lance par `python outils/calcul.py lancer`** : détaché de la conversation, sa sortie dans
`calculs/` (dans le dépôt, non versionné), sa trace dans [`notes/CALCULS.md`](../../notes/CALCULS.md). Le dossier temporaire d'une
conversation disparaît avec elle (S479 : B10 à 24 mailles est mort ainsi).

**D5 — L'amorce et le rituel de fin sont outillés** : `python outils/rituel.py debut` (les commandes d'AGENTS, l'avis du jeton) et
`fin` (vérifie les cases et le journal, régénère, libère le jeton, coche, contrôle ; refuse sans rien écrire si un point manque). Le
journal, la suite et les lignes de la liste restent écrits par la session.

**D6 — Le plan d'une session déclare ses entrées et comment il les vérifie** (REPRISE §2) — la leçon de S472, où une mer lisait le
fichier de détail d'une autre et trois séries de mesures étaient fausses.

## 2. Ce qui changerait la décision

Une boussole qui dépasse deux pages, ou qui recopie l'état au lieu d'y renvoyer, a perdu sa raison d'être : on la réduit. Un registre
généré dont l'outil lit mal sa source se corrige dans l'outil, jamais à la main dans le registre.
