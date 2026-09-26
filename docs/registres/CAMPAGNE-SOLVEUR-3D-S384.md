# La campagne du solveur volumique 3D temps réel — conception — S384

2026-09-26. **Décision de l'utilisateur** (S379) : *« une session du plus dur et complexe de la création d'un solveur […]
qui va s'occuper des simulations 3D volumétriques ultra réalistes et performantes en temps réel dynamiquement »* ; R30 :
*« ensuite le plus important le solveur 3D »* ; S384 : *« Solveur 3D ici »*. La [feuille de route](../FEUILLE-DE-ROUTE.md)
§3 ter demande **la conception d'abord** : état de l'art, ce que δ 3D et APIC donnent déjà, l'architecture, les cibles
chiffrées, le découpage en sessions.

Ce document est une **conception**, pas une preuve : il ne mesure rien de neuf. Chaque chiffre renvoie à la preuve qui
l'a mesuré ou à la publication qui l'annonce, marqué **publié** (annoncé par ses auteurs, non reproduit ici), **mesuré**
(par ce dépôt, lien) ou **estimé** (calcul de ce document, dit). Ce qui s'y décide est acté par
[ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) ; ce qui demande l'utilisateur est au §6.

---

## 1. Ce que le dépôt a déjà

### 1.1 δ 3D en colonnes — la représentation par défaut

Grille MAC x-y-z, surface **fonction hauteur** `η(x, y)` à fluide fantôme, pression scindée `p_hydro + p_dyn`, bandes
de couplage à B/W sur les quatre côtés, éponge ([ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) D5).

| ce qui existe | où | chiffre, et sa preuve |
|---|---|---|
| **référence CPU** (`Volume3`, modes linéaire et mobile) | `water-core/src/delta3d*.rs` | réceptions 2D tenues à `ny` = 1 ; onde oblique d'une cuve, dispersion tenue ([S297](../validation/DELTA3D-COUPLEE-S297.md), [S298](../validation/DELTA3D-FOND-REEL-S298.md)) |
| **production GPU**, pas résident, travail borné | `viewer/src/delta3d*.rs`, `delta3d*.wgsl` | suit la référence à **3·10⁻⁷ m** (cas 3), **< 10⁻⁴ m** (cas 1, 2) pour 3 mm exigés ([S305](../validation/CUVE-GPU-S305.md) §9) |
| **coût** : porte C | scène `Config::review` : **120 × 112 × 28 mailles de 25 cm** (30 × 28 × 7 m, 376 320 mailles) | pas entier 4,5 ms (32 cycles de gradient conjugué préconditionné par Jacobi) ; **1,92 ms au 99ᵉ centile par image** à 30 Hz, un pas en deux parts ; projection = 0,087 + 0,062 ms par cycle, bornée par la mémoire (*estimé*) ([S341](../validation/COUT-DELTA3D-S341.md) §2, §11) |
| rendu en direct, interpolé à 30 Hz | afficheur | **1,49 ms** par image avec le rendu ([S341](../validation/COUT-DELTA3D-S341.md) §12) ; R16, R17 |
| **ordonnanceur** : deux domaines 3D, déplacement, redimensionnement, rang 1 | `water-core/src/scheduler.rs`, `viewer/src/delta3d_arbitrage.rs` | aucune image affamée contre 612 ; q99 **4,983 ms pour 5** ([S344](../validation/ARBITRAGE-3D-S344.md) §5–7) |
| **faces coupées** : fond, solide quelconque fixe ou mobile, corps rigide du jeu | `delta3d_cut.rs`, `rigid_body.rs` | ordre **1,956** sur une bosse ; Archimède exact au niveau discret ; `C_m` sphère **0,508** ; C10 tenu ([S324](../validation/FACES-COUPEES-3D-S324.md), [S331](../validation/CORPS-RIGIDE-S331.md), [S336](../validation/RAYONNEMENT-COQUE-S336.md)) ; sur la carte : le pas linéaire, solide fixe ([S358](../validation/LINEAIRE-GPU-S358.md)) |
| **masse** : compteurs exacts | `delta3d_balance.rs` | exacte par télescopage ; cuve fermée, dérive 7,4·10⁻¹² m sur 5 s ([S310](../validation/BILAN-MASSE-S310.md)) |
| **contenant** : bassin en δ, masse à V | `examples/piscine_delta.rs` | niveau de V suivi à **0,1 µm** ; 40 × 20 × 9 à 20 cm, **27 ms par pas** sur la référence CPU, dynamique de quelques millimètres, **invisible** ; il faut 5 à 10 cm, donc la carte ([S375](../validation/PISCINE-DELTA-S375.md)) |

### 1.2 APIC — la seconde représentation, sur banc 2D

Particules sur la grille MAC de δ, la même pression ; surface reconstruite des particules (Zhu et Bridson 2005) ;
**choisie** par l'utilisateur, **pas reçue** ([ADR-186](../adr/ADR-186-apic-seconde-representation.md) §4).

| ce qui existe | chiffre, et sa preuve |
|---|---|
| comparaison au même niveau contre ensemble de niveaux et SPH faiblement compressible | masse exacte, aucune énergie créée, ballottement à **0,15 %** de période, le moins cher : 10 s par seconde simulée contre 27 et 405 ([S318](../validation/COMPARAISON-LOT5-S318.md)) |
| B10 : un cylindre entre dans l'eau | cavité qui se pince à `t ≈ 2,2–2,5 √(D/g)`, à 10⁻⁴ d'une échelle à l'autre, **5 %** d'une maille à sa moitié ; couronne et jet **non convergés** (40 à 60 %) : sans tension de surface, la maille les arrête ([S320](../validation/B10-APIC-S320.md)) |
| raccord particules ↔ colonnes | masse exacte ; **frontière non reçue** : surface décalée de 1,8 à 2,8 mailles, ballottement amorti jusqu'à 16 % par période (S325) ; sur 30 s, la frontière ne tient pas la **densité** des particules, la masse migre (A316, [§13](../validation/B10-APIC-S320.md)) |

**Ce qu'APIC n'a pas vu** : la troisième dimension, la carte graphique, une référence expérimentale.

### 1.3 Les limites connues, qui commandent la conception

| limite | ce qu'elle dit | registre |
|---|---|---|
| **une seule surface par colonne** | cavité, jet, déferlement, gerbe, lame de déversoir : hors de portée des colonnes | ADR-175 D5, ADR-186 |
| **A297** — bascule de mouillure | grain de maille 2,0–2,2 mm RMS, du schéma, inchangé de 32 à 512 cycles | [file](QUESTIONS-OUVERTES.md#file-active) |
| **A298** — écart séculaire carte / référence | ≈ 1,2·10⁻⁷ m par seconde sur la cuve ; suspect non démontré | file |
| **A316** — densité des particules à la frontière | la masse migre de part et d'autre du raccord | file |
| **A320** — perturbation qui croît sous houle raide | 0,05–0,06 s⁻¹ sous `ak` = 0,079, convective ; bloque l'ordre E du retour δ → W | [S369](../validation/MER-S369.md) |
| **pression sans multigrille en 3D** | 32 cycles de gradient conjugué, déjà déclarés dégradés ; la multigrille n'existe qu'en 2D mobile ([ADR-167](../adr/ADR-167-multigrille-du-mode-mobile.md)) | S341 §3 |
| **une boîte dense par domaine** | 376 320 mailles pour 30 × 28 × 7 m ; aucune grille éparse (B5 non fait) | ADR-175 D6 |
| **δ n'est pas dans Godot** | la production vit dans l'afficheur ; *« pas maintenant »* (R27) — c'était le 2026-09-26, avant R30 | ADR-202 |

---

## 2. L'état de l'art du volumique temps réel

**Méthode de lecture.** Les sources ont été identifiées par recherche (auteurs, lieu, année, résumé) ; **le réseau de la
session S384 ne permettait pas de lire les articles** (hôtes bloqués). Tout chiffre ci-dessous est donc celui d'un
**résumé publié**, dit tel ; aucun chiffre des tableaux des articles n'est repris de mémoire (I-14). Relire les articles
est une action de la file, au premier lot qui en dépend (§5).

### 2.1 Grilles hybrides : colonnes hautes, fonction hauteur, 3D et particules

| travail | idée | ce qui nous concerne |
|---|---|---|
| **Irving, Guendelman, Losasso et Fedkiw (2006)**, *Efficient simulation of large bodies of water by coupling two and three dimensional techniques*, ACM TOG 25(3), 805–811 | le gros du volume en **colonnes hautes** à profil de pression linéaire, comme une fonction hauteur ; tout le haut de l'eau en Navier-Stokes 3D à surface libre | la pression scindée `p_hydro + p_dyn` de δ en est l'esprit ; nos 28 couches de 25 cm, elles, sont toutes cubiques |
| **Chentanez et Müller (2011)**, *Real-time Eulerian water simulation using a restricted tall cell grid*, ACM TOG 30(4), 82 (SIGGRAPH 2011) | **temps réel** : cellules cubiques au-dessus d'une couche de colonnes hautes, au-dessus d'un fond quelconque ; **multigrille spécialisée** pour Poisson ; pas de temps longs stabilisés | la voie directe pour descendre la maille de δ sans payer la profondeur : l'effort va **près de la surface**, là où il compte |
| **Narita, Ochiai, Kanai et Ando (2025)**, *Quadtree Tall Cells for Eulerian Liquid Simulation*, SIGGRAPH 2025 Conference Papers | colonnes hautes sur un **quadtree** horizontal : la résolution horizontale s'adapte aussi | l'étape d'après ; à lire avant de choisir la forme des blocs (§5) |
| **Chentanez, Müller et Kim (2014)**, *Coupling 3D Eulerian, Heightfield and Particle Methods for Interactive Simulation of Large Scale Liquid Phenomena*, SCA 2014, puis IEEE TVCG 21(10), 2015 | **interactif** : fonction hauteur (eaux peu profondes) hors du domaine 3D, grille 3D, et **particules pour le gros de l'eau près de la surface** là où il le faut, avec bascule de l'une à l'autre au cours du calcul ; raccord tel que les vagues traversent la frontière | **c'est notre architecture** — B+W dehors, δ 3D, APIC là où la surface n'est pas un graphe (ADR-186 D3). Leur raccord particules ↔ grille est exactement ce qui nous manque (A316) |
| **Huang, Qu, Tan, Zhang, Michels et Jiang (2021)**, *Ships, Splashes, and Waves on a Vast Ocean*, ACM TOG 40(6) (SIGGRAPH Asia 2021) | **plusieurs domaines FLIP mobiles** près des navires, couplés dans les deux sens à un océan profond par éléments de frontière : FLIP corrige l'océan, l'océan donne à FLIP le flux ambiant | la même séparation que B+W (analytique, dispersion juste) et δ (volumique local) ; leurs domaines **mobiles** sont ceux de la porte A (S349–S350) |

### 2.2 La pression sur la carte

| travail | idée | ce qui nous concerne |
|---|---|---|
| **McAdams, Sifakis et Teran (2010)**, *A parallel multigrid Poisson solver for fluids simulation on large grids*, SCA 2010, 65–73 | un cycle **multigrille géométrique** comme **préconditionneur** du gradient conjugué (MGPCG), robuste sur domaines irréguliers, Neumann et Dirichlet mêlés ; *publié* : jusqu'à 768 × 768 × 1 152 voxels sous 16 Go | notre projection fait 32 itérations préconditionnées par **Jacobi**, déjà déclarées dégradées (S302) ; la multigrille n'existe qu'en 2D mobile (ADR-167) |
| **Chentanez et Müller (2012)**, *A multigrid fluid pressure solver handling separating solid boundary conditions*, IEEE TVCG 18(8), 1191–1201 | la multigrille étendue au **problème de complémentarité** des parois qui se séparent | l'eau qui colle au solide qui s'en va — la coque qui sort de l'eau (porte D) |
| **Weber, Mueller-Roemer, Stork et Fellner (2015)**, *A Cut-Cell Geometric Multigrid Poisson Solver for Fluid Simulation*, CGF (Eurographics 2015), 481–491 | multigrille géométrique sur **faces coupées**, systèmes compatibles à tous les niveaux ; *publié* : convergence meilleure que les méthodes antérieures | nos faces coupées (S232, S324) devront entrer dans la multigrille, sinon A315 revient (les petites cellules qui font ramper le solveur) |

### 2.3 Les grilles éparses

| travail | idée | ce qui nous concerne |
|---|---|---|
| **Museth (2013)**, *VDB: High-resolution sparse volumes with dynamic topology*, ACM TOG 32(3), 27 | arbre peu profond de blocs, topologie dynamique | le standard des volumes épars ; la forme, pas la bibliothèque (aucune dépendance, ADR-020) |
| **Setaluri, Aanjaneya, Bauer et Sifakis (2014)**, *SPGrid: a sparse paged grid structure applied to adaptive smoke simulation*, ACM TOG 33(6), 205 | grille éparse par **pages**, adressage par la mémoire virtuelle | des blocs 8³ indexés : la forme d'ADR-006 §3 |
| **Wu, Truong, Yuksel et Hoetzlein (2018)**, *Fast Fluid Simulations with Sparse Volumes on the GPU*, CGF 37(2), 157–167 | FLIP sur une hiérarchie éparse **construite et tenue sur la carte** au fil des particules ; gradient conjugué sans matrice ; *publié* : jusqu'à un ordre de grandeur plus vite que FLIP sur CPU | des blocs qui suivent la surface et les particules, sur la carte — ce qu'il faudra aux domaines qui bougent |

**Ce que 2.1 à 2.3 disent ensemble.** Le temps réel volumique à grande échelle se fait, depuis 2011, en **mettant
les mailles près de la surface** (colonnes hautes), en **résolvant la pression par multigrille**, et en **ne mettant de
particules que là où la surface cesse d'être un graphe**. Le dépôt a le troisième choix (ADR-186 D3) et le cadre des
deux autres (pression scindée, blocs 8³) ; il n'a ni colonnes hautes, ni multigrille 3D, ni blocs épars.

### 2.4 Particules sur grille : FLIP, APIC, MPM

| travail | idée | ce qui nous concerne |
|---|---|---|
| **Jiang, Schroeder, Selle, Teran et Stomakhin (2015)**, *The affine particle-in-cell method*, ACM TOG 34(4), 51 | chaque particule porte une vitesse **affine** : ni la dissipation de PIC, ni le bruit ni l'instabilité de FLIP | la représentation retenue (ADR-186) |
| **Ferstl, Ando, Wojtan, Westermann et Thuerey (2016)**, *Narrow Band FLIP for Liquid Simulations*, CGF 35(2), 225–232 | des particules **seulement dans une bande** sous la surface, le reste sur la grille ; *publié* : calculs hors pression plus de 6 fois plus rapides, 250 images de 4,4 h à 2 h, mémoire moitié | la bande est la forme naturelle de nos particules — « là où elles sont nécessaires » (ADR-186 D3) |
| **Hu, Fang, Ge, Qu, Zhu, Pradhana et Jiang (2018)**, *A moving least squares material point method…* (MLS-MPM), ACM TOG 37(4), 150 | MPM plus rapide, couplage aux corps rigides dans les deux sens, découpe | la famille d'APIC ; son intérêt pour nous est **au-delà de l'eau** — neige, boue (ADR-203 D5) |
| **Gao, Wang, Wu, Pradhana, Sifakis, Yuksel et Jiang (2018)**, *GPU optimization of material point methods*, ACM TOG 37(6), 254 ; **Fei, Huang et Gao (2021)**, *Principles towards Real-Time Simulation of Material Point Method on Modern GPUs*, arXiv 2111.00699 | le transfert particules ↔ grille **saturant la carte** : tri des particules par bloc, dispersion sans conflit | le coût d'APIC sur la carte se joue là : le transfert, pas la pression |

### 2.5 Les autres familles

| travail | idée | ce qui nous concerne |
|---|---|---|
| **Macklin, Müller et Bridson (2013)**, *Position based fluids*, ACM TOG 32(4) (SIGGRAPH 2013) | contraintes de densité dans la dynamique des positions : grands pas, **temps réel** | robuste et rapide, mais surface particulaire bruitée et compressibilité ; S318 a mesuré SPH faiblement compressible à **40 fois** le coût d'APIC ([S318](../validation/COMPARAISON-LOT5-S318.md)) |
| **Bender et Koschier (2015)**, *Divergence-free smoothed particle hydrodynamics* (DFSPH), SCA 2015 | incompressibilité et divergence nulle résolues ensemble : moins d'itérations, pas plus grands | le meilleur SPH incompressible ; il resterait SPH : voisinages, surface implicite, aucun raccord aux colonnes |
| **Lehmann** — *FluidX3D* (logiciel ; mémoire de master 2019, *High Performance Free Surface LBM on GPUs* ; thèse 2023) | **Boltzmann sur réseau**, surface libre par volume de fluide et PLIC, sur la carte | très rapide par maille, mais **un autre schéma entier** : aucune de nos réceptions (fluide fantôme, faces coupées, couplage à B/W) ne s'y transporte ; logiciel **non commercial** — seule la méthode serait reprise |
| **Ihmsen, Akinci, Akinci et Teschner (2012)**, *Unified spray, foam and air bubbles for particle-based fluids*, The Visual Computer 28, 669–677 | embruns, écume et bulles comme **particules diffuses**, classées, en post-traitement de l'eau | les phénomènes secondaires (liste 8.4 : écume, spray, bulles ; SPEC-002) ; ils se branchent **sur** le solveur, ils n'en sont pas |

**Ce que 2.4 et 2.5 disent.** Aucune famille ne remplace la grille pour le gros de l'eau d'un jeu : les particules seules
(PBF, SPH) paient les voisinages et une surface bruitée, Boltzmann paie un autre schéma entier. Le choix que le dépôt a
fait — la grille pour la pression, des particules APIC **dans une bande** là où la surface n'est pas un graphe, des
particules diffuses **par-dessus** pour ce qui est plus petit que la maille — est celui de la littérature du temps réel
depuis Chentanez, Müller et Kim (2014). **Ce qui reste à gagner est d'exécution** : colonnes hautes, multigrille,
blocs épars, transfert sur la carte — et le raccord, qui n'est reçu nulle part dans le dépôt.

---

## 3. Les cibles chiffrées

### 3.1 Les usages — ce que le solveur doit porter

Tirés de la [liste du projet fini](../LISTE-PROJET-FINI.md) §4 à §7 et des décisions de l'utilisateur ; chacun dit la
maille qu'il demande et **d'où vient ce chiffre**.

| usage | points | maille | provenance de la maille |
|---|---|---|---|
| **le joueur dans l'eau** : nage, saut, objet qui tombe — cavité, couronne, jet | 4.12, 4.16, 5.10 | **5 cm** | la piscine : une dynamique visible demande 5 à 10 cm ([S375](../validation/PISCINE-DELTA-S375.md) §6) ; une onde sous la demi-maille n'existe pas pour des particules ([S318](../validation/COMPARAISON-LOT5-S318.md) §2, faute 2) : 5 cm portent 2,5 cm |
| **la coque** : proche-coque, gerbe d'étrave, sillage près du joueur | 4.13, 6.x | **10 à 25 cm** | 25 cm : la scène reçue des portes B à D ([S302](../validation/SCENE-DELTA3D-S302.md), [S333](../validation/PORTE-D-S333.md)) ; la gerbe d'étrave, plus fine : *à calibrer* sur sa scène (C10) |
| **la surface qui cesse d'être un graphe** : déferlement, lame du déversoir, jet de pompe | 4.16, ADR-202 D5 | celle du domaine hôte | APIC dans le domaine, pas un domaine à part (ADR-186 D2) |
| **la plage, les rochers** : rouleau, mouillage, obstacles | 4.14, 4.15 | **10 à 25 cm** | *à calibrer* ; C04 et les faces coupées fixent la géométrie |
| **l'inondation** : l'eau entre dans un navire, un bâtiment ; V garde la masse | porte E, 5.x | **10 à 25 cm** | *à calibrer* ; le critère est la masse identique avec et sans δ (C21) |
| **embruns, écume, bulles** | 7.x, 8.4 | sous la maille | particules diffuses **par-dessus** δ (§2.5), rendu seulement (I-04) |

### 3.2 Les contraintes qui ne se négocient pas

| cible | valeur | provenance |
|---|---|---|
| **budget GPU de δ**, tous domaines ensemble | **≤ 2 ms par image**, 99ᵉ centile, dans un profil d'eau de 4 ms | [ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D3, machine de référence D1 |
| **cadence** | pas à **30 Hz**, étalé sur deux images de 60 Hz, rendu interpolé | [ADR-012](../adr/ADR-012-ordonnanceur-budget-degradation.md) §7 ; R17 ; S348, S353 |
| **travail borné par pas** | aucune boucle jusqu'à convergence dans l'image ; qualité mesurée, dégradation déclarée | [ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) D2–D3, I-05 |
| **production contre référence** | **≤ 3 mm** de hauteur, pente et phase publiées | ADR-175 D4 ; METHODE (S201) |
| **masse** | exacte au compteur ; V autoritaire dans les contenants | [S310](../validation/BILAN-MASSE-S310.md) ; ADR-025, ADR-200 D2 |
| **durée** | un comportement s'éprouve sur sa durée d'usage : **une minute** au moins, cinq pour un contenant | METHODE (L369) ; la piscine, 330 s ([S375](../validation/PISCINE-DELTA-S375.md)) |
| **aucune autorité, aucune sérialisation, aucune allocation** | I-04, I-17, I-06 | invariants |

### 3.3 Le budget en mailles — ce que 2 ms achètent aujourd'hui

*Estimé*, du coût mesuré : le pas de la porte B coûte **3,68 ms** pour **376 320 mailles** à 32 cycles
([file](QUESTIONS-OUVERTES.md#file-active), porte C, leviers S342–S343), soit **≈ 9,8 ns par maille et par pas** ; étalé sur
deux images, **1,92 ms** au pire centile par image ([S341](../validation/COUT-DELTA3D-S341.md) §11). **Le domaine de la
porte B consomme donc seul le budget de δ, à 4 % près.** Sans rien changer au coût par maille, la campagne dispose
d'environ **0,4 million de mailles** pour tous ses domaines.

| forme, sur ces 0,4 M mailles (*estimé*) | colonnes | côté d'un domaine carré |
|---|---:|---:|
| boîte dense actuelle, 25 cm, 28 couches (la porte B, 30 × 28 m) | 13 440 | — |
| **colonnes hautes**, 25 cm, 8 couches cubiques (2 m) + 1 haute | 44 444 | **53 m** |
| colonnes hautes, 10 cm, 20 couches (2 m) + 1 | 19 048 | **14 m** |
| colonnes hautes, **5 cm**, 20 couches (1 m) + 1 | 19 048 | **6,9 m** |
| boîte dense, 5 cm, 7 × 7 × 2 m | — | 784 000 mailles, **deux fois le budget** |

**Ce que le tableau décide.** (1) Les **colonnes hautes** sont la première marche : à la porte B, 9 mailles par colonne au
lieu de 28, **3,1 fois moins** (*estimé*, à mesurer en C2) — et ce gain paie la maille fine. (2) Un domaine du joueur à
5 cm tient en **7 m de côté** s'il est **seul** ; avec une coque en même temps, il faut aussi baisser le **coût par
maille** : cible **÷ 2**, *à calibrer* en C3 (multigrille à résidu égal, fusion des noyaux, précision mixte — S341 §3,
techniques absentes). (3) Les **blocs épars** (4.3) ne rendent rien sur une boîte pleine d'eau ; ils rendent sur un
domaine qui suit une perturbation ou une surface découpée — ils viennent après (C9).

### 3.4 Le critère d'arrêt de la campagne

La campagne est **finie** quand, sur la machine de référence, **dans la même scène vivante** : un joueur saute dans l'eau
(5 cm) pendant qu'une coque passe (≤ 25 cm) — cavité, couronne, jet, gerbe d'étrave portés par δ et APIC —, **δ ≤ 2 ms**
au 99ᵉ centile par image, la production à **3 mm** de la référence sur les cas de réception, la masse exacte, **et
l'utilisateur juge le rendu convaincant** contre des références réelles (REVUE-VISUELLE). Les points 4.1, 4.3, 4.12, 4.16 et
4.19 de la liste changent alors d'état ; aucun n'est déclaré validé hors de son périmètre final.

*Les mailles de ce document sont des niveaux d'[ADR-006](../adr/ADR-006-cellules-domaines-solveurs.md) §3.2 — 5, 10 et
25 cm parmi `{2 ; 5 ; 10 ; 25 ; 50 ; 100}` cm.*

---

## 4. L'architecture

### 4.1 Cinq décisions de structure

**A1 — Une grille, trois représentations superposées.** Dans un domaine : les **colonnes** (fonction hauteur) partout où
la surface est un graphe ; des particules **APIC dans une bande** sous la surface, là où elle ne l'est plus ; des
**particules diffuses** par-dessus, pour l'image seulement. C'est ADR-175 D5 et ADR-186 D1–D3, plus la troisième, que la
littérature du temps réel pose depuis Chentanez, Müller et Kim (2014) et Ihmsen *et al.* (2012).

**A2 — Des colonnes hautes dans chaque domaine.** Sous `k` couches cubiques qui suivent la surface, une **maille haute**
jusqu'au fond, à profil de pression linéaire (Irving *et al.* 2006 ; Chentanez et Müller 2011). C'est la réponse à la
question qu'[ADR-006](../adr/ADR-006-cellules-domaines-solveurs.md) §6.4 reportait « après B3 » — un `dx` plus fin en
vertical près de la surface — et la suite naturelle de la pression scindée `p_hydro + p_dyn` d'ADR-175 D5. `k` est une
**allocation** (mailles par colonne), donc un paramètre de profil admissible (I-16) ; sa valeur est *à calibrer* en C2
contre les réceptions de dispersion.

**A3 — La pression par gradient conjugué préconditionné par un cycle multigrille** (McAdams *et al.* 2010), sur les
mailles cubiques **et** hautes (Chentanez et Müller 2011), les faces coupées **compatibles à tous les niveaux** (Weber
*et al.* 2015 — sinon A315 revient), en **travail fixe** par pas : un nombre de cycles du profil, la divergence résiduelle
mesurée et publiée, la dégradation déclarée (ADR-175 D2–D3). La multigrille 2D du mode mobile (ADR-167) est le point de
départ du cœur.

**A4 — Des domaines faits de blocs, un `dx` par domaine, choisi par l'ordonnanceur.** Un domaine est un ensemble épars
de blocs de colonnes (ADR-006 §3 : fusion = union, séparation = partition), à un seul niveau de `dx`. Le **niveau de
détail** est ce choix : V seul, effets factices, puis δ à 25, 10 ou 5 cm selon la distance et ce qui perturbe (ADR-202
D1, ADR-203 D4 et D7), sous le budget commun et l'ordre de dégradation d'ADR-012 §4 (rang 4 : descendre d'un niveau).
La **prévision** d'ADR-013 prépare le domaine avant l'arrivée du perturbateur (ADR-202 D2).

**A5 — Deux implémentations, comme aujourd'hui** (ADR-175 D1). La **référence CPU** du cœur définit et reçoit chaque
étage contre des oracles — elle se construit **sans carte graphique**, dans une session comme S384 ; la **production**
sur la carte le reproduit à 3 mm, sous budget — elle demande le poste. Où vit la production à la fin (afficheur ou
Godot) est une question à l'utilisateur (§6).

### 4.2 Le pas, dans l'ordre

1. **Fond et couplage** : B + W factorisés par colonne et par couche (S342), bandes de couplage, éponge — inchangés.
2. **Transport** : colonnes comme aujourd'hui ; dans la bande, les particules APIC, puis leur transfert vers la grille.
3. **Forces** : `g_eff` injecté (I-07), sources et puits de V (ADR-200 D2), solides par faces coupées.
4. **Projection** (A3) sur toutes les mailles, fluide fantôme à la surface : `η` pour les colonnes, la surface
   reconstruite des particules dans la bande (ADR-186 D1).
5. **Retour** : vitesses aux particules, surface des colonnes transportée.
6. **Bascule** colonnes ↔ particules, à masse exacte (S323), selon le critère de C7.
7. **Publication** : surface immuable pour l'image (ADR-175 D7), diagnostics différés, émission des particules diffuses.

### 4.3 Écartées, et pourquoi

| alternative | raison | nature de la raison |
|---|---|---|
| boîte dense à 5 cm | 7 × 7 × 2 m = 784 000 mailles, deux fois le budget entier (§3.3) | *estimé* |
| ensemble de niveaux comme représentation principale | volume ±7,6 %, énergie créée 8 % en écoulement violent | *mesuré*, [S318](../validation/COMPARAISON-LOT5-S318.md) |
| SPH ou PBF comme représentation principale | SPH faiblement compressible : 40 fois le coût d'APIC ; surface particulaire ; aucun raccord aux colonnes | *mesuré* (S318) ; le reste, §2.5 |
| Boltzmann sur réseau à surface libre | un autre schéma entier : aucune réception du dépôt ne s'y transporte ; pas explicite lié à la maille | §2.5 ; *non mesuré* ici |
| octree ou quadtree adaptatif | ADR-006 §3.1 a choisi des blocs de taille fixe pour que fusion et séparation soient des opérations d'ensemble ; les colonnes hautes en quadtree (Narita *et al.* 2025) restent **à lire**, pas écartées | décision antérieure |

### 4.4 Ce que l'architecture fait des limites connues

- **A315** (petites cellules des faces coupées) : la multigrille compatible (A3) ; à vérifier en C1.
- **A316** (densité à la frontière du raccord) : le raccord de Chentanez, Müller et Kim (2014), **à lire** avant C6.
- **A297** (bascule de mouillure) et **A298** (écart séculaire de la carte) : non réglés par construction ; **remesurés**
  sur le nouveau pas (C2, C3).
- **A320** (perturbation qui croît sous houle raide) : relève du couplage à B (lot 2), pas de la structure du solveur ;
  inchangée.

---

## 5. Le découpage en sessions

Chaque session a son critère **« reçu si »**, écrit ici avant elle, et son **lieu** : **cloud** — fichiers, git, cargo,
Python, sans carte graphique ni Godot, comme S384 — ou **poste** — la machine de référence, carte et Godot. **Une seule
session écrit à la fois**, où qu'elle tourne (AGENTS.md) ; l'alternance rendu / physique d'ADR-191 continue, et les
sessions de rendu (la pluie, pièce 5 ; R32) se glissent entre celles-ci, au poste.

| | session | lieu | point, porte | reçu si |
|---|---|---|---|---|
| **C1** | **multigrille 3D** dans la référence : le préconditionneur de S245 (2D, `delta_projection.rs`) porté au pas 3D, lissage de Jacobi amorti **dérivé** pour le stencil à sept points (ω = 6/7, facteur de lissage 5/7 : le calcul de S246 étendu à trois dimensions, modes (π/2, 0, 0) et (π, π, π)), faces coupées comprises | cloud | 4.19, 4.1 | mêmes solutions à la tolérance d'ADR-144 ; itérations **indépendantes de la maille** à trois mailles au moins (L274) ; la bosse de S324 sans ramper (A315) ; suite entière au bit hors du chemin préconditionné |
| **C2** | **colonnes hautes** dans la référence : `k` couches cubiques sous la surface, une maille haute au fond ; la multigrille de C1 les porte | cloud | 4.1, 4.3 ; ADR-006 §6.4 | à `k` réduit, les réceptions tiennent : onde oblique de la cuve (dispersion), cas 2D contre HOS à `ny` = 1 (S253), cuve fermée (masse) ; mailles comptées : ≈ 3 fois moins à la porte B (*estimé* §3.3) ; `k` minimal consigné |
| **C3** | multigrille et colonnes hautes **sur la carte** | poste | porte C, 4.19 | production à 3 mm de la référence sur les trois cas de cuve ; divergence publiée ; δ ≤ 2 ms au 99ᵉ centile sur la scène de la porte B, **puis à 10 cm** sur une scène de même surface ; A298 remesurée |
| **C4** | **APIC 3D** dans le cœur, avec ses essais (METHODE : un banc qui entre au système y entre avec ses chiffres) : B10 en 3D, sphère qui entre dans l'eau | cloud | 4.12, 4.16 | masse exacte ; temps de pincement convergé à 5 % sur trois mailles ; comparé à une **mesure publiée** de cavité de sphère (*à trouver*, I-14) ; le banc 2D de S318 au bit |
| **C5** | le **raccord** particules ↔ colonnes en 3D : A316, d'après Chentanez, Müller et Kim (2014), **lus d'abord** | cloud | 4.16, A316 | masse exacte ; surface continue à la frontière **sous 3 mm** ; le ballottement traversant la frontière à la période d'APIC seul (0,15 % à la maille fine, S318) sur **30 s** |
| **C6** | le **critère de bascule** : où vivent les particules — pli de la surface prédit, cavité, jet, objet qui entre | cloud | 4.16, 4.10 | sur B10 et sur une vague qui déferle : particules seulement dans la bande, colonnes ailleurs ; aucune bascule qui oscille (hystérésis mesurée) ; coût compté |
| **C7** | APIC **sur la carte** : transfert trié par bloc (Gao *et al.* 2018 ; Fei *et al.* 2021) | poste | 4.19 | B10 de la production à 3 mm de la référence ; coût par particule publié ; δ ≤ 2 ms avec la bande |
| **C8** | **blocs épars** et domaine qui suit la perturbation ; niveaux de `dx` choisis par l'ordonnanceur (rang 4) ; prévision | poste (référence : cloud) | 4.3, 4.5, 4.9, 9.2 | un domaine suit un objet, change de niveau sans rupture visible (I-12) ; fusion et séparation par ensembles ; famine : issue déclarée |
| **C9** | **particules diffuses** : embruns, écume, bulles émis par δ | poste | 7.x, 8.4 | rendu seulement (I-04) ; jugé contre des photographies réelles |
| **C10** | **les scènes** : le joueur qui saute à 5 cm, la gerbe d'étrave, la lame du déversoir (ADR-202 D5) | poste | 4.12, 4.13, 5.10 | le critère d'arrêt du §3.4, revue de l'utilisateur comprise |
| **C11** | δ **dans Godot** (nuanceurs de calcul), si l'utilisateur le décide (§6) | poste | 8.1, ADR-192 | la scène de C10 rendue dans Godot, identique à la production de l'afficheur à 3 mm |

**L'ordre protège une dépendance chaque fois** (METHODE, L343) : C1 avant C2, parce que les colonnes hautes rendent le
système de pression plus anisotrope et que le gradient conjugué seul y rampe déjà (A315) ; C2 avant C3, parce que la
carte reproduit la référence et ne la définit pas (ADR-175 D1) ; C4 avant C5 et C6, parce qu'on ne raccorde pas ce qui
n'existe pas en 3D ; C3 et C7 avant C10, parce que la scène se juge sous budget. **C1, C2, C4, C5 et C6 — cinq sessions —
se font sans carte**, dans une session comme celle-ci.

**Lectures à faire**, faute de réseau en S384 (§2) : Chentanez et Müller (2011) avant C2 ; Chentanez, Müller et Kim (2014)
avant C5 ; Narita *et al.* (2025) avant C8 ; Gao *et al.* (2018) avant C7. Une session qui ne peut pas les lire le dit et
avance sur ce qui ne dépend pas d'elles.

---

## 6. Ce qui demande l'utilisateur

1. **Où vit δ à la fin.** Aujourd'hui, la production de δ est dans l'afficheur (le banc) ; le rendu final de l'eau est
   dans Godot (ADR-192) ; δ dans Godot a reçu *« pas maintenant »* (R27). **Proposition** : la campagne construit la
   production dans l'afficheur jusqu'à C10, où les dépendances sont autorisées et verrouillées ; δ entre dans Godot en
   C11, quand la scène est reçue. *À savoir* : faut-il le faire plus tôt, pour juger les scènes directement dans Godot ?
2. **Rien d'autre n'est demandé.** Mailles, colonnes hautes, multigrille, bascule et ordre relèvent de l'autonomie
   technique (S71) et sont actés par l'ADR de la campagne. Les références visuelles, nous les trouvons (S366).

---

## En une phrase

Le dépôt a déjà le **bon choix de représentation** — la grille pour la pression, APIC là où la surface n'est pas un
graphe —, celui de la littérature du temps réel depuis 2014 ; il lui manque **l'exécution qui le rend abordable** : le
domaine de la porte B consomme seul les 2 ms de δ, et la campagne les regagne par les **colonnes hautes** (÷ 3 mailles,
*estimé*) et la **multigrille**, avant d'ajouter APIC en 3D, son raccord et les scènes — onze sessions, dont cinq sans
carte graphique, la première étant la multigrille 3D de la référence.
