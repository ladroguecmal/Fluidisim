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
Session          : S36
État             : en cours
Battement        : 2026-09-06
Objectif         : `physics_shallow.rs` — les six montages de la lignée B, et la reproduction de ses chiffres
```

### Plan

Action **S35-1**. La réconciliation de S35 a importé le **solveur** de la lignée B, `shallow.rs`, qui
compile et passe ses dix tests dans cet arbre. **Ses montages de cas sont restés dehors** : six cas
canoniques — C01, C03, C04, C05, C06, C08 — vivent dans un `physics.rs` que S35 n'a pas fusionné,
parce que les deux lignées y ont des fonctions de même nom (`c03_seiche`, `ritter`,
`c08_convergence`) écrites chacune contre son propre solveur.

**Le découpage est décidé et il n'est pas une fusion** : un module neuf, `physics_shallow.rs`, et
`physics.rs` n'est pas touché. Deux jeux de montages, deux véhicules, aucun conflit de noms — et
l'oracle croisé d'`ADR-043` §3 devient exerçable (`FORK-S22-S26` §7).

> **Ce n'est pas un transport de code, c'est une reproduction.** Les verdicts de la lignée B sont
> cités dans le corpus depuis S35 **sans avoir jamais été exécutés ici** — `CAS-CANONIQUES`, « deux
> véhicules, deux colonnes », le dit en toutes lettres et appelle cette colonne *un témoignage, pas
> une mesure*. Cette session la transforme en mesure, ou montre qu'elle ne l'est pas.

*Thèse déclarée : les chiffres publiés par la lignée B se reproduisent dans cet arbre, au dernier
chiffre significatif donné.* Les quatre à confronter, pris dans les ADR importés :

| cas | grandeur | valeur annoncée | document |
|---|---|---|---|
| **C01** | `max|u|` du schéma naïf | **19,5 mm/s** pour un seuil de 1 | `ADR-038` §2 |
| **C03** | demi-vie à 800 mailles/λ | **43,1 périodes** | `ADR-039`, `ADR-040` |
| **C04** | erreur sur le front, ordre deux | **0,74 %** | `ADR-041` |
| **C08** | ordre de convergence mesuré | **`p` = 1,003** | `ADR-040` |

**Si un seul de ces quatre ne se reproduit pas, c'est le résultat de la session** — et il vaudra
plus que les quatre qui se reproduisent, parce qu'il désignera soit une dépendance non déclarée du
montage, soit une divergence entre les deux arbres que personne n'a vue.

- [x] **P1** — plan, jeton.
- [x] **P2** — `physics_shallow.rs` : squelette, déclaration dans `main.rs`, et **C01**. Compile et
      tourne avant d'aller plus loin.
- [x] **P3** — **C04** (Ritter) et ses références analytiques.
- [ ] **P4** — **C03** (seiche) et la mesure de période.
- [ ] **P5** — **C08** (convergence) et **C06** (Galilée, partiel).
- [ ] **P6** — **C05** (absorption) — le seul des six que cette lignée n'a **jamais** exécuté.
- [ ] **P7** — branchement dans `main.rs`, et le **double format d'écart** absolu / relatif
      (**A149**, action S35-2) — qui vaut pour les deux jeux de montages.
- [ ] **P8** — **confronter les quatre chiffres** à ce qui est publié, et écrire le verdict.
- [ ] **P9** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S35 laisse et qui commande cette session.**

- `shallow.rs` est **déjà dans l'arbre** et exporté (`water_core::{Flux, Shallow1D}`). Rien à
  importer côté solveur.
- Le bloc de montages de la lignée B fait **1132 lignes** et ne dépend que de `Cas`, `Shallow1D`,
  `Flux`, `G` et `crate::host_impl` — dont les trois types (`ArenaAllocator::with_capacity`,
  `SequentialJobs`, `StderrSink`) existent ici à l'identique. **Aucune adaptation d'API attendue.**
- Il se lit dans la branche conservée :
  `git diff a6cfe6f claude/reprise-projet-5134cd -- code/water-harness/src/physics.rs`
- `Shallow1D` porte tout ce qu'il faut : `regler_ordre2`, `regler_rk2`, `regler_eponge`,
  `regler_flux`, `regler_equilibrage`, et quatre `configure_*` (barrage, seiche, bosse, paquet).
- **État de départ à ne pas perdre de vue** : `cargo test` = **55 tests verts**, `water-harness
  check` = 2 scénarios, 0 échec, hashs `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`. Tout écart sur
  ces deux hashs serait un défaut de cette session, pas un résultat.
- **C05 n'a jamais tourné ici** : la lignée d'accueil ne l'a pas exécuté, et c'est lui qui a
  éliminé le réglage d'`ADR-005 §2`. Son montage porte des **conditions de mesure révisées**
  (`ADR-042` §7) — témoin à `σ_max = 0`, `σ_max·dt` rapporté. Les reprendre telles quelles.
- **Ne pas toucher à `physics.rs`.** C'est la condition qui rend l'oracle croisé possible ; la
  violer ferait perdre à la session son objet.
