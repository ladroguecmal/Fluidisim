# ADR-056 — La cause relie la prédiction à la confirmation

- **Statut : ACTÉE**, S73, 2026-09-08, délégation technique.
- **Résout au niveau du contrat interne** A190 ; complète ADR-055 et SPEC-006 §3.3.
- **Produit** `code/water-core/src/wave_journal.rs`, première tranche W2.

## Identité et autorité

Le server_seq ne peut pas être anticipé. La corrélation utilise une identité de cause
`(entity:u64, command:u64, emission:u32)` dans une époque serveur `u64`. L’hôte attribue une
identité durable à l’entité et à la commande de gameplay ; l’ordinal distingue plusieurs impacts
issus de cette commande. Ils ne dérivent jamais de l’ordre local des livraisons d’eau.
Le serveur confirme cette cause depuis le gameplay authentifié ; le client peut la connaître
avant le server_seq. L’hôte doit empêcher sa réutilisation dans une époque. Une cause spontanée
serveur reçoit aussi une identité, sans prédiction nécessaire.

Le journal appartient à une époque ; toute ingestion d’une autre époque est refusée.
Le server_seq de WaveEvent reste l’ordre total des événements confirmés. Il ne peut désigner
deux causes. Même cause avec deux contenus autoritaires différents : conflit, aucune mutation.
Les comparaisons portent les événements validés et normalisés par ADR-055.

Le codec Impact V1 reste inchangé. **L’identité de cause doit être transportée et sauvegardée
avec l’événement**, dans une enveloppe à spécifier ; les seuls 76 octets ne suffisent pas à
reconstruire le journal. Le contrat d’hôte est écrit ici, son raccordement moteur et son
encodage restent à faire. Aucun bit de provenance ne remplace l’authentification par l’hôte.

## Machine d’états

| État connu | Entrée | Résultat |
|---|---|---|
| absent | prédiction, confirmation ou rejet | ajout si place disponible, sinon Full |
| prédit | même prédiction | aucun changement |
| prédit | prédiction différente | conflit |
| prédit | confirmation | remplacement ; ancienne prédiction à rétracter |
| prédit | rejet définitif serveur | marqueur de rejet ; ancienne prédiction à rétracter |
| confirmé ou rejeté | prédiction tardive | ignorée comme dépassée |
| confirmé | confirmation identique | aucun changement |
| rejeté | rejet identique | aucun changement |
| confirmé/rejeté | décision serveur contradictoire | conflit |

Le rejet est terminal sur une cause, pas une expiration locale. Son marqueur empêche une
prédiction arrivée plus tard de ressusciter. Modifier une décision terminale exigerait une
commande de correction versionnée, hors contrat actuel. Un retrait ne supprime pas un impact
serveur accepté. `Change::Retract` restitue la prédiction à annuler ; l’appelant doit publier
son annulation et la confirmation. Ce bus multilecteur reste à construire. La mutation est
atomique dans l’appel exclusif Rust, pas avec une publication externe encore absente.

## Mémoire, ordre et réception

Stockage : tranche de `Option<Record>` prêtée exclusivement par l’hôte au démarrage, effacée par
`Journal::new`. Capacité issue du pool hôte (I-06, I-16). Aucun agrandissement ni allocation dans
les opérations. Un remplacement prend la place de la prédiction même à capacité pleine.
Chaque refus laisse les enregistrements inchangés. Aucune éviction implicite.

Les confirmés sont exposés par server_seq croissant, les autres causes par ordre lexical.
L’ensemble retenu est indépendant de l’ordre d’arrivée si les commandes sont compatibles et
tiennent dans le pool. **En saturation, le préfixe accepté dépend des arrivées** : Full doit
déclencher resynchronisation ou contre-pression hôte. Ce module ne suit pas encore la complétude
et un journal plein ne doit jamais être déclaré complet par son intégrateur.

Recherche linéaire et tri en place après mutation ; coût borné par la capacité, sans promesse
de budget temps réel déjà mesuré. Pas encore d’instantané concurrent. Rejets et prédictions
ne paraissent jamais dans l’itérateur de confirmations autoritaires.

## Rétention et limites

Aucune expiration automatique : ttl_us de la source ne prouve pas que ses effets W ont cessé.
Pas de purge des marqueurs non plus : elle réadmettrait une livraison tardive rejetée.
S72-2 reste ouverte. Avant réutilisation des emplacements, établir une frontière de rejeu,
les effets encore vivants et une preuve de clôture des décisions antérieures. Le module est
un journal borné pour une fenêtre conservée, pas un stockage viable sans limite de durée.

Tests : confirmation à pool plein, replay des événements encodés avec leurs causes, ordre
server_seq, doublons, conflits atomiques, rejet avant/après prédiction, provenance, époque
erronée et capacité nulle. Ni C19 complet ni test réseau. Prochaine tranche : enveloppe
persistée et état explicite de complétude/rétention avant propagation. Invariants I-03,
I-06, I-10, I-11, I-16 et I-17 relus ; aucun amendé.
