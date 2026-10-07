# Le relais au rivage, brique 1 : Saint-Venant 2D rend le flux de ses bords — S680 (liste 4.14)

*S680, 2026-10-08, en autonomie, vers la v2.* La première brique du relais au rivage
([conception](../registres/RELAIS-RIVAGE-S679.md), [ADR-271](../adr/ADR-271-le-film-du-rivage-a-saint-venant.md)).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s680 -- --nocapture` (≈ 1 s).

## L'interface, précisée

La conception prévoyait un flux HLL calculé à part et imposé aux deux côtés. Plus simple, et aussi exact :

- Saint-Venant 2D garde son bord caractéristique (S622, S628), nourri par l'état de la dernière colonne 3D ;
- il rend le flux de masse qu'il a réellement fait passer ;
- APIC retire ou pose des particules pour ce même volume, et un réservoir garde le reste d'un quantum de particule.

## Ce qui a été fait

`SaintVenant2D::flux_des_bords()` rend, au dernier pas, le flux de masse de chaque rangée à travers la face gauche (entrant) et la droite
(sortant). C'est la demi-somme des deux étages de Heun, exactement ce qui change le volume. Il est nul sur un mur.

## Mesuré

**Le montage** : une houle de 10 cm entrée par la gauche, le bord droit forcé, un fond en pente ; 2 000 pas.

**Ce qui départage** : un flux relevé juste rend `ΔV = dt·dx·Σ_j (gauche_j − droite_j)` à l'arrondi près. Le flux d'un seul étage s'en
écarterait de l'ordre de la variation du flux pendant le pas.

| | critère | mesuré |
|---|---|---|
| (1) le bilan, à chaque pas | 10⁻¹² en relatif | **4,6·10⁻¹⁵** (1,36 m³ entrés) |
| (1) sur un mur | nul | nul |
| (2) les essais de S613–S628 | mêmes sorties | identiques |

## Suite

- S682, la brique 2 : le bord droit d'APIC 3D, qui retire et pose des particules pour un volume donné, avec son réservoir.
- S683, le raccord au repos sur les six plages de S678.
