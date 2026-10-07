# La réévaluation des intentions fondatrices — S643, 2026-10-07

*Sur la demande de l'utilisateur* ([ADR-261](../adr/ADR-261-reponses-du-2026-10-07.md) D3) : *« toutes les anciennes intentions doivent
être jugées dépassées ou non, de meilleure solution etc. Mais l'objectif reste le même d'une performance et réalisme insane. »* Chacune
des sections des deux documents sources est rejugée, à la lumière de ce qui a été construit, mesuré et décidé depuis S01 :

- [`systeme_eau_architecture_globale`](../sources/systeme_eau_architecture_globale.md) : 20 sections ;
- [`systeme_eau_zones_ouvertes_et_decisions_a_valider`](../sources/systeme_eau_zones_ouvertes_et_decisions_a_valider.md) : 32 sections.

**Trois verdicts :**

- **Garder** : l'intention tient, elle est construite ou en route.
- **Meilleure solution** : l'intention tient, mais la façon de l'atteindre est remplacée ; ce qui la remplace est nommé, avec l'endroit où
  cela vit.
- **Dépassée** : l'intention ne sert plus l'objectif.

Le complément de l'[audit S640](AUDIT-INTENTIONS-INITIALES-S640.md) : l'audit cherchait ce qui manquait ; ce registre juge ce qui est.

**L'objectif rejugé lui-même.** La source (§1) fait du rendu *perçu* le critère et de la précision une aide, non une fin. L'utilisateur
lève la limite de réalisme (ADR-261 D4) et garde la performance au premier rang. Les deux ne se contredisent pas, mais le critère se
resserre. Une simplification reste légitime, c'est même la source de la performance, à condition d'être **indiscernable** de la physique
qu'elle remplace, et non plus seulement *crédible*. L'indiscernabilité est jugée contre une référence physique mesurée, puis par le regard
de l'utilisateur. Voir [ADR-262](../adr/ADR-262-indiscernable-du-reel-au-budget.md).

## 1. L'architecture globale

| § | intention | verdict | ce qui la porte, ou la remplace |
|---|---|---|---|
| 1 | l'eau aussi simplifiée que possible tant qu'elle reste crédible ; le rendu perçu comme critère | **meilleure solution** (le critère) | *indiscernable* d'une référence physique, au budget (ADR-262) ; la simplification reste le moyen de la performance |
| 2.1 | les grandes masses en grandeurs macroscopiques | garder | B spectral déterministe, tables FFT (2.1, ADR-195) ; la physique locale par-dessus (δ relatif à B, ADR-198) |
| 2.2 | les volumes finis, seuil adaptatif ; débordements « seulement s'ils apportent une valeur » | **meilleure solution** (pour les débordements) | V à volume **entier**, la masse exacte toujours (5.1, 5.3) ; débordements et transferts vers B/W/δ conservés (5.14) ; flaques (5.13) |
| 3.1 | une grille 3D fixe de référence | **meilleure solution** | sur une planète de 6 356 km, un **index hiérarchique sphérique** (HEALPix ou cube-sphère, étude d'ADR-261 D2) et des référentiels locaux (I-08) ; 1.5 |
| 3.2 | une subdivision à niveaux prédéfinis, anisotrope | **meilleure solution** | blocs épars et colonne graduée (ADR-208, 4.3) ; la production épars sur GPU |
| 3.3 | les cellules regroupées en domaines ; niveaux d'activité | garder | 1.6 (`activite.rs`, S608) |
| 3.4 | disparition progressive, perturbation condensée | garder | 4.5, 9.7 ; la graine (ADR-022 §3, S623) |
| 4.1–4.2 | la profondeur selon l'interaction | garder | 4.4 (le domaine qui suit l'objet qui coule) |
| 4.3 | les plages jusqu'au fond et au sable | garder | 4.14 (mouillage, séchage, rouleau 3D en campagne) |
| 5 | trois régimes continus | **meilleure solution** (déjà faite) | quatre couches B/W/δ/V (ADR-001), la transition en sens unique (ADR-005) |
| 5.1 | l'eau simplifiée assez riche pour initialiser la physique | garder | ADR-004 ; δ relatif à B (4.21) |
| 5.2 | une transition imperceptible | garder | 4.6–4.8, 8.7 ; non atteinte (A319) |
| 5.3 | la physique locale où les approximations ne suffisent plus | garder | δ (4.1–4.16) |
| 6 | fusion et séparation sans rupture | garder | 4.9 |
| 7 | une activation multicritère, au-delà de la distance | garder | 9.1 ; le critère à resserrer : l'erreur à l'écran (ADR-262) |
| 7.1 | le domaine prédictif orienté | garder | 9.2 |
| 8.1–8.4 | la prédiction filtrée, balistique, par paliers | garder | 9.3–9.5 |
| 8.5 | précalcul ; avance plus rapide que le temps réel ; pas de retour arrière | garder | 9.6 ; le calcul d'avance au loin (ADR-211 D4) |
| 9 | hors caméra : quatre niveaux, persistance perceptuelle | garder, **resserré** | 9.7 ; dans un MMO, les conséquences de jeu persistent côté serveur (V, I-10) — la « mémoire perceptuelle » ne vaut que pour le détail |
| 10 | les courants à niveau de détail propre | garder, **étendu** | 2.6 ; sans limite de réalisme, le courant de fond devient une **circulation planétaire cuite** (vent, marée, densité), non un champ posé à la main |
| 11.1 | le rouleau en 3D ; le 2D interdit s'il est faux | garder | 4.14, la campagne du rouleau 3D (S639–) |
| 11.2 | la bathymétrie agit avant la zone physique | garder, **étendu** | 2.7, 3.6 (rayons) ; un **modèle spectral de houle côtière** cuit par rivage (réfraction, diffraction, frottement, déferlement) en est la meilleure solution au réalisme visé |
| 11.3 | rochers et obstacles dans le même domaine | garder | 4.15 ; les rochers qui brisent (3.10) |
| 12 | l'adaptation interne : des éléments qui se divisent et fusionnent | garder | 4.10 (la bande de particules, les colonnes) ; la taille de particule adaptative est la piste |
| 13.1 | les microbulles surtout visuelles | **meilleure solution** | visuelles **et** physiques par leur effet cumulé : l'eau aérée porte moins (6.9) ; 7.3 |
| 13.2 | grosses bulles et poches, traitement physique | garder | 7.4, 7.5 (K2) ; l'air respirable (7.10) |
| 13.3 | embruns visuels ; seules les grandes masses agissent | garder | 7.2 ; la gerbe qui retombe dans W (4.22) |
| 14 | le couplage bidirectionnel réservé aux objets importants | garder | 6.1–6.8 ; les petits objets en couplage simple, physique quand même |
| 15 | le jeu partagé, le détail graphique libre | garder | I-04, I-15, ADR-021 — indispensable à un MMO |
| 16 | un LOD visuel distinct, par composante | garder | 8.2–8.5, 8.9 |
| 17 | budget, ordre de dégradation, profils, adaptation au matériel | garder | 9.8–9.13 ; le budget de 2 ms est un **plafond**, non une cible (ADR-262) |
| 18 | multijoueur à trois niveaux de cohérence | garder | 10.1–10.9 ; « pas encore », gardé (ADR-197 D1) |
| 19 | très grands événements : macroscopique au large, 3D local | garder | 11.3, 3.4 (tsunami jusqu'à la plage, S614–S624) |
| 20 | un orchestrateur de régimes, pas un solveur d'océan unique | garder | 1.4 (`WaterSystem`) |

## 2. Les zones ouvertes

| § | question ou proposition | verdict | état |
|---|---|---|---|
| 1 | le statut des propositions | dépassée | les propositions sont tranchées, une à une, par les ADR |
| 2 | répartition serveur–client ; les subdivisions visuelles locales | garder (la proposition retenue) | ADR-009 (des événements, pas des champs), I-10 |
| 3 | l'état minimal de la fausse eau, variable selon la zone | garder | ADR-004 |
| 4 | l'algorithme de la transition | garder, ouvert | éponge (4.7), transduction (4.8), δ relatif (ADR-198) ; l'imperceptible non atteint |
| 5 | cellules et solveurs, fusion, transfert | **meilleure solution** | ADR-006 (trois structures), le transfert d'état adopté (ADR-210), contre « jamais de transfert » |
| 6 | les niveaux numériques de subdivision | **meilleure solution** | colonne graduée, blocs épars (ADR-208) ; plus de niveaux prédéfinis |
| 7 | gestionnaire global ou hybride | garder (la proposition) | l'ordonnanceur qui répartit un budget (ADR-012), les domaines fournissent leurs données |
| 8 | les seuils d'activation et d'arrêt | garder, à calibrer | 9.1 ; mesurés, non posés (I-14, ADR-260 D3) |
| 9 | la méthode hors caméra | **meilleure solution** | le domaine détruit, W prend le relais (ADR-211 D4) au lieu d'une simulation ralentie |
| 10 | ce qui est gardé à la désactivation | garder | la graine (ADR-022 §3, S623), les événements horodatés (3.7) |
| 11–12 | le filtre et les paliers de prédiction | garder | 9.3, 9.4 |
| 13 | l'erreur acceptable d'un précalcul ; récupérer si moins cher | garder (la proposition) | 9.5, 9.6 |
| 14 | préparation ou avance physique | garder | ADR-013, ADR-211 |
| 15 | le modèle exact des courants ; température et densité « si nécessaires » | garder, **étendu** | 2.6 ; **la densité de l'eau** (salinité, température) devient un champ : eau douce et eau de mer ne portent pas pareil, l'embouchure les mêle (2.11, ajouté) |
| 16 | mer, lac, rivière, canal ; variation saisonnière | garder | 2.2–2.5 ; la saison avec la météo, à la fin (ADR-197 D5) |
| 17 | les transferts entre volumes finis | garder | 5.3, 5.5, 5.13, 5.14 |
| 18 | le solveur choisi par banc | **dépassée** (le choix par banc) | grille MAC et APIC choisis par décision (ADR-175, ADR-186) ; l'interface agnostique gardée (ADR-007) ; B3 valide le solveur retenu (ADR-261 D7) |
| 19 | changer de solveur en cours de vie | garder | 4.20 (S612) |
| 20 | les seuils des événements extrêmes, selon le coût | garder | 11.3 ; le seuil par le coût, non par la taille |
| 21 | une profondeur plafond d'environ 200 m | **dépassée** | aucun plafond global : la profondeur suit l'événement et le coût (4.4) ; un sous-marin, un vaisseau qui coule, l'abysse — sans limite de réalisme |
| 22 | la flottabilité, choix expérimental | garder (fait) | ADR-227 : sur B + W, ou V seulement ; 6.1 validé |
| 23 | l'interface simulation–rendu, plusieurs chemins | garder (fait) | le rendu dans Godot 4, nos nuanceurs (ADR-192, ADR-197) |
| 24 | le déclenchement de l'écume, du spray, des bulles | garder | 7.1 (reprise d'après des vidéos, ADR-219 D5), 7.2, 7.3 |
| 25 | l'air volumique | garder | K2 (ADR-220) ; 7.4, 7.5, 7.10 |
| 26 | les rochers turbulents persistants | garder | 3.10 |
| 27 | précalcul côtier et météo | **meilleure solution** | partout où il y a un rivage, procédural ou à la demande (ADR-261 D3, 12.3) ; la météo à la fin |
| 28 | le budget par image, à mesurer | garder | ADR-125 (60 images/s, 2 ms) — un plafond (ADR-262) |
| 29 | LOD visuel et grille de simulation partagés ou non | garder, ouvert | 8.2, 8.3 |
| 30 | autorité et déterminisme | garder | I-03 (entre plateformes, en intention ; ADR-261 D8), ADR-009, ADR-021 |
| 31 | les sept propositions de l'assistant | garder | toutes retenues, par ADR-012, 004, 013, 007, 227, 192, 009 |
| 32 | l'ordre de priorité de la conception | dépassée | les dix inconnues sont tranchées ou en route ; l'ordre est celui de la liste et d'ADR-247 |

**Compte** (recompté par script) : les 20 sections de l'architecture en 34 lignes (sous-sections séparées), les 32 sections des zones
ouvertes en 31 lignes (§11 et §12 ensemble) — aucune omise.

| verdict | architecture | zones ouvertes |
|---|---:|---:|
| garder | 28 | 23 |
| meilleure solution | 6 | 4 |
| dépassée | 0 | 4 |

## 3. Ce qui en découle

- **ADR-262** : le critère de jugement devient « indiscernable du réel, au budget ». Le budget est un plafond, et l'activation est jugée
  sur l'erreur visible à l'écran.
- **Un point ajouté**, 2.11 : la densité de l'eau (salinité, température) devient un champ.
- **Points précisés** :
  - 1.5 : l'index planétaire hiérarchique ;
  - 2.6 : la circulation planétaire cuite ;
  - 2.7 : le modèle spectral côtier ;
  - 4.4 : aucun plafond de profondeur ;
  - 9.11 : le plafond, non la cible.
- **Rien de retiré de la liste.** Les intentions dépassées l'étaient déjà dans les faits : le plafond de 200 m n'était porté par aucun
  point, et le choix du solveur par banc a été remplacé par ADR-175 et ADR-186.
