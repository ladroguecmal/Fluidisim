# La résonance de C07 en deçà du critique — S525 (listes 3.2, 13.2)

*S525, 2026-10-06, en autonomie.* Dernière branche de C07 : « la pente de `log A` contre `log|1 − Fr_h²|` vaut −½ ± 0,15 sur
`Fr_h` ∈ {0,3 ; 0,5 ; 0,7 ; 0,9} » ([CAS-CANONIQUES](CAS-CANONIQUES.md), note S30).

## Reproduire

- `python outils/reference_sillage.py resonance` — la référence seule.
- `code/target/release/examples/c07_profondeur.exe calculs/res_s525_U<U>.bin <U> 64 0 60 60 2 0.4 512 512 20 60` pour `U` = 2,1011 /
  3,5018 / 4,9025 / 6,3032 m/s (4 s chacun), puis `python outils/reference_sillage.py resonance calculs/res_s525_U2.1011.bin …`.

## 1. La théorie, calculée avant

En ondes longues (source large devant le fond), l'équation permanente `(1 − Fr²) η_xx + η_yy = −∇²p/ρg` donne sous une source isotrope
**exactement** `η(0) = −(p₀/ρg)/√(1 − Fr²)` : la moyenne angulaire de `1/((1 − Fr²)cos²θ + sin²θ)` vaut `1/√(1 − Fr²)` (le facteur de
Prandtl–Glauert). D'où la pente −½. La note de S523 (« une source fine ») était fausse : la loi demande une source **large**.

La référence exacte (profondeur finie, dispersive, en temps fini) ne la suit que hors du voisinage critique : à `Fr_h` = 0,9 le régime
n'est pas permanent (le temps d'établissement croît comme `σ/((1 − Fr)c)`) et la dispersion compte. Pente sur quatre points : −0,60 (σ
20 m, 32 s), −0,72 (64 s), −0,80 / −0,85 (σ 10 m).

## 2. Mesuré — σ 20 m, 5 m de fond, 64 s, recette 512 × 512 à coupure 0,4 (rayon honnête 2 681 m)

| `Fr_h` | 0,3 | 0,5 | 0,7 | 0,9 | pente, 4 points | pente, 0,3–0,7 |
|---|---|---|---|---|---|---|
| Prandtl–Glauert `1/√(1 − Fr²)` | 1,0483 | 1,1547 | 1,4003 | 2,2942 | −0,500 | −0,500 |
| référence, `A/(p₀/ρg)` | 1,0531 | 1,1674 | 1,4422 | 3,2077 | −0,721 | **−0,544** |
| **W** | 1,0529 | 1,1672 | 1,4418 | 3,2071 | **−0,721** | **−0,544** |

W suit la référence à **0,03 %** près sur les quatre amplitudes.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) la théorie : sur 0,3–0,7, −½ ± 0,05 ; sur quatre points, sa valeur publiée | −0,544 ; −0,721 | tenu |
| (2) W : amplitudes à 2 % de la référence ; pente sur quatre points à 0,05 de la sienne ; sur 0,3–0,7, −½ ± 0,15 | 0,03 % ; identique ; −0,544 | tenu |
| (3) la note de C07 corrige l'assertion | ci-dessous et dans CAS-CANONIQUES | fait |

## 4. Ce que cela dit

La résonance de W est celle de la théorie linéaire en profondeur finie. **L'assertion de C07, telle qu'écrite en S30, n'est pas celle de
la théorie à durée finie** : à `Fr_h` = 0,9, la dépression dépasse Prandtl–Glauert de 40 % et croît encore avec le temps, et la pente sur
les quatre points vaut −0,72. Sur le régime permanent (0,3–0,7), la pente −½ tient (−0,544) — c'est l'assertion retenue. **C07 est
exécuté dans ses trois branches** : profond (S519), peu profond au-delà du critique (S523, à `Fr_h` = 1,43), résonance (S525).
