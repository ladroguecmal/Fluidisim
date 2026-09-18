# Relaxation de surface perturbative — S268

## Contrat avant construction

Capacité bornée : le pas couplé mobile amortit hauteur **et** vitesse perturbatives
près des bords, prérequis constaté absent aux frontières ouvertes. Consommateur :
`Volume::step_perturbation_mobile`, paramètres Sponge existants. ADR-164.

Critères avant code :
1. Étape locale contre exponentielle indépendante f64, deux signes, intérieur,
   profil symétrique, 1 et 1000 sous-pas ; erreur absolue <= 8 ulps de la hauteur de
   repos plus 8 epsilon f32 fois le nombre de pas fois l'amplitude initiale. Cette
   borne numérique couvre arrondis du coefficient et de la hauteur, pas une tolérance physique.
2. Intérieur et Sponge nul identiques au bit, hauteur et reste ; fond immuable.
3. Pas réel avec fond non nul : comparer au témoin qui conserve seulement l'éponge
   de vitesse, différence de hauteur égale à la relaxation prévue à la borne du point 1 ;
   u/w/p identiques au témoin pour ce pas. Répéter 20 pas pour exercer le consommateur.
4. Expiration à tous les checkpoints d'un petit domaine : aucune avancée, champs
   et surface au bit ; reprise identique. Zéro allocation mesurée dans le pas.
5. Suite cœur/harnais release hors ligne, tests existants à fond nul et sans éponge
   conservés. Pas de nouvelle réception GPU (chemin non touché).

Arrêt : critères ci-dessus reçus, API documentée, limites et prochaine mesure inscrites.
Aucune réception de réflexion <1 %, de passage d'un fond incident, des frontières du
TOTAL, de B4 global ou du budget mural 2 ms. Ces lots suivent avec un paquet de garde
séparant incident/réfléchi selon ADR-046. Ce découpage ne réduit pas J2.


## Construction et contrôles ciblés

`delta_coupling::Volume::relax_surface` est consommé directement après
`transport_coupled`, avant la validation et la publication du pas existant.
Aucune API ni ressource supplémentaire, aucune modification de B/W ni de l'afficheur.
Le témoin « vitesse seule » est un commutateur de test, absent du code de production.

- `surface_sponge_exact_decay_compensation_and_interior_s268` : exponentielle
  indépendante f64, amplitudes positives/négatives, 1 et 1000 sous-pas, profil spatial,
  intérieur et taux nul au bit. Le reliquat sub-ulp est lui aussi amorti ; cette
  contre-épreuve échouerait si le reste ancien était simplement conservé.
- `mobile_step_consumes_surface_sponge_without_damping_background_s268` : fond
  stationnaire analytique S253, 16 colonnes, 20 pas de 1 ms, largeur 0,5 m et taux
  4/s **paramètres d'essai, non calibration d'absorption**. Témoin depuis le même
  état à chaque pas : u/w/p identiques, hauteur égale au facteur analytique dans
  la borne fixée ; 160 couples hauteur/reste modifiés, échantillons B/W intacts.
- `coupled_surface_sponge_is_atomic_and_allocation_free_s268` : **607 expirations**
  (tous les checkpoints exposés par le petit domaine), restauration et reprise
  identiques, **zéro allocation** pour chaque pas tenté et repris. Cette horloge
  simulée reçoit l'atomicité ; elle ne mesure pas le budget mural.

La dissipation locale est reçue, pas la réflexion ni l'absorption globale d'un paquet.
Le pas est d'ordre un par séparation des opérations. La fermeture extérieure du MAC
reste réfléchissante ; aucun flux transparent nouveau n'a été construit. Le volume
perturbatif peut varier dans l'éponge, et rien n'est transféré vers W (limite ADR-046).

## Reproduction

```powershell
cargo test --release --offline --locked --manifest-path code/Cargo.toml -p water-core --lib s268 -- --nocapture
cargo test --release --offline --locked --manifest-path code/Cargo.toml -p water-core --test delta_runtime s268 -- --nocapture
cargo test --release --offline --locked --manifest-path code/Cargo.toml
```


## Régression complète et verdict

Suite workspace release hors ligne verrouillée : **473 réussis, 18 ignorés, 0 échec**
(cœur 362, intégrations 16, harnais 95). Les ignorés restent des bancs explicites.
Identité S253 avec fond nul et éponge nulle, oracle harmonique S253 et prolongement
S254 conservés par leurs essais existants ; aucune réception multiplateforme ajoutée.

**Critères du lot tenus.** La capacité nouvellement consommée est la relaxation de
hauteur perturbative dans le pas mobile transactionnel. Le coût n'a pas été mesuré ;
aucun gain ni respect du budget mural n'est revendiqué. Aucun champ GPU modifié.
Suite : paquet sortant, témoin sans éponge, jauge avec fenêtres incident/réfléchi et
contrôle de contamination, puis fond traversant et frontières du total. La tolérance
historique de réflexion d'ADR-046 ne sera pas déclarée tenue sans cette mesure.
