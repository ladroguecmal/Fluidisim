# ADR-110 — Une copie de travail se ferme, et une branche sans commit unique ne se conserve pas

Statut : acté, S159, 2026-09-10, **sur demande explicite de l'utilisateur** — action S35-7,
parquée depuis longtemps comme « sort des branches ». Aucune décision antérieure n'est modifiée.

## Constat

Le dépôt a forké **trois fois** par le même mécanisme : le jeton de session est un fichier
**versionné**, donc chaque copie de travail en porte un, et une copie en retard porte un jeton
**périmé** qu'une session y trouvera `libre` (L137, FORK-S22-S26).

S159 en donne les pièces, mesurées avant toute modification
([COPIES-S159](../registres/COPIES-S159.md)) : **six copies ouvertes**, dont quatre annonçaient
`libre` au même instant avec quatre « dernière session » différentes — S158, S157, S146 et **S44**.
Une session ouvrant la dernière aurait pris le jeton de bonne foi et commencé S45, recréant cent
quinze sessions d'histoire parallèle.

Deux faits achèvent le constat.

**Les copies se recréent.** `project-status-progress-d31d78` est apparue **pendant S158**, sans
annonce. Ce n'est donc pas un héritage en voie d'extinction, et ranger ne suffit pas.

**Une seule protection a fonctionné**, l'état `archivé` ajouté en S39 après le troisième fork. La
copie qui le portait est la seule que personne n'aurait ouverte par erreur. Les cinq autres
n'avaient rien.

## Décision

**1. Une branche sans commit unique ne se conserve pas.** Elle ne contient rien qui ne soit dans
`master` : la supprimer ne perd aucune histoire, seulement un nom. Six branches sur sept étaient
dans ce cas et ont été supprimées avec `git branch -d` — jamais `-D`, qui aurait accepté d'effacer
du travail unique sans le dire.

**2. Une branche portant des commits uniques se conserve, mais pas sa copie de travail.** Ce qui
invite à travailler dans une lignée morte est le **répertoire**, pas la référence.
`claude/reprise-projet-5134cd` garde ses 44 commits de la lignée B ; son worktree est retiré.

**3. Fermer sa copie fait partie du travail.** Une session qui ouvre une copie isolée la referme :
elle fusionne en avance rapide dans `master`, puis retire le worktree et supprime la branche
devenue vide. Ce geste vit dans [`AGENTS.md`](../../AGENTS.md), **à un seul endroit** — le dépôt a
déjà forké pour avoir dupliqué une procédure.

**4. Une copie qu'on ne peut pas prouver morte se met à jour, elle ne se supprime pas.** Une
avance rapide éteint le jeton périmé sans rien détruire ; c'est le geste à faire en premier, et
c'est celui qui a été fait avant toute suppression en S159. Une copie propre, sans commit unique,
mais peut-être occupée par une autre session, reste ouverte et à jour.

**5. Le décompte des copies n'appartient pas au bloc de jeton de `REPRISE.md`.** Il y accumulait
des notes de session en session, dont la plus récente annonçait « cinq worktrees » quand il y en
avait six, et nommait des branches désormais supprimées. L'état des copies se constate par
`git worktree list` ; le document dit la **procédure**, pas l'inventaire.

## Ce que cette décision ne dit pas

- **Elle n'interdit pas les copies isolées.** Elles servent : une session peut travailler sans
  perturber `master`. Elle exige seulement qu'on les referme.
- **Elle ne touche pas au dépôt distant** (`REPRISE.md` §9), et aucune des suppressions n'y a
  d'effet : elles sont toutes locales.
- **Elle ne supprime pas le mécanisme de fond.** Le jeton restera un fichier versionné, donc
  chaque copie en portera un. La décision réduit le nombre d'univers et donne la procédure ; elle
  ne change pas la nature du dispositif. Un jeton non versionné serait une autre décision, avec
  son propre coût — il cesserait de voyager avec l'histoire, ce qui est précisément sa vertu.
