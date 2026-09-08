# ADR-063 — Préparer une fois, publier le lot seulement après succès

- **Statut : ACTÉE**, S80, 2026-09-08, délégation technique.
- **Produit** `prepared_water.rs`, complète ADR-062 et S79-1.
- **Portée** : un contexte local, champs radiaux confirmés, échantillons B déjà évalués par l’hôte.

## Préparation

Prepared::build prend le journal par emprunt immuable et une tranche hôte de
Option<RadialImpact<N>> par emprunt exclusif. Il refuse une perte connue, puis une capacité
insuffisante avant de toucher au pool. Il construit chaque confirmation en ordre server_seq,
avec le milieu et le domaine communs du Context. Le référentiel et la cellule de chaque
source doivent correspondre à ce contexte. Une erreur identifie le server_seq et sa cause.
Prédictions et rejets ne produisent aucun champ dans ce chemin.

Si une source tardive échoue, le pool de travail peut contenir un préfixe : aucun Prepared
n’est rendu. Ce pool est une zone de préparation, pas une publication vivante. Un hôte qui
veut conserver une ancienne préparation pendant le remplacement doit fournir un second pool.
Le constructeur efface les anciens emplacements non utilisés. Le préfixe reçu ne contient
aucun None ; sa taille correspond exactement aux confirmations.

Tant que Prepared vit, Rust interdit de modifier le journal et de réutiliser le pool. Cette
cohérence est structurelle dans l’API sûre, sans hash de génération supposé à jour. Ce n’est
pas encore un ordonnanceur multilecteur : pour ingérer de nouveaux événements, l’hôte doit
libérer cette vue ou publier une autre génération avec des stockages distincts. Aucun verrou,
aucune copie d’état implicite, aucune allocation pendant build.

## Interrogation et publication du lot

sample_batch reçoit les échantillons B, les points locaux, un instant commun, une limite de
pente, la sortie et un tampon temporaire hôte. Les tailles B/points doivent être égales ; les
deux tampons doivent pouvoir recevoir le lot. Ces refus précèdent toute évaluation.

Chaque point est calculé via le noyau compose dans le temporaire. Une erreur rend l’indice du
premier point fautif et sa catégorie ; **la sortie entière reste inchangée**. Une fois tous les
points reçus, le préfixe utile est copié en sortie, le reste reste intact. Un lot vide réussit
avec longueur zéro. Cela décrit une transaction sous emprunt exclusif, pas une copie atomique
visible par des lecteurs concurrents non synchronisés. Le futur bus publiera après ce succès.

Pas de construction de champ dans la boucle des points, pas d’agrandissement ni d’allocation.
Les pools de champs et de résultats temporaires doivent être dimensionnés à l’initialisation
(I-06, I-16). La comparaison complète événements/champs reste effectuée par compose à chaque
point : correct mais pas encore optimisé. Son coût doit être mesuré avant simplification.
Le coût total reste proportionnel aux points multipliés par les sources et les nœuds spectraux.

Les points non finis ou hors de la borne locale sont désormais refusés par compose même si
le journal est vide. Un journal vide permet autrement de retourner B seul. Le tampon B doit
correspondre au même point, temps, milieu et axes : l’API ne possède pas encore ces métadonnées
et ne peut pas les vérifier. Il n’y a toujours pas d’authentification ni de preuve de livraison
réseau complète attachée à un échantillon.

## Réception et suite

Quatre tests ajoutés : résultat du lot identique au chemin ponctuel, ordre de confirmations,
queue de sortie conservée, échec au dernier point sans sortie partielle, longueurs/capacités,
source invalide tardive, contexte incorrect, perte connue, lot/journal vide et point NaN.
Les comparaisons portent hauteur, vitesse et normale. Les tests de S79 restent requis.

S79-1 réalisée pour ce service local préparé, sans WaterSystem multi-référentiels ni bus.
Prochaine étape S81 : mesurer séparément préparation et évaluation de lots B+W en release,
avec mémoire et latences, puis optimiser le poste dominant sans modifier la réception physique.
Les bases B doivent être évaluées réellement dans cette mesure. Aucun budget AAA ni capacité
maximale de production ne se déduit du seul succès des tests. Rétention S72-2, index spatial,
réseau et profils non reçus restent ouverts. Invariants I-01/I-03/I-06/I-08/I-16 relus, inchangés.
