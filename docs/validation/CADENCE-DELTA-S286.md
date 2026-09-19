# S286 — cadence de calcul séparée de la cadence image

## Critères avant mesure

Suite A276 après S285. Comparer 16/32/48 ms à **temps de scène identique**, dans le chemin
Live → Layer ; compter les vrais pas et allocations. Repère hauteur S201 : 3 mm, pentes
publiées sans seuil inventé. Coût moyen par image **et** pire pas : un calcul trop long reste
incompatible avec I-05 même si les images intermédiaires sont bon marché. B reste à l'heure
de scène ; conserver le dernier profil δ entre pas est une approximation à mesurer.

## Correction intégrée de l'estimation

Le défaut préalable est reproduit : au même instant, un refus effaçait déjà une des huit
mesures de coût, puis huit appels suffisaient à oublier tout le passé pendant une pause.
`Layer::arbitrate` oublie maintenant une entrée par **16 ms simulées non financées**. Le reste
de temps est conservé ; pause et retour d'horloge n'effacent rien. Revenir en arrière réancre
l'horloge d'oubli. L'estimation reste une médiane de vrais pas réussis, sans garantie de p99.
Le test échoue avant correction (7 entrées au lieu de 8 sans temps écoulé), puis passe.

## Banc et domaine

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --delta-cadence
cargo test --release --offline --locked --manifest-path viewer/Cargo.toml
```

Trois Layer successifs par image, houle S275, grille 128×52, durée 3,072 s, 192 images de
16 ms. Deux scénarios : perturbation initialement nulle, puis onde gaussienne 0,6 m S277.
Multigrille mobile, grille de fond, pression initialisée depuis le pas précédent ; pas de
GPU δ, parallélisme, réduction spatiale ou interpolation temporelle. Budget **de banc**
1 000 ms pour éviter que l'admission masque les dépassements. Le noyau reçoit encore
l'horloge figée du banc : aucune prétention au respect d'un délai matériel.

Les pas 32/48 ms sont privés au banc ; **l'afficheur interactif reste à 16 ms**. Live maintient
le dernier profil entre échéances, ne rattrape pas plusieurs pas, renaît si l'heure recule
ou dépasse la prochaine échéance. Le banc refuse toute renaissance imprévue, extinction ou
erreur. Un refus imprime sa durée complète puis arrête ; aucun refus n'est survenu ici.

Hauteur et pente : profil Hermite CPU avec fondus, échantillonné tous les 0,5 m sur la bande
à y=0. Ce sont des maxima **échantillonnés**, pas des bornes de tout l'interpolant. La colonne
« synchronisée » ne retient que les images où le candidat calcule réellement ; elle sépare
la différence d'intégration de l'erreur liée au maintien entre calculs. Référence numérique
16 ms, pas oracle physique absolu ni verdict visuel.

## Mesure du 2026-09-19, Windows release

Ryzen AI 7 350 ; secteur vérifié aux deux bornes (BatteryStatus=2, 98 %). Un passage,
trois trajectoires entrelacées ; chiffres locaux observés, aucun certificat de temps maximal.

| onde initiale | pas | calculs | moyenne/image | médiane/calcul | p99/calcul | maximum/calcul |
|---|---:|---:|---:|---:|---:|---:|
| 0 m | 16 ms | 192 | 22,913 ms | 23,081 ms | 29,086 ms | 32,610 ms |
| 0 m | 32 ms | 96 | 12,617 ms | 25,453 ms | 31,846 ms | 31,846 ms |
| 0 m | 48 ms | 64 | 8,813 ms | 26,836 ms | 37,023 ms | 37,023 ms |
| 0,6 m | 16 ms | 192 | 25,360 ms | 25,120 ms | 37,279 ms | 46,868 ms |
| 0,6 m | 32 ms | 96 | 13,150 ms | 25,815 ms | 32,319 ms | 32,319 ms |
| 0,6 m | 48 ms | 64 | 9,108 ms | 26,699 ms | 32,708 ms | 32,708 ms |

| onde | pas | hauteur, toutes images | hauteur synchronisée | pente max (m/m) |
|---|---:|---:|---:|---:|
| 0 m | 32 ms | 1,770 mm | 1,142 mm | 0,000395 |
| 0 m | 48 ms | 3,532 mm | 2,281 mm | 0,000781 |
| 0,6 m | 32 ms | **4,445 mm** | **4,445 mm** | 0,000763 |
| 0,6 m | 48 ms | **8,916 mm** | **8,916 mm** | 0,001526 |

Zéro allocation sur les 579 updates de chaque scénario. Images sans calcul : maximum
0,0583 ms. Le maintien n'est donc pas le seul problème : sur l'onde injectée, l'écart dépasse
3 mm **aux instants calculés eux-mêmes**. Une interpolation d'affichage ne suffirait pas.

## Décision et suite

**Pas d'activation générale des cadences 32/48 ms.** La houle seule à 32 ms passe le repère
sur les points mesurés, l'onde ne le passe pas. L'intégration lente doit être corrigée/reçue
avant généralisation ; ne pas choisir un seuil plus grand après coup. I-05 reste ouvert :
9–13 ms moyens par image, et plus de 30 ms sur certaines images calculées, contre 2 ms.

Le levier a été mesuré, son gain est insuffisant et son domaine de qualité trop limité.
Priorité suivante : construire un chemin δ moins cher par pas, comparer le port GPU à
l'organisation des passes CPU ; décider l'architecture avant 3D/deuxième domaine. La reprise
de la cadence attend ce chemin, ou un schéma temporel reçu sur l'onde. Coût des échecs de
solveur/budget **non qualifié** : le dispositif de mesure est prêt, mais aucun échec représentatif
n'a été observé. Ne pas extrapoler à partir d'un refus artificiel d'arguments.
