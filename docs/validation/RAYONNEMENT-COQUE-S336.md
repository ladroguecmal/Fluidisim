# La coque qui cesse de pilonner — S336

2026-09-24. **Porte D**, chemin de la v1 ([ADR-189](../adr/ADR-189-la-v1-d-abord.md)) : δ mesure hors ligne la
masse ajoutée et l'amortissement par rayonnement de la coque de la porte D, et le corps du jeu les reçoit comme
des **constantes de son archétype** ([ADR-008](../adr/ADR-008-flottabilite-et-autorite.md) §2) — jamais comme
une force de δ au pas (I-04). Suite de [PORTE-D-S333](PORTE-D-S333.md).

## Reproduire

- Commit `06d7c380` ou plus récent ; machine de référence, CPU, un fil par passage.
- `cargo test -p water-core --release --offline s336 -- --nocapture` — deux essais ; lignes `S336`.
- `cargo run -p water-core --release --offline --example rayonnement_coque [-- --dx 0.125 --omega 3.5,4.0]` —
  lignes `RAYONNEMENT` ; 25 cm, quatre pulsations, ≈ 4 min ; 12,5 cm, ≈ 30 min par pulsation.
- `cargo run -p water-core --release --offline --example porte_d -- --couvercle-partiel --archetype --images
  viewer/captures/s336` — lignes `PORTE_D`, dont `PORTE_D energie`.
- Valeurs attendues : essai du lâcher, période 2,0056 s et décrément 1,0038 ; bilan de la scène, 352,99 J
  dissipés, 344,88 J fournis à δ.
- Cœur : 498 réussis, 14 ignorés ; intégration 23.

## En une phrase

La coque du jeu, qui pilonnait sans fin pendant que δ emportait l'énergie de ses anneaux, reçoit la masse
ajoutée — 3 200 kg — et l'amortissement — 6 400 N·s/m — que δ lui mesure : elle s'arrête en 3 à 4 s, et
l'énergie qu'elle perd est celle que δ reçoit, **à 2,3 % près**.

## 1. La mesure

Pilonnement imposé `z = Z·sin ωt`, 5 cm, démarré sur une période, coque 4 × 1,6 × 1 m à son tirant, placement
de la porte D ; δ linéaire de 16 × 16 m sur 2 m, couvercle partiel. En régime établi, sur deux périodes, la
force de δ sur la paroi se décompose en `a·sin ωt + b·cos ωt + c` ; en théorie linéaire `F = −A·z̈ − B·ż`, donc
`A = a/(Z·ω²)` et `B = −b/(Z·ω)`.

| ω, rad/s | 25 cm : A, kg | B, N·s/m | 12,5 cm : A, kg | B, N·s/m |
|---|---:|---:|---:|---:|
| 3,0 | 3 112 | 7 104 | — | — |
| 3,5 | 2 808 | 6 315 | 3 234 | 5 866 |
| 4,0 | 2 886 | 5 212 | 3 361 | 4 760 |
| 4,5 | 3 030 | 4 576 | — | — |

La masse ajoutée vaut **0,9 à 1,05 fois la masse de la coque** — l'ordre qu'ADR-008 §2 annonçait. En affinant,
`A` croît de 15 % et `B` décroît de 8 % : la résolution d'A317. **Critère 1, manqué sur le résidu** de
l'ajustement : 5,2–6,8 % à 25 cm, 9,0–9,4 % à 12,5 cm, pour 5 % visés — des sauts discrets quand le fond de la
coque franchit une face, jusqu'à 505 N d'un pas à l'autre, et une dérive lente. `A` et `B` restent déterminés à
~10 % près.

**Constantes de l'archétype** : la pulsation propre cohérente, `ω'² = K/(m + A(ω'))`, vaut ≈ 3,17 rad/s ; **A =
3 200 kg, B = 6 400 N·s/m**, extrapolées des valeurs à 12,5 cm, ± 10 %. Amortissement réduit
`ζ = B/(2√(K(m + A)))` = 0,158 ; période propre 1,98 s au lieu de 1,40.

## 2. Le corps du jeu

- **`radiation_damping`** : une force linéaire, `−B·(V − u)`, en la vitesse du centre de masse relative à l'eau
  qui le porte ; nulle par défaut, S331–S333 inchangés.
- **La masse ajoutée voit l'eau accélérée** : `(m + A)·v̇ = F + A·a_eau`. Celle de S331 agissait sur
  l'accélération absolue — juste en eau calme, fausse sur la houle, où l'eau accélérée pousse la coque de
  `A·a_eau`. B donne cette accélération, analytique, sur les mêmes phases que l'élévation
  (`WaterQuery::acceleration`).

| critère, écrit avant le code | mesure | verdict |
|---|---|---|
| 2. lâcher de 10 cm en eau calme : période et décrément à ± 2 % | 2,0056 s pour 2,0066 ; 1,0038 pour 1,0033 | tenu |
| 2 bis. *(ajouté avant le code)* pilonnement forcé sur la houle avec masse ajoutée, `(K·S − A·ω²)/(K − (m + A)·ω²)` à ± 1 % | 6 s : 1,05201·a pour 1,05191 ; 3 s : 1,15922·a pour 1,16055 — l'accélération absolue dirait 1,113 et 1,547 | tenu |

## 3. La scène de la porte D

Coque lâchée 10 cm au-dessus de son équilibre sur la houle de 6 s, couvercle partiel, constantes de
l'archétype. Le pilonnement relatif s'éteint en 3 à 4 s ; δ culmine à 6,5 cm (13,2 sans amortissement) ;
flancs 11,7 / 11,7 mm ; trajectoire de jeu identique au bit avec ou sans δ (I-04) ; volume au plancher du
transport, 1,75·10⁻⁹ m³.

**Le bilan** (critère 3) : l'énergie que la coque dissipe par son amortissement, `∫B·(V − u)²dt` = **352,99 J** ;
le travail que sa paroi fournit à δ, `−∫F·V_paroi dt` = **344,88 J**. **Rapport 0,977**, pour ± 25 % visés. La
constante que δ a mesurée hors ligne fait perdre au jeu l'énergie que l'eau emporte — le jeu et l'eau ne se
contredisent plus.

Images `viewer/captures/s336`, mêmes pose et habillage que [PORTE-D-S333](PORTE-D-S333.md) §3 : scène / carte
2 s `0x5bcee0e39127e82e` / `0xc58da8c8de0a6f8b` ; 4 s `0xe8d0b8ce46f12bd5` / `0x121fe7d4ecf5d125` ; 6 s
`0xf6b2b7b279a1c2d8` / `0xac468acaedd43afb` ; 8 s `0xc99b2d47947a0766` / `0xab83f1e8df945f07`.

## 4. Ce qui manque

- **Les autres degrés de liberté** : cavalement, embardée, tangage, roulis gardent masse ajoutée et
  amortissement nuls. Le tangage de la scène, forcé par la houle, n'est pas amorti.
- **La dépendance en fréquence** : une constante à `ω'`, juste pour le pilonnement libre ; le mouvement forcé par
  une houle longue voit `A` et `B` d'une autre pulsation, où l'amortissement relatif pèse peu.
- **La résolution** : ± 10 % sur `A` et `B` ; un archétype de production se mesurerait plus fin.
- **Le verdict visuel** de la porte D (R15).
