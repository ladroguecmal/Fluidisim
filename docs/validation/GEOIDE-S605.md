# Le géoïde dans l'outil de terrain, le terrain gravé pour le squelette — S605 (liste 12.4 ; SPEC-005 §3–4)

*S605, 2026-10-07, en autonomie (ADR-247).* 12.4 était absent : « eau en amont du terrain, géoïde dans l'outil de terrain ». Le « zéro »
d'une scène est une distance au centre de la planète ; le squelette hydrographique est une entrée du terrain.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s605 -- --nocapture` ; suite du cœur : 796.

## 1. Ce qui est construit

Un module `geoide.rs` : `Geoide` (le niveau moyen sphérique vu du plan tangent d'une ancre) ; `altitude` et `z_local` — du plan tangent à
l'altitude et retour, écrits sans soustraire deux rayons ; `ecart_plan_tangent` (la table de SPEC-005 §4) ; `Grille` et `conformer` —
l'étape 2 de l'ordre imposé : le terrain gravé pour satisfaire les biefs de S604 (lignes d'eau en altitude), les biefs jamais modifiés.

## 2. Mesuré (références calculées au plan en décimal à 50 chiffres)

| | référence | mesuré |
|---|---|---|
| l'écart sphère / plan tangent à 1 ; 3 ; 10 ; 30 km (`R` = 6 371 km) | 0,0784806 ; 0,706326 ; 7,84806 ; 70,632423 m | à 4·10⁻¹⁶ relatif |
| l'aller-retour altitude ↔ `z`, 605 points jusqu'à 50 km | 10⁻⁸ m | 4·10⁻¹⁴ m |
| le point (30 km, 0, 0) du plan tangent | 70,632162227 m au-dessus du niveau moyen | idem |
| la gravure d'un bief à 30 km (grille de 10 m, terrain à 15 m) | 400 cellules ; creusement max 5,779432256 m | 400 ; idem ; chaque cellule à son fond à 1,4·10⁻¹⁴ m |
| hors du couloir ; le squelette | inchangés | au bit ; inchangé |
| le même fond dans un outil à plan tangent | 70,655602 m trop haut | idem |
| refus | rayon, grille vide, pas, bief plat | tenu |

Critères (écrits avant) : (1)–(5) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Un lit de rivière placé à 30 km de l'ancre par un outil à plan tangent flotterait **70,7 m** au-dessus de sa place ; placé par le géoïde,
chaque cellule tombe à son fond à 10⁻¹⁴ m près. La table de SPEC-005 écrit 70,7 m à 30 km : pour 6 371 km, 70,63 m (un arrondi ; note datée
ajoutée à SPEC-005 §4).

Manquent : l'anomalie régionale et la marée du niveau moyen (ADR-002 §2.4), le trait de côte et la bathymétrie du squelette, les
dérivations de l'étape 3, les ancres multiples, le terrain de DyingStar (ses tuiles HEALPix, ADR-219 D4).
