# S102 — Sinus/cosinus à réduction commune

2026-09-08. S101-1 réalisée ; ni nouveau modèle ni changement de précision.

PhaseQ32::sin_cos calcule une fois le quadrant, l'angle dans le quadrant et sa réduction
au premier octant, puis évalue les deux polynômes existants. Les signes et permutations
reproduisent sin() et cos() séparés, y compris les zéros signés. Le décalage d'un quart
de tour utilisé par cos() conserve les trente bits internes : l'angle réduit est identique.
Seule l'interrogation spectrale utilise cette nouvelle méthode ; les autres appels restent
inchangés. Aucune allocation ni table supplémentaire, taille Slot inchangée.

## Identité

Test bit à bit sur un million de phases issues d'une récurrence entière fixe et les
257 offsets de chaque côté des huit frontières d'octant. Aucune différence constatée
en release. Le banc isolé vérifie également 65536 paires ; les hashes du champ complet
restent b435322317c15b62 (1 point), ceaa83d65bd3a69b (64), f1d889f97488bc37 (121).
L'oracle gaussien f64 conserve son écart maximal 5,478e-9. Les tests d'identité comparent
l'implémentation précédente : ils ne constituent pas une nouvelle réception physique
indépendante ni une conformité interplateforme.

## Mesures

Même machine S98, release, trois échauffements et 21 mesures, données et sorties allouées
avant chronométrage. Phases pseudo-aléatoires fixes ; positions sur grille régulière pour
la conversion spatiale. Les ensembles diffèrent : **les coûts ne s'additionnent pas pour
prédire le champ complet**. Médianes pour 65536 opérations :

| Opération | µs |
|---|---:|
| Phase spatiale, deux axes | 453,4 |
| Sinus puis cosinus séparés | 1839,7 |
| Couple conjoint | 1578,1 |

Gain isolé observé 14,2 %. Dans le banc complet, demi-spectre et virage à 3 s :
préparation 6014,6 µs ; scalaire 64 points 21727,2 µs ; lot 64 points 20696,2 µs ;
scalaire 121 points 39702,4 µs ; lot 121 points 40174,4 µs. Aucun gain global net établi
par rapport à S101, dont le lot64 valait 20050 µs. Bruit de mesure, phases et organisation
différents empêchent d'extrapoler le gain isolé. Aucun budget cible certifié.

La méthode conjointe est conservée pour partager explicitement le travail, avec identité
reçue et sans coût mémoire. Le coût de requête reste trop élevé dans ce scénario.

## Suite

**S102-1, S103 :** recevoir une stratégie de résolution adaptée au domaine effectivement
interrogé, par raffinements radiaux et angulaires indépendants, avant toute baisse du nombre
de modes. Mesurer erreur sur hauteur, pente, potentiel et vitesses, pas seulement énergie.
Conserver la référence 128² et son emprise ; ne pas réduire arbitrairement sa résolution.
Puissance/travail candidat, codec, intégration et conformité interplateforme restent ouverts.

**Actualisation S103 :** S102-1 réalisée comme campagne, [RESOLUTION-S103](RESOLUTION-S103.md). Candidat112×80 reçu sur fixture ; suite S103-1 : puissance et bilan candidat.
