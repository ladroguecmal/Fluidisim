# La nappe rompue en gouttes — S488 (K2-4, premier temps)

*S488, 2026-10-06, en autonomie. Campagne K2 ([conception](../registres/CAMPAGNE-K2-S478.md)) ; ADR-014 §4 ; A312.* **Le modèle de gouttes
existe et tient la masse ; il ne change pas la couronne de B10 — A312 n'est pas levée, et la cause est nommée.**

## Reproduire

- `cargo test -p water-core --release --offline s488` — l'apogée balistique à 1 % ; un jet : des gouttes naissent et retombent, masse exacte.
- `FILS=16 [APIC3D_GOUTTES=1] code/target/release/examples/apic3d_b10.exe 2 <8|12|16>` — B10 (quart), avec ou sans gouttes.
  Série : `calculs/20261006-…-b10-gouttes-serie2`.

## 1. Le modèle

`Apic3::enable_droplets` (`apic3d_gouttes.rs`) : une particule dont la maille **et ses six voisines** sont sans eau — une nappe détachée,
plus mince qu'une maille — et dont la vitesse donne `We = ρ·v²·d/σ > 12` (`d` = 0,62 dx, le diamètre du volume d'une particule) devient
une **goutte** : hors de la grille, balistique (`g_eff`, traînée `C_d` = 0,47), elle redevient de l'eau en entrant dans une maille d'eau.
Aucune particule ne naît ni ne meurt. Sans `enable_droplets`, au bit (non-régression, 46 essais d'APIC 3D).

**Un premier critère, plus large, a été retiré** : la seule maille d'air faisait de chaque particule de surface en mouvement une goutte
(4 936 à D/dx = 8) et changeait la cavité de B10 ; avec les six voisines, la surface reste de l'eau.

## 2. B10, trois mailles

| D/dx | couronne, sans gouttes | avec gouttes | gouttes nées | pincement (√(R/g)) |
|---|---|---|---|---|
| 8 | 0,205 D | 0,205 D | 5 | 2,131 |
| 12 | 0,204 D | 0,204 D | 1 | 2,071 |
| 16 | 0,305 D | 0,305 D | 34 | 2,084 |
| 24 (S485) | 0,386 D | — | — | 2,024 |

**Critère (3), A312 : manqué.** La couronne croît avec la maille au-delà de 12 (+ 50 % à 16, + 27 % encore à 24), et **les gouttes n'y changent
rien**.

## 3. La cause, nommée

La couronne est **la particule la plus haute avant le pincement** : le bout de la nappe. Sa hauteur est fixée par la **vitesse d'éjection**
du bord, que la maille résout de mieux en mieux — une nappe plus fine, éjectée plus vite. Une goutte hérite de cette vitesse et vole comme
la nappe aurait volé (sans pression, une nappe libre est déjà balistique) : la rompre ne change pas l'apogée. **Ce qui manque est ce qui
freine le bord d'une nappe réelle : la tension de surface**, qui rétracte le bord à la vitesse de **Taylor–Culick**, `v = √(2σ/(ρ·h))`
(h l'épaisseur) — un bourrelet qui recule, et le bord qui ne monte pas indéfiniment. La suite de K2-4 : un modèle sous-maille de la
rétraction du bord (l'épaisseur locale de la nappe estimée par le nombre de particules par maille), puis la couronne à trois mailles, et
contre une mesure publiée de la gerbe d'une sphère.

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) sans gouttes, au bit | non-régression, 46 essais | tenu |
| (2) masse exacte ; une goutte balistique à 1 % | essais de S488 | tenu |
| (3) A312 : la couronne à trois mailles à 15 % | 0,205 / 0,204 / 0,305 D, avec ou sans gouttes | **manqué** — la cause nommée (§3) |
