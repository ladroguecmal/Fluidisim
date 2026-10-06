# L'impulsion d'entrée dans l'eau — S512 (liste 6.8, C20, validée)

*S512, 2026-10-06, en autonomie.* [ADR-023](../adr/ADR-023-mecanismes-restes-a-specifier.md) §2 : une entrée dans l'eau dure de 10 à
70 ms, moins qu'un tick ; la flottabilité échantillonnée la rate ou la double selon la phase. Le terme d'impact publie une **impulsion de
masse ajoutée**, intégrée analytiquement — pas une pression.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s512 -- --nocapture` — trois essais ; suite du cœur :
  672 essais.

## 1. La construction (`code/water-core/src/rigid_body.rs`)

- **`SlamArchetype`** (relèvement `β`, demi-largeur `b`, longueur `L`, seuil 2 m/s) — une propriété d'archétype (ADR-023 §2.5) ;
  `RigidBody::slam`, `None` par défaut.
- **Le pas** : quand la quille (le point le plus bas du proxy) est au-dessus de l'eau, le corps est en chute libre — résolue **exactement**
  sur le pas (le pas symplectique descend plus vite que la chute libre et ferait manquer, selon la phase, l'instant du passage) ; si la
  quille passe sous la surface dans le pas, à une vitesse relative normale au-delà du seuil, le corps avance jusqu'à cet instant `τ`, reçoit
  l'impulsion, puis finit le pas.
- **L'impulsion** : la quantité de mouvement du corps et de l'eau entraînée (`m_a = ½πρb²L`) conservée — `v' = m·v/(m + m_a)`,
  `J = m·(v' − v)` ; elle tend vers `Δm_a·v_rel` d'ADR-023 quand `m_a ≪ m`, et ne dépasse jamais ce que la masse en jeu permet.
- **`SlamEvent`** publié : l'instant dans le pas, `J`, `v_rel`, `m_a`, la durée de Wagner `2b·tanβ/(π|v|)` — autoritaire (ADR-023 §2.4).

## 2. Mesuré (chute de 1,25 m : `v_rel` = 4,952 m/s = `√(2gh)`, exacte)

| cas | mesuré |
|---|---|
| corps lourd (1 m³ à 20 t/m³, `b` = 0,5 m, `L` = 1 m : `m_a/m` = 2 %) | `J` = 1 954 N·s, **0,980 × `Δm_a·v_rel`** ; bilan corps + eau 1,5·10⁻¹¹ kg·m/s (sur 99 000) ; Wagner 11 ms |
| coque de la porte D lâchée à plat (`b` = 0,8 m, `L` = 4 m, `β` = 10° : `m_a/m` = 1,29) | `J` = 8 921 N·s = `m_a·v/(1 + m_a/m)` ; la formule d'ADR-023 dirait 20 412 (×2,3) ; bilan 3,6·10⁻¹² ; Wagner 18 ms |
| la coque à 30 Hz, le premier pas décalé de vingt phases | `J` à **2·10⁻¹⁶** près, l'instant d'impact à **1·10⁻¹⁶ s** |
| témoin : l'impulsion au premier tick où la quille est sous l'eau | dispersion **5,3 %** sur les mêmes vingt phases |
| la coque lâchée de 0,1 m (1,4 m/s à l'entrée) | aucun impact à l'entrée |

**Le second impact.** Sans amortissement, la coque lâchée de 0,1 m rebondit hors de l'eau et y rentre trois secondes plus tard à 2,01 m/s :
un vrai second impact, au-dessus du seuil.

## 3. Les critères, écrits avant (C20)

| critère | | |
|---|---|---|
| (1) bilan à 10⁻¹² ; `J` à 5 % de `Δm_a·v_rel` quand `m_a/m` < 5 % | 1,5·10⁻¹¹ relatif 1,5·10⁻¹⁶ ; 2,0 % | tenu |
| (2) même `J` et même instant sur vingt phases à 10⁻⁹ ; le témoin au tick disperse | 2·10⁻¹⁶ ; témoin 5,3 % | tenu (après un premier manqué : les pas en l'air, symplectiques, faisaient manquer le passage — la chute libre exacte) |
| (3) sous le seuil, aucune impulsion | aucune à l'entrée | tenu |
| (4) les essais du corps inchangés | 672 | tenu |

La seconde assertion de C20 (la durée linéaire en `tan β` et en `1/v`) est la formule de Wagner que l'événement publie : elle ne mesure ici
aucun mécanisme et n'est pas jugée. **6.8 validée** ; hors du point : la gerbe que l'impact lance (4.16), le son. La liste : **10 points
validés sur 120**.
