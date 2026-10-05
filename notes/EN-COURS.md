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

Session : S492 — **terminée**. En autonomie, **A327 — le mur aligné sur la grille** ([DECOR-S490](../docs/validation/DECOR-S490.md) §3) :
la projection linéaire de δ stagne quand la paroi d'un décor qui perce la surface tombe sur un plan de la grille (ou le dépasse d'un
micromètre) ; un micromètre en deçà, tout tient.

**Ce que la session fait.** Reproduire au plus court (l'essai ignoré de S490) ; regarder la matrice au pas qui stagne (diagonales,
ouvertures des faces et du couvercle des colonnes contre la paroi) ; nommer le mécanisme ; corriger ; faire passer l'essai.

**Entrées, et comment elles se vérifient.** L'essai `a_grid_aligned_wall_through_the_surface_a327_s490` (échec au pas 307) ; le témoin à un
micromètre en deçà (passe). **Critères, écrits avant.** (1) le mécanisme nommé, chiffré au pas qui stagne ; (2) l'essai aligné passe (période
à 3 %, aucune fuite) ; (3) les essais de δ passent, la non-régression tient ; (4) 6.5 validée si (2)–(3).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le mécanisme.
- [x] **P3** — la correction ; (2)–(3).
- [x] **P4** — preuve ; 6.5 ; rituel.

### Notes de reprise
- **P2** — diagnostic : divergence par maille ≈ 5·10⁻⁸ (mailles pleines) ; aucun `d·A·d ≤ 0` ; au pas refusé, résidu sous le seuil,
  plancher atteint, divergence 2,97·10⁻⁵ avec max|u| = 1,1·10⁻⁴ m/s — **le point mort** de la seiche, pas la géométrie.
- **P3** — premier remède (le plancher dans la mesure, 2D comprise) : six essais changés au bit — retiré ; second (la seule décision
  finale de la 3D linéaire, au plancher d'arrondi) : 651 essais passent, le mur aligné passe (0,16 %).
- **P4** — ADR-225 ; preuve A327-S492 ; A327 levée ; 6.5 (manque la carte) ; index ; journal.
