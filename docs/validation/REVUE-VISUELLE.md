# Revue visuelle — l'utilisateur superviseur des rendus

**Ouverte S254, 2026-09-16.** L'utilisateur reprend le projet comme **superviseur des rendus
visuels** : il envoie, à la demande, des références réelles et aide à comprendre la réalité et la
perception humaine. Le travail technique reste délégué (S71).

Jusqu'ici, aucune réception perceptive n'était possible : il manquait un observateur et des
références. COUPURE-S249 (« perception non reçue »), A282 et le volet perception de B4 le
disaient. Ce document fixe comment cette capacité s'emploie sans se confondre avec une preuve
physique.

## 1. Ce qu'une revue peut établir, et ce qu'elle ne peut pas

| une revue établit | une revue n'établit pas |
|---|---|
| un **écart perçu** entre un rendu daté et une référence réelle | une précision physique : elle reste l'affaire des oracles et des bancs |
| le **seuil de perception** d'un défaut connu (visible, gênant, invisible) | une tolérance numérique, sans formule ou banc qui la dérive |
| la **priorité ressentie** entre défauts visibles | une réduction de périmètre (ADR-127) |
| ce qu'un joueur **attend de voir** d'un phénomène (écume, gerbe, reflet) | l'appartenance d'un phénomène à une couche : ADR-001 §2 |

Un verdict **qualifie l'implémentation rendue à sa date**, comme un dépassement de coût
(ADR-131). Il ne retire jamais une fonctionnalité, et il ne se « corrige » pas en déformant B ou W
hors de leur physique pour qu'ils paraissent justes.

## 2. Ce que la session envoie

Pour chaque image :

- le **fichier PNG**, converti d'un PPM de banc local (ADR-124), jamais publié ;
- la **commande** qui la reproduit, et l'**empreinte FNV** du PPM ;
- la **pose** (œil en m, lacet, tangage, champ vertical), l'**âge de scène** et la définition ;
- les **couches présentes** (B, impacts W, sillages W ; δ et V absents à ce jour) et les
  techniques actives (filtre spectral, grille du sillage) ;
- ce qui est de l'**habillage de banc** et ne relève pas du système d'eau : ciel, soleil,
  couleur de l'eau, brouillard, absence d'écume, de réfraction et de sous-surface ;
- des **questions précises**, et la liste des **références demandées**, avec les conditions
  qui les rendent comparables.

## 3. Ce qui revient

- **Références** : photo ou vidéo réelle, avec ce qu'on en sait — source, vent ou état de mer,
  hauteur et distance d'observation, vitesse du bateau, lumière. Une référence sans conditions
  connues sert à la perception, pas à la mesure.
- **Verdicts** en langage libre.

Les références ne sont **pas versionnées** — binaires, et leurs droits appartiennent à leurs
auteurs. Elles se rangent localement dans `viewer/captures/references/`, que `.gitignore`
exclut ; le dépôt garde leur description, leur source et leurs conditions.

## 4. Comment un verdict se consigne

Chaque retour devient une ligne du registre ci-dessous. Le verdict est **résumé fidèlement**, puis
**attribué** à l'une de ces classes, car chacune mène à un travail différent :

| classe | exemple | suite |
|---|---|---|
| **physique fausse** | angle du sillage, vitesse de phase, forme des crêtes | angle mort, oracle ou banc qui le confirme |
| **physique juste, perçue fausse** | houle linéaire aux crêtes trop rondes, lumière | modèle manquant nommé (couche et ADR), pas un réglage |
| **habillage de banc** | ciel, couleur, brouillard | réglage libre de l'hôte, **étiqueté comme tel** |
| **hors capacité construite** | écume, gerbe, cavité, vue sous-marine | jalon de la feuille de route qui la porte |
| **artefact numérique** | alias à l'horizon, coutures, scintillement | angle mort du rendu, mesure de l'artefact |

Un défaut d'une classe physique n'est tenu pour établi qu'après confirmation par une mesure ; le
verdict en est le **déclencheur**. Un verdict qui contredit une réception existante se consigne
tel quel, et c'est la réception qu'on réexamine d'abord.

## 5. Registre des revues

| revue | date | images | références | verdict résumé | classe et suite |
|---|---|---|---|---|---|
| **R1** | 2026-09-16 | sept rendus J1, §6 | demandées, §6.3 ; aucune reçue au premier verdict | **« La mer est trop lisse, on dirait un lac »** (22:13) | **confirmé par mesure** : physique juste mais incomplète — `mss` de B 0,0075 contre 0,044 observés (Cox–Munk), spectre coupé à `4 fp` (§7) |
| **R2** | 2026-09-16 | sept rendus aux poses de R1, queue spectrale d'ADR-155 ([empreintes](QUEUE-SPECTRALE-S256.md) §3) | — | **« Le résultat se raffine, mais le rendu paraît un grand lac soumis à beaucoup de vent ; la haute mer est plus déchaînée, chaotique, et la houle se forme vers les terres »** (22:45) | **physique juste mais incomplète**, établie par construction (§8) : mer de vent locale seule, sans houle longue, crêtes linéaires, fond uniforme ; écume hors capacité |
| **R3** | 2026-09-16 | sept rendus aux poses de R1, scène `--houle` : mer de vent + houle longue, étalement cos^2s (ADR-156, [empreintes](MER-MULTIMODALE-S259.md) §3) | **deux photographies** (§10), conditions inconnues | **« La haute mer reste trop lisse ; trop de petites bosses, pas assez de mini pics ; pics moyens et petits pics combinés, rien n'est uniforme »** (2026-09-17 07:00) | provisoire : rugosité insuffisante (A287), pentes gaussiennes, crêtes symétriques ; mesure au §10 |
| **R4** | 2026-09-17 | sept rendus aux poses de R1, scène `--vagues` : queue d'équilibre f⁻⁴ et CWM (ADR-157, [empreintes](VAGUES-POINTUES-S260.md) §3) | références A et B de R3 | **« Change le ciel et la couleur comme sur ma photo ; la mer a l'air trop rugueuse, la surface entre les pics est plutôt lisse, mais il y a beaucoup de petites vaguelettes »** (07:25) | habillage demandé (ciel, couleur) ; rugosité : trop forte et trop uniforme, à mesurer (§11) |
| **R5** | 2026-09-17 | sept rendus aux poses de R1, `--vagues --modulation --ciel-clair` : habillage de la référence A, rugosité ajustée à Cox–Munk (ADR-158, [empreintes](RUGOSITE-S261.md) §3) | références A et B | **« Trop rugueuse, trop de petits pics ; je ne connais pas le niveau de vent »** (08:09, après les réparations S262) | rugosité conforme à Cox–Munk **à 8,4 m/s** : le vent de la scène est en cause, vent de la référence inconnu ; calibration perceptive par le vent (§12) |
| **R6** | 2026-09-17 | calibration : même scène à 3, 5 et 8,37 m/s, trois poses, ciel clair (ADR-160, [empreintes](VENT-S263.md) §3) | références A et B | **reçu S265**, voir §13 *(cellule mise à jour S293)* | question : quel vent ressemble le plus à la mer attendue ? |
| **R10** | 2026-09-18 | **premier rendu de δ** : houle à crêtes longues, bande δ couplée rejouée, quatre poses × B seul / B+δ 4 ms / B+δ 16 ms, trois diagnostics d'écart (ADR-168, [empreintes](DELTA-VISIBLE-S275.md)) | demandées : houle longue sans mer de vent marquée, vue de 5–20 m, crêtes de travers | **reçu S277**, voir §15 — motif trop répétitif ; les vagues doivent interagir avec l'onde *(cellule mise à jour S293)* | questions : δ se voit-il, la limite de la bande se voit-elle, les deux pas se distinguent-ils ? |

## 6. R1 — la scène J1 telle qu'elle est, S254

```
cd viewer
cargo run --release --offline --locked -- --multi --revue
python ../outils/apercu_ppm.py captures/s254/<image>.ppm
```

Deux exécutions rendent les mêmes empreintes. GPU NVIDIA RTX 5070 Laptop, DX12, 1280×720, champ
vertical 50°, grille projetée à deux pixels, filtre spectral et grille du sillage actifs, visibilité
active.

### 6.1 Ce qui est rendu

- **B** : mer JONSWAP V1 de S201 — `Hs` 1,5 m, `Tp` 6 s, **32 composantes**, bande 0,5 à 4 `fp`,
  donc **aucune onde plus courte que 3,5 m**. Linéaire : crêtes et creux symétriques.
- **W** : trois sillages d'un journal commun (source de pression 19 620 N, σ 2 m, 3 m/s pendant
  16 s depuis x = −24 m) et huit impacts nés toutes les 4 s (S235).
- **Absents** : δ, V, écume, gerbes, cavités, ondes capillaires, réfraction, sous-surface.
- **Habillage de banc** : ciel uniforme, soleil, couleur de l'eau, brouillard d'horizon.

### 6.2 Images

| image | œil (m) | lacet / tangage (rad) | âge | contenu | empreinte |
|---|---|---|---:|---|---|
| `r1_reference_12s` | 0, −18, 7 | 0 / −0,131 | 12 s | B + W, 4 impacts, sillages | `0x7829238e42056201` |
| `r1_reference_fond_seul_12s` | 0, −18, 7 | 0 / −0,131 | 12 s | B seul | `0x275081ad983db7c9` |
| `r1_haute_12s` | 0, −40, 30 | 0 / −0,55 | 12 s | B + W | `0xe48879f40a422831` |
| `r1_plongeante_12s` | 0, 0, 90 | 0 / −1,2 | 12 s | B + W | `0x47815f87fba2073f` |
| `r1_rasante_12s` | 0, −18, 2 | 0 / −0,05 | 12 s | B + W | `0xdc2ef3e2cf2bb9de` |
| `r1_impact_proche_5s` | 0, −8, 3 | 0 / −0,3 | 5 s | B + W, 2 impacts | `0x1aa56f756745eff8` |
| `r1_large_horizon_29s` | 0, −18, 25 | 0,6 / −0,12 | 29 s | B + W, hors domaine honnête du sillage | `0xa329a85c503a2db3` |

La carte `captures/s248/mer_perturbation.png` (S248, `0x348c9e100ab10c7f`) accompagne R1 : ce
n'est pas un rendu, c'est la hauteur de W seule, vue de dessus, pour montrer où sont sillages et
impacts.

### 6.3 Questions posées et références demandées

Questions : échelle perçue (mer, lac, piscine ?) ; premier défaut qui saute aux yeux ; sillages et
impacts visibles ou non, et le seraient-ils en vrai à cette distance ; ce qui manque à l'horizon.

Références, avec leurs conditions si connues :

1. mer ouverte, **force 4 à 5** (vent ≈ 8–10 m/s, creux ≈ 1,5 m), vue depuis un pont à **2–7 m**,
   vers l'horizon ;
2. la même mer vue de **25–30 m** (passerelle, falaise) ;
3. **drone à la verticale**, 50–100 m, d'un petit bateau lent (**≈ 3 m/s, 6 nœuds**) et de son sillage ;
4. **objet tombant** dans l'eau libre, vu de 3–8 m : les anneaux, puis la gerbe ;
5. **vidéos** de 10–20 s des cas 1 et 3, pour le mouvement.

## 7. R1 — premier verdict : « la mer est trop lisse, on dirait un lac »

**Reçu le 2026-09-16 à 22:13**, sans référence jointe ; il porte sur l'ensemble des rendus.

**Hypothèse, écrite avant mesure.** B s'arrête à `4 fp` (`λ` ≥ 3,5 m) : aucune rugosité plus
courte ne module la lumière, et la surface paraît vitreuse comme un plan d'eau abrité. Classe
provisoire : physique juste mais incomplète. L'habillage (ciel uniforme, soleil fixe) y contribue
peut-être aussi, mais il ne se corrige pas avant la mesure.

**Mesure et critère** (SPEC-001 §1 sexies). Calculer la `mss` de la recette R1 (32 composantes
cuites) et celle du même spectre coupé à `4, 8, 16, 24, 32 fp`. La comparer à Cox–Munk au vent
minimal soutenant `Hs = 1,5 m`. **Défaut confirmé** si la `mss` de B est inférieure à la moitié de
la borne basse de Cox–Munk à ce vent. Sinon, le verdict se reporte sur l'habillage, et on le dit.

**Résultat** (`cargo run --release -p water-core --example rugosite_b`, 2026-09-16) :

| spectre de B | `λ` minimale | `mss` |
|---|---:|---:|
| recette R1 cuite, 32 composantes, `[0,5 ; 4] fp` | 3,51 m | **0,00753** |
| même spectre continu, coupé à `4 fp` (contrôle indépendant) | 3,51 m | 0,00752 |
| coupé à `8 fp` | 0,88 m | 0,01152 |
| coupé à `16 fp` | 0,22 m | 0,01553 |
| coupé à `24 fp` | 9,8 cm | 0,01787 |
| coupé à `32 fp` | 5,5 cm | 0,01953 |
| coupé à `57 fp` (limite gravité-capillarité) | 1,7 cm | 0,02287 |

Cox–Munk au vent minimal de `Hs = 1,5 m` (`U` = 8,37 m/s à 19,5 m ; 7,95 m/s pour tenir compte de
la hauteur) : `mss` 0,0437 à 0,0459. Borne basse 0,0397, seuil du critère **0,0199**.

- **Défaut confirmé** : 0,0075 < 0,0199. La mer rendue a une pente quadratique moyenne **5,8 fois
  plus faible** que la mer réelle de même `Hs`. Classe confirmée : **physique juste mais
  incomplète** (spectre coupé), liste du projet fini 2.1 et 8.9.
- **Constat qui n'était pas attendu** : prolonger la même queue JONSWAP en `f⁻⁵` jusqu'à la limite
  gravité-capillarité ne donne que **0,0229**, soit 52 % de l'observé. La queue réelle est plus
  raide en pente ; les modèles d'équilibre en `f⁻⁴` et les capillaires y contribuent. Prolonger la
  bande rend la mer **2,6 fois plus rugueuse** (coupure à `32 fp`), mais pas encore aussi rugueuse
  que la mer réelle.

**Suite de R1 (S256).** Le remède physique est construit : c'est la queue du même spectre, en
pentes par pixel ([ADR-155](../adr/ADR-155-queue-spectrale-en-pentes-par-pixel.md)). La `mss` passe
de 0,0075 à 0,0195. R2 est envoyée à l'utilisateur. Les deux manques restent nommés : la queue
JONSWAP n'atteint que 52 % de la rugosité observée, et la direction des composantes est liée à leur
fréquence.

## 8. R2 — « un grand lac soumis à beaucoup de vent »

**Reçu le 2026-09-16 à 22:45**, sans référence jointe. Verdict complet : le résultat se raffine,
mais la topologie d'un océan dépend de plusieurs paramètres ; le rendu paraît un grand lac soumis à
beaucoup de vent ; la haute mer semble plus déchaînée, de manière chaotique, et la houle se forme
petit à petit vers les terres.

**Ce qui se calcule, sans campagne.** Le verdict décrit exactement ce que B contient.

| fait | valeur | conséquence perçue |
|---|---|---|
| B est **une seule mer de vent**, JONSWAP unimodal (ADR-100 §1 le dit : « pas une description universelle des houles croisées ») | `Hs` 1,5 m, `Tp` 6 s | — |
| cette mer est **pleinement développée à environ 8,4 m/s** (Pierson–Moskowitz, SPEC-001 §1 sexies : `ωp = 0,877 g/U` donne `Tp` 6,1 s) | vent seul, sans histoire | une mer « de vent local », celle d'un grand lac ou d'une côte abritée |
| **aucune énergie au-delà de `2 Tp`** : la bande commence à `0,5 fp` | rien au-dessus de 12 s | **pas de houle longue** ; une houle de 12 s mesure 225 m, 4 fois la longueur d'onde dominante rendue (56 m) |
| cambrure des vagues dominantes `Hs/λp` | 0,027 | mer jeune et courte, sans le grand balancement de la houle |
| directions : un seul éventail de ±45°, lié au rang de fréquence (A287) | un système | aucune mer croisée, d'où l'aspect ordonné au lieu de « chaotique » |
| crêtes : somme linéaire de hauteurs, **aucun déplacement horizontal** dans le cœur ni dans l'hôte | crêtes et creux symétriques | pas de crêtes aiguës ni de creux plats |
| fond **profond et uniforme** (liste 2.7) | pas de bathymétrie | la houle ne peut ni se lever ni se réfracter à l'approche des terres |
| écume et moutons absents (liste 7.1) | — | pas de « mer déchaînée » à force 5 |

**Classement.** Physique juste mais incomplète, **établie par construction** : ces absences sont
lisibles dans la recette et le code, et aucune mesure ne pourrait les démentir. Aucune référence
n'ayant été reçue, l'**ampleur** du « chaotique » perçu n'est pas mesurée. Ce qui se mesurera au
premier lot : l'énergie par système (vent, houle) et leurs directions, contre une mer de référence
déclarée. Liste du projet fini : 2.2 (houle), 2.7 et 3.6 (bathymétrie, levée, réfraction), 7.1
(écume), A287 (directions).

**Ordre recommandé, par ce que chaque manque change à l'image.** (1) **Mer multimodale dans B** :
houle longue plus mer de vent, directions propres à chaque système et indépendantes de la
fréquence. Cela traite aussi les stries d'A287. (2) **Crêtes non linéaires** : déplacement
horizontal, ou ordre deux de Stokes, dont la référence existe hors runtime (SPEC-001 §1 ter).
(3) **Écume et moutons** (ADR-014). (4) **Levée de la houle sur la bathymétrie** : elle demande le
fournisseur de bathymétrie (SPEC-004 §8.3) et relève de J5.

## 9. Question de l'utilisateur : une surface plane avec normales, vue de dessus

**Question (S257).** Avait-on pensé à une surface lisse, avec effet de normales, déplacement ou une
autre technique, quand on regarde la mer de dessus, le rayon perpendiculaire à l'eau ? Une surface
plane devrait alors suffire.

**Ce que le dépôt avait prévu.** ADR-004 §4 (S03) prévoyait un **déplacement de Gerstner** pour les
crêtes et un **cache de tuile** de déplacement pour le rendu. §6.2 retenait une **tuile FFT pour le
détail haute fréquence** et une somme de Gerstner pour les composantes longues. Rien de cela n'est
construit. S234 a mesuré le LOD du **maillage** : il retire au plus 35 %, et **rien en vue haute**
(L313), parce que la grille projetée garde un pas écran constant et que le coût venait du sillage.
ADR-155 (S256) applique déjà l'idée aux ondes courtes : **normales seules, sans déplacement**.

**Le critère physique.** Sous un rayon qui fait l'angle `θ` avec la verticale, un déplacement
vertical `h` se projette sur l'image avec un décalage d'environ `h·sin θ / (d·α)` pixels. Ici `d`
est la distance et `α` l'angle d'un pixel : 1,21·10⁻³ rad en 1280×720 et 50° de champ. À hauteur
d'œil `H`, cela vaut `h·sin 2θ / (2H·α)`. À la verticale exacte, `θ = 0` : **le déplacement est
invisible, et une surface plane à normales rend la même image**. L'intuition est juste là.

Aux poses de R1/R2, avec `h = σ = Hs/4 = 0,375 m` :

| pose | `θ` | décalage pour `σ` | pour 2σ | queue (3 mm) |
|---|---:|---:|---:|---:|
| plongeante, 90 m, centre | 21° | 1,2 px | 2,3 px | 0,01 px |
| plongeante, bord | 46° | 1,7 px | 3,4 px | 0,01 px |
| haute, 30 m | 40–70° | 3,3 à 5,1 px | 6,6 à 10,2 px | 0,04 px |
| référence, 7 m, à 54 m | 82° | 5,7 px | 11,4 px | 0,05 px |
| rasante, 2 m, à 40 m | 87° | 7,8 px | 15,6 px | 0,06 px |

- **Ondes courtes** : moins d'un dixième de pixel partout. Les normales suffisent, et c'est ce que
  fait ADR-155.
- **Grandes vagues vues de haut** : le décalage maximal vaut `σ/(2Hα)`, sous un demi-pixel dès
  `H` ≈ 300 m pour cette mer, et sous 2 pixels à 90 m. Une surface plane à normales y est
  indiscernable ou presque, ce qui intéresse les vues d'avion et la grande échelle (J5).
- **Vues de jeu** (2 à 30 m) : 3 à 15 pixels, plus les silhouettes et les occultations près de
  l'horizon. Là, le déplacement est nécessaire.

**Deux réserves.** (1) Le **déplacement horizontal** des crêtes (Gerstner, Stokes) change le dessin
des crêtes même vu de dessus : un plan à normales doit l'intégrer à ses normales, ou il le perd.
(2) Le décalage ci-dessus est **statique** ; une vague qui avance produit un décalage mouvant, que
l'œil perçoit peut-être sous le pixel. Seul le verdict de l'utilisateur le dira.

**Décision.** C'est un **LOD de rendu dépendant de la vue** : déplacement là où son décalage dépasse
un seuil en pixels, normales seules ailleurs, raccord continu entre les deux. Il entre dans l'espace
d'optimisation de J1-bis (ADR-131). Il n'est **pas prioritaire pour le coût aujourd'hui** : à
1280×720, le GPU eau (1,8 ms) va surtout à la cuisson de la grille du sillage (1,25 ms). Il le
deviendra avec l'altitude et la grande échelle. Le seuil de décalage se fixera avec l'utilisateur :
c'est une question de perception.

## 10. R3 — « trop de petites bosses, pas assez de mini pics, rien n'est uniforme »

**Reçu le 2026-09-17 à 07:00**, avec **deux références** : les premières reçues.

| référence | ce qu'elle montre | conditions |
|---|---|---|
| A — proche, presque rasante | mer bleu vif sous ciel clair ; vagues courtes (quelques mètres) à **crêtes pointues** et creux plus larges ; rides fines sur les pentes ; reflets du ciel très contrastés, par taches | inconnues : vent, hauteur, distance |
| B — plus haute, vers l'horizon | mer bleu sombre ; texture fine partout, **trains de vagues** et motifs dus au vent lisibles jusqu'au loin ; horizon net | inconnues |

Sans conditions connues, elles servent à la perception, pas à la mesure (§3). Elles ne sont pas
versionnées.

**Verdict.** La haute mer reste trop lisse. Il y a trop de petites bosses et pas assez de mini pics.
De haut, on perçoit la houle et la formation des vagues grâce au vent. De près, la haute mer combine
des pics moyens et des petits pics, et rien n'est uniforme. Les stries de R2 ne sont plus
mentionnées.

**Hypothèses, écrites avant mesure** (SPEC-001 §1 sexies) :

1. **rugosité insuffisante** : `mss` 0,0195 contre 0,044, déjà mesuré (A287) ;
2. **pentes gaussiennes** : la somme linéaire annule la pointe (`c40`, `c22`, `c04`) et
   l'asymétrie (`c21`, `c03`) que Cox & Munk mesurent. Des pentes gaussiennes sont
   « uniformément rugueuses », sans facettes raides rares ;
3. **crêtes symétriques** : `λ3 = 0`, contre environ `3kσ` au second ordre.

**Mesure et critère.** On mesure, sur le champ rendu de la scène `--houle` (bande et queue, pentes
analytiques, 10⁶ points tirés dans l'espace et le temps), les coefficients de Gram-Charlier dans les
axes du vent et l'asymétrie de l'élévation.

- **Hypothèse 2 confirmée** si `c40` et `c22` mesurés sont chacun inférieurs à la moitié des
  valeurs de Cox–Munk (0,20 et 0,06).
- **Hypothèse 3 confirmée** si `|λ3|` est inférieur à la moitié de `3·k_m·σ`, avec `k_m` le nombre
  d'onde moyen pondéré par la variance.

Les mêmes grandeurs sont ensuite calculées pour deux remèdes candidats, sans les construire : le
modèle de vagues pointues de Lagrange (CWM, déplacement horizontal de chaque composante), et les
harmoniques liées du second ordre de chaque composante.

**Résultat** (`cargo run --release -p water-core --example statistiques_surface`, 2026-09-17 ; 10⁶
points sur 4 km × 4 km × 1 h ; axes du vent de la mer de vent). Cox–Munk pour `W` = 7,95 m/s :
`mss` 0,0437, `σu²` 0,0251, `σc²` 0,0183, `c21` −0,058, `c03` −0,222, `c40` 0,40 ± 0,23, `c22`
0,12 ± 0,06, `c04` 0,23 ± 0,41. Second ordre : `3·k_m·σ` = 0,156.

| modèle, même réalisation | `mss` | `σu²/σc²` | `c40` | `c22` | `c04` | `c03` | `λ3` |
|---|---:|---:|---:|---:|---:|---:|---:|
| **rendu actuel** (linéaire) | 0,0198 | 1,08 | −0,026 | −0,007 | −0,022 | 0,002 | 0,000 |
| CWM (déplacement de Lagrange) | 0,0200 | 1,08 | 0,057 | 0,023 | 0,073 | 0,002 | 0,003 |
| harmoniques liées, par composante | 0,0198 | 1,08 | −0,026 | −0,007 | −0,023 | 0,002 | 0,002 |
| queue d'équilibre en `f⁻⁴` | **0,0483** | 0,96 | −0,040 | −0,009 | −0,028 | 0,001 | 0,000 |
| **queue en `f⁻⁴` + CWM** | **0,0495** | 0,96 | **0,208** | **0,072** | **0,208** | 0,001 | 0,003 |
| `f⁻⁴` + CWM + modulation `M` = 2 (ajustée) | 0,0496 | 0,96 | 0,390 | 0,137 | 0,408 | 0,001 | 0,003 |
| `f⁻⁴` + CWM + modulation `M` ≥ 10 | — | — | — | — | — | — | **replis** |
| Cox–Munk | 0,0437 | 1,37 | 0,40 | 0,12 | 0,23 | −0,222 | ≈ 0,16 |

- **Hypothèses 2 et 3 confirmées** : `c40` −0,026 < 0,20, `c22` −0,007 < 0,06, `|λ3|` 0,0001 <
  0,078. Le rendu actuel a des pentes gaussiennes et des crêtes symétriques.
- **Chacun des deux remèdes candidats échoue seul.** CWM ne donne qu'un septième de la pointe observée.
  Les harmoniques prises composante par composante ne donnent rien : l'asymétrie du second ordre
  vient des interactions entre composantes, et non de chaque composante seule.
- **Constat principal, sans ajustement.** Prolonger la queue par l'**intervalle d'équilibre en
  `f⁻⁴`** (Toba, Phillips) plutôt qu'en `f⁻⁵` porte la rugosité à **0,048**, contre 0,044 observé (au
  bord haut de l'incertitude). **Avec CWM**, les trois coefficients de pointe tombent dans les
  incertitudes de Cox–Munk (0,21 ; 0,07 ; 0,21). Aucun paramètre n'est ajusté : CWM est la
  cinématique de Lagrange à l'ordre un (λ = 1), et l'intervalle d'équilibre est une loi publiée.
- **Ce qu'aucun candidat ne reproduit** : l'**asymétrie des pentes** (`c03` −0,22 : pentes plus
  raides sous le vent, effet du vent), l'**asymétrie de l'élévation** (`λ3`, interactions du second
  ordre entre composantes), et l'**alignement des ondes courtes sur le vent** (1,37 observé, 0,96
  ici, parce que l'étalement gelé de la queue est presque isotrope).
- Une modulation des ondes courtes par les longues atteint la pointe centrale pour `M` ≈ 2, mais
  c'est un **ajustement**. Au-delà de `M` ≈ 10, la surface se replie. Elle n'est pas retenue sans
  source indépendante.

## 11. R4 — « trop rugueuse ; lisse entre les pics ; beaucoup de petites vaguelettes »

**Reçu le 2026-09-17 à 07:25.** Le verdict porte deux demandes.

**1. Habillage.** « Change le ciel et la couleur comme sur ma photo » : la référence A montre un ciel
bleu profond, des nuages blancs bas et une eau bleu saturé. Classe **habillage de banc** : réglage
libre de l'hôte, sans physique, étiqueté comme tel. L'habillage brumeux S211 reste sélectionnable,
pour que R1 à R4 se rejouent au bit.

**2. Rugosité.** La mer a l'air trop rugueuse ; la surface entre les pics est plutôt lisse, avec
beaucoup de petites vaguelettes. Hypothèses, écrites avant mesure :

1. **excès de rugosité moyenne** : `mss` 0,0495 contre 0,0437 observé (Cox–Munk), soit +13 % ;
2. **rugosité fine trop uniforme** : l'observation décrit des facettes lisses entre les pics et des
   vaguelettes groupées, c'est-à-dire une énergie des ondes courtes **modulée** par les plus
   longues. Cox–Munk la mesure par la pointe `c40` (0,40 ± 0,23), contre 0,21 construit ;
3. **grain** des ondes proches de la résolution, entre deux et quatre pixels, perçu comme une
   rugosité uniforme.

Classe provisoire : physique incomplète (modulation absente) et possible artefact numérique
(grain). Mesure au §11 suite, avant toute correction.

**Critère de choix, écrit avant la mesure (S261).** Candidats, tous avec la queue d'équilibre et CWM
d'ADR-157 :
- coupure de la queue `b_Q` (32 ou 28) ;
- modulation de l'énergie des ondes courtes, soit par la bande seule, soit **en cascade** : chaque
  composante de la queue est modulée par toutes celles au moins quatre fois plus longues, rapport
  de séparation d'échelles déclaré et non ajusté ;
- intensité `M`.

On retient le candidat sans repli dont la `mss` tient dans ±0,004 de Cox–Munk, et dont les écarts
de `c40`, `c22` et `c04` à leurs valeurs centrales, rapportés à leurs incertitudes, ont la plus
petite somme des carrés. `M` et `b_Q` sont alors des **ajustements déclarés** contre Cox–Munk, et non
des lois. Indicateur de l'image : part de la surface où l'énergie des vaguelettes de moins de 50 cm
tombe sous la moitié de sa moyenne, c'est-à-dire la « surface lisse entre les pics ».

**Résultat** (`cargo run --release -p water-core --example modulation_rugosite`, journal
`code/target/s261-modulation.log`, 10⁶ points). Cox–Munk : `mss` 0,0437, `c40` 0,40, `c22` 0,12,
`c04` 0,23.

| candidat (queue f⁻⁴ + CWM) | `mss` | `c40` | `c22` | `c04` | score | surface « lisse » |
|---|---:|---:|---:|---:|---:|---:|
| `b_Q` 32, sans modulation (R4) | 0,0493 | 0,213 | 0,072 | 0,205 | 1,31 | 0,0 % |
| `b_Q` 28, sans modulation | 0,0434 | 0,178 | 0,067 | 0,170 | 1,75 | 0,0 % |
| **`b_Q` 28, bande, `M` = 2 — retenu** | **0,0435** | **0,353** | **0,129** | **0,351** | **0,150** | 0,3 % |
| `b_Q` 28, cascade, `M` = 1,5 | 0,0435 | 0,375 | 0,134 | 0,360 | 0,169 | 0,3 % |
| `b_Q` 28, bande, `M` = 3 | 0,0435 | 0,510 | 0,181 | 0,498 | 1,70 | 3,6 % |
| `b_Q` 28, cascade, `M` = 3 | 0,0436 | 0,727 | 0,250 | 0,677 | 7,94 | 9,7 % |

Aucun repli. Tous les candidats en `b_Q` 32 sont hors de ±0,004 en `mss`.

- **Hypothèse 1 confirmée** : la coupure à 32 fp donne 13 % de rugosité en trop. À 28 fp, 0,0435.
- **Hypothèse 2 confirmée, mais bornée par l'observation.** La modulation rapproche la pointe de
  Cox–Munk : le score passe de 1,75 à 0,15 avec `M` = 2. Mais **Cox–Munk n'autorise qu'une modulation
  modérée** : au-delà de `M` ≈ 2, la pointe dépasse l'observé. À `M` = 2, la surface où les vaguelettes
  tombent sous la moitié de leur énergie reste de 0,3 %. **Une mer franchement lisse entre les pics
  n'est pas compatible avec Cox–Munk à ce vent.**
- **Ce qui l'expliquerait sans contredire l'observation** : un **vent plus faible**. La scène est
  une mer de vent pleinement développée à environ 8 m/s ; à 4 m/s, Cox–Munk donne une `mss` de
  0,023, soit moitié moins. Ou le **grain** des ondes proches de la résolution (hypothèse 3), que
  la transition vers une rugosité de BRDF traiterait (Bruneton, Neyret et Holzschuch, 2010). Aucun des
  deux n'est construit dans ce lot : ils sont nommés.
- `M` = 2 et `b_Q` = 28 sont des **ajustements déclarés** contre Cox–Munk (critère ci-dessus).

## 12. R5 — « trop rugueuse, trop de petits pics » ; vent inconnu

**Reçu le 2026-09-17 à 08:09.** Ciel et couleur non contestés.

**Classement.** La rugosité rendue est celle que Cox–Munk mesure **à 8,4 m/s**, le vent de la mer de
la scène (S261). Le verdict ne contredit donc pas la physique de la rugosité : il dit que la mer de
la référence est **moins ventée** que la scène. Rien, dans une mer pleinement développée, ne se
règle indépendamment du vent : `Hs`, `Tp`, longueur d'onde et `mss` en découlent (SPEC-001 §1
sexies). Le vent de la référence étant inconnu, **il se choisit à l'œil**, parmi des mers conformes
aux observations à leur propre vent. C'est une calibration perceptive d'un paramètre physique, et
non un réglage de rugosité.

| vent `U` (m/s) | `Hs` (m) | `Tp` (s) | `λp` (m) | `mss` Cox–Munk |
|---:|---:|---:|---:|---:|
| 3 | 0,19 | 2,19 | 7,5 | 0,018 |
| 5 | 0,54 | 3,65 | 20,8 | 0,029 |
| 8,4 (scène) | 1,50 | 6,11 | 58 | 0,046 |


## 13. R6 — calibration du vent, reprise S264

**2026-09-17 : choix encore attendu.** Les rendus S263 sont conservés ; S264 retrouve les
21 PPM, vérifie leurs empreintes contre `viewer/captures/s263/revue.log` et vérifie que les
neuf PNG ont exactement les pixels des PPM correspondants. Aucun nouveau rendu GPU réalisé.
Le test `wind_sea_and_tail_cut_follow_observations_s263` passe sur cette reprise.

Images de référence présentées : `viewer/captures/s263/r6_v{3,5,8}_reference_12s.png`.
Empreintes respectives : `0x62bc4ad4f5af9a8f`, `0x0f178a2e5a4fcb7d`, `0x049094cca0ce54f5`.
Pose commune : œil `[0, -18, 7]` m, lacet 0, tangage `-atan(7/53)`, champ vertical 50°,
âge 12 s, définition 1280 × 720. B avec houle, mer de vent et queue ; impacts et sillages W ;
δ et V absents. Filtre spectral, grille du sillage, CWM et modulation actifs.
Ciel clair, couleur et éclairage sont de l'habillage de banc ; aucune écume construite.

Reproduction depuis `viewer/`, sur le code S263 (211aef6, inchangé en S264) :

```text
cargo run --release -- --multi --vagues --modulation --ciel-clair --vent=3 --revue=r6_v3
cargo run --release -- --multi --vagues --modulation --ciel-clair --vent=5 --revue=r6_v5
cargo run --release -- --multi --vagues --modulation --ciel-clair --vent=8.37 --revue=r6_v8
```

Question : quel vent se rapproche de la mer recherchée, et quel défaut reste visible ?
Sans ce retour, aucun vent représentatif n'est adopté. Le lot suivant dépend du défaut :
transition vers la BRDF si le grain reste gênant ; modulation selon le vent si la répartition
entre pics et zones lisses reste en cause. Ce sont des pistes à mesurer, pas des diagnostics reçus.

### Verdict R6 — reçu S265, 2026-09-17

« la surface a l'air trop rugueuse, entre les pic moyen et pic même plus petit la surface doit etre plus lisse ».
Le retour précise la répartition locale du détail ; **aucun vent choisi**. Hypothèse à éprouver :
reflets insuffisamment filtrés de la queue proche de la résolution, avant de retoucher sa modulation
physique. Le shader retire de la variance sans la transférer à l'éclairage. Lot ADR-161 : fermeture
statistique du reflet ; le témoin 5 m/s sert uniquement à isoler le changement. Les références
photographiques antérieures ne sont pas présentes dans cette copie ; leur description reste au registre.

## 14. R7 — reflets filtrés à vent identique

**S265, 2026-09-17, verdict attendu.** Comparaison R6 à 5 m/s et variante `--reflets-filtres`,
ordre 3 ; vent médian pris comme témoin, sans choix attribué à l'utilisateur. Formes géométriques,
modulation M=2 et habillage identiques. La queue proche de la résolution passe vers une moyenne
statistique du reflet (ADR-161). Image référence `r7_filtre3_reference_12s.png`, empreinte
`0xec4a559bf5ca0287` ; rasante `r7_filtre3_rasante_12s.png`, `0xe68252f14891aca9`.
Dossier `viewer/captures/s265`, commandes et limites dans [REFLETS-S265](REFLETS-S265.md).

Référence : œil `[0,-18,7]` m, lacet 0, tangage -0,13131544 rad ; rasante :
`[0,-18,2]`, lacet 0, tangage -0,05 rad. Âge 12 s, champ vertical 50°, 1280×720,
B + impacts et sillages W, sans δ/V/écume. Grille du sillage, filtre spectral et visibilité actifs.
Ciel, soleil et couleur : habillage. Le témoin R6 est identique au bit au rendu historique actuel.

Question : les zones entre les pics paraissent-elles suffisamment lisses, ou reste-t-il un excès
de petites bosses ? La fermeture gaussienne est approchée ; écart 3×3/5×5 publié, coût eau 2,49–2,71 ms
contre 1,80–1,83 ms, budget 2 ms non tenu. Pas d'adoption par défaut ni de verdict physique déduit de l'image.

### Verdict R7 — reçu S266, 2026-09-17

« Très bien continue », après comparaison avant/après à 5 m/s. **Aspect filtré accepté** dans
la vue montrée ; poursuivre la réduction du coût en conservant cet aspect. Cela ne reçoit
ni les vues non montrées, ni l'animation, ni le budget, ni la BRDF physique complète, et ne
vaut pas sélection explicite d'un vent représentatif. Lot suivant : ciel de banc précalculé,
ADR-162, contrôlé contre la version R7 acceptée.

### Conservation de R7 — fin S266, 2026-09-18

Le cache du ciel a été rejeté aux critères déclarés. L'optimisation retenue (ADR-163) regroupe
les covariances entièrement filtrées et fixe les bornes des boucles 3×3 : **sept images à un
niveau RGB près** de R7, sept témoins identiques au bit. Ce contrôle ne demande pas de nouveau
choix esthétique. Aperçu `viewer/captures/s266/r8_final_reference_12s.png`, mêmes pose, âge,
vent et couches que R7. Coût et limites : [CIEL-CACHE-S266](CIEL-CACHE-S266.md).

## 15. R10 — δ visible pour la première fois, S275

Revue demandée le 2026-09-18 : [DELTA-VISIBLE-S275](DELTA-VISIBLE-S275.md), captures dans
`viewer/captures/s275`, et commande interactive `--delta` (touche D) pour la comparaison en
mouvement. Mesure préalable : δ change 10 à 24 % des pixels de plus de 4 niveaux à hauteur d'œil
et en incidence rasante, rien vue d'en haut ; le pas d'image ne se distingue pas du pas de 4 ms.
Aucune conclusion d'invisibilité n'est tirée avant le verdict.

### Verdict R10 — reçu S277, 2026-09-18

Deux retours, tous deux sur la **scène**, aucun sur la couche δ elle-même :

1. **« Le motif de surface est trop répétitif et ne fait pas réaliste ; la mer a trop de petites
   variations. »**
2. **« Les variations, vagues et vaguelettes de la mer doivent interagir avec l'onde. »**

L'utilisateur a par ailleurs dit ne pas savoir quel retour on attendait de lui. C'est un défaut de
la demande, pas de la réponse : R10 lui a montré une scène de mesure en lui demandant un jugement
esthétique, sans dire que la mer y était volontairement appauvrie.

#### Ce que le premier retour mesure — `code/water-core/examples/bandes_s277.rs`

| | houle de `--delta` | mer de la scène S201 |
|---|---|---|
| longueurs d'onde | 40,1 à 198,7 m, rapport **5,0** | 3,7 à 210,7 m, rapport **56,2** |
| étalement des directions | **0,00°** | 87,19° |
| écart-type de `η` **le long des crêtes** | **0,0000 m** | 0,2872 m |
| écart-type de `η` en travers | 0,2906 m | 0,3048 m |

La houle de `--delta` ne contient **aucune vague de moins de 40 m** et ne varie **pas du tout** le
long de ses crêtes : le même profil de 128 colonnes est répété à l'identique sur 200 m de large.
Le constat de l'utilisateur est donc exact, et prévisible : c'est la mer la plus pauvre que le
dépôt sache produire.

Elle n'a pas été choisie pour son aspect. Le domaine δ est une **tranche 2D**,
`Domain { nx, nz, dx }` — aucune dimension `y`. Une houle étalée ne serait pas constante le long
des crêtes, et la tranche ne saurait pas la porter ; l'étalement nul est la condition pour que les
échantillons du fond soient exactement plans (`viewer/src/delta.rs`, ADR-168).

#### Ce que le second retour désigne

Les vaguelettes que l'utilisateur voit à l'écran sont la **queue spectrale S256**, un habillage de
pentes par pixel **non couplé à δ** — le protocole de S275 le dit déjà. Elles ne peuvent donc pas
interagir avec l'onde : elles ne sont pas dans le domaine simulé. Et δ n'a pas d'onde propre à
faire interagir : le domaine naît au repos sous la houle, et ne porte que la correction couplée de
celle-ci, au plus 10,6 cm pour `Hs` = 2 m.

Les deux retours désignent ainsi une seule et même limite, structurelle : **δ est aujourd'hui une
tranche verticale sous une houle idéalisée**. Ce que l'utilisateur veut voir — une onde qui
traverse une vraie mer et s'y déforme — demande δ sur les deux dimensions horizontales, c'est-à-dire
**δ général** au sens d'[ADR-127](../adr/ADR-127-ambition-complete-construction-progressive.md).

#### Ce que ce verdict ne dit pas

Il ne rejette rien de mesuré en S275 et S276 : la couche δ reste identique au bit entre rejeu et
direct, son coût reste celui de COUT-DIRECT-S276, et la question « le pas de 16 ms suffit-il ? »
reste tranchée par la mesure, pas par l'image. **Aucune conclusion d'invisibilité de δ n'est tirée**
— la scène ne permettait pas de la poser.

#### Onde injectée de S277 — sans verdict attendu (S294, 2026-09-19)

Réponse de l'utilisateur : « Jsp ». Conformément à [ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D7,
aucun verdict n'est plus attendu sur cette scène de mesure ; la question qu'elle portait — une onde
prise dans une vraie mer — reviendra en revue sur la scène 3D de la porte B.

## 16. R11 — δ sur les deux dimensions, sur une vraie mer, S302

**2026-09-20, verdict attendu.** C'est la revue que le [verdict R10](#verdict-r10--reçu-s277-2026-09-18)
appelait : δ n'est plus une tranche, la mer n'est plus appauvrie, et une onde traverse le domaine.
Protocole, mesures et limites : [SCENE-DELTA3D-S302](SCENE-DELTA3D-S302.md). Images dans
`viewer/captures/s302` ; commande interactive `--houle --delta3d`, touche **D** pour basculer.

**Ce qui est simulé, et ce qui ne l'est pas.** R10 avait montré une scène sans dire ce qu'elle
contenait ; ici c'est dit avant la question. La mer (64 composantes, `Hs` ≈ 2,5 m, la plus courte à
3,5 m de longueur d'onde) et l'onde injectée (65 cm, 16 m, cambrure 0,26) sont **simulées et
couplées** dans un domaine de 30 × 28 m à 25 cm de maille, un pas par image. Les **vaguelettes**
fines que l'on voit partout sont la queue spectrale de S256 : un **habillage par pixel, non
couplé** — elles ne réagissent pas à l'onde, et ce lot ne le prétend pas. Hors du domaine, la mer
est B seul, comme avant.

**Ce que les chiffres disent déjà** : l'onde traverse à 2,2 m/s (théorie 2,5), garde 0,46 m à
mi-course, et la couche change 13,5 à 15,7 % des octets de l'image. Le grain à l'échelle de la
maille vaut 2 mm d'écart-type — un cinquième de la signature de l'onde — et vient du schéma, pas
du budget de calcul (mesuré de 32 à 512 cycles).

**Les questions, dans l'ordre où elles comptent :**

1. **L'onde est-elle crédible dans cette mer ?** En basculant D, la voit-on comme une vague qui
   appartient à la scène, ou comme une pièce rapportée ?
2. **L'interaction est-elle celle que vous attendiez en R10** — l'onde qui se déforme sur les
   vagues, et la mer qui la porte — ou manque-t-il encore quelque chose de précis ?
3. **Voyez-vous un grain, des piqûres ou des scintillements** à petite échelle dans le rectangle du
   domaine, que l'on ne voit pas ailleurs ? (Nous en avons mesuré : 2 mm, localisés.)
4. **Le bord du domaine se remarque-t-il ?** Il doit se fondre en 3 m ; un raccord visible est un
   défaut à corriger.

Une réponse d'un mot suffit pour chacune ; « je ne sais pas » est une réponse utile, et dit que la
question est mal posée. Aucun verdict n'est déduit des images par nous : la mesure dit ce qui est
mesurable, le jugement vous appartient.

### Verdict R11 — reçu S303, 2026-09-20

Quatre réponses, dans l'ordre des questions posées :

1. **« Je ne sais pas s'il s'agit d'une onde circulaire ou bien linéaire, car dans la scène il se
   forme une vague qui va uniquement dans un sens. »** L'onde est **linéaire** — un front injecté,
   qui se propage dans une seule direction ; la scène ne le disait pas et ne montrait pas son
   origine. Défaut de la **scène**, pas du solveur : une onde circulaire née d'un impact serait
   plus lisible, et c'est d'ailleurs le cas d'usage de J2 (gerbe d'impact).
2. **« Je ne vois pas d'artefact visuel. »** Le grain de maille de 2 mm mesuré en S302 (§3 de la
   preuve) **ne se voit pas**. A297 reste ouverte comme défaut de schéma, mais elle n'est pas
   visible à cette échelle et sur cette scène.
3. **« Pas de problème sur la transition. »** Le fondu de 3 m au bord du domaine tient.
4. **« La mer ne fait pas réaliste […] la topologie est à revoir […] selon moi déjà les micro
   vaguelettes ou pics doivent être convexes plutôt que concaves »**, avec consigne de rechercher
   par moi-même. C'est le retour de fond, et il est exact.

**Ce que la mesure a confirmé** ([ANATOMIE-SURFACE-S303](ANATOMIE-SURFACE-S303.md)) :

- **Faute de protocole de ma part** : la scène montrée tournait sous `--houle` **seule**, sans la
  queue d'équilibre ni les vagues pointues de S260–S261. L'utilisateur a jugé la plus pauvre des
  trois mers que le dépôt sait produire. Toute revue de mer doit désormais déclarer, dans la
  demande, **quelles options de topologie sont actives**.
- **Le fond du retour tient même pour la meilleure variante** : asymétrie verticale `Sk` = 0,003
  contre 0,156 attendus, asymétrie des pentes `c₀₃` = 0,001 contre −0,222 (Cox–Munk). Crêtes et
  creux également arrondis : « concave » est le mot juste.
- Deux modèles mesurés y répondent — second ordre en bande étroite (Tayfun) et modulation
  **retardée** de la queue —, décidés par [ADR-176](../adr/ADR-176-asymetries-de-la-surface-rendue.md).

**Ce que ce verdict reçoit** : l'absence d'artefact et la transition du domaine δ. **Ce qu'il ne
reçoit pas** : la porte B, dont le critère 3 demande une mer jugée convaincante — elle ne l'est pas
encore. La revue reviendra (R12) avec les asymétries construites, et la scène dira ce qu'elle
montre.

## 17. R12 — la mer avec ses asymétries, S304

**2026-09-20, verdict attendu.** Réponse au verdict R11 : « la topologie est à revoir […] les micro
vaguelettes ou pics doivent être convexes plutôt que concaves ». Construction :
[ADR-176](../adr/ADR-176-asymetries-de-la-surface-rendue.md) ; réception et chiffres :
[ASYMETRIES-S304](ASYMETRIES-S304.md). Images dans `viewer/captures/s304`, 1280×720, âge 12 s,
**les quatre poses de R11**.

**Les options sont déclarées — c'est la leçon de R11.** Trois états de la même mer, le nom du
fichier portant le sien :

| état | ce qui est actif | pourquoi il est là |
|---|---|---|
| `a_houle_seule` | mer multimodale seule | **c'est ce que R11 a montré** : sans la queue d'équilibre ni les vagues pointues construites en S260–S261. Ma faute de protocole |
| `b_vagues_modulation` | + queue d'équilibre, vagues pointues, modulation | le meilleur d'avant ce lot |
| `c_asymetries` | + second ordre par système, modulation retardée | **ce lot** |

**Ce qui est simulé** : la mer entière — houle (`Hs` 2 m, `Tp` 12 s) et mer de vent (`Hs` 1,5 m,
`Tp` 6 s), 64 composantes, plus une queue spectrale de 60 lignes rendue en pentes par pixel. Cette
queue reste un **habillage non couplé** : elle donne le grain, pas la dynamique. Aucune couche δ
n'est rendue ici : la question posée est celle de la mer.

**Ce que la mesure dit déjà**, sur la même réalisation : l'asymétrie verticale passe de 0,003 à
**0,066** (observé ≈ 0,156), l'asymétrie des pentes de 0,001 à **−0,155** (observé −0,222), et
`c₂₁` tombe sur la valeur de Cox–Munk sans avoir été visée. Coût : +0,25 %.

**Les questions :**

1. **Entre `b` et `c`** : les crêtes vous paraissent-elles plus justes — pointues en haut, creux
   plus larges — ou ne voyez-vous pas de différence ?
2. **Entre `a` et `c`** : est-ce l'écart que vous attendiez après votre retour, ou la mer reste-t-elle
   « pas réaliste » pour une raison que vous pouvez nommer ?
3. **Le grain** : les vaguelettes, prises seules, vous semblent-elles encore trop régulières ou
   trop uniformément réparties ?
4. **Ce qui manque encore**, si vous le voyez : nous savons qu'il manque les capillaires parasites
   (le fin grain sur la face avant des vagues courtes), l'écume et le micro-déferlement.

Un mot par question suffit ; « je ne sais pas » reste une réponse utile.

## 18. R13 — d'où viennent les stries, et faut-il en enlever, S306

**2026-09-20, verdict attendu — et il ne remplace pas celui de R12**, qui porte sur la forme des
crêtes et reste demandé (§17). R13 pose une question différente, née du
[guide que vous avez transmis](../sources/guide_topologie_ocean_haute_mer_plage.md) : il place les
**normales fines** et l'**environnement lumineux** avant la topologie dans les causes d'un rendu
trop strié, et demande (§12.3) un test à quatre sorties que le dépôt n'avait jamais fait.

Il est fait. Mesures et méthode :
[STRIES-S306](STRIES-S306.md) ; lecture du guide :
[LECTURE-GUIDE-OCEAN-S306](../registres/LECTURE-GUIDE-OCEAN-S306.md).
Images dans `viewer/captures/s306`, 1280×720, âge 12 s, poses `proche` et `rasante` de R11/R12.

**Les options sont déclarées** (ADR-176 D5). Toutes les images partagent le même instant, la même
caméra, la même mer — `--vagues --modulation`, c'est-à-dire **l'état `c` de R12** :

| image | ce qu'elle montre | ce qu'elle sert à voir |
|---|---|---|
| `i_hauteur` | la hauteur seule, fausses couleurs, ±3 m, **aucun matériau** | les grandes masses que la géométrie porte vraiment |
| `ii_normales_geometriques` | les normales de la bande résolue, **sans la queue** | la forme, sans aucun détail fin |
| `iii_materiau_sans_queue` | le rendu, **queue spectrale éteinte** | l'eau sans ses micro-pentes |
| `iv_rendu_complet` | le rendu complet — **l'image de R12 au bit** | la référence |
| `coupure2 / iv_rendu_complet` | complet, coupure spectrale **élargie** (`--coupure=2`) | la même mer avec deux fois moins de stries |

**Ce que la mesure dit déjà.** La queue spectrale porte **80 à 85 %** de l'énergie haute fréquence
de l'image, et le contraste global ne le voit pas (il *baisse* de 51,41 à 51,29). Élargir la
coupure divise cette énergie par **2,2** (`--coupure=2`) ou **3,6** (`=3`) **sans changer le
contraste** et en coûtant **6 % de GPU en moins**. Et notre variance de pente totale est **13,7 %
au-dessus** de l'observation (Cox–Munk), la queue en portant 60 %.

**Ce que la mesure ne dit pas** : si c'est plus beau. Élargir la coupure enlève aussi du
micro-détail réel — une vraie mer porte cette variance de pente. Le compensateur correct est de
transférer ces pentes vers le **reflet** plutôt que de les supprimer (ADR-161, guide §5.3), et il
n'est pas construit. C'est donc un arbitrage visuel, et il vous revient.

**Les questions :**

1. **Entre `iv_rendu_complet` et `coupure2`** : laquelle ressemble le plus à une mer ? Ou
   l'écart est-il trop faible pour trancher ?
2. **`iii_materiau_sans_queue`** : trop lisse, ou au contraire plus juste que le complet ?
3. **`i_hauteur`** : les grandes masses vous paraissent-elles présentes ? C'est la question que le
   guide met en premier — si elles manquent ici, aucun réglage de détail ne les fera apparaître.
4. **Si vous choisissez une coupure**, laquelle : 1 (actuelle), 1,5, 2, 3 ? Un ADR l'actera.
5. Ce guide est-il votre **réponse à R12**, ou attendez-vous encore que je pose la question des
   crêtes séparément ?

Un mot par question suffit ; « je ne sais pas » reste une réponse utile.

## Verdict R12 / R13 — reçu S307, 2026-09-20

**« Le rendu actuel est toujours mauvais. »** Et, dans la même demande : aller **au-delà** du
guide, lire ses références, puis les références de ses références, et **multiplier les étapes**.

**Ce que ce verdict tranche.** Les deux revues reçoivent la même réponse, et c'est la réponse la
plus informative possible : ni ADR-176 (les asymétries de la forme, S304) ni la coupure spectrale
(la bande de rendu, S306) ne suffisent. Deux lots successifs ont chacun mesuré et corrigé un
défaut réel, et l'image reste mauvaise. **La cause dominante n'a donc pas encore été touchée.**

**Ce qu'il ne tranche pas**, et qu'il faut se garder d'inventer : quel défaut précis l'utilisateur
voit. Aucune des questions de §17 ni de §18 n'a reçu de réponse séparée. Une session qui choisirait
un coupable sans mesure referait exactement ce que S304 et S306 ont fait.

**Ce que la demande impose, et c'est une consigne de méthode** : cesser de raffiner ce qui est
déjà construit, et aller chercher dans la littérature **ce que notre rendu ne contient pas du
tout**. Un défaut qui résiste à deux corrections mesurées est plus probablement un terme absent
qu'un terme mal réglé.

Suite : S307, recherche à trois niveaux, puis mesure de l'écart, puis décision.

## Règle de protocole — S307, 2026-09-20

**Une demande de revue se rend avec `--meilleur`, et publie la ligne d'options complète.**

*Pourquoi cette règle existe.* Trois revues consécutives ont été envoyées avec des options que
l'utilisateur avait **déjà acceptées**, éteintes :

| revue | manquait | accepté depuis |
|---|---|---|
| R11 | `--vagues` (queue d'équilibre, vagues pointues) | S260–S261, trouvé en S303 |
| R12, R13 | `--ciel-clair` | **S261, construit d'après la photo de référence de l'utilisateur** |
| R12, R13 | `--reflets-filtres` (ADR-161) | **accepté par l'utilisateur en R7**, S266 |

Trois fois, la session avait déclaré les options qu'elle **avait en tête** — celles de son propre
lot — et oublié les autres. ADR-176 D5 demandait de déclarer les options : c'était insuffisant,
parce qu'une liste qu'il faut penser à écrire est une liste qu'on oublie.

*Le remède est du code, pas une consigne.* `--meilleur` active tout ce qui est accepté ; la
liste vit dans `viewer/src/main.rs`, à côté du code qui la consomme, et **une option acceptée s'y
ajoute le jour où elle est acceptée**. Chaque ligne de capture publie désormais `cwm`,
`modulation`, `asymetries`, `ciel_clair`, `reflets_ordre`, `queue` et `coupure` — pas un
sous-ensemble choisi par la session.

*Ce qui reste à la charge de la session* : dire ce que la revue **compare**, et pourquoi. Les
options ne sont plus une question ; la thèse en reste une.

## 19. R14 — le rendu avec tout ce que vous aviez accepté, et la couleur dérivée, S307

**2026-09-20, verdict attendu.** Cette revue **annule et remplace** R12 et R13 : leurs images
étaient rendues avec des options que vous aviez déjà acceptées, **éteintes** (voir la règle de
protocole ci-dessus). Les juger n'avait pas de sens, et votre « toujours mauvais » portait en
partie sur cela.

Images dans `viewer/captures/s306`, 1280×720, âge 12 s, poses `proche` et `rasante`.
Mesures : [RENDU-ECART-S307](RENDU-ECART-S307.md) ; décision :
[ADR-177](../adr/ADR-177-couleur-du-corps-d-eau-derivee-de-ses-sources.md).

**Options déclarées — et désormais produites par le code, pas par ma mémoire.** Toutes les
images partagent `--meilleur`, c'est-à-dire `--vagues --modulation --ciel-clair
--reflets-filtres` : mer multimodale, queue d'équilibre, vagues pointues, modulation, asymétries
d'ADR-176, ciel construit d'après votre photo, reflets filtrés d'ADR-161.

| image | ce qui s'ajoute | pourquoi |
|---|---|---|
| `tout` | rien — `--meilleur` seul | **la référence honnête** : ce que le dépôt savait déjà faire |
| `eau_g1` | `--eau-physique=1` | couleur dérivée de Pope & Fry 1997 et Morel 1974, réflectance **nue** |
| `eau_g2` | `--eau-physique=2` | la même, gain 2 |

**Ce que la mesure dit déjà.** Notre ancienne couleur d'eau était **9 fois trop verte** (B/G 1,24
contre 10,9 attendu) et n'avait aucune provenance. Les reflets filtrés, éteints depuis R7,
divisent l'énergie haute fréquence de l'image par **2,5**. Le ciel oublié remplaçait une brume à
6 km par une brume à **500 m** sur une scène qui porte à 1 500 m.

**Ce qui manque encore, et que je n'ai pas construit** : l'**écume** (0,42 % de couverture
attendue à ce vent, totalement absente), la **diffusion sous la surface aux crêtes** (le masque
existe pourtant déjà — le jacobien), un **ciel physique** et une **exposition** (A299).

**Les questions :**

1. **Entre `tout`, `eau_g1` et `eau_g2`** : laquelle est la plus proche d'une mer réelle ? Si
   c'est `eau_g1` ou `eau_g2`, je fais de la couleur dérivée le défaut.
2. **Le gain** : si aucune des deux ne va, plus sombre ou plus clair ?
3. **Ce qui reste faux**, en un mot si possible : la forme des vagues ? la lumière ? le
   scintillement ? le manque d'écume ? autre chose que vous pouvez nommer ?
4. **Avez-vous une photographie** de la mer que vous visez — même approximative ? Sans vent,
   exposition ni focale connus, toute comparaison reste qualitative, et c'est aujourd'hui la
   limite la plus dure du dispositif.

Un mot par question suffit ; « je ne sais pas » reste une réponse utile.

## Verdict R14 — reçu S308, 2026-09-20, avec la photographie de référence

**Reçu.** Trois réponses, et un arbitrage qui vaut plus que les trois.

1. **La troisième image est la plus proche d'une mer réelle** (`--meilleur --eau-physique=2`).
   Les deux autres gardent « une impression de matériau trop bleu / trop propre ».
2. **Plutôt plus sombre que plus clair** — et la formulation décrit un mécanisme, pas un goût :
   « pas uniformément plus sombre, mais avec des creux plus denses, un premier plan plus profond,
   et un contraste local plus marqué entre faces éclairées et zones ombrées. […] une eau plus
   sombre, plus neutre, dont les **reflets clairs restent portés par le ciel et la géométrie, pas
   par une couleur de base trop élevée**. » Pas de bleu saturé ni d'outremer.
3. **La photographie de référence est fournie.** Cible visuelle utile, **pas** cible physique
   calibrée : vent, focale, exposition, heure, état du ciel et réponse capteur restent inconnus.

**Arbitrage, et il ferme un sujet.** « Le problème principal n'apparaît plus comme topologique.
La géométrie semble suffisante pour poursuivre. Le travail prioritaire est désormais **optique** :
ciel / exposition / absorption-couleur / diffusion aux crêtes / écume / gestion des hautes
fréquences dans le reflet. » Et explicitement : **ne pas ouvrir de nouvelle correction de forme**
avant d'avoir terminé ces briques ; une critique résiduelle sur les crêtes ne redeviendra
informative qu'après.

**Ce que cela décide pour le dépôt** :

- la troisième image est la **base courante** ;
- le point D2 d'[ADR-177](../adr/ADR-177-couleur-du-corps-d-eau-derivee-de-ses-sources.md) est
  tranché dans son principe — **le gain baisse**, la luminosité vient du ciel et de la géométrie ;
  la valeur exacte reste à mesurer contre la photographie ;
- les lots de forme (noyau exact du second ordre, capillaires parasites, asymétrie horizontale)
  sont **suspendus**, pas retirés ;
- la photographie ne peut pas servir de cible physique, mais elle peut servir de **cible de
  statistiques d'image** — c'est ce que S308 construit.

## Clôture du lot de rendu — S308, 2026-09-20

**Il n'y aura pas de R15.** En cours de session, l'utilisateur a redirigé le projet vers la
physique ([ADR-178](../adr/ADR-178-strategie-en-trois-systemes-physiques.md)) : « La troisième
image de R14 constitue notre **référence interne provisoire** pour l'océan. Nous n'avons pas
besoin de poursuivre immédiatement son perfectionnement photoréaliste. »

**La mesure dit la même chose, et c'est ce qui rend la clôture propre.** S308 a transformé la
photographie de référence en cible chiffrée, puis a cherché la meilleure courbe de tonalité contre
elle sur 6 300 réglages : **les huit meilleurs donnent tous le même contraste local, 0,311–0,318
pour 0,455 mesurés sur la photographie**, et c'est pour tous la cible la plus dure. Une courbe de
tonalité est une fonction point à point ; le contraste local est une grandeur spatiale. **Ce qui
manque n'est pas dans l'optique, c'est dans la mer** — la structure de surface à quelques pixels
d'échelle, c'est-à-dire un lot de forme, que l'utilisateur avait suspendu à R14.

Le lot se rouvrira quand les systèmes physiques l'alimenteront (ADR-178 D2). Il rouvrira **avec sa
cible** : `outils/cible_image.py`, `outils/courbe_tonalite.py` et les mesures de la photographie
restent dans le dépôt, inchangées.

**Règle de protocole ajoutée** : comparer deux rendus demande `--horizon=<y>` **forcé**. La
détection automatique s'est trompée deux fois, en silence et avec un résultat plausible (A301).

## 20. R15 — la porte D, une coque sur la houle, S333

Revue de la **porte D**, non du lot de rendu clos en S308. Images : [PORTE-D-S333](PORTE-D-S333.md) §3 —
`viewer/captures/s333`, quatre instants, B + δ à l'échelle et B + 5·δ, carte de δ. Questions posées : les
anneaux sont-ils crédibles (espacement ≈ 3 m, dix centimètres au plus près de la coque) ; la perturbation
se perçoit-elle à l'échelle ; le bateau qui pilonne et cavale sans rouler est-il plausible. Référence
demandée : une vidéo d'un ponton, d'une barge ou d'une barque qui pilonne en eau calme ou sur une houle
faible, taille de l'objet, hauteur et distance d'observation connues.

**Retour du 2026-09-24, 08:00** : *« Continue, pour la référence je n'ai pas trouvé »*. Aucune référence,
aucun verdict sur les images : la porte D reste **en attente de son verdict** (ADR-178 D3). Le travail
continue sur son chemin — S334, A317, qui change l'amplitude des anneaux (§6 de la preuve). Un verdict sur
les images seules, sans référence, reste possible : il qualifierait la perception, pas la physique.

**Verdict R15 — reçu le 2026-09-24 (S337)**, sur les images de S336 (coque amortie, couvercle partiel) :
*« 1. Oui »* — le bateau lâché qui se pose en 3 à 4 s sur la houle est juste ; *« 2. On voit la coupure
encore »* — le bord de la grille de δ se voit, pli droit au loin ; *« 3. Pas forcément »* — pas d'autre défaut.
**Classe** : bord de domaine, non physique de l'eau — le banc linéaire de la porte D a des murs et ajoute δ
jusqu'au dernier rang de mailles, là où la production amortit ses ondes par une éponge et fond δ en cosinus
sur la largeur de celle-ci, ce que R11 a jugé sans raccord visible. **Suite** : S337, éponge du mode linéaire
et fondu de composition, images refaites ([PORTE-D-S333](PORTE-D-S333.md) §8).

**Verdict final — reçu le 2026-09-24 (S338)**, sur les images de S337 (éponge et fondu) : *« Plus de coupure »*.
Avec R15 — le bateau qui se pose est juste, pas d'autre défaut —, **la porte D est reçue sur la référence CPU**
(ADR-178 D3) : partie numérique [PORTE-D-S333](PORTE-D-S333.md) §1–8 et [RAYONNEMENT-COQUE-S336](RAYONNEMENT-COQUE-S336.md),
réception au §9. Ce verdict qualifie la perception, sans référence réelle ; il ne juge ni la mer de B, que la
porte B attend encore, ni la production GPU, où la coque n'entre pas encore.

## 21. R16 — une onde née d'un point, sur la mer de R14, S339

**Porte B, critère 3** : *« une onde traverse une mer étalée et s'y déforme »*, jugée convaincante. Mer : la
troisième image de R14 (`--meilleur --eau-physique=2`), votre référence provisoire. Images `viewer/captures/s339`,
1280×720 ; preuve [SCENE-DELTA3D-S302](SCENE-DELTA3D-S302.md) §8.

Ce que R11 n'avait pas : **l'origine de l'onde**. Deux sources ont été essayées :

- **un impact réaliste** — un cratère de 65 cm de creux sur 5 m, lâché au repos. Le creux se voit ; ses anneaux,
  10 à 17 cm, se perdent dans une mer de 2,5 m de hauteur significative (`s339_rasante_avec_1.0s`) ;
- **un anneau préparé**, comme le front de R11 l'était : 41 cm pour 10 m de longueur d'onde, autour d'un creux
  (`s339a_proche_avec_1.0s`, `…_2.0s`, `s339a_haute_avec_1.0s`).

**Les questions :**

1. **L'anneau** : se lit-il comme une onde circulaire née d'un point, qui traverse la mer et s'y déforme ? Est-ce
   convaincant ?
2. **La fine ligne claire** qui suit sa crête, vue de haut : défaut, ou pas ?
3. **L'impact réaliste** : que ses anneaux se perdent dans une mer de 2,5 m vous paraît-il juste, ou doivent-ils
   se voir ?
4. **Autre défaut** — la mer, la jonction, la lumière ?

Un mot par question suffit.

**Verdict R16 — reçu le 2026-09-24 (S340)** : *« Tout parrait bon visuellement »*. Lu sur les quatre questions :
l'anneau se lit comme une onde circulaire qui traverse la mer et s'y déforme, et convainc ; la fine ligne claire
de sa crête n'est pas un défaut ; que les anneaux d'un impact réaliste se perdent dans une mer de 2,5 m est
juste ; aucun autre défaut. **Classe** : aucune — réception. **Le critère 3 de la porte B est reçu** (ADR-175 §4) ;
restent, pour elle, les cas 1 et 2 sur la production (critère 2).

## 22. R17 — δ à 30 Hz contre 60 Hz, S347

**Porte C.** ADR-012 §7 fait tourner δ à 30 Hz, le rendu interpolant : c'est la seule cadence qui, étalée sur deux
images, passe sous 2 ms par image. Mais à 30 Hz une onde forte garde quelques pour cent d'amplitude de plus
qu'à 60 Hz — un amortissement numérique par pas (A318, [preuve](COUT-DELTA3D-S341.md) §8–9) ; aucune des deux n'est
« la vraie ». Images `viewer/captures/s347`, la scène de la porte B — le front de S302, la mer de R14 —, aux mêmes
instants (2, 5, 8 s) et aux mêmes poses, δ à 60 Hz (`s347_60hz_*`) et à 30 Hz (`s347_30hz_*`). À l'œil de la
session : les mêmes vagues ; la différence amplifiée ×6 est un grain fin sur l'emprise de δ.

**Les questions :**

1. **Voyez-vous une différence** entre les images à 60 Hz et à 30 Hz ?
2. **Si oui**, laquelle paraît la plus juste ?

Un mot par question suffit.

**Verdict R17 — reçu le 2026-09-24 (S348)** : *« Continue je valide »*. La cadence de 30 Hz d'ADR-012 §7 est
**validée à l'œil** : l'écart d'amplitude d'A318 — quelques pour cent sur une onde forte — n'y fait pas obstacle.
**Classe** : aucune — réception. Suite : le pas de δ étalé sur deux images (porte C).

## 23. R18 — δ à 30 Hz interpolé, en direct, S353

**Liste 8.7, porte C.** Depuis R17, δ tourne à 30 Hz ; mais le rendu, à 60, montrait deux fois le même pas : une
image sur deux immobile, mesurée ([preuve](COUT-DELTA3D-S341.md) §12, critère 3). Le rendu **mélange** maintenant les
deux derniers pas : chaque image avance d'un demi-pas, comme à 60 Hz. Cette revue se fait **en direct**, pas sur des
images fixes — c'est le mouvement qui se juge :

```text
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --eau-physique=2 --delta3d --anneau --pas-delta=33333
```

L'anneau part du fond de la scène vers la caméra. Touches : **`I`** coupe ou rétablit l'interpolation ; **`R`** relance
l'anneau ; **`D`** montre la mer sans δ ; flèches pour bouger ; Échap pour quitter.

**Les questions :**

1. **Interpolation active** (au lancement) : l'anneau bouge-t-il **sans à-coups**, aussi fluide que la mer autour ?
2. **Avec `I`** (interpolation coupée) : voyez-vous une **saccade** sur l'anneau ?
3. **Autre chose** qui gêne ?

Un mot par question suffit.

**Verdict R18 — reçu S369, 2026-09-26** : *« Rendu convaincant »*. δ à 30 Hz, interpolé au rendu, jugé en direct : reçu
([ADR-197](../adr/ADR-197-reponses-du-2026-09-26.md) D8) ; 8.7 n'attend plus que le verdict de sa frontière.

## 24. R19 — la mer de B dans Godot, contre l'afficheur, S357

**[ADR-192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md) D2, liste 8.1 et 8.4.** Le premier pas du rendu dans
Godot : **la même mer** — celle de `--meilleur`, ses composantes exportées du cœur —, **au même instant** (12 s), **aux
mêmes poses** (proche, rasante). Dans Godot : son éclairage, ses reflets, sa tonalité AgX, sa perspective aérienne ; le
ciel aux couleurs du ciel clair de R14 ; l'écume des crêtes, écume fraîche (0,55), et la lumière des crêtes de S356
([preuve](RENDU-CRETES-S356.md)). Dans l'afficheur : le rendu que R14 a retenu, avec la même écume et les mêmes crêtes.

| pose | afficheur | Godot |
|---|---|---|
| proche | `viewer/captures/s357/afficheur_proche_12s.png` | `viewer/captures/s357/godot_proche_12s.png` |
| rasante | `viewer/captures/s357/afficheur_rasante_12s.png` | `viewer/captures/s357/godot_rasante_12s.png` |

En mouvement : `cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --export-godot`, puis
Godot 4.4.1 sur le dossier `godot` — touches 1 à 4 pour les poses, Échap pour quitter.

**Les questions :**

1. **Lequel est le plus proche d'une mer réelle** : l'afficheur ou Godot ? Godot mérite-t-il qu'on y porte le reste ?
2. **Dans Godot, qu'est-ce qui gêne le plus** : la couleur de l'eau, le ciel, les reflets, l'écume, l'horizon, le manque
   de détail des vagues, autre chose ?
3. **L'écume** : les taches blanches sont-elles crédibles — taille, nombre, blancheur ?
4. **Une référence réelle**, si vous en avez une : une photo ou une vidéo d'une mer de force 4 (vent d'environ 8 m/s,
   quelques moutons), avec l'heure et la hauteur de prise de vue si possible.

Un mot par question suffit.

**Verdict R19 — reçu le 2026-09-25 (S359)** : *« selon moi la mer n'est pas du tout crédible, mais c'est pas grave on
continue, car je pense qu'il manque plein de chose avec la trnasparence en fonction de la prfondeur etc... »*. Lu sur
les questions : ni l'afficheur ni Godot ne convainquent ; **on continue dans Godot** (ADR-192 inchangé) ; le manque
nommé est l'**épaisseur de l'eau** — ce que la lumière y fait selon la profondeur. **Classe** : défaut de rendu,
liste 8.5 (*absent*). **Mesure qu'il déclenche** (S359, `outils/horizon_mer.py`) : sous l'horizon, au rasant, la
mer de Godot renvoie **0,15 à 0,18** de la luminance du ciel qui la surplombe, l'afficheur **0,71 à 0,73** ; bornée
à 0,05, la rugosité confiée à Godot remonte ce rapport à 0,41–0,48 — elle en explique une grande part, pas tout.
Suite : S359, la colonne d'eau et la réflexion portées dans le nuanceur.

## 25. R20 — l'eau a une épaisseur, S359

**Liste 8.5, après R19.** Dans Godot, trois changements ([preuve](EPAISSEUR-EAU-S359.md)) :
**la réflexion** est celle de l'afficheur — son ciel clair avec nuages, Fresnel, la pente non résolue intégrée — : sous
l'horizon la mer renvoie 0,71 du ciel au lieu de 0,11 ; **la colonne d'eau** — la lumière absorbée et diffusée selon la
profondeur (Maritorena, eau pure de Pope & Fry) et le fond vu par réfraction ; **une scène côtière** — du sable de 6 m
sous la caméra à 40 m, puis le large. Les vagues ne sentent pas le fond (liste 2.7) ; pas de caustiques.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s359/godot_proche_12s.png` | le large, pose proche de R19 — à comparer à `s357/godot_proche_12s.png` |
| `viewer/captures/s359/godot_rasante_12s.png` | le large, rasant |
| `viewer/captures/s359/godot_proche_cote_12s.png` | la côte, pose proche : le sable à 6–10 m sous la caméra, la cassure au loin |
| `viewer/captures/s359/godot_plongeante_cote_12s.png` | la côte, vue plongeante de 12 m : 8 à 15 m d'eau |

En mouvement : Godot 4.4.1 sur le dossier `godot`, `-- --cote` pour la côte ; touches 1 à 5.

**Les questions :**

1. **Le large** : plus crédible que R19 ? Qu'est-ce qui gêne encore le plus ?
2. **La côte** : l'eau paraît-elle transparente, avec un fond dessous ? La couleur au-dessus du sable — trop bleue, pas
   assez verte, juste ?
3. **La suite** : les taches de lumière sur le fond (caustiques), les vagues qui sentent le fond et déferlent, l'écume,
   autre chose ?
4. **Des références**, si vous en avez : une photo d'eau claire au-dessus du sable, à quelques mètres de fond (heure,
   profondeur si connue) ; une photo de mer du large prise d'un bateau, près de l'horizon.

Un mot par question suffit.

**Verdict R20 — reçu le 2026-09-25 (S360)**, question par question :
1. *« Tu as raison sur le ciel, il n'aide pas au reflets et limite la qualité du rendue final »* ; et une capture d'une
   partie de l'eau (`viewer/captures/s359/r20_capture_utilisateur.jpg`, locale) : *« je ne parviens pas avec les mots
   mais ce rendue du point de vue topologie est pas réaliste »*.
2. *« La couleur me paraît parfaite sincèrement »* — la couleur de l'eau (ADR-177, S359) est **reçue**.
3. *« Tente les caustique, mais pour l'ecume je ne parviens pas a comprendre car l'ecume n'apparaît presque jamais sur
   le vaguelettes uniquement sur des grandes vagues avec déferlement mais très rare voir quasi impossible »*.
4. *« Je n'ai pas mais tu peux faire tes recherches »*.

**Classe** : défauts de rendu — la forme fine de la surface (8.9), l'écume à la mauvaise échelle (8.4), le ciel (8.8) ;
une demande — les caustiques (8.5). **Mesures qu'il déclenche** (S360) : la queue qui dessine la capture compte 60 ondes
planes pour 5,5 octaves, sur 360°, pentes isotropes (1,04 contre 1,37 chez Cox et Munk) ; l'écume est tirée à
l'empreinte du pixel, donc sur les vaguelettes au premier plan. Suite : S360, la surface fine par FFT et l'écume au
déferlement ; puis les caustiques et le ciel.

## 26. R21 — la surface fine par FFT, l'écume au déferlement, S360

**Après R20** ([preuve](SURFACE-FINE-S360.md)). **La forme fine** : les petites vagues (7 cm à 3,5 m) ne sont plus 60
ondes planes réparties sur 360°, mais **10 612 composantes** du même spectre, en deux cascades calculées par FFT sur la
carte, étalées selon Elfouhaily et al. (1997) et courant avec le vent. **L'écume** : tirée des seules vagues dominantes,
à l'échelle du mètre — au nadir, les 4 652 taches de 4 cm deviennent quelques taches de 1,3 à 3,8 m, à la couverture de
Monahan (0,42 % à 7,8 m/s).

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s360/zoom_avant_apres.png` | la même zone, vue plongeante : en haut les 60 ondes (témoin), en bas la FFT |
| `viewer/captures/s360/godot_plongeante_cote_12s.png` | la vue plongeante entière, une tache d'écume en bas à droite |
| `viewer/captures/s360/godot_proche_12s.png` | le large, pose proche |
| `viewer/captures/s360/godot_proche_cote_12s.png` | la côte, pose proche |

**Les questions :**

1. **La forme fine** (zoom, en bas contre en haut) : plus réaliste ? Qu'est-ce qui gêne encore ?
2. **L'écume** : rare, en taches d'un à quatre mètres ; leur bord est lisse et ovale — gênant ?
3. **La suite** : les caustiques (demandées), puis le ciel ?

Un mot par question suffit.

**Verdict R21 — reçu le 2026-09-25 (S367)**, avec R22, R23 et R25 : *« Je valide les rendue sauf ecume »*. **La forme fine
par FFT est validée ; l'écume est refusée** — sans motif dit ; les taches instantanées au bord lisse et ovale que la
question 2 décrivait sont le suspect. **Ce qu'il déclenche** : S367, le champ d'écume d'ADR-014 — deux canaux, mémoire,
advection orbitale — en référence dans le cœur (liste 7.1), puis son rendu (8.4).

## 27. R22 — les caustiques sur le fond, S361

**Demandées en R20** ([preuve](CAUSTIQUES-S361.md)). La lumière du soleil réfractée par la surface — les vagues
dominantes et la surface fine de 0,5 à 3,5 m — se concentre en un réseau sur le sable : chaque bout de surface est
projeté sur le fond et y dépose sa lumière (optique géométrique, énergie conservée à 1 % près sur la scène), arrondie
par le disque du soleil. Exacte à 5 % près contre une solution calculée à part. Couverture : 64 m devant la caméra.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s361/zoom_caustiques_sous_l_eau.png` | le premier plan de la vue côtière proche, agrandi : le réseau sous l'eau |
| `viewer/captures/s361/godot_proche_cote_12s.png` | la vue côtière proche entière |
| `viewer/captures/s361/godot_plongeante_cote_12s.png` | la vue plongeante |
| `viewer/captures/s361/fond_seul_caustiques.png` | le fond seul, la mer masquée : le réseau, et le carré de la carte |

**Les questions :**

1. **Les caustiques** : crédibles ? Trop fortes, trop faibles, trop régulières ?
2. **La taille des cellules** (un à deux mètres, les vagues de 0,5 à 3,5 m) : juste pour 6 à 15 m de fond ?
3. **La suite** : le ciel, l'écume (texture, durée), autre chose ?

Un mot par question suffit.

**Verdict R22 — reçu le 2026-09-25 (S367)** : *« Je valide les rendue sauf ecume »* — **les caustiques sont validées**, leur
taille comprise.

## 28. R23 — le ciel, et la courbe calée sur la photographie, S363

**Demandé en R20** ([preuve](CIEL-S363.md)) : *« il n'aide pas aux reflets et limite la qualité du rendu final »*.
**Le ciel** : la couture verticale au centre et les nuages en blocs avaient une seule cause, un hachage qui perdait sa
précision ; remplacé par un hachage entier, les deux disparaissent. Le ciel est désormais celui de **votre
photographie de référence** (S308) : bleu profond au zénith, blanchi vers l'horizon — la mer le reflète. **La
tonalité** : une option, `TONALITE=photo`, calée sur la même photographie — ses contrastes (creux, crêtes, dynamique) et
la teinte de ses creux ; AgX reste le défaut, sa couleur ayant été jugée « parfaite » en R20.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s363/couture_avant_apres.png` | le haut du ciel au centre, agrandi trois fois : avant, la couture et les blocs ; après |
| `viewer/captures/s363/ciel_avant_apres_proche.png` | la pose proche : le ciel clair d'avant, le ciel de la photographie |
| `viewer/captures/s363/tonalite_agx_photo_proche.png` | AgX contre `TONALITE=photo`, pose proche (aussi `_rasante`, `_reference`) |
| `viewer/captures/s363/godot_*_12s.png`, `godot_*_photo_12s.png` | les images entières, AgX et `photo` |

**Les questions :**

1. **Le ciel** : plus crédible ? Les nuages vous conviennent-ils ?
2. **La tonalité** : AgX (à gauche) ou `photo` (à droite, premier plan plus profond, ciel plus pâle) — laquelle
   ressemble le plus à la mer ?
3. **La suite** : les reflets du ciel dans l'eau, l'écume, autre chose ?

Un mot par question suffit.

**Verdict R23 — reçu le 2026-09-25 (S367)** : *« Je valide les rendue sauf ecume »* — **le ciel est validé**, couture et nuages
compris. Aucun choix de tonalité n'est exprimé : **AgX reste le défaut**, `TONALITE=photo` une option validée.

## 29. R24 — sous la surface, S365

**Liste 8.6** ([preuve](SOUS-MARIN-S365.md)). La caméra dans l'eau, pour la première fois. Au-dessus de la tête, tout
le ciel tient dans un disque de 97° — la **fenêtre de Snell**, mesurée à 0,05° près — ; au-delà, la surface est un
**miroir** (réflexion totale). Entre l'œil et ce qu'il voit, l'eau pure éteint le rouge en treize mètres, le vert en
quatre-vingts, le bleu en trois cents, et ajoute sa propre lumière. Le fond, ses caustiques. **Pas encore** : la caméra
qui traverse la surface (la ligne d'eau sur l'objectif), les bulles, l'écume vue d'en dessous, les rayons de lumière dans
l'eau, le fond reflété dans le miroir.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s365/sous_eau_horizontal.png` | à 3 m sous la surface, visée horizontale : le miroir au-dessus, le fond qui se perd dans le bleu |
| `viewer/captures/s365/sous_eau_zenith_houle.png` | au zénith, sous la houle : la fenêtre fragmentée, le soleil |
| `viewer/captures/s365/sous_eau_vers_le_fond.png` | vers le fond : le sable et ses caustiques |
| `viewer/captures/s365/fenetre_de_snell_mer_plate.png` | la même visée sous une mer plate : la fenêtre entière, témoin |

**Les questions :**

1. **Crédible ?** Qu'est-ce qui gêne d'abord — la couleur, la clarté de l'eau, le miroir, la lumière ?
2. **Une référence** : une photographie sous l'eau que vous trouvez juste (eau claire, peu profonde, de jour) — elle
   servirait de cible chiffrée, comme celle de R14.
3. **La suite** : la caméra qui traverse la surface, les bulles, les rayons de lumière, autre chose ?

Un mot par question suffit.

**Verdict R24 — reçu le 2026-09-25 (S366)** : *« Pour les références trouve les sinon rien a redire cela me paraît
good, continue »*. Lu sur les trois questions : (1) crédible, rien à redire ; (2) les références, à trouver par nous ;
(3) aucune préférence de suite. **Ce qu'il déclenche** : S366 cherche des références réelles sous l'eau — une
distribution de radiance mesurée, qui calibre `f(ω)` laissé à calibrer par S365, et des photographies libres —, et les
chiffre. **Classe** : 8.6 jugée crédible sur quatre images fixes, à la pose de S365 ; ni animation, ni caméra qui traverse
la surface.

## 30. R25 — la lumière de l'eau calée sur une mesure, S366

**Après R24** ([preuve](SOUS-MARIN-S365.md) §6). Les références trouvées : une **mesure** de la lumière sous l'eau
selon la direction (Tyler 1960) et trois photographies libres. La mesure dit que l'eau est **bien plus claire face au
soleil** qu'à l'opposé — 8,7 contre 2,5 fois la lumière qui monte du fond, à l'horizontale — ; notre eau était la même
dans toutes les directions. Calée sur la mesure : une lueur face au soleil, rien de changé dos à lui.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s366/lobe_avant_apres.png` | à 4 m, visée horizontale : face au soleil et dos au soleil, avant et après |
| `viewer/captures/s366/contre_plongee_avant.png`, `contre_plongee_apres.png` | en contre-plongée, le cadrage de la photographie de Hanifaru (Maldives) |

**La question :** la lueur face au soleil — juste, trop forte, gênante ? Un mot suffit ; sans réponse, elle reste.

**Verdict R25 — reçu le 2026-09-25 (S367)** : *« Je valide les rendue sauf ecume »* — **la lueur face au soleil est validée**.
**R18** (en direct, §23), que la demande de S366 ne rappelait pas, reste attendu. *Reçu en S369 : voir §23.*

## 31. R26 — la caméra à demi immergée, S371

**Liste 8.6** ([preuve](DEMI-IMMERGEE-S371.md)). La caméra au ras de l'eau : la ligne d'eau traverse l'image. Chaque pixel
est vu de l'eau ou de l'air selon l'endroit où son rayon traverse l'objectif ; la ligne suit la vague, à 0,08 pixel près
du calcul exact, sans scintiller. Sur la ligne, un **ménisque** — l'eau qui monte sur le hublot —, calé sur une
photographie « dessus-dessous » de Raja Ampat : un trait sombre, une bande claire.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s371/demi_vers_le_large.png` | au niveau de l'eau, vers le large : la ligne inclinée par la pente de la vague, le dessous de la surface, le fond et ses caustiques |
| `viewer/captures/s371/demi_au_dessous.png` | 4 cm sous la surface, en contre-plongée : le ciel en haut, la surface vue d'en dessous, un morceau de fenêtre de Snell |
| `viewer/captures/s371/demi_au_dessus.png` | 4 cm au-dessus, plongeante : la mer, et l'eau qui coupe le bas de l'image |
| `viewer/captures/s371/demi_face_au_soleil.png` | face au soleil |
| `viewer/captures/s371/demi_sans_menisque.png` | la première, sans le ménisque |
| `viewer/captures/s371/avant_d_un_bloc.png` | la première, comme avant S371 : tout le cadre basculait d'un bloc — le ciel rendu comme de l'eau |

**Les questions :**

1. **Crédible ?** Qu'est-ce qui gêne d'abord ?
2. **Le ménisque** (trait sombre et bande claire sur la ligne) : le garder, l'enlever ?

Un mot par question suffit ; sans réponse, il reste.

**Verdict R26 — reçu le 2026-09-26 (S372)** : *« je valide continue »*. Lu sur les deux questions : (1) crédible ; (2) le
ménisque reste. **Classe** : la caméra à demi immergée jugée sur six images fixes, à Hs 2,5 m ; ni animation, ni objet
qui traverse la ligne.

## 32. R27 — le bassin de la piscine en δ 3D, S375

**Liste 5.10** ([preuve](PISCINE-DELTA-S375.md)). La piscine de S374, dont le bassin est désormais un domaine volumique 3D :
le jet de la pompe y entre, le déversoir en retire, V garde la masse. **À l'échelle réelle, rien ne se voit** : à la maille
de 20 cm que permet la référence CPU, la surface bouge de quelques millimètres.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s375/echelle_reelle_buse_6s5.png`, `…_rasante_6s5.png`, `…_ensemble_200s.png` | le rendu à l'échelle réelle, 1,5 s après le lancement du jet et en régime : la surface paraît plane |
| `viewer/captures/s375/cartes_de_hauteur_delta.png` | les **données** (écart au niveau moyen, ±3 mm ; rouge au-dessus, bleu en dessous) à 5,3 / 5,6 / 6,0 / 6,5 s et 8 / 40 / 200 / 260 s : dôme, creux, anneaux, réflexions, creux stable sous le jet |
| `viewer/captures/s375/debogage_hauteurs_x100_6s5.png` | **débogage**, hauteurs ×100 : la chaîne de rendu montre bien le creux et les anneaux |

**La question :** la suite proposée — δ sur la carte graphique, dans Godot, à une maille de 5 à 10 cm et en temps réel,
pour que le bouillonnement et les rides se voient — vous convient-elle ? Un mot suffit.

**Verdict R27 — reçu le 2026-09-26 (S376)** : *« je ne pense pas qu'il faut le faire maintenant »* — δ sur GPU dans Godot
n'est pas la suite ; suivi d'une précision de conception ([ADR-202](../adr/ADR-202-niveau-de-detail-des-contenants.md)) :
un contenant vu de loin reste V avec des effets factices, δ seulement près d'un perturbateur. **Classe** : l'invisibilité
de la dynamique à 20 cm n'est pas un défaut à corriger maintenant.


## 33. R28 — les rides de la pluie, S379

**Liste 8.9** ([preuve](RIDES-PLUIE-S379.md)). La pluie sur l'eau, **factice** comme vous l'avez demandé (ADR-202 D3,
ADR-203 D7) : aucun calcul de fluide, mais des nombres qui ont une provenance — le nombre de gouttes qui laissent un anneau
(Marshall et Palmer, Atlas : 447 par m² et par seconde à 10 mm/h, compté sur les images à 0,6 % près), la vitesse et la
longueur d'onde des anneaux (ondes capillaires-gravité : 1,7 cm à 0,23 m/s en tête, 4,4 cm à 0,18 m/s derrière), leur
extinction en 0,6 s. Au loin, quand le pixel ne peut plus montrer un anneau, il en montre la rugosité : l'eau devient mate
(pente quadratique moyenne 0,024 / 0,053 / 0,11 à 2 / 10 / 50 mm/h). Références : six photographies libres, lues sans
téléchargement ([notes de S379](../../notes/EN-COURS.md) ; *Rain in a pond at Zoo Schönbrunn*, *Waterwaves raindrops on
water surface*, *Rain on the River Pang*, *Rain falling into a swimming pool*…).

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s379/r28_bassin_10mmh.png` | le bassin à 10 mm/h : de près en oblique (1,8 m, 30°), d'aplomb, en rasant, d'ensemble (≈ 13 m, le joueur à l'abri qui regarde dehors) |
| `viewer/captures/s379/r28_bassin_sec_10_50.png` | sec, 10 et 50 mm/h, de près et en rasant : les anneaux déforment le carrelage vu à travers l'eau ; en rasant, les reflets nets des nuages font place à un voile mat piqueté |
| `viewer/captures/s379/r28_bassin_detail.png` | le détail (×3) : paquets de 2 à 3 crêtes, bosses centrales des impacts récents, anneaux lisibles surtout dans le reflet clair — comme la photographie *Waterwaves* |
| `viewer/captures/s379/r28_mer_sec_10_50.png` | la mer, sec / 10 / 50 mm/h : à 4 m de haut, un piqueté fin, discret |

**Ce qui n'y est pas** : les gerbes et gouttelettes rebondissantes (couronnes, éclats blancs), la pluie dans l'air, le ciel
de pluie (la scène garde son soleil : la météo vient à la fin), les reflets des murs et des objets (seul le ciel se reflète :
le voile mat en paraît moins marqué qu'avec des arbres ou une maison reflétés), les scintillements des anneaux au loin
(rendus en rugosité moyenne).

**La question :** ces rides sont-elles crédibles — de près, et vues de loin (la vue d'ensemble) ? Le voile mat à 10 mm/h,
qui efface les reflets des nuages en vue rasante, vous paraît-il juste, trop fort, pas assez ?

**Verdict R28 — reçu le 2026-09-26 (S379, après le rituel)** : *« Parfait »* — les rides de la pluie et le voile mat à
10 mm/h reçus tels quels, de près comme de loin. **Classe** : rendu validé ; restent hors de ce jugement les gerbes, la pluie
dans l'air et le ciel de pluie (la météo), le reflet des objets.

## 34. R29 — la pluie dans l'air, S380

**ADR-205, pièces 1 et 2** ([preuve](PLUIE-AIR-S380.md)) — votre demande : *« continue la pluie, ajoute les manquants »*.
La pluie **tombe** : près de l'œil, chaque goutte d'au moins 1 mm, au nombre de Marshall et Palmer (compté sur les images à
3 % près), à sa vitesse de chute, dessinée comme la voit une caméra (Garg et Nayar : une traînée fine et **faible**, qui ne
se détache que devant un fond sombre — ce que montrent les photographies d'averse) ; au loin, le **voile** que font les
gouttes, calculé de leur nombre et de leur taille : 2,5 km de visibilité à 10 mm/h, 0,9 km à 50.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s380/r29_bassin.png` | la piscine, sec / 10 / 50 mm/h, vue de la buse et d'ensemble : traînées sur le sol, voile |
| `viewer/captures/s380/r29_detail.png` | le détail à 50 mm/h (×3) : les traînées ; de près, les rides et les traînées ensemble |
| `viewer/captures/s380/r29_mer.png` | la mer, sec / 10 / 50 mm/h, deux poses : l'horizon voilé |

**Ce qui n'y est pas encore** : le **ciel de pluie** (pièce 3, la suivante) — le ciel reste ensoleillé, et le voile en
prend la couleur bleu pâle au lieu du gris des photographies ; les gerbes (pièce 4) ; un rideau de traînées à moyenne
distance ; le vent.

**La question :** la pluie qui tombe est-elle crédible — traînées, leur finesse, le voile ? Je propose d'enchaîner par le
ciel de pluie, qui changera beaucoup l'ensemble.

**Verdict R29 — reçu le 2026-09-26 (S380, après le rituel)** : *« Je valide »* — la pluie dans l'air (traînées, voile)
reçue ; **8.4 passe à partiel**. Suite proposée et inscrite : le ciel de pluie (ADR-205, pièce 3).

## 35. R30 — le ciel de pluie, S381

**ADR-205, pièce 3** ([preuve](CIEL-PLUIE-S381.md)). Quand il pleut, le ciel est couvert : le soleil disparaît — ni disque, ni éclat sur l'eau, ni
caustiques, ni lumière à travers les crêtes des vagues — et la lumière vient de tout le ciel, **neutre** (gris, comme sur
les photographies de pluie mesurées : bleu à +1,5 % près dans *Downpour*, +1 à +7 % dans *Rain over the Sea, Mundesley*),
trois fois plus claire au zénith qu'à l'horizon (le ciel couvert normalisé de la CIE, Moon et Spencer 1942 ; rendu à
0,15 % près). **L'œil s'adapte** (hypothèse déclarée) : une surface horizontale mate garde exactement sa luminosité d'avant ;
un mur reçoit 40 % de ce que reçoit le sol, plus ce que le sol lui renvoie. La pluie couvre le ciel ; la météo le
commandera.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s381/r30_ciel_seul.png` | temps sec, ciel clair (avant) contre ciel couvert : piscine (ensemble, buse), mer (référence, haute) — le soleil éteint, la lumière diffuse, sans pluie |
| `viewer/captures/s381/r30_bassin.png` | la piscine sous la pluie : 10 mm/h sous le soleil (R29) contre 10 et 50 mm/h sous le ciel couvert ; buse, ensemble, de près |
| `viewer/captures/s381/r30_mer.png` | la mer, mêmes colonnes : proche (4 m), référence (7 m), rasante (2 m) — grise, mate, l'horizon voilé |

**Ce qui se voit, et d'où cela vient.** Vue d'ensemble de la piscine : le bloc **se confond avec le sol**. Ce n'est pas
une erreur de calcul : béton d'albédo 0,42 sur un mur qui reçoit 0,47, sol d'albédo 0,20 qui reçoit 0,94 — 0,196 contre
0,188. Ce qui, dans la réalité, dessine encore les volumes par temps couvert — **l'occultation du ciel** : le pied des
murs et les angles voient moins de ciel et s'assombrissent — **manque à la scène**, comme les ombres portées par ciel clair.
Corrigé en préparant ces images : la lumière du soleil à travers les crêtes restait allumée sous le ciel couvert (taches
claires sur la mer).

**Ce qui n'y est pas** : l'occultation du ciel ; un ciel couvert **sans texture** (la base d'un nimbostratus a des
nuances, et au loin des rideaux de pluie sous les nuages, comme à *Mundesley*) ; sous l'eau, le lobe du soleil reste
allumé ; les gerbes (pièce 4) et les **surfaces mouillées** (pièce 5 : sol et margelles plus sombres et brillants), qui
changeront beaucoup l'image de la piscine.

**La question :** le ciel de pluie est-il crédible — gris neutre, sans soleil, lumière diffuse ? La luminosité d'ensemble
(l'œil adapté : le sol aussi clair qu'au soleil) vous paraît-elle juste, ou la scène devrait-elle être plus sombre ? Le
bloc de la piscine qui disparaît : faut-il l'occultation du ciel avant de continuer la pluie (gerbes, surfaces mouillées) ?

**Verdict R30 — reçu le 2026-09-26 (S381, après le rituel)** : *« Je valide, ajoute l'occultation du ciel puis continue,
et ensuite le plus important le solveur 3D »* — le ciel de pluie reçu, l'œil adapté compris ; **l'occultation du ciel**
demandée avant la suite de la pluie ; puis la campagne du solveur volumique 3D, dite *« le plus important »*.

## 36. R31 — l'occultation du ciel, S382

**Votre demande** (R30) : *« ajoute l'occultation du ciel »* ([ADR-206](../adr/ADR-206-la-visibilite-du-ciel-par-des-occultants-analytiques.md),
[preuve](OCCULTATION-CIEL-S382.md)). Chaque surface reçoit maintenant **le ciel qu'elle voit** : le pied d'un mur, un angle,
le fond du bassin voient moins de ciel et s'assombrissent — calculé sur les objets eux-mêmes, contre une intégration
indépendante à moins de 0,009 près (15 points). Par ciel clair, le même calcul vers le soleil donne **les ombres portées**
(bord à 2,5 mm de la géométrie). Coût : +0,1 à +0,3 ms (la part du ciel est cuite une fois aux sommets ; les ombres, à
chaque pixel). Référence : *USVI IMG 5366* (piscine et dallage sous un ciel couvert d'orage) — pas d'ombre portée, faces
verticales nettement plus sombres que le sol, dessous du pavillon couvert très sombre.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s382/r31_couvert.png` | ciel couvert, sec : sans occultation (R30) contre avec — ensemble, buse, de près |
| `viewer/captures/s382/r31_clair.png` | ciel clair, sec : avant contre après (ciel occulté et ombres portées du soleil, à 58° au nord-ouest) |
| `viewer/captures/s382/r31_pluie.png` | sous la pluie, ciel couvert : 10 mm/h de R30 contre 10 et 50 mm/h avec l'occultation |

**Ce qui n'y est pas** : la lumière renvoyée par le sol et les murs (pas d'interréflexion : les ombres et les pieds de murs
sont un peu trop sombres par rapport au réel, où le sol clair éclaire le mur) ; la pénombre du soleil rendue plus étroite
que la vraie (10 mm pour 33 mm) ; la réfraction de la lumière qui descend au fond du bassin (fenêtre de Snell) ; les reflets
des murs dans l'eau (pièce 6 de la pluie) ; la mer n'a pas d'objet à occulter.

**La question :** l'occultation du ciel et les ombres rendent-elles la scène crédible — le bloc qui se détache du sol par
temps couvert, les ombres par ciel clair ? Si oui, je reprends la pluie (les gerbes), puis la campagne du solveur.

**Verdict R31 — reçu le 2026-09-26 (S382, après le rituel)** : *« Je valide R31, continue la pluie »* — l'occultation du
ciel et les ombres portées reçues ; la pluie continue (pièce 4, les gerbes).

## 37. R32 — les gerbes de la pluie, S383

**ADR-205, pièce 4** ([preuve](GERBES-S383.md)). À chaque goutte qui laisse un anneau — **la même** que celle des rides, à son instant (contrôlé :
961 gerbes, chacune au centre de son anneau à 0,8 mm près) —, une gerbe au-dessus de l'eau : la **couronne**, puis le
**dôme** et le **jet** central. Sa forme et ses temps sont relevés sur une goutte de pluie réelle filmée (Murphy et al.
2015 : 4,1 mm à 7,2 m/s — couronne de 19 mm à 12 ms, jet de 25 mm à 18 ms, retombée vers 80 ms), ramenés à chaque taille
de goutte ; comme une caméra, l'image moyenne les 16,7 ms de la pose (la couronne, qui dure 10 ms, n'est vue que floue).
Sur la mer, les gerbes sont posées sur les vagues ; au loin, où elles tiennent sous le pixel, elles éclaircissent la mer
près de l'horizon (12 % de la surface à 2° sous 50 mm/h). Coût : 0,01 à 0,3 ms.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s383/r32_piscine.png` | la piscine, 10 mm/h sans gerbes (R31) contre avec, et 50 mm/h — de près et de la buse |
| `viewer/captures/s383/r32_mer.png` | la mer, mêmes colonnes : au ras de l'eau (4 cm), proche (4 m), rasante (2 m) |
| `viewer/captures/s383/r32_detail.png` | la gerbe de référence de profil à 1, 3, 7, 12, 18, 41, 52 ms, et des gerbes dans les scènes (×3) |

**Corrigé en chemin** (S380) : dans les images de la mer, la pluie ne suivait pas la pose de la caméra — en pose
« référence », les gouttes de R29 et R30 tombaient environ 11 m trop loin ; elles sont maintenant au premier plan.

**Ce qui n'y est pas** : les **doigts** de la couronne et les **gouttelettes** qui en partent (≈ 2 000 de 0,05 à 0,2 mm par
goutte, sous le pixel une à une) ; la silhouette est simplifiée (coupe, dôme, jet), ses opacités réglées à l'œil ; les
gerbes des petites gouttes (sous 1,5 mm) ; les éclaboussures **sur le sol** et les margelles — elles viennent avec les
surfaces mouillées (pièce 5), le sol étant alors un film d'eau ; le vent.

**La question :** les gerbes sont-elles crédibles — leur nombre, leur taille, leur forme de près au ras de l'eau (dôme et
jet) ? La silhouette simplifiée suffit-elle, ou faut-il la couronne à doigts et les gouttelettes avant de continuer ?

**Verdict R32 — reçu le 2026-09-26 (S385)** : *« Verdict R32 n'est pas valide pour plusieurs raisons : les gerbes n'apparaissent
pas sur tous les impacts de pluie, elles peuvent tenir dans le vide, et sont vraiment moches vues de près. Mais on laisse
valide pour l'instant, il s'agit de sessions de peaufinage externe. »* Image jointe : des gerbes blanches, en colonnes,
au-dessus de l'eau à distance moyenne. **Reçu pour l'instant** ; les trois défauts vont à une session de peaufinage
([file](../registres/QUESTIONS-OUVERTES.md#file-active)) — ils sont constatés, pas encore expliqués.

## 38. R33 — les surfaces mouillées, S392

**ADR-205, pièce 5a** ([preuve](SURFACES-MOUILLEES-S392.md)). Sous la pluie, le sol, les margelles et le pied des murs de la
piscine prennent un **film d'eau** : plus sombres — la lumière piégée dans le film par réflexion interne (Ångström, Lekner
et Dorf 1988) : le béton à 64 % de sa radiance sèche vu d'aplomb, le sol sombre à 57 % — et **plus brillants** : le film
reflète le ciel (le sol s'éclaircit vers l'horizon) et le bloc (reflet flou à son pied). Ce que la pluie n'atteint pas
reste **sec** : la bande de 10 cm sous le débord des margelles, les dessous. Contrôlé sur l'image : la formule à 0,03 %, le
bord sec au pixel près. Coût : 0,2 à 0,35 ms.

| image | ce qu'elle montre |
|---|---|
| `viewer/captures/s392/r33_mouille.png` | 10 mm/h, à gauche sec (la pluie d'avant), à droite mouillé : pied du mur ouest à hauteur d'œil, ensemble, proche |
| `viewer/captures/s392/r33_<vue>_sec.png`, `…_mouille.png` | les mêmes, en pleine taille |

**Ce qui n'y est pas** : les **éclaboussures au sol** (5b, prochaine session de la pièce) ; les rides des gouttes sur le film ;
les flaques (pièce 12) ; le mouillage progressif au début de l'averse et le séchage (la météo) ; le vent. **Mesuré, non
expliqué** : sur une photographie réelle (premières gouttes sur un asphalte sec), les taches mouillées sont bien plus sombres
que ne le prédit la formule (0,08 contre 0,53 en linéaire, courbe de l'appareil inconnue) — le rendu pourrait être trop
clair.

**La question :** les surfaces mouillées sont-elles crédibles — assez sombres ? le reflet du bloc dans le sol, trop net ou
trop fort ? la bande sèche sous les margelles, juste ? Faut-il assombrir davantage (le second effet de Lekner et Dorf, par
un facteur réglé sur une référence) avant de passer aux éclaboussures ?

**Verdict R33 — reçu le 2026-09-26 (S393)** : *« pour R33 je valide actuellement mais pour plus tard des sessions de
peaufinage »*. **Reçu pour l'instant** ; aucun défaut nommé — le peaufinage reprendra les questions laissées ouvertes ci-dessus
(assez sombre ? le reflet du bloc ? la bande sèche ?) et l'écart à la photographie, dans une session de rendu au poste
([file](../registres/QUESTIONS-OUVERTES.md#file-active)). La pièce 5b (les éclaboussures au sol) n'attend plus R33.

## 39. R34 — la vague qui déferle, colonnes et particules, S410

**C6b de la campagne du solveur 3D** ([preuve](BASCULE-S408.md) §6). Une houle de Stokes raide (`ka` = 0,55, le cas de Chen et
al. 1999) déferle dans APIC 3D, référence CPU, 5 cm, vue en coupe de 2 à 6 m, à six instants (0,3 à 1,4 `√(λ/g)`, `λ` = 2 m).
Points bleu foncé : les particules ; bleu clair : l'eau portée par les **colonnes** (la surface en hauteur) ; trait orange : la
**bande** de particules. À gauche, APIC seul — tout en particules, la référence ; au milieu et à droite, la bande dynamique de
S408, deux durées de maintien. Ce n'est pas un rendu de l'eau : un instrument, pour juger la forme du déferlement.

| image | ce qu'elle montre |
|---|---|
| `captures/s410/planche_R34.png` | six instants × APIC seul, bande (maintien 0,3 s), bande (maintien 0,05 s) |
| `captures/s410/<seul, m03, court>/*_t<instant>.ppm` | les coupes une à une |

**Ce que la session y voit** : à 0,05 s, le **sommet de la crête repasse en colonnes** — une bosse lisse et trop haute derrière
une lèvre de particules ; à 0,3 s, la bande couvre la crête et le déferlement ressemble à APIC seul. **Ce qui n'y est pas** :
une vue 3D, la crête courte, le rendu de l'eau (Godot) ; la maille est six fois plus grossière que celle de Chen.

**La question :** au maintien de 0,3 s, le déferlement de la bande est-il assez proche de celui d'APIC seul pour être retenu ?
Le défaut du maintien court (la bosse en colonnes) est-il bien celui que tu vois ?

**Verdict R34 — reçu le 2026-09-27 (S411)** : *« Il s'agit de 2D et de bille, encore loin du finale, qui est en 3D et une topology sans interstice visible dans l'eau sauf pour les jets. Donc difficile de réaliser un retour, mais le maintien de 0.3s parait bien, la formation de la vague est visible. »* **Reçu pour la direction** : le maintien de 0,3 s est retenu pour la suite (C6c),
le défaut du critère restant celui de S408 jusque-là ; la planche n'est pas un rendu — l'image finale est en 3D, **une surface
continue sans interstice**, seuls les jets s'en détachent. La même réponse ouvre une réflexion de conception — ne pas simuler
l'eau profonde en particules, les trucages d'un logiciel spécialisé en temps réel, les courants à niveaux de détail :
[TRUCAGES-TEMPS-REEL-S411](../registres/TRUCAGES-TEMPS-REEL-S411.md).

## 40. R35 — la vague qui déferle sur la bande étroite, S414

**C6c-2** ([preuve](BANDE-ETROITE-S413.md) §5). La vague de R34, même coupe, mêmes instants ; à droite, **la bande étroite** : les
particules seulement à quatre mailles sous la surface, l'eau dessous portée par la grille (**vert d'eau**). Au milieu, la bande
pleine retenue par R34 (maintien 0,3 s) ; à gauche, APIC seul. La bande étroite porte **six fois moins de particules** que la bande
pleine et calcule deux fois plus vite. Comme R34 : un instrument en 2D, pas le rendu final (une surface continue en 3D).

| image | ce qu'elle montre |
|---|---|
| `captures/s414/planche_R35.png` | six instants × APIC seul, bande pleine, bande étroite |

**La question :** la bande étroite déferle-t-elle comme les deux autres — la forme de la crête, le jet, sa retombée ?

**Verdict R35 — reçu le 2026-09-27 (S415)** : *« Je valides R35, continue avec ta recomandation »*. **Reçu** : la bande étroite
déferle comme APIC seul et la bande pleine. Le réglage retenu — maintien 0,3 s, fond à quatre mailles, prédiction du corps à
horizon court — devient celui de la suite (C6c-3, C7) ; les défauts du code restent ceux de S408 jusqu'à ce que C7 les porte.

## 41. R36 — la surface continue, premier rendu, S450

Le banc `surface_continue` ([SURFACE-CONTINUE-S450](SURFACE-CONTINUE-S450.md)) : B10 — une sphère de 0,4 m entre dans l'eau —,
l'isosurface du champ unique `φ` d'`Apic3` (colonnes et bande de particules d'un même champ), étanche, en rendu logiciel. Quatre
instants : le cratère (t = 0,5 et 1 `√(D/g)`), le jet de Worthington (t = 2 et 3).

| image | ce qu'elle montre |
|---|---|
| `captures/s450/b10_t{0.5,1.0,2.0,3.0}.png` | la surface continue, la sphère en gris |

**Verdict R36 — reçu le 2026-10-02 (S451)** : *« Alors le problème est que l'on voit des divisions faces plane »*. **Non reçu.** Deux
causes, mesurées : l'**ombrage à facettes** (une normale par triangle) et les **marches au raccord** bande | colonnes (0,34 maille, S450).
Suite : S451 — des normales lissées (le gradient de `φ`, interpolé par pixel) et le raccord de `φ` à la frontière pour le rendu.

## 42. R37 — la surface continue, normales lissées et raccord fondu, S451

Les mêmes instants que R36, après les deux corrections ([SURFACE-CONTINUE-S450](SURFACE-CONTINUE-S450.md), S451) : le saut au raccord
de 0,34 à 0,08 maille, l'ombrage par pixel.

| image | ce qu'elle montre |
|---|---|
| `captures/s451/b10_t{0.5,1.0,2.0,3.0}.png` | le cratère, puis le jet — lisses |

**La question :** voit-on encore des divisions — faces planes ou marches ? **Verdict R37 — reçu le 2026-10-02 (S452)** : *« Je valide le
render »*. **Reçu** : la surface continue — l'isosurface du champ unique `φ`, fondue au raccord, ombrée par pixel — est le rendu retenu
; suite, le rendu en direct sur la carte (S452).

## 43. R38 — la v1 : le saut sous la houle, en direct, S458

La scène `--v1` ([C10-SCENES-S454](C10-SCENES-S454.md) §7, [ADR-215](../adr/ADR-215-autonomie-jusqu-a-une-v1-solide.md) D3) : un joueur
(une sphère de 0,4 m) saute à répétition dans 1,4 m d'eau, sous une houle de 4 cm, la mer jusqu'à l'horizon, la lumière de l'eau
reçue ; simulée et rendue en direct sur la carte, au temps réel (0,99), masse exacte.

| image | ce qu'elle montre |
|---|---|
| `captures/s458/v1_t0.30.png`, `v1_t0.55.png` | l'entrée, la cavité |
| `captures/s458/v1_t0.85.png`, `v1_t1.60.png` | le jet de Worthington, puis les anneaux |
| `captures/s458/v1_t20.70.png`, `v1_t21.20.png` | le cinquième saut |

En mouvement : `viewer/target/release/water-viewer.exe --v1`.

**La question :** la v1 est-elle solide, à l'image et en mouvement ? Qu'est-ce qui gêne le plus ? **Verdict R38 — reçu le 2026-10-03** :
*« Correct pour une V1 »*. **Reçu** : la v1 d'ADR-215 D3 — la scène `--v1` — est solide ; aucun défaut nommé.

