# C16 : la cuve dans un référentiel accéléré — S542 (listes 4.17, 13.2)

*S542, 2026-10-06, en autonomie.* C16 : « surface au repos perpendiculaire à `g_eff` » ; « échoue immédiatement si un `−9,81·Z` traîne
quelque part » (I-07). δ recevait la grandeur de `g_eff` (ADR-007), pas sa direction : 4.17 était absent.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s542 -- --nocapture` ; suite du cœur : 703.

## 1. La construction (`code/water-core/src/delta3d.rs`)

**`Volume3::set_horizontal_gravity([g_x, g_y])`** : la composante horizontale de `g_eff` dans le pas linéaire de δ — une force de volume
ajoutée au champ prédit sur les faces ouvertes, la projection gardant les murs (une pression posée au seul couvercle les violerait). À
l'équilibre, la surface `η = (g_h/g)·x`, perpendiculaire à `g_eff`. Nulle : le pas d'avant au bit. La carte GPU ne la porte pas encore.

## 2. Mesuré — une cuve de 8 m, 1,5 m d'eau (32 × 1 × 6 mailles de 25 cm), pas de 10 ms

| | mesuré | attendu |
|---|---|---|
| sous 0,05 g latéral, partie plate : la pente moyenne sur quatre périodes | **0,05005** | `g_h/g` = 0,05 (écart 10⁻³ ; **0,003°** de la normale à `g_eff`) |
| sans pesanteur horizontale, le premier mode lâché en cosinus : la période | **4,4054 s** | `2π/√(g·(π/L)·tanh(πh/L))` = 4,4005 s (écart 1,1·10⁻³) |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) `g_h` nul : la suite au bit | 703 | tenu |
| (2) la pente à 2 % de `g_h/g` — à 1° de la normale à `g_eff` (C16) | 0,1 % ; 0,003° | tenu |
| (3) la période à 1 % (C16 : ± 10 %) | 0,11 % | tenu |

**Deux écarts à l'énoncé de C16, publiés** : sa période « ≈ 3,5 s » n'est pas celle de sa propre formule (4,40 s ; 2,49 s pour le second
mode) — la formule fait foi ; et son 0,3 g latéral dénivellerait la surface de ± 1,2 m sur 1,5 m d'eau, hors du modèle linéaire (le bord
s'assècherait) — l'essai prend 0,05 g.

## 4. Ce qui manque

**4.17 devient partiel ; C16 l'est aussi** : manquent la rotation (la surface d'équilibre cylindrique d'ADR-002 §2.2, une force
centrifuge qui dépend du point), la pesanteur horizontale sur la carte GPU et dans le pas couplé, une inclinaison au-delà du modèle
linéaire, et C06 (l'invariance galiléenne) sur le système.

## 5. S543 — la rotation

*S543, 2026-10-06.* `Volume3::set_horizontal_gravity_field(g₀, Ω², centre)` : la part horizontale de `g_eff` affine, `g₀ + Ω²·(x −
centre)` — dans une cuve d'une station tournante, la force centrifuge s'incline le long de la cuve. Une cuve de 20 m et 2 m d'eau (80 × 1 ×
8 mailles), à 100 m de l'axe (Ω = 0,3132 rad/s, `g = Ω²R`), quatre périodes du premier mode (36,7 s) : la surface moyenne, ajustée par une
parabole, a une **courbure de 0,010152 m⁻¹ pour 1/R = 0,010000 (1,5 %)** — une flèche de 0,508 m sur 20 m pour 0,501 (le cylindre). Critère
(2 % de `1/R`, écrit avant) : tenu. Coriolis n'est pas porté (il n'agit pas sur l'équilibre). **C16 est exécuté** dans ses deux parties,
dans δ linéaire (`cargo test … s543`).
