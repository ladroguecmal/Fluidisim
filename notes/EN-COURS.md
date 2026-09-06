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
Session          : S13
État             : en cours
Battement        : 2026-09-05
Objectif         : confronter SPEC-006, ADR-022 et ADR-023 au corpus — trois documents
                   structurants écrits en quatre sessions, jamais audités
```

### Plan

La revue S05 a confronté les vingt premiers ADR, la revue S08 les cinq premières SPEC. **Ces trois
documents-là n'ont jamais rencontré personne**, et ils ont été écrits vite : SPEC-006 en S09,
ADR-022 en S10, ADR-023 en S12.

Règle de conduite pour cette session : **ne pas auditer de mémoire.** J'ai écrit les trois ; ce que
je crois y avoir mis n'est pas ce qui y est. Chaque contrôle part du texte du corpus, pas de
l'intention.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [ ] **P2** — SPEC-006 contre les **17 invariants** et contre les ADR qu'il sert
  (ADR-012 budgets et dégradation, ADR-014, ADR-016, ADR-018, ADR-021 autorité).
  *Thèse : un document d'interface écrit vite viole d'abord les invariants de ressources — I-06
  allocation, I-16 profil — parce qu'ils ne se voient qu'en additionnant des tailles.*
- [ ] **P3** — SPEC-006 contre SPEC-001 à SPEC-005 : chiffres, cadences, unités, renvois.
- [ ] **P4** — ADR-022 contre le corpus : I-17 tient-il partout, la couche V, le harnais, SPEC-005.
- [ ] **P5** — ADR-023 contre le corpus : les quatre mécanismes contre ADR-008, ADR-010, ADR-013,
  ADR-015, et contre les chiffres de SPEC-001/002.
- [ ] **P6** — les trois **entre eux** : ils se citent mutuellement (ADR-023 §4 publie sur
  SPEC-006 §6 ; ADR-022 §3 recoupe SPEC-005 §6 ; ADR-022 §4 recoupe SPEC-006 §2.6).
  *Thèse : le risque le plus élevé est là. Trois documents écrits par la même session-mère à quatre
  sessions d'écart se citent avec confiance et sans vérification.*
- [ ] **P7** — rédiger `docs/registres/REVUE-CROISEE-S13.md` : écarts, gravité, résolution, et la
  liste des contrôles passés — sans elle la revue n'est pas vérifiable.
- [ ] **P8** — appliquer les résolutions : notes correctives datées, nouvel ADR si une décision
  change.
- [ ] **P9** — index, angles morts, décomptes.
- [ ] **P10** — rituel de fin (`REPRISE.md` §6) : journal S13, leçons, index, jeton libéré.

### Notes de reprise

*(Vide au démarrage. Y déposer au fil de l'eau ce qui n'est pas encore dans un fichier.)*

- **Volume** : SPEC-006 fait 808 lignes, ADR-022 et ADR-023 environ 370 et 460. C'est plus que ce
  que S08 avait confronté d'un coup, d'où le découpage en quatre passes plutôt qu'en deux.
- **Attente** : S08 avait trouvé dix écarts dont deux de gravité 1 sur un corpus plus relu que
  celui-ci. Si cette session en trouve nettement moins, la première hypothèse à écarter est que
  l'audit a été mené de mémoire.
