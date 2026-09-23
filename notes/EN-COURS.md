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

Session : S328 — **en cours**. **Lot 3 : le mode mobile sur fond coupé** (4.15), alternance
d'[ADR-188](../docs/adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« continue »*, après S327 ; suite déclarée : le mode mobile sur fond coupé. **Maillons à 3** :
une capacité est due.

**Ce que la session doit rendre possible.** La référence 3D de δ porte depuis S324 un fond coupé, mais
en mode **linéaire** seulement : le pas à surface mobile le refuse. Or c'est la surface mobile qui porte
les corps flottants du lot 4 et la frontière mobile du lot 3. Point 4.15 : « Manquent … le mode mobile ».
Le pas mobile **2D** porte déjà la découpe (S237, `delta_mobile.rs`) : la 3D doit la reproduire au bit à
`ny` = 1, comme S324 l'a fait pour le mode linéaire.

**Le portage.** Maille mouillée : fraction non nulle et centre sous la surface. Lignes de pression
pondérées par les ouvertures, fantômes compris ; second membre par la divergence ouverte ; correction et
extrapolation limitées aux faces ouvertes entre mailles de fluide ; transport des hauteurs par débits
ouverts ; advection qui saute les faces fermées ; garde de géométrie : surface à deux mailles au moins
au-dessus du plus haut coin du fond de sa colonne — celle de la 2D. Le fond plat, poids 1, reste au bit.
Le pas **couplé** à B/W continue de refuser la découpe.

Critères, écrits avant le code :
1. **Fond plat** : le pas mobile 3D inchangé au bit — tous les essais antérieurs passent.
2. **`ny` = 1**, trois fonds de S232 : 200 pas mobiles **identiques au bit** au pas mobile 2D — surface,
   `u`, `w`, pression, itérations.
3. **Lac au repos** sur la bosse 3D : 100 pas mobiles, vitesses et surface nulles en bits.
4. **Invariance transverse** : fond sans `y`, `ny` = 4 — les tranches identiques entre elles au bit, et
   à 10⁻⁵ près en relatif de la tranche `ny` = 1 après 100 pas.
5. **Ordre** du débit ouvert au premier pas, en mode mobile, sur la bosse 3D : **≥ 1,8**, comme S324.
6. **Refus** : surface à moins de deux mailles du fond coupé → `Domain`, état restauré au bit.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le pas mobile 3D porte la découpe ; fond plat au bit, suite verte.
- [x] **P3** — essais des critères 2, 3, 4 et 6.
- [ ] **P4** — banc : le premier pas mobile sur la bosse, trois mailles, ordre ; écart au mode linéaire.
- [ ] **P5** — preuve : section datée de [FACES-COUPEES-3D-S324](../docs/validation/FACES-COUPEES-3D-S324.md),
  avec « Reproduire » ; file, liste 4.15.
- [ ] **P6** — rituel.

### Notes de reprise
**P2 (22:39).** `delta3d_mobile.rs` : maille mouillée = fraction non nulle et centre sous la surface ;
lignes `(voisin, ouverture, 1/θ, valeur)`, l'ouverture valant 1 sans découpe — `a·(p−q)`, `a·p·θ⁻¹`,
`a·valeur·θ⁻¹·dx⁻²`, `a·u·dx·mouillé`, l'ordre de la 2D ; correction, extrapolation et advection
sautent les faces fermées ; garde : plancher = plus haut coin du fond de la colonne (`Cut3::floor`,
compté à la configuration) + deux mailles. Le pas mobile restaure la diagonale de Jacobi du chemin
linéaire coupé. Le pas couplé refuse toujours la découpe (essai de S324 réécrit sur lui). Cœur : **469
réussis**, 0 échec ; les identités 2D du mode mobile sur fond plat (S296, 1 604 pas) passent au bit.

**P3 (22:44).** Quatre essais `…_s328`, tous tenus. **Critère 2** : à `ny` = 1, sur les trois fonds de
S232, 200 pas mobiles **identiques au bit** au pas mobile 2D — surface, pression, `u`, `w`, itérations.
**Critère 3** : lac au repos sur la bosse 3D, 100 pas, vitesses et surface nulles en bits. **Critère 4** :
fond sans `y`, `ny` = 4 — tranches identiques au bit entre elles, **et à la tranche `ny` = 1** après cent
pas (écart nul). L'écoulement transverse, lui, n'est nul qu'à l'arrondi : 6·10⁻⁹ m/s ; **le fond plat
de S296 en fait autant** (5·10⁻⁹ pour 0,037 m/s de `u`) — la diagonale de Jacobi des rangées de bord
n'est pas celle des rangées intérieures. L'essai exigeait `v` nul au bit, ce que le critère ne demandait
pas : borne relative 10⁻⁶ de `u`. **Critère 6** : surface à 1,4 m sur la bosse refusée, état restauré au
bit ; la même profondeur passe sur un fond plat à 0,3 m. Suite complète : **594 réussis**, 0 échec.
