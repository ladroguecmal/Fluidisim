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

Session : S360 — **en cours**. **Rendu 4 : la surface fine** — la demande de l'utilisateur prime sur l'alternance.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.
Entrée — **verdict R20** : (1) *« Tu as raison sur le ciel, il n'aide pas au reflets et limite la qualité du rendue
final »* ; une capture de l'eau : *« ce rendue du point de vue topologie est pas réaliste »* ; (2) *« La couleur me
paraît parfaite sincèrement »* ; (3) *« Tente les caustique, mais pour l'ecume […] l'ecume n'apparaît presque jamais
sur le vaguelettes uniquement sur des grandes vagues avec déferlement mais très rare voir quasi impossible »* ; (4) pas
de référence, *« tu peux faire tes recherches »*. **Mesuré avant le plan** : la queue qui dessine les petites vagues
compte **60 ondes planes pour 5,5 octaves** (λ 7 cm à 3,4 m) réparties sur 360°, pentes isotropes (rapport 1,04 ;
Cox et Munk : 1,37 à 7,8 m/s) — des taches sans direction ni crête. Ordre : la topologie d'abord (les caustiques
projettent la forme de la surface) ; caustiques et ciel à la session suivante.

Critères, écrits avant le code :
1. **R20 consigné** tel quel ; **recherches sourcées** : Beaufort 4, taille des moutons, anisotropie de Cox et Munk,
   étalement d'Elfouhaily et al. (1997), océan par FFT de Tessendorf (2001).
2. **L'écume au déferlement** : tirée de la seule bande (les vagues dominantes), à une empreinte fixe ; couverture de
   Monahan tenue à ± 25 % (mesurée au nadir, mode contrôle), **plus aucune tache sous 0,5 m** de diamètre équivalent.
3. **Le spectre fin, dans le cœur** : densité continue de la queue d'équilibre de B (même niveau) avec l'étalement
   d'Elfouhaily ; sa variance sur la plage de la queue égale celle de la queue discrète à 1 % ; son rapport de pentes
   au vent / au travers à ± 10 % de Cox et Munk ; amplitudes de départ `h0` sur deux grilles 256² (32 m et 4 m), graine
   fixe, exportées par l'afficheur.
4. **La FFT dans Godot** (calcul de `RenderingDevice`, sans téléchargement) : pentes, gradient du déplacement,
   hauteur ; temps replié en double sur une période de répétition, dispersion quantifiée (I-08) ; **contrôle** — le
   champ de la FFT contre la somme directe des mêmes composantes en quatre points, écart ≤ 10⁻⁴ relatif ; pente
   quadratique moyenne à 1 % de `Σ|h0|²k²`.
5. **Le nuanceur** : la queue de 60 composantes remplacée par les deux cascades, pondérées par l'empreinte ; ce qu'elles
   ne résolvent pas va à la covariance filtrée (ADR-161). Contrôles de S359 inchangés ; rapport mer/ciel sous l'horizon
   à ± 15 % de l'afficheur.
6. **Images R21** : les poses de R20 et la zone de la capture de l'utilisateur.
7. Preuve, ADR, liste 8.9 et 8.4, index.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — R20 consigné ; recherches ; critère 1.
- [x] **P3** — l'écume au déferlement, mesurée avant et après ; critère 2.
- [x] **P4** — le spectre fin dans le cœur, ses essais ; l'export des `h0` ; critère 3.
- [x] **P5** — la FFT dans Godot et son contrôle ; critère 4.
- [x] **P6** — le nuanceur sur les cascades ; critère 5.
- [x] **P7** — les images de R21 ; critère 6.
- [ ] **P8** — preuve, ADR, file, liste, index ; critère 7.
- [ ] **P9** — rituel.

### Notes de reprise
- **P2, recherches.** Beaufort 4 (5,5–7,9 m/s, OMM) : « petites vagues devenant plus longues ; moutons assez
  fréquents » (en.wikipedia.org/wiki/Beaufort_scale ; spc.noaa.gov/faq/tornado/beaufort.html). Taille des moutons :
  Callaghan et al. (2012, JGR 117, C12015), la plupart des taches sous 10 m², au plus 26 m² ; Bondur et Sharkov (1982),
  un pic entre 8 et 16 m² — des taches d'un à quelques mètres, pas de décimètres. Étalement d'Elfouhaily, Chapron,
  Katsaros et Vandemark (1997, JGR 102(C7), 15781–15796), repris par l'Ocean Optics Web Book :
  `Φ = (1/2π)[1 + Δ(k)·cos 2φ]`, `Δ = tanh(a₀ + a_p(c/c_p)^2,5 + a_m(c_m/c)^2,5)`, `a₀ = ln 2/4 = 0,1733`, `a_p = 4`,
  `a_m = 0,13·u*/c_m`, `c_m = 0,23 m/s`, `k_m = 370 rad/m`, `c² = (g/k)(1 + (k/k_m)²)`, `u* = √(0,00144)·U₁₀`.
  Cox et Munk : SPEC-001 §1 sexies — σu² = 3,16·10⁻³·W, σc² = 0,003 + 1,92·10⁻³·W ; à 7,79 m/s, 0,0246 et 0,0180,
  rapport **1,37**. Tessendorf (2001, *Simulating Ocean Water*, cours SIGGRAPH) : réalisation par FFT, `h(k,t) = h0(k)
  e^{iωt} + h0*(−k) e^{−iωt}`, boucle en temps par quantification de ω. Capture de l'utilisateur gardée en local
  (`viewer/captures/s359/r20_capture_utilisateur.jpg`, SHA-256 `8f755dc3…`).
- **P3, critère 2 en partie manqué.** Instrument : mode contrôle 3 (couverture en sortie directe), `mer.gd
  --controle-ecume` (nadir à 12 et 40 m, neuf positions espacées de 200 m, 12 s), `outils/ecume_taches.py`
  (composantes 4-connexes, diamètre équivalent). **Avant** — 12 m : couverture 0,625 %, **4 652 taches**, diamètre
  médian **4,3 cm**, max 45 cm, toutes sous 0,5 m ; 40 m : 0,615 %, 5 552 taches, médian 12 cm. **Après** (seuils de la
  bande seule, `MerCretes::seuils_deferlement`, graine `0x5360_0001` ; empreinte d'au moins 1 m,
  `EMPREINTE_DEFERLEMENT`, Callaghan 2012 et Bondur–Sharkov 1982) — 12 m : 0,737 %, **4 taches**, 1,34 à 3,07 m ; 40 m :
  **0,420 %** (Monahan 0,42 %), 37 taches, médian **1,59 m**, max 3,79 m, mais **21,6 % sous 0,5 m** (8 taches, jusqu'à
  un pixel) : les excursions naissantes au bord du seuil. « Plus aucune tache sous 0,5 m » : **manqué** à 40 m, tenu à
  12 m ; couverture tenue à 40 m, à 12 m quatre taches ne font pas une statistique. Contrôles de S359 inchangés.
  Export relancé depuis la racine : lancé depuis `viewer/`, il écrit sous `viewer/godot/` (fichier égaré supprimé).
- **P4, critère 3 : deux tenus, un manqué.** Cœur : `equilibrium_tail_density` (variance par unité de `x`, niveau
  de la bande) et `elfouhaily_delta` ; essais `continuous_tail_matches_equilibrium_cells_s360` — 60 et 64 cellules,
  écart **3,98·10⁻⁷** et 3,92·10⁻⁷ — et `elfouhaily_spreading_on_the_tail_s360` — Δ contre la formule en f64 à
  3,1·10⁻⁸ ; rapport des pentes au vent / en travers de la queue, repliée sous le vent, **1,228** (calculé en Python
  avant le code, même valeur) pour **1,371** chez Cox et Munk : **−10,4 %, critère de ± 10 % manqué** de 0,4 point.
  Sur la mer entière, bande comprise : 0,97 aujourd'hui (la queue actuelle, 0,88, est plus forte en travers), 1,27 avec
  Elfouhaily. Afficheur : `Scene::tail_recipe`, `tail_bounds` (4 à 28,05 fp) ; `rendu_cretes::export_detail` →
  `godot/donnees/detail_h0.bin` (1 Mo, dérivé), fragment `detail` du JSON. Cascade 0 : 32 m, k 1,789 à 12, **5 742**
  composantes, mss 9,148·10⁻³ réalisée / 9,147·10⁻³ continue ; cascade 1 : 4 m, k 12 à 88,27, **4 870**, 2,567·10⁻² /
  2,551·10⁻² ; total 0,0348 contre 0,0347 pour les 60 composantes de la queue.
- **P5, critère 4 tenu.** `godot/fft_detail.comp` (GLSL, compilé au lancement par `RDShaderSource` — pas d'import ;
  **ASCII seulement**, glslang refuse le reste), cinq modes : spectre (pulsation gravité-capillarité, quantifiée sur
  `PERIODE` = 1 000 s ; seule `(t mod T)/T` arrive à la carte), FFT inverse radix 2 en mémoire de groupe (lignes, puis
  colonnes), composition (deux images RGBA32F à 9 niveaux par cascade), niveaux par moyenne 2 × 2 (LEAN). `detail.gd` :
  `RenderingDevice` principal, fil de rendu, `Texture2DRD` ; `mer.gd --controle-fft` à t₀ + 3,7 s. Somme directe en
  double contre la FFT, quatre texels : cascade 0, pire **7,4·10⁻⁶** du rms ; cascade 1, **2,7·10⁻⁵** ; mss contre
  l'export (Parseval) **6,5·10⁻⁷** et **6,7·10⁻⁷**. rms de η : 2,35 cm et 0,57 cm. GDScript n'a pas `%e` :
  `String.num_scientific`.
- **P6, critère 5 tenu.** `eau.gdshader` : les deux cascades lues à l'empreinte (`filter_linear_mipmap_anisotropic`),
  pentes moyennes, gradient du déplacement, covariance non résolue `E[s·sᵀ] − E[s]·E[s]ᵀ` vers la quadrature d'ADR-161 ;
  la boucle de 60 composantes reste en témoin (`DETAIL=0`). La lumière des crêtes prend la variable de la bande seule,
  comme l'écume (`s` complet retiré). Contrôles : profondeur 0,234 ; transmission 0,0050 (le mode contrôle aplatit
  aussi la surface fine — sans cela 0,0068) ; FFT 7,4·10⁻⁶ et 2,7·10⁻⁵ ; horizon linéaire **0,711 / 0,677** (afficheur
  0,728 / 0,713). **Vu** : plongeante, une texture fine dense, orientée, un scintillement en éclats ; une seule tache
  d'écume dans le cadre, **lisse et ovale** — un mouton réel est déchiqueté. La capture de l'utilisateur n'a pas pu être
  située dans les images de R20 (corrélation 0,19).
- **P7.** R21 écrite (REVUE-VISUELLE §26), quatre images envoyées. `viewer/captures/s360` (non versionné), SHA-256 :
  plongeante côte `accb1a1d…`, large proche `41ff26ab…`, côte proche `53cad509…`, large rasante `17851797…`, témoin 60
  composantes `f5c61aff…`, agrandissements `9921cfef…` (FFT), `714c06b2…` (témoin), `46bd872a…` (les deux).
