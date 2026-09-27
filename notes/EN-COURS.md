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
- [ ] **P2** — `ballistic.rs` : état, prédiction (RK4, traînée, Euler, contact, affinage), région, palier.
- [ ] **P3** — essais des critères 1 à 6.
- [ ] **P4** — le banc `delta3d_impact_prevu` : prédiction contre témoin, en mer ; lancé.
- [ ] **P5** — les calculs du banc ; critère 7.
- [ ] **P6** — suite entière, zéro avertissement ; critère 8.
- [ ] **P7** — preuve `IMPACT-PREVU-S405` ; liste (9.3, 9.2), file, feuille de route, index ; note datée d'ADR-013.
- [ ] **P8** — rituel.

### Notes de reprise
