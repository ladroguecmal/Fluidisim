# Une houle périodique sur une plage : la remontée de Keller & Keller — S625 (liste 4.14)

*S625, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* Un manque de 4.14 depuis S613 : « une houle sur une plage réelle ».
Une houle longue périodique entre par le bord caractéristique (S622) et monte et descend une pente, cycle après cycle ; sa remontée est
jugée contre la théorie.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s625 -- --nocapture` (≈ 75 s) ; suite du cœur : 812 essais
  listés.

## 1. Le montage et sa référence

Aucun code nouveau dans le cœur : `SaintVenant2D` d'ordre deux (S620) et `pas_avec_bord` (S622) nourri de `η = A·sin ωt`, `u = √(g/d)·η` ;
300 m de fond plat à 10 m, puis une pente 1:19,85 ; huit périodes ; la remontée d'un cycle établi : le maximum, sur les deux dernières, de la
surface `z + h` de la maille mouillée la plus haute. **Keller & Keller (1964)** : `R = 2A/√(J₀(2kL)² + J₁(2kL)²)` — exacte aussi pour la
remontée non linéaire d'une houle non déferlante (Carrier & Greenspan 1958).

## 2. Mesuré (références calculées au plan par `s625_ref.py`, numpy)

`A` = 5 cm, `T` = 60 s (λ = 594 m), `2kL` = 4,1974 : **R = 0,249190 m** (`R/A` = 4,98) ; non déferlante (`R·ω²/(g·tan²β)` = 0,11).

| maille | remontée | écart à Keller & Keller | écart à numpy |
|---|---|---|---|
| 1 m | 0,250238 m | +0,42 % | 1,7·10⁻¹³ m |
| ½ m | 0,249061 m | −0,05 % | 4,0·10⁻¹³ m |
| ¼ m | 0,249311 m | +0,05 % | 7,3·10⁻¹⁴ m |

`h ≥ 0` partout. Critères (écrits avant) : (1)–(3) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La plage monte et descend avec la houle, et sa remontée tombe sur la théorie à un demi-millième près dès la maille de ½ m : le mouillage
et le séchage de S613, l'ordre deux de S620 et le bord de S622 composent une plage juste. Le bord absorbe la houle réfléchie par la pente,
sinon le cycle ne s'établirait pas.

Manquent : la houle déferlante (le rouleau, au-delà du paramètre de Carrier–Greenspan), la dispersion (une houle courte), la houle
oblique, le rouleau 3D.
