# ADR-178 — Stratégie en trois systèmes physiques : construire et valider A, B, C, puis coupler, puis optimiser

- **Statut : actée**, S308, 2026-09-20, **décision de l'utilisateur**, reçue en cours de session
  et non sollicitée par une question du dépôt.
- **Réorganise les chantiers** de [FEUILLE-DE-ROUTE](../FEUILLE-DE-ROUTE.md) §2 et §3 bis. Les
  jalons et les portes restent vrais ; ce sont leurs **priorités relatives** qui changent.
- **Laisse entiers** [ADR-127](ADR-127-ambition-complete-construction-progressive.md) (ambition
  complète — l'utilisateur le redit explicitement), [ADR-001](ADR-001-decomposition-en-couches.md)
  (B + W + δ + V), [ADR-174](ADR-174-arbitrages-du-2026-09-19.md) (machine de référence, v1 =
  porte D) et [ADR-175](ADR-175-architecture-d-execution-de-delta-en-3d.md) (architecture
  d'exécution de δ en 3D, référence CPU / production GPU).
- **Suspend l'application** du profil de travail d'ADR-174 D3 comme **critère d'arrêt** pendant la
  phase de construction physique — sans le retirer, et sans arrêter la mesure. Voir D4.
- **Confrontation à l'état réel** : [TROIS-SYSTEMES-S308](../registres/TROIS-SYSTEMES-S308.md).

## 1. Ce que l'utilisateur a écrit

> « Je souhaite réorganiser le développement de notre moteur de fluides afin d'accélérer sa
> progression **sans réduire son ambition finale**. […] Nous consacrons beaucoup de sessions à
> perfectionner des composants isolés, notamment le rendu de l'océan, alors qu'une grande partie
> du moteur reste à construire. »

> « Je souhaite donc adopter une stratégie fondée sur **trois grands systèmes physiques**, leur
> validation indépendante, leur couplage, puis leur optimisation. »

> « **A — Haute mer superficielle** […] Nous disposons déjà d'une base avancée. L'objectif est de
> la stabiliser suffisamment pour permettre le couplage, **sans poursuivre indéfiniment son
> perfectionnement visuel**. »

> « **B — Simulation volumique physique 3D** […] Nous commencerons avec des domaines simples et
> fixes avant d'introduire les subdivisions adaptatives, les fusions, les séparations et les
> référentiels mobiles. **La priorité initiale est la validité physique, pas le temps réel.** »

> « **C — Transition et couplage** […] Des échanges cohérents de masse, de quantité de mouvement
> et d'énergie. Des frontières sans réflexion artificielle excessive ni rupture visible. Une
> continuité entre la surface affichée et la surface utilisée par la physique du jeu. »

> « **Ce découpage en trois systèmes ne remplace pas notre architecture B + W + δ + V.** Il
> organise les chantiers de développement. La couche V et les autres fonctionnalités du projet
> restent dans notre périmètre final. »

> « Pendant cette première phase, **je ne souhaite pas imposer immédiatement le budget final de
> 2 ms** pour l'eau. […] Il faut néanmoins continuer à mesurer les temps de calcul, la mémoire et
> l'évolution de la complexité. Nous ne devons pas construire une architecture intrinsèquement
> impossible à adapter au temps réel. »

> « **Une validation visuelle ne remplace pas une validation numérique ou physique, et
> inversement.** »

> « Je souhaite conserver notre rendu actuel comme outil de diagnostic et de comparaison. **La
> troisième image de R14 constitue notre référence interne provisoire pour l'océan.** Nous n'avons
> pas besoin de poursuivre immédiatement son perfectionnement photoréaliste. […] Il ne faut pas
> construire une multitude d'effets visuels indépendants si leurs informations peuvent être
> dérivées des simulations. »

## 2. Décisions

**D1 — Trois systèmes de chantier, au-dessus des quatre couches.** A = B + W (surface, sans
simulation volumique). B = δ (volumique 3D). C = la transition et le couplage entre les deux.
**Ce découpage est un découpage de travaux, pas d'architecture** : B, W, δ et V restent les
couches d'ADR-001, et V reste au périmètre avec ses déclencheurs (porte E).

**D2 — A est stabilisé, pas poursuivi.** Le système A est déclaré **suffisant pour servir B et
C**. Sa sortie physique est `WaterSample` (SPEC-004 §2) : élévation, vitesse totale, normale,
`deta_dt`, cambrure, aération ; plus le chemin de pression. Aucun lot de perfectionnement visuel
ne s'ouvre sur A tant que les systèmes physiques correspondants ne le demandent pas. **Le rendu
actuel devient un instrument de diagnostic**, et la troisième image de R14 sa référence interne
provisoire.

*Cette décision est en accord avec ce que la mesure dit.* S308 P7 a montré qu'aucun réglage de la
chaîne tonale ne rapproche davantage le rendu de la photographie de référence : les huit meilleurs
réglages donnent tous le même contraste local, 0,311–0,318 pour 0,455 mesurés, et ce qui manque
est **spatial** — la structure de la mer à quelques pixels d'échelle — donc hors de portée de
l'optique. L'arrêt n'est pas un renoncement mesuré à contre-cœur : c'est le point où le lot
optique cesse de rendre.

**D3 — La validité physique passe avant le temps réel, et elle se mesure.** Les essais sont
progressifs, reproductibles et ciblés ; leurs résultats se confrontent aux solutions analytiques,
aux références expérimentales et aux critères du projet. **Une validation visuelle ne remplace pas
une validation numérique, et inversement** — cette phrase de l'utilisateur devient une règle du
dépôt, et elle s'applique dans les deux sens : R11 a jugé le raccord d'un domaine « invisible »
sans qu'aucun bilan de masse n'ait jamais été mesuré à cette frontière.

**D4 — Le budget est mesuré, pas opposé.** Le profil de travail d'ADR-174 D3 (eau ≤ 4 ms GPU,
≤ 2 ms CPU, dont δ ≤ 2 ms GPU) **n'est pas un critère d'arrêt** pendant les lots de construction
physique. Il reste **mesuré et publié** à chaque lot selon
[ADR-131](ADR-131-un-depassement-qualifie-une-implementation.md) — temps, mémoire et **évolution
de la complexité** —, pour qu'aucune architecture intrinsèquement inadaptable au temps réel ne
s'installe sans qu'on le sache. Un solveur de référence peut être lent. ADR-174 D3 redevient
opposable à la porte C.

**D5 — Deux solveurs, et le premier juge le second.** La distinction « solveur de référence » /
« solveur de production » d'ADR-175 est étendue au-dessus de δ : les approximations d'une version
de production s'évaluent **contre la référence**, jamais contre elles-mêmes. C'est déjà la
discipline reçue en S297–S305 ; elle devient la règle générale.

**D6 — Les interfaces se définissent tôt, se valident progressivement.** Les six interfaces
manquantes sont nommées dans [TROIS-SYSTEMES-S308](../registres/TROIS-SYSTEMES-S308.md) §3 :
retour δ → W, compteurs de conservation, solide ↔ fluide 3D, seconde représentation de surface
libre, requête de jeu à travers δ, conservation du volume en domaine fermé. **Leur existence est
déclarée maintenant ; leur réception suit les solveurs.**

**D7 — L'ordre des lots** est celui de [TROIS-SYSTEMES-S308](../registres/TROIS-SYSTEMES-S308.md)
§5 : compteurs, retour vers W, faces coupées 3D, corps rigides, seconde représentation, requête de
jeu, puis adaptation et budget. Les lots 3 et 4 franchissent la **porte D**, donc la v1 d'ADR-174
D4 : l'ordre nouveau et l'ancien convergent là, ce n'est pas une coïncidence mais la raison de les
placer avant le lot le plus lourd.

## 3. Ce que cette décision ne tranche pas

- **La seconde représentation de surface libre** — particules ou surface implicite. ADR-175 D5
  l'avait laissée ouverte ; elle le reste. Elle engage un solveur entier : elle se proposera avec
  des éléments chiffrés et se tranchera avec l'utilisateur.
- **La tolérance des bilans de conservation.** Le dépôt n'en a aucune. Elle se proposera au
  premier bilan publié.
- **Le sort de la couche V** dans ce découpage. La stratégie ne la nomme pas ; elle reste en
  porte E, intouchée, avec ses déclencheurs.
- **Aucune réduction d'ambition.** L'utilisateur l'écrit deux fois, et ADR-127 reste entier :
  δ général, V, inondations complexes et grande échelle restent obligatoires.

## 4. Ce qui devient faux si cette décision est mal lue

**Ce n'est pas un abandon du rendu.** Le rendu reste l'instrument de diagnostic et de comparaison,
et les ajustements de couleur, ciel, exposition, reflets, écume et détails **restent dus** —
simplement, ils se feront quand les systèmes physiques correspondants seront assez avancés pour
les alimenter, au lieu d'être construits comme des effets indépendants. La cible chiffrée
construite en S308 (`outils/cible_image.py`, `outils/courbe_tonalite.py`, et les mesures de la
photographie de référence) est **conservée telle quelle** : elle sera l'entrée de ce lot le jour
où il s'ouvrira.

**Ce n'est pas non plus une autorisation de laisser filer le coût.** D4 suspend un critère
d'arrêt, pas la mesure. Un lot qui ne publierait pas son coût, sa mémoire et sa complexité
manquerait à cette décision autant qu'à ADR-131.
