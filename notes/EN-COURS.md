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
Session          : S20
État             : terminée
Battement        : 2026-09-05
Objectif         : écrire H1 — le premier étage du harnais. Du code qui s'exécute.
```

### Plan

Dix-neuf sessions de conception, rien qui tourne. ADR-020 est acté et l'ajout de code est autorisé :
H1 est le premier étage du chemin critique, et le seul livrable **vérifiable par quelqu'un d'autre
que moi**.

**Définition de fin, non négociable** (SPEC-003 §1) : la batterie déterministe tourne en **moins de
60 secondes**, sans GPU, à chaque commit.

- [ ] **P1** — plan, jeton, et lever la règle « Markdown uniquement » de `CLAUDE.md`.
- [x] **P2** — `water-core` : types, services d'hôte, **B minimal**, hash de conformité.
  *Thèse : le déterminisme bit à bit d'I-03 ne survit pas à un `sin()` de bibliothèque standard —
  les fonctions transcendantes ne sont pas spécifiées bit à bit et diffèrent entre plateformes.
  ADR-003 impose « sémantique IEEE stricte, ordre de sommation fixé » et ne dit rien des
  transcendantes. Si la thèse tient, c'est une découverte que dix-neuf sessions de conception n'ont
  pas faite, et elle sort à la première ligne de code.*
- [x] **P3** — `water-harness` : lecteur de scénario, hôte, allocateur compteur avec `seal()`,
  mode `check`.
- [x] **P4** — **compiler, exécuter, vérifier.** Un scénario réel, un hash, un compte
  d'allocations, un temps mesuré.
- [x] **P5** — `ADR-029` : le langage, et ce que l'écriture du code a appris.
- [x] **P6** — index, angles morts, décomptes.
- [x] **P7** — rituel de fin.

### Notes de reprise

- **Le langage se tranche empiriquement autant que techniquement.** Inventaire de la machine :
  `rustc 1.97` et `cargo` présents ; **aucun compilateur C++** — ni `cl`, ni `g++`, ni `clang`, ni
  `cmake`. Écrire le cœur en C++ produirait du code que je ne peux **ni compiler ni exécuter**,
  c'est-à-dire exactement ce que H1 doit cesser de produire.
  L'argument technique va dans le même sens, et il est plus fort : **Rust ne contracte pas les
  opérations flottantes** (`a*b+c` n'est jamais fusionné en FMA sans appel explicite), là où GCC et
  Clang le font **par défaut** — `-ffp-contract=fast`. C'est L64 écrite hier : entre deux
  représentations également capables, choisir celle dont la propriété critique survit à la
  négligence.
- **Zéro dépendance.** `water-core` n'a aucune dépendance — c'est ADR-020. Le harnais non plus : le
  lecteur de scénario est écrit à la main sur le sous-ensemble de TOML dont SPEC-003 §3 a besoin.
  Motif secondaire mais réel : la construction doit marcher sans réseau.

#### P2 et P3 — le code

`code/water-core` (types, phases, hôte, B minimal, hash) et `code/water-harness` (scénario, hôte
harnais, mode check). **Zéro dépendance**, ni dans l un ni dans l autre : le lecteur de scénario est
écrit à la main. 14 tests, tous au vert. Le harnais échoue correctement tant qu aucune référence de
hash n est inscrite — c est l état de ce commit.

**La thèse de P2 est confirmée, et c est la trouvaille de la session.** `f32::sin` n est pas
spécifié bit à bit : IEEE 754 impose l exactitude des quatre opérations et de la racine carrée,
jamais celle des transcendantes. ADR-003 §2 énumère « sémantique IEEE stricte, ordre de sommation
fixé, PRNG entier » — la liste est **incomplète**, et l omission ne se voit qu en écrivant le code.
Corrigé sans rien inventer : ADR-003 §2.2 posait déjà que « seules des phases repliées passent au
GPU ». Le même mécanisme sert ici — phase u32 en fraction de tour, part temporelle entièrement
entière, polynôme à coefficients fixes n employant que +, − et ×.

**Trois erreurs à moi, trouvées par les tests.** Logique de quadrant fausse (sin(90°) donnait 0) ;
valeur de référence FNV inventée, recalculée indépendamment en Python ; et un test qui affirmait une
propriété vraie avec des données incapables de la révéler.
