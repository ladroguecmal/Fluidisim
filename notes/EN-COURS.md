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
Session          : S39
État             : en cours
Battement        : 2026-09-07
Objectif         : Réconcilier B-S27 — et surtout : mon corpus affirme quelque chose qui a été mesuré faux
```

### Plan

**Le fork a repris.** Une session a travaillé sur `claude/reprise-projet-5134cd` **après** S38 : son
dernier commit est à **01h23**, le mien à **00h37**. C'est le troisième épisode du même mécanisme, et
il était prévu — l'action **S35-7** le nommait, et elle n'appartenait pas à une session.

Huit commits, un ADR, deux fichiers de code, trois leçons, **deux angles morts de sévérité 1**. C'est
petit : la fusion de S35 en portait trente-cinq.

> **Mais ce n'est pas la taille qui commande cette session.** B-S27 s'intitule *l'éponge en eau
> dispersive **rétracte** ADR-034* — et ADR-034 de la lignée B, c'est **mon `ADR-042`**, importé en
> S35. **Mon corpus porte donc une conclusion qui vient d'être mesurée fausse.**

`ADR-042` §6 posait sa propre réserve n° 1 en toutes lettres : *le solveur est non dispersif ; une
éponge d'eau profonde doit absorber une **bande** de célérités, et la règle `λ/2` protège peut-être
exactement de cela ; c'est la première chose à mesurer.* **Elle a été mesurée, et la réponse est
que la réserve était fondée** — `R` vaut **24 %** en milieu dispersif là où le non dispersif
donnait 1,5 pour mille.

*Thèse déclarée, en deux volets :*

1. **La rétractation s'applique intégralement à mon corpus.** `ADR-042` D2 et D4 tombent ; D1 et D3
   tiennent. C'est mécanique, puisque le document est le même.
2. **Mais `ADR-043` D1 ne tombe pas avec.** Il rouvrait la borne haute de `λ_cut` **par deux voies
   indépendantes** : l'absorbeur mesuré (ADR-042 D4) et la dissipation qui rend le masque inutile
   (ADR-037). *La première voie tombe ; la seconde porte sur un autre objet et n'est pas touchée.*
   **Si ce volet est faux, c'est la distinction absorbeur/masque d'`ADR-043` §5 qui était mal
   posée**, et il faudrait le dire.

**Nouvelles collisions d'identifiants**, la carte de `FORK-S22-S26` est à étendre :

| identifiant côté B | ce qu'il désigne ici | devient |
|---|---|---|
| `ADR-035` | le nombre de Courant *(S25)* | **ADR-046** |
| `L86`–`L88` | trois leçons de S07-S08 | **L150**–**L152** |
| `A117`–`A118` | deux angles morts antérieurs | **A166**–**A167** |
| `S27` | ma session S27 n'existe pas | **B-S27** |

- [x] **P1** — plan, jeton.
- [x] **P2** — étendre la carte de `FORK-S22-S26` à B-S27, et **dater le troisième épisode**.
- [ ] **P3** — import d'`ADR-035` de B sous **ADR-046**, renvois renumérotés, en-tête de provenance.
- [ ] **P4** — leçons **L150–L152**, angles morts **A166–A167** *(deux de sévérité 1)*.
- [ ] **P5** — **ce que la rétractation déplace chez moi** : note corrective datée sur `ADR-042`,
      vérification de `ADR-043` D1, et `DOSSIER-B2` §3.1 bis qui s'appuyait dessus.
- [ ] **P6** — le code : `dispersif.rs` et `eponge.rs`. **Attention** : `eponge.rs` factorise le
      profil qui vit dans `shallow.rs`, que S38 a instrumenté. Compiler, puis les 77 tests.
- [ ] **P7** — **le procédé** : trois épisodes du même fork, et le remède écrit à chaque fois dans
      une seule branche. Écrire ce qui aurait marché, **et le répliquer des deux côtés le jour même**.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que l'amorce a montré, et qui n'était dans aucun jeton.**

- Un worktree **nouveau**, `friendly-bhabha-6da427`, sur `claude/reprise-projet-5134cd`. Les deux
  jetons disent `libre`, chacun dans son univers — exactement le mécanisme décrit dans
  `FORK-S22-S26` §1. **Les deux commandes d'amorce ont encore fonctionné** : le fork a été vu au
  premier geste, pour la troisième fois.
- **B-S27 a du contenu solide** : un milieu à dispersion exacte vérifié avant usage (écart de
  0,001 % sur trois modes), et un cas de garde qui a **refusé deux montages** avant d'en accepter un.
  Ce n'est pas un travail à recevoir avec réserve.
- **Deux divergences de spécification à signaler, pas à trancher** : côté B, le budget de la
  batterie `physics` est de **120 s** et il est dépassé (167 s, A114) ; ici, `SPEC-003 §1` dit
  **60 s** et le mode tourne en 33,7 s. Les deux lignées ont fait diverger la même contrainte.
- **`eponge.rs` est un refactor**, pas seulement un ajout : il extrait de `shallow.rs` le profil
  d'éponge pour le partager avec `dispersif.rs`. C'est le seul point du code où un conflit réel est
  possible avec l'instrumentation de S38.
- **État de départ** : `cargo test` = **77 tests** (32 cœur + 45 harnais, un `ignore`), `check` = 0
  échec, hashs `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`. Mode `physics` : 33,7 s sur 60 s.
- **Ce que cette session ne fait pas** : trancher le seuil de sec (**S37-1**, **A163**), qui était la
  session recommandée. Elle est reportée d'un cran, pour une raison qui se dit en une ligne :
  *un corpus qui affirme faux est plus urgent qu'un corpus qui laisse une question ouverte.*
