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

Session : S380 — **en cours**. *« Pas de solveur continue la pluie ajoute les manquants »* (après R28, *« Parfait »*).
La campagne du solveur volumique n'est **pas** la suite ; la pluie se complète. Agent : Claude Opus 5.5, application
desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Thèse.** Ce qui manque à la pluie se range en trois familles — ce qu'on **voit dans l'air** (les gouttes qui tombent,
l'atténuation au loin, le ciel de pluie), ce qu'on **voit sur les surfaces** (gerbes, surfaces mouillées, reflets des
objets, scintillements, bâches sur les rides) et ce que **V calcule** (exposition tirée des objets posés, absorption par
le sol, flaques) — plus le coût. Un ADR en fait l'inventaire, l'ordre et les sources ; cette session en construit la
première pièce, la plus visible : **la pluie dans l'air**. Près de l'œil, chaque goutte d'au moins 1 mm dessinée, au
nombre de Marshall et Palmer (`N(D) = N0·e^(−Λ·D)` par m³), tombant à la vitesse d'Atlas, tracée comme la voit une
caméra (Garg et Nayar 2007 : une traînée de longueur `v·τ + D`, d'opacité `min(1, D/s)·min(1, (D + s)/(v·τ))`, `s` la
taille du pixel, de radiance ≈ 0,94 × la moyenne de l'environnement) ; au loin, l'extinction qu'elles produisent,
`β = (π/2)·∫N(D)·D² dD` — 1,55·10⁻³ m⁻¹ à 10 mm/h, une visibilité de 2,5 km.

**Critères, écrits avant.** (1) Sans pluie, les 12 images de S379 **identiques au bit**. (2) Le nombre de gouttes dans
le volume dessiné égal à `∫N dD × volume` et la loi des tailles celle de Marshall et Palmer, **à ±5 %**, comptés sur des
images de contrôle (vue d'aplomb, une tranche, chaque goutte un point coloré par classe de taille). (3) L'extinction
ajoutée égale à `β` calculé (au pour cent) et nulle par temps sec. (4) Photographies réelles cherchées et chiffrées ;
jugement de l'utilisateur (R29).

### Plan

- [x] **P1** — jeton, plan seul ; la décision de l'utilisateur consignée (file, feuille de route : pas de solveur).
- [x] **P2** — ADR-205 : la pluie complète — inventaire des manquants, familles, ordre, sources, ce qui reste à la météo.
- [x] **P3** — références : photographies de pluie qui tombe (traînées, rideau au loin) ; ce qu'elles montrent.
- [x] **P4** — les gouttes : `pluie_air.gd` (boîte autour de la caméra, nombre par classe de taille depuis `pluie.gd`),
  nuanceur de particules procédural (position, taille, vitesse par hachage de l'indice, repli dans la boîte, arrêt au sol
  et à l'eau).
- [x] **P5** — leur dessin : traînée alignée sur la chute, opacité et radiance de Garg et Nayar ; bassin et mer.
- [x] **P6** — l'extinction au loin (`β`, brume de la scène) ; critère 3.
- [x] **P7** — contrôle du nombre et des tailles (critère 2) ; critère 1 ; coût.
- [>] **P8** — images de R29 ; preuve `PLUIE-AIR-S380` ; liste, file, feuille de route, index.
- [ ] **P9** — rituel.

### Notes de reprise

**P3 — quatre photographies libres** (Wikimedia Commons, lues dans le navigateur, rien de téléchargé).

| photographie | ce qu'elle montre |
|---|---|
| *Downpour (4390180547)* — averse tropicale sur des piscines, vue d'un balcon | un **voile gris** qui mange le lointain en quelques centaines de mètres (arbres et bâtiments à 100–200 m déjà délavés) ; de **fines traînées verticales**, visibles seulement devant les fonds sombres (les arbres), invisibles devant le ciel ; ciel couvert blanc-gris, sans ombre ; piscines bleues, mates, sans reflet net |
| *Rain over the Sea, Mundesley* (geograph 6985351) | la pluie **au loin sur la mer** : un rideau de stries grises sous la base des nuages, horizon adouci ; ciel couvert |
| *Rain in Malta 03* | rue inondée, pluie forte de jour : traînées **à peine perceptibles** à cette résolution ; eau de ruissellement piquetée, mate |
| *Downpour in New York* | nuit : chaussée mouillée qui **reflète** les lumières (pièces 5, 6), éclats d'impacts sur l'asphalte (pièce 4) ; traînées courtes |

Chiffré : la visibilité de l'averse tropicale (≈ 300 à 600 m) donne `β ≈ 3,9/V` ≈ 0,007 à 0,013 m⁻¹ ; la loi retenue
(`β = (π/2)·2·N0/Λ³` = 6,9·10⁻³ m⁻¹ à 100 mm/h, V ≈ 570 m) la place à ≈ 100 mm/h, plausible pour une averse tropicale ; à
10 mm/h, 1,55·10⁻³ (V ≈ 2,5 km). Les traînées sont **faibles** de jour — l'opacité de Garg et Nayar (quelques pour cent)
est le bon ordre ; elles ne ressortent que devant un fond sombre.

**P4–P5 — les gouttes et leur dessin** (faits ensemble : le dessin est la moitié du même nuanceur ; un commit pour les
deux, dit ici). `pluie_air.gd` : deux classes, [1 ; 2) mm dans une boîte de ±4 m × 6 m (384 m³) et [2 ; 6] mm dans ±8 m ×
10 m (2 560 m³), placées aux trois quarts de leur demi-côté devant la caméra ; à 10 mm/h, 232 et 20 gouttes par m³, soit
≈ 89 000 et 51 000 particules. `pluie_air.gdshader` : tout par hachage de l'indice (position, diamètre par inversion de
l'exponentielle tronquée, vitesse d'Atlas), repli autour du centre, arrêt au sol et à l'eau des deux bacs (niveaux du
moment), traînée `v·τ + D` (τ = 1/60 s), largeur au moins un pixel, opacité de Garg et Nayar. `goutte.gdshader` : radiance
0,94 × moyenne de l'environnement (ciel à 30°, 60°, zénith ; sol à 0,2 × zénith) — **approchée**. Vu : bassin (buse) à
10 mm/h, traînées fines et faibles visibles contre le sol ; mer, presque invisibles contre le ciel — comme les photos.
Écarts d'image moyens : 4,9 (bassin, buse), 4,5 (mer, proche). Un `Array[Vector4]` typé refusait un tableau littéral :
tableaux non typés.

**P6 — l'extinction.** `Pluie.extinction` = (π/2)·2·N0/Λ³·10⁻⁶ ; posée comme brume de Godot (`1 − e^(−β·d)`, couleur du
ciel) : piscine — brume seulement sous la pluie ; mer — `BRUME_SECHE` (0,00012) + β, propagée à notre brume par pixel
(`brume()`). **Critère 3** (`--controle-pluie`, lignes `CONTROLE_PLUIE_S380`) : 0 / 5,64335·10⁻⁴ / 1,55557·10⁻³ /
4,28785·10⁻³ m⁻¹ posés pour la loi à 0 / 2 / 10 / 50 mm/h, **égaux** ; intégration numérique indépendante (numpy) :
5,64335·10⁻⁴, 1,55557·10⁻³, 4,28785·10⁻³ ; visibilités 6,9 / 2,5 / 0,9 km ; brume éteinte par temps sec. Le contrôle des
anneaux (S379) tourne désormais sans gouttes ni brume. `%e` refusé par GDScript (encore) : `String.num_scientific`.

**P7 — contrôles.** `--controle-gouttes` : vue d'aplomb, tranche de 0,5 m à 16 m, dix instants, un carré de deux pixels
par goutte, couleur par classe ; décor caché, fond noir, anticrénelage coupé.
- **Impasse n° 1** : avec le décor, les margelles (≈ 0,9 en tonalité linéaire) comptaient comme gouttes blanches (une tache
  de 63 813 pixels), et l'anticrénelage mêlait le bord des points au fond (aire médiane 1 pixel, comptes gonflés de 50 %).
- **Critère 2** — écarts par classe [1 ; 1,5) / [1,5 ; 2) / [2 ; 3) / [3 ; 6] mm : **2 mm/h** −0,84 / +1,91 / +5,21 /
  +6,11 % ; **10 mm/h** −0,73 / +0,49 / −1,48 / +1,33 % ; **50 mm/h** −2,95 / −2,09 / −2,15 / −2,64 %. **Erreur du critère
  écrit** : « ±5 % » oubliait le bruit de Poisson — à 2 mm/h, 1 014 et 30 gouttes attendues dans les deux grosses classes
  (σ 3,1 % et 18 %) ; jugé à ±5 % quand σ ≤ 2,5 %, sinon à 2 σ : **tenu** aux trois intensités (pire mesurable 1,91 /
  1,48 / 2,95 %). À 50 mm/h, −2 à −3 % partout : des fusions que l'aire médiane ne rattrape pas.
- **Critère 1** : 12 / 12 images identiques au SHA-256.
- **Coût** (720p, GPU, médiane de 240) : bassin proche 0,53 → 0,80 / 1,35 / 3,57 ms (2 / 10 / 50 mm/h), aplomb 0,48 → 0,74
  / 1,40 / 4,01, rasante 0,46 → 0,56 / 0,80 / 1,74 ; mer proche 1,60 → 1,90 / 3,24 / 8,94, rasante 1,58 → 1,90 / 3,07 /
  8,17, référence 1,56 → 1,82 / 2,97 / 7,84. Par rapport aux rides seules (S379) : les gouttes coûtent +0,1 à +0,3 ms à
  10 mm/h, +0,45 à +1,2 ms à 50 mm/h.
