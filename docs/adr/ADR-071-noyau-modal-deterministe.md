# ADR-071 — Candidat modal à horloge entière

- **Statut : ACTÉE**, S95, 2026-09-08, délégation technique.
- Complète ADR-069/070 ; conserve la référence physique f64 séparément.

## Décision

Construire `modal_pressure::ModalPressure`, préparation d'un mode et d'un segment en f32,
sans allocation ni appel transcendant à libm. Réutiliser PhaseQ32 et les opérations IEEE
strictes, sans contraction FMA. La racine carrée intervient à la préparation. Ceci constitue
un candidat au déterminisme ; la conformité entre plateformes reste à mesurer (I-03).

La solution de Duhamel d'ADR-069 reste inchangée. Les pulsations propre et Doppler sont
converties séparément en fréquences signées Q32 ; leur somme et différence sont entières.
Les produits fréquence × microsecondes utilisent i128, puis division et réduction modulo
un tour. Le temps absolu ne passe jamais en flottant ; la soustraction de naissance est entière.

La conversion de fréquence décompose le f32 en mantisse et exposant, multiplie par
`floor(2^64/(2*pi)) = 2935890503282001226`, puis décale pour obtenir Q32. La conversion
préalable par division f32 introduisait assez d'erreur pour échouer la réception à 16 s.
La constante définit aussi une approximation fixe de 1/(2*pi) ; la réception ne la confond
pas avec une conversion mathématique exacte pour toutes les entrées possibles.

Pour J(a,t), utiliser `exp(-iat/2) * 2 sin(at/2)/a` loin de zéro, et `t sinc(at/2)` près
de zéro. Dans cette seconde branche, la multiplication par la durée est une suite fixe
de doublements et additions du coefficient par microseconde, pilotée par les bits entiers
de la durée. Aucun temps f32 n'est introduit. Pour |at/2|<=pi/32, la série sinc jusqu'au
degré six omet un terme inférieur à 2,4e-14, hors arrondis. La phase bornée peut passer en f32.

Les phases négatives utilisent explicitement la parité sinus/cosinus avant évaluation.
Cela évite la perte relative sur les très petits angles négatifs dans la réduction existante.
Le module PhaseQ32 commun reste inchangé ; les références des autres couches sont conservées.

## Contrat et limites

Horizon déclaré depuis naissance, inclusif, limité à 16 secondes pour ce premier candidat ;
durée active strictement positive et inférieure ou égale à l'horizon. Cette limite est un
périmètre de travail à calibrer par réception, pas une durée physique ni un TTL.
Après extinction, la réponse continue librement jusqu'à l'horizon, puis refuse.
Avant naissance : contribution nulle. Débordement d'horloge refusé à la construction.

Origine et extrémité prescrite restent strictement dans ±4096 m par composante ; phase spatiale
par axe inférieure à 2^20 tours, borne technique de conversion, non certificat de précision.
Paramètres non finis, mode nul, fréquence non représentable et réponses non finies refusés.
Le constructeur vérifie la représentabilité ; il ne reçoit pas tous les paramètres admis.
L'addition de segments reste à recevoir pour ce candidat ; aucun codec, journal ou LiveWater
ne l'appelle. Pas de réception gameplay ni de changement aux invariants I-03/I-06/I-08.

## Mesures S95

550 réponses : cinq vecteurs k, dix rapports Doppler/pulsation dont ±1 et ±1±1e-6,
onze âges de 0 à 16 s dont 1 µs et les bords de l'extinction à 4 s. g vaut le même
f32 converti en f64 dans l'oracle ; pression 10 Pa, densité 1025, origine (0,7 ; -0,3).
Vecteurs : (0,0234375 ; 0), (1 ; 0), (0,6 ; 0,8), (6 ; 0), (9 ; 0).

| Mesure | Résultat |
|---|---:|
| Écart maximal élévation complexe contre S89 | 1,435702818e-7 m |
| Écart maximal vitesse complexe | 1,355482027e-6 m/s |
| J au seuil de changement de formule | 1,299419305e-8 s |
| Travail intégré / énergie finale, quatre vitesses | 9,172550330e-9 J/m² |
| Hash FNV-1a des 550 réponses, debug et release | 8ea15f4a3334830b |

Seuils de régression fixés avant mesure : 2e-7 m, 2e-6 m/s, 3e-7 s, 2e-6 J/m² ; à calibrer
hors fixture. Première tentative : 2,749e-7 m / 2,600e-6 m/s, refus maintenu puis conversion
corrigée. La parité corrige séparément le test à 1 µs et abaisse l'écart de J de 3,448e-7 s.
La translation d'époque jusqu'à u64::MAX conserve les bits pour tous les âges testés.
Réponse initiale comparée au développement q=-kP t²/(2rho), énergie libre conservée à 2e-6
relatif ; refus et témoins testés. Quatre tests S95 réussissent en release.

## Ce qui reste ouvert

S94-1 réalisée comme candidat local. **S95-1, S96 :** recevoir le découpage temporel et
construire la superposition gaussienne sur mémoire hôte avec ce noyau, en conservant
les interférences, les grandeurs S94 et la comparaison indépendante à la référence f64.
La transformation gaussienne, les phases spatiales, le coût, la réception multiprofils et
interplateforme restent à traiter avant intégration autoritaire. Aucun budget cible reçu.
