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
| **R6** | 2026-09-17 | calibration : même scène à 3, 5 et 8,37 m/s, trois poses, ciel clair (ADR-160, [empreintes](VENT-S263.md) §3) | références A et B | **en attente** | question : quel vent ressemble le plus à la mer attendue ? |

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
