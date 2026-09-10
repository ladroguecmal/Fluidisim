# Cycle spectral — S149, 2026-09-10

ADR-102 ; `cargo run -p water-core --release --example cycle_spectral` depuis code/.
Réception également exécutée en debug avec `--verify-only`.

## Résultat

WSPR64 octets et WLIV289 octets reconstruisent B32 + un impact W256 après destruction
des objets sources. Ancre à 1 000 000 m, référentiel7/cellule9, milieu et temps conservés
par l'hôte. 64 points, temps0/1/3/4s ; chaque échantillon compare les dix f32 bit à bit
à la composition directe, puis les hashes avant/après restauration coïncident.
Le dernier temps reçoit aussi l'expiration de l'événement. Snapshot réencodé identique ;
WLIV de mauvais référentiel refusé sans changer le snapshot publié.

Hashes identiques debug/release : `5f3b72dadb1de4aa`, `964662f5054e452d`,
`a8e2e22027a00d20`, `c09b5d104e167de5`.
Le test spectral_transport_s149 reçoit troncatures0..63, surplus, magie, réservés,
version, huit paramètres NaN, compte hors profil, graine/hash modifiés et douze
recettes gamma/N, avec conservation du zéro signé.

## Coût local

Release Windows x86_64, cargo1.97.0, échauffement100 passages,15 blocs de32,
opérations séparées ; médiane des temps moyens par bloc, microsecondes.

| Opération | Médiane | Min–max |
|---|---:|---:|
| décodage et recuisson B32 | 2661,106 | 2495,575–3474,394 |
| restauration WLIV, un W256 | 2,534 | 2,334–3,066 |
| requête64 points B32 + W256 à1s | 479,488 | 453,072–587,694 |

La recuisson coûte environ2,66ms et appartient au chargement, pas au battement.
La requête vaut environ7,49µs/point sur cette fixture ; ce n'est ni un budget garanti
ni une comparaison B1 à matériel identique. La construction allouante de Background
n'est pas incluse dans la ligne recuisson. Aucun banc canonique supplémentaire reçu.

## Suite et limites

S148-1 close pour le transport de recette et le cycle hôte en mémoire. Pas de format
disque/réseau global, d'intégration pression/mixte, ni de preuve multiplateforme.
A212 reste partielle : statistiques multigraines et bandes/directions du jeu à recevoir.
S149-1 : construire W4/sillage selon la trajectoire ADR-054, puis B2. La cuisson
spectrale est maintenant utilisable par les prochains montages ; aucun motif pour
retarder W4 par une nouvelle série d'audits du fond.
