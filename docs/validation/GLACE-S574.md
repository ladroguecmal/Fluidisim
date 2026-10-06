# La glace : croissance et portance — S574 (listes 7.6, 7.7 ; SPEC-002 §4)

*S574, 2026-10-06, en autonomie.* 7.6 (glace et vapeur) était absent ; 7.7 attendait la glace porteuse (SPEC-006 §5.1 : `ice_h`,
`ice_capacity_kg`). ADR-027 §3 retient la glace, bornée aux lacs et aux baies abritées.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s574 -- --nocapture` ; suite du cœur : 749.

## 1. Ce qui est construit

`glace.rs` : la croissance de **Stefan** (`h = √(h₀² + 2·k·ΔT·t/(ρ·L))`, et en degrés-jours de gel) ; la portance de **Gold** (`P = A·h²`,
`A` = 3,5 kg/cm², la valeur prudente de Gold pour des charges mobiles) et son inverse ; la fraction émergée ; la formation en plaque
(`Hs < 0,15 m`). L'échantillon de traversabilité porte `ice_h` et `ice_capacity_kg`, dérivée une fois (`echantillon_glace`) ;
`porte_par_la_glace(masse, échantillon)`.

## 2. Mesuré (références écrites au plan par son script)

| | référence | mesuré |
|---|---|---|
| Stefan, 10 / 50 / 100 / 200 K·jour | 0,111410 / 0,249121 / 0,352310 / 0,498242 m (SPEC-002 : 11, 25, 35, 50 cm) | identiques à 10⁻⁶ |
| Gold, 5 / 10 / 20 / 30 / 50 cm | 87,5 / 350 / 1 400 / 3 150 / 8 750 kg | identiques |
| ce que chaque épaisseur porte (masses de référence 100, 400, 1 500, 5 000 kg) | rien / une personne / un groupe, une motoneige / une voiture légère / un camion léger — la table de SPEC-002 | **identique, ligne à ligne** |
| l'échéance : depuis 10 cm sous 10 K, une voiture légère portée | 228 714,1 s (2,647 jours) | **228 714,1 s** |
| émergé en eau douce ; la plaque à `Hs` = 0,15 / 0,149 m | 8,3 % ; non / oui | 8,3 % ; non / oui |
| l'échantillon sur 30 cm de glace | 3 150 kg ; une voiture portée, un camion non, rien sans glace (ajouté en route, écrit d'abord aux notes) | tenu |

Critères (écrits avant) : (1)–(5) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Un lac gelé dit ce qu'il porte et quand il le portera : « dans deux jours et demi de ce gel, une voiture passera ». Manquent pour 7.6 :
la vapeur, la glace dans les couches (un plan d'eau qui gèle dans B/V : son état, sa fonte), le gel limité aux plans d'eau marqués
gelables (ADR-027 §3), le rendu ; pour 7.7 : la température (l'hypothermie), la marée de B, la source réelle des échantillons.
