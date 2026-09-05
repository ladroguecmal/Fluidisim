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
Session          : S08
État             : en cours
Battement        : 2026-09-05
Objectif         : recroiser les cinq SPEC entre elles (S05 n'avait confronté que les ADR)
```

### Plan

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [ ] **P2** — croisement chiffré SPEC-001 × SPEC-002 × SPEC-005 : toute valeur numérique
  apparaissant dans deux documents doit y valoir la même chose, ou l'écart doit être motivé.
  *Thèse : les chiffres recopiés d'un document à l'autre se périment en silence — S07 en a déjà
  trouvé un cas dans le README.*
- [ ] **P3** — croisement SPEC-004 × SPEC-001/002 : chaque grandeur que les fiches chiffrées
  déclarent nécessaire doit être atteignable par une signature existante.
  *Thèse (L20) : une exigence qui n'a pas d'argument dans une signature n'est pas implémentable,
  et cela ne se voit qu'en confrontant les deux documents.*
- [ ] **P4** — croisement SPEC-003 × SPEC-004/005 : le harnais peut-il instrumenter ce que les
  interfaces exposent, et la cuisson réutilise-t-elle réellement le cœur qu'elle prétend réutiliser.
  *Thèse : un harnais qui exige une observation que l'interface ne permet pas de nommer est un
  harnais non écrivable (L19 pris à l'envers).*
- [ ] **P5** — rédiger `docs/registres/REVUE-CROISEE-S08.md` : écarts trouvés, gravité, résolution,
  et la liste des contrôles **passés sans écart** — sans elle la revue n'est pas vérifiable.
- [ ] **P6** — appliquer les résolutions : notes correctives datées dans les documents touchés,
  nouvel ADR si une décision change, angles morts enregistrés.
- [ ] **P7** — rituel de fin (`REPRISE.md` §6) : journal S08, leçons, index, jeton libéré.

### Notes de reprise

*(Vide au démarrage. Y déposer au fil de l'eau ce qui n'est pas encore dans un fichier.)*
