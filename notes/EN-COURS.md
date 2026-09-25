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

Session : S361 — **en cours**. **Rendu 5 : les caustiques** — *« Tente les caustique »* (R20, question 3).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.
Entrée — S360 a rendu la surface fine par FFT ; R21 attendu. **Physique** : le soleil réfracté par la surface éclaire le
fond selon la projection `X_f = X_s + (H + η)·p(∇η(X_s))`, `p` la pente horizontale du rayon réfracté (Snell exact) ;
l'éclairement direct est multiplié par `1/|det(I + (H + η)·∂p/∂s·Hess η)|` au point de départ (conservation de
l'énergie, optique géométrique). Le point de départ se retrouve par **point fixe** depuis le point du fond. Surfaces
prises : la bande (analytique) et la cascade de 32 m (λ ≥ 0,5 m) ; la cascade fine (λ < 0,5 m) est exclue et le dit —
sa focale est de l'ordre du mètre, bien au-dessus du fond de la scène, et le disque du soleil l'estompe.

Critères, écrits avant le code :
1. **La FFT porte la hessienne** de η (deux champs complexes de plus par cascade) ; contrôle contre la somme directe,
   même tolérance que S360 (10⁻⁴ du rms).
2. **Contre une solution exacte** : une seule onde (λ = 4 m), soleil au zénith, fond plat à la moitié de la focale
   `H_f = 1/((1 − 1/n)·a·k²)` : l'éclairement rendu contre la somme exacte sur les antécédents (racines trouvées en
   double) en 50 points d'une longueur d'onde — écart ≤ 5 % ; moyenne à 1 % de 1 (énergie).
3. **Sur la scène** : moyenne de l'éclairement focalisé sur l'image du fond vu seul à ± 10 % de 1 ; plafond de
   concentration `1/det_min`, `det_min` = 0,05 *à calibrer* — sa part de pixels publiée.
4. **Images R22** ; preuve, liste 8.5, index.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la hessienne dans la FFT, son contrôle ; critère 1.
- [x] **P3** — les caustiques dans le nuanceur du fond ; la bande et la cascade de 32 m données au fond.
- [x] **P4** — le contrôle contre la solution exacte ; critère 2.
- [>] **P5** — la scène, l'énergie ; images R22 ; critères 3 et 4.
- [ ] **P5 bis** *(ajoutée en cours, sur l'échec du critère 3)* — la carte de caustiques **directe** : la surface
  projetée sur le fond triangle par triangle, rapports d'aire additionnés dans une vue orthographique (Wyman) ; même
  contrôle analytique (critère 2), énergie de la scène à ± 10 % (critère 3) ; le fond la lit.
- [ ] **P6** — preuve, file, liste, index.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2, critère 1 tenu.** Cinq champs complexes par cascade (`CHAMPS`), troisième image `(η_xx, η_yy, η_xy, η)` et ses
  niveaux. Contrôle (`--controle-fft`, η, ∂η/∂x, η_xx) : pire **1,85·10⁻⁵** et **3,27·10⁻⁵** du rms. Courbures : cascade
  de 32 m, η_xx de 0,03 à 0,42 m⁻¹ (focale ≈ 1/((1 − 1/n)·0,3) ≈ 13 m, dans la scène) ; cascade de 4 m, 3 à 9 m⁻¹
  (focale ≈ 0,8 m) — son exclusion des caustiques du fond de la scène (≥ 5 m) se tient.
- **P3.** `sol.gdshader` : `surface(x)` (bande analytique + cascade de 32 m : pente, η, hessienne), `rayon(s)` (Snell
  exact, `refract` de GLSL), `focalisation` — point fixe à quatre itérations, `∂p/∂s` par différences centrées (10⁻³),
  `J = I + p⊗∇η + (H + η)·(∂p/∂s)·Hess η`, `1/max(|det J|, det_min)` ; éclairement direct `0,4·n·soleil` multiplié.
  `mer.gd` : la bande au fond à chaque image, la cascade 0 et sa hessienne ; `CAUSTIQUES=0` les éteint. **Vu, fond
  seul** : un réseau de cellules d'un à deux mètres aux arêtes vives ; des boucles fines (replis) ; au loin un grain —
  le réseau plus fin que le pixel.
- **P4, critère 2 tenu.** `--controle-caustiques` : a = 5 cm, λ = 4 m, soleil au zénith, H_f = 31,946 m, H = 15,973 m ;
  sortie rouge C/4 et vert fract(10·C). 50 points : pire **2,35 %**, moyenne rendue **1,0016** (exacte 0,9987 sur ces
  points) ; crête 1,712 rendue contre 1,673 exacte à x = 0,80 m.
- **P5, critère 3 manqué — mesuré.** `--controle-caustiques-scene` (mer complète, soleil de la scène, fond seul,
  sortie rouge C/20, vert fract(C)) : plongeante, moyenne **2,029**, max 20,3, 2,7 % des pixels au plafond ; proche,
  **5,553**, 1,4 % au plafond. Attendu : 1 (l'énergie se déplace). **Cause** : la méthode à rebours ne suit qu'un
  antécédent ; elle ne vaut que devant la première focale — tenue à mi-focale (P4), elle sort de son domaine sur la scène,
  où la cascade de 32 m a sa focale vers 13 m et le fond descend à 40 m. Remède : la méthode directe (P5 bis).
