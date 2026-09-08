# ADR-073 — Réduction contrôlée du spectre conjugué

- **Statut : ACTÉE**, S99, 2026-09-08, délégation technique.
- Complète ADR-070/072 ; recette V1 complète et chemin complet conservés.

## Décision

Proposer `Spectrum::half_into` pour retenir une direction sur chaque paire k/-k avec
poids doublé. Le forçage réel et la réponse linéaire imposent q(-k)=conj(q(k)) et la même
relation sur q_dot. La partie réelle de chacune des deux contributions spatiales est
identique, y compris ses gradients ; leur énergie spectrale est également identique.
Cette réduction ne retire aucune direction physique du modèle reçu.

Avant écriture, vérifier dans chaque anneau que les vecteurs sont exactement opposés,
avec poids et transformées identiques en bits. Sinon refuser `NotConjugate`, sans correction
géométrique implicite. Capacité insuffisante également refusée avant écriture ; queue intacte.
La vue `HalfSpectrum` conserve le hash du spectre source comme provenance. Ce hash désigne
la source complète, **pas les octets du demi-spectre** ; aucune sérialisation n'est introduite.

La recette V1 complète ne change pas de bits ni de version. La réduction est une opération
explicite supplémentaire. Elle peut servir aux préparations S96 sans changer leur formule.
Le changement d'ordre et les arrondis de phase font que les sorties ne sont pas identiques
en bits : l'adoption est fondée sur réception d'erreur, pas sur égalité bit à bit avec le
chemin complet. Un futur codec devra identifier la représentation utilisée.

## Réception

Contrôle des paires sur toutes les 255 valeurs paires de directions de 4 à 512, un anneau ;
contrôle supplémentaire implicite des 128 anneaux de la recette nominale. Tous admis.
Une direction volontairement déplacée est refusée, sortie inchangée ; capacité et queue testées.

Campagne du virage S96 réutilisée avec 8192 nœuds : 1089 points sur [-8,12]² m et neuf
instants 0–8 s, comparaison directe à la référence f64 complète. Erreurs maximales :

| eta m | w m/s | phi m²/s | pente | u m/s |
|---:|---:|---:|---:|---:|
| 1,255e-8 | 1,305e-8 | 7,003e-9 | 4,815e-9 | 2,010e-8 |

Seuils inchangés : 1e-7 par composante, 2e-6 J pour énergie. Découpage rectiligne reçu
aux mêmes seuils. Ceci reçoit la fixture, pas tous les paramètres admis ni toutes les plateformes.

## Coût S99

Même machine et protocole S98, deux exécutions release rapprochées : demi-spectre puis
spectre complet. Trois échauffements, 21 mesures, valeurs médianes en microsecondes.

| Opération | Complet | Demi-spectre |
|---|---:|---:|
| Cuisson complète | 539,8 | 527,3 |
| Réduction contrôlée | — | 21,7 |
| Préparation de deux segments | 12883,8 | 6327,2 |
| Requête 1 point | 735,0 | 304,3 |
| Requêtes 64 points | 44798,1 | 21487,7 |
| Requêtes 121 points | 87131,1 | 41889,6 |

Gain médian environ 51 % en préparation et 52 % sur 64/121 points. Dispersion locale
et exécutions non entrelacées : aucune garantie de gain ni de pire cas. Le coût reste élevé.
Sorties 121 points à 3 s : hash complet 47820ae52df0b569, demi f1d889f97488bc37.
Le banc compare indépendamment à la référence, maxima 2,286e-8 / 5,478e-9 respectivement.

Un champ passe de 589824 à 294912 octets. Le spectre complet reste alloué dans le banc
(262144 octets), avec un demi-spectre supplémentaire de 131072 octets. Deux champs réduits
coûtent 589824 octets. Points/sorties/trajectoire inchangés, 4436 octets en tout.
Total buffers du banc avec un champ réduit : 692564 octets ; avec deux, 987476 octets.
Ces totaux incluent le spectre complet conservé ; pas de réduction mémoire fictive en l'omettant.
Métadonnées d'allocateur, pile, oracle et système exclus ; absence d'allocation par inspection.

## Suite

S98-1 réalisée. **S99-1, S100 :** déplacer hors de la boucle par point les coefficients
constants et contrôles qui peuvent être reçus à la préparation, puis comparer erreurs et coût.
Conserver les refus de domaine et de non-finis ; éviter une optimisation qui supprime le
contrat de requête. Batching plus profond et coût asymptotique restent ensuite à examiner.
Pas de conformité interplateforme ni budget cible certifiés ; LiveWater reste distinct.
