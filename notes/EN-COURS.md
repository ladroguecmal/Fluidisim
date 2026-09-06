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
Session          : S35
État             : terminée
Battement        : 2026-09-06
Objectif         : Réconcilier le second fork — la lignée S22–S26 dans la lignée S22–S34
```

### Plan

**Le dépôt a forké une seconde fois, et personne ne l'avait vu.** Le premier fork (S07→S15) est
documenté dans une lignée qui n'est pas celle-ci : `docs/registres/FORK-S08-S15.md` n'existe pas
ici. **Cette lignée ignore qu'elle est une branche.** Le second fork est parti de S21
(`a6cfe6f`, 2026-09-06 11h35) et a produit deux histoires parallèles dans la même journée :

| | commits depuis S21 | dernier | ADR | leçons | angles morts |
|---|---|---|---|---|---|
| **cette lignée** (`claude/s22-suite`) | 88 | S34, 19h14 | 37 | L121 | A148 |
| **lignée B** (`claude/reprise-projet-5134cd`) | 35 | S26, 17h12 | 34 | L85 | A116 |

Une troisième lignée, `master`, est morte à **S17** — 195 commits de retard, sans code. Elle
détient le registre du **premier** fork. Hors mandat de cette session ; voir les points ouverts.

**Cinq identifiants d'ADR sont en collision** — 030 à 034 existent des deux côtés, avec des sujets
différents. Quinze leçons (L71–L85) et douze angles morts (A105–A116) le sont aussi. Et les deux
lignées ont numéroté leurs sessions S22 à S26.

> **La lignée d'accueil est celle-ci**, parce qu'elle est la plus avancée et la plus récente. Ce
> n'est pas un jugement de valeur sur le travail de la lignée B : c'est le choix qui déplace le
> moins de documents.

*Thèse déclarée : les deux lignées ont travaillé sur le même sujet sans le savoir, et leurs
conclusions ne concordent pas.* Le point de contact est **l'éponge et la borne de `λ_cut`** :
côté B, `ADR-034` s'intitule « l'éponge mesurée, et la borne de λ_cut rouverte » ; ici, S33 conclut
qu'il est **inutile d'imposer la décroissance par une éponge**, la dissipation la produisant seule.
Deux sessions ont mesuré la même chose le même jour sans se voir. Si elles convergent, c'est une
réplication indépendante — la seule qu'ait ce projet. Si elles divergent, l'une des deux se trompe,
et il faut dire laquelle.

- [x] **P1** — plan, jeton.
- [x] **P2** — registre `FORK-S22-S26` : le constat, la carte de renumérotation complète, et la
      règle qui aurait évité le fork. Écrit **avant** tout déplacement de document.
- [x] **P3** — import des cinq ADR de la lignée B, renumérotés **038–042**, renvois internes
      réécrits, en-tête de provenance daté sur chacun.
- [x] **P4** — report des quinze leçons **L71–L85 → L122–L136**.
- [x] **P5** — report des douze angles morts **A105–A116 → A149–A160**, sévérités conservées.
- [x] **P6** — l'éponge et `λ_cut` : confronter `ADR-041` (ex-034 de B) à `ADR-037` et à S33.
      Convergence ou contradiction — et une note corrective datée du côté qui a tort.
- [x] **P7** — documents partagés modifiés par la lignée B seule : `ADR-005`, `ADR-007`,
      `DOSSIER-B2`. Notes correctives datées, jamais de réécriture.
- [x] **P7b** — `CAS-CANONIQUES`, modifié **des deux côtés** : confronter avant de reporter.
- [x] **P8** — le **code** : `shallow.rs` (1070 lignes) n'a pas d'équivalent ici, et `physics.rs`
      a été modifié des deux côtés (+1139 contre +1975). Constat et découpage du travail restant.
      **Aucun import à l'aveugle** — le code se fusionne en le compilant, pas en le recopiant.
- [x] **P9a** — rituel : journal, leçons L137-L140, actions S35-1 à S35-8.
- [x] **P9b** — rituel : index, décomptes, jeton libéré.

### Notes de reprise

**Ce qui a déclenché cette session.** L'utilisateur a demandé « reprends le projet » depuis un
worktree positionné sur `master` (S17). Les deux commandes d'amorce de `CLAUDE.md` —
`git worktree list` et `git branch -a` — ont révélé le fork immédiatement. **Elles ont fonctionné.**
Le dispositif d'amorce n'est pas en cause ; ce qui manquait, c'est que la lignée d'accueil n'a
jamais reçu le registre du premier fork et ne savait donc pas qu'elle était exposée au second.

**Repères de fusion, pour ne pas les recalculer :**

- Point de fork : `a6cfe6f` — *S21 P7 — rituel de fin : journal, leçons L67-L70, index, A103
  arbitrable, jeton libéré*. Les deux lignées en descendent directement.
- Diff de la lignée B depuis le fork : **6 ajouts, 14 modifications**. C'est peu — la fusion des
  documents est faisable ; c'est le code qui est lourd.
- Fichiers partagés modifiés des deux côtés : `REPRISE.md`, `docs/00_INDEX.md`,
  `notes/JOURNAL.md`, `notes/LECONS.md`, `docs/registres/ANGLES-MORTS.md`,
  `code/water-harness/src/physics.rs`, `code/water-harness/src/main.rs`, `code/README.md`,
  `code/water-core/src/lib.rs`.
- Fichiers partagés modifiés **par B seule** — donc reportables sans conflit de fond :
  `docs/adr/ADR-005-zone-de-transition.md`, `docs/adr/ADR-007-interface-solveur.md`,
  `docs/validation/CAS-CANONIQUES.md`, `docs/validation/DOSSIER-B2.md`.
- `shallow.rs` est un **fichier neuf** côté B : aucun conflit de nom, mais il dépend de
  modifications de `lib.rs` et de `physics.rs` qui, elles, entrent en conflit.

**Carte de renumérotation** — la référence est le registre `FORK-S22-S26` produit en P2. Les
numéros de session de la lignée B ne sont **pas** renumérotés : ils sont préfixés `B-` (B-S22 à
B-S26), parce qu'une session est un événement daté, pas un identifiant de document.
