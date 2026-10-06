# L'air scellé avec plusieurs liquides — S563 (ADR-241 ; ADR-015 T2 ; liste 5.7)

*S563, 2026-10-06, en autonomie.* `step_liquids` (S560) ignorait l'air des compartiments. Un compartiment étanche qui contient du carburant
et que la mer envahit par le fond doit comprimer son air, comme en [S538](C17-AIR-S538.md).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s563 -- --nocapture` ; suite du cœur : 728.

## 1. Ce qui est construit

`liquids::step_liquids_air(…, air, pressions_pa)` : la pression de jauge de chaque poche scellée (Boyle isotherme, `Air::Sealed`) ajoutée
des deux côtés du seuil d'un orifice ou d'une vanne ; un seuil au-dessus de la surface amont ne laisse passer aucun liquide ; les évents
par le calcul de `step_air`, désormais partagé (`vent_air`). Les déversoirs et les pompes ne voient pas la pression des poches, comme dans
`step_air`. Tout ouvert : `step_liquids` au bit.

## 2. Mesuré

Un compartiment de 1 × 1 × 2 m contenant 0,5 m³ d'huile (ρ = 850) et 1,5 m³ d'air ; la mer de 100 × 100 m à 1,5 m au-dessus de la brèche du
fond (deux orifices de 1 000 mm²) ; 3 000 s (constantes de temps calculées : 96 s scellé, 755 s ouvert). Références résolues à part
(bissection : l'équilibre des pressions au seuil et Boyle ensemble, la mer qui baisse de ce qui entre).

| | référence | mesuré |
|---|---|---|
| scellé : l'eau entrée sous l'huile | 0,126197 m (l'air à +9 307,6 Pa) | **0,126196 m** (1 µm, le quantum) |
| ouvert (le témoin) | 1,074893 m | **1,074893 m** |
| l'huile, dans les deux cas | 500 000 ml dans le compartiment | **500 000 ml** |
| tout ouvert contre `step_liquids`, le manomètre de S560 | au bit | **au bit** |
| `step_air` (la suite entière, 728 essais) | inchangé | inchangé |

Critères (écrits avant) : (1) à 10⁻⁴ m — **tenu** ; (2) à 10⁻⁴ m — **tenu** ; (3) au bit — **tenu** ; (4) — **tenu**.

## 3. Ce que cela dit

L'air enfermé retient la mer sous un liquide plus léger comme sous de l'eau : la poche de carburant d'une soute scellée monte de 12,6 cm
quand l'eau entre par le fond, au lieu de 107 cm ouverte. Côté V, 5.7 a ses couches, leur pression, leur débit, leur air et leur
instantané. Reste, hors de V : un liquide autre que l'eau qui sort vers la mer, δ ou le sol (sa nappe, son rendu ; ADR-241 D5).
