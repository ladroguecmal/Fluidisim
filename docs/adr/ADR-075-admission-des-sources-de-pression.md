# ADR-075 — Admission bornée des sources de pression

- **Statut : ACTÉE**, S109, 2026-09-09, délégation technique.
- Complète ADR-074 ; applique la séparation publication/attente de ADR-067/068.
- Ne modifie aucun format Impact, WPRS, WJNL ou WLIV.

## Décision

Construire `pressure_journal::Journal` sur un pool hôte de sources empruntées immuablement.
Chaque source conserve sa trajectoire externe ; la copie du descripteur ne copie ni les
segments ni le champ. Aucune allocation, éviction, mutation de source, prédiction ou
rétractation dans cette première tranche. L'hôte garantit la durée de vie du stockage.

`admit_authenticated` exige une provenance déjà authentifiée par l'hôte. L'appel ne
réalise pas cette authentification. L'époque de la source doit égaler celle du journal.
Dans une époque, id et cause identifient chacun une source unique. Toute collision sur
l'un des deux avec un contenu différent est un conflit. Une répétition strictement
identique retourne Unchanged, y compris quand le pool est plein.

L'identité compare tous les paramètres et tous les segments, en bits pour les f32,
y compris les zéros signés. Aucun hash ne décide de l'égalité. Source::same_content
compare la totalité du contenu encodé WPRS V1, pas son adresse mémoire. Une nouvelle
fenêtre, recette ou pression sur la même cause est donc un conflit ; aucune politique
de révision implicite. Les sources sont ordonnées par id croissant, indépendant de
l'ordre d'arrivée. Cet ordre canonique n'est pas une séquence serveur et ne certifie
pas la complétude d'une livraison.

## Saturation et reprise

Une première source valide sans place est conservée dans un emplacement pending du
journal et l'admission retourne Full. La publication antérieure reste inchangée.
`current()` refuse alors Pending ; `published()` permet explicitement de lire cette
ancienne publication. Aucun lecteur ne doit prendre published pour la vue courante.

Pendant cette attente : une répétition d'une source déjà publiée retourne Unchanged
sans effacer l'attente ; la même attente retourne Full tant que le pool est plein ;
une collision avec elle retourne Conflict ; une source indépendante retourne Pending.
Cette dernière n'est pas retenue : l'hôte doit la conserver et appliquer une contre-
pression en amont. L'emplacement pending n'est pas une file arbitrairement longue.
Un refus d'époque ou conflit ne remplace jamais l'attente. Aucun acquittement ne doit
être envoyé à l'hôte pour Full/Pending/Conflict/Epoch.

`copy_into` permet de transférer la publication et l'attente vers un pool plus grand,
sans admettre l'attente. Une capacité insuffisante refuse avant toute écriture dans la
cible. Au succès les queues sont mises à vide, les sources restent les mêmes emprunts.
L'ancien journal est intact. `retry()` est l'action explicite qui admet ensuite l'attente
et ouvre la vue courante ; sans attente, elle retourne Unchanged.

Ces sémantiques conservent L199/L200/L201 : l'échec du calcul ou de la capacité ne rend
pas une entrée reçue invisible, et une reprise plus capable ne doit pas acquitter la
commande pendant sa restauration. Le protocole n'ajoute aucun nouveau invariant.

## Réception

Quatre tests ciblés : arrivée3/1/2 publiée1/2/3, doublon même quand plein,14 mutations
de métadonnées en conflit, époque incorrecte ; pression/trajectoire/zéro signé comparés,
aller-retour codec reconnu malgré le changement de stockage ; saturation, publication
conservée, commande indépendante refusée, copie sur pool élargi conservant l'attente,
puis retry ; capacité zéro et cible trop petite inchangée, récupération depuis zéro.
Témoins nominaux présents après refus. Suite complète et tests release consignés au journal.

## Limites et suite

Le journal reçoit des descripteurs valides, pas des champs garantis préparables : les
refus numériques du candidat restent distincts. Aucun journal multi-types, mélange des
champs de plusieurs sources, admission atomique journal/champ, transport ou fichier
durable n'est construit ici. Plusieurs contextes peuvent coexister dans le journal ;
leur compatibilité devra être vérifiée lors de la préparation.

**S109-1, S110 :** sauvegarde mémoire versionnée du journal de pression incluant l'attente,
restauration transactionnelle dans des pools hôte et refus des paquets incomplets ou
conflictuels. Préserver le blocage sur une cible plus grande jusqu'à retry explicite.
Ne pas confondre codec d'instantané et durabilité disque. Authentification et preuve de
complétude réseau restent des préconditions d'intégration.
