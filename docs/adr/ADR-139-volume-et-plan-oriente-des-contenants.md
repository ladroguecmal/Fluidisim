# ADR-139 — Le plan orienté se déduit du volume de la géométrie du contenant

- **Statut** : acté sur délégation technique ; réception de l'implémentation en S228.
- **Session** : S228, 2026-09-13.
- **Remplace** : ADR-010 §2, table horizontale utilisée sous toute orientation ; précise I-08 pour V.
- **Résout** : A266, dans le domaine géométrique construit et reçu.

## Décision

Pour une géométrie locale Ω, une normale `u = −g_eff / |g_eff|` et le volume entier M, chercher
le décalage d tel que `volume(Ω ∩ {x : u·x ≤ d}) = M`. Le plan exposé est `u·x = d` dans le
repère **fixe de la géométrie**, translaté par `HydroNode.origin_um`. L'origine ne suit pas la
gravité. Une hauteur verticale n'est pas une distance normale, sauf sous `u = +Z`.

La première représentation est une **partition tétraédrique** du volume intérieur, construite
hors du pas et empruntée à l'hôte. Elle représente des polyèdres convexes ou non convexes ; elle
ne limite pas V aux prismes ni à une inclinaison ou un azimut. Les frontières courbes demandent
une approximation géométrique dont l'auteur doit qualifier l'erreur. Chaque nœud suppose un
liquide à l'équilibre hydrostatique dans un contenant communicant ; les poches séparées ou les
transferts transitoires demandent plusieurs nœuds/arêtes, pas une redistribution cachée.

Les tétraèdres sont non dégénérés, sans recouvrement intérieur. Les sommets sont des micromètres
entiers locaux, de module par composante strictement inférieur à 4096 m (I-08). Construction :
déterminants entiers élargis, contrôle des intersections par axes séparateurs, capacité tirée
du volume géométrique et arrondie au millilitre. Le nœud doit porter cette même capacité : aucune
mise à l'échelle implicite d'une forme par une capacité arbitraire.

## Calcul et unités

Les formules sont dérivées dans [SPEC-001 §6](../specs/SPEC-001-contraintes-numeriques.md#6-hydraulique-couche-v).
Le volume sous un plan est la somme, en ordre fixé, des volumes coupés de chaque tétraèdre.
Le calcul de fraction traite explicitement zéro, un, deux, trois et quatre sommets mouillés ;
les projections égales ne provoquent aucune division par zéro. La position du plan est cherchée
par dichotomie dans les projections extrêmes des sommets, avec au plus **64 subdivisions**
(borne algorithmique, pas seuil physique), arrêt à stagnation représentable.

L'état et les échanges restent entiers. Les calculs de projection, fraction et débit de V
utilisent **f64 comme intermédiaire local**, exception explicite à I-08 : f32 ne conserve déjà
plus tous les millilitres au-delà de 2²⁴ ml. Cela explicite aussi le chemin de débit S224 et la
projection S226. Aucune exception à I-03 : pas de fast-math, ordre fixé, mêmes données cuites ;
la réception sur une seconde cible demeure due. B/W et δ ne gagnent aucune exception ici.

La capacité arrondie diffère d'au plus 0,5 ml du polyèdre exact. À vide et plein, le plan prend
la projection extrême correspondante. À volume intérieur, le résidu **calculé** de l'inversion
doit être au plus 0,5 ml ; sinon refus `Resolution`. Ce contrôle ne borne pas à lui seul
l'erreur flottante du calcul direct : la réception publie les erreurs contre des oracles
indépendants et leur domaine, sans promettre le demi-millilitre sur toute taille admissible.

## Migration et consommation

`Shapes::new` garde les tables S224 pour leur **seule orientation cuite +Z** ; sous une autre
direction, refus explicite `Orientation`, avant mutation de l'état. Elles ne contiennent ni
largeur, ni azimut, ni géométrie permettant de les convertir automatiquement. Le chemin vertical
conserve ses anciens résultats. Les nouvelles formes entrent par `Shapes::from_volumes`.

Le pas consomme le plan pour **les deux extrémités** d'une arête ; les lois de débit et les
limiteurs ne changent pas. Refus atomiques, aucune allocation au pas, tranches fournies par
l'hôte. Le calcul peut être répété pour les arêtes partageant un nœud dans cette première version ;
son coût se mesure sur le pas complet. La mise en cache et l'ordonnancement restent des possibilités
d'optimisation, jamais une justification pour revenir à une table fausse.

La restauration future doit identifier la géométrie cuite et sa version ; les plans restent
dérivés du volume et de `g_eff`. Aucun état de δ ne se sérialise (I-17).

## Alternatives et réception

Une table `(volume, inclinaison)` ne suffit pas sans azimut pour une forme quelconque. Une table
d'orientation complète reste une optimisation possible après maîtrise de son interpolation.
Les corrections fermées propres à un prisme ne couvrent pas les cales et les intersections
fond/plafond. La partition permet d'abord une construction générale, bornée en nombre d'opérations.

Réception prévue : régression A266 active ; prisme, cale et forme non convexe ; vide/plein et
petits volumes ; directions axiales inversées et azimuts obliques ; références analytiques ou
intégrales indépendantes ; source **et** receveur orientés ; refus sans mutation ; témoin
d'allocation positif et pas sans allocation. Le coût reçu reste celui du montage mesuré.

Aucune nouvelle obligation d'audit : après réception, la suite est l'état V restaurable, avec
comparaison à J2 selon la règle de reprise.

## Réception datée S228 — 2026-09-13

La première version est construite et consommée par le pas de V. La régression A266 est active
et passe. [VOLUME-ORIENTE-S228](../validation/VOLUME-ORIENTE-S228.md) porte les oracles, refus,
allocations et coûts. A269 garde ouverte la qualification de précision des tailles supplémentaires.

## Réception datée S229 — 2026-09-14

La restauration du noyau identifie désormais la base et sa révision, avec une empreinte de
ses géométries ordonnées. Plans toujours dérivés, aucun état δ écrit ; contrat ADR-140 et
[réception C19-V](../validation/RESTAURATION-V-S229.md).
