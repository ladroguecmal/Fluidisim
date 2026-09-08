# ADR-067 — Publier une commande avec ses champs, conserver le refus

- **Statut : ACTÉE**, S86, 2026-09-08, délégation technique.
- Complète ADR-056, ADR-063 et ADR-066. Aucun changement de codec ni de modèle physique.

## Décision et construction

`LiveWater` possède deux journaux empruntant leurs stockages hôte et deux pools de champs.
Un couple est publié ; l'autre sert à la transaction. Il n'existe aucune référence interne
vers un membre mobile de la structure. `current()` construit une vue `Prepared` empruntée ;
son usage empêche une mutation simultanée du service par les règles d'emprunt Rust.

L'hôte fournit une commande typée de prédiction, confirmation ou rejet, avec époque et cause.
L'authentification des commandes autoritaires reste sa responsabilité. À chaque `update` :

1. Copier les enregistrements, l'époque et l'indicateur de perte du journal publié vers la réserve.
2. Appliquer la commande avec les règles existantes du journal.
3. Construire tous les champs confirmés dans le pool de réserve et vérifier la couverture de `now`.
4. Échanger les deux journaux et les deux pools seulement après succès complet.

Le retour `Change`, notamment la rétraction d'une prédiction, n'est remis à l'hôte qu'après
cette publication. Une prédiction ne produit toujours aucun champ W de gameplay. Un rejet
ne peut pas annuler une confirmation terminale : les règles ADR-056 restent inchangées.
La copie interne ne passe pas par un codec temporaire et n'alloue pas. Elle vérifie la capacité
avant écriture et conserve l'ordre des enregistrements, y compris les rejets.

## Une ancienne version cohérente peut être périmée

Si une commande valide ne peut être publiée faute de place, de champ calculable ou d'horizon
couvrant l'instant demandé, elle reste dans `pending`. L'ancien couple n'est pas modifié, mais
`current()` refuse : le présenter comme courant cacherait un événement reçu et non appliqué.
Seule la même commande peut être retentée. Un appel sans commande ou une autre commande ne peut
effacer ce blocage. La commande est consultable et reste en mémoire jusqu'au succès.

Les erreurs d'époque, d'autorité ou de conflit du journal refusent la commande sans bloquer une
vue précédemment courante. Un âge cible inférieur à l'âge publié est une erreur de précondition,
avant admission ; l'hôte conserve alors la responsabilité de représenter sa commande correctement.
Les autres refus sont retournés avec leur cause. Aucun succès B seul ne remplace une erreur.

`update(None, now, age_us)` reconstruit uniquement l'horizon lorsqu'aucune commande n'attend.
Un refus de ce renouvellement conserve la vue active ; son échéance reste vérifiable par
`renewal_deadline()` et ses requêtes continuent de refuser après expiration. N, rayon et milieu
sont fixes ; une reconstruction réussie ne décale jamais la naissance. L'appel est synchrone,
à déclencher par l'hôte, sans ordonnanceur de budget reçu.

## Réception logicielle S86

Quatre tests ciblés release et suite debug : prédiction puis confirmation avec rétraction,
doublon, rejet de prédiction, confirmations ordonnées, alternances multiples des pools,
renouvellement à 16 s et comparaison des sorties à une construction directe ; échec sur le
second champ après succès du premier, commande en attente et reprise ; saturation de journal
et de pool de champs sans effacement du blocage ; époque invalide et candidat temporellement
expiré, puis reprise avec horizon suffisant. Les fixtures n'élargissent pas la réception physique.

I-03, I-06, I-08 et I-11 inchangés. Pas de déterminisme interplateforme nouvellement certifié.
Leçon L200 ; aucun nouvel angle numéroté, complétude déjà suivie par S74-1/S72-2.

## Ce qui reste ouvert

La capacité totale est celle des deux journaux, des deux pools et d'une commande en attente.
Le service impose une contre-pression : les commandes suivantes restent à la charge de l'hôte.
Il ne constitue pas une réception réseau complète. Une source hors domaine peut bloquer le
service local : routage spatial et modèles supplémentaires ne sont pas fabriqués par le refus.

Le contrôleur S85 reste disponible pour un journal figé ; LiveWater fournit l'admission et
le renouvellement explicite sans combiner deux propriétaires des mêmes pools.
La construction consomme les journaux fournis ; si elle échoue, elle ne retourne pas leurs
poignées. Une procédure de récupération ergonomique reste à construire.

**S86-1, prochaine session S87 :** sauvegarder et restaurer le service publié avec sa commande
en attente, puis permettre une reconstruction sur des pools plus grands sans perdre le blocage.
Le codec WJNL seul ne contient pas cette attente ; l'utiliser comme sauvegarde complète du service
serait incorrect. Rétention durable S72-2, purge, publication multilecteur et durée arbitraire
restent ouvertes. Aucune migration d'époque ni abandon silencieux d'une commande ajoutés ici.
