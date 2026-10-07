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

Session : S632 — **en cours**. En autonomie (ADR-247 : la physique des partiels). **Le lot** (dû ; feuille de route S629–S631), puis **3.5 — la
hauteur réfractée par une côte courbe** (un manque de S630 : `transformer` suppose des isobathes parallèles).

**Ce que la session fait.** `portee::tracer_houle` (le rayon de houle dispersif de S603, rendu public) ; `deferlement::sur_rayons(a, b, b₀, …)`
— le long du rayon `a`, `H = H₀·K_s·K_r` (la levée de `bathymetrie`, la réfraction `√(b₀/b)` contre le rayon voisin `b` au même indice,
`refraction::coefficient`), le déferlement au premier passage de `H − 0,78·h` par zéro, interpolé. Ne fait pas : les caustiques (`b → 0`),
la diffraction, le chaînage de ces points en polyligne (S630 le ferait), le flux dissipé.

**Références, calculées avant** (`s632_ref.py`, numpy). Côte droite `h = 0,02·x`, houle de 8 s et 1,5 m, rayons partis de 6 km (120 m, en eau
profonde) à θ₀ = 0,3 rad, espacés de 10 m : l'analytique (Snell, levée, `K_r = √(cos θ₀/cos θ)`) **x_b = 112.599453093 m** ; le long des rayons, pas de
2, 1, ½ s : **112.618473302, 112.604772538, 112.600476235 m** — écarts 1.90e-02, 5.32e-03, 1.02e-03 m. **Bornes du montage**
(ADR-257 D1, assertées) : le départ en eau profonde (120 m ≥ λ), la coupure (0,5 m) sous la profondeur de déferlement. Une île conique
(rivage à 100 m, pente 0,02), des rayons en incidence normale partis de 5,2 km par paires miroir (y = ±200, ±210 m) : un cas où le gradient
du fond tourne.

**Quantum** : f64. **Critères, écrits avant.** (1) côte droite : les trois positions égales aux références à 10⁻⁹ m ; (2) l'écart à l'analytique
décroissant avec le pas, sous 2 mm à ½ s ; (3) l'île : les deux rayons de chaque paire déferlent, en points miroir (`y ↔ −y`) à 10⁻⁹ m ; (4)
refus : `b₀` non positif, un rayon vide.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [ ] **P2** — `tracer_houle`, `sur_rayons` et leurs essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 3.5 ; rituel (`--lot`).

### Notes de reprise
