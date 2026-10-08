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

Session : S705 — **terminée**. En autonomie, sans arrêt. S698–S704 : le raccord du large par particules, nourri par SGN et posé par la
grille, est à −0,068 s et −0,13 m du tout-3D. Le juge à 2,5 cm n'est pas convergé à ce niveau (S704). **Une décision, puis la suite du LOD.**

**Ce que la session fait.**

- **ADR-278.**
  - Le raccord du large retenu : le bord à particules, la pose par la grille, les données de SGN.
  - La tolérance de temps de l'étape (ADR-275 D3), rapportée à la convergence du juge et à l'usage, sans réécrire ADR-275.
  - La crête de SGN, 5 % haute, reste une question ouverte.
- **La conception de l'étape 2 du LOD** (ADR-275 D2) : la bande 3D qui naît et meurt avec la vague. Un registre de conception :
  - le déclencheur, la naissance de la 3D depuis l'état 2D, sa mort vers Saint-Venant ;
  - la masse au bit à chaque passage ;
  - les essais qui jugeront chaque pièce.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-277)

- **témoin** : sans objet (une décision et une conception).
- **instrument** : chaque nombre de l'ADR renvoie à une preuve (S693–S704).
- **calcul** : la tolérance de temps, tirée de nombres mesurés, est écrite avec son calcul.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-275 D3 : complété par une note datée et par ADR-278, non réécrit ;
  - ADR-273 D1 : chaque pièce de l'étape 2 sera jugée d'abord entre deux copies du même solveur, et le registre le dit pièce par pièce.
- **pièges** : déplacer le but après coup. La tolérance nouvelle se justifie par une mesure du juge (S704), non par l'écart obtenu, et elle
  vaut pour les étapes suivantes.

**Critères, écrits avant.** (1) ADR-278, chaque nombre sourcé ; (2) le registre de l'étape 2, une pièce par session au plus, chacune avec
son essai ; (3) la note datée sur ADR-275.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — ADR-278 ; le registre de l'étape 2 ; la note sur ADR-275.
- [x] **P3** — rituel.

### Notes de reprise
- **P2 fini** — ADR-278 (D1 le raccord retenu ; D2 la position à 0,15 m, l'instant à 0,1 s ; D3 la crête de SGN ouverte ; D4 l'étape 1 close) ; LOD-ETAPE-2-S705 (N1, N2, M1, D1, E1) ; la note sur ADR-275.
