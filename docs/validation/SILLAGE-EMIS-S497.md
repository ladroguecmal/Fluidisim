# Un corps en marche produit son sillage — S497 (liste 6.3, validée)

*S497, 2026-10-06, en autonomie.* ADR-103 raccordait au sillage « une trajectoire déclarée, pas un flux de poses moteur » ; l'émetteur
d'ADR-104 enchaîne des tronçons que l'hôte lui donne. Rien ne les choisissait depuis un corps qui bouge.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s497 -- --nocapture` — ≈ 2 s ; lignes `S497`. Suite du
  cœur : 655 essais.

## 1. La construction

`RigidBody::wake_leg` (`code/water-core/src/rigid_body.rs`) : le tronçon suivant de l'émetteur, visé du curseur — où la source en est —
vers la position **prédite** du corps à la fin du tronçon, `x + v·Δ`, sous la charge que porte sa coque, son poids `m·g`. Le chemin de la
source reste continu (l'émetteur l'exige) et se recale sur le corps à chaque tronçon : l'écart est celui de la prédiction, `½·|a|·Δ²`, et
ne s'accumule pas. L'hôte appelle `wake_leg`, prépare l'émission, l'admet au contrôleur de pression et l'acquitte (ADR-104).

## 2. Mesuré

La coque de la porte D (4 × 1,6 × 1 m, 3 200 kg, 31,4 kN) menée par le jeu sur un cercle de 20 m à 3 m/s, 12 s ; recette du sillage de
production (σ = 2 m, 64 × 128, coupure 3). La référence : la même trajectoire **déclarée** d'avance, soixante tronçons de 0,2 s entre les
positions du corps. Le sillage relevé sur 15 × 15 points de [−28, 28] m² à 4, 8 et 12 s.

| Δ | curseur / corps en fin de tronçon (prédit `½·U²/R·Δ²`) | sillage émis contre déclaré (max\|η\| = 21 cm) |
|---|---|---|
| 1 s | 0,2244 m (0,2250) | 9,5 mm — 4,47 % |
| 0,5 s | **0,0560 m (0,0563)** | **2,6 mm — 1,20 %** |
| 0,25 s | 0,0140 m (0,0141) | 0,7 mm — 0,34 % |

Ordres 1,89 et 1,83 (le `Δ²` de la prédiction) ; `P₀` = 1 249,048 Pa = `m·g/(2πσ²)` au bit ; aucun refus.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) le curseur à 1,2 × `½·(U²/R)·Δ²` du corps | 0,995 à 0,997 × la prédiction | tenu |
| (2) `P₀ = m·g/(2πσ²)` | au bit | tenu |
| (3) à Δ = 0,5 s, ≤ 10 % (prévu 2 à 6 %) ; ordre ≥ 1,7 | 1,20 % ; 1,89 et 1,83 | tenu (sous la prévision : l'erreur moyenne sur le tronçon, pas celle de sa fin) |
| (4) aucun refus | 0 | tenu |

**6.3 validée** — un objet en mouvement produit son sillage : le corps du jeu en marche émet la source de pression de W, que S495 fait
sentir aux autres corps. Hors de ce point : le champ proche d'une coque qui avance dans δ (6.4), la résistance de vague rendue au corps
(6.2, ADR-103 « pas de retour »). La liste : **6 points validés sur 120**.
