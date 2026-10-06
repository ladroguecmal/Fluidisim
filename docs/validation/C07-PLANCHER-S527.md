# C07 peu profond aux deux vitesses : la dernière crête au-dessus du plancher — S527 (listes 3.2, 13.2)

*S527, 2026-10-06, en autonomie.* S523 : la dernière crête des rayons, éprouvée sur la théorie sans bruit, prenait sur W un maximum local du
plancher f32 à 15 m/s (79,75° pour 27,83°). ADR-234 D2 : la référence d'un instrument porte le bruit de l'objet mesuré.

## Reproduire

- `SUPER_T=40,16 python outils/reference_sillage.py plancher` — la référence bruitée.
- `SUPER_T=40,16 python outils/reference_sillage.py plancher calculs/w_super10c_s523.bin calculs/w_super15c_s523.bin` — W (les champs de
  S523, montage retenu : recette 512 × 512 à coupure 3, 40 s et 16 s).

## 1. L'instrument, déclaré avant

La dernière crête **au-dessus d'un plancher** : le maximum local le plus extérieur du profil dont la valeur dépasse 10⁻³ du maximum — 160
fois au-dessus de la queue de W (6·10⁻⁶ du maximum), 230 fois sous la crête de Mach (0,23 du maximum).

## 2. Mesuré

| | 10 m/s (`Fr_h` = 1,43, attendu 44,46°) | 15 m/s (`Fr_h` = 2,14, attendu 27,83°) |
|---|---|---|
| la référence bruitée (10⁻⁷ m par point, trois tirages, trois grilles) | 43,75–44,00° | 27,00° |
| témoin : l'instrument de S523, sans plancher, sur la référence bruitée | — | **79,0–79,25°** — l'échec de S523 reproduit |
| **W** | **44,00°** | **27,00°** |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) sur la référence bruitée, `arcsin(1/Fr_h)` à 1° aux deux vitesses | 0,71° et 0,83° au pire | tenu |
| (2) sur W, à 2° | 0,46° et 0,83° | tenu |

## 4. Ce que cela dit

**C07 peu profond passe aux deux vitesses** au-delà du critique ; avec S519 (profond) et S525 (la résonance), **C07 passe entier**. Le
témoin montre que la référence bruitée au plancher de W aurait arrêté l'instrument de S523 avant qu'il ne soit appliqué : la protection
d'ADR-234 D2 aurait évité le manqué.
