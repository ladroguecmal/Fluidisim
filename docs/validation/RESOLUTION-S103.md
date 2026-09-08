# S103 — Résolution reçue sur l'emprise du virage

2026-09-08. S102-1 réalisée par campagne, sans changement des valeurs par défaut.

## Critères et méthode

Même pression gaussienne sigma=1 m, coupure=6 rad/m, 10 Pa, g=9,81 f32 et rho=1025,
virage S91 à 2 s, extinction à 4 s. Rectangle [-8,12]² et période 0–8 s inchangés.
Chaque candidat est cuit en f32, réduit par conjugaison, préparé avec le noyau candidat.
Comparaison indépendante à deux références gaussiennes f64 complètes : 128² puis 256².
Même valeur de gravité f32 convertie dans les références pour isoler la résolution.

Seuils de résolution déjà employés en S92/S94 : eta 1e-6 m, vitesse verticale 1e-5 m/s,
potentiel 1e-5 m²/s, pente 1e-6, vitesse horizontale 1e-5 m/s, énergie 1e-5 J.
Ils sont distincts du seuil 1e-7 des comparaisons f32/f64 à résolution identique en S96.
Les seuils ne sont ni relâchés ni revendiqués universels. La puissance candidate n'est
pas exposée, donc non reçue ici ; elle reste une action ouverte.

`receive_resolution` examine 14 résolutions sur 121 points et neuf instants : radial
32/64/96/112/128 à angular128 ; angular16/24/32/48/64/80 à radial128 ; combinaisons
96×32, 96×48 et 112×80. Les refus sont affichés comme résultats de campagne.
`--dense` reçoit ensuite 128² et 112×80 sur 441 points (pas 1 m), 19 instants
(pas 0,5 s plus 2 s ±1 µs), soit 8379 échantillons par candidat et référence.
Le mode dense exige le respect de tous les critères par assertion.

## Ce que les refus apprennent

- 32 rayons : élévation jusqu'à 4,326e-5 m, énergie 4,495e-4 J contre 256², refus.
- 64 rayons : potentiel 6,006e-5 m²/s, énergie 3,422e-5 J, refus.
- 96 rayons : hauteur 9,534e-7 m et énergie 6,843e-6 J passent, mais potentiel
  1,635e-5 m²/s échoue. Une réception limitée à la hauteur aurait accepté ce candidat.
- 64 directions à radial128 : hauteur 8,210e-7 m passe ; pente 5,063e-6 et vitesse
  verticale 1,008e-5 m/s échouent. Les nombres de directions inférieurs échouent aussi.
- 112×128 et 128×80 passent séparément, puis leur combinaison 112×80 passe.

## Candidat 112×80, campagne densifiée

| Grandeur | Écart maximal /128² | Écart maximal /256² |
|---|---:|---:|
| eta m | 2,272e-7 | 6,100e-7 |
| w m/s | 2,083e-7 | 3,082e-7 |
| phi m²/s | 3,917e-6 | 8,534e-6 |
| pente | 1,107e-7 | 1,107e-7 |
| u m/s | 1,754e-7 | 1,791e-7 |
| énergie J | 1,564e-6 | 3,655e-6 |

La marge la plus faible concerne le potentiel : environ 14,7 % contre le seuil et
l'oracle 256². Ce n'est pas une marge contre la solution continue : cet oracle possède
sa propre erreur. Pas de borne continue entre échantillons, réception multiprofils ou
preuve de minimalité de 112×80. Aucun choix automatique de résolution n'est introduit.

## Coût local

`bench_resolution` compare les deux recettes, même machine S98, trois échauffements
puis 21 mesures release. Préparation à 3 s, lot des 64 premiers points de la grille S98.
Séries successives, non entrelacées : indicateur local, pas garantie de performance.

| Recette | Modes après conjugaison | Préparation médiane µs | Lot64 médian µs |
|---|---:|---:|---:|
| 128×128 | 8192 | 6293,6 | 19732,1 |
| 112×80 | 4480 | 3353,5 | 11305,4 |

Gain observé 46,7 % en préparation, 42,7 % en requête. La bibliothèque et le banc historique
conservent leurs valeurs par défaut ; le candidat n'est utilisé que dans ces campagnes.
Recette nominale candidate V1 : hash 045e84b6b9ef7249. Spectre complet 143360 octets,
demi-spectre 71680, champ 179200 ; mémoire des points/scratch/sorties en supplément.
Aucune allocation dans les opérations mesurées par inspection ; pas de compteur global.

## Suite

**S103-1, S104 :** recevoir la puissance et le bilan travail/énergie du champ candidat,
sur virage et extinction, aux résolutions 128² et 112×80. Une accélération des requêtes
ne ferme pas cette lacune physique. Garder 112×80 comme candidat de fixture ; toute autre
emprise, trajectoire ou pression demande sa réception propre. Intégration autoritaire,
codec et conformité interplateforme restent ouverts. 73 ADR et 193 angles inchangés.

**Mise à jour S104, 2026-09-08 :** S103-1 réalisée sur fixture ; voir
[Puissance candidate](PUISSANCE-S104.md). La préparation calcule désormais aussi la
puissance ; les mesures ci-dessus décrivent S103. Suite S104-1 : contexte et publication.
