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

---

## Session en cours

Session : S234 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python et GPU local disponibles)
Entrée : « Reprends le projet », master propre à98430a1, trois copies propres au même commit
(avancées par S233 à 22:12), jeton libre.

**Objectif.** Premier LOD spatial **intégré à l'hôte GPU** : la densité du maillage d'eau suit
la charge que le contenu exige, et non plus seulement le pas de 2 px. Qualité entre sommets,
coutures entre niveaux et coût complet publiés face aux 2 ms (ADR-125, ADR-131 D3).
**Thèse, calculée avant construction (P1, script hors dépôt).** Pose S212 : 86,2 % des
129 600 sommets dans l'emprise du sillage ; 67 % espacés de moins de 0,25 m, soit plus de
8 échantillons par λ_min = 2,09 m. Couper les modes au-delà de Nyquist ne retire que ≈2 % du
travail (7,5 % de sommets sous-échantillonnés, en fond de scène) : c'est une correction de
repliement, pas un levier de coût. Le levier est la **densité près de la caméra**.
**Critère de qualité, à provenance.** Erreur d'interpolation linéaire bornée par
`h²/8 · Σ|a|·k²` (B, impact et sillage publiés), confrontée à la tolérance de 3 mm déjà reçue
aux sommets (`Gpu::verify`, S211). Pentes : erreur publiée contre la grille 2 px, sans seuil
inventé. Référence : cœur aux points **intérieurs** des triangles, pas seulement aux sommets.
**Arrêt.** LOD intégré, vérification sommets + intérieurs + coutures (aucune fissure), coût
fixe/balayé et cadence mesurés avec techniques présentes/absentes. Si la borne de P2 ne
permet aucune réduction de charge, ce constat est le résultat : ne pas assouplir la
tolérance pour obtenir un gain, et reporter le lot vers la technique suivante.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [x] **P2** — lectures ciblées (ADR-129/131/132, HOTE-GPU-S212) ; calculer `Σ|a|k²` et
  `Σ|a|k³` réels des trois couches sur la fixture ; charge réductible par pose ; choisir la
  forme (rangées seules, ou bandes de colonnes cousues) et déclarer le protocole.
  **Amendement P2 (22:25)** : la forme retenue n'est ni l'une ni l'autre — voir notes.
- [x] **P3** — grille locale du sillage, côté CPU : pas choisi par la borne bicubique,
  arrondi au 1/16 m inférieur, capacité fixe et annonce si la borne exige plus fin ; test d'une
  reconstruction Hermite bicubique de référence (CPU) sous sa borne, sur ondes planes.
- [x] **P4a** — passe compute des nœuds `(η, ηx, ηy, ηxy)` et reconstruction bicubique dans
  `water()` ; chemin direct conservé par option `--no-lod` ; `--verify` inchangé et vert ;
  horodatage de la passe compute compté dans le coût d'eau.
- [x] **P4b** — contrôles aux centres et milieux d'arêtes des mailles : grille contre somme
  directe GPU (isole la reconstruction) et contre le cœur (3 mm) ; saut à travers les arêtes de
  maille, LOD contre direct.
- [x] **P5** — coût : `BENCH`, cadence fixe et balayée, avec et sans LOD ; document de
  validation, en-tête ADR-131 D3.
- [ ] **P6** — rituel §6, file, feuille de route, jeton ; copies à synchroniser.

### Notes de reprise

Base : S233 413 tests réussis, 5 ignorés ; S225 passe eau 4,1585 ms fixe, 2,8479 balayée.
P1 : script `grid_geom.py` (bloc-notes de session) reproduit `ocean_vertex` sans hauteur.
Pose S201 : h>λ_min/2 sur 7,5 % des sommets de l'emprise, travail Nyquist/plein 0,978 ;
pose 30 m / tangage −0,5 : 60 % dans l'emprise, 0 % sous-échantillonné. Histogramme des pas
(0,25 m) : 75 463 sommets sous 0,25 m sur 111 715 dans l'emprise.

P2 (`--lod-charge`, `viewer/src/lod.rs`, 3 tests) : provenance du 3 mm = résidu d'intersection
de l'image S201 (IMAGE-B-S201), repris par `verify` ; critère d'interpolation linéaire
`½·M·R²`. Hessiennes (Σ|a|k²) : **B 0,349 constant**, sillage 0,067→0,164 (16 s), impact
0,19→0,03 ; somme 0,54–0,65 → pas isotrope 0,136–0,149 m (B seul 0,185).
Charge idéale du maillage sous ce critère : S212 0,649 ; balayage 0,752 / 0,578 ; **haute 30 m
0,997** ; rasante 2 m 0,359 ; hors emprise 0,807. Rangées seules : 0,695 / 0,777 / 0,641 /
0,993 / 0,433 / 0,823. **Décision** : la densité du maillage est dictée par B, la couche bon
marché (32 composantes), tandis que le coût vient du sillage (4 096 modes par sommet). Alléger
le maillage retire ≤35 % en pose S212 et rien en vue haute ; **découpler la densité
d'évaluation de chaque couche** vise directement la loi `sommets × nœuds`. Sillage : borne
Hermite bicubique `h⁴/384·(2Σ|a|k⁴ + h/4·Σ|a|k⁵)` ≤ 3 mm → pas 1,617 (1 s), 1,404 (3 s),
1,284 (8 s), **1,228 (16 s)**, 1,235 (24 s), 1,233 (39 s) ; 9 116 nœuds au pire sur l'emprise
128×104 m, contre ≈111 700 sommets évalués (pose S212), et 183 825 nœuds en linéaire.
Le LOD de maillage par rangées reste une option mesurée, non construite (au rituel : file).
Technique rattachée : LOD spatial *de couche* (densité selon le contenu) sur une grille locale
sans transformée — la ligne « espace » d'ADR-131 D2 reste absente (pas de FFT).

P3 : `Lattice::plan` (pas au 1/16 m inférieur, capacité 16 384 nœuds, `clamped` et erreur
publiée), `hermite` de référence CPU. Six tests `lod` verts. Exactitude sur polynôme bicubique
(2e-5). Soixante-quatre ondes planes |k|≤3 : pire/borne = 0,117 (h 0,25), **0,118 au pas de
la borne**, 0,061 (h 2) — la borne tient, marge ≈8,5× ; une constante fausse d'un facteur 9
ferait échouer l'essai. Pas S212 au pire (16 s) : 1,228 → 1,1875 m, 109×89 = 9 701 nœuds.

P4a : `bake` (compute 8×8, groupe 3 en écriture) + `wake_lattice` (groupe 2 en lecture) ; somme
directe d'origine intacte sous `lattice.w = 0` ; horodatages 4/5 = cuisson, comptée dans
`GPU_water_ms`. Défaut rencontré : le ciel partage la mise en page → groupe 2 à poser aussi
(panique de validation wgpu, corrigée). `--verify` vert sur les deux chemins, 13 contrôles
chacun : **grille max η 0,400 mm (3 s), 0,320 (16 s)**, pentes ≤1,14e-3 ; **direct max η
0,089 mm**, pentes ≤1,4e-4 (valeurs S225 retrouvées). Premier BENCH (sur **batterie**,
BatteryStatus 1, 68 %) : grille 64×128 à 960×540 **0,457 ms dont cuisson 0,394** (pas
1,3125–1,375 m, âges 3–5 s) ; direct 6,923 ms — plus lent que S225 (4,20). Témoin S233 extrait
hors dépôt (`viewer-s233`, chemins absolus) pour trancher entre état machine et shader.
Recette fine 128×256 grille : 1,782 ms dont cuisson 1,724.

P4b (`Gpu::verify_lattice`, âges 1/4/8/16/24/39) : centres et milieux d'arêtes (16 482 à
28 512 points). **Grille − direct GPU : 0,680 / 0,588 / 0,498 / 0,439 / 0,260 / 0,191 mm**,
bornes 2,600 / 2,581 / 2,687 / 2,614 / 2,548 / 2,567 mm, rapports 0,26 → 0,07. Grille −
cœur : mêmes valeurs à 1 µm. Pentes ≤1,83e-4 à ces points ; le maximum de l'erreur de pente
d'une Hermite cubique est vers t≈0,21/0,79, pas au centre : `VERIFY` (sondes irrégulières)
voit jusqu'à 1,14e-3 (3 s). **Saut à travers les arêtes** (10 839 à 18 812 paires, ±1 mm),
grille moins direct : η ≤6 µm, pentes ≤1,9e-5, pour une variation propre du champ ≈0,5 mm
sur 2 mm — continuité C¹ reçue au bruit f32. Pas 1,5625 → 1,1875 m, 83×68 → 109×89 nœuds.

P5 — impasse coûteuse : le script de campagne nommait un paramètre `$args` (variable
automatique PowerShell) ; le témoin est parti sans argument en fenêtre interactive, ≈20 min
perdues (22:43→23:04). Garde ajoutée : arguments vides refusés. **Alimentation changée en
cours de campagne** (batterie → secteur vers 23:06–23:08) : bancs `--verify` de 23:04/23:05
non comparables entre eux, rejoués sur secteur. Cadences toutes sur secteur (log
`bench_s234.log`), 960×540, 590 images d'intervalle, 200 sérialisées :

| passage | intervalle méd. | Hz | GPU eau méd./max | CPU méd. | acquisition |
|---|---:|---:|---:|---:|---:|
| témoin S233 fixe (1) | 5,1601 | 193,8 | 4,1413 / 4,9504 | 4,4373 | 2,4255 |
| S234 grille fixe | **2,6038** | **384,1** | **0,4288 / 0,4714** | 1,9560 | 0,0216 |
| S234 direct fixe | 5,1999 | 192,3 | 4,2263 / 5,0640 | 4,4833 | 2,5403 |
| témoin balayé | 5,0752 | 197,0 | 2,8650 / 4,4414 | 4,3368 | 2,3058 |
| S234 grille balayée | **2,5112** | **398,2** | **0,4560 / 0,4655** | 1,9081 | 0,0192 |
| S234 direct balayé | 5,0942 | 196,3 | 2,9175 / 4,1615 | 4,3855 | 2,3593 |
| témoin fixe (2) | 5,2385 | 190,9 | 4,1771 / 4,9868 | 4,4905 | 2,4832 |

Grille balayée : un intervalle max 33,23 ms (acquisition max 30,14), p95 3,55 — pic isolé
non attribué. Direct S234 à +1,2–2 % du témoin : le shader modifié ne ralentit pas le chemin
témoin. La trame devient **bornée par le CPU** (acquisition 2,4 → 0,02 ms ; sillage CPU
1,29 ms inchangé) : A265 reçoit une réponse partielle, pas de plancher indépendant du travail
vers 4,9 ms.

Bancs sur secteur (`bench_s234_secteur.log`, 23:09–23:16, témoin/S234/témoin) : 960×540
64×128 témoin 4,2448/4,2426 ; direct 4,2916 (3 s) 4,2841 (16 s) ; **grille 0,4260 (3 s,
cuisson 0,3697), 0,4242 (16 s, 0,3678)**. 640×360 : témoin 1,9123/1,9138, direct 1,9491,
grille 0,3905. 128×256 : témoin 17,56/17,72 et 8,05/8,44 ; grille 1,5061 et 1,4686. Batterie
(premier passage) : témoin 960×540 **7,0145** — facteur 1,65 d'alimentation, jamais publié
de S211 à S225 → A270. Cuisson insensible à +21–33 % de nœuds (non expliqué).
Document : [LOD-SILLAGE-S234](../docs/validation/LOD-SILLAGE-S234.md).
