# Système d’eau dynamique — Architecture conceptuelle globale

## 1. Objectif

Le projet vise un système d’eau temps réel pour jeu vidéo capable de réagir dynamiquement à son environnement, aux objets et aux joueurs sans imposer une simulation physique complète de toutes les masses d’eau.

La physique est utilisée lorsqu’elle est nécessaire à la cohérence visuelle et interactive. Les zones qui peuvent être représentées de manière crédible par des modèles simplifiés, analytiques, précalculés ou purement visuels le restent afin de préserver le budget CPU/GPU, la mémoire et le coût réseau.

Le critère de validation principal est le rendu perçu en temps réel : absence de retard visible, continuité des mouvements, interaction crédible avec le décor, conservation des effets importants et absence de ruptures flagrantes entre eau simplifiée et eau simulée. La précision scientifique est recherchée lorsqu’elle améliore la cohérence, mais elle n’est pas une fin en soi.

Principe directeur :

> L’eau reste aussi simplifiée que possible tant que cela n’altère pas sa crédibilité. La simulation physique apparaît localement, spatialement et temporellement lorsque l’interaction avec l’environnement, les objets ou l’eau elle-même l’exige. Le domaine et la précision évoluent dynamiquement avec la scène, tandis que prédiction, précalcul, transitions et artifices visuels sont utilisés pour garantir une réponse en temps réel, sans rupture perceptible avec l’environnement.

---

## 2. Familles de systèmes hydrauliques

Le moteur distingue deux familles principales.

### 2.1 Grandes masses et réseaux d’eau

Sont concernés notamment :

- mers et océans ;
- lacs ;
- rivières ;
- canaux.

Ces systèmes ne cherchent pas à conserver exactement chaque volume local d’eau. Ils utilisent des grandeurs macroscopiques suffisantes pour assurer la cohérence générale : niveau moyen, débit moyen ou fluctuant, courants, direction de propagation et paramètres dominants des vagues.

Le comportement local détaillé n’est déclenché que lorsque l’interaction devient perceptuellement ou fonctionnellement importante.

### 2.2 Volumes finis quantifiables

Sont concernés notamment :

- piscines ;
- réservoirs ;
- volumes d’eau isolés ;
- certaines eaux souterraines ;
- accumulations de pluie.

Ces systèmes suivent un volume adaptatif à une précision choisie. Le volume peut augmenter, diminuer, fuir, se transférer ou déborder.

Les très petites variations peuvent être ignorées par un seuil adaptatif. La pertinence du seuil dépend de l’échelle du contenant : une quantité significative dans un bidon peut être négligeable dans une piscine.

La pluie peut être intégrée par moyenne ou précalcul lorsque les conditions sont connues. Les ouvertures vers le ciel, couvertures et autres obstacles doivent empêcher l’ajout d’eau lorsqu’ils rendent la pluie physiquement impossible.

Les débordements ou transferts vers d’autres systèmes ne sont simulés que lorsqu’ils apportent une valeur visuelle ou gameplay suffisante. Le matériau du sol peut permettre l’absorption ou la disparition approximative d’un surplus.

---

## 3. Organisation spatiale 3D

### 3.1 Grille de référence

Le monde aquatique utilise une grille 3D fixe servant de référence spatiale stable, notamment pour :

- localiser les domaines d’eau ;
- identifier les zones actives ;
- simplifier l’adressage ;
- faciliter les échanges client/serveur ;
- fournir un repère spatial cohérent pour l’organisation et les échanges du système.

La grille de référence est stable, mais le calcul effectué à l’intérieur de ses régions est dynamique.

### 3.2 Subdivision adaptative

Les zones de simulation peuvent être subdivisées selon des niveaux prédéfinis. La subdivision peut être anisotrope : les axes X, Y et Z n’ont pas besoin d’être raffinés de la même manière.

Le but principal est d’éviter de simuler des volumes inutiles. Le raffinement spatial sert donc en priorité à épouser la forme du domaine utile.

Exemples :

- une perturbation circulaire ou elliptique ne doit pas forcer le calcul d’un grand volume rectangulaire plein ;
- une barque peut demander une zone large mais peu profonde ;
- un objet qui coule peut demander une colonne verticale localisée ;
- une traînée de bateau peut activer une région longue et étroite.

Les petites subdivisions sont utilisées principalement sur les contours et dans les régions où la forme du domaine doit être précisée. Les régions centrales plus régulières peuvent conserver des cellules plus grandes.

### 3.3 Cellules et solveurs

La cellule n’est pas nécessairement une simulation indépendante.

Le système doit pouvoir regrouper plusieurs cellules en un domaine physique commun ou, lorsque cela est plus efficace, faire fonctionner plusieurs domaines communicants. Le choix dépend du contexte, de la topologie de la région et du coût de calcul.

Les cellules peuvent avoir plusieurs niveaux d’activité :

- inactive ;
- simplifiée ;
- partiellement active ;
- simulation active ;
- simulation à niveau de détail supérieur.

Le niveau spatial et le niveau de précision physique ne sont pas strictement équivalents. Une subdivision peut être utilisée uniquement pour mieux découper le domaine sans augmenter la précision du solveur. Inversement, certains solveurs peuvent augmenter leur précision sans exiger une nouvelle subdivision spatiale.

### 3.4 Disparition et réduction des domaines

Les cellules ou domaines devenus inutiles disparaissent progressivement. Une perturbation importante peut être conservée sous une forme réduite ou compressée afin de permettre une continuité ultérieure. Les perturbations insignifiantes peuvent être supprimées.

---

## 4. Profondeur de simulation adaptative

La profondeur simulée dépend de l’interaction réelle.

### 4.1 Cas peu profonds

Pour une barque ou un objet de surface, la simulation peut se limiter à :

- la coque ;
- une marge sous la coque ;
- une couche d’eau supplémentaire nécessaire aux vagues ;
- un courant simplifié en profondeur.

Il n’est pas nécessaire de simuler toute la colonne d’eau.

### 4.2 Cas complexes

La profondeur augmente lorsque l’événement le justifie :

- grand navire ;
- immersion d’un objet ;
- objet qui coule ;
- crash ;
- explosion de surface ou sous-marine ;
- grosses poches d’air ;
- bulles ou remontées importantes.

Pour certains événements, le domaine profond peut suivre verticalement l’objet. Les couches déjà traversées peuvent être supprimées ou converties vers un état simplifié lorsque leur turbulence devient négligeable.

### 4.3 Plages

Sur les plages, le domaine peut nécessiter une continuité jusqu’au fond et jusqu’au sable lorsque la zone est pertinente pour le joueur ou la caméra. Les zones éloignées ne doivent pas conserver inutilement ce niveau de calcul.

---

## 5. Trois régimes de représentation de l’eau

Le système général repose sur trois régimes continus.

### 5.1 Eau simplifiée

La majorité des grandes masses d’eau utilise une représentation peu coûteuse : formules, champs analytiques, paramètres de houle, niveaux moyens, courants approximatifs et autres représentations adaptées au contexte.

Elle doit fournir suffisamment d’information pour :

- produire un rendu cohérent ;
- prédire les grandes composantes de mouvement ;
- alimenter une future zone de transition ;
- éviter un décalage visible lorsqu’une simulation physique est activée.

### 5.2 Zone de transition

La zone de transition assure le couplage entre eau simplifiée et simulation physique.

Elle doit notamment :

- traduire les données du modèle simplifié vers la simulation ;
- mélanger les états adjacents ;
- permettre aux vagues simplifiées d’entrer dans le domaine physique ;
- permettre aux perturbations physiques importantes de quitter le domaine détaillé sous une forme simplifiée ;
- empêcher les réflexions, discontinuités ou changements de phase visibles liés à la frontière numérique.

La frontière entre régimes doit être imperceptible.

### 5.3 Simulation physique locale

La simulation physique détaillée est réservée aux zones où les approximations ne suffisent plus, par exemple :

- plages et rouleaux ;
- contact avec rochers ou obstacles ;
- bateaux et sillages ;
- objets pénétrant dans l’eau ;
- crashs ;
- collisions importantes ;
- turbulences ;
- poches d’air significatives ;
- effets sous-marins nécessitant un volume réel.

La simulation modifie directement sa zone de transition. Le domaine actif peut grandir, rétrécir, se déplacer, fusionner ou se séparer selon l’évolution de la scène.

---

## 6. Fusion et séparation des zones dynamiques

Deux domaines physiques qui se touchent doivent être traités comme une interaction commune lorsque leurs phénomènes se couplent.

Exemples :

- bateau entrant dans une zone déjà perturbée ;
- deux sillages qui se croisent ;
- joueur entrant dans la zone d’interaction d’un objet ;
- vague arrivant sur une zone de rochers déjà active.

Lorsque les interactions s’éloignent, le domaine peut être séparé à nouveau.

La séparation ne doit pas provoquer de rupture. Les zones abandonnées doivent progressivement revenir vers l’état simplifié ou calme, éventuellement après conversion de la perturbation restante en représentation condensée.

---

## 7. Activation dynamique et priorisation

Le système n’active pas l’eau détaillée uniquement selon une distance fixe.

La décision dépend d’un ensemble de facteurs, notamment :

- proximité du joueur ;
- visibilité caméra ;
- taille occupée à l’écran ;
- direction du regard ;
- vitesse et direction du joueur ;
- probabilité d’interaction prochaine ;
- masse et vitesse d’un objet ;
- énergie potentielle d’un impact ;
- importance gameplay ;
- probabilité que l’événement devienne visible prochainement ;
- budget de calcul disponible.

La proximité du joueur et la visibilité sont des facteurs prioritaires, sans être les seuls.

### 7.1 Domaine prédictif autour du joueur

Lorsqu’un joueur s’approche d’une masse d’eau, la préparation peut être orientée vers la zone qu’il est susceptible d’atteindre plutôt que former un rayon uniforme.

Un joueur courant vers une plage peut ainsi provoquer la préparation d’une région plus longue devant lui qu’un joueur immobile ou regardant dans une autre direction.

---

## 8. Prédiction des interactions futures

Le système doit profiter des périodes où un événement futur est suffisamment prévisible pour préparer la simulation avant que l’interaction ne devienne visible.

### 8.1 Filtrage avant prédiction

Tous les objets ne doivent pas être prédits en permanence. Une prédiction n’est utile que si plusieurs conditions sont réunies :

- possibilité réelle d’atteindre l’eau ;
- impact potentiellement visible ou gameplay ;
- énergie suffisante ;
- horizon temporel utile ;
- coût de prédiction inférieur au bénéfice attendu ;
- trajectoire suffisamment contrainte.

### 8.2 Objets balistiques

Pour un véhicule quittant un pont, la trajectoire peut rapidement devenir suffisamment déterministe pour estimer :

- point d’impact ;
- vitesse ;
- orientation ;
- rotation ;
- région de simulation utile.

Le temps de vol devient alors une fenêtre de calcul permettant de préparer le domaine d’eau.

### 8.3 Objets contrôlables

Les avions, planeurs et autres objets capables de modifier activement leur trajectoire demandent une logique de certitude par paliers plutôt qu’un simple point d’impact figé.

Le système peut tenir compte notamment de :

- inertie ;
- vitesse verticale et horizontale ;
- capacité restante à éviter l’impact ;
- panne ou perte de contrôle ;
- altitude ;
- actions gameplay susceptibles de modifier la trajectoire.

### 8.4 Événements gameplay futurs

Certaines actions possibles doivent pouvoir réduire la confiance dans une prédiction, par exemple lorsqu’un projectile ou une collision future peut encore modifier la trajectoire avant l’impact.

Le moteur ne doit toutefois pas chercher à prédire arbitrairement tout le gameplay : la prédiction reste soumise à un filtre de coût et de pertinence.

### 8.5 Précalcul

Le précalcul peut préparer :

- cellules et domaines ;
- allocations mémoire ;
- géométrie de collision ;
- état initial ;
- conditions de transition ;
- premières étapes de simulation lorsque cela réduit le coût au moment critique.

Une simulation peut temporairement avancer plus vite que le temps réel lorsque cela permet d’utiliser efficacement une fenêtre disponible avant l’événement.

Aucun système de rollback temporel complet n’est prévu : une prédiction devenue fausse doit être corrigée depuis l’état courant ou abandonnée.

---

## 9. Simulation hors caméra et persistance perceptuelle

Une zone invisible ne doit pas continuer à consommer le même budget qu’une zone visible.

Le système suit une hiérarchie de conservation :

1. **visible et importante** : simulation normale ;
2. **hors caméra mais encore importante** : simulation réduite ou évolution simplifiée ;
3. **potentiellement visible plus tard** : événement ou état condensé ;
4. **insignifiante** : suppression.

Une explosion hors caméra n’a besoin d’être conservée que si ses conséquences peuvent devenir visibles ou influencer le gameplay.

Les événements peuvent être stockés sous une forme condensée : position, temps, énergie, rayon, direction et paramètres utiles à une reconstruction ultérieure.

La fidélité de conservation est limitée par la mémoire perceptuelle du joueur. Une turbulence détaillée que le joueur ne peut plus comparer précisément après plusieurs secondes n’a pas besoin d’être reproduite bit à bit.

Les perturbations importantes — grande vague, sillage significatif, déplacement d’objet, grosse poche d’air ou conséquence gameplay — doivent survivre suffisamment longtemps pour éviter une incohérence évidente.

---

## 10. Courants macroscopiques à niveau de détail adaptatif

Les simulations locales sont alimentées par un système de courants approximatifs.

Le courant possède son propre niveau de détail, distinct :

- du découpage spatial ;
- du niveau de simulation physique ;
- du LOD visuel.

Le niveau de courant peut évoluer depuis un simple vecteur ou champ de surface jusqu’à un champ 3D dépendant de la profondeur lorsque la situation l’exige.

Exemples de raffinement :

- haute mer calme : courant simplifié ;
- sillage de bateau : champ local plus précis si nécessaire ;
- embouchure : variation spatiale accrue ;
- plage ou bathymétrie complexe : raffinement important ;
- simulation profonde : composante verticale ou 3D si elle influence le résultat.

Pour les rivières et canaux, un débit macroscopique peut être utilisé comme contrainte générale afin que les perturbations locales restent compatibles avec l’écoulement principal.

Pour les lacs, la logique repose davantage sur un niveau moyen, des apports et des courants faibles que sur un débit traversant unique.

---

## 11. Côtes, plages et zones persistantes

### 11.1 Plages

Les vagues éloignées peuvent être précalculées ou simplifiées. Le niveau physique augmente avec :

- visibilité ;
- proximité du joueur ;
- agitation ;
- interaction avec le fond ;
- déferlement.

Sur une plage, la formation du rouleau est un candidat privilégié à la simulation 3D. Une représentation 2D ne doit pas être utilisée si elle produit un rouleau ou une mousse manifestement faux.

Les très petites vaguelettes proches du repos peuvent revenir vers des méthodes simplifiées lorsque cela reste crédible.

### 11.2 Bathymétrie

La bathymétrie doit influencer les vagues avant leur arrivée dans la zone physique. Cette influence peut être calculée par un modèle macroscopique ou précalculé tant que la transition vers la simulation locale reste cohérente.

### 11.3 Rochers et obstacles

Les gros rochers et structures immergées constituent des zones d’interaction potentiellement turbulentes. Lorsqu’un bateau, un joueur, une vague importante ou un autre événement dynamique rejoint une telle zone, les interactions concernées doivent être intégrées dans le même domaine physique pertinent.

Le niveau de calcul à maintenir autour de ces obstacles lorsqu’ils sont éloignés ou invisibles n’est pas fixé dans cette spécification.

---

## 12. Adaptation interne de la simulation

Le domaine physique lui-même possède un niveau de détail de simulation.

Dans une zone régulière, de grands éléments de fluide peuvent représenter les masses principales. Lorsqu’une région devient chaotique — impact, splash, contact vague-rocher, rouleau — les éléments peuvent se subdiviser afin de mieux représenter les formes locales.

Lorsque le fluide revient au repos, ces éléments peuvent fusionner ou être remplacés par une représentation plus grossière.

Cette adaptation vise à concentrer les ressources sur les régions où le détail produit un gain visuel réel.

Des effets secondaires peuvent être ajoutés artificiellement lorsque leur simulation complète coûte trop cher pour un bénéfice faible.

---

## 13. Air, bulles, embruns et phénomènes secondaires

Les microphénomènes sont hiérarchisés par importance.

### 13.1 Microbulles

La majorité des microbulles est visuelle. Elles ne deviennent physiquement importantes que lorsque leur densité ou leur volume cumulé produit un effet perceptible.

### 13.2 Grosses bulles et poches d’air

Une grosse poche d’air peut :

- perturber la surface ;
- générer des vaguelettes ;
- créer une turbulence ;
- déplacer ou bousculer un joueur ;
- participer à la flottabilité locale.

Elle peut donc recevoir un traitement physique ou semi-physique distinct.

### 13.3 Embruns et gouttelettes

Les embruns et petites gouttelettes peuvent être principalement visuels. Une petite goutte qui retombe dans l’eau n’a pas besoin de lancer une nouvelle interaction physique complète.

Seules les masses suffisamment importantes doivent transmettre une perturbation physique mesurable.

---

## 14. Interaction avec les solides

La bidirectionnalité eau-objet est réservée aux objets importants.

Les éléments gameplay peuvent :

- déplacer l’eau ;
- recevoir des forces de l’eau ;
- générer un sillage ;
- subir courant, vagues ou turbulence.

Certains objets fixes du décor peuvent rester des frontières imposées au fluide sans simulation dynamique propre.

La zone physique suit généralement les objets importants lorsqu’ils se déplacent dans ou sur l’eau.

---

## 15. Séparation physique gameplay / détail graphique

Le système distingue les phénomènes qui doivent être cohérents pour le gameplay de ceux qui peuvent être générés localement par le rendu.

### 15.1 Données gameplay

Doivent rester cohérentes entre joueurs lorsque pertinentes :

- position et mouvement des objets importants ;
- forces majeures de l’eau ;
- grandes vagues ;
- déplacements induits ;
- courant significatif ;
- impacts ayant une conséquence gameplay ;
- effets capables de renverser, pousser ou déplacer un acteur.

### 15.2 Détails graphiques

Peuvent différer entre clients :

- microbulles ;
- gouttelettes ;
- petites éclaboussures ;
- mousse fine ;
- micro-vagues ;
- détails de spray.

Le multijoueur vise donc une cohérence des conséquences et des grandes formes, pas un déterminisme bit à bit de tous les détails visuels.

---

## 16. LOD visuel

Le LOD visuel est un système distinct du niveau de simulation physique.

Une simulation précise peut tourner loin de la caméra et être rendue simplement. À l’inverse, une eau proche peut être visuellement très détaillée tout en restant presque entièrement artificielle si aucune interaction ne nécessite de physique.

Chaque composante visuelle peut disposer de son propre LOD :

- géométrie de surface ;
- vagues secondaires ;
- écume ;
- spray ;
- gouttelettes ;
- bulles ;
- transparence ;
- réfraction ;
- caustiques ;
- particules sous-marines.

Le LOD peut prendre en compte :

- distance ;
- taille à l’écran ;
- visibilité ;
- direction du regard ;
- occlusion ;
- importance de l’événement.

Un événement très éloigné n’impose un niveau visuel élevé que s’il est réellement visible ou occupe suffisamment de pixels.

Le rendu peut ajouter des détails artificiels à partir d’une simulation physique plus grossière lorsqu’ils améliorent fortement le réalisme pour un coût faible.

---

## 17. Budget temps réel et dégradation contrôlée

Le système doit maintenir une réponse sans retard perceptible.

En surcharge, la dégradation suit une priorité générale :

1. réduire la taille des domaines simulés ;
2. réduire la résolution physique ;
3. simplifier ou supprimer les interactions éloignées ;
4. réduire la fréquence de simulation ;
5. diminuer les particules et effets secondaires.

Une interaction lointaine peut être remplacée par un effet visuel ou un état simplifié tant qu’elle n’affecte pas le gameplay.

Le moteur doit combiner :

- profils de qualité configurables ;
- adaptation automatique au matériel ;
- ajustement dynamique selon la charge réelle.

Un événement critique peut temporairement dépasser le budget normal lorsque cela évite une rupture flagrante, sous réserve de ne pas provoquer un retard perceptible global.

---

## 18. Multijoueur — philosophie générale

Le système multijoueur distingue trois niveaux de cohérence :

### 18.1 Gameplay partagé

Les conséquences importantes doivent être communes : position des objets, poussées principales, grandes vagues et événements influençant le gameplay.

### 18.2 Grandes formes cohérentes

Les joueurs doivent percevoir une scène compatible : même événement principal, sillage ou perturbation majeure de forme comparable.

### 18.3 Détails locaux libres

Les microdétails sont reconstruits localement et peuvent différer entre clients sans conséquence gameplay.

Deux joueurs éloignés peuvent disposer de zones de simulation locales différentes tant que l’état du monde reste cohérent lorsqu’ils interagissent ou se rapprochent.

---

## 19. Très grands événements

Lorsqu’un phénomène devient trop grand pour être traité efficacement par une simulation volumique complète, sa propagation globale est représentée par un modèle macroscopique ou simplifié.

La simulation 3D détaillée apparaît uniquement là où les interactions locales le justifient.

Exemple de principe :

- propagation d’un tsunami en pleine mer : représentation macroscopique ;
- approche de la côte : raffinement ;
- interaction avec plage, obstacles, bâtiments ou joueur : simulation 3D locale.

Cette logique s’applique de manière générale aux très grands navires, crashs ou événements dont l’échelle excède le domaine pratique d’un solveur volumique unique.

---

## 20. Résumé architectural

Le système doit être considéré comme un **orchestrateur de régimes de simulation**, et non comme un solveur d’océan unique.

Il combine :

1. une représentation macroscopique permanente et peu coûteuse ;
2. une grille 3D stable pour organiser le monde ;
3. des domaines physiques locaux et adaptatifs ;
4. une zone de transition garantissant la continuité ;
5. des niveaux de détail indépendants pour simulation, courants et rendu ;
6. un système de prédiction et de précalcul ;
7. une persistance perceptuelle des événements hors caméra ;
8. une dégradation contrôlée selon le budget ;
9. une séparation claire entre gameplay synchronisé et détails visuels locaux ;
10. une activation de la physique uniquement là où elle améliore réellement la cohérence temps réel.

