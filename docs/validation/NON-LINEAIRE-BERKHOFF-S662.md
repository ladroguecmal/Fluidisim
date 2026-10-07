# La non-linéarité sur le haut-fond de Berkhoff — S662 (liste 2.7)

*S662, 2026-10-07, en autonomie, vers la v2.* Après le grand angle (S660), les sections 5 et 7 de Berkhoff restaient loin des mesures. La
cause encore nommée était la non-linéarité de l'expérience.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core pente_douce -- --nocapture` (≈ 2 s).

## 1. Ce qui est construit

- **`nombre_d_onde_non_lineaire`** : la forme composite de Kirby et Dalrymple (1986), bornée en eau peu profonde.
  `ω² = g·k·(1 + f₁·ε²·D)·tanh(k·h + f₂·ε)`, avec `ε = k·a`, `f₁ = tanh⁵(kh)`, `f₂ = (kh/sinh kh)⁴`,
  `D = (cosh 4kh + 8 − 2·tanh² kh)/(8·sinh⁴ kh)`.
- **`propager_non_lineaire`** : le grand angle, où le `k` de chaque point est celui de son amplitude physique `a₀·|A|` à la rangée
  précédente. `p` et `k̄` restent linéaires. `a₀` vaut 0,0232 m, l'amplitude incidente de l'expérience.

## 2. Mesuré

**L'instrument.**

- `k` au sommet du haut-fond (`h` = 0,1336 m, `a` = 2,2·a₀) : 5,37400, comme le script du plan. L'onde raide y va plus vite : `k` baisse de
  10,9 %. Une première borne de 10 %, posée de tête dans le plan, a été franchie ; l'assertion du script l'a arrêtée avant l'écriture.
- **La limite linéaire, critère (1) manqué tel qu'écrit.** À `a₀` = 10⁻⁹ m, l'écart au modèle de S660 vaut 9,76·10⁻⁸, pour 10⁻⁹ exigé.
  La forme composite a un terme d'ordre `ε` (`tanh(kh + f₂ε)`), alors que le plan supposait `ε²`. L'écart est **exactement proportionnel à
  `a₀`** (×100 de 10⁻¹¹ à 10⁻⁹ m) : le modèle tend bien vers le linéaire. C'est cette proportionnalité qui est assertée.

**Berkhoff** (écart quadratique moyen au rapport d'amplitude mesuré ; 5 et 2,5 cm à 0,003 près) :

| section | petits angles (S659) | grand angle (S660) | **non linéaire (S662)** |
|---|---|---|---|
| 2 (x = 3 m) | 0,231 | 0,171 | **0,090** |
| 3 (x = 5 m) | 0,197 | 0,161 | **0,106** |
| 5 (x = 9 m) | 0,419 | 0,342 | **0,091** (−73 %) |
| 7 (y = 0) | 0,288 | 0,233 | **0,101** (−57 %) |
| pic de la section 3 (2,207 mesuré) | 2,460 | 2,425 | **2,055** (−7 %) |

**Critères, écrits avant.**

- **(1) À moitié** (voir plus haut).
- **(2) Le verdict du témoin.** Les sections 5 et 7 baissent de 73 % et 57 % : **c'était la non-linéarité**.
- **(3) Le critère de S659 : tenu.** Les quatre sections sont sous 0,20, le pic à 7 %.

## 3. Ce que cela dit

Le cœur rend maintenant la houle derrière un haut-fond isolé **à un dixième près du rapport d'amplitude mesuré** (Berkhoff, Booy et
Radder 1982), sur les quatre sections. Cela suppose les trois ingrédients ensemble : la diffraction, le grand angle et la dispersion
d'amplitude. Les deux causes de S659 sont départagées : l'angle comptait pour un cinquième de l'écart, la non-linéarité pour l'essentiel.

C'est la base du **modèle côtier cuit par rivage** (S643) : sur une côte réelle, la houle qui sort du large, réfractée et diffractée par
les hauts-fonds, alimente la plage où le relais 2D → 3D la reprend (S650).

Manquent : la diffraction couplée à B (le champ cuit lu par la mer), la marée, la dissipation au déferlement, un spectre de houle au lieu
d'une onde monochromatique.
