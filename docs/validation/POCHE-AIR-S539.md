# La flottabilité d'une poche d'air comprimée : la coque retournée — S539 (liste 7.5)

*S539, 2026-10-06, en autonomie.* ADR-015 §2–3 : « une coque retournée flotte grâce à l'air qu'elle emprisonne » ; la poche se comprime avec
la profondeur, et « un bateau chaviré flotte, puis passe un point de non-retour et coule d'un coup. Ce comportement — dramatique, juste, et
gratuit — sort d'une seule équation. »

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s539 -- --nocapture` ; suite du cœur : 700.

## 1. La construction (`code/water-core/src/rigid_body.rs`)

**`RigidBody::air_pocket : Option<AirPocket { body, volume_surface, thickness }>`** — une poche d'air portée par le corps : noyée à la
profondeur `d` de son centre, elle déplace `V₀·p_atm/(p_atm + ρ g d)` (isotherme, ADR-015 §3 « lente »), une poussée `ρ g V` en son centre
(la fraction d'immersion comme un point du proxy ; la pente de la surface comme S333). `None` par défaut : rien ne change.

## 2. Mesuré

| | mesuré | attendu |
|---|---|---|
| la poussée de la poche (2 m³ à la surface) à 0,5 / 10 / 20 / 30 m | à 10⁻¹² | Boyle : 100 / 50,19 / 33,50 / 25,14 % du volume (la table d'ADR-015) |
| un corps de 2 000 kg, 0,5 m³ de matière, 2 m³ d'air : le point de non-retour | — | `d*` = (p_atm/ρg)·(V₀/(m/ρ − V_s) − 1) = **3,811 m** |
| lâché au repos à 3,51 m | **remonte** : −0,22 m à 60 s (il flotte) | — |
| lâché au repos à 4,11 m | **coule** : −272 m à 60 s | — |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) la poussée à 10⁻¹² de Boyle (rapport à l'arrondi > 10⁶) | 10⁻¹² | tenu |
| (2) remonte à `d*` − 0,3 m ; coule à `d*` + 0,3 m | −0,22 m ; −272 m | tenu |
| (3) sans poche, la suite au bit | 700 | tenu |

## 4. Ce qui manque

**7.5 devient partiel** : l'air comprimé d'une poche portée par un corps. Manquent la poche adiabatique (γ = 1,4, les impacts), la poche
qui s'échappe quand le corps bascule (sa position suit la géométrie de la coque), la poche libre dans l'eau (la bulle, 7.4), le vide et
l'eau dans le vide, et le couplage avec une poche de V (S538) quand la coque est un compartiment.
