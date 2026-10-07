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

Session : S584 — **terminée**. En autonomie : **le lot** (dû ; feuille de route S582–S583), puis **3.4 — le tsunami sur un rayon courbe** :
S582 le propage sur un rayon droit, S583 trace des rayons qui se courbent ; les brancher.

**Ce que la session fait.** `tsunami::sur_rayon(a0, h0, point, voisin, b0, profondeur)` : l'instant d'arrivée est l'instant du point du
rayon tracé ; l'amplitude, `A₀·(h₀/h)^(1/4)·K_r` — Green et la réfraction ensemble (le flux d'énergie `A²·√h·b` conservé dans le tube de
rayons).

**Références, calculées avant** (ce script les écrit). Le fond de S583, un tsunami de 0,5 m lancé à 30° : à 190 km (`h` = 295 m),
l'arrivée à **1 604,6227 s** (Simpson, S583) ; l'amplitude `0,5·(4000/295)^(1/4)·0,934944` = **0.897047 m** (Green seul : ×1.918932).

**Quantum** (ADR-236 D1) : f64 ; `K_r` mesuré à 10⁻⁵ (S583). **Critères, écrits avant.** (1) l'arrivée à 0,01 s ; (2) l'amplitude à 10⁻⁴
relatif ; (3) le flux `A²·√h·b` constant à 10⁻⁴ relatif en cinq points du rayon ; (4) refus : une profondeur non positive, un écart nul.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [x] **P2** — `sur_rayon` et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 3.4 ; rituel (`--lot`).

### Notes de reprise
- **P2 fini** — arrivée 1 604,6229 s ; amplitude 0,897333 pour 0,897340 ; flux constant (vrai par construction — noté dans la preuve) ;
  refus. Suite 761.
- **P3** — preuve TSUNAMI-RAYON-COURBE-S584 ; liste 3.4 ; index ; journal ; le lot.

