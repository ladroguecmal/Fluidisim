# Émission progressive du sillage — S151, 2026-09-10

ADR-104 ; `cargo test -p water-core emitter_ -- --nocapture` depuis code/,
et même commande avec `--release`. Cinq nouveaux tests reçus ; scénario de reprise
hash `ce3395b96567718c` identique debug/release (contrôleur, champ et énergie).

## Réception

L'émetteur produit un tronçon d'une seconde à2m/s selon X, puis un selon Y,
charge100N, sigma1m, g9,81/rho1025. Chaque tronçon devient une source distincte ;
la cause conserve objet/commande et incrémente émission. Spectre32×32, cutoff4,
fenêtre0–8s. Il s'agit d'une réception transactionnelle, pas d'un nouveau certificat
physique de cette finesse (référence raffinée du trajet : S150).

Le journal d'une place reçoit le premier tronçon. Au deuxième, saturation : le
curseur reste à1s, le champ publié reste identique. Copie dans deux places, reprise
de l'attente et extend_into du contrôleur ; seulement alors acquittement à2s.
Une nouvelle admission du même tronçon retourne AlreadyPresent ; exactement deux
sources sont présentes, aucune réémission du préfixe. Sept points à1/2/4/8s,
sept composantes de Surface plus énergie/puissance : égalité bit à bit avec le
trajet complet construit par Wake::build, même ordre de segments. À4/8s, puissance
nulle et énergie positive : la fin du mouvement n'efface pas son onde.

Autres contrôles : répétition de prepare sans mutation, acquittement avant admission
refusé, contenu conflictuel sous la même identité refusé, double acquittement périmé,
plage d'identités épuisée, date/position/repère discontinus. Un forçage1e30N peut être
représenté par l'adaptateur mais son énergie ne l'est pas : refus du contrôleur,
source retirée, curseur et énergie publiée inchangés. L'admission WPRS n'est donc
pas confondue avec une réception de champ physique.

## Utilisation hôte

1. Réserver stockage d'émissions et plage d'identités dans l'époque.
2. Préparer un intervalle de mouvement déclaré avec son début égal au curseur.
3. Conserver l'Emission dans le stockage hôte ; admettre sa Source au contrôleur.
4. Acquitter avec controller.journal() seulement après publication réussie.
5. En cas de saturation, conserver la même émission et reprendre dans un pool agrandi.

Le test exécutable montre l'amorçage (première source, construction du contrôleur,
acquittement), la publication incrémentale et la reprise sans trou. L'API ne reçoit
pas des poses à interpoler : l'hôte fournit vitesse, charge et durée de chaque
intervalle. Aucun changement de format WPRS ni de journal n'a été nécessaire.

## Limites et suite

S150-1 réalisée pour le contrat progressif en milieu uniforme et fenêtre existante.
W4 reste partiel : pas de moteur externe raccordé, de calibration coque, de courant,
de migration de référentiel ni de sillage illimité ; durée maximale du contexte16s.
L'hôte doit conserver/restaurer son curseur avec les sources et réserver ses identités.
La rétention longue durée reste S72-2 ; aucune purge à l'arrêt n'est ajoutée.

**S151-1 : prochaine S152, B2.** Partir de DOSSIER-B2 et du candidat désormais
impact+sillage, annoncer domaine/coût comparables avant mesure ; produire un verdict
explicitement partiel si l'absence de δ dispersif/2D ou de matériel cible le limite.
Ces absences bornent le verdict, elles ne justifient pas de reporter tout le banc.
