# Liste du projet fini — ce que le système d'eau doit avoir et savoir faire

**Demandée par l'utilisateur en S255, le 2026-09-16.** C'est la liste de contrôle du projet
**terminé**, avec l'ambition complète d'[ADR-127](adr/ADR-127-ambition-complete-construction-progressive.md).
Elle ne se limite pas à ce qui est déjà construit. Les points viennent des
[intentions d'origine](sources/systeme_eau_architecture_globale.md), des
[zones ouvertes](sources/systeme_eau_zones_ouvertes_et_decisions_a_valider.md), des couches
d'[ADR-001](adr/ADR-001-decomposition-en-couches.md), des [invariants](01_INVARIANTS.md), des
spécifications, des [cas canoniques](validation/CAS-CANONIQUES.md) et des
[bancs](validation/PLAN-BENCHMARK.md).

**Objectif des sessions après la v1** (décision de l'utilisateur, S351,
[ADR-190](adr/ADR-190-apres-la-v1-la-liste-entiere.md)) : ses 120 points validés, chacun sur son
périmètre final ; l'ordre reste celui de la feuille de route (§3 ter), qui le tire du
[registre des dépendances](registres/DEPENDANCES-LISTE.md). **Depuis S475, la condition de fin du système
de l'eau** (décision de l'utilisateur, [ADR-218](adr/ADR-218-le-systeme-de-l-eau-complet.md)) : *« la feuille
to do list devra être validée à 100% pas moins mais plus possible ou changement durant le processus »* —
119 points (5.11 hors du périmètre), dans l'ordre du [plan de complétion](registres/PLAN-COMPLETION-S475.md) ; **120 depuis
S476** — 13.4, l'eau dans le jeu DyingStar, ajouté ([ADR-219](adr/ADR-219-reponses-du-2026-10-04.md)).

**Remplissage à la demande de l'utilisateur.** Chaque remplissage met à jour les états touchés et la
ligne « État au » ci-dessous. La trajectoire et l'ordre des travaux restent dans la
[feuille de route](FEUILLE-DE-ROUTE.md) ; les preuves détaillées, dans les documents liés. Ici, on
coche et on pointe, sans recopier (L137).

## Comment lire un point

- `[x]` **validé** : construit et reçu par une preuve publiée, sur le **périmètre final** du point.
- `[ ]` **partiel** : une partie est construite et reçue ; le texte dit laquelle, et ce qui manque.
- `[ ]` **absent** : rien de construit. « Conçu » signale qu'un ADR ou une spécification existe.

Un point n'est jamais validé sur un banc isolé, un véhicule d'essai ou une seule scène quand son
énoncé vise le système. Un point partiel ne dit rien de la difficulté de ce qui reste.

**État au S475, 2026-10-04** — actualisation complète (ADR-218 D5) : le journal de S351 à S474 relu
contre la liste. **Aucun point n'a changé de catégorie depuis S408** (4.10 passé à partiel) : les
sessions S409–S474 (la campagne du solveur 3D sur la carte, la surface continue, la v1, Godot, le banc
visuel, le type d'eau) ont fait avancer des points déjà partiels — reportés par les lots de registres —
sans en amener un à son périmètre final. **3 validés, 73 partiels, 44 absents dont 5.11 hors du
périmètre : 116 points ouverts sur 119.** Le constat et l'ordre pour la suite : le
[plan de complétion](registres/PLAN-COMPLETION-S475.md) §1–2.

**État au S350, 2026-09-24** — actualisation demandée par l'utilisateur à la reprise de S350
(« mets à jour le document de la to do list »). **Seuls les points que S309–S349 ont réellement
bougés sont retouchés**, après relecture de chaque entrée du journal contre la liste ; les autres
gardent leur état et sa date. Chaque état modifié cite la preuve qui le modifie. Le décompte porte
sur le périmètre final, pas sur le nombre de correctifs ou de tests. Précédente actualisation
complète : S309, 2026-09-20, avec la stratégie en trois systèmes
([ADR-178](adr/ADR-178-strategie-en-trois-systemes-physiques.md)).

**Où cette liste rejoint les trois systèmes** : la répartition de ses 120 points entre A, B, C et
« hors des trois » est dans [TROIS-SYSTEMES-S308](registres/TROIS-SYSTEMES-S308.md) §8. Elle n'est
pas recopiée ici (L137).

---

## 1. Socle et architecture

- [ ] **1.1 Eau = somme de quatre couches B, W, δ, V, dans le code** — *partiel* : B, W, un δ
  candidat 2D et le noyau V existent ; B+W sont composés par le cœur (S214, S236) ; B/W sont
  raccordés à δ en 2D (S250–S254) puis **en 3D** (S297, S302) ; somme rendue B+δ en direct
  depuis la surface publiée ([S302](validation/SCENE-DELTA3D-S302.md)). Manque l'articulation
  V↔δ ; le volume net a un receveur **local** depuis S317 ([ordre D](validation/RESTITUTION-S317.md)) ; le **retour de δ vers W** existe en 3D sur la référence
  CPU, qualifié propriété par propriété en S316 ([ordre C](validation/ORDRE-C-S316.md)), partiel.
- [x] **1.2 Le cœur est une bibliothèque sans dépendance moteur** — *validé* : `water-core` n'a
  aucune dépendance (ADR-020, ADR-028).
- [ ] **1.3 Interfaces de SPEC-004** (solveur δ, solveur W, fond, solides, services d'hôte) —
  *partiel* : allocateur, tâches, horloge, journal d'hôte, `BackgroundSample`, `Volume`.
  Manquent l'interface des solides (SPEC-004 §7) — δ reçoit une paroi par distance signée et le cœur
  a un corps rigide depuis S329–S332, **hors** de cette interface
  ([preuve](validation/CORPS-RIGIDE-S331.md) §4) —, la bathymétrie et le GPU côté cœur.
- [ ] **1.4 Point d'entrée unique, orchestrateur des régimes** (`WaterSystem`, SPEC-004 §3) —
  *partiel* **depuis S278** : `scheduler.rs` décide quels domaines vivent et avec quel budget —
  sac à dos d'ADR-012 §1, hystérésis et durées de vie d'ADR-013 §5, poids bornés par ADR-170 ;
  éprouvé ([ORDONNANCEUR-S278](validation/ORDONNANCEUR-S278.md)), et **branché en S279 sur la
  bande δ de l'afficheur**, qui s'éteint et se rallume toute seule
  ([ORDONNANCEUR-S279](validation/ORDONNANCEUR-S279.md), ADR-171). **S344** : il arbitre **deux
  domaines δ 3D réels** sous un budget, et l'exclusion absorbante, reproduite en 3D, est levée par
  l'oubli des coûts — dans chaque hôte, pas dans le cœur ([preuve](validation/ARBITRAGE-3D-S344.md)
  §3) ; **S349–S350** : un domaine **se déplace et se redimensionne**, au bit, décidé par l'hôte
  (§5–6) ; **S351** : le **rang 1** de la dégradation — l'ordonnanceur rend une échelle, l'hôte en fait
  l'emprise (§7). Manquent le point d'entrée `WaterSystem` lui-même, les rangs 2 à 7, la position des
  domaines décidée par l'ordonnanceur, l'oubli et l'estimateur de coût dans le cœur.
- [ ] **1.5 Grille 3D de référence stable** : adressage, zones actives, échanges client/serveur —
  *absent*, conçu (ADR-006).
- [ ] **1.6 Cellules, domaines et solveurs distincts, niveaux d'activité des cellules** — *absent*,
  conçu (ADR-006).
- [ ] **1.7 Horloge de simulation entière et phases déterministes** (ADR-003) — *partiel* : temps
  entier en µs, phases en virgule fixe ; déterminisme reçu localement, pas entre plateformes. **Depuis S476** (un seul PC,
  [ADR-219](adr/ADR-219-reponses-du-2026-10-04.md) D2) : « entre plateformes » se lit entre les chemins d'exécution de ce PC
  (processeur et carte graphique, Vulkan et DirectX 12, compilations, simple et double précision).
- [ ] **1.8 Référentiels, précision f32 locale, `g_eff` injectée** (ADR-002, I-07, I-08) — *partiel* :
  positions monde et ancres locales, `g_eff` par volume. Manquent les référentiels mobiles et la
  planète sphérique.

## 2. Grandes masses d'eau et fond (B)

- [ ] **2.1 Mer et océan : état de mer spectral déterministe, sans état par cellule** — *partiel* :
  JONSWAP cuit et reproductible (ADR-100/101), 32 composantes, phases GPU ; queue jusqu'à 5,5 cm
  rendue en pentes (S256), étalement directionnel `cos^2s` (S259), queue d'équilibre et vagues
  pointues CWM : rugosité et pointe des pentes dans les mesures de Cox–Munk (S260, `--vagues`),
  ajustées à leurs valeurs centrales (S261, `--modulation`) ; vent de scène, rugosité conforme à
  Cox–Munk à chaque vent (S263, `--vent`).
  Requête CWM cohérente avec l'image à 0,30 mm (S262, A288 close, ADR-159).
  Manquent l'anisotropie, l'asymétrie des pentes, B1 complet et plusieurs régions ;
  le branchement de la requête au jeu reste au point 10.9.
- [ ] **2.2 Houles longues, mers croisées, marée, niveau moyen variable** — *partiel* : mer à plusieurs
  systèmes avec étalement `cos^2s` (ADR-156, S259), houle de 225 m dans la scène déclarée `--houle`,
  verdict R3 attendu. Manquent la marée, le niveau moyen variable, des houles issues d'une météo et
  l'adoption par défaut.
- [ ] **2.3 Lacs** : niveau moyen, apports, courants faibles — *absent*.
- [ ] **2.4 Rivières** : débit macroscopique qui contraint les perturbations locales — *absent*.
- [ ] **2.5 Canaux** — *absent*.
- [ ] **2.6 Courants macroscopiques à niveau de détail propre**, du vecteur au champ 3D — *partiel* depuis S513 (conçu par ADR-011) : les
  niveaux **C0** (le vecteur de surface) et **C2** (le profil vertical) derrière la requête de l'eau (`CurrentWater`) — les vagues advectées
  (la période de rencontre exacte, `λ/(c + U)`), le profil exact, un corps traîné qui dérive à 0,4 % de l'analytique
  ([preuve](validation/COURANT-S513.md)) ; **S534 : C1**, le champ 2D régional — une grille bilinéaire, sa pente (`g∇η = −(u·∇)u`) et
  son accélération : un corps neutre tient une rotation solide à 0,025 % ([preuve](validation/COURANT-C1-S534.md)). Manquent la production
  et le flux des grilles C1, l'advection des vagues par un courant variable, C3 (le champ local de δ), les rivières, les canaux.
- [ ] **2.7 Bathymétrie** : hauts-fonds, effet sur les vagues avant la zone physique — *partiel* depuis S362 : la
  **référence** linéaire dans le cœur — profondeur finie, levée, réfraction de Snell sur isobathes droites, phase
  intégrée, profondeur de déferlement (McCowan) —, tenue contre Fenton–McKee et la levée minimale des manuels
  ([preuve](validation/BATHYMETRIE-S362.md)) ; **S364** : l'entrée **dans B**, par composante, décidée
  ([ADR-196](adr/ADR-196-la-bathymetrie-entre-dans-b-par-composante.md)) et construite pour les isobathes droites —
  tables cuites, η à 0,13 mm de la référence, B au bit au large, requête en O(1) (§5) ; **S522** : **W en profondeur uniforme** — le
  nombre d'onde effectif `k·tanh kh` dans la pression de W, le chemin profond au bit, le sillage à `Fr_h` = 0,9 à 2,0 % de la référence
  par 5 m de fond (121 % de la profonde) ([preuve](validation/W-PROFONDEUR-S522.md)). Manquent la bathymétrie 2D (et sous W) et la
  diffraction des hauts-fonds isolés, la marée, la dissipation au déferlement, la non-linéarité peu profonde (A234), et
  les autres chemins de B jusqu'à la scène de Godot.
- [ ] **2.8 Précalcul côtier et météo** (SPEC-005 §6) — *absent* ; la météo **à la fin** (ADR-197 D5), un système
  complet, aussi poussé que l'eau, le premier après elle ; l'eau en consomme les entrées ([ADR-203](adr/ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) D1, D5).
- [ ] **2.9 Dérivées du fond pour les couches volumiques**, sous et au-dessus du plan moyen —
  *partiel* : B reçu en eau profonde uniforme (ADR-113, S177 ; ADR-154, S254). Manquent la
  profondeur finie et la bathymétrie.

## 3. Ondes propagatives (W)

- [ ] **3.1 Anneaux d'impact dispersifs** (objet qui tombe) — *partiel* : impacts radiaux, table
  de Bessel, générateur calibré, admission ; horizon reçu 56 s, eau profonde ; **S528 : l'eau peu profonde** —
  `RadialImpact::new_in_depth`, à 0,44 % d'une propagation FFT exacte par 1 m de fond (94 % pour l'eau profonde), les bornes de pente
  sûres ([preuve](validation/ANNEAUX-PROFONDEUR-S528.md)). Manquent la gerbe (point 4.12), une profondeur variable sous l'anneau et
  l'hôte qui choisit la profondeur.
- [ ] **3.2 Sillages de bateaux**, trajectoires et vitesses quelconques, eau profonde et peu
  profonde (C07) — *partiel* : source de pression mobile par tronçons, scène à trois sillages ;
  domaine honnête de 89 m et 18,5 s (ADR-132). **S519 : C07 en eau profonde passe** — le sillage de W à 0,33 % de la théorie
  linéaire exacte en temps, son angle à 19,98° par un instrument éprouvé sur la théorie seule
  ([preuve](validation/C07-PROFOND-S519.md)). **S523 : C07 peu profond passe à `Fr_h` = 1,43** (W en profondeur uniforme, S522 :
  0,4 % de la théorie, l'angle à 0,5° du coin de Mach) ; à 2,14 l'instrument prend le bruit ([preuve](validation/C07-PEU-PROFOND-S523.md)).
  **S525 : la résonance** — W à 0,03 % de la théorie, la pente −0,544 sur le régime permanent (0,3–0,7) ; l'assertion de C07 corrigée
  (−0,72 sur quatre points à durée finie, la théorie même) ([preuve](validation/C07-RESONANCE-S525.md)) ; **S527 : l'angle à `Fr_h` = 2,14**
  (27,00° pour 27,83°, un instrument éprouvé sur la référence bruitée) — **C07 passe entier** ([preuve](validation/C07-PLANCHER-S527.md)).
  Manquent les durées longues.
- [ ] **3.3 Explosions de surface et sous-marines** — *absent*.
- [ ] **3.4 Tsunamis** : propagation macroscopique, puis raffinement à la côte — *absent*.
- [ ] **3.5 Déferlement** (polyligne de SPEC-006 §6) — *absent*.
- [ ] **3.6 Réfraction bathymétrique des ondes** — *absent*.
- [ ] **3.7 Événements horodatés, journaux, instantanés et restauration avec perte connue** —
  *partiel* : impacts et pression versionnés, restaurés (ADR-055 à 068, 076). Manque le transport
  réseau.
- [ ] **3.8 Composition B+W sans refus sur toute scène** (budget de pente sur pente réelle, I-18) — *partiel* : scène
  représentative admise aux 161 instants (ADR-142, S236). Manquent le choix du mode par l'hôte
  autoritaire (A271) et la saturation par la pression seule (A261).
- [ ] **3.9 Couches W fournies au-dessus du plan moyen pour δ** — *absent* (A286).

## 4. Simulation volumique locale (δ)

- [ ] **4.1 Solveur volumique 3D à surface libre** — *partiel*, **et la 3D existe depuis S297**.
  MAC **x-y-z**, surface **fonction hauteur** à fluide fantôme, pression scindée. Référence CPU
  contre HOS à 0,148 % / 0,178 % et invariance transverse à un ulp
  ([S297](validation/DELTA3D-COUPLEE-S297.md)) ; cas limites 2D reproduits à 1,19·10⁻⁷ m
  ([S298](validation/DELTA3D-FOND-REEL-S298.md)) ; production GPU reçue étage par étage
  ([S301](validation/DELTA3D-PAS-GPU-S301.md)) ; cuve fermée à 3·10⁻⁷ m pour 3 mm exigés
  ([S305](validation/CUVE-GPU-S305.md)) ; la production sur les trois cas de cuve, et **la porte B reçue** (S340,
  [§9](validation/CUVE-GPU-S305.md)). **Manque le périmètre final** : la surface est un graphe,
  donc ni cavité, ni jet, ni déferlement (4.16) — seconde représentation, lot 5 d'ADR-178.
- [ ] **4.2 Plusieurs domaines actifs simultanés** — *partiel* **depuis S351** : deux domaines δ 3D de
  production **servis ensemble** sous un budget de 5 ms qui n'en tient pas deux entiers — le focal entier,
  l'autre rétréci au rang 1 —, 615 images de suite, au banc ([preuve](validation/ARBITRAGE-3D-S344.md) §7).
  Manquent davantage de domaines, leurs interactions (4.9 : fusion et séparation en référence depuis S396) et l'afficheur.
- [ ] **4.3 Subdivision adaptative anisotrope, blocs épars** épousant la forme utile (B5) — *partiel* **depuis S386** : la
  **colonne graduée** ([ADR-208](adr/ADR-208-la-colonne-graduee.md)), subdivision anisotrope verticale de la référence, reçue au
  pas linéaire — 11 inconnues de pression sur 28 pour une colonne de 7 m à 25 cm, dispersion calculée et tenue
  ([preuve](validation/COLONNES-HAUTES-S386.md)) ; **S387** : au pas mobile, gardée par la course de la surface, elle suit sa
  dispersion calculée (§5) — elle sert l'eau calme des contenants, pas la haute mer (course de 4,8 m sous la mer de la porte B).
  **S401** : les **blocs épars** en référence — un domaine est un ensemble de blocs dans une fenêtre, le pas mobile sur
  l'ensemble, dont le bord se comporte comme celui de la boîte (un rectangle à un ulp du dense) ; l'ensemble suit sa
  perturbation, à 0,26–0,66 mm du domaine entier ([preuve](validation/DOMAINE-EPARS-S401.md)). **S404** : le **pas couplé**
  sous l'ensemble, en mer — un rectangle à un ulp de son dense sous la houle, l'épars qui suit à 0,14 mm du domaine entier
  ([preuve](validation/MER-EPARS-S404.md)). Manquent le stockage compact (pool de blocs), le fond coupé sous l'ensemble, la carte
  (C3, C8).
- [ ] **4.4 Profondeur adaptative**, domaine qui suit un objet qui coule — *absent*.
- [ ] **4.5 Création, croissance, réduction et disparition visuellement gratuites** (I-12) —
  *partiel* : naissance à zéro reçue sous fond couplé (S251, S253) ; en 3D, un domaine qui renaît
  repart de δ = 0 (S344), et **ce qui entre dans un domaine qui se déplace naît au repos**, au bit
  (S349) ; **S350 : croissance et réduction** d'un domaine 3D, l'état gardé au bit dans le
  recouvrement, ce qui entre au repos ([preuve](validation/ARBITRAGE-3D-S344.md) §5–6) ; **S351** : elles
  servent le rang 1, et leur prix est mesuré — une réduction retire jusqu'à 11,3 cm de δ (§7). **S402** : un **changement de
  niveau** sans saut — 0,1 à 2 mm d'un pas à l'autre — par transfert d'état ([ADR-210](adr/ADR-210-changer-de-niveau-par-transfert-d-etat.md)) ;
  la disparition progressive d'ADR-005 §5 mesurée — fondu de 0,5 s, sans saut, mais le contenu perdu
  ([preuve](validation/NIVEAUX-S402.md)). **S404** : le changement de niveau d'un domaine **épars**, en mer — au bit de son dense ;
  l'ensemble n'ajoute rien au prix du niveau ([preuve](validation/MER-EPARS-S404.md)). Manquent la transduction de la disparition
  vers W et tout verdict visuel sur ces passages.
- [ ] **4.6 Entrée des vagues de B/W dans le domaine** — *partiel* : source volumique, surface
  mobile couplée, fond B prolongé (ADR-149 à 154). Flux de bande aux frontières reçus
  sur courant/niveau uniformes (S270, [preuve](validation/FOND-TRAVERSANT-S270.md), ADR-165).
  Démarrage d’une houle progressive contrôlé à 0,58 % à la maille fine
  ([S271](validation/HOULE-PROGRESSIVE-S271.md)), pas réel éprouvé à petit pas.
  **S272 : évolution sur 2 s refusée** contre oracle indépendant d'ordre deux ;
  **S273 : quadrature des bandes corrigée** (ADR-166). **S274 : l'écart brut vaut 5,8 µm
  rms sur une vague de 1 cm** et suit la dispersion de Stokes que l'oracle n'a pas ; le
  coefficient d'ordre deux est à 0,88–1,16 %, mais sa réception au budget d'ADR-120 reste à
  terminer (2,26–2,43 %), requise avant une mer de cambrure ≥ 0,08
  ([usage](validation/HOULE-USAGE-S274.md)).
  **S297–S302 : l'entrée existe en 3D** — fond B+W sommé aux faces MAC x/y/z, `eta` identique au
  bit verticalement, et une onde de 65 cm traverse un domaine de 30 × 28 m sur la mer étalée
  ([S302](validation/SCENE-DELTA3D-S302.md)) ; une onde née d'un point traverse la mer de R14 et s'y déforme,
  jugée convaincante (R16, S340).
  Manquent la houle progressive traversante reçue sur une durée utile, les frontières
  générales du total et W au-dessus du plan moyen ; B4 reste partiel. **Depuis S310, la masse
  qui entre et sort est comptée** — bande et éponge, au plancher du schéma, référence CPU
  ([preuve](validation/BILAN-MASSE-S310.md)) ; quantité de mouvement et énergie n'y sont publiées
  que comme états, sans bilan (ADR-179 D7), et le compteur n'existe pas sur la carte.
- [ ] **4.7 Frontière sans réflexion ni rupture visible** (C05) — *partiel* : éponge quadratique
  sur la vitesse (S250) et relaxation de hauteur reçue (S268, ADR-164). Effet du bord
  absorbant 0,14–0,16 % sur un paquet à fond nul, par différence contrôlée aux domaines
  longs ([S269](validation/REFLEXION-PAQUET-S269.md)). Manquent les autres régimes,
  la transparence générale et C05 sur le système ; la mesure brute S269 reste refusée.
  **En 3D**, l'éponge et les bandes existent sur les quatre côtés ; R11 a jugé le raccord invisible ;
  R15 a vu la coupure au bord d'une scène de la porte D, que S337 a levée — éponge aussi dans le mode
  linéaire, fondu de composition — : « Plus de coupure » ([preuve](validation/PORTE-D-S333.md) §8).
  **La réflexion est chiffrée depuis S311** : ≤ 1,5·10⁻⁶ en énergie pour 1 % admis, séparée par sens
  en S316, sur un cas contrôlé unidirectionnel ([preuve](validation/SORTIE-DELTA-S311.md)). Mais le
  bord de δ est une **paroi** et l'éponge **efface** ce qu'elle absorbe — 10,2 % du contenu
  perturbatif par seconde sur une scène couplée ([S310](validation/BILAN-MASSE-S310.md)) ; hors du
  transfert d'essai de 4.8, rien n'en revient dans W. **S404** : le bord d'un **ensemble épars** absorbe comme celui de la boîte —
  l'éponge mesurée depuis lui ; sans elle, l'épars qui suit s'écarte du domaine entier de 0,21 mm au lieu de 0,14
  ([preuve](validation/MER-EPARS-S404.md)). En 3D manque la réflexion d'un front oblique ou d'une scène quelconque.
- [ ] **4.8 Sortie des perturbations vers W** (transduction δ→W, W local cosmétique ; coupure W–δ
  et `λ_cut` de B2) — *partiel* : le chemin existe depuis S312 sur la référence CPU — ligne de
  contrôle, identification, train orienté de W (S314) — et **S316 l'a qualifié** propriété par
  propriété : primitive exacte, raccord à un degré et 2 à 4 % de spectre près, le reste à δ
  ([ordre C](validation/ORDRE-C-S316.md)). Le volume net est reçu par une région locale (S317,
  [ordre D](validation/RESTITUTION-S317.md)) — **en eau calme seulement** : sous une vraie mer, δ dérivait (A289, S319) ; **S369** : la dérive
  retirée à sa source, δ relatif à B ([ADR-198](adr/ADR-198-la-voie-d-a289.md)) ; une perturbation croît encore sous houle raide (A320). Manquent le couplage complet (ordre E), la production GPU et le sens W → δ. Lot 2 d'[ADR-178](adr/ADR-178-strategie-en-trois-systemes-physiques.md) D7.
- [ ] **4.9 Fusion et séparation de domaines** sans rupture — *partiel* **depuis S396**, en référence : les domaines comme
  ensembles de blocs, fusion = union, séparation = partition, critères et délai d'ADR-006 §4 ; l'état recopié au bit ; fusionnés
  au critère, deux domaines restent à 0,26 % de l'amplitude du domaine unique ; une séparation ne saute pas, puis ses murs
  réfléchissent ([preuve](validation/FUSION-S396.md)). **S401** : deux parties d'une même fenêtre évoluent séparées **sans
  recopie**, à un ulp de deux domaines, et un domaine **croît et décroît** en suivant sa perturbation, à 0,26–0,66 mm du domaine
  entier ([preuve](validation/DOMAINE-EPARS-S401.md)). Manquent le stockage par blocs et son pool, les bords non réfléchissants
  d'une partie, la carte.
- [ ] **4.10 Adaptation interne** : subdivision locale dans le chaos, fusion au repos — *partiel* depuis S408 : la
  **représentation** s'adapte — dans `Apic3`, une colonne passe aux particules là où la surface n'est pas un graphe, où le corps
  arrive, où la pente dépasse un seuil, et revient aux colonnes au repos après un maintien, à masse exacte ; sur B10 3D, 22 % des
  colonnes en particules, le pincement d'APIC seul à un pas près ([preuve](validation/BASCULE-S408.md)). **S410**, sur une vague
  qui déferle : la bande naît au front des crêtes avant le pli, mais traîne ou hésite ; son critère doit suivre la crête (§6). La
  subdivision de la maille elle-même est absente.
- [ ] **4.11 Régime substitutif** quand δ n'est plus petit, restauré depuis graine (I-17) — *absent*.
- [ ] **4.12 Cavité et gerbe d'impact** (C20, B10) — *partiel* — **la cavité est portée sur le banc
  2D d'APIC** (S320, [B10](validation/B10-APIC-S320.md)) : pincement indépendant de l'échelle, masse
  exacte ; mais **son temps ne converge pas encore** à trois mailles (2,20 → 2,30 → 2,40 `√(D/g)`, S326),
  et à maille fine la fermeture de la bulle sans pression emballe le calcul (A311). **En 3D (S393)**, la sphère :
  pincement **convergé** (0,6 % entre 12 et 16 mailles), 2,08 √(R/g) dans la plage publiée ([preuve](validation/B10-APIC3D-S393.md)).
  Hors de δ : sa surface est une fonction hauteur (ADR-175 D5). Manquent
  la gerbe, qui suit la maille (A312), la bulle, qui n'est pas de l'air (A311), le raccord aux
  colonnes — **à masse exacte** dans les deux sens depuis S323–S325, sa frontière **non reçue** :
  S354, sur 30 s, elle ne tient pas la densité des particules, la masse migre et la période s'allonge
  à 5 cm (A316 ; [§10–13](validation/B10-APIC-S320.md)) ; S394–S397 : la circulation de la frontière était un défaut du
  banc, ses colonnes n'advectant pas ; **S399–S400, en 3D** : repos à 2·10⁻⁵ m/s, masse à ±2 mm sur 30 s aux deux mailles, période à
  0,2 point ; restent la densité et un courant de surface au raccord ([RACCORD-3D](validation/RACCORD-3D-S398.md) §5–6) —, la 3D (APIC 3D dans le cœur depuis S388 ; B10 en 3D, S393 : la cavité, pas la gerbe)
  et C20. Lot 5 d'ADR-178, APIC retenue
  par l'utilisateur (ADR-186), repris après la v1 (ADR-190 D4).
- [ ] **4.13 Proche-coque et gerbe d'étrave** — *partiel* **depuis S332–S338** (ADR-001 range le
  proche-coque dans δ) : la coque d'un corps de jeu dans δ, qui perce la surface, rayonne relativement
  à l'eau qui la porte et reçoit la masse ajoutée et l'amortissement que δ lui mesure — porte D reçue
  sur la référence CPU ([preuve](validation/PORTE-D-S333.md) §9,
  [rayonnement](validation/RAYONNEMENT-COQUE-S336.md)) ; la production GPU d'une coque qui bouge depuis S503–S509 (6.4) ; **S517** : la
  coque **en marche** sur la carte, sa vague d'étrave (0,72 m de stagnation, bornée) et un sillage stable, de forme compatible avec Kelvin
  sans mesure à 2° ([preuve](validation/SILLAGE-S517.md)) ; **S520** : sur 1,49 M mailles et 30 s, contre la théorie d'une pression sur son
  empreinte, δ est 3,5 fois moins ample et l'angle n'est pas tranché (A330, [preuve](validation/SILLAGE-COQUE-S520.md)) ; **S529** :
  l'amplitude du sillage de δ stable en maille (l'écart revient au modèle de référence), le pic d'étrave divergent au coin vif
  ([preuve](validation/A330-CONVERGENCE-S529.md)). Manquent la gerbe (surface non graphe, 4.16), la résolution près de la coque
  (± 43–49 % à 25 cm), le sillage mesuré (le recoupage dans un grand domaine : 2,2 ms depuis S518, A329 levée).
- [ ] **4.14 Plage** : rouleau 3D, mouillage et séchage (C04) — *absent* : C04 exécuté sur un
  véhicule d'essai 1D seulement.
- [ ] **4.15 Rochers et obstacles immergés**, turbulence — *partiel* : faces coupées sur fonds
  lisses en 2D (S232, ordres 1,947–1,966) **et en 3D depuis S324** — référence CPU, mode linéaire,
  identique au bit à la 2D sans `y`, ordre 1,956 sur une bosse ([preuve](validation/FACES-COUPEES-3D-S324.md)).
  Petites cellules préconditionnées (S326) ; **mode mobile** depuis S328 — au bit de la 2D sans `y`,
  ordre 1,954 sur la bosse ; **obstacles immergés quelconques** depuis S329 — sphère d'ordre 1,966,
  Archimède exact au niveau discret ; **en mouvement imposé** depuis S330 — masse ajoutée d'une sphère
  à 1,6 % de la théorie ; **rotation et coque qui perce la surface** depuis S332
  ([preuve](validation/CORPS-RIGIDE-S331.md) §4), sous couvercle partiel depuis S335. Manquent le couplage à B/W
  sur fond coupé et la turbulence — **aucun modèle de turbulence n'existe nulle part dans le dépôt**. Lot 3 d'ADR-178.
- [ ] **4.16 Surface non graphe** : déferlement, éclaboussures détachées — *partiel* depuis S393 : la
  cavité qui se referme sur l'air, reçue en 3D (ci-dessous) ; le déferlement porté sur un banc (S410) ; éclaboussures détachées absentes. **C'est le point le plus lourd de la liste** : il demande un **second solveur**, pas
  une extension du premier (ADR-175 D5). Ce solveur est **choisi et écrit sur un banc 2D, hors du
  cœur** : trois représentations comparées au même niveau — APIC garde la masse exactement et ne
  crée pas d'énergie ; rupture de barrage en accord à 1,6 % entre les trois, sans validation
  ([S318](validation/COMPARAISON-LOT5-S318.md)) —, **APIC retenue** par l'utilisateur (ADR-186).
  Rien du déferlement ni des éclaboussures n'est reçu : jet et couronne suivent la maille (A312).
  **S388** : APIC **entre dans le cœur en 3D** (`apic3d.rs`, [preuve](validation/APIC3D-S388.md)) — masse exacte, repos,
  ballottements à +0,39 % (1, 0) et +1,01 % (1, 1) à 2,5 cm depuis S389 (noyau de deux mailles, parois reflétées).
  **S393** : la première surface non graphe reçue en 3D — une sphère cinématique ouvre une cavité qui se **pince à 2,08
  √(R/g)**, dans la plage publiée (1,72 à 2,29), convergé à 0,6 % entre 12 et 16 mailles par diamètre, masse exacte
  ([B10-APIC3D-S393](validation/B10-APIC3D-S393.md)) ; couronne et jet suivent encore la maille. **S398** : APIC 3D reçoit une zone de
  colonnes (surface `η`, une seule projection) pour le raccord ([preuve](validation/RACCORD-3D-S398.md)) ; **S399–S406** : la bande et
  l'échange à masse exacte ; **S407** : **tout le critère du raccord tenu aux deux mailles** — repos, migration, densité (la pose à
  la face, §8), saut, période, amortissement, courant de surface (la face de frontière appartient à la zone, §7) —, sur une frontière
  droite et fixe. **S408** : **la frontière bouge** (C6a) — la bascule colonnes ↔ particules à masse exacte et un critère avec
  hystérésis ; sur B10, 22 % des colonnes en particules, pincement un pas plus tôt qu'APIC seul, 19 s contre 36
  ([preuve](validation/BASCULE-S408.md)) ; **S410** : **le déferlement** d'une houle de Stokes (Chen et al. 1999, `ka` = 0,55)
  dans APIC 3D — retournement à 0,706 √(λ/g) (Chen : 0,72), jet qui retombe à 1,27 (1,56) ; la bande le prévoit sept pas avant, à
  masse exacte, mais son maintien traîne (52 à 68 % des colonnes) ou hésite, et chaque conversion au sommet de la crête le
  perturbe ; R34 posée ([§6](validation/BASCULE-S408.md)). **S413** : **la bande étroite en profondeur** (C6c-1, ADR-212) — l'eau profonde sur la grille sous un
  fond, les particules au-dessus seulement : au repos, 8,7·10⁻⁶ m/s ; sur 30 s de ballottement, à 0,2 point d'APIC seul, 4,7 à 7,7 fois
  moins de particules, masse au bit ([preuve](validation/BANDE-ETROITE-S413.md)). **S414** : le fond **placé par le critère** (C6c-2) — sur B10, la
  cavité d'APIC seul au chiffre près avec 4 122 particules (÷ 7 contre la bande pleine), la prédiction du corps à horizon court au
  pas même d'APIC seul ; sur la vague de Chen, ÷ 6 et deux fois plus vite, R35 posée ([§5](validation/BANDE-ETROITE-S413.md)).
  **S415** : le fond qui suit l'écoulement (C6c-3, l'idée de l'utilisateur) — sur un tourbillon enfoui, la vitesse rend l'énergie
  d'APIC seul avec 2,4 à 3 fois moins de particules ; sous une houle, la vitesse relative à B reste à faire ([§6](validation/BANDE-ETROITE-S413.md)).
  Restent les éclaboussures détachées et le saut max sous 3 mm (3,85 à 2,5 cm). Lot 5 d'ADR-178 ; commande aussi 4.12, 4.13, 4.14 et 7.2.
- [ ] **4.17 Référentiel accéléré et invariance galiléenne** (C16, C06) — *partiel* depuis S542 : la composante horizontale de
  `g_eff` dans le pas linéaire de δ — la surface au repos à 0,003° de la normale à `g_eff`, la période du ballottement à 0,11 % de sa
  formule ; **S543 : la rotation** — la surface cylindrique d'une station tournante à 1,5 % de `1/R` (C16 exécuté :
  [preuve](validation/C16-ACCELERE-S542.md)). Manquent Coriolis, la carte GPU et le pas couplé, C06 sur le système (partiel sur un
  véhicule d'essai 1D).
- [ ] **4.18 Conservation de la masse et de l'énergie** (C09) — *partiel*, **et la masse est
  désormais comptée**. S310 : bilan **exact par télescopage** tenu par les deux pas, plancher
  publié ; cuve fermée à 7,4·10⁻¹² m de dérive sur 5 s, murs à zéro exact ; scène couplée à
  1,28·10⁻¹⁰ m³ de résidu ([preuve](validation/BILAN-MASSE-S310.md)). **A298 n'est donc pas une
  fuite de volume du schéma.** Dissipation numérique mesurée : 0,0935 % par seconde. **Manquent**
  le compteur sur la carte, et les bilans d'**énergie** et de **quantité de mouvement** — publiés
  comme états, termes manquants nommés (travail de la pression au bord, flux advectif). C09 non
  exécuté. **S313** : la loi du plancher (`u₃₂·activité/√N`), et **T2 tenue** — dérive d'un domaine
  fermé 2,99·10⁻⁹ puis 5,77·10⁻¹⁰ sur 10 s pour 10⁻⁶ admis ([preuve](validation/PLANCHER-BILAN-S313.md)
  §5) ; critères C1–C3 actés (ADR-182). **S317** : le volume qui **quitte** δ a un receveur local, et
  le bilan δ + régions se ferme au résidu ([ordre D](validation/RESTITUTION-S317.md)) — la
  représentation, pas le monde.
- [ ] **4.19 Coût de δ compatible avec le budget** — *partiel* : carte du coût (S244), multigrille
  (S252), **multigrille du mode mobile (S274, ADR-167)** : le pas couplé à 16 384 mailles passe de
  280 à 49 ms, environ 24 fois le budget d'eau. **S276 : δ en direct à 40 images/s** — bande de
  6 656 mailles, 21,7 ms par image (échantillonnage du fond par grille identique au bit, départ
  depuis la pression publiée, ADR-169), un pas par image (précision S275), zéro allocation ;
  ≈ 11 fois le budget. **S299–S302 : le GPU et la 3D existent** — le pas couplé entier est
  résident sur la carte, **0,84 ms à 64 cycles sur 27 648 mailles**
  ([S301](validation/DELTA3D-PAS-GPU-S301.md)) ; sur la scène de 376 320 mailles à 32 cycles,
  **4,62 ms par pas contre 2 ms** visés, et le tampon des faces plafonne le domaine à 1,15 M faces
  ([S302](validation/SCENE-DELTA3D-S302.md) §2). **S341–S348** : fond factorisé et dix champs par face, au bit —
  3,68 ms par pas — ; **cadence de 30 Hz, un pas en deux parts : 1,92 ms au 99ᵉ centile par image**, porte C reçue
  sur le banc ([preuve](validation/COUT-DELTA3D-S341.md) §11) ; **S350** : le coût suit l'emprise d'un domaine
  redimensionné, 0,09 ms + 3,57 ms × surface ([preuve](validation/ARBITRAGE-3D-S344.md) §6) ; **S353** :
  interpolé au rendu et **mesuré en direct**, rendu concurrent — 1,49 ms de δ par image à 30 Hz contre 3,31 à
  60 Hz ([preuve](validation/COUT-DELTA3D-S341.md) §12). **S385** : la **multigrille 3D** de la référence, préconditionneur du
  pas mobile — 9 à 11 itérations de 12 288 à 786 432 mailles contre 102 à 365 ([preuve](validation/MULTIGRILLE-3D-S385.md)) ;
  **S390** : **sur la carte**, éteinte par défaut — à la porte B, le résidu de Jacobi-32 en 6 cycles, projection 1,08 ms
  contre 2,08, pas 2,75 ms au 99ᵉ centile contre 3,77 ([preuve](validation/MULTIGRILLE-3D-S385.md) §5). **S409** : **à 10
  cm**, la multigrille y est nécessaire (8 cycles ; projection ÷ 2,8 au moins à résidu égal) ; sous 2 ms par image, **8 m ×
  8 m** à 30 Hz — qui y explose en 62 s (A322) — et **5,6 m × 5,6 m** à 60 Hz, stable ([§6](validation/MULTIGRILLE-3D-S385.md)).
  **S416** : APIC 3D nu **sur la carte** (C7a), à 0,46 mm de la référence — 2,05 ms au 99ᵉ centile pour 12 800
  particules, 4,77 ms pour 102 400 (47 ns par particule), la projection d'abord ([preuve](validation/APIC-CARTE-S416.md)) ;
  **S417** : le corps (B10 nu, pincement au pas de la référence, 6,42 ms pour 131 072 particules) et la zone des colonnes sans
  échange (volume exact en entiers) ([§7–9](validation/APIC-CARTE-S416.md)) ; **S418** : l'échange à la frontière, volume
  exact à 0 quantum sur 30 s ([§10](validation/APIC-CARTE-S416.md)) ; **S419** : le fond de la bande — la bande étroite à 1,45 mm
  de la référence sur 30 s ([§11](validation/APIC-CARTE-S416.md)) ; **S420** : la bascule — B10 en bande étroite, pincement au pas de la
  référence, volume exact, mais 28,7 ms (l'échange et la bascule sur un fil) ([§12](validation/APIC-CARTE-S416.md)) ; **S421** :
  29,0 → **4,9 ms** par pas à sémantique exacte (+ 1,9 de bascule) ; restent la projection, la bascule, la surface ([§13](validation/APIC-CARTE-S416.md)) ;
  **S422** : la multigrille sur la carte — 11 à 14 itérations au lieu de 207, **3,9 ms** par pas ([§14](validation/APIC-CARTE-S416.md)). **S423** : la bascule en groupe (retrait à forme close, ensemencement par graine), la surface en coopération, la multigrille par défaut — **3,26 ms** par pas + **0,48** de bascule au p99, issues tenues ([§15](validation/APIC-CARTE-S416.md)). **S424** : la projection à **0,885 ms** au p99 (niveaux grossiers en mémoire de groupe, restrictions hors du groupe), issues identiques ; pas + bascule **3,25 ms** ([§16](validation/APIC-CARTE-S416.md)). **S425** : la fin du pas profilée (préfixes en groupe, tri par rang, séparation élaguée, solde vertical des seules colonnes dues) — pas + bascule **2,45 ms** au p99, issues identiques ([§17](validation/APIC-CARTE-S416.md)). **S426** : l'absorption face par face, à l'ordre de la référence — **2,31 ms** ([§18](validation/APIC-CARTE-S416.md)). **S427** : les gestes de l'échange groupés, une course corrigée — **2,16 ms** ([§19](validation/APIC-CARTE-S416.md)). **S428** : la projection resserrée au bit (niveaux grossiers à 512 fils, restrictions en coopération) — **2,07 ms** ([§20](validation/APIC-CARTE-S416.md)) ; **C7e reçu par l'utilisateur** (2026-10-02, S429 : le surplus de 0,07 ms accepté). **S429 : C7d conçue, C7d-1 en partie** — la vitesse propre de δ ne demande rien sous une houle calme, mais prend 0,94 de la crête raide ([§21](validation/APIC-CARTE-S416.md)). **S430** : la déformation propre écartée (le gradient de la grille bruité en surface) ; le critère (a) trouvé mal posé et réécrit ; la vitesse propre à 0,4 m/s tiendrait, sauf ses oscillations ([§21.2](validation/APIC-CARTE-S416.md)). **S431 : C7d-1 reçu** — la vitesse propre de δ avec relâche (0,3 / 0,15 m/s) : 0,64 au retournement (forme seule 0,62), aucun retour rapide, rien sous une houle calme ([§21.3](validation/APIC-CARTE-S416.md)). **S432 : C7d-2 reçu** — le critère sur la carte, décisions identiques à la référence ; B10 sans les clés identique au bit (2,06 ms) ([§21.4](validation/APIC-CARTE-S416.md)). **S433** : la conception de C7d-3 — la bande entre dans le pas couplé, les particules portent `u′` ; C7d-3a (A320, la forme de Bernoulli), C7d-3b (le mode relatif sur la carte), C7d-3c et C7d-3d (la bande relative) ([§22](validation/APIC-CARTE-S416.md)). **S434 : C7d-3a non reçu** — la forme de Bernoulli ne freine A320 que de 15 à 20 % ; l'hypothèse de S369 réfutée par bisection ([MER-S369](validation/MER-S369.md) §6). **S435** : A320 croît à la longueur d'onde de la houle — une modulation d'ordre `ω(ak)²`, pas un défaut de grille ; le critère de C7d-3a, mal posé, sera rapporté à Benjamin-Feir ; **A324** — la surface de B qui franchit un centre de maille amplifie δ — passe avant C7d-3b ([MER-S369](validation/MER-S369.md) §7). **S436 : A324 corrigée** — le point fixe du mode relatif se rompait au fantôme latéral ; il interpole désormais les fantômes verticaux (le défaut), δ nul reste nul au bit ([MER-S369](validation/MER-S369.md) §8). **S437 : C7d-3a non reçu** sous le critère réécrit (1,5 fois Benjamin-Feir à 25 cm) — le taux dépend de la place du repos dans la maille ; sous une houle de 8 m, rien au-delà de Benjamin-Feir ([MER-S369](validation/MER-S369.md) §9). **S438** : l'échelle en mailles par longueur d'onde, indécise ; cause non trouvée ; C7d-3b passe, sans la bascule des défauts ([§22.9](validation/APIC-CARTE-S416.md)). **S439** : le mode relatif sur la carte (`Step3::set_relative`) — production au bit, témoin nul au bit, horizon du millimètre doublé ; un critère manqué de 0,01 % (fenêtres inégales) ; **S440 : reçu**, l'écart accepté par l'utilisateur ([§22.10](validation/APIC-CARTE-S416.md)). **S440** : A322 sous le mode relatif — tient 120 s, mais des bouffées à l'échelle de la maille ([MULTIGRILLE-3D-S385](validation/MULTIGRILLE-3D-S385.md) §7). **S441** : les bouffées viennent de la bande relative (FTCS) ; sous Lax-Wendroff, elles disparaissent ; un critère en part relative mal posé (§8). **S442 : A322 levée en mode relatif** (l'écart accepté par l'utilisateur), la bande sous Lax-Wendroff par défaut ; A320 inchangée ([MER-S369](validation/MER-S369.md) §11). **S443 : C7d-3b reçu** — δ relatif par défaut, référence et carte ([§22.11](validation/APIC-CARTE-S416.md)). **S444** : B entre dans la bande ([ADR-214](adr/ADR-214-b-entre-dans-la-bande.md)) ; c1 non reçu ([§23.1](validation/APIC-CARTE-S416.md)) ; **S445** : c1 plafonné, la bande en eau totale ; **S446–S447** : le raccord conservatif bande ↔ mer, la masse au raccord à 0,01 % ([§23.4](validation/APIC-CARTE-S416.md)) ; **S448–S449** : c3 plafonné, le raccord instable à son bord ([§23.6](validation/APIC-CARTE-S416.md)) ; **S450–S453** : la surface continue — R37 reçu (S452), en direct sur la carte seule (S453) ([SURFACE-CONTINUE-S450](validation/SURFACE-CONTINUE-S450.md)) ; **S454–S456** : C10-1, le saut sur une scène de 4 m sous la houle, en temps réel, masse exacte ; **S458** : la v1 (R38 reçu) ; **S461–S469** : la scène dans Godot, caustiques, surface fine, le direct, la pluie, le joueur debout, éclairé, son ombre ([C10-SCENES-S454](validation/C10-SCENES-S454.md)).
  Manquent 30 Hz stable à 10 cm, d'autres scènes, plusieurs domaines en direct, un 99ᵉ centile en direct.
- [ ] **4.20 Changement de solveur pendant une simulation** (ADR-007) — *absent*, conçu.
- [ ] **4.21 Cohérence de phase entre δ et B+W sur la durée de vie d'un domaine** — *partiel* **depuis S369** :
  δ relatif à la dynamique de B ([ADR-198](adr/ADR-198-la-voie-d-a289.md)) — sous B seul, δ nul reste nul **au bit**,
  pas après pas : la phase du fond dans le domaine est celle de B, par construction ([preuve](validation/MER-S369.md)
  §1). Manquent la production GPU dans ce mode, W, et une perturbation stable sous houle raide (A320).
  Besoin découvert S274 : B est linéaire, un δ fidèle dérive de la dispersion d'amplitude
  (0,85–0,87 fois Stokes mesuré). Sous `ak` = 0,06, environ 7 cm en une minute : surface rendue
  différente de la surface de jeu. Options et déclencheur : A289. **S319 : le déclencheur est
  atteint, plus fort que prévu** — sous une seule houle de B, δ nul au départ croît jusqu'à **trois
  fois l'amplitude de la mer** en deux minutes (≈ 0,04 s⁻¹ sous 2,5 cm, ≈ 0,10 sous 5 cm) ; **S322** :
  ce n'est pas le pas de temps, taux à 0,8 % près de 20 à 2,5 ms ([preuve](validation/MER-S319.md)
  §4, §8). Il bloquait l'ordre E de 4.8 ; la voie est tranchée en S369 (ci-dessus).

## 5. Volumes finis et inondations (V)

- [x] **5.1 Contenants à volume entier qui se vident** par orifice et déversoir (C12) — *validé* :
  0,0824 % contre l'analytique, pas de 100 ms, refus atomiques
  ([S224](validation/NOYAU-V-S224.md)).
- [ ] **5.2 Géométrie réelle des contenants** : gravité dirigée, plans orientés, formes non
  convexes — *partiel* (S226, S228). Manquent la précision des grands volumes (A269) et les formes
  courbes cuites depuis les assets.
- [x] **5.3 Fuites, transferts et débordements entre contenants** — *validé* (S489) : orifices et
  déversoirs entre nœuds, arrivées collectives (S227) ; **le débordement vers l'extérieur** (`Flow::Spill`) — un contenant plein
  déverse exactement ce qu'il reçoit, au millilitre, la pluie comprise, le bilan fermé ; l'hôte lit le déversé et sa position
  ([DEBORDEMENT-S489](validation/DEBORDEMENT-S489.md)). Un débordement *dans* un autre contenant passe par un déversoir au bord.
- [ ] **5.4 Vannes et pompes** — *partiel* depuis S372 : une **commande** entière par arête, état répliqué et sauvegardé
  (WVST v2) ; la vanne, section ou largeur commandée — C12 à demi-ouverture à −0,10 % de l'analytique ; la **pompe** en
  réseau ouvert, courbe parabolique, clapet, à sec, similitude — à 0,025 % de l'intégrale analytique, barrage au
  millilitre ; masse exacte, au bit à commande pleine ([preuve](validation/VANNES-POMPES-S372.md),
  [ADR-199](adr/ADR-199-vannes-et-pompes-dans-v.md)). **S515** : la vanne et sa courbe d'ouverture (`Flow::Valve`, la courbe du
  constructeur), la pompe sur sa conduite (`Flow::PumpLine`, pertes `K·Q²`, rendement) et son énergie — à 0,005 % du gain d'énergie
  potentielle ([preuve](validation/VANNE-POMPE-S515.md)). Manque le réseau fermé (5.8). **S374** : un premier consommateur, la piscine rejouée dans Godot
  ([preuve](validation/PISCINE-V-S374.md)).
- [ ] **5.5 Pluie selon l'exposition au ciel, absorption par le sol** — *partiel* depuis S378 : la **pluie**, arête de V
  du ciel vers un contenant — surface d'ouverture × exposition × intensité (ADR-204) ; l'exposition **dynamique et
  fractionnaire** est la commande de l'arête, bâche entière ou demi-bâche posée et retirée en temps réel
  ([ADR-203](adr/ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) D2), sauvegardée ; une heure au millilitre, déversoir
  sous la pluie à 0,11 % de l'analytique ([preuve](validation/PLUIE-V-S378.md)) ; **S530 : l'absorption par le sol** — Green–Ampt,
  une arête de la flaque vers le sol intégrée exactement sur le pas, à 7·10⁻⁵ de la solution implicite ; le sol plein et la flaque à sec
  l'arrêtent ([preuve](validation/INFILTRATION-S530.md)) ; **S533 : la pluie hors contenant** — rétention de surface, infiltration,
  ruissellement : la lame infiltrée à 0,1 % de Mein–Larson et Green–Ampt décalé, la masse exacte ([preuve](validation/PLUIE-SOL-S533.md)).
  **S535 : l'assèchement** — le drainage de Brooks–Corey intégré exactement (7·10⁻⁶ de la forme fermée sur 24 h) et l'évaporation ; le cycle
  de l'eau du sol à la masse exacte ([preuve](validation/ASSECHEMENT-S535.md)). Manquent le calcul de l'exposition depuis les objets
  posés, l'évaporation du sol limitée par son humidité, la météo (à la fin).
- [ ] **5.6 Seuil adaptatif à l'échelle du contenant** — *absent*.
- [ ] **5.7 Plusieurs liquides** (`liquid_id`, A17) — *absent*.
- [ ] **5.8 Réseau fermé sous pression** — *absent*, reporté en v2 par ADR-010.
- [ ] **5.9 Compartiments, brèches, inondation de navire, limitée par l'air** (C17, ADR-015) — *partiel* depuis S538 : la poche
  d'air isotherme scellée d'un compartiment de V (`step_air`) — **C17 passe** : sans évent, la brèche n'embarque que l'équilibre de Boyle
  (0,2901 m sur 2 m, à 1,2·10⁻⁵), avec évent Torricelli à 2,9·10⁻⁴ ([preuve](validation/C17-AIR-S538.md)) ; **S547 : l'évent à débit
  limité** (`Q_eau ≤ Q_air`) — le remplissage à 0,7–1,5 % de la loi quasi permanente ([preuve](validation/C17-AIR-S538.md) §5).
  Manquent la poche adiabatique, la flottabilité de la poche d'un compartiment (6.6), les brèches en jeu, la compartimentation d'un
  navire.
- [ ] **5.10 Articulation V↔δ** : V expose sa surface, déclenche δ, garde la masse (C21, ADR-025) —
  *partiel* depuis S375 : le bassin de la piscine est un domaine δ 3D dont **V garde la masse** — niveau de δ à 0,1 µm de
  celui de V sur 330 s, sources et puits aux arêtes de V (le jet, le seuil), repos qui suit V ; V jamais lu en retour
  (C21 par construction) ; surface rendue dans Godot ([preuve](validation/PISCINE-DELTA-S375.md)). Manquent une dynamique
  visible (maille de 5 à 10 cm : δ sur GPU), le panache calé, le bac tampon, V qui déclenche δ, la porte E ; c'est la porte E. **2026-09-26, décision de
  l'utilisateur** : la dynamique des contenants se calcule en 3D volumétrique ([ADR-200](adr/ADR-200-la-dynamique-des-contenants-en-3d-volumetrique.md)) —
  premier cas, la piscine de S374 ([état de départ](validation/PISCINE-V-S374.md)).
- [ ] **5.11 Eaux souterraines** — *absent* ; **hors du périmètre** par décision de l'utilisateur (2026-09-26,
  [ADR-197](adr/ADR-197-reponses-du-2026-09-26.md) D4 : pas de terrain réaliste à hydrologie) — gardé pour mémoire.
- [ ] **5.12 Capture et restauration de V** — *partiel* : noyau restauré au bit (S229). Manquent le
  stockage durable et le réseau.

## 6. Solides et flottabilité

- [x] **6.1 Flottabilité des objets importants** (C10, C11, B6) — *validé* (S502) : **corps rigide à six degrés
  de liberté dans le cœur** depuis S331 — proxy sur B + W, masse ajoutée ; C10 tenu à 0,02 % sur le tirant
  ([preuve](validation/CORPS-RIGIDE-S331.md)) ; **sa coque pilotée dans δ** depuis S332, sans que δ touche la
  trajectoire (I-04) ; **sur une houle de B** depuis S333, pilonnement forcé à 3·10⁻⁵, δ relatif à l'eau
  qui la porte ([preuve](validation/PORTE-D-S333.md)) ; **masse ajoutée et amortissement de rayonnement de
  pilonnement** mesurés par δ depuis S336, l'énergie dissipée égale à celle que δ reçoit à 2,3 % près
  ([preuve](validation/RAYONNEMENT-COQUE-S336.md)) ; **porte D reçue** sur la référence CPU en S338, verdict
  R15 ([preuve](validation/PORTE-D-S333.md) §9) ; **W derrière la requête** depuis S494–S495 (6.2) ; **C11** depuis S498 — les trois
  régimes d'ADR-008 §3 selon `ω·dt`, la balle de ping-pong contrainte à la surface à l'écart nul, `|G|` = 1 à 10⁻¹² hors mode contraint
  ([preuve](validation/PETIT-OBJET-S498.md)) ; **B6 mesuré** en S499 — la hauteur métacentrique d'un proxy en grille suit
  `BM·(1 − 1/n²) + z_F` à 3·10⁻⁸ ([preuve](validation/B6-PROXY-S499.md)) ; **S500** : la poussée au centre de la part immergée
  ([ADR-227](adr/ADR-227-la-poussee-au-centre-de-la-part-immergee.md)) — une couche suffit, 70 points pour le navire, 49 pour la barque
  et la caisse ([preuve](validation/POUSSEE-S500.md)). **S502** : l'amortissement et l'inertie ajoutée des cinq autres degrés de liberté, mesurés par δ (le moment de sa pression sur la
  paroi), reçus par le corps comme constantes d'archétype ; lâchers à ±1 % ([preuve](validation/RAYONNEMENT-6DDL-S502.md)). Limites :
  constantes à ± 10 à 30 % (25 cm, A317), figées à une pulsation ; un pavé (ADR-227 §3).
- [ ] **6.2 Forces de l'eau sur les objets** : vagues, courant, turbulence, sous la frontière
  d'autorité d'ADR-008 — *partiel* depuis S333 : les **vagues de B** — poussée et gradient de la pression
  du proxy, la coque cavale avec la houle à 0,1 % ([preuve](validation/PORTE-D-S333.md)) ; **S494** : les **impacts de W** — B et les
  impacts confirmés composés par la composition autoritaire derrière la requête du corps (`MixedWater`) ; une bouée pilonne à
  0,81 % de l'oscillateur forcé par la surface sous elle et dérive avec l'anneau au second ordre ([preuve](validation/FORCES-W-S494.md)) ;
  **S495** : le **sillage** d'un objet en marche — une bouée dans le bras de Kelvin suit l'eau à 0,85 %
  ([preuve](validation/SILLAGE-CORPS-S495.md)) : W entier derrière la requête du corps ; **S513** : le **courant** (2.6, C0 et C2) — un corps
  traîné dérive avec lui à 0,4 % de l'analytique ([preuve](validation/COURANT-S513.md)). Manque la turbulence.
- [x] **6.3 Un objet en mouvement produit son sillage** — *validé* (S497) : mouvement et charge prescrits
  vers la source de pression (ADR-103) ; **le corps du jeu en marche émet sa source** (`RigidBody::wake_leg`, tronçons visés sur sa
  position prédite, charge `m·g`) — une coque menée sur un cercle à 3 m/s, son sillage à 1,20 % de la trajectoire déclarée, en `Δ²`
  ([preuve](validation/SILLAGE-EMIS-S497.md)) ; S495 le fait sentir aux autres corps. Le corps réel remue δ depuis S332 ; le champ
  proche d'une coque qui avance est 6.4, la résistance de vague rendue au corps 6.2.
- [x] **6.4 Parois et corps mobiles dans δ** (C23) — *validé* (S509) : un **solide en mouvement
  imposé** dans la référence 3D, masse ajoutée d'une sphère à 1,6 % ([preuve](validation/FACES-COUPEES-3D-S324.md)
  §9) ; la **coque du corps rigide**, qui tourne et perce la surface, pilotée par le jeu (S332–S337,
  [porte D](validation/PORTE-D-S333.md)) ; **S358** : le pas linéaire sur la carte (`Linear3`, ADR-193), à
  1,3·10⁻⁵ m de la référence, 0,32 ms ([preuve](validation/LINEAIRE-GPU-S358.md)) ; **S503** : un solide **immergé qui bouge**
  sur la carte — le cœur découpe, la carte applique (`set_motion`) ; une sphère menée à 2,2·10⁻⁶ m de la référence pour 1 cm
  d'élévation ([preuve](validation/MOBILE-CARTE-S503.md)) ; **S504** : la coque qui **perce** la surface en mouvement — dépôt sous
  couvercle partiel et transfert de S334 sur la carte, pilonnement et roulis à 2·10⁻⁶ m de la référence
  ([preuve](validation/COQUE-CARTE-S504.md)) ; **S505** : **C23 sur le système** — le pas borné par la vitesse gouvernante, la paroi
  comprise ([ADR-229](adr/ADR-229-la-paroi-dans-la-vitesse-gouvernante.md)), Courant à 0,4500 de 0,5 à 20 m/s
  ([preuve](validation/C23-SYSTEME-S505.md)) ; **S507** : A328 réattribuée — l'ordre 1 en temps, constante ×5 près d'une coque qui bouge
  (10 % du champ proche à ≈ 6 ms) ([preuve](validation/A328-S507.md)) ; **S508** : le recoupage dans la boîte du solide, au bit de
  l'entier — 8 ms → 1,4 à 1,55 ms par pas ([preuve](validation/RECOUPAGE-S508.md)). **S509** : 0,82 ms par pas — la carte ne reçoit que ce qui change
  ([preuve](validation/ENVOI-S509.md)). Limites : l'ordre 1 en temps près d'une coque qui bouge (A328) ; ces 0,82 ms valent pour un petit
  domaine — 25 ms sur 786 000 mailles (A329, S517), **2,2 ms depuis S518** (le recoupage, l'extraction et l'envoi limités à la boîte, au
  bit ; [preuve](validation/RECOUPAGE-GRAND-S518.md)).
- [x] **6.5 Décor fixe comme frontière imposée** — *validé* (S493) : fonds lisses coupés en 2D (S232) **et
  en 3D** depuis S324 ; **solide immergé quelconque** depuis S329, Archimède exact au niveau discret —
  référence CPU ([preuve](validation/FACES-COUPEES-3D-S324.md) §8) ; **production GPU d'un solide fixe immergé**
  depuis S358, découpe du cœur chargée telle quelle ([preuve](validation/LINEAIRE-GPU-S358.md) §2) ; **un décor qui perce la
  surface, posé sur le fond** (S490) : repos au bit, aucune fuite, la seiche de la demi-cuve à 0,3 %
  ([DECOR-S490](validation/DECOR-S490.md)) ; **aligné sur la grille** depuis S492 (A327 levée : la tolérance au point mort,
  [A327-S492](validation/A327-S492.md)) ; **sur la carte** depuis S493 — le couvercle partiel porté, la carte à 6·10⁻⁸ m de la
  référence, aucune fuite, au bit sur le couvercle plein ([DECOR-CARTE-S493](validation/DECOR-CARTE-S493.md)).
- [ ] **6.6 Grands navires** — *partiel* depuis S548 : une barge s'enfonce par un compartiment envahi — le corps rigide et un
  compartiment de V couplés, le tirant final à 0,025 % de la flottabilité perdue, l'eau embarquée à 0,1 %
  ([preuve](validation/BARGE-ENVAHIE-S548.md)) ; **S549 : la carène libre** — le centre de l'eau d'un compartiment suit la surface
  horizontale, la stabilité perd `i/∇` à 0,34 % près ([preuve](validation/CARENE-LIBRE-S549.md)) ; **S550 : l'angle de bande** d'une barge
  instable, 19,31° pour 19,08° ([preuve](validation/ANGLE-BANDE-S550.md)). Manquent l'eau qui court (le ballottement d'un compartiment),
  l'assiette, le chavirement au-delà du pont mouillé, la poche d'air porteuse, plusieurs compartiments, un navire réel, les brèches en jeu.
- [ ] **6.7 Acteur poussé, renversé ou déplacé par l'eau** (vague, poche d'air) — *partiel* depuis S514 : les règles d'ADR-018 (la
  progression selon la profondeur, le produit d'emportement : 0,5 m à 2 m/s emporte un adulte) et le nageur d'ADR-023 §3 (corps commandé,
  contraint, sa commande dans le repère de la surface) — il cesse de faire route au seuil dérivé `πH/T = 0,7 m/s`, au point près
  ([preuve](validation/ACTEUR-S514.md)). Manquent la poche d'air qui pousse un acteur et le rouleau plongeant qui décolle un nageur.
- [x] **6.8 Impulsion d'entrée dans l'eau** (slamming, C20) — *validé* (S512) : l'impulsion de masse ajoutée d'ADR-023 §2 à l'instant
  exact où la quille passe sous la surface, corps et eau entraînée d'une même quantité de mouvement ; la même à 2·10⁻¹⁶ près quelle que
  soit la phase du tick (l'échantillonnage au tick : 5,3 % de dispersion) ([preuve](validation/IMPACT-ENTREE-S512.md)).

## 7. Phénomènes secondaires

- [ ] **7.1 Écume et moutons** (C14, B9, champ d'écume de SPEC-006 §4) — *partiel* depuis S367 : la **référence** du
  champ d'ADR-014 dans le cœur — deux canaux (actif 3 s, résiduel 30 s), advection orbitale, déferlement aux crêtes les
  plus accélérées ; couverture calée sur Monahan (le seuil physique ne déferle jamais : la bande de B est autosimilaire),
  vérifiée à 6 % près de 7 à 13 m/s ; traînées le long du vent, Ly/Lx 5,3 ([preuve](validation/ECUME-S367.md)) ;
  **S368**, produit sur la carte de Godot au même pas, décroissance à 10⁻⁵ ([preuve](validation/ECUME-GODOT-S368.md)).
  Manquent les sources de W, de δ et du vent, demi-vies et transfert calés (B9), la zone de surf et le sillage
  (scénarios 2 et 3 de B9) ; suspendu par l'utilisateur sauf références photographiques (2026-09-26).
  **Depuis S476** ([ADR-219](adr/ADR-219-reponses-du-2026-10-04.md) D5) : la suspension levée — les vidéos V2 (déferlement) et V3
  (impact) sont la référence du banc.
- [ ] **7.2 Spray, embruns, gouttelettes** — *absent*.
- [ ] **7.3 Microbulles visuelles** — *absent*.
- [ ] **7.4 Grosses bulles et poches d'air physiques** (C13, ADR-015) — *partiel* depuis S479 (**S540 : C13 passe** — les petites
  bulles de 0,1 à 5 mm à −6 à −9 % de SPEC-002 §2, selon `−g_eff` ; [preuve](validation/C13-BULLES-S540.md)) : l'air enfermé en poches
  adiabatiques dans la référence APIC 3D, inconnues de la projection ; une bulle oscille à la fréquence de Minnaert (× 1,02) et
  remonte ([POCHES-AIR-S479](validation/POCHES-AIR-S479.md)). **Sur la carte depuis S481** : la bulle suit la référence à 4·10⁻⁵
  (42,47 Hz contre 42,50) ; `--v1` 60 s stable avec poches ([POCHES-CARTE-S481](validation/POCHES-CARTE-S481.md)) — au prix de la
  multigrille (71 ms contre 16,7 par pas) — **levé en S482** : les poches dans la multigrille, 15,55 ms contre 15,33 sans
  ([COUT-POCHES-S482](validation/COUT-POCHES-S482.md)). **La vitesse terminale, S484** : 56 % de Davies et Taylor à R/dx = 6
  ([REMONTEE-S484](validation/REMONTEE-S484.md)) — **attribuée en S485** : l'air se perdait, et le quart de cuve n'est pas une
  symétrie ; **cuve entière, U = 0,90 de Davies et Taylor** ([REMONTEE-S485](validation/REMONTEE-S485.md)). Manquent la convergence
  en maille de U (S487 : × 0,90 à R/dx = 4, × 0,85 à 6, la bulle s'y scinde — [REMONTEE-S487](validation/REMONTEE-S487.md)), la
  fragmentation, C13.
- [ ] **7.5 Air comprimé, vide, eau dans le vide** (ADR-015) — *partiel* depuis S539 : la poche d'air comprimée portée par un corps
  (la coque retournée d'ADR-015 §2) — sa poussée à 10⁻¹² de Boyle, et le point de non-retour : lâché 0,3 m au-dessus il remonte, 0,3 m
  en dessous il coule ([preuve](validation/POCHE-AIR-S539.md)). Manquent la poche adiabatique, la poche qui s'échappe quand le corps
  bascule, la bulle libre (7.4), le vide et l'eau dans le vide.
- [ ] **7.6 Glace et vapeur** (C15, ADR-017) — *absent*. L'évaporation et le gel des contenants, par V
  ([ADR-203](adr/ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) D6).
- [ ] **7.7 Danger et traversabilité**, publiés par tuiles (ADR-018, SPEC-006 §5) — *absent*.
- [ ] **7.8 Audio de l'eau** (ADR-016, SPEC-006 §4.2) — *absent* ; **à la fin** (ADR-197 D5), par **Wwise**, l'audio du jeu
  DyingStar (ADR-219).

## 8. Rendu et niveaux de détail visuels

- [ ] **8.1 Rendu temps réel de la surface sur GPU** — *partiel* : hôte séparé B + impacts +
  sillages (ADR-130, S211–S249), habillage de banc ; le rendu ne pilote pas la physique (I-13). Manque l'intégration au moteur du jeu —
  **Godot 4** depuis S356 ([ADR-192](adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md)) ; l'afficheur reste le banc ;
  S357, la mer de B dans Godot, prototype sans intégration native ([preuve](validation/PROTOTYPE-GODOT-S357.md)).
- [ ] **8.2 LOD de la géométrie de surface** — *partiel* : grille projetée à pas écran. Manquent le LOD
  du maillage et le choix déplacement ou normales selon la vue (critère de parallaxe chiffré S257).
- [ ] **8.3 LOD par source** : grille du sillage, visibilité, filtre spectral — *partiel* (S234,
  S235, S249) : filtre spectral reçu pour B et sillage ; cuisson des huit bandes
  accélérée de 46 % au bit (S267). Manquent le filtre des impacts, la généralisation
  aux autres sources et le LOD temporel.
- [ ] **8.4 Écume, spray, gouttes, bulles rendus, chacun avec son LOD** — *partiel* depuis S380 (les gouttes de pluie, R29) : S356, l'écume des crêtes de B
  à la couverture de Monahan dans l'afficheur, non jugée, référence à porter dans Godot ([preuve](validation/RENDU-CRETES-S356.md)) ;
  R20 : *« uniquement sur des grandes vagues avec déferlement »* ; **S360** : dans Godot, tirée des vagues dominantes —
  moutons d'un à quatre mètres ([preuve](validation/SURFACE-FINE-S360.md) §3) ; **R21 : l'écume refusée** (S367) ;
  **S368** : le champ de 7.1 produit sur la carte et rendu — moutons qui pâlissent, dentelle résiduelle
  ([preuve](validation/ECUME-GODOT-S368.md)) —, puis **suspendu par l'utilisateur** (2026-09-26) sauf photographies qui
  renseignent forme, couleur et place sur la vague : éteint par défaut. **S380** : les gouttes de pluie qui tombent, au
  nombre de Marshall et Palmer, tracées selon Garg et Nayar ([preuve](validation/PLUIE-AIR-S380.md)), **reçues en R29** (*« Je valide »*). **S383** : les
  gerbes de pluie — couronne, dôme, jet relevés sur une goutte réelle (Murphy et al. 2015), aux impacts des rides, particules
  près de l'œil et part d'aire au loin ([preuve](validation/GERBES-S383.md), R32 reçue pour l'instant, trois défauts au peaufinage). Manquent l'écume (suspendue), le
  spray, les bulles, les gouttelettes et les éclaboussures au sol, et leurs niveaux de détail.
- [ ] **8.5 Transparence, réfraction, caustiques, particules sous-marines** — *partiel* depuis S359 : dans Godot, la
  colonne d'eau — fond vu par réfraction de Snell en espace écran, absorbé et voilé selon la profondeur (Maritorena,
  eau pure de Pope & Fry), transmission à 0,005 du modèle —, et la réflexion de l'afficheur portée
  ([preuve](validation/EPAISSEUR-EAU-S359.md), ADR-194) ; **S361** : les caustiques sur le fond, méthode directe, exactes
  à 5 % et l'énergie à 1 % ([preuve](validation/CAUSTIQUES-S361.md)), **validées en R22** (S367). Manquent les
  particules, les eaux chargées, les caustiques sur les objets et dans l'eau (piste : un volume de caustiques par
  tranches, [comparable](COMPARABLES-EXTERNES.md) lu en S400). **S474–S477** : le type d'eau — sept préréglages tirés de trois
  constituants, une carte qui le fait varier dans l'espace, le côtier réglé contre une vidéo de référence
  ([TYPE-EAU-S474](validation/TYPE-EAU-S474.md)).
- [ ] **8.6 Vue sous-marine et passage de la surface** (ADR-019, B11) — *partiel* depuis S365 : dans Godot, la
  caméra sous l'eau — la surface vue d'en dessous, **fenêtre de Snell** rendue à 0,05° de `arcsin(1/n)` et réflexion
  totale au-delà ; le milieu, `exp(−c·d)` par canal à 0,004 près, la lumière de l'eau, le fond et ses caustiques
  ([preuve](validation/SOUS-MARIN-S365.md)) ; R24 *« good »* ; **S366**, la lumière de l'eau calée sur la radiance
  mesurée par Tyler (1960), résidu 0,36 → 0,12 (§6). **S371** : la caméra à demi immergée (ADR-019 §6) — le milieu par
  pixel à l'objectif, la ligne à 0,08 pixel de l'intersection exacte, aucun pixel mal classé sur 60 images, 0,03 ms GPU ;
  un ménisque calé sur une photographie ([preuve](validation/DEMI-IMMERGEE-S371.md)) ; **S373** : la brume de l'air réglée
  par pixel à demi immergée (§10). Manquent le fond dans le miroir,
  bulles, écume vue d'en dessous, rayons, turbidité, gouttes sur le hublot, l'échelle radiométrique du ciel et du soleil
  (la fenêtre terne, §6), le coût du profil immergé entier (B11), les deux mixages audio (à la fin).
- [ ] **8.7 Rendu de δ raccordé à B+W sans rupture visible** — *partiel* : **S275, ADR-168** —
  bande δ couplée sous houle à crêtes longues, précalculée hors budget et rejouée dans `viewer/`
  (touche D : B seul, B+δ 4 ms, B+δ au pas d'image), couche GPU à 7·10⁻⁸ m de sa lecture CPU,
  scène S201 inchangée au bit ; revue R10 en attente. **S276 : en direct** (`--delta-direct`, un pas
  par image, identique au bit au rejeu, 40 images/s). **S302 : δ en 3D rendu en direct** depuis la
  seule surface publiée (I-13, ADR-175 D7), Catmull-Rom bicubique, fondu de 3 m, **197 Hz** ; le
  rendu existant reste identique au bit sans la couche
  ([preuve](validation/SCENE-DELTA3D-S302.md)). **Verdicts** : R15 (S337–S338), plus de coupure au
  bord de δ dans la scène de la porte D, par l'éponge et le même fondu ; R16 (S340), une onde de δ
  sur la mer de R14 ; R17 (S348), δ à 30 Hz validé à l'œil, sur images fixes
  ([revue](validation/REVUE-VISUELLE.md) §20–22). **S353** : l'interpolation du rendu qu'impose 30 Hz — plus
  aucune image immobile, chaque image varie comme à 60 Hz — et le budget mesuré en direct avec le rendu
  ([preuve](validation/COUT-DELTA3D-S341.md) §12) ; **R18 reçu** le 2026-09-26, *« Rendu convaincant »* (§23,
  ADR-197 D8). Manquent une frontière δ↔B sans rupture autre qu'un fondu de rendu, et une tolérance de pente d'image.
- [ ] **8.8 Lointain et horizon sans artefact** — *partiel* : coupure spectrale B/sillage (S249) ;
  bande d'horizon mesurée (S247, S248) ; fin de grille à l'horizon géométrique sous le ciel clair
  (S262). **S380** : l'extinction par la pluie, `β` de Marshall et Palmer ([preuve](validation/PLUIE-AIR-S380.md)). Pas de
  certificat d'absence d'alias.
- [ ] **8.9 Détails artificiels bon marché** (micro-vagues, ondes courtes) ajoutés au rendu — *partiel* :
  queue du spectre de B en pentes par pixel, filtrée par l'empreinte, +0,38 ms GPU (ADR-155, S256) ; **S360, dans
  Godot** : la même queue réalisée par FFT, 10 612 composantes en deux cascades, étalement d'Elfouhaily, variance non
  résolue par LEAN ([preuve](validation/SURFACE-FINE-S360.md), ADR-195), **validée en R21** (S367). **S379** : les rides
  de la pluie, factices (ADR-202 D3) — taux de Marshall et Palmer × Atlas compté sur les images, deux trains
  capillaires-gravité, fondus en rugosité au loin, sur le bassin et la mer ([preuve](validation/RIDES-PLUIE-S379.md), **validée
  en R28**). Manquent les capillaires du vent, la queue des perturbations W, le coût (celui des rides : une texture à
  moments).
- [ ] **8.10 Crédibilité perçue validée par un regard humain** — *partiel* : protocole de revue
  ([REVUE-VISUELLE](validation/REVUE-VISUELLE.md)). Premier verdict (R1, « trop lisse ») mesuré et
  traité ; **R7 accepté S266**, après lissage des reflets entre les crêtes (ADR-161).
  Optimisations S266/S267 reçues. **R11 reçu S303** (δ sans artefact, raccord invisible) ;
  **R12/R13 annulées** — leurs images tournaient options acceptées éteintes (L349) ; **R14 reçu
  S308**, et sa troisième image devient la **référence interne provisoire** de l'océan (ADR-178
  D2). L'écart à une photographie réelle est désormais **chiffré** (`outils/cible_image.py`) ; dans l'afficheur, ce
  qu'il en restait était **spatial** (L351, [confrontation](registres/TROIS-SYSTEMES-S308.md) §1) ; **S363, dans
  Godot**, la surface fine de S360 le comble, et une courbe (`TONALITE=photo`) tient les quatre grandeurs et la teinte
  des creux à 0,166 près en pose proche, contre 5,8 pour AgX ([preuve](validation/CIEL-S363.md)) ; **S367** : R21 (la
  surface fine), R22, R23, R25 validés, l'écume refusée. **R15** (S333–S338) : le bateau qui se pose
  est juste, la coupure au bord de δ levée — sans référence réelle, aucune trouvée ; **R16** (S340) :
  « Tout parrait bon visuellement », une onde de δ sur la mer de R14 ; **R17** (S348) : « Continue je
  valide », δ à 30 Hz contre 60 Hz ([revue](validation/REVUE-VISUELLE.md) §20–22). **S381** : le ciel de pluie (ADR-205,
  pièce 3 ; [preuve](validation/CIEL-PLUIE-S381.md), R30 reçue) ; **S382** : l'occultation du ciel et les ombres portées
  ([ADR-206](adr/ADR-206-la-visibilite-du-ciel-par-des-occultants-analytiques.md), [preuve](validation/OCCULTATION-CIEL-S382.md), R31 reçue) ; **S392** : les surfaces mouillées (ADR-205, pièce 5a ; [preuve](validation/SURFACES-MOUILLEES-S392.md), R33 reçue pour l'instant, peaufinage à venir). Autres poses, animation et scénarios
  restent à valider perceptivement. Ce verdict local ne clôt pas la crédibilité du système, et
  **une validation visuelle ne remplace pas une validation numérique** (ADR-178 D3).

## 9. Activation, prédiction, budget et dégradation

- [ ] **9.1 Activation multicritère** : proximité, visibilité, taille à l'écran, regard, vitesse du
  joueur, énergie, enjeu de jeu, budget (ADR-013, B8) — *partiel* **depuis S278** : le **mécanisme**
  qui consomme les critères existe — sac à dos sous budget, `P/C` décroissant, hystérésis
  ([ORDONNANCEUR-S278](validation/ORDONNANCEUR-S278.md), ADR-170) ; **`W_perception` calculé** — part
  d'écran — pour la bande δ (S279) puis **deux domaines δ 3D** qui se disputent un budget (S344,
  [preuve](validation/ARBITRAGE-3D-S344.md)). Manquent `W_gameplay`, qui vient du jeu, `W_urgence`, et un banc B8
  qui fixe les seuils ; ils sont calibrés par hôte (ADR-171).
- [ ] **9.2 Domaine prédictif orienté devant le joueur** — *partiel* **depuis S401**, en référence : le domaine épars s'étend
  **devant** un objet, le long de sa vitesse, sur l'horizon d'ADR-013 §2, élargi de `½·a_max·t²` — 100 % de 300 manœuvres
  bornées dans l'ensemble prévu ; revu chaque seconde, une source à 10 m/s y reste, et en sort à 0,62 s sans prévision
  ([preuve](validation/DOMAINE-EPARS-S401.md) §5) ; **S404** : en mer, sous le pas couplé, à 0,14 mm du domaine entier, 0,43 mm
  sur 30 s ([preuve](validation/MER-EPARS-S404.md)). Depuis S349, un domaine δ 3D suit aussi la caméra
  ([S344](validation/ARBITRAGE-3D-S344.md) §5). Manquent le joueur réel — vitesse et intentions viennent du jeu (9.3, 9.4) —,
  l'enveloppe **réservée** (T2) plutôt que calculée, la carte et l'ordonnanceur.
- [ ] **9.3 Prédiction d'objets balistiques** : point, vitesse, orientation, région utile — *partiel* **depuis S405** : le
  prédicteur du cœur (`ballistic`) — gravité et traînée quadratique, rotation libre, contact avec la houle telle qu'elle sera,
  instant exact à 10⁻¹² s dans le vide, d'ordre 4 sous traînée ; région utile sous une traînée connue à ±30 % ; paliers d'ADR-013
  §2 —, **consommé** par le domaine épars en mer : la région d'impact prête 1,11 s avant l'impact quelle que soit la cadence de
  revue, quand le suivi sans prédiction la laisse hors de l'ensemble une fois sur cinq ([preuve](validation/IMPACT-PREVU-S405.md)).
  **S537 : un corps quelconque** — le contact par le sommet le plus bas de l'enveloppe convexe : une planche tournante à 1,3·10⁻¹⁰ s de
  l'analytique, quand sa sphère englobante la faisait toucher 38,8 ms trop tôt ([preuve](validation/CORPS-QUELCONQUE-S537.md)). Manquent
  un corps non convexe, le vent, l'entrée orientée consommée par δ.
- [ ] **9.4 Objets contrôlables : paliers de confiance** ; confiance réduite par le jeu — *absent*.
- [x] **9.5 Événement prédit, confirmé ou rétracté**, sans retour arrière du temps — *validé* (S510) :
  cause et confirmation des impacts dans le journal (ADR-056) ; **le consommateur** (`wave_consumer`, le chemin d'image) — confirmation au
  même effet au bit, rejet et correction en fondu (1,6 et 2,3 % de saut d'image au plus), aucun retour du temps
  ([preuve](validation/CONSOMMATEUR-S510.md)). Le transport réseau des causes est 10.1.
- [ ] **9.6 Précalcul avant l'impact** : domaines, allocations, collisions, état initial, avance
  plus rapide que le temps réel — *absent*.
- [ ] **9.7 Hors caméra : quatre niveaux** (normal, réduit, condensé, supprimé) et persistance
  perceptuelle — *partiel* : W retiré hors champ et restitué au bit au retour (S235). Manque la
  condensation.
- [ ] **9.8 Aucun solveur ne dépasse son budget** (I-05, ordonnanceur ADR-012) — *partiel* : arrêt
  coopératif atomique de δ (S230), y compris hauteur relaxée (S268) et flux de bord
  (S270 : 638 interruptions/reprises exactes). **L'ordonnanceur existe depuis S278** et est branché
  sur la bande δ (S279, ADR-171) ; **S344** : deux candidats réels — deux domaines δ 3D — se disputent
  un budget de banc, jamais dépassé (3,720 ms au pire pour 5,
  [preuve](validation/ARBITRAGE-3D-S344.md) §2) ; **S351** : le rang 1 donne une issue à la famine, et le
  temps **mesuré** tient le budget au 99ᵉ centile quand le coût annoncé est le maximum des huit derniers pas
  (4,983 ms pour 5 ; à la médiane, 20 images au-dessus, §7). **S403** : le rang 4 et l'issue déclarée dans l'ordonnanceur
  du cœur — trois domaines réels, le budget jamais dépassé, au plus 0,965 ([preuve](validation/FAMINE-S403.md)). Manquent la
  borne murale, cet estimateur dans le cœur, et les rangs 2, 3, 6 et 7.
- [ ] **9.9 Dégradation contrôlée dans l'ordre prescrit** : taille, résolution, interactions
  lointaines, fréquence, effets — *partiel* **depuis S351** : le **rang 1** — rétrécir les non-focaux, le
  focal protégé, descente immédiate, remontée rampée (ADR-012 §4–5) — est dans l'ordonnanceur du cœur et
  reçu au banc : aucune image affamée contre 612 ([preuve](validation/ARBITRAGE-3D-S344.md) §7). **S402** : le **rang 4** en
  référence — le changement de niveau par transfert d'état ([ADR-210](adr/ADR-210-changer-de-niveau-par-transfert-d-etat.md)) ; son
  prix, 1,8 mm sur ce que 50 cm résout, 15 à 17 mm près d'une source d'une maille ([preuve](validation/NIVEAUX-S402.md)). **S403** :
  la **décision** du rang 4 dans l'ordonnanceur — le non-focal qui perd le moins descend (1,4 mm contre 14 mm au témoin qui ignore
  le contenu), remontée une par seconde —, et le rang 5 **déclaré** (`starved`), appliqué par l'hôte
  ([preuve](validation/FAMINE-S403.md)). Manquent les rangs 2, 3, 6 et 7, la victime du rang 5 choisie par le contenu, le
  régulateur PI, une bande morte de l'échelle et le **prix visuel** — une descente coupe jusqu'à 11,3 cm de δ, sans verdict. En 2D, le rétrécissement construit en S283–S285 reste
  **non reçu** — dérive de 66,994 mm, A290
  ([ATTRIBUTION-RETRECISSEMENT-S285](validation/ATTRIBUTION-RETRECISSEMENT-S285.md)) : construit, non reçu,
  il ne compte pas. **S350** : en 3D, le redimensionnement
  est reçu au bit et le coût suit la surface (§6) — le moyen dont le rang 1 se sert.
- [ ] **9.10 Profils de qualité, adaptation au matériel et à la charge** (I-16) — *absent*. **Depuis S476** (un seul PC,
  ADR-219 D2) : l'adaptation se mesure sur ce PC en bridant le budget, la résolution et le pilote (WARP comme matériel faible).
- [ ] **9.11 60 images/s avec 2 ms pour l'eau sur une scène représentative** (ADR-125) — *partiel* :
  scène filtrée S267 : GPU eau médian ~1,74 ms en 1280×720, cuisson 0,574–0,585 ms,
  pointe 2,962 ms au premier passage ; CPU ~4,1 ms, pointes ~26 ms
  ([preuve](validation/CUISSON-SILLAGE-S267.md)). Le budget global n'est pas reçu ; **δ 3D à 1,92 ms au 99ᵉ
  centile par image** (30 Hz en deux parts, S348), mesuré seul. Profil de travail : ADR-174 D3, **non
  opposable pendant la construction physique** mais toujours mesuré et publié (ADR-178 D4).
- [ ] **9.12 Aucune allocation à l'exécution** (I-06) — *partiel* : pas de δ et boucle d'image de
  l'hôte reçus (S200, S240), pas couplé avec flux de bord reçu S270 ; redimensionnement de δ 3D
  sans allocation, constaté à l'allocateur de la carte (S350). Système entier non éprouvé.
- [ ] **9.13 Dépassement critique temporaire** sans retard global perceptible — *absent*.

## 10. Multijoueur, autorité et persistance

- [ ] **10.1 Réplication des événements sources, jamais de l'état** (ADR-009) — *partiel* :
  événements versionnés et instantanés. Le transport manque.
- [ ] **10.2 Le serveur n'exécute que V** (I-10) — *partiel* : V s'exécute seul, déterministe en
  local. Aucun serveur réel. **Depuis S476** (ADR-219 D2) : un processus serveur sans fenêtre sur ce PC, puis le serveur
  du jeu (Horizon) en local.
- [ ] **10.3 Déterminisme bit à bit entre plateformes pour B, W répliqué et V** (I-03, A98) —
  *partiel* : répétabilité locale. Aucune seconde cible. **Depuis S476** (un seul PC, ADR-219 D2) : entre les chemins
  d'exécution de ce PC, comme 1.7.
- [ ] **10.4 δ sans autorité de jeu, aucun chemin d'énergie du client vers le monde, grandeurs
  dérivées autoritaires** (I-04, I-11, I-15) — *partiel* : tenu par construction du cœur, et
  **éprouvé pour un corps** : trajectoire de jeu identique au bit avec ou sans δ (S332 ; S333, 800 pas
  sur la houle — [preuve](validation/CORPS-RIGIDE-S331.md) §4, [porte D](validation/PORTE-D-S333.md)) ;
  le volume qui quitte δ n'entre jamais dans B ou V répliqués (ADR-185, S317). Non éprouvé sur réseau.
- [ ] **10.5 Grandes formes cohérentes entre clients, détails locaux libres** — *absent*.
- [ ] **10.6 Sauvegarde, reconnexion, arrivée en cours de partie** (ADR-022, C19) — *partiel* :
  branche V de C19 en local, journaux W restaurés. C19 complet manque.
- [x] **10.7 Aucun état de δ sérialisé** (I-17) — *validé* : aucun codec δ n'existe, et le démarrage
  à zéro remplace la restauration (S251, S253).
- [ ] **10.8 Chemin poussé de SPEC-006** : bus `WaveEvent`, instantanés immuables, âge publié,
  anneaux sans allocation — *partiel* : `WaveEvent` existe. Bus et canaux manquent.
- [ ] **10.9 Requêtes de jeu** : hauteur, vitesse, pente en un point, dans le budget —
  *partiel* : requête mixte composée (S236), environ 0,2 ms par point, plancher jusqu'à 12 ms ;
  requête CWM cohérente avec l'image à 0,3 mm (S262, ADR-159), branchée à aucun consommateur.

## 11. Grande échelle et très grands événements

- [ ] **11.1 Monde planétaire** : planète sphérique, coordonnées lointaines, référentiels multiples
  (ADR-002) — *partiel* : positions monde entières et ancres. Pas de sphère.
- [ ] **11.2 Nombreuses régions de mer décrites par descripteur**, transitions par paramètres (I-09) — *absent*.
- [ ] **11.3 Très grands événements** (tsunami, crash, très grand navire) : macroscopique au large,
  3D locale à l'interaction — *absent*.
- [ ] **11.4 Nombreuses sources simultanées à coût maîtrisé** — *partiel* : mutualisation des
  sillages d'un journal, table de Bessel partagée (S222, S235), filtre spectral B/sillage
  reçu (S249), cuisson optimisée (S267). Manquent la généralisation et le LOD temporel.
- [ ] **11.5 Matériel cible de livraison et seconde cible** (B7 complet, A98) — *absent*. **Depuis S476** (ADR-219 D2) :
  ce PC est la cible de livraison ; la seconde cible devient le bridage de 9.10.

## 12. Outillage auteur et données cuites

- [ ] **12.1 Cuisson reproductible, empreintes, obsolescence détectée** (SPEC-005 §7) — *partiel* :
  spectre cuit et empreintes. Détection d'obsolescence absente.
- [ ] **12.2 Éditeur de rivières** : dessin, validation bloquante, gravure (SPEC-005 §5) — *absent*.
- [ ] **12.3 Précalcul côtier stocké** (SPEC-005 §6) — *absent*.
- [ ] **12.4 Eau en amont du terrain, géoïde dans l'outil de terrain** (SPEC-005 §3–4) — *absent*. **Depuis S476**
  (ADR-219 D4) : le terrain est celui de DyingStar (ses tuiles HEALPix) ; pour nos scènes, une carte de hauteurs qui l'imite.
- [ ] **12.5 Portée d'une modification bornée par partition** (SPEC-005 §8) — *absent*.

## 13. Validation du système

- [ ] **13.1 Harnais de validation** (SPEC-003) — *partiel* : étages H1 et H3, scénarios C02 et C18. **S471–S473** : le banc
  visuel ([ADR-216](adr/ADR-216-le-banc-visuel.md)) — six vidéos de référence mesurées, nos scènes contre elles, à traitement égal
  ([REFERENCES-VIDEO-S471](validation/REFERENCES-VIDEO-S471.md), [MIROIRS-S472](validation/MIROIRS-S472.md)).
- [ ] **13.2 Les 23 cas canoniques passent sur le système** — *partiel* : sur le système, C02 (B),
  C12 (V), la branche V de C19 et **C10 depuis S331** — tirant à 0,02 %, période à 2·10⁻⁶, rapport
  avec masse ajoutée 1,408 pour 1,414 ± 15 %, par le corps rigide du cœur
  ([preuve](validation/CORPS-RIGIDE-S331.md)). Sur véhicules d'essai : C01, C03, C04, C06 (partiel),
  C08, C22, C23 ; **C07 en eau profonde depuis S519** (W, [preuve](validation/C07-PROFOND-S519.md)), **peu profond au-delà du critique
  aux deux vitesses depuis S523–S527** ([preuve](validation/C07-PLANCHER-S527.md)), **sa résonance depuis S525**
  ([preuve](validation/C07-RESONANCE-S525.md)) ; **C17 depuis S538** ([preuve](validation/C17-AIR-S538.md)). C20 depuis S512 ([preuve](validation/IMPACT-ENTREE-S512.md)). **C13 depuis S540** ([preuve](validation/C13-BULLES-S540.md)) ; **C16 depuis S542–S543** (l'inclinaison, la période, la rotation ;
  [preuve](validation/C16-ACCELERE-S542.md)) ; **C21 depuis S544–S545** (fixe et accéléré ;
  [preuve](validation/C21-MASSE-S544.md)). Non exécutés : C05, C09, C11, C14, C15. **C18 partiel**
  (vérifié S258) : le harnais tient 4 lignes sur 7 — empreinte de B en local (I-03, sans seconde
  cible), allocation refusée après scellement (I-06), plus reproductibilité et indépendance au
  chemin. Non exécutées : budget par domaine (I-05), hôte serveur sans δ ni rendu (échoue par
  construction tant qu'il n'existe pas), forces avec et sans δ (I-04 — éprouvé hors du harnais en
  S332, voir 10.4), `W_rep` entre deux profils
  (I-15), capacités lues d'un profil (I-16).
- [ ] **13.3 Les onze bancs rendent leur verdict** (B1–B11) — *partiel* : B1, B2, B4 et B7 partiels ;
  **B6** partiel depuis S333–S336 — une coque sur une houle, archétype chiffré ; manquent quatre
  archétypes et trois états de mer ([porte D](validation/PORTE-D-S333.md) §9) — ; **B10** partiel
  depuis S320 — la cavité sur le banc 2D d'APIC, dont le temps de pincement ne converge pas à trois
  mailles, couronne et jet de maille ([B10](validation/B10-APIC-S320.md)). Les autres attendent leurs
  composants, dont B8, que la porte A nomme.
- [ ] **13.4 L'eau dans le jeu** ([ADR-219](adr/ADR-219-reponses-du-2026-10-04.md) D7, ajouté en S476) — *absent* : le système
  intégré à une copie locale de **DyingStar** (Godot 4.7 depuis 2026 — 4.5 en S476 —, double précision, C#, Jolt, serveur Horizon ; copie et moteur téléchargés en S482), sur une planète du jeu, sans
  régression du jeu — une surprise pour son équipe : rien ne se publie.

---

## Décompte

| section | points | validés | partiels | absents |
|---|---:|---:|---:|---:|
| 1. Socle | 8 | 1 | 5 | 2 |
| 2. Grandes masses (B) | 9 | 0 | 5 | 4 |
| 3. Ondes (W) | 9 | 0 | 4 | 5 |
| 4. Volumique (δ) | 21 | 0 | 17 | 4 |
| 5. Volumes finis (V) | 12 | 2 | 6 | 4 |
| 6. Solides | 8 | 5 | 3 | 0 |
| 7. Secondaires | 8 | 0 | 3 | 5 |
| 8. Rendu | 10 | 0 | 10 | 0 |
| 9. Activation et budget | 13 | 1 | 8 | 4 |
| 10. Multijoueur | 9 | 1 | 7 | 1 |
| 11. Grande échelle | 5 | 0 | 2 | 3 |
| 12. Outillage | 5 | 0 | 1 | 4 |
| 13. Validation | 4 | 0 | 3 | 1 |
| **total** | **121** | **10** | **74** | **37** |

*Recompté en S321, 2026-09-22* : 4.8 (S316) et 4.12 (S320) étaient passés à partiel sans que ce
tableau suive — 51 et 66 affichés pour 53 et 64 réels. Depuis S321, `python outils/etat_projet.py
--check` compare ce tableau aux points, section par section. *S338, 2026-09-24* : **6.4** passe à partiel —
parois et corps mobiles dans δ depuis S330–S332, que la liste n'avait pas suivis — ; 4.15, 6.1, 6.3 et 6.5
corrigés sans changer de case. *S350, 2026-09-24* : **4.13** passe à partiel ; vingt et un points corrigés sans
changer de case — dix-huit sur S309–S349 (actualisation complète, ci-dessous), trois sur S350 (4.19, 9.9, 9.12).
*S351* : **4.2** et **9.9** passent à partiel — deux domaines servis ensemble, le rang 1 reçu au banc ; 1.4, 4.5 et
9.8 corrigés. *S359* : **8.5** passe à partiel — la colonne d'eau et la réfraction dans Godot. *S362* : **2.7** passe à
partiel — la référence de la houle qui sent le fond. *S365* : **8.6** passe à partiel — la caméra sous l'eau. *S367* : **7.1** passe à partiel — le champ d'écume de B. *S386* : **4.3** passe à partiel — la colonne graduée. *S393* : **4.16** passe à partiel — la cavité 3D de la sphère, contre une mesure publiée. *S396* : **4.9** passe à partiel — fusion et séparation en référence. *S401* : **9.2** passe à partiel — le domaine épars qui s'étend devant l'objet, en référence. *S405* : **9.3** passe à partiel — la prédiction balistique, consommée par le domaine épars en mer. *S408* : **4.10** passe à partiel — la bande de particules qui suit la surface, rendue aux colonnes au repos.

Trois points validés sur 120. Cela ne mesure pas l'avancement du travail. Beaucoup de points
partiels portent l'essentiel de leur difficulté, et un point validé peut être petit.

**Ce que l'actualisation de S350 a changé, et ce qu'elle n'a pas changé.** Dix-neuf points
retouchés, après relecture de chaque session de S309 à S349 contre la liste : les réceptions de
cette période avaient été reportées là où elles touchaient un point de front, pas là où elles le
touchaient de biais — masse comptée à l'interface (4.6), réflexion chiffrée (4.7), C10 sur le cœur
(13.2), verdicts R15 à R17 (8.7, 8.10) ; S350 y ajoute ses propres résultats (1.4, 4.5, 4.19,
9.9, 9.12). **Un seul point change de catégorie** — **4.13**, absent →
partiel : la coque de la porte D dans δ est la première part reçue du proche-coque, qu'ADR-001 range
dans δ. **Aucun point ne devient validé**, et c'est encore le fait principal : entre S309 et S349,
trois des quatre portes de la v1 ont été reçues — D (S338), B (S340), C (S348) — sans amener un seul
point jusqu'à son périmètre final. Ce n'est pas une contradiction : une porte reçoit une **capacité**
sur une scène ; un point de cette liste vise le **système**. S309 avait fait le même constat pour
S276–S308 — un solveur 3D, porté sur GPU et rendu en direct. Deux points sont mieux connus **en
moins bien** : 4.7, dont le bord ne réfléchit presque rien parce qu'il efface ce qu'il absorbe
(S310–S311), et 4.21, dont la dérive vaut trois fois la mer (S319). Le décompte, recalculé point
par point, est vérifié par `etat_projet.py --check`.
