# Index de la connaissance projet

[Reprise](../REPRISE.md) · [Feuille de route](FEUILLE-DE-ROUTE.md) · [File active](registres/QUESTIONS-OUVERTES.md#file-active) · [Liste du projet fini](LISTE-PROJET-FINI.md)

## Carte par système — par où entrer

*S321 ([ADR-187](adr/ADR-187-methode-refondue-s321.md)).* Cet index **se consulte**, il ne se lit
pas en entier. Pour chaque système : les décisions qui le gouvernent et les preuves de son état
présent ; l'état lui-même est dans la [feuille de route](FEUILLE-DE-ROUTE.md), les travaux dans la
[file active](registres/QUESTIONS-OUVERTES.md#file-active). Le catalogue complet suit.

| système | décisions | preuves de l'état présent |
|---|---|---|
| **pilotage et méthode** | [127](adr/ADR-127-ambition-complete-construction-progressive.md) ambition, [174](adr/ADR-174-arbitrages-du-2026-09-19.md) arbitrages, [178](adr/ADR-178-strategie-en-trois-systemes-physiques.md) trois systèmes, [187](adr/ADR-187-methode-refondue-s321.md) méthode, [188](adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md) alternance, [189](adr/ADR-189-la-v1-d-abord.md) la v1 d'abord, [190](adr/ADR-190-apres-la-v1-la-liste-entiere.md) après la v1, la liste entière, [191](adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md) le rendu réaliste, un module du moteur, [192](adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md) le rendu de l'eau dans Godot 4 | [BILAN-GLOBAL-S321](registres/BILAN-GLOBAL-S321.md), [TROIS-SYSTEMES-S308](registres/TROIS-SYSTEMES-S308.md), [liste du projet fini](LISTE-PROJET-FINI.md) |
| **A — haute mer (B + W)** | [129](adr/ADR-129-chemin-image-de-w-par-table-de-bessel.md), [130](adr/ADR-130-rendu-j1-sur-gpu-par-un-hote-separe.md), [155](adr/ADR-155-queue-spectrale-en-pentes-par-pixel.md) à [163](adr/ADR-163-sommes-des-ondes-filtrees.md), [176](adr/ADR-176-asymetries-de-la-surface-rendue.md), [177](adr/ADR-177-couleur-du-corps-d-eau-derivee-de-ses-sources.md) | [S259](validation/MER-MULTIMODALE-S259.md), [S260](validation/VAGUES-POINTUES-S260.md), [S267](validation/CUISSON-SILLAGE-S267.md), [S304](validation/ASYMETRIES-S304.md), [S306](validation/STRIES-S306.md), [S307](validation/RENDU-ECART-S307.md), les crêtes de B, écume et lumière : [S356](validation/RENDU-CRETES-S356.md), [revue visuelle](validation/REVUE-VISUELLE.md) |
| **B — δ volumique 3D** | [175](adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) exécution, [184](adr/ADR-184-seconde-representation-en-parallele.md) et [186](adr/ADR-186-apic-seconde-representation.md) seconde représentation | 3D : [S297](validation/DELTA3D-COUPLEE-S297.md), [S298](validation/DELTA3D-FOND-REEL-S298.md), [S301](validation/DELTA3D-PAS-GPU-S301.md), [S302](validation/SCENE-DELTA3D-S302.md), [S305](validation/CUVE-GPU-S305.md) ; APIC : [S318](validation/COMPARAISON-LOT5-S318.md), [S320](validation/B10-APIC-S320.md) ; 2D : [S253](validation/SURFACE-COUPLEE-S253.md), [S274](validation/HOULE-USAGE-S274.md) |
| **C — couplage** | [179](adr/ADR-179-tolerances-de-conservation-et-grandeur-restituee.md) à [183](adr/ADR-183-essai-oblique-phase-a-distance-et-ordre-c.md), [185](adr/ADR-185-ordre-d-receveur-sous-i15.md) | [S310](validation/BILAN-MASSE-S310.md), [S311](validation/SORTIE-DELTA-S311.md), [S312](validation/TRANSFERT-DELTA-W-S312.md), [S313](validation/PLANCHER-BILAN-S313.md), [S314](validation/TRANSFERT-ORIENTE-S314.md), [S315](validation/ORACLE-ET-OBLIQUE-S315.md), [S316](validation/ORDRE-C-S316.md), [S317](validation/RESTITUTION-S317.md), [S319](validation/MER-S319.md) |
| **porte D — solides** | [008](adr/ADR-008-flottabilite-et-autorite.md) autorité, [188](adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md) lot 3, [189](adr/ADR-189-la-v1-d-abord.md) la v1 d'abord | faces coupées 2D : [S232](validation/FLUX-COUPES-S232.md) ; 3D, modes linéaire et mobile, solide immergé : [S324](validation/FACES-COUPEES-3D-S324.md) (§7 S328, §8 S329, §9 S330 — solide mobile) ; corps rigide du jeu, C10 : [S331](validation/CORPS-RIGIDE-S331.md) ; la scène de la porte D, coque sur la houle de B et δ relatif : [S333](validation/PORTE-D-S333.md) ; masse ajoutée et amortissement de la coque, mesurés par δ : [S336](validation/RAYONNEMENT-COQUE-S336.md) |
| **V — volumes finis** | [010](adr/ADR-010-reseau-hydraulique-volumes-finis.md), [139](adr/ADR-139-volume-et-plan-oriente-des-contenants.md), [140](adr/ADR-140-restauration-du-graphe-V.md) | [S224](validation/NOYAU-V-S224.md), [S228](validation/VOLUME-ORIENTE-S228.md), [S229](validation/RESTAURATION-V-S229.md) |
| **porte A — ordonnanceur** | [012](adr/ADR-012-ordonnanceur-budget-degradation.md), [170](adr/ADR-170-les-trois-poids-sont-bornes.md), [171](adr/ADR-171-les-seuils-d-activation-appartiennent-au-profil.md) | [S279](validation/ORDONNANCEUR-S279.md), [S285](validation/ATTRIBUTION-RETRECISSEMENT-S285.md) |
| **coût** | [125](adr/ADR-125-budget-image-60hz-deux-ms.md), [131](adr/ADR-131-un-depassement-qualifie-une-implementation.md) | [S276](validation/COUT-DIRECT-S276.md), [S291](validation/PAS-DECOMPOSE-S291.md), [S302](validation/SCENE-DELTA3D-S302.md) §2 ; porte C, le pas de δ par passe : [S341](validation/COUT-DELTA3D-S341.md) |
| **références et bancs** | — | [cas canoniques](validation/CAS-CANONIQUES.md), [plan des bancs](validation/PLAN-BENCHMARK.md), [comparables externes](COMPARABLES-EXTERNES.md), [simufluid](registres/LECTURE-SIMUFLUID-S320.md) |

## Socle et état courant

- [Intentions initiales](sources/systeme_eau_architecture_globale.md) et [questions sources](sources/systeme_eau_zones_ouvertes_et_decisions_a_valider.md).
- [Invariants](01_INVARIANTS.md), [décomposition ADR-001](adr/ADR-001-decomposition-en-couches.md).
- [Bilan global S321](registres/BILAN-GLOBAL-S321.md) : code, documents, méthode — le dispositif s'use par accumulation ; la méthode refondue.
- [Bilan global S293](registres/BILAN-GLOBAL-S293.md) : où l'avancement bloque — porte B verrouillée, architecture de δ, budget sans répartition, pilotage.
- [La liste par dépendance — S352](registres/DEPENDANCES-LISTE.md) : chacun des points non validés de la liste du projet fini, ce qu'il attend et ce qu'il débloque, son front — calculé par `outils/dependances_liste.py`, tenu par `--check` ; l'ordre est au §3 ter de la feuille de route (ADR-190 D3).
- [Trois systèmes — S308](registres/TROIS-SYSTEMES-S308.md) : la stratégie A / B / C confrontée au code — ce qui existe, les six interfaces manquantes, le banc de la piscine essai par essai, l'ordre en sept lots.
- [Bilan global S227](registres/BILAN-GLOBAL-S227.md) : dérives, procédure et correctifs.
- [Comparables externes](COMPARABLES-EXTERNES.md) : systèmes du commerce regardés, avec le statut de chaque affirmation.
- [Journal](../notes/JOURNAL.md) : comptes rendus historiques ; les états présents sont dans la feuille de route.
- [Afficheur](../viewer/README.md) : lancement et commandes.

## Contrats et validation

- [SPEC-001 — Contraintes numériques et fiche de référence chiffrée](specs/SPEC-001-contraintes-numeriques.md).
- [SPEC-002 — Phénomènes secondaires et interfaces : fiche de référence chiffrée](specs/SPEC-002-phenomenes-secondaires.md).
- [SPEC-004 — Signatures des interfaces](specs/SPEC-004-interfaces.md).
- [SPEC-005 — Outillage auteur et données précalculées](specs/SPEC-005-outillage-auteur.md).
- [SPEC-006 — Le chemin poussé : ce que le système d'eau publie](specs/SPEC-006-chemin-pousse.md).
- [SPEC-003 — Harnais de validation](validation/SPEC-003-harnais-de-validation.md).
- [Cas canoniques](validation/CAS-CANONIQUES.md).
- [Plan des benchmarks](validation/PLAN-BENCHMARK.md).
- [Réception du noyau V](validation/NOYAU-V-S224.md).
- [Gravité dirigée et A266](validation/GRAVITE-DIRIGEE-S226.md).
- [Volume et plan orienté de V](validation/VOLUME-ORIENTE-S228.md).
- [Restauration du réseau V](validation/RESTAURATION-V-S229.md).
- [Cadence de l’hôte](validation/CADENCE-HOTE-S225.md).
- [LOD spatial du sillage : grille locale, reconstruction bicubique, coût](validation/LOD-SILLAGE-S234.md).
- [Scène multi-sources : admission, pente réelle, visibilité et retour au bit](validation/SCENE-MULTI-S235.md).
- [Composition sur l'union et admission de la scène S235 par le cœur](validation/ADMISSION-UNION-S236.md).
- [Candidat δ](validation/CANDIDAT-DELTA-S199.md).
- [Contrats δ](validation/CONTRATS-DELTA-S200.md).
- [Arrêt coopératif sous budget de δ](validation/BUDGET-DELTA-S230.md).
- [Pression f32 de δ : précision, résidu réel et coût](validation/PRESSION-F32-S231.md).
- [Flux ouverts et triangles fluides de δ](validation/FLUX-COUPES-S232.md).
- [Première surface évolutive linéarisée de δ](validation/SURFACE-LINEARISEE-S233.md).
- [Surface géométriquement mobile de δ contre l'onde stationnaire HOS](validation/SURFACE-MOBILE-S237.md).
- [Plancher de la pression f32 de δ : arrêt certifié et acceptation à la tolérance S199](validation/PRESSION-PLANCHER-S238.md).
- [Tolérance physique de la pression de δ : loi contre la taille, lignes franches et lignes à fantôme](validation/TOLERANCE-PRESSION-S239.md).
- [Allocations par image de l'hôte GPU : I-06 mesurée, et un suspect de gigue disculpé](validation/ALLOCATIONS-HOTE-S240.md).
- [Préparation CPU du sillage : la loi contre les tronçons, et le poste dominant](validation/PREPARATION-SILLAGE-S242.md).
- [Parallélisme déterministe : la primitive d'écriture disjointe et son prix](validation/PARALLELISME-S243.md).
- [Coût d'un pas de δ, décomposé : où va le temps et quelle technique l'attaque](validation/COUT-DELTA-S244.md).
- [Multigrille de la pression de δ : un repli de précision, et pourquoi pas de vitesse](validation/MULTIGRILLE-S245.md).
- [Prolongation de la multigrille : cinq suspects écartés, un amortissement corrigé](validation/PROLONGATION-S246.md).
- [Angles rasants : le coût tient, l'échantillonnage du champ lointain non](validation/RASANT-S247.md).
- [Topologie de la mer et maillage du LOD, en images de banc](validation/IMAGES-S248.md).
- [Coupure spectrale de l'image B/sillage : réception, coût et limites](validation/COUPURE-S249.md).
- [Revue visuelle : l'utilisateur superviseur des rendus, protocole et registre des verdicts](validation/REVUE-VISUELLE.md).
- [Premier raccordement volumique B/W→δ et démarrage plat refusé](validation/RACCORDEMENT-DELTA-S250.md).
- [Démarrage couplé plat : oracle f64, affinage de divergence, coût et attribution par pas](validation/DEMARRAGE-PLAT-S251.md).
- [Multigrille : le β du gradient conjugué, le coût du démarrage plat et 32 768 mailles](validation/MULTIGRILLE-BETA-S252.md).
- [Surface mobile couplée B/W→δ contre HOS : construction, témoin, affinage et réception](validation/SURFACE-COUPLEE-S253.md).
- [Prolongement du fond au-dessus du plan moyen : règle bornée, oracle et fournisseur B](validation/PROLONGEMENT-FOND-S254.md).
- [Queue spectrale de B en pentes par pixel : rugosité, filtre et coût](validation/QUEUE-SPECTRALE-S256.md).
- [Mer multimodale et étalement directionnel : houle longue, mer de vent, R3](validation/MER-MULTIMODALE-S259.md).
- [Queue d'équilibre et vagues pointues (CWM) : statistiques de Cox–Munk, R4](validation/VAGUES-POINTUES-S260.md).
- [Rugosité ajustée à Cox–Munk, habillage ciel clair, R5](validation/RUGOSITE-S261.md).
- [Défauts restants réparés : horizon, coût GPU, requête de jeu sous CWM](validation/DEFAUTS-S262.md).
- [Le vent comme paramètre de scène : Pierson–Moskowitz, Cox–Munk, calibration R6](validation/VENT-S263.md).
- [Reflets de la queue non résolue, comparaison R7 et coût](validation/REFLETS-S265.md).
- [Optimisation des reflets validés : cache rejeté, sommes exactes retenues](validation/CIEL-CACHE-S266.md).
- [Cuisson du sillage : mêmes grilles et images, coût réduit](validation/CUISSON-SILLAGE-S267.md).
- [Relaxation de hauteur dans le pas mobile couplé](validation/RELAXATION-SURFACE-S268.md).
- [Absorption du paquet : mesure différentielle et limites](validation/REFLEXION-PAQUET-S269.md).
- [Fond uniforme traversant : flux de bande et réception](validation/FOND-TRAVERSANT-S270.md).
- [Houle progressive : oracle initial et limites](validation/HOULE-PROGRESSIVE-S271.md).
- [Résidu progressif temporel : refus et diagnostic de quadrature](validation/RESIDU-TEMPOREL-S272.md).
- [Bande du fond en quadrature linéaire : critères et réception](validation/BANDE-LINEAIRE-S273.md).
- [Houle progressive : quelle précision sert l'usage](validation/HOULE-USAGE-S274.md).
- [Coût du pas couplé mobile : multigrille](validation/COUT-MOBILE-S274.md).
- [δ visible dans l'afficheur : protocole et revue](validation/DELTA-VISIBLE-S275.md).
- [δ en direct : coût d'un pas dans l'image](validation/COUT-DIRECT-S276.md).
- [L'onde injectée dans δ, et ce que la houle lui fait](validation/ONDE-INJECTEE-S277.md).
- [L'ordonnanceur : ce qui décide qu'une zone est simulée](validation/ORDONNANCEUR-S278.md).
- [La bande δ décidée par l'ordonnanceur](validation/ORDONNANCEUR-S279.md).
- [Un pic ne condamne plus un domaine](validation/COUT-ROBUSTE-S280.md).
- [Rétrécissement perturbatif consommé, saut borné et coût](validation/RETRECISSEMENT-S283.md).
- [Préparation progressive du rétrécissement et dérive temporelle](validation/PREPARATION-RETRECISSEMENT-S284.md).
- [Attribution de la dérive : préparation et domaine réduit](validation/ATTRIBUTION-RETRECISSEMENT-S285.md).
- [Cadence δ : coût moyen, fidélité et vieillissement des mesures](validation/CADENCE-DELTA-S286.md).
- [Passes de pression : parcours mémoire éprouvé et suite GPU](validation/PASSES-PRESSION-S287.md).
- [Opérateur et lissage GPU : précision, coût complet et limites](validation/PRESSION-GPU-S288.md), [contrat ADR-172](adr/ADR-172-candidat-pression-residente-gpu.md).
- [Solveur de pression résident GPU, consommé par le pas réel](validation/PRESSION-RESIDENTE-S289.md), [ADR-173](adr/ADR-173-le-candidat-de-pression-ne-fournit-qu-un-depart.md).
- [Coût d'appel du cycle résident : enregistrement des commandes, trois encodages au bit](validation/ENCODAGE-CYCLE-S290.md).
- [Le pas de δ décomposé étape par étape, puis allégé au bit](validation/PAS-DECOMPOSE-S291.md).
- [Référence δ tridimensionnelle, surface linéarisée — porte B, lot 1](validation/DELTA3D-LINEAIRE-S295.md) : identique au bit à la 2D quand `ny = 1`, onde oblique reçue.
- [Référence δ tridimensionnelle, surface mobile — porte B, lot 2](validation/DELTA3D-MOBILE-S296.md) : trajectoires 2D au bit, HOS et onde oblique reçus.
- [Référence δ 3D couplée et aperçu animé local — S297](validation/DELTA3D-COUPLEE-S297.md) : HOS reçu, courant traversant, limites des frontières et animation reproductible.
- [Fond spectral réel et frontières de la référence 3D — S298](validation/DELTA3D-FOND-REEL-S298.md) : B du cœur échantillonné sur la grille MAC, cas limites 2D reproduits en 3D, et la maille qu’exige une mer étalée.
- [Premier étage du pas δ 3D résident sur la carte — S299](validation/DELTA3D-GPU-S299.md) : opérateur et problème assemblés sur le GPU et reçus à moins d'un ulp, projection à travail borné jugée par le cœur, coût sur la machine de référence.
- [Le fond B évalué sur la carte et le second membre couplé — S300](validation/DELTA3D-FOND-GPU-S300.md) : paramètres publiés au lieu d'échantillons, champ reçu champ par champ, couplage reçu, et le CPU 500 fois plus lent.
- [Le pas couplé complet résident sur la carte — S301](validation/DELTA3D-PAS-GPU-S301.md) : un seul device, chaque étage reçu contre le cœur, somme compensée sauvée du compilateur, trajectoire jugée contre la sensibilité propre de la référence (A297), coût et diagnostics D3.
- [La scène du critère 3 : une onde qui traverse une mer étalée — S302](validation/SCENE-DELTA3D-S302.md) : δ 3D rendu en direct depuis sa surface publiée, front injecté à 65 cm, à-coups et grain mesurés, revue R11 demandée.
- [Anatomie d'une surface de mer, et ce qui manque à la nôtre — S303](validation/ANATOMIE-SURFACE-S303.md) : recherche sourcée sur les asymétries d'une mer réelle, mesure de la nôtre, et les deux modèles qui y répondent.
- [Les asymétries de la surface rendue, construites — S304](validation/ASYMETRIES-S304.md) : second ordre en bande étroite et modulation retardée, cinq critères d'ADR-176 tenus, +0,25 % de coût, images de R12.
- [Deux domaines δ 3D se disputent un budget — S344](validation/ARBITRAGE-3D-S344.md) : porte A, premier critère au banc ; parts d'écran, coûts mesurés, hystérésis ; l'exclusion absorbante reproduite en 3D, levée par l'oubli ; §5–6 (S349–S350) : un domaine qui se déplace et se redimensionne, au bit, coût proportionnel à sa surface ; §7 (S351) : le rang 1 de la dégradation, la porte A reçue.
- [Le coût du pas de δ sur la scène de la porte B — S341](validation/COUT-DELTA3D-S341.md) : porte C, 99ᵉ centile et décomposition par passe, techniques présentes et absentes, premier levier choisi sur la mesure ; §11 (S348) la porte C au banc ; §12 (S353) l'interpolation du rendu, δ en direct à 1,49 ms par image.
- [La production contre la référence dans une cuve — S305](validation/CUVE-GPU-S305.md) : le critère 2 de la porte B sur les cas de §4.1, mode sans fond du pas de production, chaînon CPU identique au bit, 3·10⁻⁷ m pour 3 mm exigés, phase décroissante, et l'écart séculaire chiffré.
- [D'où viennent les stries de notre mer — S306](validation/STRIES-S306.md) : le test A/B du guide reçu, la queue spectrale portant 80–85 % de l'énergie haute fréquence sans que le contraste le voie, `replis = 0`, et une coupure qui divise les stries par 3,6 pour 6 % de GPU en moins.
- [B10 sur APIC : un cylindre entre dans l'eau — S320](validation/B10-APIC-S320.md) : à `Fr` = 2 et 4, la cavité se pince vers `t ≈ 2,2 à 2,5 √(D/g)`, au même instant à deux échelles et à 5 % près quand la maille est divisée par deux ; masse exacte, air enfermé résorbé. Couronne et jet changent de 40 à 60 % avec la maille ; la masse exacte cachait un volume géométrique qui ne l'est pas.
- [L'ordre E, premiers échelons : sous une vraie mer, δ ne reste pas petit — S319](validation/MER-S319.md) : sous une houle B seule, δ **croît** jusqu'à trois fois l'amplitude de la mer en deux minutes, à un taux qui dépend de l'amplitude et presque pas de la maille ; la restitution de S317 reçoit des centaines de fois un paquet. A289 se matérialise et bloque l'ordre E.
- [Trois représentations à plusieurs couches, comparées au même niveau — S318](validation/COMPARAISON-LOT5-S318.md) : APIC, ensemble de niveaux et SPH sur le repos, le ballottement et la rupture de barrage ; **APIC** garde la masse exactement, tient la période à 0,15 % et coûte le moins ; l'ensemble de niveaux perd jusqu'à 7,6 % de volume et crée 8 % d'énergie ; SPH faiblement compressible coûte 40 fois plus. Proposition à l'utilisateur : APIC.
- [L'ordre D : le volume net de δ a un receveur — S317](validation/RESTITUTION-S317.md) : le flux à la ligne de contrôle **ferme l'intérieur** de δ au plancher d'arrondi ; le paquet déplace de l'eau de l'arrière vers l'avant ; une région **locale** adossée à chaque ligne la reçoit contre un **reçu**, l'attente tombe à **exactement zéro** — la représentation se ferme, le monde n'est pas revendiqué (I-15).
- [L'ordre C : six propriétés d'un même transfert, chacune attribuée — S316](validation/ORDRE-C-S316.md) : la phase à dix longueurs d'onde **déroulée** (−0,914 ; −0,237 ; −0,045 tour) ; la **primitive** exacte à 10⁻⁴ ; au **raccord** un degré de phase et 2 à 4 % de spectre ; tout le reste **converge avec la maille de δ**, sauf une part non linéaire que W ne porte pas. Réflexion séparée par sens ; A306 et A307 closes.
- [La phase à dix longueurs d'onde, et l'oblique à la frontière — S315](validation/ORACLE-ET-OBLIQUE-S315.md) : δ sert d'oracle à lui-même par **deux lignes de contrôle** ; `k_δ` se mesure à 0,07 % sans rien supposer, et la dissipation de δ vaut 6,3 % sur dix longueurs d'onde — mais la **phase à distance reste indéterminée**, une phase n'étant connue que modulo un tour dès que les vitesses de groupe diffèrent.
- [Le transfert orienté δ → W — S314](validation/TRANSFERT-ORIENTE-S314.md) : `WaveTrain`, une somme d'ondes planes bornée en bande et en secteur, dont l'horizon et le rayon se **calculent** ; la direction passe de 0,500 à **1,00000**, le spectre de deux octaves à 3,5 %, et la phase devient possible ; l'essai oblique à la frontière reste dû.
- [Le plancher d'un bilan de masse — S313](validation/PLANCHER-BILAN-S313.md) : la loi du résidu est `u₃₂·activité/√N`, mesurée sur quatre décades d'amplitude et deux hypothèses réfutées ; l'instrument voit une fuite de bilan à 10⁻¹³ m³ par pas et **ne voit pas** une fuite d'état, donc T1 et T2 ne sont pas redondantes ; T2 est tenue sur 10 s ; trois seuils proposés.
- [Le premier transfert δ → W — S312](validation/TRANSFERT-DELTA-W-S312.md) : W n'a de mode `k = 0` dans aucune de ses trois productions, donc le volume net relève de V ou du niveau moyen de B ; W est une couche d'**eau profonde** ; le transfert passe T3 en amplitude, et ses trois pertes — direction, spectre, volume — sont chiffrées.
- [Ce qui sort d'un domaine δ — S311](validation/SORTIE-DELTA-S311.md) : la frontière est une paroi, le cas contrôlé du lot 2, la réflexion mesurée en 3D à 1,5·10⁻⁶, et la composante que W ne peut pas porter — le volume net.
- [Le premier bilan de conservation d'un domaine δ — S310](validation/BILAN-MASSE-S310.md) : pourquoi le bilan de masse est exact et pas seulement précis, ce que l'éponge efface réellement (10,2 % du domaine par seconde), la dissipation numérique du schéma, et trois tolérances proposées.
- [L'écart entre notre rendu et l'état de l'art — S307](validation/RENDU-ECART-S307.md) : trois revues envoyées avec des options acceptées éteintes, la couleur de l'eau fausse d'un facteur 9 contre Pope & Fry et Morel, et la liste ordonnée de ce que le rendu ne contient pas du tout.
- [Bilan B4](validation/BILAN-B4-S176.md).
- [Lecture de simufluid, ciblée — S320](registres/LECTURE-SIMUFLUID-S320.md) : l'ancien banc d'un autre agent, même architecture fond + résidu. Son résidu dérive aussi sous houle, **à un taux porté par le nombre de pas** (A289 : essai à faire avant l'arbitrage) ; sa conservative level set garde le volume mais **échoue sur coque mobile** ; la loi de conservation géométrique nomme notre tassement de B10 (A313).
- [Lecture du guide de topologie d'océan reçu — S306](registres/LECTURE-GUIDE-OCEAN-S306.md) : ce qu'un guide externe redit du dépôt, ce qu'il couvre et que nous n'avons pas, et les trois points actionnables — dont le test A/B à quatre sorties, jamais fait. Source : [le guide lui-même](sources/guide_topologie_ocean_haute_mer_plage.md).
- [Angles morts](registres/ANGLES-MORTS.md).
- [Dossier de décisions et faits externes](DOSSIER-REUNIONS.md).

Les autres preuves sont dans `docs/validation/`, retrouvables par identifiant ou depuis le
journal. Elles conservent leur périmètre et leur date ; un ancien « reste à faire » ne pilote
pas une nouvelle session.

## Décisions d’architecture

Catalogue des fichiers, **sans requalification de leurs statuts historiques**. Lire la décision,
ses notes datées et les ADR qui la remplacent. L’en-tête « proposée » d’un ADR ancien ne rouvre
pas les arbitrages ultérieurs explicites (notamment ADR-027 et REPRISE §5).

| ADR | décision |
|---|---|
| [ADR-001](adr/ADR-001-decomposition-en-couches.md) | Décomposition de l'eau en quatre couches (B / W / δ / V) |
| [ADR-002](adr/ADR-002-referentiels-precision-planete.md) | Référentiels, précision numérique et planète sphérique |
| [ADR-003](adr/ADR-003-horloge-et-determinisme.md) | Horloge de simulation et déterminisme du fond |
| [ADR-004](adr/ADR-004-etat-minimal-eau-simplifiee.md) | État minimal de l'eau simplifiée |
| [ADR-005](adr/ADR-005-zone-de-transition.md) |  |
| [ADR-006](adr/ADR-006-cellules-domaines-solveurs.md) | Cellules, domaines et solveurs : trois structures distinctes |
| [ADR-007](adr/ADR-007-interface-solveur.md) | Interface de solveur et stratégie de remplacement |
| [ADR-008](adr/ADR-008-flottabilite-et-autorite.md) | Flottabilité, forces sur les solides et frontière d'autorité |
| [ADR-009](adr/ADR-009-reseau-autorite-et-replication.md) | Réseau : réplication d'événements, pas de champs |
| [ADR-010](adr/ADR-010-reseau-hydraulique-volumes-finis.md) | Réseau hydraulique des volumes finis (couche V) |
| [ADR-011](adr/ADR-011-courants-et-ecoulements-diriges.md) | Courants et écoulements dirigés |
| [ADR-012](adr/ADR-012-ordonnanceur-budget-degradation.md) | Ordonnanceur, budget et dégradation contrôlée |
| [ADR-013](adr/ADR-013-prediction-activation-precalcul.md) | Prédiction, activation et précalcul |
| [ADR-014](adr/ADR-014-mousse-spray-bulles.md) | Mousse, écume, spray et bulles |
| [ADR-015](adr/ADR-015-air-poches-et-cavites.md) | Air : poches, cavités et eau dans le vide |
| [ADR-016](adr/ADR-016-audio.md) | Audio de l'eau |
| [ADR-017](adr/ADR-017-phases-glace-et-vapeur.md) | Phases : glace et vapeur |
| [ADR-018](adr/ADR-018-traversabilite-et-navigation.md) | Traversabilité, navigation et danger |
| [ADR-019](adr/ADR-019-vue-sous-marine.md) | Vue sous-marine et interface de surface |
| [ADR-020](adr/ADR-020-bibliotheque-sans-dependance-moteur.md) | Le système d'eau est une bibliothèque sans dépendance moteur |
| [ADR-021](adr/ADR-021-autorite-des-grandeurs-derivees.md) | Autorité des grandeurs dérivées |
| [ADR-022](adr/ADR-022-persistance-de-l-eau.md) | La persistance de l'eau |
| [ADR-023](adr/ADR-023-mecanismes-restes-a-specifier.md) | Quatre mécanismes restés à spécifier |
| [ADR-024](adr/ADR-024-amendement-des-invariants-I11-I12.md) | Amendement des invariants I-11 et I-12 |
| [ADR-025](adr/ADR-025-propriete-de-la-masse-entre-V-et-delta.md) | La propriété de la masse ne quitte jamais la couche V |
| [ADR-026](adr/ADR-026-amendement-de-six-invariants.md) | Amendement de six invariants |
| [ADR-027](adr/ADR-027-les-cinq-arbitrages-tranches.md) | Les cinq arbitrages en attente, tranchés |
| [ADR-028](adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md) | ADR-020 acté, et il n'y a pas d'autres équipes |
| [ADR-029](adr/ADR-029-ce-que-la-premiere-ligne-de-code-a-appris.md) | Le langage, et ce que la première ligne de code a appris |
| [ADR-030](adr/ADR-030-l-equilibrage-est-un-critere-d-elimination.md) | L'équilibrage sur fond variable est un critère d'élimination, pas un réglage |
| [ADR-031](adr/ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md) | Le front de mouillage élimine l'ordre 1, et une position de front n'existe pas sans seuil |
| [ADR-032](adr/ADR-032-c08-n-est-pas-executable-tel-qu-enonce.md) | C08 n'est pas exécutable tel qu'énoncé : un ordre est une propriété du couple (solveur, cas) |
| [ADR-033](adr/ADR-033-lambda-cut-a-deux-definitions.md) | `λ_cut` a deux définitions, et la dissipative est mesurable aujourd'hui |
| [ADR-034](adr/ADR-034-la-dissipation-est-un-filtre-passe-bas.md) | La dissipation numérique n'est pas une coupure, c'est un filtre passe-bas dont la loi est connue |
| [ADR-035](adr/ADR-035-le-nombre-de-courant-definition-borne-valeur.md) | Le nombre de Courant : sa définition d'abord, sa borne ensuite, sa valeur en dernier |
| [ADR-036](adr/ADR-036-delta-ne-porte-pas-la-houle-il-porte-l-ecart.md) | δ ne porte pas la houle, il porte l'écart : la réinjection se dissout, le sillage devient le problème |
| [ADR-037](adr/ADR-037-la-dissipation-est-un-allie-pour-la-moitie-de-delta.md) | La dissipation est un allié pour la moitié du contenu de δ, et le dimensionnant pour l'autre |
| [ADR-038](adr/ADR-038-ce-que-les-deux-premiers-cas-de-solveur-ont-appris.md) | Ce que les deux premiers cas de solveur ont appris |
| [ADR-039](adr/ADR-039-un-cas-sans-conditions-de-mesure-ne-classe-personne.md) | Un cas sans conditions de mesure ne classe personne |
| [ADR-040](adr/ADR-040-l-ordre-deux-et-ce-qu-il-deplace.md) | L'ordre deux, et ce qu'il déplace |
| [ADR-041](adr/ADR-041-le-dernier-cas-rouge-etait-rouge-a-cause-de-sa-mesure.md) | Le dernier cas rouge était rouge à cause de sa mesure |
| [ADR-042](adr/ADR-042-l-eponge-mesuree-et-la-borne-de-lambda-cut-rouverte.md) | L'éponge mesurée, et la borne de `λ_cut` rouverte |
| [ADR-043](adr/ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) | Deux lignées ont écrit le même solveur le même jour, et cela vaut moins et plus qu'il n'y paraît |
| [ADR-044](adr/ADR-044-ce-que-l-oracle-croise-peut-dire.md) | Ce que l'oracle croisé peut dire, ce qu'il ne peut pas, et les deux choses qu'il a trouvées |
| [ADR-045](adr/ADR-045-la-saturation-est-un-detecteur-pas-un-filet.md) | La saturation d'état n'est pas un filet, c'est un détecteur de divergence — et il était muet |
| [ADR-046](adr/ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md) | L'éponge en eau dispersive, et la rétractation d'ADR-042 |
| [ADR-047](adr/ADR-047-le-seuil-de-sec-ne-decide-de-rien-de-publiable.md) | Le seuil de sec ne décide de rien de publiable, et la question était mal posée |
| [ADR-048](adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md) | La masse volumique de l'eau est une propriété du milieu, et le cas qui devait l'arbitrer est aveugle |
| [ADR-049](adr/ADR-049-le-filtre-de-contamination-n-est-pas-mal-calibre-il-est-mal-attribue.md) | Le filtre de contamination n'est pas mal calibré, il est mal attribué |
| [ADR-050](adr/ADR-050-le-filtre-de-contamination-est-une-condition-geometrique.md) | Le filtre de contamination est une condition géométrique, et il présuppose l'ordre qu'il sert à mesurer |
| [ADR-051](adr/ADR-051-la-fenetre-de-Hs-passe-a-3072-m-et-le-cas-y-perd-du-pouvoir.md) | La fenêtre de `Hs` passe à 3072 m, et le cas y perd du pouvoir de détection |
| [ADR-052](adr/ADR-052-separer-phase-et-statistique.md) | Séparer la précision de phase et la statistique locale |
| [ADR-053](adr/ADR-053-le-projet-passe-a-la-construction.md) | Le projet passe à la construction, et il commence par W |
| [ADR-054](adr/ADR-054-construire-w-sans-faux-prealable.md) | Construire W sans faux préalable |
| [ADR-055](adr/ADR-055-evenement-impact-versionne.md) | Le premier événement W est un impact versionné |
| [ADR-056](adr/ADR-056-cause-et-journal-impact.md) | La cause relie la prédiction à la confirmation |
| [ADR-057](adr/ADR-057-restauration-et-perte-connue.md) | Restaurer un journal entier et conserver sa perte connue |
| [ADR-058](adr/ADR-058-premier-impact-dispersif.md) | Première expansion dispersive d’un impact, à domaine explicite |
| [ADR-059](adr/ADR-059-impact-regional-sans-repetition.md) | Le support périodique ne devient pas régional en raccourcissant sa durée |
| [ADR-060](adr/ADR-060-candidat-radial-borne.md) | Un candidat radial borné, sans pavage du monde |
| [ADR-061](adr/ADR-061-integration-radiale-limitee.md) | Le candidat radial peut passer à l’intégration limitée |
| [ADR-062](adr/ADR-062-vitesses-et-composition.md) | Les vitesses rendent la composition B+W cohérente |
| [ADR-063](adr/ADR-063-preparation-et-lots.md) | Préparer une fois, publier le lot seulement après succès |
| [ADR-064](adr/ADR-064-bessel-interpole.md) | Bessel interpolé avec réception séparée de l’erreur |
| [ADR-065](adr/ADR-065-requete-commune-b-w.md) | Calculer B et W depuis une requête commune |
| [ADR-066](adr/ADR-066-horizon-et-retention.md) | Séparer horizon numérique et rétention des événements |
| [ADR-067](adr/ADR-067-admission-transactionnelle.md) | Publier une commande avec ses champs, conserver le refus |
| [ADR-068](adr/ADR-068-sauvegarde-du-service.md) | Sauvegarder le service publié et son attente |
| [ADR-069](adr/ADR-069-pression-mobile-et-sillage.md) | Construire le sillage depuis un forçage de pression mobile |
| [ADR-070](adr/ADR-070-pression-localisee.md) | Champ de référence d'une pression gaussienne mobile |
| [ADR-071](adr/ADR-071-noyau-modal-deterministe.md) | Candidat modal à horloge entière |
| [ADR-072](adr/ADR-072-cuisson-gaussienne-reproductible.md) | Recette de cuisson gaussienne V1 |
| [ADR-073](adr/ADR-073-demi-spectre-conjugue.md) | Réduction contrôlée du spectre conjugué |
| [ADR-074](adr/ADR-074-source-de-pression-versionnee.md) | Source candidate de pression versionnée |
| [ADR-075](adr/ADR-075-admission-des-sources-de-pression.md) | Admission bornée des sources de pression |
| [ADR-076](adr/ADR-076-instantane-du-journal-de-pression.md) | Instantané du journal de pression avec attente |
| [ADR-077](adr/ADR-077-requete-mixte-impacts-et-pressions.md) | Requête commune aux impacts et pressions |
| [ADR-078](adr/ADR-078-controleur-de-publication-pression.md) | Contrôleur de publication du champ de pression |
| [ADR-079](adr/ADR-079-horizon-effectif-du-montage-mixte.md) | Horizon effectif du montage mixte |
| [ADR-080](adr/ADR-080-annonce-des-points-du-montage-mixte.md) | Annonce des points du montage mixte |
| [ADR-081](adr/ADR-081-separer-limite-physique-et-limite-numerique.md) | Séparer la limite physique de la limite numérique |
| [ADR-082](adr/ADR-082-nommer-la-borne-qui-refuse.md) | Nommer la borne qui refuse |
| [ADR-083](adr/ADR-083-portee-du-champ-d-impact.md) | La portée d'un champ d'impact, et ce qui la borne |
| [ADR-084](adr/ADR-084-portee-etendue-par-l-asymptotique.md) | Portée étendue par l'asymptotique, bornée par la phase |
| [ADR-085](adr/ADR-085-profils-radiaux-selon-le-domaine.md) | Dimensionner le profil radial au domaine commun |
| [ADR-086](adr/ADR-086-admission-dynamique-de-la-pression.md) | Admission dynamique des sources de pression |
| [ADR-087](adr/ADR-087-sortie-de-saturation-annoncee.md) | La capacité qui résout une attente s'annonce |
| [ADR-088](adr/ADR-088-admission-incrementale-exacte.md) | Admission incrémentale, exacte ou pas du tout |
| [ADR-089](adr/ADR-089-extension-sans-interruption.md) | Étendre sans interrompre |
| [ADR-090](adr/ADR-090-la-condition-d-ordre-reste-et-s-ecrit.md) | La condition d'ordre reste, et s'écrit |
| [ADR-091](adr/ADR-091-admissibilite-annoncee-entre-couches.md) | Annoncer l'admissibilité plutôt que coordonner les couches |
| [ADR-092](adr/ADR-092-generateur-d-impact.md) |  |
| [ADR-093](adr/ADR-093-ou-se-calibre-la-source-d-impact.md) |  |
| [ADR-094](adr/ADR-094-d-ou-vient-la-limite-de-pente.md) |  |
| [ADR-095](adr/ADR-095-ce-que-la-pression-peut-annoncer-de-sa-pente.md) |  |
| [ADR-096](adr/ADR-096-les-deux-champs-disent-la-meme-chose-de-max-slope.md) |  |
| [ADR-097](adr/ADR-097-ce-qui-garde-le-contrat-de-pente.md) |  |
| [ADR-098](adr/ADR-098-trois-causes-trois-noms-dans-le-budget-de-pente.md) |  |
| [ADR-099](adr/ADR-099-b1-trente-deux-composantes.md) |  |
| [ADR-100](adr/ADR-100-spectre-de-fond-et-bande-explicite.md) | Un spectre de fond avec une bande explicite |
| [ADR-101](adr/ADR-101-cuisson-du-fond-spectral.md) | Cuisson explicite du fond spectral |
| [ADR-102](adr/ADR-102-transport-recette-spectrale.md) | Transport de la recette spectrale |
| [ADR-103](adr/ADR-103-mouvement-charge-sillage.md) | Raccorder mouvement et charge prescrits au sillage |
| [ADR-104](adr/ADR-104-emission-progressive-sillage.md) | Émettre le sillage progressivement |
| [ADR-105](adr/ADR-105-profil-radial-b2-soixante-secondes.md) | Profil radial explicite pour le volet B2 à60s |
| [ADR-106](adr/ADR-106-horizon-d-observation-et-duree-de-forcage.md) | L'horizon d'observation n'est pas la durée de forçage |
| [ADR-107](adr/ADR-107-le-domaine-d-un-sillage-se-deduit-de-sa-recette.md) | Le domaine d'un sillage se déduit de sa recette, il ne se déclare pas |
| [ADR-108](adr/ADR-108-pas-de-garde-fou-sans-tolerance-declaree.md) | Pas de garde-fou sans tolérance déclarée |
| [ADR-109](adr/ADR-109-le-repliement-est-une-infidelite-pas-une-faute.md) | Le repliement est une infidélité, pas une faute |
| [ADR-110](adr/ADR-110-une-copie-de-travail-se-ferme.md) | Une copie de travail se ferme, et une branche sans commit unique ne se conserve pas |
| [ADR-111](adr/ADR-111-le-critere-de-bascule-s-exprime-en-profondeur.md) |  |
| [ADR-112](adr/ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md) | La superposition indépendante ne reçoit pas le couplage |
| [ADR-113](adr/ADR-113-fournisseur-differentiel-du-fond.md) | Le fournisseur différentiel de B explicite sa profondeur |
| [ADR-114](adr/ADR-114-source-continue-du-fond-profond.md) | Le résidu continu de B est une accélération à soustraire |
| [ADR-115](adr/ADR-115-differentiel-radial-et-composition.md) | Le différentiel radial conserve sa limite au centre |
| [ADR-116](adr/ADR-116-differentiel-de-pression-forcee.md) | La pression imposée entre dans le champ profond |
| [ADR-117](adr/ADR-117-composition-differentielle-mixte.md) | Composition différentielle mixte |
| [ADR-118](adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md) | Le réseau d'échantillonnage s'ancre sur ses frontières et gradue son pas |
| [ADR-119](adr/ADR-119-le-budget-conjoint-se-borne-par-la-somme.md) | Le budget d'erreur conjoint se borne par la somme ; la loi du maximum n'est pas portable |
| [ADR-120](adr/ADR-120-b4-tolerance-de-deux-pour-cent.md) | B4 : erreur acceptable de 2 % |
| [ADR-121](adr/ADR-121-la-projection-lineaire-et-la-borne-de-composition.md) | Projection linéaire et borne de composition avec résidu |
| [ADR-122](adr/ADR-122-l-ordre-en-amplitude-d-un-vehicule-non-lineaire.md) | L'ordre en amplitude d'un véhicule non linéaire dispersif |
| [ADR-123](adr/ADR-123-le-domaine-de-validite-de-la-superposition.md) | Le domaine de validité de la superposition perturbative |
| [ADR-124](adr/ADR-124-image-budget-et-effets-bornes.md) | Image, budget, puis effets volumiques bornés |
| [ADR-125](adr/ADR-125-budget-image-60hz-deux-ms.md) | Profil initial : 60 images/s, eau2 ms par image |
| [ADR-126](adr/ADR-126-emprise-d-un-impact-visible.md) | L'emprise d'un impact visible se dimensionne par ses coutures |
| [ADR-127](adr/ADR-127-ambition-complete-construction-progressive.md) | Ambition finale complète, construction progressive par versions de plus en plus capables |
| [ADR-128](adr/ADR-128-le-budget-de-pente-borne-les-perturbations.md) | Le budget de pente borne ce que les perturbations ajoutent, pas la mer |
| [ADR-129](adr/ADR-129-chemin-image-de-w-par-table-de-bessel.md) | Le chemin d'image de W radial passe par une table de Bessel précalculée |
| [ADR-130](adr/ADR-130-rendu-j1-sur-gpu-par-un-hote-separe.md) | La version interactive J1 rend l'eau sur GPU, par un hôte séparé |
| [ADR-131](adr/ADR-131-un-depassement-qualifie-une-implementation.md) | Un dépassement de budget qualifie une implémentation ; le budget s'éprouve sur la combinaison des optimisations |
| [ADR-132](adr/ADR-132-domaine-d-image-d-un-sillage.md) | Le domaine d'image d'un sillage se calcule depuis sa recette, et l'hôte l'annonce |
| [ADR-133](adr/ADR-133-le-majorant-de-pente-suit-la-dispersion.md) | Le majorant de pente d'un impact suit la dispersion, et le budget de composition avec lui |
| [ADR-134](adr/ADR-134-l-enveloppe-de-pente-tient-compte-des-directions.md) | L'enveloppe de pente d'un champ de pression tient compte de l'étalement des directions |
| [ADR-135](adr/ADR-135-borne-locale-de-pente-du-champ-prepare.md) | Borne locale de pente du champ préparé |
| [ADR-136](adr/ADR-136-borne-locale-d-ordre-deux-a-hessienne-signee.md) | Borne locale de pente d'ordre deux, à Hessienne signée |
| [ADR-137](adr/ADR-137-coupure-spectrale-de-la-borne-locale.md) | Coupure spectrale de la borne locale de pente |
| [ADR-138](adr/ADR-138-le-budget-de-pente-tient-compte-de-la-position-relative.md) | Le budget de pente tient compte de la position relative des impacts |
| [ADR-139](adr/ADR-139-volume-et-plan-oriente-des-contenants.md) | Le plan orienté se déduit du volume de la géométrie du contenant |
| [ADR-140](adr/ADR-140-restauration-du-graphe-V.md) | Restaurer les écarts de V et ses restes de débit |
| [ADR-141](adr/ADR-141-surface-linearisee-et-coefficients-temporels.md) | Surface linéarisée et coefficients temporels de δ |
| [ADR-142](adr/ADR-142-composition-sur-l-union-des-emprises.md) | Composition mixte sur l'union des emprises, sous plancher certifié |
| [ADR-143](adr/ADR-143-la-pression-f32-converge-a-sa-precision-representable.md) | La pression f32 de δ s'arrête à sa précision représentable, acceptée à la tolérance physique S199 |
| [ADR-144](adr/ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md) | La tolérance physique de la projection est une condition d'acceptation, sur les lignes franches |
| [ADR-145](adr/ADR-145-i-06-pour-l-hote-graphique.md) | I-06 pour l'hôte graphique : tenue par notre code, comptée et publiée pour la pile |
| [ADR-146](adr/ADR-146-l-ecriture-disjointe-est-inconditionnellement-deterministe.md) | L'écriture disjointe est inconditionnellement déterministe, et c'est elle qu'on parallélise |
| [ADR-147](adr/ADR-147-la-multigrille-est-un-repli-de-precision.md) | La multigrille est un repli de précision, pas le solveur ordinaire |
| [ADR-148](adr/ADR-148-filtrage-spectral-image.md) | Filtrer les amplitudes de l'image selon le pas projeté |
| [ADR-149](adr/ADR-149-premier-raccordement-volumique.md) | Premier raccordement volumique B/W→δ à surface imposée |
| [ADR-150](adr/ADR-150-correction-de-divergence-couplee.md) | Corriger le défaut de divergence sur la vitesse couplée, une fois, au plancher |
| [ADR-151](adr/ADR-151-affinage-au-pas-fixe-et-travail-compte.md) | L'affinage de divergence vaut aussi pour le pas à couvercle fixe ; le rapport compte tout le travail |
| [ADR-152](adr/ADR-152-surface-mobile-couplee.md) | Surface mobile couplée : géométrie totale, hauteur perturbative, bande du fond |
| [ADR-153](adr/ADR-153-affinage-en-mode-mobile-couple.md) | Affiner la divergence au plancher dans le pas couplé mobile |
| [ADR-154](adr/ADR-154-prolongement-borne-du-fond.md) | Prolonger le fond au-dessus du plan moyen : vitesse horizontale constante, par mode |
| [ADR-155](adr/ADR-155-queue-spectrale-en-pentes-par-pixel.md) | La queue du spectre de B se rend en pentes par pixel |
| [ADR-156](adr/ADR-156-mer-multimodale-et-etalement.md) | Une mer à plusieurs systèmes, chacun avec sa loi d'étalement directionnel |
| [ADR-157](adr/ADR-157-queue-d-equilibre-et-vagues-pointues.md) | Queue d'équilibre en f⁻⁴ et vagues pointues de Lagrange |
| [ADR-158](adr/ADR-158-rugosite-ajustee-a-cox-munk.md) | Rugosité ajustée à Cox–Munk : coupure de la queue et modulation par la bande |
| [ADR-159](adr/ADR-159-requete-de-jeu-sous-cwm.md) | La requête de jeu suit la surface rendue sous CWM |
| [ADR-160](adr/ADR-160-vent-parametre-de-scene.md) | Le vent est un paramètre de la scène, calibré à l'œil |
| [ADR-161](adr/ADR-161-reflets-de-la-queue-non-resolue.md) | Filtrer les reflets de la queue non résolue |
| [ADR-162](adr/ADR-162-ciel-precalcule-des-reflets.md) | Ciel précalculé des reflets (candidat rejeté, remplacé par ADR-163) |
| [ADR-163](adr/ADR-163-sommes-des-ondes-filtrees.md) | Regrouper les ondes filtrées sans changer le reflet |
| [ADR-164](adr/ADR-164-relaxation-hauteur-perturbative.md) | Relaxation de la hauteur perturbative dans l’éponge mobile |
| [ADR-165](adr/ADR-165-bande-du-fond-aux-frontieres.md) | Flux de bande du fond aux frontières latérales |
| [ADR-166](adr/ADR-166-quadrature-lineaire-de-la-bande.md) | Quadrature linéaire de la bande du fond |
| [ADR-167](adr/ADR-167-multigrille-du-mode-mobile.md) | La multigrille préconditionne le mode à surface mobile |
| [ADR-168](adr/ADR-168-premier-rendu-de-delta.md) | Premier rendu de δ : bande couplée rejouée dans l'afficheur |
| [ADR-169](adr/ADR-169-depart-depuis-la-pression-publiee.md) | Les pas mobiles partent de la pression publiée |
| [ADR-170](adr/ADR-170-les-trois-poids-sont-bornes.md) | Les trois poids de l'ordonnanceur sont bornés |
| [ADR-171](adr/ADR-171-les-seuils-d-activation-appartiennent-au-profil.md) | Les seuils d'activation appartiennent au profil |
| [ADR-172](adr/ADR-172-candidat-pression-residente-gpu.md) | Candidat de pression résidente GPU dans l'hôte |
| [ADR-173](adr/ADR-173-le-candidat-de-pression-ne-fournit-qu-un-depart.md) | Un candidat de pression externe ne fournit qu'un départ (voir A295) |
| [ADR-174](adr/ADR-174-arbitrages-du-2026-09-19.md) | Arbitrages de l'utilisateur du 2026-09-19 : machine de référence, temps de l'eau au service de l'objectif, v1, ordre |
| [ADR-175](adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) | Architecture d'exécution de δ en 3D : production résidente sur GPU à travail borné, référence CPU pour la réception |
| [ADR-176](adr/ADR-176-asymetries-de-la-surface-rendue.md) | Les asymétries de la surface rendue : second ordre en bande étroite par système et modulation de la queue retardée, dans le rendu seul |
| [ADR-177](adr/ADR-177-couleur-du-corps-d-eau-derivee-de-ses-sources.md) | La couleur du corps d'eau se dérive de ses sources (Pope & Fry 1997, Morel 1974) ; le gain d'échelle est nommé pour ce qu'il est, un substitut d'irradiance de ciel |
| [ADR-178](adr/ADR-178-strategie-en-trois-systemes-physiques.md) | Stratégie en trois systèmes physiques : A stabilisé, B volumique 3D, C couplage ; validité physique avant temps réel ; budget mesuré mais non opposable pendant la construction ; ordre en sept lots |
| [ADR-179](adr/ADR-179-tolerances-de-conservation-et-grandeur-restituee.md) | Tolérances de conservation T1/T2/T3 tranchées ; la grandeur restituée est le **flux sortant**, jamais l'activité de l'éponge ni son net ; ce que W ne peut pas porter se déclare perdu |
| [ADR-180](adr/ADR-180-retour-delta-w-et-conservation-du-volume.md) | Premier transfert δ → W limité aux tests ; trois catégories publiées séparément et le non-transféré **registré**, jamais dit restitué ; le receveur du volume net se cherche dans **V ou le niveau moyen de B** avant toute primitive nouvelle de W |
| [ADR-181](adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md) | Le volume net va à **V ou à un niveau moyen régional de B** — jamais à un niveau global arbitraire ; T1 devient un critère à **quatre grandeurs** sans chiffre avant démonstration, éprouvé sur une erreur volontaire ; le transfert δ → W **n'est pas validé** ; ordre A → E du lot 2 |
| [ADR-182](adr/ADR-182-criteres-de-conservation-actes-et-ordre-b.md) | C1, C2 et C3 actés sous conditions : le plancher reste une **loi observée** et non une borne démontrée, le cas ouvert reste **non conforme à C2**, et les trois s'appliquent **ensemble** ; ordre B autorisé, A305 maintenu ouvert et parallèle |
| [ADR-183](adr/ADR-183-essai-oblique-phase-a-distance-et-ordre-c.md) | L'essai oblique se fait **à la frontière** et non dans la seule primitive ; l'oracle de phase à dix longueurs d'onde est **une autre physique**, et un défaut de phase ne se rattrape ni par l'amplitude ni par un décalage ; l'ordre C **attribue** chaque écart à la primitive, au raccord ou à δ ; le transfert est **partiel** jusque-là |
| [ADR-184](adr/ADR-184-seconde-representation-en-parallele.md) | La **seconde représentation** de surface libre — plusieurs couches d'eau sur une même verticale — avance **en parallèle** du lot 2, par sessions alternées ; elle commence par une **comparaison chiffrée** (particules sur grille, SPH, surface implicite) et son choix revient à l'utilisateur |
| [ADR-185](adr/ADR-185-ordre-d-receveur-sous-i15.md) | L'ordre D est ouvert ; un volume issu de δ n'entre **jamais** dans l'état répliqué de B ni de V (I-11, I-15) : en eau ouverte, le receveur est le niveau de B **tel que ce client le représente**, porté par une région déclarée adossée à la ligne ; en contenant, δ s'asservit à V ; rien ne se restitue sans reçu |
| [ADR-186](adr/ADR-186-apic-seconde-representation.md) | **APIC** est la seconde représentation de surface libre — décision de l'utilisateur sur la comparaison de S318 : particules sur la **même grille** et la même pression que δ, là où plusieurs couches d'eau tiennent sur une verticale ; les colonnes restent la représentation par défaut |
| [ADR-187](adr/ADR-187-methode-refondue-s321.md) | La méthode refondue sur l'analyse de S321 : rituel en deux parties, lecture à froid bornée, `EN-COURS` limité à la session en cours, **protections plutôt que leçons**, contrôles plutôt que consignes, réceptions reproductibles, un fil une preuve — sur demande de l'utilisateur ; le périmètre et l'ordre des lots inchangés |
| [ADR-188](adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md) | Le **lot 3** prend la place du lot 2 dans l'alternance d'ADR-184 tant que l'ordre E est bloqué par A289 — décision de l'utilisateur ; la voie d'A289 et la sauvegarde restent à trancher |
| [ADR-189](adr/ADR-189-la-v1-d-abord.md) | **La v1 d'abord** — *« continue, jusqu'à la v1 »* : les sessions suivent les lots 3 et 4 jusqu'à la porte D ; l'alternance avec le lot 5 est suspendue jusque-là (interprétation à confirmer) ; arrêt sur toute décision qui revient à l'utilisateur |
| [ADR-190](adr/ADR-190-apres-la-v1-la-liste-entiere.md) | **Après la v1, la liste entière** — *« Continue, après la V1 ton objectif seras de completer entièrement la to do liste »* : jusqu'à la v1 rien ne change ; ensuite l'objectif est la liste du projet fini, ses 120 points validés au périmètre final, rangés par dépendance dans la feuille de route ; le lot 5 reprend après la v1 entière ; ce qu'une session ne peut valider seule se demande, ne se retire pas |
| [ADR-191](adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md) | **Le rendu réaliste, un module du moteur** — *« pas assez réaliste »* : l'architecture de l'eau ne change pas ; le rendu final est celui du moteur maison, à construire ; nous en écrivons la part de l'eau en module, depuis l'afficheur ; une session de rendu alterne avec une session de physique ; remplace ADR-178 D2 en ce qu'il arrêtait le rendu |
| [ADR-192](adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md) | **Le rendu de l'eau dans Godot 4** — *« rendu toujours pas convaincant »* : Godot 4 (présent sur le poste) plutôt qu'Unreal ou notre rendu ; premier pas, la mer de B rendue dans Godot et jugée sur images ; ensuite l'intégration native par GDExtension ; l'afficheur reste le banc ; remplace ADR-191 D2 |

## Travail et historique

[Méthode](../notes/METHODE.md) · [Plan courant](../notes/EN-COURS.md) · [Leçons](../notes/LECONS.md).

Les anciens récits de cet index restent dans Git à `dfd1507`. Refonte S227 : ne plus y ajouter
les comptes rendus déjà présents au journal. L’inventaire se recalcule avec
`python outils/etat_projet.py` ; `--check` vérifie navigation, plafonds et, **depuis S309**, que le battement du jeton n'est pas dans le futur (L237, A303). Les images locales de banc (ADR-124) se regardent avec
`python outils/apercu_ppm.py <image.ppm>`, qui écrit un PNG à côté du PPM. Une image de mer se
**mesure** avec `python outils/cible_image.py [--horizon=<y>] <image.ppm>…` (S308 : horizon,
histogramme, couleur, contraste local, fraction claire, chaque grandeur déclarée comparable ou
non) ; `python outils/courbe_tonalite.py <rendu_sans_courbe.ppm>` cherche une courbe de tonalité
contre ces cibles. **Comparer deux rendus demande `--horizon` forcé** : la détection automatique
se trompe en silence quand le ciel porte un fort gradient (A301).
