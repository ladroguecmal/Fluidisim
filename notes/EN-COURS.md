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

Session : S379 — **terminée**. *« Continue mais il faudra prévoir une session du plus dur et complexe de la création d'un
solveur […] 3D volumétrique ultra réaliste et performant en temps réel dynamiquement »*. Deux choses : **inscrire la
campagne du solveur** comme prochaine session de physique (conception d'abord) ; **continuer** par la session de rendu
prévue — **les rides de pluie factices** (ADR-202 D3, ADR-203 D7), sur le bassin et sur la mer.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Thèse.** Un effet de rendu, **sans simulation**, mais dont les nombres ont une provenance : les gouttes qui laissent un
anneau net (D ≥ 1,5 mm) arrivent au taux de **Marshall et Palmer** (1948 ; `N0` = 8 000 m⁻³·mm⁻¹, `Λ = 4,1·R^−0,21`) × vitesse
terminale d'**Atlas** (1973) — **448 m⁻²·s⁻¹ à 10 mm/h** ; l'anneau s'étend à la vitesse minimale des ondes
capillaires-gravité, **0,23 m/s** (λ ≈ 1,7 cm), et s'éteint en ≈ 0,6 s. Procédural (cellules, couches, âges), réglé par la
distance : quand l'empreinte d'un pixel dépasse l'anneau, sa pente devient de la **rugosité** (la variance de LEAN, ADR-161)
— la surface mate d'une eau sous la pluie.

**Critères, écrits avant.** (1) Sans pluie, les rendus d'avant **identiques au bit** (mer et bassin). (2) Le taux de
naissance des anneaux, compté sur des images de contrôle (cœurs d'anneaux de moins de 30 ms), **à ±10 %** de Marshall et
Palmer à 10 mm/h. (3) Au loin, aucun motif : les anneaux cèdent à la variance avant que l'empreinte du pixel n'atteigne
leur longueur d'onde (vérifié par le calcul de l'empreinte au seuil). (4) Photographies réelles cherchées et chiffrées ;
jugement de l'utilisateur (R28).

### Plan

- [x] **P1** — jeton, plan seul ; la campagne du solveur inscrite (file).
- [x] **P2** — références : photographies de pluie sur l'eau (libres, lues sans téléchargement) ; ce qu'elles montrent.
- [x] **P3** — les rides dans `bassin.gdshader` (le taux, l'anneau, le fondu en variance) ; critère 1 sur le bassin.
- [x] **P4** — les rides sur la mer (`eau.gdshaderinc`, pente et covariance de la queue) ; critère 1 sur la mer.
- [x] **P5** — contrôle du taux (critère 2), du fondu (critère 3) ; images de R28.
- [x] **P6** — preuve `RIDES-PLUIE-S379`, liste (8.9 ou 8.4), file, feuille de route, index ; la campagne du solveur dans
  la feuille de route.
- [x] **P7** — rituel ; session suivante : **la campagne du solveur volumique 3D** (conception).

### Notes de reprise

**P2 — six photographies libres, Wikimedia Commons, lues dans le navigateur, rien de téléchargé.**

| photographie | vue | ce qu'elle montre |
|---|---|---|
| *Rain in a pond at Zoo Schönbrunn 2018* a et b | proche, oblique (≈ 30°), pluie faible à modérée | chaque impact = **un paquet de 2 à 5 crêtes** concentriques, pas un cercle seul ; rayons jusqu'à ≈ 15–25 cm avant l'extinction ; les jeunes impacts portent un **dôme ou une bulle** au centre ; recouvrements, interférences ; ellipses aplaties par l'obliquité |
| *Waterwaves raindrops on water surface* | presque verticale | mêmes paquets (2 à 4 crêtes) ; **visibles là où le ciel clair se reflète, presque invisibles sur les reflets sombres** : l'anneau est une perturbation de la **normale**, pas une couleur |
| *Rain on the river, Warwick* | lointaine, pluie faible | au loin, **les reflets restent nets** ; les anneaux ne se voient qu'au premier plan : la rugosité lointaine suit le taux, presque nulle en pluie faible |
| *Rain on the River Pang, near Tidmarsh* | moyenne distance, rasante, crépuscule | les impacts deviennent de **petits éclats clairs**, ellipses très aplaties (tirets horizontaux) qui prennent le ciel sur un fond sombre ; les reflets des berges sont rompus, pas effacés |
| *Rain falling into a swimming pool* | piscine à ≈ 10 m, pluie modérée à forte | plus d'anneau distinct : **un voile mat uniforme**, reflets brouillés — la variance, pas le motif |

Conséquences pour P3–P4 : perturber la **normale** seulement (jamais d'albédo) ; un **paquet** de crêtes par anneau (≈ 3,
enveloppe gaussienne), une petite bosse centrale au premier dixième de seconde ; au loin, la pente devient de la variance,
**proportionnelle au taux** (pluie faible : presque rien).

**P3 — le bassin.** `pluie.gd` (statistiques : taux, Λ, moments) + `pluie.gdshaderinc` (géométrie, tirage, niveau de
détail) + `bassin.gdshader` (pente, variance intégrée au reflet par 3 × 3 Gauss-Hermite, éclat élargi en forme close, fond
flouté par la chaîne de l'écran). Chiffres :
- **Critère 1 (bassin)** : sept images d'avant (ensemble, rasante, buse à 6,5 et 200 s ; sans δ à 40 s), **identiques au
  SHA-256** après ; empreintes d'avant dans le bloc-notes de la session (`avant.sha`), déterministes (deux passes égales).
- **Critère 2, déjà mesuré** (`--controle-pluie`, 20 instants, 32 m²) : 2 mm/h : 1 275 cœurs pour 1 290,6 (**−1,21 %**) ;
  10 mm/h : 8 529 pour 8 584,2 (**−0,64 %**) ; 50 mm/h : 36 646 pour 37 091,4 (**−1,20 %**). Le compte brut des taches
  perdait 5,2 % à 50 mm/h (fusions de cœurs voisins, 58 cœurs jeunes par m²) : chaque tache compte `arrondi(aire/médiane)`.
  Le taux de forme close est 447,1 m⁻²·s⁻¹ à 10 mm/h (le plan disait 448 : mon arrondi de tête).
- **Impasse** : le niveau de détail à l'empreinte **isotrope** (la plus grande) et au seuil λ/6–λ/3 effaçait tout au-delà
  de 1 à 2 m en 720p — aucune ride visible dans les vues de S374. Corrigé : empreinte **dans la direction du rayon** de
  chaque anneau (`|Mᵀ·dir|`), fondu λ/4–λ/2 (Nyquist, toujours sous λ : critère 3), et la variance floute aussi le fond
  vu (sans quoi l'eau vue à travers ne montre rien).
- **Coût GPU** (`--cout-pluie`, 1280 × 720, médiane de 240 images) : proche 0,53 → 0,76 / 1,27 / 3,24 ms à 2 / 10 / 50
  mm/h ; aplomb 0,48 → 0,71 / 1,35 / 3,85 ; rasante 0,46 → 0,52 / 0,66 / 1,20. Cinq hachages pcg4d par couche : une
  table des décalages par couche, calculée par l'hôte, en retirerait trois (optimisation possible, non faite).
- Vues ajoutées : `pluie_proche` (1,8 m, 30°), `pluie_aplomb`. À l'œil : paquets de 2–3 crêtes, bosses centrales,
  anneaux lisibles surtout dans le reflet clair — comme *Waterwaves raindrops* ; le ciel de la scène reste ensoleillé
  (la météo n'est pas faite).

**P4 — la mer, et ce qu'elle a appris au bassin.** Les rides entrent dans `eau.gdshaderinc` en coordonnées de **Lagrange**
(elles suivent le mouvement orbital) : pente ajoutée avant le passage en Euler, covariance avant son transport ; l'horloge
`temps_pluie` repliée sur l'heure par `mer.gd`. **Critère 1 (mer)** : les cinq poses d'avant identiques au SHA-256 ; la
variante à demi immergée compile et rend.
- **Impasse n° 2** : à 10 mm/h la mer changeait de 2 à 4 niveaux sur 765 en moyenne, rien à l'œil — à 4 m de haut, les
  crêtes de 1,7 cm sont sous le pixel partout. Deux enrichissements **fondés**, pas un gain arbitraire : (a) un **second
  train** par anneau, λ 4,4 cm à la vitesse de groupe minimale 0,178 m/s (les crêtes d'espacement croissant des photos),
  pente 0,08 — résolu 2,6 fois plus loin ; il tient dans la même maille (0,178·0,6 + 0,05 = 0,157 m) ; (b) la variance de
  bande **anisotrope**, le long du rayon (covariance xx, xy, yy), comme la pente qu'elle remplace ; le bassin l'intègre par
  Cholesky comme la mer. Après : mer à 50 mm/h, écart moyen 8,8 niveaux, p99 72 — des éclats fins, discrets ; bassin rasant
  piqueté, carrelage déformé.
- **Coût GPU** après (1280 × 720) : bassin proche 0,53 → 0,78 / 1,34 / 3,52 ms (2 / 10 / 50 mm/h), aplomb 0,48 → 0,72 /
  1,43 / 4,20, rasante 0,46 → 0,53 / 0,68 / 1,29 ; **mer** proche 1,60 → 1,84 / 2,94 / 7,76, rasante 1,58 → 1,84 / 2,85 /
  7,00, référence 1,56 → 1,75 / 2,68 / 6,45. Hachage court par couche (lowbias32) au lieu de deux pcg4d : −8 % seulement ;
  le coût est le **nombre d'itérations** (2 × 167 couches à 50 mm/h, sur tout pixel dont la plus petite empreinte est sous
  5 cm). **À faire** (inscrit) : le champ d'anneaux calculé **une fois par image** dans une texture à moments (pente et
  second moment, filtrée par mipmaps comme la queue FFT, S360) — coût indépendant de la résolution et de l'étendue d'eau.
- Taux recompté après le hachage court : −4,16 % (1,5 σ) / −0,61 % / −1,66 % à 2 / 10 / 50 mm/h.

**P5 — contrôles à l'état final** (bosse centrale éteinte de R/2 à R, 3 à 6 mm, au lieu du train court) :
- **Critère 1** : les 12 images d'avant (7 du bassin, 5 de la mer) identiques au SHA-256.
- **Critère 2** : 1 237 / 1 290,6 (−4,16 %, 1,5 σ) ; 8 532 / 8 584,2 (−0,61 %) ; 36 477 / 37 091,4 (−1,66 %, 3,2 σ — le
  reste des fusions que l'arrondi de l'aire ne rattrape pas) — **tenu** (±10 %). Le taux de forme close vérifié par
  intégration numérique indépendante (numpy, trapèzes sur [1,5 ; 30] mm) : 67,22 / 447,09 / 1 931,84.
- **Critère 3, par le calcul** : crêtes éteintes à λ/2 — 8,6 mm pour λ 17,1 mm, 22 mm pour λ 44 mm ; bandes (supports 4 et
  10 cm) passées à l'uniforme à 2 et 5 cm, quatre échantillons au moins par bande ; bosse (Ø ≈ 12 mm) éteinte à 6 mm ;
  aucune boucle au-delà de 5 cm de plus petite empreinte. Aucun motif n'atteint le pixel sous sa longueur d'onde.
- Variance lointaine : 5,6·10⁻⁴ / 2,75·10⁻³ / 1,25·10⁻² (pente RMS 0,024 / 0,053 / 0,11) à 2 / 10 / 50 mm/h — petites gouttes
  97,7 / 290,9 / 660,3, anneaux 51,5 / 437,2 / 2 629,6 m⁻²·s⁻¹ (moments). Pour comparaison, Cox et Munk à 5 m/s : 0,029.
- **R28** : quatre planches dans `viewer/captures/s379/` (locales, non versionnées, comme R27), REVUE-VISUELLE §33. Vu :
  en rasant, les reflets nets des nuages font place à un voile mat dès 10 mm/h — posé comme question.
- **Message de l'utilisateur en cours de session** : *« Dis-toi que les systèmes d'optimisation tels que les LOD vont
  complètement bouleverser les performances »* — le coût de P4 (effet isolé, sans les niveaux de détail d'ADR-202) n'appelle
  ni optimisation improvisée ni réduction : mesuré, publié, l'optimisation inscrite (texture à moments), on continue.
