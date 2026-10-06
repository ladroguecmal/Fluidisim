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

Session : S580 — **terminée**. En autonomie, **2.2 — le courant de marée** (S579 : « manque le courant de marée ») ; il sert aussi 7.7
(SPEC-006 §5.5 : `flow_speed` est le courant de B, sans l'orbitale).

**Ce que la session fait.** La quantité de mouvement linéaire, sans frottement ni Coriolis : `∂u/∂t = −g·∇η`. Pour chaque composante
`η = Re(H·e^(iωt))`, `u = Re(i·g·∇H·e^(iωt)/ω)` = `−(g/ω)·(∇Re H·sin ωt + ∇Im H·cos ωt)` — **sans la profondeur** : la carte cotidale porte
déjà la propagation. `CarteCotidale::courant(x, y, t)` (le gradient de l'interpolation bilinéaire) ; `maree::avec_courant(échantillon, u)` :
le courant horizontal ajouté à `u_total`. Ne fait pas : le frottement sur le fond, Coriolis (`f` ≈ 10⁻⁴ s⁻¹, comparable à ω de M2 : à
reprendre avant les grandes baies), le courant dans les zones peu profondes (non linéaire).

**Références, calculées avant** (ce script les écrit). L'onde progressive de S578 (chenal de 20 m, `c` = 14.0071 m/s, `A` = 1 m) : `u =
(g·k/ω)·η` = **0.700356 m/s** d'amplitude (`g/c` = 0.700357). La pente de la corde au milieu d'une maille de 10 km :
`sinc(k·Δx/2)` = **0.999580720**, soit **0.700062 m/s**, la phase exacte — le courant y est maximal en même temps que η.

**Quantum** (ADR-236 D1) : f32 (10⁻⁷ relatif) ; la dérive de phase de S577 (2·10⁻⁵ sur 25 h). **Critères, écrits avant.** (1) au milieu
d'une maille, le maximum de `u` sur 25 h (pas d'une minute) à 10⁻⁴ m/s de 0.700062 m/s, et `v` nul à 10⁻⁶ m/s ; (2) au même point, le
pic du courant et celui du niveau au même instant à 60 s près ; (3) `avec_courant` ne change que `u_total[0..2]` (les autres champs au bit) ;
(4) un point hors de la grille refusé.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le courant et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 2.2 ; rituel.

### Notes de reprise
- **P2 fini** — u max 0,700060 m/s (0,700062) ; v nul ; pics à 3 900 s tous deux ; `avec_courant` ; refus. Suite 756.
- **P3** — preuve COURANT-MAREE-S580 ; liste 2.2 ; index ; journal.

