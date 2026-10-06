# L'assèchement du sol : drainage et évaporation dans V — S535 (liste 5.5)

*S535, 2026-10-06, en autonomie.* Depuis S530–S533 la pluie entre dans le sol ; il ne rendait jamais rien.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s535 -- --nocapture` — trois essais ; suite du cœur : 696.

## 1. La construction (`code/water-core/src/hydro_network.rs`)

- **`Flow::Drainage { area_mm2, conductivity_nm_s, exponent_pm }`** : le drainage gravitaire d'un sol vers le dessous (une nappe, ou
  dehors), `q = K·Sᶜ` par unité d'aire (Brooks–Corey, gradient unitaire) ; `S` le remplissage du nœud rapporté à sa capacité, la lame de
  stockage sa capacité rapportée à l'aire. **Intégré exactement** sur le pas : `S₁ = (S₀^{1−c} + (c − 1)·a·dt)^{1/(1−c)}`, `a = K/lame`.
- **`Flow::Evaporation { area_mm2, rate_nm_s }`** : vers dehors, au taux potentiel d'auteur fois la commande (l'exposition), borné par
  le contenu. La météo viendra à la fin (ADR-197 D5).
- La validation (pas et instantané) et l'empreinte de la base connaissent les deux lois.

## 2. Mesuré

| | mesuré |
|---|---|
| un sol plein de 100 mm sur 1 m², K = 10,9 mm/h, `c` = 4, contre `(1 + 3·a·t)^{−1/3}` | `S` = 0,91000 / 0,61638 / 0,48348 à 1 / 10 / 24 h ; écarts **2·10⁻⁶ / 2·10⁻⁶ / 7·10⁻⁶** ; masse exacte |
| une flaque de 1 cm, 35 nm/s (3,02 mm/jour) | **3 024 ml en un jour** (attendu 3 024) ; à sec exactement au bout de quatre jours, 10 000 ml partis, pas un de plus |
| le cycle : 1 h de pluie à 30 mm/h, rétention, infiltration, ruissellement, drainage, évaporation, 6 h | pluie = rétention + sol + sorties **au millilitre à chaque pas** (29 999 ml de pluie ; 28 636 dans le sol, 1 363 sortis) |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) les lois d'avant au bit | suite 696 | tenu |
| (2) le drainage à 10⁻³ de la forme fermée ; masse exacte | 7·10⁻⁶ au pire | tenu |
| (3) l'évaporation au taux, à 1 ml par jour ; l'arrêt à zéro exact | 3 024 ; exact | tenu |
| (4) le cycle à la masse au millilitre | à chaque pas | tenu |

## 4. Ce qui manque

L'eau du sol de V a désormais son cycle — pluie, rétention, infiltration, ruissellement, drainage, évaporation. Manquent le calcul de
l'exposition depuis les objets posés, une évaporation du sol lui-même (limitée par son humidité), et la météo qui fixe les taux.
