# La réfraction par tracé de rayons — S583 (liste 3.6)

*S583, 2026-10-07, en autonomie.* 3.6 était absent. La référence de B (S362) ne traite que des isobathes droites et parallèles ; ici, un
fond quelconque, et ce dont le tsunami de S582 a besoin : des rayons qui se courbent.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s583 -- --nocapture` ; suite du cœur : 760.

## 1. Ce qui est construit

`refraction.rs` : le tracé d'un rayon d'onde longue (`c = √(g·h)`) sur un fond `h(x, y)` fourni avec son gradient — `ẋ = c·cos θ`,
`ẏ = c·sin θ`, `θ̇ = sin θ·∂c/∂x − cos θ·∂c/∂y` —, Runge-Kutta d'ordre 4 à pas de temps fixe, en f64 (déterministe), arrêté à la côte ;
le coefficient de réfraction `K_r = √(b₀/b)` entre deux rayons voisins (l'écart projeté perpendiculairement au rayon, à même instant).

## 2. Mesuré (références écrites au plan par son script)

Un fond `h = 4 000 − 0,0195·x`, un rayon lancé à 30° de la normale aux isobathes, au pas d'une seconde.

| | référence | mesuré |
|---|---|---|
| `sin θ/c` le long du rayon (Snell) | constant | **3,1·10⁻¹¹** au pire |
| l'instant au passage de 190 km | 1 604,6227 s (Simpson) | **1 604,6229 s** |
| l'ordonnée atteinte | 72 949,931 m | **72 949,930 m** |
| `K_r` (deux rayons voisins à 100 m) | `√(cos θ₀/cos θ)` = 0,934944 | **0,934931** |
| un fond uniforme | le rayon droit, `θ` constant | au bit |

Critères (écrits avant) : (1), (2), (4) — **tenus** ; (3) — **manqué à la première mesure (0,981), tenu après correction du montage**.

**Le montage était faux, pas le module.** Le rayon voisin partait à 30° d'un point situé 50 m plus au large : un autre invariant de Snell
(`sin θ/c` plus petit de 1,2·10⁻⁴), dont l'écart d'angle se cumule sur 190 km — 10 m sur 115. Un tracé indépendant en Python a redonné
0,981 avant toute correction (ADR-239 D1 : la formule et le montage relus d'abord). `√(cos θ₀/cos θ)` vaut pour deux rayons **de la même
famille** : le voisin part désormais avec l'angle que Snell lui donne à son abscisse. Le critère est inchangé.

## 3. Ce que cela dit — et ne dit pas

Une onde longue tourne vers l'eau peu profonde et s'y élargit ou s'y resserre comme la théorie le dit, sur un fond quelconque. Ne dit
rien : les caustiques (`b → 0`), la diffraction, les ondes courtes (la dispersion), l'entrée dans W ; et ce traceur n'est pas encore branché
au tsunami de S582 (un rayon courbe au lieu d'un rayon droit).
