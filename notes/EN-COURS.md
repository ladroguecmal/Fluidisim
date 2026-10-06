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

Session : S549 — **en cours**. En autonomie, **6.6 — la carène libre** : l'eau d'un compartiment à demi plein garde sa surface horizontale
quand la coque gîte ; son centre se déplace vers le bord bas, et la stabilité perd `i/∇` (la hauteur métacentrique, `GM → GM − i/∇`,
`i` le moment d'inertie de la surface libre) — un navire qui chavire par l'eau qu'il embarque.

**Ce que la session fait.** Deux pièces du cœur. (a) `VolumeShape::centroid_below_um(plan)` : le centre de la part mouillée d'une forme de
V — chaque tétraèdre découpé par le plan (un, deux ou trois sommets mouillés). (b) `RigidBody::loads` : des charges ponctuelles (un point
du corps, une force du monde) ajoutées aux forces et au moment — vide par défaut. L'essai : la barge de S548 (20 × 8 × 4 m), son
compartiment central (5 × 8 m) à 1 m d'eau, sans brèche ; son centre de masse décalé latéralement (le proxy déplacé) ; l'eau du compartiment
pèse en son centre, calculé à chaque pas sous la pesanteur vue du navire (« libre »), ou fixe à son centre au repos (« figée »).

**Ordre de grandeur, calculé.** 246 t + 41 t d'eau : `∇` = 280 m³, `T` = 1,75 m, `KB` 0,875, `BM` 3,048, `KG` 1,786 → `GM` figé **2,137 m** ;
`i = l·b³/12` = 213,3 m⁴, `i/∇` = **0,762 m** → `GM` libre 1,375 m, rapport **1,554**. Décalage du centre total de 8,6 cm : gîte figée
2,30°, libre 3,57° ; à 4°, l'eau monte de 0,28 m au bord (sous les 1 m : la surface reste entre les parois).

**Critères, écrits avant.** (1) Le centre mouillé d'une boîte inclinée : le déplacement latéral `b²·tan θ/(12 h)` à 10⁻⁹ près (rapport à
l'arrondi f64 > 10⁶). (2) Sans charges, la suite au bit. (3) Le rapport des tangentes de gîte libre / figée à 3 % de `GM_f/(GM_f − i/∇)`,
`GM_f` mesuré sur la gîte figée (le proxy a son erreur propre, S499 : elle se retire par le rapport).

### Plan

- [ ] **P1** — jeton, plan seul.
- [ ] **P2** — le centre mouillé, les charges, les essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 6.6 ; rituel.

### Notes de reprise
