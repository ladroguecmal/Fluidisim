# Le raccord du large par particules, nourri par le profil de SGN — S698 (liste 4.14)

*S698, 2026-10-08, en autonomie.* S697 a mis tout l'écart du retournement sur le raccord du large à la façon de S650 : une zone de
colonnes hydrostatique, et une vitesse uniforme sur la verticale. Ici, ni zone ni colonnes : la 3D entre et sort par des particules,
comme au rivage (S682–S683), et le porteur SGN (S694) donne la vitesse sur la verticale.

## Ce qui est fait

- **`apic3d_gauche.rs`, le bord gauche par particules** :
  - une particule qui franchit le bord est retirée, son volume compté ;
  - le volume qui entre est donné **face par face** (rangée × couche) au réservoir de la face ;
  - chaque quantum entier est posé dans la maille derrière sa face, à la sous-maille la moins occupée, avec la vitesse de sa hauteur.
- **Le profil vertical de SGN** sur fond plat (z depuis le fond) : `u(z) = ū + (h²/6 − z²/2)·ū_xx`, `w(z) = −z·ū_x`, avec `ū = q/h`
  et ses dérivées par différences sur les mailles de SGN. Le bord ouvert reçoit `u(z_k)` couche par couche. Le volume de chaque face
  est `u(z_k)·dt·dx²`, la couche de surface au prorata. Le profil s'intègre exactement en `ū·h` : `∫₀ʰ (h²/6 − z²/2) dz = 0`.

**Une première pose, fausse, corrigée en cours de session.** La première version recevait le volume par rangée et le posait au plus bas
de la colonne. Les couches hautes se vidaient, et un trou d'air restait sous la surface, dans la colonne d'entrée. Le lecteur de
retournement l'a pris pour un retournement dès 0,14 s, à 5,01 m. La pose face par face l'efface : l'eau entre à la hauteur où elle
franchit le bord.

## Reproduire

- `python outils/essai.py the_offshore_relay_by_particles_s698 --ignore` (≈ 7 min) ;
- `python outils/essai.py the_offshore_relay_by_particles_not_crossed_s698 --ignore` (≈ 11 min).

## Mesuré

Témoin : le même montage sans raccord au large (S697), par la même fonction. Une seule chose change : le raccord.

| raccord du large | premier retournement | écart | air enfermé |
|---|---|---|---|
| aucun (S697, le témoin) | 2,637 s, 9,988 m | — | 2,790 s, 10,325 m |
| colonnes à 1,0 m (S693) | 2,582 s, 9,888 m | −0,055 s | 2,759 s, 10,313 m |
| colonnes à 5,0 m (S693, S695) | 2,524 s, 9,838 m | −0,113 s | 2,693 s, 10,245 m |
| **particules à 1,0 m** | **2,620 s, 9,938 m** | **−0,017 s** | 2,805 s, 10,379 m |
| **particules à 5,0 m** | **2,590 s, 9,888 m** | **−0,047 s** | 2,792 s, 10,346 m |

La masse : 2,5·10⁻¹⁵ et 2,1·10⁻¹⁶. La dette sous un quantum. 7,2 min et 11 min.

**Critère 1 (0,02 s, 0,15 m au raccord à 5,0 m) : échoue**, −0,047 s. Critère 2 : tenu.

## Ce que cela dit

| écart | la seule cause qui change | colonnes | particules |
|---|---|---|---|
| raccord à 1,0 m − témoin | le bord, que l'onde ne traverse pas | −0,055 s | **−0,017 s** |
| raccord à 5,0 m − raccord à 1,0 m | la traversée de l'onde | −0,058 s | **−0,030 s** |

- **Le bord par particules, non traversé, tient le critère** (−0,017 s, moins de 0,02 s) : trois fois mieux que la zone de colonnes.
- **La traversée coûte encore −0,030 s**, la moitié de celle des colonnes. Trois causes restent mêlées :
  - le porteur (SGN, une moyenne sur la verticale, contre la 3D) ;
  - le profil (sur fond plat, à l'ordre `μ²`) ;
  - la pose (une particule posée sans sa matrice affine, `C = 0`).
- **L'air enfermé est presque juste** (0,002 s) : la vague garde sa masse et son énergie, elle se retourne un peu plus tôt.

**La suite (S699)** : départager les trois causes, une à la fois (ADR-276 D2). D'abord le raccord entre deux copies du même solveur
(ADR-273 D1) : la 3D du large enregistrée par le tout-3D, rejouée au bord, sans SGN. Si l'écart tombe, le porteur est en cause ;
sinon, le bord.
