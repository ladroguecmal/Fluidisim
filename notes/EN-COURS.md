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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S572 — **terminée**. En autonomie : **le lot** (dû ; feuille de route S569–S571), puis **7.7 — les tuiles** (SPEC-006 §5.2) :
l'unité de publication de la traversabilité.

**Ce que la session fait.** `TileDesc` (référentiel, code de Morton, classe de cadence `Maree | Debit | NoeudV | Immediat`, subdivision,
séquence propre à la tuile) ; `publier(…)` : les `(16·2^s)²` échantillons d'une tuile depuis un échantillonneur (profondeur, courant) de
l'appelant, la prévision par cellule (`t_next_cross` et sa cause, S570) quand une profondeur prévisible est fournie, la séquence
incrémentée, et **les événements de franchissement** — une cellule dont la classe de profondeur ou de danger a changé depuis la publication
précédente — dans un tampon de l'appelant (I-06).

**Références, calculées avant** (ce script les écrit). Une tuile de 16 × 16 cellules de 64 m sur une plage (fond de −2 à +2 m en `x`),
une marée de 0,4 m (M2), courant nul ; publiée à `t₀` = 0 puis `t₁ = T/12` (0 puis 0,2 m) : les colonnes [2, 3, 4, 6, 7] changent de classe →
**80 événements**. La prévision de la colonne 6 à `t₁` (fond -0.3750 m, 0.5750 m d'eau) : la marée ne monte pas jusqu'à
1,00 m ; elle repasse 0,50 m en descendant dans **16368.323 s**. Le code de Morton de la tuile (3, 5) : **39**. Séquences 1 puis 2.

**Quantum** (ADR-236 D1) : la bissection à 1 ms ; les classes, des entiers. **Critères, écrits avant.** (1) 80 événements
exactement, aux colonnes attendues, avec les classes d'avant et d'après ; (2) la prévision de la colonne 6 à 10 ms, seuil 0,50 m, en
descendant, cause `Maree` ; (3) Morton 39 ; séquences 1 puis 2 ; une subdivision 1 donne 32 × 32 échantillons ; (4) refus : tampons
trop courts, subdivision au-delà de 4.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [x] **P2** — les tuiles et leurs essais ; (1)–(4).
- [x] **P3** — preuve ; liste 7.7 ; rituel (`--lot`).

### Notes de reprise
- **P2 fini** — 80 événements aux colonnes 2, 3, 4, 6, 7 ; la colonne 6 annoncée à 16 368,323 s ; Morton 39 ; séquences ; 1 024
  échantillons en subdivision 1 ; refus. Suite 745.
- **P3** — preuve TRAVERSABILITE-TUILES-S572 ; liste 7.7 ; index ; journal ; le lot.

