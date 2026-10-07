# Le frottement et Coriolis dans le courant de marée — S634 (liste 2.2)

*S634, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* Un manque de S580 : le courant de marée y était `−g·∇η` intégré, sans
frottement ni Coriolis.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s634 -- --nocapture` ; suite du cœur : 819 essais listés.

## 1. Ce qui est construit

`CarteCotidale::courant_amorti(x, y, t, g, f, r)` : pour chaque composante, la solution harmonique établie de `∂u/∂t + r·u − f·v = −g·∂η/∂x`,
`∂v/∂t + r·v + f·u = −g·∂η/∂y` — `a = iω + r`, `U = −g(a·Gx + f·Gy)/(a² + f²)`, `V = −g(a·Gy − f·Gx)/(a² + f²)`, `G` le gradient complexe de
l'interpolation de S580. En f32, des opérations de base : déterministe.

## 2. Mesuré (références calculées au plan)

Une carte M2 (onde progressive selon `x`, 400 km, 1 m), un point intérieur.

| | référence | mesuré |
|---|---|---|
| `f` = `r` = 0, contre `courant` (S580), 200 instants | à 10⁻⁶ m/s | 2,4·10⁻⁷ |
| `f` = `r` = 10⁻⁴ s⁻¹, contre une intégration RK4 indépendante (pas de 60 s, 10 jours ; la dernière période) | à 10⁻⁶ m/s | 2,3·10⁻⁷ |
| l'atténuation par le frottement seul, `ω/√(ω² + r²)` | 0,814749 (à 10⁻⁴) | 0,814748 |
| l'ellipse par Coriolis seul, `f/ω` | 0,711648 (à 10⁻⁴) | 0,711647 |
| refus : `r` négatif, `g` nul | | tenu |

Bornes du montage assertées au plan (ADR-257 D1) : le transitoire éteint après 10 jours (`e^(−86)`), `f < ω`. Critères (écrits avant) :
(1)–(5) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Le courant de marée sait maintenant tourner et s'amortir : sous Coriolis, une onde qui progresse selon `x` décrit une ellipse dont le petit
axe vaut `f/ω` du grand (0,71 à mi-latitude pour M2) ; sous un frottement de 10⁻⁴ s⁻¹, il perd 19 % et prend du retard. La solution établie
est celle qu'une intégration pas à pas atteint, à 2·10⁻⁷ m/s.

Manquent : le frottement quadratique (`r` linéarisé ici), le transitoire, `f` variable avec la latitude, la profondeur dans le frottement,
les corrections nodales, le niveau moyen par la météo.
