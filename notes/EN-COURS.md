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

Session : S371 — **en cours**. **Rendu 10 : la caméra à demi immergée** (ADR-019 §6, liste 8.6), session de rendu de
l'alternance d'ADR-191 D3, suite déclarée par S369 et S370. Aujourd'hui (S365) le mode immergé bascule **d'un bloc** sur
la hauteur de la bande sous l'œil : quand la ligne d'eau traverse l'objectif, tout le cadre est faux d'un côté.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local
(`~/Downloads/Godot_v4.4.1-stable_win64.exe/Godot_v4.4.1-stable_win64_console.exe`).

**Thèse.** Le milieu se décide **par pixel**, au point où le rayon du pixel traverse le plan proche — l'objectif : sous la
surface de B à son aplomb (déplacement horizontal inversé par point fixe), le pixel est vu de l'eau, sinon de l'air. Une
seule fonction, dans `optique_eau.gdshaderinc`, lue par l'eau, le fond et le ciel ; analytique, donc sans le scintillement
d'une ligne émergée du maillage (ADR-019 §6). Hors de la bande des vagues, le résultat est celui du drapeau global.
L'audio (deux mixages) est hors session : à la fin (ADR-197 D5).

**Critères, écrits avant.** (1) La ligne d'eau rendue contre l'intersection analytique de la surface et du plan proche,
recalculée en double dans `mer.gd`, sur au moins 16 colonnes et trois états de mer : **≤ 1 pixel**. (2) Scintillement :
sur 60 images consécutives à 1/60 s, caméra fixe dans la houle, **aucun pixel ne change de milieu à plus de 2 pixels de
la ligne analytique**. (3) Non-régression : les poses au-dessus (proche, rasante) et sous l'eau (sous_eau, zénith)
**identiques au bit** au rendu d'avant. (4) La ligne sur l'objectif (ménisque) décrite d'après des photographies réelles
cherchées par nous, avant d'être rendue ; jugée par l'utilisateur (R26).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les images témoins d'avant (quatre poses, commit de départ) ; références réelles de la ligne d'eau
  (photographies « dessus-dessous » libres, lues sans téléchargement) : ce qu'elles montrent, consigné ici.
- [x] **P3** — la classification par pixel dans `optique_eau.gdshaderinc` (point du plan proche, hauteur de la bande à
  son aplomb, borne pour sortir tôt) ; la bande passée au fond et au ciel.
- [x] **P4** — l'eau, le fond et le ciel lisent le milieu du pixel (profondeur d'origine par pixel, trajet dans l'eau
  depuis l'objectif) ; la brume de Godot sur les pixels vus de l'eau.
- [x] **P5** — `--controle-ligne-eau` : critère 1, puis critère 2 ; non-régression (critère 3).
- [ ] **P6** — la ligne sur l'objectif (ménisque), d'après P2 ; les images de R26.
- [ ] **P7** — preuve `DEMI-IMMERGEE-S371`, ADR-019 note datée, liste 8.6, file, feuille de route, index ; revue R26.
- [ ] **P8** — rituel.

### Notes de reprise

**P2 — images témoins d'avant** (commit `d66ac1f1`, scratchpad `avant/`) : `godot_proche_12s`, `godot_rasante_12s`
(sans `--cote`), `godot_sous_eau_cote_12s`, `godot_sous_eau_zenith_cote_12s`, `godot_proche_cote_12s`.

**P2 — références réelles de la ligne d'eau**, lues dans le navigateur, chiffrées par un canevas (luminance sRGB décodée,
moyenne sur 7 à 41 colonnes), rien téléchargé :

- **A** — *Reef Scenic Split Shot in the Bird's Head Seascape* (Jones/Shimlock, Secret Sea Visions ; Wikimedia Commons,
  CC BY-SA 4.0) ; 1 000 × 670, hublot en dôme (ligne légèrement courbée), récif peu profond, soleil. De haut en bas, à
  trois colonnes : la surface vue d'au-dessus, en rasant, tassée sur ≈ 12 px, qui reflète la rive (0,44 ; 0,43 ; 0,15) ;
  **un trait sombre de 1 à 2 px** (0,12) ; **une bande claire de 5 à 7 px** (0,61 ; 0,62 ; 0,37), la plus lumineuse de la
  région ; **passage à l'eau en ≈ 3 px** (le rouge tombe de 0,26 à 0,01) ; l'eau, uniforme (0,00 ; 0,19 ; 0,21). Ligne
  entière ≈ 10 px, **1,5 % de la hauteur** ; côté eau ≈ 0,5 fois la surface vue d'au-dessus.
- **B** — *Over-under with flippers* (Gerry Thomasen ; Commons, CC BY 2.0) ; 2 592 × 1 944, compact (hublot plan
  probable), rivière à l'ombre, eau verte. **Passage continu, sans bande claire**, sur ≈ 24 px, **1,2 % de la hauteur** ;
  côté eau 40 fois plus sombre que la roche au-dessus (exposition faite pour l'air).
- Commun : une ligne **continue et lisse**, jamais crénelée ; une transition d'environ **1 % de la hauteur d'image** (le
  hublot est au foyer nul : la surface qui le touche est floue) ; le côté eau plus sombre. La bande claire tient au dôme
  au soleil (A), pas à B. **Choix déclaré** : hublot en dôme — pas de grossissement de la moitié immergée, les objets
  restent continus à travers la ligne.

**P2 — la brume de Godot** (source 4.4-stable, `scene_forward_clustered.glsl`, `fog_process`) : écrire `FOG` dans un
nuanceur supprime entièrement la brume du moteur pour ce matériau (`CUSTOM_FOG_USED`), et la brume lit le cube de
radiance, inaccessible au code utilisateur : pas de brume par pixel au bit. **Décision** : la brume s'éteint quand un
pixel au moins est vu de l'eau (la zone des vagues au plan proche), comme sous l'eau ; à hauteur d'œil de vague, la mer
vue d'au-dessus est rasante et reflète l'horizon, que la brume ne change presque pas — **à mesurer** (P5).

**P3 — fait.** `surface_b.gdshaderinc` : la bande (déclarations déplacées d'`eau.gdshader`), `surface_a_l_aplomb` (Newton,
quatre évaluations, résidu corrigé au premier ordre), `milieu_du_pixel` (part d'eau, profondeur de l'objectif, distance
signée en pixels) ; inclus par l'eau, le fond, le ciel — pas encore lu. `mer.gd` : la bande et la base de la caméra sur
les trois matériaux, `surface_exacte` (le même calcul en double), le mode `demi_immergee`. **Trouvé** : la borne de |η|
(`Σ|a|` + Tayfun) vaut **6,43 m** pour Hs 2,5 m — la pose proche (4 m) passait en mode demi, brume éteinte (jusqu'à 13
niveaux d'écart). **Test serré, rigoureux** : `η` exact au centre du plan proche, plus `pente·R/(1 − G)` sur l'étendue
`R` du plan (G = Σ a·k = 0,726 ; pente bornée 1,238 ; R = 0,095 m à 50° : **0,43 m**). Ensuite : **les cinq poses
témoins identiques au bit**.

**P4 — fait.** L'eau, le fond et le ciel lisent `milieu_du_pixel` ; transition mélangée ; le fond relu par réfraction
n'est pris que s'il a été vu de l'air (sinon le fond non réfracté du pixel) ; le ciel prend la position de la caméra en
uniforme (lire `POSITION` passerait le ciel en mise à jour continue). **Le milieu d'un bloc et la profondeur se prennent
désormais à l'objectif, sur la surface exacte** (S365 : l'œil, la bande sans déplacement ni second ordre) —
`PROFONDEUR=s365` la garde : alors les cinq poses témoins **identiques au bit** ; sans elle, sous l'eau, ≤ 1 niveau sur 5 à
9 % des pixels (2,949 m contre 2,969 m ; 4,115 contre 4,233 au zénith). Poses `demi`, `demi_soleil`, `demi_dessus`,
`demi_dessous`, hauteur relative à la surface exacte. **Deux défauts vus et corrigés** : (a) l'éventail central de 0,25 m —
cent anneaux ajoutés sous R_MIN au même pas (1,2 cm), les anneaux existants inchangés ; 33 pixels à ±1 au zénith sous
l'eau, où le centre est vu ; (b) **la cause réelle des facettes** : la cascade de 4 m lue en bilinéaire, un texel de 1,6 cm
vu sur des dizaines de pixels — le bord de la fenêtre en escalier (`DETAIL=0` l'efface). Lecture **bicubique B-spline**
au-delà d'un grossissement de 32 (fondu à 64). **Impasse** : au seuil 4, les poses validées changeaient (jusqu'à 69
niveaux) — la cascade de 32 m y est déjà grossie de 5 à 15 fois, en bilinéaire ; ce régime-là reste à examiner (file).

**P5 — fait.** `--controle-ligne-eau` (≈ 100 s ; `--cote` implicite) : l'eau, le fond et le ciel rendent la part d'eau
(`controle_milieu`), la ligne rendue (passage à ½) contre l'intersection analytique en double. **Critère 1** : 32
colonnes, quatre cas (t₀, t₀ + 2,3 s, autre position à t₀ + 5,7 s, roulis 0,4 rad) — Newton par pixel **0,020 px** au pire.
**Critère 2** : 60 images à 1/60 s, caméra fixe (la ligne visible au milieu 42 fois, jusqu'à 60 px par image) et
flottante (60 fois, 0,2 px) : **0 pixel mal classé** à plus de 2 px. **Mais le coût** (`--cout-demi`, médiane sur 240
images) : **4,7 à 5,1 ms GPU** de plus — Newton, 4 × 64 composantes, jusqu'à trois fois par pixel. **Corrigé : la surface
au plan proche à l'ordre 2**, calculée une fois par image par `surface_ordre2` (en double, hessienne eulérienne
analytique) ; terme d'ordre 3 borné par `Σ a·k³·r³/6` = 0,37 px au coin du cadre à 50° (2,4 px à 100°, 5,1 à 120°).
Mesuré : **0,078 px** au pire, 0 pixel mal classé ; **surcoût GPU −0,001 et 0,028 ms** ; CPU `immersion()` 0,47 ms en
GDScript, en mode demi seulement. `MILIEU=exact` garde Newton : image contre ordre 2, écarts le long de la ligne seule.
**Brume** (`BRUME=1` la garde, pose `demi`) : au-dessus de la ligne, 0,8 % des pixels, 17 niveaux au pire (p99,9 = 2) ;
au-dessous, 12 % et **jusqu'à 48 niveaux** — le défaut de S366 évité. Témoins : identiques au bit (zénith : 33 px à ±1).
