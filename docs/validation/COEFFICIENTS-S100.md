# S100 — Coefficients constants préparés

2026-09-08. Réalise S99-1 sans changer de modèle, recette ou représentation spectrale.

## Changement

Slot conserve désormais k/(2*pi), poids*k et poids au lieu du Node entier. Les opérations
sont les mêmes, avec les mêmes parenthèses, simplement effectuées à la préparation.
La transformée de pression du Node n'est plus conservée dans chaque Slot après usage.
La division dépendante du point pour le potentiel reste en place : la déplacer changerait
l'ordre des opérations. L'ordre de sommation demeure celui de S99.

Pour chaque axe, la préparation calcule la borne
`abs(k_turns) * max(abs(min),abs(max))` dans la même arithmétique f32 que la requête.
Si toutes ces bornes sont finies et strictement sous 2^20 tours, les contrôles de phase
par mode sont inutiles à l'intérieur du rectangle et sont sautés. La monotonie de la
multiplication positive arrondie garantit la borne pour les coordonnées admises.

Si la borne ne passe pas, **le champ reste préparable** et conserve les anciens contrôles
par point. Un rectangle partiellement représentable n'est donc pas refusé en bloc.
Les contrôles d'entrée (finitude et rectangle) et de sortie non finie restent obligatoires.
Une seule condition par champ sélectionne le chemin ; aucun invariant n'est amendé.

## Vérification

Deux tests S100 : hash des sept composantes sur 121 points du virage à 3 s, et garde-fous.
Hash demi-spectre conservé : f1d889f97488bc37. Le banc conserve également les hashes des
préfixes 1 et 64 points : b435322317c15b62 et ceaa83d65bd3a69b.
L'écart maximal du banc contre la référence reste 5,477796e-9, sous les seuils antérieurs.

Test de garde : mode k=(10^6,0), rectangle [-8,12]², pression nulle. Origine acceptée,
point (12,0) refusé ; le même mode sur [-1,1]² reçoit la borne et accepte les coins.
NaN et sortie du rectangle refusés dans tous les cas. Une réponse interne non finie
est explicitement refusée même avec phase sûre. Le témoin nominal empêche un refus universel.

## Coût et compromis

Protocole S98, demi-spectre S99, même machine, trois échauffements et 21 mesures release.

| Opération | Minimum µs | Médiane µs | Maximum observé µs |
|---|---:|---:|---:|
| Préparation | 6069,7 | 6470,7 | 8186,9 |
| Requête 1 point | 259,0 | 326,5 | 554,2 |
| Requêtes 64 points | 19126,5 | 19799,8 | 23091,3 |
| Requêtes 121 points | 38847,2 | 40547,7 | 42836,8 |

Par rapport aux médianes S99 : -7,9 % pour 64 points, -3,2 % pour 121, +7,3 % pour un
point et +2,3 % en préparation. Sessions distinctes, mesures bruitées et non entrelacées :
ce résultat ne certifie pas un gain général. Le déplacement est conservé pour supprimer
le travail invariant dans la boucle, avec identité locale contrôlée et coût mémoire explicite.
Il ne résout pas le coût dominant, encore proche de 20 ms pour 64 points.

Slot passe de 36 à 40 octets : champ de 8192 modes = 327680 octets, deux = 655360.
Avec spectres complet/réduit conservés et points/sorties/trajectoire du banc :
725332 octets pour un champ, 1053012 pour deux. Hors métadonnées, pile, oracle et système.
L'augmentation est de 32768 octets par champ ; aucune allocation nouvelle dans les requêtes.

## Suite

**S100-1, S101 :** construire une interrogation par lot qui réutilise les données modales
sur plusieurs points, sans changer les sommes par point ; comparer une référence scalaire
et mesurer le coût avant adoption. Conserver les refus et empêcher la publication d'un lot
partiel. Puissance/travail candidat, codec, LiveWater et conformité interplateforme restent ouverts.

**Actualisation S101 :** S100-1 close, [LOTS-S101](LOTS-S101.md) : lot atomique construit, tuiles rejetées après mesure. Suite S101-1 : phase et sinus/cosinus.
