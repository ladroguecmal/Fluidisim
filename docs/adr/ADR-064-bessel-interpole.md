# ADR-064 — Bessel interpolé avec réception séparée de l’erreur

- **Statut : ACTÉE**, S82, 2026-09-08, délégation technique.
- **Remplace** le calcul Bessel de production d’ADR-060 ; référence angulaire conservée.
- **Décision** : interpolation cubique Hermite de J0/J1 sur [0,64], pas h=1/16.

## Construction et portée de la borne

1025 nœuds : J0, J1 et J1’ ; J0’=-J1. Le générateur Python fournit des bits f32 depuis une
quadrature angulaire f64 à 2048 directions. J1’=J0-J1/x, et J1’(0)=1/2. Le calcul d’exécution
n’emploie que la table, les opérations f32 et un index ; mêmes refus hors [0,64] et non-finis.
La table contient 12300 octets ; directions S81 conservées pour le chemin de référence.
Générateur : `python code/water-core/examples/generate_bessel.py`. Le fichier versionné de bits
fait foi ; les bibliothèques mathématiques du générateur ne sont pas une dépendance d’exécution.

La représentation angulaire borne en valeur absolue les dérivées quatrièmes de J0 et J1 par 1.
Pour des données nodales exactes, le reste Hermite est borné par h^4/384, soit environ 3,974e-8.
**Cette borne exclut les erreurs des valeurs tabulées et les arrondis f32.** Ne pas la présenter
comme une preuve uniforme du code complet. La réception numérique ci-dessous mesure ces effets ;
une borne globale formelle du générateur et des flottants reste distincte.

## Réception

Commande : `cargo test --release --manifest-path code/Cargo.toml -p water-core radial_impact -- --nocapture`.
8193 arguments uniformes sur [0,64], soit huit sous-intervalles par maille, comparés à la
quadrature indépendante f64 à 4096 directions décalées : écart maximal J0/J1 **5,856990026e-8**.
Écart maximal au calcul angulaire f32 précédent **8,353963494e-7**. Tolérance annoncée 2e-7,
non modifiée. Cette grille teste les nœuds et l’intérieur ; pas chaque flottant du domaine.

Les huit tests radiaux passent : petits arguments, zéros, symétrie, raffinement spectral,
énergie initiale, vitesse par différences et bilan temporel. Tolérances S77/S78/S79 inchangées.
À 4 s, R20/N128/512 anneaux : énergie/prescription 1,00000154, rayon moyen 4,456888 m.
La dérivée de l’interpolant J0 n’est pas identiquement -l’interpolant J1 entre nœuds ; les
contrôles de vitesse/énergie vérifient le résidu sur leur scénario, pas toutes les conditions.
Le domaine physique reçu n’est pas élargi. Aucun nouveau résultat B2 ou interplateforme.

## Coût local

Même banc bench_water, Ryzen AI 7 350, Windows, rustc 1.97.0 release, trois échauffements,
21 mesures. Runs successifs non isolés : comparaison locale, pas garantie de percentile cible.

| Sources × points | Médiane S81 (ms) | Médiane S82 (ms) | p90 S82 (ms) | Max S82 (ms) |
|---|---|---|---|---|
| 1×16 | 2,3146 | 0,0387 | 0,0388 | 0,0391 |
| 1×64 | 9,9559 | 0,1553 | 0,1719 | 0,2056 |
| 4×64 | 40,2427 | 0,4406 | 0,4869 | 0,6426 |
| 16×64 | 159,1579 | 1,5978 | 1,8130 | 2,0897 |

Microbanc 1024 Bessel : 2949,4→9,4 µs médian. Préparation : 0,6/2,5/9,7 µs pour 1/4/16 sources.
B réel est inclus dans les lots ; les tailles des pools et tampons de S81 restent inchangées.

Les résultats ne sont plus identiques bit à bit à S81 : changement numérique explicite, pas
simple tabulation de directions. Nouveaux hashs locaux des lots : 924abd0a6bc0230a,
e65377858cf5ad90, 2d41b046bb55edaf, ae9185ea76cb1b12 dans l’ordre du tableau.
Les références B seules C02/C18 restent inchangées. Une migration réseau d’un W déjà déployé
exigerait de versionner la reconstruction ; aucun déploiement de ce candidat n’a été constaté.

## Suite

S81-1 réalisée : candidat reçu sur les contrôles existants et adopté. Arrêter ici cette
optimisation jusqu’à nouvelle charge mesurée. Prochaine construction S83 : lier les résultats B
à leurs point/temps/contexte dans l’API de lot, pour supprimer la précondition muette qui permet
aujourd’hui d’assembler deux eaux évaluées à des instants différents. Puis étendre la couverture
physique et l’intégration, sans certifier un budget AAA depuis ce microbanc. Rétention S72-2,
index spatial et publication multilecteur restent ouverts. Aucun invariant amendé.
