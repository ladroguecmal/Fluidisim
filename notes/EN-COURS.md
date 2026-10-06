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

Session : S574 — **en cours**. En autonomie, **la glace** — 7.6 (absent) et 7.7 (la glace porteuse, SPEC-006 §5.1 `ice_h`,
`ice_capacity_kg`). SPEC-002 §4 ; ADR-027 §3 (la glace retenue, bornée aux lacs et baies abritées).

**Ce que la session fait.** `glace.rs` : la croissance de **Stefan**, `h = √(h₀² + 2·k·ΔT·t/(ρ·L))` (`k` = 2,2 W/m/K, `L` = 334 kJ/kg,
`ρ` = 917 kg/m³), et son écriture en degrés-jours ; la portance de **Gold**, `P = A·h²`, **A = 3,5 kg/cm²** (la valeur prudente de Gold,
charges mobiles) ; la fraction émergée (`1 − ρ_glace/ρ_eau`) ; la formation en plaque (`Hs < 0,15 m`). L'échantillon de traversabilité
reçoit `ice_h` et `ice_capacity_kg` (dérivé une fois, ici : SPEC-006 §5.1) ; un agent de masse donnée traverse si la charge admissible la
couvre ; l'échéance où la glace portera une charge se prédit par le franchissement de `h_min = √(m/A)`.

**Références, calculées avant** (ce script les écrit). Stefan : `h = 0.035231·√FDD` — à 10, 50, 100, 200 K·jour : **0.1114,
0.2491, 0.3523, 0.4982 m** (la table de SPEC-002 : 11, 25, 35, 50 cm). Gold : à 5, 10, 20, 30, 50 cm,
**87.5, 350, 1400, 3150, 8750 kg** — avec les masses de référence 100 kg (une
personne équipée), 400 kg (un groupe, une motoneige), 1 500 kg (une voiture légère), 5 000 kg (un camion léger), la table de SPEC-002
est reproduite ligne à ligne (le script l'a vérifié avant d'écrire). L'échéance : depuis 10 cm sous 10 K de gel, une voiture légère
(`h_min` = 0.20702 m) portée dans **228714.1 s** (2.647 jours). Émergé en eau douce : **8.3 %**.

**Quantum** (ADR-236 D1) : f64 pour la croissance (`h` à 10⁻⁹) ; la bissection à 1 ms sur des jours (10⁻⁸ relatif). **Critères, écrits
avant.** (1) Stefan à 10⁻⁶ m de ces valeurs ; (2) Gold à 10⁻⁶ relatif, et la table des charges ; (3) l'échéance à 1 s ; (4) l'émergé à
10⁻⁶ ; la plaque refusée à `Hs` = 0,15 m, formée à 0,149 ; (5) refus : épaisseur, gel ou temps négatifs.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `glace.rs`, l'échantillon ; (1)–(5).
- [ ] **P3** — preuve ; listes 7.6 et 7.7 ; rituel.

### Notes de reprise
