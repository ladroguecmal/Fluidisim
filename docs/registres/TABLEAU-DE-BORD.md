# Tableau de bord — la liste à 100 %

*Généré par `python outils/tableau_de_bord.py --ecrire` (S480) — ne pas modifier à la main.* La liste du projet fini dit l'état de
chaque point ; le plan de complétion, sa campagne ; ce tableau les croise. La fin du système de l'eau : tous les points validés
([ADR-218](../adr/ADR-218-le-systeme-de-l-eau-complet.md)).

**Périmètre : 120 points** (5.11 hors). **Validés : 10** (8.3 %) — partiels : 108 — absents : 2.

## Par campagne

Légende : ✅ validé, ◐ partiel, · absent. L'ordre est celui du [plan de complétion](PLAN-COMPLETION-S475.md).

| campagne | validés | partiels | absents | points |
|---|---:|---:|---:|---|
| **K1** Rendu final et banc visuel | 0 | 9 | 0 | ◐8.1 ◐8.2 ◐8.3 ◐8.5 ◐8.6 ◐8.8 ◐8.9 ◐8.7 ◐8.10 |
| **K2** La surface non graphe | 0 | 10 | 0 | ◐4.16 ◐4.1 ◐4.12 ◐4.20 ◐7.4 ◐7.5 ◐7.2 ◐7.3 ◐3.3 ◐3.1 |
| **K3** Le fond et la côte | 1 | 10 | 0 | ◐2.7 ◐2.9 ◐3.6 ◐3.2 ◐3.5 ◐4.14 ◐4.15 ✅6.5 ◐2.6 ◐3.9 ◐4.6 |
| **K4** Les eaux intérieures | 0 | 4 | 0 | ◐2.3 ◐2.4 ◐2.5 ◐12.2 |
| **K5** δ, le système | 0 | 14 | 0 | ◐4.2 ◐4.3 ◐4.4 ◐4.5 ◐4.7 ◐4.8 ◐4.9 ◐4.10 ◐4.11 ◐4.17 ◐4.18 ◐4.19 ◐4.21 ◐1.6 |
| **K6** Les solides | 4 | 5 | 0 | ✅6.1 ◐6.2 ✅6.3 ✅6.4 ◐6.7 ✅6.8 ◐6.6 ◐4.13 ◐1.3 |
| **K7** Les volumes finis | 1 | 8 | 0 | ◐5.2 ✅5.3 ◐5.4 ◐5.6 ◐5.7 ◐5.8 ◐5.9 ◐5.10 ◐5.12 |
| **K8** Activation, prédiction, budget | 1 | 16 | 0 | ◐9.1 ◐9.2 ◐9.3 ◐9.4 ✅9.5 ◐9.6 ◐9.7 ◐9.8 ◐9.9 ◐9.10 ◐9.11 ◐9.12 ◐9.13 ◐1.4 ◐1.7 ◐1.8 ◐11.4 |
| **K9** Réseau et persistance | 0 | 11 | 0 | ◐10.1 ◐10.2 ◐10.3 ◐10.4 ◐10.5 ◐10.6 ◐10.8 ◐10.9 ◐1.5 ◐3.7 ◐3.8 |
| **K10** Grande échelle | 0 | 6 | 0 | ◐11.1 ◐11.2 ◐11.3 ◐3.4 ◐2.1 ◐2.2 |
| **K11** Outillage | 0 | 4 | 0 | ◐12.1 ◐12.3 ◐12.4 ◐12.5 |
| **K12** La fin de l'eau | 0 | 6 | 1 | ◐2.8 ◐7.1 ◐7.6 ◐7.7 ·7.8 ◐5.5 ◐8.4 |
| **K13** La validation du système, et l'eau dans le jeu | 0 | 5 | 1 | ◐13.1 ◐13.2 ◐13.3 ·13.4 ◐1.1 ◐11.5 |

**Points ouverts hors de toute campagne : 0**.

## Les points ouverts, un par ligne

| point | état | titre |
|---|---|---|
| 1.1 | partiel | Eau = somme de quatre couches B, W, δ, V, dans le code |
| 1.3 | partiel | Interfaces de SPEC-004 |
| 1.4 | partiel | Point d'entrée unique, orchestrateur des régimes |
| 1.5 | partiel | Grille 3D de référence stable |
| 1.6 | partiel | Cellules, domaines et solveurs distincts, niveaux d'activité des cellules |
| 1.7 | partiel | Horloge de simulation entière et phases déterministes |
| 1.8 | partiel | Référentiels, précision f32 locale, `g_eff` injectée |
| 2.1 | partiel | Mer et océan : état de mer spectral déterministe, sans état par cellule |
| 2.2 | partiel | Houles longues, mers croisées, marée, niveau moyen variable |
| 2.3 | partiel | Lacs |
| 2.4 | partiel | Rivières |
| 2.5 | partiel | Canaux |
| 2.6 | partiel | Courants macroscopiques à niveau de détail propre |
| 2.7 | partiel | Bathymétrie |
| 2.8 | partiel | Précalcul côtier et météo |
| 2.9 | partiel | Dérivées du fond pour les couches volumiques |
| 3.1 | partiel | Anneaux d'impact dispersifs |
| 3.2 | partiel | Sillages de bateaux |
| 3.3 | partiel | Explosions de surface et sous-marines |
| 3.4 | partiel | Tsunamis |
| 3.5 | partiel | Déferlement |
| 3.6 | partiel | Réfraction bathymétrique des ondes |
| 3.7 | partiel | Événements horodatés, journaux, instantanés et restauration avec perte connue |
| 3.8 | partiel | Composition B+W sans refus sur toute scène |
| 3.9 | partiel | Couches W fournies au-dessus du plan moyen pour δ |
| 4.1 | partiel | Solveur volumique 3D à surface libre |
| 4.2 | partiel | Plusieurs domaines actifs simultanés |
| 4.3 | partiel | Subdivision adaptative anisotrope, blocs épars |
| 4.4 | partiel | Profondeur adaptative |
| 4.5 | partiel | Création, croissance, réduction et disparition visuellement gratuites |
| 4.6 | partiel | Entrée des vagues de B/W dans le domaine |
| 4.7 | partiel | Frontière sans réflexion ni rupture visible |
| 4.8 | partiel | Sortie des perturbations vers W |
| 4.9 | partiel | Fusion et séparation de domaines |
| 4.10 | partiel | Adaptation interne |
| 4.11 | partiel | Régime substitutif |
| 4.12 | partiel | Cavité et gerbe d'impact |
| 4.13 | partiel | Proche-coque et gerbe d'étrave |
| 4.14 | partiel | Plage |
| 4.15 | partiel | Rochers et obstacles immergés |
| 4.16 | partiel | Surface non graphe |
| 4.17 | partiel | Référentiel accéléré et invariance galiléenne |
| 4.18 | partiel | Conservation de la masse et de l'énergie |
| 4.19 | partiel | Coût de δ compatible avec le budget |
| 4.20 | partiel | Changement de solveur pendant une simulation |
| 4.21 | partiel | Cohérence de phase entre δ et B+W sur la durée de vie d'un domaine |
| 5.2 | partiel | Géométrie réelle des contenants |
| 5.4 | partiel | Vannes et pompes |
| 5.5 | partiel | Pluie selon l'exposition au ciel, absorption par le sol |
| 5.6 | partiel | Seuil adaptatif à l'échelle du contenant |
| 5.7 | partiel | Plusieurs liquides |
| 5.8 | partiel | Réseau fermé sous pression |
| 5.9 | partiel | Compartiments, brèches, inondation de navire, limitée par l'air |
| 5.10 | partiel | Articulation V↔δ |
| 5.12 | partiel | Capture et restauration de V |
| 6.2 | partiel | Forces de l'eau sur les objets |
| 6.6 | partiel | Grands navires |
| 6.7 | partiel | Acteur poussé, renversé ou déplacé par l'eau |
| 7.1 | partiel | Écume et moutons |
| 7.2 | partiel | Spray, embruns, gouttelettes |
| 7.3 | partiel | Microbulles visuelles |
| 7.4 | partiel | Grosses bulles et poches d'air physiques |
| 7.5 | partiel | Air comprimé, vide, eau dans le vide |
| 7.6 | partiel | Glace et vapeur |
| 7.7 | partiel | Danger et traversabilité |
| 7.8 | absent | Audio de l'eau |
| 8.1 | partiel | Rendu temps réel de la surface sur GPU |
| 8.2 | partiel | LOD de la géométrie de surface |
| 8.3 | partiel | LOD par source |
| 8.4 | partiel | Écume, spray, gouttes, bulles rendus, chacun avec son LOD |
| 8.5 | partiel | Transparence, réfraction, caustiques, particules sous-marines |
| 8.6 | partiel | Vue sous-marine et passage de la surface |
| 8.7 | partiel | Rendu de δ raccordé à B+W sans rupture visible |
| 8.8 | partiel | Lointain et horizon sans artefact |
| 8.9 | partiel | Détails artificiels bon marché |
| 8.10 | partiel | Crédibilité perçue validée par un regard humain |
| 9.1 | partiel | Activation multicritère |
| 9.2 | partiel | Domaine prédictif orienté devant le joueur |
| 9.3 | partiel | Prédiction d'objets balistiques |
| 9.4 | partiel | Objets contrôlables : paliers de confiance |
| 9.6 | partiel | Précalcul avant l'impact |
| 9.7 | partiel | Hors caméra : quatre niveaux |
| 9.8 | partiel | Aucun solveur ne dépasse son budget |
| 9.9 | partiel | Dégradation contrôlée dans l'ordre prescrit |
| 9.10 | partiel | Profils de qualité, adaptation au matériel et à la charge |
| 9.11 | partiel | 60 images/s avec 2 ms pour l'eau sur une scène représentative |
| 9.12 | partiel | Aucune allocation à l'exécution |
| 9.13 | partiel | Dépassement critique temporaire |
| 10.1 | partiel | Réplication des événements sources, jamais de l'état |
| 10.2 | partiel | Le serveur n'exécute que V |
| 10.3 | partiel | Déterminisme bit à bit entre plateformes pour B, W répliqué et V |
| 10.4 | partiel | δ sans autorité de jeu, aucun chemin d'énergie du client vers le monde, grandeurs
  dérivées autoritaires |
| 10.5 | partiel | Grandes formes cohérentes entre clients, détails locaux libres |
| 10.6 | partiel | Sauvegarde, reconnexion, arrivée en cours de partie |
| 10.8 | partiel | Chemin poussé de SPEC-006 |
| 10.9 | partiel | Requêtes de jeu |
| 11.1 | partiel | Monde planétaire |
| 11.2 | partiel | Nombreuses régions de mer décrites par descripteur |
| 11.3 | partiel | Très grands événements |
| 11.4 | partiel | Nombreuses sources simultanées à coût maîtrisé |
| 11.5 | partiel | Matériel cible de livraison et seconde cible |
| 12.1 | partiel | Cuisson reproductible, empreintes, obsolescence détectée |
| 12.2 | partiel | Éditeur de rivières |
| 12.3 | partiel | Précalcul côtier stocké |
| 12.4 | partiel | Eau en amont du terrain, géoïde dans l'outil de terrain |
| 12.5 | partiel | Portée d'une modification bornée par partition |
| 13.1 | partiel | Harnais de validation |
| 13.2 | partiel | Les 23 cas canoniques passent sur le système |
| 13.3 | partiel | Les onze bancs rendent leur verdict |
| 13.4 | absent | L'eau dans le jeu |

## Historique du décompte

Une ligne par session qui a écrit le tableau avec `--session`.

<!-- historique -->
| session | date | validés | partiels | absents | périmètre |
|---|---|---:|---:|---:|---:|
| S480 | 2026-10-04 | 3 | 74 | 43 | 120 |
| S481 | 2026-10-05 | 3 | 74 | 43 | 120 |
| S482 | 2026-10-05 | 3 | 74 | 43 | 120 |
| S483 | 2026-10-05 | 3 | 74 | 43 | 120 |
| S484 | 2026-10-05 | 3 | 74 | 43 | 120 |
| S485 | 2026-10-05 | 3 | 74 | 43 | 120 |
| S486 | 2026-10-05 | 3 | 74 | 43 | 120 |
| S487 | 2026-10-06 | 3 | 74 | 43 | 120 |
| S488 | 2026-10-06 | 3 | 74 | 43 | 120 |
| S489 | 2026-10-06 | 4 | 73 | 43 | 120 |
| S490 | 2026-10-06 | 4 | 73 | 43 | 120 |
| S491 | 2026-10-06 | 4 | 73 | 43 | 120 |
| S492 | 2026-10-06 | 4 | 73 | 43 | 120 |
| S493 | 2026-10-06 | 5 | 72 | 43 | 120 |
| S494 | 2026-10-06 | 5 | 72 | 43 | 120 |
| S495 | 2026-10-06 | 5 | 72 | 43 | 120 |
| S496 | 2026-10-06 | 5 | 72 | 43 | 120 |
| S497 | 2026-10-06 | 6 | 71 | 43 | 120 |
| S498 | 2026-10-06 | 6 | 71 | 43 | 120 |
| S499 | 2026-10-06 | 6 | 71 | 43 | 120 |
| S500 | 2026-10-06 | 6 | 71 | 43 | 120 |
| S501 | 2026-10-06 | 6 | 71 | 43 | 120 |
| S502 | 2026-10-06 | 7 | 70 | 43 | 120 |
| S503 | 2026-10-06 | 7 | 70 | 43 | 120 |
| S504 | 2026-10-06 | 7 | 70 | 43 | 120 |
| S505 | 2026-10-06 | 7 | 70 | 43 | 120 |
| S506 | 2026-10-06 | 7 | 70 | 43 | 120 |
| S507 | 2026-10-06 | 7 | 70 | 43 | 120 |
| S508 | 2026-10-06 | 7 | 70 | 43 | 120 |
| S509 | 2026-10-06 | 8 | 69 | 43 | 120 |
| S510 | 2026-10-06 | 9 | 68 | 43 | 120 |
| S511 | 2026-10-06 | 9 | 68 | 43 | 120 |
| S512 | 2026-10-06 | 10 | 68 | 42 | 120 |
| S513 | 2026-10-06 | 10 | 69 | 41 | 120 |
| S514 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S515 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S516 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S517 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S518 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S519 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S520 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S521 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S522 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S523 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S524 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S525 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S526 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S527 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S528 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S529 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S530 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S531 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S532 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S533 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S534 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S535 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S536 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S537 | 2026-10-06 | 10 | 70 | 40 | 120 |
| S538 | 2026-10-06 | 10 | 71 | 39 | 120 |
| S539 | 2026-10-06 | 10 | 72 | 38 | 120 |
| S540 | 2026-10-06 | 10 | 72 | 38 | 120 |
| S541 | 2026-10-06 | 10 | 72 | 38 | 120 |
| S542 | 2026-10-06 | 10 | 73 | 37 | 120 |
| S543 | 2026-10-06 | 10 | 73 | 37 | 120 |
| S544 | 2026-10-06 | 10 | 73 | 37 | 120 |
| S545 | 2026-10-06 | 10 | 73 | 37 | 120 |
| S546 | 2026-10-06 | 10 | 73 | 37 | 120 |
| S547 | 2026-10-06 | 10 | 73 | 37 | 120 |
| S548 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S549 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S550 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S551 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S552 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S553 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S554 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S555 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S556 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S557 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S558 | 2026-10-06 | 10 | 74 | 36 | 120 |
| S559 | 2026-10-06 | 10 | 75 | 35 | 120 |
| S560 | 2026-10-06 | 10 | 75 | 35 | 120 |
| S561 | 2026-10-06 | 10 | 75 | 35 | 120 |
| S562 | 2026-10-06 | 10 | 75 | 35 | 120 |
| S563 | 2026-10-06 | 10 | 75 | 35 | 120 |
| S564 | 2026-10-06 | 10 | 76 | 34 | 120 |
| S565 | 2026-10-06 | 10 | 77 | 33 | 120 |
| S566 | 2026-10-06 | 10 | 77 | 33 | 120 |
| S567 | 2026-10-06 | 10 | 77 | 33 | 120 |
| S568 | 2026-10-06 | 10 | 77 | 33 | 120 |
| S569 | 2026-10-06 | 10 | 77 | 33 | 120 |
| S570 | 2026-10-06 | 10 | 78 | 32 | 120 |
| S571 | 2026-10-06 | 10 | 78 | 32 | 120 |
| S572 | 2026-10-06 | 10 | 78 | 32 | 120 |
| S573 | 2026-10-06 | 10 | 78 | 32 | 120 |
| S574 | 2026-10-06 | 10 | 79 | 31 | 120 |
| S575 | 2026-10-06 | 10 | 79 | 31 | 120 |
| S576 | 2026-10-07 | 10 | 79 | 31 | 120 |
| S577 | 2026-10-07 | 10 | 79 | 31 | 120 |
| S578 | 2026-10-07 | 10 | 79 | 31 | 120 |
| S579 | 2026-10-07 | 10 | 79 | 31 | 120 |
| S580 | 2026-10-07 | 10 | 79 | 31 | 120 |
| S581 | 2026-10-07 | 10 | 79 | 31 | 120 |
| S582 | 2026-10-07 | 10 | 80 | 30 | 120 |
| S583 | 2026-10-07 | 10 | 81 | 29 | 120 |
| S584 | 2026-10-07 | 10 | 81 | 29 | 120 |
| S585 | 2026-10-07 | 10 | 81 | 29 | 120 |
| S586 | 2026-10-07 | 10 | 81 | 29 | 120 |
| S587 | 2026-10-07 | 10 | 82 | 28 | 120 |
| S588 | 2026-10-07 | 10 | 83 | 27 | 120 |
| S589 | 2026-10-07 | 10 | 84 | 26 | 120 |
| S590 | 2026-10-07 | 10 | 85 | 25 | 120 |
| S591 | 2026-10-07 | 10 | 85 | 25 | 120 |
| S592 | 2026-10-07 | 10 | 86 | 24 | 120 |
| S593 | 2026-10-07 | 10 | 87 | 23 | 120 |
| S594 | 2026-10-07 | 10 | 88 | 22 | 120 |
| S595 | 2026-10-07 | 10 | 89 | 21 | 120 |
| S596 | 2026-10-07 | 10 | 89 | 21 | 120 |
| S597 | 2026-10-07 | 10 | 90 | 20 | 120 |
| S598 | 2026-10-07 | 10 | 91 | 19 | 120 |
| S599 | 2026-10-07 | 10 | 93 | 17 | 120 |
| S600 | 2026-10-07 | 10 | 94 | 16 | 120 |
| S601 | 2026-10-07 | 10 | 94 | 16 | 120 |
| S602 | 2026-10-07 | 10 | 95 | 15 | 120 |
| S603 | 2026-10-07 | 10 | 96 | 14 | 120 |
| S604 | 2026-10-07 | 10 | 97 | 13 | 120 |
| S605 | 2026-10-07 | 10 | 98 | 12 | 120 |
| S606 | 2026-10-07 | 10 | 98 | 12 | 120 |
| S607 | 2026-10-07 | 10 | 99 | 11 | 120 |
| S608 | 2026-10-07 | 10 | 100 | 10 | 120 |
| S609 | 2026-10-07 | 10 | 101 | 9 | 120 |
| S610 | 2026-10-07 | 10 | 102 | 8 | 120 |
| S611 | 2026-10-07 | 10 | 102 | 8 | 120 |
| S612 | 2026-10-07 | 10 | 103 | 7 | 120 |
| S613 | 2026-10-07 | 10 | 104 | 6 | 120 |
| S614 | 2026-10-07 | 10 | 105 | 5 | 120 |
| S615 | 2026-10-07 | 10 | 106 | 4 | 120 |
| S616 | 2026-10-07 | 10 | 106 | 4 | 120 |
| S617 | 2026-10-07 | 10 | 107 | 3 | 120 |
| S618 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S619 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S620 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S621 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S622 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S623 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S624 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S625 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S626 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S627 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S628 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S629 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S630 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S631 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S632 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S633 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S634 | 2026-10-07 | 10 | 108 | 2 | 120 |
| S635 | 2026-10-07 | 10 | 108 | 2 | 120 |
<!-- fin de l'historique -->
