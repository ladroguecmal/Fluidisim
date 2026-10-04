# Journal des sessions — S400 à S469 (archive)

*Archivé en S480, 2026-10-04, depuis [`notes/JOURNAL.md`](../JOURNAL.md) ; texte inchangé (les liens relatifs recalés d'un niveau), 70 entrées.* Le journal vivant
garde les entrées depuis S470 ; une archive ne se modifie plus.

---

## S400 — 2026-09-27 — physique : C5b, troisième part — la zone lit comme la bande ; une référence de rendu rangée

**Entrée.** *« https://scottiefox.github.io/caustic-volume/ […] les objectifs de cette référence ne sont pas les mêmes que mon projet.
Continue »*. **Fait.** (1) La référence, lue à son dépôt (three.js, MIT), rangée aux [comparables](../../docs/COMPARABLES-EXTERNES.md) :
même méthode que nos caustiques (Wyman) ; elle montre ce qui nous manque — rayons et caustiques **dans** l'eau par un volume de
tranches (8.5) — à juger par conservation et photographie, sans son gain artistique ; aucun de ses nombres comme seuil. (2) Le
raccord ([preuve](../../docs/validation/RACCORD-3D-S398.md) §6) : **la zone lit sa surface comme la bande** (`η + e(η)`, table du
réseau nominal, seulement s'il y a une bande) ; la séparation tenue du côté de la bande. **Tenus** : repos **2·10⁻⁵ m/s** (S399 :
1,007 cm/s ; vu échouer sans la table) ; migration à 5 cm +1,8 mm (±2) ; sans zone et toutes colonnes au bit ; suite 697 réussis.
**Manqués** : densité (7,2 à 2,5 cm ; 7,54 à 5 cm) ; courant de surface à 5 cm (−6,7 mm/s). **Témoins** : la migration était la
réponse d'équilibre au biais de lecture ; le courant n'est pas la lecture (aucune marche lue ; la zone seule n'en a pas) — il est
au raccord ; la séparation ne change rien. **Rituel.** Maillons **4** : aucun point ne change d'état. **Justification** : la demande
(terminer le solveur) ; deux des trois défauts levés, deux attributions réfutées et dites. **Règle de S294** : pas de quatrième
session consécutive du raccord — il revient sous la condition d'A316 (un témoin court, 5 cm et 45 s, qui abaisse le courant sous
5 mm/s par un geste attribué d'avance). Suivant : dans le cloud, **C8b**, le domaine épars ; au poste, C3b puis la pluie 5b.

## S401 — 2026-09-27 — physique : C8b, le domaine épars qui suit la perturbation, en référence

**Entrée.** *« Reprends le projet »*, session cloud neuve partie de `main` (S383) : la lignée vivante était
`claude/blissful-pasteur-m4j5rn` (S400, jeton libre, après `poste`, S392) — avance rapide, aucun fork ; suite de S400 : **C8b**.
**Fait** ([preuve](../../docs/validation/DOMAINE-EPARS-S401.md)). `delta3d_sparse.rs` — un domaine est un **ensemble de blocs** dans une
fenêtre : les colonnes dehors, solides et fermées (le chemin de la découpe de S328, multigrille comprise), au repos ; le bord de
l'ensemble lu **comme celui de la boîte** ; ce qu'une colonne libérée portait, publié. `Follow` — un bloc requis à moins de 4 m
d'un bloc actif (1 mm) ou de l'**enveloppe** d'ADR-013 §2 (`p + V·t`, élargie de `½·a_max·t²`), libéré 0,25 s après. **Tenus** :
ensemble plein au bit sur 50 pas ; un rectangle et deux parties à **un ulp** du dense (0,14 mm si l'advection lit le bord à zéro :
vu échouer) ; 100 % de 300 manœuvres bornées dans l'enveloppe ; une source mobile suivie à **0,26–0,66 mm** du domaine entier
(critère 3 mm), 26 à 71 % des mailles ; revu chaque seconde, la source sort à 0,62 s sans prévision, reste dedans avec ; suite
**707 réussis**, zéro avertissement. **Trouvé** : à cadence fine, l'enveloppe coûte 1,7 à 2,6 fois les mailles sans gain — sa place
est T2 (ADR-013) ; le seuil absolu fait 25 % d'une amplitude de 2,6 mm ; un premier essai de suivi, dont l'ensemble couvrait la
fenêtre, ne mesurait rien — agrandi. **Limites** : pas de pool de blocs (la mémoire reste la fenêtre) ; la référence ne gagne que
ce que ses opérateurs sautent ; bassin fermé ; ni pas couplé, ni corps sous l'ensemble. **Rituel.** Maillons **0** : **9.2 passe à
partiel** — **devient possible** un domaine δ qui suit sa perturbation sur un ensemble épars et s'étend devant l'objet, sans
rupture visible ; **chemin** : le pool de blocs sur la carte (C8, poste), l'ordonnanceur (rang 4), la scène C10 ; **preuve** :
DOMAINE-EPARS-S401. Suivant : dans le cloud, **C8c** — niveaux de `dx` et famine ; au poste, C3b puis la pluie 5b.

## S402 — 2026-09-27 — physique : ADR-005 restauré ; C8c, changer un domaine de niveau par transfert d'état

**Entrée.** *« Continue »* ; S401 proposait C8c. **Trouvé en lisant le lot** : I-12, ADR-006 §3.2 et ADR-012 §4 renvoient au §5
d'ADR-005, que le fichier n'avait plus — S35 (`c2eb75ba`) avait écrit à sa place la note qu'il devait lui ajouter (−202 lignes),
S39 de même ; aucun autre ADR ni spécification atteint (audit `--numstat`). **Fait** ([preuve](../../docs/validation/NIVEAUX-S402.md)).
(1) ADR-005 restauré au bit (texte de S16, notes B-S26 et B-S27, note datée) ; un contrôle de l'outil — un ADR commence par son
titre — vu échouer sur les versions de S35 et S39 ; METHODE, L373. (2) `resample_from` : surface par recouvrement d'une
reconstruction bilinéaire conservative, vitesses interpolées et moyennées. **Tenus** : état uniforme au bit ; volume exact à
25 ↔ 50 cm ; aller-retour d'une onde de seize mailles **0,304 %** (manqué d'abord, 1,03 % : le terme croisé manquait) ; vu échouer
sans pente, 10,2 %. (3) Au banc, 25 → 50 cm à 2 s et retour à 5 s : **sauts de 0,1 à 2 mm** par les deux mécanismes ; sur une
bosse, le transfert reste à 1,8 mm du domaine fin, **ADR-005 §5 la perd (10,2 mm)** ; une source d'une maille grossière coûte 15 à
17 mm aux deux. **ADR-210** actée : le changement de niveau par transfert d'état. Suite **712 réussis**, zéro avertissement.
**Limites** : quand descendre (famine, contenu) non écrit ; rapport 2,5 non éprouvé au banc ; ni pas couplé ni carte.
**Rituel.** Maillons **0** : **correction d'intégrité** — **devient possible** de lire le cycle de vie qu'I-12 et trois ADR
invoquent, et un ADR réécrit se voit au contrôle ; **chemin** : C8c l'a consommé (ADR-210) ; **preuve** : le contrôle sur les
versions de S35 et S39, NIVEAUX-S402 §1. Suivant : dans le cloud, **C8d**, la décision du rang 4 et la famine ; au poste, C3b
puis la pluie 5b.

## S403 — 2026-09-27 — physique : C8d, le rang 4 dans l'ordonnanceur et l'issue de la famine

**Entrée.** *« Continue »* ; S402 proposait C8d. **Fait** ([preuve](../../docs/validation/FAMINE-S403.md)). Dans `scheduler.rs` : un
non-focal **déclare** (`declare_coarsen`) son coût un niveau plus bas et ce que son image y perdrait — l'écart d'un aller-retour
de son contenu (ADR-210 D2) ; quand le rang 1, à son minimum, affame encore, celui qui **perd le moins par milliseconde rendue**
descend, immédiatement ; remontée une par seconde, la plus forte perte d'abord ; le focal jamais ; ce qui reste sans budget est
**déclaré** (`starved`) — le rang 5, à l'hôte. `Bid` inchangé : l'hôte `viewer/` le construit et ne se compile pas ici. **Tenus** :
sans déclaration, S351 au bit (28 essais, empreinte S278 `6aebff024c734fc9`) ; six essais neufs ; au banc, trois domaines δ réels
(un focal, une source, une bosse) sous un budget en quatre phases — en famine, la bosse descend : **1,4 mm**, quand un témoin qui
ignore le contenu descend la source : **14,0 mm** ; sauts ≤ 2,5 mm ; budget jamais dépassé ; suite **718 réussis**, zéro
avertissement. **Trouvé** : en famine sévère, la victime du rang 5 suit l'ordre du sac à dos, pas le contenu, et une descente qui
n'y met pas fin change seulement qui est nourri ; un domaine détruit ne retrouve pas son contenu (20,5 mm). **Limites** : rangs 2, 3,
6, 7 et régulateur PI absents ; coûts déclarés par une loi ; ni carte ni hôte. **Rituel.** Maillons **1** : aucun point ne change de
case (9.8 et 9.9 restent partiels). Suivant : dans le cloud, **C8e** — l'épars et les niveaux sous le pas couplé (δ sous B + W) ;
le raccord, puis C6, sous la condition d'A316 ; au poste, C3b puis la pluie 5b.

## S404 — 2026-09-27 — physique : C8e, l'épars et les niveaux sous le pas couplé

**Entrée.** *« Continue »* ; S403 proposait C8e (file : « le pas couplé sous l'ensemble : avant C10 »). **Fait**
([preuve](../../docs/validation/MER-EPARS-S404.md)). Au pas couplé, le bord de la boîte fait plus que fermer δ : la bande de B le
traverse, l'éponge y absorbe. Le bord de l'ensemble épars fait désormais de même — faces fermées non prédites, bande aux murs
comme au bord, éponge mesurée dans l'étendue de chaque colonne : il devient **absorbant** ; en mode relatif seulement (ADR-198 D1),
le pas de S297 le refuse. Le transfert de niveau porte l'ensemble (dehors au repos ; `Follow::require_cover`). **Tenus** : plein au
bit ; sous une houle réelle, un rectangle à **un ulp** de son dense (Jacobi et multigrille), deux rectangles de même, bilan au
plancher ; vu échouer quatre fois (0,26 à 32 mm) ; un rectangle 25 → 50 → 25 cm **au bit** de son dense, volume exact, perte
publiée ; au banc en mer, l'épars qui suit à **0,14 mm** du domaine entier (témoin aux murs réfléchissants 0,21), **0,43 mm** sur
30 s, et à 0,14 mm de l'entier à travers un changement de niveau (sauts ≤ 2,04 mm) ; suite **726 réussis**, zéro avertissement.
**Trouvé** : une colonne dehors porte le repos, pas rien (sinon −79,9 m³). **Manqué** : l'ensemble ne se vide pas en 10 s (0,81 des
mailles ; enveloppe et dilatation couvrent la moitié de cette fenêtre, calculé), seulement en 30 s (0,55). **Limites** : B seul,
houle douce (A320 intouchée), le bord absorbant efface (4.8), 25 ↔ 50 cm seulement, ni corps ni carte. **Rituel.** Maillons **2** :
aucun point ne change de case (4.3, 4.5, 4.7, 9.2 restent partiels). Suivant : dans le cloud, un lot qui fait changer de case un
point — proposé **9.3**, la prédiction balistique consommée par le suivi (le saut de C10), à comparer au raccord C5 (S397, A316) ;
au poste, C3b puis la pluie 5b.

## S405 — 2026-09-27 — physique : 9.3, la prédiction balistique consommée par le domaine épars

**Entrée.** *« continue »* ; S404, à deux maillons, proposait un lot qui fait changer de case un point : **9.3** (front 0, débloque
9.4 et 9.6, sert le saut de C10), préféré au raccord C5, où aucun changement de case n'est en vue. **Fait**
([preuve](../../docs/validation/IMPACT-PREVU-S405.md)). `ballistic` (cœur) : gravité et traînée quadratique, rotation libre (Euler),
contact de la sphère englobante avec la houle telle qu'elle sera, instant affiné à 10⁻¹² s ; région utile sous une traînée bornée ;
paliers d'ADR-013 §2. **Tenus** : parabole à 5·10⁻¹³ s ; traînée d'ordre 4 (rapports 14,4 et 15,6) ; toupie symétrique à 10⁻¹³ ;
houle rencontrée où elle est (l'ignorer : +2,1 ms, +2,2 cm) ; ±30 % de traînée dans la région. Au banc en mer, une sphère lancée à
12 m/s de 6 m, l'ensemble revu toutes les 0,5 s : avec la prédiction, la région d'impact prête **1,11 s** avant, la source jamais
dehors, 0,59 mm du domaine entier, à toutes les phases de revue ; le témoin (l'objet suivi là où il est) prête 0,01 à 0,41 s avant
et laisse la source **dehors une fois sur cinq** — la prédiction « témoin refusé » manquée à la phase prévue ; prix : 10 à 15 % de
mailles. Suite **732 réussis**, zéro avertissement. **Limites** : sphère englobante, ni vent ni portance, entrée par volume
déplacé, orientation non consommée, région calculée et non réservée. **Rituel.** Maillons **0** — **devient possible** : préparer
le domaine d'eau pendant le vol d'un objet, quelle que soit la cadence de revue ; **chemin** : le domaine épars en mer (`Follow`,
pas couplé) le consomme ; **preuve** : IMPACT-PREVU-S405 §3 ; **9.3 absent → partiel**. Suivant : dans le cloud, le raccord C5
(A316), priorité de l'utilisateur (S397) ; sinon 9.6, le précalcul avant l'impact ; au poste, C3b puis la pluie 5b.

## S406 — 2026-09-27 — physique : C5c, le courant de surface au raccord levé

**Entrée.** *« Continue, je confirmes »* — décision de l'utilisateur, à la file : la priorité du solveur passe avant la règle des
maillons (S405 avait quitté la campagne pour 9.3) ; suite : le raccord C5, A316. **Fait**
([preuve](../../docs/validation/RACCORD-3D-S398.md) §7). Les trois suspects de S400 en options d'essai de `Apic3`, éprouvés par des
témoins courts (5 cm, 30 s, 1 min) : **le courant est porté par la face de frontière**, dont la vitesse venait du seul transfert
des particules de la bande — prise pour moitié à la zone, −6,7 → −3,6 mm/s ; prise à la zone, −0,4 (APIC seul −0,5). La prédiction
écrite avant désignait le débit de la rangée du haut : **manquée**, il aggrave. Défaut : la face appartient à la zone, la quantité
de mouvement absorbée est rendue (la migration revient au critère) ; `TRIAL_S400` rend S400 au chiffre près. **Tenus**, critère 4
de S399 aux deux mailles : niveau, saut, période, amortissement, **courant ≤ 0,6 et ≤ 1,1 mm/s** ; suite **733 réussis**, zéro
avertissement. **Manqué** : la **densité** de la dernière colonne de la bande, 7,59 à 5 cm, 7,24–7,31 à 2,5 cm (8 ± 0,4) — ni la
face, ni le débit, ni la quantité de mouvement, ni le lieu du retrait. **Rituel.** Maillons **1** : aucun point ne change de case
(4.16 partiel). Suivant : dans le cloud, **C5d, la densité au raccord** — condition d'une cinquième session sur A316 : un témoin
court (5 cm) qui relève la densité à 7,6 au moins par un geste nommé d'avance (la pose contre la face à la vitesse de la grille ;
la séparation près des particules virtuelles) ; sinon C6 sur la frontière telle qu'elle est ; au poste, C3b puis C7.

## S407 — 2026-09-27 — physique : C5d, la densité au raccord — le critère du raccord tenu aux deux mailles

**Entrée.** *« Continue »* ; S406 proposait C5d, sous condition : un témoin court qui relève la densité à 7,6 par un geste nommé
d'avance. **Fait** ([preuve](../../docs/validation/RACCORD-3D-S398.md) §8). **D'abord le diagnostic**, sans rien changer au calcul
(compteurs de l'échange, densité des quatre dernières colonnes ; S406 au chiffre près) : **la densité n'est pas perdue, elle est
déplacée** — la dernière colonne à 7,6 et 7,3, l'avant-dernière à 8,5 et 8,6 — et **aucune particule ne traverse la face** : tout
passe par des retraits (la plus proche de la face, celle qui allait traverser) et des poses. **Le geste, nommé avant le calcul** :
poser **à la face**, au centre de la tranche entrée (`dx/16`), et non à `dx/4` — un quart de maille trop loin, que l'aller-retour du
nœud amassait à côté. **Tenus** : densité 7,79 / 7,85 / 8,00 à 5 cm, 7,78 / 7,83 / 7,83 à 2,5 cm ; les particules traversent de
nouveau (13 000 absorptions par tranche, presque aucun retrait) ; migration sous le demi-millimètre ; **tout le critère de S399 tenu
aux deux mailles**, pour la première fois ; `APIC3D_ESSAI=16` et `=1` rendent S406 et S400 au chiffre près ; suite **734 réussis**,
zéro avertissement. **Limites** : une frontière droite, fixe, au nœud ; au critère de la campagne, le saut max vaut 3,35 mm à
2,5 cm pour 3 (APIC seul : 19,6 mm entre voisines). **Rituel.** Maillons **2** : aucun point ne change de case (4.16 partiel) ; la
campagne continue par décision de l'utilisateur (S406). Suivant : dans le cloud, **C6**, le critère de bascule — la frontière qui
bouge, où A316 se rejoue ; au poste, C3b puis C7.

## S408 — 2026-09-27 — physique : C6a, la frontière qui bouge — la bascule colonnes ↔ particules en 3D et son critère

**Entrée.** *« continue »* ; S407 proposait C6, le critère de bascule. **Fait** ([preuve](../../docs/validation/BASCULE-S408.md)) :
`set_columns_mask` bascule une colonne d'`Apic3` aux particules (ensemencée sous `η`) et retour (si convertible : un seul segment,
mailles occupées d'un seul tenant, sans corps ; voie mixte de S323), à masse exacte, réserve et soldes, sans allocation ;
`ColumnsSwitch` requiert en particules le non convertible, l'empreinte du corps sur son horizon, la pente > 1 ; dilatation,
hystérésis. **B10** (`Fr` = 2, `D/dx` = 8) : pincement **un pas plus tôt** qu'APIC seul (1,4797 contre 1,5067 — à la limite du
critère, et dans les cinq réglages : c'est la zone, pas l'étendue de la bande), **22 %** des colonnes en particules, 29 000
particules contre 131 000, 19 s contre 36, volume 10⁻¹². **Ce que la mesure a corrigé** : deux pertes de masse **du pas de S399**
(absorption au cœur en `f32` sans reste ; débits `f32` contre soldes `f64` — 1,2·10⁻⁹ → 10⁻¹²), qui rompent à dessein le « au bit »
du critère 1 (raccord : critère de S399 tenu, saut à 2,5 cm 3,35 → 3,85 mm) ; la voie mixte bornée à un quart de maille (deux
colonnes comprimées décalées de 12,8 cm ; impasse : exiger la densité nominale, 82 % de bande). **Manqué** : la hauteur après dix
allers-retours (0,31 maille pour 0,2 ; 0,10 après deux). Suite **739 réussis**, zéro avertissement. **Limites** : un seul corps,
une maille ; le maintien par défaut plus long que B10 ; la vague qui déferle absente. **Rituel.** Maillons **0** : 4.10 passe à
partiel — devient possible une bande de particules qui suit la surface à masse exacte ; la consomment C6b puis les scènes de C10 ;
preuve BASCULE-S408. Suivant : dans le cloud, **C6b**, la vague qui déferle ; au poste, C3b puis C7.
**Clôture** (12:39) : *« je vais continuer sur mon pc »* — `main` et `poste` avancés en avance rapide jusqu'à cette tête ;
`claude/blissful-pasteur-m4j5rn` (S393–S400, entièrement contenue) : suppression refusée depuis le cloud (le commit `fecde440`
la disait faite), **faite depuis le poste** par l'utilisateur ; la suite au poste, C3b ou C6b.

## S409 — 2026-09-27 — physique : C3b, la maille de 10 cm sur la carte — et A298 refermée

**Entrée.** *« Reprends le projet »*, au poste ; suite désignée par S408 : C3b. **Fait** ([preuve](../../docs/validation/MULTIGRILLE-3D-S385.md)
§6) : « à 10 cm sur une scène de même surface » était impossible — 5,9 M mailles, au-delà d'une liaison de 128 Mio et ≈ 56 ms
par pas — ; lu comme la même mer et la même boîte sur l'emprise que le budget permet (`Config::at_mesh`, `MAILLE=`, `EMPRISE=`).
**Qualité** : la multigrille garde son taux par cycle (≈ 0,45) mais part cinq fois plus haut — **8 cycles** pour le résidu de
Jacobi-32 à 25 cm, quelle que soit l'emprise ; deux prédictions manquées, publiées. **Coût** : 0,53 ms + 7,1 ns par maille ;
projection ÷ 2,8 au moins à résidu égal ; **8 m × 8 m** sous 2 ms par part à 30 Hz. **Mais à 10 cm, 30 Hz explose à 62 s**,
quel que soit le solveur, sans paquet aussi (**A322**) : `u'·∇u'` ou le résidu du fond éteints, ou 25 ms, la font tenir ;
hypothèse, le Courant de l'advection. À 60 Hz, stable : **5,6 m × 5,6 m** en mg 6, 1,94 ms. **A298 refermée** : au pas
d'usage, 27 µm en deux minutes avec la multigrille contre 1,4 mm avec Jacobi-32 — la sous-convergence, pas un biais. **Trouvé
en plus** : la référence **gagne de l'énergie** au pas long dans la cuve fermée, +41 % en 2 min, sans ADR-209 aussi (**A323**).
Afficheur 38 réussis, zéro avertissement ; aucun défaut changé. **Limites** : un domaine, une mer, sans rendu concurrent ;
la cuve à 10 cm sur 10 s seulement (hiérarchie tronquée). **Rituel.** Maillons **1** : 4.19 reste partiel ; la priorité du
solveur passe avant la règle (S406). Suivant : C7 au poste ou C6b ; A322 avant toute scène à 10 cm (C10).

## S410 — 2026-09-27 — physique : C6b, la vague qui déferle — et R34, jugée à l'image

**Entrée.** *« Réalise C6b »*. **Fait** ([preuve](../../docs/validation/BASCULE-S408.md) §6) : le cas de Chen et al. (1999), une houle de
Stokes d'ordre 3, `ka` = 0,55, dans un bassin de quatre longueurs d'onde (`apic3d_deferlement`, vitesses posées par
`Apic3::set_particle_velocities`). **APIC 3D déferle** : retournement à **0,706 √(λ/g)** (Chen : 0,72), jet qui retombe à 1,27 (1,56).
**La bande de S408** naît au front des crêtes **sept pas avant le pli**, masse à 10⁻¹² ; mais au maintien de 0,5 s elle **traîne**
(68 % des colonnes, plus chère qu'APIC seul), au maintien court elle **hésite** (7 bascules) ; l'hystérésis de la pente, essayée
deux fois (la seconde : gardée sans dilater, `slope_release`, défaut au bit), est une impasse. **Le fait** : APIC seul perturbé
bouge de 0,05 %, la bande de 3 à 30 % selon le réglage — **chaque conversion au sommet de la crête perturbe le déferlement**.
**Demande de l'utilisateur en cours de session** : juger plutôt à l'image que poursuivre des écarts sous le visible — avis rendu
(oui ; les chiffres pour masse, coût, durée) ; **planche R34** (`APIC3D_IMAGES`) : au maintien court, le sommet de la crête
repasse en colonnes, une bosse lisse que les chiffres ne voyaient pas ; à 0,3 s, le déferlement ressemble à APIC seul. Suite
**741 réussis**, zéro avertissement. **Limites** : 40 mailles par longueur d'onde, crête uniforme ; la crête courte : la bande
atteint les bords après l'impact. **Rituel.** Maillons **2** (aucun état de la liste ne change ; la priorité du solveur passe
avant la règle, S406). Suivant : **R34**, puis C6c (une bande qui suit la crête) ou C7 au poste ; A322 avant C10.

## S411 — 2026-09-27 — conception : les trucages d'une eau de qualité cinéma, en temps réel

**Entrée.** Verdict **R34** : *« 2D et bille, encore loin du finale […] mais le maintien de 0.3s parait bien, la formation de la
vague est visible »* ; puis trois réflexions de l'utilisateur — ne pas simuler l'eau profonde en particules, une qualité digne
des logiciels spécialisés en temps réel par des trucages, des courants à niveaux de détail — et une précision : *« Houdini était
une référence pas forcément le choix adpaté »*. **Fait** : R34 consigné (maintien 0,3 s retenu pour C6c, défaut inchangé d'ici
là) ; [TRUCAGES-TEMPS-REEL-S411](../../docs/registres/TRUCAGES-TEMPS-REEL-S411.md) — la qualité d'image pour cible, pas la méthode ;
huit trucages rapportés aux couches et à l'état du dépôt. **Trouvé** : la conception de la campagne voulait déjà la bande **sous
la surface** (A1), l'implémentation l'a faite pleine hauteur — c'est le Narrow Band FLIP (Ferstl et al. 2016), proposé comme
**C6c** avant C7 (particules ÷ 4 à 10 sur nos bancs, estimé) ; les courants à niveaux de détail sont **conçus depuis S01**
(ADR-011, C0 à C3), jamais construits (2.6). **Non fait** : aucun code ; aucun ADR avant la réponse de l'utilisateur (§8, quatre
questions). **Rituel.** Maillons **3** — justifié : session de conception demandée par l'utilisateur, dont la réponse rend C6c
exécutable. Suivant : les réponses du §8, puis **C6c**, la bande étroite en profondeur.

## S412 — 2026-09-27 — conception : les trucages retenus, et la bande étroite en profondeur

**Entrée.** Réponses de l'utilisateur au §8 des trucages : *« 1. Je valides ton choix 2. Je valides ton choix 3. Ok 4. cela
dépends une simulation d'un joueur de 15m peut être calculé a l'avance et plus le joueur se rapproche de la simulation et peux
intérargir et simule en temps réel, a réfléchir »*. **Fait** : [ADR-211](../../docs/adr/ADR-211-les-trucages-retenus.md) — C6c = la
bande étroite en profondeur, avant C7 ; la surface continue avant C10 ; les courants (ADR-011) après la campagne ; au loin le
calcul d'avance, de près le vivant — première analyse : un niveau « cuit » entre le factice et δ, passage par transfert d'état
(ADR-210), I-17 par la graine (ADR-022), point de file. Lecture du code de la zone, puis
[ADR-212](../../docs/adr/ADR-212-la-bande-etroite-en-profondeur.md) : **une hauteur eulérienne `β` par colonne** sous les particules
— colonne (`β` = `η`), bande étroite, bande pleine (`β` = 0, S398–S410 au bit) ; sous `β` la machinerie de la zone ; la face à
`β`, frontière à solde vertical, masse au bit ; `β` à `k` mailles sous la surface la plus basse (prédiction `k` = 4, `h` = 2) ;
six critères ; C6c-1 puis C6c-2. **Non fait** : aucun code. **Rituel.** Maillons **0** : une décision de l'utilisateur lève le
choix de C6c et nomme le lot exécutable — **devient possible** C6c-1 (`β` fixe), **le chemin** C6c-2 puis C7 et les scènes,
**la preuve** ADR-211 et ADR-212. Suivant : **C6c-1**.

## S413 — 2026-09-27 — physique : C6c-1, la bande étroite en profondeur, le fond fixe

**Entrée.** *« Continue »* ; suite déclarée : C6c-1 (ADR-212 §4). **Fait** ([preuve](../../docs/validation/BANDE-ETROITE-S413.md)) : un
**fond** par colonne de la bande (`set_band_floor`, arrondi à une face de maille) sous lequel l'eau est **à la grille** — étiquettes
`φ = z − fond`, particules virtuelles jusqu'au fond, faces advectées maille par maille, la face du fond comprise ; la part
eulérienne est un **contenant plein** dont chaque débit charge un **solde vertical**, réglé par des particules posées ou retirées
au-dessus du fond ; absorption sous le fond ; la frontière latérale lue maille par maille. **Mesuré** : sans fond, tout au bit
(25 essais, B10, la vague, le raccord aux deux mailles) ; **repos** 8,7·10⁻⁶ m/s, volume 0, densité 8,000 ; **ballottement de
30 s** à 0,2 point d'APIC seul en période et amortissement aux deux mailles, **4,7 à 7,7 fois moins de particules**, calcul 2,4
fois plus court ; masse à 10⁻¹⁵. Suite **745 réussis**, zéro avertissement. **Écart à ADR-212** : le fond fixe sur une face, les
débits au solde vertical (note datée). **Limites** : le fond posé à la main ; la cuve toute en bande étroite à 2,5 cm, densité 7,14
au milieu en fin de calcul (APIC seul 7,62) — à surveiller. **Rituel.** Maillons **1** (4.16 reste partiel ; la priorité du solveur
passe avant la règle, S406). Suivant : **C6c-2**, le fond placé par le critère — B10 et la vague de Chen, jugée sur planche.

## S414 — 2026-09-27 — physique : C6c-2, le fond de la bande placé par le critère

**Entrée.** *« Tu peux commit tout, les pousses. Pour le fond automatique il serait intéréssant que les systèmes de prédictions
permettent de jouer sur la position du fond […] Mais sans prédictions comme tu le pensais cela me convient. »* — poussé
(`617ea1b4..a554f0cb`). **Fait** ([preuve](../../docs/validation/BANDE-ETROITE-S413.md) §5) : `move_band_floor` — descente par
ensemencement, remontée par absorption, l'écart au solde vertical ; bande → colonne avec l'eau sous le fond ; le critère place le
fond à `k` mailles sous la première maille non-eau, hystérésis, et **en option la prédiction du corps** (l'idée de l'utilisateur).
**Corrigé** : le volume sous le fond compté en `f32` perdait 7,4·10⁻⁹ par déplacement — désormais en mailles entières. **Mesuré** :
dix allers-retours du fond à 1,3·10⁻¹⁵ ; **B10** — la cavité d'APIC seul au chiffre près (1,937 D, 0,078 D³), pincement à un pas,
**4 122 particules** (÷ 7 contre la bande pleine, ÷ 32 contre APIC seul) ; la prédiction au **pas même** d'APIC seul à horizon
court (0,05 s), deux pas trop tôt à 0,2 s ; **la vague de Chen** ÷ 6 et deux fois plus vite, **R35 posée**. Suite **748 réussis**.
**L'utilisateur, ensuite** : *« le mesh du fond malaxable en fonction du courant, les particules peuvent naitres et disparaitre en
fonction de leurs vitesse »* — c'est l'Extended Narrow Band FLIP (Sato et al. 2018) : **C6c-3** proposée. **Limites** : aucun défaut
changé ; crête courte non rejouée. **Rituel.** Maillons **2** (4.16 reste partiel ; la priorité du solveur passe avant la règle,
S406). Suivant : R35, puis C6c-3 ou C7.

## S415 — 2026-09-27 — physique : C6c-3, le fond qui suit l'écoulement

**Entrée.** *« Je valides R35, continue avec ta recomandation »* — R35 reçu ; la recommandation : C6c-3, l'idée de l'utilisateur
(*« le mesh du fond malaxable en fonction du courant, les particules peuvent naitres et disparaitre en fonction de leurs vitesse »*).
**Fait** ([preuve](../../docs/validation/BANDE-ETROITE-S413.md) §6) : trois critères d'écoulement dans `ColumnsSwitch`, éteints par
défaut — vorticité de la grille, vitesse, part de rotation (critère Q sans dimension) — qui rendent une colonne requise et descendent
son fond sous l'eau concernée ; banc `apic3d_tourbillon` (Lamb–Oseen enfoui). **Mesuré** : la grille perd 2,5 fois l'énergie
qu'APIC perd ; la vorticité garde le cœur (à 1 %) mais pas l'énergie de l'écoulement extérieur, irrotationnel ; **la vitesse** — les
mots de l'utilisateur — rend l'énergie d'APIC seul (0,800 contre 0,806 à 2,5 cm) avec 2,4 à 3 fois moins de particules. **Mais** sous
la vague de Chen, vorticité absolue (la déformation fausse la vorticité de la grille) et vitesse prennent toute la houle ; la part de
rotation ne prend que le déferlement, et y hésite. **La voie** : la vitesse propre de δ, relative à B (ADR-198), gratuite sous la
houle — avec C7. Suite **752 réussis**, zéro avertissement. **Rituel.** Maillons **3** — justifié : la demande de l'utilisateur, et
la priorité du solveur (S406). Suivant : **C7** au poste, APIC sur la carte dans sa version étroite.

## S416 — 2026-09-30 — physique : C7a, APIC nu sur la carte

**Entrée.** *« Reprends le projet »* — jeton libre, branches synchronisées ; la suite déclarée : C7, APIC sur la carte. **Fait**
([preuve](../../docs/validation/APIC-CARTE-S416.md)) : C7 **découpé en cinq** — nu (C7a), corps (C7b), zone et fond (C7c), relatif à B
(C7d), budget (C7e) — chaque morceau reçu contre la référence avant le suivant ; transferts en **collecte** sur les particules
triées par maille (aucun atomique flottant ; chaque tranche triée par indice : l'ordre de la référence, un pas déterministe). Cœur :
`step_upto` (le pas arrêté à un étage ; `Full` au bit) et accesseurs de banc. Carte : `apic3d_carte.rs`/`.wgsl`, vingt-sept noyaux.
**Mesuré** : chaque étage à l'arrondi — tri identique, transfert 1,6·10⁻⁷ m/s, `φ` 1,6·10⁻⁶ m, **94 itérations du gradient
conjugué des deux côtés**, positions 1,2·10⁻⁷ m ; **le ballottement (1, 0), 10 s : surface à 0,46 mm** de la référence, période
+0,003 % ; à 2,5 cm, 0,77 mm. **Coût** p99 : 2,05 ms pour 12 800 particules, **4,77 ms pour 102 400 (47 ns)** ; deux tiers dans
la projection — les dispatchs **enregistrés**, même vides après convergence, non les calculs ; la reconstruction ensuite. Suite
**753 réussis**, zéro avertissement. **Pièges** : FXC refuse `v[a] = …` indexé dynamiquement ; le `!` d'un script en ligne casse le
shell de l'outil. **Limites** : ni zone, ni fond, ni corps ; `dt` choisi par la référence ; DX12 seul. **Non fait** : Gao *et al.*
(2018) non relu. **Rituel.** Maillons **4** — justifié : la priorité du solveur passe avant la règle (S406) ; 4.19 reste partiel.
Suivant : **C7b puis C7c** au poste — le corps, puis la zone et le fond ; B10 en bande étroite sur la carte, le critère de C7.

## S417 — 2026-10-01 — physique : C7b et C7c-1, le corps et la zone des colonnes sur la carte

**Entrée.** *« Continue »* — C7b puis C7c. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §7–9) : **C7b** — le corps
cinématique sur la carte (mailles solides, image radiale, faces imposées, particules repoussées) ; étages de B10 à l'arrondi (211
itérations des deux côtés) ; **B10 nu, pincement au pas 54 comme la référence, au chiffre près** (profondeur, air, cavité,
couronne). L'écart de `φ` à l'interface, lui, pointe à 9,2 mm au col juste avant le pincement : **le témoin** (la référence contre
elle-même, vitesses perturbées de 10⁻⁶ m/s) y fait 22,6 mm — `φ` est discontinu où le noyau ne voit presque plus de particule. Le
critère de 3 mm, manqué tel qu'écrit, est intenable ; C7c-4 se jugera contre le témoin, pas à pas. **C7c conçu en quatre**
(zone, échange, fond, bascule ; `n` résident). **C7c-1** — la zone sans échange : advection au pied de la caractéristique,
étiquettes, particules virtuelles, transport de `η` ; **les volumes en entiers** (quantum `dx³/8·2⁻²⁴`) — wgpu n'offre pas `f64`
sous DX12, et FXC ne garantit pas le `mad` fusionné d'un double flottant. **Mesuré** : cuve mixte à l'arrondi jusqu'à l'advection
(`η` 2,4·10⁻⁷ m) ; tout en colonnes, 10 s, **surface à 0,003 mm, volume constant exactement**. Coût B10 : 6,42 ms, 49 ns par
particule, la projection d'abord. Suite **753**, zéro avertissement. **Limites** : l'échange (C7c-2) non porté ; `dt` choisi par la
référence. **Piège** : un `sed` sur `EN-COURS` l'a abîmé en entier — restauré depuis le commit ; les notes s'écrivent par fichier.
**Rituel.** Maillons **5** — justifié : la priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7c-2**, l'échange sur la carte.

## S418 — 2026-10-01 — physique : C7c-2, l'échange à la frontière sur la carte

**Entrée.** *« Continue »* — C7c-2. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §10) : `n` résident, soldes en quanta
entiers chargés par le transport, compactage stable (exact), séparation tenue côté bande. **Décision** : l'échange de la référence
dépend de l'ordre (le mélange aux faces : jusqu'à 1/64 de l'écart des vitesses) — la carte garde ses tableaux **indice pour
indice** : absorbées listées en parallèle, traitées sur un fil dans l'ordre de visite de la référence (reconstruit à deux pointeurs),
retraits et poses sur un fil, suppression par échange avec la dernière. **Mesuré** : un pas entier, cuve mixte, 2 à 90 pas de
chauffe — gestes et `n` identiques, positions à 6·10⁻⁸ m indice pour indice. **Le raccord, 30 s** : volume **constant à 0
quantum** ; période −0,036 % ; surface à **4,41 mm** — critère de 3 mm manqué ; **les témoins** (la référence contre elle-même,
±10⁻⁶ et ±10⁻⁴ m/s) : 3,40 et 4,03 mm, même courbe ; l'ordre diverge au premier retrait (des particules à égale distance de la
face, l'arrondi tranche). Reçu **à l'échelle du témoin**. B10, colonnes inchangés ; suite **753**, zéro avertissement. **Limites** :
deux fils ≈ 0,7 ms (coloriage en C7e) ; un troisième témoin trancherait l'écart de 10 %. **Rituel.** Maillons **6** — justifié : la
priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7c-3**, le fond sur la carte.

## S419 — 2026-10-01 — physique : C7c-3, le fond de la bande sur la carte

**Entrée.** *« Continue »* — C7c-3. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §11) : le fond par colonne sur la carte
— `φ = z − fond` et l'eau dessous, virtuelles jusqu'au fond, faces à la grille advectées ; le transport charge les soldes verticaux
(chaque face garde ses deux contributions, un noyau par colonne les rassemble, sans atomique) ; l'échange absorbe sous le fond, lit
la frontière maille par maille, règle le solde vertical. Le déplacement du fond part avec la bascule (C7c-4). **Mesuré** : étages à
l'arrondi (soldes verticaux 8,2·10⁻¹⁰ m³, sous la borne de la vitesse admise) ; un pas entier : gestes et `n` identiques, positions
1,2·10⁻⁷ m — sauf, après 20 pas, 3 poses sur 32 au miroir en `y` : le cas est invariant en `y`, les emplacements miroirs à distances
quasi égales (une tolérance aggrave ; l'évaluation sans `mad`, `square_sum`, gardée). **La bande étroite, 30 s : surface à 1,45 mm**,
période −0,015 %, volume exact à 0 quantum ; témoins 0,92 et 1,18 mm. **C7c-3 reçu.** Ballottement, B10, colonnes identiques ; le
raccord bouge dans sa dispersion (4,48 mm) ; suite **753**, zéro avertissement. **Rituel.** Maillons **7** — justifié : la priorité
du solveur (S406) ; 4.19 reste partiel. Suivant : **C7c-4**, la bascule et le déplacement du fond — B10 en bande étroite, le critère de C7.

## S420 — 2026-10-01 — physique : C7c-4, la bascule sur la carte ; B10 en bande étroite

**Entrée.** *« Continue »* — C7c-4. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §12) : la décision de la bascule par
colonne en parallèle ; la bascule sur un fil dans l'ordre de la référence — voie mixte en quanta, ensemencement, réserve exacte ; le
fond placé et déplacé ; la liste des retirées construite triée en parallèle (le tri sur un fil faisait tomber la carte à la bascule
initiale, 120 000 particules). **Mesuré** : décisions identiques à 21 instants ; bascules forcées (186 colonnes ensemencées, 3
converties, la bascule initiale de 200) aux positions de la référence indice pour indice, volume à 0 quantum ; `η` 1,9·10⁻⁶ m à la
bascule initiale (critère de 10⁻⁶ plus serré que `φ`, sa source). **B10 en bande étroite : pincement identique au chiffre près**
(pas 55, cavité 1,937 D, air 0,0781 D³), volume exact ; `φ` au col 50 mm, **dans l'enveloppe de trois témoins** (2,5 / 50 / 197 mm),
manqué contre le seul témoin à 10⁻⁶ nommé. Premier écart : une pose au pas 35 (un solde au seuil). **Coût : 28,7 ms** (l'échange et
la bascule sur un fil). Non-régression complète ; suite **753**, zéro avertissement. **Rituel.** Maillons **8** — justifié : la
priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7e**, le coût — δ ≤ 2 ms avec la bande.

## S421 — 2026-10-01 — physique : C7e, premier temps — le coût de la bande étroite sur la carte

**Entrée.** *« Continue »* — après le signal d'un compteur de maillons élevé (S420), lu comme la confirmation de la priorité du
solveur. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §13) : l'instrument d'abord (horodatage par sous-étage, fils
séparés de leur préparation) ; puis, à sémantique exacte : la réserve réglée en parallèle, la **liste ordonnée des faces-mailles
actives**, la pose en un parcours (elle relisait quatre fois toutes les particules posées), l'**absorption en groupe** (24 faces
distinctes par absorbée), l'**échange en groupe de 64 fils** (minimums de groupe sur la clé de départage de la référence), le **plafond
d'itérations adaptatif** (le dispatch indirect nul est refusé par wgpu). **Mesuré** : B10 en bande étroite, **29,0 → 4,9 ms** par pas
(+ 1,9 de bascule) ; pincement, gestes, `n`, volume, pas entiers du raccord et de la bande identiques. **Les bits bougent sans que la
sémantique change** : un appel sans effet remis dans un noyau rendait les bits d'avant — FXC compile le flottant d'un noyau selon son
code (L345) ; « au bit près » n'est pas un instrument tenable ici. Suite **753**, zéro avertissement. **Limites** : δ ≤ 2 ms non atteint
— restent la projection (2,5–3 ms, la multigrille), la bascule (1,9), la surface (0,7). **Rituel.** Maillons **9** — justifié : la
priorité du solveur (S406), confirmée par « Continue » après S420 ; 4.19 reste partiel. Suivant : **C7e**, la multigrille.

## S422 — 2026-10-01 — physique : C7e, la multigrille de la projection d'APIC sur la carte

**Entrée.** *« Continue »* — C7e. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §14) : la recette de C1/C3a transposée à
APIC — gradient conjugué préconditionné par un cycle en V (Jacobi 6/7, 2 + 2 lissages, 8 au plus grossier ; restriction par la
moyenne, prolongation par injection), le niveau fin exact (fluide fantôme, solide), les grossiers rediscrétisés (active si une fille
est d'eau, d'air sinon, solide si toutes le sont), la hiérarchie arrêtée à 64 mailles ; **les niveaux ≥ 2 dans un seul groupe**.
**FXC** : il refuse plusieurs barrières dans une boucle de bornes non constantes (et après un `continue` qui dépend du fil) — bornes
constantes, travail gardé. **Mesuré** : cycle symétrique (≤ 1,9·10⁻⁷) et positif ; 9 à 14 itérations au lieu de 94 et 207, vitesses à
≤ 5,2·10⁻⁶ m/s de la référence ; issues : ballottement 0,055 mm (0,456), raccord 4,18 mm (4,48 ; témoins 3,40–4,03), bande 1,71 mm,
B10 nu et en bande étroite au pincement de la référence, volumes exacts. **Coût** : B10 en bande étroite 3,9 ms par pas (+ 1,9 de
bascule) ; projection 1,45 ms — dix-sept dispatchs par itération, l'objectif d'1 ms manqué. Suite **753**, zéro avertissement.
**Limites** : la multigrille est une option (`MULTIGRILLE=1`) ; le défaut reste le gradient diagonal. **Rituel.** Maillons **10** —
justifié : la priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7e** — la bascule en groupe, la surface, les dispatchs.

## S423 — 2026-10-01 — physique : C7e, la bascule en groupe, la surface en coopération, la multigrille par défaut

**Entrée.** *« Continue »* — C7e. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §15) : la bascule et le fond en groupe
de 256 fils ; **le retrait à forme close** — la visite de la référence (échange avec la dernière) laisse un arrangement connu
d'avance, chaque gardée trouve sa place par dichotomie dans la liste triée des retirées ; **l'ensemencement par graine** (préfixe des
morceaux de colonnes, dichotomie) ; la surface en coopération (32 fils par maille) et son réemploi local (exact, sans gain sur B10) ;
la multigrille : le nombre de niveaux en constante de pipeline (le niveau 1 dans le groupe, essayé, plus lent, retiré) ; **la
multigrille et le plafond adaptatif par défaut**. **Mesuré**, B10 en bande étroite au p99 : **pas 3,26 ms + bascule 0,48** (3,90 +
1,9 en S422) ; surface 0,73 → 0,11 ; projection 1,35 (visé 1 : manqué). **Issues** : bascules forcées identiques à la référence,
B10 nu et en bande étroite au pincement de la référence, raccord 4,02 mm, bande 1,32, colonnes 0,002, volumes exacts. **Le
ballottement passe de 0,055 à 0,447 mm** : l'ordre des sommes de la surface coopérative (1,6·10⁻⁶ m) suffit à faire tomber sur
l'événement que la diagonale donnait déjà ; **témoin du ballottement** ajouté : 0,036 · 0,336 · 0,439 mm à ε = 10⁻⁶ · 10⁻⁵ · 10⁻⁴ —
0,055 était l'ordre de la référence, pas une précision. Suite **753**, zéro avertissement. **Rituel.** Maillons **11** — justifié :
la priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7e** — la projection (fusionner les noyaux du cycle), les fils de
l'échange et de l'absorption, la séparation : δ ≤ 2 ms (3,74 aujourd'hui).

## S424 — 2026-10-01 — physique : C7e, la projection sous la milliseconde

**Entrée.** *« Continue »* — C7e. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §16) : les niveaux ≥ 2 de la
multigrille en mémoire de groupe ; les noyaux fusionnés (`α` dans la mise à jour, `β` dans la direction, `r·z` en double tampon par
la parité de l'itération) — dix-sept dispatchs → douze pour 4 µs seulement : le nombre de dispatchs n'était pas le coût ; **un profil
par noyau** (`PROFIL=1`) : le groupe des niveaux grossiers 34 µs, dont 13 pour la restriction vers le niveau 2 faite par un seul
groupe ; **sortis du groupe** en dispatchs parallèles, à expression identique, `A·z` fin, `L₁·x₁`, les deux restrictions, la
prolongation vers le niveau 1. **Incident** : la pipeline en mémoire de groupe mettait 283 s à se créer — FXC déroule la mise à zéro de
la mémoire de groupe que wgpu ajoute ; coupée partout (chaque noyau écrit avant de lire) : **création des pipelines 80 → 27 s**.
**Mesuré** : itération 69 → 51 µs ; B10 en bande étroite, **projection 1,35 → 0,885 ms au p99** (visé 1 : tenu), pas 2,77 ms, pas +
bascule 3,25. **Issues identiques à S423 au chiffre près** (étages, cycle, bascules forcées, B10 nu et en bande, ballottement,
raccord, bande, gestes). Suite **753**, zéro avertissement. **Rituel.** Maillons **12** — justifié : la priorité du solveur (S406) ;
4.19 reste partiel. Suivant : **C7e** — les fils de l'échange et de l'absorption (0,62 + 0,40), la séparation (0,41) : δ ≤ 2 ms.

## S425 — 2026-10-01 — physique : C7e, la fin du pas profilée

**Entrée.** *« Continue »* — C7e. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §17) : le profil de la fin du pas
(`PROFIL=1` : chaque préfixe de la suite répété, différences — un noyau du tri répété seul a fait perdre la carte au premier essai),
les gestes par pas et le coût des fils par geste (2 µs), l'occupation des mailles (30 à 50 particules dans quelques-unes). **Le profil
a désigné autre chose que les fils** : le préfixe des listes ordonnées sur un fil (80 µs, quatre fois par pas) et celui du tri
(13,5) — **en groupe**, 2,2 et 1,2 ; le tri par insertion des tranches (64) — **par rang**, 4,7 ; la séparation (102) — **élaguée**
aux mailles voisines à moins de `dmin`, 50 ; le solde vertical du fil de l'échange qui visitait les 256 colonnes (66 µs fixes) —
**les seules colonnes dues**, 30. Tout à résultat identique (entiers ; mêmes sommes dans le même ordre ; termes nuls omis). P4b
ajouté au plan en cours de session, déclaré dans les notes. **Mesuré**, B10 en bande étroite au p99 : **pas 2,75 → 2,21 ms, bascule
0,48 → 0,23 ; pas + bascule 2,45** (visé 2,5 : tenu). **Issues identiques à S424 au chiffre près** (étages, cycle, bascules forcées,
B10 nu et en bande, ballottement, raccord, bande, gestes). Suite **753**, zéro avertissement. **Rituel.** Maillons **13** —
justifié : la priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7e** — les fils par vagues à faces disjointes (0,55 +
0,40 au p99), la projection (0,885) : δ ≤ 2 ms.

## S426 — 2026-10-01 — physique : C7e, l'absorption face par face

**Entrée.** *« Continue »* — C7e. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §18) : l'absorption sans fil séquentiel
— l'ordre de la visite calculé d'avance, sans geste ; des vagues d'absorbées à faces disjointes essayées d'abord (45 vagues pour
≈ 100 absorbées : pas de gain) ; **un fil par face touchée**, sa propriétaire y appliquant les mélanges dans l'ordre de la visite (un
premier essai cinq fois plus lent, puis le test de nœud rendu immédiat par un masque de 24 bits) ; soldes par cible, retrait à forme
close. L'échange : les marquées triées par rang. **Mesuré**, B10 en bande étroite au p99 : fil de l'absorption 0,40 → 0,29, de
l'échange 0,55 → 0,49 ; **pas + bascule 2,45 → 2,31 ms** (visé 2 : manqué). **Issues identiques sauf la bande sur 30 s** (1,210 mm
au lieu de 1,318) — **isolé au bit** (`AW_REPLI`, `DUMP`) : une unité du dernier chiffre sur 8 des 41 120 valeurs, la même formule
arrondie autrement par FXC dans un autre noyau (L345), amplifiée par 30 s chaotiques. Suite **753**, zéro avertissement.
**Rituel.** Maillons **14** — justifié : la priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7e** — les poses de
l'échange, la projection : δ ≤ 2 ms (2,31).

## S427 — 2026-10-01 — physique : C7e, les gestes de l'échange groupés, une course trouvée

**Entrée.** *« Continue »* — C7e. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §19) : les poses d'un solde d'un coup
(une réduction, puis les choix du fil 0 par minimums exacts, les vitesses en parallèle) et les retraits d'un coup (les K plus petites
clés, rangées par rang) — au bit. **Une course trouvée** : B10 a pincé au pas 54 ; le noyau n'était plus déterministe (`DUMP_B10`,
nouveau : l'état après chaque pas) ; isolée par moitiés jusqu'à `workgroupUniformLoad` sur un élément de tableau de groupe, dans la
boucle des colonnes dues de S425 — déterministe jusque-là par chance de cadence ; remplacé par la diffusion du fil 0 : trois
exécutions identiques au bit. **Mesuré**, B10 en bande étroite au p99 : fil de l'échange 0,49 → 0,38 ; **pas 1,93 ms, pas + bascule
2,16** (visé 2 : manqué de 0,16). **Issues identiques à S426** (étages, bascules forcées, B10 nu et en bande, ballottement, raccord,
bande, gestes) ; contre le binaire précédent, une vitesse de posée d'une unité du dernier chiffre (FXC, L345). Suite **753**, zéro
avertissement. **Rituel.** Maillons **15** — justifié : la priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7e** — la
projection (0,886), la part fixe de l'échange (53 µs) : δ ≤ 2 ms.

## S428 — 2026-10-01 — physique : C7e, la projection resserrée, au bit

**Entrée.** *« Continue »* — C7e. **Écart à la procédure, déclaré** : un essai écrit et mesuré avant le plan — le départ chaud de la
projection (13,9 → 13,4 itérations, p99 inchangé ; plus proche de la référence, pas plus rapide) — retiré, consigné dans le plan et la
preuve. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §20), à arithmétique identique : le groupe des niveaux grossiers
sans ses barrières à vide (18 → 17 µs : les barrières coûtent peu), puis à **512 fils** (13,8 µs) ; **les restrictions en
coopération** (huit fils par maille grossière, la somme dans l'ordre des filles) — deux dispatchs de moins par itération ; la réduction
des poses sautée sans pose. Un essai retiré (l'échange avec la dernière calculé d'avance : aucun gain). **Mesuré** : itération 51 →
45 µs, projection p99 0,886 → 0,785 ms ; **pas + bascule 2,16 → 2,07 ms** (visé 2 : manqué de 0,07). **Issues identiques à S427, B10
au bit sur 74 pas, déterministe.** Suite **753**, zéro avertissement. **Rituel.** Maillons **16** — justifié : la priorité du solveur
(S406) ; 4.19 reste partiel. Suivant : **C7e** — les dispatchs indirects taillés sur `n` (les noyaux lancés sur la capacité).

## S429 — 2026-10-02 — physique : C7e reçu ; C7d conçue, C7d-1 en partie

**Entrée.** *« On accepte ce petit surplus au critère, continue »* — **C7e reçu** à 2,07 ms au p99 par décision de l'utilisateur
(mesuré sur B10 en bande étroite, non sur la scène de la porte B). **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §21) :
**la conception de C7d** — la production GPU n'a pas le mode relatif d'ADR-198, A320 est ouverte, la bande simule l'eau totale ;
**C7d-1** le critère relatif en référence, **C7d-2** sur la carte, **C7d-3** la bande dans la production couplée après sa propre
conception. **C7d-1** : `LinearSwell` et `ColumnsSwitch::background` — le seuil de vitesse du fond sur `|u − U_B|` ; un essai (la
houle ne demande rien, le jet enfoui est pris). **Mesuré** sur la vague de Chen, B le premier ordre qui l'initialise : **sous une
houle calme, rien** (0/200 dans la fenêtre, quand la vitesse absolue prenait tout en oscillant) ; **sous la houle raide, 0,94 de la
fenêtre** (critère : sous 0,5 — manqué), à 0,58 aux seuils 0,4 et 0,6 avec des oscillations : B linéaire n'a pas les harmoniques que
δ porte (ADR-198 D3), la vitesse propre est grande sans déformation. Suite **754**, zéro avertissement. **Écarts** : trois battements
mal recopiés dans des messages de commit locaux, corrigés. **Rituel.** Maillons **17** — justifié : la priorité du solveur (S406) ;
4.19 reste partiel. Suivant : **C7d-1** — la déformation propre de δ (le gradient de `u − U_B`), mêmes critères.

## S430 — 2026-10-02 — physique : C7d-1, la déformation propre de δ, et un critère mal posé

**Entrée.** *« Continue »* — C7d-1. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §21.2) : `Apic3::deformation` (le
gradient de `u − U_B`, le gradient discret de B retranché) et `floor_deformation` ; un essai (la houle ne se déforme pas relativement
à B, le cisaillement enfoui est pris). **Mesuré** sur la vague de Chen : pire que la vitesse propre — toute la fenêtre sous la houle
raide, et la houle calme prise à 1 s⁻¹ (187 retours rapides) : le gradient de la grille est bruité près de la surface. Écartée.
**Trouvé en relisant** : le critère (a) de C7d-1 (S429) comparait la part au retournement à une part *moyenne dans le temps* ; la
forme seule prend déjà 0,62 de la fenêtre au retournement — (a) était inatteignable. Non changé en cours de mesure ; **réécrit pour la
suite** : le critère d'écoulement n'ajoute pas plus de 0,1 à la forme seule, ni de retour rapide de plus. Sous cette lecture, la
vitesse propre à 0,4 m/s tiendrait (0,58), sauf ses 25 retours rapides. Suite **755**, zéro avertissement. **Rituel.** Maillons
**18** — justifié : la priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7d-1** — l'hystérésis du seuil de vitesse propre.

## S431 — 2026-10-02 — physique : C7d-1 reçu, la vitesse propre de δ avec relâche

**Entrée.** *« Continue »* — C7d-1. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §21.3) : `floor_speed_release`
— une colonne de la bande au-delà de la relâche, sous le seuil, est gardée sans dilatation, comme la pente (S410) ; un essai (une
erreur de construction de l'essai corrigée : la zone est posée toute en bande). **Mesuré** sur la vague de Chen avec fond B, contre le
critère réécrit en S430 : 0,4 / 0,2 et 0,4 / 0,3 oscillent encore (11 et 18 retours rapides) ; **0,3 / 0,15 tient** — 0,64 de la
fenêtre au retournement (forme seule 0,62), aucun retour rapide, le retournement au même pas, rien sous une houle calme. **C7d-1
reçu.** Publié : après le déferlement, la bande garde l'eau agitée (six fois les particules de la forme seule). Suite **756**, zéro
avertissement. **Rituel.** Maillons **19** — justifié : la priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7d-2** —
le seuil de vitesse propre, sa relâche et le fond B sur la carte.

## S432 — 2026-10-02 — physique : C7d-2 reçu, la vitesse propre de δ sur la carte

**Entrée.** *« continue »* — C7d-2. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §21.4) : le seuil de vitesse
propre, sa relâche et le fond B dans la décision de la bascule sur la carte — paramètres à 256 octets, `switch_flow` entre la pente et
la dilatation, `floor_place` sous les mailles rapides ; `SwitchSettings` refuse les critères d'écoulement non portés. **Mesuré** : le
banc de décision **identique à la référence** dans les quatre séries, avec et sans les clés (le critère y travaille : jusqu'à 44 070
particules contre 8 686) ; B10 en bande étroite sans les clés **identique au bit à S428**, 2,06 ms (C7e tenu) ; avec les clés, la carte
suit la référence au chiffre près (pincement au pas 53 des deux côtés). Non-régression identique ; suite **756**, zéro avertissement.
**Écart** : P2 et P3 en un seul commit. **Rituel.** Maillons **20** — justifié : la priorité du solveur (S406) ; 4.19 reste partiel.
Suivant : **la conception de C7d-3** — la bande dans la production couplée ; le mode relatif sur la carte (ADR-198 D1) et A320.

## S433 — 2026-10-02 — physique : la conception de C7d-3

**Entrée.** *« Continue »* — la conception de C7d-3. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §22) : **la bande
entre dans le pas couplé** (`Volume3`, qui porte la production : couplage, éponge, épars, niveaux, faces coupées, production GPU) — non
B dans `Apic3`, qui reste le banc ; **les particules portent la vitesse propre `u′`** et se déplacent avec `U + u′` — le long d'une
particule ne reste que `u′·∇U`, le terme d'A320 ; l'identité `U·∇u′ + u′·∇U = ∇(U·u′) − U×ω′` (B irrotationnel) rend la forme de
Bernoulli exacte pour tout δ. Découpage : **C7d-3a** A320 en référence, **C7d-3b** le mode relatif sur la carte et la bascule des
défauts, **C7d-3c** la bande relative en référence, **C7d-3d** sur la carte — chacun son « reçu si ». **Mesuré** : le témoin d'A320
rejoué au chiffre près (0,1151 s⁻¹, 32,3 mm à 59 s). Aucun code changé. **Rituel.** Maillons **21** — justifié : la priorité du solveur
(S406) ; 4.19 reste partiel. Suivant : **C7d-3a** — A320, la forme `∇(U·u′) − U×ω′`, sans carte.

## S434 — 2026-10-02 — physique : C7d-3a, la forme de Bernoulli éprouvée — non reçu

**Entrée.** *« Continue »* — C7d-3a, A320. **Fait** ([preuve](../../docs/validation/MER-S369.md) §6) : `set_cross_bernoulli` — les
termes croisés du pas couplé relatif sous la forme `∂_a(U·u′) + Σ_b U_b (∂_b u′_a − ∂_a u′_b)` (G gradient discret exact, R
rotationnelle) ; δ nul reste nul au bit (un essai). **Mesuré** : la forme ne freine A320 que de 15 à 20 % (0,115 → 0,097 s⁻¹ sous
7,5 cm ; 0,050 sous 5 cm ; le paquet à 6,4 fois son amplitude) ; **le terme d'ADR-209**, actif en production mais absent du banc,
**n'y fait rien** — A320 n'est pas l'instabilité FTCS d'A321 ; **la bisection** : G seule et R seule ne croissent pas, leur somme oui —
**l'hypothèse de S369 est réfutée**. C7d-3a **non reçu** ; A320 annotée. Un calcul à 12,5 cm arrêté (trop long, sans témoin).
Suite **757**, zéro avertissement. **Rituel.** Maillons **22** — justifié : la priorité du solveur (S406) ; 4.19 reste partiel.
Suivant : **C7d-3a** — l'advection antisymétrique de `u′` par `U`, la condition de surface, la question physique.

## S435 — 2026-10-02 — physique : A320, la question physique ; A324

**Entrée.** *« Continue »* — C7d-3a, A320. **Fait** ([preuve](../../docs/validation/MER-S369.md) §7) : `MER_SPECTRE` (le spectre de δ,
la bande de Benjamin-Feir, la part sous `4·dx`, tracés chaque seconde), `MER_PROLONGEMENT`. **Mesuré**, houle de 7,5 cm, germe de
1 mm : δ croît **à la longueur d'onde de la houle** (99 % de son énergie dans la bande, rien sous `4·dx`) ; taux 0,093 · 0,112 · 0,106 ·
0,084 s⁻¹ à 50 · 31,25 · 25 · 15,6 cm ; 0,056 extrapolé à maille nulle, **2,05 fois** Benjamin-Feir — indécis, tel qu'écrit ; pas
numérique ; le domaine allongé n'y change rien. A320 est une modulation d'ordre `ω(ak)²`, l'ordre où δ, linéarisé autour d'Airy, est
faux : l'advection antisymétrique n'est plus la piste ; **le critère « < 0,01 s⁻¹ » de C7d-3a était mal posé** (une vraie houle de
7,5 cm module à 0,027). **Trouvé en route, A324** (sévérité 3) : en mode relatif, quand la surface de B franchit un centre de maille
(houle de plus d'une demi-maille), δ est amplifié huit fois en une seconde à l'échelle de la maille — seuil net entre 6 et 6,5 cm à
12,5 cm. Suite **757**, zéro avertissement. **Rituel.** Maillons **23** — justifié : la priorité du solveur (S406) ; 4.19 reste
partiel. Suivant : **A324**, puis C7d-3a (le critère rapporté à Benjamin-Feir, B de Stokes).

## S436 — 2026-10-02 — physique : A324 corrigée ; A320 à maille fine

**Entrée.** *« Continue »* — A324. **Fait** ([preuve](../../docs/validation/MER-S369.md) §8) : un banc pas à pas
(`a324_franchissement`) **corrige S435** — le banc `mer` ne fait pas avancer son témoin en mode germe ; pas à pas, il n'est pas nul :
A324 est **une rupture du point fixe** du mode relatif. Cause : le fantôme latéral (entre une colonne mouillée et une sèche) retranchait
l'erreur de B interpolée entre les colonnes, non celle du point de surface. Quatre essais ; **retenu** : il interpole les fantômes
verticaux des deux colonnes (`set_lateral_own_ghost`, **le défaut**). **Mesuré** : à 12,5 cm sous 6,5 et 7,5 cm, le germe reste à
1,1 mm (avant : 8,6 et 10 mm), rien sous `4·dx` ; le témoin nul au bit ; 25 cm au bit jusqu'à 70 s, A320 inchangée. **A324 corrigée.**
**A320, enfin mesurable à 12,5 cm** : 0,027 s⁻¹ sous 7,5 cm (1,0 fois Benjamin-Feir ; 25 cm : 0,106) ; sous 6 cm sans franchissement,
≈ 0,031 à maille nulle (1,8 fois). Suite **758**, zéro avertissement. **Rituel.** Maillons **24** — justifié : la priorité du solveur
(S406) ; 4.19 reste partiel. Suivant : **C7d-3a** — le critère rapporté à Benjamin-Feir, à la maille de la production.

## S437 — 2026-10-02 — physique : C7d-3a, le critère réécrit — non reçu ; A320 tient à la place de la surface

**Entrée.** *« Continue »* — C7d-3a. **Le critère réécrit avant mesure** : à 25 cm, au plus 1,5 fois Benjamin-Feir sous 6 et 7,5 cm,
toute place du repos dans la maille. **Fait** ([preuve](../../docs/validation/MER-S369.md) §9) : le banc `mer` gagne `MER_DECALAGE`,
`MER_TEMOIN` (le témoin avance enfin en mode germe), `MER_AIR`, `MER_TP`, `MER_PROFONDEUR`, `MER_GERME_LAMBDA` ; un essai d'amputation
aux faces de surface (`set_cross_surface_trial`). **Mesuré** : le témoin nul au bit partout ; sous la houle de 4 m, **le taux dépend de
la place du repos** — 7,5 cm : 0,106 sur une face, 0,068 à ¼, **0,036 au centre** ; 6 cm : 0,077 · 0,066 · **0,018** — un défaut de
discrétisation près de la surface ; ni les termes croisés aux faces de surface, ni la bande. Sous une **houle de 8 m** (32 mailles par
longueur d'onde), rien au-delà de Benjamin-Feir, aux deux places. **C7d-3a non reçu**, le critère tenu tel qu'écrit. Suite **758**.
**Rituel.** Maillons **25** — justifié : la priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7d-3a** — localiser le défaut.

## S438 — 2026-10-02 — physique : A320, l'échelle en mailles par longueur d'onde — indécise ; C7d-3b passe

**Entrée.** *« Continue »* — C7d-3a, localiser le défaut. **Écartée avant tout code** : une quasi-résonance de triades ouverte par la
dispersion discrète (en eau profonde, l'écart est d'ordre `Ω/2`). **Éprouvée** ([preuve](../../docs/validation/MER-S369.md) §10) :
l'excès viendrait de l'onde liée `2K`, collée à la surface et mal résolue — une fonction de `λ_B/dx` seul. **Mesuré**, `ak` = 0,118,
16 mailles par longueur d'onde : 8 m à 50 cm, 2,2 fois Benjamin-Feir sur une face, 0,5 au centre ; 2 m à 12,5 cm, 5,4 et 1,9 fois ;
mais 4 m à 12,5 cm (32 mailles) montait à 3,1 fois (S436). **Indécis**, tel qu'écrit ; la cause n'est pas trouvée. La règle déclarée en
S437 s'applique : le constat et l'enveloppe à 25 cm s'écrivent, **C7d-3b vient, sans la bascule des défauts**. Aucun code du cœur
changé. **Rituel.** Maillons **26** — justifié : la priorité du solveur (S406) ; 4.19 reste partiel. Suivant : **C7d-3b**.

## S439 — 2026-10-02 — physique : C7d-3b, le mode relatif sur la carte — non reçu tel qu'écrit, d'un cheveu

**Entrée.** *« Continue »* — C7d-3b. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §22.10) : `RELATIVE` dans le pas
et le couplage de la carte (prédiction sans le résidu de B, bande relative, fantômes moins l'erreur de B, fantôme latéral d'A324),
pipelines compilés à la demande (`Step3::set_relative`) ; bancs `RELATIF`, `TEMOIN`, `--delta3d-temoin-relatif`. **Trouvé** : la bande
`band(surface) − band(own)` laissait 9·10⁻¹¹ m (une somme réassociée par le compilateur, L345) ; réécrite entre les deux surfaces, en
différences exactes. **Mesuré** : production **au bit** ; témoin **nul au bit** sur 400 pas ; horizon du millimètre 260 contre 130 ;
mais l'écart avant l'horizon, 2,0003 fois celui du pas de S297 — **manqué de 0,01 %**, sur des fenêtres inégales (sur la même, 500 fois
mieux). **Non reçu tel qu'écrit.** Cœur inchangé. **Rituel.** Maillons **27** — justifié : la priorité du solveur (S406) ; 4.19 reste
partiel. Suivant : **C7d-3b**, le critère (3) réécrit sur une même fenêtre, puis rejoué.

## S440 — 2026-10-02 — physique : C7d-3b reçu ; A322 sous le mode relatif

**Entrée.** *« Continue sinon j'accepte l'écart »* — **C7d-3b reçu**, l'écart de 0,01 % accepté. **Fait**
([preuve](../../docs/validation/MULTIGRILLE-3D-S385.md) §7) : le banc d'A321 gagne `RELATIF=1`. **Mesuré**, la scène d'A322 (10 cm,
80 × 80, 30 Hz, mg 8) : le témoin explose toujours au pas 1 860 ; **le mode relatif tient 120 s** (`max_u` ≤ 1,25 m/s, divergence et
résidu dans l'ordre du témoin) — le résidu du fond retiré, comme l'attribution de S409 le laissait prévoir ; mais **des bouffées à
l'échelle de la maille** dans `w`, près de la surface (jusqu'à 0,6 m/s ; part de maille > 0,05 pendant 65 s sur 120). **A322 non levée
telle qu'écrite**, transformée. Cœur inchangé. **Rituel.** Maillons **28** — justifié : la priorité du solveur (S406) ; 4.19 reste
partiel. Suivant : **A322** — localiser les bouffées du mode relatif à 10 cm.

## S441 — 2026-10-02 — physique : A322, les bouffées du mode relatif — la bande FTCS ; Lax-Wendroff

**Entrée.** *« Continue »* — A322. **Mesuré** ([preuve](../../docs/validation/MULTIGRILLE-3D-S385.md) §8) : les bouffées presque éteintes
à 60 Hz, absentes sans paquet (δ nul au bit sur la scène entière) et à 25 cm, inchangées à 24 cycles — une limite de pas ; attribution
(les commutateurs de S391 en mode relatif) : aucun terme d'advection ; **la bande relative**, éteinte, les éteint toutes. **Cause** :
`η′` transporté à la vitesse de B, hauteur de face centrée, pas explicite — **FTCS** (`C²/2` par pas, `C` ≈ 0,3). **Fait** : la bande
sous Lax-Wendroff, référence et carte, éteinte par défaut ; essai du point fixe. **Mesuré** : la scène tient 120 s, 10 s au-dessus de
0,05 (contre 65), sur un champ presque nul ; la trajectoire suit sa référence à 1,4·10⁻⁵ m ; production au bit. **A322 non levée telle
qu'écrite** (le critère en part relative, mal posé). Suite **759**. **Rituel.** Maillons **29** — justifié : la priorité du solveur
(S406) ; 4.19 reste partiel. Suivant : **A322** — le critère en amplitude absolue, ou l'écart accepté.

## S442 — 2026-10-02 — physique : A322 levée en mode relatif ; Lax-Wendroff par défaut ; A320 inchangée

**Entrée.** *« J'accepte et continue »* — **A322 levée en mode relatif**, l'écart accepté. **Fait** : la bande relative sous
Lax-Wendroff devient le défaut du mode relatif, référence et carte (`MER_BANDE_LW=0`, `BANDE_LW=0` rendent l'ancienne). **Mesuré** :
production au bit ; témoin relatif nul au bit ; trajectoire relative à 1,4·10⁻⁵ m ; suite **759**. **A320 sous Lax-Wendroff**
([preuve](../../docs/validation/MER-S369.md) §11) : inchangée au chiffre près (7,5 cm : 0,106 sur une face, 0,035 au centre) — le FTCS de la
bande n'en est pas la cause (`C` ≈ 0,01 à 25 cm). **C7d-3a non reçu.** **Rituel.** Maillons **30** — justifié : la priorité du solveur
(S406) ; 4.19 reste partiel. Suivant : **la bascule des défauts vers le mode relatif**, proposée sans attendre C7d-3a.

## S443 — 2026-10-02 — méthode et physique : ADR-213 ; la bascule des défauts — C7d-3b reçu en entier

**Entrée.** *« J'accepte ta proposition »* (la bascule sans attendre C7d-3a) ; *« où cela bloque »* — expliqué (A320, cinq sessions ;
un verrou en chaîne ; des calculs longs ; un rituel lourd ; deux arrêts pour des écarts infimes) ; *« Ok go »* — **ADR-213** : tolérance
de 5 %, plafond de deux sessions, registres par lots de trois, bancs courts. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md)
§22.11) : `Volume3` et `Step3` naissent en mode relatif ; deux essais épinglés au pas de S297 ; `RELATIF=0` aux bancs. **Mesuré** : S297
au bit sous `RELATIF=0` ; trajectoire 1,4·10⁻⁵ m ; témoin nul au bit ; cas 2 à 7,6·10⁻⁷ m ; coût 3,67 ms contre 3,73 ; la scène de
revue tient 120 s ; suite **759**. **C7d-3b reçu.** **Ce qui devient possible** : δ relatif dans la production — point fixe exact sous B,
la scène à 10 cm tenue ; **chemin** : `Step3`, le pas de la scène. **Rituel** (allégé). Maillons **0**. Suivant : **C7d-3c**.

## S444 — 2026-10-02 — physique : C7d-3c, voie (B) — c1 non reçu

**Entrée.** *« Continue »* ; à la question de la voie de C7d-3c, *« (B) B dans la bande »* — **ADR-214** (remplace D1 de S433 : la
bande n'entre pas dans le pas couplé, B entre dans la bande ; 3 à 5 sessions estimées au lieu de 8 à 12). **Fait**
([preuve](../../docs/validation/APIC-CARTE-S416.md) §23.1) : `LinearSwell` complet ; `Apic3` en mode relatif (particules en `u′` déplacées
par `U + u′`, `u′·∇U` exact, `p′` de surface), une ou deux composantes de B. **Mesuré** : une houle progressive dans une cuve fermée
fait `u′ = −U` (faute du banc) ; une houle stationnaire partie à plat — `|u′|` croît jusqu'à 54 % de `aω` en 5 s, 16 mm d'écart à l'eau
totale. **c1 non reçu** ; hypothèse : la lecture de la surface des particules forcée en résonance avec B. Suite **761**. **Rituel**
(allégé). Maillons **1**. Suivant : c1 — éprouver l'hypothèse, ou l'eau totale dans la bande (ADR-213 D2) ; **le lot des registres**.

## S445 — 2026-10-02 — physique : le lot des registres ; c1 plafonné — l'eau totale dans la bande

**Entrée.** *« Continue »*. **Fait** : le lot des registres (ADR-213 D3) pour S443–S444. **Mesuré**
([preuve](../../docs/validation/APIC-CARTE-S416.md) §23.2), `Apic3` relatif, départ à plat : `|u′|` ∝ `a` (57, 56, 54 % de `aω` pour 1,25,
2,5, 5 cm), 39 % à 12,5 cm — une erreur du premier ordre du schéma discret appliqué à B, en résonance ; le pas couplé l'évite (surface de
B analytique), une bande de particules non. **c1 non reçu, plafonné** (ADR-213 D2). **Décidé** (note d'ADR-214) : la bande simule
l'eau totale, B n'entre qu'à sa frontière ; le raccord avec la mer relative devient c2. **Rituel** (allégé). Maillons **2** — justifié :
la priorité du solveur (S406). Suivant : **c2**, la conception du raccord.

## S446 — 2026-10-02 — physique : c2, le raccord bande ↔ mer — première session

**Entrée.** *« Continue »* — c2. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §23.3) : `Apic3` aux bords ouverts en `x`
(vitesse normale imposée, débit de bord compté ; essai : le volume change exactement de ce qui passe) ; `set_grid_velocities` ; banc
`raccord_bande_mer` — une bande `Apic3` en eau totale (4 m, zone de colonnes) dans une mer `Volume3` relative (16 m). **Mesuré** sous B
seul (5 cm, 4 m, 10 s) : δ hors de la bande **9,86 mm** (tenu, sous 1 cm ; il plafonne) ; la masse oscille de **4,4 %** d'une
demi-période (manqué, sous 1 %) — le flux de l'interface n'est pas compté pareil des deux côtés. Suite **762**. **Rituel** (allégé).
Maillons **3** — justifié : la priorité du solveur (S406). Suivant : **c2**, le raccord conservatif (seconde session, ADR-213 D2).

## S447 — 2026-10-02 — physique : c2, le raccord conservatif — la masse au raccord à 0,01 %

**Entrée.** *« Continue »* — c2, seconde session. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §23.4) : le raccord
conservatif — la mer seule comptable de la masse de δ, la forme de la bande décalée d'un `c` uniforme, le même `c` rendu à la bande ; le
banc publie le bilan du pas de la mer. **Mesuré** (5 cm, 4 m, 10 s) : δ hors de la bande **9,4 mm** (tenu) ; dérive brute 3,5 % d'une
demi-période (**manqué tel qu'écrit**), dont l'éponge et la bande de B expliquent tout sauf **3,9·10⁻⁶ m³ — 0,01 % au raccord** (S446 :
6,8 %). Le critère (2) mêlait l'éponge au raccord. **c2 plafonné, le raccord retenu.** Suite **762**. **Rituel** (allégé). Maillons
**4** — justifié : la priorité du solveur (S406). Suivant : **c3**, la vague de Chen dans une houle ; le lot des registres (S448).

## S448 — 2026-10-02 — physique : le lot des registres ; le raccord dans le système ; c3, premier jet

**Entrée.** *« Continue »*. **Fait** : le lot des registres (ADR-213 D3) ; **le raccord dans le système** —
`water_core::band_in_sea::BandInSea`, les chiffres de S447 au bit ([preuve](../../docs/validation/APIC-CARTE-S416.md) §23.5) ; le banc
`deferlement_en_mer` — un groupe de Stokes qui déferle au milieu d'une bande de 8 m, dans une mer de 24 m sous une houle calme.
**Mesuré** : retournement dans la mer à 0,736 s ; masse au raccord ≈ 0 (3,4·10⁻⁷ m³) ; mais le témoin (la bande seule) dépend de sa
largeur (0,576 à 0,846 s) — critère mal posé ; sous une houle calme, 42 colonnes en particules (la bascule à ses défauts). **c3 non
reçu**, première session. Suite **762**. **Rituel** (allégé). Maillons **5** — justifié : la priorité du solveur (S406). Suivant : **c3**
— le témoin juste (la bande sur toute la mer), la bascule avec les réglages de C7d-1/C7d-2.

## S449 — 2026-10-02 — physique : c3, seconde session — plafonné

**Entrée.** *« Continue »* — c3. **Fait** ([preuve](../../docs/validation/APIC-CARTE-S416.md) §23.6) : le témoin juste (la bande sur
toute la mer), la bascule réglée (C7d-1/C7d-2), la même fenêtre partout, 1 m d'air ; l'essai de la hauteur lue sur les colonnes de
particules (`set_particle_heights`, éteint). **Mesuré** : retournement dans la mer 0,482 s contre 0,562 s au témoin (**14 %**, manqué) ;
part de la fenêtre et masse au raccord tenues ; sous une houle calme, la mer devient instable au bord de la bande (manqué). Deux
exigences du raccord se contredisent au bord des colonnes de particules. **c3 plafonné** (ADR-213 D2). Suite **762**. **Rituel**
(allégé). Maillons **6** — justifié : la priorité du solveur (S406). Suivant proposé : **la surface continue** (ADR-211 D2).

## S450 — 2026-10-02 — rendu : la surface continue, une première image

**Entrée.** *« Continue »* — la surface continue (ADR-211 D2). **Fait** ([preuve](../../docs/validation/SURFACE-CONTINUE-S450.md)) :
le banc `surface_continue` — l'isosurface du champ unique `φ` d'`Apic3` (colonnes et bande d'un même champ) par tétraèdres marchants,
un rendu logiciel en PPM ; B10 (une sphère entre dans l'eau) en bande étroite. **Mesuré** : **0 arête ouverte** (le maillage est étanche) ;
**le saut au raccord bande | colonnes, 0,34 maille** (manqué, au plus un quart) — **visible en marches carrées** autour de la zone agitée ;
28 s. Quatre images envoyées à l'utilisateur. Suite **762**. **Rituel** (allégé). Maillons **7** — justifié : la priorité du solveur
(S406). Suivant : raccorder `φ` à la frontière pour le rendu, des normales lissées ; le lot des registres (S451).

## S451 — 2026-10-02 — rendu : la surface continue, après R36

**Entrée.** **R36** : *« Alors le problème est que l'on voit des divisions faces plane »* — non reçu. **Fait** : le lot des registres
(ADR-213 D3) ; au banc `surface_continue` ([preuve](../../docs/validation/SURFACE-CONTINUE-S450.md) § S451), `φ` fondu au raccord bande |
colonnes et des normales lissées, ombrées par pixel — le calcul n'est pas touché. **Mesuré** : le saut au raccord **0,083 maille**
(S450 : 0,337) ; **0 arête ouverte** ; les images lisses, sans marches. **R37** envoyé à l'utilisateur. Suite **762**. **Rituel**
(allégé). Maillons **8** — justifié : la priorité du solveur (S406). Suivant : selon R37 — le rendu en direct sur la carte, puis C10.

## S452 — 2026-10-02 — rendu : la surface continue en direct, sur la carte

**Entrée.** **R37** : *« Je valide le render »* — reçu. **Fait** ([preuve](../../docs/validation/SURFACE-CONTINUE-S450.md) § S452) :
`surface_carte` dans l'afficheur — le fondu de S451 en deux passes de calcul et un lancer de rayons par pixel dans `φ`, sur le device de
la carte, sans retour au CPU ; le banc `--surface-carte` (B10 en bande étroite). **Mesuré** : le fondu égal à celui du CPU à
**7·10⁻⁷** ; **0,23 ms** par image de 960 × 600 ; quatre images comparables à R37, envoyées. Suite **762**. **Rituel** (allégé).
Maillons **9** — justifié : la priorité du solveur (S406). Suivant : le rendu dans la boucle vivante de l'afficheur, puis C10.

## S453 — 2026-10-02 — rendu : la surface continue dans la boucle vivante

**Entrée.** *« Continue »*. **Fait** ([preuve](../../docs/validation/SURFACE-CONTINUE-S450.md) § S453) : la carte choisit son pas
(`ApicCarte::stable_step_us`, la formule de la référence) — B10 avance **sans référence CPU** ; la fenêtre `--surface-direct`
(`surface_direct.rs`) : la simulation au temps réel sur la carte, `surface_carte` rendu à chaque image, orbite, pause, relance.
**Mesuré** : masse en quanta exacte, images de S452 à 0,1 % des pixels près ; image 1,3 ms en médiane (14 ms au 99e centile),
simulé / réel 0,97. Suite **762**. **Rituel** (allégé). Maillons **10** — justifié : la priorité du solveur (S406). Suivant : le lot
des registres (dû en S454), puis C10 ou le verdict de l'utilisateur sur la fenêtre.

## S454 — 2026-10-03 — C10 conçue ; C10-1, le saut sur un domaine entier ; ADR-215

**Entrée.** *« Continue »*, puis la décision : *« ne t'arrête pas de travailler jusqu'à une v1 solide visuellement et physiquement,
prends les décisions »* — [ADR-215](../../docs/adr/ADR-215-autonomie-jusqu-a-une-v1-solide.md). **Fait** : le lot des registres ; C10 en
cinq lots ; **C10-1** ([preuve](../../docs/validation/C10-SCENES-S454.md) §3) — B10 sur un domaine entier, carte seule. **Mesuré** : le
quart déplié s'accorde au quart (cavité 0,75 maille ; le jet sur l'axe 2,7 mailles plus haut — tranché) ; la scène de 4 m **divergeait
au plafond** (une nappe plaquée) — avec 2,5 m d'air, stable jusqu'à t = 16, masse exacte ; 27 ms par pas, simulé / réel 0,11. Suite
**762**. **Rituel** (allégé). Maillons **11** — justifié : la priorité du solveur (S406). Suivant : le temps réel à 4 m.

## S455 — 2026-10-03 — le temps réel à 4 m

**Entrée.** ADR-215 D4, étape 2, sans « Continue ». **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §4) : le profil des
étages (la projection, 13,9 ms sur 24,7) ; la vitesse maximale réduite sur la carte (le pas stable sans relecture, identique au pas
près) ; une scène de jeu (1,4 m d'eau, 1,5 m d'air) ; les horodatages éteints en direct ; le nombre de Courant 1,0, stable et
indiscernable à l'image (tranché). **Mesuré** : la fenêtre à 4 m passe de **0,11 à 0,98** du temps réel (0,83 pendant le saut) ; masse
exacte, stable jusqu'à t = 16. Suite **762**. **Rituel** (allégé). Maillons **12** — justifié : la priorité du solveur (S406).
Suivant : la houle.

## S456 — 2026-10-03 — la houle

**Entrée.** ADR-215 D4, étape 3. **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §5) : le bord ouvert sur la carte (B imposée
aux faces des bords, son débit compté dans les colonnes, le volume entré cumulé) ; l'état initial de B ; les zones de relaxation —
la surface seule (ramener les vitesses faisait dériver le niveau, écarté). **Mesuré** : masse exacte ; la houle sans décroissance
(0,84 à 1,05 `a`, modulation de ±10 % inscrite — tranché) ; l'écart à B sous 0,25 `a` ; le saut sous la houle stable, la fenêtre à 0,98
du temps réel. Suite **762**. **Rituel** (allégé). Maillons **13** — justifié : la priorité du solveur (S406). Suivant : le lot des
registres, puis la lumière de l'eau.

## S457 — 2026-10-03 — le lot des registres ; la lumière de l'eau

**Entrée.** ADR-215 D4, étape 4. **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §6) : le lot des registres (S454–S456) ;
la lumière reçue (R14, R20, R24) portée de Godot dans `surface_carte` — le ciel de la photographie, Fresnel, la colonne d'eau de
Maritorena sur un fond de sable ; la mer de B analytique au-delà du domaine (tranché). **Mesuré** : le champ inchangé ; 1,2 ms par
image ; le raccord corrigé d'un trait d'une demi-maille ; images envoyées. Suite **762**. **Rituel** (allégé). Maillons **14** —
justifié : la priorité du solveur (S406). Suivant : la scène `--v1`.

## S458 — 2026-10-03 — la scène `--v1`

**Entrée.** ADR-215 D4, étape 5. **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §7) : les sauts répétés (`sphere_saut`),
`--v1` et `--v1-banc`, la tolérance de la projection réglable, un budget de pas par image. **Mesuré** : 60 s, une douzaine de sauts,
masse exacte, aucun arrêt ; la fenêtre de 0,83 à **0,99** du temps réel (0,95 pendant les sauts), **22,8 ms** au 99e centile —
Courant 1,5 et 10⁻⁴ tranchés. **Ce qui devient possible** : la scène vivante jouée en temps réel ; **le chemin qui la consomme** :
R38, puis C10-2 ; **la preuve** : §7. Maillons **0**. Suite **762**. **Rituel** (allégé). Suivant : le verdict R38.

## S459 — 2026-10-03 — C10-2, premier pas : le saut dans la mer δ, au CPU

**Entrée.** R38 reçu (*« Correct pour une V1 »*), puis *« Parfait continue »*, avec une question sur les moteurs de rendu
(répondue : l'afficheur, Godot, les bancs, le lancer de rayons — voulu, δ entre dans Godot en C11). **Fait**
([preuve](../../docs/validation/C10-SCENES-S454.md) §8) : la couronne du raccord épinglée en colonnes, le volume déplacé, l'anneau des
vitesses ; le banc `saut_en_mer`. **Mesuré** : masse au raccord 10⁻⁷ ; **la mer refuse à 0,7 s sous le jet**, quoi qu'on donne à
l'intérieur ; le témoin du cratère mal posé (parois). Suite verte (+1). **Rituel** (allégé). Maillons **1**. Suivant : localiser le
refus de la mer.

## S460 — 2026-10-03 — C10-2, seconde session : le refus localisé, plafonné

**Entrée.** *« Continue »*, puis *« Continue par la suite avec le branchement dans godot »*. **Fait**
([preuve](../../docs/validation/C10-SCENES-S454.md) §9) : le refus de la mer localisé — sous les colonnes de particules, où la mer gardait
sa hauteur, poussée par le jet ; corrigé (`set_particle_rest`) ; l'instabilité passe alors à la marge du raccord (la dent de scie de
S449). **Plafonné** (ADR-213 D2, troisième session). Suite verte. **Rituel** (allégé). Maillons **2**. Suivant : C11, la scène `--v1`
dans Godot.

## S461 — 2026-10-03 — C11 : la scène `--v1` dans Godot

**Entrée.** La décision de S460 : Godot. **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §10) : l'enregistrement de la
scène par l'afficheur (`EXPORT_GODOT` : `φ` sur 8 bits, 30 images/s) et son rejeu dans Godot (`saut.tscn` : lancer de rayons dans la
texture 3D, la mer de B au-delà, l'optique reçue, AgX). **Mesuré** : quantification 0,39 mm ; 416 images/s ; images envoyées, R39
posée. **Ce qui devient possible** : le rendu de Godot sur la simulation ; **le chemin** : R39, la suite de C11 ; **la preuve** : §10.
Maillons **0**. Suite verte. **Rituel** (allégé). Suivant : selon R39.

## S462 — 2026-10-03 — C11 : les caustiques sur le sable

**Entrée.** *« … puis continue »* (la commande de Godot corrigée pour PowerShell). **Fait**
([preuve](../../docs/validation/C10-SCENES-S454.md) §11) : la focalisation de la surface simulée, déposée par l'afficheur à chaque image
enregistrée, lue par le sable dans Godot ; B analytique au-delà. **Mesuré** : la formule ponctuelle ne conservait pas l'énergie (1,1 à
2) ; le dépôt, 0,98 à 1 ; 405 images/s ; images envoyées. Suite verte. **Rituel** (allégé). Maillons **1**. Suivant : la surface fine,
ou ce que R39 désigne.

## S463 — 2026-10-03 — C11 : la surface fine

**Entrée.** *« Continue en autonomie »* (inscrit). **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §12) : les cascades FFT de
S360 sur la scène du saut, à mi-force (tranché : la mer de la scène est calme). **Mesuré** : pente quadratique ajoutée 0,0087 ; le
raccord ne se voit plus ; 391 images/s ; images envoyées. Suite verte. **Rituel** (allégé). Maillons **2**. Suivant : le lot des
registres (en retard d'une session), puis le direct.

## S464 — 2026-10-03 — le lot des registres ; C11, le direct

**Entrée.** En autonomie. **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §13) : le lot des registres (S457–S463) ; **le
direct** — l'afficheur calcule la scène au temps réel et la pousse à Godot par un lien local (tranché, plutôt que godot-rust). **Mesuré** :
de 0,55 à **0,939** du temps réel et **28 images/s** — la relecture sans attente, l'encodage sur un fil, et un `φ` nul (l'uniforme du
fondu jamais écrit) corrigé. **Ce qui devient possible** : la simulation jouée dans Godot en temps réel ; **le chemin** : la pluie, R39 ;
**la preuve** : §13. Maillons **0**. Suite verte. **Rituel** (allégé). Suivant : la pluie.

## S465 — 2026-10-03 — C11 : la pluie

**Entrée.** En autonomie. **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §14) : la pluie d'ADR-205 sur la scène du saut dans
Godot — les rides, le ciel couvert, les gouttes, les gerbes, l'extinction. **Mesuré** : 447 anneaux/m²/s à 10 mm/h ; ni éclat ni
caustiques sous le couvert ; 174 images/s à 10 mm/h, 60 à 50 mm/h. Images envoyées. Suite verte. **Rituel** (allégé). Maillons **1**.
Suivant : le joueur (une capsule debout).

## S466 — 2026-10-03 — le joueur : une capsule debout

**Entrée.** En autonomie. **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §15) : le corps de la carte en capsule (la
sphère au bit pour `L = 0`) ; le joueur de `--v1` debout, 0,3 × 1,7 m, qui saute pieds en avant, freine et s'arrête à 0,5 m du
fond ; le rendu suit (l'afficheur, Godot, l'export). **Mesuré** : 60 s, 3118 pas, écart de masse 0. Arrêté net à 0,2 m du fond,
la scène divergeait (15 à 31 s) ; les zones de relaxation en `y`, essayées, la cassaient : retirées. Images envoyées. **Rituel**
(allégé). Maillons **1**. Suivant : le lot des registres.

## S467 — 2026-10-03 — le lot des registres ; le joueur éclairé comme l'eau

**Entrée.** En autonomie. **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §16) : le lot des registres pour S464–S466
(FEUILLE-DE-ROUTE, LISTE, REPRISE ; prochain au plus tard S470) ; le joueur éclairé par notre nuanceur (`joueur.gdshader`), le même
albédo que le corps vu à travers l'eau — un gris continu à travers la surface. Images envoyées. **Rituel** (allégé). Maillons **1**.
Suivant : l'ombre du joueur.

## S468 — 2026-10-03 — l'ombre du joueur

**Entrée.** En autonomie. **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §17) : le soleil direct occulté par le joueur,
sur le fond (chemin réfracté), ses caustiques, le corps d'eau et l'éclat, dans Godot. **Mesuré** : l'ombre vers le sud-est, à
l'opposé du soleil ; ciel couvert identique au bit ; 397 images/s (−2 %). Image envoyée. **Rituel** (allégé). Maillons **1**.
Suivant : le direct avec le joueur debout.

## S469 — 2026-10-03 — le direct avec le joueur debout

**Entrée.** En autonomie. **Fait** ([preuve](../../docs/validation/C10-SCENES-S454.md) §18) : le direct remesuré sur la scène du joueur
debout — **0,995 du temps réel, 29,9 images/s**. Deux défauts de `saut.gd` corrigés (des matériaux nuls touchés avant l'en-tête,
depuis S465 ; le lien fermé lu). Image envoyée. **Rituel** (allégé). Maillons **1**. Suivant : la caméra qui suit le joueur ; le
lot des registres (dû en S470).
