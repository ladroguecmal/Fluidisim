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

Session : S255 — en cours
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo, Python et GPU local disponibles.
Entrée (2026-09-16 22:05), demande de l'utilisateur **avant son premier retour visuel** : « un .md
qui est une to do list avec tous les éléments que le projet fini doit avoir et pouvoir faire ;
cette liste, tu valideras ou non les points, et de temps en temps je te demanderai de la remplir ».
Master propre 97129a4, copie unique, jeton libre. La demande prime sur la suite prévue (bords
ouverts) ; R1 reste en attente de l'utilisateur.

Objectif : `docs/LISTE-PROJET-FINI.md`, liste exhaustive des capacités du projet **fini**
(ambition complète, ADR-127), tirée des sources et non du seul état construit. Chaque point porte un
état **validé / partiel / absent**, avec sa preuve ou ce qui manque. « Validé » exige une réception
publiée sur le périmètre final du point, pas un banc isolé.

Critère : chaque section des intentions d'origine (architecture globale, zones ouvertes), chaque
couche d'ADR-001, chaque phénomène de SPEC-002, chaque cas canonique et chaque banc trouvent au
moins un point. Aucun point n'est validé sans lien vers sa preuve. Rôle défini face à la feuille de
route (qui porte la trajectoire, L137) : la liste pointe, elle ne recopie pas les états détaillés.

### Plan

- [>] **P1** — amorce, jeton et plan seuls.
- [ ] **P2** — inventaire des sources : architecture globale, zones ouvertes, ADR-001, SPEC-002,
  cas canoniques, bancs, invariants ; squelette de la liste par domaine.
- [ ] **P3** — états : validé / partiel / absent, point par point, contre feuille de route, file
  active et preuves ; compte par section.
- [ ] **P4** — contrôle de couverture (sources, cas, bancs, invariants), rôle écrit dans la liste,
  METHODE et index.
- [ ] **P5** — rituel §6 : journal, jeton.

### Notes de reprise

