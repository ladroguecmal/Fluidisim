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

Session : S605 — **en cours**. En autonomie (ADR-247) : **12.4 — l'eau en amont du terrain, le géoïde dans l'outil de terrain**
(SPEC-005 §3–4 ; absent). Le « zéro » d'une scène est une distance au centre de la planète ; le squelette hydrographique est une entrée du
terrain, jamais une sortie.

**Ce que la session fait.** Un module `geoide.rs` : `Geoide { rayon_m }` (le niveau moyen sphérique) ; `altitude(local)` — l'altitude
au-dessus du niveau moyen d'un point du plan tangent d'une ancre ; `z_local(x, y, altitude)` — l'inverse ; `ecart_plan_tangent(d)` —
`R·(1 − cos(d/R))`, la table de SPEC-005 §4 ; `Grille` (le terrain de l'outil, en `z` du plan tangent) et `conformer(grille, géoïde,
biefs)` — **l'étape 2 de l'ordre imposé** : le terrain gravé pour satisfaire le squelette (les biefs de S604, leurs lignes d'eau en
altitude), le squelette jamais modifié ; chaque cellule à moins d'une demi-largeur d'un segment descend au fond `z_eau − h` (Manning), placé
par le géoïde. Ne fait pas : l'anomalie régionale et la marée du niveau moyen (ADR-002 §2.4), le trait de côte et la bathymétrie du
squelette, les dérivations (étape 3), les ancres multiples.

**Références, calculées avant** (ce script, en décimal à 50 chiffres). La table, `R` = 6 371 km : 1 km : 0.078480615 m ; 3 km : 0.706325525 m ; 10 km : 7.848059918 m ; 30 km : 70.632423247 m (SPEC-005 écrit 70,7 m à 30 km : un
arrondi — 70,63). Le point (30 km, 0, 0) du plan tangent est à **70.632162227 m** au-dessus du niveau moyen. La gravure : un bief le long de
`y` en `x` = 30 km (2 km, ligne d'eau 12,0 → 11,0 m, 20 m, 30 m³/s, n = 0,035 : `h` = 1.781932256 m), une grille de 10 m (20 × 200 cellules),
le terrain à 15 m d'altitude : **400 cellules** dans le couloir, le plus grand creusement **5.779432256 m** ; dans un outil à
plan tangent, le même fond serait à **70.655602 m** au-dessus de sa place.

**Quantum** : f64 au rayon de la planète (l'ulp de 6,4·10⁶ m : 9,3·10⁻¹⁰ m) ; la tolérance des allers-retours **10⁻⁸ m** (rapport 10,7).
**Critères, écrits avant.** (1) la table à 10⁻⁹ relatif ; l'aller-retour altitude ↔ `z` à 10⁻⁸ m sur des points jusqu'à 50 km et ±100 m ;
(2) l'altitude du point (30 km, 0, 0) à 10⁻⁸ m ; (3) la gravure : 400 cellules gravées, chacune à l'altitude de son fond à 10⁻⁸ m,
le plus grand creusement à 10⁻⁸ m, les autres cellules inchangées au bit, le squelette inchangé ; (4) l'outil à plan tangent : l'écart
du fond au centre du couloir à 10⁻⁶ m ; (5) refus : rayon non positif, grille vide ou pas non positif, un bief qui ne descend pas.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `geoide.rs` et ses essais ; (1)–(5).
- [ ] **P3** — preuve ; liste 12.4 ; rituel.

### Notes de reprise
