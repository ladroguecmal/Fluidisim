# C13 : la remontée des petites bulles — S540 (listes 7.4, 13.2)

*S540, 2026-10-06, en autonomie.* C13 (non exécuté) : « bulles de 0,1, 1 et 5 mm lâchées à 3 m de profondeur ; vitesse terminale à
± 15 % [de SPEC-002 §2] ; trajectoire verticale selon `−g_eff`, pas selon `+Z` ». Les grosses bulles d'APIC existent (S479–S487) ; les
petites — celles des microbulles et de l'aération — n'avaient pas de modèle.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s540 -- --nocapture` — deux essais ; suite du cœur : 702.

## 1. La construction (`code/water-core/src/bulle.rs`)

Une bulle ponctuelle : poussée `(ρ − ρ_air)·V·(−g_eff)`, masse ajoutée ½ρV, traînée de **Tomiyama pour bulles contaminées**,
`C_D = max(24/Re·(1 + 0,15·Re^0,687), (8/3)·Eo/(Eo + 4))` — Schiller–Naumann tant qu'elle reste sphérique, puis le régime où elle se
déforme ; la traînée implicite sur le pas (le temps de relaxation d'une bulle de 0,1 mm est de 0,3 ms). `vitesse_terminale` par
bisection. L'eau douce de SPEC-002 (μ = 10⁻³ Pa·s, σ = 0,072 N/m).

## 2. Mesuré

| diamètre | vitesse terminale | table de SPEC-002 §2 | écart | intégrée (lâchée au repos à 3 m) | 3 m en |
|---|---|---|---|---|---|
| 0,1 mm | **4,98 mm/s** | 5,5 mm/s (Stokes) | −9 % | égale à 10⁻⁶ | ≈ 10 min |
| 1 mm | **0,1124 m/s** | 0,12–0,25 m/s | −6 % sous le bas | égale à 10⁻⁶ | ≈ 27 s |
| 5 mm | **0,2309 m/s** | ≈ 0,25 m/s | −8 % | égale à 10⁻⁶ | 13,0 s |
| sous `g_eff` incliné de 20° (1 mm) | la trajectoire selon `−g_eff` | | 0 rad (sous l'arrondi) | | |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) la vitesse intégrée à 10⁻⁶ de la vitesse terminale | les trois | tenu |
| (2) **C13** : à ± 15 % de la table | −9 %, −6 %, −8 % | tenu |
| (3) selon `−g_eff` à 10⁻⁹ rad | 0 | tenu |

Les vitesses avaient été calculées au plan, avant le code (ADR-232 D2) ; la loi de traînée a été choisie dessus.

## 4. Ce qui manque

**C13 passe.** Manquent les bulles dans un écoulement (la vitesse relative est portée, pas encore branchée sur W ou δ), leur
dissolution et leur fragmentation (le critère de Weber de SPEC-002), et le rendu des microbulles (7.3).
