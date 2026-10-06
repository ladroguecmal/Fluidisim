# Les anneaux d'impact de W en eau peu profonde — S528 (liste 3.1)

*S528, 2026-10-06, en autonomie.* `RadialImpact` refusait le régime peu profond (`Error::Regime`, profondeur ≤ π/k_min). W porte la
profondeur uniforme depuis S522 (le nombre d'onde effectif `κ = k tanh kh`).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s528 -- --nocapture` — deux essais ; suite du cœur : 687.
- `code/target/release/examples/anneaux_profondeur.exe 1 calculs/anneaux_s528_h1.bin` (1 s), puis
  `python outils/reference_anneaux.py calculs/anneaux_s528_h1.bin 1`.

## 1. La construction (`radial_impact.rs`)

**`RadialImpact::new_in_depth`** : `κ` dans la pulsation (`ω² = g κ`), le potentiel (`η_t/κ`) et la vitesse horizontale (`k/κ` fois celle de
l'eau profonde) ; le régime peu profond n'est plus refusé ; `slope_max_at` ne resserre pas (la table `RHO_DISPERSION` est mesurée en eau
profonde), `slope_max_beyond` (une enveloppe de J1, indépendante de la dispersion) reste. `new` inchangé.

## 2. La référence, indépendante de la somme de Bessel

Le champ initial de W (1 ms, ± 80 m — la somme à 128 modes ne vaut qu'en deçà de ≈ 85 m, sa récurrence en `2π/dk`), prolongé par des
zéros, propagé par FFT avec la dispersion exacte `cos(√(g k tanh kh) t)` (`outils/reference_anneaux.py`) ; trois grilles.

## 3. Mesuré — λ = 4 m, 1 m de fond (`tanh(k₀h)` = 0,917), disque de 40 m

| | 5 s | 10 s |
|---|---|---|
| W contre la référence en profondeur finie (256 m et 384 m à 25 cm, 256 m à 12,5 cm) | **0,44 %** (les trois) | **0,44 %** (les trois) |
| le même champ initial propagé en eau profonde | 51 % | 94 % |
| `new_in_depth` à 30 m (`2 k_min h` > 32) contre `new` | au bit (quatre âges, quatre points) | |
| la pente réelle par 1 m de fond, 0–10 s, contre `slope_max_at` / `slope_max_beyond` | au plus 0,990 de la première ; sous la seconde partout | |

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) `new` au bit ; `new_in_depth` en eau assez profonde au bit de `new` | suite 687 ; au bit | tenu |
| (2) W contre la référence ≤ 2 % ; l'eau profonde à plus de 5 fois | 0,44 % ; 51 % et 94 % | tenu |
| (3) la pente réelle sous les deux bornes | 0,990 ; partout | tenu |

## 5. Ce que cela dit, et ce qui manque

Les anneaux de W suivent la dispersion en profondeur finie à 0,4 % près. Manquent la gerbe (4.12), une profondeur variable sous un
anneau, et l'hôte qui choisit `new_in_depth` selon le fond (les points d'appel de `new` restent en eau profonde).
