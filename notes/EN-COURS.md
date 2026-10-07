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

Session : S588 — **terminée**. En autonomie (ADR-247 : la physique d'abord), **3.5 — le déferlement** (absent ; « polyligne de SPEC-006
§6 »). Une donnée **cuite** : dérivée hors ligne de la bathymétrie et de l'état de mer, republiée par phase de marée.

**Ce que la session fait.** `deferlement.rs` : sur une grille de profondeurs (la côte orientée le long de `y`, le large vers `+x`), la
hauteur de la houle en chaque nœud — levée et réfraction de la référence de B (`bathymetrie::transformer`, S362) —, l'écart
`H − 0,78·h` (McCowan), et, ligne par ligne depuis le large, le premier passage par zéro, interpolé linéairement entre deux nœuds : un
sommet de la polyligne. Chaque sommet porte le **flux d'énergie dissipé** `ρ·g·H²/8·c_g` (kW/m, SPEC-006 §6) et la **direction de crête**.
Ne fait pas : une côte quelconque (les marching squares et le chaînage des segments), la largeur de la zone de déferlement, plusieurs
phases de marée, la publication (le chemin poussé).

**Références, calculées avant par ce script, avec ses propres formules** (dispersion par Newton, `K_s = √(c_g0/c_g)`, Snell,
`K_r = √(cos θ₀/cos θ)`) : une plage `h = 0,02·x`, une houle de 2 m, 8 s, à 20° : **`h_b` = 2.841963 m**, **`x_b` = 142.0982 m**, `H_b` =
2.216731 m, la crête à **8.0632°**, le flux dissipé **29.799753 kW/m**. Sur la grille de 5 m, l'interpolation linéaire place
le croisement à **142.1033 m** (+0.0052 m de la racine : la courbure de l'écart entre deux nœuds).

**Quantum** (ADR-236 D1) : l'interpolation, 5.2 mm ; f64 ailleurs. **Critères, écrits avant.** (1) chaque sommet de la
polyligne (une par ligne de la grille) à 1 mm de 142.1033 m (l'algorithme) et à 0.010 m de la racine ; (2) le
flux à 10⁻³ relatif de 29.799753 kW/m, la direction à 0,01° ; (3) une ligne sans déferlement (une houle trop petite pour la plus grande
profondeur de la grille… ou une ligne toute à terre) : aucun sommet ; (4) refus : grille de moins de 2 × 2, tampon trop court.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `deferlement.rs` et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 3.5 ; rituel.

### Notes de reprise
- **P2 fini** — 8 sommets à 142,1033 m ; flux 29,799763 kW/m ; crête 8,0634° ; une houle de 1 cm sans sommet ; refus. Suite 763.
- **P3** — preuve DEFERLEMENT-S588 ; liste 3.5 (absent → partiel) et décompte ; index ; journal.

