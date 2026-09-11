# Source par flux partagés — S171

## Protocole déclaré

Suite S170-1/A225, véhicule Saint-Venant1D mouillé, sans choix du runtime.
Même onde, Q figé, fenêtres, fantômes analytiques, moyenne de cellule et budget physique
que SOURCE-DECIMEE-S170. La sonde `source_decimee` conserve ses témoins Exact,
Omitted et Linear et ajoute FluxRaw et FluxAnchored.

FluxRaw interpole linéairement F(Q) sur les nœuds grossiers. Chaque face fine est
évaluée une fois ; S_i=(Fhat_i-Fhat_{i+1})/dx. La somme télescope :
Σ(S_i-Sexact_i)dx=Fhat(30)-Fhat(90)-[F(Q(30))-F(Q(90))].
La télescopie seule n'annule donc pas l'erreur du budget physique aux bornes.

FluxAnchored remplace les segments de bord par une interpolation passant par les
flux physiques exacts à30 et90m ; les nœuds strictement intérieurs sont conservés.
Aucune correction uniforme de source. Le coût supplémentaire est l'accès aux flux
exacts aux bornes ; leur disponibilité n'est pas établie pour le runtime.
Le compteur nodes est le nombre de nœuds retenus, pas un coût de calcul : le constructeur
expérimental évalue aussi des nœuds extérieurs avant de les filtrer.

H=1,2,4,8,16m, phase0/H/2 ; N=120,240,480 et pas de temps divisé par deux àN240.
128 évolutions. Comparer erreur locale de source, hauteur/débit face à la référence,
écart de champ au témoin Exact et défaut de volume prédit par l'injection nette.
La source issue du flux linéaire est constante par segment grossier : elle peut
conserver son intégrale tout en étant localement moins précise que Linear.

## Résultats reçus

N240, pas nominal. E : erreur maximale espace/temps de hauteur /0,05m ; D :
écart au champ du témoin Exact /0,05m ; eS : erreur maximale de source de masse
normalisée par son maximum exact ; V : défaut maximal du budget physique /volume initial.
Même normalisation que S170. Tableaux issus des128 lignes de la campagne release.

| Méthode | H | phase | E | D | eS | V |
|---|---:|---:|---:|---:|---:|---:|
| Exact | 0 | 0 | 1.356122e-1 | 0.000000e0 | 0.000000e0 | 9.234265e-16 |
| Omitted | 0 | 0 | 9.979493e-1 | 9.896758e-1 | 1.000000e0 | 8.881982e-7 |
| Linear | 1 | 0 | 1.372320e-1 | 3.767878e-3 | 6.100123e-3 | 4.254010e-8 |
| FluxRaw | 1 | 0 | 1.368460e-1 | 4.834882e-3 | 7.391394e-2 | 1.064906e-15 |
| FluxAnchored | 1 | 0 | 1.368460e-1 | 4.834882e-3 | 7.391394e-2 | 1.064906e-15 |
| Linear | 1 | 0.5 | 1.372314e-1 | 3.747551e-3 | 6.168954e-3 | 4.435084e-8 |
| FluxRaw | 1 | 0.5 | 1.368094e-1 | 4.872679e-3 | 7.484592e-2 | 6.489463e-8 |
| FluxAnchored | 1 | 0.5 | 1.368094e-1 | 4.872679e-3 | 7.484592e-2 | 7.942816e-16 |
| Linear | 2 | 0 | 1.420558e-1 | 1.466680e-2 | 3.364154e-2 | 1.664352e-7 |
| FluxRaw | 2 | 0 | 1.416509e-1 | 2.035409e-2 | 2.226738e-1 | 7.458493e-16 |
| FluxAnchored | 2 | 0 | 1.416509e-1 | 2.035409e-2 | 2.226738e-1 | 7.458493e-16 |
| Linear | 2 | 0.5 | 1.420366e-1 | 1.509891e-2 | 3.199464e-2 | 1.954109e-7 |
| FluxRaw | 2 | 0.5 | 1.416858e-1 | 1.962650e-2 | 2.153791e-1 | 2.669130e-7 |
| FluxAnchored | 2 | 0.5 | 1.416858e-1 | 1.962650e-2 | 2.153791e-1 | 1.122246e-15 |
| Linear | 4 | 0 | 1.606933e-1 | 5.785863e-2 | 1.330580e-1 | 1.076957e-6 |
| FluxRaw | 4 | 0 | 1.600327e-1 | 7.737153e-2 | 4.989280e-1 | 1.189436e-6 |
| FluxAnchored | 4 | 0 | 1.600327e-1 | 7.737153e-2 | 4.989280e-1 | 8.889320e-16 |
| Linear | 4 | 0.5 | 1.606837e-1 | 5.913340e-2 | 1.330580e-1 | 6.124864e-7 |
| FluxRaw | 4 | 0.5 | 1.605743e-1 | 7.610619e-2 | 4.989280e-1 | 7.921452e-16 |
| FluxAnchored | 4 | 0.5 | 1.605743e-1 | 7.610619e-2 | 4.989280e-1 | 7.921452e-16 |
| Linear | 8 | 0 | 3.022206e-1 | 2.462233e-1 | 4.844428e-1 | 6.309842e-5 |
| FluxRaw | 8 | 0 | 2.493705e-1 | 2.280473e-1 | 8.342942e-1 | 2.098241e-6 |
| FluxAnchored | 8 | 0 | 2.493705e-1 | 2.280473e-1 | 8.342942e-1 | 6.797353e-16 |
| Linear | 8 | 0.5 | 2.103723e-1 | 1.721816e-1 | 3.353031e-1 | 4.940147e-5 |
| FluxRaw | 8 | 0.5 | 3.117388e-1 | 2.621326e-1 | 9.538549e-1 | 1.297606e-5 |
| FluxAnchored | 8 | 0.5 | 3.117387e-1 | 2.621239e-1 | 9.538549e-1 | 8.013675e-16 |
| Linear | 16 | 0 | 4.801917e-1 | 4.448515e-1 | 7.148072e-1 | 6.024954e-3 |
| FluxRaw | 16 | 0 | 8.435420e-1 | 8.073996e-1 | 1.105069e0 | 2.262070e-6 |
| FluxAnchored | 16 | 0 | 8.435420e-1 | 8.073996e-1 | 1.105069e0 | 8.389845e-16 |
| Linear | 16 | 0.5 | 1.092567e0 | 1.067309e0 | 1.129711e0 | 6.398198e-3 |
| FluxRaw | 16 | 0.5 | 3.477239e-1 | 3.042484e-1 | 7.761584e-1 | 1.718575e-4 |
| FluxAnchored | 16 | 0.5 | 3.477239e-1 | 3.042484e-1 | 7.761584e-1 | 8.488673e-16 |

Variation indépendante de dx et dt : FluxAnchored, H8m, phase0.

| N | facteur temporel | E | D | eS | V |
|---:|---:|---:|---:|---:|---:|
| 120 | 1 | 2.843850e-1 | 2.113349e-1 | 7.608818e-1 | 6.252010e-16 |
| 240 | 1 | 2.493705e-1 | 2.280473e-1 | 8.342942e-1 | 6.797353e-16 |
| 480 | 1 | 2.507959e-1 | 2.399160e-1 | 8.692842e-1 | 2.046372e-15 |
| 240 | 2 | 2.493853e-1 | 2.280754e-1 | 8.342942e-1 | 8.223145e-16 |

## Lecture et limites

Les40 évolutions FluxAnchored ferment le volume à<=2,32e-15. Toutes les128
prédictions du défaut signé sont reçues à<=2,52e-15 ; Courant maximal0,217062.
FluxRaw ferme aussi le volume quand30 et90 coïncident avec ses nœuds (H1/2 phase0,
H4 phase0,5). Cela ne constitue pas une propriété indépendante du placement du réseau.

ÀN240/H8, passer de phase0 à0,5 inverse le classement en hauteur : FluxAnchored
fait mieux que Linear pour phase0 (E0,249 contre0,302), moins bien pour phase0,5
(0,312 contre0,210). H16/phase0 conserve le volume mais garde E0,844 et D0,807.
Conservation et précision locale ne sont pas des critères interchangeables.

ÀH8/phase0 fixé, raffiner N120→480 ne fait pas disparaître D (0,211→0,240).
Diviser dt par deux change D de0,228047 à0,228075 : ce défaut ne se résout pas
par le seul pas temporel dans cette campagne. Aucune convergence générale extrapolée.

Les témoins S170 sont conservés. Les7 tests de la sonde passent, dont3 nouveaux :
télescopie et erreur aux bornes dans les deux composantes ; budget ancré exact avec
source/champ inexacts ; raffinement de source améliorant le champ à budget déjà fermé.
Commandes depuis code/ :

```text
cargo test -p water-core --example source_decimee
cargo run -p water-core --release --example source_decimee
```

Aucun support partagé modifié ; tests de bibliothèque non rejoués (dernière réception
S163 :299 réussis,5 ignorés). Sept tests de la sonde et128 évolutions reçus en S171.
Les tolérances1e-10/1e-12 sont des gardes numériques du véhicule f64, pas des seuils
physiques ni une calibration is_smooth_at. Pas de performance3D, forces ou perception reçues.

**S170-1 réalisée ; A225 traitée dans ce périmètre1D à flux de bord connus.** A50 reste
partielle : Q est encore exact, seul S est reconstruit. Aucun ADR ou runtime adopté.

**Suite S172 : S171-1/A50**, reconstruire aussi Q sur le réseau grossier, puis construire
S à partir de ce même fond reconstruit. Initialiser d=Tinitial-Qreconstruit pour garder
le même état total physique, mesurer séparément erreur de représentation, évolution
et bilan. Garder le témoin Q exact et les frontières analytiques ; commencer par le fond
figé. Porteur : session de construction, poursuite B4/BILAN-S145.

**Suivi S172 : S171-1 réalisée sur véhicule figé**, voir
[FOND-RECONSTRUIT-S172](FOND-RECONSTRUIT-S172.md). Même total initial pour Q exact et
reconstruit ; source physique conjointe, témoin discret indépendant. A50 partielle ;
suite S172-1 : fond reconstruit mobile et cohérence temporelle de sa source.
