# S67 — Les battements survivent à la fenêtre, 2026-09-08

S64-3 / A187. **Les 6,612 % historiques sont reproduits et expliqués par les covariances
sur la fenêtre finie.** Pas de défaut de normalisation ni de précision expliquant cet écart.
Le diagnostic de S66 à 32 composantes ne suffisait pas ; cette mesure porte explicitement
sur les 256 composantes et restaure les phases historiques dans le code de test uniquement.

## Référence sur la grille finie

Même table de composantes que Background, même instant 1735689600000000 µs, Hs configuré
1,2 m, Tp 6 s. Grilles centrées à zéro, pas 3 m, côtés 1024/2048 points (3072/6144 m),
entièrement dans la portée du référentiel. Six graines de S65 et une réalisation historique.
La table est évaluée en f64, en conservant la phase temporelle entière.

Pour une composante sinusoïdale, la moyenne sur la grille se calcule avec la somme géométrique
finie. Dans une dimension, avec incrément a=k·pas, r=a−round(a), N points et première
phase φ₀ :

`moyenne exp(i2πφ) = exp(i2π[φ₀+(N−1)r/2]) · sin(πNr)/(N sin(πr))`.

Pour r=0, le facteur vaut 1. Le produit des deux facteurs spatiaux donne la moyenne 2D.
`sin(a)sin(b) = [cos(a−b)−cos(a+b)]/2` donne les seconds moments, puis
`Var(Σηᵢ) = ΣVar(ηᵢ) + 2Σᵢ<ⱼCov(ηᵢ,ηⱼ)` donne la décomposition. Cette référence reproduit
la **grille finie**, pas une moyenne sur une fenêtre infinie ni une vérité du spectre.

Tests ordinaires : somme directe contre formule à fréquence nulle, alias exact et fréquence
oblique ; moments directs à 1/32/256 composantes sur 48² points, écart absolu inférieur à
1e-12 m². Monochromatique : covariance croisée nulle. Dans le diagnostic, deux comparaisons
directes supplémentaires sur 1024² points à 256 composantes (historique et graine nominale)
donnent des écarts de variance f64 inférieurs à 2e-15 m². Aucun nouveau chemin de production.

## L'écart historique et sa cause

À 256 composantes, phases historiques i×0x9E3779B9, fenêtre 3072 m :

| Mesure | Valeur |
|---|---:|
| Hs production | 1,279349902 m |
| Hs par moments finis f64 | 1,279349889 m |
| Hs associé à la seule somme des variances individuelles | 1,200001971 m |
| Contribution croisée à la variance | +0,012295713 m² |
| Dont paires d'indices voisins i,i+1 | +0,008987660 m² (environ 73,1 %) |

La contribution croisée suffit à expliquer l'écart à 1,2 m ; la précision f32 ne le produit
pas. Les variances individuelles sont correctement normalisées sur ce montage, mais leur
somme n'est pas la variance du champ total sur cette fenêtre. La remplacer dans le test Hs
supprimerait précisément ce qu'on cherche à mesurer.

À fenêtre **6144 m**, Hs historique devient **1,218126498 m**, soit environ **+1,511 %** ;
la covariance croisée tombe à **0,002739891 m²**. Le constat S64 excluait un défaut de pas,
mais sa phrase « ni la fenêtre ni le pas » allait trop loin : doubler ici la fenêtre réduit
fortement l'écart. Cela ne garantit pas une réduction monotone pour toute graine.

## Pourquoi le spectre dense change l'échelle utile

Les produits entre composantes contiennent les différences de vecteurs d'onde. Pour deux
voisines, la longueur de battement mesurée est `1 / ||kᵢ₊₁−kᵢ||` (k en tours/m).

| Nombre de composantes | Battement voisin minimal | Maximal |
|---|---:|---:|
| 32 | 161,391 m | 2361,333 m |
| 256 | 1276,845 m | 20208,431 m |

Les périodes individuelles restent dans [Tp/2,2Tp], mais densifier rapproche les vecteurs
d'onde : les interférences peuvent varier sur des distances beaucoup plus longues que les
ondes individuelles. Le support spatial du second moment dépend donc aussi de ces différences.
Ces longueurs radiales expliquent une échelle, sans fournir à elles seules une borne pour
la fenêtre carrée ni pour les réalisations possibles.

## Six graines actuelles à 256 composantes

Hs issu des moments finis, en mètres :

| Graine | Fenêtre 3072 m | Fenêtre 6144 m |
|---|---:|---:|
| 0 | 1,152468002 | 1,181206425 |
| 1 | 1,129132579 | 1,182752011 |
| 2 | 1,197332586 | 1,196133328 |
| 3 | 1,174829938 | 1,229216659 |
| 20260905 | 1,270122885 | 1,227240940 |
| u64::MAX | 1,112765695 | 1,184029017 |

À 3072 m, la valeur issue des variances individuelles reste entre 1,199990521 et
1,200006943 m ; les termes croisés changent de signe avec les phases. Les Hs directs
reproduisent les mesures S65 à quelques dizaines de nanomètres près. La graine 3 montre
qu'élargir la fenêtre ne réduit pas toujours l'écart absolu. Les observations sont des
fluctuations de réalisation finie, pas une preuve de biais moyen du générateur.

## Portée, vérification et suite

Commande depuis code/ : `cargo test --offline --release spectre_dense_s67 -- --ignored --nocapture`.
28 montages par moments finis, deux contrôles directs denses, diagnostic réussi en 49,40 s.
La formule peut servir à explorer des ensembles sans répéter un million de points à chaque
graine ; sa validité est limitée au présent fond sinusoïdal, à sa table et à cette grille.

**S64-3 close, A187 expliqué.** La calibration de tolérance n'est pas effectuée : six graines
ne garantissent rien sur la population de mers admissibles. S64-2 peut être instruite par ADR
avec cette cause connue, sans supposer qu'elle doit nécessairement resserrer le seuil.
S66-1 reste à trancher pour le contrôle d'homogénéité. Production, références, fenêtres et
tolérances inchangées ; C04 et homogénéité restent en échec. S63-1 reste le blocage de B2.

Suite complète : **135 tests réussis** (43 cœur + 92 harnais), **cinq ignorés** par défaut.
Le diagnostic S67 ignoré a été exécuté séparément. Check : deux scénarios verts, hashs
0x9babd7e12935c263 et 0xd57d81f47d9f8611 inchangés. Aucun invariant invalidé, aucun état
sérialisé ; campagne physics générale non répétée, tous les ajouts sont sous cfg(test).
