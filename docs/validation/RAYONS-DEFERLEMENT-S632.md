# Le déferlement le long des rayons de houle — S632 (liste 3.5)

*S632, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* Un manque de S630 : la hauteur de houle supposait des isobathes
parallèles (`transformer`). Ici, elle se lit le long des rayons de houle dispersifs (S603), quel que soit le fond.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s632 -- --nocapture` ; suite du cœur : 817 essais listés.

## 1. Ce qui est construit

`portee::tracer_houle` (le rayon dispersif de S603, public) ; `deferlement::sur_rayons(a, b, b₀, …)` — le long du rayon `a`,
`H = H₀·K_s·K_r` (la levée de `bathymetrie`, la réfraction `√(b₀/b)` contre le rayon voisin `b`, `refraction::coefficient`), le premier passage
de `H − 0,78·h` par zéro, interpolé.

## 2. Mesuré (références calculées au plan par `s632_ref.py`, numpy)

| | référence | mesuré |
|---|---|---|
| côte droite `h = 0,02·x`, houle de 8 s et 1,5 m à θ₀ = 0,3 rad : l'analytique (Snell, levée, `K_r`) | x_b = 112,59945 m | — |
| le long des rayons, pas 2 ; 1 ; ½ s | 112,61847 ; 112,60477 ; 112,60048 m (écarts 1,9 cm ; 5,3 mm ; 1,0 mm) | à 10⁻¹⁴ m de numpy |
| île conique, incidence normale, rayons à ±200 et ±300 m | les deux de chaque paire déferlent, en miroir | (193,888 ; ±130,041) et (138,799 ; ±185,242), miroir à 3·10⁻¹² m |
| refus : `b₀` nul, rayon vide | | tenu |

Bornes du montage assertées au plan (ADR-257 D1) : le départ en eau profonde, la coupure sous la profondeur de déferlement. Critères (écrits
avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Le long des rayons, la houle déferle là où l'analytique le dit sur une côte droite, à un millimètre près ; autour d'une île, la
convergence des rayons la fait déferler plus au large (à 233 m du centre au lieu des 214 m de l'incidence normale de S630) : c'est la
hauteur réfractée par une côte courbe.

Manquent : les caustiques (`b → 0` : là où les rayons se croisent derrière l'île), la diffraction, le chaînage de ces points en polyligne
(avec S630), le flux dissipé et la direction sur ces points.
