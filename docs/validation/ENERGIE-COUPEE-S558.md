# L'énergie du chemin coupé de δ linéaire — S558 (liste 4.18)

*S558, 2026-10-06, en autonomie.* S557 ([ENERGIE-DISCRETE-S557](ENERGIE-DISCRETE-S557.md)) a établi l'invariant du pas linéaire sur fond
plat. Le chemin coupé — un fond quelconque, des mailles en partie solides — est celui de toute scène réelle.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s558 -- --nocapture` ; suite du cœur : 716.

## 1. La dérivation, depuis le code

`divergence_cut` pèse chaque face par son ouverture `a_f` ; `correct_cut` corrige, sans poids, les seules faces ouvertes entre deux mailles
fluides (le couvercle à sa demi-maille) ; une face ouverte sur du solide n'est jamais corrigée et reste au repos. La sommation par parties
de S557 tient dans le produit scalaire **pondéré par les ouvertures** : pour `D_a v = 0`, `Σ a_f·v_f·(G p)_f·dx³ = Σ p_c·w_c·dA`. D'où

**`Q_a = ½ρ·Σ a_f·ω_f·u_f²·dx³ + ½ρg·Σ (η^n − z₀)·(η^{n+1} − z₀)·dA`** (`ω` = ½ au couvercle),

égal à E₀ au départ. Sans le poids `a_f` (le témoin), rien ne le conserve.

## 2. Le montage et la mesure

La cuve de S557 (16 × 8 × 6 mailles de 25 cm, z₀ = 1,5 m) sur un fond en pente de 0,2 à 0,7 m portant une bosse de 0,3 m ; **280 faces
en partie ouvertes** sur 2 576. La bosse de surface de 2 cm, 12 000 pas de 10 ms.

| | mesuré |
|---|---|
| `Q_a`, au pire | **2,4·10⁻⁶ de E₀** (E₀ = 1,57335 J ; final 1,573350 J) |
| la plus forte hausse de `Q_a` d'un pas à l'autre | **2,2·10⁻⁶ de E₀** |
| le témoin sans poids | de +0,003 % à **+1,835 %** de E₀ |

## 3. Les critères, écrits avant

| critère | seuil (plancher ≈ 10⁻⁶ de E₀, S557) | mesuré | |
|---|---|---|---|
| (1) `|Q_a/E₀ − 1|` à chaque pas | 10⁻⁴ | 2,4·10⁻⁶ | tenu |
| (2) la hausse d'un pas à l'autre | 10⁻⁵ de E₀ | 2,2·10⁻⁶ | tenu |
| (3) le montage coupe vraiment | > 0 face partielle | 280 | tenu |

## 4. Ce que cela dit

Le chemin coupé du pas linéaire **ne dissipe pas et ne crée pas d'énergie** : son énergie est celle que pondèrent les ouvertures, conservée
au plancher. Une mesure d'énergie sur fond coupé qui oublie ce poids se trompe de 1,8 % ici — comme le couvercle compté plein en S555. Ne
disent rien : le pas mobile (surface relâchée, couvercle en partie ouvert), le pas couplé (advection), une coque qui bouge (le terme de
paroi travaille), la carte graphique.
