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

Session : S660 — **en cours**. En autonomie vers la v2 ; 2.7. En S659, le modèle parabolique aux petits angles s'écarte des mesures de
Berkhoff de 0,2 à 0,4 sur trois sections ; la maille et l'axe des mesures sont écartés.

**Le témoin : le grand angle** (Booij 1981, Kirby 1986). De `∂_xφ = i·k̄·√(1 + X)·φ`, `X = [(k² − k̄²) + (1/p)·∂_y(p·∂_y)]/k̄²`, la racine
approchée par Padé [1,1], `(1 + ¾X)/(1 + ¼X)`, au lieu de `1 + ½X` : `(1 + X/4)·(A_x − lev·A) = (i·k̄/2)·X·A` (dérivé au plan ; à
`X` petit, l'équation de S659 ; à `X` = 0, la levée). Crank–Nicolson, les coefficients au demi-pas, tridiagonal. `propager_grand_angle`, à
côté de `propager` (S659 reste reproductible).

**Contrôles du plan** (ADR-266)

- **témoin** : deux causes nommées en S659 — l'angle, la non-linéarité. Le grand angle supprime (une grande part de) la première ; si
  l'écart des sections 2 et 5 baisse d'au moins 30 % chacune, c'était l'angle ; sinon, c'est la non-linéarité ou autre chose.
- **instrument** : le même lecteur et les mêmes mesures qu'en S659 ; le nouveau modèle éprouvé d'abord sur les deux cas analytiques
  (le plat à 10⁻⁶, la levée à 0,5 %), et sur un cas où le grand angle doit gagner : une onde plane **oblique** à 30° sur fond plat, dont
  `|A|` doit rester 1 (les petits angles la déforment) — rapporté pour les deux modèles.
- **calcul** : aucun nombre nouveau hors l'onde oblique (`A = e^(i·k·sin30°·y)` posé au bord, `|A|` = 1 attendu).
- **ADR** : ADR-259 D1, ADR-263 D2, ADR-266 ; S659 inchangé.
- **pièges** : l'approximation de Padé à grand angle n'est pas exacte au-delà de ~45° ; les parois latérales réfléchissent l'onde
  oblique (lire loin des parois) ; l'ordre des opérateurs (le terme de levée traité en diagonale).

**Critères, écrits avant.** (1) Le grand angle : le plat à 10⁻⁶, la levée à 0,5 %. (2) Le verdict du témoin : la baisse des écarts des
sections 2 et 5 (≥ 30 % chacune : l'angle). (3) Le critère de S659 rejugé (≤ 0,20 sur chaque section, le pic de la section 3 à 15 %).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `propager_grand_angle` ; les essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 2.7 ; rituel.

### Notes de reprise
