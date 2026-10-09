# La renaissance de la 3D par la surface — S715 (liste 4.14 ; LOD, étape 2, N2)

*S715, 2026-10-09, en autonomie ; l'utilisateur dort.* S707 : renée depuis le compte de ses particules, la 3D perdait 6 % d'amplitude.
APIC tasse ses particules sous la crête, si bien que le compte n'est pas la surface (S708). Ici, la renaissance lit et repose la surface.

## Ce qui est fait

- **`Renaissance::DepuisLaSurface`** : chaque colonne est réduite à la hauteur de sa surface reconstruite, `Σ clamp(½ − φ/dx, 0, 1)·dx`
  par colonne et par rangée. `birth_from_columns` (S707) repose alors les particules à la densité nominale jusqu'à elle, avec le profil
  vertical de SGN tiré de `ū` lissé.
- **La lecture par la surface** : le profil de la surface reconstruite, moyen sur les rangées, comparé par la comparaison intégrale de S707.

## Reproduire

- E1 : `python outils/essai.py the_3d_reborn_by_its_surface_s715 --ignore` (≈ 5 min) ;
- E2 : `python outils/essai.py the_3d_born_from_serre_by_its_surface_s715 --ignore` (≈ 5 min).

## Mesuré

Chaque case donne le décalage, puis le facteur d'amplitude, contre la 3D ininterrompue. La lecture est par la surface.

| photo | S707 E2 : renée par le compte | **E1 : renée par la surface** | **E2 : née de SGN** |
|---|---|---|---|
| 0,6 s | −1,8 cm ; 0,995 | **+1,1 cm ; 0,996** | −0,6 cm ; 1,098 |
| 1,0 s | −6,3 cm ; **0,935** | **−2,9 cm ; 0,992** | −2,8 cm ; 1,065 |
| 1,6 s | +17,8 cm ; 0,945 | **−0,5 cm ; 0,999** | +3,6 cm ; 1,040 |

- E1 : les particules passent de 271 952 à 270 652, le tassement disparaît. `V_φ` est à −0,07 % de la 3D ininterrompue à 0,6 s, à
  −0,31 % à 1,6 s. L'écart qui reste est de 2 à 3 mm.
- E2 : la crête par la surface, née de SGN, vaut 0,147 m à 0,6 s, 0,142 m à 1,0 s et 0,135 m à 1,6 s. Celle de la 3D ininterrompue :
  0,134, 0,132, 0,137 m.

**Une réserve.** La photo de 0,4 s, prise juste après la renaissance, lit un φ reconstruit avant elle : φ ne se refait qu'au pas suivant.
Le « plancher » prévu à 0,4 s était donc trivial, et la mesure vaut à partir de 0,6 s.

**Critères. E1 : tenu** (le décalage sous 5 cm, le facteur à 2 %, aux photos de 1,0 et 1,6 s). **E2 : rapporté.**

## Ce que cela dit

- **N2 est acquis** : la 3D renaît dans une onde en marche, fidèlement, quand on lit et repose sa surface. La faute de S707 était le
  compte, non le profil vertical de SGN.
- **Née de SGN, la 3D garde la phase**, mais naît avec l'onde de SGN, 10 % plus haute. Elle se repose ensuite vers sa propre forme en
  ≈ 1 s. C'est l'écart connu entre SGN et la 3D sur l'onde de départ (S704, ADR-278 D3), une question de porteur, non de naissance.
- **La règle pour M1 (la mort) et pour le relais** : ce que la 3D rend à la 2D se lit par la surface (ADR-280 D1).

## La suite

**S716** : la quarante-septième revue (S711–S715). Puis **M1**, la mort de la 3D vers Saint-Venant, par la surface.
