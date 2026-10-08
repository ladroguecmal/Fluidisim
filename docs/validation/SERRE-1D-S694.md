# Le porteur dispersif : Serre–Green–Naghdi 1D — S694 (liste 4.14 ; A234)

*S694, 2026-10-08, en autonomie.* S693 a montré que Saint-Venant, sans dispersion, raidit l'onde du large : la vague se retourne trop tôt.
Les équations de Serre, Green et Naghdi (SGN) gardent la non-linéarité entière et ajoutent la dispersion faible.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s694 -- --nocapture` (≈ 15 s).

## Ce qui a été fait

Le module `serre_1d.rs` (catégorie P), en 1D, sur fond plat et domaine périodique. Le pas :

- Saint-Venant en volumes finis (MUSCL minmod, Rusanov, Heun) ;
- plus la correction dispersive de Bonneton et al. (2011). On résout `h·A − ⅓(h³·A_x)_x = −⅓(h³(2u_x² + g·h_xx))_x` (tridiagonal
  cyclique), puis `(hu)_t` reçoit `h·A`.

Linéarisé, le schéma rend la dispersion de Serre, `ω² = g·d·k²/(1 + (kd)²/3)` (la réduction est écrite au plan).

## Mesuré

**L'instrument** : l'onde solitaire exacte de SGN, `η = a·sech²(κ(x − ct))`, `c = √(g(d+a))`, `κ = √(3a)/(2d√(d+a))`, après 40 `d`.

| | `a/d` = 0,1 | `a/d` = 0,3 | critère |
|---|---|---|---|
| écart de forme, `d/20` → `d/40` (rapporté à `a`) | 0,133 → **0,038 %** (÷ 3,5) | 0,345 → **0,086 %** (÷ 4,0) | < 2 %, ÷ 3 au moins |
| célérité de la crête | +0,073 → **+0,027 %** | +0,089 → **+0,038 %** | < 0,2 % |
| masse | 7·10⁻¹⁶, 2·10⁻¹⁵ | 1·10⁻¹⁵, 2·10⁻¹⁵ | au bit |
| **témoin : Saint-Venant** (sans le terme) | forme **59 %**, célérité +8,5 % | forme **61 %**, célérité +10,4 % | > 10 % |

Les quatre critères tiennent. L'écart converge à l'ordre deux. Le témoin dit ce que S693 soupçonnait : sur 40 profondeurs de parcours,
Saint-Venant déforme l'onde de 60 %.

**Une valeur de tête dans le plan.** La largeur `1/κ` pour `a/d` = 0,3 y était écrite « 2,08 m » ; le script calcule 2,40 m. La formule
de l'onde, écrite de mémoire, est confirmée par l'essai : un état initial faux ne resterait pas stationnaire.

## Ce que cela dit

- Le large a maintenant un porteur qui propage une onde non linéaire comme APIC, avec la dispersion.
- Il est aussi la référence que A234 n'avait pas : la non-linéarité en eau peu profonde, contre laquelle B se jugera.

**Ne fait pas** : le fond variable, le mouillage et le séchage, la 2D, le déferlement. La suite (S695) : le relais au large de S693 nourri
par SGN au lieu de Saint-Venant.
