# C09 : la masse et l'énergie de δ dans une cuve close — S555 (listes 4.18, 13.2)

*S555, 2026-10-06, en autonomie.* C09 (non exécuté) : « domaine clos, sans frottement, perturbation initiale, 120 s ; `|dm/dt| < 10⁻³ s⁻¹` ;
`dE/dt ≤ 0` en tout temps ».

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s555 -- --nocapture` ; suite du cœur : 714.

## 1. Le montage

δ linéaire (le cœur), une cuve close de 4 × 2 × 1,5 m (16 × 8 × 6 mailles de 25 cm), une bosse gaussienne de 2 cm (σ = 0,5 m), 120 s au pas
de 10 ms. À chaque pas : la masse `ρ·Σ(η − z₀)·dA` rapportée à la masse d'eau ; l'énergie **naturelle**
`E = ½ρ·Σ|u|²·dx³ + ½ρg·Σ(η − z₀)²·dA` (vitesses aux faces, surface aux colonnes, en fin de pas). Le plan avait écrit, avant la mesure, que
cette énergie peut osciller d'un pas à l'autre dans un schéma décalé sans qu'aucune instabilité n'existe.

## 2. Mesuré

| | mesuré |
|---|---|
| `|dm/dt|`, au pire sur 12 000 pas | **2,1·10⁻⁶ s⁻¹** (la masse de perturbation 30,807 → 30,806 kg) |
| énergie naturelle, de pas en pas | **5 955 hausses sur 12 000**, la plus forte 0,79 % de E₀ |
| énergie initiale / finale | 1,573 J → 1,777 J (+13 %, l'instant final) |
| **énergie moyenne par fenêtre de 5 s** (24 fenêtres) | **de 1,7032 à 1,7083 J** : constante à ± 0,15 %, +8,4 % de E₀ dès la première fenêtre |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) `|dm/dt|` < 10⁻³ s⁻¹ | 2,1·10⁻⁶ | tenu |
| (2) `dE/dt ≤ 0` à chaque pas (énergie naturelle) | 5 955 hausses | **manqué** |
| (3) l'énergie finale sous l'initiale | +13 % | **manqué** |

## 4. Ce que cela dit — et ce que cela ne dit pas

**Aucune instabilité** : l'énergie moyenne ne croît pas sur 120 s (± 0,15 % d'une fenêtre à l'autre), et la masse est exacte. Mais **C09
tel qu'écrit ne peut pas passer avec l'énergie naturelle** : vitesses et surface vivent à des demi-pas différents, l'énergie qui les mêle
oscille à la période du schéma, et sa moyenne se tient au-dessus de l'énergie initiale (purement potentielle, vitesses nulles). La forme
discrète que le schéma conserve ou dissipe n'est pas connue : **A332**. Aucune dissipation n'est mesurable non plus sur 120 s (S310 en
mesurait 0,0935 % par seconde sur un autre montage). L'essai affirme le critère (1) et une garde de non-régression posée après coup (les
moyennes à 1 % l'une de l'autre), pas les critères (2) et (3).
