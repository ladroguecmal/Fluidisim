# S96 — Superposition sur pool hôte

2026-09-08. Application des ADR-070/071 ; aucun changement de modèle physique.

`spectral_pressure` reçoit un tableau de nœuds (vecteur k, transformée de pression,
poids de quadrature), une trajectoire et un pool candidat. Il prépare la réponse totale
complexe avant de calculer son énergie, puis expose une vue empruntée bornée. Pente,
potentiel et vitesses suivent les formules S94. Aucun Vec ni libm dans ce chemin.
La préparation recalcule actuellement le noyau de chaque couple nœud/segment : coût à mesurer.

La cuisson du spectre est explicitement extérieure. Le test cuit en f64/libm les nœuds
gaussiens puis les convertit en f32 ; **ceci ne rend pas la cuisson déterministe**.
Les octets du spectre devront être identiques entre participants ; leur production,
format et provenance restent à construire. L'API accepte un spectre général fourni,
sans prétendre vérifier qu'il provient d'une gaussienne.

## Contrats

Trajectoire non vide, contiguïté des dates et jonctions spatiales exactes selon la
multiplication entière de S95. Instant interrogé compris entre première naissance et
échéance commune ; chaque segment doit tenir dans son horizon modal, au plus 16 s.
Bornes spatiales inclusives, strictement dans ±4096 m par composante.
Capacité refusée avant écriture ; autres erreurs peuvent modifier le pool candidat,
mais ne retournent aucune vue. Un pool actif séparé reste utilisable.
Les calculs non finis refusent la publication ou l'échantillon entier.

La somme d'énergie utilise une compensation en f32, dans l'ordre des nœuds. La première
tentative avec somme naïve échouait au seuil 2e-6 J contre la référence ; la compensation
fait passer ce même seuil. Les champs gardent leur sommation dans l'ordre fourni.

## Réception

Deux tests S96 en release : virage puis découpage rectiligne, et refus avec témoin de reprise.
Spectre 128×128, coupure 6 rad/m, sigma 1 m, pression 10 Pa, densité 1025 et g=9,81 f32
converti identiquement dans la référence f64. Virage S91 à 2 s ; neuf instants 0–8 s,
grille 11×11 sur [-8,12]² : 1089 comparaisons à la référence gaussienne complète.

| Grandeur | Écart maximal absolu |
|---|---:|
| Élévation | 8,079e-9 m |
| Vitesse verticale | 3,941e-8 m/s |
| Potentiel | 6,100e-8 m²/s |
| Pente, par composante | 9,963e-9 |
| Vitesse horizontale, par composante | 2,976e-8 m/s |

Seuil de régression fixé avant mesure : 1e-7 dans chaque unité, 2e-6 J sur énergie.
Ces valeurs sont à calibrer hors fixture ; elles ne bornent pas tous les paramètres admis.
Découpage 4 s en deux segments de 2 s vérifié à 2/4/8 s et trois points, mêmes seuils
sur toutes les grandeurs et énergie. Le virage conserve les interférences par construction.
Refus : pool trop court, NaN de pression, trou temporel, point hors bornes et NaN ;
champ actif inchangé et nouvelle préparation valide acceptée ensuite.

## Suite

S95-1 réalisée pour ce chemin sur spectre fourni. **S96-1, S97 :** produire un spectre
gaussien reproductible sur mémoire hôte, fixer son contrat de paramètres et sa provenance,
puis recevoir identité et erreur sans cuisson libm implicite. Ensuite mesurer les coûts
et raccorder la composition. Conformité interplateforme, codec de trajectoire, LiveWater,
puissance du champ candidat et bilan travail/énergie candidat restent ouverts.
