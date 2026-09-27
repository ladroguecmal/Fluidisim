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

Session : S409 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Reprends le projet »* ; au poste. Suite désignée
au poste par S408 : **C3b** (conception S384 §5, C3 : « δ ≤ 2 ms au 99ᵉ centile sur la scène de la porte B, **puis à 10 cm**
sur une scène de même surface ; A298 remesurée »). Agent : Claude Code (Opus 5.5), application de bureau, **au poste** — fichiers,
git, cargo, Python, **RTX 5070 Laptop** ; Godot non utilisé. Branche `poste` (= `main` = `claude/eager-volta-lf0kw3`).

**Ce que « même surface » ne peut pas vouloir dire** (calcul, avant toute mesure) : la scène de la porte B (30 × 28 m, boîte de
7 m) à 10 cm, c'est 300 × 280 × 70 = **5,9 M mailles**, 17,8 M faces — le tampon des faces (10 flottants) passe la liaison de
128 Mio vers 3,4 M faces (≈ 1,1 M mailles), et le coût par maille de S350 (3,57 ms pour 376 320) donnerait ≈ 56 ms par pas.
Le critère de S384 reposait sur les colonnes hautes (14 m de côté à 10 cm), que S386–S387 ont réservées à l'eau calme. **Lu
ici** : la même mer, la même boîte verticale, une perturbation à la même pente, sur l'**emprise que le budget permet** ; la
mesure dit laquelle.

**Thèse.** À 10 cm, Jacobi rampe (son taux par itération se dégrade avec `N`) et la multigrille, indépendante de la maille (C1),
doit payer davantage qu'à 25 cm (projection ÷ 1,9 en S390). Une scène à 10 cm tient-elle deux minutes (L369), à 30 Hz, avec le
terme de second ordre d'ADR-209 — dont le nombre de Courant, lui, croît de 2,5 ?

**Critères, écrits avant.** (1) Sans les variables nouvelles, au bit : `--delta3d-empreinte` inchangé (60 et 600 pas). (2) À
10 cm, boîte de 7,2 m (`nz` = 72 : trois niveaux grossiers), deux emprises au moins : la multigrille atteint le **résidu médian
de Jacobi-32 à 25 cm** (7,1·10⁻⁵) en **≤ 8 cycles** ; *prédiction* : Jacobi-32 y est ≥ 5 fois moins bon qu'à 25 cm, la
multigrille aux mêmes cycles qu'à 25 cm. (3) À résidu égal, pas multigrille ≤ Jacobi, gain de projection **> 1,9** (*prédit*).
(4) Durée d'usage : la scène à 10 cm tient **deux minutes** à 30 Hz avec la multigrille retenue — sinon à 60 Hz, et l'on
nomme ce qui casse (témoin, termes éteints un à un, L136). (5) Budget : l'**emprise carrée la plus grande** à 10 cm dont les
deux parts de 30 Hz tiennent **≤ 2 ms au 99ᵉ centile**, publiée avec la loi coût/emprise ; *prédiction* ≈ 0,45 M mailles,
≈ 8 m de côté. (6) **A298** sur le pas retenu (multigrille, terme d'ADR-209 ; cuve fermée) : l'écart carte/référence au pas
d'usage (33,333 ms) sur **deux minutes**, et au pas de S305 (1 ms, 5 s) pour comparaison ; **close** si l'écart à deux
minutes ≤ 3 mm **et** la pente extrapolée franchit 3 mm après plus d'une heure ; sinon ouverte, pente publiée. (7) Suite de
l'afficheur, zéro avertissement. **Arrêt** : si la scène à 10 cm explose aux deux cadences, publier et ne rien changer aux
défauts ; la multigrille ne devient défaut que pour la scène à 10 cm, si elle y est nécessaire.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la scène à une maille et une emprise données (`MAILLE=`, `EMPRISE=`) : boîte de 7,2 m, éponge en mailles, un
  impact à la pente de celui de R16 et à l'échelle de l'emprise ; sans variable, la scène de S390 au bit ; critère 1.
- [x] **P3** — qualité à 10 cm : Jacobi 32/64/128 contre multigrille 4/6/8, deux emprises (300 pas) ; critère 2.
- [x] **P4** — coût à 10 cm, pas entier et deux parts ; l'emprise la plus grande sous 2 ms ; critères 3 et 5.
- [x] **P5** — deux minutes à 10 cm, 30 Hz puis 60 Hz si besoin ; critère 4.
- [x] **P5b** — *ajoutée* : 30 Hz explose à 10 cm quel que soit le solveur ; attribution sur le banc d'A321 (`MAILLE=`, `EMPRISE=`), termes éteints un à un (L136).
- [>] **P5c** — *ajoutée* : à cadence stable (60 Hz, un pas par image), l'emprise à 10 cm sous 2 ms ; deux minutes.
- [ ] **P6** — A298 : `longue_cuve` paramétrée (`PAS_US`, `PAS`, `MULTIGRILLE`) ; 1 ms × 5 000 et 33,333 ms × 3 600 ; critère 6.
- [ ] **P7** — suite de l'afficheur (et du cœur si touché), zéro avertissement ; critère 7.
- [ ] **P8** — preuve : MULTIGRILLE-3D-S385 §6 (un fil, une preuve) ; liste 4.19, file, feuille de route, index ; A298.
- [ ] **P9** — rituel.

### Notes de reprise
- **P2** — `Config::at_mesh(dx, nx, ny)` (`delta3d_scene.rs`) : la scène de R11 à l'échelle `s = nx·dx / 30 m` — paquet
  (amplitude, longueur, écarts, place ; cambrure gardée), éponge ; boîte `nz` = arrondi de 7 m / `dx` au multiple de 4 (72 à
  10 cm : trois niveaux grossiers) ; repos à 3,5 m. **Écart au plan** : le **paquet** mis à l'échelle et non un impact — S390
  a mesuré la multigrille sur le paquet de `review`, la comparaison reste à scène égale. `review_from_env` : `MAILLE=`,
  `EMPRISE=nx,ny`, refus au-delà de 128 Mio de faces ; branché sur `--delta3d-mg-scene` (ligne `MG_SCENE_S409`). Essai
  `_s409` : `at_mesh(0,25, 120, 112)` = `review` ; à 10 cm, 72 couches, trois niveaux, cambrure à 10⁻⁶. **Critère 1 tenu** :
  `--delta3d-empreinte` avant (tête `0194a572`) et après, identiques (60 pas : surface `0xacd172ae252fe5a6` ; 600 :
  `0x28f35d9d7580ffed`). Essai à 10 cm, 64 × 64 (6,4 m, 294 912 mailles), 60 pas : Jacobi-32 résidu médian 3,9·10⁻⁴,
  multigrille 6 : 1,4·10⁻⁴ ; rien n'explose.
- **P3** — `MAILLE=0.1 EMPRISE=n,n COUT=0 … --delta3d-mg-scene`, 300 pas à 30 Hz ; témoin à scène égale : 25 cm sur 8 m
  (32 × 32 × 28). Résidu relatif **médian** (max) :

  | variante | 6,4 m (294 912) | 8 m (460 800) | 9,6 m (663 552) | 8 m à 25 cm | S390, 30 × 28 m à 25 cm |
  |---|---|---|---|---|---|
  | Jacobi 32 | 3,06·10⁻⁴ | 2,41·10⁻⁴ | 2,15·10⁻⁴ | 9,9·10⁻⁵ | 7,1·10⁻⁵ |
  | Jacobi 64 | 6,7·10⁻⁵ | 5,5·10⁻⁵ | 5,1·10⁻⁵ | 8,9·10⁻⁶ | 7,4·10⁻⁶ |
  | Jacobi 128 | 8,5·10⁻⁶ | 6,4·10⁻⁶ | 6,3·10⁻⁶ | 2,7·10⁻⁷ (plancher) | — |
  | mg 4 | 4,3·10⁻⁴ | 5,0·10⁻⁴ | 4,6·10⁻⁴ | 1,3·10⁻⁴ | 2,0·10⁻⁴ |
  | mg 6 | 1,29·10⁻⁴ | 1,13·10⁻⁴ | 1,08·10⁻⁴ | 2,0·10⁻⁵ | 4,9·10⁻⁵ |
  | **mg 8** | **2,3·10⁻⁵** | **2,2·10⁻⁵** | **2,4·10⁻⁵** | 4,2·10⁻⁶ | 1,2·10⁻⁵ |

  mg 1 et 2 explosent avant le pas 30 à 10 cm (mg 2 au pas 300 à 6,4 m) ; mg 3 et plus, Jacobi 8 et plus : 300 pas tenus.
  Écart entre les deux références (mg 24, Jacobi 512) : 0,7 à 44 mm aux points de contrôle — A297, la surface ne juge rien.
  **Critère 2 tenu à 8 cycles** (≤ 8) : 2,2 à 2,4·10⁻⁵ pour 7,1·10⁻⁵, **indépendant de l'emprise** ; mg 6 ne suffit plus
  (1,1·10⁻⁴). **Prédictions manquées, les deux** : Jacobi-32 est 3,0 à 4,3 fois moins bon qu'à 25 cm (S390), pas ≥ 5 — 2,4 fois
  à scène égale ; la multigrille demande **deux cycles de plus** qu'à 25 cm. **Lecture** (à scène égale, 8 m) : le **taux par
  cycle** de la multigrille ne bouge pas (≈ 0,45 : 5,0 → 1,1 → 0,22·10⁻⁴ à 10 cm ; 1,3 → 0,20 → 0,042·10⁻⁴ à 25 cm) — c'est son
  **point de départ** qui est ≈ 5 fois plus haut ; celui de Jacobi, lui, se dégrade (×0,23 par 32 itérations contre ×0,09). À
  résidu égal : Jacobi 64 (337 dispatchs) ≈ mg 7 ; Jacobi 128 (657) < mg 8 (286). Le coût, en P4, tranche.
- **P4** — `MAILLE=0.1 EMPRISE=n,n PAS=30 REFERENCES=0 VARIANTES=jacobi32,jacobi64,jacobi128,mg6,mg8 MG_CYCLES=8` ; le banc
  de coût suit désormais `VARIANTES=` quand elle est donnée (sans elle, la liste de S390). Chaque pas soumis seul et attendu,
  200 pas horodatés, secteur. **q99, ms** — projection / pas entier :

  | emprise (mailles) | Jacobi 32 | Jacobi 64 | Jacobi 128 | mg 6 | **mg 8** | deux parts mg 8, meilleur `k` |
  |---|---|---|---|---|---|---|
  | 6,4 m (294 912) | 1,62 / 2,96 | 3,17 / 4,51 | 6,28 / 7,60 | 1,01 / 2,32 | 1,30 / **2,62** | `k`=1 : 1,39 / 1,28 |
  | **8 m (460 800)** | 2,48 / 4,57 | 4,84 / 6,90 | 9,60 / 11,67 | 1,34 / 3,41 | 1,73 / **3,81** | **`k`=0 : 1,89 / 1,96** |
  | 9,6 m (663 552) | 3,47 / 6,43 | 6,77 / 9,71 | 13,47 / 16,43 | 1,73 / 4,67 | 2,23 / **5,18** | `k`=0 : 2,70 / 2,55 |
  | 11,2 m (903 168) | 4,73 / 8,73 | 9,23 / 13,22 | 18,35 / 22,37 | 2,28 / 6,29 | 2,93 / **6,94** | `k`=0 : 3,64 / 3,33 |

  **Loi** (mg 8, pas entier q99) : **0,53 ms + 7,1 ns par maille** (6,4 et 11,2 m ; 8 et 9,6 m à 0,06 ms). Un cycle mg ≈ 0,19 ms à
  8 m = 2,5 itérations de Jacobi (0,074) — le rapport de S390. **Critère 5 tenu** : **8 m × 8 m**, 460 800 mailles, deux parts
  **1,89 / 1,96 ms** (`k` = 0 : à 10 cm, fond, prédiction et correction pèsent ≈ 2,1 ms, toute la projection va dans la seconde
  part) — à 2 % de la limite ; 9,6 m la dépasse (2,70). *Prédiction ≈ 0,45 M mailles, ≈ 8 m : tenue.* 8,8 m (88, trois niveaux) :
  ≈ 2,25 ms par part par la loi, non mesuré. **Critère 3 tenu** : à 8 m, mg 8 (2,2·10⁻⁵) est encadrée par Jacobi 64 (5,5·10⁻⁵,
  moins bon) et 128 (6,4·10⁻⁶) : projection **1,73 contre 4,84 ms** (÷ 2,8) au moins, ≈ ÷ 4,2 contre Jacobi ≈ 96 interpolé —
  plus que ÷ 1,9 à 25 cm (*prédit > 1,9 : tenu*) ; pas 3,81 contre 6,90. Même contre Jacobi-32, la production, 11 fois moins
  précise : 1,73 contre 2,48.
- **P5** — 8 m à 10 cm, `PAS=3600 COUT=0 REFERENCES=0`. **30 Hz : Jacobi-32, mg 6 et mg 8 explosent tous trois entre les pas
  1 831 et 1 860** (≈ 61–62 s ; contrôle toutes les 30 pas) — résidus médians 2,3·10⁻⁴, 1,2·10⁻⁴, 2,2·10⁻⁵ : **la pression
  n'est pas en cause**, le motif d'A321 avant ADR-209. **60 Hz** (`PAS_US=16667 PAS=7200`), mg 8 : **deux minutes tenues**,
  résidu médian 1,4·10⁻⁵, divergence franche médiane 2,1·10⁻⁴. Critère 4 : **tenu à 60 Hz, manqué à 30 Hz**. Hypothèse avant
  mesure : le terme de second ordre (Lax-Wendroff, `V = U + u'`) a une limite de Courant ; à 10 cm, `V·dt/dx` est 2,5 fois
  celui de 25 cm (≈ 0,5 par axe sous la houle de `Hs` 2,5 m à 30 Hz). À éprouver : témoin, termes éteints un à un, 25 ms.
- **P5b** — `--delta3d-a321` branché sur `review_from_env` (`MAILLE=0.1 EMPRISE=80,80 MULTIGRILLE=1 CYCLES=8 SECONDES=90`),
  30 Hz. Explosion (non fini) ou tenue :

  | variante | issue | | variante | issue |
  |---|---|---|---|---|
  | témoin | **62 s** (pas 1 860) | | sans éponge | 44 s |
  | sans terme d'ADR-209 (64) | **7 s** | | sans bande de B (16) | 82 s |
  | sans `u'·∇u'` (1) | **tient 90 s** | | sans paquet | **62 s, même pas** |
  | sans `U·∇u'` (2) ; sans `u'·∇U` (4) | 62 s, même pas | | sans résidu du fond (8) | **tient 90 s** |
  | pas de **25 ms** | **tient 90 s** | | pas de 16,7 ms (P5, `mg-scene`) | tient 120 s |

  **Faits** : l'explosion est brutale (témoin : `max_u` 1,16 m/s à 60 s, 8,3 à 61 s ; part de l'échelle de la maille de `w`
  0,03 → 0,18) ; ni la pression, ni le paquet, ni `U·∇u'`, ni `u'·∇U` n'en changent l'instant ; le schéma d'avant ADR-209 explose
  en 7 s ; deux termes la suppriment chacun — l'auto-advection de δ et le résidu de quantité de mouvement du fond qui la
  nourrit ; la cadence la supprime dès 25 ms. δ porte 1 à 2 m/s par endroits (échantillons à la seconde ; témoin : 1,92 avant
  55 s). **Explication — hypothèse, non démontrée** : la limite de Courant du terme de second ordre, `V = U + u'` — à 10 cm et
  33 ms, 1 m/s vaut déjà 0,33 maille par pas ; « sans résidu du fond » tient pourtant avec des échantillons à 2,7 m/s : la
  vitesse seule ne suffit pas à l'expliquer. **Angle mort nouveau** (sévérité 2) : à 10 cm, 30 Hz n'est pas stable sur la
  minute. Piège de banc : `sans_uu`, `sans_Uu`, `sans_uU` — un seul fichier sous Windows ; relancés sous `c1`, `c2`, `c4`.
