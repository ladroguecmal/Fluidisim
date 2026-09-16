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

**État au S255, 2026-09-16.** Voir le décompte en fin de document.

---

## 1. Socle et architecture

- [ ] **1.1 Eau = somme de quatre couches B, W, δ, V, dans le code** — *partiel* : B, W, un δ
  candidat 2D et le noyau V existent ; B+W sont composés par le cœur (S214, S236) ; B/W sont
  raccordés à δ en 2D (S250–S254). Manquent l'articulation V↔δ et la somme rendue avec δ.
- [x] **1.2 Le cœur est une bibliothèque sans dépendance moteur** — *validé* : `water-core` n'a
  aucune dépendance (ADR-020, ADR-028).
- [ ] **1.3 Interfaces de SPEC-004** (solveur δ, solveur W, fond, solides, services d'hôte) —
  *partiel* : allocateur, tâches, horloge, journal d'hôte, `BackgroundSample`, `Volume`.
  Manquent solides, bathymétrie et GPU côté cœur.
- [ ] **1.4 Point d'entrée unique, orchestrateur des régimes** (`WaterSystem`, SPEC-004 §3) —
  *absent*, conçu.
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
  JONSWAP cuit et reproductible (ADR-100/101), 32 composantes, phases GPU. Manquent les ondes plus
  courtes que 3,5 m, B1 complet et plusieurs régions.
- [ ] **2.2 Houles longues, marée, niveau moyen variable** — *absent*.
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

- [ ] **4.1 Solveur volumique 3D à surface libre** — *partiel* : MAC x-z en 2D, surface mobile
  graphe reçue contre HOS (S237, S253, S254), pression f32 reçue jusqu'à 32 768 mailles. Manque la 3D.
- [ ] **4.2 Plusieurs domaines actifs simultanés** — *absent*.
- [ ] **4.3 Subdivision adaptative anisotrope, blocs épars** épousant la forme utile (B5) — *absent*.
- [ ] **4.4 Profondeur adaptative**, domaine qui suit un objet qui coule — *absent*.
- [ ] **4.5 Création, croissance, réduction et disparition visuellement gratuites** (I-12) —
  *partiel* : naissance à zéro reçue sous fond couplé (S251, S253). Manquent la croissance, la
  réduction et la disparition.
- [ ] **4.6 Entrée des vagues de B/W dans le domaine** — *partiel* : source volumique, surface
  mobile couplée, fond B prolongé (ADR-149 à 154), en bassin à murs ; B4 partiel. Manquent les bords ouverts et
  W au-dessus du plan moyen.
- [ ] **4.7 Frontière sans réflexion ni rupture visible** (C05) — *partiel* : éponge quadratique
  sur la vitesse (S250). Manquent les bords ouverts, la relaxation de la hauteur et C05 sur le système.
- [ ] **4.8 Sortie des perturbations vers W** (transduction δ→W, W local cosmétique ; coupure W–δ et `λ_cut` de B2) — *absent*.
- [ ] **4.9 Fusion et séparation de domaines** sans rupture — *absent*.
- [ ] **4.10 Adaptation interne** : subdivision locale dans le chaos, fusion au repos — *absent*.
- [ ] **4.11 Régime substitutif** quand δ n'est plus petit, restauré depuis graine (I-17) — *absent*.
- [ ] **4.12 Cavité et gerbe d'impact** (C20, B10) — *absent*.
- [ ] **4.13 Proche-coque et gerbe d'étrave** — *absent*.
- [ ] **4.14 Plage** : rouleau 3D, mouillage et séchage (C04) — *absent* : C04 exécuté sur un
  véhicule d'essai 1D seulement.
- [ ] **4.15 Rochers et obstacles immergés**, turbulence — *partiel* : faces coupées sur fonds
  lisses 2D (S232). Manquent les obstacles, la 3D et la turbulence.
- [ ] **4.16 Surface non graphe** : déferlement, éclaboussures détachées — *absent*.
- [ ] **4.17 Référentiel accéléré et invariance galiléenne** (C16, C06) — *absent* sur le système ;
  C06 partiel sur un véhicule d'essai 1D.
- [ ] **4.18 Conservation de la masse et de l'énergie** (C09) — *partiel* : dérive de volume
  ≤ 10⁻⁸ m sur les bancs de surface. C09 non exécuté.
- [ ] **4.19 Coût de δ compatible avec le budget** — *partiel* : carte du coût (S244) et multigrille
  (S252). Un pas à 16 384 mailles coûte environ 130 fois le budget d'eau.
- [ ] **4.20 Changement de solveur pendant une simulation** (ADR-007) — *absent*, conçu.

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

- [ ] **6.1 Flottabilité des objets importants** (C10, C11, B6) — *partiel* : véhicules d'essai 1D
  et cube statique. Rien dans le système.
- [ ] **6.2 Forces de l'eau sur les objets** : vagues, courant, turbulence, sous la frontière
  d'autorité d'ADR-008 — *absent*.
- [ ] **6.3 Un objet en mouvement produit son sillage** — *partiel* : mouvement et charge prescrits
  vers la source de pression (ADR-103). Manque le corps réel couplé.
- [ ] **6.4 Parois et corps mobiles dans δ** (C23) — *absent* sur le système ; C23 exécuté sur un
  véhicule d'essai.
- [ ] **6.5 Décor fixe comme frontière imposée** — *partiel* : fonds lisses coupés en 2D (S232).
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
- [ ] **8.2 LOD de la géométrie de surface** — *partiel* : grille projetée à pas écran. LOD du
  maillage absent.
- [ ] **8.3 LOD par source** : grille du sillage, visibilité, filtre spectral — *partiel* (S234,
  S235, S249). Manquent le filtre des impacts, le LOD spectral et le LOD temporel.
- [ ] **8.4 Écume, spray, gouttes, bulles rendus, chacun avec son LOD** — *absent*.
- [ ] **8.5 Transparence, réfraction, caustiques, particules sous-marines** — *absent*.
- [ ] **8.6 Vue sous-marine et passage de la surface** (ADR-019, B11) — *absent*.
- [ ] **8.7 Rendu de δ raccordé à B+W sans rupture visible** — *absent*.
- [ ] **8.8 Lointain et horizon sans artefact** — *partiel* : coupure spectrale B/sillage (S249) ;
  bande d'horizon mesurée (S247, S248). Pas de certificat d'absence d'alias.
- [ ] **8.9 Détails artificiels bon marché** (micro-vagues, ondes courtes) ajoutés au rendu — *absent*.
- [ ] **8.10 Crédibilité perçue validée par un regard humain** — *partiel* : protocole de revue
  ([REVUE-VISUELLE](validation/REVUE-VISUELLE.md)) ; R1 envoyée, en attente.

## 9. Activation, prédiction, budget et dégradation

- [ ] **9.1 Activation multicritère** : proximité, visibilité, taille à l'écran, regard, vitesse du
  joueur, énergie, enjeu de jeu, budget (ADR-013, B8) — *absent*.
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
  coopératif atomique de δ (S230). L'ordonnanceur manque.
- [ ] **9.9 Dégradation contrôlée dans l'ordre prescrit** : taille, résolution, interactions
  lointaines, fréquence, effets — *absent*.
- [ ] **9.10 Profils de qualité, adaptation au matériel et à la charge** (I-16) — *absent*.
- [ ] **9.11 60 images/s avec 2 ms pour l'eau sur une scène représentative** (ADR-125) — *partiel* :
  GPU eau 0,43 ms ; préparation CPU du sillage 3,1 ms pendant le forçage ; δ hors budget.
- [ ] **9.12 Aucune allocation à l'exécution** (I-06) — *partiel* : pas de δ et boucle d'image de
  l'hôte reçus (S200, S240). Système entier non éprouvé.
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
  *partiel* : requête mixte composée (S236), environ 0,2 ms par point, plancher jusqu'à 12 ms.

## 11. Grande échelle et très grands événements

- [ ] **11.1 Monde planétaire** : planète sphérique, coordonnées lointaines, référentiels multiples
  (ADR-002) — *partiel* : positions monde entières et ancres. Pas de sphère.
- [ ] **11.2 Nombreuses régions de mer décrites par descripteur**, transitions par paramètres (I-09) — *absent*.
- [ ] **11.3 Très grands événements** (tsunami, crash, très grand navire) : macroscopique au large,
  3D locale à l'interaction — *absent*.
- [ ] **11.4 Nombreuses sources simultanées à coût maîtrisé** — *partiel* : mutualisation des
  sillages d'un journal, table de Bessel partagée (S222, S235). LOD spectral et temporel absents.
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
  (statique), C22, C23. Non exécutés : C05, C07, C09, C11, C13 à C17, C20, C21 ; C18 à relire.
- [ ] **13.3 Les onze bancs rendent leur verdict** (B1–B11) — *partiel* : B1, B2, B4 et B7 partiels ;
  les autres attendent leurs composants.

---

## Décompte

| section | points | validés | partiels | absents |
|---|---:|---:|---:|---:|
| 1. Socle | 8 | 1 | 4 | 3 |
| 2. Grandes masses (B) | 9 | 0 | 2 | 7 |
| 3. Ondes (W) | 9 | 0 | 4 | 5 |
| 4. Volumique (δ) | 20 | 0 | 7 | 13 |
| 5. Volumes finis (V) | 12 | 1 | 3 | 8 |
| 6. Solides | 8 | 0 | 3 | 5 |
| 7. Secondaires | 8 | 0 | 0 | 8 |
| 8. Rendu | 10 | 0 | 5 | 5 |
| 9. Activation et budget | 13 | 0 | 5 | 8 |
| 10. Multijoueur | 9 | 1 | 7 | 1 |
| 11. Grande échelle | 5 | 0 | 2 | 3 |
| 12. Outillage | 5 | 0 | 1 | 4 |
| 13. Validation | 3 | 0 | 3 | 0 |
| **total** | **119** | **3** | **46** | **70** |

Trois points validés sur 119. Cela ne mesure pas l'avancement du travail. Beaucoup de points
partiels portent l'essentiel de leur difficulté, et un point validé peut être petit.
