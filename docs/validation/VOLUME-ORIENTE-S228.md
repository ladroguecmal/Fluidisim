# S228 — Le volume détermine le plan orienté de V

2026-09-13. Décision : [ADR-139](../adr/ADR-139-volume-et-plan-oriente-des-contenants.md).
Base : a86bd43, fin S227. Géométrie construite en ac39870, branchée au pas en 6e64d99 ;
les mesures ci-dessous portent sur ce chemin, complété par les tests et l'exemple de P5.

## Résultat consommé

**A266 corrigée dans le domaine polyédrique reçu.** `Shapes::from_volumes` fournit le plan local
`u·x = d` depuis une partition tétraédrique. `step` utilise ce même plan pour source et receveur.
`surface_plane` l'expose aux consommateurs. Les anciennes tables restent reçues sous +Z et
refusent toute autre orientation avant mutation ; aucun faux prisme n'est reconstruit depuis
une table qui ne contient ni largeur ni azimut.

| cas traversant le pas réel | résultat S228 |
|---|---|
| Régression A266 : prisme à mi-remplissage, hublot central 1,01 m | **0 ml**, contre 506 en S227 ; témoin à 0,99 m débite ; test désormais actif |
| Cale de section `|x| ≤ z ≤ 2`, longueur 1 m, volume 1 m³, pente 0,3 | Hublot central à 0,96 m sec, à 0,94 m mouillé ; cote exacte `√(1−0,3²) ≈ 0,953939 m` |
| Source et receveur prismatiques aux cotes centrales 1,02 et 1 m | **38 ml** au premier pas, oracle de charge normale ; même résultat à origine entière extrême |
| Hublot latéral S226 | **1 736 ml**, contre 1 829 : le décalage erroné du plan est corrigé |
| C16, seuil de mise en débit à x = −2 / +2 m | **400 007 / 1 599 989 µm** ; inclinaison 16,6990° pour 16,6992° attendus |
| C12 et chaîne historiques, anciennes tables sous +Z | **727,4 s**, chaîne `[532351,300688,166961]`, inchangés |

La pente correcte de S226 est conservée ; son ancienne cote centrale et son débit incliné
ne sont pas des références à figer. Les valeurs historiques restent dans leur reçu daté.

## Géométrie et précision

Six tests de géométrie, trois nouveaux tests d'intégration au réseau, régression A266 activée.
Boîte : dix directions, axes inversés, azimut oblique et norme minuscule ; volumes de 1 ml
jusqu'à capacité−1 ml, plus vide/plein. Cale : neuf pentes de −2 à +2 ; plan touchant fond et
plafond. Forme en L non convexe ; 24 permutations des sommets ; projections égales ; dalle de
1 mm à 3 km de l'origine. Les références ne découpent pas de tétraèdres : intégrales séparables
du pavé et intégration exacte de segments affines pour la cale.

À l'impression au nanomillilitre, les erreurs maximales mesurées avant les deux pentes
supplémentaires de P5 sont **0,000000003 ml** (boîte) et **0,000000001 ml** (cale).
Tous les cas finaux respectent le **demi-millilitre**, demi-unité du volume entier. Ce résultat
ne certifie pas toute taille ou toute partition possible.

Les déterminants, recouvrements et capacité arrondie sont contrôlés en entier élargi. Un test
refuse deux tétraèdres entrelacés sans sommet contenu ; contacts sans volume autorisés. Le
constructeur ne prouve pas l'adéquation à un maillage d'auteur ni sa connexité hydraulique.
Les contenants courbes restent à qualifier par leur erreur d'approximation.

**Limite explicite** : dans un cube de 3 km, demander `2⁵³+1 ml` rend `Resolution` : le calcul
direct f64 ne distingue plus tous les millilitres. L'entrée entière n'est pas arrondie pour
faire réussir le pas. Le résidu calculé n'est pas un certificat de l'erreur géométrique réelle ;
qualification des géométries et tailles supplémentaires à l'usage (A269).

## Allocation, refus et coût

Un compteur du tas reçoit une contre-épreuve positive puis observe **zéro allocation** sur
les plans et 100 pas, cinq directions, avec transferts réels. Le refus est aussi sans allocation.
`Orientation`, capacité incohérente et `Resolution` tardif préservent nœuds et restes d'arêtes.

Coût sur Windows, **AMD Ryzen AI 7 350**, Rust 1.97.0, release hors réseau, un fil. Géométrie
partagée de six tétraèdres par forme ; prismes 4 × 1 × 2 m initialement à moitié pleins ;
`g=[2,943;0;−9,81]`, pas 100 ms. Cent pas de chauffe puis 500 mesures, sans réinitialiser les
volumes. Temps CPU du **pas complet**, horodatage inclus ; initialisation exclue.

| topologie | nœuds / arêtes | médiane (µs) | p95 (µs) | maximum observé (µs) |
|---|---:|---:|---:|---:|
| fuites extérieures indépendantes | 1 / 1 | 7,9 | 13,0 | 146,9 |
| fuites extérieures indépendantes | 16 / 16 | 121,4 | 197,3 | 1 680,9 |
| fuites extérieures indépendantes | 64 / 64 | 447,5 | 840,9 | 1 474,9 |
| anneau à surface libre, cotes espacées de 1 mm | 16 / 16 | 222,9 | 347,5 | 616,2 |
| anneau à surface libre, cotes espacées de 1 mm | 64 / 64 | 812,6 | 1 366,0 | 1 807,4 |

Rang : deuxième passage des fuites, premier des anneaux, après ajout du receveur à la mesure.
Le premier passage des fuites donnait 7,3 / 108,7 / 464,8 µs en médiane. La variabilité des
maximums interdit d'en faire une garantie. Les anneaux conservent exactement la masse et
transfèrent effectivement ; ils ne sont pas des réseaux pleins sous pression.

**Présent** : volume coupé, dichotomie bornée, quantification et limites collectives.
**Absent** : cache de plan par nœud, index d'adjacence, interruption sous budget, réseau pressurisé.
Ces chiffres ne reçoivent ni I-05 ni la grande échelle. La construction des formes est O(T²),
hors pas ; leur inversion O(64T), potentiellement répétée par arête. Un consommateur dépassant
son budget motive cache/index et nouveaux montages, sans bloquer la restauration V.

## Reproduire et poursuivre

Depuis `code/` :

```text
cargo test --workspace --release --offline --quiet
cargo test -p water-core hydro_network --lib --offline -- --nocapture
cargo test -p water-core --test hydro_geometry_runtime --offline
cargo run -p water-core --example hydro_oriented_cost --release --offline
```

Suite complète : **398 réussis, 5 ignorés**, contre 387/6 en S227. Aucun nouvel ignoré ni nouvel
avertissement de compilation. A266 sort des ignorés ; les cinq autres gardent leur motif.
La précision des deux pentes supplémentaires de cale a aussi passé la suite release finale.

**Suite : état V restaurable**, nœuds et restes, identité/version de la géométrie. Pas de nouvelle
campagne géométrique préalable. A269, la cuisson d'assets réels, A264 sur le chemin tabulé,
multiplateforme et budget restent avec leurs déclencheurs dans la file active.
