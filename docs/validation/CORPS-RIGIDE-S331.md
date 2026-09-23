# Le corps rigide du jeu — S331

2026-09-23. **Lot 4**, premier pas ([ADR-178](../adr/ADR-178-strategie-en-trois-systemes-physiques.md) D7),
sur le chemin de la v1 ([ADR-189](../adr/ADR-189-la-v1-d-abord.md)) : le corps que **le jeu** fait flotter,
jugé sur le cas canonique [C10](CAS-CANONIQUES.md) — tirant, période, masse ajoutée.

## Reproduire

- Commit `f64a96fe` ou plus récent ; machine de référence, CPU, un fil.
- `cargo test -p water-core --release --offline s331 -- --nocapture` — cinq essais, 0,2 s ; lignes `S331`.
- Valeurs attendues : tirant 0,243958 m, période 0,990724 s ; rapport des périodes avec masse ajoutée
  1,40775 ; roulis du pavé 0,86178 s.
- Cœur : 487 réussis.

## En une phrase

`water-core` a un corps rigide à six degrés de liberté, poussé par un proxy de points sur l'eau que lui
donne B + W, et il tient C10 **à 0,02 % sur le tirant et 2·10⁻⁶ sur la période**, avec la masse ajoutée
que C10 attendait depuis S21.

## 1. Le modèle

`rigid_body.rs`. Un corps — centre de masse, quaternion, vitesses linéaire et angulaire, moments
principaux d'inertie, masse ajoutée diagonale dans le repère du monde — et un **proxy de flottabilité**
([ADR-008](../adr/ADR-008-flottabilite-et-autorite.md) §2) : des points attachés au corps, chacun portant
un volume, dont l'immersion passe de 0 à 1 sur son épaisseur. Pour un point tenu à plat sous une ligne
d'eau plane, cette rampe est **exacte** : le volume immergé d'un pavé droit est celui de sa géométrie. Une
traînée quadratique facultative par point immergé. Le pas est **symplectique** — vitesses sous les
forces présentes, puis positions et quaternion avec les vitesses nouvelles —, en `f64`.

**D'où vient l'eau — I-04.** Le corps n'interroge l'eau que par `WaterQuery` : hauteur de surface et
vitesse, que fournit B + W — une eau calme pour C10. **δ n'entre pas ici** : il reçoit le corps comme une
paroi mobile (`Volume3::set_solid`, [S330](FACES-COUPEES-3D-S324.md) §9) et ne lui rendra qu'un décalage
visuel borné (ADR-008 §1).

## 2. Ce qui est mesuré

Eau de mer, ρ = 1025 kg/m³ ([ADR-048](../adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md)) ; cube de C10, 0,5 m à 500 kg/m³, proxy de 8 points par
axe, lâché la base à la surface ; pas de 1 ms.

| critère, écrit avant le code | mesure | référence | verdict |
|---|---:|---:|---|
| 1. volume immergé du cube droit | `A·d` à 10⁻¹² | exact | tenu |
| 2. tirant, moyenne sur des périodes entières | **0,243958 m** | `(ρ_c/ρ)·H` = 0,243902 | tenu, 0,02 % (± 1 %) |
| 3. période sans masse ajoutée | **0,990724 s** | `2π√(ρ_c·H/(ρ·g))` = 0,990726 | tenu, 2·10⁻⁶ (± 5 %) |
| 4. rapport des périodes, masse ajoutée du disque équivalent (61,359 kg) | **1,40775** | `√(1 + m_a/m)` = 1,40774 ; C10 : 1,414 ± 15 % | tenu |
| 5. roulis d'un pavé 1 × 1 × 0,3 m lâché à 5° | **0,86178 s** | `2π√(I/(m·g·GM))` = 0,86142, GM 0,4926 m | tenu, 0,04 % (± 5 %) |
| 6. deux trajectoires à six degrés de liberté, traînée comprise | identiques au bit | — | tenu |

**Un fait physique, écrit avant la mesure.** Le cube de C10, à 500 kg/m³ en mer, a une hauteur
métacentrique **négative** — `GM = d/2 + a²/(12d) − a/2` = −4,3 cm : il est instable en roulis, et un
cube réel de cette densité flotte incliné. Lâché parfaitement droit, il pilonne sans tourner, parce que
le proxy est symétrique au bit ; l'arrondi y croîtrait en `e^{3,2·t}` et le ferait basculer au bout d'une
dizaine de secondes. L'essai de pilonnement dure 3,5 s, et la rotation s'éprouve sur un pavé plat, stable.

## 3. Ce qui devient possible, et ce qui manque

**Possible** : un objet qui flotte, pilonne et roule sous l'autorité de B + W, déterministe — le corps du
jeu que la porte D demande. **Manquent** : la mer B + W derrière `WaterQuery` (la houle et ses vitesses
orbitales) ; la masse ajoutée en rotation ; l'amortissement par rayonnement ; le lien au δ — le corps
qui déplace sa paroi dans δ, et le décalage visuel borné que δ lui rend ; un corps qui perce la surface
du δ linéaire.
