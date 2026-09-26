# La fusion et la séparation de domaines δ, en référence — S396

2026-09-26. **C8a** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)
D5), en référence CPU : **fusion = union, séparation = partition** d'ensembles de blocs, les critères et le délai
d'[ADR-006](../adr/ADR-006-cellules-domaines-solveurs.md) §3–4. Session cloud, un fil, sans carte graphique : les durées sont
celles de ce conteneur. Liste **4.9** (fusion et séparation sans rupture) ; 1.5, 1.6.

## Reproduire

- Commit `e0e51437` ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s396 -- --nocapture` — sept essais, une
  seconde ; ligne `S396 aller-retour` (vitesse et débit perdus à la coupure).
- `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_fusion -- <fusion|separation>`
  — ligne `FUSION_S396`, 43 s par cas ; `FUSION_A=<s>` force l'instant de la fusion. Valeurs attendues : §3.

## En une phrase

Deux domaines δ du même réseau **fusionnent** quand leurs ensembles actifs, dilatés du rayon de couplage de 4 m, se
touchent — et la fusion n'est qu'une recopie, au bit, de leurs états dans un domaine qui couvre les deux : décidée par ce
critère, elle reste à **0,26 %** de l'amplitude du domaine unique tenu depuis le départ ; faite après que les ondes ont frappé
les murs des parties, à 3,4 puis 8,5 % ; **une séparation** ne saute pas, puis ses nouveaux murs réfléchissent.

## 1. La construction

**Les ensembles** (`code/water-core/src/domain_blocks.rs`). Un domaine est un **ensemble de blocs** d'un réseau commun
(ADR-006 §3). Les domaines de δ couvrent toute la profondeur d'eau (ADR-175) : un bloc est ici une **colonne** de 8 × 8
mailles — simplification déclarée des blocs de 8³ d'ADR-006. `BlockSet` : trié, capacité réservée auprès de l'hôte (I-06),
union, bornes, composantes par union-find sans allocation. **Une seule relation** pour fusionner et pour séparer : deux blocs
sont **liés** au rayon `r` si leurs dilatations de `r` blocs se touchent (Chebyshev ≤ `2r + 1`) — sans quoi une fusion
pourrait se défaire au pas suivant. `SplitClock` : une séparation n'est due qu'après **une seconde continue** à plusieurs
composantes ; l'horloge repart de zéro dès que la partition revient à une composante, et à la naissance d'un domaine — une
fusion en fait naître un. La durée de vie minimale de 0,75 s d'ADR-006 §4 règle l'**extinction**, qui est à l'ordonnanceur ;
ici, un domaine fusionné vit au moins une seconde avant de se séparer.

**L'état** (`delta3d_regions.rs`, `Volume3::transplant`). Dans la référence, l'état d'un domaine vit dans la boîte qui
l'enveloppe. `transplant(src, offset)` recopie, sur le recouvrement, **tout ce que le pas mobile lit d'un pas à l'autre** —
surface et son reste compensé, trois vitesses, pression de départ ; les murs du receveur restent nuls (la leçon de S350 :
des vitesses figées dans les murs écartaient la surface de 7 cm) ; refus hors du réseau commun (`dx`, `nz`, repos, densité,
gravité — ADR-006 §3.1) ou avec une découpe du fond, dont l'état n'est pas porté. `clear_to_rest` prépare le receveur.

## 2. Critères écrits avant

| critère | résultat |
|---|---|
| **1** — ensembles : union ; une relation pour fusionner et séparer ; séparation après 1,0 s continue ; un clignotement ne sépare jamais | **tenu** : quatre essais ; écarts de 4 à 7 blocs au rayon 2, liés jusqu'à 5 ; clignotement de 0,9 s / 0,1 s pendant 10 s, jamais ; **vu échouer** avec une liaison à `2r` |
| **2** — l'état : aller-retour `C → (A, B) → C'` au bit hors de la coupure ; le débit perdu publié | **tenu** : 32 × 16 × 12 à 25 cm, après 20 pas ; la coupure, à 4 m, perd au plus **7,8 mm/s** (0,058 m³/s : l'onde l'a déjà atteinte) ; la coupure rendue, `C'` refait vingt pas **au bit** avec `C`, mêmes itérations ; **vu échouer** sans la pression de départ |
| **3** — deux bosses, fusion au critère, puis 5 s : écart au domaine unique ≤ 1 % de l'amplitude | **tenu** : **0,26 %** (§3) |
| **4** — suite, zéro avertissement | **tenu** : **691 réussis**, 18 ignorés, zéro avertissement |

## 3. Sans rupture — contre le domaine unique

Bassin de 4 m de large à 25 cm, 2 m d'eau sous 1 m d'air, pas mobile de 20 ms, bosses gaussiennes de 5 cm. **Blocs actifs** :
ceux où la surface s'écarte du repos de plus de 1 mm. **Référence** : un seul domaine qui couvre tout depuis le départ.

**Fusion** — 32 m, bosses à 5 et 27 m ; A sur [0, 14 m), B sur [18, 32 m), séparés par 4 m de repos :

| instant de la fusion | écart avant | écart à la fusion | écart par seconde ensuite, % de l'amplitude |
|---|---:|---:|---|
| au départ (forcé) | 0 | 0 | **au bit** sur 5 s |
| **au critère : 0,94 s** | 0,16 % | 0,19 % | 0,26 / 0,20 / 0,17 / 0,14 / 0,19 |
| forcé à 2 s | 2,0 % | 2,9 % | 3,4 / 2,8 / 2,2 / 1,8 / 2,6 |
| forcé à 3 s | 3,1 % | 8,2 % | 8,5 / 6,3 / 4,9 / 4,7 / 7,1 |

**L'instant du critère compte.** Avant la fusion, chaque partie a ses murs ; tant que les ondes ne les ont pas atteints, une
partie diffère du domaine unique de ce que la pression — elliptique, instantanée — porte au-delà, 0,16 % ici. Quand elles les
ont frappés, la réflexion est dans l'état recopié, et la fusion ne l'efface pas. Le critère d'ADR-006 (4 m de dilatation) les a
fusionnés à 0,94 s, avant que les ondes n'atteignent les murs (vers 1,7 s, estimé à `√(g·h)`).

**Séparation** — 40 m, bosses à 5 et 31 m, un seul domaine : deux composantes dès le départ, séparation due à **1,00 s**,
coupure au milieu de l'écart, sur une frontière de bloc (18 m). Écart par seconde : **0,00** / 0,03 / 0,32 / 1,45 / 2,61 % —
rien ne saute à la coupure ; ensuite, les ondes de chaque bosse se réfléchissent sur le nouveau mur, que le domaine unique n'a
pas. **Trouvé en chemin** : au premier passage, bosses à 5 et 35 m, l'écart était nul sur cinq secondes — elles étaient
symétriques autour de la coupure à 20 m, qui était donc un plan de symétrie, où un mur ne change rien. Témoin sans valeur,
remplacé par le cas asymétrique.

## 4. Ce que ce document ne dit pas

- **Les murs.** Un domaine de la référence est fermé par des murs ; une partie séparée réfléchit donc ce qui l'atteint. Dans
  le système, le bord de δ a son éponge et rend (ou devra rendre) ce qui sort à W (4.7, 4.8) : l'écart d'une séparation après
  quelques secondes est celui des murs, pas de la partition.
- **Des boîtes, pas des blocs épars.** L'état vit dans la boîte qui enveloppe l'ensemble ; le stockage par blocs, la
  projection sur un ensemble épars et le pool de blocs (ADR-006 §4, point 3) restent à faire — le banc configure ses domaines
  en cours de route, ce que la production ne fera pas.
- **Un domaine qui suit la perturbation** : ici, les domaines ne grandissent pas ; leurs ensembles **actifs** grandissent, et
  décident. Croissance par blocs, niveaux de `dx` choisis par l'ordonnanceur (rang 4) et prévision : la suite de C8.
- Rien sur la carte ; aucun verdict visuel ; la fusion de domaines de `dx` différents est refusée par construction (ADR-006
  §3.1 : ils se couplent par transduction).
