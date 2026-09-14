# Flux coupés — S232, 2026-09-14

## Diagnostic avant correction du noyau

Base `4a48c74`. Distinguer mesure du débit et défaut du solveur avant toute reconstruction.
Le filtre S199 additionne `u*dx` sans ouverture ; son ordre ≈0,90 n'est pas celui du débit
ouvert. La contre-épreuve remplace seulement cette somme par `u*ouverture*dx`, ouverture
recalculée indépendamment depuis les hauteurs du fond. Aucun champ du solveur ne change.

| fond | Q32 | Q64 | Q128 (m²/s) | ordre |
|---|---:|---:|---:|---:|
| plat | 2,288878613e-4 | 2,295123513e-4 | 2,296743171e-4 | 1,947 |
| lisse | 2,252495855e-4 | 2,258485786e-4 | 2,260030263e-4 | 1,955 |
| tanh dite « marche » | 2,250029393e-4 | 2,256003443e-4 | 2,257517146e-4 | 1,981 |

Richardson final : 0,025 / 0,024 / 0,023 %. La troisième forme est lisse, pas une
discontinuité géométrique. Ces fonctionnelles ne reçoivent ni l'ordre local des vitesses,
ni l'advection (premier pas depuis le repos), ni toute géométrie coupée.
La piste du décentrage de face de S199 n'est donc pas démontrée par son ancien chiffre.

Un second contrôle trouve un défaut réel : le sous-échantillonnage à huit points du volume
fluide manque les triangles étroits. Domaine3×3, dx1, fond fourni[1,4;1;0,98] : les arêtes
de la cellule(1,0) sont1,2 et0,99 ; son aire triangulaire indépendante vaut
2,380947407e-4 m², mais `fluid_fraction` vaut zéro. Ses deux faces restent ouvertes.
La régression `thin_cut_wedges_remain_fluid_and_project_their_open_flux_s232` échoue
avant correction avec « triangle perdu ». Le miroir du fond fait partie du contrôle.

## Critères de correction

Intégrer exactement le profil linéaire déjà choisi, en f32, sans seuil supprimant des cellules.
Vérifier aire triangulaire contre calcul indépendant et bilan des quatre flux après le pas
sur les deux orientations. Conserver la tolérance de projection existante1e-5, la précision
pression, le repos, l'absence d'allocation et les refus atomiques. Rejouer le filtre corrigé.
Pas de nouvel opérateur de pression sans défaut discriminé qui l'exige.
