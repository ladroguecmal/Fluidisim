# La côte 2D qui déferle : Cote2D jusqu'au rivage — S670 (liste 2.7)

*S670, 2026-10-07, en autonomie, vers la v2.* S669 a mis le déferlement d'une mer dans la marche parabolique. `Cote2D` marchait encore
chaque composante seule, sans dissipation : elle devait s'arrêter à 2 m de fond.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s670 -- --nocapture` (≈ 4 s).

## Ce qui a été fait

`Cote2D::cuire_deferlante` cuit toutes les composantes de B ensemble (`propager_spectre_periodique`), amorties au même taux par Battjes
et Janssen. `γ` vient de Battjes et Stive, tiré de B : `Hrms₀ = 2·√(Σ a²)` et la période moyenne. `cuire_decime` passe par le même
chemin sans déferlement ; ses tables sont au bit d'avant. L'empreinte de trois composantes, à `m` = 1 et 4, est inchangée :
`40c657593a299c83`.

## Mesuré

**Le montage** : la mer de S667 (huit composantes, `Hs` 2,2 m) sur la plage de 80 m à 1 m de fond (3,95 km, pente 1:50), tables au pas
de 2 m.

**L'instrument** : l'équilibre d'énergie 1D de S669, rendu par composante. Il part de l'eau profonde par la conservation du flux,
indépendamment de `transformer`.

**Ce qui départage** : une cuisson juste rend le facteur de chaque composante à la précision de la marche ; un départ mal normalisé
(le facteur WKB oublié dans `Hrms`) déplacerait le déferlement de plusieurs %.

| | critère | mesuré |
|---|---|---|
| (1) sans déferlement | au bit d'avant | l'empreinte inchangée ; les essais S664–S668 inchangés |
| (2) le facteur de chaque composante, à chaque nœud | < 2 % de l'équilibre 1D | **0,40 % au plus** (au rivage, 1 m) |
| (2) au large (`s ≤ 0`) | B au bit | au bit |
| (3) `Hrms` au rivage | rapporté | **0,513 m** (1D 0,511) ; `Hrms/h` 0,51 |
| (3) `η` au rivage, avec et sans déferlement | rapporté | jusqu'à **1,19 m** d'écart |
| (3) la cuisson ; la mémoire à `m` = 4 | rapportées | 1,7 s pour 8 composantes ; 2,0 Mo (3,9 km × 192 m) |

## Ce que cela dit

La côte 2D de B va maintenant jusqu'à 1 m de fond, et la mer y déferle comme l'équilibre d'énergie le veut, à 0,4 % près. Sans
déferlement, la surface au rivage était fausse de plus d'un mètre.

Le pas des tables à 8 m (`m` = 4) tient jusqu'au rivage : la correction y avance de 1,7 rad par nœud. À 16 m (`m` = 8), elle avancerait
de 3,4 rad, et la cuisson le refuse.

**Ne fait pas** : le jet de rive et la terre ; la remontée du niveau moyen (le *setup*) ; les courants de dérive ; l'écume, suspendue ;
le rouleau qui retarde la dissipation. Le relais entre ce `Hrms` et le déferlement 3D (S647–S652), et la côte 2D dans Godot, restent à
faire.
