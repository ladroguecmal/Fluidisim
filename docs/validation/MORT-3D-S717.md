# La mort de la 3D vers Saint-Venant, par la surface — S717 (liste 4.14 ; LOD, étape 2, M1)

*S717, 2026-10-09, en autonomie ; l'utilisateur dort.* LOD-ETAPE-2-S705, M1 : quand la vague a passé, la 3D s'éteint et rend son eau à
Saint-Venant. Elle la rend lue par la surface (S715, ADR-280 D1).

## Ce qui est fait

- **`mort_vers_sv`**. Par colonne et par rangée :
  - la hauteur de la surface reconstruite au-dessus du fond d'APIC, rapportée au fond continu de Saint-Venant (le niveau gardé) ;
  - `h·ū`, avec `ū` la moyenne des vitesses des particules de la colonne.
- **Le montage** : deux modes nommés (ADR-277 D2).
  - `Large::AucunJusqua5`, le tout-3D jusqu'à 5 s, est le témoin.
  - `Large::AucunMort` : à 3,2 s, après le déferlement (2,637 s) et l'air enfermé (2,790 s), toute la 3D meurt. Un seul Saint-Venant
    reprend alors la plage entière, la 3D et le rivage réunis, jusqu'à 5 s, à son propre pas de Courant (0,4).
- **La remontée** sous la maille (S688) est enregistrée à chaque pas.

## Reproduire

- E1 : `python outils/essai.py the_3d_dies_at_rest_s717 --ignore` (≈ 1 s) ;
- E2 : `python outils/essai.py the_3d_dies_after_the_break_s717 --ignore` (≈ 29 min).

## Mesuré

**E1 — la mort au repos : tenu.** Le volume rendu égale le volume de la surface (+1,1·10⁻¹³). Après 1 s, Saint-Venant est immobile : une
vitesse de 5,7·10⁻⁷ m/s, `|η|` de 1,6·10⁻⁷ m.

**E2 — la mort après le déferlement : tenu.**

| | la remontée maximale | son instant | coût jusqu'à 5 s |
|---|---|---|---|
| le tout-3D (le témoin) | 0,3387 m | 3,917 s | 1 191 s |
| **la 3D morte à 3,2 s, puis Saint-Venant** | **0,3385 m** | **3,919 s** | **519 s** |

Le volume rendu à la mort : 0,445561 m³, contre 0,445732 m³ pour la surface et le rivage (−3,9·10⁻⁴). Les critères (1,25 cm, 0,1 s,
0,5 %) sont tenus avec une large marge : 0,2 mm et 2 ms.

## Ce que cela dit

- **M1 est acquis** : la 3D rend son eau à Saint-Venant sans choc. La remontée qui suit est celle du tout-3D, au dixième de millimètre.
- **La première mesure de l'économie du LOD** : de 3,2 à 5 s, Saint-Venant seul ne coûte presque rien. Les 5 s passent de 1 191 s à
  519 s, soit 2,3 fois moins, et le gain grandit avec la durée après la vague.
- La 3D sait maintenant naître au repos (N1), renaître dans l'onde (N2) et mourir (M1). Il reste le déclencheur (D1) et l'ensemble (E1).

## La suite

**S718** : la vague de bout en bout.
- SGN seul au large ;
- la bande 3D qui naît au repos quand le front de l'onde approche (le déclencheur, au plus simple : l'élévation de SGN au bord de la
  bande) ;
- le déferlement en 3D, puis la mort vers Saint-Venant.

Le tout se juge contre le tout-3D (ADR-278 D2 : le déferlement à 0,1 s et 0,15 m, la remontée à 1,25 cm), avec le coût mesuré. Le
déclencheur qui décide *s'il faut* la 3D (la prédiction du déferlement) attend, à la demande de l'utilisateur.
