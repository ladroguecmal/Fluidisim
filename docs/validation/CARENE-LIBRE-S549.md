# La carène libre : l'eau d'un compartiment fait gîter la coque — S549 (liste 6.6)

*S549, 2026-10-06, en autonomie.* L'eau d'un compartiment à demi plein garde sa surface horizontale quand la coque gîte ; son centre se
déplace vers le bord bas, et la stabilité perd `i/∇` (`GM → GM − i/∇`, `i` le moment d'inertie de la surface libre).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s549 -- --nocapture` ; suite du cœur : 709.

## 1. La construction

- **`VolumeShape::centroid_below_um(plan)`** (`hydro_geometry.rs`) : le centre de la part mouillée d'une forme de V — chaque tétraèdre
  découpé par le plan (un sommet mouillé : le petit tétraèdre ; deux : le coin en trois tétraèdres ; trois : le tétraèdre moins le petit du
  sommet sec), sans allocation.
- **`RigidBody::loads`** (`rigid_body.rs`) : des charges ponctuelles (un point du corps, une force du monde), ajoutées aux forces et au
  moment ; vide par défaut.

## 2. Mesuré

| | mesuré | attendu |
|---|---|---|
| le centre mouillé d'une boîte de 5 × 8 × 4 m à 1 m d'eau, pesanteur inclinée de 1 / 3 / 5° | 93 093,682 / 279 508,175 / 466 606,202 µm | `b²·tan θ/(12 h)`, au µm près (10⁻⁹) |
| la barge de S548, son compartiment central à 1 m d'eau, centre de masse décalé de 10 cm : gîte, eau figée | 2,736° | — |
| la même, eau libre (son centre calculé à chaque pas sous la pesanteur vue du navire) | 4,283° | — |
| `GM` figé mesuré (`Δy/tan θ`) | 2,093 m | 2,137 m (la théorie de la barge : 2 %, l'erreur du proxy, S499) |
| rapport des tangentes libre / figée | **1,5672** | `GM_f/(GM_f − i/∇)` = 1,5725 (`i/∇` = 0,762 m) : écart **0,34 %** |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) le centre mouillé à 10⁻⁹ | au µm | tenu |
| (2) sans charges, la suite au bit | 709 | tenu |
| (3) le rapport à 3 % | 0,34 % | tenu |

**Une erreur d'analyse, et un diagnostic faux, avant de conclure.** Le plan écrivait `tan θ = m_h·Δy/(m·GM)` ; or le proxy décalé de Δy
déplace *toute* la poussée, qui porte la coque et l'eau : le moment inclinant vaut `m·g·Δy`, et `GM_f = Δy/tan θ`. Avec la formule fausse,
le critère semblait manqué de 10 %. J'ai d'abord attribué l'écart au proxy (la flottaison sur une limite de couche) et changé le montage
— l'écart est resté, la cause n'était pas là. Le montage déclaré est revenu ; seule la formule d'analyse a été corrigée.

## 4. Ce qui manque

La carène libre est portée : un compartiment à demi plein fait gîter la coque de 57 % de plus. **6.6 avance.** Manquent la dynamique de
l'eau qui court (le ballottement du compartiment, son retard sur la gîte), un compartiment décentré et l'assiette, la stabilité aux grands
angles jusqu'au chavirement, et les compartiments que l'eau envahit pendant que la coque gîte (S548 et S549 ensemble).
