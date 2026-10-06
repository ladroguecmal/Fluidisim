# L'absorption par le sol : Green–Ampt dans V — S530 (liste 5.5)

*S530, 2026-10-06, en autonomie.* La pluie tombe dans V depuis S378 (ADR-204), mais rien n'entrait dans le sol.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s530 -- --nocapture` — trois essais ; suite du cœur : 690.

## 1. La construction (`code/water-core/src/hydro_network.rs`)

- **`Flow::Infiltration { area_mm2, conductivity_nm_s, suction_um, deficit_pm }`** : de la flaque (`from` ; la surface du sol à la cote de
  l'arête, `h₀` la lame au-dessus) vers le sol (`to`, obligatoire), un nœud de V dont le remplissage rapporté à l'aire est la lame infiltrée
  cumulée `F` — aucun état nouveau. Capacité de **Green–Ampt** `f = K·(1 + (ψ + h₀)·Δθ/F)`.
- **Intégrée exactement sur le pas** (`green_ampt_step`) : `F₁ − F₀ − M ln((M + F₁)/(M + F₀)) = K·dt`, `M = (ψ + h₀)Δθ`, par bissection en
  f64. Un Euler explicite (`F` au début du pas) aurait donné +212 % à 60 s, +37 % à 10 min, +7,6 % à 1 h (calculé avant, au plan).
- Le sol plein, le limiteur d'arrivée l'arrête ; la flaque à sec, rien ne passe ; une flaque que plusieurs arêtes vident, la normalisation
  la partage. La validation (pas et instantané, une seule fonction) et l'empreinte de la base connaissent la loi (l'empreinte n'est pas
  éprouvée par un essai propre).

## 2. Mesuré — limon sableux : K = 1,09 cm/h, ψ = 11 cm, Δθ = 0,3

| | mesuré |
|---|---|
| flaque de 1 000 m² et 1 cm (charge quasi constante) sur 1 m² de sol, contre la solution implicite | `F` = 3,7390 / 12,6790 / 35,7050 mm à 60 s / 10 min / 1 h ; écarts **3·10⁻⁵ / 1·10⁻⁵ / 7·10⁻⁵** |
| masse (flaque + sol) | exacte à chaque pas (36 000 pas) |
| un sol de 10 L sur 1 m² | plein à 10 000 ml exactement, puis plus rien ; la flaque a donné exactement 10 000 ml |
| une flaque à sec | rien d'infiltré |
| 5 mm/h de pluie sur 1 m² de sol pendant 1 h | 4 999 ml dans le sol (la pluie elle-même : 4 999 ml, son quantum), la flaque au plus 1 ml |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) les lois d'avant au bit | suite 690 verte | tenu |
| (2) `F(t)` à 10⁻³ de la solution implicite ; masse exacte | 7·10⁻⁵ au pire ; exacte | tenu |
| (3) le sol plein arrête l'infiltration au millilitre ; une flaque à sec n'infiltre rien | 10 000 ml exactement ; rien | tenu |
| (4) tout entre (5 L au millilitre), la flaque sous 2 ml | 4 999 ml ; 1 ml | tenu |

## 4. Ce qui manque

5.5 avance : la pluie et l'absorption existent dans V. Manquent **la pluie hors contenant** (un sol sans nœud de flaque explicite), le
calcul de l'exposition depuis les objets posés, l'assèchement du sol (drainage, évaporation) et la météo (à la fin, ADR-197 D5).
