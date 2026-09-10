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

Session : S141 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **S139-1** — rendre le budget de pente homogène. Chaque terme consomme le meilleur
majorant exact de sa pente réelle (ADR-095) : `slope_max()` pour l'impact radial,
`slope_envelope_tight()` pour la pression, `steepness_B·π` inchangé pour le fond ; et
`max_slope` devient la limite physique **0,4488** (ADR-094) là où il tenait lieu de limite
physique. **C'est le premier lot de la série qui change des bits publiés.**

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**.
- [x] **P2** — **témoins avant**, et rien d'autre. Hachages de campagne des deux scénarios,
      budgets des fixtures mixtes, frontières de refus. Relevés et committés **avant** toute
      modification : sans cela, « ce qui a bougé » ne se démontre plus, il se raconte.
- [x] **P3** — migrer l'**impact radial** : `RadialImpact::new` compare `slope_max()`, et
      `composition.rs` / `mixed_water.rs` somment `slope_max()`. Étage seul, tests verts.
- [x] **P4** — migrer la **pression** : `slope_envelope_tight()` dans `slope_floor` et dans
      l'enveloppe de `mixed_water`. Étage seul, tests verts.
- [ ] **P5** — poser **`max_slope = 0,4488`** dans les fixtures où 0,1 tenait lieu de limite
      physique — **et pas dans celles qui exercent un refus**, où la valeur est choisie pour
      refuser et doit le dire. Recevoir : quels refus se déplacent, quels bits bougent.
- [ ] **P6** — livrable de réception, rituel de fin (§6), jeton rendu, fusion `--ff-only`.

### Notes de reprise

Départ d10cc45 = master ; worktree `886155`. 271 tests/cinq ignorés.

Sites à migrer, relevés avant de commencer :
- `composition.rs:67` — `bound += field.slope_bound()` ; **publie** `base.steepness = bound/π`,
  donc c'est ce site qui change des bits publiés, pas seulement une frontière de refus ;
- `mixed_water.rs:172` et `:249` — `slope_bound()` dans `slope_floor` et dans l'enveloppe ;
- `mixed_water.rs:175` et `:268` — `slope_envelope()`, mêmes deux fonctions ;
- `radial_impact.rs` — `if slope > medium.max_slope { Steepness }` dans `new`.

Reproduisent la formule et suivront : `tests_mixed_water.rs:299, 430, 432, 752` et
`examples/receive_mixed.rs:471`. Ce ne sont pas des bits gelés mais des recalculs parallèles —
ils doivent rester d'accord avec la bibliothèque, c'est leur seul rôle.

Ne bougent pas : `slope_envelope()` elle-même reste publiée et testée (`tests_pressure_multi`,
`restart_multisource` comparent deux chemins entre eux, pas des constantes) ; `bound_pressure.rs`
la retient comme métadonnée de préparation — à vérifier en P4, c'est peut-être un cinquième site.

Piège à éviter : changer les quatre sites et la valeur du seuil dans la même étape. Les refus se
déplacent alors pour deux raisons à la fois, et plus rien ne dit laquelle. **Un étage par
commit**, tests verts entre chaque.

Deuxième piège, plus coûteux : accepter un hachage qui bouge sans savoir dire pourquoi. Un
hachage global ne dit pas *lequel* des scénarios a changé — relever par scénario en P2.
