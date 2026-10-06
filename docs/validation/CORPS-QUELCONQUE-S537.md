# Le contact d'un corps quelconque : son enveloppe convexe — S537 (liste 9.3)

*S537, 2026-10-06, en autonomie.* Le prédicteur balistique de S405 détectait le contact par la sphère englobante. Une planche qui tourne
touche l'eau par un coin, bien après sa sphère.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s537 -- --nocapture` ; suite du cœur : 697.

## 1. La construction (`code/water-core/src/ballistic.rs`)

**`predict_hull(objet, sommets, …)`** : le contact quand le **sommet le plus bas** de l'enveloppe convexe (ses sommets dans le repère du
corps, tournés par l'orientation intégrée en vol) atteint la surface à sa propre position horizontale ; l'instant par la même bisection sur
un pas de RK4 que `predict` ; la région utile, la plus grande distance d'un sommet au centre. `predict` inchangé.

## 2. Mesuré (lâché de 10 m, plan d'eau)

| | mesuré | attendu |
|---|---|---|
| boîte alignée (demi-hauteur 0,1 m) | 1,420685969472 s | `√(2(z₀ − h)/g)` : écart **3·10⁻¹³ s** |
| planche de 4 × 0,2 × 0,2 m tournant à 3 rad/s autour de `x` | 1,315550418023 s | la racine de `z₀ − ½gt² − (h_y\|sin ωt\| + h_z\|cos ωt\|)` par une bisection indépendante : écart **1,3·10⁻¹⁰ s** |
| la même par sa sphère englobante (`predict`) | 1,276703 s | **38,8 ms trop tôt** |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) la boîte alignée à 10⁻⁹ s | 3·10⁻¹³ s | tenu |
| (2) la planche tournante à 10⁻⁹ s ; la sphère publiée | 1,3·10⁻¹⁰ s ; 38,8 ms d'avance | tenu |
| (3) `predict` au bit | suite 697 | tenu |

Le seuil (10⁻⁹ s) contre la résolution de la bisection (10⁻¹² s) : un rapport de 1 000 (ADR-236 D1).

## 4. Ce qui manque

9.3 avance : le contact suit la forme réelle d'un corps convexe. Manquent un corps non convexe (une union d'enveloppes), le vent, et
l'entrée orientée consommée par δ (l'orientation et le coin au contact sont rendus, δ ne les lit pas encore).
