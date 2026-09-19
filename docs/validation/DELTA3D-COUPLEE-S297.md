# Couplage de la référence δ 3D — S297

2026-09-19. Porte B, lot 3 d'ADR-175. `delta3d_coupling.rs` : source et advection croisée
ADR-149, géométrie totale et fantômes ADR-152, affinage homogène ADR-153, relaxation de
hauteur ADR-164, bandes prescrites sur les quatre bords ADR-165, quadrature linéaire ADR-166.
La référence CPU reste hors boucle d'image ; ni coût de production ni rendu final reçus.

## 1. Contrat

`BackgroundFaces3` emprunte trois tableaux de `BackgroundSample`, déjà sommés B+W. Vérifications
de dimensions, instant, densité, gravité, finitude et constance verticale de l'élévation.
`eta` publiée porte repos + perturbation ; la géométrie porte eta + élévation du fond.
`Sponge3` reçoit deux largeurs et un taux, sans seuil physique nouveau ; les taux quadratiques
des deux axes s'ajoutent aux coins. Pour le cas limite 2D, désactiver l'axe y.

`step_perturbation_mobile` conserve les six champs publiés sur refus et n'alloue rien.
Un affinage homogène est tenté une fois si la projection manque la tolérance au plancher,
comme en 2D ; aucune tolérance n'a été déplacée. Réserves comptées avant scellement.
Le fond doit être linéaire au plan moyen, incompressible et sans flux au fond (ADR-152).
L'interface accueille B+W ; les cas physiques de ce lot utilisent le fond analytique reçu S253,
pas encore les fournisseurs complets de la scène de mer.

## 2. Tests ciblés

- Fond nul : identité au bit avec le pas mobile S296 sur 200 pas, ny=1 et ny=4, tous les champs
  publiés et reste de hauteur. Le schéma de la 2D est conservé.
- Fond uniforme, vitesse (0,6 ; −0,4 ; 0) m/s, élévation 7 cm : traverse les quatre bords sur
  100 pas sans produire de perturbation, zéro itération, même avec éponge.
- Onde stationnaire couplée : indépendance de y et transposition x↔y sur 400 pas, écarts
  maximaux **2,3841858·10⁻⁷ m** (un ulp autour de 2 m).
- Refus `Convergence`, `BackgroundContext`, `NotFinite`, `Domain` : état publié intact,
  géométrie couplée désarmée, reprise du mode total ensuite.
- Zéro allocation au pas couplé et au refus, allocateur compteur, arène scellée.

## 3. Cas limite HOS, ny=1

Commande : `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_mobile -- coupled`.

L=h=2 m, g=9,81 m/s² fourni, repos à 2 m, domaine jusqu'à 2,25 m, départ perturbatif nul ;
fond stationnaire linéaire de S253 prolongé analytiquement. HOS M=3, Q=16, K=256, 1 604 pas de
1 ms, plafond 4 000 par projection. Total comparé au HOS au même instant après chaque pas.
Tolérances antérieures : profil <2 %, harmonique <20 % à 128, décroissants en raffinant.

| amplitude | nx | profil max / a (%) | erreur harmonique (%) | dérive moyenne eta (m) | itérations max | affinages |
|---|---:|---:|---:|---:|---:|---:|
| 5 cm | 32 | 1,505218 | 1,556997 | 3,725·10⁻⁸ | 139 | 0 |
| 5 cm | 64 | 0,337548 | 0,564404 | 2,608·10⁻⁸ | 284 | 0 |
| 5 cm | 128 | **0,148100** | **0,345649** | 2,328·10⁻⁸ | 1 145 | 3 |
| 10 cm | 32 | 0,906532 | 1,613891 | 3,725·10⁻⁸ | 146 | 0 |
| 10 cm | 64 | 0,297990 | 0,714415 | 3,539·10⁻⁸ | 562 | 3 |
| 10 cm | 128 | **0,177888** | **0,531870** | 2,328·10⁻⁸ | 1 178 | 1 |

**Reçu.** L'affinage a réellement été exercé sur les cas fins. Les nombres diffèrent des anciens
bancs 2D multigrilles ; les critères sont ceux de S253, pas une identité avec ce préconditionneur.
Hauteur absolue fine : 0,074 mm et 0,178 mm. Aucune conclusion perceptive sur ces seuls nombres.
Le diagnostic `wet` imprimé initialement par le banc lisait eta perturbative après désarmement ;
il ne mesurait donc pas la topologie totale. Non utilisé pour la réception ni repris ici.

## 4. Ce qui reste à recevoir

Les bandes et l'éponge sont construites en 3D, mais le seul fond traversant uniforme ne reçoit
pas l'absorption d'un paquet ni la précision en houle progressive de S269–S274. Ces campagnes
restent dues, ainsi que le fournisseur B/W réel, la production GPU, la scène de mer étalée et
sa revue. A274 reste ouverte (plancher des lignes fantômes), A295 aussi (production et scène).
Techniques : MAC f32, Jacobi, départ chaud, affinage ; sans multigrille 3D ni GPU.
Les durées de banc, avec tâches concurrentes, ne sont pas une réception de coût.

## 5. Aperçu demandé

En préparation : deux ondes analytiques croisées, impulsion localisée, témoin sans impulsion,
images raster locales calculées depuis le véritable pas 3D. Aucun état δ sérialisé (I-17).
