# La boîte qui naît et meurt avec le corps — S729 (liste 4.14 ; LOD, étape 3, B5)

*S729, 2026-10-09, en autonomie.* LOD-ETAPE-3-S722, B5, le déclencheur de présence : la boîte de 3D naît quand un corps touche l'eau, le
suit, et meurt quand il en sort.

## Ce qui est fait

- **`RelaisBoite::naitre`** : une boîte naît au milieu d'un Saint-Venant entier, de son état (`birth_from_columns`, la vitesse de chaque
  colonne). **Le niveau est réglé sur ce que la 3D lit** (ADR-283 D1) : une première naissance mesure le biais de lecture, et la seconde
  pose ce qu'il faut pour que la 3D lise le niveau de Saint-Venant. Les particules qui tombent dans le corps sont retirées. Les écarts de
  volume vont à `reste`, compté dans la masse.
- **`RelaisBoite::mourir`** : chaque colonne rend à Saint-Venant sa surface (φ) et sa quantité de mouvement (`fermer_trou`). Ce qui reste
  (les dettes, les réservoirs, `reste`) est réparti également sur le trou ; la masse est exacte.
- **Le déclencheur** : la boîte naît quand le bas du corps passe sous le niveau, et meurt 0,2 s après qu'il en est sorti (l'hystérésis).
- `Apic3::drop_particle`.

## Reproduire

- `python outils/essai.py the_box_is_born_and_dies_with_the_body_s729 --ignore` (≈ 36 min, le témoin compris).

## Mesuré

La sphère (rayon 8 cm) fait quatre phases :
1. elle descend de 15 cm au-dessus de l'eau jusqu'à mi-immersion ;
2. elle avance de 0,45 m ;
3. elle remonte et sort ;
4. elle attend.

Cela fait 3,2 s, dans un Saint-Venant de 3 m × 2 m, contre un APIC entier.

| critère | mesuré | seuil |
|---|---|---|
| (1) les naissances, les morts | **une naissance (0,24 s), une mort (2,47 s)** | une, une |
| (2) la force dans l'eau, l'écart moyen au témoin | **6,0 %** | 10 % |
| (3) la masse, la naissance et la mort comprises | **2,4·10⁻¹⁴** | 10⁻¹² |
| (4) le niveau autour du trou, avant et après | **0 à la naissance (0,39931 m), 0 à la mort (0,39617 m)** | 1 mm |

À la naissance, `reste` vaut −6,9·10⁻⁴ m³ : la 3D pose 0,7 mm d'eau de plus que Saint-Venant ne lui cède, pour lire le même niveau. La mort le
rend.

**Critères : tenus.**

## Ce que cela dit

- **L'étape 3 du LOD est faite** : une bulle de 3D naît autour d'un corps qui entre dans une eau en 2D, le suit, et s'éteint quand il en
  sort. Rien ne se voit au passage, ni choc de niveau ni perte de masse. La force sur le corps reste à 6 % de la 3D entière.
- Le niveau lu par la 3D, réglé à la naissance, est ce qui rend le passage invisible (ADR-283 D1).

## Une idée de l'utilisateur, inscrite (2026-10-09)

« La simulation 3D peut se réaliser jusqu'à la plage si une vague éclate trop proche du bord. » Elle est inscrite au déclencheur de
l'étape 2 (LOD-ETAPE-2-S705, D1), avec la difficulté mesurée.
