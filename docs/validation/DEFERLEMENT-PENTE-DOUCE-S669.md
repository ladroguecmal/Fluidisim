# Le déferlement d'une mer dans la pente douce : Battjes et Janssen — S669 (liste 2.7)

*S669, 2026-10-07, en autonomie, vers la v2.* La marche parabolique (S659–S665) n'avait aucune dissipation. Sans elle, la levée croît
sans borne vers le rivage, et `Cote2D` devait s'arrêter avant que la mer ne déferle.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s669 -- --nocapture` (≈ 1 s).

## Ce qui a été fait

- **La marche par rangée.** `marche_grand_angle` devient un état (`Marche`) qui avance d'une rangée à la fois, sans changer son
  arithmétique. L'empreinte de trois marches existantes (Berkhoff linéaire et non linéaire, la plage périodique), prise avant le
  remaniement, est inchangée : `4a10864d51e1108b`.
- **`propager_spectre_periodique`** : les composantes d'une mer marchent ensemble. En chaque nœud, la mer entière donne
  `Hrms = 2·√(Σ (a_c·|A_c|)²)`. Battjes et Janssen (1978) en tirent le taux `D/E = 2α·f̄·Q_b·(H_max/Hrms)²`, avec
  `H_max = 0,88/k̄·tanh(γ·k̄·h/0,88)`. Chaque composante est amortie au même taux, comme dans REF/DIF-S (Chawla, Özkan-Haller et Kirby
  1998) : `A_x` reçoit `−(w/2)·A`, avec `w = (D/E)·ω/(p·k_x)`, la dissipation rapportée au flux normal.
- **`γ`** suit Battjes et Stive (1985) : `0,5 + 0,4·tanh(33·s₀)`, avec `α` = 1.

## Mesuré

**Le montage** : la mer de S667 (huit composantes, 7 à 12 s, −20° à +20°, `Hrms₀` 1,55 m) sur une plage de 80 m à 1 m de fond
(pente 1:50, 3,95 km), avec les bords périodiques tournés.

**L'instrument** : l'équilibre d'énergie 1D de la même mer (chaque composante réfractée par Snell), intégré à part par RK4 au mètre, avec
son propre `Q_b` (une bissection sur `ln Q`).

**Ce qui départage** :

- une dissipation juste suit la référence à la précision de la marche sans déferlement ;
- un `Q_b` inversé s'en écarte de plusieurs dizaines de %. Au calcul du plan, la bissection inversée donnait `Hrms` = 0,20 m au lieu de
  0,89 m à 2 m de fond.

| | critère | mesuré |
|---|---|---|
| (1) le remaniement | au bit | l'empreinte inchangée ; la marche spectrale sans déferlement au bit des huit marches séparées |
| (2) `Hrms(s)` contre l'équilibre 1D, à chaque rangée | < 3 % | **0,28 % au plus** (au rivage, à 1 m) |
| (3) `Hrms` au rivage (1 m) | rapporté | **0,513 m** (1D : 0,511) ; sans déferlement, **2,43 m** |
| (3) `Hrms/h` au rivage | entre 0,35 et 0,55 | **0,51** |

À 6 m de fond, où le déferlement commence (`Q_b` ≈ 0,006), la marche et la référence donnent toutes deux 1,618 m. Le repère de
Thornton et Guza (1982), `Hrms/h` ≈ 0,42 dans une zone de déferlement saturée, est cité de mémoire. Il sert de vraisemblance, pas de
mesure : 0,51 à 1 m de fond, au bord intérieur, reste dans l'intervalle écrit avant.

## Ce que cela dit

La marche parabolique porte maintenant une mer qui déferle. La dissipation est commune à toutes les composantes, juste à 0,3 % de
l'équilibre d'énergie. Sans elle, la mer arrivait au rivage avec une hauteur près de cinq fois trop grande.

**Ne fait pas** : le jet de rive (la profondeur doit rester positive), la remontée du niveau moyen, le rouleau qui retarde la
dissipation (Svendsen 1984), le frottement sur le fond. Le jugement contre une mesure 2D avec déferlement (Vincent et Briggs 1989) reste à
faire : ses données ne sont pas dans le dépôt.

**Suivant** : la côte 2D cuite avec le déferlement (`Cote2D`), jusqu'au rivage.
