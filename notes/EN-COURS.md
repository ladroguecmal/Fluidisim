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

Session : S557 — **en cours**. En autonomie : **le lot des registres** (dû ; feuille de route S552–S556), puis **A332 — l'énergie discrète
de δ linéaire** (liste 4.18, C09) : S555 a montré que l'énergie naturelle oscille ; laquelle le schéma conserve-t-il ?

**La dérivation, faite avant la mesure, depuis le code** (`step_surface_linear`). Le pas est un avant-arrière : `u^{n+1} = u^n − (dt/ρ)·G p`
avec `D u^{n+1} = 0` et la pression imposée au couvercle `p_c = ρg·(η^n − z₀)` (fantôme à deux fois la demi-maille) ; puis
`η^{n+1} = η^n + dt·w_c^{n+1}` (la somme des flux de colonne d'un champ sans divergence est la vitesse au couvercle, le fond étant clos).
Pour un champ sans divergence, `⟨v, G p⟩ = Σ p_c·w_c·dA` **si la face du couvercle pèse une demi-maille** dans le produit scalaire (son
gradient est pris sur `dx/2`). Alors, avec `K(u) = ½ρ·Σ ω_f·u_f²·dx³` (`ω` = ½ au couvercle, 1 ailleurs) :
`K^{n+2} − K^{n+1} = −½ρg·Σ (η^{n+1} − z₀)·(η^{n+2} − η^n)·dA`, d'où **l'invariant exact**
`Q = K(u^{n+1}) + ½ρg·Σ (η^n − z₀)·(η^{n+1} − z₀)·dA` — le schéma n'est pas dissipatif (sans éponge), et au départ (`u^0 = 0`)
`K(u^1) = −½ρg·dt·Σ (η^0 − z₀)·w_c^1·dA` donne **`Q = E₀` exactement**. L'énergie naturelle `K^{n+1} + ½ρg·Σ (η^{n+1} − z₀)²·dA` vaut
`Q + ½ρg·dt·Σ (η^{n+1} − z₀)·w_c^{n+1}·dA` : elle oscille. La formule s'éprouve par un calcul indépendant (ADR-239 D1) : l'oscillateur
`p_{n+1} = p_n − h·q_n`, `q_{n+1} = q_n + h·p_{n+1}` conserve `p_{n+1}² + q_n·q_{n+1}` (développé à la main : les deux pas consécutifs
donnent `p_{n+1}² + q_{n+1}² − h·p_{n+1}·q_{n+1}`) ; l'essai le vérifie aussi en f64 sur l'oscillateur.

**Ordre de grandeur et quantum** (ADR-236 D1). `E₀` = 1,573 J (S555). Le plancher : l'ulp de `η` en f32 au voisinage de z₀ = 1,5 m,
1,2·10⁻⁷ m, sur une bosse de 2 cm → `ρg·0,02·1,2·10⁻⁷·dA` = 1,5·10⁻⁶ J par colonne, ≈ 10⁻⁶ de E₀ ; la projection itère jusqu'au
plancher (résidu relatif 10⁻⁶).

**Critères, écrits avant.** (1) `|Q_n/E₀ − 1|` < 10⁻⁴ à chaque pas des 12 000 de S555 (rapport au plancher : 100). (2) La hausse de `Q`
d'un pas à l'autre, au pire, sous 10⁻⁵ de E₀ (10 fois le plancher) — C09, `dE/dt ≤ 0`, sur l'énergie du schéma au plancher près.
(3) L'oscillateur f64 : `p_{n+1}² + q_n·q_{n+1}` constant à 10⁻¹² sur 10⁵ pas. L'énergie naturelle, avec et sans le demi-poids du couvercle,
publiée, sans critère.

### Plan

- [x] **P1** — jeton ; le lot ; la dérivation ; plan.
- [ ] **P2** — l'essai ; (1)–(3).
- [ ] **P3** — preuve ; A332 ; liste 4.18, 13.2 ; C09 ; rituel (`--lot`).

### Notes de reprise
