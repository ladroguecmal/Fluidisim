# Où la projection freine la lame : la surface dilatée — S748 (liste 4.1 ; ADR-289 D3.1)

*S748, 2026-10-09.* DENSITE-HYBRIDE-S747 : la projection hybride garde le repos et l'onde, mais freine la remontée de S645 (−12 %). **La
question** : où et quand ? Un diagnostic, aucun remède.

## Ce qui est fait

- `onde_sur_pente_observee_s748` : le montage de S645, avec un observateur après chaque pas, sans effet sur le calcul.
- `Apic3::density_projection_shift_place` : la place de la particule la plus déplacée par la projection, une simple lecture.
- `where_the_projection_slows_the_swash_s748` relève, sans projection puis avec la projection hybride :
  - tous les 0,05 s, le front, l'énergie au-delà du pied moins 0,5 m, le déplacement et sa place ;
  - tous les 0,25 s, les profils, tracés dans `captures/s748_ressaut.png`.

## Reproduire

`python outils/essai.py where_the_projection_slows_the_swash_s748 --ignore` (13 min).

## Mesuré

| t | la remontée (front par les particules), sans / hybride | l'énergie, hybride − sans | le déplacement le plus grand de la projection, et sa place |
|---|---|---|---|
| 0,81 s | −0,019 / −0,019 m | −1,8 % | 0,65 mm, x = 4,71 m, z = 0,401 m (la surface) |
| 1,21 s | −0,019 / **+0,015 m** | −3,4 % | 0,88 mm, x = 5,58 m, z = 0,336 m |
| 1,60 s | 0,082 / 0,090 m | −4,3 % | 2,61 mm, x = 5,96 m, z = 0,471 m (le front de la lame) |
| 2,01 s | 0,165 / 0,157 m | −6,3 % | 0,88 mm, x = 3,02 m, z = 0,400 m (la surface, sur le fond plat) |
| 2,40 s | **0,207 / 0,182 m** | −9,3 % | 0,96 mm, x = 5,83 m, z = 0,413 m |
| 3,00 s | 0,132 / 0,182 m | −15,5 % | 1,03 mm, x = 5,53 m, z = 0,378 m |

(a) **Les fronts divergent dès 1,26 s** : la lame avec projection part d'abord 3 cm en avance, puis plafonne à 0,18 m, contre 0,21 m sans
projection.

(b) **L'énergie de la lame baisse continûment** avec la projection, de 2 % à 0,8 s jusqu'à 15 % à 3 s.

(c) **Le déplacement n'est pas concentré dans le ressaut** : environ 1 mm à chaque pas, presque toujours **à la surface libre**
(z ≈ 0,40 m), sur le fond plat comme sur la pente.

**Le profil** : avec la projection, **toute la surface est plus haute de 3 à 5 mm**, jusque loin derrière la vague, sur le fond plat.

## Ce que cela dit

- **La cause est localisée : la projection dilate la couche de surface.** Aux mailles de surface, elle ne corrige que l'excès de densité.
  C'est une correction d'un seul côté, le défaut que S709 avait vu à l'intérieur (+7,4 % en 1,6 s). L'eau soulevée perd l'énergie de sa
  vague, et la lame monte moins haut.
- `WithoutSurface` n'y touche pas : c'est pourquoi la lame y remontait juste (S745). Mais le tassement sous la crête n'y était plus corrigé,
  d'où la traîne derrière l'onde.

## La suite (S749)

**La surface corrigée dans les deux sens, vers sa densité attendue** : une maille de surface n'est que partiellement remplie, et sa densité
cible est sa fraction d'eau, lue par φ (`clamp(½ − φ/dx, 0, 1)`), et non 1. Jugée sur les trois essais du banc (le repos, l'onde
solitaire, la remontée), avec le niveau moyen de la surface relevé.
