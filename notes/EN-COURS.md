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

Session : S634 — **en cours**. En autonomie (ADR-247 : la physique des partiels). **2.2 — le frottement et Coriolis dans le courant de
marée** (un manque de S580 : `∂u/∂t = −g·∇η` seul).

**Ce que la session fait.** `CarteCotidale::courant_amorti(x, y, t, g, f, r)` : pour chaque composante, la solution harmonique établie de
`∂u/∂t + r·u − f·v = −g·∂η/∂x`, `∂v/∂t + r·v + f·u = −g·∂η/∂y` — avec `a = iω + r`, `U = −g(a·Gx + f·Gy)/(a² + f²)`, `V = −g(a·Gy −
f·Gx)/(a² + f²)`, `G` le gradient complexe de l'interpolation de S580 ; `u = Re U·cos ωt − Im U·sin ωt`. En f32, des opérations de base :
déterministe. Ne fait pas : le frottement quadratique (`r` linéarisé), le transitoire (la solution établie seulement), `f` variable.

**Références, calculées avant** (ce script). Une carte M2 (onde progressive selon `x`, 400 km, 1 m), `f` = `r` = 10⁻⁴ s⁻¹. L'atténuation par
le frottement seul, `ω/√(ω² + r²)` = **0.814748670** ; le rapport des axes de l'ellipse par Coriolis seul (onde selon `x`), `f/ω` =
**0.711648028**. **Bornes du montage** (ADR-257 D1, assertées) : le transitoire décroît en `e^(−r·t)`, `e^(−86)` après les 10 jours
d'intégration ; `f < ω` (pas de résonance inertielle).

**Quantum** : f32 (10⁻⁷ relatif). **Critères, écrits avant.** (1) `f` = `r` = 0 : égal à `courant` (S580) à 10⁻⁶ m/s ; (2) contre une
intégration RK4 indépendante de l'équation du mouvement (pas de 60 s, 10 jours, l'écart `w = u − u₀` au courant sans frottement de S580
intégré), à 10⁻⁶ m/s sur la dernière période ; (3) l'atténuation, rapport des maxima sur une période échantillonnée en 1 000 points, à 10⁻⁴
de 0.814749 ; (4) l'ellipse, rapport des maxima de `v` et `u`, à 10⁻⁴ de 0.711648 ; (5) refus : `r` négatif, `g` non positif.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `courant_amorti` et ses essais ; (1)–(5).
- [ ] **P3** — preuve ; liste 2.2 ; rituel.

### Notes de reprise
