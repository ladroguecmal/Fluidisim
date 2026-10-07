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

Session : S582 — **en cours**. En autonomie, **3.4 — les tsunamis** (absent ; « propagation macroscopique, puis raffinement à la côte »).
ADR-001 §3.1 : un tsunami est un objet de W — dérivé d'un événement horodaté, déterministe —, pas un très grand domaine δ.

**Ce que la session fait.** `tsunami.rs` : un **profil de profondeur le long d'un rayon** (des sommets `(s, h)`, linéaire entre eux) ; le
**temps de parcours** `τ(s) = ∫ ds/√(g·h)`, exact par segment (`2L/(√g·(√h_a + √h_b))`) ; la **levée de Green** `A(s) = A₀·(h₀/h)^(1/4)`
(le flux d'énergie `A²·√h` conservé, sans étalement latéral) ; **le niveau** `η(s, t) = A(s)·f((t − t₀ − τ(s))/T)`, `f(u) = (1 − u²)²` pour
`|u| < 1` — un polynôme : aucune transcendante, le niveau est le même sur toute plateforme (I-03 ; la racine carrée est exacte en IEEE).
Ne fait pas : la dispersion (une onde longue `kh ≪ 1` n'en a guère au large), l'étalement d'une source ponctuelle, le déferlement et le
raffinement à la côte (la suite de 3.4), l'entrée dans B/W (un événement).

**Références, calculées avant** (ce script les écrit). 1 000 km sur 4 000 m : **5048.188 s** ; puis une pente de 4 000 à 10 m sur
100 km : **961.560 s** (forme fermée ; Simpson indépendant, 10⁶ intervalles : 961.560 s, écart 0.0e+00) — à la
côte en **6009.747 s** ; Green à 10 m : **×4.472136**.

**Quantum** (ADR-236 D1) : f64 (10⁻¹² relatif) pour `τ` ; `η` en f32. **Critères, écrits avant.** (1) `τ` à 10⁻⁶ s des trois valeurs ;
(2) Green à 10⁻⁹ relatif, et `A²·√h` constant à 10⁻¹² relatif en cinq points du profil ; (3) le pic de `η` en un point de la pente arrive à
`t₀ + τ(s)` à 1 s près (échantillonné à la seconde), d'amplitude `A(s)` à 10⁻⁶ relatif ; avant `t₀ + τ − T`, `η` = 0 exactement ; (4) au
bit, deux évaluations ; refus : moins de deux sommets, `s` non croissant, une profondeur non positive, un point hors du profil.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `tsunami.rs` et ses essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 3.4 ; rituel.

### Notes de reprise
- **Avant la mesure, une faute du plan relevée** (ADR-236 D1) : l'amplitude du pic **échantillonné à la seconde** porte un quantum de
  `2·(0,5/600)²` = 1.4e-06 relatif (l'instant du pic tombe jusqu'à 0,5 s d'une seconde entière, `T` = 600 s), au-dessus du seuil de 10⁻⁶ :
  ce seuil, ainsi appliqué, est disqualifié. L'amplitude se mesure donc **à l'instant exact `t₀ + τ(s)`** (le quantum devient l'arrondi
  f32, 6·10⁻⁸), seuil inchangé ; l'instant du pic reste mesuré à la seconde (à 1 s près).

