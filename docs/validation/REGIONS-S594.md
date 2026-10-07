# Les régions de mer par descripteur — S594 (liste 11.2 ; I-09)

*S594, 2026-10-07, en autonomie (ADR-247).* 11.2 était absent : « nombreuses régions de mer décrites par descripteur, transitions par
paramètres » (I-09 : on interpole des paramètres, jamais des réalisations).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s594 -- --nocapture` ; suite du cœur : 770.

## 1. Ce qui est construit

`regions.rs` : un **descripteur** (Hs, niveau moyen) sur un rectangle ; `parametres_en(x, y)` — chaque région pèse 1 dedans, 0 au-delà,
une transition en `smoothstep` sur une bande centrée sur son bord ; les poids normalisés, **les paramètres mélangés** ; `echelle` — l'échantillon
de B à l'échelle `Hs_local/Hs_réf` (les composantes de B sont les mêmes partout, ADR-004 §2.1) et décalé du niveau moyen local.

## 2. Mesuré (références écrites au plan par son script)

Deux régions, Hs = 1 et 3 m, une bande de 1 000 m ; une mer de B de 32 composantes ; Hs mesuré par `4·σ(η)` sur 2 h, en 9 points.

| | référence | mesuré |
|---|---|---|
| les poids : dedans ; au milieu ; le plus grand saut au mètre | 1 ; Hs = 2 m ; continu | 1 ; **2 m** ; 0,003 m |
| au milieu de la bande, **par les paramètres** | Hs = 2,0 m | **2,0009 m** (l'erreur de mesure, deux fenêtres : 2·10⁻⁴) |
| au milieu, **le témoin** : deux réalisations (graines 42, 43) mélangées ½–½ | 1,5811 m (indépendantes) | **1,3619 m — manqué** |
| le témoin **en moyenne sur 800 couples de graines** (ajouté en route, écrit d'abord aux notes) | `Hs²` = 2,5 | **2,5038** |
| loin de la bande | au bit de B mis à son échelle | au bit |
| hors de toute région, bande nulle, rectangle vide | refus | tenu |

Critères (écrits avant) : (1), (3), (4) — **tenus** ; (2) par les paramètres — **tenu** ; (2) le témoin sur un couple — **manqué**.

**Pourquoi le témoin d'un couple manque.** La formule était juste : chaque mer a bien Hs = 0,2001, et `√((1 + 9 + 2·ρ·3)/4)` avec la
corrélation mesurée ρ = −0,43 redonne 1,361 m. **L'hypothèse d'indépendance était fausse** : deux réalisations aux mêmes composantes
(mêmes fréquences et directions, phases différentes) gardent une corrélation fixe `Σaₖ²·cos Δφₖ/Σaₖ²`, nulle seulement en moyenne sur les
tirages — ici ~10 composantes efficaces, un écart-type de ~0,3. Ce n'est pas un défaut de B. L'ensemble le confirme : 2,5038 pour 2,5.

## 3. Ce que cela dit — et ne dit pas

Le monde peut se découper en régions de mer qui se raccordent par leurs paramètres : la mer garde sa hauteur dans la transition. Mélanger
les champs, au contraire, la calme — de 21 % **en moyenne**, et d'autant que le hasard d'un couple le veut (32 % ici) : I-09 n'est pas une
précaution de style. Manquent pour 11.2 : la période et la direction par région (une pondération spectrale), la marée de chaque région
(la carte cotidale, S578), de nombreuses régions à coût borné (un index spatial), leur placement sur la planète (11.1).
