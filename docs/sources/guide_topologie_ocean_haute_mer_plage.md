# Océan temps réel — Topologie de surface, haute mer, littoral et qualité du rendu

**Document technique autonome — version 1.0 — 20 septembre 2026**  
**Public :** programmation graphique, simulation numérique, moteur de jeu, direction technique et artistique.  
**Objectif :** construire une surface d'océan continue, stable et convaincante, depuis l'horizon jusqu'à la plage et aux contacts avec les objets, tout en sachant exactement **ce qui appartient au maillage, au modèle de vagues, à la simulation volumique et au shader**.

> **Périmètre et statut.** Ce document propose une architecture de référence ; ce n'est pas une description d'un moteur existant ni une garantie de performance sur un GPU donné. Les équations classiques et résultats de recherche sont signalés par des références [S1]–[S12]. Les budgets, interfaces, algorithmes de raccordement et paramètres de production sont des **propositions d'ingénierie à profiler et valider**. Les deux images reçues servent de comparaison visuelle, et non de mesures bathymétriques ou spectrales.

---

## Sommaire

1. Lire correctement les deux images
2. Les quatre « topologies » à ne pas confondre
3. Convention spatiale, données et invariants
4. Haute mer : génération physique et géométrie
5. Topologie du maillage : grille, clipmap, projection, raccords
6. Passage de la haute mer à la côte
7. Bord de plage : déferlement, swash, montée et retrait
8. Rochers, bateaux, crashs : limites du heightfield et domaines 3D
9. Raccordement entre modèles, continuité et conservation
10. Normales, réflexion, mousse et amélioration du rendu
11. Implémentation GPU et ordonnancement
12. Scènes de validation, mesures et diagnostics
13. Plan de réalisation, arbitrages, glossaire et sources

---

# 1. Lecture technique des deux images de référence

**Image A — photographie de haute mer :** mer bleu foncé, grandes masses de relief lisibles, rides secondaires plus désordonnées, crêtes non uniformes, reflets sombres et clairs distribués de manière discontinue ; ciel saturé, horizon relativement net. **Image B — rendu :** mer beaucoup plus claire, nombreuses stries lumineuses fines et assez continues, grosses déformations moins lisibles au premier plan, horizon avec un relief doux. Ces descriptions sont qualitatives : angle de caméra, météo, exposition et résolution peuvent expliquer une partie de l'écart.

Ne pas conclure qu'une « mauvaise topologie » explique nécessairement l'image B. Plusieurs causes produisent un aspect similaire :

| Observation | Hypothèses à tester | Test discriminant |
|---|---|---|
| Crêtes larges peu lisibles | énergie insuffisante aux grandes longueurs d'onde ; caméra ; normales fines dominantes | matériau mat, sans normal map, vue fixe |
| Stries blanches nombreuses | ciel clair réfléchi ; normales trop fortes ; aliasing de pentes ; exposition | remplacer ciel par environnement contrôlé, puis désactiver les normales de détail |
| Surface qui paraît « plate » | déplacement vertical insuffisant ; échelle de scène ; perspective | wireframe + carte de hauteur en fausses couleurs |
| Détails comparables à toutes distances | filtrage spectral/LOD inadéquat, mipmapping des normales | capture fixe et vidéo avec zoom et déplacement caméra |
| Ondulations trop parallèles | étalement directionnel trop faible ou artefact d'échantillonnage | carte des directions et spectre polaire |

**Ordre de diagnostic :** (1) caméra commune, (2) maillage et surface sans matériau, (3) spectre et pentes, (4) éclairage et matériau, (5) normales fines et antialiasing. Ne pas modifier tout simultanément.

---

# 2. Les quatre topologies à ne pas confondre

Le mot *topologie* désigne ici quatre choses différentes. Leur séparation est l'une des décisions les plus importantes du moteur.

1. **Topologie du domaine physique** : quelles zones d'eau communiquent, où sont les fonds, quais et obstacles, quels flux passent d'une cellule à l'autre ? Elle peut utiliser une grille 2D, un maillage triangulaire ou des blocs 3D.
2. **Topologie mathématique de la surface libre** : la surface est-elle une fonction de hauteur unique `z = eta(x,y,t)` ? Peut-elle se retourner, se séparer, générer des gouttes ou enfermer de l'air ? Un heightfield ne peut pas représenter ces phénomènes.
3. **Topologie du maillage de rendu** : sommets, triangles, indices, coutures entre niveaux de détail, découpes près des rivages et des objets. Elle sert à *afficher* la surface et ne doit pas dicter à elle seule la résolution de la physique.
4. **Topologie apparente du shader** : normal maps, mousse, transparence et reflets produisent de petits détails **sans créer de véritable volume**. Une normal map ne modifie ni la silhouette ni la collision.

### Schéma 1 — architecture conceptuelle

```text
                    ÉTAT MÉTÉO / VENT / MARÉE
                              |
                    +---------v----------+
                    | Spectre haute mer   |----> vagues à l'horizon
                    | eta, pente, vitesse |
                    +---------+----------+
                              |
                  bathymétrie / profondeur
                              v
                    +--------------------+
                    | Propagation côtière |<----> courants, fond
                    | réfraction, shoaling|
                    +---------+----------+
                              |
                +-------------+------------------+
                |                                |
          eau non déferlante              perte de validité
                |                                |
      +---------v----------+          +----------v----------+
      | Heightfield côtier |          | Déferlement / 3D    |
      | surface continue   |<-------->| zone locale        |
      +---------+----------+          +----------+----------+
                |                                |
                +---------------+----------------+
                                v
                   MAillage VISUEL + SHADER
                    réflexion, mousse, écume
                                |
                     profondeur / collisions
```

**Invariant fondamental :** la densité de triangles à l'écran n'est pas la taille des cellules de la simulation ; une division visuelle n'est pas obligatoirement une division physique ; l'ajout d'une zone 3D ne doit pas supprimer la contribution des vagues distantes ni créer une double surface visible.

---

# 3. Convention spatiale, données et invariants

Choisir une seule convention et l'écrire dans tous les modules. Exemple : `x,y` horizontaux en mètres, `z` vertical positif vers le haut, `z=0` niveau marin moyen de référence, `b(x,y)` altitude du fond, `eta(x,y,t)` élévation libre, `h=max(eta-b,0)` profondeur d'eau instantanée. Attention : **la bathymétrie statique `d0=-b` n'est pas la profondeur instantanée `h`** ; les confondre casse la plage mobile.

### Contrat minimal de données

| Donnée | Unité | Producteur | Consommateur |
|---|---:|---|---|
| `bathymetry b(x,y)` | m | terrain / authoring | propagation, collisions, optique |
| `eta(x,y,t)` | m | spectre / solveur local | rendu, couplage, gameplay |
| `U(x,y,t)` | m/s | courants / solveur | transport des vagues, écume |
| `u(x,y,z,t)` | m/s | modèle orbital / 3D | objets, injection aux limites |
| `N(f,theta,x,y,t)` ou modes cohérents | selon convention | champ de vagues | transport côtier, synthèse |
| `wet, h, flux` | bool, m, m²/s par largeur | solveur littoral | plage, bilan de masse |
| `break_intensity, foam_age` | adimensionnel, s | déferlement | rendu / écume |
| `surface_owner` | identifiant | arbitre de domaine | rendu exclusif / transitions |

**Précision numérique :** calculer les positions lointaines relativement à un origine locale flottante (floating origin) ; le temps de phase et les coordonnées spectrales doivent conserver une cohérence quand l'origine bouge. Ne pas modifier silencieusement la phase des vagues lors d'une translation du monde. Toutes les couches doivent recevoir le **même** temps de simulation, ou une interpolation temporelle explicitement définie.

**Invariants à surveiller à chaque pas :** `h >= 0`, pas de NaN, flux partagés égaux et opposés aux interfaces, continuité de `eta` et de la vitesse normale aux jonctions, absence de triangles dégénérés, normales orientées vers l'air pour la peau externe, surface visible unique sauf couches volontaires (écume, gouttes).

---

# 4. Haute mer : génération physique et géométrie

## 4.1 Hauteur, directions et spectre

Pour un modèle de surface non déferlante :

```math
eta(x,y,t) = sum_j a_j cos(k_j · (x,y) - omega_j t + phi_j)
```

Pour un champ gaussien synthétisé par FFT, les amplitudes et phases résultent d'un spectre directionnel et d'une symétrie hermitienne adéquate ; une FFT inverse fournit une grille de hauteur, éventuellement avec déplacement horizontal et dérivées. Tessendorf décrit cette famille de techniques [S1]. Le modèle historique GPU Gems sépare déplacement géométrique et détails de normales [S2].

Les modèles de spectres ont des usages différents :

- **Phillips** : construction informatique commode, non garantie comme état de mer calibré ; à ne pas prendre automatiquement pour un modèle océanographique validé.
- **Pierson–Moskowitz** : mer développée dans certaines hypothèses ; pas un spectre universel.
- **JONSWAP** : mer en développement à pic spectral marqué ; coefficients à paramétrer selon le scénario.
- **TMA** : famille de spectres à profondeur finie, extension de JONSWAP ; utile pour un *spectre local*, mais ne remplace pas la propagation spatiale, la réfraction ni le déferlement [S6].

La **hauteur significative** `Hs ≈ 4 sqrt(m0)` caractérise statistiquement le champ de vagues pour une convention de spectre cohérente, avec `m0` moment spectral d'ordre zéro. Ne pas l'utiliser comme hauteur de chaque vague individuelle.

## 4.2 Dispersion : le fond commence à influer bien avant la plage

Pour les petites ondes de gravité de surface, sans courant, en eau de profondeur `d` :

```math
omega² = g k tanh(k d)
c_phase = omega / k
c_group = (c_phase / 2) * [1 + 2 k d / sinh(2 k d)]
```

Avec `k=2π/λ`. Régimes pratiques (approximatifs) : `d/λ > 1/2` profond, `d/λ < 1/20` peu profond, entre les deux intermédiaire. En eau profonde, `omega² ≈ gk` ; en eau peu profonde, `c ≈ sqrt(gd)`. Les frontières ne sont pas des interrupteurs à utiliser directement dans un shader : la formule complète fonctionne continûment pour la théorie linéaire.

En présence d'un courant lent et presque uniforme, la fréquence intrinsèque s'écrit approximativement `sigma = omega - k·U`; employer la dispersion intrinsèque et tenir compte du Doppler, des blocages et de la réfraction. Ce raccourci n'est pas suffisant au voisinage de forts gradients de courant.

**Exemple purement illustratif :** une houle de `λ=100 m` commence à ressentir la profondeur dès l'ordre de `50 m` ; sa transformation vers la plage ne doit pas être repoussée aux cinq derniers mètres. La valeur exacte dépend du champ spectral local.

## 4.3 Géométrie du déplacement : hauteur et déplacement horizontal

Un champ de hauteur simple :

```text
P(x,y,t) = (x, y, eta(x,y,t))
```

Une variante « choppy » ou Gerstner déplace aussi les positions horizontales :

```text
P(x,y,t) = (x + Dx(x,y,t), y + Dy(x,y,t), eta(x,y,t))
```

Cette seconde représentation est **paramétrique** : les coordonnées `(x,y)` identifient des particules/points du paramétrage, pas forcément leur projection finale. La surface peut cesser d'être représentable par `z = eta(X,Y)` si la projection horizontale se replie. Le Jacobien horizontal

```math
J = det( I + ∂D_horizontal / ∂(x,y) )
```

doit être surveillé. `J → 0` signale une forte compression ; `J < 0` indique une inversion locale du paramétrage. Ne pas forcer une couche heightfield à produire des rouleaux : c'est le moment d'utiliser un modèle de déferlement, un domaine 3D ou un effet d'écume contrôlé. Pour une forme Gerstner particulière, GPU Gems discute la cambrure maximale et les boucles [S2].

**Crête haute ≠ bonne vague automatiquement :** la cambrure `ka = 2πa/λ` est distincte de l'amplitude absolue ; il faut aussi vérifier la profondeur relative `kh`, le spectre et le sens de propagation.

## 4.4 Structure multi-échelle sans double compter

Une proposition de rendu :

```text
eta_total = eta_houle_large + eta_vagues_moyennes + eta_locales_coherentes
N_shading  = combinaison des pentes physiques résolues et des rides sous-maille
```

Il ne faut **pas** ajouter séparément au shader une onde déjà présente dans le déplacement géométrique si cela la fait apparaître deux fois dans la pente. Distinguer :

- **Bande A :** grandes formes résolues dans la géométrie.
- **Bande B :** détails intermédiaires, géométrie proche / normales lointaines suivant leur taille projetée.
- **Bande C :** capillarité et micro-rides uniquement dans une représentation filtrée des normales et de la réflexion.

Ces bandes sont des **rôles visuels**, pas des compartiments de physique indépendants. La plage ou les objets peuvent injecter de l'énergie dans plusieurs bandes.

---

# 5. Topologie du maillage : de la caméra au triangle

## 5.1 Quelle surface mailler ?

En haute mer, un quad maillé régulièrement en plan `XY` est efficace si l'on évalue `P(x,y,t)` de manière cohérente. Une grille uniforme jusqu'à l'horizon coûte inutilement cher. On réutilise donc des patches, en choisissant leur densité **pour l'image**, indépendamment des grilles de simulation.

**Ne pas confondre :** `N×N` échantillons FFT, `N×N` cellules de solveur littoral, `N×N` pixels de normal map, et `N×N` sommets du patch. Ces résolutions ont des contraintes et peuvent être différentes.

## 5.2 Trois stratégies géométriques

| Stratégie | Mécanisme | Atout | Risque / usage |
|---|---|---|---|
| Grilles annulaires / geometry clipmaps | anneaux concentriques, cellule plus grande au loin | coût borné, densité près caméra | coutures, morphing, origine flottante [S3] |
| Projected grid | échantillonnage basé sur projection de la caméra | distribution des sommets adaptée à l'écran | cas rasants, frustum, horizon et bords [S4] |
| Patches / quadtree adaptatif | subdivisions selon erreur visuelle et zone d'intérêt | densité ciblée plage, objets | gestion des voisinages, raccords, hystérésis |

**Proposition pour ce projet :** une grille océanique caméra-centrée / clipmap pour la haute mer, plus un maillage côtier contraint par la bathymétrie et l'état mouillé, et des maillages indépendants pour les volumes 3D. Un projected grid peut être une variante, pas un prérequis. Le clipmap constitue une analogie géométrique issue du rendu de terrain ; ses raccords doivent être adaptés à une surface dynamique [S3].

## 5.3 Résolution de la géométrie et erreur projetée

La condition de Nyquist `λ > 2Δ` est seulement une **condition minimale d'échantillonnage**. Deux échantillons par période ne donnent pas une belle silhouette ; viser à titre de **point de départ** plusieurs segments par vague géométrique (par exemple `λ/Δ ≥ 4–8`) et affiner selon l'erreur projetée, sans prétendre que ce ratio garantit la fidélité. Un pic non linéaire nécessite davantage de sommets qu'une sinusoïde de même longueur d'onde.

Pour une arête de taille monde `Δ`, estimer l'écart entre surface interpolée et vraie fonction au milieu de l'arête, puis projeter l'erreur en pixels. Raffiner tant que `e_pixels > e_target`, avec hysteresis pour éviter le popping. Ajouter un test indépendant pour les zones où `J` est faible, la silhouette est critique ou la profondeur change rapidement.

À l'horizon, il faut **filtrer les petites longueurs d'onde avant échantillonnage**. Supprimer brutalement l'énergie crée un changement d'état de mer ; transférer sa contribution visuelle vers une représentation de pente/BRDF correctement filtrée, si le modèle le permet. Ce transfert est une approximation graphique, non une conservation automatique de l'énergie physique.

## 5.4 Le problème des coutures : un exemple concret

```text
Vue du dessus, raccord « ratio 2 » :

grille fine :   o---o---o---o---o
                 \ / \ / \ / \ /
grille large :  O-------O-------O

                 ^
        point fin sans voisin grossier
        = risque de T-junction/fissure
```

Approches valides, à choisir :

1. **Topologie de transition** : générer les triangles spécifiques suivant l'écart de résolution ; préférer ratio 2 entre voisins immédiats.
2. **Edge stitching** : adapter les indices et faire correspondre les sommets de bord des deux niveaux.
3. **Morphing géométrique** : amener progressivement les sommets fins à la surface grossière ; à utiliser avec une zone de transition et une position monde de référence commune.
4. **Skirts** : jupe cachant de petites fissures ; solution de secours **visuelle**, potentiellement visible en transparence, en silhouette et au contact de plage.

**Invariants de raccord visuel :** évaluer la même fonction globale `P(world_xy,t)` sur les sommets communs, même temps et même bathymétrie ; ne jamais remettre aléatoirement les phases au changement de patch ; éviter que des normales différentes révèlent le raccord même si la position est continue. Contrôler aussi la **tangence/continuité des pentes** lorsque c'est visible. Deux maillages qui se touchent en position `C0` peuvent présenter une ligne de reflet s'ils n'ont pas les mêmes dérivées (`C1`).

## 5.5 Quadtree / clipmap et origine mobile

```text
                +------------------------------+
                |             L3               |
                |  +------------------------+  |
                |  |          L2            |  |
                |  |   +----------------+   |  |
                |  |   |       L1       |   |  |
                |  |   |    +------+    |   |  |
                |  |   |    | L0   |    |   |  |
                |  |   |    |caméra|    |   |  |
                |  |   |    +------+    |   |  |
                |  |   +----------------+   |  |
                |  +------------------------+  |
                +------------------------------+
```

Ancrer les grilles sur des coordonnées monde *quantifiées* (multiples de cellule) pour ne pas les faire glisser continuellement ; interpoler/morpher la géométrie affichée si nécessaire. Les textures et phases de vagues doivent, elles, être évaluées dans un référentiel monde stable. Prévoir un anneau de sécurité pour que les grandes vagues ne quittent pas soudainement le domaine visible lorsque la caméra bouge.

## 5.6 LOD visuel ≠ LOD simulation

Un anneau plus éloigné peut utiliser de grands triangles **tout en lisant le même état physique**. Inversement, une zone simulée à haute résolution derrière la caméra peut ne nécessiter aucun rendu. Toute transition de résolution **physique** est un problème distinct de restriction/prolongation des états et des flux ; ne jamais utiliser le morphing des sommets comme substitut au couplage des solveurs.

---

# 6. De la haute mer à la côte : les processus physiques et la topologie

## 6.1 La bathymétrie définit une transition progressive

```text
Coupe schématique — distance vers la plage  --->

au large                     zone de shoaling        plage
~~~~~~~~~~~                   ~~~~~~/\__        __/\___
                           __/          \______/     \___
__________________________/                            \___ terre
    d >> λ/2               d ~ λ                 d << λ
```

**Schéma conceptuel, non à l'échelle.** La position du déferlement dépend de la houle, du fond et des courants. Le fond sous-marin doit être échantillonné à une précision suffisante pour les variations physiques utiles, pas obligatoirement à la densité du sable visuel.

En se rapprochant du rivage :

1. La profondeur diminue ; la relation de dispersion change.
2. La célérité et la longueur d'onde évoluent ; le front se réoriente (**réfraction**).
3. L'amplitude peut augmenter par concentration d'énergie (**shoaling**) ; elle peut aussi diminuer selon friction, dissipation, réfraction, courants, diffraction, etc.
4. La non-linéarité et la contrainte de profondeur finissent par provoquer du **déferlement**.
5. Après déferlement, les turbulences et le transport de masse dominent localement ; le front de mouillage monte sur le sable puis se retire.

Les modèles côtiers de type SWAN intègrent propagation, réfraction, shoaling et diverses sources/pertes d'énergie dans une équation de bilan d'action [S5]. Le modèle TMA ajuste une forme spectrale pour une profondeur finie mais **ne résout pas à lui seul** les changements spatiaux [S6].

## 6.2 Préconisation de zones (physiques, non distances arbitraires)

| Zone | Critère d'entrée | État physique | Géométrie |
|---|---|---|---|
| Océan profond | `kh` grand pour les composantes dominantes | spectre profond + interactions paramétrées | grille caméra-centrée |
| Eau intermédiaire | `kh` devient significatif | dispersion complète, variation de `k`, réfraction, shoaling | surface heightfield avec évaluation locale |
| Avant-déferlement | cambrure / profondeur / vitesse critique approchées | ondes non linéaires, limite de validité estimée | surface plus dense si nécessaire |
| Zone de déferlement | événement de rupture détecté | dissipation + rouleau/spray, éventuellement 3D | peau spéciale ou volume 3D |
| Swash / runup | front mouillé mobile | bilan de masse et quantité de mouvement, wet/dry | masque et contour dynamique |

Le seuil est **par vague/bande et par endroit** : un même point peut être « profond » pour une ride courte et « intermédiaire » pour une longue houle. Éviter un seul booléen global « près plage ». Conserver une hystérésis pour l'activation des modules afin d'éviter les basculements répétés.

## 6.3 Transport de l'énergie et préservation des phases

Le modèle spectral côtier travaille souvent avec une densité d'action `N=E/sigma`, où `E` est l'énergie spectrale et `sigma` la fréquence intrinsèque. Forme conceptuelle :

```math
∂t N + ∇_x · (c_g N) + ∂theta(c_theta N) + ∂sigma(c_sigma N) = S_total / sigma
```

En présence d'un courant, l'advection spatiale inclut son effet suivant la convention employée. Les termes sources/pertes décrivent vent, déferlement, friction, interactions non linéaires, etc. [S5]. **Attention : le bilan spectral d'action seul ne conserve pas les phases cohérentes** nécessaires à des interférences visuellement exactes ou à un raccord surface-à-surface instantané. Deux options :

- **Mode quasi-cohérent pratique :** advecter des modes/phases en plus de l'enveloppe spectrale, reconstruire `eta` et les vitesses au bord du domaine côtier ; valider les interférences.
- **Mode simulation locale :** utiliser le champ large comme condition d'entrée d'un solveur dispersif/3D, et laisser le solveur local créer la réponse. La génération et la transmission aux limites doivent être cohérentes en hauteur **et** en vitesse.

Un simple fondu entre deux vagues qui n'ont pas la même phase produit des creux et des pics fictifs. Ne pas appeler cela conservation de l'énergie.

## 6.4 Choisir le bon modèle près du rivage

- **Spectre + rayons / action :** pertinent pour les variations d'état de mer et l'orientation à grande échelle ; ne produit pas automatiquement chaque rouleau.
- **Boussinesq / dispersif :** décrit des vagues de surface avec dispersion et effets de profondeur sur un domaine 2D ; plus adapté aux trains d'ondes cohérents jusqu'à son domaine de validité. Celeris en est un exemple de solveur GPU avec trait de côte mobile [S7].
- **Shallow-water equations (SWE)** : bien adaptées aux variations de profondeur, à l'inondation, au runup et aux écoulements dominants quand l'hypothèse hydrostatique est acceptable ; **les équations classiques ne reproduisent pas correctement la dispersion des vagues de profondeur intermédiaire**. Une méthode hybride bulk-flow + vagues dispersives est étudiée par Jeschke & Wojtan [S8].
- **Simulation 3D locale :** nécessaire si l'on veut résoudre géométriquement le retournement de la crête, les poches d'air et la rupture en gouttes ; réservée aux contacts et scènes proches pour des raisons de coût.

**Architecture recommandée :** ne pas remplacer l'océan analytique par des cubes de simulation partout où la profondeur change. Utiliser un modèle côtier 2D/2,5D et réserver le 3D aux zones où la surface libre n'est plus une hauteur unique ou où le critère visuel l'exige.

---

# 7. Bord de plage : du déferlement au retrait de l'eau

## 7.1 Le déferlement n'est pas une simple texture de mousse

Les rouleaux peuvent être **spilling** (rupture progressive), **plunging** (lèvre qui se retourne) ou **surging** (montée vive sur pente) ; les transitions sont continues. La pente du fond, la cambrure incidente et le vent influencent le type de déferlement. Le rapport `H_b/d_b ≈ 0,78` est un **repère classique issu du cas de vague solitaire sur fond horizontal**, pas une loi universelle ni le seul déclencheur à coder [S9].

Pour un déclencheur de production, combiner :

```text
break_score = f( H/d, ka, kh, pente_du_fond,
                 asymétrie_de_crête, vitesse_orbitale/c_phase,
                 compression_J, interactions_locales )
```

La fonction `f` doit être calibrée. Pour un rendu non résolu physiquement, l'intensité de mousse et la dissipation peuvent être paramétrées ; pour un solveur, appliquer les pertes et les transferts de quantité de mouvement selon son schéma numérique. **Ne pas utiliser `J < 0` pour faire passer silencieusement des triangles sous l'eau : c'est un indicateur d'invalidité du paramétrage.**

## 7.2 Quelle topologie pour la lèvre qui se retourne ?

Un heightfield `z=eta(x,y)` ne peut pas avoir deux hauteurs pour une même paire `(x,y)`. Trois niveaux visuels possibles :

1. **Déferlement lointain :** surface heightfield + atténuation de l'onde + mousse surfacique ; silhouette approximée.
2. **Déferlement proche mais non interactif :** géométrie additionnelle de rouleau alignée avec la crête, particules et spray ; attention au raccord avec la surface et aux collisions trompeuses.
3. **Déferlement interactif 3D :** surface libre volumique (par exemple liquide sur grille + particules et reconstruction de surface) ; représentation de la lèvre, des éclaboussures, du mélange air-eau à une fidélité qui dépend du solveur.

Ne pas promettre qu'un unique quad « subdivisé » se transforme automatiquement en rouleau correctement conservatif : le problème est d'abord celui de la **représentation de la surface et de la dynamique**, pas de la densité de triangles.

## 7.3 Wetting / drying : vraie topologie du rivage mobile

Pour le bord de plage, la frontière entre eau et terre se déplace. On distingue :

- **Masque statique** issu de `b(x,y)` et du niveau moyen : utile au pré-calcul, insuffisant pour le runup.
- **État mouillé dynamique** issu de `h=eta-b` : sert au solveur et à la reconstruction visuelle du contour.
- **Carte de sable humide** avec mémoire temporelle : effet optique indépendant de l'épaisseur de la lame d'eau.

Exemple de cycle d'une cellule littorale :

```text
DRY --(flux entrant + h > h_on)--> WET
WET --(flux sortant + h < h_off)--> DRY
```

`h_on` et `h_off` sont des seuils numériques choisis selon l'échelle, idéalement avec hystérésis ; ne pas supprimer brutalement la masse d'une cellule à cause du seuil. Le traitement wet/dry doit préserver `h >= 0`, une solution d'eau au repos équilibrée (*well-balanced*) et, dans les conditions applicables, le bilan de masse [S10].

Une découpe visuelle de contour peut être construite à partir des croisements de `eta-b` sur les arêtes (marching squares/triangles ou clipping de polygones). Les cas ambigus des cellules diagonales exigent une règle déterministe, et la triangulation ne doit pas créer de triangles à aire nulle. **Ne pas faire porter au masque de rendu la responsabilité de décider des volumes transférés** : le solveur calcule les flux, puis la géométrie reflète cet état.

### Schéma 2 — séparation entre calcul et découpe du bord

```text
            cellule proche plage
       sommet A (mouillé)   B (mouillé)
                 o----------o
                 | \ eau    |      bord extrait par
                 |   \      |      interpolation de h
                 |     \    |
                 o------\---o
       C (sec)          D (sec)

Physique : volume/flux aux faces de cellules.
Rendu   : polygone humide/découpe du triangle.
```

## 7.4 Runup / swash : un bilan, pas seulement un decal

Le front de plage doit avancer puis reculer en réponse au champ local. Pour un modèle SWE 2D simplifié, avec `h` profondeur et `U=(u,v)` vitesse moyenne sur la profondeur :

```math
∂t h + ∇·(h U) = q_sources
∂t(h U) + ∇·[h U⊗U + (g h²/2) I] = -g h ∇b + forces_et_pertes
```

Vérifier signes et convention de `b` lors de l'implémentation (ici `b` est l'altitude du fond). Les sources et pertes incluent, selon le modèle, friction, injection ou couplage ; l'eau au repos sur fond non plat doit rester au repos numériquement. Un schéma conservatif de volumes finis avec flux partagés, reconstruction hydrostatique/équilibrée et limiteur de positivité est une piste robuste [S10].

**Éviter :** forcer la surface à l'altitude de la plage, couper les triangles dès que `h` baisse, et laisser visuellement une feuille d'eau sans volume. Le sable humide et le film d'eau ultra-mince peuvent avoir une approximation purement graphique quand leur masse et leurs interactions ne sont plus pertinentes ; la transition doit être déclarée.

## 7.5 Mousse, bulle, écume, sable humide : quatre couches différentes

| Couche | Naissance | Vie / advection | Fin |
|---|---|---|---|
| Mousse de crête | déferlement, dissipation, compression | transportée avec écoulement de surface | décroissance et étirement |
| Eau aérée / rouleau | rupture réelle ou approximée | surface/volume local, bruit contrôlé | retour dans le liquide, perte d'air |
| Spray / gouttes | émission au rouleau/impact | trajectoires balistiques + vent | collision / disparition |
| Sable humide | passage antérieur de l'eau | texture mémoire en coordonnées terrain | séchage lent / réhumidification |

Une simple bande de mousse collée au contour `h=0` ressemble à un liseré blanc artificiel. La crête en déferlement peut apparaître **avant** le rivage ; la mousse peut continuer à avancer dans le swash, puis rester en fragments pendant le retrait.

---

# 8. Rochers, bateaux et impacts : quand la surface change réellement de topologie

## 8.1 Obstacles fixes

Un rocher émergé : `b(x,y)` ou obstacle solide doit définir les cellules humides/solides et les conditions de flux ; s'il possède surplombs ou cavités, une simple bathymétrie 2D perd l'information géométrique. Une berge verticale ou un quai peut être représenté par un obstacle rigide avec condition de non-pénétration. Un passage étroit peut demander une représentation sub-cellulaire, un maillage littoral plus fin ou un domaine local 3D.

À ne pas faire : cacher l'océan dans le rocher avec un simple depth test et considérer l'interaction physique résolue. Ajouter au besoin vagues réfléchies, diffraction approximée, sillage, écume d'impact et ruissellement sur les surfaces.

## 8.2 Objets mobiles

L'objet flotte/interagit avec `eta` et `u` du champ cohérent ; il peut ensuite produire un sillage, une dépression, des gerbes et des flux de déplacement. Un navire de grande taille demande un échange bidirectionnel si ses effets sont visibles ; une perturbation ajoutée **seulement aux normales** ne déplace pas réellement le volume d'eau ni ne modifie la flottabilité.

## 8.3 Activation locale d'une simulation 3D

Critères possibles : objet entre dans l'eau, retournement visible, choc violent, interaction entre plusieurs surfaces, compression `J`, forte vorticité, caméra proche. Mais **la visibilité seule ne définit pas la validité physique** : utiliser au moins deux familles de critères, perception et domaine de validité du modèle.

```text
        champ analytique / côtier
  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         |   zone tampon   |
         | +-------------+ |
         | |  solveur 3D | | <-- choc, rouleau, obstacle
         | | VOF / FLIP  | |
         | +-------------+ |
         +-----------------+
                 |
             fond solide
```

**Topologie du volume 3D :** les voxels/blocs sont une discrétisation du domaine, non les cellules constitutives d'une eau « cubique » visible. La surface est extraite de la fraction de liquide/particules (ex. iso-surface), lissée, et raccordée au reste ; la taille de voxel est dictée par le phénomène résolu et le budget, indépendamment du maillage final. Le split/merge des blocs doit transférer les états et leurs bilans, pas simplement rééchantillonner les positions des vertices de rendu.

## 8.4 Deux façons correctes d'éviter la double surface

- **Propriété exclusive** : un masque `surface_owner` confie le rendu de la peau visible au solveur local dans son domaine ; le champ large continue éventuellement d'alimenter les conditions aux limites, mais est masqué là où le solveur local possède la surface.
- **Couche résiduelle définie** : le solveur local fournit `delta_eta`/flux perturbateur par rapport à un fond large connu ; reconstruction unique `eta=eta_base+delta_eta` tant que la surface demeure un heightfield. Dès qu'elle se retourne, passer à une peau 3D possédée par le solveur local.

Ne pas moyenner arbitrairement une surface retournée avec une hauteur FFT : cela peut détruire la lèvre, former deux membranes et violer le bilan de masse.

---

# 9. Raccordement entre modèles : exigence de continuité et de conservation

## 9.1 Ce qu'il faut raccorder

| Quantité | Pourquoi | Méthode attendue |
|---|---|---|
| Hauteur `eta` | éviter marche visible / onde parasite | valeur imposée ou continuité faible au bord |
| Vitesse normale / débit | conserver masse, éviter jet fictif | flux interfacial unique et partagé |
| Quantité de mouvement | éviter impulsion non physique | transfert conservatif ou force de couplage identifiée |
| Phase / direction | continuité du train de vagues | champ cohérent commun ou conditions d'entrée physiques |
| Normales | éviter couture lumineuse | dérivées communes ou mélange contrôlé après continuité géométrique |
| Écume / trace | persistance sans popping | advection et injection contrôlées |
| Objet et collision | ne pas changer de mer selon la zone | interrogation d'un champ de surface/volume unifié |

`C0` signifie positions continues ; `C1` pentes continues. Sur une interface numérique, on vise d'abord flux, stabilité et absence d'artefact, et on évalue séparément la continuité visuelle.

## 9.2 Zone tampon et conditions aux limites

Au bord extérieur d'un solveur local : injecter des vagues entrantes via hauteur et vitesse compatibles ; laisser sortir les ondes perturbatrices avec conditions radiatives/absorbeurs (*sponge layer*). Séparer, autant que possible, **champ incident** et **champ perturbé** ; une condition de Dirichlet dure sur toute la frontière peut réfléchir artificiellement l'onde sortante.

Pour un transfert entre niveaux physiques : restriction conservatrice des grandeurs moyennes, prolongation qui ne crée pas de masse, pas de temps compatibles et somme des flux identique des deux côtés. Si le solveur 3D tourne à fréquence différente, intégrer/interpoler les **flux** sur le pas partagé ; ne pas injecter deux fois le volume au changement de régime.

## 9.3 Protocole de split / merge des zones

```text
1. Détecter besoin de raffinement (+ hystérésis).
2. Créer domaine fils avec cellules fantômes / tampon.
3. Prolonger h, momentum, pression/volume selon méthode choisie.
4. Réconcilier les flux et vérifier h >= 0 / divergence.
5. Activer le propriétaire de surface et amorcer le raccord visuel.
6. Simuler ; enregistrer bilans / états.
7. Avant fusion, restreindre les états conservatifs et les traces.
8. Libérer le fils quand l'erreur et l'activité sont acceptables.
```

Pour un domaine 3D incompressible, prolonger seulement les vitesses interpolées ne garantit pas `div(u)=0` : il peut falloir une projection/correction de pression compatible avec les frontières. Les conditions de Neumann demandent une compatibilité globale du flux net (ou traitement approprié de la frontière libre). Les décisions exactes dépendent du solveur retenu.

---

# 10. Améliorer le rendu : mesures et actions ciblées

## 10.1 Réglage de la géométrie et des pentes

**Essai R1 — matériau diagnostic.** Sortir la surface en gris mat + wireframe, enregistrer `eta`, normale géométrique, courbure, `J`, `kh`, `ka` et contribution de chaque bande. Si les grandes vagues ne sont pas lisibles *sans shader*, le problème est d'abord géométrique/spectral/caméra.

**Essai R2 — spectre directionnel.** Visualiser densité d'énergie par `k` et direction. Chercher des grosses formes lisibles mais non périodiques et des interférences plausibles. Ne pas augmenter le bruit procédural au hasard : cela peut accentuer l'aspect synthétique.

**Essai R3 — pentes analytiques.** Calculer `dP/dx`, `dP/dy` à partir de la même surface que le déplacement, puis `N = normalize(cross(dP/dx,dP/dy))` avec ordre de produit correct. Les différences finies sur une grille différente peuvent créer des reflets discontinus ; valider analytique vs numérique.

## 10.2 Shader d'eau : ordre de vérification

1. **Ciel et exposition.** Rendre sous un ciel qui ressemble à la référence : changer le ciel change réellement la couleur de la mer réfléchissante. Fixer le tone mapping et la caméra pendant la comparaison.
2. **Fresnel.** Utiliser un modèle diélectrique eau/air cohérent (IOR visible autour de 1,33 comme point de départ) ; à incidence normale la réflexion spéculaire est de l'ordre de 2 %, et augmente en incidence rasante. Le ciel réfléchi peut être clair ou foncé suivant la direction.
3. **Réflexion.** Hiérarchiser capture d'environnement / réflexion écran / ray tracing / approximation distante selon moteur ; contrôler les trous SSR, horizon, objets et discontinuités de LOD.
4. **Absorption et diffusion.** Appliquer l'atténuation par trajet optique, par exemple `T_lambda = exp(-sigma_a(lambda) * L)` dans un modèle simplifié ; la couleur transmise dépend de profondeur, turbidité et fond, pas d'une unique couleur de base.
5. **Micro-rugosité et normales.** Réduire temporairement la force des petites pentes. Filtrer la distribution de pentes en fonction du pixel ; éviter un specular aliasing ressemblant à un tapis de stries blanches.
6. **Écume.** La créer selon rupture et transport ; shading mousse plus diffus et moins transparent que l'eau claire, avec variations de taille et d'âge.
7. **Sous-surface locale.** Sur crêtes minces et eau peu profonde, intégrer une transmission/diffusion modérée si le moteur le permet ; ne pas illuminer artificiellement les creux profonds.

Les méthodes historiques de rendu couplant vagues de maillage et normal map [S2] et les techniques de déplacement GPU [S11] servent de références conceptuelles ; leurs détails doivent être adaptés à la génération de GPU et au moteur utilisés.

## 10.3 Ce que la seconde image semble demander en premier

| Rang de test | Ajustement | Ce que l'on veut observer |
|---|---|---|
| A | reproduire la caméra / ciel de la référence | comparaison contrôlée et suppression du biais de couleur |
| B | afficher seulement déplacement géométrique | grosses formes lisibles et silhouettes non répétitives |
| C | vérifier la part de basse fréquence du spectre | vallées/crêtes et interactions crédibles |
| D | remettre le shader sans normal map fine | vérifier si les stries proviennent des micro-pentes |
| E | réintroduire la bande fine filtrée | richesse sans écraser les masses d'eau |
| F | vidéo caméra mobile | absence de scintillement et de popping |

**Point essentiel :** ne pas assombrir simplement l'albédo ou ajouter des polygones comme réponse universelle. L'écart observé peut être principalement une conséquence des normales de détail et de l'environnement lumineux.

## 10.4 Antialiasing temporel et filtrage

Évaluer les petites vagues en fonction de leur taille projetée ; utiliser des mipmaps/variance de pentes, un filtrage de normales ou une approximation de rugosité effective. Le TAA ne doit pas servir à masquer une fréquence spatiale non résolue : il peut produire du ghosting sur les crêtes en mouvement. Vérifier motion vectors sur **positions déplacées** (`P(t)` vs `P(t-Δt)`), y compris lors du changement de patch et d'origine flottante.

## 10.5 Horizons et atmosphère

Éviter une ligne de raccord visible entre plan océanique fini, skybox et brume. Le point à l'horizon n'est pas une coupure physique de la surface : maintenir un domaine rendu suffisant, cohérence atmosphérique et filtrage de lointain ; contrôler le clipping des crêtes lorsque la caméra s'élève.

---

# 11. Implémentation GPU : pipeline proposé

```text
CPU / scène : météo, fond, contraintes, caméra, objets
   |
   v
GPU 0 : spectres profonds, phases, champs de base et dérivées
   |
   v
GPU 1 : bathymétrie, propagation côtière / solveur local
   |
   +--> GPU 2 : détection de rupture, flux, domaine 3D actif
   |        |
   |        +--> extraction éventuelle de surface 3D
   v
GPU 3 : construction/mise à jour patches et indices de raccord
   |
GPU 4 : évaluation positions monde et normales cohérentes
   |
GPU 5 : masque de propriétaire, ombres, réflexions, eau, écume
   |
GPU 6 : post-process / anti-aliasing / métriques debug
```

**Quelques règles de conception :**

- Définir une seule API `sample_water(world_xy, t, required_outputs)` côté jeu ; les interactions 3D peuvent nécessiter une requête volume séparée.
- Regrouper le contenu constant (bathymétrie) et dynamique (hauteur, vitesses, mousse) ; éviter les copies GPU–CPU par frame.
- Évaluer les sommets de deux patches adjacents avec le même état physique et le même timestamp.
- Ajouter un canal `debug_reason` qui explique l'activation d'une zone 3D : visibilité, rupture, collision, validité `kh/ka/J` ou combinaison.
- Répartir le coût avec un budget **mesuré** : FFT, propagation côtière, 3D, extraction, vertices, pixel shader, transparence et réflexion. Un chiffre de fps seul ne permet pas de trouver le goulot.

### Pseudocode : décision d'affichage d'un point

```python
# Schéma conceptuel : pas un code directement exécutable.
def sample_visible_surface(xy, t, camera):
    base = ocean.sample(xy, t)             # hauteur, vitesse, dérivées
    coast = coastal.query(xy, t, base)     # modifie / corrige base si actif
    local = local3d.lookup(xy, t)

    if local.owns_surface:
        # Une seule peau visible ; la sortie est une surface 3D reconstruite.
        return local.visible_surface

    field = coast.field if coast.valid else base
    if coast.wetdry_enabled and not coast.is_wet:
        return NO_WATER_SURFACE
    return field.surface
```

L'API réelle doit expliciter les zones de transition et les sorties non scalaires ; cette fonction illustre seulement la **priorité de propriété**.

### Pseudocode : raffinement d'un patch

```python
# Schéma conceptuel ; comparer à des tolérances de production mesurées.
def should_refine(patch, camera, physics):
    err_px = projected_midpoint_error(patch, camera, physics.surface)
    steep = physics.max_resolved_curvature(patch)
    shore = physics.has_dynamic_wet_front(patch)
    return (err_px > target_px or
            steep > curvature_limit or
            shore) and patch.can_subdivide
```

Ne pas fonder tout le LOD sur la distance seule : une crête au premier plan et un front de plage nécessitent des critères différents.

---

# 12. Validation : scènes, mesures et critères de sortie

## 12.1 Tests géométriques minimaux

| Scène | Ce qu'on vérifie | Échec révélateur |
|---|---|---|
| Plan plat, zéro vague | orientation triangles/normales, absence de fissure | scintillement ou surface inversée |
| Onde analytique unique | amplitude, longueur, vitesse de phase | phase dérivante et erreur d'échelle |
| Vagues croisées | dispersion directionnelle, pentes | motif qui se répète trop vite |
| Caméra qui traverse les anneaux | couture et origine stable | popping, T-junction, normales cassées |
| Caméra rasante à l'horizon | clip/filtrage | trous et bruit spéculaire |
| Bathymétrie en pente douce | évolution de k, c, H | vague identique jusqu'au sable |
| Banc de sable oblique | réfraction, concentration d'énergie | front qui traverse le fond sans déviation |
| Plage sèche puis vague | conservation, `h >= 0`, contour wet/dry | eau fantôme, triangles secs, masse perdue |
| Mur/rocher | non-pénétration, réflexion/diffraction | eau qui traverse le solide |
| Objet qui entre dans l'eau | activation 3D, transition | double peau et impulsion fictive |
| Split/merge de blocs | bilan global et continuité | volume créé/perdu |
| Retour zone 3D vers champ large | perturbations sortantes, absence de couture | suppression instantanée du sillage |

## 12.2 Mesures à enregistrer

- `E_eta_rms`, erreur de phase et vitesse de propagation contre une onde analytique.
- Spectre 1D et 2D calculé du rendu géométrique ; spectre des pentes (important pour les reflets).
- Carte `(ka,kh)` donnant le domaine de validité observé de chaque modèle.
- `V(t)=∫ h dA` sur les domaines 2D, quantité de liquide 3D et flux aux frontières ; erreurs cumulées relatives.
- `min(h)`, `min(J)`, cellules sèches/mouillées, triangles dégénérés, amplitude au bord des patches.
- Déviation de normale et saut de position aux coutures ; nombre d'images avec popping.
- Coûts GPU par passe et percentiles, allocation mémoire, nombre de blocs 3D, surface de pixels transparents.

### Carte de validité proposée

```text
      ka = cambrure
       ^
  fort |   modèle global à valider/raffiner
       |     +------------------------------
       |    /
       |   /  domaine de transition mesuré
       |  /
 faible+-------------------------------> kh
      peu profond                   profond
```

**Ce dessin n'encode aucun seuil universel.** Construire la frontière avec des tests de phase, hauteur, vitesse et rendu, y compris la pente du fond, les courants et les impacts. La profondeur relative `kh` est aussi importante que `ka` : un champ qui reste convaincant au large peut échouer bien avant une valeur de cambrure élevée quand la profondeur diminue.

## 12.3 Test A/B incontournable pour les images

Rendre quatre sorties au même instant : (i) position/hauteur brute, (ii) normales géométriques sans détails, (iii) matériau d'eau sans normales fines, (iv) rendu complet. Conserver caméra, météo, exposition et tone mapping identiques. Si le rendu (iii) ressemble déjà au défaut de la seconde image, investiguer environnement et matériau ; si (i) manque de grandes masses, investiguer champ et maillage ; si seul (iv) dérive, investiguer filtrage des détails.

---

# 13. Décisions recommandées, limites et plan de réalisation

## 13.1 Architecture cible en une phrase

**Un champ de vagues large et cohérent fournit hauteur/pentes/vitesses ; la profondeur transforme sa propagation ; un solveur côtier gère la proximité de la plage et le mouillage ; une simulation 3D prend temporairement la propriété des zones de rupture/contact qui ne sont plus des heightfields ; le rendu recouvre ces états physiques avec une géométrie caméra-adaptée et un shader spectralement filtré.**

## 13.2 Décisions à acter explicitement avant production

1. Modèle haute mer (FFT, somme de vagues ou hybride) et calibration des spectres.
2. Stratégie de phase cohérente pour alimenter les domaines côtiers.
3. Solveur côtier retenu (SWE seul, dispersif, modèle hybride) et ses limites.
4. Règle de rupture (`ka`, `kh`, pente, vitesses, score calibré) et représentation des rouleaux proches.
5. Représentation 3D, méthode de reconstruction de surface, pas de temps et conditions aux limites.
6. Règles de propriété unique / perturbation résiduelle aux interfaces.
7. Maillage de rendu : clipmap, projected grid ou patches ; procédé exact de couture.
8. Échelle de précision et cas de référence : mer calme, houle, plage douce, plage raide, rochers, bateau, impact.

## 13.3 Plan de travail incrémental

**Phase A — diagnostic de l'océan actuel.** Captures brutes, spectre mesuré, carte de pentes et d'erreur de maillage ; refaire l'image B sans shader. Livrable : cause dominante de l'écart documentée, pas une simple préférence artistique.

**Phase B — haute mer stable.** Surface globale cohérente, phases, dérivées, maillage LOD sans fissures, filtrage du lointain et collision utilisable. Livrable : tests d'onde et de caméra, coût GPU par passe.

**Phase C — première côte.** Bathymétrie, dispersion complète, shoaling/réfraction, plage wet/dry conservatrice ; rouleaux lointains paramétrés. Livrable : une plage à pente contrôlée, front mouillé mobile, métriques de masse.

**Phase D — interactions locales.** Rochers, obstacles et entrée d'objet dans l'eau ; conditions de bord et domaine 3D limité. Livrable : test de double surface, flux et sortie du domaine.

**Phase E — finition du rendu.** Réflexions, ciel, filtration spectrale des normales, mousse advectée, spray, sable humide, optimisation de la transparence. Livrable : comparaison A/B stable en mouvement et à plusieurs hauteurs de caméra.

## 13.4 Ce que cette architecture ne garantit pas

- Les modèles spectraux ne résolvent pas tous les mouvements d'un liquide turbulent.
- Un modèle SWE classique ne restitue pas toute la dispersion des ondes océaniques.
- Un champ de hauteur, même très subdivisé, ne peut pas représenter un rouleau retourné ni des gouttes détachées.
- Une simulation 3D locale ne devient pas conservative par simple mélange alpha avec une mer FFT.
- Une normal map n'ajoute ni flottabilité ni silhouette.
- Une photographie unique ne fournit pas les conditions de vent, le spectre, la profondeur, la focale ou les paramètres de matériau : toute correspondance reste qualitative tant qu'ils ne sont pas mesurés.

---

# Glossaire opérationnel

- **Bathymétrie :** altitude/profondeur du fond sous l'eau ; elle gouverne la propagation locale.
- **Heightfield :** surface à une seule hauteur pour chaque point horizontal.
- **Spectre directionnel :** répartition de l'énergie des vagues par fréquence/nombre d'onde et direction.
- **Phase :** position d'une oscillation dans son cycle ; essentielle au raccord cohérent.
- **Dispersion :** dépendance de la vitesse à la longueur d'onde et à la profondeur.
- **Shoaling :** transformation de l'amplitude liée notamment aux variations de profondeur et de flux d'énergie.
- **Réfraction :** déviation des fronts d'onde par variation spatiale de célérité.
- **Déferlement :** rupture de la vague avec dissipation et génération possible de rouleau, mousse et gouttes.
- **Runup / swash :** montée puis écoulement en retour de l'eau sur la plage.
- **Wet/dry :** gestion numérique et géométrique des cellules alternativement mouillées/sèches.
- **LOD :** niveau de détail ; distinguer géométrie, simulation et shader.
- **`C0` / `C1` :** continuité des positions / des premières dérivées au raccord.
- **`J` :** déterminant de la projection horizontale d'une surface paramétrique ; proche de zéro, repli menaçant.
- **Sponge layer :** zone d'amortissement pour limiter les réflexions parasites aux frontières.
- **VOF / FLIP :** familles de représentations/solveurs de fluide 3D et de reconstruction de surface ; choix d'implémentation à faire séparément.

---

# Sources et lectures techniques

Les références suivantes sont des **sources méthodologiques**, pas des déclarations selon lesquelles tous leurs algorithmes seraient déjà intégrés au projet. Consultation : septembre 2026.

**[S1] Jerry Tessendorf, *Simulating Ocean Water* (2001).** Référence fondatrice pour les synthèses spectrales/FFT, vagues et « choppiness ».  
https://www.researchgate.net/publication/264839743_Simulating_Ocean_Water

**[S2] Mark Finch, *Effective Water Simulation from Physical Models*, GPU Gems, chapitre 1 (2004).** Séparation géométrie/normal map, vagues sinusoïdales et Gerstner, calcul des dérivées, limites du repli.  
https://developer.nvidia.com/gpugems/gpugems/part-i-natural-effects/chapter-1-effective-water-simulation-physical-models

**[S3] Frank Losasso et Hugues Hoppe, *Geometry Clipmaps: Terrain Rendering Using Nested Regular Grids*, SIGGRAPH (2004), et version GPU (2005).** Structure LOD régulière, continuité et mise à jour incrémentale. Application à l'océan : adaptation d'ingénierie.  
https://hhoppe.com/proj/geomclipmap/  
https://hhoppe.com/proj/gpugcm/

**[S4] Claes Johanson, *Real-time Water Rendering: Introducing the Projected Grid Concept* (2004).** Distribution d'un maillage d'eau selon la projection caméra.  
https://citeseerx.ist.psu.edu/document?doi=92cf2a60ad229459f7b86d86f4702b052f7b46e6&repid=rep1&type=pdf

**[S5] SWAN, *Scientific and Technical Documentation*, Delft University of Technology.** Bilan d'action, propagation de la mer profonde à la zone de surf, shoaling, réfraction, dissipation, courants et obstacles.  
https://swanmodel.sourceforge.io/online_doc/swantech/swantech.html  
https://swanmodel.sourceforge.io/features/features.htm

**[S6] E. Bouws et al., *Similarity of the Wind Wave Spectrum in Finite Depth Water: 1. Spectral Form*, JGR (1985).** Construction et limites d'usage du spectre TMA en profondeur finie.  
https://doi.org/10.1029/JC090iC01p00975

**[S7] *Celeris: A GPU-accelerated open source software with a Boussinesq-type wave solver for real-time interactive simulation and visualization*, Computer Physics Communications (2017).** Exemple de solveur dispersif littoral GPU avec trait de côte mobile.  
https://doi.org/10.1016/j.cpc.2017.03.002

**[S8] Stefan Jeschke et Chris Wojtan, *Generalizing Shallow Water Simulations with Dispersive Surface Waves*, SIGGRAPH (2023).** Découplage écoulement de fond / ondes de surface dispersives, y compris interactions plage–sillage.  
https://research.nvidia.com/labs/prl/shallow-water-simulation/

**[S9] U.S. Army Corps of Engineers, *Coastal Engineering Manual*, Part II, chapitre 4, breaker criteria.** Indices de déferlement, portée du repère `H_b/d_b ≈ 0,78` et influence de la pente et de la cambrure.  
https://coastalengineeringmanual.tpub.com/Part-II-Chap4/Part-II-Chap40005.htm

**[S10] Travaux sur les schémas shallow-water avec wetting/drying, positivité et équilibre.** Motivation des contraintes de conservation, d'état sec/mouillé et de stabilité.  
https://www.sciencedirect.com/science/article/pii/S0309170810001491  
https://adcirc.github.io/adcirc/user_guide/model_configuration/model_parameters/nolifa.html

**[S11] NVIDIA, *Using Vertex Texture Displacement for Realistic Water Rendering*, GPU Gems 2, chapitre 18 (2005).** Déplacement GPU et approche historique de génération de mousse.  
https://developer.nvidia.com/gpugems/gpugems2/part-ii-shading-lighting-and-shadows/chapter-18-using-vertex-texture-displacement

**[S12] SWAN, *Features* et documentation de la propagation côtière.** Exemples de grilles structurées/non structurées et phénomènes côtiers ; utile pour discuter la topologie *du solveur*, distincte du mesh de rendu.  
https://swanmodel.sourceforge.io/features/features.htm

---

**Conclusion utilisable pour l'équipe :** une surface crédible ne naît pas d'un maillage uniformément très dense. Elle exige une définition cohérente du champ de vagues, une profondeur qui transforme leur propagation, un traitement spécifique de la frontière humide et du déferlement, des interfaces conservatrices avec la 3D locale, un rendu à propriété unique, et des détails de normales/réflexions filtrés par la résolution de l'image. Diagnostiquer séparément chacun de ces éléments avant toute optimisation ou refonte.
