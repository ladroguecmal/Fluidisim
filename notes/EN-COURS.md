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

Session : S740 — **en cours**. En autonomie ; session longue. ONDE-SOLITAIRE-3D-S739 : la 3D raidit l'onde solitaire sur fond plat. **La
question** : quelle partie du pas en est la cause ? Les suspects, un à la fois (ADR-276 D2), sur le canal à 5 cm, qui montre le défaut en
6 min (S739). **Aucun remède adopté dans cette session** : un suspect qui guérit le canal est désigné, puis éprouvé à 2,5 cm.

**Ce que la session fait.**
- Deux réglages du cœur, **leurs défauts au bit** : `set_pressure_max_iterations` (4 000 par défaut) et `set_separation_passes` (2 par
  défaut).
- Le canal relève, à chaque pas, les itérations et le résidu de la pression (`ApicReport`) : le plus grand nombre d'itérations, la part des
  pas au plafond, le plus grand résidu.
- Le résumé du canal prend sa référence à 0,25 s (S739 la prenait à 0 s, avant toute surface : ±inf).

**Les essais, et leurs critères écrits avant** (le canal A à 5 cm, 4,25 s ; « guérit » = la largeur à mi-hauteur au-dessus de **80 %** de
celle de 0,25 s, et le creux sous **10 % de `H`**) :
- **E1 — la pression telle quelle** : les itérations, le résidu. Le plafond est-il atteint ? Rapporté.
- **E2 — la pression convergée** : le plafond à 100 000. Si E1 atteignait le plafond, E2 dit si c'est la cause.
- **E3 — sans la séparation des particules** (`set_separation_passes(0)`).
- **E4 — le pas plafonné à 2,5 ms** (au lieu de 10 ms ; S713 : une maille par pas).

On s'arrête au premier qui guérit, et on l'éprouve à 2,5 cm (**E5**, les mêmes critères). Si aucun ne guérit, la question reste ouverte,
avec les nombres, et les suspects suivants sont nommés.

**Contrôles du plan** (ADR-226, ADR-276, ADR-286)

- **témoin** : le canal de S739, au bit (les réglages à leurs défauts) ; SGN garde la même onde (S694).
- **instrument** : la largeur à mi-hauteur et le creux, par la surface ; les itérations et le résidu, par le pas lui-même.
- **calcul** : E1 à E4, ≈ 6 min chacun (A seulement, 3 min) ; E5 ≈ 15 min.
- **ADR** : ADR-276 D2 (une cause à la fois) ; ADR-286 D1 (le canal de 24 m, 5,1 m de marge, inchangé).
- **pièges** :
  - un plafond d'itérations levé peut rendre un pas très long ;
  - sans séparation, des particules se tassent : le volume est rapporté.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — les réglages, le relevé de la pression ; E1, E2.
- [ ] **P3** — E3, E4 ; **ajoutés après E4** (aucun n'a guéri ; le profil montre l'onde qui se scinde, un long plateau devant, un pic
  serré derrière, comme un tassement des particules sous la crête) : **E5, la projection de densité** (S709, complète) et **E6, faible**
  (κ = 0,05, S710). Les mêmes critères. Puis l'essai à 2,5 cm du suspect désigné.
- [ ] **P4** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** — E1 : la pression converge en **138 itérations au plus**, jamais au plafond (0 sur 523 pas), résidu 1,0·10⁻⁶ ; la largeur
  tombe à 20 %, le creux à 54 mm ; ne guérit pas. E2 (le plafond à 100 000) : **identique au bit**. Le plafond de la pression n'est pas la
  cause.
- **E3** (sans séparation) : la largeur à 22 %, le creux 46 mm, la crête finale 82 mm ; ne guérit pas.
- **E4** (le pas à 2,5 ms) : la largeur à 23 %, le creux 25 mm, la crête finale 62 mm ; ne guérit pas.
- **Le profil** (`captures/s740_profils.png`, A à 5 cm, toutes les 0,5 s) : l'onde se scinde. Un long plateau bas, d'environ 20 mm, file
  devant sur plusieurs mètres ; un pic étroit, plus lent, reste derrière ; du bruit et des creux suivent.
