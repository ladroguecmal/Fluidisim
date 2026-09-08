# S94 — Pente et vitesses de la pression gaussienne

2026-09-08. Réalise S93-1 dans la référence profonde linéaire f64 des ADR-069/070.
Aucune modification des invariants ni raccordement au runtime autoritaire.

## Construction

Avec la convention Fourier d'ADR-070, un nœud porte l'élévation complexe q et sa
dérivée temporelle q_dot. La condition cinématique profonde donne le potentiel
de surface psi = q_dot / |k|. Le mode nul n'est pas échantillonné ; la jauge ne
contient pas de potentiel constant. Les expressions reconstruites à z=0 sont :

```
eta        = somme poids Re(q exp(i k·x))                      [m]
w          = somme poids Re(q_dot exp(i k·x))                  [m/s]
phi        = somme poids Re(psi exp(i k·x))                    [m²/s]
pente_j    = -somme poids k_j Im(q exp(i k·x))                  [1]
u_j        = -somme poids k_j Im(psi exp(i k·x))                [m/s]
```

Il s'agit de dérivées des expressions déjà adoptées, pas d'un changement de modèle.
La norme de k est celle du vecteur effectivement utilisé par PressureMode. Le potentiel
complexe est préparé une fois par nœud ; sample ne refait ni réponse temporelle ni division.
Surface expose potential, slope et horizontal_velocity en plus des deux champs antérieurs.
Les deux chemins possédé/emprunté partagent le calcul. Toute composante non finie refuse
la sortie entière ; le potentiel modal non fini refuse déjà la préparation, sans vue publiée.

La vitesse est celle du fluide dans le repère local, pas celle de la pression mobile.
Les champs sont évalués au plan moyen z=0, conformément à la linéarisation : ils ne sont
pas une évaluation non linéaire à z=eta. Aucun champ immergé n'est exposé. Une composition
future doit sommer les pentes puis reconstruire la normale, et sommer toutes les vitesses.

## Réception des identités

Trois tests S94, plus extension du test d'identité possédé/emprunté S93 aux nouveaux champs.
Les différences centrées utilisent dx=1e-4 m et dt=100 µs, hors des discontinuités du forçage.
Virage S91, quadrature 32×48, instants 1/3/6 s, points (0,7 ; -0,8), (4,2 ; 2,3), (8 ; 6).
Chaque grandeur possède un témoin non nul >1e-5, pour empêcher une réussite par champ nul.

| Identité | Erreur absolue maximale release |
|---|---:|
| gradient eta = pente | 4,487e-12 |
| gradient phi = vitesse horizontale | 5,518e-12 m/s |
| eta_t = vitesse verticale | 2,502e-11 m/s |
| phi_t = -g eta - p/rho | 1,498e-10 m²/s² |

Seuil de régression 1e-9 dans chaque unité, propre à ce montage, à calibrer pour usage
élargi. Le dernier contrôle reconstruit p avec la même quadrature : il reçoit l'identité
dynamique, pas la précision spatiale du profil (L204).

Un second test vérifie rotation de 90°, réflexion y→-y avec changement de signe de la
composante transverse, et invariance au découpage rectiligne, à 0/1/2/4/8 s. Seuil 1e-14
par composante. Un troisième injecte des débordements distincts de pente, vitesse horizontale
et potentiel alors que eta et w restent finis ; chaque sortie est refusée, avec témoin accepté.

## Raffinement indépendant

Exemple receive_gaussian étendu : même virage et domaine [-8,12]² m, neuf instants de 0 à
8 s et grille 11×11, soit 1089 points par comparaison. Base 128×128, coupure 6 rad/m.
Le maximum vectoriel est pris composante par composante ; ce n'est pas une norme euclidienne.

| Raffinement | Potentiel m²/s | Pente | Vitesse horizontale m/s |
|---|---:|---:|---:|
| Radial 128→256 | 4,616751111e-6 | 1,702834811e-9 | 3,585612579e-8 |
| Directions 128→256 | 6,591949209e-17 | 2,883977779e-17 | 6,288372600e-17 |
| Coupure 6→9, radial 128→192 à pas constant | 3,528949181e-11 | 9,776261566e-11 | 1,884187793e-10 |

Seuils de régression déclarés avant mesure : 1e-5 m²/s, 1e-6 et 1e-5 m/s, à calibrer
hors fixture par ce banc. Tous respectés. Les quatre maxima antérieurs de S92 sont inchangés.
L'erreur du potentiel est reçue séparément : sa division par |k| change la pondération
spectrale. La bonne précision de la hauteur ne suffit pas à recevoir ses autres grandeurs.
Pas de borne continue ni réception de toutes les trajectoires (L205).

## Suite

S93-1 réalisée. **S94-1, S95 :** construire le noyau modal f32 à phases déterministes,
avec traitement des résonances et comparaison à la référence S89, avant portage du champ
gaussien et composition. L'instrument actuel reste f64/libm ; ses pools ne le rendent pas
conforme à I-03/I-08. Aucun coût, codec de trajectoire, couplage de coque ou Kelvin complet reçu.
