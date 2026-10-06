# L'énergie que le pas linéaire de δ conserve — S557 (A332 ; listes 4.18, 13.2 ; C09)

*S557, 2026-10-06, en autonomie.* S555 ([C09-ENERGIE-S555](C09-ENERGIE-S555.md)) a trouvé l'énergie naturelle de δ linéaire oscillante,
sa moyenne 8,4 % au-dessus de E₀, et ouvert A332 : quelle énergie le schéma conserve-t-il ?

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s557 -- --nocapture` ; suite du cœur : 715.

## 1. La dérivation, depuis le code, avant la mesure

`step_surface_linear` est un pas **avant-arrière** :

1. `u^{n+1} = u^n − (dt/ρ)·G p`, avec `D u^{n+1} = 0` et la pression imposée au couvercle `p_c = ρg·(η^n − z₀)` (la condition de
   Dirichlet par un fantôme, le gradient de la face du couvercle pris sur `dx/2`) ;
2. `η^{n+1} = η^n + dt·w_c^{n+1}` : la somme des flux de colonne d'un champ sans divergence est la vitesse au couvercle, le fond étant clos.

Pour un champ `v` sans divergence, la sommation par parties donne `⟨v, G p⟩ = Σ p_c·w_c·dA` **à condition que la face du couvercle pèse
une demi-maille** dans le produit scalaire. Avec `K(u) = ½ρ·Σ ω_f·u_f²·dx³` (`ω` = ½ au couvercle, 1 ailleurs) :

`K^{n+2} − K^{n+1} = ½ρ·⟨u^{n+2} + u^{n+1}, u^{n+2} − u^{n+1}⟩ = −½ρg·Σ (η^{n+1} − z₀)·(η^{n+2} − η^n)·dA`,

d'où l'invariant exact

**`Q = K(u^{n+1}) + ½ρg·Σ (η^n − z₀)·(η^{n+1} − z₀)·dA`**

(l'énergie potentielle **mixte**, entre les deux surfaces qui encadrent la vitesse). Au départ (`u^0 = 0`),
`K(u^1) = −½ρg·dt·Σ (η^0 − z₀)·w_c^1·dA`, donc `Q = E₀` exactement. Le pas linéaire, sans éponge, **ne dissipe pas**. L'énergie naturelle
`K^{n+1} + ½ρg·Σ (η^{n+1} − z₀)²·dA` vaut `Q + ½ρg·dt·Σ (η^{n+1} − z₀)·w_c^{n+1}·dA` : elle oscille autour de `Q`.

La formule est éprouvée à part (ADR-239 D1) sur l'oscillateur de même structure, `p_{n+1} = p_n − h·q_n`, `q_{n+1} = q_n + h·p_{n+1}`, qui
conserve `p_{n+1}² + q_n·q_{n+1}`.

## 2. Mesuré — la cuve close de S555 (16 × 8 × 6 mailles, bosse de 2 cm, 12 000 pas de 10 ms)

| | mesuré |
|---|---|
| l'oscillateur f64, 10⁵ pas | `p_{n+1}² + q_n·q_{n+1}` constant à **2·10⁻¹⁴** |
| `Q`, au pire sur 12 000 pas | **2,1·10⁻⁵ de E₀** (E₀ = 1,57335 J ; `Q` final 1,57332 J) |
| la plus forte hausse de `Q` d'un pas à l'autre | **4,5·10⁻⁶ de E₀** |
| énergie naturelle, demi-poids du couvercle | de −1,38 % à +1,43 % de E₀ — elle oscille **autour de E₀** |
| énergie naturelle, poids plein du couvercle (l'instrument de S555) | de −0,11 % à **+16,72 %** |

## 3. Les critères, écrits avant

| critère | seuil (plancher ≈ 10⁻⁶ de E₀ : l'ulp de `η` près de z₀ sur une bosse de 2 cm) | mesuré | |
|---|---|---|---|
| (1) `|Q/E₀ − 1|` à chaque pas | 10⁻⁴ | 2,1·10⁻⁵ | tenu |
| (2) la hausse de `Q` d'un pas à l'autre | 10⁻⁵ de E₀ | 4,5·10⁻⁶ | tenu |
| (3) l'oscillateur | 10⁻¹² | 2·10⁻¹⁴ | tenu |

## 4. Ce que cela dit

- **A332 est levée** : l'énergie discrète du pas linéaire est `Q`, conservée au plancher d'arrondi. Sur l'énergie du schéma, **C09 passe** :
  `dQ/dt ≤ 0` au plancher près (la plus forte hausse, 4,5·10⁻⁶ de E₀, est cinq fois le plancher), et `Q` finit sous E₀ (−1,9·10⁻⁵ en
  120 s — l'arrondi et le résidu de la projection).
- **Le +8,4 % de S555 était une erreur de l'instrument**, pas du schéma : la vitesse au couvercle comptée pour une maille pleine alors que
  sa face n'en porte qu'une demie. Avec le bon poids, l'énergie naturelle oscille de ± 1,4 % autour de E₀ — l'écart `O(ω·dt)` attendu
  d'un schéma décalé. S555 a écrit « sa moyenne se tient au-dessus de E₀ » : c'était l'instrument.
- Le pas linéaire n'a pas de dissipation numérique : les 0,0935 % par seconde de S310 viennent d'un autre montage (éponge ou pas couplé), à
  ne pas confondre avec le cœur linéaire.
- Ne disent rien : l'énergie du pas couplé (advection), celle du chemin coupé (ouvertures partielles : le poids de chaque face change), la
  carte graphique.
