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
