# La masse ajoutée implicite du corps libre — S655 (listes 6.7, 6.4)

*S655, 2026-10-07, en autonomie, vers la v2.* Le corps libre de S653 était lancé plus vite que l'eau par son couplage explicite (S654 :
4,68 m/s à 10 ms, 2,56 à 5 ms, la force lissée inchangée).

## Reproduire

- La flottaison, dans la suite : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core a_free_sphere_floats -- --nocapture`.
- Le corps sous le rouleau : `… the_added_mass_steadies_the_free_body -- --ignored --nocapture` ; les pas de 10 et de 5 ms ont tourné en
  parallèle, sur deux copies du binaire (≈ 8 et 18 min).
- Les essais d'APIC 3D : 55 passent, 16 longs ignorés.

## 1. Ce qui est construit

**La masse ajoutée traitée implicitement** : `(m + m_a)·aₙ₊₁ = F + m·g + m_a·aₙ`.

- `m_a = ½·ρ·V_imm`, où `V_imm` (`Apic3::immersed_body_volume`) compte les mailles du corps où `φ < 0`. La reconstruction reflète l'eau à
  travers la sphère depuis S393.
- L'accélération du pas précédent est gardée.
- À l'équilibre, rien ne change : on retrouve `m·a = F + m·g`.
- Sans masse, le corps reste imposé, comme avant.

## 2. Mesuré

| | couplage explicite (S653, S654) | masse ajoutée implicite (S655) |
|---|---|---|
| flottaison : `|v_z|` max entre 3 et 4 s | 4,1 cm/s | **1,0 cm/s** |
| flottaison : centre (niveau 0,40 m) | 0,387 m | 0,387 m |
| rouleau, 10 ms : vitesse max du corps / de l'eau de la sonde | 4,68 / 1,80 m/s | **1,87 / 2,00 m/s** |
| rouleau, 5 ms : vitesse max du corps / de l'eau de la sonde | 2,56 / 1,45 m/s | 2,32 / 1,50 m/s |
| rouleau : avance (10 ms ; 5 ms) | 2,03 ; 2,11 m | 1,75 ; 2,02 m |
| la masse de l'eau | au bit | au bit |

**Critères, écrits avant.**

- **(1) À moitié.**
  - L'écart entre les deux pas, 1,87 contre 2,32 m/s, vaut 19 % rapporté au plus grand, 24 % rapporté au plus petit. Le critère ne disait
    pas lequel : je le compte **manqué**.
  - À 10 ms, le corps reste sous la vitesse de l'eau de la sonde. **À 5 ms, il la dépasse** (2,32 contre 1,50) : manqué tel qu'écrit.
  - Le corps est emporté, la masse est exacte : tenu.
- **(2) Tenu**, et mieux : la flottaison tient maintenant le critère entier que S653 manquait.
- **(3) Tenu.**

**Le comparant n'était pas éprouvé (ADR-263 D2).** C'est une erreur de la famille relevée le jour même. La sonde de vitesse reste trois
mailles devant la position **de départ** du corps. Le corps, lui, voyage de 2 m jusque sous le jet, qui va à 3 m/s (S652). « Plus vite que
l'eau de la sonde » ne veut donc pas dire « plus vite que l'eau qui le porte ». Le prochain jugement compare la vitesse du corps à celle de
l'eau **autour de lui**, au même instant.

## 3. Ce que cela dit

La masse ajoutée implicite est le bon remède au couplage :

- le corps flotte et se pose, quatre fois plus vite amorti ;
- à 10 ms, sa vitesse sous le rouleau tombe de 4,68 à 1,87 m/s, l'ordre de l'eau qui le porte ;
- la dépendance au pas tombe de 83 % à 19–24 %.

Reste à juger sa vitesse contre l'eau qui l'entoure.

Manquent : la vitesse du corps comparée à l'eau locale, la rotation, un corps plus léger que sa masse ajoutée.
