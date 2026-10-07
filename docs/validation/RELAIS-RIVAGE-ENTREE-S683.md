# Le relais au rivage, brique 3 : l'entrée à droite d'APIC 3D — S683 (liste 4.14)

*S683, 2026-10-08, en autonomie, vers la v2.* La troisième brique du relais au rivage
([conception](../registres/RELAIS-RIVAGE-S679.md), [ADR-271](../adr/ADR-271-le-film-du-rivage-a-saint-venant.md)) : le reflux qui revient
de Saint-Venant 2D entre dans APIC 3D.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s683 -- --nocapture` (≈ 3 s).

## Ce qui a été fait

`Apic3::feed_right(volumes, vitesse)` s'appuie sur la sortie de S682.

- Un volume donné par rangée s'ajoute à un **réservoir**.
- Chaque quantum entier (`dx³/8`) devient une particule. Elle est posée dans la dernière colonne, sur le réseau au quart de maille, à
  la place la moins occupée sous la surface et au-dessus du fond, la plus basse d'abord. Elle reçoit la vitesse d'entrée.
- La masse se compte : particules × quantum + réservoir + sorti = départ + reçu.

Une particule que la capacité refuse est comptée, et son quantum reste au réservoir.

## Mesuré

**Le montage** : le bassin de S682, l'eau à 0,2 m, nourri par le bord droit à 0,1 m/s sur sa hauteur mouillée pendant 1 s.

**Ce qui départage** :

- une entrée juste rend le bilan exact et la colonne du bord peu tassée ;
- des particules posées au même endroit surchargeraient la colonne et lanceraient une vitesse parasite.

| | critère | mesuré |
|---|---|---|
| (1) sans entrée | au bit | les essais d'APIC passent ; S682 inchangé |
| (2) le bilan, à chaque pas | 10⁻¹² en relatif | **3,5·10⁻¹⁶** (1,98 L reçus, 126 particules, 11,25 cm³ au réservoir, moins d'un quantum de 15,6 cm³) |
| (3) la dernière colonne | ≤ 10 par maille mouillée | **7,12** |
| (4) la vitesse maximale | < 0,5 m/s | **0,165 m/s** |

Le plan attendait 2,0 L et 128 particules : la hauteur mouillée lue aux étiquettes est un peu plus basse que 0,2 m.

## Suite

Le raccord lui-même : APIC 3D et Saint-Venant 2D côte à côte, au repos sur les six plages de S678 (ADR-272 D1 : trois places au moins
de la ligne d'eau dans la maille).
