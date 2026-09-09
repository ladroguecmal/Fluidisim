# ADR-076 — Instantané du journal de pression avec attente

- **Statut : ACTÉE**, S110, 2026-09-09, délégation technique.
- Complète ADR-074/075, applique ADR-068 sans modifier WLIV, WJNL ou WPRS.

## Décision et format

Construire `pressure_journal::snapshot_into` et `restore_into`, instantané mémoire
**WPJR V1**, little-endian. Il conserve les sources publiées dans l'ordre canonique
croissant de leurs identifiants, puis la source en attente si elle existe. Chaque
source est un bloc WPRS V1 complet, avec sa propre longueur et sa trajectoire.
Aucun coefficient de champ n'est enregistré ; I-17 est conservé.

| Offset | Taille | Champ |
|---|---:|---|
|0|4|magic ASCII WPJR|
|4|2|version1|
|6|2|réservés zéro|
|8|4|taille totale u32|
|12|4|nombre de sources publiées u32|
|16|8|époque du journal u64|
|24|1|attente :0 ou1|
|25|7|réservés zéro|
|32|variable|sources WPRS publiées, puis attente|

Taille =32 + somme des tailles WPRS, attente incluse. Vide :32 octets ; une source
publiée et une en attente, chacune à un segment :344 octets. Aucune capacité de pool
ancienne n'est sérialisée : elle n'est pas un fait publié. L'attente est un fait,
elle subsiste même si le pool cible permettrait de l'admettre immédiatement.

## Restauration transactionnelle

L'hôte fournit l'époque attendue, un pool de descripteurs et un pool de segments.
Le lecteur vérifie avant toute écriture : magic/version/réservés, longueur exacte,
comptes bornés par la taille reçue, époque, validité de chaque WPRS via le même
validateur que le codec source, ordre strict des identifiants publiés, unicité de
l'identifiant et de la cause parmi toutes les sources, attente incluse, capacités.
Ni les doublons exacts ni les suffixes inconnus ne sont silencieusement éliminés.
Une attente peut avoir un id inférieur aux sources publiées ; elle n'est pas triée
avec elles tant que retry ne l'a pas admise.

Un refus conserve intégralement les deux pools. Au succès, les segments sont copiés
dans le pool destination, les descripteurs les empruntent, le journal n'emprunte pas
les octets de l'instantané. La queue du pool de segments est intacte ; les emplacements
inutilisés du pool de descripteurs sont mis à None. La vue restaurée n'est retournée
qu'après reconstruction complète. Les expect internes du second passage s'appuient
sur les mêmes données immuables et capacités déjà vérifiées, pas sur une confiance
accordée aux octets avant validation.

Un journal restauré avec attente refuse current jusqu'à retry explicite. published
reste consultable selon ADR-075. Une cible à capacité zéro peut restaurer un journal
ne contenant qu'une attente si elle possède le stockage de ses segments.
La restauration n'admet aucune nouvelle source et n'effectue aucun calcul de champ.

L'encodeur contrôle taille et capacité avant toute écriture, conserve la queue du
tampon destination, puis sérialise publication et attente. Aucune allocation dans
ces deux opérations. Le contrôle de causes est quadratique en nombre de sources,
sans tableau temporaire, par comparaison des identités sur les blocs déjà validés.
La validation des trajectoires est linéaire en segments ; le second passage répète
leur validation via le codec source. Aucun coût ni budget temps réel certifié ici.

## Réception

Quatre tests ajoutés : aller-retour binaire exact et effacement du tampon source sans
altération du journal ; attente conservée sur cible agrandie, retry donnant l'ordre
canonique ; toutes344 troncatures d'un instantané témoin, suffixe, comptes, époque,
réservés, NaN tardif et conflits conservant les deux pools ; capacités insuffisantes,
cas vide/attente seule, queue de sortie et de segments ; ordre publié inversé refusé.
Tests ciblés aussi en release, suite complète consignée au journal. Les tests de
source continuent de vérifier la reconstruction physique identique après WPRS.

Le contrôle source a été extrait dans une primitive interne sans allocation. La
priorité des erreurs de decode_into change si les octets sont invalides ET le pool
trop petit : la validité des octets est désormais examinée avant la capacité.
Aucun changement de format ni des critères de validité des sources.

## Limites et suite

Il s'agit de sérialisation mémoire, pas de durabilité disque, détection générale de
corruption, authentification ni preuve de livraison complète. Un bit altéré donnant
un autre message valide reste indétectable sans protection de transport/stockage.
L'époque attendue protège seulement du mauvais contexte temporel déclaré. Pas de
checksum de sécurité inventé, ni d'extension automatique du journal Impact.

**S110-1, S111 :** scénario de bout en bout source WPRS → admission → saturation →
instantané WPJR → restauration → retry → préparation et requête B+pression, avec
comparaison directe des résultats et mesure séparée des coûts de codec/reconstruction.
Cela doit éprouver les liens entre les étapes, pas seulement leurs tests isolés.
La préparation de plusieurs sources en un champ commun et les interactions avec
les impacts restent à construire ; pas de somme implicite d'énergies par source.
