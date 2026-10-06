# La poussée du proxy au centre de la part immergée — S500 (liste 6.1)

*S500, 2026-10-06, en autonomie.* La suite de [B6-PROXY-S499](B6-PROXY-S499.md) ; la décision : [ADR-227](../adr/ADR-227-la-poussee-au-centre-de-la-part-immergee.md).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core -- s500 s499 s494 --nocapture` ; la suite du cœur :
  660 essais. L'essai long du sillage : `… s495 -- --include-ignored`.

## 1. La construction

`RigidBody::forces` (`code/water-core/src/rigid_body.rs`) : le bras de levier d'un point en partie immergé est son milieu abaissé de
`(1 − f)·e/2` le long de l'axe du corps. La force est inchangée ; le moment seul change.

## 2. Mesuré

| | mesuré |
|---|---|
| à l'équilibre droit, `\|z_F − (KB − KG)\|`, trois archétypes, `nz` = 1 à 8 | **≤ 2,4·10⁻¹⁴ m** |
| B6 refait : la prédiction `BM·(1 − 1/n²) + z_F` | à 8,6·10⁻⁸ |
| plus petit proxy sans compensation — navire / barque / caisse | **7 × 10 × 1 = 70** / 7 × 7 × 1 = 49 / 7 × 7 × 1 = 49 (S499 : 560 / 196 / 147) |
| leur GM roulis / tangage contre l'analytique | −3,4 / −2,1 % ; −2,2 / −2,1 % ; −2,1 / −2,1 % |

## 3. Ce que la décision a changé ailleurs

- **La bouée de S494** (0,25 × 0,25 × 0,2 m, proxy 4 × 4 × 4, 500 kg/m³) : son GM vrai vaut +2,2 mm ; avec l'inertie discrète de 4 × 4
  points (`BM` −1/16), celui du proxy devient −1,2 mm. L'ancienne poussée, trop haute, la stabilisait à tort ; exacte, elle la fait rouler,
  et son pilonnement s'écartait de l'oscillateur de 3,06 % (critère : 3 %). Les bouées de S494 sont désormais plates (hauteur 0,4 × le côté,
  ADR-227 D3), comme celles de S495.
- **Ce que cela dit de S494** : avec la bouée plate, l'écart horizontal à 1 kJ tombe de 102 % à **35 %** — une part de la « dérive du
  second ordre » publiée en S494 était le roulis de la bouée haute. La loi d'échelle reste du second ordre (×105,6 pour une énergie ×100) ;
  au linéaire, 1,33 % à 0,25 m (S494 : 1,26 %). La preuve de S494 garde ses chiffres d'alors ; celle-ci les corrige.
- Les autres essais du corps (S331–S499) tiennent à leurs tolérances : 660 essais.

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) `z_F = KB − KG` à 10⁻⁹, toute grille | ≤ 2,4·10⁻¹⁴ | tenu |
| (2) la prédiction à 10⁻³ ; sans compensation = avec, 7 × 10 × 1 et 7 × 7 × 1 | 8,6·10⁻⁸ ; 70 / 49 / 49 | tenu |
| (3) la suite du cœur ; les valeurs changées, relevées | 660 ; la bouée de S494 (§3) | tenu |

**6.1 reste partielle** : l'amortissement des autres degrés de liberté.
