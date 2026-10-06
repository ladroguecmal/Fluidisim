# C07 en eau profonde : le sillage de W contre la théorie linéaire, et son angle — S519 (liste 3.2, 13.2)

*S519, 2026-10-06, en autonomie.* Le cas canonique C07 (« sillage profond et peu profond », angle de Kelvin à ±2°,
[CAS-CANONIQUES](CAS-CANONIQUES.md)) n'avait jamais été exécuté : le sillage de W — la source de pression gaussienne mobile de
`wake_source` — n'avait jamais été mesuré contre sa théorie. S517 avait montré qu'un instrument d'angle non éprouvé ne tranche pas.

## Reproduire

- `python outils/reference_sillage.py instrument` — l'instrument de S517 sur la référence, neuf (σ, U), trois grilles (≈ 10 min ; calcul
  `ref-sillage-s519`).
- `python outils/reference_sillage.py instrument_fige` — l'instrument figé sur la référence, trois grilles.
- `cargo run -p water-core --release --example c07_sillage -- 0.5 2.5 calculs/c07_w_s519.bin` (7 s), puis
  `python outils/reference_sillage.py comparer calculs/c07_w_s519.bin`.

## 1. La référence, indépendante de W

La réponse linéaire **exacte en temps** de l'eau profonde à une pression `p₀·exp(−r²/2σ²)` (`p₀ = F/2πσ²`, la charge de `wake_source`,
F = 19 620 N) partie du repos à `t = 0` et menée à `U` constante : par mode,
`η̂(k,T) = −(k/ρ)·p̂(k)·∫₀ᵀ sin ω(T−s)/ω · e^{−i kₓ U s} ds`, en forme fermée, sommée par FFT, le spectre tronqué au disque de la recette
(coupure 6 rad/m). Le transitoire du départ y est, comme dans W. **Convergée** : 256 × 128 m et 512 × 256 m à 25 cm, 512 × 256 m à
12,5 cm donnent les mêmes angles au 0,25° près.

## 2. L'instrument, éprouvé sur la référence avant W

| instrument, sur la référence (T = 24 s, 2–7 λ₀ derrière la source) | lit |
|---|---|
| S517 : le maximum de la moyenne de \|η\| le long des rayons | 16,5–17,75° (σ = 0,5 m) ; 5–10° (σ = 1 m, les ondes transverses dominent) ; 40° (σ = 2 m, les ondes éteintes) — **ne lit pas Kelvin** |
| **le bord d'Airy** : passé ce maximum (au-delà de 12°), l'angle où le profil retombe à Ai(0)/max Ai = 0,663 — près de la ligne des cuspides l'amplitude suit Ai(z), la ligne de Kelvin à z = 0 | par fenêtres d'1 λ₀ (σ = 0,5, U = 2,5) : 22,3 / 21,1 / 20,3 / 19,9 / 19,9° de 2 à 7 λ₀ — il converge vers Kelvin |

**Figé avant W** : le bord d'Airy sur la fenêtre 4–6 λ₀ ; σ = 0,5 m, U = 2,5 m/s (`Fr_σ` = 1,13, λ₀ = 4 m, sillage établi jusqu'à
`U·T/2` = 30 m). Sur la référence : **19,98 / 19,98 / 19,96°** sur les trois grilles.

## 3. W mesuré

Recette 256 × 256 à coupure 6 (le domaine honnête d'[ADR-132](../adr/ADR-132-domaine-d-image-d-un-sillage.md) : 89 m, 26,2 s), trois tronçons de
8 s, le champ à `T` = 24 s sur 26 825 points de 25 cm (7 s d'échantillonnage).

| | mesuré |
|---|---|
| W contre la référence, zone établie (de 2 λ₀ à `U·T/2` − 2 λ₀ derrière la source, coin de 30°, 3 811 points) | écart quadratique relatif **0,33 %** ; pire écart **0,57 %** du maximum (1,357 m contre 1,362) |
| l'instrument figé sur W / sur la référence | **19,98° / 19,98°** (maximum à 17,75° des deux côtés) |

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) sur la référence, l'instrument lit 19,47° à 1° | celui de S517 : non (changé avant W, comme prévu) ; le bord d'Airy : 19,98° | tenu |
| (2) W contre la référence, écart quadratique ≤ 10 % dans la zone établie | 0,33 % | tenu |
| (3) C07 profond : l'instrument sur W à 2° de 19,47° | 19,98° (0,51°) | tenu |

## 5. Ce que cela dit

Le sillage de W est la théorie linéaire de l'eau profonde à 0,3 % près sur la zone établie — transitoire du départ compris —, et son
angle, lu par un instrument éprouvé sur la théorie seule, est celui de Kelvin à 0,5° : l'angle n'est pas codé en dur (A05), il sort de la
dispersion. **C07 profond passe.** Hors de portée : **l'eau peu profonde** de C07 (`arcsin(1/Fr_h)`, la résonance en `1/√|1 − Fr_h²|`) —
la pression de W est en eau profonde (aucune `tanh`) ; elle attend la dispersion en profondeur finie (2.7, K3). L'angle n'est pas lu sur
« le champ d'écume » de l'énoncé, mais sur η, d'où l'écume dérive.
