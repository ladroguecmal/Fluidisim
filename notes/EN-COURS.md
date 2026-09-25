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

Session : S368 — **en cours**. **Rendu 9 : l'écume qui dure** — *« Continue »* ; alternance d'ADR-191 D3 après S367
(physique). L'écume rendue a été **refusée** (R21) : sans mémoire, des taches instantanées au bord lisse. S367 a
construit la référence du champ d'ADR-014 ([ECUME-S367](../docs/validation/ECUME-S367.md)) ; ici sa **production sur la
carte de Godot** (SPEC-006 §4) et l'écume rendue qui en naît (liste 8.4, absente).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Ce qui se construit.** `ecume.comp` + `ecume.gd` : un champ de 1 024 × 1 024 texels de 0,25 m (256 m) autour de la
caméra, deux canaux (actif, résiduel) en RGBA32F — l'état interne en f32 : la décroissance pas à pas ne tiendrait pas en
demi-précision —, le pas de S367 à l'identique (advection semi-lagrangienne par la vitesse orbitale de la bande,
décroissance exacte, sources aux crêtes les plus accélérées, seuil `κ·σ_a`). Mise en régime simulée avant la capture
(60 s de passé : B est analytique). Au rendu : l'actif en écume blanche au bord irrégulier, le résiduel en dentelle.

Critères, écrits avant le code :
1. **Le pas sur la carte = la référence** : décroissance d'un champ uniforme contre la solution fermée (10⁻⁵ relatif,
   f32 sur 600 pas) ; advection d'une bosse par une vitesse uniforme (mode de contrôle), centre à 1 cm près.
2. **La couverture** : la part où l'actif dépasse ½, relue sur la carte au régime, contre `couverture_monahan` de
   l'export (0,42 %) ; κ recalé ici (la bande de l'afficheur n'est pas celle du cœur), **dit** ; prédiction de S367 :
   κ ≈ 2,98.
3. **R26** : l'écume d'avant contre celle-ci, plusieurs poses ; une séquence de trois images espacées de 2 s — la durée
   se juge dans le temps.

### Plan

- [x] **P1** — jeton, plan seul.
- [>] **P2** — `ecume.comp`, `ecume.gd` : le champ sur la carte ; contrôles de décroissance et d'advection ; critère 1.
- [ ] **P3** — la couverture relue, κ recalé ; critère 2.
- [ ] **P4** — l'écume rendue depuis le champ (`eau.gdshader`) : actif et dentelle ; images R26 ; critère 3.
- [ ] **P5** — preuve ECUME-GODOT-S368 ; liste 8.4, file, dépendances, feuille de route, index.
- [ ] **P6** — rituel.

### Notes de reprise
