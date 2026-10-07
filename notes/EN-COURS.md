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

Session : S589 — **terminée**. En autonomie (ADR-247), **3.9 — les couches fournies au-dessus du plan moyen pour δ** (absent ; A286) :
le pas couplé mobile exige un champ prolongé au-dessus de `z = 0` **de façon incompressible** ; B refuse `z > 0` (ADR-113) ; Taylor d'ordre
un n'est pas incompressible (`div = z·U_xz`), l'exponentielle `e^{kz}` amplifie les ondes courtes sous les crêtes des longues.

**Ce que la session fait.** Le remède qu'A286 propose : `Background::vitesse_au_dessus(x, y, z, t)` — la vitesse horizontale **constante**
au-dessus de `z = 0` (`U(z) = U(0)`), la verticale **fermée par la continuité** : `w(z) = w(0) − z·∇ₕ·U(0)` ; incompressible par
construction, linéaire en `z` (pour un mode d'Airy, `w = −a·ω·cos φ·(1 + kz)` : l'ordre un de l'exponentielle). Mêmes phases entières que
`eval_local`. Ne fait pas : W (les anneaux) au-dessus du plan, le raccord au pas couplé de δ (la réception contre l'oracle S253).

**Références, calculées avant** (ce script les écrit). Un mode de 1 m, λ = 50 m (`k` = 0.125664 rad/m, `ω` = 1.110298 rad/s), à `z` = 0,5 m :
le témoin Taylor a une divergence de **8.7666e-03 s⁻¹** au plus ; l'écart à l'exponentielle sur `w`, `a·ω·(e^{kz} − 1 − kz)`, au plus
**2.2383e-03 m/s**. Le bruit d'une divergence par différences finies centrées en f32 au pas de 1 cm : ≈ **1.3e-05 s⁻¹**.

**Quantum** (ADR-236 D1) : ce bruit. **Critères, écrits avant.** (1) la divergence du prolongement, par différences finies, sous 10⁻³ de
celle du témoin (rapport au bruit : 1) en 16 points ; (2) `w` égal à `−a·ω·cos φ·(1 + kz)` à 10⁻⁵ m/s, et
son écart à l'exponentielle sous 2.2383e-03 m/s ; (3) à `z = 0`, `(u, v, w)` identiques **au bit** à la vitesse de surface
d'`eval_local` (la continuité) ; (4) refus : `z < 0` ou non fini.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `vitesse_au_dessus` et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 3.9 ; A286 ; rituel.

### Notes de reprise
- **Avant la mesure, une faute du plan relevée** (ADR-236 D1) : le script avait imprimé, pour le critère (1), un rapport au bruit de « 1 »
  (des différences finies f32 au pas de 1 cm : bruit ≈ 1,3·10⁻⁵ s⁻¹ contre un seuil de 8.8e-06) — le seuil, ainsi appliqué, est
  disqualifié. Procédure corrigée, seuil inchangé : différences centrées au pas de 0,5 m et 0,25 m, combinées par Richardson (troncature
  en `h⁴` ≈ 7.2e-08, bruit ≈ 2.4e-07 s⁻¹ ; rapport au seuil ≈ 36). Le témoin Taylor passe par la même
  procédure.
- **P2 fini** — divergence 9,9·10⁻⁷ (témoin 1,3·10⁻²) ; w à 5,8·10⁻⁸ ; l'exponentielle à sa borne ; au bit à z = 0 ; refus. Suite 764.
- **P3** — preuve AU-DESSUS-S589 ; liste 3.9 (absent → partiel) et décompte ; note A286 ; index ; journal.

