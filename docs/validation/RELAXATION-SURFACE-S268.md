# Relaxation de surface perturbative — S268

## Contrat avant construction

Capacité bornée : le pas couplé mobile amortit hauteur **et** vitesse perturbatives
près des bords, prérequis constaté absent aux frontières ouvertes. Consommateur :
`Volume::step_perturbation_mobile`, paramètres Sponge existants. ADR-164.

Critères avant code :
1. Étape locale contre exponentielle indépendante f64, deux signes, intérieur,
   profil symétrique, 1 et 1000 sous-pas ; erreur absolue <= 8 ulps de la hauteur de
   repos plus 8 epsilon f32 fois le nombre de pas fois l'amplitude initiale. Cette
   borne numérique couvre arrondis du coefficient et de la hauteur, pas une tolérance physique.
2. Intérieur et Sponge nul identiques au bit, hauteur et reste ; fond immuable.
3. Pas réel avec fond non nul : comparer au témoin qui conserve seulement l'éponge
   de vitesse, différence de hauteur égale à la relaxation prévue à la borne du point 1 ;
   u/w/p identiques au témoin pour ce pas. Répéter 20 pas pour exercer le consommateur.
4. Expiration à tous les checkpoints d'un petit domaine : aucune avancée, champs
   et surface au bit ; reprise identique. Zéro allocation mesurée dans le pas.
5. Suite cœur/harnais release hors ligne, tests existants à fond nul et sans éponge
   conservés. Pas de nouvelle réception GPU (chemin non touché).

Arrêt : critères ci-dessus reçus, API documentée, limites et prochaine mesure inscrites.
Aucune réception de réflexion <1 %, de passage d'un fond incident, des frontières du
TOTAL, de B4 global ou du budget mural 2 ms. Ces lots suivent avec un paquet de garde
séparant incident/réfléchi selon ADR-046. Ce découpage ne réduit pas J2.
