# Une barge s'enfonce par un compartiment envahi — S548 (liste 6.6)

*S548, 2026-10-06, en autonomie.* 6.6 (« grands navires ») était absent. Son premier cas : un navire qui s'enfonce parce qu'un
compartiment s'envahit — le corps rigide (S331–S539) et un compartiment de V (S538–S547) couplés.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s548 -- --nocapture` ; suite du cœur : 707.

## 1. Le montage (aucun code neuf du cœur)

Une barge de 20 × 8 × 4 m, 246 t (tirant 1,5 m), corps rigide au proxy en couches de 25 cm, amortissement de pilonnement 1 MN·s/m. Dans
son repère, un compartiment central de V (5 × 8 × 4 m, ouvert à l'air), une brèche de 0,1 m² à son fond ; la mer, vue du navire, un nœud de
V dont la surface suit le pilonnement. À chaque pas de V (0,1 s) : la mer replacée, le pas de V, l'eau du compartiment ajoutée à la masse
portée par la coque, dix pas du corps. Le compartiment est central : le pilonnement seul, sans assiette (ADR-232 D1).

## 2. Mesuré

| | mesuré | attendu |
|---|---|---|
| tirant final (1 500 s) | **1,9995 m** | flottabilité perdue `T' = T·A/(A − A_c)` = 2,000 m (écart 0,025 %) |
| eau embarquée | **79,915 m³** | `A_c·T'` = 80 m³ (0,1 %) |
| surface intérieure | **−1,6 mm** de la flottaison | 0 |
| masse d'eau de V | exacte à chaque pas | — |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) le tirant à 1 % de 2,000 m | 0,025 % | tenu |
| (2) l'eau à 1 % de 80 m³ ; la surface intérieure à 1 cm | 0,1 % ; 1,6 mm | tenu |
| (3) la masse de V exacte | exacte | tenu |

## 4. Ce que cela dit, et ce qui manque

Le couplage d'un corps et de ses compartiments donne l'équilibre de la théorie navale sans rien inventer : l'eau entre par la brèche,
pèse dans la coque, la coque s'enfonce, l'eau monte encore jusqu'à la flottaison. **6.6 devient partiel.** Manquent l'assiette et la gîte
d'un compartiment décentré (l'eau qui court dans un compartiment incliné, la carène libre), la poche d'air d'un compartiment scellé qui
porte la coque (S538–S539 couplés), la stabilité jusqu'au chavirement, plusieurs compartiments et leurs cloisons, un navire réel (la coque de
δ, son sillage, 4.13) et les brèches qui s'ouvrent en jeu.
