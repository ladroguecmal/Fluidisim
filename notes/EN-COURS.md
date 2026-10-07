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

Session : S625 — **en cours**. En autonomie (ADR-247 : la physique des partiels). **4.14 — « une houle sur une plage réelle »** (un manque
de S613) : une houle longue périodique entre par le bord caractéristique (S622) et monte et descend une pente, cycle après cycle.

**Ce que la session fait.** Aucun code nouveau dans le cœur : un essai — `SaintVenant2D` d'ordre deux (S620), `pas_avec_bord` (S622) nourri
d'une houle `η = A·sin ωt`, `u = √(g/d)·η` ; une pente 1:19,85 précédée de 300 m de fond plat à 10 m ; huit périodes ; la remontée d'un cycle
établi — le maximum, sur les deux dernières, de la surface `z + h` de la maille mouillée la plus haute (continue, non quantifiée). La
référence analytique : **Keller & Keller (1964)**, `R = 2A/√(J₀(2kL)² + J₁(2kL)²)`, que Carrier & Greenspan (1958) montrent exacte aussi
pour la remontée non linéaire d'une houle non déferlante. Ne fait pas : la houle déferlante, la dispersion, la houle oblique.

**Références, calculées avant** (`s625_ref.py`, numpy ; `J₀`, `J₁` par leurs séries). `A` = 5 cm, `T` = 60 s (λ = 594 m) : `2kL` =
4.197441, **R = 0.249190301 m** (`R/A` = 4.9838) ; non déferlante (`R·ω²/(g·tan²β)` = 0.1098 < 1). Remontée
mesurée, maille 1, ½, ¼ m : **0.250237555, 0.249060917, 0.249310645 m** — +0.420 %, -0.052 %, +0.048 %.

**Quantum** : f64 ; l'accord avec numpy est demandé à **10⁻⁶ m**, non au bit — S622 a mesuré que le limiteur minmod amplifie un ulp jusqu'à
10⁻⁸ m dans les zones presque plates. **Critères, écrits avant.** (1) les trois remontées égales aux références à 10⁻⁶ m ; (2) à moins de 1 %
de Keller & Keller aux trois mailles, de 0,1 % à ½ et ¼ m ; (3) `h ≥ 0`.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — l'essai ; (1)–(3).
- [ ] **P3** — preuve ; liste 4.14 ; rituel.

### Notes de reprise
