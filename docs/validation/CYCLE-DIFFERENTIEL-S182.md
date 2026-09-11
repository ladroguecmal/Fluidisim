# Cycle vivant du consommateur différentiel — S182

S181-1/A50 ; applique ADR-117 sans modifier le contrat de publication.

## Protocole déclaré avant tests

- Comparer toutes les grandeurs différentielles et la source en bits, contrôleur
  de pression contre préparation directe du même journal au même instant. Inclure
  des retours temporels, naissances/extinctions et absence d'actualisation nécessaire.
- Après une mise à jour refusée, retrouver exactement la publication précédente ;
  une vue périmée doit être refusée au nouvel instant sans sortie partielle.
- Admettre une source en dernier (voie incrémentale), puis une source intercalée
  (recalcul complet), et comparer à la voie directe. Recevoir saturation/reprise
  sur un journal élargi, sans confondre dernier champ publié et journal complet.
- Renouveler les impacts via le service vivant, refuser un horizon insuffisant puis
  reprendre ; comparer à une reconstruction directe. Sauvegarder puis restaurer les
  événements et contextes, reconstruire les champs et retrouver dérivées/source.
- Workspace, ciblés release et C02/C18 inchangés. Ces contrôles ne mesurent pas le
  coût, ne reçoivent pas δ3D et ne remplacent pas les références physiques S177–S181.

## Résultats

Trois nouveaux tests de bibliothèque dans tests_differential_cycle.rs, reçus en debug et release.
Aucun code d'exécution corrigé. Le protocole applique ADR-117 ; aucun nouvel ADR.

Chaque comparaison porte sur34 scalaires en bits : les26 grandeurs du champ profond,
la pression appliquée, son gradient, rho et les trois composantes de la source à
nu=1e-6m²/s. Trois points : (0;0;0), (0,75;0,25;-0,5), (-1;1;-2)m, ancre monde
(1e9;-1e9;0)m. B16 composantes, pression192 nœuds, impacts N64. Ce sont les recettes
du montage de test existant, pas un profil de production choisi par cette réception.

### Actualisation et rejeu de pression

Séquence0,499999,500000,750000,1999999,2000000,2500000,3000000,750000,750000µs :
publication courante comparée à Prepared::from_journal à chaque instant ; second
update identique rend Unchanged. Les champs changent effectivement entre deux dates.
La séquence couvre naissance et extinction des deux sources, ainsi qu'un retour temporel.

Update à8000001µs refusé : instant publié et34 scalaires préservés. Une vue précédente
àun instant décalé de1µs fait refuser la requête entière et laisse la sortie intacte.
Snapshot WPJR restauré dans de nouveaux pools de sources/segments, champs reconstruits
àcinq instants : dérivées et source identiques. Aucun champ dérivé n'est sérialisé.

### Admission et saturation

Journal initial id0/1 ; admission id3 en dernier (voie incrémentale), puis id2 entre
id1 et id3 (recalcul complet) : même résultat que la préparation directe, source
nouvelle ayant un effet non nul. Réadmettre id2 donne AlreadyPresent.

Id4 sature le journal de quatre places. L'attente est conservée ; update vers un autre
instant refuse Pending, la dernière publication au même instant reste servable et
identique. Elle représente les quatre sources publiées, pas l'attente. Copie vers cinq
places, retry puis extend_into : le nouveau champ égale la reconstruction directe avec
cinq sources et diffère de l'ancien ; l'ancien reste inchangé jusqu'au basculement hôte.

### Impacts vivants et restauration du service

À4,1s, renouvellement avec horizon4s refusé ; publication à3s inchangée, requête à4,1s
refusée atomiquement. Extension à8s acceptée et identique à une reconstruction directe.
Une seconde confirmation, demandant un horizon100s non représentable, reste en attente :
current refuse Pending. Le snapshot du service conserve cette attente et l'horizon
publié8s. Un second service restauré refuse lui aussi de se dire courant.

Les deux services reprennent la même commande avec horizon8s ; deux impacts publiés.
À0,5/0,75/4,1/6s, dérivées et source de chaque service identiques à la reconstruction
directe. Snapshot publié tronqué refusé sans altération ; restauration complète reçue.

### Conformité

Workspace :331 tests réussis, cinq ignorés (238+93 réussis,2+3 ignorés).
C18 hash0x85c8bc610f551d11 et C02 hash0x0a3a3bcc945db263 inchangés ; zéro échec.
Le compteur historique alloc_post_seal=1 des scénarios ne mesure pas ce consommateur.
Pas de benchmark dans ces tests ; les allocations des fixtures/oracles ne mesurent
pas la requête d'exécution. Les références physiques restent celles de S177–S181.

Commandes depuis code/ :

```text
cargo test -p water-core prepared_water::mixed::tests::differential::cycle -- --nocapture
cargo test -p water-core prepared_water::mixed::tests::differential::cycle --release
cargo test --workspace
cargo run -p water-harness --release -- check scenarios/C18-invariants.toml scenarios/C02-dispersion.toml
```

## Portée et suite

S181-1 réalisée sur le montage de bibliothèque : publication, admission, saturation,
renouvellement et rejeu reçus pour le consommateur différentiel. A50/B4 restent partiels ;
aucun solveur3D, seuil de bascule, coût temps réel ou déterminisme multiplateforme reçu.

**S182-1, priorité S183 :** mesurer le coût complet de la requête différentielle mixte
et de sa source sur plusieurs tailles de lots/recettes, en distinguant préparation,
actualisation, évaluation et refus. Comparer au chemin de surface à entrées identiques,
mesurer ou instrumenter les allocations de requête, publier les conditions et limites
de mesure avant de choisir un budget de consommation perturbative. BILAN-S145/S176
portés par la construction, pas par une nouvelle variante du véhicule1D.
