# Pression f32 du candidat δ — S231, 2026-09-14

## Critères déclarés avant modification

I-08 demande f32 dans δ ; l'exception d'ADR-139 concerne V seulement. Construire stockage,
opérateur, projection et réductions en f32. L'interface d'hôte f64 peut transporter exactement
les scalaires f32 entre callbacks, mais ne doit pas effectuer les additions en double précision.
Les rapports peuvent élargir un diagnostic déjà calculé ; cela ne modifie pas la simulation.

Comparer au candidat f64 de `88b98fe` avec le même exemple `delta_precision` : domaine8×4m,
rho1025, g9,81, dt0,002s, surface4+0,01sin(2π(i+0,5)/nx), fonds plat0,5m et bosse S199,
nx16/32/64/128, nz=nx/2 ; un pas, plus100 pas à32×16. Champs complets, rapport et stockage
publiés hors fenêtre de mesure. Les sorties brutes sont des produits locaux dans `code/target/`,
pas des snapshots δ de jeu ; ce banc ne construit aucune persistance runtime.

Conserver repos exact et les critères existants (projection32×16 : divergence<1e-5, filtre
spatial S199). Quantifier l'écart au témoin f64, sans exiger les mêmes bits. Pour la sonde de
débit S199, vérifier que l'écart de précision reste inférieur à son erreur spatiale déjà mesurée
(Richardson f64 :0,025% plat et0,963% lisse au niveau128). Aucun seuil physique nouveau.
Le résidu récurrent de CG ne suffit pas à attester `b−Ap` : vérifier aussi le résidu recalculé,
et ne jamais publier une convergence que le vrai résidu ne permet pas d'affirmer.

Comparer le coût à charge/qualité identiques, stockage exact et mesures répétées hors I/O.
Conserver l'arrêt coopératif, les refus atomiques et le compteur du tas. Ne pas réduire les
seuils de qualité pour faire passer f32 ; signaler explicitement ses limites si elles apparaissent.
Ce lot ne reçoit ni I-05 complet, ni B3, ni surface mobile/3D/faces coupées.

## Construction et réception

Les six tableaux de pression, l'opérateur, la projection et les sommes sont maintenant f32.
Le callback hôte transporte un f32 élargi en f64 ; chaque fusion revient en f32 avant addition.
`pressure()` expose désormais `&[f32]`. Les diagnostics publics élargissent leur résultat f32.

La conversion naïve échouait : neuf des dix cas dépassaient le vrai résidu relatif de 1e-6
(jusqu'à 4,45e-6), malgré la convergence du résidu récurrent. Le pas recalcule désormais
`b−Ap`, relance CG depuis cette correction si nécessaire, et conserve le plafond **global**
d'itérations. Stagnation ou plafond : rapport dégradé, sans fausse convergence. Une norme
calculée nulle avec des composantes fluides non nulles est refusée, pas appelée repos.

Les dix cas convergent sans relever le seuil : résidus réels f32 de 4,96e-7 à 8,70e-7.
Un test indépendant assemble les lignes en f64 depuis la géométrie et les entrées, sans
réutiliser l'opérateur ni son second membre stocké. Sur le fond coupé de test aux quatre
résolutions, il trouve respectivement 4,94e-7, 6,31e-7, 8,75e-7 et 8,06e-7, tous sous 1e-6.
Ce contrôle reçoit ces domaines, pas toutes les échelles numériques possibles.

Écart de champ : norme maximale de la différence divisée par le maximum absolu du témoin
pour chaque champ, sans division locale près de zéro. Maximum vitesse : 6,16e-6 relatif ;
pression : 8,73e-7, continuation de 100 pas comprise. À 128, l'écart relatif de débit au
témoin vaut 1,63e-7 sur fond plat et 5,04e-8 sur bosse, inférieur à l'erreur spatiale annoncée.
Le filtre S199 conserve ses verdicts : ordres 1,947 plat, 0,898 lisse et 0,895 marche.
Son empreinte passe de `0x0ad3f695685ca27a` à `0x3710c97033f99db7` ; répétabilité locale
reçue, ni identité avec f64 ni réception multiplateforme revendiquées.

## Stockage et coût complet

| cellules | stockage f64 (octets) | stockage f32 (octets) |
|---|---:|---:|
| 16×8 | 11 264 | 8 192 |
| 32×16 | 44 032 | 31 744 |
| 64×32 | 174 080 | 124 928 |
| 128×64 | 692 224 | 495 616 |

Économie exacte : 24 octets par cellule, soit environ 28 % du stockage compté du volume.
Aucune nouvelle allocation dans le pas ; abandon par échange des buffers conservé.

Même banc `block_cost` compilé pour les deux sources : domaine 8×4m, fond 0,4m,
surface 4+0,02sin, dt=1/60, trois échauffements puis onze mesures, vitesses réinitialisées.
Pas complet, hors impressions, plafond 512 itérations, sans limite temporelle injectée.

| cellules | itérations f64 → f32 | médiane f64 → f32 (ms) | maximum f64 → f32 (ms) |
|---|---|---|---|
| 16×8 | 30 → 30 | 0,0756 → 0,0822 | 0,0837 → 0,0823 |
| 32×16 | 60 → 61 | 0,5759 → 0,6303 | 0,5803 → 0,6435 |
| 64×32 | 113 → 114 | 4,5775 → 4,5874 | 6,3362 → 5,4911 |

Les deux variantes convergent. La nouvelle contrôle aussi le résidu réel et sa correction :
**aucun gain de vitesse reçu**. Mesures locales séquentielles Windows x86_64, pas classement
statistique ni borne murale. Le bloc 64×32 dépasse encore les 2 ms ; limites S230 inchangées.

## Reproduction et portée

Depuis `code/` : `cargo run -p water-core --release --offline --example delta_precision`,
`cargo run -p water-core --release --offline --example delta_filter`, puis
`cargo test --workspace --release --offline --quiet` : **409 réussis, 5 ignorés**.
Deux tests ajoutés : oracle indépendant et refus de sous-débordement. Les sept tests runtime
confirment refus atomiques, reprise, réductions bornées et absence d'allocation.
Depuis la racine, après construction release : `python outils/compare_delta_precision.py`
recompile le témoin `88b98fe` et le candidat courant avec le même banc de coût.
Les champs témoins ont été capturés avant conversion avec l'exemple ajouté en P2 (`82c3ae8`).
Sorties locales : `code/target/s231-f64.txt`, `s231-f32.txt` et `s231-cost/` ; les résultats
utiles sont consignés ici pour ne pas dépendre de la conservation de ces fichiers ignorés.

La précision **de pression** est reçue dans ce domaine. Ce n'est pas I-08 global : l'API de
temps du candidat reste f32. Flux coupés, gravité générale, surface mobile, 3D, intégration
B/W et admission temporelle demeurent hors réception. Aucune exception nouvelle à l'invariant.
