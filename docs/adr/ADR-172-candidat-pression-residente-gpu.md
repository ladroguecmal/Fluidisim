# ADR-172 — candidat de pression résidente GPU

Actée S288, 2026-09-19, autonomie technique S71. Construction expérimentale, sans activation
du solveur graphique dans le chemin image. Applique ADR-020/130, I-04/I-06/I-13/I-17.

## Décision

Le candidat δ graphique vit dans `viewer/`, avec la pile wgpu déjà autorisée. Le cœur conserve
sa référence CPU et n'importe aucune dépendance graphique. Il peut écrire dans un tampon
fourni les coefficients de son opérateur mobile à géométrie figée : ouvertures de faces et
facteurs de fantômes, ordre gauche/droite/bas/haut. Ce produit est une entrée de calcul,
pas une surface publiée au rendu, pas une sauvegarde de δ. L'hôte le transmet au calcul GPU.

Les itérations travaillent sur des tampons GPU réservés, alternés entre lecture et écriture.
Aucun rapatriement par itération. Le premier lot construit l'application de l'opérateur et
le lissage Jacobi amorti 4/5 d'ADR-167. Il ne remplace ni le gradient conjugué ni la multigrille.
L'export est invalide dès que la géométrie change : la future intégration doit le renouveler
pour chaque projection, y compris l'affinage, et préserver le contrat de pression fantôme.

L'activation demande ensuite : cycle et réductions, mêmes portes d'acceptation ADR-143/144,
refus sans publication partielle, mémoire réservée, mesure du pas complet et de la trajectoire
contre CPU, puis réception visuelle de toute différence. Une erreur de port faible sur un
opérateur n'est ni la réception physique du solveur ni celle du budget eau de 2 ms.

## Pourquoi cet ordre

S286 refuse la cadence lente sur fidélité et pics. S287 ne reçoit pas le gain du parcours
mémoire CPU. L'hôte possède déjà compute et horodatage GPU ; déplacer une seule passe avec
aller-retour à chaque appel ne constitue pas un solveur résident. Le premier consommateur
est donc le banc exécutant plusieurs lissages sans retour CPU, puis le futur cycle de pression.

δ demeure cosmétique et non répliqué ; gameplay et V restent CPU. Aucune exigence d'identité
inter-GPU n'est introduite. Le coût inclut les transferts et synchronisations ; l'horodatage
GPU seul est publié distinctement. Les allocations de l'hôte/de sa pile sont comptées selon
ADR-145 ; le banc n'est pas une preuve I-06 du futur chemin image.
