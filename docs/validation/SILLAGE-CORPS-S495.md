# Le sillage d'un objet en marche pousse un corps — S495 (liste 6.2, partielle)

*S495, 2026-10-06, en autonomie.* La suite de [FORCES-W-S494](FORCES-W-S494.md) (les impacts de W derrière la requête du corps) : l'autre
part de W, la pression d'un objet en marche (ADR-103), n'y entrait pas.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s495 -- --nocapture --include-ignored` — deux essais ;
  le sillage est ignoré par défaut (≈ 8,5 min : 4 096 modes, ≈ 150 échantillons par pas). Suite du cœur : 654 essais, 17 ignorés.

## 1. La construction

- **La composition mixte extraite par point** (`code/water-core/src/mixed_water.rs`, `compose_local`) : le corps de la boucle de
  `sample_world_batch` (B, impacts puis pressions, une normalisation, ADR-077), tel quel ; `mixed::sample_local` l'expose au point local
  de l'ancre de B, avec les mêmes contrôles de montage et le même budget de pente.
- **`MixedWater.pressure`** (`rigid_body.rs`) : la pression publiée par son contrôleur à l'instant de la requête entre dans la même
  composition ; publiée à un autre instant, elle est refusée (comptée). L'accélération ne porte que celle des impacts.

## 2. Mesuré

Une source gaussienne (σ = 1 m, 64 × 128 nœuds, coupure 6) en marche à 3 m/s de (−15, 0) à (15, 0) ; une bouée plate (hauteur 0,4 × le
côté) en (−6, 6), à 6σ de la route, 12 s au pas de 2 ms ; le bras de Kelvin y passe vers 9 s.

| cas | mesuré |
|---|---|
| sans pression : `mixed::sample_local` contre `Prepared::sample_local` (S494), 11 560 points × instants | **au bit** |
| 200 Pa, bouée de 0,5 m : surface / pilonnement dû au sillage | 8,4 mm / **9,6 mm** |
| 200 Pa : pilonnement contre l'oscillateur forcé par la surface sous l'empreinte | **0,05 %** de max\|η̄\| |
| 2 Pa (linéaire), bouée de 0,25 m : déplacement contre `∫u dt` | **0,85 %** |
| 200 Pa : déplacement contre `∫u dt` | 12,5 % — la dérive du second ordre (S494), publiée |
| refus | **0** |

## 3. Trois manqués, et ce qu'ils disent

Le critère 3 a manqué trois fois avant d'être diagnostiqué ; les chaînes auraient dû être séparées d'abord.

1. **Bouée à 3 m de la route, recette d'essai 16 × 24 : 40 %.** Le corps égale au µm la double intégrale de `−g∇η` sous lui ; l'écart
   est dans le champ : `du/dt ≠ −g∇η` à 9 m de la source. La recette de `tests_mixed_water` ne reconstruit la gaussienne qu'à
   `r ≲ 24/k_max` ≈ 4 m ; au-delà, une pression repliée pousse l'eau et pas le corps.
2. **Recette 64 × 128 : 215 %.** Au passage de la source, à 3 m (3σ), la **vraie** queue de la gaussienne (`0,011·p₀`) : son gradient
   l'emporte sur la pente du sillage (les deux ∝ `p₀`, rapport ≈ 3/(k·0,01)). La bouée passe à 6σ (route allongée).
3. **À 6σ : 25 % (coupure 3), 19 % (coupure 6).** Champ seul, au point de la bouée : résidu `du/dt + g∇η` de 4·10⁻⁸ — **le champ est
   cohérent** ; `∫u − ∫∫(−g∇η)` croît linéairement : la source naît en marche, l'eau a une vitesse au départ, la bouée partait au
   repos. Lâchée à la vitesse de l'eau composée : **0,85 %**.

**Ce qui n'est pas tranché** : la coupure 3 du sillage de production (S212 : σ = 2 m, `k·σ` = 3) laisse-t-elle une pression parasite
loin de la source ? Les 25 % contre 19 % le suggèrent, mais l'erreur de départ s'y mêlait ; non remesuré.

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) l'extraction au bit ; sans pression, S494 à 10⁻⁶ m | `tests_mixed_water` tenus ; 11 560 points au bit ; S494 aux mêmes chiffres | tenu |
| (2) à 200 Pa, le pilonnement à 3 % ; le sillage fait bouger la bouée de ≥ 30 % de max\|η̄_P\| | 0,05 % ; 114 % | tenu |
| (3) à 2 Pa, le déplacement à 5 % de `∫u dt` (bouée de 0,25 m) | 0,85 % | tenu, après trois manqués (§3) |
| (4) aucun refus | 0 | tenu |

**Avec S494, W entier** — impacts et sillage — est derrière la requête du corps. **6.2 reste partielle** : le courant, la turbulence.
