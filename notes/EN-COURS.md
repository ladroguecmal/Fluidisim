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

Session : S694 — **en cours**. En autonomie, sans arrêt (l'utilisateur, 2026-10-08). S693 : Saint-Venant, sans dispersion, raidit l'onde
du large et fait se retourner la vague trop tôt. **Le porteur dispersif**, Serre–Green–Naghdi (SGN), d'abord en 1D sur fond plat ; il
servira aussi de référence à A234.

**Le schéma** (`serre_1d.rs`, catégorie P) :

- un pas de Saint-Venant (volumes finis, MUSCL, Rusanov, Heun) ;
- plus la correction dispersive de Bonneton et al. (2011). On résout, à chaque étage, `h·A − ⅓(h³·A_x)_x = −⅓(h³(2u_x² + g·h_xx))_x`
  (tridiagonal), puis `(hu)_t` reçoit `h·A`.

Linéarisé, le schéma rend `ω² = g·d·k²/(1 + (kd)²/3)`, la dispersion de Serre (calculé à la main au plan).

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-274)

- **témoin** : le même schéma sans le terme dispersif (Saint-Venant), sur la même onde : le front se raidit.
- **instrument** : l'onde solitaire exacte de SGN, `η = a·sech²(κ(x − ct))`, `c = √(g(d+a))`, `κ = √(3a)/(2d√(d+a))`, sur un domaine
  périodique, après 40 `d`. Ce que rendrait chaque hypothèse :
  - si les équations sont justes, la forme est gardée et la célérité est celle de la formule, l'écart convergeant à l'ordre deux ;
  - si un signe ou un facteur est faux dans le terme non linéaire, l'onde se déforme ou change de vitesse, sans converger vers
    l'exacte ;
  - le témoin se raidit.
- **calcul** (ce script) :
  - `a/d` = 0,1 et 0,3 ; `c` = 3,285 et 3,571 m/s ; la largeur `1/κ` = 3,83 et 2,08 m ;
  - mailles `d/20` et `d/40`, Courant 0,4 ;
  - la borne de forme : **2 %** de `a`, sous la condition d'ordre (÷ 3 au moins d'une maille à l'autre) ;
  - la célérité à **0,2 %**. Le plancher : la lecture de la crête, interpolée par une parabole (sans quantum de maille).
- **ADR** : ADR-271, ADR-273 D1, ADR-274 D1 (le coût mesuré : quelques secondes).
- **pièges** :
  - la formule de l'onde solitaire, de mémoire. C'est l'essai qui la confirme : un état initial faux ne serait pas stationnaire, et le
    témoin le distinguerait ;
  - le périodique dans le système tridiagonal (Sherman–Morrison, comme S665) ;
  - `u = hu/h` dans les termes dispersifs.

**Critères, écrits avant.**

1. À `a/d` = 0,1 et 0,3, sur 40 `d` : l'écart de forme au plus **2 %** de `a` à `d/40`, divisé par 3 au moins depuis `d/20`.
2. La célérité de la crête à **0,2 %** de `√(g(d+a))`.
3. Le témoin (sans le terme) s'écarte de plus de 10 % de `a`.
4. La masse au bit (le schéma est conservatif en `h`).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `serre_1d.rs` ; l'essai ; (1)–(4).
- [ ] **P3** — preuve ; rituel.

### Notes de reprise

- **La réduction linéaire, à la main.** Serre : `u_t + uu_x + gh_x = (1/(3h))(h³(u_xt + uu_xx − u_x²))_x`. Soit `A = u_t + uu_x + gh_x`.
  - On a `u_xt + uu_xx − u_x² = A_x − 2u_x² − gh_xx`, d'où `hA − ⅓(h³A_x)_x = −⅓(h³(2u_x² + gh_xx))_x`.
  - Linéarisé, en Fourier : `A(1 + (kd)²/3) = (i/3)·g·d²·k³·η`, d'où `u_t = −igkη/(1 + (kd)²/3)` et `ω² = gdk²/(1 + (kd)²/3)`.
