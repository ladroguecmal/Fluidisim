# Les dispatchs à deux dimensions ; la remontée en cuve entière — S487

*S487, 2026-10-06, en autonomie.* [A326](../registres/ANGLES-MORTS.md) en partie levée ; la suite de [REMONTEE-S485](REMONTEE-S485.md).

## Reproduire

- `python outils/non_regression.py` — le chemin sous 65 535 groupes, au bit.
- `CAS=remontee QUART=0 R_DX=<4|6> DUREE=1.0 [TRACE=1] water-viewer --apic3d-poches` — la bulle de 4 cm en cuve entière, sur la carte.

## 1. Les dispatchs à deux dimensions

`ApicCarte::dispatch` lance en deux dimensions au-delà de 65 535 groupes ; les dix noyaux par particule (tri, transferts vers les particules,
advection, séparation, corps, compactage) reconstruisent l'indice linéaire (`lin128` : `g.x + g.y·65 535·128`). Sous la borne `g.y` vaut 0 :
**le banc de non-régression tient au bit**. **9 724 888 particules** passent (la cuve entière à R/dx = 6).

**Ce qui reste borné** : les noyaux sur les faces et les mailles, et ceux des poches, n'ont pas d'indice à deux dimensions — au-delà de
8,4 M faces (≈ 2,8 M mailles), une **assertion** les arrête (liste explicite des noyaux admis) au lieu qu'ils calculent faux en silence.
La cuve entière à R/dx = 8 (9,9 M faces) s'y arrête. A326 reste ouverte pour eux.

## 2. La remontée en cuve entière

| R/dx | U, la bulle entière (m/s) | Davies–Taylor (`d_e` mesuré) | rapport | première / seconde moitié |
|---|---|---|---|---|
| 4 | 0,548 | 0,607 | **0,90** | 0,514 / 0,590 |
| 6 | 0,525 | 0,621 | **0,85** | 0,645 / 0,453 |

À R/dx = 6, la bulle monte d'abord à la vitesse de Davies et Taylor (× 1,04), puis **se scinde en trois poches** vers 0,25 s ; la plus
grande (5,95 cm de diamètre équivalent) monte ensuite à ≈ 0,47 m/s, × 0,87 de Davies et Taylor pour sa taille ; l'air de l'ensemble est
conservé à 5 %. Une calotte de 8 cm est stable dans l'eau réelle : la scission est vraisemblablement numérique (la résolution de la jupe).

**Lecture** : dans les 15 % à R/dx = 4, juste au-delà à R/dx = 6 (15,4 %) ; deux points ne disent pas une convergence (ADR-223, la
protection des trois points), et la scission brouille la grandeur mesurée. R/dx = 8 attend les faces à deux dimensions.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) non-régression au bit | tenue | tenu |
| (2) la cuve entière à R/dx = 6 passe et donne U | 0,525 m/s | tenu |
| (3) U à R/dx = 4 et 6, l'écart dit ; R/dx = 8 si la mémoire le permet | 0,548 et 0,525 (4 %) ; R/dx = 8 arrêté par les faces, non par la mémoire | tenu, sans le troisième point |
