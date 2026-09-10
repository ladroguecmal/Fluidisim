# Trajet de sillage — S150, 2026-09-10

ADR-103 ; exécuter depuis code/ : `cargo run -p water-core --release --example wake_motion`.
Exécuté aussi en debug : hash identique `13f0b5fd14a4ac9b`.

## Chaîne reçue

Objet7/cause1, charge100N, sigma1m, deux tronçons de2s : vitesse(2,0), puis(0,2)m/s.
Pression résolue transportée en WPRS196 octets ; objet Wake détruit avant décodage,
admission authentifiée de fixture au journal, préparation W puis composition avec B spectral32.
Ancre monde1 000 000m, référentiel7/cellule9, g9,81 et rho1025 injectés.

49 points sur [-6,12]², neuf instants0/1/1,999999/2/2,000001/3/4/6/8s :441 comparaisons.
Candidat pression128×128, cutoff6/sigma ; référence f64/libm256×256, même coupure.
Le trajet et P0=100/(2pi) de référence sont écrits indépendamment du constructeur ;
le champ de référence utilise la quadrature complète et le noyau f64 S89.
Tolérance fixée avant exécution :1e-5 absolu par grandeur ; aucun relèvement.

| Grandeur B+W | Écart absolu maximal |
|---|---:|
| hauteur | 6,050583e-7m |
| vitesse verticale/dérivée temporelle | 1,687880e-7m/s |
| vitesse horizontale X/Y | 4,958611e-8 / 5,721517e-8m/s |
| normale X/Y/Z | 2,302057e-8 / 4,128715e-9 / 1,094741e-7 |

Après extinction à4s, puissance nulle à6s et8s ; énergie0,3198232J aux deux dates,
écart relatif inférieur à1e-5. Hauteur perturbative maximale encore0,000610994m
sur les points à8s : le mouvement n'est pas effacé avec le forçage.

## Refus et contrôles

Quatre tests debug/release : intégrale de charge et effet sigma×2 (pression/4),
continuité du virage, charge nulle, valeurs non finies/négatives, sous-débordement,
trop de tronçons, durée nulle, dépassements temporels et spatiaux. Transport et
admission idempotente reçus. Au repos, charge positive enfonce l'eau ; charge×2
multiplie hauteur par2 et énergie par4 exactement sur la fixture.

## Portée

Premier raccordement objet/charge/trajectoire déclaré, pas un modèle de coque reçu.
La référence raffinée teste cette fixture et cette coupure, pas un Kelvin stationnaire,
un changement de repère, une mer avec courant ou la résistance renvoyée au corps.
Les contrôles de domaine WPRS ne certifient pas la précision de toutes les recettes.
Aucun nouveau benchmark canonique ni budget matériel cible revendiqué.

S149-1 partielle, W4 construit plus avant. **S150-1 : recevoir l'alimentation progressive
par le mouvement hôte**, sans redéposer l'historique déjà émis ni supprimer les ondes
à l'arrêt ; puis B2. L'hôte devra annoncer sa résolution temporelle et ses refus
(capacité, téléportation/changement de repère). Pas de nouveau prérequis sur B.
