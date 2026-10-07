# Le ressaut mobile et le front sec : Stoker et Ritter en 2D — S627 (liste 4.14)

*S627, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* En eau peu profonde, une vague brisée est un ressaut mobile, que
Saint-Venant porte comme un choc. Cette session juge la capture des chocs de l'ordre deux (S620) sur deux ruptures de barrage analytiques :
Stoker (fond mouillé) et Ritter (fond sec — C04, qui ne tournait qu'en 1D).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s627 -- --nocapture` (1 s) ; suite du cœur : 813 essais
  listés.

## 1. Mesuré (références calculées au plan par `s627_ref.py`, numpy)

Une bande de trois mailles, 100 m, le barrage à 50 m ; 1 m d'eau à gauche, 0,5 m (Stoker) ou rien (Ritter) à droite ; t = 6 s. Stoker :
`h_m` = 0,726920 m, `u_m` = 0,923364 m/s, le ressaut à 2,957918 m/s.

| maille | Stoker, écart L1 | Ritter, écart L1 | front de Ritter au mm (exact 85,802 m) |
|---|---|---|---|
| 0,5 m | 3,6729·10⁻³ | 6,7456·10⁻³ | 79,75 m |
| 0,25 m | 1,7348·10⁻³ | 3,3755·10⁻³ | 81,625 m |
| 0,125 m | 8,6067·10⁻⁴ | 1,6938·10⁻³ | 83,0625 m |

Rapports de convergence 2,12 et 2,02 (Stoker), 2,00 et 1,99 (Ritter) ; la distance du front à sa position exacte divisée par 1,45 puis 1,53 ;
masse exacte, `h ≥ 0`. À 10⁻¹⁷ de numpy — la sensibilité à un ulp mesurée au plan (10⁻¹⁶) justifiait la tolérance de 10⁻¹² (ADR-256 D1).

Critères (écrits avant) : (1)–(4) — **tenus**.

## 2. Ce que cela dit — et ne dit pas

Le ressaut part à la bonne vitesse et laisse derrière lui la bonne hauteur ; l'écart, concentré au choc et au pied de la détente, se divise
par deux à chaque raffinement — l'ordre un, attendu à une discontinuité. Le front sec avance plus lentement qu'en théorie près de son bord
(la couche de moins d'un millimètre s'étale), et s'en rapproche à chaque raffinement : C04 tient en 2D.

Manquent : le passage d'une houle lisse au ressaut sur une pente (le déferlement lui-même, avec son critère), le rouleau 3D, la turbulence
du ressaut.
