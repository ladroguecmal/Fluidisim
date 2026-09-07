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
Session          : S43
État             : en cours
Battement        : 2026-09-07
Objectif         : L'essai à zéro de C08 — un solveur qui ne converge pas reçoit l'ordre 1
```

### Plan

Action **S42-1**. C08 mesure un **ordre de convergence** par Richardson, `p = log₂(|e₀−e₁|/|e₁−e₂|)`.
S42 laissait la question ouverte : *que veut dire « résultat attendu zéro » pour une mesure d'ordre ?*

**La réponse est que l'essai à zéro de C08 ne porte pas sur le solveur mais sur l'estimateur.** Un
montage sans objet à mesurer, ici, c'est une suite d'erreurs qui **ne converge pas** : trois grilles,
trois erreurs identiques. Il n'y a pas d'ordre. L'estimateur doit le dire.

**L'inspection préalable a trouvé le défaut avant d'écrire une ligne**, et il est du même genre que
celui de S42 — c'est l'action **S42-2**, arrivée un jour plus tôt que prévu.

**Le harnais porte TROIS estimateurs d'ordre**, et ils ne refusent pas pareil :

| | où | ce qu'il fait quand `d₁ = 0` |
|---|---|---|
| `Convergence::ordre()` | `physics.rs` | **`Ordre::Indetermine`** — un refus typé, avec plancher |
| `ordre_grossier_estime` | `physics.rs` | **rend `1.0`** |
| `c08_convergence` | `physics_shallow.rs` | **`log₂(0/0)`**, aucun garde |

> **Le repli à `1.0` est le pire des trois, et pas parce qu'il est faux.** `d₁ = 0` veut dire que
> deux grilles successives donnent **la même erreur** — le solveur ne converge pas. L'estimateur
> répond alors « ordre 1 », **c'est-à-dire exactement l'ordre nominal du schéma**, la valeur qu'on
> espère lire. Et le garde-fou **G10**, qui signale un ordre hors de `[0,3 ; 3,0]`, ne bronche pas :
> `1,0` est dedans. **Le repli est silencieux par construction.**

*Thèse déclarée : sur une suite d'erreurs constante — aucune convergence — `ordre_grossier_estime`
rend `1,0` sans aucun signalement, et le filtre d'oracle qui en dépend s'applique comme si de rien
n'était.*

**Et une observation sur l'audit de S34**, qui a examiné G10 et corrigé son bornage : le repli est
**sur la ligne juste au-dessus du clamp**. L'audit a regardé le `clamp` et pas le `if`. Si la thèse
tient, c'est une leçon sur ce que voit un audit de garde-fous.

- [x] **P1** — plan, jeton.
- [x] **P2** — **l'essai à zéro de l'estimateur**, sur des suites synthétiques : `e_k = C·dx_k^p`
      doit rendre `p` **exactement** ; une suite **constante** doit être refusée. Constater d'abord.
- [x] **P3** — remplacer les replis par un refus, **avec leur témoin** (**L119**) — et vérifier
      que le signalement se déclenche, ce que `1,0` empêchait.
- [x] **P4** — **les trois estimateurs refusent-ils pareil ?** Le corpus contient déjà la bonne
      solution, `Ordre::Indetermine` ; les deux autres l'ignorent (**S42-3**, **L162**).
- [x] **P5** — vérifier qu'**aucun chiffre publié ne bouge** : `p = 0,9997` (S36), les ordres du
      tableau de `ADR-040` §3, et les deux hashs.
- [x] **P6** — répercussions : `A170` étendu ou confirmé, `CAS-CANONIQUES` C08, et ce que S34
      n'avait pas vu.
- [x] **P7a** — rituel : journal, leçons L163-L164, actions S43-1 à S43-3.
- [ ] **P7b** — rituel : index, décomptes, jeton libéré.

### Notes de reprise

**Ce qui commande cette session.**

- **`ordre_grossier_estime` est déjà une fonction pure**, extraite en S34 pour être testable, et
  elle **a déjà deux tests** (`pre_asymptotique`, `saine`). Aucun ne lui donne une suite qui ne
  converge pas. *Un garde-fou testé sur ce qu'on a pensé à lui donner n'est pas un garde-fou testé.*
- **Elle a deux replis, pas un** : `if erreurs.len() < 3 { return (1.0, 1.0) }` en plus du
  `else { 1.0 }`. Les deux rendent l'ordre nominal.
- **Le chemin du défaut** : `p_brut` → `p_grossier` → `e_oracle = e_max / ratio^p` → filtre
  `retain(|e| e >= 30·e_oracle)`. Un `p` faux déplace le seuil de filtrage des grilles.
- **`Ordre::Indetermine` existe et fonctionne** — c'est le modèle à suivre plutôt qu'à réinventer.
- **Ne pas casser** : `p = 0,9997` (C08 sur `shallow.rs`, S36), et le tableau d'ordres d'`ADR-040`
  §3 — 0,654 / 0,621 / 1,000 pour les trois schémas.
- **État de départ** : `cargo test` = **93 tests** (38 cœur + 55 harnais, deux `ignore`), `check` = 0
  échec, hashs `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`.
