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

Session : S466 — **terminée**. En autonomie : **le joueur** — une capsule debout au lieu de la sphère de B10.

**Ce que la session fait.** Le corps de la carte devient une **capsule** : un axe unitaire, une demi-longueur `L`, un rayon ; chaque
formule de la sphère (la reconstruction et ses images, les étiquettes, l'éloignement des particules) mesure la distance au **point le
plus proche de l'axe**, `c + a·clamp((q − c)·a, −L, L)` ; l'empreinte de la bascule s'abaisse de `L·|a_z|` et s'élargit de `L·|a_xy|`.
`L = 0` : la sphère, au bit. **Le joueur** (`JOUEUR=debout`, le défaut de `--v1` si la scène tient ; `JOUEUR=boule` : la sphère) : une
capsule verticale de 0,3 m de diamètre et 1,7 m, qui saute pieds en avant. Le rendu suit (l'afficheur, Godot, l'export).

**Critères, écrits avant.** (1) `L = 0` rend la sphère au bit (le banc `--surface-direct-banc` inchangé : 125 pas, images identiques) ;
(2) la scène `--v1` avec le joueur debout : 60 s, masse exacte, sans arrêt ; (3) les images montrées.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la capsule dans la carte ; le joueur dans la scène et le rendu ; mesures ; images.
- [x] **P3** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — la carte : `body_point` (le point de l'axe le plus proche) dans la reconstruction, ses images, les étiquettes,
  l'éloignement des particules, l'empreinte de la bascule ; `ApicCarte::set_body_shape`. La scène : `JOUEUR=debout` (le défaut de
  `--v1`), le saut pieds en avant qui **freine** dans l'eau (décélération constante, des pieds à la surface jusqu'en bas) et
  s'arrête les pieds à **0,5 m du fond** (`JOUEUR_FOND`). Le rendu : la capsule de Quílez dans `surface_carte.wgsl` et
  `saut_optique.gdshaderinc`, un `CapsuleMesh` dans `saut.gd`, `demi_longueur` dans l'en-tête de l'export et du direct.
  **Mesuré** : (1) `L = 0` : le banc S453, 125 pas, écart de masse 0, inchangé ; (2) `--v1-banc` : **60 s, 3118 pas, écart de
  masse 0** ; (3) images Godot `saut_t{0.30,0.55,1.60}.png` envoyées. **Ce qui a cassé avant** : arrêt net à 4 m/s les pieds à
  0,2 m du fond (quatre mailles) — divergence à la deuxième, troisième ou quatrième entrée (15 à 31 s) ; le freinage seul ne
  suffit pas (22 s) ; les pieds à 0,5 m suffisent, au rayon prévu (0,15 m). Des zones de relaxation le long des parois `y`,
  essayées, cassaient la scène dès 4 s (au coin du bord ouvert) : retirées.
- **P3** — C10-SCENES-S454 §15 ; journal ; jeton libre ; maillons 1 ; suivant : S467, le lot des registres (dû) et la couleur du corps sous l'eau.
