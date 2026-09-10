# ADR-103 — Raccorder mouvement et charge prescrits au sillage

Statut : acté, S150, 2026-09-10. Complète ADR-069/070/074, lot W4 d'ADR-054.

La pression mobile existe depuis S89 et son chemin runtime depuis S96–S115.
On ajoute son entrée depuis un objet : position initiale, naissance, tronçons de
vitesse horizontale et charge verticale constante. La charge est une donnée prescrite
par l'hôte, non déduite de la vitesse, de la masse ou d'une coque inconnue.

## Conversion

Pour `p(r)=P0 exp(-r²/(2 sigma²))`, l'intégrale spatiale vaut `2 pi sigma² P0`.
Donc `P0=F/(2 pi sigma²)` avec F en newtons, vers le bas. Cette identité découle
de l'intégrale gaussienne d'ADR-070 ; elle n'est pas une calibration hydrodynamique.
F positif ou nul ; une aspiration demande un modèle distinct. Sigma reste celui
de la recette de pression WPRS. Le calcul f32 fixe l'ordre `(TAU*sigma)*sigma`,
puis la division ; infinis, sous-débordement à zéro d'une pression non nulle,
et surface non représentable sont refusés.

Wake::build produit jusqu'à64 segments sur stockage fixe, sans allocation (I-06).
Les dates s'additionnent en entiers contrôlés ; les positions avancent avec la même
arithmétique scale_integer que le contrôle WPRS. Une durée nulle, un endpoint hors
repère ou une date hors fenêtre sont refusés par Source::new, sans publication partielle.
La vitesse est relative à l'eau dans le milieu uniforme sans courant d'ADR-069.
Virages et changements de charge sont instantanés aux frontières des tronçons.

## Intégration et limites

Wake possède ses segments et expose une Source immuable : codec WPRS, admission,
contrôleur de pression et composition B+W existants sont réutilisés. L'identité est
celle de Metadata (objet/cause/époque). L'hôte authentifie avant admission ; aucun
client n'obtient une nouvelle autorité (I-10/I-11). La recette mouvement/charge
n'a pas de second codec : on transporte le forçage résolu WPRS.

Zéro charge ou fin du dernier tronçon coupe le forçage ; les ondes restent dans
le champ. Une vitesse nulle avec charge non nulle reste une pression, et son
apparition excite l'eau (ADR-069). Aucun anneau Impact créé artificiellement.

Ce premier adaptateur prend une trajectoire déclarée, pas un flux de poses moteur.
Pas de retour de résistance de vague sur le corps, coque calibrée, courant, changement
de repère ou Kelvin stationnaire reçu. W4 reste partiel. Les preuves multiplateformes
I-03 restent ouvertes ; I-02/I-07/I-08/I-09 inchangés. Réception et suite dans
TRAJET-SILLAGE-S150 ; prochaine étape : raccordement hôte, puis comparaison B2.
