# L'alimentation par SGN départagée contre l'enregistrement de la 3D — S700 (liste 4.14)

*S700, 2026-10-08, en autonomie.* S699 : nourri exactement par la 3D, le bord par particules est transparent (−0,011 s). L'écart de S698
(−0,047 s) vient donc de la manière de le nourrir par SGN. Ici, les causes une à une (ADR-276 D2), chacune contre le même enregistrement
du tout-3D au plan x = 5,0 m.

## Reproduire

- `python outils/essai.py the_serre_feed_split_against_the_3d_record_s700 --ignore` (≈ 28 min : 13 min pour l'enregistrement, 2 × 7 min
  pour les rejeux).

## Mesuré

**Les rejeux.** Le témoin, le tout-3D, se retourne à 2,637 s et 9,988 m.

| montage au raccord de 5,0 m | ce qui change | retournement | écart | air enfermé |
|---|---|---|---|---|
| rejeu exact (S699) | — | 2,626 s, 9,963 m | −0,011 s | 2,776 s |
| **R2** | les particules sans leur matrice affine | **2,601 s**, 9,913 m | **−0,037 s** | 2,750 s |
| **R3** | la pose par faces de S698 : des quanta posés dans la maille de la face, avec la vitesse de la 3D à cette face (`w = 0`, `C = 0`) | **2,711 s**, 10,088 m | **+0,074 s** | 2,849 s |
| S698 | la pose par faces, avec les vitesses du profil de SGN | 2,590 s, 9,888 m | −0,047 s | 2,792 s |

La masse : 2,6·10⁻¹⁴ et 1,0·10⁻¹⁵. La dette sous un quantum.

**Au plan, pendant l'enregistrement** :
- `h` de la 3D : la plus haute particule de la tranche ± dx, + dx/4, lu au quantum de 6 mm ;
- `ū` de la 3D : la moyenne des faces sous `h`.

| | SGN | la 3D |
|---|---|---|
| crête de `h` | 0,6501 m à 0,650 s | 0,6394 m à 0,620 s |
| crête de `ū` | 0,5756 m/s à 0,650 s | 0,5525 m/s à 0,670 s |
| écart maximal | `|Δh|` 1,73 cm | `|Δū|` 0,090 m/s |

## Ce que cela dit

Les écarts se lisent deux à deux, une cause à la fois :

| écart | la cause | l'effet |
|---|---|---|
| R2 − exact | la matrice affine des particules posées | **−0,026 s** |
| R3 − exact | la pose par faces, comme un tout (positions sur la grille des sous-mailles, vitesse moyenne de la couche, `w = 0`, `C = 0`, le volume par le flux de la face) | **+0,085 s** |
| S698 − R3 | les vitesses : SGN et son profil, contre la 3D, à pose égale | **−0,121 s** |

- **S698 tombait à −0,047 s par compensation** : la pose par faces retarde de 0,085 s, les vitesses de SGN avancent de 0,121 s. Ni l'une ni
  l'autre n'est juste ; l'accord partiel de S698 n'était pas une preuve.
- **Le porteur SGN, au plan, a une crête 1,1 cm plus haute** que la 3D (1,7 %) et une vitesse 4 % plus forte. C'est cohérent avec un
  retournement plus précoce. Mais 1,1 cm, c'est moins de deux quanta de lecture : la forme de l'onde, non la seule crête, est à comparer.
- **La matrice affine compte** (−0,026 s) : une particule posée doit emporter son gradient de vitesse.

**Ce qui reste à faire** :
- une pose qui reproduise le rejeu exact à partir d'une vitesse donnée : des positions tirées dans la maille et non sur la grille des
  sous-mailles, `w`, et la matrice affine prise au gradient du profil ;
- puis le porteur comparé à la 3D sur toute la forme de l'onde.

Le pas d'après est la revue (S701).

## Un piège rencontré

`outils/essai.py` imprimait « ū » sur la console Windows (charmap). Il a planté, et sa mort a tué l'essai, à 13 min. Corrigé : la sortie
passe en UTF-8, les caractères inconnus remplacés.
