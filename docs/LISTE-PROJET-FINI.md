# Liste du projet fini — ce que le système d'eau doit avoir et savoir faire

**Demandée par l'utilisateur en S255, le 2026-09-16.** C'est la liste de contrôle du projet
**terminé**, avec l'ambition complète d'[ADR-127](adr/ADR-127-ambition-complete-construction-progressive.md).
Elle ne se limite pas à ce qui est déjà construit. Les points viennent des
[intentions d'origine](sources/systeme_eau_architecture_globale.md), des
[zones ouvertes](sources/systeme_eau_zones_ouvertes_et_decisions_a_valider.md), des couches
d'[ADR-001](adr/ADR-001-decomposition-en-couches.md), des [invariants](01_INVARIANTS.md), des
spécifications, des [cas canoniques](validation/CAS-CANONIQUES.md) et des
[bancs](validation/PLAN-BENCHMARK.md).

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

**État au S309, 2026-09-20** — actualisation demandée par l'utilisateur avec la stratégie en
trois systèmes ([ADR-178](adr/ADR-178-strategie-en-trois-systemes-physiques.md)). **Seuls les
points que S277–S308 ont réellement bougés sont retouchés** ; les autres gardent leur état de
S276 et sa date. Chaque état modifié cite la preuve qui le modifie. Le décompte porte sur le
périmètre final, pas sur le nombre de correctifs ou de tests.

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
  Manquent solides, bathymétrie et GPU côté cœur.
- [ ] **1.4 Point d'entrée unique, orchestrateur des régimes** (`WaterSystem`, SPEC-004 §3) —
  *partiel* **depuis S278** : `scheduler.rs` décide quels domaines vivent et avec quel budget —
  sac à dos d'ADR-012 §1, hystérésis et durées de vie d'ADR-013 §5, poids bornés par ADR-170 ;
  éprouvé ([ORDONNANCEUR-S278](validation/ORDONNANCEUR-S278.md)), et **branché en S279 sur la
  bande δ de l'afficheur**, qui s'éteint et se rallume toute seule
  ([ORDONNANCEUR-S279](validation/ORDONNANCEUR-S279.md), ADR-171). Manquent le point d'entrée
  `WaterSystem` lui-même, la dégradation (ADR-012 §4), la forme des domaines, plusieurs candidats
  réels, et la sortie de l'exclusion absorbante (L336).
- [ ] **1.5 Grille 3D de référence stable** : adressage, zones actives, échanges client/serveur —
  *absent*, conçu (ADR-006).
- [ ] **1.6 Cellules, domaines et solveurs distincts, niveaux d'activité des cellules** — *absent*,
  conçu (ADR-006).
- [ ] **1.7 Horloge de simulation entière et phases déterministes** (ADR-003) — *partiel* : temps
  entier en µs, phases en virgule fixe ; déterminisme reçu localement, pas entre plateformes.
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
- [ ] **2.6 Courants macroscopiques à niveau de détail propre**, du vecteur au champ 3D — *absent*,
  conçu (ADR-011).
- [ ] **2.7 Bathymétrie** : hauts-fonds, effet sur les vagues avant la zone physique — *absent* :
  fond uniforme seulement (A234, S116-2).
- [ ] **2.8 Précalcul côtier et météo** (SPEC-005 §6) — *absent*.
- [ ] **2.9 Dérivées du fond pour les couches volumiques**, sous et au-dessus du plan moyen —
  *partiel* : B reçu en eau profonde uniforme (ADR-113, S177 ; ADR-154, S254). Manquent la
  profondeur finie et la bathymétrie.

## 3. Ondes propagatives (W)

- [ ] **3.1 Anneaux d'impact dispersifs** (objet qui tombe) — *partiel* : impacts radiaux, table
  de Bessel, générateur calibré, admission ; horizon reçu 56 s, eau profonde. Manquent l'eau peu
  profonde, et la gerbe (point 4.12).
- [ ] **3.2 Sillages de bateaux**, trajectoires et vitesses quelconques, eau profonde et peu
  profonde (C07) — *partiel* : source de pression mobile par tronçons, scène à trois sillages ;
  domaine honnête de 89 m et 18,5 s (ADR-132). Manquent C07, les durées longues et l'eau peu
  profonde.
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
  ([S305](validation/CUVE-GPU-S305.md)). **Manque le périmètre final** : la surface est un graphe,
  donc ni cavité, ni jet, ni déferlement (4.16) — seconde représentation, lot 5 d'ADR-178.
- [ ] **4.2 Plusieurs domaines actifs simultanés** — *absent*.
- [ ] **4.3 Subdivision adaptative anisotrope, blocs épars** épousant la forme utile (B5) — *absent*.
- [ ] **4.4 Profondeur adaptative**, domaine qui suit un objet qui coule — *absent*.
- [ ] **4.5 Création, croissance, réduction et disparition visuellement gratuites** (I-12) —
  *partiel* : naissance à zéro reçue sous fond couplé (S251, S253). Manquent la croissance, la
  réduction et la disparition.
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
  ([S302](validation/SCENE-DELTA3D-S302.md)).
  Manquent la houle progressive traversante reçue sur une durée utile, les frontières
  générales du total et W au-dessus du plan moyen ; B4 reste partiel. **Et l'entrée n'est
  comptée par aucun bilan** : masse, quantité de mouvement et énergie ne sont pas mesurées à
  l'interface (A302, lot 1 d'ADR-178).
- [ ] **4.7 Frontière sans réflexion ni rupture visible** (C05) — *partiel* : éponge quadratique
  sur la vitesse (S250) et relaxation de hauteur reçue (S268, ADR-164). Effet du bord
  absorbant 0,14–0,16 % sur un paquet à fond nul, par différence contrôlée aux domaines
  longs ([S269](validation/REFLEXION-PAQUET-S269.md)). Manquent les autres régimes,
  la transparence générale et C05 sur le système ; la mesure brute S269 reste refusée.
  **En 3D, l'éponge et les bandes existent sur les quatre côtés et R11 a jugé le raccord
  « invisible » — mais la réflexion n'a jamais été chiffrée** (A302). Un jugement visuel ne vaut
  pas une mesure (ADR-178 D3).
- [ ] **4.8 Sortie des perturbations vers W** (transduction δ→W, W local cosmétique ; coupure W–δ
  et `λ_cut` de B2) — *partiel* : le chemin existe depuis S312 sur la référence CPU — ligne de
  contrôle, identification, train orienté de W (S314) — et **S316 l'a qualifié** propriété par
  propriété : primitive exacte, raccord à un degré et 2 à 4 % de spectre près, le reste à δ
  ([ordre C](validation/ORDRE-C-S316.md)). Le volume net est reçu par une région locale (S317,
  [ordre D](validation/RESTITUTION-S317.md)) — **en eau calme seulement** : sous une vraie mer, δ dérive (A289, S319). Manquent le couplage complet (ordre E), la production GPU et le sens W → δ. Lot 2 d'[ADR-178](adr/ADR-178-strategie-en-trois-systemes-physiques.md) D7.
- [ ] **4.9 Fusion et séparation de domaines** sans rupture — *absent*.
- [ ] **4.10 Adaptation interne** : subdivision locale dans le chaos, fusion au repos — *absent*.
- [ ] **4.11 Régime substitutif** quand δ n'est plus petit, restauré depuis graine (I-17) — *absent*.
- [ ] **4.12 Cavité et gerbe d'impact** (C20, B10) — *partiel* — **la cavité est portée sur le banc
  2D d'APIC** (S320, [B10](validation/B10-APIC-S320.md)) : pincement indépendant de l'échelle, masse
  exacte ; mais **son temps ne converge pas encore** à trois mailles (2,20 → 2,30 → 2,40 `√(D/g)`, S326),
  et à maille fine la fermeture de la bulle sans pression emballe le calcul (A311).
  Hors de δ : sa surface est une fonction hauteur (ADR-175 D5). Manquent
  la gerbe, qui suit la maille (A312), la bulle, qui n'est pas de l'air (A311), le raccord aux
  colonnes, la 3D et C20. Lot 5 d'ADR-178, APIC retenue par l'utilisateur (ADR-186).
- [ ] **4.13 Proche-coque et gerbe d'étrave** — *absent*.
- [ ] **4.14 Plage** : rouleau 3D, mouillage et séchage (C04) — *absent* : C04 exécuté sur un
  véhicule d'essai 1D seulement.
- [ ] **4.15 Rochers et obstacles immergés**, turbulence — *partiel* : faces coupées sur fonds
  lisses en 2D (S232, ordres 1,947–1,966) **et en 3D depuis S324** — référence CPU, mode linéaire,
  identique au bit à la 2D sans `y`, ordre 1,956 sur une bosse ([preuve](validation/FACES-COUPEES-3D-S324.md)).
  Petites cellules préconditionnées (S326) ; **mode mobile** depuis S328 — au bit de la 2D sans `y`,
  ordre 1,954 sur la bosse ; **obstacles immergés quelconques** depuis S329 — sphère d'ordre 1,966,
  Archimède exact au niveau discret ; **en mouvement imposé** depuis S330 — masse ajoutée d'une sphère
  à 1,6 % de la théorie. Manquent la rotation, les obstacles qui percent la surface, le couplage à B/W
  sur fond coupé et la turbulence — **aucun modèle de turbulence n'existe nulle part dans le dépôt**. Lot 3 d'ADR-178.
- [ ] **4.16 Surface non graphe** : déferlement, éclaboussures détachées — *absent*. **C'est le
  point le plus lourd de la liste** : il demande un **second solveur**, pas une extension du
  premier (ADR-175 D5). Lot 5 d'ADR-178 ; commande aussi 4.12, 4.13, 4.14 et 7.2.
- [ ] **4.17 Référentiel accéléré et invariance galiléenne** (C16, C06) — *absent* sur le système ;
  C06 partiel sur un véhicule d'essai 1D.
- [ ] **4.18 Conservation de la masse et de l'énergie** (C09) — *partiel*, **et la masse est
  désormais comptée**. S310 : bilan **exact par télescopage** tenu par les deux pas, plancher
  publié ; cuve fermée à 7,4·10⁻¹² m de dérive sur 5 s, murs à zéro exact ; scène couplée à
  1,28·10⁻¹⁰ m³ de résidu ([preuve](validation/BILAN-MASSE-S310.md)). **A298 n'est donc pas une
  fuite de volume du schéma.** Dissipation numérique mesurée : 0,0935 % par seconde. **Manquent**
  le compteur sur la carte, et les bilans d'**énergie** et de **quantité de mouvement** — publiés
  comme états, termes manquants nommés (travail de la pression au bord, flux advectif). C09 non
  exécuté. **S317** : le volume qui **quitte** δ a un receveur local, et le bilan δ + régions se
  ferme au résidu ([ordre D](validation/RESTITUTION-S317.md)) — la représentation, pas le monde.
- [ ] **4.19 Coût de δ compatible avec le budget** — *partiel* : carte du coût (S244), multigrille
  (S252), **multigrille du mode mobile (S274, ADR-167)** : le pas couplé à 16 384 mailles passe de
  280 à 49 ms, environ 24 fois le budget d'eau. **S276 : δ en direct à 40 images/s** — bande de
  6 656 mailles, 21,7 ms par image (échantillonnage du fond par grille identique au bit, départ
  depuis la pression publiée, ADR-169), un pas par image (précision S275), zéro allocation ;
  ≈ 11 fois le budget. **S299–S302 : le GPU et la 3D existent** — le pas couplé entier est
  résident sur la carte, **0,84 ms à 64 cycles sur 27 648 mailles**
  ([S301](validation/DELTA3D-PAS-GPU-S301.md)) ; sur la scène de 376 320 mailles à 32 cycles,
  **4,62 ms par pas contre 2 ms** visés, et le tampon des faces plafonne le domaine à 1,15 M faces
  ([S302](validation/SCENE-DELTA3D-S302.md) §2). Manque la cadence de δ découplée de l'image.
  **Le budget n'est plus opposable pendant la construction** (ADR-178 D4) : il reste mesuré.
- [ ] **4.20 Changement de solveur pendant une simulation** (ADR-007) — *absent*, conçu.
- [ ] **4.21 Cohérence de phase entre δ et B+W sur la durée de vie d'un domaine** — *absent*.
  Besoin découvert S274 : B est linéaire, un δ fidèle dérive de la dispersion d'amplitude
  (0,85–0,87 fois Stokes mesuré). Sous `ak` = 0,06, environ 7 cm en une minute : surface rendue
  différente de la surface de jeu. Options et déclencheur : A289.

## 5. Volumes finis et inondations (V)

- [x] **5.1 Contenants à volume entier qui se vident** par orifice et déversoir (C12) — *validé* :
  0,0824 % contre l'analytique, pas de 100 ms, refus atomiques
  ([S224](validation/NOYAU-V-S224.md)).
- [ ] **5.2 Géométrie réelle des contenants** : gravité dirigée, plans orientés, formes non
  convexes — *partiel* (S226, S228). Manquent la précision des grands volumes (A269) et les formes
  courbes cuites depuis les assets.
- [ ] **5.3 Fuites, transferts et débordements entre contenants** — *partiel* : orifices et
  déversoirs entre nœuds, arrivées collectives (S227). Manque le débordement vers l'extérieur.
- [ ] **5.4 Vannes et pompes** — *absent*.
- [ ] **5.5 Pluie selon l'exposition au ciel, absorption par le sol** — *absent*.
- [ ] **5.6 Seuil adaptatif à l'échelle du contenant** — *absent*.
- [ ] **5.7 Plusieurs liquides** (`liquid_id`, A17) — *absent*.
- [ ] **5.8 Réseau fermé sous pression** — *absent*, reporté en v2 par ADR-010.
- [ ] **5.9 Compartiments, brèches, inondation de navire, limitée par l'air** (C17, ADR-015) — *absent*.
- [ ] **5.10 Articulation V↔δ** : V expose sa surface, déclenche δ, garde la masse (C21, ADR-025) —
  *absent*.
- [ ] **5.11 Eaux souterraines** — *absent*.
- [ ] **5.12 Capture et restauration de V** — *partiel* : noyau restauré au bit (S229). Manquent le
  stockage durable et le réseau.

## 6. Solides et flottabilité

- [ ] **6.1 Flottabilité des objets importants** (C10, C11, B6) — *partiel* : **corps rigide à six degrés
  de liberté dans le cœur** depuis S331 — proxy sur B + W, masse ajoutée ; C10 tenu à 0,02 % sur le tirant
  ([preuve](validation/CORPS-RIGIDE-S331.md)) ; **sa coque pilotée dans δ** depuis S332, sans que δ touche la
  trajectoire (I-04). Manque la houle derrière la requête d'eau.
- [ ] **6.2 Forces de l'eau sur les objets** : vagues, courant, turbulence, sous la frontière
  d'autorité d'ADR-008 — *absent*.
- [ ] **6.3 Un objet en mouvement produit son sillage** — *partiel* : mouvement et charge prescrits
  vers la source de pression (ADR-103). Manque le corps réel couplé.
- [ ] **6.4 Parois et corps mobiles dans δ** (C23) — *absent* sur le système ; C23 exécuté sur un
  véhicule d'essai.
- [ ] **6.5 Décor fixe comme frontière imposée** — *partiel* : fonds lisses coupés **en 2D
  seulement** (S232) ; le solveur 3D n'a pas de faces coupées. Lot 3 d'ADR-178, qui conditionne
  6.1, 6.4 et les essais 2 et 3 du banc de la piscine.
- [ ] **6.6 Grands navires** — *absent*.
- [ ] **6.7 Acteur poussé, renversé ou déplacé par l'eau** (vague, poche d'air) — *absent*.
- [ ] **6.8 Impulsion d'entrée dans l'eau** (slamming, C20) — *absent*.

## 7. Phénomènes secondaires

- [ ] **7.1 Écume et moutons** (C14, B9, champ d'écume de SPEC-006 §4) — *absent*, conçu (ADR-014).
- [ ] **7.2 Spray, embruns, gouttelettes** — *absent*.
- [ ] **7.3 Microbulles visuelles** — *absent*.
- [ ] **7.4 Grosses bulles et poches d'air physiques** (C13, ADR-015) — *absent*.
- [ ] **7.5 Air comprimé, vide, eau dans le vide** (ADR-015) — *absent*.
- [ ] **7.6 Glace et vapeur** (C15, ADR-017) — *absent*.
- [ ] **7.7 Danger et traversabilité**, publiés par tuiles (ADR-018, SPEC-006 §5) — *absent*.
- [ ] **7.8 Audio de l'eau** (ADR-016, SPEC-006 §4.2) — *absent*.

## 8. Rendu et niveaux de détail visuels

- [ ] **8.1 Rendu temps réel de la surface sur GPU** — *partiel* : hôte séparé B + impacts +
  sillages (ADR-130, S211–S249), habillage de banc ; le rendu ne pilote pas la physique (I-13). Manque l'intégration au moteur du jeu.
- [ ] **8.2 LOD de la géométrie de surface** — *partiel* : grille projetée à pas écran. Manquent le LOD
  du maillage et le choix déplacement ou normales selon la vue (critère de parallaxe chiffré S257).
- [ ] **8.3 LOD par source** : grille du sillage, visibilité, filtre spectral — *partiel* (S234,
  S235, S249) : filtre spectral reçu pour B et sillage ; cuisson des huit bandes
  accélérée de 46 % au bit (S267). Manquent le filtre des impacts, la généralisation
  aux autres sources et le LOD temporel.
- [ ] **8.4 Écume, spray, gouttes, bulles rendus, chacun avec son LOD** — *absent*.
- [ ] **8.5 Transparence, réfraction, caustiques, particules sous-marines** — *absent*.
- [ ] **8.6 Vue sous-marine et passage de la surface** (ADR-019, B11) — *absent*.
- [ ] **8.7 Rendu de δ raccordé à B+W sans rupture visible** — *partiel* : **S275, ADR-168** —
  bande δ couplée sous houle à crêtes longues, précalculée hors budget et rejouée dans `viewer/`
  (touche D : B seul, B+δ 4 ms, B+δ au pas d'image), couche GPU à 7·10⁻⁸ m de sa lecture CPU,
  scène S201 inchangée au bit ; revue R10 en attente. **S276 : en direct** (`--delta-direct`, un pas
  par image, identique au bit au rejeu, 40 images/s). **S302 : δ en 3D rendu en direct** depuis la
  seule surface publiée (I-13, ADR-175 D7), Catmull-Rom bicubique, fondu de 3 m, **197 Hz** ; le
  rendu existant reste identique au bit sans la couche
  ([preuve](validation/SCENE-DELTA3D-S302.md)). Manquent le budget, une frontière δ↔B sans rupture
  autre qu'un fondu de rendu, et une tolérance de pente d'image.
- [ ] **8.8 Lointain et horizon sans artefact** — *partiel* : coupure spectrale B/sillage (S249) ;
  bande d'horizon mesurée (S247, S248) ; fin de grille à l'horizon géométrique sous le ciel clair
  (S262). Pas de certificat d'absence d'alias.
- [ ] **8.9 Détails artificiels bon marché** (micro-vagues, ondes courtes) ajoutés au rendu — *partiel* :
  queue du spectre de B en pentes par pixel, filtrée par l'empreinte, +0,38 ms GPU (ADR-155, S256).
  Manquent les capillaires, la queue des perturbations W et le LOD de la queue.
- [ ] **8.10 Crédibilité perçue validée par un regard humain** — *partiel* : protocole de revue
  ([REVUE-VISUELLE](validation/REVUE-VISUELLE.md)). Premier verdict (R1, « trop lisse ») mesuré et
  traité ; **R7 accepté S266**, après lissage des reflets entre les crêtes (ADR-161).
  Optimisations S266/S267 reçues. **R11 reçu S303** (δ sans artefact, raccord invisible) ;
  **R12/R13 annulées** — leurs images tournaient options acceptées éteintes (L349) ; **R14 reçu
  S308**, et sa troisième image devient la **référence interne provisoire** de l'océan (ADR-178
  D2). L'écart à une photographie réelle est désormais **chiffré** (`outils/cible_image.py`) et ce
  qu'il en reste est **spatial**, hors de portée de l'optique (L351,
  [confrontation](registres/TROIS-SYSTEMES-S308.md) §1). Autres poses, animation et scénarios
  restent à valider perceptivement. Ce verdict local ne clôt pas la crédibilité du système, et
  **une validation visuelle ne remplace pas une validation numérique** (ADR-178 D3).

## 9. Activation, prédiction, budget et dégradation

- [ ] **9.1 Activation multicritère** : proximité, visibilité, taille à l'écran, regard, vitesse du
  joueur, énergie, enjeu de jeu, budget (ADR-013, B8) — *partiel* **depuis S278** : le **mécanisme**
  qui consomme les critères existe — sac à dos sous budget, `P/C` décroissant, hystérésis
  ([ORDONNANCEUR-S278](validation/ORDONNANCEUR-S278.md), ADR-170). Manquent **les critères
  eux-mêmes** : `W_gameplay` vient du jeu et `W_perception` du rendu, et aucun n'est calculé ;
  aucun banc B8 ne fixe leurs seuils.
- [ ] **9.2 Domaine prédictif orienté devant le joueur** — *absent*.
- [ ] **9.3 Prédiction d'objets balistiques** : point, vitesse, orientation, région utile — *absent*.
- [ ] **9.4 Objets contrôlables : paliers de confiance** ; confiance réduite par le jeu — *absent*.
- [ ] **9.5 Événement prédit, confirmé ou rétracté**, sans retour arrière du temps — *partiel* :
  cause et confirmation des impacts dans le journal (ADR-056). Manque le consommateur.
- [ ] **9.6 Précalcul avant l'impact** : domaines, allocations, collisions, état initial, avance
  plus rapide que le temps réel — *absent*.
- [ ] **9.7 Hors caméra : quatre niveaux** (normal, réduit, condensé, supprimé) et persistance
  perceptuelle — *partiel* : W retiré hors champ et restitué au bit au retour (S235). Manque la
  condensation.
- [ ] **9.8 Aucun solveur ne dépasse son budget** (I-05, ordonnanceur ADR-012) — *partiel* : arrêt
  coopératif atomique de δ (S230), y compris hauteur relaxée (S268) et flux de bord
  (S270 : 638 interruptions/reprises exactes). **L'ordonnanceur existe depuis S278** et est branché
  sur la bande δ (S279, ADR-171). Manquent la borne murale, la dégradation d'ADR-012 §4 et
  plusieurs candidats réels se disputant un budget.
- [ ] **9.9 Dégradation contrôlée dans l'ordre prescrit** : taille, résolution, interactions
  lointaines, fréquence, effets — *absent* au sens de cette liste : le **rétrécissement** d'un
  domaine est construit (S283–S285) mais **non reçu** — dérive de 66,994 mm, A290
  ([ATTRIBUTION-RETRECISSEMENT-S285](validation/ATTRIBUTION-RETRECISSEMENT-S285.md)). Construit et
  non reçu ne vaut pas partiel (§ « Comment lire un point »).
- [ ] **9.10 Profils de qualité, adaptation au matériel et à la charge** (I-16) — *absent*.
- [ ] **9.11 60 images/s avec 2 ms pour l'eau sur une scène représentative** (ADR-125) — *partiel* :
  scène filtrée S267 : GPU eau médian ~1,74 ms en 1280×720, cuisson 0,574–0,585 ms,
  pointe 2,962 ms au premier passage ; CPU ~4,1 ms, pointes ~26 ms
  ([preuve](validation/CUISSON-SILLAGE-S267.md)). Le budget global n'est pas reçu ; **δ 3D mesuré
  à 4,62 ms par pas sur la scène de S302**, contre 2 ms. Profil de travail : ADR-174 D3, **non
  opposable pendant la construction physique** mais toujours mesuré et publié (ADR-178 D4).
- [ ] **9.12 Aucune allocation à l'exécution** (I-06) — *partiel* : pas de δ et boucle d'image de
  l'hôte reçus (S200, S240), pas couplé avec flux de bord reçu S270. Système entier non éprouvé.
- [ ] **9.13 Dépassement critique temporaire** sans retard global perceptible — *absent*.

## 10. Multijoueur, autorité et persistance

- [ ] **10.1 Réplication des événements sources, jamais de l'état** (ADR-009) — *partiel* :
  événements versionnés et instantanés. Le transport manque.
- [ ] **10.2 Le serveur n'exécute que V** (I-10) — *partiel* : V s'exécute seul, déterministe en
  local. Aucun serveur réel.
- [ ] **10.3 Déterminisme bit à bit entre plateformes pour B, W répliqué et V** (I-03, A98) —
  *partiel* : répétabilité locale. Aucune seconde cible.
- [ ] **10.4 δ sans autorité de jeu, aucun chemin d'énergie du client vers le monde, grandeurs
  dérivées autoritaires** (I-04, I-11, I-15) — *partiel* : tenu par construction du cœur. Non éprouvé sur réseau.
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
- [ ] **11.5 Matériel cible de livraison et seconde cible** (B7 complet, A98) — *absent*.

## 12. Outillage auteur et données cuites

- [ ] **12.1 Cuisson reproductible, empreintes, obsolescence détectée** (SPEC-005 §7) — *partiel* :
  spectre cuit et empreintes. Détection d'obsolescence absente.
- [ ] **12.2 Éditeur de rivières** : dessin, validation bloquante, gravure (SPEC-005 §5) — *absent*.
- [ ] **12.3 Précalcul côtier stocké** (SPEC-005 §6) — *absent*.
- [ ] **12.4 Eau en amont du terrain, géoïde dans l'outil de terrain** (SPEC-005 §3–4) — *absent*.
- [ ] **12.5 Portée d'une modification bornée par partition** (SPEC-005 §8) — *absent*.

## 13. Validation du système

- [ ] **13.1 Harnais de validation** (SPEC-003) — *partiel* : étages H1 et H3, scénarios C02 et C18.
- [ ] **13.2 Les 23 cas canoniques passent sur le système** — *partiel* : sur le système, C02 (B),
  C12 (V) et la branche V de C19. Sur véhicules d'essai : C01, C03, C04, C06 (partiel), C08, C10
  (statique), C22, C23. Non exécutés : C05, C07, C09, C11, C13 à C17, C20, C21. **C18 partiel**
  (vérifié S258) : le harnais tient 4 lignes sur 7 — empreinte de B en local (I-03, sans seconde
  cible), allocation refusée après scellement (I-06), plus reproductibilité et indépendance au
  chemin. Non exécutées : budget par domaine (I-05), hôte serveur sans δ ni rendu (échoue par
  construction tant qu'il n'existe pas), forces avec et sans δ (I-04), `W_rep` entre deux profils
  (I-15), capacités lues d'un profil (I-16).
- [ ] **13.3 Les onze bancs rendent leur verdict** (B1–B11) — *partiel* : B1, B2, B4 et B7 partiels ;
  les autres attendent leurs composants.

---

## Décompte

| section | points | validés | partiels | absents |
|---|---:|---:|---:|---:|
| 1. Socle | 8 | 1 | 5 | 2 |
| 2. Grandes masses (B) | 9 | 0 | 3 | 6 |
| 3. Ondes (W) | 9 | 0 | 4 | 5 |
| 4. Volumique (δ) | 21 | 0 | 9 | 12 |
| 5. Volumes finis (V) | 12 | 1 | 3 | 8 |
| 6. Solides | 8 | 0 | 3 | 5 |
| 7. Secondaires | 8 | 0 | 0 | 8 |
| 8. Rendu | 10 | 0 | 7 | 3 |
| 9. Activation et budget | 13 | 0 | 6 | 7 |
| 10. Multijoueur | 9 | 1 | 7 | 1 |
| 11. Grande échelle | 5 | 0 | 2 | 3 |
| 12. Outillage | 5 | 0 | 1 | 4 |
| 13. Validation | 3 | 0 | 3 | 0 |
| **total** | **120** | **3** | **53** | **64** |

*Recompté en S321, 2026-09-22* : 4.8 (S316) et 4.12 (S320) étaient passés à partiel sans que ce
tableau suive — 51 et 66 affichés pour 53 et 64 réels. Depuis S321, `python outils/etat_projet.py
--check` compare ce tableau aux points, section par section.

Trois points validés sur 120. Cela ne mesure pas l'avancement du travail. Beaucoup de points
partiels portent l'essentiel de leur difficulté, et un point validé peut être petit.

**Ce que l'actualisation de S309 a changé, et ce qu'elle n'a pas changé.** Le décompte a été
**recalculé point par point**, pas corrigé à vue — et le total de S276 était faux de deux unités :
la ligne « Socle » comptait encore **1.4** en absent alors que S278 l'avait rendu partiel, ce que
l'audit [S293](registres/BILAN-GLOBAL-S293.md) §2 avait signalé sans que la table soit refaite.
Un seul point change de catégorie aujourd'hui — **9.1**, absent → partiel, l'ordonnanceur ayant
été écrit en S278. Quatorze autres voient leur **texte** corrigé sans changer de case : pour la
plupart parce que la 3D de δ, absente au S276, existe depuis S297 et tourne sur la carte depuis
S301. **Aucun point ne devient validé**,
et c'est le fait le plus important de ce décompte : entre S276 et S308, le dépôt a construit un
solveur 3D, l'a porté sur GPU et l'a rendu en direct — sans amener **un seul** point de cette
liste jusqu'à son périmètre final. Trois points sont même mieux compris **en moins bien** qu'avant
(4.8, 4.18, 4.7) : ce que S308 a découvert, c'est qu'ils étaient surestimés.
