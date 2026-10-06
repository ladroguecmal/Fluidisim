# Les forces de W sur un corps — S494 (liste 6.2, partielle)

*S494, 2026-10-06, en autonomie.* La suite de [PORTE-D-S333](PORTE-D-S333.md) (le corps sur la houle de B) : la liste 6.2 disait
« Manquent W, le courant, la turbulence ».

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s494 -- --nocapture` — deux essais, ≈ 20 s ;
  lignes `S494`. `cargo test … -p water-core` : la suite du cœur, 653 essais.

## 1. La construction

Le corps rigide (`code/water-core/src/rigid_body.rs`) n'interroge l'eau que par `WaterQuery` (I-04 : B + W, jamais δ). Il n'avait
qu'une eau calme et B. **`MixedWater`** lui donne B **et les impacts confirmés de W**, composés par la composition autoritaire
(`composition::compose`, ADR-077) au point local de l'ancre de B — `Prepared::sample_local`, la même composition que
`sample_world_batch`, sans passer par une position du monde. Un point que la composition refuse (hors du domaine d'un impact,
au-delà de sa validité, pente au-delà de `max_slope`) rend B seul, et **le refus est compté** (`refusals`). L'accélération de W —
que seule la masse ajoutée lit — est une différence centrée de 1 ms sur sa vitesse de surface. Le sillage de pression (l'autre
part de W) n'y entre pas encore.

## 2. Mesuré

La bouée : 0,5 × 0,5 × 0,4 m, 500 kg/m³, proxy 4 × 4 × 4, à 5 m d'un impact (λ = 4 m, 256 modes, 32 m sur 12 s), 10 s au pas de
2 ms ; l'oscillateur de référence `z'' = (ρgA/m)(η̄ − z + h/2) − g`, forcé par la surface moyenne sous les seize colonnes du proxy,
intégré en RK4 à 0,1 ms.

| cas | mesuré |
|---|---|
| sans impact, `MixedWater` contre `BackgroundWater`, 20 s de houle de 5 cm | **2,0·10⁻¹⁰ m** |
| 1 kJ sur 2 cm de houle : pilonnement contre l'oscillateur | **0,81 %** de max\|η̄\| (6,3 cm) |
| la part de l'impact : surface / pilonnement | 3,8 cm / **6,9 cm** (`ω/ωₙ` ≈ 0,55 : amplifié) |
| 1 kJ : déplacement horizontal contre `∫u dt` | **102 %** — une dérive vers l'extérieur |
| l'écart horizontal, de 10 J à 1 kJ | **× 97,5** (second ordre : 100 ; premier : 10) |
| 0,1 J (linéaire), écart horizontal / excursion, côté 0,5 m puis 0,25 m | **4,90 %** puis **1,26 %** (÷ 3,90, en L²) |
| refus de la composition | **0** |

**Le manqué, et ce qu'il dit.** L'ordre de grandeur écrit avant ne comptait que l'empreinte (2,5 %). À 1 kJ (`k·a` ≈ 0,1), la bouée
dérive de 7,7 cm vers l'extérieur en dix secondes : le corps suit `x'' = −g·∂η/∂x`, la particule `x'' = −g·∂η/∂x + u·∂u/∂x` ; la
différence est du second ordre et **s'accumule** (`k·a·ω·t` ≈ 1 sur la durée, pas `k·a`). C'est la dérive des vagues : un objet
flottant est poussé dans le sens de propagation de l'anneau. L'écart croît comme l'énergie (× 97,5 pour × 100), donc comme `a²`. Au
régime linéaire, ce qui reste est l'empreinte — l'anneau large bande vu à travers 0,5 m, ses composantes plus courtes que 4 m
comprises : 4,9 %, divisé par 3,9 quand le côté est divisé par 2.

**Une observation, non cherchée.** Sous B seule, le pilonnement s'écarte de l'oscillateur temporel de 3·10⁻⁵ à 1,4·10⁻⁴ m pour des
houles de 2,5 mm à 2 cm, sans proportion — sous 1 % de la houle. S333 avait reçu B par moindres carrés sur le régime forcé, pas
dans le temps.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) sans impact, la trajectoire de `BackgroundWater` à 10⁻⁶ m | 2,0·10⁻¹⁰ m | tenu |
| (2) le pilonnement à 3 % de max\|η̄\| ; l'impact fait bouger la bouée de ≥ 30 % de max\|η̄_W\| | 0,81 % ; 182 % | tenu |
| (3) le déplacement horizontal à 5 % de `∫u dt` | 102 % à 1 kJ | **manqué** ; réécrit après : la dérive du second ordre (× ≥ 30), l'empreinte (÷ ≥ 3, ≤ 2 % à 0,25 m) — tenus |
| (4) aucun refus | 0 | tenu |

**6.2 reste partielle** : le sillage de pression (W des objets en marche), le courant, la turbulence ; et pour un corps en jeu,
l'ensemble des impacts que l'hôte prépare (`Prepared`) à chaque instant.
