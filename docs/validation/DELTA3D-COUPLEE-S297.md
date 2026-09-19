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

**Livré localement** : `viewer/captures/s297/couplage_3d.gif` et quatre PNG aux instants
0 / 1,5 / 3 / 6 s. 121 images, 20 images/s ; les six secondes physiques sont lues à vitesse
réelle (dernière image tenue 50 ms). Images inspectées à plusieurs instants, aucun état δ
sérialisé (I-17). Commandes depuis la racine :

```powershell
cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_preview -- viewer/captures/s297
python outils/apercu_delta3d.py
```

Domaine 32×24×12, maille 0,25 m, emprise 8×6 m, repos 2 m ; 1 200 pas de 5 ms,
plafond 4 000, éponge de 1 m sur les quatre côtés à 2 s⁻¹. Fond : deux ondes stationnaires
linéaires de profondeur finie, amplitudes 10 et 7 cm, k=π/4 et π/3, directions (1,0) et
(0,6 ; 0,8), g=9,81 fourni. Impulsion initiale `0,18(1−r)exp(−r)` m,
`r=((x−3)²+(y−2,5)²)/(2·0,55²)`, vitesses perturbatives nulles. Témoin identique sans
impulsion, calculé séparément : la différence isole son effet, interactions comprises.
Pas une mer spectrale de production ni une réception de S269.

Vue orthographique : direction vers l'œil (0,48 ; 0,64 ; 0,6), projection de 43 pixels/mètre,
champ centré sur le domaine ; pas de champ perspectif. À gauche : surface totale à son échelle
réelle. À droite : différence au témoin, **relief amplifié ×4**, couleurs signées pour la lire.
Habillage de banc : éclairage fixe, grille métrique, couleurs ; pas d'écume, réfraction, ciel
physique ni matériau de production. Aucun verdict de réalisme demandé sur cet habillage.

Empreintes FNV-1a des PPM complets (en-tête et pixels) :

| image | âge | FNV |
|---|---:|---|
| frame_0000.ppm | 0.00 s | `323783da806686fd` |
| frame_0030.ppm | 1.50 s | `81abecd9e350e151` |
| frame_0060.ppm | 3.00 s | `57b3e9756f84b8ec` |
| frame_0120.ppm | 6.00 s | `cc39f67fe6af1134` |

Calcul des deux trajectoires terminé sans refus, 83 itérations au maximum, aucun affinage,
divergence franche maximale **9,126218·10⁻⁶** (<10⁻⁵). Le relief initial évolue en anneaux,
puis se déforme et se disperse. Cela montre le calcul ; cela ne reçoit pas encore les critères
perceptifs de la porte B. **Suite visible** : raccorder la production GPU à la scène de mer.

## 6. Vérification globale

`cargo test --manifest-path code/Cargo.toml --workspace --release --offline` : **537 réussis**
(420 cœur, 19 exécution δ, 2 géométrie, 1 table radiale, 95 harnais), 18 ignorés, aucun échec.
Afficheur inchangé. Les bancs HOS et aperçu ont été exécutés explicitement en plus des tests.
