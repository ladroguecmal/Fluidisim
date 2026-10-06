# La vanne selon son ouverture, les pertes et l'énergie de la pompe — S515 (liste 5.4)

*S515, 2026-10-06, en autonomie.* [ADR-199](../adr/ADR-199-vannes-et-pompes-dans-v.md) (S372) laissait « le `C_d` selon l'ouverture, à
calibrer sur la courbe du constructeur », et « ni puissance ni énergie consommée, ni pertes de charge » ; son §3 disait la voie : une
courbe tabulée à la place de la section, une seconde loi plutôt qu'une modification.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s515 -- --nocapture` — trois essais ; suite du cœur :
  681 essais.

## 1. La construction (`code/water-core/src/hydro_network.rs`)

- **`Flow::Valve { area_mm2, curve_pm }`** : Torricelli, la section multipliée par la **courbe d'ouverture** — la fraction de débit en ‰
  aux ouvertures 0, 100, …, 1 000 ‰ de la commande, interpolée linéairement (la courbe du constructeur : linéaire, à pourcentage égal,
  à ouverture rapide). La courbe ne décroît pas et reste sous 1 000 ; sinon l'arête est refusée.
- **`Flow::PumpLine { …, loss_um_per_l2s2, efficiency_pm }`** : la loi de `Pump`, plus la perte de charge de sa conduite `K·Q²` et son
  rendement. Point de fonctionnement `Q = √((n²·H₀ − Δh)/(H₀/Qmax² + K))`, zéro à sec, à commande nulle, ou sous la hauteur statique.
- **`pump_operating_point`** : débit, hauteur (`Δh + K·Q²`), puissance hydraulique `ρ·g·Q·H` et à l'arbre (divisée par le rendement) —
  l'hôte cumule l'énergie ; le même calcul que le pas.
- Les anciennes lois intactes ; la validation (pas et instantané, une seule fonction) et l'empreinte de la base connaissent les deux
  nouvelles.

## 2. Mesuré

| | mesuré |
|---|---|
| courbe linéaire contre l'orifice commandé (cinq commandes, 200 pas) | fraction à 10⁻¹² ; débits au nanolitre (le quantum de V) |
| courbe à pourcentage égal de rapport 50, tabulée tous les 10 % | 0,141 à mi-ouverture (0,5 en linéaire) ; à 0,0157 de la loi, borne 0,0163 |
| `PumpLine` à perte nulle contre `Pump` (la vidange de S372, 10 000 pas) | au nanolitre |
| 10 l/s, 10 m de barrage, `K` = 0,05 m/(l/s)², `Δh` = 2 m | Q = 7,3030 l/s = la forme fermée ; H = 4,667 m ; 334 W hydrauliques, 478 W à l'arbre (η = 0,7) |
| une pompe noyée remplit une cuve 2 m plus haut : `∫ P_h dt` contre le gain d'énergie potentielle | **0,002 %** sans perte ; **−0,005 %** avec `K` = 0,5 m/(l/s)² (gain + pertes) |
| masse | exacte |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) courbe linéaire = orifice commandé ; pourcentage égal exact aux points et dans la borne | fraction à 10⁻¹², débits au quantum ; dans la borne | tenu (le « 10⁻¹² » des débits se lit au quantum de V, le nanolitre) |
| (2) `PumpLine` à `K` = 0 = `Pump` ; à `K` > 0, la forme fermée à 10⁻⁹ | au nanolitre ; exacte | tenu |
| (3) l'énergie à 0,1 % ; l'arbre `P_h/η` | 0,002 % et 0,005 % ; exact | tenu |
| (4) masse ; suite ; instantané | exacte ; 681 ; l'empreinte les connaît | tenu |

Les chiffres d'ordre de grandeur du plan étaient légèrement faux (l'écart d'interpolation : 1,6 % et non 1,9 % ; le débit avec pertes
à `Δh` = 3 m : 6,83 l/s et non 6,32) ; l'essai les calcule. **5.4 reste partielle** par le seul réseau fermé sous pression (5.8, la v2
d'ADR-010).
