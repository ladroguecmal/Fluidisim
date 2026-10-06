# La poche porteuse d'un compartiment scellé — S554 (liste 6.6)

*S554, 2026-10-06, en autonomie.* S548 avec le compartiment étanche de S538 : l'air comprimé retient l'eau d'une brèche, et la barge
s'enfonce bien moins qu'un compartiment ouvert ne l'y forcerait.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s554 -- --nocapture` ; suite du cœur : 713.

## 1. Le montage (aucun code neuf)

L'essai de S548 (barge de 20 × 8 × 4 m, 246 t ; compartiment central de 5 × 8 × 4 m, brèche de 0,1 m² au fond ; la mer vue du navire) sous
`step_air` : le compartiment scellé (une poche isotherme, `p·V` = `p_atm·160 m³`, S538), puis ouvert.

## 2. La référence, indépendante (ADR-239 D1)

Boyle et l'équilibre du navire résolus ensemble par bisection, hors de l'essai : `ρ·A·T' = m + ρ·A_c·h` et
`p_atm·D/(D − h) − p_atm = ρg·(T' − h)` — eau 0,4207 m (**16,83 m³**), tirant **1,6052 m**, l'air à 113,2 kPa.

## 3. Mesuré (la trace : l'équilibre dès 300 s, scellé ; 600 s, ouvert)

| | mesuré | référence |
|---|---|---|
| scellé : tirant final | **1,6052 m** | 1,6052 m |
| scellé : eau embarquée | **16,825 m³** | 16,829 m³ (0,02 %) |
| ouvert : tirant final | **1,9995 m** | 2,000 m (S548) |
| masse de V | exacte à chaque pas | — |

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) scellé : tirant à 0,5 %, eau à 1 % | au dix-millième ; 0,02 % | tenu |
| (2) ouvert : 2,000 m à 1 % | 0,025 % | tenu |
| (3) la masse exacte | exacte | tenu |

## 5. Ce que cela dit

Un compartiment étanche percé sous la flottaison n'embarque qu'un dixième de ce qu'embarquerait un compartiment ouvert : l'air y porte la
coque. Avec l'évent à débit limité (S547), le navire coule à la vitesse où son air s'échappe — « un navire coule en dégazant » (ADR-015 §2).
**6.6 avance.** Manquent le ballottement d'un compartiment, la coque retournée réelle (sa poche qui se déplace quand elle bascule), le
chavirement au-delà du pont mouillé, un navire réel et les brèches en jeu.
