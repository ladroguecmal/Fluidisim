# La bulle d'une explosion sous-marine — S587 (liste 3.3)

*S587, 2026-10-07, en autonomie (ADR-247 : la physique d'abord).* 3.3 était absent ; ADR-001 range les ondes d'explosion dans W.
Première pièce : la bulle.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s587 -- --nocapture` ; suite du cœur : 762.

## 1. Ce qui est construit

`explosion.rs` : l'énergie de la bulle (une fraction d'auteur de l'énergie de la charge ; TNT à 4,184 MJ/kg), le rayon maximal à
l'équilibre `E_b = (4/3)·π·R³·p` contre la pression du fond, la période du premier battement `T = 2·t_c`, `t_c = C·R·√(ρ/p)`
(l'effondrement de Rayleigh d'une cavité vide, `C = √(3π/2)·Γ(5/6)/Γ(1/3)`). Les lois d'échelle `R ∝ (W/p)^(1/3)` et
`T ∝ W^(1/3)·p^(−5/6)` — la forme de Willis — en découlent.

## 2. Mesuré (références écrites au plan par son script)

| | référence | mesuré |
|---|---|---|
| la constante de Rayleigh | intégration RK4 indépendante de `R·R̈ + 1,5·Ṙ² = −Δp/ρ` : 0,91478 (pas 10⁻⁵), 0,91473 (pas 5·10⁻⁶) → la forme fermée 0,914681 | `RAYLEIGH` = 0,914681 |
| 1 kg à 20 m, fraction 0,4 | `R_max` 1,097268 m, `T` 0,11685897 s | **identiques à 10⁻⁹** |
| 8 kg | `T` ×2 | **×2,000000000000** |
| à 60 m | `T` ×`(p₆₀/p₂₀)^(−5/6)` = 0,494175617660 | **×0,494175617660** |
| masse, profondeur, fraction invalides | refus | tenu |

Critères (écrits avant) : (1)–(3) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Une charge sous l'eau a la bulle et la pulsation que la physique lui donne, aux lois d'échelle de Willis. Manquent pour 3.3 : les
battements suivants (les pertes), la migration de la bulle vers la surface, l'onde de choc, les ondes de surface et la gerbe, les
explosions de surface, l'entrée dans W.
