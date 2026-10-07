# La ligne d'eau d'une rivière et son remous — S593 (liste 2.4)

*S593, 2026-10-07, en autonomie (ADR-247).* 2.4 était absent : « débit macroscopique qui contraint les perturbations locales ». Première
pièce : l'état macroscopique d'une rivière dans V — sa ligne d'eau, le remous qu'un seuil aval lui impose, la vitesse moyenne de chaque bief.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s593 -- --nocapture` (65 s : 216 000 pas de 20 biefs) ;
  suite du cœur : 767.

## 1. Le montage

La loi de Manning de [S592](CANAL-MANNING-S592.md) : vingt biefs de 100 × 5 m, `S` = 10⁻³, `n` = 0,015, 5 m³/s en amont ; le dernier bief se
vide par un déversoir de 5 m dont la crête est à 1,5 m au-dessus de son fond ; trois heures au pas de 0,05 s (au pas d'une seconde la
surface presque plate du remous oscillerait — l'amplitude `(dt·C/A)²` calculée au plan : ~8 cm contre ~10⁻⁵ m).

## 2. Mesuré (références écrites au plan par son script)

| | référence | mesuré |
|---|---|---|
| la ligne d'eau (20 profondeurs, de 0,76 m en amont à 2,17 m contre le seuil) | (1) l'état stationnaire exact du découpage, à 1 mm | **0,014 mm** au pire |
| contre la ligne d'eau continue de l'onde diffusive | (2) l'écart du découpage, 0,75 mm, calculé | tenu |
| la vitesse moyenne de chaque bief | `Q/(b·y)` : de 1,3127 à 0,4612 m/s | à 10⁻³ |
| le bilan | volume = reçu − sorti | **exact au millilitre** |
| la ligne d'eau complète (avec l'inertie, `1 − Fr²`) | l'écart de l'onde diffusive : 40,4 mm au plus | publié, sans critère |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

V porte l'état macroscopique d'une rivière — sa ligne d'eau et la vitesse de chacun de ses biefs, au remous près d'un seuil. **V est une
onde diffusive** : sans le terme d'inertie, sa ligne d'eau s'écarte de la ligne complète de 4 cm au plus ici (Froude ≈ 0,5) — à reprendre si
une rivière rapide l'exige. Manquent pour 2.4 : la contrainte elle-même (ces vitesses transmises à δ et W comme courant — `current_field`,
C1), un lit de forme quelconque et de pente variable, le régime torrentiel, la surface de la rivière dans le rendu.
