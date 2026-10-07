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

Session : S587 — **en cours**. En autonomie (ADR-247 : la physique d'abord, les absents) : **le lot** (dû ; feuille de route S584–S586),
puis **3.3 — les explosions de surface et sous-marines** (absent ; ADR-001 : les ondes d'explosion sont de W). Première pièce : **la bulle
d'une explosion sous-marine**.

**Ce que la session fait.** `explosion.rs` : l'énergie de la bulle (une fraction de l'énergie de la charge — un paramètre d'auteur, 0,4
pour le TNT à 4,184 MJ/kg), le rayon maximal à l'équilibre d'énergie `E_b = (4/3)·π·R³·p` (`p = p_atm + ρ·g·d`), la période du premier
battement `T = 2·t_c`, `t_c = 0,914681·R·√(ρ/p)` (l'effondrement de Rayleigh d'une cavité vide). Les lois d'échelle en découlent :
`R ∝ (W/p)^(1/3)`, `T ∝ W^(1/3)·p^(−5/6)` (la forme de Willis). Ne fait pas : les battements suivants (les pertes), la migration de la
bulle vers la surface, l'onde de choc, les ondes de surface et la gerbe (la suite de 3.3), l'entrée dans W.

**Références, calculées avant** (ce script les écrit). La constante de Rayleigh par une intégration RK4 indépendante de `R·R̈ + 1,5·Ṙ² =
−Δp/ρ` : **0.914780** (pas 10⁻⁵) et **0.914730** (pas 5·10⁻⁶) ; la forme fermée `√(3π/2)·Γ(5/6)/Γ(1/3)` = **0.914681**. Une charge
de 1 kg à 20 m : **`R_max` = 1.09727 m, `T` = 0.116859 s** ; 8 kg : `T` ×**2.000000** ; à 60 m : `T` ×**0.494176** (= `(p₆₀/p₂₀)^(−5/6)`
= 0.494176).

**Quantum** (ADR-236 D1) : f64 ; la constante gardée à 10⁻⁶ (sa forme fermée). **Critères, écrits avant.** (1) `R_max` et `T` à 10⁻⁹
relatif de ces valeurs ; (2) les lois d'échelle à 10⁻¹² relatif (indépendantes de la constante) ; (3) refus : masse, profondeur ou
fraction non positives.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [ ] **P2** — `explosion.rs` et ses essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 3.3 ; rituel (`--lot`).

### Notes de reprise
