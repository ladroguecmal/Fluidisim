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

Session : S401 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Reprends le projet »*. Suite déclarée par S400 : dans le
cloud, **C8b, le domaine épars** ; décision de S397 : *« terminer le solveur »*. Agent : Claude Opus 5.5, session cloud Claude
Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot. Branche `claude/eager-volta-lf0kw3`, partie de `main` (S383) et
avancée par avance rapide jusqu'à `claude/blissful-pasteur-m4j5rn` (S400, jeton libre) : aucun fork, une seule lignée. Suite de
départ : 697 réussis, 18 ignorés, zéro avertissement.

**Thèse.** Un domaine δ est un **ensemble de blocs** (ADR-006 §3 ; colonnes de 8 × 8, toute la profondeur, S396) dans une
**fenêtre** du réseau commun. Une colonne hors de l'ensemble est **hors du domaine** : les faces qui la bordent sont des murs, ses
mailles sortent de tous les opérateurs — par le chemin que la découpe prend déjà (`open3`, `solid3`, S328), et le bord de
l'ensemble se comporte **comme le bord de la boîte** (l'advection y lit la face elle-même, pas un zéro). L'ensemble **suit la
perturbation** : il contient les blocs à moins de `r_c` = 4 m d'un bloc actif (`|η − repos|` > 1 mm, S396) et de l'**enveloppe
prévue** d'un objet — `p + V·t`, élargie de `½·a_max·t²`, jusqu'à l'horizon `√(2R/a_max)` (ADR-013 §2) ; un bloc que rien ne
requiert depuis 0,25 s (ADR-006 §4) est rendu au repos, et ce qu'il portait est publié. **Ce que la session ne fait pas** : le
stockage par blocs et son pool (la mémoire reste la fenêtre ; c'est la forme de la production, C8 au poste), les niveaux de `dx`
(rang 4), le bord non réfléchissant d'une partie.

**Critères, écrits avant.** (1) Sans ensemble, la suite **au bit** ; ensemble plein, 50 pas mobiles au bit du pas sans ensemble.
(2) Oracles indépendants — (a) un rectangle dans une fenêtre plus grande contre un domaine dense de ce rectangle, (b) deux
rectangles disjoints contre deux domaines denses : écart de surface **≤ 10 µm** sur 5 s pour une bosse de 5 cm (prédiction : à
l'arrondi du solveur, ≤ 1 µm) ; (c) un L : volume conservé au plancher de la cuve dense ; **vu échouer** avec une face du bord
laissée ouverte. (3) Une source mobile traverse un bassin ; le domaine épars qui la suit, contre le domaine entier : écart de
surface **≤ 3 mm** partout et à tout instant (tolérance d'image, I-12 ; prédiction : ≤ 1 mm, le seuil d'activité) ; la source
toujours dans l'ensemble. (4) Publiés : part des mailles de l'ensemble, volume rendu au repos, itérations, durée d'un pas. (5) La
prévision : pour des manœuvres tirées avec `|a|` ≤ `a_max`, la position à `t` ≤ `H` reste dans l'ensemble prévu, **100 %** ; au
banc, une source qui tourne y reste. (6) Suite entière, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'ensemble épars dans `Volume3` (`delta3d_sparse.rs`) : réserve du masque, `set_active_columns`, `open3` et `solid3`
  qui le voient, advection au bord de l'ensemble comme au bord de la boîte ; pas linéaire, couplé, gradué et `transplant`
  refusés ; critère 1.
- [x] **P3** — les oracles (a), (b), (c), et vu échouer ; critère 2.
- [x] **P4** — le suivi et la prévision (`domain_blocks.rs`) : blocs actifs, dilatation, enveloppe d'ADR-013 §2, libération après
  0,25 s ; sans allocation ; critère 5 (propriété de l'enveloppe).
- [x] **P5** — le banc `delta3d_epars` : une source mobile (dipôle de volume), droite puis qui tourne ; domaine entier contre
  domaine épars ; critères 3, 4 et 5 (banc).
- [ ] **P6** — les chiffres de réception en essais ; suite entière, zéro avertissement ; critère 6.
- [ ] **P7** — preuve `DOMAINE-EPARS-S401` ; liste (4.3, 4.9, 9.2), file, feuille de route, index.
- [ ] **P8** — rituel.

### Notes de reprise

- **P2** — `delta3d_sparse.rs` : `Sparse3` (deux masques de colonnes, réservés avant `seal()`), `enable_sparse`,
  `set_active_columns`, `set_active_blocks` (blocs du réseau commun coupés à la fenêtre), `SparseChange` publié (colonnes
  entrées et sorties, volume et hauteur rendus au repos, plus grande vitesse effacée). `open3` nul sur une face qui touche une
  colonne hors de l'ensemble ; `solid3` vrai dans ces colonnes — tout le pas mobile suit, multigrille comprise (`fine_kind3`,
  `coarsen3`). **Le bord de l'ensemble comme celui de la boîte** : une face qui ne touche aucune colonne de l'ensemble est
  « hors de la grille » (`sparse_outside3`) — l'advection y lit la face elle-même, le terme d'ADR-209 y omet la direction, et à
  un coin rentrant le terme croisé dont une diagonale est dehors. Refus (`Domain`) : pas linéaire, pas couplé, colonne graduée,
  `transplant`, surface écartée du repos ou source hors de l'ensemble ; `shift_rest` emmène les colonnes dehors. Critère 1 :
  ensemble plein, 50 pas **au bit**, itérations égales, avec et sans le terme d'ADR-209.
- **P3** — oracles (`tests_delta3d_sparse.rs`), bosse de 5 cm, 5 s à 20 ms : (a) rectangle 16 × 8 dans une fenêtre 32 × 24
  contre le dense 16 × 8 : **2,384·10⁻⁷ m** (un ulp de f32 à 2 m), Jacobi (13 203 itérations contre 13 209) et multigrille
  (1 539 contre 1 538) ; (b) deux rectangles 16 × 16 séparés de 8 colonnes, contre deux denses : **2,384·10⁻⁷ m**, l'écart au
  repos au bit ; (c) un L de blocs : dérive du volume **3,8·10⁻¹⁰ m³** (cuve dense 24 × 24 : 2,3·10⁻¹⁰). **Vu échouer** —
  défaut 1, faces `x` du bord laissées ouvertes : **aucun effet** (la colonne dehors est aussi solide : pression, correction et
  extrapolation l'ignorent — le mécanisme est doublé) ; défaut 2, faces `x` ouvertes **et** colonnes dehors non solides : l'eau
  passe d'un rectangle à l'autre par l'écart (repos cassé, essais (b) et de sortie), (a) tient — cul-de-sac, aucun débit — mais
  les itérations passent de 13 209 à 20 703 ; défaut 3, bord de l'advection lu à zéro au lieu de la face elle-même : (a)
  **1,36·10⁻⁴ m**, (b) **9,8·10⁻⁵ m** — au-dessus du critère : la règle du bord compte au dixième de millimètre en 5 s.
- **P4** — `domain_blocks.rs` : `useful_horizon` (ADR-013 §2, `√(2R/a_max)`, borné ; sa table retrouvée : 1,41 / 3,65 / 4,90 s),
  `Tracked` (position, vitesse, `a_max`, horizon, rayon), `Follow` (réservé avant `seal()` : un instant et deux octets par
  bloc). Mise à jour : blocs **marqués** — une colonne à plus de 1 mm du repos, ou touchée par l'enveloppe (disques de centre
  `p + V·t` et de rayon `r + ½·a_max·t²`, intervalles d'un demi-bloc de trajet, chaque disque élargi du demi-trajet) —, dilatés
  de `r_c` = 2 blocs (Chebyshev, séparable), **requis** ; un bloc que rien ne requiert depuis 0,25 s sort. **Critère 5 tenu** :
  300 manœuvres tirées (`a_max` 0,5 / 2 / 5 m/s², vitesse jusqu'à 10 m/s, horizon `√(2·4 m/a_max)` borné à 5 s, accélération
  constante par 0,1 s), **1 764 931 blocs vérifiés, 100 %** dans le domaine prévu, marge de `r_c` comprise ; 114 blocs par objet
  en moyenne (sur 1 024). **Vu échouer** sans le terme `½·a_max·t²` : essai 22, `t` = 1,76 s, bloc hors du domaine.
- **P5** — `examples/delta3d_epars.rs` : bassin 40 × 20 m à 25 cm, 2 m d'eau, 20 ms, multigrille ; source = dipôle de volume
  (0,05 m³/s, ±0,5 m, gaussiennes σ 0,5 m tronquées à 1,5 m, débit monté en 1 s) ; domaine entier contre domaine épars
  (`Follow` : 4 m, 1 mm, 0,25 s ; enveloppe `a_max`, horizon `√(2·4/a_max)` ≤ 5 s). Quatre calculs en parallèle (durées
  indicatives). Résultats (écart max partout ; amplitude ; part des mailles moy / max ; rendu au repos ; pas entier / épars) :
  droite 2 m/s 10 s, prévision 2,83 s : **0,26 mm** ; 24,2 mm ; 0,713 / 0,880 ; −3,1·10⁻³ m³, 0,23 mm ; 387 / 409 ms. Sans
  prévision : **0,56 mm** ; 0,416 / 0,635 ; 389 / 261 ms. Virage (0,5 m/s², `a_max` 1) 8 s : **0,63 mm** ; 24,7 mm ; 0,533 /
  0,605 ; 0,72 mm rendus. Rapide 10 m/s 3 s, cadence 0,5 s, prévision 2 s : **0,66 mm** ; **2,6 mm** d'amplitude (25 %) ;
  0,675 / 0,870 ; 601 / 626 ms. **Témoin sans prévision : la source ne sort pas** — 0,66 mm aussi, 0,257 / 0,300, 488 / 218
  ms : à 10 m/s et 0,5 s, la marge de la dilatation (≥ 6 m devant le centre) suffit ; l'enveloppe coûte 2,6 fois les mailles
  sans rien changer à l'écart. **À 1 s de cadence** (`EPARS_CADENCE_S=1`) : **sans prévision, la source sort de l'ensemble à
  0,62 s** (vu échouer) ; avec, elle reste dedans 3 s — **0,63 mm**, 0,713 / 0,830, 466 / 486 ms. Critère 3 tenu dans tous les
  cas où la source reste dedans ; critère 5 au banc tenu (virage ; rapide à 1 s). **Constat** : l'enveloppe ne sert que si
  `V·cadence` dépasse la marge de la dilatation (≈ 6 m) ; sinon elle coûte des mailles calculées sans gain — ADR-013 §2 la
  range au palier T2 (blocs réservés, δ à 0), pas au calcul (T1) : à séparer (file). Petit essai (ignoré, 32 × 8 m, 3 s,
  balistique, horizon 1 s) : 4,2·10⁻⁵ m, 20 mm, part 0,47.
