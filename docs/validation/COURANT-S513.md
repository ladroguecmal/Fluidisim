# Le courant derrière la requête de l'eau — S513 (listes 2.6 et 6.2)

*S513, 2026-10-06, en autonomie.* Le courant n'existait nulle part dans l'eau : 2.6 (« courants macroscopiques ») était absente, conçue par
[ADR-011](../adr/ADR-011-courants-et-ecoulements-diriges.md) ; 6.2 attendait « le courant, la turbulence ».

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s513 -- --nocapture` — trois essais ; suite du cœur :
  675 essais.

## 1. La construction (`code/water-core/src/rigid_body.rs`)

- **`Current`** — les niveaux C0 et C2 d'ADR-011 : le vecteur de surface, le vecteur au fond, l'échelle `D` du profil vertical
  `u(z) = u_fond + (u_surface − u_fond)·exp(z/D)`. En lecture seule (ADR-011 §2).
- **`CurrentWater`** — une requête qui en enveloppe une autre (B, ou B + W) : le champ de vagues **advecté** par le courant de surface,
  `η(x, t) = η₀(x − U·t, t)` (Galilée : pour un courant uniforme, l'effet Doppler `ω = ω₀ + k·U`) ; la vitesse de l'eau augmentée du profil
  C2 à la profondeur du point. Sans courant, l'enveloppée telle quelle.

## 2. Mesuré

| | mesuré |
|---|---|
| sans courant : surface, pente, vitesse, accélération, 40 points × 10 instants | **au bit** de `BackgroundWater` |
| houle de 6 s sous 1 m/s de courant dans son sens : période de rencontre au point fixe (dix périodes) | **5,42129 s** = `λ/(c + U)` (6 s sans courant) |
| le courant à 2 m sous la surface (`u_s` = 1, `u_f` = 0,2, `D` = 5 m) | 0,736256 m/s, **exactement** le profil |
| un pavé neutre immergé, traîné (`C_d` = 1), au repos dans 1 m/s : vitesse relative contre `w₀/(1 + k·w₀·t)` | **0,37 %** au pire sur 3 s (`k` = 10 m⁻¹) |

**Le manqué, et ce qu'il dit.** Le premier corps d'essai était la bouée flottante : 17 % d'écart. La traînée s'applique sous son centre de
gravité, la fait tanguer (jusqu'à 0,11 rad), et le tangage module la traînée — un couplage réel, que l'analytique de translation pure n'a
pas. Le pavé neutre entièrement immergé, traîné symétriquement, n'a aucun couple (ADR-228 D1 : le corps d'essai loin des couplages qu'on ne
juge pas).

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) sans courant, l'enveloppée au bit | au bit | tenu |
| (2) la période de rencontre `λ/(c + U)` à 0,5 % | exacte aux cinq chiffres | tenu |
| (3) le profil vertical à 10⁻¹² | exact | tenu |
| (4) la dérive à 1 % de l'analytique | 0,37 % | tenu (après un corps d'essai mal choisi) |

**2.6 passe à partiel** (C0 et C2 dans la requête ; manquent C1, le champ 2D régional précalculé, et C3, le champ 3D local de δ, les
rivières et les canaux). **6.2** : le courant fait ; reste la turbulence.
