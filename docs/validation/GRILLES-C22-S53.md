# Admission des grilles C22 — S53, 2026-09-07

Action S52-1, suite de [S52](MESURES-PARTAGEES-S52.md). Base : 348f5f5.
Aucun solveur, seuil ou scénario modifié.

## Défauts reproduits

Trois tests échouent sur le code initial :

| Entrée | Résultat initial observé | Résultat corrigé |
|---|---|---|
| tailles 100,200,800,1600,3200 ; erreurs 1,½,¼,⅛,1/16 | stabilité Some(true), alors que le raffinement ne double pas entre 200 et 800 | triplets irréguliers indéterminés, stabilité None, aucun succès dans Bilan |
| C22 demandé avec taille zéro | arrêt sur modulo par zéro | famille invalide signalée avant toute allocation, tailles conservées avec mesures NaN |
| allocation de la grille 8 refusée une fois, après oracle et grille 4 | famille demandée 4,8,16,32,64 rendue sous la forme 4,16,32 après filtration | cinq tailles conservées, NaN à 8, mesures des grilles suivantes présentes, filtre suspendu |

La première suite isole le contrôle de support : ses erreurs seraient celles d'un ordre un
si les tailles doublaient. Le même témoin avec 100,200,400,800,1600 reste stable et donne p=1.
Ce test n'affirme pas qu'un solveur réel a produit ces nombres ; il montre ce que le
validateur pouvait accepter sans contrôler les tailles. A177.

## Corrections

- Convergence::ordre vérifie deux doublements successifs avant toute classification,
  y compris Plancher. ordre_grossier_estime applique le même contrôle.
  Les multiplications de tailles sont vérifiées ; un indice d'appel excessif est refusé.
- C22 delta valide la famille entière avant modulo/allocation : tailles positives,
  doublements, emboîtement dans un oracle strictement plus fin, taille d'allocation
  représentable et temps fini non négatif. Une famille invalide reste visible, sans calcul.
- Un oracle non alloué conserve les tailles demandées avec refus. Une grille non allouée
  conserve son emplacement NaN ; les autres mesures peuvent encore être calculées.
- Toute mesure non finie suspend la filtration. La famille conserve ses trous et ne peut
  pas obtenir une stabilité établie. L'affichage signale explicitement cette indisponibilité.
- Quand les mesures sont exploitables, le filtre ×30 garde seulement le préfixe sain.
  Une erreur repassant au-dessus du seuil après un rejet ne réintègre pas la famille.

Le calcul numérique d'ordre, son seuil, l'extrapolation de l'oracle et le choix du triplet
grossier restent inchangés. Les projections delta et shallow restent séparées : cette
session corrige leur admission, elle ne prétend pas uniformiser tous leurs contrats.

## Tests et portée

Quatre tests ciblés couvrent les trois reproductions, leurs témoins, les tailles nulles,
répétées, décroissantes ou débordantes, un oracle indisponible et un filtre avec un trou
suivi de valeurs admissibles. L'allocateur d'essai refuse un seul appel puis accepte les
suivants : il vérifie bien la conservation d'un trou intérieur, pas seulement une fin de série.

Le contrôle S46 conservait les triplets déjà indéterminés ; il ne pouvait pas restituer
une grille retirée en amont. S53 protège à la fois le montage et le calcul sur les tailles.
Le refus ne valide pas la physique et ne transforme pas une campagne absente en échec du
solveur : le bilan reste sans verdict. S52-1 est close.

**Validation finale :** 115 tests réussis (38 cœur + 77 harnais), deux ignorés.
Compilation release réussie. Rapport physics comparé à S52 : seules trois lignes de
durées diffèrent ; erreurs, ordres, familles et verdicts nominaux inchangés. Sortie 1
attendue pour C04 ordre un. check : zéro échec, hashs 0x3e2c06a7b00e73e3 et
0x1a8b0629a9f51b6e inchangés. Campagne coûteuse shallow non répétée.
