# ADR-148 — Filtrer les amplitudes de l'image selon le pas projeté

- Actée S249, 2026-09-16 ; autonomie technique S71 ; A282.
- Applique ADR-130 (image cosmétique), ADR-131 et I-09. Aucun invariant amendé.

Le maillage projeté sous-échantillonne les modes lointains (S247/S248). Le filtre
appartient à l'hôte : le cœur, les événements, la recette et son domaine ADR-132 restent
inchangés. Aucun nœud de quadrature n'est décimé ; les phases ne sont pas interpolées.

Pour chaque sommet, `h` est la plus longue distance horizontale aux huit voisins de
grille, bornés aux bords. Les diagonales couvrent aussi les arêtes triangulées. Pour
un nombre d'onde majorant `k`, poser `r = k h / π`. Le poids vaut 1 pour `r ≤ 1/2`,
0 pour `r ≥ 1`, et `1 - 3t² + 2t³` entre les deux, `t = 2r - 1`.
Nyquist donne la borne dure ; une octave de transition est un **choix de filtre**,
non une constante physique ni une calibration perceptive. Il est C¹ aux deux bornes.

B applique ce poids à chaque amplitude. Le sillage est cuit en huit bandes dyadiques,
de bornes supérieures `k_max / 2^b` ; la dernière reçoit tout le reliquat inférieur.
Chaque bande garde les coefficients, phases et dérivées d'origine. Son poids utilise
sa borne supérieure : conservateur, parfois plus dissipatif qu'un filtre par mode.
Une neuvième grille conserve la somme complète pour le chemin proche et le témoin.
Les bandes sont des groupes de la **même réalisation**, jamais deux mers aléatoires
mélangées. La reconstruction linéaire par rapport aux coefficients commute avec
leur pondération locale. Stockage réservé à l'initialisation (I-06).

La hauteur et la pente modale sont filtrées séparément avec les mêmes poids.
La normale cosmétique n'inclut pas la dérivée spatiale du filtre de caméra : elle
n'est donc pas exactement la normale géométrique de la hauteur filtrée dans la
transition. Cette limite doit rester explicite, et ne touche aucune force de jeu.

La réception distingue l'erreur numérique au champ filtré (3 mm S201) de la quantité
volontairement retirée au champ complet. Le chemin sans filtre reste disponible.
Les impacts tabulés restent non filtrés : A282 n'est pas une réception de tout W.

Preuve et coût : [COUPURE-S249](../validation/COUPURE-S249.md).
