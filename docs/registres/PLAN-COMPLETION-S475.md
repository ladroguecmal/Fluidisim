# Le plan de complétion — la liste à 100 %

*Ouvert en S475, 2026-10-04*, par [ADR-218](../adr/ADR-218-le-systeme-de-l-eau-complet.md) — décision de l'utilisateur : le système
de l'eau est fini quand la [liste du projet fini](../LISTE-PROJET-FINI.md) est validée à 100 %. La liste dit **quoi** ;
[DEPENDANCES-LISTE](DEPENDANCES-LISTE.md) dit **ce que chaque point attend** ; ce plan dit **dans quel ordre, en quelles campagnes,
et ce qu'il faut à l'utilisateur**. Il ne change l'état d'aucun point.

## 1. Où en est la liste (actualisation de S475)

**Périmètre : 119 points** (120 moins 5.11, hors du périmètre, ADR-197 D4). **3 validés, 73 partiels, 43 absents.**

Recompté par la relecture du journal de S351 à S474 contre la liste : **aucun point n'a changé de catégorie depuis S408** (4.10 passé
à partiel) ; les sessions S409–S474 — la campagne du solveur 3D sur la carte (C7), la surface continue, la v1 (R38), Godot (C11, R39),
le banc visuel et le type d'eau — ont fait avancer des points **déjà partiels** (4.x, 8.x, 9.x), reportés par les lots de registres,
sans en amener un à son périmètre final. Les absents qu'elles ont effleurés sans les ouvrir : 6.7 et 6.8 (le joueur est un corps
imposé, aucune force ne se mesure), 7.2 (les gouttes de la bande APIC ne sont pas encore un spray), 12.x (le type d'eau est une donnée
de scène, pas encore de l'éditeur de carte).

**Ce que dit ce constat** : depuis S351, tout l'effort est allé en profondeur sur une scène (δ, rendu) ; pour atteindre 100 %, il faut
désormais couvrir la largeur — des points jamais ouverts (lacs, rivières, tsunamis, navires, inondations, réseau, outillage). Le
plan les ordonne pour que chaque campagne ferme des points, pas seulement les fasse avancer.

## 2. Les campagnes

Dans l'ordre des dépendances (fronts de DEPENDANCES-LISTE). Une campagne ferme ses points ; elle peut en avancer d'autres en passant.
Les estimations sont en sessions (une session : plan, construction, preuve) ; **ce sont des ordres de grandeur**, à corriger au fil
des campagnes. Les sessions de rendu continuent d'alterner avec la physique (ADR-191).

| | campagne | points qu'elle ferme | attend | sessions |
|---|---|---|---|---:|
| **K1** | **Rendu final et banc visuel** — le type d'eau dans l'espace (S476), la lumière sous l'eau (E3), le contraste au loin (E5), les vues de contrôle, la non-régression ; le LOD | 8.1, 8.2, 8.3, 8.5, 8.6, 8.8, 8.9 ; 8.7, 8.10 (verdicts) | — | 20 |
| **K2** | **La surface non graphe** — δ qui se retourne, se sépare, se referme ; la cavité, la gerbe, les bulles | 4.16, 4.1, 4.12, 4.20, 7.4, 7.5, 7.2, 7.3, 3.3, 3.1 | — | 30 |
| **K3** | **Le fond et la côte** — la bathymétrie complète, les chemins de B, la réfraction, le déferlement, la plage | 2.7, 2.9, 3.6, 3.2, 3.5, 4.14, 4.15, 6.5, 2.6, 3.9, 4.6 | — | 30 |
| **K4** | **Les eaux intérieures** — lacs, rivières, canaux, l'éditeur de rivières | 2.3, 2.4, 2.5, 12.2 | K3 | 15 |
| **K5** | **δ, le système** — domaines multiples, création et disparition, fusion, adaptation, profondeur adaptative, régime substitutif, frontière, conservation, phase, coût | 4.2, 4.3, 4.4, 4.5, 4.7, 4.8, 4.9, 4.10, 4.11, 4.17, 4.18, 4.19, 4.21, 1.6 | K2 (4.20) | 30 |
| **K6** | **Les solides** — flottabilité, forces, sillage, corps mobiles, acteur poussé, slamming, grands navires, proche-coque | 6.1, 6.2, 6.3, 6.4, 6.7, 6.8, 6.6, 4.13, 1.3 | K2, K3 | 25 |
| **K7** | **Les volumes finis** — contenants réels, fuites, vannes, seuils, plusieurs liquides, réseau sous pression, brèches et inondation de navire, articulation V↔δ, capture | 5.2, 5.3, 5.4, 5.6, 5.7, 5.8, 5.9, 5.10, 5.12 | K6 (5.9) | 25 |
| **K8** | **Activation, prédiction, budget** — multicritère, prédiction, précalcul avant l'impact, hors caméra, budgets, dégradation, profils, 60 images/s, aucune allocation, dépassement critique | 9.1–9.13, 1.4, 1.7, 1.8, 11.4 | K5 | 25 |
| **K9** | **Réseau et persistance** — réplication par événements, serveur V seul, déterminisme entre plateformes, sauvegarde et reconnexion, requêtes de jeu | 10.1–10.6, 10.8, 10.9, 1.5, 3.7, 3.8 | fait F1, F2 | 25 |
| **K10** | **Grande échelle** — la planète, les régions de mer, tsunamis, très grands événements, sources nombreuses | 11.1, 11.2, 11.3, 3.4, 2.1, 2.2 | K3, K6 | 20 |
| **K11** | **Outillage** — cuisson reproductible, précalcul côtier stocké, terrain et géoïde, portée bornée | 12.1, 12.3, 12.4, 12.5 | fait F4 | 12 |
| **K12** | **La fin de l'eau** — la part de l'eau dans la météo, glace et vapeur, danger et traversabilité, l'écume (7.1), l'audio de l'eau | 2.8, 7.1, 7.6, 7.7, 7.8, 5.5, 8.4 | K1–K7 | 20 |
| **K13** | **La validation du système** — le harnais, les 23 cas canoniques, les onze bancs, les quatre couches réunies, la cible de livraison | 13.1, 13.2, 13.3, 1.1, 11.5 | tout ; fait F2 | 15 |
| | **total** | **116 points ouverts** | | **≈ 310** |

L'ordre de départ : **K1** (en cours : le type d'eau) en alternance avec **K2** et **K3** — les deux points qui commandent le plus
(4.16 : 22 points en aval ; 2.7 : 20) ; K5 et K6 ensuite ; K9 dès que F1 est tranché ; K12 et K13 en dernier.

## 3. Les faits que seul l'utilisateur peut fournir

Chacun avec la recommandation de l'agent ; **aucun ne bloque le travail d'aujourd'hui** (36 points au front 0). Ils se demandent
quand leur campagne arrive (ADR-218, conséquences).

| | fait | points | recommandation |
|---|---|---|---|
| **F1** | **Le réseau** : « pas encore » (ADR-197 D1) — mais le multijoueur reste, et 14 points en dépendent | 10.1–10.6, 1.5, 1.6, 3.7, 4.9, 4.11, 5.12 | construire **notre** format de réplication (ADR-197 D1 le permet) et le valider entre deux processus sur ce PC — client et serveur locaux —, sans infrastructure extérieure |
| **F2** | **Un second matériel et un serveur réel** (« pas encore », ADR-197 D7) | 1.7, 9.10, 10.2, 10.3, 11.5 | une seconde machine, même modeste (un portable d'une autre marque de carte graphique), suffit au déterminisme entre plateformes et à l'adaptation ; un serveur peut être un processus sans fenêtre sur une seconde machine |
| **F3** | **Les objets du jeu** : `W_gameplay`, les objets contrôlables, un consommateur des requêtes | 9.1, 9.4, 10.9 | l'agent construit dans Godot un **jeu d'essai** minimal (un joueur, un bateau, des objets), qui consomme l'eau comme le jeu le fera ; le jeu réel le remplace quand il existe |
| **F4** | **L'outil de terrain du jeu dans Godot** (ADR-197 D2 : lequel) | 12.4 | un terrain par carte de hauteurs, notre format, lu par Godot — l'éditeur de carte du jeu (ADR-217) en sera la suite |
| **F5** | **L'écume** : des photographies de forme, couleur et place sur la vague (décision du 2026-09-26) | 7.1, 8.4 | **les vidéos V2 et V3 (S471) les fournissent** — l'écume de déferlement et d'impact, mesurable par le banc ; à confirmer par l'utilisateur |
| **F6** | **Les verdicts** visuels (8.7, 8.10, 4.5, 9.9) | 8.7, 8.10, 4.5, 9.9 | aux jalons de K1 et K5, avec les mesures du banc |
| — | Le mode de l'hôte autoritaire, intersection ou union (A271) | 3.8 | **tranché ici** (ADR-215 D2) quand K9 y arrive : l'intersection, la plus sûre |
