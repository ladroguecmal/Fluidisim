# Premier raccordement volumique B/W→δ — S250

## Protocole avant construction

Contrat : [ADR-149](../adr/ADR-149-premier-raccordement-volumique.md).
Capacité visée : `Volume` consomme les échantillons différentiels B/W dans son pas
MAC, au lieu de ne recevoir qu'une hauteur imposée. Surface imposée, ν=0, coupe planaire.

1. Fond nul : même prédicteur que l'advection reçue et même pas sous la précision f32.
2. Source manufacturée : signe -S connu ; champ affine de perturbation et fond affine,
   termes croisés comparés à leur expression analytique, témoin sans terme en défaut.
3. Fournisseurs B et pression W réels planaires : somme avant contraction, réponse
   non nulle, contre-épreuve sans source ; projection à la tolérance existante S199.
4. Éponge : σ=0 identité ; facteur exponentiel analytique, intérieur intact,
   énergie décroissante sur un champ solénoïdal sous couvercle homogène. Aucun seuil
   de réflexion transmis d'un autre solveur.
5. Métadonnées, non-planarité, non-finis, pression refusée, expiration/reprise :
   publication atomique ; pas sans allocation. Rejouer la suite et l'empreinte delta_filters.

Le banc de réception doit traverser le candidat ; ni une simple somme a posteriori
ni une suppression de S ne reçoit son couplage. Les fournisseurs sont évalués à toutes
les faces ; la décimation B4, la surface mobile et les résidus de frontière sont hors lot.

## Construction et contre-épreuves

`delta_projection::Volume::step_perturbation` consomme `BackgroundFaces` dans le
prédicteur, avant la projection existante. Les six tests unitaires S250 couvrent
le fond nul, le signe, les deux termes croisés, le profil quadratique exponentiel,
les métadonnées/refus et la reprise après expiration. Le test d'intégration
`perturbation_step_has_no_runtime_allocation_s250` compte **zéro allocation globale**
sur un pas forcé et sur son expiration. Les tampons de faces sont alloués par l'hôte.

Le banc `delta_coupling` emploie un mode B (`Hs=0,1 m`, `Tp=4 s`, graine 7) et deux
modes W planaires (`k=0,7 et 1,2 rad/m`, poids 0,5, transformée 1), pression 80 Pa,
trajet x à 0,5 m/s actif 2 s. Domaine 8×4 m, départ à 1 s, 20 pas de 1 ms,
surface perturbative imposée `0,01 cos(πx/8)`, éponge 1 m et 2 s⁻¹.
Ce sont des paramètres de banc, aucune calibration gameplay. Somme de tous les
champs différentiels à chaque face puis contraction ; aucune décimation.
Le témoin garde les mêmes champs, hauteur et éponge, mais retire S de `du_dt`.
Il ne représente pas une solution physique : il vérifie que la source est consommée.

### Cas plat refusé, conservé

`--flat` retire la hauteur perturbative imposée et démarre avec v=0. Le pas retourne
`Convergence` à 16×8 et 32×16, même après le repli multigrille ; le banc vérifie
u/w/p/eta conservés exactement. Une trace temporaire (retirée du code livré) donne
respectivement D=1,4185·10⁻⁴ et 8,0636·10⁻⁵, avec arrêt au plancher et erreur inverse
3,93·10⁻⁷ / 3,74·10⁻⁷. **Aucune tolérance relevée pour faire passer ce cas.**
La proximité d'une source gradient et la normalisation par la petite vitesse
projetée sont des suspects, pas une cause isolée. Comparer à une projection f64
indépendante avant de modifier le solveur ou son critère. Ce démarrage reste non reçu.

### Reproduction

```powershell
cargo test --release --offline --manifest-path code/Cargo.toml
cargo run --release --offline --manifest-path code/Cargo.toml -p water-core --example delta_coupling
cargo run --release --offline --manifest-path code/Cargo.toml -p water-core --example delta_coupling -- --flat
cargo run --release --offline --manifest-path code/Cargo.toml -p water-core --example delta_coupling -- --flat --fine
cargo run --release --offline --manifest-path code/Cargo.toml -p water-core --example delta_filters
```

## Limites de réception

Résultats locaux du 2026-09-16, Windows x86-64, release, un fil, secteur avant/après
(batterie 99 %, état 2). Suite cœur/harnais : **444 réussis, 16 ignorés, zéro échec**.
Empreinte `delta_filters` inchangée **0xfb12b2092df4ee6d**, ordres 1,947/1,957/1,959.
Vingt pas couplés reçus à 16×8 : divergence maximale **5,787·10⁻⁶** ; différence
avec le témoin sans S **2,471·10⁻⁷ m/s** au premier pas ; termes croisés manquants
si les résidus étaient additionnés séparément : **1,520·10⁻³ m/s²** au maximum.
Préparation médiane **0,1833 ms**, pas médian **0,1747 ms**, maximum **0,2743 ms**.
Mesure descriptive sur trajectoire de 20 pas, premier passage inclus, sans chauffe
écartée, hors suite de tests ; ni extrapolation à 60 Hz ni gain contre un autre chemin.

Pas de raccordement au rendu, d'ordonnancement multi-domaines, de frontière totale
imperméable, de résidu de surface mobile ni de transfert d'énergie vers W_local.
L'éponge dissipe ici ; B2/réflexion reste non reçu. Une coupe radiale 3D est refusée.
Pas de précision B4 globale reçue par ce seul branchement. Le budget exclut les
fournisseurs et ne constitue pas une réception I-05 murale. Les champs restent f32 ;
durée/instant entiers, coefficients calculés en f64 selon ADR-141.
