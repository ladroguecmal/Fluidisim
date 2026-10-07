# Le tsunami sur un rayon courbe — S584 (liste 3.4)

*S584, 2026-10-07, en autonomie.* [S582](TSUNAMI-S582.md) propage un tsunami sur un rayon droit, [S583](REFRACTION-S583.md) trace des rayons
qui se courbent : les voici branchés.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s584 -- --nocapture` ; suite du cœur : 761.

## 1. Ce qui est construit

`tsunami::sur_rayon(a0, h0, point, voisin, b0, h)` : en un point du rayon tracé, l'arrivée est l'instant du point ; l'amplitude,
`A₀·(h₀/h)^(1/4)·K_r` — la levée de Green et la réfraction ensemble.

## 2. Mesuré (références écrites au plan par son script)

Le fond de S583, un tsunami de 0,5 m lancé à 30° de la normale aux isobathes.

| | référence | mesuré |
|---|---|---|
| l'arrivée à 190 km | 1 604,6227 s (Simpson) | **1 604,6229 s** |
| l'amplitude au point du rayon (calculée dans l'essai à son abscisse : Green × `√(cos θ₀/cos θ)`) | 0,897340 m (à 190 km exactement : 0,897047) | **0,897333 m** (8·10⁻⁶) |
| le flux `A²·√h·b` en cinq points | constant | constant |
| une profondeur nulle, un écart nul | refus | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**. Le critère (3) est vrai **par construction** : `sur_rayon` calcule `A` de façon que
`A²·√h·b` reste égal à sa valeur de départ ; il vérifie l'assemblage, pas la physique — c'est le critère (2), contre la forme fermée, qui
la juge.

## 3. Ce que cela dit — et ne dit pas

Un tsunami arrive à la côte à l'heure et à la hauteur que la réfraction et la levée prédisent ensemble, sur un rayon qui se courbe. Manquent
pour 3.4 : l'étalement d'une source ponctuelle (un faisceau de rayons), les caustiques, la dispersion des sources courtes, le déferlement
et le raffinement à la côte, l'événement de W qui le porte.
