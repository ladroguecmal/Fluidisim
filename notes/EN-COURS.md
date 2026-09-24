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

Session : S357 — **en cours**. **Rendu 2 (ADR-192 D2) : la mer de B dans Godot 4.4.1** — le prototype que l'utilisateur
a choisi comme premier pas, jugé sur images avant tout portage.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.
Entrée — S356 : les crêtes de B dans l'afficheur, puis *« rendu toujours pas convaincant »* et le choix de Godot 4.
**Ce que le prototype est** : un projet Godot dans `godot/`, **sans intégration native ni téléchargement** — les
composantes de la mer de `--meilleur` exportées par l'afficheur en un fichier de données (dérivé, non versionné) ; un
nuanceur d'eau dans le langage de Godot qui porte le CWM de la bande, la queue en pentes filtrées, l'écume et la
lumière des crêtes de S356 ; l'environnement de Godot — ciel, soleil à la direction de la scène, tonalité, reflets à
l'écran, brume. **Ce qu'il n'est pas** : ni W, ni δ, ni la requête de jeu ; une image à juger.

Critères, écrits avant le code :
1. **L'export** (`--export-godot`) : composantes de bande et de queue, `M`, retard, seuils de l'écume, gain ; le même
   instant rejoué dans Godot rend la **même hauteur** que le cœur en quelques points (écart < 1 mm), lu par un script.
2. **La scène Godot** : grille autour de la caméra, nuanceur, environnement ; phases temporelles repliées côté script en
   double précision (I-08) ; lancée sans erreur de compilation du nuanceur.
3. **Les images** : les poses de R14 (proche, rasante) à 12 s, capturées par Godot lui-même, et les mêmes poses de
   l'afficheur à côté ; images pour la revue **R19**, questions et références demandées.
4. Preuve, file, feuille de route, liste 8.1.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'export depuis l'afficheur ; critère 1 (partie données).
- [x] **P3** — le projet Godot : scène, nuanceur, environnement ; critère 2 et la hauteur du critère 1.
- [ ] **P4** — les captures et la revue R19 ; critère 3.
- [ ] **P5** — preuve, file, feuille de route, liste ; critère 4.
- [ ] **P6** — rituel.

### Notes de reprise
- **P2.** `--meilleur --export-godot[=fichier]` (`rendu_cretes::export_godot`) → `godot/donnees/mer_b.json`, dérivé et
  ignoré par git : 64 lignes de bande, 60 de queue, `[a, kx, ky, φ, ω]` à l'origine et à 12 s ; `M` = 2, retard −0,2,
  système 1 de 32 composantes, `k̄` 0,1742 et 0,0317 rad/m ; 14 seuils ; soleil (−0,424 ; 0,318 ; 0,848) ; cinq
  points de contrôle, `η` linéaire de la bande à `t₀ + 3 s` calculé par le cœur (0,15376 m à l'origine).
- **P3.** `godot/` : `project.godot` (Forward+, 1280 × 720, MSAA 2), `mer.tscn`, `mer.gd` (grille polaire 720 × 360
  de 0,25 m à 12 km autour de la caméra, environnement, phases repliées en double, poses de R14, `--captures`,
  `--controle`), `eau.gdshader` (bande CWM + Tayfun, queue filtrée, pente eulérienne, écume à seuils par empreinte,
  crêtes en `BACKLIGHT`, pente non résolue en rugosité GGX `mss^(1/4)`, albédo `0,54·R(0⁻)`). **Contrôle** (`--headless
  -- --controle`) : `η` de la bande recalculé par Godot contre le cœur à `t₀ + 3 s`, **1,0·10⁻⁷ m au pire** sur cinq
  points (critère : 1 mm). Nuanceur compilé sans erreur (Vulkan 1.4, RTX 5070 Laptop). **Ce que les images ont dit** :
  le ciel physique par défaut de Godot rend un **crépuscule gris** → ciel procédural aux couleurs du ciel clair de
  l'afficheur (photo de R14) ; en rasant, **les reflets à l'écran remplaçaient le ciel par l'eau sombre** → éteints
  (`REFLETS_ECRAN=1` les rallume) ; le demi-ciel bas pris à la couleur de l'horizon.

