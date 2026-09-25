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

Session : S358 — **en cours**. **La physique (ADR-191 D3) : la coque dans la production de δ, 1 — `Linear3`, le pas
linéaire de la porte D sur la carte.** Le plan de S355 (commit `a2c81dea`), reporté tel quel, plus le solide fixe.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.
Entrée — *« Reprends le projet »*, sans verdict R19 : la physique en attendant (S357). Liste 6.4, *manque la production
GPU*. **Constat de S355** : la coque a été reçue dans le **mode linéaire** de δ (couvercle, faces coupées, couvercle
partiel, S330–S337) ; la production est le pas **mobile couplé**, qui n'a de solide nulle part. **Chemin** (ADR à écrire
sur les mesures) : le domaine δ d'une coque est un domaine **linéaire** porté sur la carte ; le pas mobile couplé garde
les autres. Découpage : ici le pas à ouvertures, toutes ouvertes **puis celles d'un solide fixe** (6.5) ; ensuite la
coque qui bouge et perce le couvercle ; puis la scène.

Critères, écrits avant le code :
1. **`Linear3` sur la carte** : le pas de `Volume3::step_surface_linear` — prédit = courant, éponge sur les vitesses
   prédites, divergence pondérée par les ouvertures, terme du couvercle, opérateur pondéré (couvercle à demi-maille),
   gradient conjugué de Jacobi à **cycles fixes repartant du pas précédent**, correction entre mailles fluides, flux de
   colonne pondérés, hauteur compensée (différence exacte de S301), rappel de l'éponge. Ouvertures et fractions
   **réservées à la création** (I-06), toutes ouvertes ou données par le cœur ; aucune relecture dans le pas.
2. **Contre la référence** : bosse gaussienne de 10 cm (σ = 1 m) au centre, grille de la porte D (96 × 96 × 8, 25 cm,
   2 m), pas de 10 ms, 200 pas, éponge de 3 m à 2,5 /s : **|Δη| ≤ 10⁻⁴ m** sur toutes les colonnes, relevé tous les
   vingt pas. Sans éponge, le volume de la carte ne dérive pas de plus de **10⁻⁶** du volume initial en 200 pas.
   **2 bis — solide fixe** : sphère de 0,5 m centrée à 1 m de profondeur sous la bosse, découpe du cœur
   (`configure_with_solid`) chargée telle quelle : même critère ; faces fermées à vitesse nulle, exactement.
   **Prédiction** : 32 cycles au plus suffisent, départ chaud.
3. **Le coût** sur cette grille : p50 et p99 du pas horodaté, au plus petit nombre de cycles qui tient 2 et 2 bis ;
   alimentation relevée (A270) ; techniques présentes et absentes (ADR-131). Prédiction : sous 2 ms (ADR-174 D3).
4. ADR-193 (le domaine d'une coque est linéaire, sur la carte), preuve ouverte par « Reproduire », file, feuille de
   route, liste 6.4 et 6.5.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `delta3d_linear.wgsl` : les noyaux du pas et du gradient conjugué à ouvertures.
- [x] **P3** — `delta3d_linear.rs` : `Linear3`, tampons, encodage du pas, relectures de banc ; critère 1.
- [x] **P4** — le banc contre `Volume3`, bosse, avec et sans éponge, balayage des cycles ; critère 2.
- [ ] **P4 bis** *(ajoutée en cours, après P3)* — le même défaut de publication dans la production `Step3`
  (`published`, `ghost_up`) : la dérive de la moyenne de S305 §7, mesurée avant et après sur la cuve.
- [x] **P5** — le solide fixe immergé ; critère 2 bis.
- [ ] **P6** — le coût ; critère 3.
- [ ] **P7** — ADR-193, preuve, file, feuille de route, liste ; critère 4.
- [ ] **P8** — rituel.

### Notes de reprise
- **P2–P3.** `viewer/src/delta3d_linear.{wgsl,rs}` : quinze noyaux, un seul groupe de liaison (vitesses courantes et
  prédites, géométrie `[ouvertures | fractions]`, colonnes `[η | reste | flux x | flux y | publiée]`, sept tranches du
  gradient conjugué, partiels, scalaires, uniforme). `toutes_ouvertes` : murs et fond fermés, la convention de `Cut3`.
  Couvercle partiellement fermé refusé à la création. Dispatchs : `10 + 5·cycles`, une passe. Bancs : `--lineaire-carte
  [--solide] [--sans-eponge] [--cycles=…]`, `--lineaire-cout`, `--lineaire-avance` (instrument). Zéro avertissement.
- **Défaut trouvé au premier passage, corrigé.** Sans éponge, volume publié de la carte **+4,6·10⁻⁵** en 200 pas, le
  cœur **exact** (0,314159274 = v0 par `surface_roundoff_for_trials`). Instrument (`--lineaire-avance`) : l'avance
  rejouée en f32 depuis les entrées de la carte — **hauteur au bit sur 9 216 colonnes**, reste = la version à produit
  fusionné ; la somme vraie `Σ(η − z₀) − Σ reste` tient à 10⁻⁹ m, la **somme publiée** saute de 3,4·10⁻⁵ m dès le pas 1.
  Cause : `(η − z₀) − reste` réassocié par le compilateur, le reste perdu dans l'ulp de `z₀` (famille L345). Remède :
  `difference(η, z₀)` en entiers (Sterbenz), dans la publication **et le couvercle**. Après : dérive 6·10⁻¹⁰ à 4·10⁻⁹.
  **La production `Step3` a le même motif** (`published`, `ghost_up`) — c'est la dérive de la moyenne de S305 §7.
- **P4, critère 2 tenu.** Cœur : 105,1 itérations de gradient conjugué par pas, départ froid, 17–18 s les 200 pas.
  Éponge — pire |Δη| sur les dix relevés : **4 cycles 1,74·10⁻⁴ m (manqué)**, 8 cycles 5,67·10⁻⁵, 16 cycles 2,18·10⁻⁵,
  32 cycles 7,2·10⁻⁷, 64 cycles 2,38·10⁻⁷ (l'ulp de 2 m) ; résidu relatif 4,2·10⁻³ à 8 cycles, 2,2·10⁻⁶ à 64 ; volume
  final 0,282982947 m³ contre 0,282982986 au cœur (64 cycles). Sans éponge : mêmes écarts ; dérive du volume publié
  **6·10⁻¹⁰ à 4·10⁻⁹** pour 10⁻⁶ exigé (après correction). Aucune face fermée non nulle. En usage : 5,7·10⁻⁵ m à 8
  cycles, cinquante fois sous les 3 mm de l'image (S201).
- **P5, critère 2 bis tenu.** Sphère de 0,5 m à 1 m sous le couvercle, découpe du cœur chargée telle quelle
  (12 324 faces fermées contre 12 288) ; cœur 104,4 itérations (Jacobi), 40 s. Pire |Δη| : 4 cycles 2,33·10⁻⁴, **8
  cycles 1,09·10⁻⁴ (manqué de peu)**, **16 cycles 1,26·10⁻⁵**, 32 cycles 1,07·10⁻⁶, 64 cycles 2,38·10⁻⁷ ; faces fermées
  à vitesse nulle, exactement ; volume 0,283311980 contre 0,283312029. **Témoin privé de la découpe** (`--temoin` : la
  carte toutes ouvertes contre le cœur avec la sphère) : **4,68·10⁻³ m** — le banc voit le solide, vingt mille fois
  au-dessus de l'écart qu'il mesure avec. Prédiction « 32 cycles au plus » tenue : **16** pour les deux cas.
