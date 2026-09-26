# La campagne du solveur volumique 3D temps réel — conception — S384

2026-09-26. **Décision de l'utilisateur** (S379) : *« une session du plus dur et complexe de la création d'un solveur […]
qui va s'occuper des simulations 3D volumétriques ultra réalistes et performantes en temps réel dynamiquement »* ; R30 :
*« ensuite le plus important le solveur 3D »* ; S384 : *« Solveur 3D ici »*. La [feuille de route](../FEUILLE-DE-ROUTE.md)
§3 ter demande **la conception d'abord** : état de l'art, ce que δ 3D et APIC donnent déjà, l'architecture, les cibles
chiffrées, le découpage en sessions.

Ce document est une **conception**, pas une preuve : il ne mesure rien de neuf. Chaque chiffre renvoie à la preuve qui
l'a mesuré ou à la publication qui l'annonce, marqué **publié** (annoncé par ses auteurs, non reproduit ici), **mesuré**
(par ce dépôt, lien) ou **estimé** (calcul de ce document, dit). Ce qui s'y décide est acté par
l'ADR de la campagne ; ce qui demande l'utilisateur est au §6.

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
