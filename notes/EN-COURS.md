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
Session          : S65
État             : en cours
Agent            : Codex (git et cargo disponibles)
Objectif         : Brancher la graine sur les phases (S64-1), conserver la reproductibilité.
```

### Plan

- [x] **P1** — état réel, lectures de reprise, jeton et plan seul.
- [x] **P2** — fonction entière indexée par graine/composante, raccord scénario ; tests de reproductibilité et sensibilité.
- [>] **P3** — mesurer les déplacements nominaux et plusieurs graines ; consigner puis committer les nouveaux hashs avant leur vérification.
- [ ] **P4** — vérifier tests et scénarios sur références committées ; documenter portée statistique et limites.
- [ ] **P5** — rituel : journal, actions, index/décomptes, corrections datées, passation et jeton libre.

### Notes de reprise

Départ master 6a128e7, copie reprise-projet-29ef50 au même commit et propre. S57–S64 réalisées
par Claude Code : C22 clos, A103 close. Suite prioritaire S64-1 puis S64-3, puis S64-2 par ADR.
S63-1 (couche dispersive) reste ouverte. Aucun seuil de Hs resserré ici.
Les références H1 devront changer puisque les phases changent ; inscription dans un commit
précédant le check qui les juge. Pas de mode compatible caché pour une graine particulière.
La copie 29ef50 sera avancée vers master à chaque étape pour conserver une passation commune.

P2 : SeaState.graine raccordée au scénario ; SplitMix64 indexé, 32 bits hauts vers PhaseQ32.
Vecteurs graine zéro et accès direct, six graines distinctes répétées, témoin mer plate.
Suite initiale (avant test intégration ajouté) verte ; test intégration ciblé vert ; release
compilée. Diagnostic ignoré ajouté pour six graines x 32/256 composantes, à lancer en P3.
