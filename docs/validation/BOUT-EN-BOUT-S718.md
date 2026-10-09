# La vague de bout en bout — S718 (liste 4.14 ; LOD, étape 2, E1)

*S718, 2026-10-09, en autonomie ; l'utilisateur dort.* N1, N2 et M1 sont acquis (S707, S715, S717). Ici, les pièces assemblées :
- SGN au large ;
- la bande 3D (5 à 10,775 m), nourrie au bord gauche par la pose par la grille (S703) ;
- le déferlement en 3D, le relais Saint-Venant au rivage ;
- à 3,2 s, la mort de la bande, puis un seul Saint-Venant sur la plage entière jusqu'à 5 s.

## Ce qui est fait

- **`Large::BoutEnBout`**. À la mort, le Saint-Venant réuni prend le large depuis SGN (cellule à cellule), la bande depuis sa surface
  (`mort_vers_sv`, le fond décalé de `x_r`), le rivage depuis le relais.
- **`Large::BandeJusqua5`** est le témoin : la même bande, jusqu'à 5 s, sans la mort.
- Le déclencheur, ici : le front de l'onde de référence touche déjà la bande au départ (η = 2,6 cm à 5 m), et la bande naît à t = 0. Le
  déclencheur qui décide *s'il faut* la 3D attend, à la demande de l'utilisateur.

## Reproduire

- `python outils/essai.py the_wave_from_end_to_end_s718 --ignore` (≈ 5 min) ;
- le témoin : `python outils/essai.py the_band_fed_by_serre_without_death_s718 --ignore` (≈ 10 min).

## Mesuré

| | retournement | air | la remontée maximale | coût jusqu'à 5 s |
|---|---|---|---|---|
| le tout-3D (S717, le même montage) | 2,637 s, 9,988 m | 2,790 s | 0,3387 m à 3,917 s | 1 191 s |
| **de bout en bout** | **2,569 s, 9,863 m** | 2,747 s | **0,3714 m à 3,859 s** | **302 s** |
| le témoin : la bande nourrie par SGN, sans la mort | 2,569 s, 9,863 m | — | 0,3714 m à 3,856 s | 574 s |

Le volume rendu à la mort : −3,8·10⁻⁴. La masse : 1,7·10⁻¹⁵. La dette sous un quantum.

**Critères** :
- (1) le retournement à 0,1 s et 0,15 m, l'air après : **tenu** (−0,068 s, −0,125 m ; comme S703) ;
- (2) la remontée à 1,25 cm : **échoue**, +3,3 cm ;
- (3) le volume, la masse : **tenus** ;
- (4) le coût : **4 fois moins** que le tout-3D.

## Ce que cela dit

- **La vague de bout en bout fonctionne** : le large en SGN, la bande 3D qui déferle, la mort, Saint-Venant jusqu'au sable. Le tout
  coûte **4 fois moins** que le tout-3D. Le gain grandit avec la durée après la vague, et avec la part du large.
- **La mort est innocente** : avec ou sans elle, la remontée est la même, à 0,0 mm et 3 ms près.
- **Les +3,3 cm viennent du large.** C'est la question ouverte d'ADR-278 D3 :
  - SGN porte l'onde de départ, un profil de Boussinesq, plus haute que la 3D ne la porte (S704 : 0,150 contre 0,143 m ; S715 : +10 %
    à la naissance) ;
  - une onde plus haute déferle plus tôt et remonte plus haut.

  Ramenés à l'usage, 3,3 cm de hauteur sur une pente de 1:12 font ≈ 40 cm de lame en plus sur le sable : visible côte à côte, discret
  seul.
- **Ce cas est extrême** : une onde très non linéaire (H/d = 0,3), qui naît à 1,6 m de la bande. La houle d'un jeu, plus douce au large,
  sera plus près de SGN.

## La suite

**S719** : la même onde pour SGN et pour la 3D. Le tout-3D et SGN partent de l'onde solitaire exacte de SGN (le profil de Rayleigh,
`k = √(3a/(4d²(d+a)))`, plus large de 14 %), au lieu du profil de Boussinesq, qui n'est l'équilibre d'aucun des deux. Si la remontée de bout
en bout rejoint alors le tout-3D, la question d'ADR-278 D3 est celle de l'onde de départ, et non du porteur.
