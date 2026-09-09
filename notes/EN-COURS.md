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

Session : S119 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : A194 — l'hôte doit pouvoir savoir **avant de publier** quelles dates le montage
mixte peut servir. Construire l'horizon effectif et l'état du montage, et prouver par
balayage que ce qui est annoncé est exactement ce que la requête accepte.

### Plan

- [x] **P1** — passation, jeton, plan.
- [ ] **P2** — ADR-079 : ce que le contrôleur tient, ce que le montage exige, et pourquoi
      l'annonce est une fonction séparée plutôt qu'un contrôle de plus dans `update`.
- [ ] **P3** — construire `mixed::horizon` et `mixed::plan`, `Controller::context`,
      factoriser les contrôles indépendants des points depuis `sample_world_batch`.
- [ ] **P4** — le test qui compte : balayage d'instants, `plan(t)` comparé à ce que la
      séquence réelle (update puis requête à lot vide) fait vraiment.
- [ ] **P5** — recevoir dans la campagne `cycle_mixed`, avec bloc de mise en régime (A195).
- [ ] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 6700773 = master ; trois copies coïncident (master, 29ef50, 2d3506), 5134cd archivée,
c107bf sur la ligne S44. Worktree `claude/reprise-projet-2d3506`.

Le problème, tel que S118 l'a constaté : `update(6 s)` réussit alors que les impacts expirent
à 4 s. Le contrôleur valide **sa** fenêtre ; la requête mixte refuse ensuite sur
`renewal_deadline`. Rien ne permet à l'hôte de le savoir avant de publier.

Ce qui est disponible sans publication : `bound`, `impacts` (donc `renewal_deadline` et
`loss_known`), et le contrôleur (donc sa fenêtre). Tout ce que `sample_world_batch` vérifie
indépendamment des points est donc décidable avant publication — c'est ce qui rend
l'équivalence annonçable *et* testable, et non une simple heuristique.

Piège à éviter : réimplémenter les contrôles dans l'annonce. Deux implémentations du même
contrôle divergent (L137 est la même leçon, appliquée au code). Factoriser, ne pas recopier.
