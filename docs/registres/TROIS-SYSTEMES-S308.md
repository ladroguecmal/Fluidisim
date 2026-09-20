# S308 — La stratégie en trois systèmes, confrontée à l'état réel

2026-09-20. Demande de l'utilisateur, reçue en cours de session : réorganiser le développement
autour de **trois systèmes** — A haute mer superficielle, B simulation volumique 3D, C transition
et couplage —, valider chacun indépendamment, puis les assembler, puis seulement optimiser.
Confronter cette stratégie à l'état réel du dépôt et à la [liste du projet fini](../LISTE-PROJET-FINI.md),
nommer ce qui manque, proposer un ordre.

**Base examinée** : `b5553d2`, copie unique, branche `master`, arbre propre. Suites exécutées ce
jour : **539 essais du cœur et du harnais passent, 0 échec, 18 ignorés** (mesures à lancer
explicitement). Aucune décision actée n'est rouverte ; aucune ambition n'est retirée
([ADR-127](../adr/ADR-127-ambition-complete-construction-progressive.md)).

---

## 1. La réponse courte

**La stratégie est compatible avec le dépôt, et elle ne remplace pas son architecture : elle en
regroupe les chantiers.** A recouvre B+W, B recouvre δ, C recouvre les bandes de couplage et la
surface publiée. La couche V reste hors des trois et garde ses déclencheurs.

Trois écarts changent l'ordre des travaux, et ils sont tous mesurés.

1. **Le dépôt a déjà la séparation « référence / production » que la stratégie réclame en §3.**
   [ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) l'a actée en S294 : une
   référence CPU sans contrainte de budget porte la réception physique, une production GPU est
   jugée **par comparaison** avec elle. Ce n'est pas à construire, c'est à utiliser.

2. **Le système B existe en 3D, mais dans une représentation qui exclut par construction le banc
   que la stratégie met en avant.** La surface libre de δ est une **fonction hauteur** `η(x, y)`
   (ADR-175 D5). Une cavité, un jet et un déferlement ne sont pas des fonctions hauteur. **Le test
   5 — une boule qui tombe dans l'eau, impact, cavité, projections — est hors du domaine du
   solveur actuel, et aucune quantité de réglage ne l'y fera entrer.** ADR-175 le dit déjà et
   renvoie à une **seconde représentation** (particules ou surface implicite) qui n'a jamais été
   choisie. C'est le plus grand écart entre la stratégie et le dépôt.

3. **Le système C est le plus construit et le moins validé.** Les bandes de couplage, l'éponge et
   la surface publiée existent et tournent ; mais le couplage est **à sens unique** — B/W entre
   dans δ, δ ne ressort pas vers W —, et **aucun bilan de masse, de quantité de mouvement ou
   d'énergie n'a jamais été mesuré à l'interface**. La stratégie demande explicitement ces
   échanges cohérents : c'est une capacité à construire, pas seulement à éprouver.

Et un quatrième point, qui n'est pas un écart mais une confirmation : **arrêter le
perfectionnement optique est justifié par la mesure**, pas seulement par la priorité. S308 P7 a
montré qu'aucun réglage de la chaîne tonale ne rapproche le rendu de la photographie de référence
au-delà de ce qui est déjà obtenu — ce qui manque est la structure de la mer elle-même, à quelques
pixels d'échelle. Le rendu actuel est donc un **point d'arrêt légitime** ; la troisième image de
R14 peut servir de référence interne sans dette technique cachée.

---

## 2. Ce qui existe, système par système

Les preuves sont dans `docs/validation/` ; la trajectoire reste portée par la
[feuille de route](../FEUILLE-DE-ROUTE.md), que ce document ne double pas.

### A — Haute mer superficielle (couches B et W)

| | état |
|---|---|
| état de mer spectral | **reçu** : multimodal, étalement `cos^2s`, queue spectrale, `Hs` 2,5 m (ADR-155, 156) |
| forme des vagues | **reçu** : CWM (crêtes pointues), rugosité Cox–Munk, asymétries du second ordre `Sk` 0,066 (ADR-157, 158, 176) |
| vent, courants | vent de scène **reçu** (ADR-160) ; courant macroscopique présent dans `u_total` |
| impacts, sillages | **reçus** : table de Bessel, journal de sillages, grille locale, filtrage spectral |
| **sortie physique pour les autres systèmes** | **`WaterSample`** (SPEC-004 §2) : `eta`, `u_total` (orbitale + courant), `normal`, `deta_dt`, `steepness`, `aeration`. Requête admise par le cœur, ≈ 0,2 ms par point |
| pression | chemin séparé (`modal_pressure`, `spectral_pressure`), borné et admis |
| coût | **GPU eau 1,74 ms** médian à 1280×720 ; **CPU 4,1 ms** contre ≤ 2 ms visés (ADR-174 D3) |
| rendu | hôte GPU séparé, temps réel, jugé par l'utilisateur de R1 à R14 |

**Ce qui manque pour le couplage, et rien d'autre** : la cohérence de phase δ/B (A289 — δ porte la
dispersion d'amplitude, B ne l'a pas, ≈ 7 cm/min sous `ak` = 0,06) ; les couches W au-dessus du
plan moyen (A286, partielle) ; le CPU sous budget (A278, question d'optimisation, pas de capacité).

**Verdict : A est suffisant pour servir B et C.** Sa sortie physique est définie, testée et déjà
consommée par δ. La stratégie a raison de l'arrêter là.

> **Précision ajoutée en S309, et elle borne le verdict ci-dessus.** « A » ne veut pas dire ici
> tout le système A de la liste du projet fini, mais **la mer de vent et de houle en eau profonde
> uniforme**. Douze des vingt points de A y sont absents — lacs, rivières, canaux, bathymétrie,
> hauts-fonds, courants 3D, tsunamis, explosions, déferlement de W, écume. Le verdict vaut **pour
> le couplage**, pas pour les autres milieux que l'objectif nomme. Voir §8.

### B — Simulation volumique 3D (couche δ)

| | état |
|---|---|
| grille et solveur | **MAC x-y-z**, pression scindée `p_hydro + p_dyn`, projection à multigrille, transport, bandes |
| surface libre | **fonction hauteur `η(x, y)` à fluide fantôme** — reçue contre HOS à 0,148 % / 0,178 % ([S297](../validation/DELTA3D-COUPLEE-S297.md)) |
| **cuve fermée** | **reçue** : fond plat, murs sur les quatre côtés, mode (1,1) d'une cuve 8 × 4 × 4 m ; production contre référence à **3·10⁻⁷ m pour 3 mm exigés**, phase 1,22° → 0,065° ([S305](../validation/CUVE-GPU-S305.md)) |
| fond et bathymétrie locale | fond spectral réel reçu, cas limites 2D reproduits à 1,19·10⁻⁷ m ([S298](../validation/DELTA3D-FOND-REEL-S298.md)) |
| production GPU | pas couplé **entier** résident sur la carte : **0,84 ms à 64 cycles sur 27 648 mailles** ([S301](../validation/DELTA3D-PAS-GPU-S301.md)) |
| scène réelle | **30 × 28 m à 25 cm sur la mer étalée**, front de 65 cm qui la traverse à 2,2 m/s, rendu en direct à 197 Hz ([S302](../validation/SCENE-DELTA3D-S302.md)) |
| coût sur cette scène | **4,62 ms par pas à 376 320 mailles**, 32 cycles, contre 2 ms visés |
| faces coupées (obstacles) | **en 2D seulement** (S232, ordres 1,947–1,966). `delta3d.rs` le déclare : « ni faces coupées ni budget coopératif » |
| corps rigides | **aucun**. `body.rs` est un modèle **statique** de flottaison (cas C10) : ni intégrateur, ni rotation, ni masse ajoutée |
| cavité, jet, déferlement | **hors représentation** par décision (ADR-175 D5) ; seconde représentation **non choisie** |
| turbulence | aucune (ni modèle, ni banc) |
| référentiel mobile, subdivision, fusion | aucun |
| dérive longue | **mesurée** : écart carte/référence ≈ 1,2·10⁻⁷ m par seconde sur la cuve (A298, suspect nommé, non démontré) ; bascule de mouillure A297 |

**Verdict : B existe et tourne, sur un domaine fixe et simple, avec une surface en graphe.** C'est
exactement le point de départ que la stratégie décrit — « domaines simples et fixes avant les
subdivisions adaptatives ». Ce que la stratégie ajoute au périmètre actuel, ce sont les
**obstacles mobiles**, les **corps libres** et les **phénomènes non graphes**.

### C — Transition et couplage

| | état |
|---|---|
| entrée B/W → δ | **reçue** : fond sommé aux faces MAC x/y/z, `eta` identique au bit verticalement, advection croisée, démarrage progressif (ADR-149 à 154, 164 à 166) |
| frontières | **éponge quadratique** sur les paires de bords, bandes de couplage sur les quatre côtés ; R11 : « raccord du domaine invisible » |
| surface publiée | **reçue** (I-13, ADR-175 D7) : à la fin du pas, δ écrit une surface immuable ; le rendu ne lie jamais les tampons internes |
| continuité surface affichée / surface physique | le **rendu** consomme la surface publiée de δ ; la **requête de jeu** ne consomme que B/W. δ n'entre pas dans la requête |
| **sortie δ → W** | **n'existe pas**. L'éponge **absorbe** vers B+W ; rien n'écrit en retour dans W |
| **bilan masse / quantité de mouvement / énergie** | **jamais mesuré**, ni à l'entrée ni à la sortie |
| réflexion artificielle aux frontières | jugée **visuellement** invisible (R11) ; **jamais chiffrée** |

**Verdict : C a ses tuyaux dans un sens et aucun compteur.** C'est le système où la stratégie
demande le plus de construction neuve, et c'est aussi celui où l'effort est le plus rentable :
tout ce qu'il faut pour mesurer est déjà en place.

---

## 3. Les interfaces qui manquent

Nommées, avec ce qu'elles bloquent. L'ordre est celui de la dépendance, pas de la difficulté.

| | interface | ce qu'elle bloque aujourd'hui |
|---|---|---|
| **I1** | **retour δ → W** : écrire les perturbations sortantes de δ dans W au lieu de les absorber | les échanges cohérents demandés par la stratégie §1-C ; toute scène où une perturbation locale doit se propager au loin |
| **I2** | **compteurs de conservation à l'interface** : masse, quantité de mouvement, énergie, entrants et sortants, par pas | la validation numérique que la stratégie §2 exige ; la réflexion artificielle, aujourd'hui jugée à l'œil |
| **I3** | **solide ↔ fluide en 3D** : faces coupées 3D (extension de S232), puis intégrateur de corps rigide et couplage bidirectionnel | les tests 2, 3 et 4 de la piscine ; la porte D et donc la v1 |
| **I4** | **seconde représentation de surface libre** (particules ou surface implicite), et son raccord au domaine en graphe | le test 5 (impact, cavité, projections), le déferlement, les jets — décidés hors périmètre du solveur actuel par ADR-175 D5 |
| **I5** | **requête de jeu traversant δ** : aujourd'hui le gameplay interroge B/W seul, alors que l'image montre δ | « la continuité entre la surface affichée et la surface utilisée par la physique du jeu » (stratégie §1-C) |
| **I6** | **conservation du volume sur la durée** dans un domaine fermé | le test 1 (piscine au repos) au sens fort ; A298 mesure déjà une dérive, sans loi |

**Ce ne sont pas six chantiers de taille comparable.** I2 et I6 sont des instruments : quelques
heures, sur du code qui existe. I1 et I5 sont des chemins de données à ouvrir. I3 est un lot
complet. **I4 est un second solveur**, et c'est le plus gros travail de toute la stratégie.

---

## 4. Le banc de la piscine, confronté à ce qui existe

L'utilisateur propose six essais. Voici ce qu'ils demandent réellement.

| | essai | faisable aujourd'hui | ce qui manque |
|---|---|---|---|
| **1** | piscine au repos — stabilité, hydrostatique, volume | **oui, en grande partie** : la cuve fermée est reçue à 3·10⁻⁷ m ([S305](../validation/CUVE-GPU-S305.md)) | **I6** : le volume n'est pas compté ; A298 dit qu'il dérive |
| **6** | piscine recevant une houle extérieure | **oui** : c'est la scène S302, en domaine fermé au lieu d'ouvert | **I2** pour chiffrer transmission et réflexion au lieu de les regarder |
| **2** | boule statique immergée — volume déplacé, flottabilité | **non** | **I3** (faces coupées 3D). `body.rs` donnerait la poussée, mais sur un modèle statique à ligne de flottaison plane |
| **3** | boule à mouvement imposé — déplacement, forces, vagues | **non** | **I3** avec frontière mobile ; les faces coupées 2D de S232 en sont la moitié |
| **4** | boule libre — couplage bidirectionnel | **non** | **I3** complet : intégrateur de corps rigide, forces rendues, masse ajoutée |
| **5** | boule qui tombe — impact, cavité, projections | **non, et pas par manque de réglage** | **I4** : la surface en graphe ne peut pas représenter une cavité |

**Conséquence sur l'ordre** : les essais 1 et 6 sont à portée immédiate et livrent les
**instruments** dont tous les autres auront besoin. Les essais 2 à 4 forment un seul lot (I3). Le 5
est un lot à part, et le plus lourd.

---

## 5. Ordre de réalisation proposé

Chaque lot nomme ce qu'il rend possible et à quoi il s'arrête. Aucun ne rouvre une décision actée.
Le budget de 2 ms n'est pas opposable pendant les lots 1 à 5 (stratégie §3) — **mais le coût reste
mesuré et publié à chaque lot**, selon ADR-131, pour qu'aucune architecture impossible à optimiser
ne s'installe sans qu'on le sache.

| ordre | lot | rend possible | arrêt |
|---|---|---|---|
| **1** | **Les compteurs (I2, I6).** Masse, quantité de mouvement et énergie entrants/sortants par pas ; volume d'un domaine fermé sur la durée. Sur la cuve S305 et sur la scène S302, qui existent | toute la validation numérique de la stratégie §2 ; les essais 1 et 6 au sens fort | un bilan publié sur les deux scènes, avec sa dérive et son plancher numérique ; réflexion aux frontières **chiffrée** |
| **2** | **Le retour δ → W (I1).** Écrire la perturbation sortante dans W au lieu de l'absorber, sous le compteur du lot 1 | les échanges cohérents ; une perturbation locale qui se propage au loin | masse et énergie conservées à travers l'interface dans les deux sens, à une tolérance déclarée ; scènes antérieures inchangées au bit quand le retour est éteint |
| **3** | **Faces coupées 3D (I3, moitié basse).** Extension de S232 à la grille x-y-z, fixes d'abord, puis à frontière mobile | essais 2 et 3 de la piscine ; les obstacles en général | ordre de convergence local mesuré en 3D comme il l'a été en 2D (1,947–1,966) ; stabilité des petites cellules |
| **4** | **Corps rigides et couplage bidirectionnel (I3, moitié haute).** Intégrateur, forces rendues, masse ajoutée ; sans autorité de δ sur le gameplay (I-04) | essai 4 ; **la porte D, donc la v1** | une boule libre flotte à son tirant d'eau théorique, puis oscille à la période que la raideur hydrostatique implique (références de C10, déjà écrites) |
| **5** | **Seconde représentation (I4).** Choix — particules ou surface implicite — puis premier domaine d'impact (B10), et son raccord au domaine en graphe | essai 5 ; cavité, jet, déferlement ; tout ce qu'ADR-175 D5 a mis de côté | une cavité se forme, se referme et rend son volume ; le raccord au domaine en graphe ne fabrique ni source ni puits, sous le compteur du lot 1 |
| **6** | **La requête de jeu à travers δ (I5).** | « la surface affichée est celle dont le jeu se sert » | un point interrogé dans un domaine δ actif rend la surface de δ, hors domaine celle de B/W, sans discontinuité à la frontière |
| **7** | **Adaptation et optimisation** : domaines qui se déplacent et se redimensionnent, subdivision/fusion, LOD de simulation, dégradation, puis budget | la porte C et la porte A complètes | ADR-174 D3 : δ ≤ 2 ms GPU, techniques présentes et absentes publiées |

**Pourquoi cet ordre.** Le lot 1 est d'abord parce qu'il est l'instrument de tous les autres : sans
compteur, « couplage cohérent » n'est pas une propriété mais une impression — et le dépôt vient de
payer cher, sur le rendu, une chaîne de décisions prises sans mesure de ce qu'on regardait (A301).
Les lots 3 et 4 sont avant le 5 parce qu'ils sont plus courts, qu'ils franchissent une porte déjà
décidée (D, donc la v1, ADR-174 D4) et qu'ils n'exigent pas de choisir une représentation neuve.
Le lot 5 est en dernier des lots de physique parce qu'il est un second solveur, et qu'il se
raccordera au premier : mieux vaut que le premier soit mesuré avant.

**Ce que cet ordre ne change pas** : V et les inondations complexes (porte E), la grande échelle
(porte F), les phénomènes secondaires et la finition visuelle gardent leurs déclencheurs et
restent dus (ADR-127). Ce document est un **ordre**, pas un périmètre.

---

## 6. Ce qu'il reste à trancher, et par qui

Trois points ne se décident pas dans une session.

1. **La seconde représentation (lot 5)** — particules (SPH/FLIP) ou surface implicite (level set /
   VOF). Le choix engage un solveur entier et son coût. ADR-175 D5 l'a laissé ouvert
   explicitement. *À proposer avec des éléments chiffrés, puis à trancher avec l'utilisateur.*
2. **La tolérance des bilans du lot 1.** À quel écart de masse un couplage est-il « cohérent » ?
   Le dépôt a des tolérances d'image (3 mm) et de pression, aucune de conservation.
   *Proposition par la session, confirmation par l'utilisateur au premier bilan publié.*
3. **La place de V dans les trois systèmes.** La stratégie ne la nomme pas ; la feuille de route la
   garde en porte E. *Aucune action ; à rappeler quand le lot 4 sera franchi.*

## 7. Limites de ce document

Les états proviennent des preuves citées et du code lu ce jour ; les coûts cités sont ceux
mesurés à leur date, sur la machine de référence, et ne sont pas re-mesurés ici. La
[liste du projet fini](../LISTE-PROJET-FINI.md) n'est pas modifiée : son décompte (3 validés,
49 partiels, 68 absents sur 120) date de S276 et sous-estime l'état de δ 3D, reçu depuis. Aucun
banc nouveau n'a été exécuté ; aucune réception n'est revendiquée.

---

## 8. La liste du projet fini, rangée par système — S309

*Ajouté en S309, 2026-09-20 : l'utilisateur demandait deux confrontations, à l'état du dépôt et à
la [liste de contrôle du projet terminé](../LISTE-PROJET-FINI.md). Voici la seconde.*

**Méthode.** Chaque point est rangé d'après son **énoncé**, pas d'après le code qui l'approche : A
s'il décrit la haute mer superficielle, B la simulation volumique, C la transition entre les deux.
Un point qui ne décrit aucun des trois est dit **hors des trois** — ce n'est pas un rebut, c'est le
reste du moteur. Les états sont ceux de la liste actualisée en S309, recomptés point par point.

| | points | validés | partiels | absents |
|---|---:|---:|---:|---:|
| **A** — haute mer superficielle | 20 | 0 | 8 | **12** |
| **B** — volumique 3D | 29 | 0 | 6 | **23** |
| **C** — transition et couplage | 7 | 0 | 5 | 2 |
| hors des trois | 64 | 3 | 32 | 29 |
| **total** | **120** | **3** | **51** | **66** |

**Quatre lectures, et la première corrige une phrase de ce document.**

1. **« A est une base avancée » est vrai pour la haute mer, et faux pour le reste de A.** Douze de
   ses vingt points sont absents, et ce sont : **lacs (2.3), rivières (2.4), canaux (2.5)**,
   courants macroscopiques 3D (2.6), **bathymétrie et hauts-fonds (2.7)**, précalcul côtier (2.8),
   explosions (3.3), tsunamis (3.4), déferlement de W (3.5), réfraction bathymétrique (3.6),
   couches W au-dessus du plan moyen (3.9), écume (7.1). Ce qui est avancé, c'est **la mer de vent
   et de houle en eau profonde uniforme** — et c'est elle, et elle seule, que §2 de ce document
   déclarait « suffisante pour servir B et C ». La déclaration reste juste **pour le couplage** ;
   elle ne dit rien des autres milieux. Or l'objectif nomme explicitement « les plages, les
   rivières, les piscines » : les rivières sont absentes de A, et une plage demande en plus 2.7,
   3.6 et 4.14, tous absents.
2. **B est l'endroit où le moteur reste à construire** : 23 absents sur 29. Mais ils ne sont pas 23
   travaux indépendants — **la surface non graphe (4.16) en commande à elle seule cinq**, cavité et
   gerbe (4.12), proche-coque (4.13), plage (4.14), spray (7.2) et microbulles (7.3). C'est ce qui
   justifie que le lot 5 soit à la fois le plus lourd et le plus rentable de la stratégie.
3. **C est le plus petit des trois — sept points — et celui qui décide de l'assemblage.** Cinq sont
   déjà partiels : les tuyaux existent. Ses **deux absents sont exactement les lots 1 et 2** —
   sortie des perturbations vers W (4.8) et cohérence de phase δ/B sur la durée de vie d'un domaine
   (4.21). Un troisième point, W au-dessus du plan moyen (3.9), est rangé en A par son énoncé mais
   **bloque C** (A286).
4. **Soixante-quatre points sont hors des trois systèmes.** La stratégie en couvre donc **56 sur
   120**. Ce n'est pas un manque : V (12 points), le rendu (10), activation et budget (13), le
   multijoueur (9), la grande échelle (5), l'outillage (5) et la validation du système (3) viennent
   **après**, par construction de la stratégie elle-même. Mais il faut le dire : **finir A, B et C
   ne fait pas le moteur fini** — cela en fait un peu moins de la moitié, et c'est la moitié dont
   tout le reste dépend.

**Ce que le décompte dit d'autre, et qui n'est pas confortable.** Entre S276 et S308, le dépôt a
écrit un solveur 3D, l'a porté sur GPU et l'a rendu en direct — **sans amener un seul point de
cette liste jusqu'à son périmètre final**. Aucun point ne devient validé. Trois sont même revus
*en moins bien* (4.7, 4.8, 4.18), parce que S308 les a trouvés surestimés. Ce n'est pas un
argument contre le travail fait : c'est un argument pour les **critères de réception** que la
stratégie demande, et pour le lot 1, qui en construit l'instrument.
