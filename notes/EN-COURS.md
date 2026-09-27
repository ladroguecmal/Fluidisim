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

Session : S405 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« continue »*. Suite proposée par S404 : deux maillons, un
lot qui fait changer de case un point de la liste — **9.3**, la prédiction balistique, à comparer au raccord C5. **Choix** : 9.3.
Il est au front 0 (rien à attendre), il débloque 9.4 et 9.6, et il sert la scène de C10 — le saut ; C5 reste sous la condition
d'A316, et un changement de case n'y est pas en vue (4.16 est déjà partiel). Agent : Claude, session cloud Claude Code ; fichiers,
git, cargo, Python ; ni carte graphique, ni Godot. Branche `claude/eager-volta-lf0kw3`, la plus avancée.

**Thèse.** La source (§8.2) : pour un objet balistique — un véhicule qui quitte un pont —, estimer **point d'impact, vitesse,
orientation, rotation, région de simulation utile** ; le temps de vol devient la fenêtre qui prépare le domaine d'eau. ADR-013 §2
en donne les paliers (T4 veille, T3 réservation à `t < 8 s`, T2 construction quand `½·a_max·t² ≤ R`, T1 actif à `t < 0,3 s`) :
un objet balistique (`a_max` = 0) passe en T2 dès qu'il est à moins de 8 s de l'eau. Rien de cela n'existe dans le code : `Follow`
(S401) prolonge la vitesse **horizontale** d'un objet, sans chute ni surface. S405 écrit le prédicteur dans le cœur — translation
sous gravité et traînée quadratique (RK4), rotation libre d'un corps rigide (équations d'Euler), contact de sa sphère englobante
avec une **surface mouvante** (la houle de B), affinage de l'instant — ; la **région utile** couvre l'incertitude de traînée ; le
**palier** suit ADR-013 §2. Le consommateur : le domaine épars en mer, dont l'ensemble est préparé au point d'impact prévu.

**Critères, écrits avant.** (1) Vide, surface plane : instant et point d'impact exacts à 10⁻⁹ s et 10⁻⁸ m (RK4 est exact sur une
parabole) ; pas d'impact dans l'horizon, ou objet déjà sous la surface : `None`. (2) Chute verticale avec traînée quadratique
contre la solution analytique : ordre 4 lu sur trois pas (rapport 16 à 20 % près) ; ≤ 10⁻⁶ s à 10 ms. (3) Rotation libre : toupie
symétrique contre sa précession analytique, rotation autour d'un axe principal contre l'orientation exacte, ≤ 10⁻⁸ à 1 ms sur 2 s.
(4) Surface mouvante (houle de 5 cm) : l'instant de contact à ≤ 1 ms d'une référence à 0,1 ms ; publié : l'erreur d'une prédiction
qui ignorerait la houle (prédiction : quelques ms, quelques cm). (5) Paliers d'ADR-013 §2 aux frontières. (6) Région : une traînée
connue à ±30 % près — les trois prédictions dans la région. (7) Banc en mer : un objet de 0,25 m lancé à 12 m/s de 6 m, l'ensemble
revu toutes les 0,5 s ; **avec prédiction**, la région d'impact dans l'ensemble ≥ 0,5 s avant l'impact, la source d'entrée (le
volume déplacé) toujours dedans, l'écart au domaine entier ≤ 3 mm (prédiction ≤ 1 mm) ; **le témoin** — l'objet suivi là où il est —
voit sa source refusée à l'impact (prédiction). (8) Suite entière, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `ballistic.rs` : état, prédiction (RK4, traînée, Euler, contact, affinage), région, palier.
- [x] **P3** — essais des critères 1 à 6.
- [x] **P4** — le banc `delta3d_impact_prevu` : prédiction contre témoin, en mer ; lancé.
- [x] **P5** — les calculs du banc ; critère 7.
- [x] **P6** — suite entière, zéro avertissement ; critère 8.
- [x] **P7** — preuve `IMPACT-PREVU-S405` ; liste (9.3, 9.2), file, feuille de route, index ; note datée d'ADR-013.
- [ ] **P8** — rituel.

### Notes de reprise
- **P2** — `code/water-core/src/ballistic.rs` (module public `ballistic`) : `Ballistic` (position, vitesse, quaternion, ω du corps,
  inertie principale, traînée `k`, rayon englobant), `predict` (RK4 sur translation et rotation, orientation renormalisée ; contact
  du point le plus bas avec `surface(p, t)` ; bisection sur un pas partant du début du pas, 10⁻¹² s), `predict_region` (région :
  rayon + plus grand écart horizontal aux impacts des bornes de traînée), `tier` (ADR-013 §2 ; T2 exige aussi `t < 8 s`).
  Aucune allocation. Essais dans `tests_ballistic.rs` (P3).
- **P3** — six essais `_s405`. **Critère 1 tenu** : parabole à **5,4·10⁻¹³ s**, 6,5·10⁻¹² m ; `None` hors horizon, au contact,
  paramètres invalides. **Critère 2 tenu** : chute avec traînée (`v_t` = 14 m/s, 20 m) — écarts 2,51·10⁻⁶ / 1,74·10⁻⁷ /
  1,12·10⁻⁸ s à 0,2 / 0,1 / 0,05 s, rapports **14,4 et 15,6** (ordre 4) ; 1,9·10⁻¹¹ s à 10 ms. **Critère 3 tenu** : toupie
  symétrique 7,3·10⁻¹⁴ rad/s, axe principal 4,1·10⁻¹⁴ sur 2,02 s. **Critère 4 tenu** : houle de 5 cm, contact à 1,105745 s,
  pas de 10 ms à 2·10⁻¹² s de la référence ; **ignorer la houle coûte +2,10 ms et +2,2 cm**. **Critère 5 tenu** : paliers aux
  frontières (dont l'avion de chasse : T2 jusqu'à √2 s). **Critère 6 tenu** : ±30 % de traînée, région 0,753 m ; la plus éloignée
  de 21 traînées à 0,503 m du nominal.
- **P4** — `examples/delta3d_impact_prevu.rs` : la mer de S404 (32 × 16 m à 25 cm, houle de S369, relatif, éponge 2 m) ; une
  sphère de 0,25 m lancée de (3, 8, 6) m à 12 m/s, traînée vraie 0,012, nominale 0,01 ± 30 % ; impact vrai (pas de 1 ms, sur la
  houle) à **1,1135 s en (15,334 ; 8,000) m**, vitesse (10,15 ; 0 ; −10,03) m/s ; entrée : le volume de la calotte immergée, en
  gaussienne de 0,5 m ; ensemble revu toutes les 0,5 s. `ballistic::advance` ajouté (le même intégrateur, sans contact), vérifié
  dans l'essai de la parabole. Cas `prevu` et `temoin` lancés à 10:32.
- **P5** (en cours) — phase 0 (revues à 0 ; 0,5 ; 1,0 s) : **`prevu`** — prédictions T2 à 0 s (erreur 0,099 m, région 0,401 m,
  instant −5,6 ms), T2 à 0,5 s (0,017 / 0,276 m, −3,1 ms), T1 à 1,0 s (0,000 m, −0,2 ms) ; la région d'impact dans l'ensemble
  **1,11 s** avant l'impact ; source jamais dehors ; écart 0,595 mm ; part moyenne 0,468. **`temoin`** — la région dans l'ensemble
  0,11 s avant, **source pas refusée** : la dernière revue tombe 0,11 s avant l'impact, l'objet n'est plus qu'à 1,2 m de son point
  d'impact. **La prédiction du critère 7 (témoin refusé) manquée à cette phase.** Ce qui sépare la dernière revue de l'impact
  décide : `IMPACT_PHASE_S` décale les revues ; balayage 0,1 à 0,4 s des deux cas, lancé à 10:38.
- **P5** (fin) — balayage de la phase des revues, 0 à 0,4 s. **`prevu`**, cinq phases : région dans l'ensemble **1,11 s** avant
  l'impact, source **jamais** dehors, écart **0,594–0,595 mm** ; à chaque revue l'erreur du point décroît (0,099 → 0,000 m), toujours
  dans la région (0,401 → 0,250 m), l'instant de −5,6 à 0 ms ; part moyenne 0,468–0,480. **`temoin`** : dernière revue 0,11 / 0,01 /
  0,41 / 0,31 / 0,21 s avant l'impact (phases 0 / 0,1 / 0,2 / 0,3 / 0,4) — **refusé à la phase 0,2** (source dehors à 1,10 s : la
  gaussienne d'entrée, 1,5 m, dépasse l'ensemble centré sur l'objet 0,41 s plus tôt), dedans aux quatre autres (écart 0,59–0,93
  mm) ; part moyenne 0,407–0,433. **Critère 7** : la part prédiction tenue (1,11 s ≥ 0,5 s ; jamais dehors ; 0,595 mm ≤ 1 mm) ; la
  prédiction « témoin refusé » manquée à la phase prévue, vérifiée à une phase sur cinq. La prédiction coûte 10 à 15 % de mailles
  (la région calculée à δ = 0 avant l'impact).
- **P6** — suite entière : **732 réussis**, 19 ignorés, zéro échec, zéro avertissement (726 + 6 essais `_s405`).
- **P7** — preuve `docs/validation/IMPACT-PREVU-S405.md` (Reproduire au commit `ea9c123c`) ; **9.3 absent → partiel** (liste,
  décompte 3 / 72 / 45, section 9 : 9 partiels et 4 absents ; REPRISE §4, feuille de route §3 et §3 ter) ; registre des
  dépendances réécrit (9.3 : « un corps quelconque, le vent, l'entrée orientée ») ; index ; note datée d'ADR-013 (le palier lu :
  T2 exige aussi `t < 8 s`).
