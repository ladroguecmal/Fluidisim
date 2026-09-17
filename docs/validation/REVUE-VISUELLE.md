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

