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
Session          : S38
État             : en cours
Battement        : 2026-09-07
Objectif         : Les saturations de modèle — filet ou maquillage, et combien de masse elles créent
```

### Plan

Action **S34-1**, angle mort **A146**, **reportée quatre fois**. S34 a audité les **garde-fous du
harnais** — chacun a-t-il été vu refuser ? Il a laissé dehors les **saturations du solveur**, qui
sont d'une autre nature : elles ne refusent pas un montage, elles **corrigent un état physiquement
impossible en cours de calcul**.

Les deux véhicules portent la même, écrite indépendamment — une convergence de plus entre les deux
lignées (`ADR-043`) :

```rust
if h < 0.0 { h = 0.0; hu = 0.0; }   // delta.rs, pas_naif et pas_equilibre
fn saturer(h, hu) { if *h < 0.0 { *h = 0.0; *hu = 0.0; } }   // shallow.rs
```

> **Ce que cette ligne fait vraiment, et que personne n'a chiffré.** Elle ne « corrige » pas : elle
> **crée de la masse** — remonter `h` de `−5·10⁻⁷` à `0` ajoute de l'eau qui n'existait pas — et elle
> **détruit de la quantité de mouvement** en remettant `hu` à zéro. Sur un schéma dont la
> conservativité est un argument écrit (`ADR-038` §2, C01-volume), c'est une fuite non comptée.

**Le critère de la session, en une ligne : une saturation rare est un filet, une saturation
fréquente est un solveur qui produit des états impossibles et qu'on maquille — et les deux sont
indiscernables tant que personne ne compte** (A146).

*Thèse déclarée, en trois volets falsifiables :*

1. **C01 et C03 ne déclenchent jamais la saturation.** Domaine entièrement mouillé, régime
   linéaire : un seul déclenchement y serait un défaut, pas un filet.
2. **C04 la déclenche, et de façon localisée** — quelques cellules au voisinage du front sec, pas
   une fraction notable du domaine. Si elle touchait des dizaines de cellules à chaque pas, le
   « front » mesuré par C04 serait en partie un artefact de saturation.
3. **Le résidu de `10⁻¹⁰ m` trouvé en S37 derrière le front de `shallow.rs` n'est pas produit par la
   saturation**, qui écrit **zéro exactement**. Il vient donc du flux du pas suivant, et c'est une
   autre affaire. *(Action S37-3.)*

**Le volet 2 est celui qui décide.** S'il est faux, C04 — qui sert de critère d'entrée au banc B3
(`ADR-031`) — mesure en partie son propre maquillage.

- [x] **P1** — plan, jeton.
- [x] **P2** — **recensement écrit** des saturations des deux solveurs : où elles sont, ce qu'elles
      empêchent, **ce qu'elles détruisent**. Distinguer l'initialisation, la protection de racine,
      la reconstruction hydrostatique et la vraie saturation d'état.
- [x] **P3** — instrumenter `delta.rs` : **compteur de déclenchements** et **masse créée cumulée**.
      Sans allocation (**I-06**), sans changer un seul résultat — **hashs de conformité vérifiés**.
- [x] **P4** — instrumenter `shallow.rs` de même.
- [x] **P5** — **mesurer sur C01, C03 et C04**, et classer chaque saturation : *filet* · *maquillage*
      · *jamais déclenchée*. Une saturation jamais déclenchée n'est pas innocente : elle n'a pas été
      testée (**L118**).
- [x] **P6** — le résidu de `10⁻¹⁰ m` (**S37-3**) : la saturation en est-elle la cause ? Le test doit
      pouvoir répondre **non**.
- [x] **P7** — l'ADR-045, A165, et la requalification datée d'A146.
- [x] **P7b** — exposer les compteurs au rapport du mode `physics` (décision D1).
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S37 laisse et qui commande cette session.**

- **Les deux solveurs saturent au même endroit et de la même façon**, sans s'être vus. C'est un
  argument de plus pour `ADR-043` §1 — et cela veut dire que la mesure vaudra pour les deux.
- **Le résidu de S37** : à la cellule 625 de C04, `delta.rs` porte `0` exactement et `shallow.rs`
  `1,05·10⁻¹⁰ m`. Les deux saturent pourtant à zéro. **Donc l'un des deux écrit ce résidu après
  avoir saturé** — c'est le flux, pas la saturation.
- **Le témoin naturel existe déjà** : `C01-volume` mesure la dérive relative du volume et la
  déclare *diagnostic, non probant*. Il devient probant le jour où une saturation se déclenche —
  **et C04 n'a aucune assertion de ce genre**.
- **Ce qu'il ne faut pas faire** : « corriger » une saturation trop fréquente en la retirant ou en
  la déplaçant. Le mandat est de **compter**, puis de dire. Retirer un filet sans savoir ce qu'il
  retient est le geste qui transforme un défaut visible en défaut invisible.
- **Ne pas toucher au seuil de sec** (**A163**, **S37-1**) : c'est une décision de conception en
  attente, et cette session la croisera sans la trancher.
- **État de départ** : `cargo test` = **74 tests** (32 cœur + 42 harnais, un `ignore`), `check` = 0
  échec, hashs `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`. Mode `physics` : **31 s** sur 60 s.
