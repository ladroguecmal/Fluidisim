# S98 — Coût du chemin gaussien complet

2026-09-08. `bench_gaussian`, exécution release sur AMD Ryzen AI 7 350, Windows,
rustc 1.97.0. Mesure locale, sans budget cible certifié ni garantie de pire cas.

## Protocole

Recette V1 sigma=1 m, kmax=6 rad/m, 128×128 nœuds. Virage S91, deux segments de 2 s,
préparation à 3 s, échéance commune 8 s. Grille 11×11 sur [-8,12]² m ; requêtes sur
ses préfixes de 1,64,121 points. Le lot 64 n'est donc pas une grille 8×8 autonome.
Trois échauffements puis 21 mesures par opération, médiane et extrema observés.
`black_box` maintient entrées et sorties observables. Aucun hash, affichage ou oracle
dans le chronométrage des requêtes. La cuisson inclut son hash, produit de son API.

Les buffers sont alloués avant mesure. Les fonctions de préparation et de requête restent
sans allocation par inspection ; aucun compteur global n'a mesuré cette propriété.
La préparation reconstruit chaque couple mode/segment ; la cuisson n'est pas répétée
implicitement à chaque requête. Chaque point expose les sept composantes S94.

## Résultats

| Opération | Minimum µs | Médiane µs | Maximum observé µs |
|---|---:|---:|---:|
| Cuisson + hash de recette | 518,4 | 532,0 | 678,8 |
| Préparation, deux segments | 12188,7 | 12501,0 | 13522,0 |
| Requête 1 point | 599,9 | 621,5 | 1064,2 |
| Requêtes 64 points | 40785,9 | 44637,8 | 55774,5 |
| Requêtes 121 points | 81243,6 | 87066,5 | 133001,9 |

La requête domine. Un usage par image de cette implémentation à cette résolution est
inadapté à une simulation de jeu en temps réel. Ce constat porte sur le chemin mesuré,
pas sur une impossibilité du modèle. Le ratio des médianes 121/64 vaut environ 1,95,
proche du ratio de points 1,89 ; la dispersion des durées interdit une loi de coût précise.
Réduire la résolution sans nouvelle réception invaliderait la précision des S92/S94.

## Fidélité et mémoire

Comparaison hors chronométrage des 121 résultats à la référence f64 complète :
écart absolu maximal sur les sept composantes 2,285177e-8 ; seuil 1e-7 dans chaque unité.
Cette valeur agrégée mélange les unités : elle contrôle le seuil de régression mais ne
constitue pas une norme physique. Hash recette 20e64a392ae237a1 ; sorties :

- 1 point : 887e05db8c4568aa ;
- 64 points : 9bf74d6a25a9e32b ;
- 121 points : 47820ae52df0b569.

Tailles `size_of` constatées : Node 16 octets, Slot 36, Surface 28.

| Buffer | Octets |
|---|---:|
| Spectre, 16384 nœuds | 262144 |
| Un champ | 589824 |
| Deux champs, actif et candidat | 1179648 |
| 121 points | 968 |
| 121 sorties | 3388 |
| Deux segments | 80 |

Spectre + un champ + points + sorties + trajectoire = 856404 octets ; avec deux champs,
1446228 octets. Le banc alloue un champ ; le chiffre de deux champs est le calcul exact
de capacité nécessaire à deux pools identiques, pas une mesure d'usage mémoire du processus.
Exclus : métadonnées d'allocateur, pile temporaire, objets de vue, oracle, système et code.

## Suite

S97-1 réalisée. **S98-1, S99 :** exploiter la conjugaison k/-k du spectre réel pour
réduire les calculs, avec identité physique, comparaison de toutes les composantes,
réception des arrondis et nouveau coût. Les directions étant paires, la géométrie offre
un candidat naturel ; les bits cuits ne sont pas présumés exactement opposés sans vérification.
Avant tout changement de représentation/version, mesurer cette propriété et conserver la
référence complète. Aucun budget cible ni conformité interplateforme reçus en S98.

**Actualisation S99 — 2026-09-08 :** S98-1 réalisée,
[ADR-073](../adr/ADR-073-demi-spectre-conjugue.md), gain local ~52 % ; coût restant
21,488 ms pour 64 points. Suite S99-1 : préparation des coefficients constants.
