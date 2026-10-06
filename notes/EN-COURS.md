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

Session : S578 — **terminée**. En autonomie : **le lot** (dû ; feuille de route S575–S577), puis **2.2 — la carte cotidale** : la marée de
S577 vaut en un lieu ; amplitude et phase varient dans l'espace (la marée se propage).

**Ce que la session fait.** `CarteCotidale` : pour chaque composante, l'amplitude complexe `H = A·e^(−ig)` sur une grille régulière
(origine, pas, dimensions), **interpolée sous forme complexe** (bilinéaire sur `Re H` et `Im H`) — `η = Z₀ + Σ (Re Hₖ·cos ωₖt + Im Hₖ·sin ωₖt)`
(avec `H = A·e^(−ig)` : `A·cos(ωt − g)`), le cosinus et le sinus par `PhaseQ32` : tout est fait d'opérations IEEE de base et de nos
polynômes — déterministe, sans `atan2`, sans saut de phase à 2π. Hors de la grille : refus.

**Références, calculées avant** (ce script les écrit). Une onde M2 progressive dans un chenal de 20 m : `c = √(g·h)` = **14.0071 m/s**,
longueur d'onde **626.3 km** ; une carte de 10 km de pas (`A` = 1 m, `g = k·x`). Au milieu d'une maille, l'interpolation de la corde
creuse l'amplitude de `1 − cos(k·Δx/2)` = **1.258e-03** (la phase y reste exacte, par symétrie) ; aux nœuds, l'onde exacte. Le retard
de la pleine mer entre `x` = 0 et 50 km : **3569.6 s**.

**Quantum** (ADR-236 D1) : η en f32 (10⁻⁷ m) ; la dérive de phase de S577 (3·10⁻⁴ m en 15 jours) — on mesure sur 25 h (2·10⁻⁵ m).
**Critères, écrits avant.** (1) aux nœuds, η à 10⁻⁴ m de `cos(ωt − kx)` sur 25 h ; (2) au milieu d'une maille, l'amplitude (le maximum
sur 25 h, pas d'une minute) à 10⁻⁴ de `1 − 1.258e-03` ; (3) le retard de la pleine mer entre 0 et 50 km à 60 s de 3569.6 s (le pas
d'échantillonnage) ; (4) un point hors de la grille refusé ; au bit à `(x, t)` donnés.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [x] **P2** — la carte cotidale et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 2.2 ; rituel (`--lot`).

### Notes de reprise
- **P2 fini** — nœuds 1,98·10⁻⁵ m ; milieu 0,998739 ; retard 3 540 s ; refus, au bit. Le plan avait un signe faux dans sa formule (`+ Im H·sin`
  pour `H = A·e^(−ig)`) ; la forme voulue (`A·cos(ωt − g)`) implémentée. Suite 753.
- **P3** — preuve CARTE-COTIDALE-S578 ; liste 2.2 ; index ; journal ; le lot.

