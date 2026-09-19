# S285 — séparer préparation et réduction

Le [banc S284](PREPARATION-RETRECISSEMENT-S284.md) mélangeait deux opérations. S285 conserve
trois trajectoires : **I** large intact, **P** large préparé, **R** préparé puis réduit.
Le résultat interdit d'attribuer les 66,994 mm au seul déplacement des frontières.

## Protocole reproductible

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --delta-attribution
cargo test --release --offline --locked --manifest-path viewer/Cargo.toml
```

Même houle, onde de 0,6 m, maille de 2 m et pas de 16 ms que S284. Demande après le pas 64
(1,024 s), fin au pas 320 (5,120 s). R traverse le vrai `Layer::update`, budget de banc
1 000 ms ; I et P consomment `Live::advance`. P reprend le drapeau de préparation de R avant
chaque pas, puis cesse donc de préparer **au même instant que R**. Continuer de préparer P
après la permutation aurait changé deux choses au lieu d'isoler l'effet de la réduction.

Le banc refuse une interruption, une désynchronisation ou une différence de champs préparés :
η, u et w sont identiques au bit avant transfert, y compris au pas de permutation grâce au
grand domaine conservé dans la réserve. **48 pas préparés appariés**. Les deux témoins gardent
128 colonnes jusqu'au bout ; R passe à 64. Aucune permutation forcée, garde S283 inchangé.

Mesure aux 16 centres de colonnes de −15 à +15 m, où les fondus sont unitaires. Les hauteurs
de I/P viennent de Live, celle de R du profil CPU effectivement publié. À ces nœuds, Hermite
restitue la hauteur nodale. Les trois états sont égaux avant la demande. La décomposition
signée `(P−I) + (R−P) = R−I` est contrôlée en chaque point. Les maxima absolus ci-dessous ne
s'additionnent pas : ils peuvent être atteints en des points ou instants différents.

## Résultats locaux, Windows release, 2026-09-19

| instant | max spatial P−I | max spatial R−P | max spatial R−I |
|---|---:|---:|---:|
| demande, 1,024 s | 0 mm | 0 mm | 0 mm |
| permutation, 1,792 s | 8,118 mm | 0 mm | 8,118 mm |
| 3,072 s | 35,904 mm | 3,754 mm | 39,658 mm |
| 5,120 s | 42,488 mm | 25,391 mm | 66,994 mm |
| maximum sur toute la fenêtre | **45,517 mm** | **25,391 mm** | **66,994 mm** |

Avant le pas de permutation, le maximum P−I est déjà 7,706 mm. Le centre reste inchangé
par l'opération locale d'amortissement, mais **l'évolution entre ces opérations change**.
Après permutation, P−I continue d'évoluer alors que P ne prépare plus. R−P mesure l'effet
supplémentaire du transfert puis de la dynamique sur le domaine étroit, conditionné par cette
histoire préparée ; ce n'est pas une décomposition linéaire universelle des mécanismes.

**Zéro allocation** dans les 963 appels consommés (321 fois deux Live et un Layer).
Les impressions et la construction initiale sont hors comptage. Les 34 tests du viewer
réussissent, un ignoré, aucun échec. Cœur inchangé depuis S284 : reçu antérieur de 507 tests,
non rejoué. Aucun nouveau chiffre de coût : cette campagne mesure l'attribution, pas la vitesse.

## Conséquence pour la construction

Une correction du seul transfert ne suffit pas : au moment où il conserve exactement le
centre, la préparation a déjà dépassé le repère de hauteur de 3 mm. Il faudra recevoir
l'évolution de la préparation **avant** une réduction automatique. Un garde de pente ou de
vitesse instantanée ne prouve pas non plus, à lui seul, une fidélité future.

Le banc n'identifie pas encore le mécanisme interne (projection, propagation, énergie retirée)
et ne justifie donc pas de modifier arbitrairement le solveur. Pas de référence physique
absolue, ni oracle de frontière : I est un témoin numérique commun, pas la vérité de l'eau.
Pas de réception des pentes, reflets, interpolant hors nœuds ou qualité visuelle. A290 reste ouvert.

Ce troisième lot spatial s'arrête ici : l'attribution était nécessaire pour choisir une
correction, mais son approfondissement passe après **A276, cadence découplée et budget**, avant
3D et deuxième domaine. Revenir à A290 lors de l'intégration de la réduction automatique, avec
une stratégie de préparation dont la trajectoire complète est qualifiée. L'ambition 3D reste
obligatoire ; ni ce banc ni le rétrécissement n'en apportent les interactions manquantes.
