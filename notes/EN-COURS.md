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
Session          : S10
État             : en cours
Battement        : 2026-09-05
Objectif         : `CondensedState`, la persistance hors caméra, et sa confrontation avec
                   `CoastalState` — configuration L22 signalée par S09
```

### Plan

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [ ] **P2** — l'analyse, couche par couche : qu'est-ce qui doit réellement persister quand un
  domaine quitte la caméra, quand la partie est sauvegardée, quand un joueur rejoint.
  *Thèse (L03) : la question « quel format pour `CondensedState` » est probablement **mal posée**.
  ADR-013 §6 a dissous la simulation hors caméra — le repli **est** la destruction du domaine. Si
  c'est vrai, il n'y a rien à condenser, et le format cherché n'a pas d'objet.*
- [ ] **P3** — `ADR-022` §1–2 : la décision, et la démonstration couche par couche.
  *Forme : un ADR, pas une note. ADR-007 §7.3 appelle explicitement « un ADR à écrire », et une
  décision qui en change une autre ne se corrige pas, elle se remplace.*
- [ ] **P4** — `ADR-022` §3 : `SeedState` — ce que `condense`/`restore` échangent réellement, et
  l'unification avec `CoastalState`.
  *Thèse : `condense` est une opération **d'outil de cuisson**, pas d'exécution ; `restore` est une
  opération d'exécution. Ce sont les deux moitiés d'un même mécanisme, écrites à deux sessions
  d'intervalle sous deux noms — la configuration L22 exacte.*
- [ ] **P5** — `ADR-022` §4 : ce que l'eau met dans une sauvegarde, et le rechargement, l'arrivée
  en cours de partie, le redémarrage serveur.
- [ ] **P6** — `ADR-022` §5 : la couche V, seule persistance vraie, dans un monde partagé ·
  §6 conséquences sur les interfaces · §7 ce qui reste ouvert. Invariant **I-17** si la
  démonstration de P2 tient.
- [ ] **P7** — répercussions : notes correctives dans SPEC-004 (§10.2 et les signatures),
  ADR-007 §7.3, renvoi depuis SPEC-005 §6.
- [ ] **P8** — index, invariants, angles morts, README si nécessaire.
- [ ] **P9** — rituel de fin (`REPRISE.md` §6) : journal S10, leçons, index, jeton libéré.

### Notes de reprise

*(Vide au démarrage. Y déposer au fil de l'eau ce qui n'est pas encore dans un fichier.)*

- **Contradiction trouvée avant même d'ouvrir le sujet, et elle est interne à S01** : ADR-013 §6
  pose que le hors caméra est une **destruction** de domaine (« il n'existe pas de simulation
  ralentie hors caméra »), tandis qu'ADR-007 §7.3 réclame un format de `CondensedState` « pour la
  persistance hors caméra ». Les deux ADR sont de la même session. La revue croisée S05 a confronté
  les vingt ADR et ne l'a pas vue ; la revue S08 a confronté les cinq SPEC et ne l'a pas vue non
  plus. Hypothèse à vérifier en P2 : **un point inscrit dans une liste « ce qui reste ouvert »
  échappe aux audits**, parce qu'un audit vérifie ce qui est affirmé et qu'un point reporté se lit
  comme une lacune connue, pas comme une contradiction.
