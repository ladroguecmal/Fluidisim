# Le courant de marée — S580 (listes 2.2, 7.7)

*S580, 2026-10-07, en autonomie.* S579 a fait entrer le niveau de la marée dans B ; manquait son courant — celui que la traversabilité
lit (SPEC-006 §5.5 : `flow_speed` est le courant de B, sans l'orbitale).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s580 -- --nocapture` ; suite du cœur : 756.

## 1. Ce qui est construit

La quantité de mouvement linéaire, sans frottement ni Coriolis : `∂u/∂t = −g·∇η`. Pour chaque composante `η = Re(H·e^(iωt))`,
`u = −(g/ω)·(∇Re H·sin ωt + ∇Im H·cos ωt)` — **sans la profondeur** : la carte cotidale porte déjà la propagation (une onde progressive
redonne `u = η·√(g/h)`). `CarteCotidale::courant(x, y, t, g)` (le gradient de l'interpolation bilinéaire) ; `maree::avec_courant` ajoute
le courant à la vitesse horizontale de l'échantillon de B.

## 2. Mesuré (références écrites au plan par son script)

L'onde M2 progressive de S578 (chenal de 20 m, `A` = 1 m), au milieu d'une maille de 10 km, 25 h au pas d'une minute.

| | référence | mesuré |
|---|---|---|
| l'amplitude de `u` | `(g·k/ω)·sinc(k·Δx/2)` = 0,700062 m/s (l'onde : 0,700356) | **0,700060 m/s** |
| `v` (transverse) | 0 | **0** |
| le pic du courant et celui du niveau | au même instant (onde progressive) | **3 900 s et 3 900 s** |
| `avec_courant` | `u_total[0..2]` seulement | tenu, le reste au bit |
| hors de la grille | refus | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La marée de B a son courant : un gué sous le flot devient dangereux par la vitesse, pas seulement par la profondeur. Ne dit rien : le
frottement sur le fond, Coriolis (`f` ≈ 10⁻⁴ s⁻¹, comparable à la pulsation de M2 — à reprendre avant les grandes baies), le courant
non linéaire des hauts-fonds.
