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

Session : S444 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »*, puis, à la question de la voie de C7d-3c,
*« (B) B dans la bande »* — [ADR-214](../docs/adr/ADR-214-b-entre-dans-la-bande.md) (remplace D1 de S433).

**Ce que la session fait (c1, en référence).** (1) `LinearSwell` complet : l'élévation `η_B(x, t)`, le gradient exact de la vitesse,
la pression dynamique `p_dyn` (Airy en profondeur infinie : `p = −ρg·ζ + ρg·a·e^{kζ}·cos θ`). (2) `Apic3` en mode relatif
(`set_relative_background`), sans zone de colonnes : particules en `u′`, déplacées par `U + u′` ; `−dt·u′·∇U` sur la grille ; pas de
gravité en volume ; la valeur de pression aux mailles d'air voisines de l'eau, `p′ = ρg(z_s − niveau) − p_dyn(z_s)` moins l'erreur de B
à sa propre surface — dans le second membre et la correction. Sans fond : `Apic3` au bit.

**Critères, écrits avant.** (1) `LinearSwell` : gradient contre différences finies (écart relatif ≤ 10⁻³), divergence et rotationnel
nuls (≤ 10⁻⁴ de `|∇U|`), `p_dyn` cohérente avec `∂U/∂t = −∇p_dyn/ρ` à l'ordre linéaire (≤ 10⁻³). (2) Sans fond, `Apic3` au bit (la
suite et un essai). (3) **Sous B seul**, une nappe de particules au repos relatif (`u′` = 0), houle de 5 cm, 4 m, 25 cm : `|u′|` maximal
sur 5 s sous 2 % de `aω` — le reste vient de ce que les particules suivent `U` et non la surface linéaire exacte (ordre `ak`). (4) La
même nappe **sans** le mode relatif mais initialisée à `U` (l'eau totale) sert de témoin : écart des surfaces au plus 5 mm sur 5 s.
Suite du cœur, zéro avertissement.

### Plan

- [x] **P1** — ADR-214 ; jeton, plan seul.
- [x] **P2** — `LinearSwell` complet ; essais.
- [ ] **P3** — `Apic3` relatif, sans zone ; essais ; mesures (3), (4).
- [ ] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `LinearSwell::elevation`, `velocity_gradient` (exact), `dynamic_pressure` ; essai `linear_swell_gradient_and_pressure_s444` :
  gradient 8,1·10⁻⁵ des différences finies, divergence et rotationnel nuls, quantité de mouvement 7,6·10⁻⁴ — (1) tenu.

