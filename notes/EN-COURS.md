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

Session : S307 — en cours : recherche profonde après un verdict négatif.
Agent : Claude Opus 5, application desktop ; fichiers, git, cargo, carte réelle, **accès web**.
Entrée : le verdict, enfin — « **le rendu actuel est toujours mauvais** » — et une consigne de
méthode : « va au-delà du guide, regarde les références liées, et les références des références,
multiplie les étapes ». Verdict consigné dans [REVUE-VISUELLE](../docs/validation/REVUE-VISUELLE.md).

**Ce que ce verdict veut dire, et pourquoi il change la méthode.** S304 a mesuré l'absence
d'asymétries et l'a corrigée ; S306 a mesuré l'origine des stries et l'a rendue réglable. Deux
corrections réelles, chacune reçue sur ses chiffres, et **l'image reste mauvaise**. Un défaut qui
résiste à deux corrections mesurées est plus probablement un **terme absent** qu'un terme mal
réglé. Donc : arrêter de raffiner ce qui existe, et aller chercher ce que le rendu ne contient
pas du tout.

Capacité visée : **savoir, sourcé, ce qui manque à notre rendu pour qu'une mer soit crédible** —
une liste ordonnée par ce qui se voit, chaque poste rattaché à une source primaire, chaque écart
chiffré ou chiffrable sur nos propres images. Consommateur : la décision de construction (un ou
plusieurs ADR) et les lots qui suivront.

Ce que je ne fais pas : choisir un coupable sans mesure — c'est exactement ce que S304 et S306
ont fait, et deux fois ça n'a pas suffi ; reprendre les cinq arbitrages d'ADR-027 ; réduire
ADR-127 ; prendre un chiffre de seconde main sans remonter à sa source (I-14).

Critères, écrits avant la recherche :
1. **Trois niveaux de lecture atteints** : les références du guide lues à la source ; leurs
   propres références ; et l'état de l'art que le guide **ne cite pas**. Un niveau qui n'apporte
   rien se dit tel quel.
2. **Chaque poste identifié porte** : sa source primaire, ce que notre rendu en fait aujourd'hui
   (ligne de code ou ADR), et l'ordre de grandeur de l'écart.
3. **La liste est ordonnée par ce qui se voit**, pas par ce qui est facile — et l'ordre est
   argumenté.
4. **Rien n'est construit avant la décision**, et la décision cite la mesure.

### Plan

- [x] **P1** — amorce, verdict consigné, plan seul.
- [x] **P2** — niveau 1, **et un fait qui déclasse la recherche** : le rendu soumis n'était pas
  le meilleur rendu du dépôt.
- [x] **P3** — niveau 2 : les références **des** références, et l'état de l'art du rendu d'océan
  que le guide ne cite pas.
- [x] **P4** — niveau 3 : l'optique de l'eau et du ciel — couleur, absorption, diffusion, écume.
- [x] **P5** — *(fait avant la synthèse, parce qu'il devenait urgent)* le remède de protocole :
  `--meilleur`, la ligne d'options complète, la règle dans REVUE-VISUELLE, la leçon L349.
- [x] **P6** — synthèse ordonnée : ce qui manque, sourcé, chiffré, rattaché à notre code.
- [x] **P7** — la couleur de l'eau depuis ses sources, et ce qu'elle change à l'image.
- [x] **P8** — décision : ADR-177 (la couleur se dérive), et la revue R14 qui remplace R12/R13.
- [ ] **P9** — rituel REPRISE §6. *(Écume, diffusion aux crêtes et spectre ECKV : déclarés en file, pas construits ici.)*


### Notes de reprise

## P2 — la découverte qui passe avant toute la recherche

**Pour la troisième revue consécutive, les images envoyées n'étaient pas celles du meilleur
rendu du dépôt.** R11 tournait sous `--houle` seule (trouvé en S303). R12 et R13 tournaient
**sans `--ciel-clair`**, c'est-à-dire avec la brume et le ciel de S211 — et non avec l'habillage
construit en **S261 d'après la photo de référence de l'utilisateur lui-même**.

J'ai d'abord **regardé** notre image, ce qu'aucune session n'avait fait ; puis lu le nuanceur.
Le chemin par défaut (`p.eye.w = 0`), celui de R12 :

```wgsl
mix(color, vec3(0.66,0.78,0.84), 1 - exp(-length(v.local)/500.0))
```

**Une brume dont la longueur caractéristique est 500 m**, sur une scène qui porte à 1 500 m. À
500 m il reste 37 % de l'image, à 1 000 m 14 %. Le chemin `--ciel-clair` fogge à **6 km** et
porte un ciel à dégradé et nuages. Mesuré sur la même mer, même instant, même caméra :

| | moyenne linéaire (bande d'eau) | B/G | B/R | plus sombre B/G |
|---|---|---:|---:|---:|
| **R12 tel qu'envoyé** | R 0,102 G 0,198 B 0,234 | **1,18** | 2,29 | 1,24 |
| **`--ciel-clair`** | R 0,059 G 0,122 B 0,247 | **2,02** | 4,23 | 2,84 |

L'écart se voit à l'œil immédiatement : l'une est une nappe gris-vert, l'autre une mer bleue avec
un horizon. **ADR-176 D5 demandait de déclarer les options d'une revue** ; S304 a déclaré les
options de **vagues** (a/b/c) et pas celles d'**environnement**, parce que personne ne pensait au
ciel comme à une option. Or la mer est un miroir : le ciel est la moitié de l'image.

**Ce que cela ne dit pas** : que le rendu est bon. Avec `--ciel-clair` il est bien meilleur et
reste insuffisant — tapis de scintillement uniforme, aucune écume, et un **artefact brun-olive**
en moyenne distance que la brume cachait jusqu'ici.

## P2 — recherche de niveau 1 : les références du guide portent sur la géométrie, pas sur l'aspect

Lues à la source, les douze références du guide sont des références de **forme** (FFT, Gerstner,
clipmap, projected grid) et de **physique côtière** (SWAN, TMA, Celeris, SWE, CEM). Exemple
mesurable : [GPU Gems ch. 1](https://developer.nvidia.com/gpugems/gpugems/part-i-natural-effects/chapter-1-effective-water-simulation-physical-models)
(2004) donne la limite de boucle de Gerstner `Σ Q_i w_i A_i ≤ 1`, les normales analytiques, et
**quatre** vagues géométriques plus ~15 vagues de texture. Notre rendu est très au-delà.

**Aucune des douze ne traite** : la couleur de l'eau, l'absorption, la diffusion, l'écume, le
ciel physique, l'exposition. Or c'est exactement ce que notre rendu traite **par des constantes
écrites à la main**. Le guide le dit d'ailleurs lui-même en §10.2, sans sources.

## P2 — la couleur de l'eau n'a aucune provenance, et elle est fausse d'un facteur 4 à 9

La constante du nuanceur est `vec3(0.012, 0.105, 0.13)` (défaut) ou `vec3(0.004, 0.060, 0.170)`
(ciel clair). Aucune provenance — I-14 l'interdit, et personne ne l'avait relevé.

Calcul depuis les sources primaires : absorption de l'eau pure
([Pope & Fry 1997](https://omlc.org/spectra/water/data/pope97.txt), données téléchargées),
diffusion moléculaire `b = 0,0029·(550/λ)^4,30` avec `b_b = b/2` (Morel 1974), réflectance
d'irradiance `R(0⁻) ≈ 0,33·b_b/(a + b_b)` :

| λ | `a` (1/m) | `b_b` (1/m) | `R(0⁻)` |
|---|---:|---:|---:|
| 450 nm | 0,00922 | 0,00344 | **0,0896** |
| 550 nm | 0,0565 | 0,00145 | **0,00826** |
| 650 nm | 0,340 | 0,00071 | **0,00068** |

**B/G attendu = 10,9 ; B/R = 131.** Nos constantes donnent B/G = **1,24** (défaut) et **2,83**
(ciel clair) : **9 fois et 3,8 fois trop vert**. Et la mesure le confirme dans l'image : les
pixels les plus sombres — ceux où l'on voit dans l'eau et non le ciel — rendent exactement le
rapport de la constante (1,24 et 2,84).

## P2 — ce que la recherche a déjà rapporté pour la suite

- **Spectre ECKV / Elfouhaily et al. 1997** ([formules complètes](https://www.oceanopticsbook.info/view/surfaces/level-2/wave-variance-spectra-examples)) :
  un spectre unifié gravité → capillarité, avec `k_m = 370 rad/m`, `c_m = 0,23 m/s`, un âge de
  vague `Ω_c` et un étalement `Δ(k) = tanh[a₀ + a_p(c/c_p)^2,5 + a_m(c_m/c)^2,5]`. Il **remplace
  à lui seul** notre queue `f⁻⁴` continuée à la main (ADR-157) **et** le calage empirique sur
  Cox–Munk (ADR-158) : la variance de pente en sort, elle ne s'ajuste plus.
- **Écume** : couverture `W = 3,84·10⁻⁶·U₁₀^3,41` (Monahan & O'Muircheartaigh 1980), réflectance
  effective de Koepke **0,22** — pas 1. À `U₁₀ ≈ 7,8 m/s` (notre mer de vent, `Hs` 1,5 m), cela
  fait **0,42 % de couverture**. Faible, mais c'est le seul objet de l'image qui donne l'échelle,
  et il est **totalement absent**.
- **Bruneton, Neyret & Holzschuch 2010**, *Real-time Realistic Ocean Lighting using Seamless
  Transitions from Geometry to BRDF* (Computer Graphics Forum 29(2)) : la référence exacte de
  notre problème de transition géométrie → BRDF. Le guide ne la cite pas.

## P2 — correction d'une affirmation que j'avais écrite trop vite

J'avais noté un « artefact brun-olive en moyenne distance ». **Mesuré : il n'existe pas.** Zéro
pixel sur 921 600 n'a `R > B`. Les taches que je prenais pour un défaut ont `B/G` = 1,36, celui
du **ciel** (1,47) : ce sont des reflets de la couche de nuages, pas un bug. Ce qui est vrai en
revanche, c'est que l'eau y paraît terne — parce que partout où l'on voit *dans* l'eau plutôt que
le ciel, on voit notre constante trop verte. L'œil avait raison sur le symptôme et tort sur la
cause.

## P3 — l'état de l'art que le guide ne cite pas, et qui répond à notre question

**Bruneton et al. 2010**, lu par son implémentation de référence
([portage Unity](https://github.com/Scrawk/Brunetons-Ocean/blob/master/Assets/BrunetonsOcean/Shaders/Ocean.shader)) :

```
float2 sigmaSq = tex3D(_Variance, ...);          // variance de pente, par pixel
float fresnel  = 0.02 + 0.98 * MeanFresnel(V, N, sigmaSq);
col += ReflectedSunRadiance(SUN_DIR, V, N, Tx, Ty, sigmaSq) * Lsun;
col += MeanSkyRadiance(V, N, Tx, Ty, sigmaSq) * fresnel;
float3 Lsea = _SeaColor * Esky / M_PI;  col += Lsea * (1.0 - fresnel);
```

**Ce que cela nous apprend, et c'est important** : le remède au tapis de scintillement n'est pas
de **couper** les pentes non résolues (ce que fait notre `--coupure` de S306, et ce que je
proposais) mais de les **convertir en rugosité de BRDF** — Fresnel **moyen** sur la distribution,
soleil et ciel intégrés sur elle. On garde toute l'énergie **et** on perd l'aliasing. **Nous
avons déjà cela** : c'est `--reflets-filtres` ([ADR-161](../docs/adr/ADR-161-reflets-de-la-queue-non-resolue.md),
accepté en **R7**) — et il n'était pas actif non plus dans les images de R11, R12 et R13.

Sa couleur d'eau : `(0.0039, 0.046, 0.09)`, soit **B/G = 1,96, B/R = 23** — appliquée à
l'irradiance du **ciel**, donc bien plus bleue en sortie que notre constante appliquée à une
constante. Et **aucune écume** : confirmation qu'elle est une couche à part, absente partout.

**Sea of Thieves, SIGGRAPH 2018** (Ang et al.) : la couleur mélange une *deep water colour* et
une *sub-surface colour* selon l'angle de vue, la direction du soleil **et un masque de crête
tiré du déplacement horizontal de la FFT** — plus la crête est comprimée, plus la lumière
traverse une faible épaisseur, plus la diffusion se voit. L'écume naît aux crêtes, puis est
floutée avec rétroaction pour simuler sa dispersion.
**Ce masque de crête, nous l'avons déjà** : c'est le **déterminant du jacobien** de CWM, calculé
à chaque pixel depuis S260 et **jamais employé pour autre chose qu'un repli**.

## P4 — les deux postes que notre rendu traite par des constantes, chiffrés

1. **Couleur du corps d'eau** — §P2 ci-dessus : B/G attendu 10,9, obtenu 1,24 (défaut) / 2,83
   (ciel clair).
2. **Perspective aérienne** — brume à 500 m par défaut, 6 km en ciel clair, contre des dizaines
   de kilomètres de portée visuelle en air marin propre.
3. **Écume** — `W = 3,84·10⁻⁶·U₁₀^3,41` (Monahan & O'Muircheartaigh 1980), réflectance effective
   **0,22** (Koepke), soit **0,42 %** de couverture à `U₁₀ ≈ 7,8 m/s`. Absente.
4. **Diffusion sous la surface aux crêtes** — absente ; le masque existe déjà (jacobien).
5. **Spectre** — ECKV/Elfouhaily remplacerait la queue `f⁻⁴` **et** le calage Cox–Munk.

## P5 — le remède de protocole

`--meilleur` active `--vagues --modulation --ciel-clair --reflets-filtres` ; la liste vit dans
`main.rs` à côté du code qui la consomme. Chaque ligne de capture publie **toutes** les options.
Leçon **L349**. Scènes de référence toujours au bit.

**Mesure qui corrige S306** : `--reflets-filtres` divise `hf_rms` par **2,5** (15,05 → 5,97) sans
retirer d'énergie de pente, là où `--coupure=2` n'atteignait 2,2 qu'en en supprimant. Le bon
levier existait et était éteint.

## P7 — la couleur, et une erreur de ma part corrigée par l'image

`--eau-physique[=gain]` : la teinte vient de `R(0⁻) = (0,00068 ; 0,00826 ; 0,08960)` dérivée de
Pope & Fry 1997 et Morel 1974, **et le gain est un paramètre libre déclaré**, qui tient lieu
d'irradiance de ciel (`E/π` chez Bruneton).

**Première tentative, fausse, et il faut le dire.** J'avais remis `R(0⁻)` à la **luminance** de
la constante historique (facteur 4,476), pour ne changer que la teinte. Résultat : un bleu
outremer artificiel — et pour cause, le bleu valait alors **0,40 de réflectance**, ce qu'aucune
eau ne fait (le maximum physique est 0,09). La renormalisation compensait en réalité une
irradiance de ciel absente. **Regarder l'image l'a montré en une seconde ; aucun chiffre ne
l'avait dit.**

`gain = 1`, c'est-à-dire `R(0⁻)` **nue**, donne la meilleure image que le projet ait produite :
creux bleu marine profonds, crêtes qui renvoient le ciel, contraste franc. `gain = 2` est plus
clair. **Le gain est un arbitrage visuel** : R14.

Scènes de référence toujours identiques au bit (`--eau-physique` éteint par défaut).

**Ce que le rendu contient aujourd'hui, à avoir en tête pendant la recherche** : mer multimodale
JONSWAP à étalement `cos^2s` (ADR-156), queue d'équilibre `f⁻⁴` continuée à la main (ADR-157),
rugosité calée sur Cox–Munk (ADR-158), queue en pentes par pixel (ADR-155), reflets de la queue
non résolue (ADR-161), ciel **procédural** (ADR-162), CWM pour les crêtes pointues, asymétries
d'ADR-176. **Ce qu'il ne contient pas du tout** : écume, spray, micro-déferlement, couleur d'eau
dérivée d'un modèle d'absorption, diffusion sous la surface, ciel physique, exposition contrôlée.
La couleur de base est **une constante RGB écrite dans le nuanceur** (`vec3(0.012,0.105,0.13)`,
ou `vec3(0.004,0.060,0.170)` en habillage « ciel clair »).

---

## Archive — notes de S304 (lot de la mer, en attente du verdict R12)

**Construit** : `tayfun()` et `lagged_eps()` dans `water.wgsl`, accumulations par système dans la
boucle existante de `band_cwm` (branches explicites — FXC refuse l'indexation dynamique en
écriture, L345) ; douzième `vec4` de l'uniforme `(split, k̄₁, k̄₂, retard)` ; mêmes deux termes dans
la référence CPU `cwm_reference` ; activation par `--vagues --modulation`, témoin `--sans-asym`,
balayage `--retard=`. Le fragment n'a pas changé : le sommet lui transmet la déformation **retardée**.

**Réception d'ADR-176 §3, cinq critères tenus** :

| critère | exigé | obtenu |
|---|---|---|
| 1. statistiques | `Sk` ≥ 0,06 ; `c₀₃` ∈ [−0,18 ; −0,13] ; `c₂₁` −0,058 ± 0,02 ; `c₄₀` ∈ [0,26 ; 0,45] ; `mss` ± 2 % | **0,0656** ; **−0,155** ; **−0,057** ; **0,340** ; +0,2 % |
| 2. GPU contre CPU | 3 mm ; 5·10⁻⁴ | 1,59·10⁻⁶ m ; 2,69·10⁻⁴, aucun repli |
| 3. scènes au bit | défaut, `--houle`, `--vagues`, `--sans-asym` | **identiques** au binaire de S301 |
| 4. écart au jeu | publié | 0,3651 → **0,3999 m** (+3,5 cm ; CWM en porte 36,5) |
| 5. coût | publié, secteur relevé | 1,0138 → **1,0163 ms** (+0,25 %) |

`k̄` par système : **0,1742** (mer de vent) et **0,0317** (houle) — l'instrument et l'hôte donnent
les mêmes, donc les deux implémentations portent bien le même modèle.

**Images R12** : `--revue-mer=<état>`, quatre poses de R11, trois états nommés dans le fichier
(`a_houle_seule`, `b_vagues_modulation`, `c_asymetries`). Regardées ici : l'écart `a` → `c` est
franc (la mer de R11 était lisse et striée) ; l'écart `b` → `c` est plus fin — crêtes plus marquées,
contraste crête/creux plus net. C'est à l'utilisateur de juger, R12 pose quatre questions.

---

## Archive — notes de S303 (pour le lot)

**Ce que la recherche donne** (sources dans la preuve) : l'asymétrie verticale d'une mer profonde
vient des **harmoniques liées du second ordre** (`Sk = 3k̄σ` en bande étroite, Longuet-Higgins 1963,
Tayfun 1980) ; les pentes se mesurent au miroitement depuis Cox & Munk 1954, révisées par IASI ; et
— le plus visible — les **rides ne sont pas uniformes** : elles se raccourcissent et se redressent
sur les crêtes des vagues longues, s'aplatissent dans les creux (JFM 2024 : pente modulée de 20 %
à `ε_L` = 0,1, doublée à 0,4), avec un **retard** qui place leur maximum en avant de la crête.

**Ce que la mesure donne** (instrument de S260 étendu, 10⁶ points, même réalisation que le rendu) :

| | `mss` | `c₂₁` | `c₀₃` | `c₄₀` | `Sk` |
|---|---:|---:|---:|---:|---:|
| observations (Cox–Munk 7,95 m/s ; 3k̄σ) | 0,0437 | −0,058 | −0,222 | 0,40 | 0,156 |
| `--houle` — **la scène montrée en R11** | 0,0198 | −0,001 | 0,002 | −0,026 | −0,0001 |
| `--vagues --modulation` (le meilleur construit) | 0,0496 | 0,001 | 0,001 | 0,390 | 0,0030 |
| **S303 retenu** (+ Tayfun + retard −0,20) | 0,0497 | **−0,057** | **−0,155** | 0,340 | **0,066** |

**Trois choses apprises, dans l'ordre d'importance.**
1. **Faute de protocole de ma part** : la scène soumise à R11 était `--houle` **seule**, c'est-à-dire
   sans la queue d'équilibre ni les vagues pointues construites en S260–S261. C'est la
   configuration que S260 avait déjà mesurée « pentes quasi gaussiennes ». L'utilisateur a jugé la
   plus pauvre des trois mers du dépôt.
2. **Même la meilleure n'a aucune asymétrie** : `Sk` 0,003 contre 0,156 ; crêtes et creux aussi
   arrondis les uns que les autres — le « concave plutôt que convexe » du verdict, chiffré.
3. **Le second ordre par composante ne peut pas la produire** (mesuré : 0,0022) : ce sont les
   termes **croisés** qui la portent. D'où Tayfun, qui les contient tous pour une somme de plus.

**Le retard est le seul paramètre libre**, calé sur `c₀₃` ; son **signe** ne l'est pas — seul un
retard négatif (rides en avant de la crête) donne l'asymétrie du signe observé, et `c₂₁` tombe sur
Cox–Munk sans avoir été visé. Reste ouvert : le noyau exact du second ordre pour deux systèmes
(la vérité est entre 0,063 et 0,150), les capillaires parasites, l'asymétrie horizontale.

---

## Archive — notes de S302 (pour le lot)

**P2 — la scène tient** (`--delta3d-scene-mesure`, mer `--houle` à 64 composantes, domaine
96×128×28 à 25 cm = 344 064 mailles, repos 3,5 m, éponge 3 m, 32 cycles, 60 Hz, 12 s) :
- **aucune colonne hors bornes**, jamais, avec et sans paquet ;
- le paquet garde ses 25 cm et traverse de `y = 26` à `y ≈ 7` en 11 s, soit **1,7 m/s** — la
  vitesse de groupe théorique d'une onde de 8 m vaut 1,77 m/s. L'onde isolée (paquet − témoin)
  reste à 0,18–0,26 m ;
- δ **sans paquet** porte déjà 9 à 19 cm : la correction couplée de B à Hs 2,5 m (R10 : 10,6 cm
  mesurés en 2D à Hs 2 m). À dire dans la revue : une partie de ce que l'utilisateur verra n'est
  pas l'onde mais cette correction ;
- coût **4,23 ms** par pas (médiane, 177 dispatchs) : au-delà des 2 ms d'ADR-174 D3, ce qui est un
  point de la porte C, pas de la revue ; tenable à 60 Hz avec le rendu (≈ 2 ms) ;
- **tous les pas reçus sont déclarés dégradés** au sens d'ADR-144 (divergence franche 2·10⁻²).
  Les diagnostics ne reviennent que 60 fois sur 720 : l'anneau est plein tant que la carte
  travaille — perdus, jamais retardants, comme D3 le prévoit.

`Step3::on_device` : le pas se construit sur un device **fourni**, celui du rendu ; la surface
publiée devient un tampon que le rendu lie (D7), sans passage par le CPU. `published_buffer()` est
le seul tampon exposé.

**P3 — les à-coups d'A297 se voient dans les chiffres** (`--delta3d-scene-acoups`, dérivée seconde
temporelle par colonne, 2,2 à 4,4 millions d'échantillons) :

| | d2 RMS | q99,99 | max | localité | rugosité de maille RMS | max |
|---|---:|---:|---:|---:|---:|---:|
| témoin, 32 cycles | 9,2·10⁻⁵ m | 1,8·10⁻³ | 7,2·10⁻³ | **7,4** | 2,17·10⁻³ m | 5,9·10⁻² |
| témoin, 128 | 9,2·10⁻⁵ | 1,8·10⁻³ | 4,8·10⁻³ | 7,3 | 1,99·10⁻³ | 2,1·10⁻² |
| témoin, 512 | 9,2·10⁻⁵ | 1,7·10⁻³ | 4,8·10⁻³ | 7,3 | 2,03·10⁻³ | 2,2·10⁻² |
| paquet, 32 | 1,6·10⁻⁴ | 2,0·10⁻³ | 6,7·10⁻³ | 7,3 | 2,68·10⁻³ | 4,7·10⁻² |

« Localité 7,4 » : au pire à-coup, la colonne fautive vaut 7,4 fois la moyenne de ses huit
voisines — la signature d'une bascule, pas d'une onde. **Le balayage de cycles tranche** : de 32 à
512 cycles la rugosité ne bouge pas (2,17 → 1,99 → 2,03 mm) ; ce n'est **pas** une pression
sous-convergée, c'est le schéma. À comparer à la signature du paquet lui-même à l'échelle de la
maille, `a·k²·dx²` = 9,6·10⁻³ m : le bruit vaut environ **un cinquième** du signal utile en
hauteur, et jusqu'à 5 fois au pire point. Conséquence pour la revue : le bruit de maille est à
montrer et à nommer, pas à cacher ; 32 cycles suffisent visuellement (le maximum seul gagne à
128), donc le choix de cycles est une question de coût.

**P4 et P5 — la scène est rendue et soumise.** `Step3::on_device` fait naître le domaine sur le
device du rendu ; le nuanceur lie **la seule surface publiée** (groupe 3), l'interpole en
Catmull-Rom bicubique et l'ajoute à la somme des couches avec un fondu de 3 m. Témoin : les sept
images de la revue R9 gardent leurs empreintes **au bit** (0x8ae42dfb1fe7d1b3…) — le rendu existant
n'a pas bougé. Fenêtre : `--houle --delta3d`, **D** bascule, **R** relance l'onde ; un pas fixe de
16,667 ms par image, donc le fond que δ consomme et celui que l'image montre sont au même instant.
Cadence **197 Hz** (5,06 ms), essai de 120 images passé.

**Dimensionnement, deux allers-retours mesurés** : d'abord 24×32 m avec une onde de 25 cm à 8 m —
l'onde se noyait dans une mer de 2,5 m (les deux images se ressemblaient). Puis 32×32 m : **refusé
par le device**, le tampon des faces (26 flottants par face, S300) franchissant les 128 Mio d'une
liaison de stockage. Retenu : **30 × 28 m** (120×112×28) et un **front** de 65 cm sur 16 m, crête
longue de 12 m — visible sans être hors du régime perturbatif (`ak` 0,26 ; le refus non diagnostiqué
de 2D est à 0,335). Une vue plongeante a été essayée et écartée : rien ne s'y lit (déjà mesuré S275).

Captures : quatre poses × avec/sans × quatre instants, plus deux images de différence ×6.
La couche change **13,5 à 15,7 %** des octets à la pose de référence (8,5 à 11,2 % de plus de
quatre niveaux). Preuve : [SCENE-DELTA3D-S302](../docs/validation/SCENE-DELTA3D-S302.md) ; demande
de revue : REVUE-VISUELLE §16 (R11), avec **quatre questions explicites** — R10 avait échoué faute
de dire ce qu'on attendait.

*Tenue du plan* : le battement du commit P2+P3 a été écrit **sans lire l'horloge** (23:18 au lieu
de 23:06, L237) ; corrigé au commit suivant. Une fenêtre interactive s'est ouverte une fois parce
que l'ancien binaire ignorait un drapeau neuf après un échec de compilation — vérifier que la
compilation a réussi avant de lancer.

---

## Archive — notes de S301 (pour le lot)

**Architecture retenue (P2).** `Step3` crée **un** device et y compile les trois sources telles
quelles — `delta3d_background.wgsl` (S300), `delta3d_cg.wgsl` (S299), `delta3d_step.wgsl`
(nouveau) — chacune avec sa propre disposition de liaisons, sur des tampons communs. Rien de
S299/S300 n'est réécrit (L137). Rangements : `vel = [u|v|w courants | u|v|w prédits]`, l'indice
d'une face étant aussi celui de son échantillon de fond ; `cells_in = [eta | divergence |
eta_roundoff]` — **eta vit là**, c'est l'entrée de `couple_columns` ; `cells_out` = celui de S300 ;
`work = [flux_x | bande_x | flux_y | bande_y | surface publiée]`. Les passages vers la projection
(surface totale → `heights`, second membre → tranche B, préconditionneur → tranche M) seront des
copies de tampon **dans l'encodeur**, sans retour CPU.

**P2 reçu** (`--delta3d-prediction`, 15×11×14, 7 459 faces, fond S300 à 64 composantes, vitesses
d'ordre 0,2 m/s, éponge (1 ; 0,75 ; 2 s⁻¹), dt 5 ms, trois instants) : pire écart **6,0·10⁻⁸ m/s**,
soit **≤ 9,0·10⁻⁶ de l'incrément** du pas, sur les trois familles ; 70 à 85 % des faces au bit ;
zéro face fautive. Refus : durée nulle, éponge trop large, `dt²g/dx > 1`, longueur.

**Défaut trouvé par le banc, corrigé avant commit** : le noyau couplait les faces `i = nx` (u) et
`j = ny` (v), que le cœur saute — borne comparée à `n + 1` au lieu de `n`. Écart de **60 % de
l'incrément**, sur ces seules faces. Le compteur `faces_fautives` (> 5 % de l'incrément) reste
dans le banc : c'est lui qui voit une règle de bord portée autrement, l'arrondi ne le peut pas.
Rapporter l'écart à l'incrément et non à la vitesse était nécessaire : rapporté à la vitesse, le
même défaut ne pesait que 1 %.

**Incident d'outillage, corrigé par un commit séparé** : le battement de P2 a été écrit par
`Get-Content -Raw | Set-Content -Encoding utf8` de Windows PowerShell 5.1, qui **lit en ANSI** :
`REPRISE.md` est parti ré-encodé (mojibake) dans `fff03d5`. Restauré depuis `7ebeeb5`, battement
réécrit à l'outil d'édition. **Ne jamais réécrire un fichier du dépôt par `Get-Content` /
`Set-Content`** ; `[IO.File]::ReadAllText/WriteAllText` (UTF-8 par défaut) ou l'outil d'édition.

**P3a reçu** (`--delta3d-pression`, même fixture, 1 510 mailles mouillées sur 2 310, départ
`p = 0` des deux côtés). Le cœur converge en 65 et 68 itérations, sans affinage. La carte :

| cycles | dispatchs | écart p (t=0) | relatif | écart p (t=1,23 s) | relatif |
|---|---|---|---|---|---|
| 8 | 51 | 2 757 Pa | 0,17 | 2 992 Pa | 0,18 |
| 32 | 171 | 218 Pa | 1,3·10⁻² | 196 Pa | 1,2·10⁻² |
| 64 | 331 | **0,27 Pa** | 1,6·10⁻⁵ | **0,20 Pa** | 1,2·10⁻⁵ |
| 128 | 651 | 0,38 Pa | 2,3·10⁻⁵ | 0,077 Pa | 4,7·10⁻⁶ |

Échelle 16 400 Pa. Plateau dès 128 cycles : plancher f32. En usage : 0,27 Pa ≈ **0,03 mm d'eau**.
Aucune maille sèche non nulle. Résidu vrai de la carte 3,6·10⁻⁷ à 64 cycles, celui du cœur
2,3·10⁻⁷. À 8 cycles l'écart vaut 27 cm d'eau **depuis p = 0** : en trajectoire le départ chaud
part de la pression du pas précédent, et c'est P5 qui dira combien de cycles il faut alors.
`couple_rhs` lit désormais `eta_roundoff` (`cells_in` après la divergence) ; banc S300 rejoué,
chiffres identiques (1,06·10⁻⁶ ; 6,0·10⁻⁸ ; 3,9·10⁻⁷). `init_warm` ajouté à `delta3d_cg.wgsl`.

**P3b reçu** (`--delta3d-correction`, même fixture, vitesses de fin de pas contre celles du cœur
après `step_perturbation_mobile`, extrapolées comprises) : incrément du pas ≈ 0,5 à 0,76 m/s (le
champ de départ est fortement divergent, la projection le redresse). Écart **≤ 1,4·10⁻⁵ m/s**,
**≤ 2,6·10⁻⁵ de l'incrément** à 128 cycles, ≤ 1,7·10⁻⁵ à 64 ; **zéro face fautive** sur les trois
familles et trois instants. Fantômes latéraux avec le fond de la face (`p_dyn`, `grad_p_dyn`),
fantôme du haut avec `eta_roundoff` et `ghost_up` de S300 : aucune règle de bord divergente.

**P4 reçu** (`--delta3d-pas`, pas complet à 128 cycles, **655 dispatchs**, trois instants) :
- hauteur : incrément du pas 4,2 à 5,1 mm ; écart **2,4·10⁻⁷ m** (un ulp de η à 2,25 m),
  159 à 162 colonnes sur 165 au bit ; surface publiée au même écart ;
- **hauteur vraie** `η − reste` : écart **3 à 4·10⁻⁸ m**, six fois sous l'ulp — la compensation
  est portée ;
- vitesses ≤ 1,4·10⁻⁵ m/s (2·10⁻⁵ de l'incrément), pression ≤ 0,38 Pa (2,3·10⁻⁵).
Surface publiée = `(η − repos) − reste` par colonne, **tampon à part** (liaison 7) : le rendu ne
liera jamais `cells_in` ni `vel` (D7, I-13). Accès d'essai `surface_roundoff_for_trials` ajouté au
cœur.

**Défaut trouvé, corrigé avant commit — la somme compensée détruite par le compilateur.** Premier
passage : reste nul sur les 165 colonnes. Expérience : sur la carte `(η + inc) − η` rendait `inc`
**au bit** pour 165/165 colonnes, quand la même addition est inexacte sur CPU pour 164 à 165/165.
Le compilateur de la carte (DX12) simplifie `(a + b) − a → b` ; la compensation de S233 disparaît
sans bruit — hauteur au bit près dans 130/165 colonnes seulement, et une perte systématique de
l'ordre de l'ulp de η par pas. Remède : `exact_difference(s, a)`, soustraction **en entiers sur
les bits IEEE**, exacte par Sterbenz pour deux hauteurs à moins d'un facteur deux. Même famille
que L345 (fraction de phase) : **une identité flottante du source n'est pas une identité du
binaire compilé** — généralisation à écrire en leçon. *(Écrite : L346.)*

**P5 — trajectoire S298, premier passage** (`--delta3d-trajectoire`, 32×24×36, 64 composantes,
1 200 pas, cycles 8/16/32/64 en parallèle, 4 min 39) : écart de hauteur **≤ 5·10⁻⁶ m jusqu'à
t = 1 s** pour 32 et 64 cycles (1·10⁻⁴ à 8 cycles, 5·10⁻⁵ à 16), puis saut au millimètre vers
t ≈ 1,1 s et 1 à 3,5 cm ensuite, **quel que soit le nombre de cycles**. Donc pas la projection.
Localisation (`PAS`, `CYCLES`, `SONDE_PAS` en variables d'environnement) : le saut naît à
**n = 215** dans la maille **(19,10,32)** — **sèche pour le cœur (p = 0), mouillée pour la
carte (p = 37 Pa)** : une surface passée à moins de 10⁻⁶ m d'un centre de maille, classée des deux
côtés opposés. Une seule bascule donne 0,25 m/s d'écart sur une face `v`, 0,08 sur `u`, puis des
centimètres de hauteur. Avant la bascule, les faces adjacentes aux mailles de surface à petit θ
diffèrent déjà de 2·10⁻³ m/s (coefficient 1/θ jusqu'à 1 000) sans effet visible sur η.
**Fausses pistes écartées** : B n'est pas en cause (élévation carte/cœur à 9·10⁻⁸ m sur ce fond,
`--delta3d-fond-s298`) ; l'éponge non plus (colonne intérieure). Une première sonde décalée d'un
pas (lue après le pas du cœur) avait fait croire à un écart de B — artefact de l'instrument.
**Question ouverte à trancher par la mesure** : sensibilité **propre** du schéma de référence
(deux cœurs à ±10⁻⁶ m au départ, `--delta3d-sensibilite`) — si deux cœurs se séparent pareil,
l'écart est une propriété de la référence et le critère ponctuel de §4.2 n'est tenable que sur
une durée déclarée avant la première bascule.
Artefact de banc corrigé : la surface publiée n'était écrite qu'au premier pas (écart 0,16 m à
t = 0) ; `set_state` la publie désormais.

**P5 — tranché par la sensibilité de la référence** (`--delta3d-sensibilite`, deux cœurs, le
second à ±10⁻⁶ m au départ, motif haché, 1 200 pas) : **ils se séparent exactement comme la carte
et le cœur** — 4,6 à 7·10⁻⁵ m jusqu'à t = 1 s, premier dépassement du millimètre au **pas 260**
(1,3 s), 1 à 4,5 cm ensuite, écart quadratique 2,3 à 3,1 mm de t = 3 à 6 s. Carte contre cœur à
64 cycles : quadratique 1,9 à 2,4 mm sur la même période, premier millimètre vers le pas 216.
**L'écart de P5 est une propriété du schéma de référence, pas de la production.** Avant son
horizon, la carte suit le cœur **mieux** (≤ 5,5·10⁻⁶ m à 32–64 cycles) qu'un cœur perturbé d'un
ulp ne suit le cœur (≤ 7·10⁻⁵ m).
**Mécanisme** (sonde) : le transport de la hauteur lit la vitesse de la couche **partiellement
mouillée** (fraction `(surface − k·dx)/dx`). Quand la surface passe le centre de cette couche, sa
face bascule de « projetée » (correction à 1/θ, θ petit) à « extrapolée » depuis la couche
inférieure : 0,25 m/s d'écart sur une face, soit ~6·10⁻⁴ m de hauteur par pas
(`dt/dx · Δu · dx · ½`), observé 5,8·10⁻⁴ puis 2,8·10⁻⁴ m par pas. La hauteur est donc
**discontinue** en la position de la surface par rapport aux centres de maille : deux états
distants de 10⁻⁶ m divergent de centimètres en une demi-seconde. Nouvel angle mort (A297).
**Lecture du critère 2** (ADR-175 §4.2, « sur la durée déclarée ») : la durée ponctuelle ne
peut excéder l'horizon de prévisibilité **de la référence elle-même**, mesuré ; au-delà, la
production se compare à l'enveloppe que la référence a contre elle-même. Aucun seuil relevé.

**P6 mesuré** (`--delta3d-cout-pas`, pas entier horodaté de la première passe à la dernière,
copies comprises, fond S298 à 64 composantes, 30 passages, premier écarté ; secteur aux deux
bornes, BatteryStatus = 2, 98 %) — médianes de banc :

| domaine | mailles | 8 cycles | 16 | 32 | 64 |
|---|---:|---:|---:|---:|---:|
| 32×24×36 (cas S298) | 27 648 | 0,305 ms | 0,382 | 0,532 | 0,838 |
| 32³ | 32 768 | 0,329 | 0,410 | 0,561 | 0,876 |
| 64×64×32 | 131 072 | 1,106 | 1,299 | 1,697 | 2,504 |

Dispatchs : 57, 97, 177, 337. Un maximum isolé à 7,34 ms (64×64×32, 8 cycles, premier passage
8,39 ms) : pic d'amorçage de la famille A294, non attribué. **Ce n'est pas la porte C** (pas de
scène, pas de 99ᵉ centile) : 64×64×32 tient sous 2 ms jusqu'à 32 cycles en médiane de banc.

**P5, horizon mesuré** (trajectoire relancée avec la métrique) : premier millimètre au pas
260 (8 et 16 cycles) et 220 (32 et 64), contre 260 cœur contre cœur ; pire écart avant l'horizon
9,1·10⁻⁴ (8), 5,8·10⁻⁵ (16), 1,0·10⁻⁴ (32), 2,4·10⁻⁵ m (64) ; pire sur 6 s 3,4 à 4,3 cm, contre
4,5 cm pour la référence contre elle-même.

*Tenue du plan* — **fusion déclarée P5+P6+P7 en un commit** : leur code a été écrit pendant les
calculs de P5 (4 à 8 min chacun) dans le même fichier, et leurs mesures sont consignées ici.
Le banc de localisation `--delta3d-fond-s298` est gardé : il a écarté B.

**P7 reçu** (`--delta3d-diagnostics`) — justesse, depuis l'état de P4 : divergence des lignes
franches carte **1,1 à 1,8·10⁻⁶** à 128 cycles contre **2,7 à 7,3·10⁻⁶** pour le cœur (sa
projection s'arrête à ses critères, la carte fait 128 cycles) ; toutes lignes 2,5·10⁻⁶ à
8,4·10⁻⁵. À 16 cycles depuis p = 0 : 1,6 à 2,7·10⁻² → **déclaré dégradé**. Différé, 100 pas du
cas S298 à 16 cycles par le seul `step` : 97 diagnostics rendus, **âge 1 pas** pour 96 d'entre
eux, 3 appels sans retour (premiers pas), aucune attente, 68 ms au mur pour les 100 pas.
**84 pas sur 100 déclarés dégradés à 16 cycles** (franche jusqu'à 1,2·10⁻³) alors que la
trajectoire à 16 cycles suit le cœur à 5·10⁻⁵ m jusqu'à t = 1 s : la tolérance d'ADR-144 est
bien plus stricte que l'usage en hauteur — à documenter, sans la relever.
Anneau de 3 emplacements ; si aucun n'est libre, le diagnostic du pas est perdu, jamais le pas
retardé.

---
