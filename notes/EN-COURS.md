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

Session : S137 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S136-1 — spécifier le banc B2. **La prémisse était fausse** : il l'est déjà.

### Plan

**Ce plan a été écrit après coup, et c'est un manquement.** La règle veut qu'il soit déclaré et
committé seul *avant* la première modification ; il ne l'a pas été. Une coupure en cours de
session n'aurait laissé aucune trace d'intention, ce qui est exactement le défaut que
`notes/EN-COURS.md` existe pour éviter. Consigné plutôt que masqué.

- [x] **P1** — amorce : état réel, copies, jeton.
- [x] **P2** — lire avant d'écrire : B2 est spécifié dans PLAN-BENCHMARK et DOSSIER-B2 (S16).
- [x] **P3** — établir ce que B2 et B10 mesurent réellement, et constater le trou.
- [x] **P4** — ADR-093, extension de PLAN-BENCHMARK §B10, trois notes correctives datées.
- [x] **P5** — livrable, leçon, rituel de fin, fusion `--ff-only`.

### Notes de reprise

La session n'a produit aucun code : elle corrige un renvoi et comble un trou de plan.

Ce qu'elle a trouvé : B2 choisit la technologie de W, B10 mesure la cavité, et **aucun banc ne
mesure la source d'onde d'un impact**. Le renvoi « à calibrer B2 » d'ADR-060, recopié par ADR-083
puis ADR-092 — la mienne — a masqué ce trou depuis S77.

Second défaut trouvé en fin de session : mes leçons de S136 et S137 réutilisaient L214 et L215,
déjà prises par Codex. Renumérotées L216 et L217. **Vérifier le dernier numéro dans le fichier,
jamais depuis sa propre mémoire de la session précédente** — surtout après avoir fusionné le
travail d'un autre agent.

Pour S138 sans relire : S137-1 est un audit des renvois. La méthode est celle de S11 et S15 —
recherche de texte plutôt que relecture. Motifs : « à calibrer », « voir ADR-0xx », « traité en
S », les identifiants de bancs, les sections de SPEC. Pour chacun, ouvrir la cible et vérifier
qu'elle porte ce qu'on lui confie. Commencer par les renvois vers des **bancs** et des **sections
de SPEC**, les plus susceptibles de n'avoir jamais couvert ce qu'on leur attribue. Et y ajouter
les **numéros** — leçons, angles, ADR — dont S137 vient de montrer qu'ils peuvent entrer en
collision quand deux agents travaillent en parallèle.
