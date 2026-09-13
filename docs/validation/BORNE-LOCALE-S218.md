# S218 — Borne locale de pente avec reste spatial

## Conditions et critères

ADR-135, file J1/W, A255. **Techniques présentes** : source gaussienne du cœur,
demi-spectre 64×128 (4096 modes), préparation depuis journal, phases entières,
majorant directionnel et annonce locale f32. **Absentes** : GPU, LOD, visibilité,
mutualisation, partition adaptative, cache des bornes. Un fil sur AMD Ryzen AI 7 350,
Windows, compilation release, aucun téléchargement.

Fixtures S217 : sigma2/cutoff3, charge19620N, origine(-12,0), emprise128×96m,
D8/v3, D8/v1,5, D16/v3, à tau4 ; D8/v3 à tau24 en diagnostic hors durée d'image.
Références : maxima S217 (minorants échantillonnés, pas preuves de sûreté).
Partition complète en carrés de2/1/0,5m ; aucun trou, coins communs. Max des bornes
comparé au max des centres, à la référence et au majorant global. Préparation séparée,
100 appels de chauffe, passages indépendants explicités avec les résultats.

Critères avant réception : aucun point testé au-dessus de sa borne ; centre trompeur
effectivement rejeté comme majorant ; contexte/temps/domaine/non-fini refusés ; zéro
exact conservé. Mesurer le gain, sans seuil de vitesse inventé ni verdict GPU.

## Construction

`Field::local_slope_envelope(min,max)` et `Prepared::local_slope_envelope(context,time,min,max)`
publient borne retenue, pente au centre, reste spatial, réserve numérique. Aucun cache,
aucune allocation (inspection : accumulateurs scalaires, pas de Vec ni de pool demandé).
Les échantillons historiques et les trois préparations restent inchangés.

Le reste exploite la monotonie des produits f32 `turns*x` aux extrémités du rectangle,
avant repliement. Il couvre donc les sauts de quantification spatiale, que `|k|*distance`
seul ignorerait. Sommes positives arrondies vers le haut ; réserve commune aux branches
locale/globale. **La preuve trigonométrique ne certifie pas toute l'arithmétique f32**.

Quatre tests ciblés : zéro/refus/débordement et norme sous-passant à zéro ; rectangle
autour d'un zéro avec pente intérieure >0,45 et borne <0,6 ; translation à4000m ;
rectangle ponctuel ; couverture de64 rectangles multidirectionnels et leurs coins ;
identité des échantillons avant/après ; contrôle contexte/instant de la vue préparée.
Supprimer le reste spatial ferait échouer le contre-exemple du centre.

## Limites de l'intégration

**Aucune admission ne consomme encore cette annonce.** `slope_floor` continue de couvrir
toute l'emprise avec ADR-134 ; un rectangle local n'autorise pas à réduire ce plancher
global. Un résultat plus serré est une capacité nouvelle de W, pas un gain de coût image.

Reproduction depuis `code/` :

```
cargo test --offline -p water-core local_bound --lib
cargo test --offline --release --workspace
cargo run --offline --release -p water-core --example borne_locale_s218
```

## Réception des bornes

| fixture | globale ADR-134 | partition 0,5m | réserve numérique | globale/locale | locale/max S217 |
|---|---:|---:|---:|---:|---:|
| base | 0,115171090 | 0,112293623 | 0,000266517 | 1,025625 | 1,596824 |
| lente | 0,032634243 | 0,026652928 | 0,000089339 | 1,224415 | 1,857050 |
| longue | 0,134346545 | 0,116187394 | 0,000305917 | 1,156292 | 1,734464 |
| base tardive, hors durée d'image | 0,116219424 | 0,086482756 | 0,000268573 | 1,343845 | 1,941127 |

Les partitions de2m et1m ne resserrent aucun des quatre cas : la branche globale
prend le relais, et la réserve ajoute environ0,23–0,27 %. À0,5m, le reste spatial
domine encore largement la réserve ; réduire seulement cette réserve n'apporterait
pas le gain recherché. Aucun maximum de référence ou de centres n'est au-dessus de
sa borne. La comparaison à S217 vérifie la cohérence, les tests locaux portent la
contre-épreuve qui manquerait à une seule comparaison de maxima.

**A255 est partiellement traitée par une capacité locale**, sans effet sur le budget
global de composition. La marge reste substantielle. Une partition uniforme assez
fine n'est pas retenue comme chemin de production : son coût est mesuré ci-dessous.

## Suite de construction

Partition adaptative sur pool de rectangles fourni par l'hôte : ne raffiner que les
rectangles dont la borne peut encore dominer le maximum, conserver leur borne au
plafond de travail. Recevoir la couverture complète, la reprise et le refus de capacité
avant de remplacer le calcul uniforme. Cette proposition n'est pas construite S218.
Le majorant local, les phases du champ et les tests S218 en sont les briques reçues.
La migration d'admission, A254, la loi GPU et les jalons δ/V gardent leurs préalables.

## Coût et vérification

| base, partition | rectangles | passage isolé 2 (ms) | passage isolé 3 (ms) |
|---|---:|---:|---:|
| 2m | 3072 | 1661,728 | 1616,573 |
| 1m | 12288 | 6729,315 | 6542,758 |
| 0,5m | 49152 | 28208,797 | 27545,679 |

Préparation séparée : 6,957 puis 6,279ms. Les valeurs numériques imprimées sont
identiques sur les trois passages. Le premier passage complet partageait la machine
avec compilation/tests : ses temps sont conservés comme trace, pas comme coût nominal.
Les deux suivants exécutent uniquement la fixture base, avec le binaire final.
Ces mesures portent le parcours uniforme CPU, sans conclusion sur le coût d'un futur
chemin adaptatif ou GPU. Le gain maximal recevable est un facteur1,224415, soit une
réduction de borne d'environ18,3 %, pour un parcours encore beaucoup trop coûteux.

[Relevés bruts des trois passages](BORNE-LOCALE-S218-MESURES.md).
Workspace release : **360 tests réussis, 5 ignorés**, avant la protection finale contre
la norme sous-passant à zéro. Après cette protection, les **4 tests ciblés réussissent
en debug et release**. Aucune instrumentation d'allocation ; constat par inspection.
La réception empirique est favorable ; la certification complète des arrondis reste ouverte.
