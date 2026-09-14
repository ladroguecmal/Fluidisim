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

## Correction consommée et résultats

`cut_fraction` intègre le rectangle plein et le trapèze coupé du profil linéaire ; la formule
reste f32 et n'ajoute aucun stockage. La fraction positive réintègre les triangles dans les
lignes de pression et dans la correction des faces du pas réel. L'opérateur n'a pas été
remplacé pour compenser un défaut du banc.

La régression initialement rouge passe sur le triangle et son miroir : aire indépendante
respectée, injection d'une vitesse non nulle, pas non dégradé puis somme indépendante des
quatre flux par cellule sous le critère normalisé1e-5. Un deuxième test couvre rectangles,
trapèzes, triangles, symétrie et additivité de la partition. Les oracles de pression S231
passent encore (résidus indépendants de4,94e-7 à8,75e-7 sur leurs quatre résolutions).

| fond après correction | Q128 (m²/s) | ordre 32/64/128 | itérations à128 |
|---|---:|---:|---:|
| plat | 2,296743171e-4 | 1,947 | 219 |
| lisse | 2,260028664e-4 | 1,957 | 347 |
| transition tanh | 2,257532700e-4 | 1,966 | 330 |

Les niveaux32/64 conservent les valeurs de la première table. Richardson arrondi inchangé.
Deux causes de changement de bits sont séparées : correction de la mesure seule,
empreinte `0x4a26292e4a905a8b`, puis correction géométrique, `0xc5ab1eadb094d058`.
La référence S231 `0x3710c97033f99db7` mesurait une autre fonctionnelle.
Le banc impose désormais son critère annoncé d'ordre≥1,8 et vérifie le rejeu local en bits.

La découpe n'est exécutée qu'à la configuration. Aucun tampon ni boucle du pas ajoutés ;
le nombre de cellules actives et d'itérations peut néanmoins changer (326→330 au dernier
cas). Aucun gain de coût revendiqué ; la mémoire allouée est inchangée, I-05 reste ouvert.

## Reproduction et limites

Depuis la racine :

```text
cargo test --manifest-path code/Cargo.toml -p water-core --release --offline delta_projection
cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta_filters
cargo test --manifest-path code/Cargo.toml --workspace --release --offline --quiet
```

Les douze tests unitaires δ passent. Le contrôle ne transforme pas une auto-convergence
de débit en solution manufacturée analytique : le libellé initial S199 était trop fort.
Le bilan discret prouve sa cohérence, pas l'ordre local de la vitesse ni la stabilité
d'une cellule arbitrairement petite. Pas de réception de discontinuité du fond, de surface
mobile, d'advection générale, de 3D ou de scénario B3. La suite utile est de construire
l'évolution de surface consommant les flux, avec son oracle et son domaine annoncés.

Suite finale : **411 réussis, 5 ignorés**, dont sept tests runtime δ (budget, reprise,
refus et absence d'allocation). Navigation active sans erreur ; avertissements préexistants
uniquement. Réception locale de `4f3eacd`, aucun code modifié après cette suite.
