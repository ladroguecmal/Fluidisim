# Inventaire des copies de travail — S159, 2026-09-10

Établi avant toute modification, à la demande de l'utilisateur : action **S35-7**, parquée depuis
longtemps comme « sort des branches, décision de l'utilisateur ».

Rien n'a été supprimé avant que ce tableau existe.

## 1. Les six copies, au 2026-09-10

| copie | branche | tête | date | en retard | commits **uniques** | propre |
|---|---|---|---|---:|---:|---|
| `Fluidisim` | `master` | 3f25fe5 | 10-09 | — | — | oui |
| `reprise-projet-2d3506` | `claude/reprise-projet-2d3506` | 3f25fe5 | 10-09 | 0 | 0 | oui |
| `reprise-projet-29ef50` | `claude/reprise-projet-29ef50` | 3f25fe5 | 10-09 | 0 | 0 | oui |
| `project-status-progress-d31d78` | `claude/project-status-progress-d31d78` | 7d474cd | 10-09 | 7 | 0 | oui |
| `reprise-projet-886155` | `claude/reprise-projet-886155` | 66cd765 | 10-09 | 58 | 0 | oui |
| `reprise-projet-c107bf` | `claude/reprise-projet-c107bf` | b521129 | 07-09 | 434 | 0 | oui |
| `friendly-bhabha-6da427` | `claude/reprise-projet-5134cd` | 5d9bf2f | 07-09 | 603 | **44** | oui |

Une septième branche, `claude/s22-suite`, existe **sans copie de travail** : 434 commits de retard,
aucun commit unique.

**Aucune copie ne porte de travail non committé.** Vérifié en premier, avant tout le reste : c'est
la seule chose dont la perte serait irréversible.

## 2. Le danger, avec ses pièces

Le jeton de session est un fichier **versionné**. Chaque copie en porte donc un, et une copie en
retard porte un jeton **périmé**. Voici ce que quatre d'entre elles annonçaient au même instant :

| copie | jeton | « dernière session » qu'elle croit |
|---|---|---|
| `Fluidisim` et `reprise-projet-29ef50` | `libre` | S158 |
| `project-status-progress-d31d78` | `libre` | S157 |
| `reprise-projet-886155` | `libre` | S146 |
| `reprise-projet-c107bf` | `libre` | **S44** |
| `friendly-bhabha-6da427` | `archivé` | S27 |

Une session qui ouvrirait `reprise-projet-c107bf` y lirait un jeton **libre**, une dernière session
**S44**, prendrait le jeton de bonne foi et commencerait S45. Elle recréerait cent quinze sessions
d'histoire parallèle, avec les collisions d'identifiants qui vont avec.

**Ce n'est pas une hypothèse : c'est le mécanisme qui a forké ce dépôt trois fois** — S07, S21,
B-S27 — et la troisième fois précisément parce qu'une branche avait été **conservée exprès** pour
son historique, sans que rien ne la distingue d'un point de départ (FORK-S22-S26 §9, L137).

Seule `friendly-bhabha-6da427` est protégée, parce que son jeton dit `archivé` — l'état ajouté en
S39 après ce troisième épisode. Il fonctionne. Les cinq autres n'ont aucune protection.

## 3. Le fait nouveau qui a déclenché la demande

`project-status-progress-d31d78` **est apparue pendant S158**, sur la tête de S157, sans que
personne l'annonce. Les copies ne sont donc pas un héritage en voie d'extinction : elles se
recréent. Ranger ne suffira pas ; il faut une procédure, et à un seul endroit.

## 4. Ce que l'inventaire autorise

Une branche sans commit unique ne contient **rien** qui ne soit dans `master` : la supprimer ne
perd aucune histoire, seulement un nom. Six branches sur sept sont dans ce cas.

La septième, `claude/reprise-projet-5134cd`, porte **44 commits uniques** — la lignée B, réconciliée
dans master par renumérotation mais non fusionnée. **Elle est conservée.** Seule sa copie de
travail est retirée : ce qui invitait à y travailler était le répertoire, pas la branche.

## 5. Ce qui a été fait, dans cet ordre

**D'abord mettre à jour, ensuite seulement supprimer.** Les cinq copies sans commit unique ont été
remises en avance rapide sur `master`. À partir de cet instant elles lisaient toutes le **même
jeton**, la même dernière session, le même état : le danger était éteint **avant** qu'une seule
suppression n'ait eu lieu. Si la session s'était interrompue là, le dépôt aurait déjà été plus sûr
qu'au départ.

Puis les retraits, chacun vérifié :

| geste | objet | perte |
|---|---|---|
| `git worktree remove` | `friendly-bhabha-6da427` | aucune — **la branche archivée est conservée**, avec ses 44 commits |
| `git worktree remove` | `reprise-projet-c107bf`, `reprise-projet-886155`, `reprise-projet-29ef50` | aucune |
| `git branch -d` | les trois branches correspondantes et `claude/s22-suite` | aucune — toutes ancêtres de `master` |

`-d` et jamais `-D` : `-d` refuse une branche portant un commit unique. Aucune n'a été refusée, ce
qui est la vérification, et non la formalité.

**Six copies ramenées à trois, sept branches à quatre.** Vérifié après coup : la branche archivée
porte toujours ses 44 commits.

## 6. Ce qui n'a pas été supprimé, et pourquoi

`project-status-progress-d31d78` est propre, sans commit unique, et remise à jour. Elle n'a pas
été retirée : rien ne prouve qu'aucune session ne l'occupe, et la règle posée avant de commencer
dit qu'une copie peut-être vivante **se met à jour, elle ne se supprime pas**. La retirer est une
commande d'une ligne, laissée à qui saura qu'elle est morte :

```bash
git worktree remove .claude/worktrees/project-status-progress-d31d78
git branch -d claude/project-status-progress-d31d78
```

La copie de S159 elle-même reste ouverte, puisque la session y travaille. Sa fermeture revient à
qui travaillera ensuite sur `master`.

## 7. Le correctif durable

Ranger ne suffit pas : `project-status-progress-d31d78` est apparue **pendant S158**. La procédure
de fermeture vit désormais dans [`AGENTS.md`](../../AGENTS.md), **à un seul endroit**, et la
décision est [ADR-110](../adr/ADR-110-une-copie-de-travail-se-ferme.md).

Le bloc de jeton de `REPRISE.md` a perdu son inventaire : il vieillissait de session en session,
annonçait « cinq worktrees » quand il y en avait six, et prescrivait encore de refusionner une
branche supprimée depuis. **Une consigne qui nomme une ressource disparue n'instruit plus, elle
égare.** L'état des copies se constate par `git worktree list` ; le document dit la procédure.

Ce qui n'est pas réglé, et ne peut pas l'être ici : le jeton reste un fichier **versionné**, donc
chaque copie en portera toujours un. Réduire le nombre d'univers et écrire la procédure ne change
pas la nature du dispositif. **A215.**
