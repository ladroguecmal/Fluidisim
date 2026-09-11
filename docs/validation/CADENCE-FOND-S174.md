# Cadence du fond indépendante — S174

## Protocole déclaré

Suite S173-1/A50. Extension de fond_mobile par le mode --cadence. Même onde simple,
fenêtre [30,90] m, durée6 s et deux amplitudes totales0,05/0,06 ; Q amplitude0,05.
Le fond spatial est exact ou linéaire H8m/phase0,5, avec bornes spatiales exactes.

Tau=0 reprend l’accès temporel continu de S173. Tau=0,25/1/2 s préévalue des
instantanés de moyennes de cellule et d’états de face. Entre deux instantanés,
interpolation linéaire des variables conservatives, puis F de cet état interpolé.
La source utilise la même représentation : sécante Q1-Q0 et flux intégrés. Tout pas
traversant une réactualisation est découpé pour la quadrature du flux ; le solveur
conserve son pas propre. Q est continu à la réactualisation, sa dérivée ne l’est pas.

Ce véhicule connaît l’instantané suivant : fond analytique déterministe. Ce n’est pas
une preuve que le futur d’un événement inconnu de W soit disponible au runtime.
Les instantanés sont précalculés pour la mesure, sans benchmark de stockage ou de coût.

Conserver les quatre sources S173 : Integrated, Trapezoid, Discrete et Omitted.
Deux budgets calculés indépendamment depuis les flux réellement utilisés :

- représentation : flux numérique total - flux numérique Q + flux intégré du Q interpolé ;
- référence analytique : même flux numérique, mais flux Q analytique intégré aux bornes.

Discrete garde son budget numérique total dans les deux colonnes. La différence
entre les deux budgets des autres modes est l’intégrale de F(Q_interpolé)-F(Q_exact)
aux bornes. Le défaut signé de référence est prédit par cette différence plus le défaut
de source déjà dérivé en S173. Aucun recalage du volume.

N120/240 et demi-pas àN240, indépendamment de tau ; deux fonds spatiaux, deux amplitudes,
quatre cadences et quatre sources :192 évolutions résiduelles et24 témoins totaux.
Mesurer h/q, les deux volumes, l’identité discrète, les défauts signés prédits et l’erreur
au premier pas traversant chaque réactualisation intérieure (pas une mesure de saut).
Tolérances f64 et quadrature1e-12 ; aucun seuil physique is_smooth_at.

## Réception et portée

Depuis code/ :

```text
cargo test -p water-core --example fond_mobile
cargo run -p water-core --release --example fond_mobile -- --cadence
```

Six tests reçus, dont trois nouveaux : continuité des instantanés et intégration à
travers une réactualisation ; bilan interpolé fermé avec bilan analytique non fermé ;
raffinement de cadence distinct de celui du pas solveur. Les trois tests S173 sont
rejoués. Supports et bibliothèques inchangés ; tests S172 reçus S173, workspace reçu
S163 (299 réussis/cinq ignorés), non rejoués ici.

Le compteur crossings porte sur les pas qui traversent une réactualisation intérieure,
pas sur le nombre d’étages ni sur un saut de Q. Les tableaux refresh_h mesurent l’erreur
au terme de ces pas. Aucun mécanisme de remise à zéro ou recalage de d n’est utilisé.
Les allocations d’instantanés appartiennent au banc ; aucun coût temps réel certifié.

## Résultats reçus

Integrated, N240, pas nominal. a : amplitude du total ; H=0 : fond spatial exact.
E : erreur maximale de hauteur /0,05 m ; Eq : débit /0,05√g.
V : défaut du budget interpolé /volume initial ; Va : défaut du budget de référence
analytique. Er : erreur de hauteur au premier pas traversant une réactualisation.

| a | tau (s) | H (m) | E | Eq | V | Va | Er |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 0.05 | 0 | 0 | 2.269296e-12 | 2.680656e-12 | 9.688458e-16 | 9.688458e-16 | 0.000000e0 |
| 0.05 | 0 | 8 | 6.377213e-2 | 6.848401e-2 | 9.893571e-16 | 9.893570e-16 | 0.000000e0 |
| 0.05 | 0.25 | 0 | 7.672812e-4 | 8.193040e-4 | 8.163029e-16 | 3.580072e-7 | 7.389018e-4 |
| 0.05 | 0.25 | 8 | 6.400558e-2 | 6.873360e-2 | 1.113212e-15 | 3.580072e-7 | 6.074188e-2 |
| 0.05 | 1 | 0 | 1.153544e-2 | 1.224508e-2 | 9.901499e-16 | 5.574358e-6 | 9.789204e-3 |
| 0.05 | 1 | 8 | 6.729876e-2 | 7.227199e-2 | 8.070922e-16 | 5.574358e-6 | 4.993213e-2 |
| 0.05 | 2 | 0 | 3.794250e-2 | 3.996959e-2 | 8.860006e-16 | 2.045138e-5 | 2.657918e-2 |
| 0.05 | 2 | 8 | 7.514335e-2 | 8.064908e-2 | 7.428692e-16 | 2.045138e-5 | 5.360408e-2 |
| 0.06 | 0 | 0 | 3.123801e-2 | 3.355441e-2 | 8.258407e-16 | 8.258407e-16 | 0.000000e0 |
| 0.06 | 0 | 8 | 9.163707e-2 | 9.971673e-2 | 6.357324e-16 | 6.357324e-16 | 0.000000e0 |
| 0.06 | 0.25 | 0 | 3.196597e-2 | 3.433678e-2 | 8.550962e-16 | 3.571730e-7 | 3.087303e-2 |
| 0.06 | 0.25 | 8 | 9.186931e-2 | 9.996486e-2 | 8.240877e-16 | 3.571730e-7 | 9.022443e-2 |
| 0.06 | 1 | 0 | 4.244210e-2 | 4.565137e-2 | 8.010607e-16 | 5.561368e-6 | 3.582291e-2 |
| 0.06 | 1 | 8 | 9.513140e-2 | 1.034690e-1 | 8.538863e-16 | 5.561368e-6 | 7.425378e-2 |
| 0.06 | 2 | 0 | 6.740420e-2 | 7.246782e-2 | 6.610253e-16 | 2.040372e-5 | 4.701322e-2 |
| 0.06 | 2 | 8 | 1.031365e-1 | 1.120307e-1 | 1.004312e-15 | 2.040372e-5 | 6.886654e-2 |

Fond spatial exact, tau2 s fixé : raffinement du solveur.

| N | facteur dt | a | E | Va |
|---:|---:|---:|---:|---:|
| 120 | 1 | 0.05 | 5.887856e-2 | 2.045138e-5 |
| 240 | 1 | 0.05 | 3.794250e-2 | 2.045138e-5 |
| 240 | 2 | 0.05 | 3.794150e-2 | 2.045138e-5 |
| 120 | 1 | 0.06 | 1.068063e-1 | 2.040372e-5 |
| 240 | 1 | 0.06 | 6.740420e-2 | 2.040372e-5 |
| 240 | 2 | 0.06 | 6.739333e-2 | 2.040372e-5 |

Sources comparées, N240/a0,05/H0/tau1 s. Discrete garde son budget numérique total.

| source | E | V | Va | Er |
|---|---:|---:|---:|---:|
| Trapezoid | 1.146509e-2 | 3.925403e-9 | 5.578283e-6 | 9.754861e-3 |
| Integrated | 1.153544e-2 | 9.901499e-16 | 5.574358e-6 | 9.789204e-3 |
| Discrete | 1.295382e-1 | 8.790097e-16 | 8.790097e-16 | 1.110070e-1 |
| Omitted | 4.543372e-2 | 5.574358e-6 | 4.331135e-6 | 5.383421e-3 |

## Conclusions limitées au véhicule

192 évolutions résiduelles et24 témoins totaux reçus. Les48 variantes Integrated
ferment le budget de leur représentation à<=1,21e-15 ; les192 prédictions signées
des deux budgets sont reçues à<=1,21e-15. Les48 témoins Discrete restent identiques
au solveur total à<=2,23e-16. Courant maximal0,219722.

ÀN240/a0,05/H0, augmenter tau de0,25 à1 puis2 s fait passer E de0,0007673 à0,01154
puis0,03794. L’accès continu donnait E2,27e-12. Le budget interpolé reste fermé,
mais Va vaut3,580e-7,5,574e-6 puis2,045e-5. Ces défauts sont des intégrales de flux
erronés aux bornes ; raffiner dx ou dt àtau fixé ne les élimine pas.

Àtau2 s fixé, E descend de0,05888 à0,03794 entre N120 et240. Le demi-pas N240
change E seulement de0,0379425 à0,0379415 et laisse Va identique aux chiffres publiés.
Le solveur et la cadence du fond sont deux paramètres de précision distincts.

Les réactualisations intérieures sont effectivement traversées :23,5 et2 pour les
cadences0,25/1/2 s. Q reste continu, aucun résidu n’est remis à zéro. Er mesure une
erreur de transport/représentation autour de ces instants ; elle ne démontre ni saut
visuel ni réception perceptive. La cadence continue a crossings=0 et Er=0 par convention.

L’échantillon futur utilisé pour interpoler est connu dans ce cas analytique ; le
protocole ne reçoit pas l’anticipation d’un événement extérieur inconnu. La frontière
continue encore à recevoir le total analytique exact : l’assemblage autonome est à faire.

**S173-1 réalisée sur véhicule à instantanés connus ; A50 partielle.** A225 reçoit ici
son prolongement temporel : une conservation relative aux flux reconstruits peut
coexister avec leur erreur de référence. Aucun nouvel angle, ADR ou runtime adopté.

**Suite S175 : S174-1/A50**, assembler le fond à cadence réduite avec la frontière
autonome ancrée de S169. Comparer aux fantômes analytiques de S174, recevoir la
préservation, le transport et les deux budgets ; conserver les hypothèses d’accès àQ
et ne pas supposer un résidu extérieur inconnu disponible. Porteur : construction,
poursuite B4/BILAN-S145.

**Suivi S175 : S174-1 réalisée sur véhicule subcritique àfond connu**, voir
[FRONTIERE-FOND-DECIME-S175](FRONTIERE-FOND-DECIME-S175.md). Comparaison de champs
appariés, sortie effective de la crête, témoins totaux aux mêmes frontières. A50 reste
partielle ; suite S175-1, bilan de réception B4 et prochain lot de construction.
