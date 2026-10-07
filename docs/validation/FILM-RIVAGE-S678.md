# Le film du rivage dans APIC 3D : deux causes, un remède qui ne tient pas partout — S678 (liste 4.14)

*S678, 2026-10-07, en autonomie, vers la v2.* Sur le fond lisse de S640, l'eau au repos avec un rivage court à 0,26 m/s. S640 avait
nommé la cause : le film d'eau plus mince que le noyau, où la surface de Zhu et Bridson se trompe.

## Reproduire

- `cargo test --release --offline -p water-core --lib the_shore_film_remedies_measured_s678 -- --ignored --nocapture`, depuis
  `code/water-core` (≈ 4 min ; lancé depuis une copie du binaire, ADR-265 D1).

## Ce qui a été essayé

1. **Le remède du plan : la surface du film par sa dernière couche** (`film_smooth`). Dans une colonne de moins de trois mailles d'eau,
   la surface est la plus haute particule plus `dx/4`, corrigée de l'écart que le noyau lirait d'une nappe au repos à cette place dans
   la maille (une table de 64 décalages, `lattice_read_error`). `φ = z − surface` y est mêlé à celui du noyau.
2. **Trouvé en route** : à la pointe de la ligne d'eau, les faces que la pente n'ouvre qu'à peine donnent des vitesses fausses aux
   particules. Ces faces (moins de 40 % d'ouverture) gardent leur fraction dans la projection, mais leur vitesse est extrapolée des
   faces voisines (`seuil_face`), comme une face d'air.

Les deux sont **éteints par défaut**. Sans eux, le comportement d'avant est retrouvé au bit (0,2591 m/s, S640).

## Mesuré

La vitesse parasite maximale sur 2 s, l'eau au repos :

| plage | maille | rien | le film seul (le plan) | le film et les faces |
|---|---|---|---|---|
| 1:3, eau 0,40 m | 5 cm | 0,259 | 0,328 | **0,0025** |
| 1:3, eau 0,40 m | 2,5 cm | 0,285 | 0,224 | **0,0031** |
| 1:10, eau 0,18 m | 5 cm | 0,583 | 0,395 | 0,394 |
| 1:10, eau 0,18 m | 2,5 cm | 0,398 | 0,355 | **0,0022** |
| 1:3, eau 0,31 m | 5 cm | 0,259 | 0,328 | **0,0025** |
| 1:3, eau 0,31 m | 2,5 cm | 0,208 | 0,144 | 0,145 |

En m/s. Le témoin (le film et les faces, sans la correction de lecture, 1:3 à 5 cm) donne **1,66 cm/s**. Le calcul du plan annonçait
« quelques cm/s » : la correction de lecture compte.

| critère (écrit avant) | mesuré |
|---|---|
| (1) au plus 1 cm/s au repos, à 5 et 2,5 cm, avec le remède du plan | **manqué** : 0,33 et 0,22 m/s avec le film seul |
| (2) la pente immergée, au bit | tenu : 8,06·10⁻⁶ m/s |
| (3) le témoin sans correction, au-dessus de 1 cm/s | 1,66 cm/s |
| (4) les essais du fond en escalier inchangés | tenu par construction (film éteint, seuil nul) ; banc de non-régression passé |

**Ce que l'on sait maintenant.**

- **Deux causes, et non une.** La surface du film, et les vitesses des faces à peine ouvertes. Chacune seule ne suffit pas ; les deux
  ensemble tiennent le repos dans quatre cas sur six, à quelques mm/s.
- **Dans les deux cas qui manquent, le remède ne change rien.** La particule la plus rapide est toujours à la pointe de la ligne d'eau, à
  3 mm du fond. La place de la ligne d'eau dans la maille décide, pas la pente ni la maille seules.
- **Les plages 1:3 à 0,40 m et 0,31 m donnent les mêmes vitesses** (à 10⁻⁴ près) à 5 cm, la particule la plus rapide à la pointe dans
  les deux cas. Ce n'est pas expliqué, seulement relevé.

## Ce que cela dit

La ligne de contact d'un film sur un fond coupé, dans une méthode à particules, ne se règle pas par la seule reconstruction de la
surface. Le remède le plus prometteur n'est pas dans APIC : c'est de confier le film du rivage (moins de deux ou trois mailles d'eau) à
Saint-Venant 2D. Ce solveur porte déjà le mouillage et le séchage exactement (Thacker, S613–S620), et le relais 2D → 3D existe (S650).
Le relais dans les deux sens est ce que 4.14 nomme encore parmi ce qui manque.
