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
Session          : S37
État             : en cours
Battement        : 2026-09-07
Objectif         : Exercer l'oracle croisé — et d'abord établir ce qu'il peut dire
```

### Plan

Action **S35-3**. `ADR-043` §3 promet un **oracle croisé** : deux implémentations indépendantes du
même modèle, dont le désaccord sur un cas sans solution analytique désigne une faute
d'implémentation. S36 a monté les deux véhicules dans le même binaire, mais **aucun cas ne compare
leurs deux sorties** : chacun rejoue ses propres chiffres. La promesse n'est pas tenue.

> **Ce qui a été vérifié avant d'écrire ce plan, et qui le rend possible.** Les montages de C01
> coïncident **rigoureusement** : même grille — centre de cellule à `(i+½)·dx` des deux côtés —
> même fond `−3 + 0,05·x`, même `η₀`, même `dx = 0,25 m`, mêmes 160 cellules, même CFL de 0,45.
> Rien n'a été à adapter. Ce n'est pas un hasard : les deux lignées lisaient le même
> `CAS-CANONIQUES`.

**Mais un oracle non calibré ne dit rien, et celui-ci a un plancher qu'aucun document ne mentionne :
`delta.rs` calcule en `f32`, `shallow.rs` en `f64`.** Sept ordres de grandeur séparent leurs
arrondis. Un écart entre les deux n'est donc lisible **qu'au-dessus** d'un plancher qu'il faut
mesurer d'abord — sans quoi on lirait la précision machine comme un défaut de schéma, ou l'inverse.
C'est **L131** — *avant de corriger, vérifier qu'on mesure la bonne chose* — et **A157** : un seuil
posé sans fondement est reproductible et dénué de sens.

*Thèse déclarée, en deux volets :*

1. **Sur C01**, les deux concordent **au plancher `f32`** : l'écart mesuré est de l'ordre de
   `ε_f32 × h`, soit quelques `10⁻⁷ m`, et **pas davantage**.
2. **Sur C04**, où la solution n'est pas triviale, l'écart entre les deux reste **au niveau de la
   troncature du schéma** — quelques pour mille sur le front — et non au-dessus.

**Si le volet 1 est faux, c'est une faute d'implémentation dans l'un des deux**, et l'oracle aura
servi exactement comme `ADR-043` l'annonce. **Si le volet 2 est faux, c'est plus intéressant
encore** : deux schémas réputés identiques ne le seraient pas, et il faudrait dire en quoi.

- [x] **P1** — plan, jeton.
- [x] **P2** — **vérifier que les deux montages sont le même**, à `t = 0`, champ à champ : fond,
      hauteur, grille. Sans cela, tout ce qui suit compare deux objets différents.
- [x] **P3** — **établir le plancher de l'oracle** : ce que la seule différence `f32`/`f64` produit
      comme écart, et donc au-dessous de quoi un désaccord ne dit rien.
- [x] **P4** — le comparateur : même montage, même temps final, écarts champ à champ en `L∞` et
      `L¹`. **Pas de comparaison pas à pas** : les deux ne partagent pas leurs pas de temps.
- [x] **P5** — **C01 comparé**, le cas où les deux sont exacts et où un désaccord serait sans
      ambiguïté.
- [x] **P6** — **C04 comparé**, montage aligné à 800 mailles de 5 cm : le cas où un désaccord serait
      **physique** et non arithmétique.
- [ ] **P7** — le verdict, et un ADR : **ce que cet oracle peut dire, et ce qu'il ne peut pas**.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S36 laisse et qui commande cette session.**

- Les deux véhicules sont dans le même binaire : `water_core::{Delta1D, Bassin}` et
  `water_core::{Shallow1D, Flux}`. Rien à importer.
- **`delta.rs` est en `f32`, `shallow.rs` en `f64`.** `ε_f32 ≈ 1,19·10⁻⁷` ; sur une hauteur de 3 m,
  l'arrondi vaut déjà `≈ 3,6·10⁻⁷ m`. C'est le fait central de la session et il n'est écrit nulle
  part dans le corpus.
- **Alignement des grilles, vérifié avant le plan** : `delta.rs` place le centre de la cellule
  interne `i` à `origine + (i+½)·dx` (ses tableaux bruts portent deux cellules fantômes, `k = i+1`) ;
  `shallow.rs` à `(i+½)·dx`, sans fantômes. **Les cellules internes coïncident.**
- **Les temps de sortie doivent être comparés, pas les pas.** `delta.rs` avance par
  `avancer_equilibre(duree_s)`, `shallow.rs` par `avancer_jusqu_a(t_fin, cfl)` ; leurs pas de temps
  diffèrent dès le premier, puisque `dt_cfl` est calculé dans deux précisions.
- **C01 des deux côtés** : `Bassin::c01()` et `montage_c01()` — 40 m, 160 cellules, `dx = 0,25 m`,
  fond `−3 → −1`, `η₀ = 0`, repos.
- **C04 demande un alignement** : `Bassin::c04()` est à 800 cellules de 5 cm sur `[−20, +20]`, tandis
  que `c04_ritter` de la lignée B tourne à 1600 mailles de 2,5 cm sur `[0, 40]`. `barrage()` étant
  paramétrique, l'aligner coûte un appel — mais **l'origine diffère de 20 m** et il faut en tenir
  compte dans la comparaison des abscisses.
- **État de départ** : `cargo test` = **68 tests** (32 cœur + 36 harnais, un `ignore`), `check` = 0
  échec, hashs `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`. Mode `physics` : **31 s** sur 60 s de
  budget, dont 12,5 s pour le second véhicule — **cette session doit surveiller ce qu'elle ajoute**
  (S36-4, **A158**).
