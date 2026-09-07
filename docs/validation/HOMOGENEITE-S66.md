# S66 — L'homogénéité compare deux réalisations locales, 2026-09-08

Action S65-1. Après raccordement de la graine, le cas nominal rend 1,397506641 pour un ratio
attendu à 1, tolérance 15 %. Son commentaire attribuait ce contrôle à la précision spatiale.
**Cet échec est reproduit en f64 ; il est dominé par les interférences sur la fenêtre.**
Aucun changement du solveur, des phases, de la fenêtre nominale, des références ou du verdict.

## Protocole et séparation des causes

La table de 32 composantes de Background reste exactement celle du scénario C18 : Hs 1,2 m,
Tp 6 s, instant 1735689600000000 µs. Deux fenêtres centrées à x=0 et x=3000 m, pas 3 m ;
côtés 144, 576 et 1536 m. Six graines fixées avant calcul : 0,1,2,3,20260905,u64::MAX.
La plus grande fenêtre reste dans la portée : x maximal 3765 m. Une fenêtre de 3072 m à
3000 m sortirait du rayon 4096 m : elle ne peut pas être reprise directement du test Hs.

Pour chaque point, comparer :

- le champ produit par eval, avec phases spatiales et sommation f32 ;
- la même table évaluée en f64 pour projection spatiale, sinus et sommation, en conservant
  la phase temporelle entière ;
- la somme des variances de chaque composante évaluée séparément en f64.

Ce diagnostic partage les composantes du générateur : il isole leur évaluation, sans valider
le modèle spectral ni la phase temporelle. Les coordonnées utilisées sont représentables
exactement dans les deux chemins. Les variances sont calculées sur les mêmes points.

L'identité appliquée est : Var(Σηᵢ) = ΣVar(ηᵢ) + 2Σᵢ<ⱼCov(ηᵢ,ηⱼ).
Le résidu entre variance totale et somme des variances individuelles mesure donc les termes
croisés sur cette fenêtre ; les supprimer changerait la grandeur physique mesurée.

## Résultats

Ratio variance loin/proche en production :

| Graine | 144 m | 576 m | 1536 m |
|---|---:|---:|---:|
| 0 | 0,532792352 | 1,138533604 | 1,008039967 |
| 1 | 1,392764794 | 1,029011794 | 0,970724071 |
| 2 | 0,855699821 | 0,741527829 | 0,975808008 |
| 3 | 0,990723741 | 0,837107510 | 0,937355279 |
| 20260905 | 1,397506641 | 1,079318732 | 0,943614087 |
| u64::MAX | 1,807574033 | 0,773179789 | 1,013642700 |

Sur 144 m, quatre graines sur six échouent au seuil existant. Sur 576 m, trois échouent ;
sur 1536 m, aucune. Cela ne calibre pas la tolérance : six réalisations ne donnent aucune
garantie sur les autres. L'effet de fenêtre n'est pas monotone pour chaque graine.

Nominal, fenêtre 144 m :

| Grandeur | Proche | Loin |
|---|---:|---:|
| Variance totale f64, m² | 0,095377406 | 0,133290526 |
| Contribution croisée, m² | 0,006459304 | 0,046176100 |
| Somme des variances individuelles, m² (par différence) | 0,088918102 | 0,087114426 |

Ratio f64 **1,397506306**, différence avec production **3,35e-7**. Le ratio des sommes de
variances individuelles vaut **0,979715316**. La contribution croisée augmente de
0,039716796 m² alors que la variance totale augmente de 0,037913120 m² ; elle explique
l'augmentation et compense une légère baisse de la partie individuelle.

Sur les dix-huit couples, le plus grand écart production/f64 du ratio est environ **6,94e-6** ;
le plus grand écart ponctuel de hauteur est **4,01e-5 m**. Ces erreurs existent mais sont très
inférieures à l'écart nominal de 39,75 %. Aucun défaut de précision expliquant le refus établi.

## Portée et suite

Une variance locale d'une somme d'ondes conserve des covariances liées aux phases. Une propriété
d'ensemble ne devient pas une égalité entre deux petites fenêtres d'une réalisation. Les
composantes proches peuvent avoir des battements longs ; augmenter la fenêtre réduit ici le
résidu, sans preuve uniforme ni choix d'une nouvelle fenêtre nominale.

S65-1 close : cause du refus isolée. **A188** corrige l'attribution du contrôle. **S66-1** garde
la décision à prendre : séparer diagnostic statistique et contrôle de précision, avec témoin
de défaut injecté avant toute nouvelle assertion. Le présent cas reste en échec. On ne remplace
pas sa variance par sa seule partie individuelle pour le rendre vert.
S64-3 (A187 à 256 composantes) reste à mesurer ; ce résultat à 32 composantes ne le clôt pas.
S64-2 reste bloquée ; S63-1, couche dispersive de B2, demeure ouverte.

## Reproduction et vérification

Depuis code/ : `cargo test --offline --release balayer_homogeneite -- --ignored --nocapture`.
Diagnostic : 18 couples, terminé en 9,46 s. Code sous cfg(test), sans nouvelle API publique.
Test ordinaire : la référence f64 conserve le refus nominal tandis que les variances
individuelles diffèrent de moins de 3 % ; témoin monochromatique à deux positions, contribution
croisée exactement nulle. Les seuils de ce test distinguent ces mécanismes, sans calibrer
une tolérance du produit. La campagne générale ne nécessite pas de nouvelle mesure : aucun
chemin de production n'est modifié, seul son commentaire reçoit la limite établie ici.

Suite complète : **134 tests réussis** (42 cœur + 92 harnais), **quatre ignorés** par défaut ;
le nouveau diagnostic ignoré est exécuté explicitement ci-dessus. Check : deux scénarios verts,
hashs 0x9babd7e12935c263 et 0xd57d81f47d9f8611 inchangés. I-02/I-03/I-06 inchangés,
I-08 respecté par toutes les fenêtres. Aucun état de simulation sérialisé.
