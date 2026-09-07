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
Session          : S41
État             : terminée
Battement        : 2026-09-07
Objectif         : Relire les sept angles morts de sévérité 1 importés — et vérifier, pas commenter
```

### Plan

Actions **S35-5** et **S39-3**. Sept angles morts de sévérité 1 sont entrés par deux
réconciliations — **A152**, **A155**, **A156**, **A157**, **A159** de B-S22–B-S26, **A166** et
**A167** de B-S27. **Aucun n'a été examiné par cette lignée.** Ils ont été reportés tels quels,
sévérités comprises, comme `FORK-S22-S26` §4 le dit en toutes lettres.

> **S40 vient de montrer pourquoi cela ne suffit pas.** `A163` était un angle mort importé, de
> sévérité 1, et son énoncé — *deux seuils incompatibles* — désignait une incompatibilité **qui
> n'existe pas**. Le fait rapporté était exact ; le défaut nommé était à côté. Un angle mort peut
> être **vrai et mal formulé**, et il coûte alors quatre reports (**A169**).

**Une relecture qui se contenterait de commenter ne vaudrait rien.** Chaque fiche reçoit donc quatre
questions, dont **trois se vérifient** :

| | question | comment on y répond |
|---|---|---|
| **Q1** | le fait rapporté est-il exact ? | relire la source, et rejouer si un chiffre est en jeu |
| **Q2** | l'énoncé désigne-t-il le bon défaut ? | le seul jugement des quatre — celui qui a manqué à A163 |
| **Q3** | **le défaut existe-t-il ici, aujourd'hui ?** | **dans le code et le corpus d'accueil** — vérifiable |
| **Q4** | une action en découle-t-elle, et existe-t-elle ? | `QUESTIONS-OUVERTES` |

**Q3 est le cœur.** Ces sept défauts ont été trouvés sur **le code de la lignée B**. Rien ne dit
qu'ils valent ici : certains y sont déjà traités par une autre voie, d'autres y sont peut-être
présents et non vus. C'est cela qui se mesure.

*Thèse déclarée : les sept énoncés sont exacts, mais **au moins deux désignent un défaut présent
dans le code ou le corpus d'accueil et non traité**.* Les deux plus probables :

- **A167** — *tout montage doit venir avec un essai dont le résultat attendu est zéro.* C01 et C05
  ont un témoin ; **C03, C04 et C08 n'en ont pas** à ma connaissance.
- **A155** — *une simplification algébrique efface son domaine.* `delta.rs` est à l'ordre un, donc la
  forme courte du terme de fond y est valide — **et rien n'y rappelle l'hypothèse**. Le piège est
  armé pour le jour où quelqu'un l'étendra.

**Si la thèse est fausse et que les sept sont sans objet ici**, c'est un résultat aussi : cela
voudrait dire qu'un angle mort trouvé sur un véhicule ne se transporte pas, et la procédure d'import
de `FORK-S22-S26` §4 devrait le dire.

- [x] **P1** — plan, jeton.
- [x] **P2** — **A152**, **A156** : les conditions de mesure et le classement par scalaire. Les deux
      portent sur `CAS-CANONIQUES`, et se vérifient en le lisant cas par cas.
- [x] **P3** — **A155**, **A159** : les deux défauts d'écriture — une simplification dans le code,
      une formule dans le corpus. **Q3 se vérifie sur `delta.rs` et sur les ADR à constantes.**
- [x] **P4** — **A157**, **A166** : les deux limites de méthode. A157 recoupe `ADR-047` ; A166 dit
      qu'une mesure ne peut pas dire de quel cadre elle dépend — **lesquelles des nôtres sont dans
      ce cas ?**
- [x] **P5** — **A167** : l'essai à zéro. **Compter combien de nos montages en ont un**, et écrire
      celui qui manque au plus exposé.
- [x] **P6** — le registre `AUDIT-ANGLES-IMPORTES-S41`, et les requalifications s'il y en a.
- [x] **P7a** — rituel : journal, leçons L158-L160, actions S41-1 à S41-4.
- [x] **P7b** — rituel : index, décomptes, jeton libéré.

### Notes de reprise

**Ce qui commande cette session.**

- **La leçon d'A163** : *le fait peut être exact et le défaut mal nommé*. C'est Q2, et c'est le seul
  point où cette session juge plutôt qu'elle ne mesure.
- **A157 recoupe directement `ADR-047`** : le seuil de 10⁻⁶ y est décrit comme *reproductible et
  dénué de sens*, et S40 vient d'établir que sa valeur est **libre sur sept décades**. Les deux
  énoncés ne sont pas contradictoires — A157 parle du seuil de **mesure du front**, `ADR-047` du
  seuil **du solveur** (**L148**) — mais la fiche ne le dit pas, et un lecteur les confondra.
- **A159 est actionnable et ne l'est pas encore** : *repérer les formules dont dépend une décision et
  les refaire une fois, avec leurs constantes*. Personne ne l'a fait ici.
- **A166 est la seule des sept qui énonce une limite de la méthode elle-même**, pas un défaut
  réparable. Sa relecture doit dire quelles conclusions du corpus sont exposées — c'est une liste,
  pas une opinion.
- **État de départ** : `cargo test` = **85 tests** (38 cœur + 47 harnais, deux `ignore`), `check` = 0
  échec, hashs `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`. Mode `physics` : 33,7 s sur 60.
