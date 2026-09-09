# S107 — Scénario hôte du virage B+pression

2026-09-09. `host_pressure`, exécutable release avec assertions. Bibliothèque inchangée.

## Montage et réception

Virage S103/S104 : gaussienne σ1 m, coupure6/m,10 Pa, vitesse(2,0) pendant2 s puis
(0,2) pendant2 s, départ(0,0), arrêt à4 s, observation jusqu'à8 s. Densité1025,
g9,81f32 ; contexte frame7/cell9 et rectangle[-8,12]². Fond configuré via les services
hôte : Hs0,1 m, Tp6 s, direction0,125 tour,16 composantes, graine42. Ancre entière
à(10⁹,-10⁹,0) m. Ces choix définissent une fixture, pas un profil de jeu universel.

64 premiers points de la grille11² de S103, pas2 m (pas un carré8×8). Neuf dates :
0,1,1,999999,2,2,000001,3,4,6,8 s.576 points-temps par résolution. Plafond de pente
0,1 pour ce test, comme les fixtures de composition précédentes ; aucune calibration
universelle. Enveloppe maximale observée0,0280734, donc le témoin nominal est accepté.

La référence compose B évalué directement et la réponse gaussienne f64 à la même
résolution, en recalculant la normale en f64. Elle reçoit le raccordement et les
arrondis, pas une borne vers le continuum ni une validation indépendante de B.
Comparaison explicite des valeurs finies, tolérance absolue2e-7 pour les huit valeurs
élévation, dérivée temporelle, trois vitesses et trois composantes de normale.
La normale est sans unité ; élévation en m, dérivée et vitesses en m/s. Cette tolérance
est celle de cette comparaison de composition ; elle ne remplace pas celles de S103.

| Résolution | Max élévation m | Max dérivée m/s | Max vitesse m/s | Max composante normale |
|---|---:|---:|---:|---:|
|128²|1,302e-8|1,329e-8|1,995e-8|1,148e-7|
|112×80|4,747e-9|9,462e-9|9,462e-9|1,147e-7|

L'aération est identique à B. La cambrure rapporte l'enveloppe S106, elle n'est pas
comparée à une pente ponctuelle de référence. Les bilans restent ceux de la pression.
À chaque date : dernier point monde invalide, puis date décalée de1 µs, sorties
inchangées. Hors t0, pression1e30 provoquant un candidat refusé ; la publication active
reste interrogeable. Reconstruction valide dans le pool refusé puis nouvelle requête :
identité des dix champs de sortie vérifiée par hash. Àt0 l'énergie reste nulle, donc
cet essai de débordement n'est pas un témoin de refus et n'est pas revendiqué.

Hashes à3 s des64 sorties :128² `52c1b810097014e8`,112×80 `70561fb44b0a4add`.
Les neuf hashes sont imprimés par l'exécutable ; ils ne certifient pas deux plateformes.

## Coût mesuré

Windows, AMD Ryzen AI7 350, rustc1.97.0 (2d8144b78), cargo1.97.0, release.
Trois échauffements,21 mesures par opération ; séries successives, aucune campagne
concurrente lancée par cette session. Allocation des pools, cuisson et oracles hors
chronométrie. Instant mesuré3 s ; B16 composantes,64 points. Valeurs en millisecondes.

| Résolution / opération | Min | Médiane | Max |
|---|---:|---:|---:|
|128² préparation|6,6732|6,9898|7,9831|
|128² requête B+pression|20,3250|20,9700|23,3897|
|128² cycle complet|27,3164|29,3672|32,6399|
|112×80 préparation|3,5734|3,8079|4,8559|
|112×80 requête B+pression|11,8431|12,2401|14,1536|
|112×80 cycle complet|15,6257|16,2846|17,3818|

Le cycle complet est chronométré directement, pas obtenu en additionnant deux médianes.
Il reconstruit la vue et interroge le lot ; il ne mesure pas une arrivée réseau, un
contrôleur de commandes, le rendu ou un stockage durable. Coût important et aucun
budget cible certifié. Ne pas extrapoler à un nombre quelconque de sources ou points.

| Mémoire utile octets |128²|112×80|
|---|---:|---:|
|Nœuds complets|262144|143360|
|Demi-spectre|131072|71680|
|Deux pools de champs|655360|358400|
|64 points monde|1536|1536|
|Scratch et sortie|5120|5120|
|Composantes B|512|512|

Métadonnées, trajectoire, pile, allocateur, code et oracle exclus. Les trois tables
spectrales et deux pools sont conservés dans ce montage. Le compteur des services hôte
ne relève aucun appel refusé après scellement ; il ne voit pas les allocations globales
Rust de l'instrument. Absence d'allocation du chemin mesuré constatée par inspection,
pas par un compteur global. Le programme alloue son instrumentation hors mesures.

## État et suite

Scénario exécuté avec toutes ses assertions, formatage/diff vérifiés. Suite225 réussis,
cinq ignorés inchangée, non relancée : aucun code de bibliothèque modifié.
73 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés. S106-1 réalisée.

**S107-1, S108 :** définir et construire une source de pression immuable versionnée,
trajectoire et contexte explicites, codec et refus des entrées invalides/tronquées.
Préserver les formats existants des impacts ; qualifier sa future admission dans le
journal et les sauvegardes avant de déclarer une persistance ou autorité du sillage.
La construction des causes persistantes est prioritaire à une nouvelle micro-optimisation.

**Mise à jour S108, 2026-09-09 :** S107-1 réalisée ; [ADR-074](../adr/ADR-074-source-de-pression-versionnee.md),
source et codec WPRS V1. Suite S108-1 : admission bornée et conflits, avant sauvegarde.
