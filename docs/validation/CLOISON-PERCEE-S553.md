# Deux compartiments et une cloison percée : l'envahissement progressif — S553 (liste 6.6)

*S553, 2026-10-06, en autonomie.* La brèche envahit un compartiment, qui envahit le suivant par une cloison percée.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s553 -- --nocapture` ; suite du cœur : 712.

## 1. Le montage (aucun code neuf : les pièces de S548–S552)

La barge de S548 ; deux compartiments de V de 5 × 8 × 4 m, l'avant (`x` ∈ [0 ; 5]) et l'arrière (`x` ∈ [−5 ; 0]), formes volumiques ; une
brèche de 0,1 m² au fond de l'avant, un trou de 0,05 m² au pied de la cloison (deux arêtes d'orifice, une par sens) ; la mer volumique
recentrée sous la brèche ; la pesanteur du navire ; l'eau de chaque compartiment en son centre mouillé.

## 2. Mesuré

| temps | avant | arrière | centre de la coque |
|---|---|---|---|
| 300 s | 54,97 m³ | 26,89 m³ | −0,011 m |
| 900 s | 98,39 m³ | 84,81 m³ | −0,645 m |
| 1 500 s | 117,26 m³ | 115,60 m³ | −0,955 m |
| 1 800 s → 3 000 s | 119,88 m³ | 119,88 m³ | −0,9985 m |

| | mesuré | attendu |
|---|---|---|
| tirant final au centre | **2,9985 m** | `T·A/(A − A₁ − A₂)` = 3,000 m (0,05 %) |
| assiette finale | 0,0000° | 0 |
| eau par compartiment | **119,88 m³** | 120 m³ (0,1 %) |
| l'arrière en retard sur l'avant (60 s) | oui | — |
| masse de V | exacte à chaque pas | — |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) le tirant à 1 %, l'assiette sous 0,1° | 0,05 % ; 0° | tenu |
| (2) chaque compartiment à 2 % de 120 m³ | 0,1 % | tenu |
| (3) l'arrière en retard ; la masse exacte | oui ; exacte | tenu |

*Une durée trop courte d'abord* : l'essai écrit s'arrêtait à 900 s, en plein envahissement (2,64 m de tirant) ; avant d'accuser le modèle,
la trace a montré que l'équilibre vient vers 1 800 s (ADR-239 D1, relire la mesure d'abord). La durée n'était pas un critère du plan ; elle
est portée à 3 000 s.

## 4. Ce qui manque

L'envahissement progressif par une cloison percée est porté, l'assiette du transitoire comprise. **6.6 avance.** Manquent le ballottement
de l'eau dans un compartiment, les cloisons qui cèdent, la poche d'air porteuse d'un compartiment scellé (S538–S539 couplés au navire), le
chavirement au-delà du pont mouillé, un navire réel, les brèches en jeu.
