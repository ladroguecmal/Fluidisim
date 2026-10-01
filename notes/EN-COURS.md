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

Session : S422 — **en cours**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7e**, la multigrille
pour la projection d'APIC sur la carte ([preuve](../docs/validation/APIC-CARTE-S416.md) §13 : la projection pèse 2,5 à 3 ms sur
B10 en bande étroite, 207 itérations du gradient conjugué diagonal, cinq dispatchs chacune).

**La recette** — celle de C1/C3a (S385, S390), transposée : gradient conjugué **préconditionné par un cycle en V** ; Jacobi amorti
ω = 6/7, deux lissages avant et deux après, huit au plus grossier ; restriction par la moyenne des huit filles, prolongation par
injection (adjointes à un facteur près : le préconditionneur reste symétrique défini positif) ; niveaux grossiers rediscrétisés à
chaque projection, l'opérateur divisé par 4 à chaque niveau. **Pour APIC** : le niveau fin est l'opérateur exact (fluide fantôme
`θ`, solide sans flux) ; une maille grossière est active si une fille est d'eau, d'air (Dirichlet à demi-maille) sinon, solide (sans
flux) si toutes ses filles le sont ; le domaine est clos ; dimensions impaires : les filles hors du domaine sont ignorées. **Pour les
dispatchs** (le coût dominant, S421) : le niveau fin et le premier niveau grossier par dispatchs, **tous les niveaux suivants dans un
seul groupe** de 256 fils, barrières entre phases. Arrêt au même critère que la référence (`‖r‖² ≤ tol²·‖b‖²`) ; plafond adaptatif.

**Critères, écrits avant.** (1) Le cycle est **symétrique** (`⟨u, M⁻¹v⟩ = ⟨M⁻¹u, v⟩` à 10⁻⁵ relatif) et positif, sur la carte. (2)
Étages : la projection converge au critère de la référence, vitesses corrigées à **10⁻⁴ m/s** de la référence (cuve du
ballottement, raccord, B10). (3) Les issues : ballottement, raccord, bande, B10 nu et en bande étroite — pincement au pas de la
référence, surfaces dans leurs tolérances (3 mm ou témoin), volume exact. (4) **Le coût de la projection** sur B10 en bande étroite :
publié, visé ≤ 1 ms. (5) Suite, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la hiérarchie : niveaux, tampons, natures des mailles (niveau 1 depuis les étiquettes, les suivants dans un groupe).
- [x] **P3** — le cycle en V (fin, niveau 1, le groupe des niveaux grossiers) ; banc de symétrie et de positivité ; critère 1.
- [ ] **P4** — le gradient conjugué préconditionné par le cycle, option `MULTIGRILLE=1` ; étages ; critère 2.
- [ ] **P5** — les issues et le coût ; critères 3 et 4.
- [ ] **P6** — non-régression, suite ; preuve §14 ; registres.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2** — la hiérarchie : niveaux divisés par deux (arrondi au-dessus) tant qu'une dimension dépasse 2, huit au plus (ballottement
  40×4×20 : six niveaux ; B10 16×16×84 : sept) ; `mgb` (nature, x, r, t par niveau grossier), `mgl` (la table) ; `mg_kind1` depuis les
  étiquettes, `mg_kind_coarse` dans un groupe. Le nombre de niveaux et les blocs du niveau 2 passent par l'uniforme.
- **P3** — le cycle en V : `mg_f_first`, `mg_f_qz`, `mg_restrict1`, `mg_l1_first`, `mg_l1_tx`, **`mg_coarse` (niveaux ≥ 2 dans un groupe
  de 256 fils)**, `mg_l1_xt`, `mg_l1_tx`, `mg_prolong0`, `mg_f_zq`, `mg_f_qz_fold` (onze dispatchs). **FXC, appris** : il refuse une
  barrière (X3663, X4026) dans une boucle dont la borne vient de la mémoire, après un `continue` qui dépend du fil, et — le dernier
  verrou — **dès qu'une boucle de bornes non constantes porte plus d'une barrière** ; toutes les boucles à barrières de `mg_coarse` ont
  donc des bornes constantes (huit niveaux, sept lissages), le travail gardé ; `mg_row` et `mg_child` à sortie unique. Banc
  `--apic3d-carte-mg-cycle` (`CAS=`, `raccord`, `b10`) : **symétrie relative 9,5·10⁻⁸, 1,8·10⁻⁷, 2,5·10⁻⁷ ; positivité tenue** ; la
  projection converge au critère de la référence en **9, 9 et 11 itérations** (diagonale : 94, 94, 207). Critère 1 tenu.
