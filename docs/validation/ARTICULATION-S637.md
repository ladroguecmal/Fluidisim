# V qui déclenche δ et le tient par la masse — S637 (liste 5.10 ; ADR-025 §3, C21)

*S637, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* Un manque de 5.10 : « V qui déclenche δ » ; et la mécanique d'ADR-025
§3 — l'amorçage au niveau du nœud, la relaxation de la masse de δ vers celle du nœud.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s637 -- --nocapture` ; suite du cœur : 821 essais listés.

## 1. Ce qui est construit

Un module `articulation.rs` : `declenche` (le seuil de S564 en hauteur de surface, contre le dernier état publié) ; `amorcer` (δ —
`SaintVenant2D` — né au niveau du nœud, au repos) ; `forcer` (la correction `(M_nœud − M_δ)·dt_V/τ`, une couche uniforme sur les mailles
mouillées). Aucune fonction n'écrit dans V (C21).

## 2. Mesuré (références calculées au plan)

La piscine de S564 (50 m², 1 m d'eau), un robinet de l'hôte de 2 L par pas de V (10 Hz), le seuil de 500 µm, τ = 1 s.

| | référence | mesuré |
|---|---|---|
| le déclenchement | au pas 13 (40 µm par pas) | au pas 13 |
| l'amorçage | δ au niveau du nœud, 1,00052 m | à 10⁻¹² m, uniforme |
| le retard de masse `V − M_δ`, pas à pas sur 300 pas | la récurrence discrète ; établi à `Q·(τ − dt)` = 0,018 m³ | à 3,8·10⁻¹³ m³ de la récurrence ; 0,0180000000002 m³ |
| la surface de δ sous le forçage | plate | vitesse 4·10⁻¹⁶ m/s |
| V | ne reçoit que le robinet | au ml (assemblage, C21) |
| refus : `dt_V`, `τ` | | tenu |

Critères (écrits avant) : (1)–(6) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

V décide quand δ doit naître — dès que sa surface a monté d'un demi-millimètre —, δ naît à son niveau et suit sa masse avec un retard
établi exactement prévu : 18 L, soit 360 µm de niveau sous un débit de 20 L/s, sous le perceptible (ADR-025 §3.2). La masse ne remonte
jamais de δ vers V.

Manquent : un bassin quelconque (l'amorçage par `shape_lut` et `g_eff`), la dérive d'un solveur réel mesurée contre le critère d'admission
d'ADR-025 §3.3, la destruction de δ quand V se calme, la piscine 3D de S375 branchée sur ce déclenchement, la porte E.
