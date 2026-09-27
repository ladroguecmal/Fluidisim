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

Session : S404 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Continue »*. Suite proposée par S403 : **C8e**, l'épars et
les niveaux sous le pas couplé ; file, « Le domaine épars — ce que S401 laisse » : *le pas couplé sous l'ensemble : avant C10*.
Agent : Claude, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot. Branche
`claude/eager-volta-lf0kw3`, la plus avancée (`main`, `poste` et `claude/blissful-pasteur-m4j5rn` en sont ancêtres).

**Thèse.** La scène de C10 — le joueur qui saute, la gerbe d'étrave — est δ **sous B + W** : son domaine doit suivre l'objet en
ensemble épars et changer de niveau au rang 4. S401 (l'ensemble) et S402 (le transfert) refusent le pas couplé. Au pas couplé, le
bord de la boîte n'est pas qu'un mur : B y est prescrit, sa bande y passe, et l'éponge d'ADR-164 y absorbe δ. « Le bord de
l'ensemble se comporte comme le bord de la boîte » veut donc dire : δ fermé aux murs de l'ensemble, la bande de B comptée aux murs
comme au bord, l'éponge mesurée depuis le bord de l'ensemble — qui devient **absorbant** (S401 : « les murs de l'ensemble
réfléchissent »). Hors de l'ensemble, δ nul est le point fixe du pas **relatif** (ADR-198 D1, le mode de la production) ; le pas
de S297 donne à δ les restes de B partout, ce qu'un ensemble ne peut pas porter : refusé. Le transfert de niveau porte l'ensemble
de la même façon : le bord de l'ensemble de départ comme le bord de la boîte, rien dehors, les murs d'arrivée fermés ; le volume
exact quand l'arrivée couvre le départ, publié sinon. B seul au banc ; W s'y somme de la même façon (`BackgroundFaces3`).

**Critères, écrits avant.** (1) Sans ensemble, la suite au bit ; ensemble plein sous le pas couplé relatif, 50 pas au bit du pas
sans ensemble, avec et sans le terme d'ADR-209. (2) Sous une houle réelle (B, relatif) : (a) un rectangle dans une fenêtre contre
le dense de ce rectangle, mêmes échantillons de B, éponge sur les deux axes, ≤ 10 µm sur 5 s (prédiction ≤ 1 µm : l'ordre des
sommes) ; (b) deux rectangles séparés contre deux denses, l'écart au repos au bit ; (c) le bilan fermé au plancher du dense, les
murs comptés au bord ; vu échouer : la bande des murs en face intérieure, l'éponge depuis le bord de la boîte, les faces fermées
prédites ; refus du pas de S297 avec un ensemble. (3) Transfert d'un rectangle 25 → 50 → 25 cm contre le dense du rectangle :
≤ 1 µm (prédiction : la surface au bit, `dx` dyadiques) ; volume exact au plancher quand l'arrivée couvre, la perte publiée sinon.
(4) Banc en mer (houle de 5 cm, λ ≈ 16 m) : une source à 2 m/s, l'épars qui suit (`Follow` de S401) à ≤ 3 mm du domaine entier
partout et toujours (prédiction ≤ 1 mm), la source dedans ; la part des mailles au cours du temps — prédiction : l'ensemble se
vide derrière l'objet, part finale ≤ 0,6 (S401, bassin fermé : 0,85) ; témoin aux murs réfléchissants (éponge au seul bord de la
boîte) pour l'attribuer ; un cas long, 30 s (L369). (5) Niveaux sous la houle : l'épars qui suit passe 25 → 50 cm à 3 s et revient
à 6 s, contre le dense qui fait les mêmes passages : ≤ 3 mm (prédiction ≤ 1 mm) ; sauts aux passages ≤ 3 mm ; volume des
transferts publié ; l'écart au domaine fin publié (le prix du contenu, ADR-210 D2). (6) Suite entière, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le pas couplé porte l'ensemble (`delta3d_coupling.rs`, `delta3d_sparse.rs`) : mode relatif exigé ; faces fermées non
  prédites, gradients au bord de l'ensemble comme au bord de la boîte ; bande de B aux murs comme au bord, colonnes dehors
  intouchées ; éponge depuis le bord de l'ensemble (étendues par colonne, réservées) ; bilan aux murs.
- [x] **P3** — essais du pas couplé : critère 1, oracles (a) (b) (c) sous la houle, refus ; vu échouer (trois défauts injectés).
- [x] **P4** — le transfert de niveau porte l'ensemble (`delta3d_levels.rs`) ; `Follow::require_cover` ; essais du critère 3.
- [x] **P5** — le banc `delta3d_mer_epars` : cas `suivi`, `murs` (témoin), `long`, `niveaux` ; lancés en arrière-plan.
- [>] **P6** — les calculs du banc ; critères 4 et 5.
- [ ] **P7** — suite entière, zéro avertissement ; critère 6.
- [ ] **P8** — preuve `MER-EPARS-S404` ; liste (4.3, 4.7), file, feuille de route, index ; notes datées d'ADR-006 et d'ADR-210.
- [ ] **P9** — rituel.

### Notes de reprise
- **P2** — `delta3d_coupling.rs` : faces fermées par l'ensemble non prédites ; gradients de la prédiction, voisin hors de la grille
  de l'ensemble lu comme la face (règle d'advection de S401) ; transport : un mur lu comme le bord de la boîte du côté de sa
  colonne (surface de la colonne + η de B à la face, surface de B `repos + η`), colonnes dehors intouchées ; éponge
  `sponge_factor3` : rampes mesurées dans l'**étendue** de la colonne (`Sparse3::extent`, `[i0, i1, j0, j1]`, réservée,
  recalculée à chaque changement), la plus forte des deux colonnes d'une face ; `Sponge3::ramp`/`from_ramps` extraits de `factor`
  au bit ; bilan : les murs au bord, signés vers l'ensemble. Refus : ensemble sans les trois bits relatifs. Témoin d'essai
  `set_sparse_edge_sponge_for_trials(false)` : l'éponge au seul bord de la boîte. **104 essais delta3d tenus** tels quels (S297 à
  S402), zéro avertissement.
- **P3** — `tests_delta3d_sparse_mer.rs`, cinq essais `_s404`, houle de S369 oblique (36°), relatif, éponge 1 × 0,5 m à 2 s⁻¹,
  bosse de 5 cm, 5 s. **Critère 1 tenu** : ensemble plein, 50 pas au bit, avec et sans ADR-209, bilans égaux. **Critère 2 tenu** :
  (a) rectangle 16 × 8 dans 32 × 24 contre le dense (B échantillonnée aux mêmes points, au bit) : **2,384·10⁻⁷ m**, un ulp à 2 m,
  Jacobi et multigrille ; (c) bilan, résidu/échelle 9,0·10⁻⁸ (épars) contre 7,7·10⁻⁸ (dense), 6,1 contre 6,4·10⁻⁸ en multigrille ;
  bandes entrées à 1,3–1,7·10⁻⁷ relatif ; δ ne traverse rien (`perturbation_in` nul) ; dehors au repos au bit ; (b) deux
  rectangles : 2,384·10⁻⁷ m ; refus : pas de S297, deux bits sur trois. **Vu échouer** : (1) bande des murs en face intérieure :
  **8,4·10⁻⁴ m**, bandes à 10,7 % ; (2) témoin, éponge au seul bord de la boîte : **3,2·10⁻² m** (l'éponge du dense couvre une
  bonne part du rectangle) ; (3) faces fermées prédites : **2,6·10⁻⁴ m**, et le dehors n'est plus au repos ; (4) gradients qui
  lisent le dehors à zéro : **2,8·10⁻⁴ m**. Tous retirés.
- **P4** — `delta3d_levels.rs` : les refus d'ensemble levés (la découpe reste refusée) ; départ : une colonne dehors porte le
  repos, sans pente ; pente décentrée et terme croisé nul au bord de l'ensemble ; vitesses : un coin hors de la grille de
  l'ensemble prend, axe par axe, la valeur de son voisin dedans (`staggered_in_set`, la borne de la boîte) ; arrivée : colonnes
  dehors au repos, murs fermés. `Follow::require_cover` (`domain_blocks.rs`) : les blocs qui couvrent l'ensemble d'un autre niveau,
  requis à l'instant. **Critère 3 tenu** : un rectangle 25 → 50 → 25 cm contre son dense, surface et vitesses **au bit** (0,000),
  volumes exacts ; une arrivée qui ne couvre que la moitié perd exactement ce que portait l'autre (1,039676·10⁻² m³), publié ; un
  dense reçoit tout d'un épars. **Trouvé par l'essai** : une colonne de départ dehors ne doit pas « ne rien porter » mais porter
  le **repos** — la reconstruction est en hauteur absolue ; sans cela, un dense recevait −79,9 m³. L'essai S402 qui refusait un
  ensemble ne le refuse plus (son commentaire le dit) ; les chiffres de S402 inchangés (0,304 %).
- **P5** — `examples/delta3d_mer_epars.rs` : fenêtre 32 × 16 m à 25 cm (128 × 64 × 12 ; 64 × 32 × 6 à 50 cm), houle de S369
  oblique, relatif, éponge 2 m à 4 s⁻¹ (`10·c_g/largeur`, S315), dipôle de S401, `Follow` (4 m, 1 mm, 0,25 s, prévision
  `a_max` 1). Cas `suivi`, `murs`, `long` (0,8 m/s, 30 s), `niveaux` (3 s / 6 s) ; `MER_DUREE_S` pour la fumée. Fumée : 0,4 s de
  `suivi`, écart nul, ≈ 330 ms par pas ; 3,2 s de `niveaux` : saut au passage 2,04 mm (entier et épars), épars contre entier
  0,12 mm, volumes exacts. Lancés à 10:04 en parallèle (quatre cœurs), journaux `mer_<cas>.log` dans le brouillon ; durée
  attendue ≈ 10 min (`suivi`, `murs`, `niveaux`), ≈ 25 min (`long`).
- **P6** (en cours) — `suivi` : écart max **0,138 mm** (dans l'ensemble 0,108), amplitude 25,9 mm, source dedans ; part moyenne
  0,830, max 0,875, toutes les 2 s 0,734/0,852/0,875/0,875/0,812 ; rendu au repos −7,7·10⁻⁴ m³, 0,12 mm au plus ; 6,6
  itérations ; 325/356 ms par pas. `murs` (témoin) : écart **0,210 mm**, part **identique** au millième, rendu 0,29 mm au plus.
  **Critère 4** : écart tenu (prédiction ≤ 1 mm tenue) ; la prédiction « part finale ≤ 0,6 » **manquée** (0,812) — et le témoin
  dit que le bord absorbant n'y change rien : il ôte un tiers de l'écart (0,21 → 0,14 mm), pas des mailles.
