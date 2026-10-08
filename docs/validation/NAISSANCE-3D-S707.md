# La naissance de la 3D depuis un état 2D — S707 (liste 4.14 ; LOD, étape 2)

*S707, 2026-10-08, en autonomie ; la première session longue (ADR-279 D3).* Les pièces N1 et N2 de
[LOD-ETAPE-2-S705](../registres/LOD-ETAPE-2-S705.md).

## Ce qui est fait

`birth_from_columns` (`apic3d_naissance.rs`) fait naître la 3D d'un état 2D :
- elle retire toutes les particules ;
- par colonne, elle pose `round(volume / quantum)` particules ;
- chaque sous-colonne (x, y) s'emplit du fond à la même hauteur, d'un pas vertical régulier ;
- chaque particule prend la vitesse et l'affine du G2P, sur une grille remplie par l'appelant ;
- l'écart de volume revient à l'appelant.

## Reproduire

- `python outils/essai.py the_3d_born_at_rest_s707 --ignore` (E1, ≈ 1,5 min) ;
- `python outils/essai.py the_3d_reborn_in_the_wave_s707 --ignore` (E2, ≈ 5 min) ;
- `python outils/essai.py the_3d_reposed_with_its_own_grid_s707 --ignore` (E2b, ≈ 5 min) ;
- `python outils/essai.py the_3d_reborn_diagnostic_s707 --ignore` (la densité à la naissance, ≈ 3 min).

## E1 (N1) — la naissance au repos : tenue

Un fond plat de 4 m, `h` = 0,49 m (une couche partielle), 1 s.

| | particules | vitesse maximale après 1 s |
|---|---|---|
| le semis du réseau (le témoin) | 99 840 | 6,8·10⁻⁶ m/s |
| une première pose (une couche partielle en haut, une place sur quatre) | 100 480 | **3,95·10⁻³ m/s** |
| **la naissance** (chaque sous-colonne emplie à la même hauteur) | 100 480 | **1,1·10⁻⁵ m/s** |

Posé + écart = donné, au bit. La première pose laissait une surface bosselée, que l'eau réarrangeait ; la pose par sous-colonnes la
garde plate.

## E2 (N2) — la renaissance dans l'onde, entre deux copies de la 3D : échoue, cause nommée

L'onde de S704 sur un fond plat de 10 m, 1,6 s. À 0,4 s, la 3D est réduite à `(h, ū)` par colonne, puis renaît par le profil vertical de
SGN. On la compare à la 3D ininterrompue.

**L'instrument, corrigé quatre fois avant toute attribution.** Chaque lecture s'est révélée plus bruitée que les critères, sur le témoin
lui-même :
1. le maximum dans le temps d'une tranche de 10 cm : 0,185 m au plan de 7 m, contre 0,15 m ;
2. le sommet du profil lissé : il sautait de 40 cm, car le haut d'une onde solitaire est plat (≈ 5 mm sur ±20 cm) ;
3. la comparaison intégrale de profils bruts : chaque colonne varie de ±50 % quand une file de particules passe sa frontière ;
4. **l'instrument retenu** : la hauteur par un noyau en tente de ±10 cm sur la position continue des particules, puis la comparaison
   intégrale des profils. Elle rend le décalage qui superpose le mieux (la phase), le facteur d'échelle (l'amplitude) et l'écart qui
   reste. Son plancher, à 0,4 s juste après la renaissance : un décalage de 2 mm, un facteur de 0,9994, un écart de 2 mm.

Ces mesures sont relatives à la 3D ininterrompue :

| photo | E2 : renée par le profil de SGN | E2b : reposée, sa propre grille |
|---|---|---|
| 0,4 s | −0,2 cm ; 0,9994 ; 2 mm | −0,2 cm ; 0,9994 ; 2 mm |
| 0,6 s | −1,8 cm ; 0,995 ; 9 mm | −5,1 cm ; 0,993 ; 12 mm |
| 1,0 s | **−6,3 cm ; 0,935** ; 18 mm | **−6,1 cm ; 0,927** ; 18 mm |
| 1,6 s | +17,8 cm ; 0,945 ; 14 mm | +24,1 cm ; 0,955 ; 16 mm |

Chaque case donne le décalage, le facteur d'amplitude, puis l'écart qui reste.

- **E2b** ne change qu'une chose : la grille de la 3D elle-même au lieu du profil de SGN. Elle perd autant. **Le profil de SGN n'est pas en
  cause : la cause est la nouvelle disposition des particules.**
- **La densité, au moment de la renaissance** :

| colonnes | haut de l'eau (la plus haute particule + dx/4) − hauteur du compte | densité relative des particules |
|---|---|---|
| sous l'onde (426) | **−2,1 cm** | **1,038** |
| hors de l'onde (804) | +0,75 cm | 0,986 |

**APIC tasse ses particules sous la crête et les écarte ailleurs.** Le nombre de particules d'une colonne n'en donne donc pas la surface,
à 2 cm près. La renaissance à la densité nominale efface ce tassement, et l'onde change.

E3, la naissance depuis SGN, ne s'est pas lancée : le plan la subordonnait à E2.

## Ce que cela dit, au-delà de la naissance

- **La vraie naissance, depuis SGN, n'est pas touchée** : il n'y a pas d'histoire de densité à garder. Poser à la densité nominale sur le
  `h` du porteur est juste par construction. C'est **la renaissance depuis la 3D** (et la mort, M1, qui réduit la 3D à `h`) qui doit lire
  la surface, non le compte.
- **Les instruments par comptage mesurent densité × hauteur, non la surface d'APIC.** Cela touche S704 : la crête plus haute à 2,5 cm
  qu'à 1,25 cm (comptée), et la croissance du témoin E2 (0,136 → 0,178 m, comptée), peuvent être du tassement. Elles sont à relire par la
  surface.
- **La masse « au bit » des raccords (S682–S703) compte des particules.** Le volume géométrique d'APIC, celui de ses étiquettes, varie
  localement de ±4 %. Sa dérive globale sur la vague de S690 n'a jamais été mesurée.

## La suite (S708)

1. Le volume d'APIC mesuré par la surface (étiquettes ou plus haute particule) contre le compte, sur le tout-3D de S690 et sur l'onde plate.
2. Si la dérive globale compte, la nommer, avec son effet sur les raccords.
3. Puis N2 de nouveau, la renaissance lue et reposée par la surface ; E3 ; M1, la mort par la surface.
