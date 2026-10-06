# C07 peu profond : l'angle du sillage de W au-delà du critique — S523 (listes 3.2, 13.2)

*S523, 2026-10-06, en autonomie.* Par 5 m de fond, au-delà du critique (`Fr_h` > 1), le sillage est contenu dans un coin de demi-angle
`arcsin(1/Fr_h)` (CAS-CANONIQUES C07). W porte la profondeur uniforme depuis S522.

## Reproduire

- `SUPER_T=40,16 python outils/reference_sillage.py supercritique` — l'instrument sur la référence, trois grilles.
- `code/target/release/examples/c07_profondeur.exe calculs/w_super10c_s523.bin 10 40 90 102 100 1 3 512 512` et
  `… w_super15c_s523.bin 15 16 90 102 100 1 3 512 512` (21 s chacun), puis
  `SUPER_T=40,16 python outils/reference_sillage.py supercritique calculs/w_super10c_s523.bin calculs/w_super15c_s523.bin`.

## 1. L'instrument, éprouvé sur la référence de la même famille (ADR-233)

| instrument, sur la référence par 5 m de fond (σ 2 m, coupure 3) | 10 m/s (attendu 44,46°) | 15 m/s (27,83°) |
|---|---|---|
| déclaré : le maximum de la moyenne de \|η\| le long des rayons, 20–60 m, 24 s | 14,5° | 11,25° — **il lit le sillage intérieur d'ondes courtes** ; changé avant W |
| le profil, regardé : la crête des ondes longues, juste en dedans du coin, par fenêtre de 20 m de 20 à 120 m | 41,5 / 43,25 / 43,75 / 44,0 / 44,5° | 24,25 / 26,0 / 26,75 / 27,0 / 27,25° |
| **figé** : la dernière crête (maximum local le plus extérieur), 80–100 m, trois grilles | **43,75–44,00°** | **27,00°** |

## 2. W mesuré — trois montages, deux écartés et pourquoi

| montage | W contre la référence (écart quadratique, 80–100 m) | ce qu'il dit |
|---|---|---|
| recette 512 × 256 à coupure 3 (rayon honnête 179 m), 40 s : trajets de 400 et 600 m | 23 % / 62 % | **hors du domaine d'ADR-132** : les ondes émises à 300–500 m des points mesurés sont repliées par l'échantillonnage angulaire |
| 512 × 512 à coupure 1,5 (rayon 715 m) | **0,03 %** à 10 m/s ; 13 % à 15 m/s | le repli disparaît ; mais la coupure à 1,5 fait sonner le spectre (Gibbs) : l'instrument lit 77° **sur la référence même** — une autre famille, non éprouvée |
| **512 × 512 à coupure 3 (357 m), 40 s à 10 m/s, 16 s à 15 m/s** (la distance du départ à la zone : 310 et 150 m) | **0,38 % / < 0,01 %** | le montage retenu |

| instrument figé, montage retenu | W | référence |
|---|---|---|
| 10 m/s (`Fr_h` = 1,43) | **44,00°** (attendu 44,46) | 44,00° |
| 15 m/s (`Fr_h` = 2,14) | **79,75°** — une crête du bruit f32 dans la queue (6–8·10⁻⁸ m, 1/160 000 du maximum du profil) | 27,00° |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) sur la référence, l'instrument lit `arcsin(1/Fr_h)` à 1° | le déclaré : non ; la dernière crête : 0,46–0,71° et 0,83° | tenu (instrument changé avant W) |
| (2) sur W, à 2° de 44,46° et de 27,83° | 44,00° ; 79,75° | **tenu à 10 m/s, manqué à 15 m/s** — l'instrument figé prend le bruit ; les champs de W et de la référence sont identiques à 10⁻⁴ près, mais un champ identique n'est pas un angle mesuré |
| (3) W contre la référence ≤ 10 % | 0,38 % ; < 0,01 % (montage retenu) | tenu |

## 4. Ce que cela dit

Au-delà du critique, le sillage de W est la théorie linéaire en profondeur finie à 0,4 % près, et à 10 m/s son angle mesuré est celui du
coin de Mach à 0,5° : **C07 peu profond passe à `Fr_h` = 1,43**. À 2,14, l'instrument d'une crête sans seuil ne résiste pas au bruit de
la queue ; un instrument qui l'ignore doit être éprouvé d'abord (sur une référence bruitée au niveau de W). Deux leçons pour la revue :
le domaine honnête d'un sillage se juge **sur la distance du chemin émetteur aux points mesurés**, pas sur la position de la source ; et
W ne le vérifie pas (**A331**). La résonance (la pente −½ en deçà du critique) attend une source plus fine (σ ≤ 0,25 m).
