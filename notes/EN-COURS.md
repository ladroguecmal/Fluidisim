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

Session : S502 — **terminée**. En autonomie, **6.1, l'amortissement des autres degrés de liberté** : S336 a mesuré par δ la masse ajoutée
et l'amortissement de rayonnement de la coque de la porte D **en pilonnement** seulement ; elle roule et tangue sans perte.

**Ce que la session fait.** (a) δ rend le **moment** de sa pression sur la paroi (`solid_wall_force` → force et moment, la force au bit) ;
(b) `rayonnement_coque` impose un roulis puis un tangage `θ = Θ·sin ωt` et ajuste le moment `M = −A·θ̈ − B·θ̇` ; (c) le corps reçoit un
amortissement angulaire (constante d'archétype), relatif à la rotation de la surface qui le porte ; un essai de lâcher en eau calme.

**Ordre de grandeur, écrit avant.** La coque 4 × 1,6 × 1 m, 3 200 kg, tirant 0,488 m : roulis `GM` = 0,18 m, `C₄₄` ≈ 5,7 kN·m/rad,
`I₄₄` ≈ 950 kg·m², `ω` ≈ 2,45 rad/s ; tangage `GM` = 2,48 m, `C₅₅` ≈ 78 kN·m/rad, `I₅₅` ≈ 4 530 kg·m², `ω` ≈ 4,2 rad/s. Le roulis d'une
barge rayonne peu (la coque déplace peu d'eau en tournant autour de son axe long) : `ζ₄₄` attendu de l'ordre de 0,01 à 0,05 ; le tangage
pousse l'eau comme le pilonnement aux extrémités : `ζ₅₅` de l'ordre de celui du pilonnement (0,16).

**Critères, écrits avant.** (1) la force de δ inchangée au bit (les essais de S330–S336) ; le moment d'une pression hydrostatique sur un
pavé incliné contre l'analytique `ρgV·(KB − …)` à 1 % (cas de réponse connue) ; (2) `A` et `B` en roulis et tangage à trois pulsations
autour de leur `ω` propre, résidu d'ajustement ≤ 10 %, `B` > 0 ; (3) le corps amorti : lâché à 0,1 rad en eau calme, période et décrément
à ±2 % de l'oscillateur `(I + A)·θ̈ + B·θ̇ + C·θ = 0` ; sans amortissement, ses crêtes ne décroissent pas.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le moment de δ ; son essai hydrostatique.
- [x] **P3** — roulis et tangage imposés (calcul détaché) ; `A`, `B`.
- [x] **P4** — l'amortissement angulaire du corps ; l'essai de lâcher ; preuve ; liste 6.1 ; rituel.

### Notes de reprise
- **P2** — `solid_wall_load` / `Volume3::solid_load` ; pavé incliné noyé : moment contre `r_c × F` à 7,4·10⁻⁴, 1,5·10⁻⁴, 9,9·10⁻⁶ (n = 24, 48, 96), force au bit. (Critère 1 réécrit avant mesure : `r_c × F`, plus simple que `ρgV·KB`.)
- **P3** — roulis A₄₄ 214 / 209 / 204 kg·m², B₄₄ 71 / 99 / 130 (ω 2 / 2,5 / 3), résidus 8,4 / **15,8** / 6,0 % — manqué à 2,5 : dix sauts
  jusqu'à 15 N·m (moment de 69 N·m) aux franchissements de faces, comme S336 ; tangage A₅₅ 2 941 / 2 745 / 2 674, B₅₅ 4 780 / 4 738 / 4 353
  (ω 3,5 / 4 / 4,5), résidus 8,6–9,7 %. En plus (hors critères) : cavalement A 615 / 461 / 320 kg, B 450 / 924 / 1 668 (2 / 3 / 4 rad/s,
  ≤ 9,6 %) ; embardée 2 477 / 1 939 / 923 kg, 1 692 / 4 958 / 6 761 (16–17 % à 2 et 3) ; lacet 1 809 / 2 228 / 1 534 kg·m², 264 / 1 933 /
  7 001 (71 / 28 / 7,7 % : les ondes longues reviennent des murs du domaine de 16 m).
- **P4 (en cours)** — `added_inertia`, `radiation_damping_angular` (relatif à la rotation de la surface). Lâchers : roulis période 0,43 %,
  décrément 0,9 % ; tangage 0,07 / 0,03 % ; sans amortissement crêtes constantes ; glissades à 0,1 %. 663 essais.
- **P4** — preuve RAYONNEMENT-6DDL-S502 ; **6.1 validée** (7 / 120) ; index ; journal.
