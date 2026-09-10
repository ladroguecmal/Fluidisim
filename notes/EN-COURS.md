# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S159 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **demande explicite de l'utilisateur** — régler tous les problèmes liés aux copies de
travail. S158-1 (le facteur 2,5) est reporté ; il ne se périme pas, les jeux de données sont dans
le dépôt.

C'est l'action **S35-7**, parquée depuis longtemps comme « sort des branches, décision de
l'utilisateur ». Elle vient d'être donnée.

### Ce qui a déclenché la demande, et qui est un fait nouveau

Un **sixième** worktree est apparu pendant S158 : `project-status-progress-d31d78`, créé sur la
tête de S157, sept commits en retard, aucun commit unique. Personne ne l'a annoncé. C'est
exactement le mécanisme qui a forké le dépôt **trois fois** (L137, FORK-S22-S26) : le jeton est un
fichier **versionné**, donc chaque copie en possède un, et une copie en retard porte un jeton
périmé qu'une session y trouvera `libre`.

### Plan

- [x] **P1** — état réel, jeton, plan déclaré et committé seul.
- [x] **P2** — inventaire **factuel** des six copies et des six branches : commits uniques,
      propreté, âge, état du jeton que chacune porte. Rien ne sera supprimé avant que ce tableau
      existe et soit publié.
- [x] **P3** — remettre en avance rapide toute copie sans commit unique. C'est le geste le moins
      risqué et il éteint à lui seul le danger du jeton périmé : toutes les copies liront alors
      le même jeton.
- [ ] **P4** — retirer les worktrees morts et supprimer les branches **sans commit unique**.
      La branche archivée porte 44 commits uniques : son worktree est retiré, **la branche est
      conservée**. Aucune histoire n'est perdue.
- [ ] **P5** — le correctif **durable**, sans lequel tout repoussera : la procédure de fermeture
      d'une copie doit vivre dans `AGENTS.md`, à un seul endroit, et le décompte périmé de
      `REPRISE.md` doit être corrigé. Décision à acter par ADR : une branche sans commit unique
      ne se conserve pas.
- [ ] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 3f25fe5 = master. Six copies, toutes **propres** — aucune modification non committée nulle
part, vérifié avant tout.

| branche | retard | unique |
|---|---|---|
| master | — | — |
| claude/reprise-projet-2d3506 (la mienne) | 0 | 0 |
| claude/reprise-projet-29ef50 | 0 | 0 |
| claude/project-status-progress-d31d78 | 7 | 0 |
| claude/reprise-projet-886155 | 58 | 0 |
| claude/reprise-projet-c107bf | 434 | 0 |
| claude/s22-suite (sans worktree) | 434 | 0 |
| claude/reprise-projet-5134cd (archivée) | 603 | **44** |

Règles que je me donne avant de toucher à quoi que ce soit, parce qu'une suppression se regrette :
1. **Ne jamais supprimer une branche portant un commit unique.** Une seule est dans ce cas et elle
   est archivée volontairement.
2. **Ne rien retirer qui contienne du travail non committé.** Vérifié : rien nulle part.
3. **Ne pas toucher au dépôt distant** — `REPRISE.md` §9, hors de ma portée, et la demande porte
   sur les copies locales.
4. **Ne pas retirer ma propre copie** : j'y travaille. La dernière fermeture revient à qui
   travaillera sur master.
5. Une copie qu'une autre session pourrait utiliser en ce moment se **remet à jour**, elle ne se
   supprime pas sans preuve qu'elle est morte.

Piège à éviter, et il est réel : croire que ranger les copies règle le problème. Le problème est
que **le jeton est versionné**, et il le restera. Ranger réduit le nombre d'univers ; seule une
procédure écrite au bon endroit empêche qu'ils se remultiplient.

P2 — inventaire publie dans COPIES-S159, avant toute modification.
Le danger est demontrable, pas hypothetique : quatre copies disent `libre` au meme instant avec
quatre "derniere session" differentes — S158, S157, S146 et **S44**. Une session ouvrant c107bf
prendrait le jeton de bonne foi et commencerait S45, recreant cent quinze sessions d'histoire
parallele. Seule la copie archivee est protegee, par l'etat `archivé` ajoute en S39 : il fonctionne,
et il est la seule chose qui fonctionne.
Fait nouveau : project-status-progress-d31d78 est **apparue pendant S158**, sans annonce. Les
copies ne s'eteignent pas toutes seules, elles se recreent.
Aucune copie ne porte de travail non committe — verifie en premier, c'est la seule perte qui serait
irreversible.

P3 — les cinq copies sans commit unique sont en avance rapide sur master. Elles lisent maintenant
**le meme jeton**, la meme derniere session, le meme etat : le danger du jeton perime est eteint
**avant** qu'aucune suppression n'ait eu lieu. C'est le point important de l'ordre choisi — si la
session s'interrompait ici, le depot serait deja plus sur qu'au depart.
Rien n'a change dans ma copie : le travail de cette etape est dans les autres repertoires, et
seul ce journal en garde la trace. C'est exactement le cas ou l'ecriture anticipee sert.
