# S206 — Ce que l'eau coûte par image, et ce qui peut le réduire

2026-09-13. **A247**, bloquant du jalon J1 ([feuille de route](../FEUILLE-DE-ROUTE.md)), sous
ADR-125 (60 images/s, eau 2 ms) et ADR-127 D7 (le budget est une cible ; une incompatibilité
s'arbitre explicitement, aucune fonctionnalité ne se retire en silence). Banc
`code/water-core/examples/frame_cost.rs`, sans dépendance ; la scène est celle de `render_impact`,
incluse comme module et non recopiée.

## 1. La scène, déclarée avant la mesure

Observateur S201 (640×360, 50°), mer S201 **Hs 1,5 m** (JONSWAP N32), un impact S203 (N256,
R 52 m, A 56 s) à +3 s, composé par le chemin hôte : `Prepared::sample_world_batch` dans
l'emprise, `Background::eval` hors emprise, budget d'impact ADR-128. **Charge : une grille de
sommets projetée**, un sommet par `c` pixels, intersection du rayon avec z = 0 jusqu'à 600 m.
C'est ce qu'un maillage de surface évalué sur CPU demande à chaque image — pas le lancer de
rayons hors ligne de S201/S203. Temps mur, médiane et maximum de 11 images après 3 de chauffe,
AMD Ryzen AI 7 350 (16 fils matériels), release.

```text
cargo run -p water-core --release --example frame_cost frame
cargo run -p water-core --release --example frame_cost levers
cargo test -p water-core --example frame_cost
```

## 2. Quelle densité l'observateur exige

Au point d'impact (28,86 m), S203 a mesuré λ = 3,35 m sur **11,05 px le long de la visée**. La
plus courte composante de l'impact, λ/2 = 1,675 m, y occupe 5,5 px ; la plus courte de B —
bande jusqu'à 4 fp, λ = 3,51 m — environ 11,6 px. Échantillonner deux fois par longueur d'onde
demande donc **c ≤ 5,8 px pour B et c ≤ 2,75 px pour les anneaux de l'impact** à cette distance.
Une grille à 8 px n'est pas une version dégradée de la scène : elle **perd les anneaux**, c'est-à-
dire l'impact visible. Elle est mesurée comme borne basse de coût, pas comme option.

## 3. Un fil : l'incompatibilité est d'un ordre de grandeur

Deux exécutions ; la seconde est archivée avec les leviers.

| pas | sommets | dans R | B seul, méd. | image B+W, méd. / max | B | B+W dans R | image ÷ 2 ms |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 8 px | 2 240 | 1 918 | 2,94 – 3,01 ms | 17,2 – 18,5 / 22,8 ms | 1,31 µs | 8,7 – 9,4 µs | 8,6 – 9,2 |
| 4 px | 9 044 | 7 652 | 11,7 – 12,0 ms | 72,4 – 74,0 / 84,2 ms | 1,30 µs | 9,2 – 9,4 µs | 36 – 37 |
| 2 px | 36 160 | 30 614 | 42,0 – 42,8 ms | 279,8 – 293,1 / 306,6 ms | 1,17 µs | 8,9 – 9,4 µs | 140 – 147 |

**86 % des sommets tombent dans l'emprise** : la grille projetée est dense au premier plan, et
l'emprise de 52 m contient l'observateur (S203). **B seul dépasse déjà 2 ms à 8 px.** À la
densité qui voit les anneaux (2 px), l'image coûte 140 fois le budget.

## 4. Levier 1 — le parallélisme

Fils lancés et rejoints **dans** l'image, ce que paierait un hôte sans groupe de fils persistant ;
le lancement seul est mesuré à part. Dans les quinze cas, **les bits de chaque sommet sont
identiques** au calcul sur un fil — le découpage ne change rien au résultat (essai
`threads_do_not_change_any_bit`).

| pas | 1 fil | 2 | 4 | 8 | 16 fils | accélération à 16 | lancement seul à 16 | ÷ 2 ms à 16 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 8 px | 18,19 | 12,20 | 7,39 | 5,01 | **3,63 ms** | ×5,0 | 1,57 ms | 1,8 |
| 4 px | 71,57 | 45,09 | 26,91 | 16,22 | **10,05 ms** | ×7,1 | 1,39 ms | 5,0 |
| 2 px | 289,07 | 178,92 | 105,66 | 61,34 | **36,19 ms** | ×8,0 | 1,24 ms | 18,1 |

Sur 16 fils matériels (8 cœurs), l'accélération plafonne vers ×8. Même en retirant le lancement,
la scène à 2 px reste vers 35 ms : **le parallélisme seul ne ferme pas l'écart**, et il consomme
tous les cœurs de la machine pour l'eau — ce qu'ADR-125 ne dit pas pouvoir faire (« sans
pipeline parallèle mesuré, somme conservatrice »).

## 5. Levier 2 — la table radiale à matrice de Bessel

Le champ radial ne dépend que de r et de t. S203 construisait une table η(r), η'(r) par
échantillons directs : 2,7 ms par image, parce que chaque échantillon réévalue N fonctions de
Bessel. Or `J0(k_n r_i)` et `J1(k_n r_i)` **ne dépendent pas du temps** : précalculées une fois
par impact, il ne reste par image que N phases et N×M produits. Noyau mesuré sur tableaux de la
bonne taille, valeurs synthétiques — le coût ne dépend que des tailles ; l'exactitude est celle
de la table S203, mêmes valeurs aux nœuds à l'ordre de sommation près, Hermite entre les nœuds.

| pas de table | M | noyau par image | mémoire par impact | erreur table | Hermite par sommet | impacts dans 2 ms | 4 096 impacts |
|---:|---:|---:|---:|---:|---:|---:|---:|
| λ/16 = 0,209 m | 249 | **0,027 ms** | 502 Ko | 0,0061 mm | 8,1 ns | 75 | 109 ms, 2,0 Go |
| λ/8 = 0,419 m | 125 | 0,016 ms | 254 Ko | 0,090 mm | 8,8 ns | 122 | 67 ms, 1,0 Go |

**W cesse d'être le goulot** : de ~9 µs par sommet à 27 µs par impact plus 8 ns par sommet, un
facteur **100** sur la construction de table, sans perte visible (0,006 mm contre 3 mm de
tolérance). Estimation à 4 px, un fil, avec ce levier : 11,7 ms de B + 0,03 ms de noyau + 0,06 ms
d'Hermite ≈ **11,8 ms — c'est B qui reste**.

**`paquets_W_max = 4096`** (ADR-012 §3) : 109 ms par image et 2,0 Go à ce pas, contre 2 ms et
les 384 Mo du profil mémoire d'ADR-012. La valeur n'est ni tenable ni de celles qu'un profil a le
droit de déclarer : I-16 veut qu'une telle capacité **se calcule** à l'initialisation à partir des
coûts mesurés. Mesurée ici : 75 impacts de ce type dans 2 ms, avant tout coût de B.

## 6. Ce qui reste : B sur CPU

B à 32 composantes coûte **1,2 à 1,3 µs par sommet** (48 ns par composante, BANC-B1-S146,
retrouvé). À la densité qui résout la scène (2 px, 36 160 sommets) : **42 ms sur un fil**, et — estimation, non mesurée —
vers 5 à 6 ms sur 16 fils avec un groupe persistant et l'accélération ×8 du §4 : encore trois
fois le budget, en occupant tous les cœurs. Leviers non mesurés, techniques : évaluation de B en structure de tableaux et
vectorisée (sans dépendance ; le sinus polynomial à branches de quadrant ne se vectorise pas tel
quel), nombre de composantes décroissant avec la distance (dégradation prévue par ADR-012, à
recevoir par B1), densité adaptative plutôt que grille uniforme. Levier non mesurable ici :
**l'évaluation sur GPU**, que le corpus prévoit depuis ADR-003 (« seules des phases repliées
passent au GPU », I-08) et qu'aucun hôte du dépôt ne peut exercer.

## 7. Verdict

**Incompatible** avec 60 images/s / eau 2 ms sur cette machine, pour cette scène, sur CPU, à
toute densité qui montre l'impact : 280 ms sur un fil, 36 ms sur seize. L'écart n'est pas un
défaut de W — un levier mesuré le divise par cent — c'est **l'évaluation de B par sommet sur
CPU**. L'arbitrage est posé dans la feuille de route (§4) et la file active ; la décision technique
qui en découle est [ADR-129](../adr/ADR-129-chemin-image-de-w-par-table-de-bessel.md).

## 8. Ce qui n'est pas mesuré

- aucun GPU, aucun groupe de fils persistant (lancement mesuré à part, pas retiré) ;
- une seule machine, un seul observateur, un impact, aucun sillage ni pression dans l'image ;
- le noyau L2 sur valeurs synthétiques : son coût est reçu, son intégration dans la bibliothèque
  ne l'est pas ;
- ni l'évaluation vectorisée de B, ni la densité adaptative, ni le nombre de composantes par
  distance ;
- la variabilité : deux exécutions, 7 % d'écart au plus sur les médianes ; les maximums ne sont
  pas des bornes.
