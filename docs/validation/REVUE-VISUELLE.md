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
