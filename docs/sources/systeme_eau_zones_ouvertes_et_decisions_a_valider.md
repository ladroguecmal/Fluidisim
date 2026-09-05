# Système d’eau dynamique — Zones ouvertes, incertitudes et décisions à valider

## 1. Objet du document

Ce document recense uniquement les points qui ne sont pas encore suffisamment définis pour être considérés comme des exigences techniques définitives.

Il distingue trois catégories :

- **Ouvert** : aucune décision finale n’a été prise.
- **Partiellement défini** : la direction générale est connue, mais les règles précises manquent.
- **Proposition de l’assistant** : une architecture ou une règle a été suggérée pour structurer la réflexion, mais ne doit pas être considérée comme une décision du projet tant qu’elle n’a pas été explicitement validée.

Aucune proposition listée ici ne doit être implémentée comme contrainte ferme sans validation.

---

## 2. Répartition serveur / client des subdivisions

**Statut : ouvert**

La grille globale est fixe et doit faciliter le réseau, mais la répartition exacte des responsabilités n’est pas définie.

Questions restantes :

- le serveur doit-il connaître les subdivisions physiques détaillées de chaque zone ?
- serveur et clients doivent-ils avoir exactement la même hiérarchie de cellules ?
- les subdivisions purement visuelles peuvent-elles être entièrement locales au client ?
- quelles cellules ou états doivent être synchronisés lorsqu’ils ont une conséquence gameplay ?
- faut-il un identifiant hiérarchique déterministe pour chaque cellule et sous-cellule ?

**Proposition de l’assistant non encore validée comme règle ferme :**

Synchroniser côté serveur uniquement les événements et états ayant une conséquence gameplay, tandis que les subdivisions visuelles et une partie de la simulation locale resteraient propres à chaque client.

---

## 3. Données minimales de la « fausse eau »

**Statut : ouvert — critique**

La structure générale `eau simplifiée → transition → simulation` est définie, mais le contenu exact de l’état simplifié ne l’est pas.

À déterminer selon les types de zones :

- hauteur de surface ;
- phase des vagues ;
- amplitude dominante ;
- direction ;
- vitesse horizontale ;
- courant ;
- énergie ;
- turbulence approximative ;
- autres variables nécessaires à une initialisation cohérente.

Le problème principal est de stocker suffisamment d’information pour initialiser une simulation physique crédible sans rendre la fausse eau coûteuse.

**Proposition de l’assistant non formellement validée :**

Ne pas imposer un état universel lourd. Utiliser un état minimal dépendant du type de zone : haute mer, côte, rivière, transition, etc.

---

## 4. Algorithme exact de la zone de transition

**Statut : partiellement défini — critique**

La fonction de la zone de transition est validée, mais pas son implémentation.

À définir :

- largeur de transition ;
- interpolation spatiale ;
- interpolation temporelle ;
- transmission d’énergie ;
- conversion d’une onde analytique vers un état volumique ;
- conversion inverse d’une perturbation simulée vers un état simplifié ;
- traitement des réflexions numériques ;
- conservation de phase ;
- conservation approximative de vitesse et d’énergie ;
- comportement lorsque le domaine physique grandit ou rétrécit rapidement.

La disparition complète des frontières perceptibles est une exigence, mais la méthode reste ouverte.

---

## 5. Relation exacte entre cellules et solveurs

**Statut : partiellement défini**

Le modèle est hybride : plusieurs cellules peuvent former un même domaine, et plusieurs domaines peuvent éventuellement communiquer.

Reste à définir :

- quand regrouper les cellules dans un solveur commun ;
- quand conserver plusieurs solveurs séparés ;
- coût maximal acceptable du regroupement ;
- gestion des résolutions différentes dans un même domaine ;
- remaillage ou transfert d’état lors d’une fusion ;
- transfert d’état lors d’une séparation ;
- continuité physique entre domaines voisins.

---

## 6. Niveaux numériques de subdivision

**Statut : partiellement défini**

Le système utilise des niveaux prédéfinis et une subdivision indépendante des axes, mais les valeurs numériques restent à choisir.

À déterminer :

- dimensions de la cellule de base ;
- facteurs de subdivision ;
- nombre maximal de niveaux ;
- tailles minimales par type d’événement ;
- profondeur maximale de la hiérarchie ;
- coût mémoire de la structure ;
- coût serveur de création/suppression des subdivisions ;
- stratégie anti-oscillation lorsque les cellules alternent entre deux niveaux.

---

## 7. Orchestrateur global ou architecture hybride

**Statut : ouvert**

La logique d’activation doit tenir compte de la caméra, du joueur, du gameplay, du futur probable et du budget, mais la responsabilité logicielle exacte reste ouverte.

Deux directions restent possibles :

- gestionnaire global décidant de toutes les activations ;
- gestionnaire global fixant les priorités et cellules locales affinant les décisions.

**Proposition de l’assistant non explicitement validée :**

Un `Water Manager` global gère budget, priorités et événements, tandis que les cellules/domaines fournissent les données locales nécessaires à la décision.

---

## 8. Seuils d’activation et d’arrêt

**Statut : ouvert**

Les facteurs sont connus mais leurs seuils ne le sont pas.

À calibrer :

- distance joueur ;
- distance caméra ;
- taille à l’écran ;
- angle de vue ;
- occlusion ;
- vitesse du joueur ;
- temps estimé avant interaction ;
- masse et énergie d’un objet ;
- probabilité de visibilité future ;
- importance gameplay ;
- durée minimale avant désactivation ;
- seuil d’énergie résiduelle ;
- hystérésis pour éviter des activations/désactivations répétées.

---

## 9. Simulation hors caméra

**Statut : partiellement défini**

La hiérarchie de comportement est définie, mais la méthode mathématique exacte ne l’est pas.

À décider selon les événements :

- simulation ralentie ;
- intégration à fréquence réduite ;
- modèle analytique ;
- état condensé ;
- simple événement mémorisé ;
- suppression complète.

Il faut également définir comment reconstruire un état plausible lorsqu’une zone redevient visible après plusieurs secondes.

---

## 10. Données conservées lors de la désactivation

**Statut : ouvert**

La règle générale est de conserver uniquement les perturbations importantes, mais le format de sauvegarde reste indéfini.

Variables candidates :

- grande onde ;
- énergie ;
- direction ;
- phase ;
- temps depuis l’événement ;
- vitesse moyenne ;
- sillage ;
- objets déplacés ;
- poche d’air ;
- durée estimée avant dissipation.

Le niveau de compression acceptable doit être défini expérimentalement.

---

## 11. Filtre de prédiction des objets

**Statut : partiellement défini**

Le système général de prédiction est accepté, mais les critères exacts restent ouverts.

À définir :

- quels types d’objets sont prédits ;
- horizon temporel maximal ;
- seuil d’énergie ;
- seuil de visibilité ;
- fréquence de réévaluation ;
- coût maximal autorisé pour la prédiction ;
- traitement des objets contrôlables ;
- traitement des collisions potentielles ;
- prise en compte des actions gameplay possibles.

---

## 12. Paliers de confiance et zones d’impact probables

**Statut : partiellement défini**

L’idée d’une confiance par paliers est présente, notamment pour les avions et véhicules contrôlables, mais les paliers eux-mêmes ne sont pas définis.

À préciser :

- nombre de paliers ;
- critères de passage d’un palier à l’autre ;
- taille de la région d’impact préparée ;
- cas où une grande enveloppe coûte plus cher qu’un démarrage temps réel tardif ;
- stratégie lorsque l’objet redevient contrôlable ;
- traitement des influences gameplay potentielles.

---

## 13. Erreur acceptable d’un précalcul

**Statut : ouvert**

Aucun seuil de position, rotation ou temps n’est défini pour décider si un précalcul reste valable.

À tester :

- seuil absolu ;
- seuil relatif à la taille de l’impact ;
- seuil basé sur le coût de correction ;
- abandon immédiat en cas de divergence ;
- récupération partielle pour certains phénomènes, notamment bulles ou coulées.

**Proposition de l’assistant non validée :**

Conserver ou corriger un précalcul uniquement si sa récupération est estimée moins coûteuse qu’un nouveau calcul.

---

## 14. Précalcul : préparation ou avance physique

**Statut : partiellement défini**

Le projet accepte le précalcul et l’avance temporaire plus rapide que le temps réel si cela réduit la charge au moment critique, mais la frontière entre simple préparation et simulation du futur reste ouverte.

À mesurer :

- coût d’allocation anticipée ;
- coût de génération des cellules ;
- coût de construction des voisinages ;
- coût de préparation des collisions ;
- intérêt de simuler quelques pas futurs ;
- coût du calcul perdu lorsqu’une prédiction devient fausse.

Aucun rollback temporel complet n’est prévu.

---

## 15. Modèle exact des courants

**Statut : ouvert — important**

Le LOD de courant est validé, mais sa formulation physique ne l’est pas.

À déterminer :

- quand un vecteur 2D suffit ;
- quand utiliser un champ 2D spatialement variable ;
- quand activer un champ 3D ;
- influence de la profondeur ;
- influence du vent ;
- marée ;
- bathymétrie ;
- température et densité si nécessaires ;
- apports de rivières ;
- interaction avec les vagues ;
- interaction avec les domaines physiques locaux ;
- possibilité ou non pour une perturbation locale de modifier temporairement le courant macroscopique.

---

## 16. Modèles spécifiques mer, lac, rivière et canal

**Statut : partiellement défini**

Les différences générales sont connues, mais les équations et contraintes restent ouvertes.

### Rivière

- valeur de débit imposée ;
- variation saisonnière ;
- récupération du débit après obstacle ;
- couplage avec zones 3D locales.

### Canal

- débit contrôlé ;
- possibilité de réglage gameplay ;
- vannes, biefs ou autres structures éventuelles.

### Lac

- conservation du niveau moyen ;
- apports et sorties ;
- courants faibles ;
- réaction aux perturbations locales.

### Mer / océan

- champ de courant global ;
- relation exacte entre courant et houle ;
- niveau de détail régional.

---

## 17. Transferts entre volumes finis

**Statut : partiellement défini**

Les transferts ne doivent être simulés que s’ils sont utiles visuellement ou pour le gameplay, mais les cas limites restent ouverts.

À définir :

- fuite entre deux volumes ;
- débordement ;
- infiltration dans sol ;
- ruissellement ;
- évacuation vers caniveau ;
- transfert vers rivière ou mer ;
- seuil de création d’une nouvelle flaque ;
- moment où un transfert devient trop faible pour être représenté.

---

## 18. Solveur ou combinaison de solveurs

**Statut : ouvert — décision expérimentale majeure**

Aucune méthode physique n’est encore retenue.

Le choix doit être fondé sur des benchmarks comparant :

- qualité visuelle ;
- coût CPU/GPU ;
- stabilité ;
- comportement sous raffinement adaptatif ;
- capacité à gérer les frontières mobiles ;
- couplage avec solides ;
- transfert entre niveaux de précision ;
- suitability pour plages, impacts, bulles et immersion ;
- facilité de synchronisation gameplay.

Candidats possibles à évaluer ultérieurement : méthodes particulaires, grille, hybrides ou solveurs spécialisés.

**Proposition de l’assistant non validée comme architecture finale :**

Concevoir une interface de domaine indépendante du solveur afin de pouvoir remplacer ou comparer plusieurs technologies sans modifier l’orchestrateur général.

---

## 19. Changement de solveur pendant une simulation

**Statut : ouvert**

Il n’est pas défini si une même zone peut :

- changer de solveur en cours de vie ;
- utiliser plusieurs solveurs simultanément ;
- déléguer les bulles, surface ou splash à des modules spécialisés ;
- transférer son état vers une autre méthode sans rupture.

Ce point doit être testé avant toute généralisation.

---

## 20. Échelle maximale et événements extrêmes

**Statut : partiellement défini**

Le principe macroscopique-global / physique-locale est validé, mais les seuils de changement de modèle ne le sont pas.

À déterminer pour :

- très grand navire ;
- avion ;
- vaisseau ;
- explosion majeure ;
- tsunami ;
- effondrement massif ;
- événement dépassant plusieurs domaines locaux.

Le seuil doit probablement dépendre du coût plutôt que d’une taille géométrique unique.

---

## 21. Profondeur maximale de simulation

**Statut : ouvert**

Une profondeur de l’ordre de 200 m a été évoquée comme plafond possible, mais elle n’est pas validée.

À définir :

- plafond global ;
- plafond par type d’événement ;
- comportement en eau très profonde ;
- remplacement de la simulation volumique par un modèle de bulles, courant ou événement condensé ;
- coût mémoire des colonnes profondes.

---

## 22. Flottabilité

**Statut : ouvert — décision expérimentale majeure**

Le système de flottabilité n’est pas choisi.

Options à benchmarker :

- flottabilité directement issue du solveur ;
- modèle de flottabilité séparé ;
- modèle hybride avec flottabilité principale séparée et corrections provenant de la simulation dynamique.

Critères de validation :

- sensation ;
- réalisme visuel ;
- stabilité ;
- roulis ;
- tangage ;
- réponse aux vagues ;
- latence ;
- coût CPU/GPU ;
- coût réseau ;
- fonctionnement lorsqu’une zone détaillée disparaît.

**Proposition de l’assistant :**

Traiter la flottabilité comme une décision expérimentale liée aux benchmarks du solveur plutôt que la figer théoriquement.

---

## 23. Interface simulation → rendu

**Statut : ouvert — décision expérimentale majeure**

Il n’est pas encore défini si le solveur doit :

- générer directement la surface visible ;
- produire des champs physiques utilisés ensuite par le rendu ;
- générer une surface grossière raffinée ensuite par le renderer ;
- utiliser plusieurs méthodes selon les situations.

Données potentielles :

- hauteur ;
- vitesse ;
- pression ;
- turbulence ;
- densité ou fraction d’eau ;
- énergie ;
- normales ;
- gradients ;
- métriques de mousse ou de splash.

**Proposition de l’assistant non validée :**

Supporter plusieurs chemins de reconstruction derrière une interface commune, puis choisir par benchmark le meilleur rapport coût/rendu selon le type de domaine.

---

## 24. Déclenchement de mousse, spray et bulles

**Statut : partiellement défini**

La logique sera probablement hybride, mais les métriques ne sont pas choisies.

À définir :

- vitesse ;
- courbure ;
- énergie ;
- turbulence ;
- compression ;
- collision ;
- seuils visuels ;
- distance caméra ;
- densité de particules ;
- durée de vie ;
- conditions de passage d’un phénomène visuel à un phénomène physique.

---

## 25. Simulation de l’air

**Statut : ouvert**

L’air doit être approximé dans la majorité des situations, mais les cas nécessitant une véritable interaction volumique ne sont pas formalisés.

À tester :

- grosses poches d’air ;
- cavité créée par impact ;
- air emprisonné sous un objet ;
- remontée après immersion ;
- influence sur flottabilité ;
- influence sur joueur ou objets ;
- approximation par champs de courant plutôt que solveur multiphasique complet.

---

## 26. Rochers et zones turbulentes persistantes

**Statut : partiellement défini**

Il a été envisagé que certains gros rochers conservent une interaction persistante, mais le niveau de simulation à distance reste à préciser.

À déterminer :

- précalcul permanent ;
- boucle dépendante de la houle ;
- activation seulement lorsque visible ;
- activation physique lorsque bateau/joueur arrive ;
- coût mémoire d’une bibliothèque d’états précalculés.

---

## 27. Précalcul côtier et météo

**Statut : partiellement défini**

Les conditions météorologiques peuvent être connues à l’avance et exploitées pour préparer les côtes, mais il reste à définir :

- nombre d’états précalculés ;
- interpolation entre états ;
- dépendance à la marée ;
- dépendance à la houle ;
- bathymétrie ;
- réaction à un changement brutal causé par gameplay ;
- volume de données stockées.

---

## 28. Budget numérique par frame

**Statut : ouvert**

Le projet exige une réponse temps réel sans retard perceptible, mais aucun budget chiffré n’est fixé.

À mesurer sur les plateformes cibles :

- budget CPU ;
- budget GPU ;
- mémoire ;
- bande passante réseau ;
- temps maximal d’un domaine ;
- coût acceptable d’un événement critique ;
- seuil de dégradation automatique ;
- vitesse de récupération après surcharge.

L’ordre de dégradation est défini, mais pas les valeurs numériques.

---

## 29. Couplage entre LOD visuel et grille de simulation

**Statut : partiellement défini**

Le LOD visuel et la simulation sont conceptuellement séparés, mais il a été envisagé de réutiliser la même grille spatiale pour réduire le coût de gestion.

À définir :

- partage ou non des cellules ;
- fréquence de mise à jour indépendante ;
- capacité du rendu à demander un raffinement sans demander plus de physique ;
- capacité de la physique à rester précise hors caméra ;
- gestion des transitions visuelles lorsque la simulation change de niveau.

---

## 30. Autorité multijoueur et déterminisme

**Statut : partiellement défini — critique**

La philosophie générale est validée : gameplay partagé, grandes formes cohérentes, microdétails libres.

Reste à définir :

- données autoritaires serveur ;
- fréquence de synchronisation ;
- tolérance de divergence ;
- prédiction client ;
- resynchronisation lors du rapprochement de deux joueurs ;
- synchronisation des grandes vagues ;
- gestion de deux zones locales éloignées ;
- persistance serveur des événements hors caméra ;
- niveau de déterminisme nécessaire pour la flottabilité et les collisions.

---

## 31. Décisions proposées par l’assistant mais non verrouillées par l’utilisateur

Les points suivants ont été proposés pour structurer l’architecture. Ils ne doivent pas être confondus avec des exigences validées :

1. **Water Manager global + logique locale** pour gérer budget, priorités et événements.
2. **État minimal variable de la fausse eau selon la zone** plutôt qu’un format universel lourd.
3. **Récupération d’un précalcul uniquement si elle coûte moins cher qu’un nouveau calcul.**
4. **Interface de solveur agnostique** permettant de benchmarker plusieurs méthodes.
5. **Flottabilité traitée comme choix expérimental** plutôt que fixée immédiatement.
6. **Interface simulation/rendu pouvant supporter plusieurs chemins de reconstruction.**
7. **Serveur autoritaire sur le gameplay, clients libres sur les détails visuels**, dans la limite de la cohérence multijoueur générale déjà acceptée.

Ces propositions peuvent être retenues, modifiées ou rejetées après benchmark ou décision de conception.

---

## 32. Priorité recommandée pour la prochaine phase de conception

Sans choisir les solutions à la place de l’équipe, les inconnues les plus structurantes à résoudre en premier sont :

1. contenu minimal de la fausse eau ;
2. algorithme de zone de transition ;
3. relation cellule/domaine/solveur ;
4. stratégie serveur/client ;
5. interface générique du solveur ;
6. modèle de courant adaptatif ;
7. interface simulation/rendu ;
8. flottabilité ;
9. budgets numériques ;
10. seuils de prédiction, activation et désactivation.

Ces décisions conditionnent directement les prototypes et benchmarks suivants.
