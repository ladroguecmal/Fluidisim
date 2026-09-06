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

```
Session          : S15
État             : en cours
Battement        : 2026-09-05
Objectif         : auditer les registres — le dernier corpus jamais passé au filtre
```

### Plan

Sept registres, jamais audités : `ANGLES-MORTS` (89 points), `QUESTIONS-OUVERTES` (traçabilité des
30 sections sources), et cinq registres d'audit dont les actions décidées n'ont **pas** toutes été
exécutées — c'est l'angle mort A89, écrit en S14.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [ ] **P2** — les **actions décidées** des cinq registres d'audit : exécutées ou non, et statut
  à jour ou non. *Thèse : les deux échouent, et dans les deux sens — une action faite reste marquée
  « à faire », une action à faire reste non faite.*
- [ ] **P3** — les actions qui visent la **validation** (`PLAN-BENCHMARK`, `SPEC-003`,
  `CAS-CANONIQUES`), recensées dans tout le corpus et non dans les seuls registres.
  *Thèse : c'est là que le taux d'exécution s'effondre, parce que ces documents n'appartiennent à
  aucune session — aucune n'a travaillé la validation depuis S03.*
- [ ] **P4** — `ANGLES-MORTS` : un angle mort comblé se distingue-t-il d'un angle mort ouvert ?
  Cas d'A16, explicitement **supprimé** par ADR-021.
- [ ] **P5** — `QUESTIONS-OUVERTES` : la traçabilité des 30 sections et les sept verdicts tiennent-ils
  encore, treize sessions plus tard ?
- [ ] **P6** — rédiger `docs/registres/AUDIT-REGISTRES-S15.md`.
- [ ] **P7** — appliquer : exécuter les actions retrouvées, corriger les statuts, corriger l'erreur
  factuelle de S14 sur A89.
- [ ] **P8** — index, angles morts, décomptes.
- [ ] **P9** — rituel de fin (`REPRISE.md` §6) : journal S15, leçons, index, jeton libéré.

### Notes de reprise

**Deux constats vérifiés avant de déclarer ce plan.**

- **Une action faite, encore marquée « à faire ».** La table « Suite » de `REVUE-CROISEE-S05`
  inscrit « Critère de répétition de tuile FFT | B1 | **à ajouter au protocole** ». Or
  `PLAN-BENCHMARK` B1 porte l'ajout depuis S05 (« Ajout S05, écart R09 »). Le statut est périmé
  depuis dix sessions, et il ferait refaire un travail déjà fait — l'inverse exact d'A89, et aussi
  coûteux.
- **Erreur factuelle de S14, à corriger.** S14 affirme, en trois endroits, que l'action sur I-16
  était « inscrite dans une **table Suite** » d'`AUDIT-POINTS-OUVERTS-S11`. **Ce registre n'a pas de
  table Suite** : ses actions vivent dans le corps des sections, sous la forme « **Action** : … ».
  L'énoncé d'A89 est donc faux dans sa prémisse.
  Le constat sous-jacent, lui, se renforce : **le seul registre d'audit dépourvu de table d'actions
  est celui dont les actions n'ont pas été exécutées.** C'est une meilleure formulation que celle
  de S14, et elle désigne une cause au lieu d'un symptôme.
