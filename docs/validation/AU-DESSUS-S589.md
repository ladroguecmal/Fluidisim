# La vitesse de B au-dessus du plan moyen — S589 (liste 3.9 ; A286)

*S589, 2026-10-07, en autonomie (ADR-247).* 3.9 était absent. A286 : le pas couplé mobile de δ exige un champ prolongé au-dessus de
`z = 0` **de façon incompressible** ; B refuse `z > 0` (ADR-113) ; Taylor d'ordre un n'est pas incompressible, l'exponentielle `e^{kz}`
amplifie les ondes courtes sous les crêtes des longues.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s589 -- --nocapture` ; suite du cœur : 764.

## 1. Ce qui est construit

`Background::vitesse_au_dessus(xy, z, t)` — le remède qu'A286 proposait : la vitesse horizontale **constante** au-dessus du plan moyen,
la verticale **fermée par la continuité**, `w(z) = w(0) − z·∇ₕ·U(0)` ; incompressible par construction, linéaire en `z` (pour un mode
d'Airy, `w = −a·ω·cos φ·(1 + kz)`, l'ordre un de l'exponentielle). Mêmes phases entières et même ordre de sommation qu'`eval_local`.

## 2. Mesuré (références écrites au plan par son script)

Un mode de 1 m, λ = 50 m, en 16 points.

| | référence | mesuré |
|---|---|---|
| la divergence (différences centrées + Richardson), à `z` = 0,75 m | sous 10⁻³ du témoin (8,8·10⁻⁶ s⁻¹) | **9,9·10⁻⁷ s⁻¹** ; le témoin Taylor : **1,3·10⁻² s⁻¹** |
| `w` à `z` = 0,5 m contre `−a·ω·cos φ·(1 + kz)` | à 10⁻⁵ m/s | **5,8·10⁻⁸ m/s** |
| l'écart à l'exponentielle | ≤ `a·ω·(e^{kz} − 1 − kz)` = 2,238·10⁻³ m/s | **2,24·10⁻³** (la borne, atteinte où `cos φ = ±1`) |
| à `z = 0`, contre la vitesse de surface d'`eval_local` | au bit | **au bit**, 16 points |
| `z < 0`, `z` non fini | refus | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**. **Avant la mesure**, une faute du plan relevée et écrite aux notes : le script avait
imprimé pour (1) un rapport au bruit de 1 (des différences finies f32 au pas de 1 cm) — seuil disqualifié tel qu'appliqué (ADR-236) ; la
procédure a été corrigée (pas de 0,5 et 0,25 m, Richardson), le seuil gardé.

## 3. Ce que cela dit — et ne dit pas

δ dispose au-dessus du plan moyen d'un champ de B incompressible et borné. **A286 n'est pas levée** : reste à **recevoir** ce champ
contre l'oracle S253 dans le pas couplé mobile, et à fournir W (les anneaux) au-dessus du plan de la même manière.
