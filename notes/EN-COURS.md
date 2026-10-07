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

Session : S620 — **en cours**. En autonomie (ADR-247 : la physique des partiels). **Le lot** (dû ; feuille de route S617–S619), puis
**l'ordre deux de `SaintVenant2D`** — un manque de 4.14 (la plage) et de 11.3 (la remontée) : l'ordre un est diffusif (un quart d'écart à
Thacker après une période à 100²).

**Ce que la session fait.** `SaintVenant2D::regler_ordre_deux(ε)` : reconstruction MUSCL (minmod) de `h`, `η = h + z`, `u`, `v` par
direction, pente nulle aux mailles de bord ; **reconstruction hydrostatique d'ordre deux** d'Audusse (2004) — les états de face reconstruits,
le fond de face `η − h`, et le **terme source centré** `−g·h·Δz` qui garde le lac au repos ; Heun en temps ; vitesse de Kurganov–Petrova à
`ε` = (0,1 mm)⁴ — l'`ε` de S613, (1 mm)⁴, amortissait les couches minces du rivage et figeait l'écart à 0,030 (mesuré au plan). L'ordre un
et ses références ne changent pas. Sans allocation (tableaux préalloués, S619).

**Références, calculées avant** (`s620_ref.py`, `s620_remontee.py`, numpy). **Thacker**, écart L1 après une période : **0.047826108** (50²),
**0.015728084** (100²), **0.005876495** (200²) — l'ordre un donnait 0,427, 0,242, 0,129 : **8.9, 15.4,
21.9 fois moins** ; rapport 100 → 200 : 2.676. Masse exacte, `h ≥ 0`, Courant ≤ 0.1897.
Le lac au repos : 3.100e-16 m/s. **La remontée** (S614, bord gauche mouillé : les murs éprouvés, ADR-254 D2), maille 1, ½, ¼ m :
**0.806045340, 0.843828715, 0.875314861 m** pour Synolakis 0.861419 (l'ordre un : 0,705, 0,793, 0,850) ;
Courant ≤ 0.4071.

**Quantum** : f64 ; la remontée à la cote d'une maille (`dx/cot β`). **Critères, écrits avant.** (1) les écarts L1 de Thacker égaux aux références
à 10⁻⁹, chacun au moins huit fois sous celui de l'ordre un, le rapport 100 → 200 au moins 2,5 ; (2) masse à 10⁻¹³, `h ≥ 0`, aux trois
résolutions et sur la plage ; (3) le lac au repos sous 10⁻¹⁴ m/s ; (4) les trois remontées égales aux références au bit ; à la maille de 1 m,
plus près de Synolakis que l'ordre un ; à ¼ m, à moins de dix quanta (0,126 m) ; (5) les essais de S613, S614, S619 inchangés ; (6) refus :
`ε` non positif.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [ ] **P2** — l'ordre deux dans `saint_venant_2d.rs` et ses essais ; (1)–(6).
- [ ] **P3** — preuve ; listes 4.14, 11.3 ; rituel (`--lot`).

### Notes de reprise
