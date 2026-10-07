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

Session : S583 — **terminée**. En autonomie, **3.6 — la réfraction bathymétrique des ondes** (absent) ; elle sert 3.4 (les rayons d'un
tsunami se courbent). La référence de B (S362) ne traite que des isobathes droites et parallèles ; ici, un fond quelconque.

**Ce que la session fait.** `refraction.rs` : le **tracé d'un rayon** d'onde longue (`c = √(g·h)`) sur un fond `h(x, y)` fourni avec son
gradient par l'appelant — `ẋ = c·cos θ`, `ẏ = c·sin θ`, `θ̇ = sin θ·∂c/∂x − cos θ·∂c/∂y` —, Runge-Kutta d'ordre 4 à pas de temps fixe
(f64, déterministe) ; le **coefficient de réfraction** `K_r = √(b₀/b)` par deux rayons voisins (l'écart mesuré perpendiculairement au
rayon). Ne fait pas : les caustiques (`b → 0`), la diffraction, la réfraction des ondes courtes (la dispersion : `c` dépend alors de
`k·h`), l'entrée dans W.

**Références, calculées avant** (ce script les écrit). Un fond `h = 4 000 − 0,0195·x` (m), un rayon lancé à 30° de la normale aux
isobathes : Snell, `sin θ/c` constant — à `x` = 190 km (`h` = 295 m), **θ = 7.804001°** ; l'instant d'arrivée, `∫ dx/(c·cos θ)`
par Simpson (10⁶ intervalles) : **1604.6227 s** ; l'ordonnée atteinte `∫ tan θ dx` : **72949.931 m** ; `K_r = √(cos θ₀/cos θ)` =
**0.934944**.

**Quantum** (ADR-236 D1) : f64 ; l'erreur du RK4 au pas d'une seconde (`c` ≈ 200 m/s, la courbure lente) — estimée sous 10⁻⁹ relatif ;
l'instant d'arrivée mesuré par interpolation linéaire entre deux pas. **Critères, écrits avant.** (1) `sin θ/c` constant le long du rayon à
10⁻⁹ relatif ; (2) l'instant où le rayon passe `x` = 190 km à 0,01 s de 1604.6227 s, l'ordonnée à 0,1 m de 72949.931 m ; (3) `K_r` à 10⁻⁴ de
0.934944 (deux rayons écartés de 100 m) ; (4) un fond uniforme : le rayon droit, `θ` constant au bit ; refus : profondeur non positive,
pas non positif, tampon trop court.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `refraction.rs` et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 3.6 ; rituel.

### Notes de reprise
- **Première mesure : (1) et (2) tenus, (3) manqué — `K_r` 0,981 pour 0,935.** Relu d'abord (ADR-239 D1) par un tracé indépendant en Python :
  il redonne 0,981 — le module est juste. **Le montage était faux** : le rayon voisin partait à 30° d'un point situé 50 m plus au large,
  donc avec un autre invariant de Snell (`sin θ/c` plus petit de 1,2·10⁻⁴) ; l'écart d'angle se cumule sur 190 km (10 m sur 115). La
  formule `K_r = √(cos θ₀/cos θ)` vaut pour deux rayons **de la même famille** (même invariant). Correction : le rayon voisin part avec
  l'angle que Snell lui donne à son abscisse, `sin θ_b = sin θ₀·c(x_b)/c(0)`. Critère inchangé.
- **P2 fini** — Snell 3,1·10⁻¹¹ ; arrivée 1 604,6229 s ; y 72 949,930 m ; `K_r` 0,934931 après correction du montage ; fond plat au bit ;
  refus. Pour la revue de S586 : deux trajectoires comparées doivent appartenir à la même famille (le même invariant). Suite 760.
- **P3** — preuve REFRACTION-S583 ; liste 3.6 (absent → partiel) et décompte ; index ; journal.

