# L'impact prévu — la prédiction balistique consommée par le domaine épars — S405

2026-09-27. **Liste 9.3**, *Prédiction d'objets balistiques : point, vitesse, orientation, région utile* — absent jusqu'ici. La
source (§8.2) : pour un objet dont la trajectoire est devenue déterministe — un véhicule qui quitte un pont —, estimer le point
d'impact, la vitesse, l'orientation, la rotation et la région de simulation utile ; « le temps de vol devient alors une fenêtre de
calcul permettant de préparer le domaine d'eau ». [ADR-013](../adr/ADR-013-prediction-activation-precalcul.md) §2 en donne les
paliers. Le consommateur : le domaine épars en mer de [MER-EPARS-S404](MER-EPARS-S404.md), sur le chemin de la scène de C10 — le
saut. Session cloud, sans carte graphique.

## Reproduire

- Commit `ea9c123c` ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s405 -- --nocapture` — six essais, < 1 s ;
  lignes `S405 …` : §2.
- `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_impact_prevu -- <prevu|temoin>`,
  `IMPACT_PHASE_S=<0|0.1|0.2|0.3|0.4>` — ligne `IMPACT_S405`, ≈ 3 min par cas : §3.

## En une phrase

Le cœur prédit désormais l'impact d'un objet balistique — instant exact à 10⁻¹² s dans le vide, d'ordre 4 sous traînée, rotation
libre au 10⁻¹³, contact avec la **houle** telle qu'elle sera (l'ignorer coûte 2 ms et 2 cm), région utile couvrant une traînée
connue à ±30 %, palier d'ADR-013 §2 — et, consommée par le domaine épars en mer, cette prédiction prépare la région d'impact
**1,11 s** avant l'impact à toutes les phases de revue, quand le suivi de l'objet sans prédiction la prépare de 0,01 à 0,41 s
avant et laisse la source d'entrée **hors de l'ensemble** à une phase sur cinq ; le prix : 10 à 15 % de mailles.

## 1. Le prédicteur

`code/water-core/src/ballistic.rs`, fonctions pures, sans allocation :

- **`Ballistic`** : position, vitesse, orientation (quaternion), vitesse angulaire dans le repère du corps, moments d'inertie
  principaux, traînée quadratique `k = ρ_air·C_d·A/(2m)`, rayon de la sphère englobante.
- **`predict`** : Runge-Kutta d'ordre 4 sur la translation (`dv/dt = g − k·|v|·v`) et la rotation libre (équations d'Euler, sans
  couple ; orientation renormalisée) ; le **contact** est celui du point le plus bas de la sphère englobante avec
  `surface(p, t)`, une fonction de l'hôte — la houle de B, un plan — ; l'instant s'affine par bisection sur un pas partant du début
  du pas, à 10⁻¹² s. Rend l'instant, le centre, la vitesse, l'orientation, la rotation au contact ; `None` hors de l'horizon, au
  contact, ou sur des paramètres invalides.
- **`predict_region`** : la région utile — la sphère englobante, élargie de la plus grande distance horizontale aux impacts des
  deux bornes d'une traînée mal connue.
- **`tier`** : le palier d'ADR-013 §2 — T1 sous 0,3 s ; T2 sous 8 s si `½·a_max·t²` tient dans le domaine ; T3 sous 8 s ; T4.
  Un objet balistique (`a_max` = 0) est en T2 dès qu'il est à moins de 8 s de l'eau.
- **`advance`** : le même intégrateur sans contact — ce que la physique de l'hôte ferait de l'objet, pour un banc.

## 2. Critères écrits avant — le prédicteur

| critère | résultat |
|---|---|
| **1** — vide, surface plane : instant et point exacts à 10⁻⁹ s et 10⁻⁸ m ; `None` hors horizon, au contact, paramètres invalides | **tenu** : **5,4·10⁻¹³ s**, 6,5·10⁻¹² m |
| **2** — chute verticale avec traînée, contre la solution analytique : ordre 4 sur trois pas (rapport 16 à 20 % près) ; ≤ 10⁻⁶ s à 10 ms | **tenu** : écarts 2,51·10⁻⁶ / 1,74·10⁻⁷ / 1,12·10⁻⁸ s à 0,2 / 0,1 / 0,05 s, rapports **14,4** et **15,6** ; 1,9·10⁻¹¹ s à 10 ms |
| **3** — rotation libre : toupie symétrique contre sa précession analytique, rotation autour d'un axe principal contre l'orientation exacte, ≤ 10⁻⁸ à 1 ms sur 2 s | **tenu** : 7,3·10⁻¹⁴ rad/s ; 4,1·10⁻¹⁴ |
| **4** — surface mouvante (houle de 5 cm, λ = 16 m) : l'instant à ≤ 1 ms d'une référence à 0,1 ms ; publié, l'erreur d'une prédiction qui ignorerait la houle | **tenu** : 2·10⁻¹² s ; **ignorer la houle : +2,10 ms, +2,2 cm** |
| **5** — paliers d'ADR-013 §2 aux frontières | **tenu** — dont l'avion de chasse d'ADR-013 : T2 jusqu'à √2 s |
| **6** — traînée connue à ±30 % : toute traînée dans les bornes tombe dans la région | **tenu** : région 0,753 m (sphère 0,25) ; la plus éloignée de 21 traînées à 0,503 m du nominal |

## 3. Le consommateur — le domaine épars en mer, préparé avant l'impact

`delta3d_impact_prevu` : la mer de S404 — fenêtre de 32 × 16 m à 25 cm, 2 m d'eau, houle de S369 oblique, pas couplé relatif de
20 ms, éponge de 2 m au bord de la boîte et de l'ensemble. **L'objet** : une sphère de 0,25 m lancée de (3 ; 8 ; 6) m à 12 m/s,
traînée vraie 0,012 m⁻¹ ; le prédicteur ne connaît que 0,01 ± 30 %. Impact vrai (pas de 1 ms, sur la houle) à **1,1135 s** en
(15,334 ; 8,000) m, à (10,1 ; 0 ; −10,0) m/s. **L'entrée** : le volume de la calotte immergée ajouté à la surface à chaque pas, en
gaussienne de 0,5 m tronquée à 1,5 m, l'objet poursuivant sa course jusqu'à être immergé — un modèle au premier ordre, pas la
physique de l'entrée (C4, C10). **L'ensemble** : `Follow` de S401 (4 m, 1 mm, 0,25 s), **revu toutes les 0,5 s**. `prevu` : à
chaque revue, l'impact prédit depuis l'état courant sur la houle de B ; en T2 ou T1, sa région est suivie. `temoin` : l'objet
suivi là où il est. `IMPACT_PHASE_S` décale les revues : ce qui sépare la dernière revue de l'impact en dépend.

**Critère 7, écrit avant** : avec la prédiction, la région d'impact dans l'ensemble ≥ 0,5 s avant l'impact, la source toujours
dedans, l'écart au domaine entier ≤ 3 mm (prédiction ≤ 1 mm) ; le témoin voit sa source refusée à l'impact (prédiction).

| phase des revues | dernière revue avant l'impact | `prevu` : région prête avant l'impact / source dehors / écart | `temoin` : région prête / source dehors / écart |
|---:|---:|---|---|
| 0 | 0,11 s | **1,11 s** / jamais / 0,595 mm | 0,11 s / jamais / 0,595 mm |
| 0,1 | 0,01 s | 1,11 s / jamais / 0,594 mm | 0,51 s / jamais / 0,594 mm |
| 0,2 | 0,41 s | 1,11 s / jamais / 0,594 mm | 0,41 s / **à 1,10 s** / — |
| 0,3 | 0,31 s | 1,11 s / jamais / 0,594 mm | 0,31 s / jamais / 0,604 mm |
| 0,4 | 0,21 s | 1,11 s / jamais / 0,595 mm | 0,21 s / jamais / 0,926 mm |

« Région prête » : le disque de 0,5 m autour du point d'impact vrai entièrement dans l'ensemble ; la source, elle, s'étend sur
1,5 m — c'est elle que `add_column_volume` refuse hors de l'ensemble.

**Les prédictions revue après revue** (phase 0,1) : T2 à 0 s — point à **0,099 m** du vrai, région 0,401 m, instant −5,6 ms ; T2 à
0,1 s — 0,075 / 0,363 m ; T2 à 0,6 s — 0,010 / 0,265 m, −2,4 ms ; T1 à 1,1 s — 0,000 m. L'erreur, due à la traînée mal connue,
décroît avec le temps de vol restant et reste dans la région.

**Ce que les chiffres disent.**

- **Critère 7** : la part de la prédiction est **tenue** à toutes les phases — la région prête 1,11 s avant (dès le lancer), la
  source jamais dehors, 0,59 mm du domaine entier. La prédiction « témoin refusé » est **manquée** à la phase prévue (0) et
  vérifiée à une phase sur cinq (0,2) : le suivi sans prédiction ne prépare la région que si la dernière revue tombe assez près de
  l'impact — ici quand l'objet est à moins de ~4 m de son point d'impact.
- **Ce que la prédiction apporte n'est pas l'écart, c'est l'avance** : une fois l'objet dans l'eau, les deux ensembles suivent la
  même activité et donnent le même écart. La prédiction rend la préparation **indépendante de la cadence** de revue : 1,11 s
  d'avance, contre 0,01 à 0,41 s — la fenêtre de la source §8.2 et du palier T2.
- **Le prix** : 10 à 15 % de mailles en plus (part moyenne 0,468–0,480 contre 0,407–0,433) — la région calculée à δ = 0 avant
  l'impact. ADR-013 §2 la range en T2, *réservée* ; la référence la calcule (S401 §5, à la file).

## 4. Ce que ce document ne dit pas

- **Un corps quelconque** : la sphère englobante seulement ; la région d'un véhicule orienté, son point de contact réel.
- **Le vent**, une traînée qui dépend de l'orientation, la portance : absents ; l'incertitude est celle d'une traînée bornée.
- **L'entrée** est un volume déplacé, pas la physique de l'impact (la cavité de C4, la scène de C10) ; l'orientation et la rotation
  prédites ne sont consommées par rien encore.
- **Les objets contrôlables** (9.4) : `tier` porte `a_max`, rien ne vient du jeu ; la confiance réduite par le jeu (source §8.4).
- **Le précalcul** (9.6) : la région est préparée (l'ensemble), pas réservée dans un pool ni pourvue de proxys de collision ; aucune
  avance plus rapide que le temps réel.
- Aucun verdict visuel ; rien sur la carte.
