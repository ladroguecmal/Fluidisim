# La liste du projet fini, rangée par dépendance

*Ouvert en S352, 2026-09-24, par [ADR-190](../adr/ADR-190-apres-la-v1-la-liste-entiere.md) D3* — décision de
l'utilisateur : après la v1, l'objectif des sessions est la [liste du projet fini](../LISTE-PROJET-FINI.md) entière.
La liste dit **quoi** ; ce registre dit, pour chacun de ses points non validés, **ce qu'il attend et ce qu'il
débloque** ; la [feuille de route](../FEUILLE-DE-ROUTE.md) en tire **l'ordre**. Ici, rien ne change d'état.

## Comment le lire, et comment il se tient

- **sys.** — A haute mer (B + W), B volumique (δ), C couplage, H hors des trois : rangement par énoncé, refait point
  par point, [TROIS-SYSTEMES-S308](TROIS-SYSTEMES-S308.md) §8 n'en publiant que les comptes. Sur les 117 points ouverts :
  A 20, B 25, C 7, H 65 ; S309 comptait B 29 et H 64 sur 120 — des points à cheval, rangés ici hors des trois.
- **maintenant** — ce qu'une session peut en faire aujourd'hui, sans rien attendre ; « — » quand tout attend.
- **attend** — les points dont son périmètre final a besoin. Une attente **extérieure** (ADR-190 D5) va en colonne front.
- **débloque** — calculé : les points ouverts qui l'attendent.
- **front** — calculé. **0** : rien d'autre à attendre qu'une session ; **n** : après un point du front n − 1 ; **E** :
  un fait ou une action de l'utilisateur, directement ou par un point attendu. Un point E peut avoir une part faisable
  **maintenant**.
- **Tenue** — les données vivent dans [`outils/dependances_liste.py`](../../outils/dependances_liste.py) :
  `--ecrire` régénère les tables ci-dessous, et `etat_projet.py --check` refuse un registre qui ne suit plus la liste.
  Une dépendance se corrige dans les données, jamais dans les tables.

## Ce que les fronts disent

**S476** ([ADR-219](../adr/ADR-219-reponses-du-2026-10-04.md)) : onze attentes extérieures levées — le réseau est le nôtre, un seul PC
(les points à second matériel reformulés), le jeu est DyingStar, le terrain le sien, l'écume par les vidéos ; 13.4 ajouté. **118
points ouverts (dont 5.11, hors du périmètre) : 46 au front 0, 32 au front 1, 27 aux fronts 2 à 5, 13 en E** — les verdicts (4.5, 8.7,
8.10, 9.9), la fin du projet (2.8, 7.8) et ce qui les attend. Le texte ci-dessous est celui de S369.

- **36 points au front 0** : une session peut les faire avancer tout de suite, sans décision ni fait extérieur ;
  **25 au front 1**, 18 aux fronts 2 à 5. Le plus long chemin mène à 11.3, les très grands événements :
  4.16 → 7.4 → 7.5 → 5.9 → 6.6 → 11.3.
- **Deux points commandent le plus**, en aval transitif : **4.16**, la surface non graphe — 22 points, dont 4.1,
  4.12, 4.13, 4.14, 7.2 et 7.4 — et **2.7**, la bathymétrie — 20 : lacs, rivières, réfraction, déferlement de W,
  tsunamis, plage. Suivent 10.1 (16), 7.4 (12), 3.6 et 1.5 (9 chacun).
- **38 points attendent un fait extérieur ou une décision, dont 19 directement** (S369, après
  [ADR-197](../adr/ADR-197-reponses-du-2026-09-26.md) et [ADR-198](../adr/ADR-198-la-voie-d-a289.md)). Une cause en commande 14 : **le réseau** (10.1 : *« pas
  encore »* — aucun format n'existe, le multijoueur reste) — transport, serveur, sauvegarde, et, par les échanges
  client/serveur de la grille de référence (1.5), la grille, les blocs, la fusion et le régime substitutif. Puis la
  **météo**, placée à la fin (2.8, quatre points), le **verdict** sur le prix du rang 1 (9.9, trois) ; les autres, un fait
  chacun ; la voie d'A289 est tranchée (4.8 et 4.21 au front 0). 5.11 n'attend plus rien : il est hors du
  périmètre.
- **Pour l'ordre** : commencer par ce qui débloque le plus sans rien demander — 4.16, déjà une session sur deux
  (ADR-184 D1, ADR-190 D4), et 2.7 — ; réunir les portes de la v1 en une scène vivante (6.4, 4.2, 4.19) ; poser
  chaque question E quand son point bloque, pas avant. L'ordre lui-même est dans la feuille de route, §3 ter.

## Les points

<!-- tables : outils/dependances_liste.py --ecrire -->

| front | points | lesquels |
|---|---:|---|
| **0** | 46 | 1.7, 1.8, 2.6, 2.7, 3.2, 3.8, 3.9, 4.2, 4.4, 4.7, 4.8, 4.15, 4.16, 4.18, 4.19, 4.21, 5.2, 5.4, 5.6, 5.7, 5.10, 7.1, 7.6, 7.7, 8.1, 8.2, 8.3, 8.5, 8.8, 8.9, 9.1, 9.2, 9.3, 9.7, 9.8, 9.10, 9.13, 10.1, 10.2, 10.3, 10.8, 10.9, 11.2, 12.1, 12.4, 13.1 |
| **1** | 29 | 1.1, 1.3, 1.5, 2.1, 2.3, 2.4, 2.9, 3.6, 3.7, 4.1, 4.6, 4.13, 4.17, 4.20, 5.8, 5.12, 6.2, 7.2, 7.4, 8.6, 9.4, 9.6, 9.11, 10.4, 10.5, 11.1, 11.4, 11.5, 12.5 |
| **2** | 11 | 1.6, 2.5, 3.3, 3.4, 3.5, 4.9, 4.12, 6.7, 7.5, 10.6, 12.2 |
| **3** | 5 | 3.1, 4.3, 4.14, 5.9, 7.3 |
| **4** | 5 | 4.10, 6.6, 8.4, 13.2, 13.3 |
| **5** | 2 | 11.3, 13.4 |
| **E** | 13 | 1.4, 2.2, 2.8, 4.5, 4.11, 5.5, 5.11, 7.8, 8.7, 8.10, 9.9, 9.12, 12.3 |

**Ce qu'on demandera à l'utilisateur**, au moment où le point bloque (ADR-190 D5) :

| point | attente extérieure |
|---|---|
| **2.8** | la fin du projet : météo et son en dernier (ADR-197 D5) |
| **4.5** | un verdict visuel des passages (A319) |
| **5.11** | hors du périmètre par décision de l'utilisateur (ADR-197 D4) ; ne se rouvre que par lui |
| **7.8** | la fin du projet, par Wwise, l'audio du jeu (ADR-197 D5, ADR-219) |
| **8.7** | le verdict de la frontière (R18 reçu, ADR-197 D8) |
| **8.10** | les verdicts de l'utilisateur : poses, animation, scénarios |
| **9.9** | un verdict visuel du prix du rang 1 (A319) |

### 1. Socle et architecture

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **1.1** Eau = somme de quatre couches B, W, δ, V, dans le code | C | — | 3.9, 4.8, 5.10 | — | **1** |
| **1.3** Interfaces de SPEC-004 | H | l'interface des solides de SPEC-004 §7, sur la paroi de δ existante | 2.7 | — | **1** |
| **1.4** Point d'entrée unique, orchestrateur des régimes | H | `WaterSystem`, qui porte l'ordonnanceur, l'oubli et l'estimateur de coût | 1.6, 9.9 | 9.12 | **E**, par 9.9 |
| **1.5** Grille 3D de référence stable | B | la grille de référence et ses zones actives (ADR-006) | 10.1 | 1.6, 4.9 | **1** |
| **1.6** Cellules, domaines et solveurs distincts, niveaux d'activité des cellules | B | — | 1.5 | 1.4, 4.3, 4.11 | **2** |
| **1.7** Horloge de simulation entière et phases déterministes | H | le déterminisme entre les chemins d'exécution de ce PC (ADR-219 D2) | — | — | **0** |
| **1.8** Référentiels, précision f32 locale, `g_eff` injectée | H | les référentiels mobiles, puis la planète (ADR-002) | — | 4.17, 11.1 | **0** |

### 2. Grandes masses d'eau et fond (B)

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **2.1** Mer et océan : état de mer spectral déterministe, sans état par cellule | A | anisotropie, asymétrie des pentes, B1 complet | 11.2 | 13.3 | **1** |
| **2.2** Houles longues, mers croisées, marée, niveau moyen variable | A | marée, niveau moyen variable, adoption par défaut | 2.8 | — | **E**, par 2.8 |
| **2.3** Lacs | A | — | 2.6, 2.7 | — | **1** |
| **2.4** Rivières | A | — | 2.6, 2.7 | 2.5, 12.2 | **1** |
| **2.5** Canaux | A | — | 2.4 | — | **2** |
| **2.6** Courants macroscopiques à niveau de détail propre | A | le courant macroscopique, du vecteur au champ (ADR-011) | — | 2.3, 2.4, 6.2 | **0** |
| **2.7** Bathymétrie | A | l'entrée dans B, isobathes droites, faite (ADR-196, S364) ; les chemins de B et Godot, puis la 2D et la marée ; hauts-fonds isolés | — | 1.3, 2.3, 2.4, 2.8, 2.9, 3.4, 3.5, 3.6, 4.14 | **0** |
| **2.8** Précalcul côtier et météo | A | — | 2.7, 3.6 | 2.2, 5.5, 12.3 | **E** — la fin du projet : météo et son en dernier (ADR-197 D5) |
| **2.9** Dérivées du fond pour les couches volumiques | A | — | 2.7 | — | **1** |

### 3. Ondes propagatives (W)

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **3.1** Anneaux d'impact dispersifs | A | l'eau peu profonde faite (S528) ; la gerbe, la profondeur variable | 4.12 | — | **3** |
| **3.2** Sillages de bateaux | A | durées longues ; C07 passe entier (S519–S527) | — | 6.6, 13.2 | **0** |
| **3.3** Explosions de surface et sous-marines | A | la source d'explosion de W, champ lointain | 4.16, 7.4 | — | **2** |
| **3.4** Tsunamis | A | — | 2.7, 3.6 | 11.3 | **2** |
| **3.5** Déferlement | A | — | 2.7, 3.6 | 4.14 | **2** |
| **3.6** Réfraction bathymétrique des ondes | A | — | 2.7 | 2.8, 3.4, 3.5 | **1** |
| **3.7** Événements horodatés, journaux, instantanés et restauration avec perte connue | A | — | 10.1 | — | **1** |
| **3.8** Composition B+W sans refus sur toute scène | A | la saturation par la pression seule (A261) | — | — | **0** |
| **3.9** Couches W fournies au-dessus du plan moyen pour δ | A | W évalué au-dessus du plan moyen, comme B (ADR-154) | — | 1.1, 4.6 | **0** |

### 4. Simulation volumique locale (δ)

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **4.1** Solveur volumique 3D à surface libre | B | — | 4.16 | — | **1** |
| **4.2** Plusieurs domaines actifs simultanés | B | plus de deux domaines ; l'ordonnanceur dans l'afficheur | — | 4.9 | **0** |
| **4.3** Subdivision adaptative anisotrope, blocs épars | B | — | 1.6 | 4.10, 9.9, 13.3 | **3** |
| **4.4** Profondeur adaptative | B | `nz` variable : le redimensionnement vertical | — | — | **0** |
| **4.5** Création, croissance, réduction et disparition visuellement gratuites | B | une disparition progressive | — | — | **E** — un verdict visuel des passages (A319) |
| **4.6** Entrée des vagues de B/W dans le domaine | C | la houle progressive traversante sur une durée utile ; B4 | 3.9 | — | **1** |
| **4.7** Frontière sans réflexion ni rupture visible | C | la réflexion d'un front oblique ; C05 | — | 13.2 | **0** |
| **4.8** Sortie des perturbations vers W | C | A320 (les termes croisés sous forme de Bernoulli), puis l'ordre E, critère refondu (ADR-198) | — | 1.1 | **0** |
| **4.9** Fusion et séparation de domaines | B | — | 1.5, 4.2 | — | **2** |
| **4.10** Adaptation interne | B | — | 4.3 | — | **4** |
| **4.11** Régime substitutif | B | — | 1.6, 4.20, 12.3 | — | **E**, par 12.3 |
| **4.12** Cavité et gerbe d'impact | B | le raccord particules ↔ colonnes (A316) | 4.16, 7.4 | 3.1, 7.3, 13.3 | **2** |
| **4.13** Proche-coque et gerbe d'étrave | B | la coque en marche dans la production de δ | 4.16 | 6.6 | **1** |
| **4.14** Plage | B | — | 2.7, 3.5, 4.16 | — | **3** |
| **4.15** Rochers et obstacles immergés | B | le couplage à B/W sur fond coupé ; un modèle de turbulence | — | 6.2 | **0** |
| **4.16** Surface non graphe | B | le raccord (A316), puis APIC en 3D | — | 3.3, 4.1, 4.12, 4.13, 4.14, 4.20, 7.2, 7.4 | **0** |
| **4.17** Référentiel accéléré et invariance galiléenne | B | C16 fait dans δ linéaire (S542–S543) ; Coriolis, la carte, C06 | 1.8 | 13.2 | **1** |
| **4.18** Conservation de la masse et de l'énergie | C | le compteur sur la carte ; énergie et quantité de mouvement ; C09 | — | 13.2 | **0** |
| **4.19** Coût de δ compatible avec le budget | B | d'autres scènes ; plusieurs domaines en direct ; un 99ᵉ centile en direct | — | 9.11 | **0** |
| **4.20** Changement de solveur pendant une simulation | B | — | 4.16 | 4.11 | **1** |
| **4.21** Cohérence de phase entre δ et B+W sur la durée de vie d'un domaine | C | le mode relatif dans la production GPU ; W ; A320 | — | — | **0** |

### 5. Volumes finis et inondations (V)

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **5.2** Géométrie réelle des contenants | H | la précision des grands volumes (A269) ; des formes courbes cuites | — | 5.9 | **0** |
| **5.4** Vannes et pompes | H | le `C_d` selon l'ouverture ; pertes et énergie de la pompe | — | 5.8 | **0** |
| **5.5** Pluie selon l'exposition au ciel, absorption par le sol | H | l'absorption par le sol ; la pluie hors contenant ; l'exposition calculée depuis les objets posés | 2.8 | — | **E**, par 2.8 |
| **5.6** Seuil adaptatif à l'échelle du contenant | H | le seuil adaptatif | — | — | **0** |
| **5.7** Plusieurs liquides | H | `liquid_id` (A17) | — | — | **0** |
| **5.8** Réseau fermé sous pression | H | — | 5.4 | — | **1** |
| **5.9** Compartiments, brèches, inondation de navire, limitée par l'air | H | C17 passé (S538) ; l'évent à débit limité, la flottabilité de la poche, les brèches en jeu | 5.2, 7.5 | 6.6, 13.2 | **3** |
| **5.10** Articulation V↔δ | H | une dynamique visible (δ sur GPU, 5 à 10 cm) ; le bac tampon ; V qui déclenche δ | — | 1.1, 13.2 | **0** |
| **5.11** Eaux souterraines | H | — | — | — | **E** — hors du périmètre par décision de l'utilisateur (ADR-197 D4) ; ne se rouvre que par lui |
| **5.12** Capture et restauration de V | H | le stockage durable | 10.1 | 10.6 | **1** |

### 6. Solides et flottabilité

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **6.2** Forces de l'eau sur les objets | H | W derrière la requête | 2.6, 4.15 | 6.7 | **1** |
| **6.6** Grands navires | H | — | 3.2, 4.13, 5.9 | 11.3 | **4** |
| **6.7** Acteur poussé, renversé ou déplacé par l'eau | B | — | 6.2 | — | **2** |

### 7. Phénomènes secondaires

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **7.1** Écume et moutons | A | sources de W, δ, vent ; demi-vies et transfert (B9) | — | 8.4, 13.2, 13.3 | **0** |
| **7.2** Spray, embruns, gouttelettes | B | — | 4.16 | 8.4, 9.9 | **1** |
| **7.3** Microbulles visuelles | B | — | 4.12 | 8.4 | **3** |
| **7.4** Grosses bulles et poches d'air physiques | B | — | 4.16 | 3.3, 4.12, 7.5, 8.4, 13.2 | **1** |
| **7.5** Air comprimé, vide, eau dans le vide | B | la poche comprimée d'un corps faite (S539) ; l'adiabatique, la poche qui s'échappe, le vide | 7.4 | 5.9 | **2** |
| **7.6** Glace et vapeur | H | — | — | 13.2 | **0** |
| **7.7** Danger et traversabilité | H | la publication par tuiles depuis B, W et V (ADR-018) | — | — | **0** |
| **7.8** Audio de l'eau | H | les événements et paramètres publiés (ADR-016) | — | — | **E** — la fin du projet, par Wwise, l'audio du jeu (ADR-197 D5, ADR-219) |

### 8. Rendu et niveaux de détail visuels

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **8.1** Rendu temps réel de la surface sur GPU | H | le cœur branché dans Godot, moteur du jeu entier (GDExtension ; ADR-197 D2) | — | — | **0** |
| **8.2** LOD de la géométrie de surface | H | le LOD du maillage ; déplacement ou normales selon la vue | — | — | **0** |
| **8.3** LOD par source | H | le filtre des impacts ; le LOD temporel | — | 11.4 | **0** |
| **8.4** Écume, spray, gouttes, bulles rendus, chacun avec son LOD | H | l'écume (suspendue) ; le spray ; les bulles ; les gerbes de pluie (ADR-205, pièce 4) ; leurs niveaux de détail | 7.1, 7.2, 7.3, 7.4 | — | **4** |
| **8.5** Transparence, réfraction, caustiques, particules sous-marines | H | particules, eaux chargées, caustiques sur les objets (transparence, réfraction : S359 ; caustiques du fond : S361) | — | 8.6 | **0** |
| **8.6** Vue sous-marine et passage de la surface | H | la caméra à demi immergée (ADR-019 §6) ; bulles, rayons, turbidité ; le coût du profil immergé (B11) | 8.5 | 13.3 | **1** |
| **8.7** Rendu de δ raccordé à B+W sans rupture visible | C | une frontière sans fondu ; la tolérance de pente | — | — | **E** — le verdict de la frontière (R18 reçu, ADR-197 D8) |
| **8.8** Lointain et horizon sans artefact | H | un certificat d'absence d'alias | — | — | **0** |
| **8.9** Détails artificiels bon marché | H | capillaires du vent ; queue des perturbations W ; coût (la queue de B par FFT dans Godot : S360 ; les rides de pluie, S379 : une texture à moments) | — | — | **0** |
| **8.10** Crédibilité perçue validée par un regard humain | H | de nouvelles revues, préparées | — | — | **E** — les verdicts de l'utilisateur : poses, animation, scénarios |

### 9. Activation, prédiction, budget et dégradation

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **9.1** Activation multicritère | H | `W_urgence` ; le banc B8 ; `W_gameplay` par le jeu d'essai, puis DyingStar (ADR-219 D3) | — | 13.3 | **0** |
| **9.2** Domaine prédictif orienté devant le joueur | H | le domaine qui précède la caméra : prédiction, orientation | — | — | **0** |
| **9.3** Prédiction d'objets balistiques | H | un corps quelconque, le vent, l'entrée orientée consommée par δ | — | 9.4, 9.6 | **0** |
| **9.4** Objets contrôlables : paliers de confiance | H | — ; les objets du jeu d'essai, puis de DyingStar (ADR-219 D3) | 9.3 | — | **1** |
| **9.6** Précalcul avant l'impact | H | — | 9.3 | — | **1** |
| **9.7** Hors caméra : quatre niveaux | H | la condensation hors caméra | — | — | **0** |
| **9.8** Aucun solveur ne dépasse son budget | H | la borne murale ; l'estimateur dans le cœur | — | — | **0** |
| **9.9** Dégradation contrôlée dans l'ordre prescrit | H | rangs 3, 6 et 7 ; régulateur PI ; bande morte de l'échelle | 4.3, 7.2 | 1.4 | **E** — un verdict visuel du prix du rang 1 (A319) |
| **9.10** Profils de qualité, adaptation au matériel et à la charge | H | les profils de qualité (I-16), mesurés sur ce PC bridé (ADR-219 D2) | — | 11.5 | **0** |
| **9.11** 60 images/s avec 2 ms pour l'eau sur une scène représentative | H | la scène représentative réunie, mesurée | 4.19 | 13.4 | **1** |
| **9.12** Aucune allocation à l'exécution | H | — | 1.4 | — | **E**, par 1.4 |
| **9.13** Dépassement critique temporaire | H | la réserve d'événement d'ADR-012 §6 | — | — | **0** |

### 10. Multijoueur, autorité et persistance

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **10.1** Réplication des événements sources, jamais de l'état | H | notre format de réplication, client et serveur locaux (ADR-219 D1) | — | 1.5, 3.7, 5.12, 10.4, 10.5, 10.6, 13.4 | **0** |
| **10.2** Le serveur n'exécute que V | H | un hôte serveur sans δ ni rendu (C18), un processus sans fenêtre sur ce PC (ADR-219 D2) | — | — | **0** |
| **10.3** Déterminisme bit à bit entre plateformes pour B, W répliqué et V | H | le déterminisme entre les chemins d'exécution de ce PC (ADR-219 D2) | — | — | **0** |
| **10.4** δ sans autorité de jeu, aucun chemin d'énergie du client vers le monde, grandeurs
  dérivées autoritaires | H | — | 10.1 | — | **1** |
| **10.5** Grandes formes cohérentes entre clients, détails locaux libres | H | — | 10.1 | — | **1** |
| **10.6** Sauvegarde, reconnexion, arrivée en cours de partie | H | C19 complet, en local | 5.12, 10.1 | 13.2 | **2** |
| **10.8** Chemin poussé de SPEC-006 | H | le bus `WaveEvent` et ses canaux (SPEC-006) | — | — | **0** |
| **10.9** Requêtes de jeu | H | le jeu d'essai, puis DyingStar (ADR-219 D3) | — | — | **0** |

### 11. Grande échelle et très grands événements

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **11.1** Monde planétaire | H | — | 1.8 | 13.4 | **1** |
| **11.2** Nombreuses régions de mer décrites par descripteur | A | le descripteur de région de mer (I-09) | — | 2.1 | **0** |
| **11.3** Très grands événements | H | — | 3.4, 6.6 | — | **5** |
| **11.4** Nombreuses sources simultanées à coût maîtrisé | H | la généralisation, le LOD temporel | 8.3 | — | **1** |
| **11.5** Matériel cible de livraison et seconde cible | H | ce PC, cible de livraison ; la seconde cible : le bridage de 9.10 (ADR-219 D2) | 9.10 | 13.3 | **1** |

### 12. Outillage auteur et données cuites

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **12.1** Cuisson reproductible, empreintes, obsolescence détectée | H | la détection d'obsolescence | — | 12.5 | **0** |
| **12.2** Éditeur de rivières | H | — | 2.4 | — | **2** |
| **12.3** Précalcul côtier stocké | H | — | 2.8 | 4.11 | **E**, par 2.8 |
| **12.4** Eau en amont du terrain, géoïde dans l'outil de terrain | H | une carte de hauteurs qui imite les tuiles HEALPix de DyingStar (ADR-219 D4) | — | 13.4 | **0** |
| **12.5** Portée d'une modification bornée par partition | H | — | 12.1 | — | **1** |

### 13. Validation du système

| point | sys. | maintenant | attend | débloque | front |
|---|---|---|---|---|---|
| **13.1** Harnais de validation | H | les étages manquants de SPEC-003 | — | — | **0** |
| **13.2** Les 23 cas canoniques passent sur le système | H | chaque cas exécuté sur le système | 3.2, 4.7, 4.17, 4.18, 5.9, 5.10, 7.1, 7.4, 7.6, 10.6 | 13.4 | **4** |
| **13.3** Les onze bancs rendent leur verdict | H | chaque banc exécuté | 2.1, 4.3, 4.12, 7.1, 8.6, 9.1, 11.5 | — | **4** |
| **13.4** L'eau dans le jeu | H | lire les dépôts publics de DyingStar ; le jeu d'essai sur sa pile (ADR-219 D3) | 9.11, 10.1, 11.1, 12.4, 13.2 | — | **5** |

<!-- fin des tables -->
