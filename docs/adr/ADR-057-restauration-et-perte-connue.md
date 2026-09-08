# ADR-057 — Restaurer un journal entier et conserver sa perte connue

- **Statut : ACTÉE**, S74, 2026-09-08, délégation technique.
- **Complète** ADR-056 et sa première tranche W2 ; traite A191 dans le journal local.
- **Code** : `code/water-core/src/wave_journal.rs`.

## État de validité

Tout refus Full mémorise `loss_known=true`, y compris une prédiction ou un rejet non stocké.
Les enregistrements restent inchangés, mais le journal ne peut plus oublier cet incident.
Un succès ultérieur ne remet pas ce bit à zéro. Être plein sans avoir refusé ne le positionne pas.
Cette politique conservatrice ne confond pas capacité utilisée et événement perdu.

**false signifie aucune perte locale connue, jamais livraison réseau complète.** Les trous de
transport ne se détectent pas avec un server_seq global : une région peut ne pas être intéressée
par tous les événements. Acquittements et frontières d’intérêt appartiennent à l’intégration.
La confirmation publique reste consultable comme sous-ensemble ; son consommateur doit examiner
le marqueur. Ce n’est pas encore une API de gameplay certifiant la complétude.

## Enveloppe de sauvegarde V1

Little endian, sans padding. Taille : `24 + 97*N`, dérivée des champs ci-dessous.

| Offset en-tête | Taille | Sens |
|---|---|---|
| 0 | 4 | ASCII WJNL |
| 4 | 2 | version 1 |
| 6 | 1 | perte connue, 0 ou 1 |
| 7 | 1 | réservé zéro |
| 8 | 8 | époque serveur |
| 16 | 8 | nombre d’enregistrements |

Chaque enregistrement contient : entité u64, commande u64, ordinal u32, état u8, puis les
76 octets Impact V1. État 0 = prédit, 1 = confirmé, 2 = rejeté. Pour un rejet, les 76 octets
sont nuls. Une prédiction/confirmation doit porter la provenance correspondante. Les causes
sont uniques ; les server_seq confirmés aussi. Les ordres d’entrée peuvent varier ; le journal
restauré les trie selon ADR-056. La sauvegarde restitue cet ordre et les valeurs canoniques.

L’enveloppe conserve les prédictions pour une reprise locale. **Ce n’est pas un message de
réplication serveur à diffuser tel quel** : l’intégrateur doit distinguer sauvegarde locale,
instantané autoritaire et confidentialité des prédictions. Le codec ne fournit ni signature,
ni somme de contrôle, ni protection contre une altération qui reste structurellement valide.
La source de restauration doit être de confiance et son intégrité assurée par l’hôte.

## Transaction et mémoire

`save` vérifie la taille avant de toucher la sortie, écrit seulement sa portion utile et rend
sa longueur. `restore` exige une taille exacte, une version reconnue et la même époque.
Il vérifie les multiplications de taille, décode tous les enregistrements et les insère dans
un journal temporaire sur une tranche fournie par l’hôte. Doublons, conflits, provenance,
formats et scalaires invalides provoquent un refus avant modification du journal vivant.

Dernière étape seulement : copier le contenu validé, vider les emplacements restants et
remplacer le marqueur. Le temporaire peut être modifié en cas d’erreur ; le journal vivant,
son époque et sa perte connue restent intacts. Aucun agrandissement ni allocation. Ce chemin
nécessite un second pool à prévoir à l’initialisation (I-06, I-16). Il est réservé à une phase
exclusive hors publication ; pas de transaction concurrente avec les lecteurs du futur bus.

Restaurer un instantané où le bit vaut zéro peut effacer une perte courante : c’est un
**remplacement explicite par une référence hôte**, pas une réparation prouvée par le codec.
Une référence ancienne ne doit pas servir de resynchronisation actuelle sans protocole hôte.
La capacité du journal et celle du temporaire doivent chacune couvrir tous les enregistrements.
La restauration emploie les insertions et tris existants ; sa latence reste à mesurer.

## Réception et suite

Quatre tests ajoutés : vecteurs d’octets vide/rejeté indépendants de save, états mixtes et
perte persistés, restauration atomique face aux erreurs jusque dans le dernier enregistrement,
troncatures, doublons, capacité, provenance et collisions de server_seq. Les neuf tests du
journal passent. C19 complet reste ouvert : intégration fichier/transport, W propagé et V absents.

S73-1 est réalisée dans le périmètre bibliothèque : enveloppe et perte connue persistées,
restauration transactionnelle. A191 traité localement ; protocole de complétude réseau ouvert.
S72-2 reste ouverte : aucune purge TTL, aucune preuve de durée des effets. Prochaine tranche :
W3, premier impact propagé en milieu uniforme, avec contrat physique et contrôles indépendants.
Les durées de ses effets éclaireront ensuite la rétention. Invariants I-03, I-06, I-16 et I-17
relus, inchangés ; aucune donnée δ sérialisée.
